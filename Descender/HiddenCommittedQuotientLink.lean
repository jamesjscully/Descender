import Descender.CommittedSigma
import Descender.DescentInterface

namespace Descender

/-!
# Hidden Committed Quotient Link

This sidecar module records the surviving quotient-level-link research branch.
Public quotient links failed the hiding-support replay: making the link rows
public conditions the hidden representative sampler too strongly.  The remaining
possibility is a link value hidden behind commitments, with a proof of the link
equation that is not merely a public projection of the sigma equation.

This file is intentionally not imported by `Descender.lean` or deployment proof
files.  It is a theorem-shape prototype for the research program.
-/

namespace HiddenCommittedQuotientLink

open CommittedSigma

/-- Named break buckets for hidden committed quotient links. -/
structure HiddenCommittedLinkBreaks where
  commitmentBindingBreak : Prop
  linkEquationBreak : Prop
  publicProjectionBreak : Prop
  samplerHidingBreak : Prop

namespace HiddenCommittedLinkBreaks

/-- Any break allowed by the hidden-commitment branch. -/
def any (B : HiddenCommittedLinkBreaks) : Prop :=
  B.commitmentBindingBreak ∨
    B.linkEquationBreak ∨
      B.publicProjectionBreak ∨
        B.samplerHidingBreak

theorem any_of_commitmentBindingBreak
    {B : HiddenCommittedLinkBreaks}
    (h : B.commitmentBindingBreak) : B.any :=
  Or.inl h

theorem any_of_linkEquationBreak
    {B : HiddenCommittedLinkBreaks}
    (h : B.linkEquationBreak) : B.any :=
  Or.inr (Or.inl h)

theorem any_of_publicProjectionBreak
    {B : HiddenCommittedLinkBreaks}
    (h : B.publicProjectionBreak) : B.any :=
  Or.inr (Or.inr (Or.inl h))

theorem any_of_samplerHidingBreak
    {B : HiddenCommittedLinkBreaks}
    (h : B.samplerHidingBreak) : B.any :=
  Or.inr (Or.inr (Or.inr h))

end HiddenCommittedLinkBreaks

/--
Hidden commitment interface for quotient-level links.

`LinkValue` is deliberately abstract.  Concrete instantiations may use a
two-coordinate q-vector, a sparse linear row value, or another quotient-level
object.  The account stores commitments to the relevant link values, not the
link values themselves.

The three Prop fields before `accepts_accountBacked_or_break` are review gates:
they are the exact places where the public-link shortcut failed.
-/
structure Interface
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof LinkValue :
      Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop) where
  witnessCommit : Witness -> WitnessCommitment
  randomizerCommit : Randomizer -> RandomizerCommitment
  hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment
  proofCommitmentOfRandomizer : Randomizer -> StatementDigest
  linkOfWitness : Witness -> LinkValue
  linkOfRandomizer : Randomizer -> LinkValue
  linkOfResponse : Response -> LinkValue
  localOpeningAccepts :
    Transcript StatementDigest WitnessCommitment StatementDigest
      RandomizerCommitment Challenge Response LocalOpening -> Prop
  linkProofAccepts :
    AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
      RandomizerCommitment LinkCommitment Challenge Response
      (AccountCarriedOpening LocalOpening LinkProof) -> Prop
  localOpeningRelation :
    Transcript StatementDigest WitnessCommitment StatementDigest
      RandomizerCommitment Challenge Response LocalOpening ->
    Witness -> Randomizer -> Prop
  breaks : HiddenCommittedLinkBreaks
  commitmentHidesLinkValues : Prop
  nonPublicProjection : Prop
  samplerSurfacePreserved : Prop
  accepts_accountBacked_or_break :
    ∀ T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof),
      accountSlotWellFormed T.accountSlot ->
      A.verifierEquation T.proofCommitment T.challenge
        T.accountSlot.statementDigest T.response ->
      localOpeningAccepts (AccountCarriedOpening.localTranscript T.toTranscript) ->
      linkProofAccepts T ->
        AccountBackedByHiddenWitness A witnessCommit randomizerCommit
          proofCommitmentOfRandomizer hiddenLinkCommit
          (fun Tfull w r =>
            localOpeningRelation
              (AccountCarriedOpening.localTranscript Tfull) w r) T ∨
        breaks.any

