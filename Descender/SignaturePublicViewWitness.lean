import Descender.EfficientPublicView
import Descender.SharedRowProductBlock
import Descender.SignatureTranscript

namespace Descender
namespace SignatureTranscript

/-!
Byte-level signature targets for public-view certificate violations.

`SignatureTranscript` already proves the linear byte theorem for an arbitrary
partial keygen witness.  This module instantiates that witness with the two
certificate-violation objects from `EfficientPublicView`:

* `PublicViewNarrowDualWitness`, a direct narrow public dual relation;
* `SystematicLeeViolation`, the structured `A = [I | P]` short relation.

The resulting theorem shape is the desired pivot: a clean queried forgery only
has to extract a certificate violation, and the no-break assumption is supplied
by the existing Lee/no-narrow certificate.
-/

noncomputable section

variable {q : Nat} [NeZero q]

/--
Treat a public narrow-dual relation as the partial keygen-breaking object.

The witness structure already contains the mathematical validity proof, so the
partial-witness predicate is `True`; no-break is obtained from the public-view
certificate ruling out inhabitants of this witness type.
-/
def publicViewNarrowDualKeygenBreakGame
    {LocalFree : Type*} {ViewCoord DualTarget : Type}
    [Fintype ViewCoord] [Zero DualTarget]
    (secret : N32Block × LocalFree -> ZMod q)
    (dualMap : (ViewCoord -> ZMod q) -> DualTarget)
    (eta : Nat) :
    N32PartialKeygenBreakGame (q := q) LocalFree
      (PublicViewNarrowDualWitness (q := q) dualMap eta)
      (PublicViewNarrowDualWitness (q := q) dualMap eta) where
  secret := secret
  ValidWitness := fun _ => True
  buildBreaker := id
  BreaksKeygen := fun _ => True
  buildBreaker_sound := by
    intro _witness _hvalid
    trivial

theorem noPublicViewNarrowDualBreaker_of_noNarrowDual
    {LocalFree : Type*} {ViewCoord DualTarget : Type}
    [Fintype ViewCoord] [Zero DualTarget]
    {secret : N32Block × LocalFree -> ZMod q}
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {eta : Nat}
    (C : NoNarrowQaryDualRelations q eta dualMap) :
    ∀ breaker,
      ¬ (publicViewNarrowDualKeygenBreakGame
          secret dualMap eta).BreaksKeygen breaker := by
  intro breaker _hbreak
  exact PublicViewNarrowDualWitness.impossible_of_noNarrowDual C breaker

theorem noPublicViewNarrowDualBreaker_of_certificate
    {LocalFree : Type*} {ViewCoord DualTarget : Type}
    [Fintype ViewCoord] [Zero DualTarget]
    {secret : N32Block × LocalFree -> ZMod q}
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {eta : Nat}
    (C : PublicViewLeeDualCertificate (q := q) dualMap eta) :
    ∀ breaker,
      ¬ (publicViewNarrowDualKeygenBreakGame
          secret dualMap eta).BreaksKeygen breaker :=
  noPublicViewNarrowDualBreaker_of_noNarrowDual C.noNarrowDual

/--
Byte-concrete linear target whose clean-forgery extractor returns a public
narrow-dual witness.
-/
structure N32BytePublicViewNarrowDualInputs
    {State QueryIdx LocalFree : Type*} {ViewCoord DualTarget : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype ViewCoord]
    [Zero DualTarget]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (dualMap : (ViewCoord -> ZMod q) -> DualTarget)
    (eta : Nat) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  forgery : N32QueryTrace State -> Bool
  structuralFailure : N32QueryTrace State -> Bool
  extract :
    N32QueryTrace State ->
      Option (PublicViewNarrowDualWitness (q := q) dualMap eta)
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb structuralFailure <= structuralFailureBound
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true
  clean_queried_forgery_extracts_witness :
    ∀ τ,
      forgery τ = true ->
      E.pointQueried τ = true ->
      structuralFailure τ = false ->
      ∃ witness,
        extract τ = some witness

namespace N32BytePublicViewNarrowDualInputs

variable {State QueryIdx LocalFree : Type*}
variable {ViewCoord DualTarget : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype ViewCoord]
variable [Zero DualTarget]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
variable {eta : Nat}

def forgerSuccessProb
    (I :
      N32BytePublicViewNarrowDualInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret dualMap eta) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

def keygenBreakGame :
    N32PartialKeygenBreakGame (q := q) LocalFree
      (PublicViewNarrowDualWitness (q := q) dualMap eta)
      (PublicViewNarrowDualWitness (q := q) dualMap eta) :=
  publicViewNarrowDualKeygenBreakGame secret dualMap eta

noncomputable def toPartialWitnessInputs
    (I :
      N32BytePublicViewNarrowDualInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret dualMap eta) :
    N32BytePartialWitnessLinearInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E
      (keygenBreakGame
        (LocalFree := LocalFree) (dualMap := dualMap) (eta := eta)
        (secret := secret)) where
  verifier := I.verifier
  forgery := I.forgery
  structuralFailure := I.structuralFailure
  extract := I.extract
  structuralFailureBound := I.structuralFailureBound
  structuralFailure_acceptProb_le := I.structuralFailure_acceptProb_le
  forgery_accepts := I.forgery_accepts
  clean_queried_forgery_extracts_valid_witness := by
    intro τ hforgery hqueried hstructural
    rcases I.clean_queried_forgery_extracts_witness
        τ hforgery hqueried hstructural with
      ⟨witness, hextract⟩
    exact ⟨witness, hextract, trivial⟩

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_noNarrowDual
    (I :
      N32BytePublicViewNarrowDualInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret dualMap eta)
    (C : NoNarrowQaryDualRelations q eta dualMap) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  simpa [forgerSuccessProb, toPartialWitnessInputs] using
    N32BytePartialWitnessLinearInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
      I.toPartialWitnessInputs
      (noPublicViewNarrowDualBreaker_of_noNarrowDual
        (secret := secret) C)

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_certificate
    (I :
      N32BytePublicViewNarrowDualInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret dualMap eta)
    (C : PublicViewLeeDualCertificate (q := q) dualMap eta) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound :=
  I.forgerSuccessProb_le_guess_add_structuralFailure_of_noNarrowDual
    C.noNarrowDual

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_efficientCertificate
    {FullCoord PTNDSecret Noise : Type}
    [Fintype FullCoord]
    {Pview : P_TND_Decision q FullCoord PTNDSecret Noise}
    {B : SampleView.ViewReductionBudget}
    (I :
      N32BytePublicViewNarrowDualInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret dualMap B.viewEta)
    (C : EfficientPublicViewCertificate (q := q) Pview dualMap B) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound :=
  I.forgerSuccessProb_le_guess_add_structuralFailure_of_certificate C.leeDual

end N32BytePublicViewNarrowDualInputs

/--
Packaged security certificate for the generic public-view narrow-dual target.

The only scheme-specific extraction work is inside `inputs`: a clean queried
forgery must return a `PublicViewNarrowDualWitness`.  The public-view Lee
certificate supplies the no-break side.
-/
structure N32BytePublicViewNarrowDualSecurityCertificate
    {State QueryIdx LocalFree : Type*} {ViewCoord DualTarget : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype ViewCoord]
    [Zero DualTarget]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (dualMap : (ViewCoord -> ZMod q) -> DualTarget)
    (eta : Nat) where
  inputs :
    N32BytePublicViewNarrowDualInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret dualMap eta
  leeDual : PublicViewLeeDualCertificate (q := q) dualMap eta

namespace N32BytePublicViewNarrowDualSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {ViewCoord DualTarget : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype ViewCoord]
variable [Zero DualTarget]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
variable {eta : Nat}

def forgerSuccessProb
    (C :
      N32BytePublicViewNarrowDualSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret dualMap eta) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32BytePublicViewNarrowDualSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret dualMap eta) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_certificate
      C.leeDual

end N32BytePublicViewNarrowDualSecurityCertificate

/--
Packaged security certificate when the public view is represented by an
`EfficientPublicViewCertificate`.
-/
structure N32ByteEfficientPublicViewNarrowDualSecurityCertificate
    {State QueryIdx LocalFree : Type*}
    {FullCoord ViewCoord PTNDSecret Noise DualTarget : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype FullCoord]
    [Fintype ViewCoord]
    [Zero DualTarget]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (Pview : P_TND_Decision q FullCoord PTNDSecret Noise)
    (dualMap : (ViewCoord -> ZMod q) -> DualTarget)
    (B : SampleView.ViewReductionBudget) where
  inputs :
    N32BytePublicViewNarrowDualInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret dualMap B.viewEta
  publicView :
    EfficientPublicViewCertificate (q := q) Pview dualMap B

namespace N32ByteEfficientPublicViewNarrowDualSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {FullCoord ViewCoord PTNDSecret Noise DualTarget : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype FullCoord]
variable [Fintype ViewCoord]
variable [Zero DualTarget]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {Pview : P_TND_Decision q FullCoord PTNDSecret Noise}
variable {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
variable {B : SampleView.ViewReductionBudget}

def forgerSuccessProb
    (C :
      N32ByteEfficientPublicViewNarrowDualSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret Pview dualMap B) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteEfficientPublicViewNarrowDualSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret Pview dualMap B) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_efficientCertificate
      C.publicView

end N32ByteEfficientPublicViewNarrowDualSecurityCertificate

/--
A short public relation in the kernel of a systematic public view.

This is the algebraic object a concrete verifier/extractor ultimately needs to
produce.  It is independent of byte serialization and challenge sampling.
-/
structure ShortSystematicKernelRelation
    {Pivot Free : Type}
    [Fintype Pivot] [Fintype Free]
    (P : Pivot -> Free -> ZMod q) (leeMin : Nat) where
  r : Sum Pivot Free -> ZMod q
  kernel : systematicDualMap q P r = 0
  nonzero : r ≠ 0
  short : centeredNormL1 q r < leeMin

namespace ShortSystematicKernelRelation

variable {Pivot Free : Type}
variable [Fintype Pivot] [Fintype Free]
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

def freePart
    (R : ShortSystematicKernelRelation (q := q) P leeMin) :
    Free -> ZMod q :=
  fun f => R.r (Sum.inr f)

def toSystematicLeeViolation
    (R : ShortSystematicKernelRelation (q := q) P leeMin) :
    SystematicLeeViolation (q := q) P leeMin where
  y := R.freePart
  nonzero := by
    intro hfree
    apply R.nonzero
    have hsys := eq_systematicRelation_of_systematicDualMap_eq_zero
      q P R.kernel
    rw [hsys]
    exact (systematicRelation_eq_zero_iff q P).mpr hfree
  short := by
    have hsys := eq_systematicRelation_of_systematicDualMap_eq_zero
      q P R.kernel
    change
      centeredNormL1 q
          (systematicRelation q P (fun f => R.r (Sum.inr f))) < leeMin
    rw [← hsys]
    exact R.short

end ShortSystematicKernelRelation

/--
Verifier-equation form of a short systematic relation.

The concrete extractor can expose separate pivot/free coordinates.  The
verifier equation is exactly the systematic public adjoint condition
`pivot + P * free = 0`; nonzero free part and a Lee bound turn it into a
certificate violation.
-/
structure ShortSystematicVerifierRelation
    {Pivot Free : Type}
    [Fintype Pivot] [Fintype Free]
    (P : Pivot -> Free -> ZMod q) (leeMin : Nat) where
  pivot : Pivot -> ZMod q
  free : Free -> ZMod q
  equation :
    ∀ p, pivot p + ∑ f, P p f * free f = 0
  free_nonzero : free ≠ 0
  short :
    centeredNormL1 q (Sum.elim pivot free) < leeMin

namespace ShortSystematicVerifierRelation

variable {Pivot Free : Type}
variable [Fintype Pivot] [Fintype Free]
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

def relation
    (R : ShortSystematicVerifierRelation (q := q) P leeMin) :
    Sum Pivot Free -> ZMod q :=
  Sum.elim R.pivot R.free

def toKernelRelation
    (R : ShortSystematicVerifierRelation (q := q) P leeMin) :
    ShortSystematicKernelRelation (q := q) P leeMin where
  r := R.relation
  kernel := by
    funext p
    simpa [relation, systematicDualMap] using R.equation p
  nonzero := by
    intro hzero
    apply R.free_nonzero
    funext f
    have hf := congrFun hzero (Sum.inr f)
    simpa [relation] using hf
  short := by
    simpa [relation] using R.short

def toSystematicLeeViolation
    (R : ShortSystematicVerifierRelation (q := q) P leeMin) :
    SystematicLeeViolation (q := q) P leeMin :=
  R.toKernelRelation.toSystematicLeeViolation

end ShortSystematicVerifierRelation

/--
Treat a systematic Lee-distance violation as the partial keygen-breaking
object.  Again, the witness carries its own validity proof, while certificates
prove that no such breaker exists.
-/
def systematicLeeViolationKeygenBreakGame
    {LocalFree : Type*} {Pivot Free : Type}
    [Fintype Pivot] [Fintype Free]
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) :
    N32PartialKeygenBreakGame (q := q) LocalFree
      (SystematicLeeViolation (q := q) P leeMin)
      (SystematicLeeViolation (q := q) P leeMin) where
  secret := secret
  ValidWitness := fun _ => True
  buildBreaker := id
  BreaksKeygen := fun _ => True
  buildBreaker_sound := by
    intro _witness _hvalid
    trivial

theorem noSystematicLeeViolationBreaker_of_distanceAtLeast
    {LocalFree : Type*} {Pivot Free : Type}
    [Fintype Pivot] [Fintype Free]
    {secret : N32Block × LocalFree -> ZMod q}
    {P : Pivot -> Free -> ZMod q}
    {leeMin : Nat}
    (hLee : SystematicLeeDistanceAtLeast q P leeMin) :
    ∀ breaker,
      ¬ (systematicLeeViolationKeygenBreakGame
          secret P leeMin).BreaksKeygen breaker := by
  intro breaker _hbreak
  exact SystematicLeeViolation.impossible_of_distanceAtLeast hLee breaker

theorem noSystematicLeeViolationBreaker_of_systematicCertificate
    {LocalFree : Type*} {Pivot Free : Type}
    [Fintype Pivot] [Fintype Free]
    {secret : N32Block × LocalFree -> ZMod q}
    {P : Pivot -> Free -> ZMod q}
    (C : SystematicLeeCertificate q Pivot Free P) :
    ∀ breaker,
      ¬ (systematicLeeViolationKeygenBreakGame
          secret P C.lowerBound).BreaksKeygen breaker :=
  noSystematicLeeViolationBreaker_of_distanceAtLeast C.lowerBoundProof

theorem noSystematicLeeViolationBreaker_of_publicViewLeeCertificate
    {LocalFree : Type*} {Pivot Free : Type}
    [Fintype Pivot] [Fintype Free]
    {secret : N32Block × LocalFree -> ZMod q}
    {P : Pivot -> Free -> ZMod q}
    {eta : Nat}
    (C : PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta) :
    ∀ breaker,
      ¬ (systematicLeeViolationKeygenBreakGame
          secret P C.leeMin).BreaksKeygen breaker := by
  intro breaker _hbreak
  exact
    SystematicLeeViolation.impossible_of_publicViewLeeDualCertificate C breaker

/--
Byte-concrete linear target whose clean-forgery extractor returns a systematic
Lee violation.
-/
structure N32ByteSystematicLeeViolationInputs
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  forgery : N32QueryTrace State -> Bool
  structuralFailure : N32QueryTrace State -> Bool
  extract :
    N32QueryTrace State ->
      Option (SystematicLeeViolation (q := q) P leeMin)
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb structuralFailure <= structuralFailureBound
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true
  clean_queried_forgery_extracts_witness :
    ∀ τ,
      forgery τ = true ->
      E.pointQueried τ = true ->
      structuralFailure τ = false ->
      ∃ witness,
        extract τ = some witness

namespace N32ByteSystematicLeeViolationInputs

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

