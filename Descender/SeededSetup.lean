import Descender.TransparentSetup
import Descender.Compactness

namespace Descender

/-!
Seeded public setup certificates.

This file is the proof-layer hook for compact seeded public descriptions.  A
KEM implementation may publish seeds and deterministic regeneration rules for
the explicit projection data instead of publishing every coefficient,
incidence edge, and diagonal entry directly.  The cryptographic proof still
talks about explicit data; the certificate below records that public
verification accepts exactly the deterministic regenerated data, and that the
regenerated data satisfies the explicit setup predicate used by the rest of
the proof.
-/

/-- Public seeds for the three compact setup components used by the parameter search. -/
structure SeededProjectionSeeds
    (CoefficientSeed IncidenceSeed DiagonalSeed : Type*) where
  coefficient : CoefficientSeed
  incidence : IncidenceSeed
  diagonal : DiagonalSeed

/-- Explicit public projection data obtained after expanding public seeds. -/
structure RegeneratedProjectionData
    (Coefficients Incidence Diagonal : Type*) where
  coefficients : Coefficients
  incidence : Incidence
  diagonal : Diagonal

/--
Deterministic public regeneration interface.

`verify` is the public checker for a claimed explicit expansion.  The
certificate below supplies the two security-relevant facts: generated data
verifies, and any verifying data is equal to the deterministic regeneration.
-/
structure SeededProjectionRegenerator
    (CoefficientSeed IncidenceSeed DiagonalSeed : Type*)
    (Coefficients Incidence Diagonal : Type*) where
  regenerateCoefficients : CoefficientSeed -> Coefficients
  regenerateIncidence : IncidenceSeed -> Incidence
  regenerateDiagonal : DiagonalSeed -> Diagonal
  verify :
    SeededProjectionSeeds CoefficientSeed IncidenceSeed DiagonalSeed ->
      RegeneratedProjectionData Coefficients Incidence Diagonal -> Bool

namespace SeededProjectionRegenerator

variable {CoefficientSeed IncidenceSeed DiagonalSeed : Type*}
variable {Coefficients Incidence Diagonal : Type*}

/-- Expand all public setup seeds into the explicit public data. -/
def regenerate
    (R : SeededProjectionRegenerator CoefficientSeed IncidenceSeed
      DiagonalSeed Coefficients Incidence Diagonal)
    (seeds : SeededProjectionSeeds CoefficientSeed IncidenceSeed DiagonalSeed) :
    RegeneratedProjectionData Coefficients Incidence Diagonal where
  coefficients := R.regenerateCoefficients seeds.coefficient
  incidence := R.regenerateIncidence seeds.incidence
  diagonal := R.regenerateDiagonal seeds.diagonal

/-- Predicate form of public verification. -/
def Verifies
    (R : SeededProjectionRegenerator CoefficientSeed IncidenceSeed
      DiagonalSeed Coefficients Incidence Diagonal)
    (seeds : SeededProjectionSeeds CoefficientSeed IncidenceSeed DiagonalSeed)
    (explicit : RegeneratedProjectionData Coefficients Incidence Diagonal) :
    Prop :=
  R.verify seeds explicit = true

/-- The public verifier accepts deterministic regenerated data. -/
def GeneratedVerifies
    (R : SeededProjectionRegenerator CoefficientSeed IncidenceSeed
      DiagonalSeed Coefficients Incidence Diagonal) : Prop :=
  ∀ seeds, R.Verifies seeds (R.regenerate seeds)

/--
Every accepted explicit expansion is the deterministic public expansion.
This is the no-hidden-choice condition for seeded public setup.
-/
def Reproducible
    (R : SeededProjectionRegenerator CoefficientSeed IncidenceSeed
      DiagonalSeed Coefficients Incidence Diagonal) : Prop :=
  ∀ ⦃seeds explicit⦄, R.Verifies seeds explicit -> explicit = R.regenerate seeds

end SeededProjectionRegenerator

/--
Certificate that seeded setup is only a compact public representation of the
same explicit data used by the proof.

`ValidExplicitData` is intentionally abstract: concrete instantiations should
use it for the local-independence, no-backdoor, transversality, and sampler
facts that the explicit setup proof already requires.
-/
structure SeededProjectionCertificate
    {CoefficientSeed IncidenceSeed DiagonalSeed : Type*}
    {Coefficients Incidence Diagonal : Type*}
    (R : SeededProjectionRegenerator CoefficientSeed IncidenceSeed
      DiagonalSeed Coefficients Incidence Diagonal)
    (ValidExplicitData :
      RegeneratedProjectionData Coefficients Incidence Diagonal -> Prop) where
  generatedVerifies : R.GeneratedVerifies
  reproducible : R.Reproducible
  regeneratedValid : ∀ seeds, ValidExplicitData (R.regenerate seeds)

namespace SeededProjectionCertificate

variable {CoefficientSeed IncidenceSeed DiagonalSeed : Type*}
variable {Coefficients Incidence Diagonal : Type*}
variable {R : SeededProjectionRegenerator CoefficientSeed IncidenceSeed
  DiagonalSeed Coefficients Incidence Diagonal}
variable {ValidExplicitData :
  RegeneratedProjectionData Coefficients Incidence Diagonal -> Prop}

