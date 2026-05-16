import Descender.Security

namespace Descender

/-!
Quantum-aware security interfaces.

This module deliberately does not formalize quantum computation, circuit
families, oracle access, or concrete probability spaces.  It records the
parameters that cryptographic certificates should expose when their claims are
intended to range over nonuniform quantum polynomial-time adversaries:

* an explicit security-parameter model;
* rational advantage functions indexed by valid parameters;
* a negligible-bound predicate;
* algorithm-class markers for classical/quantum and uniform/nonuniform classes;
* quantitative reductions that preserve algorithm-class membership and account
  for an explicit loss term.

The definitions are conservative wrappers around the existing propositional
security layer in `Descender.Security`.
-/

namespace QuantumSecurity

/-- A family of valid security parameters with a natural-size measure. -/
structure SecurityParameterModel where
  Param : Type
  size : Param -> Nat
  valid : Param -> Prop

/-- A concrete valid security parameter for a model. -/
structure SecurityParameter (M : SecurityParameterModel) where
  val : M.Param
  isValid : M.valid val

namespace SecurityParameter

def size {M : SecurityParameterModel} (param : SecurityParameter M) : Nat :=
  M.size param.val

end SecurityParameter

/-- A rational-valued advantage function over valid security parameters. -/
abbrev Advantage (M : SecurityParameterModel) : Type :=
  SecurityParameter M -> Rat

/-- Pointwise nonnegativity for advantage bounds. -/
def NonnegativeAdvantage {M : SecurityParameterModel}
    (ε : Advantage M) : Prop :=
  ∀ param, 0 ≤ ε param

/-- Pointwise comparison between two advantage functions. -/
def AdvantageBoundedBy {M : SecurityParameterModel}
    (ε δ : Advantage M) : Prop :=
  ∀ param, ε param ≤ δ param

/--
The inverse-polynomial comparison scale used by `Negligible`.

The `n + 1` denominator avoids a separate nonzero-size side condition.
-/
def invPolynomialBound (c n : Nat) : Rat :=
  (1 : Rat) / (((n + 1) ^ c : Nat) : Rat)

/--
A conservative negligible predicate for rational advantage functions.

This is the standard "eventually below every inverse polynomial" shape, stated
over the model's size function.  It is an interface predicate, not an analytic
development of asymptotic complexity.
-/
def Negligible {M : SecurityParameterModel} (ε : Advantage M) : Prop :=
  NonnegativeAdvantage ε ∧
    ∀ c : Nat, ∃ N : Nat, ∀ param : SecurityParameter M,
      N ≤ param.size -> ε param ≤ invPolynomialBound c param.size

/-- A named advantage bound with its nonnegativity proof. -/
structure AdvantageBound (M : SecurityParameterModel) where
  bound : Advantage M
  nonnegative : NonnegativeAdvantage bound

namespace AdvantageBound

def negligible {M : SecurityParameterModel} (B : AdvantageBound M) : Prop :=
  Negligible B.bound

end AdvantageBound

/-- Marker for the abstract execution model of an algorithm family. -/
inductive ExecutionModel where
  | classical
  | quantum
  deriving DecidableEq

/-- Marker for whether a family is uniform or carries nonuniform advice. -/
inductive Uniformity where
  | uniform
  | nonuniform
  deriving DecidableEq

/--
An algorithm class over a security-parameter model.

The `polynomialTime` predicate is intentionally abstract: this file only
requires the class to expose which algorithms count as polynomial-time.  For
QPT or nonuniform QPT claims, use the `model` and `uniformity` fields together
with `member_polynomialTime`.
-/
structure AlgorithmClass (M : SecurityParameterModel) (Alg : Type*) where
  model : ExecutionModel
  uniformity : Uniformity
  polynomialTime : Alg -> Prop
  member : Alg -> Prop
  member_polynomialTime : ∀ A, member A -> polynomialTime A

namespace AlgorithmClass

def IsQuantum {M : SecurityParameterModel} {Alg : Type*}
    (C : AlgorithmClass M Alg) : Prop :=
  C.model = ExecutionModel.quantum

def IsUniform {M : SecurityParameterModel} {Alg : Type*}
    (C : AlgorithmClass M Alg) : Prop :=
  C.uniformity = Uniformity.uniform

def IsNonuniform {M : SecurityParameterModel} {Alg : Type*}
    (C : AlgorithmClass M Alg) : Prop :=
  C.uniformity = Uniformity.nonuniform

def IsQPT {M : SecurityParameterModel} {Alg : Type*}
    (C : AlgorithmClass M Alg) : Prop :=
  C.IsQuantum ∧ C.IsUniform

def IsNonuniformQPT {M : SecurityParameterModel} {Alg : Type*}
    (C : AlgorithmClass M Alg) : Prop :=
  C.IsQuantum ∧ C.IsNonuniform

def qpt {M : SecurityParameterModel} {Alg : Type*}
    (polynomialTime member : Alg -> Prop)
    (member_polynomialTime : ∀ A, member A -> polynomialTime A) :
    AlgorithmClass M Alg where
  model := ExecutionModel.quantum
  uniformity := Uniformity.uniform
  polynomialTime := polynomialTime
  member := member
  member_polynomialTime := member_polynomialTime

