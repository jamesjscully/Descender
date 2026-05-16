import Descender.PTND

namespace Descender

/-!
Advantage-based security interfaces.

The existing `P_TND_Decision.Distinguishes` predicate is a zero-error Boolean
idealization.  This file adds a compatible probability/advantage layer for
future reductions: adversaries are still Boolean distinguishers, but security is
measured by real-valued acceptance probabilities and distinguishing advantage.

The probability interface is deliberately lightweight.  Finite experiments are
represented by real-valued mass functions over finite types; abstract
experiments expose only bounded acceptance probabilities, avoiding a dependency
on Mathlib's measure/probability hierarchy.
-/

namespace Advantage

/-- Indicator of Boolean acceptance, as a real number. -/
def boolIndicator (b : Bool) : ℝ :=
  if b then 1 else 0

@[simp] theorem boolIndicator_true : boolIndicator true = 1 := by
  simp [boolIndicator]

@[simp] theorem boolIndicator_false : boolIndicator false = 0 := by
  simp [boolIndicator]

theorem boolIndicator_nonneg (b : Bool) : 0 ≤ boolIndicator b := by
  cases b <;> simp [boolIndicator]

theorem boolIndicator_le_one (b : Bool) : boolIndicator b ≤ 1 := by
  cases b <;> simp [boolIndicator]

/--
A finite probability distribution, stated only as the facts needed for
acceptance-probability calculations.
-/
structure FiniteProbability (Ω : Type*) [Fintype Ω] where
  mass : Ω -> ℝ
  nonnegative : ∀ ω, 0 ≤ mass ω
  total_mass : (Finset.univ.sum fun ω => mass ω) = 1

namespace FiniteProbability

variable {Ω : Type*} [Fintype Ω]

/-- Acceptance probability of a Boolean test under a finite distribution. -/
def acceptProb (D : FiniteProbability Ω) (A : Ω -> Bool) : ℝ :=
  Finset.univ.sum fun ω => D.mass ω * boolIndicator (A ω)

theorem acceptProb_nonneg (D : FiniteProbability Ω) (A : Ω -> Bool) :
    0 ≤ D.acceptProb A := by
  unfold acceptProb
  exact Finset.sum_nonneg (by
    intro ω _hω
    exact mul_nonneg (D.nonnegative ω) (boolIndicator_nonneg (A ω)))

theorem acceptProb_le_one (D : FiniteProbability Ω) (A : Ω -> Bool) :
    D.acceptProb A ≤ 1 := by
  unfold acceptProb
  calc
    (Finset.univ.sum fun ω => D.mass ω * boolIndicator (A ω))
        ≤ Finset.univ.sum fun ω => D.mass ω := by
          exact Finset.sum_le_sum (by
            intro ω _hω
            exact mul_le_of_le_one_right (D.nonnegative ω)
              (boolIndicator_le_one (A ω)))
    _ = 1 := D.total_mass

end FiniteProbability

/--
Abstract probability evaluation for Boolean tests.

This is the minimal interface reductions need: every Boolean distinguisher has
a real acceptance probability in `[0,1]`.
-/
structure AbstractProbability (Ω : Type*) where
  acceptProb : (Ω -> Bool) -> ℝ
  acceptProb_nonneg : ∀ A, 0 ≤ acceptProb A
  acceptProb_le_one : ∀ A, acceptProb A ≤ 1

namespace AbstractProbability

variable {Ω : Type*}

/-- Every finite distribution induces an abstract probability evaluator. -/
def ofFinite [Fintype Ω] (D : FiniteProbability Ω) :
    AbstractProbability Ω where
  acceptProb := D.acceptProb
  acceptProb_nonneg := D.acceptProb_nonneg
  acceptProb_le_one := D.acceptProb_le_one

/--
Push an abstract probability evaluator through a deterministic map.

This is the probability-side model of public serialization, bit-packing, and
rounding: an adversary on encoded samples is pulled back to an adversary on the
original sample space.
-/
def map {Ω' : Type*} (D : AbstractProbability Ω) (f : Ω -> Ω') :
    AbstractProbability Ω' where
  acceptProb := fun A => D.acceptProb (fun ω => A (f ω))
  acceptProb_nonneg := fun A => D.acceptProb_nonneg (fun ω => A (f ω))
  acceptProb_le_one := fun A => D.acceptProb_le_one (fun ω => A (f ω))

end AbstractProbability

/-- A real-vs-null distinguishing game over a common sample space. -/
structure DistinguishingGame (Sample : Type*) where
  real : AbstractProbability Sample
  null : AbstractProbability Sample

namespace DistinguishingGame

variable {Sample : Type*}

/-- Probability that an adversary accepts in the real experiment. -/
def realAcceptProb (G : DistinguishingGame Sample) (A : Sample -> Bool) : ℝ :=
  G.real.acceptProb A

/-- Probability that an adversary accepts in the null/ideal experiment. -/
def nullAcceptProb (G : DistinguishingGame Sample) (A : Sample -> Bool) : ℝ :=
  G.null.acceptProb A