def forgerSuccessProb
    (I :
      N32ByteSystematicLeeViolationInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

def keygenBreakGame :
    N32PartialKeygenBreakGame (q := q) LocalFree
      (SystematicLeeViolation (q := q) P leeMin)
      (SystematicLeeViolation (q := q) P leeMin) :=
  systematicLeeViolationKeygenBreakGame secret P leeMin

noncomputable def toPartialWitnessInputs
    (I :
      N32ByteSystematicLeeViolationInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    N32BytePartialWitnessLinearInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E
      (keygenBreakGame
        (LocalFree := LocalFree) (P := P) (leeMin := leeMin)
        (secret := secret)) where
  verifier := I.verifier
  forgery := I.forgery
  structuralFailure := I.structuralFailure
  extract := I.extract
  structuralFailureBound := I.structuralFailureBound
  structuralFailure_acceptProb_le := I.structuralFailure_acceptProb_le
  forgery_accepts := I.forgery_accepts
  clean_queried_forgery_extracts_valid_witness := by
    intro τ hforgery hqueried hstructural
    rcases I.clean_queried_forgery_extracts_witness
        τ hforgery hqueried hstructural with
      ⟨witness, hextract⟩
    exact ⟨witness, hextract, trivial⟩

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
    (I :
      N32ByteSystematicLeeViolationInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (hLee : SystematicLeeDistanceAtLeast q P leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  simpa [forgerSuccessProb, toPartialWitnessInputs] using
    N32BytePartialWitnessLinearInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
      I.toPartialWitnessInputs
      (noSystematicLeeViolationBreaker_of_distanceAtLeast
        (secret := secret) hLee)

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_systematicCertificate
    {leeMin : Nat}
    (I :
      N32ByteSystematicLeeViolationInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (C : SystematicLeeCertificate q Pivot Free P)
    (hbound : leeMin = C.lowerBound) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  subst hbound
  exact I.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
    C.lowerBoundProof

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
    (I :
      N32ByteSystematicLeeViolationInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    {eta : Nat}
    (C : PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta)
    (hbound : leeMin = C.leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  subst hbound
  simpa [forgerSuccessProb, toPartialWitnessInputs] using
    N32BytePartialWitnessLinearInputs.forgerSuccessProb_le_guess_add_structuralFailure_of_no_keygenBreaker
      I.toPartialWitnessInputs
      (noSystematicLeeViolationBreaker_of_publicViewLeeCertificate
        (secret := secret) C)

end N32ByteSystematicLeeViolationInputs

/--
Byte-level extractor target phrased in terms of the concrete systematic
verifier equation.

This is one step closer to an implementation than extracting
`SystematicLeeViolation` directly: the extractor returns pivot/free coordinates
that satisfy `pivot + P * free = 0`, are nonzero in the free part, and have
short serialized Lee norm.
-/
structure N32ByteShortSystematicRelationInputs
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  forgery : N32QueryTrace State -> Bool
  structuralFailure : N32QueryTrace State -> Bool
  relationExtractor :
    N32QueryTrace State ->
      Option (ShortSystematicVerifierRelation (q := q) P leeMin)
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb structuralFailure <= structuralFailureBound
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true
  clean_queried_forgery_extracts_relation :
    ∀ τ,
      forgery τ = true ->
      E.pointQueried τ = true ->
      structuralFailure τ = false ->
      ∃ relation,
        relationExtractor τ = some relation

namespace N32ByteShortSystematicRelationInputs

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

def forgerSuccessProb
    (I :
      N32ByteShortSystematicRelationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

def extract
    (I :
      N32ByteShortSystematicRelationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (τ : N32QueryTrace State) :
    Option (SystematicLeeViolation (q := q) P leeMin) :=
  match I.relationExtractor τ with
  | some relation => some relation.toSystematicLeeViolation
  | none => none

noncomputable def toSystematicLeeViolationInputs
    (I :
      N32ByteShortSystematicRelationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    N32ByteSystematicLeeViolationInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeMin where
  verifier := I.verifier
  forgery := I.forgery
  structuralFailure := I.structuralFailure
  extract := I.extract
  structuralFailureBound := I.structuralFailureBound
  structuralFailure_acceptProb_le := I.structuralFailure_acceptProb_le
  forgery_accepts := I.forgery_accepts
  clean_queried_forgery_extracts_witness := by
    intro τ hforgery hqueried hstructural
    rcases I.clean_queried_forgery_extracts_relation
        τ hforgery hqueried hstructural with
      ⟨relation, hrelation⟩
    exact ⟨relation.toSystematicLeeViolation, by
      simp [extract, hrelation]⟩

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
    (I :
      N32ByteShortSystematicRelationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (hLee : SystematicLeeDistanceAtLeast q P leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  simpa [forgerSuccessProb, toSystematicLeeViolationInputs,
    N32ByteSystematicLeeViolationInputs.forgerSuccessProb] using
    I.toSystematicLeeViolationInputs
      |>.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
        hLee

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
    (I :
      N32ByteShortSystematicRelationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    {eta : Nat}
    (C : PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta)
    (hbound : leeMin = C.leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  subst hbound
  simpa [forgerSuccessProb, toSystematicLeeViolationInputs,
    N32ByteSystematicLeeViolationInputs.forgerSuccessProb] using
    I.toSystematicLeeViolationInputs
      |>.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
        C rfl

end N32ByteShortSystematicRelationInputs

/--
Parser surface for extracting a candidate systematic relation from byte
responses.

The parser sees all seven response byte strings and the canonical challenge.
Concrete implementations can decode whichever response coordinates their
verifier relation uses, then expose the resulting systematic pivot/free vector.
-/
structure N32SystematicRelationByteParser
    {q : Nat} [NeZero q]
    (responseLen : Nat) (Pivot Free : Type) where
  pivot :
    (N32Repetition -> ByteVector responseLen) ->
      N32CanonicalChallenge -> Pivot -> ZMod q
  free :
    (N32Repetition -> ByteVector responseLen) ->
      N32CanonicalChallenge -> Free -> ZMod q

namespace N32SystematicRelationByteParser

variable {responseLen : Nat}
variable {Pivot Free : Type}
variable [Fintype Pivot] [Fintype Free]
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

def relation
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    (response : N32Repetition -> ByteVector responseLen)
    (challenge : N32CanonicalChallenge) :
    Sum Pivot Free -> ZMod q :=
  Sum.elim (D.pivot response challenge) (D.free response challenge)

noncomputable def equationCheck
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    (P : Pivot -> Free -> ZMod q)
    (response : N32Repetition -> ByteVector responseLen)
    (challenge : N32CanonicalChallenge) :
    Bool := by
  classical
  exact decide
    (∀ p,
      D.pivot response challenge p +
          ∑ f, P p f * D.free response challenge f =
        0)

noncomputable def freeNonzeroCheck
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    (response : N32Repetition -> ByteVector responseLen)
    (challenge : N32CanonicalChallenge) :
    Bool := by
  classical
  exact decide (D.free response challenge ≠ 0)

noncomputable def shortCheck
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    (leeMin : Nat)
    (response : N32Repetition -> ByteVector responseLen)
    (challenge : N32CanonicalChallenge) :
    Bool := by
  classical
  exact decide (centeredNormL1 q (D.relation response challenge) < leeMin)

noncomputable def verifierChecks
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat)
    (response : N32Repetition -> ByteVector responseLen)
    (challenge : N32CanonicalChallenge) :
    Bool :=
  (D.equationCheck P response challenge &&
      D.freeNonzeroCheck response challenge) &&
    D.shortCheck leeMin response challenge

theorem equationCheck_of_verifierChecks
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    {response : N32Repetition -> ByteVector responseLen}
    {challenge : N32CanonicalChallenge}
    (hchecks : D.verifierChecks P leeMin response challenge = true) :
    D.equationCheck P response challenge = true := by
  unfold verifierChecks at hchecks
  cases heq : D.equationCheck P response challenge <;>
    simp [heq] at hchecks ⊢

theorem freeNonzeroCheck_of_verifierChecks
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    {response : N32Repetition -> ByteVector responseLen}
    {challenge : N32CanonicalChallenge}
    (hchecks : D.verifierChecks P leeMin response challenge = true) :
    D.freeNonzeroCheck response challenge = true := by
  unfold verifierChecks at hchecks
  cases heq : D.equationCheck P response challenge <;>
    cases hnonzero : D.freeNonzeroCheck response challenge <;>
    simp [heq, hnonzero] at hchecks ⊢

theorem shortCheck_of_verifierChecks
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    {response : N32Repetition -> ByteVector responseLen}
    {challenge : N32CanonicalChallenge}
    (hchecks : D.verifierChecks P leeMin response challenge = true) :
    D.shortCheck leeMin response challenge = true := by
  unfold verifierChecks at hchecks
  cases heq : D.equationCheck P response challenge <;>
    cases hnonzero : D.freeNonzeroCheck response challenge <;>
    cases hshort : D.shortCheck leeMin response challenge <;>
    simp [heq, hnonzero, hshort] at hchecks ⊢

theorem equation_of_verifierChecks
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    {response : N32Repetition -> ByteVector responseLen}
    {challenge : N32CanonicalChallenge}
    (hchecks : D.verifierChecks P leeMin response challenge = true) :
    ∀ p,
      D.pivot response challenge p +
          ∑ f, P p f * D.free response challenge f =
        0 := by
  classical
  have heq := D.equationCheck_of_verifierChecks
    (P := P) (leeMin := leeMin) hchecks
  simpa [equationCheck] using heq

theorem free_nonzero_of_verifierChecks
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    {response : N32Repetition -> ByteVector responseLen}
    {challenge : N32CanonicalChallenge}
    (hchecks : D.verifierChecks P leeMin response challenge = true) :
    D.free response challenge ≠ 0 := by
  classical
  have hnonzero := D.freeNonzeroCheck_of_verifierChecks
    (P := P) (leeMin := leeMin) hchecks
  simpa [freeNonzeroCheck] using hnonzero

theorem short_of_verifierChecks
    (D : N32SystematicRelationByteParser (q := q) responseLen Pivot Free)
    {response : N32Repetition -> ByteVector responseLen}
    {challenge : N32CanonicalChallenge}
    (hchecks : D.verifierChecks P leeMin response challenge = true) :
    centeredNormL1 q (D.relation response challenge) < leeMin := by
  classical
  have hshort := D.shortCheck_of_verifierChecks
    (P := P) (leeMin := leeMin) hchecks
  simpa [shortCheck] using hshort

end N32SystematicRelationByteParser

/--
Default concrete carrier for response layouts that store the systematic
relation in one repetition.

Schemes can use another repetition by setting the `carrier` field of
`N32ExplicitSystematicRelationByteDecoder`; this value only records the most
direct layout choice.
-/
def n32FirstRelationCarrier : N32Repetition := 0

/--
Implementation-facing decoder for the systematic relation when one response
repetition explicitly carries the relation bytes.

This separates the byte packing problem from the algebraic verifier proof.  A
concrete scheme fixes a carrier repetition and two coordinate decoders for that
single response byte string; `toParser` then embeds that concrete layout into
the seven-response parser used by the reduction theorem.
-/
structure N32ExplicitSystematicRelationByteDecoder
    {q : Nat} [NeZero q]
    (responseLen : Nat) (Pivot Free : Type) where
  carrier : N32Repetition
  decodePivot :
    ByteVector responseLen -> N32CanonicalChallenge -> Pivot -> ZMod q
  decodeFree :
    ByteVector responseLen -> N32CanonicalChallenge -> Free -> ZMod q

namespace N32ExplicitSystematicRelationByteDecoder

variable {responseLen : Nat}
variable {Pivot Free : Type}

def toParser
    (D :
      N32ExplicitSystematicRelationByteDecoder
        (q := q) responseLen Pivot Free) :
    N32SystematicRelationByteParser (q := q) responseLen Pivot Free where
  pivot := fun response challenge =>
    D.decodePivot (response D.carrier) challenge
  free := fun response challenge =>
    D.decodeFree (response D.carrier) challenge

@[simp] theorem toParser_pivot
    (D :
      N32ExplicitSystematicRelationByteDecoder
        (q := q) responseLen Pivot Free)
    (response : N32Repetition -> ByteVector responseLen)
    (challenge : N32CanonicalChallenge)
    (p : Pivot) :
    D.toParser.pivot response challenge p =
      D.decodePivot (response D.carrier) challenge p :=
  rfl

@[simp] theorem toParser_free
    (D :
      N32ExplicitSystematicRelationByteDecoder
        (q := q) responseLen Pivot Free)
    (response : N32Repetition -> ByteVector responseLen)
    (challenge : N32CanonicalChallenge)
    (f : Free) :
    D.toParser.free response challenge f =
      D.decodeFree (response D.carrier) challenge f :=
  rfl

end N32ExplicitSystematicRelationByteDecoder

/-- Fixed-width slice of a byte vector at a proven-valid byte offset. -/
def byteVectorSlice {n width : Nat}
    (bytes : ByteVector n) (offset : Nat)
    (hfit : offset + width <= n) :
    ByteVector width :=
  fun i =>
    bytes
      ⟨offset + i.1,
        lt_of_lt_of_le (Nat.add_lt_add_left i.2 offset) hfit⟩

/-- Read a little-endian bit from a byte vector, returning `0` out of range. -/
def byteVectorBitLE {n : Nat} (bytes : ByteVector n) (bit : Nat) : Nat :=
  let byteIdx := bit / 8
  let bitIdx := bit % 8
  let byteNat :=
    if h : byteIdx < n then
      (bytes ⟨byteIdx, h⟩).1
    else
      0
  (byteNat / 2 ^ bitIdx) % 2

/--
Read `width` little-endian packed bits starting at `bitOffset`.

The result is total: out-of-range source bits decode as zero.  Concrete packed
layouts carry a separate `totalBitsFits` proof so the intended coordinates do
not rely on that fallback.
-/
def packedBitsNatLE {n : Nat}
    (bytes : ByteVector n) (bitOffset width : Nat) : Nat :=
  ∑ i : Fin width,
    byteVectorBitLE bytes (bitOffset + i.1) * 2 ^ i.1

theorem nat_div_pow_mod_two_eq_testBit_toNat (n i : Nat) :
    (n / 2 ^ i) % 2 = (n.testBit i).toNat := by
  rw [Nat.testBit, Nat.shiftRight_eq_div_pow, Nat.one_and_eq_mod_two]
  have hlt : (n / 2 ^ i) % 2 < 2 := Nat.mod_lt _ Nat.zero_lt_two
  interval_cases (n / 2 ^ i) % 2 <;> rfl

theorem sum_bool_bits_eq_ofBits (w : Nat) (f : Fin w -> Bool) :
    (∑ i : Fin w, (f i).toNat * 2 ^ i.1) = Nat.ofBits f := by
  induction w with
  | zero => simp
  | succ w ih =>
      rw [Nat.ofBits_succ]
      simp [Fin.sum_univ_succ]
      rw [← ih (f ∘ Fin.succ)]
      simp only [Function.comp_apply, pow_succ]
      have hsum :
          (∑ x : Fin w, (f x.succ).toNat * (2 ^ x.1 * 2)) =
            2 * ∑ x : Fin w, (f x.succ).toNat * 2 ^ x.1 := by
        rw [Finset.mul_sum]
        apply Finset.sum_congr rfl
        intro x _hx
        ring
      rw [hsum]
      omega

theorem packedBitLength_fits_bytePackedLength (entries bits : Nat) :
    entries * bits <= 8 * bytePackedLength entries bits := by
  unfold bytePackedLength
  omega

/--
Fixed-offset byte layout for the carried systematic relation.

Each pivot/free coordinate is stored in one fixed-width coefficient chunk.  The
layout supplies only a coefficient decoder; endian choice, rejection of
out-of-range residues, and signed/centered interpretation all live inside that
single decoder.
-/
structure N32FixedOffsetSystematicRelationByteLayout
    {q : Nat} [NeZero q]
    (responseLen bytesPerCoeff : Nat) (Pivot Free : Type) where
  carrier : N32Repetition := n32FirstRelationCarrier
  decodeCoeff : ByteVector bytesPerCoeff -> ZMod q
  pivotOffset : Pivot -> Nat
  freeOffset : Free -> Nat
  pivotFits : ∀ p, pivotOffset p + bytesPerCoeff <= responseLen
  freeFits : ∀ f, freeOffset f + bytesPerCoeff <= responseLen

namespace N32FixedOffsetSystematicRelationByteLayout

variable {responseLen bytesPerCoeff : Nat}
variable {Pivot Free : Type}

def decodePivot
    (L :
      N32FixedOffsetSystematicRelationByteLayout
        (q := q) responseLen bytesPerCoeff Pivot Free)
    (bytes : ByteVector responseLen) (_challenge : N32CanonicalChallenge)
    (p : Pivot) :
    ZMod q :=
  L.decodeCoeff (byteVectorSlice bytes (L.pivotOffset p) (L.pivotFits p))

def decodeFree
    (L :
      N32FixedOffsetSystematicRelationByteLayout
        (q := q) responseLen bytesPerCoeff Pivot Free)
    (bytes : ByteVector responseLen) (_challenge : N32CanonicalChallenge)
    (f : Free) :
    ZMod q :=
  L.decodeCoeff (byteVectorSlice bytes (L.freeOffset f) (L.freeFits f))

def toExplicitDecoder
    (L :
      N32FixedOffsetSystematicRelationByteLayout
        (q := q) responseLen bytesPerCoeff Pivot Free) :
    N32ExplicitSystematicRelationByteDecoder
      (q := q) responseLen Pivot Free where
  carrier := L.carrier
  decodePivot := L.decodePivot
  decodeFree := L.decodeFree

@[simp] theorem toExplicitDecoder_decodePivot
    (L :
      N32FixedOffsetSystematicRelationByteLayout
        (q := q) responseLen bytesPerCoeff Pivot Free)
    (bytes : ByteVector responseLen) (challenge : N32CanonicalChallenge)
    (p : Pivot) :
    L.toExplicitDecoder.decodePivot bytes challenge p =
      L.decodeCoeff (byteVectorSlice bytes (L.pivotOffset p) (L.pivotFits p)) :=
  rfl

@[simp] theorem toExplicitDecoder_decodeFree
    (L :
      N32FixedOffsetSystematicRelationByteLayout
        (q := q) responseLen bytesPerCoeff Pivot Free)
    (bytes : ByteVector responseLen) (challenge : N32CanonicalChallenge)
    (f : Free) :
    L.toExplicitDecoder.decodeFree bytes challenge f =
      L.decodeCoeff (byteVectorSlice bytes (L.freeOffset f) (L.freeFits f)) :=
  rfl

end N32FixedOffsetSystematicRelationByteLayout

/--
Contiguous pivot-then-free byte layout for the carried systematic relation.

This is the default efficient arrangement: no per-coordinate tags, no gaps, and
no repetition-local metadata.  Pivot coordinates occupy the first
`pivotCount * bytesPerCoeff` bytes and free coordinates follow immediately.
-/
structure N32ContiguousSystematicRelationByteLayout
    {q : Nat} [NeZero q]
    (responseLen bytesPerCoeff pivotCount freeCount : Nat)
    (Pivot Free : Type) where
  carrier : N32Repetition := n32FirstRelationCarrier
  decodeCoeff : ByteVector bytesPerCoeff -> ZMod q
  pivotIndex : Pivot -> Fin pivotCount
  freeIndex : Free -> Fin freeCount
  totalFits : (pivotCount + freeCount) * bytesPerCoeff <= responseLen

namespace N32ContiguousSystematicRelationByteLayout

variable {responseLen bytesPerCoeff pivotCount freeCount : Nat}
variable {Pivot Free : Type}

def pivotOffset
    (L :
      N32ContiguousSystematicRelationByteLayout
        (q := q) responseLen bytesPerCoeff pivotCount freeCount Pivot Free)
    (p : Pivot) :
    Nat :=
  (L.pivotIndex p).1 * bytesPerCoeff

def freeOffset
    (L :
      N32ContiguousSystematicRelationByteLayout
        (q := q) responseLen bytesPerCoeff pivotCount freeCount Pivot Free)
    (f : Free) :
    Nat :=
  (pivotCount + (L.freeIndex f).1) * bytesPerCoeff

def exact
    (decodeCoeff : ByteVector bytesPerCoeff -> ZMod q)
    (pivotIndex : Pivot -> Fin pivotCount)
    (freeIndex : Free -> Fin freeCount)
    (carrier : N32Repetition := n32FirstRelationCarrier) :
    N32ContiguousSystematicRelationByteLayout
      (q := q) ((pivotCount + freeCount) * bytesPerCoeff)
      bytesPerCoeff pivotCount freeCount Pivot Free where
  carrier := carrier
  decodeCoeff := decodeCoeff
  pivotIndex := pivotIndex
  freeIndex := freeIndex
  totalFits := le_rfl

def toFixedOffsetLayout
    (L :
      N32ContiguousSystematicRelationByteLayout
        (q := q) responseLen bytesPerCoeff pivotCount freeCount Pivot Free) :
    N32FixedOffsetSystematicRelationByteLayout
      (q := q) responseLen bytesPerCoeff Pivot Free where
  carrier := L.carrier
  decodeCoeff := L.decodeCoeff
  pivotOffset := L.pivotOffset
  freeOffset := L.freeOffset
  pivotFits := by
    intro p
    have hidx : (L.pivotIndex p).1 + 1 <= pivotCount :=
      Nat.succ_le_of_lt (L.pivotIndex p).2
    calc
      L.pivotOffset p + bytesPerCoeff =
          ((L.pivotIndex p).1 + 1) * bytesPerCoeff := by
            simp [pivotOffset, Nat.add_mul]
      _ <= pivotCount * bytesPerCoeff :=
          Nat.mul_le_mul_right bytesPerCoeff hidx
      _ <= (pivotCount + freeCount) * bytesPerCoeff :=
          Nat.mul_le_mul_right bytesPerCoeff
            (Nat.le_add_right pivotCount freeCount)
      _ <= responseLen := L.totalFits
  freeFits := by
    intro f
    have hidx : (L.freeIndex f).1 + 1 <= freeCount :=
      Nat.succ_le_of_lt (L.freeIndex f).2
    have hcoord :
        pivotCount + (L.freeIndex f).1 + 1 <= pivotCount + freeCount := by
      simp [Nat.add_assoc, Nat.add_le_add_left hidx pivotCount]
    calc
      L.freeOffset f + bytesPerCoeff =
          (pivotCount + (L.freeIndex f).1 + 1) * bytesPerCoeff := by
            simp [freeOffset, Nat.add_mul, Nat.add_assoc]
      _ <= (pivotCount + freeCount) * bytesPerCoeff :=
          Nat.mul_le_mul_right bytesPerCoeff hcoord
      _ <= responseLen := L.totalFits

def toExplicitDecoder
    (L :
      N32ContiguousSystematicRelationByteLayout
        (q := q) responseLen bytesPerCoeff pivotCount freeCount Pivot Free) :
    N32ExplicitSystematicRelationByteDecoder
      (q := q) responseLen Pivot Free :=
  L.toFixedOffsetLayout.toExplicitDecoder

end N32ContiguousSystematicRelationByteLayout

/--
Bit-packed contiguous pivot-then-free layout for the carried systematic
relation.

This is the compact field-element layout: coefficients are adjacent
`bitsPerCoeff`-bit words inside one response byte string.  It is the right
shape for the `q = 12289` coordinate-subview profile, whose canonical field
encoding uses 14 bits per coefficient.
-/
structure N32ContiguousPackedSystematicRelationByteLayout
    {q : Nat} [NeZero q]
    (responseLen bitsPerCoeff pivotCount freeCount : Nat)
    (Pivot Free : Type) where
  carrier : N32Repetition := n32FirstRelationCarrier
  decodeCoeffAt : ByteVector responseLen -> Nat -> ZMod q
  pivotIndex : Pivot -> Fin pivotCount
  freeIndex : Free -> Fin freeCount
  totalBitsFits : (pivotCount + freeCount) * bitsPerCoeff <= 8 * responseLen

namespace N32ContiguousPackedSystematicRelationByteLayout

variable {responseLen bitsPerCoeff pivotCount freeCount : Nat}
variable {Pivot Free : Type}

def pivotBitOffset
    (L :
      N32ContiguousPackedSystematicRelationByteLayout
        (q := q) responseLen bitsPerCoeff pivotCount freeCount Pivot Free)
    (p : Pivot) :
    Nat :=
  (L.pivotIndex p).1 * bitsPerCoeff

def freeBitOffset
    (L :
      N32ContiguousPackedSystematicRelationByteLayout
        (q := q) responseLen bitsPerCoeff pivotCount freeCount Pivot Free)
    (f : Free) :
    Nat :=
  (pivotCount + (L.freeIndex f).1) * bitsPerCoeff

def decodePivot
    (L :
      N32ContiguousPackedSystematicRelationByteLayout
        (q := q) responseLen bitsPerCoeff pivotCount freeCount Pivot Free)
    (bytes : ByteVector responseLen) (_challenge : N32CanonicalChallenge)
    (p : Pivot) :
    ZMod q :=
  L.decodeCoeffAt bytes (L.pivotBitOffset p)

def decodeFree
    (L :
      N32ContiguousPackedSystematicRelationByteLayout
        (q := q) responseLen bitsPerCoeff pivotCount freeCount Pivot Free)
    (bytes : ByteVector responseLen) (_challenge : N32CanonicalChallenge)
    (f : Free) :
    ZMod q :=
  L.decodeCoeffAt bytes (L.freeBitOffset f)

def toExplicitDecoder
    (L :
      N32ContiguousPackedSystematicRelationByteLayout
        (q := q) responseLen bitsPerCoeff pivotCount freeCount Pivot Free) :
    N32ExplicitSystematicRelationByteDecoder
      (q := q) responseLen Pivot Free where
  carrier := L.carrier
  decodePivot := L.decodePivot
  decodeFree := L.decodeFree

def exact
    (decodeCoeffAt :
      ByteVector (bytePackedLength (pivotCount + freeCount) bitsPerCoeff) ->
        Nat -> ZMod q)
    (pivotIndex : Pivot -> Fin pivotCount)
    (freeIndex : Free -> Fin freeCount)
    (carrier : N32Repetition := n32FirstRelationCarrier) :
    N32ContiguousPackedSystematicRelationByteLayout
      (q := q) (bytePackedLength (pivotCount + freeCount) bitsPerCoeff)
      bitsPerCoeff pivotCount freeCount Pivot Free where
  carrier := carrier
  decodeCoeffAt := decodeCoeffAt
  pivotIndex := pivotIndex
  freeIndex := freeIndex
  totalBitsFits := packedBitLength_fits_bytePackedLength
    (pivotCount + freeCount) bitsPerCoeff

end N32ContiguousPackedSystematicRelationByteLayout

/-- Concrete packed coefficient codec for `q = 12289`. -/
def zmod12289PackedCoeff
    {responseLen : Nat} (bitsPerCoeff : Nat)
    (bytes : ByteVector responseLen) (bitOffset : Nat) :
    ZMod 12289 :=
  (packedBitsNatLE bytes bitOffset bitsPerCoeff : ZMod 12289)

/-- Fourteen-bit packed coefficient codec for the `q = 12289` profile. -/
def zmod12289Packed14Coeff
    {responseLen : Nat} (bytes : ByteVector responseLen)
    (bitOffset : Nat) :
    ZMod 12289 :=
  zmod12289PackedCoeff 14 bytes bitOffset

def zmod12289Packed14CoeffNat
    {responseLen : Nat} (bytes : ByteVector responseLen)
    (bitOffset : Nat) :
    Nat :=
  packedBitsNatLE bytes bitOffset 14

def zmod12289Packed14CoeffNatAt
    {responseLen : Nat} (bytes : ByteVector responseLen)
    (baseBitOffset index : Nat) :
    Nat :=
  zmod12289Packed14CoeffNat bytes (baseBitOffset + index * 14)

/--
Canonical range check for the unused codes in a 14-bit `q = 12289`
coefficient encoding.
-/
def zmod12289Packed14RangeCheck
    {responseLen : Nat} (coordCount : Nat)
    (bytes : ByteVector responseLen) :
    Bool :=
  decide
    (∀ i : Fin coordCount,
      zmod12289Packed14CoeffNat bytes (i.1 * 14) < 12289)

theorem zmod12289Packed14Coeff_lt_of_rangeCheck
    {responseLen coordCount : Nat} {bytes : ByteVector responseLen}
    (hcheck : zmod12289Packed14RangeCheck coordCount bytes = true)
    (i : Fin coordCount) :
    zmod12289Packed14CoeffNat bytes (i.1 * 14) < 12289 := by
  exact (of_decide_eq_true hcheck) i

def zmod12289Packed14RangeCheckAt
    {responseLen : Nat} (baseBitOffset coordCount : Nat)
    (bytes : ByteVector responseLen) :
    Bool :=
  decide
    (∀ i : Fin coordCount,
      zmod12289Packed14CoeffNatAt bytes baseBitOffset i.1 < 12289)

theorem zmod12289Packed14CoeffAt_lt_of_rangeCheckAt
    {responseLen baseBitOffset coordCount : Nat}
    {bytes : ByteVector responseLen}
    (hcheck :
      zmod12289Packed14RangeCheckAt baseBitOffset coordCount bytes = true)
    (i : Fin coordCount) :
    zmod12289Packed14CoeffNatAt bytes baseBitOffset i.1 < 12289 := by
  exact (of_decide_eq_true hcheck) i

namespace N32Q12289SystematicRelationLayout

abbrev modulus : Nat := 12289
abbrev bitsPerCoeff : Nat := 14
abbrev target32PivotCount : Nat := 32
abbrev target32FreeCount : Nat := 32
abbrev validated48PivotCount : Nat := 48
abbrev validated48FreeCount : Nat := 32

def finPacked14
    (pivotCount freeCount : Nat)
    (carrier : N32Repetition := n32FirstRelationCarrier) :
    N32ContiguousPackedSystematicRelationByteLayout
      (q := modulus) (bytePackedLength (pivotCount + freeCount) bitsPerCoeff)
      bitsPerCoeff pivotCount freeCount (Fin pivotCount) (Fin freeCount) :=
  N32ContiguousPackedSystematicRelationByteLayout.exact
    (q := modulus)
    (bitsPerCoeff := bitsPerCoeff)
    (pivotCount := pivotCount)
    (freeCount := freeCount)
    (Pivot := Fin pivotCount)
    (Free := Fin freeCount)
    (fun bytes bitOffset => zmod12289Packed14Coeff bytes bitOffset)
    id id carrier

def target32x32 :
    N32ContiguousPackedSystematicRelationByteLayout
      (q := modulus) 112 bitsPerCoeff target32PivotCount target32FreeCount
      (Fin target32PivotCount) (Fin target32FreeCount) :=
  finPacked14 target32PivotCount target32FreeCount

theorem target32x32_responseLen_eq :
    bytePackedLength
        (target32PivotCount + target32FreeCount) bitsPerCoeff =
      112 := by
  native_decide

def target32x32Decoder :
    N32ExplicitSystematicRelationByteDecoder
      (q := modulus) 112 (Fin target32PivotCount) (Fin target32FreeCount) :=
  target32x32.toExplicitDecoder

def target32x32RangeCheck
    (response : N32Repetition -> ByteVector 112) :
    Bool :=
  zmod12289Packed14RangeCheck
    (target32PivotCount + target32FreeCount)
    (response n32FirstRelationCarrier)

def validated48x32 :
    N32ContiguousPackedSystematicRelationByteLayout
      (q := modulus) 140 bitsPerCoeff validated48PivotCount validated48FreeCount
      (Fin validated48PivotCount) (Fin validated48FreeCount) :=
  finPacked14 validated48PivotCount validated48FreeCount

theorem validated48x32_responseLen_eq :
    bytePackedLength
        (validated48PivotCount + validated48FreeCount) bitsPerCoeff =
      140 := by
  native_decide

def validated48x32Decoder :
    N32ExplicitSystematicRelationByteDecoder
      (q := modulus) 140 (Fin validated48PivotCount) (Fin validated48FreeCount) :=
  validated48x32.toExplicitDecoder

def validated48x32RangeCheck
    (response : N32Repetition -> ByteVector 140) :
    Bool :=
  zmod12289Packed14RangeCheck
    (validated48PivotCount + validated48FreeCount)
    (response n32FirstRelationCarrier)

namespace Validated48x32ProductBlock

abbrev Block := SharedRowProductBlock12289Rows3.Block
abbrev LocalPivot := SharedRowProductBlock12289Rows3.LocalPivot
abbrev LocalFree := SharedRowProductBlock12289Rows3.LocalFree
abbrev Pivot := SharedRowProductBlock12289Rows3.Pivot
abbrev Free := SharedRowProductBlock12289Rows3.Free

def pivotIndex (p : Pivot) : Fin validated48PivotCount :=
  ⟨p.1.1 * 3 + p.2.1, by
    change p.1.1 * 3 + p.2.1 < 48
    have hb := p.1.2
    have hp := p.2.2
    omega⟩

def freeIndex (f : Free) : Fin validated48FreeCount :=
  ⟨f.1.1 * 2 + f.2.1, by
    change f.1.1 * 2 + f.2.1 < 32
    have hb := f.1.2
    have hf := f.2.2
    omega⟩

def layout :
    N32ContiguousPackedSystematicRelationByteLayout
      (q := modulus) 140 bitsPerCoeff validated48PivotCount
      validated48FreeCount Pivot Free :=
  N32ContiguousPackedSystematicRelationByteLayout.exact
    (q := modulus)
    (bitsPerCoeff := bitsPerCoeff)
    (pivotCount := validated48PivotCount)
    (freeCount := validated48FreeCount)
    (Pivot := Pivot)
    (Free := Free)
    (fun bytes bitOffset => zmod12289Packed14Coeff bytes bitOffset)
    pivotIndex freeIndex

def decoder :
    N32ExplicitSystematicRelationByteDecoder
      (q := modulus) 140 Pivot Free :=
  layout.toExplicitDecoder

def rangeCheck (response : N32Repetition -> ByteVector 140) : Bool :=
  zmod12289Packed14RangeCheck
    (validated48PivotCount + validated48FreeCount)
    (response n32FirstRelationCarrier)

def payloadRangeCheck (payload : ByteVector 140) : Bool :=
  zmod12289Packed14RangeCheck
    (validated48PivotCount + validated48FreeCount) payload

def baseAcceptsPayload
    {publicKeyLen messageLen commitmentLen : Nat}
    (coreAccepts :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen) ->
      N32CanonicalChallenge -> ByteVector 140 -> Bool)
    (point :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (challenge : N32CanonicalChallenge)
    (payload : ByteVector 140) :
    Bool :=
  coreAccepts point challenge payload && payloadRangeCheck payload

end Validated48x32ProductBlock

end N32Q12289SystematicRelationLayout

/--
Serialized verifier whose accept predicate explicitly includes the systematic
relation check induced by a carried-response decoder.

`baseAccepts` contains all non-relation checks: parsing, challenge
recomputation, domain separation, and any scheme-specific well-formedness.
The constructed verifier accepts only when both `baseAccepts` and
`decoder.toParser.verifierChecks` accept.
-/
structure N32RelationCheckingByteVerifierInputs
    {State QueryIdx : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  domainTag : ByteVector domainLen
  response : State -> N32Repetition -> ByteVector responseLen
  decoder :
    N32ExplicitSystematicRelationByteDecoder
      (q := q) responseLen Pivot Free
  baseAccepts :
    N32AcceptingTranscript
      (ByteVector publicKeyLen) (ByteVector messageLen)
      (ByteVector commitmentLen) (ByteVector responseLen) -> Bool
  verifierPointFrame :
    State ->
      N32ChallengePointByteFrame domainLen publicKeyLen
        messageLen commitmentLen
  base_verifierPointFrame_matches_output :
    ∀ τ : N32QueryTrace State,
      baseAccepts
          { point := E.outputPoint τ.1
            challenge := E.outputChallenge τ.1
            response := response τ.1 } = true ->
        verifierPointFrame τ.1 =
          (N32ChallengePointByteCodec.byteVector domainTag).encodePoint
            (E.outputPoint τ.1)
  base_recomputedChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      baseAccepts
          { point := E.outputPoint τ.1
            challenge := E.outputChallenge τ.1
            response := response τ.1 } = true ->
        E.outputChallenge τ.1 = E.actualChallenge τ

namespace N32RelationCheckingByteVerifierInputs

variable {State QueryIdx : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

noncomputable def acceptsTranscript
    (I :
      N32RelationCheckingByteVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E P leeMin)
    (transcript :
      N32AcceptingTranscript
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen) (ByteVector responseLen)) :
    Bool :=
  I.baseAccepts transcript &&
    I.decoder.toParser.verifierChecks P leeMin
      transcript.response transcript.challenge

noncomputable def toVerifier
    (I :
      N32RelationCheckingByteVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E P leeMin) :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E where
  domainTag := I.domainTag
  response := I.response
  acceptsTranscript := I.acceptsTranscript
  verifierPointFrame := I.verifierPointFrame
  verifierPointFrame_matches_output := by
    intro τ haccept
    let transcript :
        N32AcceptingTranscript
          (ByteVector publicKeyLen) (ByteVector messageLen)
          (ByteVector commitmentLen) (ByteVector responseLen) :=
      { point := E.outputPoint τ.1
        challenge := E.outputChallenge τ.1
        response := I.response τ.1 }
    have hbase :
        I.baseAccepts transcript = true := by
      change I.acceptsTranscript transcript = true at haccept
      cases hb : I.baseAccepts transcript
      · simp [acceptsTranscript, hb] at haccept
      · rfl
    exact I.base_verifierPointFrame_matches_output τ hbase
  recomputedChallenge_matches_actual := by
    intro τ haccept
    let transcript :
        N32AcceptingTranscript
          (ByteVector publicKeyLen) (ByteVector messageLen)
          (ByteVector commitmentLen) (ByteVector responseLen) :=
      { point := E.outputPoint τ.1
        challenge := E.outputChallenge τ.1
        response := I.response τ.1 }
    have hbase :
        I.baseAccepts transcript = true := by
      change I.acceptsTranscript transcript = true at haccept
      cases hb : I.baseAccepts transcript
      · simp [acceptsTranscript, hb] at haccept
      · rfl
    exact I.base_recomputedChallenge_matches_actual τ hbase

theorem relationCheck_of_acceptsTranscript
    (I :
      N32RelationCheckingByteVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E P leeMin)
    {transcript :
      N32AcceptingTranscript
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen) (ByteVector responseLen)}
    (haccept : I.toVerifier.acceptsTranscript transcript = true) :
    I.decoder.toParser.verifierChecks P leeMin
      transcript.response transcript.challenge = true := by
  simp [toVerifier] at haccept
  cases hb : I.baseAccepts transcript
  · simp [acceptsTranscript, hb] at haccept
  · simp [acceptsTranscript, hb] at haccept
    exact haccept

theorem relationCheck_of_oneShotAccepts
    (I :
      N32RelationCheckingByteVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E P leeMin)
    {state : State} {challenge : N32CanonicalChallenge}
    (haccept :
      I.toVerifier.toSerializedVerifier.oneShotAccepts state challenge =
        true) :
    I.decoder.toParser.verifierChecks P leeMin
      (I.response state) challenge = true := by
  exact
    I.relationCheck_of_acceptsTranscript
      (transcript :=
        { point := E.outputPoint state
          challenge := challenge
          response := I.response state })
      (by
        simpa [N32ByteSerializedVerifierInputs.toSerializedVerifier,
          N32SerializedVerifier.oneShotAccepts,
          N32SerializedVerifier.programmedTranscript] using haccept)

end N32RelationCheckingByteVerifierInputs

/-- Fixed all-zero byte vector used for non-carrier proof-interface slots. -/
def zeroByteVector (n : Nat) : ByteVector n :=
  fun _ => 0

/--
Single-carrier serialized relation payload.

The concrete signature carries one response byte string, `payload`.  To reuse
the existing seven-repetition verifier interface, the payload is expanded into
the decoder's carrier repetition and every non-carrier repetition is filled by
a fixed zero byte vector.  The relation decoder only reads its carrier, so the
extra proof-interface slots do not contribute to serialized signature size.
-/
structure N32SingleCarrierRelationPayloadVerifierInputs
    {State QueryIdx : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (decoder :
      N32ExplicitSystematicRelationByteDecoder
        (q := q) responseLen Pivot Free)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  domainTag : ByteVector domainLen
  payload : State -> ByteVector responseLen
  baseAcceptsPayload :
    N32ChallengePoint
      (ByteVector publicKeyLen) (ByteVector messageLen)
      (ByteVector commitmentLen) ->
      N32CanonicalChallenge -> ByteVector responseLen -> Bool
  verifierPointFrame :
    State ->
      N32ChallengePointByteFrame domainLen publicKeyLen
        messageLen commitmentLen
  base_verifierPointFrame_matches_output :
    ∀ τ : N32QueryTrace State,
      baseAcceptsPayload
          (E.outputPoint τ.1) (E.outputChallenge τ.1)
          (payload τ.1) = true ->
        verifierPointFrame τ.1 =
          (N32ChallengePointByteCodec.byteVector domainTag).encodePoint
            (E.outputPoint τ.1)
  base_recomputedChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      baseAcceptsPayload
          (E.outputPoint τ.1) (E.outputChallenge τ.1)
          (payload τ.1) = true ->
        E.outputChallenge τ.1 = E.actualChallenge τ

namespace N32SingleCarrierRelationPayloadVerifierInputs

variable {State QueryIdx : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {decoder :
  N32ExplicitSystematicRelationByteDecoder
    (q := q) responseLen Pivot Free}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

def response
    (I :
      N32SingleCarrierRelationPayloadVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E decoder P leeMin)
    (state : State) (r : N32Repetition) :
    ByteVector responseLen :=
  if r = decoder.carrier then I.payload state else zeroByteVector responseLen

@[simp] theorem response_carrier
    (I :
      N32SingleCarrierRelationPayloadVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E decoder P leeMin)
    (state : State) :
    I.response state decoder.carrier = I.payload state := by
  simp [response]

noncomputable def toRelationCheckingInputs
    (I :
      N32SingleCarrierRelationPayloadVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E decoder P leeMin) :
    N32RelationCheckingByteVerifierInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E P leeMin where
  domainTag := I.domainTag
  response := I.response
  decoder := decoder
  baseAccepts := fun transcript =>
    I.baseAcceptsPayload transcript.point transcript.challenge
      (transcript.response decoder.carrier)
  verifierPointFrame := I.verifierPointFrame
  base_verifierPointFrame_matches_output := by
    intro τ hbase
    exact I.base_verifierPointFrame_matches_output τ (by
      simpa [response] using hbase)
  base_recomputedChallenge_matches_actual := by
    intro τ hbase
    exact I.base_recomputedChallenge_matches_actual τ (by
      simpa [response] using hbase)

noncomputable def toVerifier
    (I :
      N32SingleCarrierRelationPayloadVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E decoder P leeMin) :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E :=
  I.toRelationCheckingInputs.toVerifier

theorem relationCheck_of_oneShotAccepts
    (I :
      N32SingleCarrierRelationPayloadVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E decoder P leeMin)
    {state : State} {challenge : N32CanonicalChallenge}
    (haccept :
      I.toVerifier.toSerializedVerifier.oneShotAccepts state challenge =
        true) :
    decoder.toParser.verifierChecks P leeMin
      (I.response state) challenge = true :=
  I.toRelationCheckingInputs.relationCheck_of_oneShotAccepts haccept

def serializedSignatureBytes (responseLen : Nat) : Nat :=
  N32CanonicalChallengeByteCodec.packedChallengeBytes + responseLen

end N32SingleCarrierRelationPayloadVerifierInputs

namespace N32Q12289SystematicRelationLayout

def target32x32SingleCarrierSignatureBytes : Nat :=
  N32SingleCarrierRelationPayloadVerifierInputs.serializedSignatureBytes 112

theorem target32x32SingleCarrierSignatureBytes_eq :
    target32x32SingleCarrierSignatureBytes = 129 := by
  native_decide

def validated48x32SingleCarrierSignatureBytes : Nat :=
  N32SingleCarrierRelationPayloadVerifierInputs.serializedSignatureBytes 140

theorem validated48x32SingleCarrierSignatureBytes_eq :
    validated48x32SingleCarrierSignatureBytes = 157 := by
  native_decide

abbrev Target32x32SingleCarrierVerifierInputs
    {State QueryIdx : Type*}
    [Fintype State] [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (P : Fin target32PivotCount -> Fin target32FreeCount -> ZMod modulus)
    (leeMin : Nat) :=
  N32SingleCarrierRelationPayloadVerifierInputs
    (q := modulus)
    (domainLen := domainLen)
    (publicKeyLen := publicKeyLen)
    (messageLen := messageLen)
    (commitmentLen := commitmentLen)
    (responseLen := 112) E target32x32Decoder P leeMin

abbrev Validated48x32SingleCarrierVerifierInputs
    {State QueryIdx : Type*}
    [Fintype State] [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (P : Fin validated48PivotCount -> Fin validated48FreeCount -> ZMod modulus)
    (leeMin : Nat) :=
  N32SingleCarrierRelationPayloadVerifierInputs
    (q := modulus)
    (domainLen := domainLen)
    (publicKeyLen := publicKeyLen)
    (messageLen := messageLen)
    (commitmentLen := commitmentLen)
    (responseLen := 140) E validated48x32Decoder P leeMin

abbrev Validated48x32ProductBlockSingleCarrierVerifierInputs
    {State QueryIdx : Type*}
    [Fintype State] [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)) :=
  N32SingleCarrierRelationPayloadVerifierInputs
    (q := modulus)
    (domainLen := domainLen)
    (publicKeyLen := publicKeyLen)
    (messageLen := messageLen)
    (commitmentLen := commitmentLen)
    (responseLen := 140) E
    N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.decoder
    SharedRowProductBlock12289Rows3.globalMatrix
    SharedRowProductBlock12289Rows3.targetLee

/--
Canonical base verifier for the validated `q = 12289` product-block route.

The supplied `coreAcceptsPayload` contains the implementation-specific checks
such as message/public-key parsing and Fiat-Shamir recomputation.  This wrapper
adds the canonical 14-bit range check for the packed relation payload, so the
single-carrier verifier cannot accidentally accept noncanonical field encodings.
-/
structure Validated48x32ProductBlockCanonicalBaseVerifierInputs
    {State QueryIdx : Type*}
    [Fintype State] [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)) where
  domainTag : ByteVector domainLen
  payload : State -> ByteVector 140
  coreAcceptsPayload :
    N32ChallengePoint
      (ByteVector publicKeyLen) (ByteVector messageLen)
      (ByteVector commitmentLen) ->
      N32CanonicalChallenge -> ByteVector 140 -> Bool
  verifierPointFrame :
    State ->
      N32ChallengePointByteFrame domainLen publicKeyLen
        messageLen commitmentLen
  core_verifierPointFrame_matches_output :
    ∀ τ : N32QueryTrace State,
      coreAcceptsPayload
          (E.outputPoint τ.1) (E.outputChallenge τ.1)
          (payload τ.1) = true ->
        Validated48x32ProductBlock.payloadRangeCheck (payload τ.1) = true ->
        verifierPointFrame τ.1 =
          (N32ChallengePointByteCodec.byteVector domainTag).encodePoint
            (E.outputPoint τ.1)
  core_recomputedChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      coreAcceptsPayload
          (E.outputPoint τ.1) (E.outputChallenge τ.1)
          (payload τ.1) = true ->
        Validated48x32ProductBlock.payloadRangeCheck (payload τ.1) = true ->
        E.outputChallenge τ.1 = E.actualChallenge τ

namespace Validated48x32ProductBlockCanonicalBaseVerifierInputs

variable {State QueryIdx : Type*}
variable [Fintype State] [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}

noncomputable def toSingleCarrierVerifierInputs
    (I :
      Validated48x32ProductBlockCanonicalBaseVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E) :
    Validated48x32ProductBlockSingleCarrierVerifierInputs
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen) E where
  domainTag := I.domainTag
  payload := I.payload
  baseAcceptsPayload :=
    Validated48x32ProductBlock.baseAcceptsPayload I.coreAcceptsPayload
  verifierPointFrame := I.verifierPointFrame
  base_verifierPointFrame_matches_output := by
    intro τ hbase
    have hcore :
        I.coreAcceptsPayload
            (E.outputPoint τ.1) (E.outputChallenge τ.1)
            (I.payload τ.1) = true := by
      unfold Validated48x32ProductBlock.baseAcceptsPayload at hbase
      cases hc :
          I.coreAcceptsPayload
            (E.outputPoint τ.1) (E.outputChallenge τ.1)
            (I.payload τ.1) <;> simp [hc] at hbase
      rfl
    have hrange :
        Validated48x32ProductBlock.payloadRangeCheck
          (I.payload τ.1) = true := by
      unfold Validated48x32ProductBlock.baseAcceptsPayload at hbase
      cases hc :
          I.coreAcceptsPayload
            (E.outputPoint τ.1) (E.outputChallenge τ.1)
            (I.payload τ.1) <;> simp [hc] at hbase
      exact hbase
    exact I.core_verifierPointFrame_matches_output τ hcore hrange
  base_recomputedChallenge_matches_actual := by
    intro τ hbase
    have hcore :
        I.coreAcceptsPayload
            (E.outputPoint τ.1) (E.outputChallenge τ.1)
            (I.payload τ.1) = true := by
      unfold Validated48x32ProductBlock.baseAcceptsPayload at hbase
      cases hc :
          I.coreAcceptsPayload
            (E.outputPoint τ.1) (E.outputChallenge τ.1)
            (I.payload τ.1) <;> simp [hc] at hbase
      rfl
    have hrange :
        Validated48x32ProductBlock.payloadRangeCheck
          (I.payload τ.1) = true := by
      unfold Validated48x32ProductBlock.baseAcceptsPayload at hbase
      cases hc :
          I.coreAcceptsPayload
            (E.outputPoint τ.1) (E.outputChallenge τ.1)
            (I.payload τ.1) <;> simp [hc] at hbase
      exact hbase
    exact I.core_recomputedChallenge_matches_actual τ hcore hrange

theorem payloadRangeCheck_of_baseAccepts
    (I :
      Validated48x32ProductBlockCanonicalBaseVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E)
    {point :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)}
    {challenge : N32CanonicalChallenge}
    {payload : ByteVector 140}
    (hbase :
      Validated48x32ProductBlock.baseAcceptsPayload
        I.coreAcceptsPayload point challenge payload = true) :
    Validated48x32ProductBlock.payloadRangeCheck payload = true :=
  by
    unfold Validated48x32ProductBlock.baseAcceptsPayload at hbase
    cases hcore : I.coreAcceptsPayload point challenge payload <;>
      simp [hcore] at hbase
    exact hbase

end Validated48x32ProductBlockCanonicalBaseVerifierInputs

/--
Fiat-Shamir recomputing core verifier for the validated q12289 product-block
route.

This is the implementation-facing shape of `coreAcceptsPayload`: the concrete
verifier performs a scheme-specific payload check and recomputes the canonical
challenge from the fixed byte frame of the public key, message, and
commitments.  The wrapper below conjoins that recomputation check with the
payload check, and the canonical-base layer separately conjoins the 14-bit
payload range check.
-/
structure Validated48x32ProductBlockFSCoreVerifierInputs
    {State QueryIdx : Type*}
    [Fintype State] [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)) where
  domainTag : ByteVector domainLen
  payload : State -> ByteVector 140
  corePayloadCheck :
    N32ChallengePoint
      (ByteVector publicKeyLen) (ByteVector messageLen)
      (ByteVector commitmentLen) ->
      N32CanonicalChallenge -> ByteVector 140 -> Bool
  recomputeChallenge :
    N32ChallengePointByteFrame domainLen publicKeyLen messageLen
      commitmentLen ->
      N32CanonicalChallenge
  recomputedChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      corePayloadCheck
          (E.outputPoint τ.1) (E.outputChallenge τ.1)
          (payload τ.1) = true ->
        Validated48x32ProductBlock.payloadRangeCheck (payload τ.1) = true ->
        recomputeChallenge
            ((N32ChallengePointByteCodec.byteVector domainTag).encodePoint
              (E.outputPoint τ.1)) =
          E.actualChallenge τ

namespace Validated48x32ProductBlockFSCoreVerifierInputs

variable {State QueryIdx : Type*}
variable [Fintype State] [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}

def challengePointFrame
    (I :
      Validated48x32ProductBlockFSCoreVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E)
    (point :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)) :
    N32ChallengePointByteFrame domainLen publicKeyLen messageLen
      commitmentLen :=
  (N32ChallengePointByteCodec.byteVector I.domainTag).encodePoint point

def coreAcceptsPayload
    (I :
      Validated48x32ProductBlockFSCoreVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E)
    (point :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (challenge : N32CanonicalChallenge)
    (payload : ByteVector 140) :
    Bool :=
  I.corePayloadCheck point challenge payload &&
    decide (I.recomputeChallenge (I.challengePointFrame point) = challenge)

theorem corePayloadCheck_of_coreAcceptsPayload
    (I :
      Validated48x32ProductBlockFSCoreVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E)
    {point :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)}
    {challenge : N32CanonicalChallenge}
    {payload : ByteVector 140}
    (hcore : I.coreAcceptsPayload point challenge payload = true) :
    I.corePayloadCheck point challenge payload = true := by
  unfold coreAcceptsPayload at hcore
  cases hcheck : I.corePayloadCheck point challenge payload <;>
    simp [hcheck] at hcore
  rfl

theorem recomputeChallenge_eq_of_coreAcceptsPayload
    (I :
      Validated48x32ProductBlockFSCoreVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E)
    {point :
      N32ChallengePoint
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)}
    {challenge : N32CanonicalChallenge}
    {payload : ByteVector 140}
    (hcore : I.coreAcceptsPayload point challenge payload = true) :
    I.recomputeChallenge (I.challengePointFrame point) = challenge := by
  unfold coreAcceptsPayload at hcore
  cases hcheck : I.corePayloadCheck point challenge payload <;>
    simp [hcheck] at hcore
  exact hcore

noncomputable def toCanonicalBaseVerifierInputs
    (I :
      Validated48x32ProductBlockFSCoreVerifierInputs
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E) :
    Validated48x32ProductBlockCanonicalBaseVerifierInputs
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen) E where
  domainTag := I.domainTag
  payload := I.payload
  coreAcceptsPayload := I.coreAcceptsPayload
  verifierPointFrame := fun state => I.challengePointFrame (E.outputPoint state)
  core_verifierPointFrame_matches_output := by
    intro τ _hcore _hrange
    rfl
  core_recomputedChallenge_matches_actual := by
    intro τ hcore hrange
    have hpayload :
        I.corePayloadCheck
            (E.outputPoint τ.1) (E.outputChallenge τ.1)
            (I.payload τ.1) = true :=
      I.corePayloadCheck_of_coreAcceptsPayload hcore
    have hrecompute_output :
        I.recomputeChallenge (I.challengePointFrame (E.outputPoint τ.1)) =
          E.outputChallenge τ.1 :=
      I.recomputeChallenge_eq_of_coreAcceptsPayload hcore
    have hrecompute_actual :
        I.recomputeChallenge
            ((N32ChallengePointByteCodec.byteVector I.domainTag).encodePoint
              (E.outputPoint τ.1)) =
          E.actualChallenge τ :=
      I.recomputedChallenge_matches_actual τ hpayload hrange
    exact hrecompute_output.symm.trans (by
      simpa [challengePointFrame] using hrecompute_actual)

end Validated48x32ProductBlockFSCoreVerifierInputs

end N32Q12289SystematicRelationLayout

/--
Algebraic soundness interface for an implicit-commitment product-block
verifier.

The generic byte bridge proves that an accepted transcript's reconstructed
commitment bytes equal the output challenge point.  This structure supplies the
semantic decoders and the coherence statement saying that those bytes decode to
the mathematical reconstructed commitment `A*z - chi*u`.  The theorem below
then turns accepted byte transcripts into the product-block verifier equation
used by the full-response extractor.
-/
structure N32ImplicitProductBlockVerifierEquationSoundness
    {State QueryIdx LocalPivot LocalFree : Type*}
    [Fintype State]
    [Fintype LocalFree]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen)) where
  verifier :
    N32ImplicitCommitmentByteVerifierInputs
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E
  Plocal : N32Block -> LocalPivot -> LocalFree -> ZMod q
  decodePublicKey :
    ByteVector publicKeyLen ->
      Sum (N32Block × LocalPivot) (N32Block × LocalFree) -> ZMod q
  decodeCommitment :
    ByteVector commitmentLen ->
      Sum (N32Block × LocalPivot) (N32Block × LocalFree) -> ZMod q
  decodeResponse :
    (N32Repetition -> ByteVector responseLen) ->
      N32Repetition -> N32Block × LocalFree -> ZMod q
  decoded_reconstructCommitment_eq :
    ∀ {point :
          N32ChallengePoint
            (ByteVector publicKeyLen) (ByteVector messageLen)
            (ByteVector commitmentLen)}
        {challenge : N32CanonicalChallenge}
        {response : N32Repetition -> ByteVector responseLen}
        {r : N32Repetition},
      verifier.corePayloadCheck point challenge response = true ->
      verifier.reconstructCommitment point.publicKey challenge response r =
        point.commitments r ->
      decodeCommitment (point.commitments r) =
        SignatureDistribution.productBlockReconstructedCommitment
          Plocal (fun b => (challenge r).scalar (q := q) b)
          (decodePublicKey point.publicKey)
          (decodeResponse response r)

namespace N32ImplicitProductBlockVerifierEquationSoundness

variable {State QueryIdx LocalPivot LocalFree : Type*}
variable [Fintype State]
variable [Fintype LocalFree]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}

theorem reconstructedCommitment_eq_of_acceptsTranscript
    (S :
      N32ImplicitProductBlockVerifierEquationSoundness
        (q := q)
        (State := State)
        (QueryIdx := QueryIdx)
        (LocalPivot := LocalPivot)
        (LocalFree := LocalFree)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    {transcript :
      N32AcceptingTranscript
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen) (ByteVector responseLen)}
    (haccept : S.verifier.acceptsTranscript transcript = true)
    (r : N32Repetition) :
    S.decodeCommitment (transcript.point.commitments r) =
      SignatureDistribution.productBlockReconstructedCommitment
        S.Plocal (fun b => (transcript.challenge r).scalar (q := q) b)
        (S.decodePublicKey transcript.point.publicKey)
        (S.decodeResponse transcript.response r) := by
  have hcore :=
    S.verifier.corePayloadCheck_of_acceptsTranscript
      (transcript := transcript) haccept
  have hcommit :=
    S.verifier.commitments_eq_of_acceptsTranscript
      (transcript := transcript) haccept r
  exact S.decoded_reconstructCommitment_eq hcore hcommit

theorem verifierEquation_of_acceptsTranscript
    (S :
      N32ImplicitProductBlockVerifierEquationSoundness
        (q := q)
        (State := State)
        (QueryIdx := QueryIdx)
        (LocalPivot := LocalPivot)
        (LocalFree := LocalFree)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    {transcript :
      N32AcceptingTranscript
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen) (ByteVector responseLen)}
    (haccept : S.verifier.acceptsTranscript transcript = true)
    (r : N32Repetition)
    {secret mask : N32Block × LocalFree -> ZMod q}
    (hpk :
      S.decodePublicKey transcript.point.publicKey =
        SignatureDistribution.productBlockPublicKey S.Plocal secret)
    (hcommit :
      S.decodeCommitment (transcript.point.commitments r) =
        SignatureDistribution.productBlockCommitment S.Plocal mask) :
    SignatureDistribution.productBlockVerifierEquation
      S.Plocal (fun b => (transcript.challenge r).scalar (q := q) b)
      (SignatureDistribution.productBlockPublicKey S.Plocal secret)
      (SignatureDistribution.productBlockCommitment S.Plocal mask)
      (S.decodeResponse transcript.response r) := by
  have hreconstructed :=
    S.reconstructedCommitment_eq_of_acceptsTranscript haccept r
  have htarget :
      SignatureDistribution.productBlockReconstructedCommitment
        S.Plocal (fun b => (transcript.challenge r).scalar (q := q) b)
        (SignatureDistribution.productBlockPublicKey S.Plocal secret)
        (S.decodeResponse transcript.response r) =
      SignatureDistribution.productBlockCommitment S.Plocal mask := by
    calc
      SignatureDistribution.productBlockReconstructedCommitment
          S.Plocal (fun b => (transcript.challenge r).scalar (q := q) b)
          (SignatureDistribution.productBlockPublicKey S.Plocal secret)
          (S.decodeResponse transcript.response r) =
        SignatureDistribution.productBlockReconstructedCommitment
          S.Plocal (fun b => (transcript.challenge r).scalar (q := q) b)
          (S.decodePublicKey transcript.point.publicKey)
          (S.decodeResponse transcript.response r) := by
            rw [hpk]
      _ = S.decodeCommitment (transcript.point.commitments r) := by
            exact hreconstructed.symm
      _ = SignatureDistribution.productBlockCommitment S.Plocal mask := hcommit
  exact
    SignatureDistribution.productBlockVerifierEquation_of_reconstructedCommitment_eq
      htarget

theorem verifierEquation_of_oneShotAccepts
    (S :
      N32ImplicitProductBlockVerifierEquationSoundness
        (q := q)
        (State := State)
        (QueryIdx := QueryIdx)
        (LocalPivot := LocalPivot)
        (LocalFree := LocalFree)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E)
    {state : State} {challenge : N32CanonicalChallenge}
    (haccept :
      S.verifier.toSerializedVerifier.oneShotAccepts state challenge = true)
    (r : N32Repetition)
    {secret mask : N32Block × LocalFree -> ZMod q}
    (hpk :
      S.decodePublicKey (E.outputPoint state).publicKey =
        SignatureDistribution.productBlockPublicKey S.Plocal secret)
    (hcommit :
      S.decodeCommitment ((E.outputPoint state).commitments r) =
        SignatureDistribution.productBlockCommitment S.Plocal mask) :
    SignatureDistribution.productBlockVerifierEquation
      S.Plocal (fun b => (challenge r).scalar (q := q) b)
      (SignatureDistribution.productBlockPublicKey S.Plocal secret)
      (SignatureDistribution.productBlockCommitment S.Plocal mask)
      (S.decodeResponse (S.verifier.response state) r) := by
  let transcript :
      N32AcceptingTranscript
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen) (ByteVector responseLen) :=
    { point := E.outputPoint state
      challenge := challenge
      response := S.verifier.response state }
  have haccept' : S.verifier.acceptsTranscript transcript = true := by
    simpa [transcript,
      N32ImplicitCommitmentByteVerifierInputs.toSerializedVerifier,
      N32ImplicitCommitmentByteVerifierInputs.toSerializedVerifierInputs,
      N32ByteSerializedVerifierInputs.toSerializedVerifier,
      N32SerializedVerifier.oneShotAccepts,
      N32SerializedVerifier.programmedTranscript] using haccept
  exact S.verifierEquation_of_acceptsTranscript
    (transcript := transcript) haccept' r hpk hcommit

end N32ImplicitProductBlockVerifierEquationSoundness

namespace N32Q12289ImplicitCommitmentByteVerifier

abbrev Block := SharedRowProductBlock12289Rows3.Block
abbrev LocalPivot := SharedRowProductBlock12289Rows3.LocalPivot
abbrev LocalFree := SharedRowProductBlock12289Rows3.LocalFree
abbrev Pivot := SharedRowProductBlock12289Rows3.Pivot
abbrev Free := SharedRowProductBlock12289Rows3.Free
abbrev ViewCoord := SharedRowProductBlock12289Rows3.ViewCoord

def publicKeyPayloadBitOffset : Nat := 32 * 8
def fieldCoordinateCount : Nat := 80
def pivotCoordinateCount : Nat := 48
def freeCoordinateCount : Nat := 32
def responseBitsPerCoordinate : Nat := 9
def signed9NegativeMinRaw : Nat := 512 - responseBound

def publicKeyRangeCheck (publicKey : ByteVector publicKeyBytes) : Bool :=
  zmod12289Packed14RangeCheckAt
    publicKeyPayloadBitOffset fieldCoordinateCount publicKey

def commitmentRangeCheck
    (commitment : ByteVector commitmentBytesPerRepetition) : Bool :=
  zmod12289Packed14RangeCheckAt 0 fieldCoordinateCount commitment

def signed9Raw (response : ByteVector responseBytesPerRepetition)
    (f : Free) : Nat :=
  packedBitsNatLE response
    ((N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex f).1 *
      responseBitsPerCoordinate)
    responseBitsPerCoordinate

def signed9WithinBound (raw : Nat) : Bool :=
  decide (raw <= responseBound ∨ signed9NegativeMinRaw <= raw)

def signed9Coeff (response : ByteVector responseBytesPerRepetition)
    (f : Free) : ZMod 12289 :=
  let raw := signed9Raw response f
  if raw < 256 then
    (raw : ZMod 12289)
  else
    (raw : ZMod 12289) - (512 : ZMod 12289)

def responseRangeCheckRound
    (response : ByteVector responseBytesPerRepetition) : Bool :=
  decide (∀ f : Free, signed9WithinBound (signed9Raw response f) = true)

def responseRangeCheck
    (response : N32Repetition -> ByteVector responseBytesPerRepetition) :
    Bool :=
  decide (∀ r : N32Repetition, responseRangeCheckRound (response r) = true)

def pointRangeCheck
    {messageLen : Nat}
    (point :
      N32ChallengePoint
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition)) :
    Bool :=
  publicKeyRangeCheck point.publicKey &&
    decide (∀ r : N32Repetition,
      commitmentRangeCheck (point.commitments r) = true)

def corePayloadCheck
    {messageLen : Nat}
    (point :
      N32ChallengePoint
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition))
    (_challenge : N32CanonicalChallenge)
    (response : N32Repetition -> ByteVector responseBytesPerRepetition) :
    Bool :=
  pointRangeCheck point && responseRangeCheck response

def publicKeyPivot (publicKey : ByteVector publicKeyBytes)
    (p : Pivot) : ZMod 12289 :=
  zmod12289Packed14Coeff publicKey
    (publicKeyPayloadBitOffset +
      (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.pivotIndex p).1 *
        14)

def publicKeyFree (publicKey : ByteVector publicKeyBytes)
    (f : Free) : ZMod 12289 :=
  zmod12289Packed14Coeff publicKey
    (publicKeyPayloadBitOffset +
      (pivotCoordinateCount +
        (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex f).1) *
        14)

def publicKeyView (publicKey : ByteVector publicKeyBytes) :
    ViewCoord -> ZMod 12289
  | Sum.inl p => publicKeyPivot publicKey p
  | Sum.inr f => publicKeyFree publicKey f

def commitmentPivot
    (commitment : ByteVector commitmentBytesPerRepetition)
    (p : Pivot) : ZMod 12289 :=
  zmod12289Packed14Coeff commitment
    ((N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.pivotIndex p).1 *
      14)

def commitmentFree
    (commitment : ByteVector commitmentBytesPerRepetition)
    (f : Free) : ZMod 12289 :=
  zmod12289Packed14Coeff commitment
    ((pivotCoordinateCount +
      (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex f).1) *
      14)

def commitmentView
    (commitment : ByteVector commitmentBytesPerRepetition) :
    ViewCoord -> ZMod 12289
  | Sum.inl p => commitmentPivot commitment p
  | Sum.inr f => commitmentFree commitment f

def responseView
    (response : N32Repetition -> ByteVector responseBytesPerRepetition)
    (r : N32Repetition) :
    Free -> ZMod 12289 :=
  fun f => signed9Coeff (response r) f

def reconstructedCommitmentView
    (publicKey : ByteVector publicKeyBytes)
    (challenge : N32CanonicalChallenge)
    (response : N32Repetition -> ByteVector responseBytesPerRepetition)
    (r : N32Repetition) :
    ViewCoord -> ZMod 12289 :=
  SignatureDistribution.productBlockReconstructedCommitment
    SharedRowProductBlock12289Rows3.localMatrix
    (fun b => (challenge r).scalar (q := 12289) b)
    (publicKeyView publicKey)
    (responseView response r)

def reconstructedCommitmentPivot
    (publicKey : ByteVector publicKeyBytes)
    (challenge : N32CanonicalChallenge)
    (response : N32Repetition -> ByteVector responseBytesPerRepetition)
    (r : N32Repetition)
    (p : Pivot) :
    ZMod 12289 :=
  -systematicPivotImage 12289
      (SharedRowProductBlock12289Rows3.localMatrix p.1)
      (fun f => responseView response r (p.1, f)) p.2 -
    (challenge r).scalar (q := 12289) p.1 *
      publicKeyPivot publicKey p

def reconstructedCommitmentFree
    (publicKey : ByteVector publicKeyBytes)
    (challenge : N32CanonicalChallenge)
    (response : N32Repetition -> ByteVector responseBytesPerRepetition)
    (r : N32Repetition)
    (f : Free) :
    ZMod 12289 :=
  responseView response r f -
    (challenge r).scalar (q := 12289) f.1 *
      publicKeyFree publicKey f

def commitmentViewCoordinate
    (view : ViewCoord -> ZMod 12289)
    (i : Fin fieldCoordinateCount) :
    ZMod 12289 :=
  if h : i.1 < pivotCoordinateCount then
    let b : Block :=
      ⟨i.1 / 3, by
        have hlt : i.1 < 3 * 16 := by
          have hpivot : i.1 < 48 := by
            simpa [pivotCoordinateCount] using h
          simpa using hpivot
        exact Nat.div_lt_of_lt_mul hlt⟩
    let p : LocalPivot := ⟨i.1 % 3, Nat.mod_lt _ (by decide)⟩
    view (Sum.inl (b, p))
  else
    let j := i.1 - pivotCoordinateCount
    let b : Block :=
      ⟨j / 2, by
        have hi : i.1 < 80 := by
          exact i.2
        have hge : 48 <= i.1 := by
          have hge' : pivotCoordinateCount <= i.1 := Nat.le_of_not_gt h
          simpa [pivotCoordinateCount] using hge'
        have hj : j < 2 * 16 := by
          dsimp [j]
          norm_num [pivotCoordinateCount]
          omega
        exact Nat.div_lt_of_lt_mul hj⟩
    let f : LocalFree := ⟨j % 2, Nat.mod_lt _ (by decide)⟩
    view (Sum.inr (b, f))

theorem commitmentPivotBlock_roundtrip
    (b : Block) (p : LocalPivot)
    (h : (b.1 * 3 + p.1) / 3 < 16) :
    (⟨(b.1 * 3 + p.1) / 3, h⟩ : Block) = b := by
  apply Fin.ext
  fin_cases p
  · change (b.1 * 3 + 0) / 3 = b.1
    simp
  · change (b.1 * 3 + 1) / 3 = b.1
    rw [Nat.mul_comm b.1 3, Nat.mul_add_div (by decide : 3 > 0)]
    rw [Nat.div_eq_of_lt (by norm_num : 1 < 3), Nat.add_zero]
  · change (b.1 * 3 + 2) / 3 = b.1
    rw [Nat.mul_comm b.1 3, Nat.mul_add_div (by decide : 3 > 0)]
    rw [Nat.div_eq_of_lt (by norm_num : 2 < 3), Nat.add_zero]

theorem commitmentPivotLocal_roundtrip
    (b : Block) (p : LocalPivot)
    (h : (b.1 * 3 + p.1) % 3 < 3) :
    (⟨(b.1 * 3 + p.1) % 3, h⟩ : LocalPivot) = p := by
  apply Fin.ext
  fin_cases p <;> simp

theorem commitmentFreeBlock_roundtrip
    (b : Block) (f : LocalFree)
    (h : (b.1 * 2 + f.1) / 2 < 16) :
    (⟨(b.1 * 2 + f.1) / 2, h⟩ : Block) = b := by
  apply Fin.ext
  fin_cases f
  · change (b.1 * 2 + 0) / 2 = b.1
    simp
  · change (b.1 * 2 + 1) / 2 = b.1
    rw [Nat.mul_comm b.1 2, Nat.mul_add_div (by decide : 2 > 0)]
    rw [Nat.div_eq_of_lt (by norm_num : 1 < 2), Nat.add_zero]

theorem commitmentFreeLocal_roundtrip
    (b : Block) (f : LocalFree)
    (h : (b.1 * 2 + f.1) % 2 < 2) :
    (⟨(b.1 * 2 + f.1) % 2, h⟩ : LocalFree) = f := by
  apply Fin.ext
  fin_cases f <;> simp

theorem commitmentViewCoordinate_pivot
    (view : ViewCoord -> ZMod 12289)
    (p : Pivot)
    (h : (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.pivotIndex p).1 <
      fieldCoordinateCount) :
    commitmentViewCoordinate view
        ⟨(N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.pivotIndex p).1,
          h⟩ =
      view (Sum.inl p) := by
  rcases p with ⟨b, p⟩
  have hpivot : b.1 * 3 + p.1 < pivotCoordinateCount := by
    have hb := b.2
    have hp := p.2
    norm_num [pivotCoordinateCount]
    omega
  simp [commitmentViewCoordinate,
    N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.pivotIndex,
    hpivot, commitmentPivotBlock_roundtrip]
  congr
  exact Nat.mod_eq_of_lt p.2

theorem commitmentViewCoordinate_free
    (view : ViewCoord -> ZMod 12289)
    (f : Free)
    (h :
      pivotCoordinateCount +
          (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex f).1 <
        fieldCoordinateCount) :
    commitmentViewCoordinate view
        ⟨pivotCoordinateCount +
            (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex f).1,
          h⟩ =
      view (Sum.inr f) := by
  rcases f with ⟨b, f⟩
  have hnot :
      ¬ pivotCoordinateCount + (b.1 * 2 + f.1) < pivotCoordinateCount := by
    omega
  simp [commitmentViewCoordinate,
    N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex,
    pivotCoordinateCount, commitmentFreeBlock_roundtrip]
  congr
  exact Nat.mod_eq_of_lt f.2

def commitmentPackedBit
    (view : ViewCoord -> ZMod 12289) (bit : Nat) :
    Bool :=
  if h : bit / 14 < fieldCoordinateCount then
    (commitmentViewCoordinate view ⟨bit / 14, h⟩).val.testBit (bit % 14)
  else
    false

def commitmentPackedByteNat
    (view : ViewCoord -> ZMod 12289) (byteIdx : Nat) :
    Nat :=
  Nat.ofBits (fun k : Fin 8 =>
    commitmentPackedBit view (byteIdx * 8 + k.1))

theorem commitmentPackedByteNat_lt
    (view : ViewCoord -> ZMod 12289) (byteIdx : Nat) :
    commitmentPackedByteNat view byteIdx < 256 := by
  unfold commitmentPackedByteNat
  have hlt :=
    Nat.ofBits_lt_two_pow (fun k : Fin 8 =>
      commitmentPackedBit view (byteIdx * 8 + k.1))
  norm_num at hlt ⊢
  exact hlt

def commitmentPackedByte
    (view : ViewCoord -> ZMod 12289)
    (i : Fin commitmentBytesPerRepetition) :
    Byte :=
  ⟨commitmentPackedByteNat view i.1, commitmentPackedByteNat_lt view i.1⟩

def encodeCommitmentView
    (view : ViewCoord -> ZMod 12289) :
    ByteVector commitmentBytesPerRepetition :=
  commitmentPackedByte view

theorem byteVectorBitLE_encodeCommitmentView
    (view : ViewCoord -> ZMod 12289)
    {bit : Nat}
    (hbit : bit < fieldCoordinateCount * 14) :
    byteVectorBitLE (encodeCommitmentView view) bit =
      (commitmentPackedBit view bit).toNat := by
  unfold byteVectorBitLE encodeCommitmentView commitmentPackedByte
    commitmentPackedByteNat
  have hbyte : bit / 8 < commitmentBytesPerRepetition := by
    apply Nat.div_lt_of_lt_mul
    have hlt : bit < 8 * commitmentBytesPerRepetition := by
      norm_num [fieldCoordinateCount, commitmentBytesPerRepetition] at hbit ⊢
      exact hbit
    exact hlt
  simp [hbyte]
  rw [nat_div_pow_mod_two_eq_testBit_toNat]
  rw [Nat.testBit_ofBits_lt _ _ (Nat.mod_lt _ (by decide : 0 < 8))]
  have hidx : bit / 8 * 8 + bit % 8 = bit := by
    rw [Nat.mul_comm (bit / 8) 8]
    exact Nat.div_add_mod bit 8
  simp [hidx]

theorem zmod12289Packed14Coeff_encodeCommitmentView
    (view : ViewCoord -> ZMod 12289)
    (i : Fin fieldCoordinateCount) :
    zmod12289Packed14Coeff (encodeCommitmentView view) (i.1 * 14) =
      commitmentViewCoordinate view i := by
  unfold zmod12289Packed14Coeff zmod12289PackedCoeff
  have hbits :
      packedBitsNatLE (encodeCommitmentView view) (i.1 * 14) 14 =
        (commitmentViewCoordinate view i).val := by
    unfold packedBitsNatLE
    calc
      (∑ k : Fin 14,
          byteVectorBitLE (encodeCommitmentView view) (i.1 * 14 + k.1) *
            2 ^ k.1) =
          ∑ k : Fin 14,
            ((commitmentViewCoordinate view i).val.testBit k.1).toNat *
              2 ^ k.1 := by
            apply Finset.sum_congr rfl
            intro k _hk
            rw [byteVectorBitLE_encodeCommitmentView]
            · unfold commitmentPackedBit
              have hcoord : (i.1 * 14 + k.1) / 14 = i.1 := by
                rw [Nat.mul_comm i.1 14, Nat.mul_add_div (by decide : 14 > 0)]
                rw [Nat.div_eq_of_lt k.2, Nat.add_zero]
              have hmod : (i.1 * 14 + k.1) % 14 = k.1 := by
                rw [Nat.mul_comm i.1 14, Nat.mul_add_mod]
                exact Nat.mod_eq_of_lt k.2
              simp [hcoord, hmod]
            · have hi := i.2
              have hklt := k.2
              omega
      _ = Nat.ofBits (fun k : Fin 14 =>
            (commitmentViewCoordinate view i).val.testBit k.1) := by
            rw [sum_bool_bits_eq_ofBits]
      _ = (commitmentViewCoordinate view i).val := by
            rw [Nat.ofBits_testBit]
            apply Nat.mod_eq_of_lt
            exact Nat.lt_trans
              (ZMod.val_lt (commitmentViewCoordinate view i))
              (by norm_num)
  rw [hbits]
  simp

theorem commitmentView_encodeCommitmentView
    (view : ViewCoord -> ZMod 12289) :
    commitmentView (encodeCommitmentView view) = view := by
  funext coord
  cases coord with
  | inl p =>
      have hpivot :
          (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.pivotIndex p).1 <
            fieldCoordinateCount := by
        have h :=
          (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.pivotIndex p).2
        change
          (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.pivotIndex p).1 <
            48 at h
        change
          (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.pivotIndex p).1 <
            80
        omega
      change commitmentPivot (encodeCommitmentView view) p = view (Sum.inl p)
      unfold commitmentPivot
      exact
        (zmod12289Packed14Coeff_encodeCommitmentView view
          ⟨(N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.pivotIndex p).1,
            hpivot⟩).trans
          (commitmentViewCoordinate_pivot view p hpivot)
  | inr f =>
      have hfree :
          pivotCoordinateCount +
              (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex f).1 <
            fieldCoordinateCount := by
        have h :=
          (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex f).2
        change
          (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex f).1 <
            32 at h
        change
          48 +
              (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex f).1 <
            80
        omega
      change commitmentFree (encodeCommitmentView view) f = view (Sum.inr f)
      unfold commitmentFree
      exact
        (zmod12289Packed14Coeff_encodeCommitmentView view
          ⟨pivotCoordinateCount +
              (N32Q12289SystematicRelationLayout.Validated48x32ProductBlock.freeIndex f).1,
            hfree⟩).trans
          (commitmentViewCoordinate_free view f hfree)

/--
Byte encoder for one q12289 implicit commitment view.

This isolates the remaining low-level packing obligation: a concrete q14
encoder only has to prove that decoding its bytes with `commitmentView` recovers
the requested 48 pivot and 32 free coordinates.  Once this is available, the
production reconstruction function can be defined by encoding the already
formalized algebraic value `A*z - chi*u`.
-/
structure CommitmentViewEncoder where
  encode : (ViewCoord -> ZMod 12289) -> ByteVector commitmentBytesPerRepetition
  decode_encode :
    ∀ view, commitmentView (encode view) = view

namespace CommitmentViewEncoder

def reconstructCommitment
    (Enc : CommitmentViewEncoder)
    (publicKey : ByteVector publicKeyBytes)
    (challenge : N32CanonicalChallenge)
    (response : N32Repetition -> ByteVector responseBytesPerRepetition)
    (r : N32Repetition) :
    ByteVector commitmentBytesPerRepetition :=
  Enc.encode (reconstructedCommitmentView publicKey challenge response r)

theorem reconstructCommitment_correct
    (Enc : CommitmentViewEncoder)
    {messageLen : Nat}
    {point :
      N32ChallengePoint
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition)}
    {challenge : N32CanonicalChallenge}
    {response : N32Repetition -> ByteVector responseBytesPerRepetition}
    {r : N32Repetition}
    (_hcore : corePayloadCheck point challenge response = true) :
    commitmentView
        (Enc.reconstructCommitment point.publicKey challenge response r) =
      reconstructedCommitmentView point.publicKey challenge response r :=
  Enc.decode_encode _

end CommitmentViewEncoder

def commitmentViewEncoder : CommitmentViewEncoder where
  encode := encodeCommitmentView
  decode_encode := commitmentView_encodeCommitmentView

abbrev ProductBlockEquationSoundness
    {State QueryIdx : Type*}
    [Fintype State]
    {messageLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition)) :=
  N32ImplicitProductBlockVerifierEquationSoundness
    (q := 12289)
    (State := State)
    (QueryIdx := QueryIdx)
    (LocalPivot := LocalPivot)
    (LocalFree := LocalFree)
    (domainLen := domainBytes)
    (publicKeyLen := publicKeyBytes)
    (messageLen := messageLen)
    (commitmentLen := commitmentBytesPerRepetition)
    (responseLen := responseBytesPerRepetition) E

def productBlockEquationSoundnessFromReconstruction
    {State QueryIdx : Type*}
    [Fintype State]
    {messageLen : Nat}
    {E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition)}
    (verifier : Inputs E)
    (hcore_eq :
      ∀ point challenge response,
        verifier.corePayloadCheck point challenge response =
          corePayloadCheck point challenge response)
    (hreconstruct :
      ∀ {point :
            N32ChallengePoint
              (ByteVector publicKeyBytes) (ByteVector messageLen)
              (ByteVector commitmentBytesPerRepetition)}
          {challenge : N32CanonicalChallenge}
          {response : N32Repetition -> ByteVector responseBytesPerRepetition}
          {r : N32Repetition},
        corePayloadCheck point challenge response = true ->
        commitmentView
            (verifier.reconstructCommitment point.publicKey challenge
              response r) =
          reconstructedCommitmentView point.publicKey challenge response r) :
    ProductBlockEquationSoundness E where
  verifier := verifier
  Plocal := SharedRowProductBlock12289Rows3.localMatrix
  decodePublicKey := publicKeyView
  decodeCommitment := commitmentView
  decodeResponse := responseView
  decoded_reconstructCommitment_eq := by
    intro point challenge response r hcoreVerifier hcommit
    have hcore : corePayloadCheck point challenge response = true := by
      simpa [hcore_eq point challenge response] using hcoreVerifier
    rw [← hcommit]
    exact hreconstruct hcore