theorem generated_verifies
    (C : SeededProjectionCertificate R ValidExplicitData)
    (seeds : SeededProjectionSeeds CoefficientSeed IncidenceSeed DiagonalSeed) :
    R.Verifies seeds (R.regenerate seeds) :=
  C.generatedVerifies seeds

theorem verified_eq_regenerated
    (C : SeededProjectionCertificate R ValidExplicitData)
    {seeds : SeededProjectionSeeds CoefficientSeed IncidenceSeed DiagonalSeed}
    {explicit : RegeneratedProjectionData Coefficients Incidence Diagonal}
    (h : R.Verifies seeds explicit) :
    explicit = R.regenerate seeds :=
  C.reproducible h

theorem generated_valid
    (C : SeededProjectionCertificate R ValidExplicitData)
    (seeds : SeededProjectionSeeds CoefficientSeed IncidenceSeed DiagonalSeed) :
    ValidExplicitData (R.regenerate seeds) :=
  C.regeneratedValid seeds

theorem verified_valid
    (C : SeededProjectionCertificate R ValidExplicitData)
    {seeds : SeededProjectionSeeds CoefficientSeed IncidenceSeed DiagonalSeed}
    {explicit : RegeneratedProjectionData Coefficients Incidence Diagonal}
    (h : R.Verifies seeds explicit) :
    ValidExplicitData explicit := by
  rw [C.verified_eq_regenerated h]
  exact C.generated_valid seeds

end SeededProjectionCertificate

/--
The public layer needed by a compact seeded KEM instantiation.

It combines the existing transparent-setup and compact-evaluator certificates
with the seeded-regeneration certificate.  The rest of the KEM proof can then
consume transparent setup, compact public evaluators, and valid explicit setup
data without knowing whether the public data was transmitted densely or
expanded from public seeds.
-/
structure SeededPublicKEMLayer
    {Seed Parameters Description Step PrivateState : Type*}
    {Secret Measurement : Type*}
    {CoefficientSeed IncidenceSeed DiagonalSeed : Type*}
    {Coefficients Incidence Diagonal : Type*}
    (T : TransparentSetup Seed Parameters Description Step PrivateState)
    (operator : PublicOperator Secret Measurement)
    (sizeBudget : PublicSizeProfile) (costBudget : EvaluatorCost)
    (R : SeededProjectionRegenerator CoefficientSeed IncidenceSeed
      DiagonalSeed Coefficients Incidence Diagonal)
    (ValidExplicitData :
      RegeneratedProjectionData Coefficients Incidence Diagonal -> Prop) where
  setupCertificate : TransparentSetupCertificate T
  compactOperator : CompactOperatorCertificate operator sizeBudget costBudget
  seededCertificate : SeededProjectionCertificate R ValidExplicitData

namespace SeededPublicKEMLayer

variable {Seed Parameters Description Step PrivateState : Type*}
variable {Secret Measurement : Type*}
variable {CoefficientSeed IncidenceSeed DiagonalSeed : Type*}
variable {Coefficients Incidence Diagonal : Type*}
variable {T : TransparentSetup Seed Parameters Description Step PrivateState}
variable {operator : PublicOperator Secret Measurement}
variable {sizeBudget : PublicSizeProfile} {costBudget : EvaluatorCost}
variable {R : SeededProjectionRegenerator CoefficientSeed IncidenceSeed
  DiagonalSeed Coefficients Incidence Diagonal}
variable {ValidExplicitData :
  RegeneratedProjectionData Coefficients Incidence Diagonal -> Prop}

theorem transparent
    (L : SeededPublicKEMLayer T operator sizeBudget costBudget R
      ValidExplicitData) :
    T.IsTransparent :=
  L.setupCertificate.isTransparent

theorem public_bits_within_budget
    (L : SeededPublicKEMLayer T operator sizeBudget costBudget R
      ValidExplicitData) :
    L.compactOperator.compact.sizes.totalBits <= sizeBudget.totalBits :=
  L.compactOperator.totalPublicBits_le_budget

theorem evaluator_cost_within_budget
    (L : SeededPublicKEMLayer T operator sizeBudget costBudget R
      ValidExplicitData) :
    L.compactOperator.compact.evaluatorCost.total <= costBudget.total :=
  L.compactOperator.evaluatorCost_total_le_budget

theorem verified_seeded_data_valid
    (L : SeededPublicKEMLayer T operator sizeBudget costBudget R
      ValidExplicitData)
    {seeds : SeededProjectionSeeds CoefficientSeed IncidenceSeed DiagonalSeed}
    {explicit : RegeneratedProjectionData Coefficients Incidence Diagonal}
    (h : R.Verifies seeds explicit) :
    ValidExplicitData explicit :=
  L.seededCertificate.verified_valid h

theorem supplies_public_proof_inputs
    (L : SeededPublicKEMLayer T operator sizeBudget costBudget R
      ValidExplicitData) :
    T.IsTransparent ∧
      L.compactOperator.compact.sizes.totalBits <= sizeBudget.totalBits ∧
        L.compactOperator.compact.evaluatorCost.total <= costBudget.total ∧
          (∀ ⦃seeds explicit⦄,
            R.Verifies seeds explicit -> ValidExplicitData explicit) :=
  ⟨L.transparent, L.public_bits_within_budget,
    L.evaluator_cost_within_budget, by
      intro seeds explicit h
      exact L.verified_seeded_data_valid h⟩

end SeededPublicKEMLayer

end Descender