/-- Absolute distinguishing advantage. -/
def advantage (G : DistinguishingGame Sample) (A : Sample -> Bool) : ℝ :=
  |G.realAcceptProb A - G.nullAcceptProb A|

theorem realAcceptProb_nonneg (G : DistinguishingGame Sample)
    (A : Sample -> Bool) : 0 ≤ G.realAcceptProb A :=
  G.real.acceptProb_nonneg A

theorem realAcceptProb_le_one (G : DistinguishingGame Sample)
    (A : Sample -> Bool) : G.realAcceptProb A ≤ 1 :=
  G.real.acceptProb_le_one A

theorem nullAcceptProb_nonneg (G : DistinguishingGame Sample)
    (A : Sample -> Bool) : 0 ≤ G.nullAcceptProb A :=
  G.null.acceptProb_nonneg A

theorem nullAcceptProb_le_one (G : DistinguishingGame Sample)
    (A : Sample -> Bool) : G.nullAcceptProb A ≤ 1 :=
  G.null.acceptProb_le_one A

theorem advantage_nonneg (G : DistinguishingGame Sample)
    (A : Sample -> Bool) : 0 ≤ G.advantage A := by
  unfold advantage
  exact abs_nonneg _

/-- Deterministically post-process both real and null experiments. -/
def map {Sample' : Type*} (G : DistinguishingGame Sample)
    (f : Sample -> Sample') : DistinguishingGame Sample' where
  real := G.real.map f
  null := G.null.map f

/--
Data processing for deterministic post-processing: the advantage after applying
`f` is exactly the advantage of the pulled-back adversary before applying `f`.
-/
theorem advantage_map_eq {Sample' : Type*}
    (G : DistinguishingGame Sample) (f : Sample -> Sample')
    (A : Sample' -> Bool) :
    (G.map f).advantage A = G.advantage (fun x => A (f x)) :=
  rfl

/-- Security predicate for one adversary at advantage bound `ε`. -/
def AdvantageAtMost (G : DistinguishingGame Sample)
    (A : Sample -> Bool) (ε : ℝ) : Prop :=
  G.advantage A ≤ ε

/-- Security predicate for a family of adversaries with a common bound. -/
def SecureAgainst {Adversary : Type*} (G : DistinguishingGame Sample)
    (run : Adversary -> Sample -> Bool) (allowed : Adversary -> Prop)
    (ε : ℝ) : Prop :=
  0 ≤ ε ∧ ∀ A, allowed A -> G.AdvantageAtMost (run A) ε

theorem advantageAtMost_map_of_pullback
    {Sample' : Type*} (G : DistinguishingGame Sample) (f : Sample -> Sample')
    {A : Sample' -> Bool} {ε : ℝ}
    (h : G.AdvantageAtMost (fun x => A (f x)) ε) :
    (G.map f).AdvantageAtMost A ε := by
  simpa [AdvantageAtMost, advantage_map_eq] using h

theorem secureAgainst_map_of_source_secure
    {Sample' Adversary : Type*} (G : DistinguishingGame Sample)
    (f : Sample -> Sample') (run : Adversary -> Sample' -> Bool)
    (allowed : Adversary -> Prop) {ε : ℝ}
    (hSecure :
      G.SecureAgainst (fun A x => run A (f x)) allowed ε) :
    (G.map f).SecureAgainst run allowed ε := by
  constructor
  · exact hSecure.1
  · intro A hA
    exact G.advantageAtMost_map_of_pullback f (hSecure.2 A hA)

theorem advantageAtMost_of_secureAgainst {Adversary : Type*}
    (G : DistinguishingGame Sample) {run : Adversary -> Sample -> Bool}
    {allowed : Adversary -> Prop} {ε : ℝ}
    (hSecure : G.SecureAgainst run allowed ε)
    {A : Adversary} (hA : allowed A) :
    G.AdvantageAtMost (run A) ε :=
  hSecure.2 A hA

end DistinguishingGame

/-- Finite real-vs-null distinguishing game. -/
structure FiniteDistinguishingGame (Sample : Type*) [Fintype Sample] where
  real : FiniteProbability Sample
  null : FiniteProbability Sample

namespace FiniteDistinguishingGame

variable {Sample : Type*} [Fintype Sample]

/-- View a finite game through the abstract game interface. -/
def toGame (G : FiniteDistinguishingGame Sample) :
    DistinguishingGame Sample where
  real := AbstractProbability.ofFinite G.real
  null := AbstractProbability.ofFinite G.null

def realAcceptProb (G : FiniteDistinguishingGame Sample)
    (A : Sample -> Bool) : ℝ :=
  G.toGame.realAcceptProb A

def nullAcceptProb (G : FiniteDistinguishingGame Sample)
    (A : Sample -> Bool) : ℝ :=
  G.toGame.nullAcceptProb A

def advantage (G : FiniteDistinguishingGame Sample)
    (A : Sample -> Bool) : ℝ :=
  G.toGame.advantage A

end FiniteDistinguishingGame

/--
An advantage-based P-TND experiment.  The `problem` field is the existing
deterministic P-TND decision surface; `game` supplies the real and null
probability evaluations used for average-case security.
-/
structure PTNDExperiment (q : Nat) [NeZero q]
    (Coord Secret Noise : Type*) [Fintype Coord] where
  problem : P_TND_Decision q Coord Secret Noise
  game : DistinguishingGame (Coord -> ZMod q)

namespace PTNDExperiment

variable {q : Nat} [NeZero q]
variable {Coord Secret Noise : Type*} [Fintype Coord]

def Adversary (_E : PTNDExperiment q Coord Secret Noise) : Type _ :=
  (Coord -> ZMod q) -> Bool

def realAcceptProb (E : PTNDExperiment q Coord Secret Noise)
    (A : E.Adversary) : ℝ :=
  E.game.realAcceptProb A

def nullAcceptProb (E : PTNDExperiment q Coord Secret Noise)
    (A : E.Adversary) : ℝ :=
  E.game.nullAcceptProb A

def advantage (E : PTNDExperiment q Coord Secret Noise)
    (A : E.Adversary) : ℝ :=
  E.game.advantage A

/--
Compatibility hook to the old zero-error idealization.  This is intentionally a
separate predicate; future reductions can relate it to `advantage` assumptions
without changing `P_TND_Decision.Distinguishes`.
-/
def ZeroErrorDistinguishes (E : PTNDExperiment q Coord Secret Noise)
    (A : E.Adversary) : Prop :=
  E.problem.Distinguishes A

def AdvantageAtMost (E : PTNDExperiment q Coord Secret Noise)
    (A : E.Adversary) (ε : ℝ) : Prop :=
  E.advantage A ≤ ε

def SecureAgainst (E : PTNDExperiment q Coord Secret Noise)
    (allowed : E.Adversary -> Prop) (ε : ℝ) : Prop :=
  0 ≤ ε ∧ ∀ A, allowed A -> E.AdvantageAtMost A ε

end PTNDExperiment

/--
A generic advantage problem: an adversary type, a way to run it as a Boolean
test, and the game in which it is measured.
-/
structure AdvantageProblem where
  Sample : Type
  Adversary : Type
  game : DistinguishingGame Sample
  run : Adversary -> Sample -> Bool
  allowed : Adversary -> Prop

namespace AdvantageProblem

def advantage (P : AdvantageProblem) (A : P.Adversary) : ℝ :=
  P.game.advantage (P.run A)

def AdvantageAtMost (P : AdvantageProblem) (A : P.Adversary)
    (ε : ℝ) : Prop :=
  P.advantage A ≤ ε

def SecureAt (P : AdvantageProblem) (ε : ℝ) : Prop :=
  0 ≤ ε ∧ ∀ A, P.allowed A -> P.AdvantageAtMost A ε

end AdvantageProblem

/-- A monotone loss function maps target bounds to source bounds. -/
def MonotoneLoss (loss : ℝ -> ℝ) : Prop :=
  (∀ ε, 0 ≤ ε -> 0 ≤ loss ε) ∧
    ∀ ε δ, 0 ≤ ε -> ε ≤ δ -> loss ε ≤ loss δ

/--
Pointwise reduction-loss predicate: the source advantage is bounded by applying
`loss` to the target advantage of the mapped adversary.
-/
def HasReductionLoss (source target : AdvantageProblem)
    (map : source.Adversary -> target.Adversary)
    (loss : ℝ -> ℝ) : Prop :=
  ∀ A, source.allowed A -> target.allowed (map A) ∧
    source.advantage A ≤ loss (target.advantage (map A))

/--
Bound-level reduction-loss predicate: any target bound `ε` transfers to the
source with bound `loss ε`.
-/
def TransfersSecurityBound (source target : AdvantageProblem)
    (map : source.Adversary -> target.Adversary)
    (loss : ℝ -> ℝ) : Prop :=
  ∀ A ε, source.allowed A -> target.AdvantageAtMost (map A) ε ->
    source.AdvantageAtMost A (loss ε)

structure AdvantageReduction (source target : AdvantageProblem) where
  map : source.Adversary -> target.Adversary
  loss : ℝ -> ℝ
  monotone_loss : MonotoneLoss loss
  sound : HasReductionLoss source target map loss

namespace AdvantageReduction

theorem transfersSecurityBound {source target : AdvantageProblem}
    (R : AdvantageReduction source target) :
    TransfersSecurityBound source target R.map R.loss := by
  intro A ε hA hTarget
  have hSound := R.sound A hA
  exact le_trans hSound.2
    ((R.monotone_loss.2 (target.advantage (R.map A)) ε
      (target.game.advantage_nonneg (target.run (R.map A))) hTarget))

theorem secureAt_of_target_secure {source target : AdvantageProblem}
    (R : AdvantageReduction source target) {ε : ℝ}
    (hTarget : target.SecureAt ε) :
    source.SecureAt (R.loss ε) := by
  constructor
  · exact R.monotone_loss.1 ε hTarget.1
  · intro A hA
    have hSound := R.sound A hA
    exact R.transfersSecurityBound A ε hA (hTarget.2 (R.map A) hSound.1)

end AdvantageReduction

end Advantage

end Descender
