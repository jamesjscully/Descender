import Descender.Compactness
import Descender.DiagonalAmplification

namespace Descender

/-!
Proof-strategy comparison layer.

The project now has more than one plausible publicSize-sample proof route.  This
file keeps those routes as first-class certificate shapes so parameter search
can compare key size, setup/key-generation cost, validation cost, and proof
obligations without discarding conservative baselines.
-/

namespace SampleView

variable {q : Nat}
variable {FullCoord ViewCoord Secret Noise Seed : Type*}

/-- The proof route used to justify a publicSize sample view. -/
inductive StrategyKind where
  /-- Original/full coordinate publication, modeled as a surjective selected
  coordinate view. -/
  | fullSurjective
  /-- Exact compression: selected coordinates hit every relevant support. -/
  | supportCovered
  /-- Lossy compression: correctness is paid for by an explicit view budget. -/
  | lossyBudget
  /-- External or future proof strategy not yet classified. -/
  | other
deriving DecidableEq, Repr

/--
Operational accounting for a proof strategy.

`publicSize` counts transmitted/stored public material.  The other fields separate
the costs we should compare experimentally: key generation, encapsulation,
decapsulation, and setup/seed validation.
-/
structure StrategyCostProfile where
  publicSize : PublicSizeProfile
  keygen : EvaluatorCost
  encaps : EvaluatorCost
  decaps : EvaluatorCost
  validate : EvaluatorCost

namespace StrategyCostProfile

def totalWork (C : StrategyCostProfile) : Nat :=
  (((C.keygen.total + C.encaps.total) + C.decaps.total) + C.validate.total)

def BoundedBy (C B : StrategyCostProfile) : Prop :=
  C.publicSize.BoundedBy B.publicSize ∧
    C.keygen.BoundedBy B.keygen ∧
    C.encaps.BoundedBy B.encaps ∧
    C.decaps.BoundedBy B.decaps ∧
    C.validate.BoundedBy B.validate

theorem publicBits_le_of_boundedBy {C B : StrategyCostProfile}
    (h : C.BoundedBy B) :
    C.publicSize.totalBits <= B.publicSize.totalBits :=
  PublicSizeProfile.totalBits_le_totalBits_of_boundedBy h.1

theorem totalWork_le_of_boundedBy {C B : StrategyCostProfile}
    (h : C.BoundedBy B) :
    C.totalWork <= B.totalWork := by
  unfold totalWork
  exact Nat.add_le_add
    (Nat.add_le_add
      (Nat.add_le_add
        (EvaluatorCost.total_le_total_of_boundedBy h.2.1)
        (EvaluatorCost.total_le_total_of_boundedBy h.2.2.1))
      (EvaluatorCost.total_le_total_of_boundedBy h.2.2.2.1))
    (EvaluatorCost.total_le_total_of_boundedBy h.2.2.2.2)

end StrategyCostProfile

/--
A concrete proof-backed strategy candidate.

The `correctness` field is deliberately an existence certificate over the
strategy seed type.  For randomized diagonal strategies this seed is the public
diagonal; for a deterministic/lossy view it can be `PUnit`.
-/
structure StrategyCertificate
    (q : Nat) [NeZero q] (ViewCoord Secret Noise Seed : Type*)
    [Fintype ViewCoord] where
  kind : StrategyKind
  costs : StrategyCostProfile
  correctness :
    Nonempty (Σ _seed : Seed,
      PTNDCorrectnessCertificate q ViewCoord Secret Noise)

namespace StrategyCertificate

def publicBits {q : Nat} [NeZero q]
    {ViewCoord Secret Noise Seed : Type*} [Fintype ViewCoord]
    (C : StrategyCertificate q ViewCoord Secret Noise Seed) : Nat :=
  C.costs.publicSize.totalBits

def totalWork {q : Nat} [NeZero q]
    {ViewCoord Secret Noise Seed : Type*} [Fintype ViewCoord]
    (C : StrategyCertificate q ViewCoord Secret Noise Seed) : Nat :=
  C.costs.totalWork

theorem publicBits_le_budget {q : Nat} [NeZero q]
    {ViewCoord Secret Noise Seed : Type*} [Fintype ViewCoord]
    (C : StrategyCertificate q ViewCoord Secret Noise Seed)
    (B : StrategyCostProfile) (h : C.costs.BoundedBy B) :
    C.publicBits <= B.publicSize.totalBits :=
  StrategyCostProfile.publicBits_le_of_boundedBy h

theorem totalWork_le_budget {q : Nat} [NeZero q]
    {ViewCoord Secret Noise Seed : Type*} [Fintype ViewCoord]
    (C : StrategyCertificate q ViewCoord Secret Noise Seed)
    (B : StrategyCostProfile) (h : C.costs.BoundedBy B) :
    C.totalWork <= B.totalWork :=
  StrategyCostProfile.totalWork_le_of_boundedBy h

end StrategyCertificate