def productBlockEquationSoundness
    {State QueryIdx : Type*}
    [Fintype State]
    {messageLen : Nat}
    {E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition)}
    (verifier : Inputs E)
    (hcore_eq :
      ∀ point challenge response,
        verifier.corePayloadCheck point challenge response =
          corePayloadCheck point challenge response)
    (hdecoded :
      ∀ {point :
            N32ChallengePoint
              (ByteVector publicKeyBytes) (ByteVector messageLen)
              (ByteVector commitmentBytesPerRepetition)}
          {challenge : N32CanonicalChallenge}
          {response : N32Repetition -> ByteVector responseBytesPerRepetition}
          {r : N32Repetition},
        corePayloadCheck point challenge response = true ->
        verifier.reconstructCommitment point.publicKey challenge response r =
          point.commitments r ->
        commitmentView (point.commitments r) =
          reconstructedCommitmentView point.publicKey challenge response r) :
    ProductBlockEquationSoundness E where
  verifier := verifier
  Plocal := SharedRowProductBlock12289Rows3.localMatrix
  decodePublicKey := publicKeyView
  decodeCommitment := commitmentView
  decodeResponse := responseView
  decoded_reconstructCommitment_eq := by
    intro point challenge response r hcoreVerifier hcommit
    have hcore : corePayloadCheck point challenge response = true := by
      simpa [hcore_eq point challenge response] using hcoreVerifier
    exact hdecoded hcore hcommit

