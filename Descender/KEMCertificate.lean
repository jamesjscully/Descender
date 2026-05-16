import Descender.Security
import Descender.QuantumSecurity
import Descender.Advantage
import Descender.TransparentSetup
import Descender.Compactness
import Descender.QLatticeHardness
import Descender.SelfRandomizationConcrete

namespace Descender

/-!
Top-level Descender KEM certificate.

This module does not add a new cryptographic assumption.  It gathers the
separate proof obligations needed for the final claim:

* transparent public setup, with no hidden setup-side state;
* compact public evaluator descriptions for `A_mul` and `AT_mul`;
* explicit nonuniform-QPT adversary-class metadata;
* q-ary/noise promises for the concrete family;
* FO/CCA-to-CPA soundness;
* self-randomization from average P-TND to worst-case projected coboundary;
* worst-case hardness for the declared solver algorithm class.

The main theorem is still conditional, but the conditions are now the intended
system-level certificate fields rather than scattered assumptions.
-/

structure DescenderKEMCertificate
    (q : Nat) [NeZero q] (Coord Secret Noise : Type) [Fintype Coord]
    (Seed Parameters Description Step PrivateState Measurement
      PrimalWitness DualTarget : Type)
    [Zero DualTarget] where
  core : DescenderCPACore q Coord Secret Noise
  family : ProjectedCoboundaryFamily q Coord Secret
  worstCase : DescenderCPACore.WorstCaseProjectedCoboundaryProblem family
  CCAAdversary : Type
  BreaksCCA : CCAAdversary -> Prop

  setup : TransparentSetup Seed Parameters Description Step PrivateState
  setupCertificate : TransparentSetupCertificate setup

  publicOperator : PublicOperator Secret Measurement
  sizeBudget : PublicSizeProfile
  costBudget : EvaluatorCost
  compactOperator :
    CompactOperatorCertificate publicOperator sizeBudget costBudget

  securityModel : QuantumSecurity.SecurityParameterModel
  adversaryClass :
    QuantumSecurity.AlgorithmClass securityModel CCAAdversary
  adversariesAreNonuniformQPT : adversaryClass.IsNonuniformQPT

  qaryPromise :
    QaryPrimalDualSmoothingPromise q Coord PrimalWitness DualTarget
  noisePromise : CenteredNoiseSupportPromise q Coord Noise
  dualAmplitudeEta_le_noiseEta :
    qaryPromise.dualAmplitudeEta <= noisePromise.eta

  fo : core.FOImplicitRejectionProof CCAAdversary BreaksCCA
  selfRandomization : core.SelfRandomizationReduction worstCase
  worstCaseHard : Hard worstCase.Solves

namespace DescenderKEMCertificate

variable {q : Nat} [NeZero q] {Coord Secret Noise : Type} [Fintype Coord]
variable {Seed Parameters Description Step PrivateState Measurement
  PrimalWitness DualTarget : Type}
variable [Zero DualTarget]

theorem transparent
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    C.setup.IsTransparent :=
  C.setupCertificate.isTransparent

theorem no_hidden_setup_state
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget)
    (seed : Seed) (params : Parameters) :
    C.setup.privateState seed params =
      C.setup.privateStateFromTranscript (C.setup.transcript seed params) :=
  C.setupCertificate.no_hidden_state seed params

theorem verified_setup_reproducible
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget)
    {D : PublicDescription Seed Parameters Description}
    {tr : SetupTranscript Seed Parameters Description Step}
    (h : C.setup.Verifies D tr) :
    tr = C.setup.transcript D.seed D.parameters :=
  C.setupCertificate.verified_transcript_eq h

theorem compact_A_mul_spec
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget)
    (s : Secret) :
    C.compactOperator.compact.evaluators.A_mul.apply s =
      C.publicOperator.A s :=
  C.compactOperator.A_mul_spec s

theorem compact_AT_mul_spec
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget)
    (r : Measurement) :
    C.compactOperator.compact.evaluators.AT_mul.apply r =
      C.publicOperator.AT r :=
  C.compactOperator.AT_mul_spec r

theorem public_bits_within_budget
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    C.compactOperator.compact.sizes.totalBits <= C.sizeBudget.totalBits :=
  C.compactOperator.totalPublicBits_le_budget

theorem evaluator_cost_within_budget
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    C.compactOperator.compact.evaluatorCost.total <= C.costBudget.total :=
  C.compactOperator.evaluatorCost_total_le_budget

theorem adversary_class_nonuniform_qpt
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    C.adversaryClass.IsNonuniformQPT :=
  C.adversariesAreNonuniformQPT

theorem average_ptnd_hard
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    Hard C.core.SolvesAveragePTND :=
  C.selfRandomization.hard_average_ptnd_of_worst_case_hard C.worstCaseHard

theorem no_narrow_qary_dual_relations
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    NoNarrowQaryDualRelations q C.qaryPromise.dualAmplitudeEta
      C.qaryPromise.dualMap :=
  C.qaryPromise.noNarrowDual

theorem no_narrow_qary_dual_relations_at_noise
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    NoNarrowQaryDualRelations q C.noisePromise.eta
      C.qaryPromise.dualMap :=
  no_narrow_qary_dual_relations_mono_eta
    C.dualAmplitudeEta_le_noiseEta C.qaryPromise.noNarrowDual

theorem ind_cca_secure
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    NoBreak C.BreaksCCA :=
  C.core.no_cca_break_of_self_randomization C.family C.worstCase
    C.CCAAdversary C.BreaksCCA C.fo C.selfRandomization C.worstCaseHard

theorem ind_cca_secure_for_declared_class
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    ∀ A, C.adversaryClass.member A -> ¬ C.BreaksCCA A := by
  intro A _hA
  exact C.ind_cca_secure A

def kemSecurityCertificate
    (C : DescenderKEMCertificate q Coord Secret Noise Seed Parameters
      Description Step PrivateState Measurement PrimalWitness DualTarget) :
    DescenderKEMSecurityCertificate :=
  C.core.securityCertificateFromSelfRandomization C.family C.worstCase
    C.CCAAdversary C.BreaksCCA C.fo C.selfRandomization C.worstCaseHard

end DescenderKEMCertificate

end Descender