/--
Build the exact support-covered compression strategy certificate from the
compiled diagonal amplification theorem.
-/
def supportCoveredStrategyCertificate
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (costs : StrategyCostProfile)
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport eta failureBudget : Nat)
    (hMinSupport : 0 < minSupport)
    (hCover :
      ∀ pair : Secret × Secret,
        pair.1 ≠ pair.2 ->
        minSupport <=
          weight (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i) ->
        SupportCoveredByView (q := q) select
          (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i))
    (hNoise :
      ∀ seed : FullDiagonalSeed q ViewCoord,
        (scaledCoordinateRestriction
          (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmall P eta)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hFailureBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * eta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        failureBudget)
    (hBudget : failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)) :
    StrategyCertificate q ViewCoord Secret Noise
      (FullDiagonalSeed q ViewCoord) where
  kind := StrategyKind.supportCovered
  costs := costs
  correctness :=
    exists_scaledCoordinateRestriction_correctnessCertificate_full_of_supportCovered_select_prime
      (q := q) select P minSupport eta failureBudget hMinSupport hCover
      hNoise hSupport hFailureBudget hBudget

/-- Build the original/full-view strategy certificate from the surjective-view
special case. -/
def fullSurjectiveStrategyCertificate
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (costs : StrategyCostProfile)
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport eta failureBudget : Nat)
    (hselect : Function.Surjective select)
    (hMinSupport : 0 < minSupport)
    (hNoise :
      ∀ seed : FullDiagonalSeed q ViewCoord,
        (scaledCoordinateRestriction
          (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmall P eta)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hFailureBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * eta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        failureBudget)
    (hBudget : failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)) :
    StrategyCertificate q ViewCoord Secret Noise
      (FullDiagonalSeed q ViewCoord) where
  kind := StrategyKind.fullSurjective
  costs := costs
  correctness :=
    exists_scaledCoordinateRestriction_correctnessCertificate_full_of_surjective_select_prime
      (q := q) select P minSupport eta failureBudget hselect hMinSupport
      hNoise hSupport hFailureBudget hBudget

/--
Build a lossy-budget strategy certificate from the existing deterministic
view-budget theorem.

This does not use a random diagonal seed; the witness seed is `PUnit`.
-/
def lossyBudgetStrategyCertificate
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord]
    (costs : StrategyCostProfile)
    (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (B : ViewReductionBudget)
    (hNoise : V.NoiseSmallWithBudget P B)
    (hSep : V.EvalSeparatedWithBudget P B) :
    StrategyCertificate q ViewCoord Secret Noise PUnit where
  kind := StrategyKind.lossyBudget
  costs := costs
  correctness :=
    ⟨⟨PUnit.unit, V.correctnessCertificateWithBudget P B hNoise hSep⟩⟩

/--
Build a lossy-budget strategy certificate using the public diagonal
support-to-amplitude bridge.

Unlike `lossyBudgetStrategyCertificate`, this route has an explicit strategy
seed: the public diagonal that makes the viewed operator centered-separated at
the lossy radius `B.viewEta`.
-/
def lossyBudgetDiagonalStrategyCertificate
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq Seed] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (costs : StrategyCostProfile)
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget)
    (hNoise :
      ∀ seed,
        (scaledCoordinateRestriction (q := q) select (scale seed)).NoiseSmallWithBudget
          P B)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hBound :
      DiagonalAmplificationFailureCountAtMost (q := q) select scale P.eval
        minSupport B.viewEta B.failureBudget)
    (hBudget : B.failureBudget < Fintype.card Seed) :
    StrategyCertificate q ViewCoord Secret Noise Seed where
  kind := StrategyKind.lossyBudget
  costs := costs
  correctness :=
    exists_scaledCoordinateRestriction_correctnessCertificateWithBudget_of_failureCount
      (q := q) select scale P minSupport B hNoise hSupport hBound hBudget

/-- Prime-field full-diagonal specialization of the lossy-budget diagonal route
for support-covered compressed views. -/
def lossyBudgetSupportCoveredStrategyCertificate
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (costs : StrategyCostProfile)
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget)
    (hMinSupport : 0 < minSupport)
    (hCover :
      ∀ pair : Secret × Secret,
        pair.1 ≠ pair.2 ->
        minSupport <=
          weight (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i) ->
        SupportCoveredByView (q := q) select
          (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i))
    (hNoise :
      ∀ seed : FullDiagonalSeed q ViewCoord,
        (scaledCoordinateRestriction
          (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmallWithBudget
          P B)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hFailureBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * B.viewEta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        B.failureBudget)
    (hBudget : B.failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)) :
    StrategyCertificate q ViewCoord Secret Noise
      (FullDiagonalSeed q ViewCoord) where
  kind := StrategyKind.lossyBudget
  costs := costs
  correctness :=
    exists_scaledCoordinateRestriction_correctnessCertificateWithBudget_full_of_supportCovered_select_prime
      (q := q) select P minSupport B hMinSupport hCover hNoise hSupport
      hFailureBudget hBudget

end SampleView

end Descender