def implicitCommitmentDomainTag : ByteVector domainBytes :=
  fun i =>
    ⟨match i.1 with
      | 0 => 68 | 1 => 101 | 2 => 115 | 3 => 99 | 4 => 101
      | 5 => 110 | 6 => 100 | 7 => 101 | 8 => 114 | 9 => 46
      | 10 => 83 | 11 => 105 | 12 => 103 | 13 => 110 | 14 => 97
      | 15 => 116 | 16 => 117 | 17 => 114 | 18 => 101 | 19 => 46
      | 20 => 113 | 21 => 49 | 22 => 50 | 23 => 50 | 24 => 56
      | 25 => 57 | 26 => 46 | 27 => 105 | 28 => 109 | 29 => 112
      | 30 => 108 | 31 => 105 | 32 => 99 | 33 => 105 | 34 => 116
      | 35 => 95 | 36 => 99 | 37 => 111 | 38 => 109 | 39 => 109
      | 40 => 105 | 41 => 116 | 42 => 109 | 43 => 101 | 44 => 110
      | 45 => 116 | 46 => 46 | 47 => 118 | 48 => 49
      | _ => 0,
      by
        fin_cases i <;> norm_num⟩

theorem implicitCommitmentDomainTag_len :
    domainBytes = 49 := by
  rfl