def nonuniformQPT {M : SecurityParameterModel} {Alg : Type*}
    (polynomialTime member : Alg -> Prop)
    (member_polynomialTime : ∀ A, member A -> polynomialTime A) :
    AlgorithmClass M Alg where
  model := ExecutionModel.quantum
  uniformity := Uniformity.nonuniform
  polynomialTime := polynomialTime
  member := member
  member_polynomialTime := member_polynomialTime

theorem qpt_is_qpt {M : SecurityParameterModel} {Alg : Type*}
    (polynomialTime member : Alg -> Prop)
    (member_polynomialTime : ∀ A, member A -> polynomialTime A) :
    (qpt (M := M) polynomialTime member member_polynomialTime).IsQPT := by
  constructor <;> rfl

theorem nonuniformQPT_is_nonuniformQPT
    {M : SecurityParameterModel} {Alg : Type*}
    (polynomialTime member : Alg -> Prop)
    (member_polynomialTime : ∀ A, member A -> polynomialTime A) :
    (nonuniformQPT (M := M) polynomialTime member
      member_polynomialTime).IsNonuniformQPT := by
  constructor <;> rfl

end AlgorithmClass

/--
A quantitative security game or problem family.

The advantage function is problem-specific, while `algClass` records which
algorithms are admissible for the claim.
-/
structure SecurityGame (M : SecurityParameterModel) where
  Algorithm : Type
  algClass : AlgorithmClass M Algorithm
  advantage : Algorithm -> Advantage M

namespace SecurityGame

/-- The game is secure against its declared algorithm class. -/
def Secure {M : SecurityParameterModel} (G : SecurityGame M) : Prop :=
  ∀ A, G.algClass.member A -> Negligible (G.advantage A)

/--
Qualitative no-break statement restricted to the declared algorithm class.

This bridges the existing `NoBreak` style with the new class-aware interface.
-/
def NoBreak {M : SecurityParameterModel} (G : SecurityGame M)
    (Breaks : G.Algorithm -> Prop) : Prop :=
  ∀ A, G.algClass.member A -> ¬ Breaks A

end SecurityGame

/--
A quantitative reduction between security games.

The `soundAdvantage` field states that the source advantage is bounded by the
target advantage of the transformed algorithm plus an explicit loss function.
-/
structure ReductionWithLoss {M : SecurityParameterModel}
    (Source Target : SecurityGame M) where
  map : Source.Algorithm -> Target.Algorithm
  preservesClass :
    ∀ A, Source.algClass.member A -> Target.algClass.member (map A)
  loss : Advantage M
  loss_nonnegative : NonnegativeAdvantage loss
  soundAdvantage :
    ∀ A param, Source.algClass.member A ->
      Source.advantage A param ≤ Target.advantage (map A) param + loss param

namespace ReductionWithLoss

variable {M : SecurityParameterModel}

def lossBound {Source Target : SecurityGame M}
    (R : ReductionWithLoss Source Target) : AdvantageBound M where
  bound := R.loss
  nonnegative := R.loss_nonnegative

/-- Quantitative reductions compose, and their losses add pointwise. -/
def compose {A B C : SecurityGame M}
    (R₁ : ReductionWithLoss A B) (R₂ : ReductionWithLoss B C) :
    ReductionWithLoss A C where
  map := fun a => R₂.map (R₁.map a)
  preservesClass := by
    intro a ha
    exact R₂.preservesClass (R₁.map a) (R₁.preservesClass a ha)
  loss := fun param => R₁.loss param + R₂.loss param
  loss_nonnegative := by
    intro param
    exact add_nonneg (R₁.loss_nonnegative param) (R₂.loss_nonnegative param)
  soundAdvantage := by
    intro a param ha
    have h₁ := R₁.soundAdvantage a param ha
    have h₂ := R₂.soundAdvantage (R₁.map a) param
      (R₁.preservesClass a ha)
    calc
      A.advantage a param ≤ B.advantage (R₁.map a) param +
          R₁.loss param := h₁
      _ ≤ (C.advantage (R₂.map (R₁.map a)) param + R₂.loss param) +
            R₁.loss param := by
          linarith
      _ = C.advantage (R₂.map (R₁.map a)) param +
            (R₁.loss param + R₂.loss param) := by
          ring

end ReductionWithLoss

/--
Class-aware quantitative analogue of the current Descender reduction pipeline.
-/
structure ParameterizedSecurityProblems (M : SecurityParameterModel) where
  cca : SecurityGame M
  cpa : SecurityGame M
  averagePTND : SecurityGame M
  worstCaseProjectedCoboundary : SecurityGame M

namespace ParameterizedSecurityProblems

structure ReductionPipeline {M : SecurityParameterModel}
    (P : ParameterizedSecurityProblems M) where
  ccaToCpa : ReductionWithLoss P.cca P.cpa
  cpaToAveragePTND : ReductionWithLoss P.cpa P.averagePTND
  averageToWorstCase :
    ReductionWithLoss P.averagePTND P.worstCaseProjectedCoboundary

def ccaToWorstCase {M : SecurityParameterModel}
    {P : ParameterizedSecurityProblems M} (R : P.ReductionPipeline) :
    ReductionWithLoss P.cca P.worstCaseProjectedCoboundary :=
  (R.ccaToCpa.compose R.cpaToAveragePTND).compose R.averageToWorstCase

end ParameterizedSecurityProblems

end QuantumSecurity

end Descender