namespace Interface

variable {Witness Randomizer StatementDigest WitnessCommitment
  RandomizerCommitment LinkCommitment Challenge Response LocalOpening LinkProof
  LinkValue : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {accountSlotWellFormed :
  AccountCarriedSlot StatementDigest WitnessCommitment RandomizerCommitment
    LinkCommitment -> Prop}

/-- Opening acceptance is the local opening plus the hidden link proof. -/
def accountOpeningAccepts
    (H :
      Interface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        (LinkValue := LinkValue) A accountSlotWellFormed)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof)) : Prop :=
  H.localOpeningAccepts (AccountCarriedOpening.localTranscript T.toTranscript) ∧
    H.linkProofAccepts T

/--
The hidden-commitment interface is a refinement of the existing generic
account-link proof system.
-/
def toAccountLinkProofSystem
    (H :
      Interface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        (LinkValue := LinkValue) A accountSlotWellFormed) :
    AccountLinkProofSystem
      (Witness := Witness) (Randomizer := Randomizer)
      (StatementDigest := StatementDigest)
      (WitnessCommitment := WitnessCommitment)
      (RandomizerCommitment := RandomizerCommitment)
      (LinkCommitment := LinkCommitment)
      (Challenge := Challenge) (Response := Response)
      (LocalOpening := LocalOpening) (LinkProof := LinkProof)
      A accountSlotWellFormed where
  witnessCommit := H.witnessCommit
  randomizerCommit := H.randomizerCommit
  hiddenLinkCommit := H.hiddenLinkCommit
  proofCommitmentOfRandomizer := H.proofCommitmentOfRandomizer
  localOpeningAccepts := H.localOpeningAccepts
  linkProofAccepts := H.linkProofAccepts
  localOpeningRelation := H.localOpeningRelation
  accountLinkProofBreak := H.breaks.any
  accepts_accountBacked_or_break := by
    intro T hSlot hEquation hLocal hLink
    exact H.accepts_accountBacked_or_break T hSlot hEquation hLocal hLink

/--
If all hidden-commitment break buckets are ruled out, an accepting account-aware
transcript is backed by hidden witness/randomizer state.
-/
theorem accepts_accountBacked_of_noBreaks
    (H :
      Interface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        (LinkValue := LinkValue) A accountSlotWellFormed)
    (hNoBreaks : ¬ H.breaks.any)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof))
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        H.accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A H.witnessCommit H.randomizerCommit
      H.proofCommitmentOfRandomizer H.hiddenLinkCommit
      (fun Tfull w r =>
        H.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T := by
  exact
    (H.toAccountLinkProofSystem).accepts_accountBacked_of_noBreak hNoBreaks
      T hAccept

/--
The descent-interface wrapper for the hidden-commitment branch.

This theorem is intentionally generic: it states the shape needed for a future
candidate-specific q12289 instantiation without importing that instantiation
into deployment proofs.
-/
theorem accepts_glues_or_breaks_of_effectiveDescent
    {Public Representative PublicQuotient Hidden AccountEvidence
      DescentBreak : Type*}
    {D :
      DescentInterface.LocalQuotientDescent Public Representative
        PublicQuotient Hidden AccountEvidence DescentBreak}
    (C : DescentInterface.EffectiveDescentCertificate D)
    {accepts : Public -> Prop}
    (accepts_yields_descent :
      forall {pub : Public}, accepts pub -> D.DescentData pub)
    {pub : Public}
    (hAccept : accepts pub) :
    (exists representative hidden evidence,
      D.LocallyValid pub representative ∧
        D.hiddenRepresents pub representative hidden ∧
          D.accountBacked pub hidden evidence) ∨
    (exists representative breakEvent,
      D.LocallyValid pub representative ∧
        D.breakFor pub representative breakEvent) :=
  DescentInterface.EffectiveDescentCertificate.accepts_glues_or_breaks
    C accepts_yields_descent hAccept

end Interface

end HiddenCommittedQuotientLink
end Descender