abbrev ByteString := List Byte

def byteStringOfVector {n : Nat} (bytes : ByteVector n) : ByteString :=
  List.ofFn bytes

@[simp] theorem byteStringOfVector_length {n : Nat}
    (bytes : ByteVector n) :
    (byteStringOfVector bytes).length = n := by
  simp [byteStringOfVector]

def natLEByte (value byteIdx : Nat) : Byte :=
  ⟨(value / 2 ^ (8 * byteIdx)) % 256, Nat.mod_lt _ (by decide)⟩

def natLEBytes (width value : Nat) : ByteString :=
  List.ofFn (fun i : Fin width => natLEByte value i.1)

@[simp] theorem natLEBytes_length (width value : Nat) :
    (natLEBytes width value).length = width := by
  simp [natLEBytes]

def u16LE (value : Nat) : ByteString :=
  natLEBytes 2 value

def u64LE (value : Nat) : ByteString :=
  natLEBytes 8 value

@[simp] theorem u16LE_length (value : Nat) :
    (u16LE value).length = 2 := by
  simp [u16LE]

@[simp] theorem u64LE_length (value : Nat) :
    (u64LE value).length = 8 := by
  simp [u64LE]

def hashFrameField (label data : ByteString) : ByteString :=
  u16LE label.length ++ label ++ u64LE data.length ++ data

@[simp] theorem hashFrameField_length (label data : ByteString) :
    (hashFrameField label data).length =
      10 + label.length + data.length := by
  simp [hashFrameField]
  omega

def hashFrameU64 (label : ByteString) (value : Nat) : ByteString :=
  hashFrameField label (u64LE value)

@[simp] theorem hashFrameU64_length (label : ByteString) (value : Nat) :
    (hashFrameU64 label value).length = 18 + label.length := by
  simp [hashFrameU64]
  omega

def hashLabelDomain : ByteString := [100, 111, 109, 97, 105, 110]
def hashLabelProfile : ByteString := [112, 114, 111, 102, 105, 108, 101]
def hashLabelQ : ByteString := [113]
def hashLabelGroups : ByteString := [103, 114, 111, 117, 112, 115]
def hashLabelLocalPivot : ByteString :=
  [108, 111, 99, 97, 108, 95, 112, 105, 118, 111, 116]
def hashLabelLocalFree : ByteString :=
  [108, 111, 99, 97, 108, 95, 102, 114, 101, 101]
def hashLabelRepetitions : ByteString :=
  [114, 101, 112, 101, 116, 105, 116, 105, 111, 110, 115]
def hashLabelResponseBound : ByteString :=
  [114, 101, 115, 112, 111, 110, 115, 101, 95, 98, 111, 117, 110, 100]
def hashLabelPublicKey : ByteString :=
  [112, 117, 98, 108, 105, 99, 95, 107, 101, 121]
def hashLabelMessage : ByteString := [109, 101, 115, 115, 97, 103, 101]
def hashLabelReconstructedCommitments : ByteString :=
  [114, 101, 99, 111, 110, 115, 116, 114, 117, 99, 116, 101, 100,
    95, 99, 111, 109, 109, 105, 116, 109, 101, 110, 116, 115]

def hashProfileBytes : ByteString :=
  [118, 97, 108, 105, 100, 97, 116, 101, 100, 45, 113, 49, 50, 50,
    56, 57, 45, 110, 52, 56, 45, 109, 49, 45, 51, 50, 45, 115,
    105, 110, 103, 108, 101, 45, 99, 97, 114, 114, 105, 101, 114]

def flattenedCommitmentBytes
    (commitments : N32Repetition -> ByteVector commitmentBytesPerRepetition) :
    ByteString :=
  byteStringOfVector
    (fun i : Fin (7 * commitmentBytesPerRepetition) =>
      commitments
        ⟨i.1 / commitmentBytesPerRepetition, by
          apply Nat.div_lt_of_lt_mul
          exact i.2⟩
        ⟨i.1 % commitmentBytesPerRepetition,
          Nat.mod_lt _ (by decide)⟩)

@[simp] theorem flattenedCommitmentBytes_length
    (commitments : N32Repetition -> ByteVector commitmentBytesPerRepetition) :
    (flattenedCommitmentBytes commitments).length =
      7 * commitmentBytesPerRepetition := by
  simp [flattenedCommitmentBytes]

def q12289HashFrame
    {messageLen : Nat}
    (frame :
      N32ChallengePointByteFrame domainBytes publicKeyBytes messageLen
        commitmentBytesPerRepetition) :
    ByteString :=
  hashFrameField hashLabelDomain (byteStringOfVector frame.domainTag) ++
  hashFrameField hashLabelProfile hashProfileBytes ++
  hashFrameU64 hashLabelQ 12289 ++
  hashFrameU64 hashLabelGroups 16 ++
  hashFrameU64 hashLabelLocalPivot 3 ++
  hashFrameU64 hashLabelLocalFree 2 ++
  hashFrameU64 hashLabelRepetitions 7 ++
  hashFrameU64 hashLabelResponseBound responseBound ++
  hashFrameField hashLabelPublicKey (byteStringOfVector frame.publicKey) ++
  hashFrameField hashLabelMessage (byteStringOfVector frame.message) ++
  hashFrameField hashLabelReconstructedCommitments
    (flattenedCommitmentBytes frame.commitments)

theorem q12289HashFrame_length
    {messageLen : Nat}
    (frame :
      N32ChallengePointByteFrame domainBytes publicKeyBytes messageLen
        commitmentBytesPerRepetition) :
    (q12289HashFrame frame).length = 1508 + messageLen := by
  simp [q12289HashFrame,
    hashLabelDomain, hashLabelProfile, hashLabelQ, hashLabelGroups,
    hashLabelLocalPivot, hashLabelLocalFree, hashLabelRepetitions,
    hashLabelResponseBound, hashLabelPublicKey, hashLabelMessage,
    hashLabelReconstructedCommitments, hashProfileBytes, domainBytes,
    publicKeyBytes, commitmentBytesPerRepetition, responseBound,
    Nat.add_assoc, Nat.add_comm, Nat.add_left_comm]

theorem q12289HashFrame_fixtureMessage_length
    (frame :
      N32ChallengePointByteFrame domainBytes publicKeyBytes 32
        commitmentBytesPerRepetition) :
    (q12289HashFrame frame).length = 1540 := by
  simpa using q12289HashFrame_length frame

def hashFrameRecomputeChallenge
    (hashChallenge : ByteString -> N32CanonicalChallenge)
    {messageLen : Nat}
    (frame :
      N32ChallengePointByteFrame domainBytes publicKeyBytes messageLen
        commitmentBytesPerRepetition) :
    N32CanonicalChallenge :=
  hashChallenge (q12289HashFrame frame)

structure ConcreteVerifierCertificate
    {State QueryIdx : Type*}
    [Fintype State]
    {messageLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition)) where
  response : State -> N32Repetition -> ByteVector responseBytesPerRepetition
  reconstructCommitment :
    ByteVector publicKeyBytes ->
      N32CanonicalChallenge ->
      (N32Repetition -> ByteVector responseBytesPerRepetition) ->
      N32Repetition -> ByteVector commitmentBytesPerRepetition
  recomputeChallenge :
    N32ChallengePointByteFrame domainBytes publicKeyBytes messageLen
      commitmentBytesPerRepetition ->
      N32CanonicalChallenge
  oracleChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      corePayloadCheck
          (E.outputPoint τ.1) (E.outputChallenge τ.1)
          (response τ.1) = true ->
        (∀ r,
          reconstructCommitment
              (E.outputPoint τ.1).publicKey
              (E.outputChallenge τ.1) (response τ.1) r =
            (E.outputPoint τ.1).commitments r) ->
        recomputeChallenge
            ((N32ChallengePointByteCodec.byteVector
                implicitCommitmentDomainTag).encodePoint
              (E.outputPoint τ.1)) =
          E.actualChallenge τ
  reconstructCommitment_correct :
    ∀ {point :
          N32ChallengePoint
            (ByteVector publicKeyBytes) (ByteVector messageLen)
            (ByteVector commitmentBytesPerRepetition)}
        {challenge : N32CanonicalChallenge}
        {response : N32Repetition -> ByteVector responseBytesPerRepetition}
        {r : N32Repetition},
      corePayloadCheck point challenge response = true ->
      commitmentView
          (reconstructCommitment point.publicKey challenge response r) =
        reconstructedCommitmentView point.publicKey challenge response r

namespace ConcreteVerifierCertificate

variable {State QueryIdx : Type*}
variable [Fintype State]
variable {messageLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyBytes) (ByteVector messageLen)
    (ByteVector commitmentBytesPerRepetition)}

def toVerifierInputs
    (C : ConcreteVerifierCertificate E) :
    Inputs E where
  domainTag := implicitCommitmentDomainTag
  response := C.response
  corePayloadCheck :=
    N32Q12289ImplicitCommitmentByteVerifier.corePayloadCheck
  reconstructCommitment := C.reconstructCommitment
  recomputeChallenge := C.recomputeChallenge
  oracleChallenge_matches_actual := C.oracleChallenge_matches_actual

def toProductBlockEquationSoundness
    (C : ConcreteVerifierCertificate E) :
    ProductBlockEquationSoundness E where
  verifier := C.toVerifierInputs
  Plocal := SharedRowProductBlock12289Rows3.localMatrix
  decodePublicKey := publicKeyView
  decodeCommitment := commitmentView
  decodeResponse := responseView
  decoded_reconstructCommitment_eq := by
    intro point challenge response r hcoreVerifier hcommit
    have hcore :
        corePayloadCheck point challenge response = true := by
      simpa [toVerifierInputs] using hcoreVerifier
    rw [← hcommit]
    exact C.reconstructCommitment_correct hcore

theorem verifierEquation_of_oneShotAccepts
    (C : ConcreteVerifierCertificate E)
    {state : State} {challenge : N32CanonicalChallenge}
    (haccept :
      C.toVerifierInputs.toSerializedVerifier.oneShotAccepts state challenge =
        true)
    (r : N32Repetition)
    {secret mask : N32Block × LocalFree -> ZMod 12289}
    (hpk :
      publicKeyView (E.outputPoint state).publicKey =
        SignatureDistribution.productBlockPublicKey
          SharedRowProductBlock12289Rows3.localMatrix secret)
    (hcommit :
      commitmentView ((E.outputPoint state).commitments r) =
        SignatureDistribution.productBlockCommitment
          SharedRowProductBlock12289Rows3.localMatrix mask) :
    SignatureDistribution.productBlockVerifierEquation
      SharedRowProductBlock12289Rows3.localMatrix
      (fun b => (challenge r).scalar (q := 12289) b)
      (SignatureDistribution.productBlockPublicKey
        SharedRowProductBlock12289Rows3.localMatrix secret)
      (SignatureDistribution.productBlockCommitment
        SharedRowProductBlock12289Rows3.localMatrix mask)
      (responseView (C.response state) r) := by
  exact
    N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_oneShotAccepts
      C.toProductBlockEquationSoundness
      haccept r hpk hcommit

end ConcreteVerifierCertificate

/--
Concrete verifier certificate once the commitment-byte encoder is fixed.

Compared with `ConcreteVerifierCertificate`, this shape removes the algebraic
reconstruction field from the implementation-facing API.  The verifier's
reconstructed commitment bytes are defined to be the encoder applied to the
formal q12289 view `A*z - chi*u`; the encoder's `decode_encode` theorem supplies
`reconstructCommitment_correct`.  The only remaining semantic external field is
the hash/random-oracle agreement for accepted transcripts.
-/
structure EncodedReconstructionVerifierCertificate
    {State QueryIdx : Type*}
    [Fintype State]
    {messageLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition))
    (Enc : CommitmentViewEncoder) where
  response : State -> N32Repetition -> ByteVector responseBytesPerRepetition
  recomputeChallenge :
    N32ChallengePointByteFrame domainBytes publicKeyBytes messageLen
      commitmentBytesPerRepetition ->
      N32CanonicalChallenge
  oracleChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      corePayloadCheck
          (E.outputPoint τ.1) (E.outputChallenge τ.1)
          (response τ.1) = true ->
        (∀ r,
          Enc.reconstructCommitment
              (E.outputPoint τ.1).publicKey
              (E.outputChallenge τ.1) (response τ.1) r =
            (E.outputPoint τ.1).commitments r) ->
        recomputeChallenge
            ((N32ChallengePointByteCodec.byteVector
                implicitCommitmentDomainTag).encodePoint
              (E.outputPoint τ.1)) =
          E.actualChallenge τ

namespace EncodedReconstructionVerifierCertificate

variable {State QueryIdx : Type*}
variable [Fintype State]
variable {messageLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyBytes) (ByteVector messageLen)
    (ByteVector commitmentBytesPerRepetition)}
variable {Enc : CommitmentViewEncoder}

def toConcreteVerifierCertificate
    (C : EncodedReconstructionVerifierCertificate E Enc) :
    ConcreteVerifierCertificate E where
  response := C.response
  reconstructCommitment := Enc.reconstructCommitment
  recomputeChallenge := C.recomputeChallenge
  oracleChallenge_matches_actual := C.oracleChallenge_matches_actual
  reconstructCommitment_correct := Enc.reconstructCommitment_correct

def toVerifierInputs
    (C : EncodedReconstructionVerifierCertificate E Enc) :
    Inputs E :=
  C.toConcreteVerifierCertificate.toVerifierInputs

def toProductBlockEquationSoundness
    (C : EncodedReconstructionVerifierCertificate E Enc) :
    ProductBlockEquationSoundness E :=
  C.toConcreteVerifierCertificate.toProductBlockEquationSoundness

theorem verifierEquation_of_oneShotAccepts
    (C : EncodedReconstructionVerifierCertificate E Enc)
    {state : State} {challenge : N32CanonicalChallenge}
    (haccept :
      C.toVerifierInputs.toSerializedVerifier.oneShotAccepts state challenge =
        true)
    (r : N32Repetition)
    {secret mask : N32Block × LocalFree -> ZMod 12289}
    (hpk :
      publicKeyView (E.outputPoint state).publicKey =
        SignatureDistribution.productBlockPublicKey
          SharedRowProductBlock12289Rows3.localMatrix secret)
    (hcommit :
      commitmentView ((E.outputPoint state).commitments r) =
        SignatureDistribution.productBlockCommitment
          SharedRowProductBlock12289Rows3.localMatrix mask) :
    SignatureDistribution.productBlockVerifierEquation
      SharedRowProductBlock12289Rows3.localMatrix
      (fun b => (challenge r).scalar (q := 12289) b)
      (SignatureDistribution.productBlockPublicKey
        SharedRowProductBlock12289Rows3.localMatrix secret)
      (SignatureDistribution.productBlockCommitment
        SharedRowProductBlock12289Rows3.localMatrix mask)
      (responseView (C.response state) r) := by
  exact
    C.toConcreteVerifierCertificate.verifierEquation_of_oneShotAccepts
      haccept r hpk hcommit

end EncodedReconstructionVerifierCertificate

/--
Encoded-reconstruction certificate with the concrete q12289 hash frame exposed.

This is the tight implementation boundary for the Rust verifier's Fiat-Shamir
step.  The reconstructed commitment bytes are still supplied by `Enc`, but the
challenge recomputation is no longer an arbitrary function on semantic frames:
it is a hash-to-challenge function applied to the exact length-prefixed byte
string `q12289HashFrame`.
-/
structure HashFrameEncodedReconstructionVerifierCertificate
    {State QueryIdx : Type*}
    [Fintype State]
    {messageLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition))
    (Enc : CommitmentViewEncoder) where
  response : State -> N32Repetition -> ByteVector responseBytesPerRepetition
  hashChallenge : ByteString -> N32CanonicalChallenge
  hashFrameChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      corePayloadCheck
          (E.outputPoint τ.1) (E.outputChallenge τ.1)
          (response τ.1) = true ->
        (∀ r,
          Enc.reconstructCommitment
              (E.outputPoint τ.1).publicKey
              (E.outputChallenge τ.1) (response τ.1) r =
            (E.outputPoint τ.1).commitments r) ->
        hashChallenge
            (q12289HashFrame
              ((N32ChallengePointByteCodec.byteVector
                  implicitCommitmentDomainTag).encodePoint
                (E.outputPoint τ.1))) =
          E.actualChallenge τ

namespace HashFrameEncodedReconstructionVerifierCertificate

variable {State QueryIdx : Type*}
variable [Fintype State]
variable {messageLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyBytes) (ByteVector messageLen)
    (ByteVector commitmentBytesPerRepetition)}
variable {Enc : CommitmentViewEncoder}

def recomputeChallenge
    (C : HashFrameEncodedReconstructionVerifierCertificate E Enc) :
    N32ChallengePointByteFrame domainBytes publicKeyBytes messageLen
      commitmentBytesPerRepetition ->
      N32CanonicalChallenge :=
  hashFrameRecomputeChallenge C.hashChallenge

def toEncodedReconstructionVerifierCertificate
    (C : HashFrameEncodedReconstructionVerifierCertificate E Enc) :
    EncodedReconstructionVerifierCertificate E Enc where
  response := C.response
  recomputeChallenge := C.recomputeChallenge
  oracleChallenge_matches_actual := by
    intro τ hcore hcommit
    exact C.hashFrameChallenge_matches_actual τ hcore hcommit

def toVerifierInputs
    (C : HashFrameEncodedReconstructionVerifierCertificate E Enc) :
    Inputs E :=
  C.toEncodedReconstructionVerifierCertificate.toVerifierInputs

def toProductBlockEquationSoundness
    (C : HashFrameEncodedReconstructionVerifierCertificate E Enc) :
    ProductBlockEquationSoundness E :=
  C.toEncodedReconstructionVerifierCertificate.toProductBlockEquationSoundness

theorem verifierEquation_of_oneShotAccepts
    (C : HashFrameEncodedReconstructionVerifierCertificate E Enc)
    {state : State} {challenge : N32CanonicalChallenge}
    (haccept :
      C.toVerifierInputs.toSerializedVerifier.oneShotAccepts state challenge =
        true)
    (r : N32Repetition)
    {secret mask : N32Block × LocalFree -> ZMod 12289}
    (hpk :
      publicKeyView (E.outputPoint state).publicKey =
        SignatureDistribution.productBlockPublicKey
          SharedRowProductBlock12289Rows3.localMatrix secret)
    (hcommit :
      commitmentView ((E.outputPoint state).commitments r) =
        SignatureDistribution.productBlockCommitment
          SharedRowProductBlock12289Rows3.localMatrix mask) :
    SignatureDistribution.productBlockVerifierEquation
      SharedRowProductBlock12289Rows3.localMatrix
      (fun b => (challenge r).scalar (q := 12289) b)
      (SignatureDistribution.productBlockPublicKey
        SharedRowProductBlock12289Rows3.localMatrix secret)
      (SignatureDistribution.productBlockCommitment
        SharedRowProductBlock12289Rows3.localMatrix mask)
      (responseView (C.response state) r) := by
  exact
    C.toEncodedReconstructionVerifierCertificate.verifierEquation_of_oneShotAccepts
      haccept r hpk hcommit

end HashFrameEncodedReconstructionVerifierCertificate

abbrev ProductionHashFrameVerifierCertificate
    {State QueryIdx : Type*}
    [Fintype State]
    {messageLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition)) :=
  HashFrameEncodedReconstructionVerifierCertificate E commitmentViewEncoder

/--
External random-oracle/SHA agreement for the production verifier.

This is the only remaining cryptographic-model assumption at this boundary.
All byte layout choices are fixed by `q12289HashFrame`, and all reconstruction
bytes are fixed by `commitmentViewEncoder`; this record only says that the
production hash-to-challenge function agrees with the query-log experiment on
accepted transcripts whose reconstructed commitments are the output point.
-/
structure ProductionHashFrameOracleAgreement
    {State QueryIdx : Type*}
    [Fintype State]
    {messageLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyBytes) (ByteVector messageLen)
        (ByteVector commitmentBytesPerRepetition)) where
  response : State -> N32Repetition -> ByteVector responseBytesPerRepetition
  hashChallenge : ByteString -> N32CanonicalChallenge
  hashFrameChallenge_matches_actual :
    ∀ τ : N32QueryTrace State,
      corePayloadCheck
          (E.outputPoint τ.1) (E.outputChallenge τ.1)
          (response τ.1) = true ->
        (∀ r,
          commitmentViewEncoder.reconstructCommitment
              (E.outputPoint τ.1).publicKey
              (E.outputChallenge τ.1) (response τ.1) r =
            (E.outputPoint τ.1).commitments r) ->
        hashChallenge
            (q12289HashFrame
              ((N32ChallengePointByteCodec.byteVector
                  implicitCommitmentDomainTag).encodePoint
                (E.outputPoint τ.1))) =
          E.actualChallenge τ

namespace ProductionHashFrameOracleAgreement

variable {State QueryIdx : Type*}
variable [Fintype State]
variable {messageLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyBytes) (ByteVector messageLen)
    (ByteVector commitmentBytesPerRepetition)}

def toProductionHashFrameVerifierCertificate
    (A : ProductionHashFrameOracleAgreement E) :
    ProductionHashFrameVerifierCertificate E where
  response := A.response
  hashChallenge := A.hashChallenge
  hashFrameChallenge_matches_actual :=
    A.hashFrameChallenge_matches_actual

def toVerifierInputs
    (A : ProductionHashFrameOracleAgreement E) :
    Inputs E :=
  A.toProductionHashFrameVerifierCertificate.toVerifierInputs

def toProductBlockEquationSoundness
    (A : ProductionHashFrameOracleAgreement E) :
    ProductBlockEquationSoundness E :=
  A.toProductionHashFrameVerifierCertificate.toProductBlockEquationSoundness

theorem verifierEquation_of_oneShotAccepts
    (A : ProductionHashFrameOracleAgreement E)
    {state : State} {challenge : N32CanonicalChallenge}
    (haccept :
      A.toVerifierInputs.toSerializedVerifier.oneShotAccepts state challenge =
        true)
    (r : N32Repetition)
    {secret mask : N32Block × LocalFree -> ZMod 12289}
    (hpk :
      publicKeyView (E.outputPoint state).publicKey =
        SignatureDistribution.productBlockPublicKey
          SharedRowProductBlock12289Rows3.localMatrix secret)
    (hcommit :
      commitmentView ((E.outputPoint state).commitments r) =
        SignatureDistribution.productBlockCommitment
          SharedRowProductBlock12289Rows3.localMatrix mask) :
    SignatureDistribution.productBlockVerifierEquation
      SharedRowProductBlock12289Rows3.localMatrix
      (fun b => (challenge r).scalar (q := 12289) b)
      (SignatureDistribution.productBlockPublicKey
        SharedRowProductBlock12289Rows3.localMatrix secret)
      (SignatureDistribution.productBlockCommitment
        SharedRowProductBlock12289Rows3.localMatrix mask)
      (responseView (A.response state) r) := by
  exact
    A.toProductionHashFrameVerifierCertificate.verifierEquation_of_oneShotAccepts
      haccept r hpk hcommit

end ProductionHashFrameOracleAgreement

end N32Q12289ImplicitCommitmentByteVerifier

/--
Accepted-transcript soundness for a systematic relation byte parser.

This is the proof obligation closest to an implementation: if the byte
serialized verifier accepts the programmed transcript and the trace is outside
the declared structural-failure event, then the parser output satisfies the
systematic equation, is nonzero on the free side, and is Lee-short.
-/
structure N32ByteSystematicVerifierEquationSoundness
    {State QueryIdx : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (V :
      N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
        commitmentLen responseLen E)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  parser :
    N32SystematicRelationByteParser (q := q) responseLen Pivot Free
  structuralFailure : N32QueryTrace State -> Bool
  verifier_equation_of_accept :
    ∀ τ,
      V.toSerializedVerifier.oneShotAccepts τ.1 (E.actualChallenge τ) = true ->
      structuralFailure τ = false ->
      ∀ p,
        parser.pivot (V.response τ.1) (E.actualChallenge τ) p +
            ∑ f, P p f *
              parser.free (V.response τ.1) (E.actualChallenge τ) f =
          0
  free_nonzero_of_accept :
    ∀ τ,
      V.toSerializedVerifier.oneShotAccepts τ.1 (E.actualChallenge τ) = true ->
      structuralFailure τ = false ->
      parser.free (V.response τ.1) (E.actualChallenge τ) ≠ 0
  short_of_accept :
    ∀ τ,
      V.toSerializedVerifier.oneShotAccepts τ.1 (E.actualChallenge τ) = true ->
      structuralFailure τ = false ->
      centeredNormL1 q
          (parser.relation (V.response τ.1) (E.actualChallenge τ)) <
        leeMin

/--
Checked-verifier form of the systematic parser soundness obligation.

Instead of proving the three parser facts independently, an implementation can
exhibit one Boolean relation check in its verifier and prove that every accepted
non-failing transcript passed that check.
-/
structure N32ByteCheckedSystematicVerifierEquationSoundness
    {State QueryIdx : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (V :
      N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
        commitmentLen responseLen E)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  parser :
    N32SystematicRelationByteParser (q := q) responseLen Pivot Free
  structuralFailure : N32QueryTrace State -> Bool
  accepted_relation_check :
    ∀ τ,
      V.toSerializedVerifier.oneShotAccepts τ.1 (E.actualChallenge τ) = true ->
      structuralFailure τ = false ->
      parser.verifierChecks P leeMin
          (V.response τ.1) (E.actualChallenge τ) = true

namespace N32ByteCheckedSystematicVerifierEquationSoundness

variable {State QueryIdx : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {V :
  N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
    commitmentLen responseLen E}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

noncomputable def toEquationSoundness
    (C :
      N32ByteCheckedSystematicVerifierEquationSoundness
        (q := q) E V P leeMin) :
    N32ByteSystematicVerifierEquationSoundness
      (q := q) E V P leeMin where
  parser := C.parser
  structuralFailure := C.structuralFailure
  verifier_equation_of_accept := by
    intro τ haccept hstructural p
    exact
      C.parser.equation_of_verifierChecks
        (P := P) (leeMin := leeMin)
        (C.accepted_relation_check τ haccept hstructural) p
  free_nonzero_of_accept := by
    intro τ haccept hstructural
    exact
      C.parser.free_nonzero_of_verifierChecks
        (P := P) (leeMin := leeMin)
        (C.accepted_relation_check τ haccept hstructural)
  short_of_accept := by
    intro τ haccept hstructural
    exact
      C.parser.short_of_verifierChecks
        (P := P) (leeMin := leeMin)
        (C.accepted_relation_check τ haccept hstructural)

end N32ByteCheckedSystematicVerifierEquationSoundness

/--
Checked accepted-transcript soundness for an explicit carried-response
systematic decoder.

This is the implementation-shaped version of
`N32ByteCheckedSystematicVerifierEquationSoundness`: the parser is no longer an
arbitrary function of all seven responses, but is induced by a byte decoder for
one designated carrier repetition.
-/
structure N32ByteExplicitCheckedSystematicVerifierEquationSoundness
    {State QueryIdx : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (V :
      N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
        commitmentLen responseLen E)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  decoder :
    N32ExplicitSystematicRelationByteDecoder
      (q := q) responseLen Pivot Free
  structuralFailure : N32QueryTrace State -> Bool
  accepted_relation_check :
    ∀ τ,
      V.toSerializedVerifier.oneShotAccepts τ.1 (E.actualChallenge τ) = true ->
      structuralFailure τ = false ->
      decoder.toParser.verifierChecks P leeMin
          (V.response τ.1) (E.actualChallenge τ) = true

namespace N32ByteExplicitCheckedSystematicVerifierEquationSoundness

variable {State QueryIdx : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {V :
  N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
    commitmentLen responseLen E}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

noncomputable def toCheckedSoundness
    (C :
      N32ByteExplicitCheckedSystematicVerifierEquationSoundness
        (q := q) E V P leeMin) :
    N32ByteCheckedSystematicVerifierEquationSoundness
      (q := q) E V P leeMin where
  parser := C.decoder.toParser
  structuralFailure := C.structuralFailure
  accepted_relation_check := C.accepted_relation_check

noncomputable def toEquationSoundness
    (C :
      N32ByteExplicitCheckedSystematicVerifierEquationSoundness
        (q := q) E V P leeMin) :
    N32ByteSystematicVerifierEquationSoundness
      (q := q) E V P leeMin :=
  C.toCheckedSoundness.toEquationSoundness

end N32ByteExplicitCheckedSystematicVerifierEquationSoundness

namespace N32RelationCheckingByteVerifierInputs

variable {State QueryIdx : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

noncomputable def toExplicitCheckedSoundness
    (I :
      N32RelationCheckingByteVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E P leeMin)
    (structuralFailure : N32QueryTrace State -> Bool) :
    N32ByteExplicitCheckedSystematicVerifierEquationSoundness
      (q := q) E I.toVerifier P leeMin where
  decoder := I.decoder
  structuralFailure := structuralFailure
  accepted_relation_check := by
    intro τ haccept _hstructural
    exact I.relationCheck_of_oneShotAccepts haccept

end N32RelationCheckingByteVerifierInputs

namespace N32SingleCarrierRelationPayloadVerifierInputs

variable {State QueryIdx : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {decoder :
  N32ExplicitSystematicRelationByteDecoder
    (q := q) responseLen Pivot Free}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

noncomputable def toExplicitCheckedSoundness
    (I :
      N32SingleCarrierRelationPayloadVerifierInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E decoder P leeMin)
    (structuralFailure : N32QueryTrace State -> Bool) :
    N32ByteExplicitCheckedSystematicVerifierEquationSoundness
      (q := q) E I.toVerifier P leeMin where
  decoder := decoder
  structuralFailure := structuralFailure
  accepted_relation_check := by
    intro τ haccept _hstructural
    exact I.relationCheck_of_oneShotAccepts haccept

end N32SingleCarrierRelationPayloadVerifierInputs

/--
Byte-level verifier-equation interface for the systematic relation target.

This is the current concrete algorithmic boundary.  It does not ask the scheme
to construct a certificate violation directly.  Instead the scheme supplies a
response parser and proves that a clean accepted transcript parses to a short
nonzero vector satisfying the systematic verifier equation.
-/
structure N32ByteSystematicVerifierEquationInputs
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  parser :
    N32SystematicRelationByteParser (q := q) responseLen Pivot Free
  forgery : N32QueryTrace State -> Bool
  structuralFailure : N32QueryTrace State -> Bool
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb structuralFailure <= structuralFailureBound
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true
  verifier_equation_of_clean :
    ∀ τ,
      forgery τ = true ->
      E.pointQueried τ = true ->
      structuralFailure τ = false ->
      ∀ p,
        parser.pivot (verifier.response τ.1) (E.actualChallenge τ) p +
            ∑ f, P p f *
              parser.free (verifier.response τ.1) (E.actualChallenge τ) f =
          0
  free_nonzero_of_clean :
    ∀ τ,
      forgery τ = true ->
      E.pointQueried τ = true ->
      structuralFailure τ = false ->
      parser.free (verifier.response τ.1) (E.actualChallenge τ) ≠ 0
  short_of_clean :
    ∀ τ,
      forgery τ = true ->
      E.pointQueried τ = true ->
      structuralFailure τ = false ->
      centeredNormL1 q
          (parser.relation (verifier.response τ.1) (E.actualChallenge τ)) <
        leeMin

namespace N32ByteSystematicVerifierEquationInputs

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

def forgerSuccessProb
    (I :
      N32ByteSystematicVerifierEquationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

def pivot
    (I :
      N32ByteSystematicVerifierEquationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (τ : N32QueryTrace State) :
    Pivot -> ZMod q :=
  I.parser.pivot (I.verifier.response τ.1) (E.actualChallenge τ)

def free
    (I :
      N32ByteSystematicVerifierEquationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (τ : N32QueryTrace State) :
    Free -> ZMod q :=
  I.parser.free (I.verifier.response τ.1) (E.actualChallenge τ)

def relation
    (I :
      N32ByteSystematicVerifierEquationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (τ : N32QueryTrace State) :
    Sum Pivot Free -> ZMod q :=
  I.parser.relation (I.verifier.response τ.1) (E.actualChallenge τ)

def shortRelationOfClean
    (I :
      N32ByteSystematicVerifierEquationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (τ : N32QueryTrace State)
    (hclean :
      I.forgery τ = true ∧ E.pointQueried τ = true ∧
        I.structuralFailure τ = false) :
    ShortSystematicVerifierRelation (q := q) P leeMin where
  pivot := I.pivot τ
  free := I.free τ
  equation := by
    intro p
    simpa [pivot, free] using
      I.verifier_equation_of_clean τ hclean.1 hclean.2.1 hclean.2.2 p
  free_nonzero := by
    simpa [free] using
      I.free_nonzero_of_clean τ hclean.1 hclean.2.1 hclean.2.2
  short := by
    simpa [relation, N32SystematicRelationByteParser.relation, pivot, free]
      using I.short_of_clean τ hclean.1 hclean.2.1 hclean.2.2

noncomputable def relationExtractor
    (I :
      N32ByteSystematicVerifierEquationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (τ : N32QueryTrace State) :
    Option (ShortSystematicVerifierRelation (q := q) P leeMin) :=
  if hclean :
      I.forgery τ = true ∧ E.pointQueried τ = true ∧
        I.structuralFailure τ = false then
    some (I.shortRelationOfClean τ hclean)
  else
    none

noncomputable def toShortSystematicRelationInputs
    (I :
      N32ByteSystematicVerifierEquationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    N32ByteShortSystematicRelationInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeMin where
  verifier := I.verifier
  forgery := I.forgery
  structuralFailure := I.structuralFailure
  relationExtractor := I.relationExtractor
  structuralFailureBound := I.structuralFailureBound
  structuralFailure_acceptProb_le := I.structuralFailure_acceptProb_le
  forgery_accepts := I.forgery_accepts
  clean_queried_forgery_extracts_relation := by
    intro τ hforgery hqueried hstructural
    let hclean :
        I.forgery τ = true ∧ E.pointQueried τ = true ∧
          I.structuralFailure τ = false :=
      ⟨hforgery, hqueried, hstructural⟩
    exact ⟨I.shortRelationOfClean τ hclean, by
      simp [relationExtractor, hclean]⟩

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
    (I :
      N32ByteSystematicVerifierEquationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (hLee : SystematicLeeDistanceAtLeast q P leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  simpa [forgerSuccessProb, toShortSystematicRelationInputs,
    N32ByteShortSystematicRelationInputs.forgerSuccessProb] using
    I.toShortSystematicRelationInputs
      |>.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
        hLee

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
    (I :
      N32ByteSystematicVerifierEquationInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    {eta : Nat}
    (C : PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta)
    (hbound : leeMin = C.leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  subst hbound
  simpa [forgerSuccessProb, toShortSystematicRelationInputs,
    N32ByteShortSystematicRelationInputs.forgerSuccessProb] using
    I.toShortSystematicRelationInputs
      |>.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
        C rfl

end N32ByteSystematicVerifierEquationInputs

/--
Byte-level verifier-equation inputs derived from accepted-transcript parser
soundness.

The only remaining first-run connection is `forgery_accepts`: a successful
forgery must be accepted by the byte serialized verifier.  The parser facts
then follow from `N32ByteSystematicVerifierEquationSoundness` and
`oneShotAccepts_of_outputAccepts`.
-/
structure N32ByteSystematicVerifierEquationSoundnessInputs
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  soundness :
    N32ByteSystematicVerifierEquationSoundness
      (q := q) E verifier P leeMin
  forgery : N32QueryTrace State -> Bool
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb soundness.structuralFailure <=
      structuralFailureBound
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true

namespace N32ByteSystematicVerifierEquationSoundnessInputs

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

def forgerSuccessProb
    (I :
      N32ByteSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

noncomputable def toVerifierEquationInputs
    (I :
      N32ByteSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    N32ByteSystematicVerifierEquationInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeMin where
  verifier := I.verifier
  parser := I.soundness.parser
  forgery := I.forgery
  structuralFailure := I.soundness.structuralFailure
  structuralFailureBound := I.structuralFailureBound
  structuralFailure_acceptProb_le := I.structuralFailure_acceptProb_le
  forgery_accepts := I.forgery_accepts
  verifier_equation_of_clean := by
    intro τ hforgery _hqueried hstructural p
    have honeShot :
        I.verifier.toSerializedVerifier.oneShotAccepts τ.1
          (E.actualChallenge τ) = true :=
      I.verifier.toSerializedVerifier.oneShotAccepts_of_outputAccepts τ
        (I.forgery_accepts τ hforgery)
    exact
      I.soundness.verifier_equation_of_accept τ honeShot hstructural p
  free_nonzero_of_clean := by
    intro τ hforgery _hqueried hstructural
    have honeShot :
        I.verifier.toSerializedVerifier.oneShotAccepts τ.1
          (E.actualChallenge τ) = true :=
      I.verifier.toSerializedVerifier.oneShotAccepts_of_outputAccepts τ
        (I.forgery_accepts τ hforgery)
    exact I.soundness.free_nonzero_of_accept τ honeShot hstructural
  short_of_clean := by
    intro τ hforgery _hqueried hstructural
    have honeShot :
        I.verifier.toSerializedVerifier.oneShotAccepts τ.1
          (E.actualChallenge τ) = true :=
      I.verifier.toSerializedVerifier.oneShotAccepts_of_outputAccepts τ
        (I.forgery_accepts τ hforgery)
    exact I.soundness.short_of_accept τ honeShot hstructural

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
    (I :
      N32ByteSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (hLee : SystematicLeeDistanceAtLeast q P leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  simpa [forgerSuccessProb, toVerifierEquationInputs,
    N32ByteSystematicVerifierEquationInputs.forgerSuccessProb] using
    I.toVerifierEquationInputs
      |>.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
        hLee

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
    (I :
      N32ByteSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    {eta : Nat}
    (C : PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta)
    (hbound : leeMin = C.leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  subst hbound
  simpa [forgerSuccessProb, toVerifierEquationInputs,
    N32ByteSystematicVerifierEquationInputs.forgerSuccessProb] using
    I.toVerifierEquationInputs
      |>.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
        C rfl

end N32ByteSystematicVerifierEquationSoundnessInputs

/--
First-run inputs using the checked-verifier form of parser soundness.
-/
structure N32ByteCheckedSystematicVerifierEquationSoundnessInputs
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  checkedSoundness :
    N32ByteCheckedSystematicVerifierEquationSoundness
      (q := q) E verifier P leeMin
  forgery : N32QueryTrace State -> Bool
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb checkedSoundness.structuralFailure <=
      structuralFailureBound
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true

namespace N32ByteCheckedSystematicVerifierEquationSoundnessInputs

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

def forgerSuccessProb
    (I :
      N32ByteCheckedSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

noncomputable def toSoundnessInputs
    (I :
      N32ByteCheckedSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    N32ByteSystematicVerifierEquationSoundnessInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeMin where
  verifier := I.verifier
  soundness := I.checkedSoundness.toEquationSoundness
  forgery := I.forgery
  structuralFailureBound := I.structuralFailureBound
  structuralFailure_acceptProb_le := I.structuralFailure_acceptProb_le
  forgery_accepts := I.forgery_accepts

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
    (I :
      N32ByteCheckedSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (hLee : SystematicLeeDistanceAtLeast q P leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  simpa [forgerSuccessProb, toSoundnessInputs,
    N32ByteSystematicVerifierEquationSoundnessInputs.forgerSuccessProb] using
    I.toSoundnessInputs
      |>.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
        hLee

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
    (I :
      N32ByteCheckedSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    {eta : Nat}
    (C : PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta)
    (hbound : leeMin = C.leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  subst hbound
  simpa [forgerSuccessProb, toSoundnessInputs,
    N32ByteSystematicVerifierEquationSoundnessInputs.forgerSuccessProb] using
    I.toSoundnessInputs
      |>.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
        C rfl

end N32ByteCheckedSystematicVerifierEquationSoundnessInputs

/--
First-run inputs using the explicit carried-response checked decoder.

This is the most concrete reduction boundary currently exposed: the verifier
only has to prove that accepted non-failing transcripts pass the checked
relation test induced by the designated-carrier decoder.
-/
structure N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (leeMin : Nat) where
  verifier :
    N32ByteSerializedVerifierInputs domainLen publicKeyLen messageLen
      commitmentLen responseLen E
  explicitSoundness :
    N32ByteExplicitCheckedSystematicVerifierEquationSoundness
      (q := q) E verifier P leeMin
  forgery : N32QueryTrace State -> Bool
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb explicitSoundness.structuralFailure <=
      structuralFailureBound
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifier.toSerializedVerifier.outputAccepts τ = true

namespace N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {leeMin : Nat}

def forgerSuccessProb
    (I :
      N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    ℝ :=
  E.traceDist.acceptProb I.forgery

noncomputable def toCheckedSoundnessInputs
    (I :
      N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin) :
    N32ByteCheckedSystematicVerifierEquationSoundnessInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeMin where
  verifier := I.verifier
  checkedSoundness := I.explicitSoundness.toCheckedSoundness
  forgery := I.forgery
  structuralFailureBound := I.structuralFailureBound
  structuralFailure_acceptProb_le := I.structuralFailure_acceptProb_le
  forgery_accepts := I.forgery_accepts

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
    (I :
      N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    (hLee : SystematicLeeDistanceAtLeast q P leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  simpa [forgerSuccessProb, toCheckedSoundnessInputs,
    N32ByteCheckedSystematicVerifierEquationSoundnessInputs.forgerSuccessProb]
    using
      I.toCheckedSoundnessInputs
        |>.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
          hLee

theorem forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
    (I :
      N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P leeMin)
    {eta : Nat}
    (C : PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta)
    (hbound : leeMin = C.leeMin) :
    I.forgerSuccessProb <= n32ChallengeGuessBound + I.structuralFailureBound := by
  subst hbound
  simpa [forgerSuccessProb, toCheckedSoundnessInputs,
    N32ByteCheckedSystematicVerifierEquationSoundnessInputs.forgerSuccessProb]
    using
      I.toCheckedSoundnessInputs
        |>.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
          C rfl

end N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs

/--
Packaged certificate for the explicit carried-response checked route, using a
raw systematic Lee certificate.
-/
structure N32ByteExplicitCheckedSystematicVerifierEquationSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (Clee : SystematicLeeCertificate q Pivot Free P) where
  inputs :
    N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P Clee.lowerBound

namespace N32ByteExplicitCheckedSystematicVerifierEquationSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {Clee : SystematicLeeCertificate q Pivot Free P}

def forgerSuccessProb
    (C :
      N32ByteExplicitCheckedSystematicVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteExplicitCheckedSystematicVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
      Clee.lowerBoundProof

end N32ByteExplicitCheckedSystematicVerifierEquationSecurityCertificate

/--
Packaged certificate for the explicit carried-response checked route, using
the serialized public-view Lee-dual certificate.
-/
structure N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (eta : Nat) where
  leeDual :
    PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta
  inputs :
    N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeDual.leeMin

namespace N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {eta : Nat}

def forgerSuccessProb
    (C :
      N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
      C.leeDual rfl

end N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate

/--
Final named security wrapper for the validated q=12289, n=48, m1=32
single-carrier systematic-relation signature route.

The concrete matrix `P` is the shared-row product-block sidecar
`q12289_m132_target512_rows_per_group3`.  The local bounded-free Lee check is
internalized by `SharedRowProductBlock12289Rows3.internalLocalLeeCertificate`;
all global composition, byte parsing, checked-verifier implication, and final
probability bound are then Lean theorems.
-/
structure N32Q12289Validated48x32SingleCarrierSecurityCertificate
    {State QueryIdx SchemeLocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × SchemeLocalFree ->
      ZMod N32Q12289SystematicRelationLayout.modulus) where
  verifierInputs :
    N32Q12289SystematicRelationLayout.Validated48x32ProductBlockSingleCarrierVerifierInputs
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen) E
  structuralFailure : N32QueryTrace State -> Bool
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb structuralFailure <= structuralFailureBound
  forgery : N32QueryTrace State -> Bool
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      verifierInputs.toVerifier.toSerializedVerifier.outputAccepts τ = true

namespace N32Q12289Validated48x32SingleCarrierSecurityCertificate

variable {State QueryIdx SchemeLocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × SchemeLocalFree ->
  ZMod N32Q12289SystematicRelationLayout.modulus}

def leeDual
    (_C :
      N32Q12289Validated48x32SingleCarrierSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    PublicViewLeeDualCertificate
      (q := N32Q12289SystematicRelationLayout.modulus)
      (ViewCoord := Sum SharedRowProductBlock12289Rows3.Pivot
        SharedRowProductBlock12289Rows3.Free)
      (DualTarget := SharedRowProductBlock12289Rows3.Pivot ->
        ZMod N32Q12289SystematicRelationLayout.modulus)
      (systematicDualMap N32Q12289SystematicRelationLayout.modulus
        SharedRowProductBlock12289Rows3.globalMatrix)
      SharedRowProductBlock12289Rows3.eta :=
  SharedRowProductBlock12289Rows3.leeDualCertificateFromRepeatedLocal
    SharedRowProductBlock12289Rows3.internalLocalLeeCertificate
    SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower

noncomputable def explicitSoundness
    (C :
      N32Q12289Validated48x32SingleCarrierSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    N32ByteExplicitCheckedSystematicVerifierEquationSoundness
      (q := N32Q12289SystematicRelationLayout.modulus)
      E C.verifierInputs.toVerifier
      SharedRowProductBlock12289Rows3.globalMatrix
      SharedRowProductBlock12289Rows3.targetLee :=
  C.verifierInputs.toExplicitCheckedSoundness C.structuralFailure

noncomputable def soundnessInputs
    (C :
      N32Q12289Validated48x32SingleCarrierSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
      (q := N32Q12289SystematicRelationLayout.modulus)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := 140) E secret
      SharedRowProductBlock12289Rows3.globalMatrix
      SharedRowProductBlock12289Rows3.targetLee where
  verifier := C.verifierInputs.toVerifier
  explicitSoundness := C.explicitSoundness
  forgery := C.forgery
  structuralFailureBound := C.structuralFailureBound
  structuralFailure_acceptProb_le := C.structuralFailure_acceptProb_le
  forgery_accepts := C.forgery_accepts

noncomputable def toPublicViewSecurityCertificate
    (C :
      N32Q12289Validated48x32SingleCarrierSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate
      (q := N32Q12289SystematicRelationLayout.modulus)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := 140) E secret
      SharedRowProductBlock12289Rows3.globalMatrix
      SharedRowProductBlock12289Rows3.eta where
  leeDual := C.leeDual
  inputs := by
    change
      N32ByteExplicitCheckedSystematicVerifierEquationSoundnessInputs
        (q := N32Q12289SystematicRelationLayout.modulus)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := 140) E secret
        SharedRowProductBlock12289Rows3.globalMatrix
        SharedRowProductBlock12289Rows3.targetLee
    exact C.soundnessInputs

def forgerSuccessProb
    (C :
      N32Q12289Validated48x32SingleCarrierSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    ℝ :=
  E.traceDist.acceptProb C.forgery

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32Q12289Validated48x32SingleCarrierSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.structuralFailureBound := by
  simpa [forgerSuccessProb, soundnessInputs] using
    C.soundnessInputs
      |>.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
        C.leeDual rfl

end N32Q12289Validated48x32SingleCarrierSecurityCertificate

/--
Final validated q12289 wrapper using the canonical payload-base verifier.

Compared with `N32Q12289Validated48x32SingleCarrierSecurityCertificate`, this
variant fixes the concrete base payload predicate to be
`coreAcceptsPayload && payloadRangeCheck`.  The remaining implementation proofs
are therefore only about the core verifier checks and Fiat-Shamir binding under
canonical payload encodings.
-/
structure N32Q12289Validated48x32CanonicalBaseSecurityCertificate
    {State QueryIdx SchemeLocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × SchemeLocalFree ->
      ZMod N32Q12289SystematicRelationLayout.modulus) where
  verifierInputs :
    N32Q12289SystematicRelationLayout.Validated48x32ProductBlockCanonicalBaseVerifierInputs
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen) E
  structuralFailure : N32QueryTrace State -> Bool
  structuralFailureBound : ℝ
  structuralFailure_acceptProb_le :
    E.traceDist.acceptProb structuralFailure <= structuralFailureBound
  forgery : N32QueryTrace State -> Bool
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      (verifierInputs.toSingleCarrierVerifierInputs.toVerifier.toSerializedVerifier).outputAccepts
        τ = true

namespace N32Q12289Validated48x32CanonicalBaseSecurityCertificate

variable {State QueryIdx SchemeLocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × SchemeLocalFree ->
  ZMod N32Q12289SystematicRelationLayout.modulus}

noncomputable def toSingleCarrierSecurityCertificate
    (C :
      N32Q12289Validated48x32CanonicalBaseSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    N32Q12289Validated48x32SingleCarrierSecurityCertificate
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen) E secret where
  verifierInputs := C.verifierInputs.toSingleCarrierVerifierInputs
  structuralFailure := C.structuralFailure
  structuralFailureBound := C.structuralFailureBound
  structuralFailure_acceptProb_le := C.structuralFailure_acceptProb_le
  forgery := C.forgery
  forgery_accepts := C.forgery_accepts

noncomputable def toPublicViewSecurityCertificate
    (C :
      N32Q12289Validated48x32CanonicalBaseSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate
      (q := N32Q12289SystematicRelationLayout.modulus)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := 140) E secret
      SharedRowProductBlock12289Rows3.globalMatrix
      SharedRowProductBlock12289Rows3.eta :=
  C.toSingleCarrierSecurityCertificate.toPublicViewSecurityCertificate

def forgerSuccessProb
    (C :
      N32Q12289Validated48x32CanonicalBaseSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    ℝ :=
  E.traceDist.acceptProb C.forgery

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32Q12289Validated48x32CanonicalBaseSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.structuralFailureBound := by
  simpa [forgerSuccessProb, toSingleCarrierSecurityCertificate,
    N32Q12289Validated48x32SingleCarrierSecurityCertificate.forgerSuccessProb]
    using
      C.toSingleCarrierSecurityCertificate
        |>.forgerSuccessProb_le_guess_add_structuralFailure

end N32Q12289Validated48x32CanonicalBaseSecurityCertificate

/--
No-structural-failure specialization of the final validated q12289 wrapper.

Use this when the implementation makes every structural condition an explicit
verifier check: canonical 14-bit payload range checking is built into
`Validated48x32ProductBlockCanonicalBaseVerifierInputs`, and the systematic
relation check is built into the constructed verifier.  The remaining
implementation fields are then the core payload predicate, byte-frame binding,
Fiat-Shamir recomputation agreement, and the usual forgery-accepts link.
-/
structure N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate
    {State QueryIdx SchemeLocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × SchemeLocalFree ->
      ZMod N32Q12289SystematicRelationLayout.modulus) where
  verifierInputs :
    N32Q12289SystematicRelationLayout.Validated48x32ProductBlockCanonicalBaseVerifierInputs
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen) E
  forgery : N32QueryTrace State -> Bool
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      (verifierInputs.toSingleCarrierVerifierInputs.toVerifier.toSerializedVerifier).outputAccepts
        τ = true

namespace N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate

variable {State QueryIdx SchemeLocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × SchemeLocalFree ->
  ZMod N32Q12289SystematicRelationLayout.modulus}

omit [Fintype QueryIdx] in
theorem falseStructuralFailure_acceptProb_le_zero :
    E.traceDist.acceptProb (fun _ : N32QueryTrace State => false) <= 0 := by
  have hzero :
      E.traceDist.acceptProb (fun _ : N32QueryTrace State => false) = 0 :=
    acceptProb_eq_zero_of_forall_not E.traceDist
      (fun _ : N32QueryTrace State => false)
      (by
        intro τ hfalse
        cases hfalse)
  exact le_of_eq hzero

noncomputable def toCanonicalBaseSecurityCertificate
    (C :
      N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    N32Q12289Validated48x32CanonicalBaseSecurityCertificate
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen) E secret where
  verifierInputs := C.verifierInputs
  structuralFailure := fun _ => false
  structuralFailureBound := 0
  structuralFailure_acceptProb_le := falseStructuralFailure_acceptProb_le_zero
  forgery := C.forgery
  forgery_accepts := C.forgery_accepts

noncomputable def toPublicViewSecurityCertificate
    (C :
      N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate
      (q := N32Q12289SystematicRelationLayout.modulus)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := 140) E secret
      SharedRowProductBlock12289Rows3.globalMatrix
      SharedRowProductBlock12289Rows3.eta :=
  C.toCanonicalBaseSecurityCertificate.toPublicViewSecurityCertificate

def forgerSuccessProb
    (C :
      N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    ℝ :=
  E.traceDist.acceptProb C.forgery

theorem forgerSuccessProb_le_guess
    (C :
      N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    C.forgerSuccessProb <= n32ChallengeGuessBound := by
  simpa [forgerSuccessProb, toCanonicalBaseSecurityCertificate,
    N32Q12289Validated48x32CanonicalBaseSecurityCertificate.forgerSuccessProb]
    using
      C.toCanonicalBaseSecurityCertificate
        |>.forgerSuccessProb_le_guess_add_structuralFailure

end N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate

/--
Final no-failure q12289 certificate using the Fiat-Shamir recomputing core
verifier interface directly.

This is the most implementation-facing endpoint in the current repository:
the concrete verifier supplies a payload predicate, a byte-frame challenge
recomputation function, and the query-log agreement lemma for accepted
payloads.  The wrapper derives canonical range checking, byte-frame binding,
systematic relation checking, and the no-structural-failure security bound.
-/
structure N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate
    {State QueryIdx SchemeLocalFree : Type*}
    [Fintype State]
    [Fintype QueryIdx]
    {domainLen publicKeyLen messageLen commitmentLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × SchemeLocalFree ->
      ZMod N32Q12289SystematicRelationLayout.modulus) where
  verifierInputs :
    N32Q12289SystematicRelationLayout.Validated48x32ProductBlockFSCoreVerifierInputs
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen) E
  forgery : N32QueryTrace State -> Bool
  forgery_accepts :
    ∀ τ, forgery τ = true ->
      (verifierInputs.toCanonicalBaseVerifierInputs
        |>.toSingleCarrierVerifierInputs
        |>.toVerifier
        |>.toSerializedVerifier).outputAccepts τ = true

namespace N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate

variable {State QueryIdx SchemeLocalFree : Type*}
variable [Fintype State]
variable [Fintype QueryIdx]
variable {domainLen publicKeyLen messageLen commitmentLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × SchemeLocalFree ->
  ZMod N32Q12289SystematicRelationLayout.modulus}

noncomputable def toCanonicalNoFailureSecurityCertificate
    (C :
      N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen) E secret where
  verifierInputs := C.verifierInputs.toCanonicalBaseVerifierInputs
  forgery := C.forgery
  forgery_accepts := C.forgery_accepts

noncomputable def toPublicViewSecurityCertificate
    (C :
      N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    N32ByteExplicitCheckedSystematicPublicViewVerifierEquationSecurityCertificate
      (q := N32Q12289SystematicRelationLayout.modulus)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := 140) E secret
      SharedRowProductBlock12289Rows3.globalMatrix
      SharedRowProductBlock12289Rows3.eta :=
  C.toCanonicalNoFailureSecurityCertificate.toPublicViewSecurityCertificate

def forgerSuccessProb
    (C :
      N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    ℝ :=
  E.traceDist.acceptProb C.forgery

theorem forgerSuccessProb_le_guess
    (C :
      N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen) E secret) :
    C.forgerSuccessProb <= n32ChallengeGuessBound := by
  simpa [forgerSuccessProb, toCanonicalNoFailureSecurityCertificate,
    N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate.forgerSuccessProb]
    using
      C.toCanonicalNoFailureSecurityCertificate.forgerSuccessProb_le_guess

end N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate

/--
Packaged certificate for the checked-verifier parser route, using a raw
systematic Lee certificate.
-/
structure N32ByteCheckedSystematicVerifierEquationSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (Clee : SystematicLeeCertificate q Pivot Free P) where
  inputs :
    N32ByteCheckedSystematicVerifierEquationSoundnessInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P Clee.lowerBound

namespace N32ByteCheckedSystematicVerifierEquationSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {Clee : SystematicLeeCertificate q Pivot Free P}

def forgerSuccessProb
    (C :
      N32ByteCheckedSystematicVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteCheckedSystematicVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
      Clee.lowerBoundProof

end N32ByteCheckedSystematicVerifierEquationSecurityCertificate

/--
Packaged certificate for the checked-verifier parser route, using the
serialized public-view Lee-dual certificate.
-/
structure N32ByteCheckedSystematicPublicViewVerifierEquationSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (eta : Nat) where
  leeDual :
    PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta
  inputs :
    N32ByteCheckedSystematicVerifierEquationSoundnessInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeDual.leeMin

namespace N32ByteCheckedSystematicPublicViewVerifierEquationSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {eta : Nat}

def forgerSuccessProb
    (C :
      N32ByteCheckedSystematicPublicViewVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteCheckedSystematicPublicViewVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
      C.leeDual rfl

end N32ByteCheckedSystematicPublicViewVerifierEquationSecurityCertificate

/--
Packaged certificate for accepted-transcript parser soundness, using a raw
systematic Lee certificate.
-/
structure N32ByteSystematicVerifierEquationSoundnessSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (Clee : SystematicLeeCertificate q Pivot Free P) where
  inputs :
    N32ByteSystematicVerifierEquationSoundnessInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P Clee.lowerBound

namespace N32ByteSystematicVerifierEquationSoundnessSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {Clee : SystematicLeeCertificate q Pivot Free P}

def forgerSuccessProb
    (C :
      N32ByteSystematicVerifierEquationSoundnessSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteSystematicVerifierEquationSoundnessSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
      Clee.lowerBoundProof

end N32ByteSystematicVerifierEquationSoundnessSecurityCertificate

/--
Packaged certificate for accepted-transcript parser soundness, using the
serialized public-view Lee-dual certificate.
-/
structure N32ByteSystematicPublicViewVerifierEquationSoundnessSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (eta : Nat) where
  leeDual :
    PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta
  inputs :
    N32ByteSystematicVerifierEquationSoundnessInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeDual.leeMin

namespace N32ByteSystematicPublicViewVerifierEquationSoundnessSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {eta : Nat}

def forgerSuccessProb
    (C :
      N32ByteSystematicPublicViewVerifierEquationSoundnessSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteSystematicPublicViewVerifierEquationSoundnessSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
      C.leeDual rfl

end N32ByteSystematicPublicViewVerifierEquationSoundnessSecurityCertificate

/--
Packaged security certificate for the byte-parser verifier-equation interface,
using a raw systematic Lee certificate.
-/
structure N32ByteSystematicVerifierEquationSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (Clee : SystematicLeeCertificate q Pivot Free P) where
  inputs :
    N32ByteSystematicVerifierEquationInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P Clee.lowerBound

namespace N32ByteSystematicVerifierEquationSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {Clee : SystematicLeeCertificate q Pivot Free P}

def forgerSuccessProb
    (C :
      N32ByteSystematicVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteSystematicVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
      Clee.lowerBoundProof

end N32ByteSystematicVerifierEquationSecurityCertificate

/--
Packaged security certificate for the byte-parser verifier-equation interface,
using the serialized public-view Lee-dual certificate.
-/
structure N32ByteSystematicPublicViewVerifierEquationSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (eta : Nat) where
  leeDual :
    PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta
  inputs :
    N32ByteSystematicVerifierEquationInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeDual.leeMin

namespace N32ByteSystematicPublicViewVerifierEquationSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {eta : Nat}

def forgerSuccessProb
    (C :
      N32ByteSystematicPublicViewVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteSystematicPublicViewVerifierEquationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
      C.leeDual rfl

end N32ByteSystematicPublicViewVerifierEquationSecurityCertificate

/--
Packaged security certificate for the concrete systematic verifier-equation
extractor, using a raw systematic Lee certificate.
-/
structure N32ByteShortSystematicRelationSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (Clee : SystematicLeeCertificate q Pivot Free P) where
  inputs :
    N32ByteShortSystematicRelationInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P Clee.lowerBound

namespace N32ByteShortSystematicRelationSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {Clee : SystematicLeeCertificate q Pivot Free P}

def forgerSuccessProb
    (C :
      N32ByteShortSystematicRelationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteShortSystematicRelationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
      Clee.lowerBoundProof

end N32ByteShortSystematicRelationSecurityCertificate

/--
Packaged security certificate for the concrete systematic verifier-equation
extractor, using the serialized public-view Lee-dual certificate.
-/
structure N32ByteShortSystematicPublicViewRelationSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (eta : Nat) where
  leeDual :
    PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta
  inputs :
    N32ByteShortSystematicRelationInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeDual.leeMin

namespace N32ByteShortSystematicPublicViewRelationSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {eta : Nat}

def forgerSuccessProb
    (C :
      N32ByteShortSystematicPublicViewRelationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteShortSystematicPublicViewRelationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
      C.leeDual rfl

end N32ByteShortSystematicPublicViewRelationSecurityCertificate

/--
Packaged security certificate for a systematic Lee-distance target.

Here the input type is indexed by `C.lowerBound`, so no separate equality proof
is needed at use sites.
-/
structure N32ByteSystematicLeeViolationSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (Clee : SystematicLeeCertificate q Pivot Free P) where
  inputs :
    N32ByteSystematicLeeViolationInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P Clee.lowerBound

namespace N32ByteSystematicLeeViolationSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {Clee : SystematicLeeCertificate q Pivot Free P}

def forgerSuccessProb
    (C :
      N32ByteSystematicLeeViolationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteSystematicLeeViolationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P Clee) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_distanceAtLeast
      Clee.lowerBoundProof

end N32ByteSystematicLeeViolationSecurityCertificate

/--
Packaged security certificate for a systematic public-view Lee-dual certificate.

This is the most direct endpoint for `A = [I | P]`: the public-view certificate
itself fixes the Lee bound used by the extracted `SystematicLeeViolation`.
-/
structure N32ByteSystematicPublicViewLeeViolationSecurityCertificate
    {State QueryIdx LocalFree : Type*} {Pivot Free : Type}
    [Fintype State]
    [Fintype QueryIdx]
    [Fintype Pivot]
    [Fintype Free]
    {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
    (E :
      N32QueryLogExperiment State QueryIdx
        (ByteVector publicKeyLen) (ByteVector messageLen)
        (ByteVector commitmentLen))
    (secret : N32Block × LocalFree -> ZMod q)
    (P : Pivot -> Free -> ZMod q)
    (eta : Nat) where
  leeDual :
    PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta
  inputs :
    N32ByteSystematicLeeViolationInputs
      (q := q)
      (domainLen := domainLen)
      (publicKeyLen := publicKeyLen)
      (messageLen := messageLen)
      (commitmentLen := commitmentLen)
      (responseLen := responseLen) E secret P leeDual.leeMin

namespace N32ByteSystematicPublicViewLeeViolationSecurityCertificate

variable {State QueryIdx LocalFree : Type*}
variable {Pivot Free : Type}
variable [Fintype State]
variable [Fintype QueryIdx]
variable [Fintype Pivot]
variable [Fintype Free]
variable {domainLen publicKeyLen messageLen commitmentLen responseLen : Nat}
variable {E :
  N32QueryLogExperiment State QueryIdx
    (ByteVector publicKeyLen) (ByteVector messageLen)
    (ByteVector commitmentLen)}
variable {secret : N32Block × LocalFree -> ZMod q}
variable {P : Pivot -> Free -> ZMod q}
variable {eta : Nat}

def forgerSuccessProb
    (C :
      N32ByteSystematicPublicViewLeeViolationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    ℝ :=
  C.inputs.forgerSuccessProb

theorem forgerSuccessProb_le_guess_add_structuralFailure
    (C :
      N32ByteSystematicPublicViewLeeViolationSecurityCertificate
        (q := q)
        (domainLen := domainLen)
        (publicKeyLen := publicKeyLen)
        (messageLen := messageLen)
        (commitmentLen := commitmentLen)
        (responseLen := responseLen) E secret P eta) :
    C.forgerSuccessProb <=
      n32ChallengeGuessBound + C.inputs.structuralFailureBound := by
  simpa [forgerSuccessProb] using
    C.inputs.forgerSuccessProb_le_guess_add_structuralFailure_of_publicViewLeeCertificate
      C.leeDual rfl

end N32ByteSystematicPublicViewLeeViolationSecurityCertificate

end

end SignatureTranscript
end Descender
