import Mathlib.Tactic
import Descender.SignatureTranscript
import Descender.LocalDeltaByteCommitment

namespace Descender

/-!
# Committed Sigma Proof Surface

This module starts the replacement proof architecture for the stateful
signature.  The old representative verifier is intentionally not imported as a
theorem dependency: it remains a control path and a negative-test source.

The deployed object we want to reason about has three hidden-witness layers:

* a slot witness committed in account/key state,
* a prechallenge randomizer committed before Fiat-Shamir challenge derivation,
* a response/opening proof tying `z = r + c*w` to the same hidden objects.

The critical security boundary is that a verifier must not accept merely
because some public post-challenge representative satisfies the linear quotient
equation.  Acceptance must be backed by a prechallenge witness/randomizer
commitment, or one of the named binding/proof assumptions is broken.
-/

namespace CommittedSigma

open SignatureTranscript

/--
Abstract linear sigma algebra.

`relationMap` is the public linear map checked by the verifier.  `randomizerMap`
is the first-message image `a`, and `witnessDigest` is the public statement
digest.  `responseOf` is the honest response constructor.  The field
`honest_response_equation` is the algebraic completeness equation:

```text
L(responseOf r c w) = A(r) + c * digest(w)
```
-/
structure LinearSigmaAlgebra
    (Witness Randomizer Challenge Response Digest : Type*) where
  relationMap : Response -> Digest
  randomizerMap : Randomizer -> Digest
  witnessDigest : Witness -> Digest
  add : Digest -> Digest -> Digest
  scale : Challenge -> Digest -> Digest
  responseOf : Randomizer -> Challenge -> Witness -> Response
  honest_response_equation :
    ∀ r c w,
      relationMap (responseOf r c w) =
        add (randomizerMap r) (scale c (witnessDigest w))

namespace LinearSigmaAlgebra

variable {Witness Randomizer Challenge Response Digest : Type*}

def verifierEquation
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response Digest)
    (proofCommitment : Digest) (challenge : Challenge)
    (statementDigest : Digest) (response : Response) : Prop :=
  A.relationMap response = A.add proofCommitment (A.scale challenge statementDigest)

theorem honest_verifierEquation
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response Digest)
    (r : Randomizer) (c : Challenge) (w : Witness) :
    A.verifierEquation (A.randomizerMap r) c (A.witnessDigest w)
      (A.responseOf r c w) := by
  exact A.honest_response_equation r c w

end LinearSigmaAlgebra

/--
Commitment interface used by the new proof.

This module keeps the commitment assumption abstract.  Concrete byte/hash
instantiations should reuse `LocalDeltaByteCommitment.ByteHashBinding` or a
more specialized hash-binding game, then instantiate this interface.
-/
structure BindingCommitment (Hidden Commitment : Type*) where
  commit : Hidden -> Commitment
  bindingBreak : Prop
  eq_of_same_commit_noBreak :
    ¬ bindingBreak -> ∀ {x y}, commit x = commit y -> x = y

namespace BindingCommitment

variable {Hidden Commitment : Type*}

theorem same_commit_eq
    (C : BindingCommitment Hidden Commitment)
    (hNoBreak : ¬ C.bindingBreak)
    {x y : Hidden} (h : C.commit x = C.commit y) :
    x = y :=
  C.eq_of_same_commit_noBreak hNoBreak h

end BindingCommitment

/-- Public account/key-state slot for the committed-sigma architecture. -/
structure Slot
    (StatementDigest WitnessCommitment : Type*) where
  statementDigest : StatementDigest
  witnessCommitment : WitnessCommitment

/-- Prechallenge first message bound into the Fiat-Shamir frame. -/
structure Prechallenge
    (ProofCommitment RandomizerCommitment : Type*) where
  proofCommitment : ProofCommitment
  randomizerCommitment : RandomizerCommitment

/--
The signature transcript shape after Fiat-Shamir challenge derivation.

`opening` is intentionally abstract.  For the local-delta construction it will
package the active local openings and any proof that they are tied to the same
hidden response/randomizer.
-/
structure Transcript
    (StatementDigest WitnessCommitment ProofCommitment RandomizerCommitment
      Challenge Response Opening : Type*) where
  slot : Slot StatementDigest WitnessCommitment
  prechallenge : Prechallenge ProofCommitment RandomizerCommitment
  challenge : Challenge
  response : Response
  opening : Opening

/--
A transcript is backed when the same hidden witness and prechallenge
randomizer explain every public field.

The `openingRelation` argument is where the local-delta proof plugs in.  It is
allowed to inspect the transcript and the hidden objects, but a deployed proof
must later show that the serialized verifier enforces it without revealing the
hidden objects.
-/
def BackedByHiddenWitness
    {Witness Randomizer StatementDigest WitnessCommitment ProofCommitment
      RandomizerCommitment Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (witnessCommit : Witness -> WitnessCommitment)
    (randomizerCommit : Randomizer -> RandomizerCommitment)
    (proofCommitmentOfRandomizer : Randomizer -> ProofCommitment)
    (openingRelation :
      Transcript StatementDigest WitnessCommitment ProofCommitment
        RandomizerCommitment Challenge Response Opening ->
      Witness -> Randomizer -> Prop)
    (T :
      Transcript StatementDigest WitnessCommitment ProofCommitment
        RandomizerCommitment Challenge Response Opening) : Prop :=
  ∃ w r,
    T.slot.statementDigest = A.witnessDigest w ∧
    T.slot.witnessCommitment = witnessCommit w ∧
    T.prechallenge.proofCommitment = proofCommitmentOfRandomizer r ∧
    T.prechallenge.randomizerCommitment = randomizerCommit r ∧
    T.response = A.responseOf r T.challenge w ∧
    openingRelation T w r

/--
Certificate that revealing the prechallenge randomizer makes the witness
recoverable from the public sigma response.

For ordinary linear sigma responses this is the algebra

```text
w = c^{-1} * (z - r)
```

whenever the challenge action is invertible.  A byte-level opening that reveals
`r` is therefore not a hiding proof; it turns every accepted backed transcript
into a public witness recovery transcript.
-/
structure RevealedRandomizerWitnessRecovery
    {Witness Randomizer Challenge Response Digest : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response Digest) where
  recoverWitness : Randomizer -> Challenge -> Response -> Witness
  recovers_honest :
    ∀ r c w, recoverWitness r c (A.responseOf r c w) = w

namespace RevealedRandomizerWitnessRecovery

variable {Witness Randomizer StatementDigest WitnessCommitment ProofCommitment
  RandomizerCommitment Challenge Response Opening : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {witnessCommit : Witness -> WitnessCommitment}
variable {randomizerCommit : Randomizer -> RandomizerCommitment}
variable {proofCommitmentOfRandomizer : Randomizer -> ProofCommitment}
variable {openingRelation :
  Transcript StatementDigest WitnessCommitment ProofCommitment
    RandomizerCommitment Challenge Response Opening ->
  Witness -> Randomizer -> Prop}

theorem recovers_from_backed_transcript
    (R : RevealedRandomizerWitnessRecovery A)
    {T :
      Transcript StatementDigest WitnessCommitment ProofCommitment
        RandomizerCommitment Challenge Response Opening}
    (hbacked :
      BackedByHiddenWitness A witnessCommit randomizerCommit
        proofCommitmentOfRandomizer openingRelation T) :
    ∃ w r,
      T.slot.statementDigest = A.witnessDigest w ∧
      T.slot.witnessCommitment = witnessCommit w ∧
      T.prechallenge.proofCommitment = proofCommitmentOfRandomizer r ∧
      T.prechallenge.randomizerCommitment = randomizerCommit r ∧
      R.recoverWitness r T.challenge T.response = w := by
  rcases hbacked with
    ⟨w, r, hDigest, hWitnessCommit, hProofCommit, hRandomizerCommit,
      hResponse, _hOpening⟩
  refine ⟨w, r, hDigest, hWitnessCommit, hProofCommit, hRandomizerCommit, ?_⟩
  rw [hResponse]
  exact R.recovers_honest r T.challenge w

end RevealedRandomizerWitnessRecovery

/--
Verifier predicate for the public byte path.

The important design constraint is that this predicate must be stronger than
the public equation alone.  The `openingAccepts` predicate is the public
serialized opening proof, and `prechallengeWellFormed` is where canonical
packing, commitment-domain agreement, and slot-state checks live.
-/
def VerifierAccepts
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (prechallengeWellFormed :
      Prechallenge StatementDigest RandomizerCommitment -> Prop)
    (openingAccepts :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening -> Prop)
    (T :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening) : Prop :=
  prechallengeWellFormed T.prechallenge ∧
  A.verifierEquation T.prechallenge.proofCommitment T.challenge
    T.slot.statementDigest T.response ∧
  openingAccepts T

/--
Named failure events for the new proof target.

The old public quotient verifier failed because `postChallengeRepresentative`
was possible: after seeing the challenge, an attacker could solve the public
linear equations and then choose an accepted local representative.
-/
structure SoundnessBreaks where
  witnessCommitmentBindingBreak : Prop
  randomizerCommitmentBindingBreak : Prop
  openingProofBreak : Prop
  fiatShamirFrameBreak : Prop
  postChallengeRepresentativeBreak : Prop

namespace SoundnessBreaks

def any (B : SoundnessBreaks) : Prop :=
  B.witnessCommitmentBindingBreak ∨
  B.randomizerCommitmentBindingBreak ∨
  B.openingProofBreak ∨
  B.fiatShamirFrameBreak ∨
  B.postChallengeRepresentativeBreak

theorem any_of_openingProofBreak
    {B : SoundnessBreaks} (h : B.openingProofBreak) :
    B.any := by
  unfold any
  tauto

theorem any_of_postChallengeRepresentativeBreak
    {B : SoundnessBreaks} (h : B.postChallengeRepresentativeBreak) :
    B.any := by
  unfold any
  tauto

end SoundnessBreaks

/--
Main abstract review certificate for the replacement proof.

This is the first theorem target: every accepting serialized transcript is
either backed by hidden witness/randomizer material committed before the
challenge, or it triggers a named break.  Concrete byte-level work should
instantiate `accepting_implies_backed_or_break`, not weaken the verifier to the
public equation alone.
-/
structure CommittedSigmaSoundnessCertificate
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (witnessCommit : Witness -> WitnessCommitment)
    (randomizerCommit : Randomizer -> RandomizerCommitment)
    (proofCommitmentOfRandomizer : Randomizer -> StatementDigest)
    (prechallengeWellFormed :
      Prechallenge StatementDigest RandomizerCommitment -> Prop)
    (openingAccepts :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening -> Prop)
    (openingRelation :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening ->
      Witness -> Randomizer -> Prop)
    where
  breaks : SoundnessBreaks
  accepting_implies_backed_or_break :
    ∀ T,
      VerifierAccepts A prechallengeWellFormed openingAccepts T ->
        BackedByHiddenWitness A witnessCommit randomizerCommit
          proofCommitmentOfRandomizer openingRelation T ∨
        breaks.any

namespace CommittedSigmaSoundnessCertificate

variable {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
  Challenge Response Opening : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {witnessCommit : Witness -> WitnessCommitment}
variable {randomizerCommit : Randomizer -> RandomizerCommitment}
variable {proofCommitmentOfRandomizer : Randomizer -> StatementDigest}
variable {prechallengeWellFormed :
  Prechallenge StatementDigest RandomizerCommitment -> Prop}
variable {openingAccepts :
  Transcript StatementDigest WitnessCommitment StatementDigest
    RandomizerCommitment Challenge Response Opening -> Prop}
variable {openingRelation :
  Transcript StatementDigest WitnessCommitment StatementDigest
    RandomizerCommitment Challenge Response Opening ->
  Witness -> Randomizer -> Prop}

theorem accepted_backed_or_break
    (C :
      CommittedSigmaSoundnessCertificate A witnessCommit randomizerCommit
        proofCommitmentOfRandomizer prechallengeWellFormed openingAccepts
        openingRelation)
    (T :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening)
    (hAccept : VerifierAccepts A prechallengeWellFormed openingAccepts T) :
    BackedByHiddenWitness A witnessCommit randomizerCommit
      proofCommitmentOfRandomizer openingRelation T ∨
      C.breaks.any :=
  C.accepting_implies_backed_or_break T hAccept

theorem accepted_backed_of_no_breaks
    (C :
      CommittedSigmaSoundnessCertificate A witnessCommit randomizerCommit
        proofCommitmentOfRandomizer prechallengeWellFormed openingAccepts
        openingRelation)
    (hNoBreaks : ¬ C.breaks.any)
    (T :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening)
    (hAccept : VerifierAccepts A prechallengeWellFormed openingAccepts T) :
    BackedByHiddenWitness A witnessCommit randomizerCommit
      proofCommitmentOfRandomizer openingRelation T := by
  rcases C.accepted_backed_or_break T hAccept with hbacked | hbreak
  · exact hbacked
  · exact False.elim (hNoBreaks hbreak)

end CommittedSigmaSoundnessCertificate

/--
Minimal interface for the new prechallenge-randomizer opening proof.

This is the structural gate missing from the representative verifier.  It says
that once the public sigma equation is true, the serialized opening proof must
still force the transcript to be backed by committed hidden witness/randomizer
material, unless the opening proof system itself is broken.

The interface is deliberately agnostic about how the proof is implemented:
hash-preimage proof, Merkle-authenticated stateful pool, specialized algebraic
proof, or another construction.  What matters for soundness is the
`accepts_backed_or_break` field.
-/
structure PrechallengeRandomizerOpeningInterface
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (prechallengeWellFormed :
      Prechallenge StatementDigest RandomizerCommitment -> Prop)
    where
  witnessCommit : Witness -> WitnessCommitment
  randomizerCommit : Randomizer -> RandomizerCommitment
  proofCommitmentOfRandomizer : Randomizer -> StatementDigest
  openingAccepts :
    Transcript StatementDigest WitnessCommitment StatementDigest
      RandomizerCommitment Challenge Response Opening -> Prop
  openingRelation :
    Transcript StatementDigest WitnessCommitment StatementDigest
      RandomizerCommitment Challenge Response Opening ->
    Witness -> Randomizer -> Prop
  openingProofBreak : Prop
  accepts_backed_or_break :
    ∀ T,
      prechallengeWellFormed T.prechallenge ->
      A.verifierEquation T.prechallenge.proofCommitment T.challenge
        T.slot.statementDigest T.response ->
      openingAccepts T ->
        BackedByHiddenWitness A witnessCommit randomizerCommit
          proofCommitmentOfRandomizer openingRelation T ∨
        openingProofBreak

namespace PrechallengeRandomizerOpeningInterface

variable {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
  Challenge Response Opening : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {prechallengeWellFormed :
  Prechallenge StatementDigest RandomizerCommitment -> Prop}

def breaks
    (I :
      PrechallengeRandomizerOpeningInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A prechallengeWellFormed) :
    SoundnessBreaks where
  witnessCommitmentBindingBreak := False
  randomizerCommitmentBindingBreak := False
  openingProofBreak := I.openingProofBreak
  fiatShamirFrameBreak := False
  postChallengeRepresentativeBreak := False

theorem accepts_backed_or_openingBreak
    (I :
      PrechallengeRandomizerOpeningInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A prechallengeWellFormed)
    (T :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening)
    (hAccept : VerifierAccepts A prechallengeWellFormed I.openingAccepts T) :
    BackedByHiddenWitness A I.witnessCommit I.randomizerCommit
      I.proofCommitmentOfRandomizer I.openingRelation T ∨
      I.openingProofBreak :=
  I.accepts_backed_or_break T hAccept.1 hAccept.2.1 hAccept.2.2

def toSoundnessCertificate
    (I :
      PrechallengeRandomizerOpeningInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A prechallengeWellFormed) :
    CommittedSigmaSoundnessCertificate A I.witnessCommit I.randomizerCommit
      I.proofCommitmentOfRandomizer prechallengeWellFormed
      I.openingAccepts I.openingRelation where
  breaks := I.breaks
  accepting_implies_backed_or_break := by
    intro T hAccept
    rcases I.accepts_backed_or_openingBreak T hAccept with hbacked | hbreak
    · exact Or.inl hbacked
    · exact Or.inr (SoundnessBreaks.any_of_openingProofBreak hbreak)

theorem verifier_accepts_backed_of_no_openingBreak
    (I :
      PrechallengeRandomizerOpeningInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A prechallengeWellFormed)
    (hNoBreak : ¬ I.openingProofBreak)
    (T :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening)
    (hAccept : VerifierAccepts A prechallengeWellFormed I.openingAccepts T) :
    BackedByHiddenWitness A I.witnessCommit I.randomizerCommit
      I.proofCommitmentOfRandomizer I.openingRelation T := by
  rcases I.accepts_backed_or_openingBreak T hAccept with hbacked | hbreak
  · exact hbacked
  · exact False.elim (hNoBreak hbreak)

end PrechallengeRandomizerOpeningInterface

/--
Account slot state for the low-byte stateful route.

The generic `Slot` only carries the public statement digest and witness
commitment.  The account-carried construction needs persistent state that also
binds the prechallenge randomizer and the hidden witness/randomizer link.  These
fields are consumed with the slot, so they are paid in account state rather than
in every signature.
-/
structure AccountCarriedSlot
    (StatementDigest WitnessCommitment RandomizerCommitment LinkCommitment :
      Type*) where
  statementDigest : StatementDigest
  witnessCommitment : WitnessCommitment
  randomizerCommitment : RandomizerCommitment
  hiddenLinkCommitment : LinkCommitment

namespace AccountCarriedSlot

def toSlot
    {StatementDigest WitnessCommitment RandomizerCommitment LinkCommitment :
      Type*}
    (S :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment) :
    Slot StatementDigest WitnessCommitment where
  statementDigest := S.statementDigest
  witnessCommitment := S.witnessCommitment

end AccountCarriedSlot

/--
Serialized transcript shape for the account-carried hidden-link route.

The proof commitment `a = L(r)` is still included in the signature and bound into
Fiat-Shamir, while the randomizer and link commitments are read from the consumed
account slot.
-/
structure AccountCarriedTranscript
    (StatementDigest WitnessCommitment ProofCommitment RandomizerCommitment
      LinkCommitment Challenge Response Opening : Type*) where
  accountSlot :
    AccountCarriedSlot StatementDigest WitnessCommitment
      RandomizerCommitment LinkCommitment
  proofCommitment : ProofCommitment
  challenge : Challenge
  response : Response
  opening : Opening

namespace AccountCarriedTranscript

def toTranscript
    {StatementDigest WitnessCommitment ProofCommitment RandomizerCommitment
      LinkCommitment Challenge Response Opening : Type*}
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        ProofCommitment RandomizerCommitment LinkCommitment Challenge Response
        Opening) :
    Transcript StatementDigest WitnessCommitment ProofCommitment
      RandomizerCommitment Challenge Response Opening where
  slot := T.accountSlot.toSlot
  prechallenge :=
    { proofCommitment := T.proofCommitment
      randomizerCommitment := T.accountSlot.randomizerCommitment }
  challenge := T.challenge
  response := T.response
  opening := T.opening

end AccountCarriedTranscript

/--
The account slot precommits the sigma first message.

This is a necessary low-byte stateful invariant: the proof commitment `a` in the
signature must be the same first-message value reserved in the consumed slot.
It prevents choosing `a` after seeing the message/challenge frame, but it is not
by itself a hidden-link proof.
-/
def SlotBindsProofCommitment
    {StatementDigest WitnessCommitment LinkCommitment Challenge Response
      Opening : Type*}
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest StatementDigest LinkCommitment Challenge Response
        Opening) : Prop :=
  T.accountSlot.randomizerCommitment = T.proofCommitment

/--
Backed account-carried transcript.

Compared with `BackedByHiddenWitness`, this additionally requires the account
slot's randomizer and hidden-link commitments to be explained by the same hidden
`w, r`.
-/
def AccountBackedByHiddenWitness
    {Witness Randomizer StatementDigest WitnessCommitment ProofCommitment
      RandomizerCommitment LinkCommitment Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (witnessCommit : Witness -> WitnessCommitment)
    (randomizerCommit : Randomizer -> RandomizerCommitment)
    (proofCommitmentOfRandomizer : Randomizer -> ProofCommitment)
    (hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment)
    (openingRelation :
      Transcript StatementDigest WitnessCommitment ProofCommitment
        RandomizerCommitment Challenge Response Opening ->
      Witness -> Randomizer -> Prop)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment ProofCommitment
        RandomizerCommitment LinkCommitment Challenge Response Opening) : Prop :=
  ∃ w r,
    T.accountSlot.statementDigest = A.witnessDigest w ∧
    T.accountSlot.witnessCommitment = witnessCommit w ∧
    T.accountSlot.randomizerCommitment = randomizerCommit r ∧
    T.accountSlot.hiddenLinkCommitment = hiddenLinkCommit w r ∧
    T.proofCommitment = proofCommitmentOfRandomizer r ∧
    T.response = A.responseOf r T.challenge w ∧
    openingRelation T.toTranscript w r

/--
Account-carried backing before the response link has been proved.

This is the useful intermediate target for the zero-signature-byte homomorphic
route.  The account slot and local opening identify the same hidden `w,r`, but
the final equality `response = responseOf r c w` is deferred to the deterministic
homomorphic link/kernel argument.
-/
def AccountBackedExceptResponse
    {Witness Randomizer StatementDigest WitnessCommitment ProofCommitment
      RandomizerCommitment LinkCommitment Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (witnessCommit : Witness -> WitnessCommitment)
    (randomizerCommit : Randomizer -> RandomizerCommitment)
    (proofCommitmentOfRandomizer : Randomizer -> ProofCommitment)
    (hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment)
    (openingRelation :
      Transcript StatementDigest WitnessCommitment ProofCommitment
        RandomizerCommitment Challenge Response Opening ->
      Witness -> Randomizer -> Prop)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment ProofCommitment
        RandomizerCommitment LinkCommitment Challenge Response Opening) : Prop :=
  ∃ w r,
    T.accountSlot.statementDigest = A.witnessDigest w ∧
    T.accountSlot.witnessCommitment = witnessCommit w ∧
    T.accountSlot.randomizerCommitment = randomizerCommit r ∧
    T.accountSlot.hiddenLinkCommitment = hiddenLinkCommit w r ∧
    T.proofCommitment = proofCommitmentOfRandomizer r ∧
    openingRelation T.toTranscript w r

/--
The response-link obligation for a deterministic account-carried link.

Given already-backed account fields for `w,r`, the public link check must force
the response to be the sigma response for those same hidden values, or else a
named link-kernel break has occurred.
-/
def ResponseLinkSoundness
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (witnessCommit : Witness -> WitnessCommitment)
    (randomizerCommit : Randomizer -> RandomizerCommitment)
    (proofCommitmentOfRandomizer : Randomizer -> StatementDigest)
    (hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment)
    (openingRelation :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening ->
      Witness -> Randomizer -> Prop)
    (linkProofAccepts :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response Opening -> Prop)
    (linkKernelBreak : Prop) : Prop :=
  ∀ T w r,
    T.accountSlot.statementDigest = A.witnessDigest w ->
    T.accountSlot.witnessCommitment = witnessCommit w ->
    T.accountSlot.randomizerCommitment = randomizerCommit r ->
    T.accountSlot.hiddenLinkCommitment = hiddenLinkCommit w r ->
    T.proofCommitment = proofCommitmentOfRandomizer r ->
    openingRelation T.toTranscript w r ->
    A.verifierEquation T.proofCommitment T.challenge
      T.accountSlot.statementDigest T.response ->
    linkProofAccepts T ->
    T.response = A.responseOf r T.challenge w ∨ linkKernelBreak

/--
Bounded joint-kernel certificate for deterministic response-link codes.

The concrete `[0, 3]` code search is evidence for this shape: if an accepted
response and the honest response for the same hidden `w,r` are both in the
allowed response-difference box, and they have the same public quotient image
and deterministic link-code image, then they are equal unless the bounded
link-kernel assumption breaks.
-/
structure BoundedResponseLinkKernelCertificate
    {Witness Randomizer Challenge Response StatementDigest LinkCommitment : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (responseCommit : Response -> LinkCommitment)
    (linkKernelBreak : Prop) where
  inResponseDifferenceBox : Response -> Response -> Prop
  eq_or_break_of_same_maps_in_box :
    ∀ {z expected},
      A.relationMap z = A.relationMap expected ->
      responseCommit z = responseCommit expected ->
      inResponseDifferenceBox z expected ->
      z = expected ∨ linkKernelBreak

namespace AccountBackedByHiddenWitness

variable {Witness Randomizer StatementDigest WitnessCommitment ProofCommitment
  RandomizerCommitment LinkCommitment Challenge Response Opening : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {witnessCommit : Witness -> WitnessCommitment}
variable {randomizerCommit : Randomizer -> RandomizerCommitment}
variable {proofCommitmentOfRandomizer : Randomizer -> ProofCommitment}
variable {hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment}
variable {openingRelation :
  Transcript StatementDigest WitnessCommitment ProofCommitment
    RandomizerCommitment Challenge Response Opening ->
  Witness -> Randomizer -> Prop}

theorem toBackedByHiddenWitness
    {T :
      AccountCarriedTranscript StatementDigest WitnessCommitment ProofCommitment
        RandomizerCommitment LinkCommitment Challenge Response Opening}
    (h :
      AccountBackedByHiddenWitness A witnessCommit randomizerCommit
        proofCommitmentOfRandomizer hiddenLinkCommit openingRelation T) :
    BackedByHiddenWitness A witnessCommit randomizerCommit
      proofCommitmentOfRandomizer openingRelation T.toTranscript := by
  rcases h with
    ⟨w, r, hDigest, hWitnessCommit, hRandomizerCommit, hLinkCommit,
      hProofCommit, hResponse, hOpening⟩
  refine
    ⟨w, r, hDigest, hWitnessCommit, hProofCommit, hRandomizerCommit,
      hResponse, hOpening⟩

theorem slotBindsProofCommitment_of_randomizerCommit_eq_proofCommitment
    {T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest StatementDigest LinkCommitment Challenge Response
        Opening}
    {randomizerCommit' : Randomizer -> StatementDigest}
    {proofCommitmentOfRandomizer' : Randomizer -> StatementDigest}
    {hiddenLinkCommit' : Witness -> Randomizer -> LinkCommitment}
    {openingRelation' :
      Transcript StatementDigest WitnessCommitment StatementDigest
        StatementDigest Challenge Response Opening ->
      Witness -> Randomizer -> Prop}
    (hCommit :
      ∀ r, randomizerCommit' r = proofCommitmentOfRandomizer' r)
    (h :
      AccountBackedByHiddenWitness A witnessCommit randomizerCommit'
        proofCommitmentOfRandomizer' hiddenLinkCommit' openingRelation' T) :
    SlotBindsProofCommitment T := by
  rcases h with
    ⟨_w, r, _hDigest, _hWitnessCommit, hRandomizerCommit, _hLinkCommit,
      hProofCommit, _hResponse, _hOpening⟩
  rw [SlotBindsProofCommitment, hRandomizerCommit, hProofCommit, hCommit r]

end AccountBackedByHiddenWitness

namespace AccountBackedExceptResponse

variable {Witness Randomizer StatementDigest WitnessCommitment
  RandomizerCommitment LinkCommitment Challenge Response Opening : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {witnessCommit : Witness -> WitnessCommitment}
variable {randomizerCommit : Randomizer -> RandomizerCommitment}
variable {proofCommitmentOfRandomizer : Randomizer -> StatementDigest}
variable {hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment}
variable {openingRelation :
  Transcript StatementDigest WitnessCommitment StatementDigest
    RandomizerCommitment Challenge Response Opening ->
  Witness -> Randomizer -> Prop}

theorem accountBacked_or_linkKernelBreak
    {linkProofAccepts :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response Opening -> Prop}
    {linkKernelBreak : Prop}
    {T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response Opening}
    (hPre :
      AccountBackedExceptResponse A witnessCommit randomizerCommit
        proofCommitmentOfRandomizer hiddenLinkCommit openingRelation T)
    (hLink : linkProofAccepts T)
    (hSound :
      ResponseLinkSoundness A witnessCommit randomizerCommit
        proofCommitmentOfRandomizer hiddenLinkCommit openingRelation
        linkProofAccepts linkKernelBreak) :
    A.verifierEquation T.proofCommitment T.challenge
      T.accountSlot.statementDigest T.response ->
    AccountBackedByHiddenWitness A witnessCommit randomizerCommit
      proofCommitmentOfRandomizer hiddenLinkCommit openingRelation T ∨
    linkKernelBreak := by
  intro hEquation
  rcases hPre with ⟨w, r, hs, hw, hr, hh, hp, ho⟩
  rcases hSound T w r hs hw hr hh hp ho hEquation hLink with
    hResponse | hBreak
  · exact Or.inl ⟨w, r, hs, hw, hr, hh, hp, hResponse, ho⟩
  · exact Or.inr hBreak

end AccountBackedExceptResponse

namespace BoundedResponseLinkKernelCertificate

variable {Witness Randomizer StatementDigest WitnessCommitment
  RandomizerCommitment HiddenLinkCommitment Challenge Response Opening
  LinkCommitment : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {witnessCommit : Witness -> WitnessCommitment}
variable {randomizerCommit : Randomizer -> RandomizerCommitment}
variable {proofCommitmentOfRandomizer : Randomizer -> StatementDigest}
variable {hiddenLinkCommit : Witness -> Randomizer -> HiddenLinkCommitment}
variable {openingRelation :
  Transcript StatementDigest WitnessCommitment StatementDigest
    RandomizerCommitment Challenge Response Opening ->
  Witness -> Randomizer -> Prop}

theorem responseLinkSoundness
    {responseCommit : Response -> LinkCommitment}
    {linkProofAccepts :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment HiddenLinkCommitment Challenge Response Opening ->
      Prop}
    {linkKernelBreak : Prop}
    (K : BoundedResponseLinkKernelCertificate A responseCommit linkKernelBreak)
    (hProofCommit :
      ∀ r, proofCommitmentOfRandomizer r = A.randomizerMap r)
    (hLinkEquation :
      ∀ T w r,
        T.accountSlot.statementDigest = A.witnessDigest w ->
        T.accountSlot.witnessCommitment = witnessCommit w ->
        T.accountSlot.randomizerCommitment = randomizerCommit r ->
        T.accountSlot.hiddenLinkCommitment = hiddenLinkCommit w r ->
        T.proofCommitment = proofCommitmentOfRandomizer r ->
        openingRelation T.toTranscript w r ->
        linkProofAccepts T ->
        responseCommit T.response =
          responseCommit (A.responseOf r T.challenge w))
    (hDifferenceBox :
      ∀ T w r,
        T.accountSlot.statementDigest = A.witnessDigest w ->
        T.accountSlot.witnessCommitment = witnessCommit w ->
        T.accountSlot.randomizerCommitment = randomizerCommit r ->
        T.accountSlot.hiddenLinkCommitment = hiddenLinkCommit w r ->
        T.proofCommitment = proofCommitmentOfRandomizer r ->
        openingRelation T.toTranscript w r ->
        A.verifierEquation T.proofCommitment T.challenge
          T.accountSlot.statementDigest T.response ->
        linkProofAccepts T ->
        K.inResponseDifferenceBox T.response
          (A.responseOf r T.challenge w)) :
    ResponseLinkSoundness A witnessCommit randomizerCommit
      proofCommitmentOfRandomizer hiddenLinkCommit openingRelation
      linkProofAccepts linkKernelBreak := by
  intro T w r hs hw hr hh hp ho hEquation hLink
  have hRelation :
      A.relationMap T.response =
        A.relationMap (A.responseOf r T.challenge w) := by
    unfold LinearSigmaAlgebra.verifierEquation at hEquation
    calc
      A.relationMap T.response =
          A.add T.proofCommitment
            (A.scale T.challenge T.accountSlot.statementDigest) := hEquation
      _ = A.add (proofCommitmentOfRandomizer r)
            (A.scale T.challenge T.accountSlot.statementDigest) := by
              rw [hp]
      _ = A.add (A.randomizerMap r)
            (A.scale T.challenge T.accountSlot.statementDigest) := by
              rw [hProofCommit r]
      _ = A.add (A.randomizerMap r)
            (A.scale T.challenge (A.witnessDigest w)) := by
              rw [hs]
      _ = A.relationMap (A.responseOf r T.challenge w) := by
              exact (A.honest_response_equation r T.challenge w).symm
  have hCommit :
      responseCommit T.response =
        responseCommit (A.responseOf r T.challenge w) :=
    hLinkEquation T w r hs hw hr hh hp ho hLink
  exact
    K.eq_or_break_of_same_maps_in_box hRelation hCommit
      (hDifferenceBox T w r hs hw hr hh hp ho hEquation hLink)

end BoundedResponseLinkKernelCertificate

/--
Minimal account-carried hidden-link interface.

This is the concrete theorem shape for the 256-byte stateful target: if the
account slot is well-formed, the sigma equation holds, and the public opening
accepts, then the consumed account slot is backed by a single hidden witness and
prechallenge randomizer, unless the account-link proof breaks.
-/
structure AccountCarriedHiddenLinkInterface
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop)
    where
  witnessCommit : Witness -> WitnessCommitment
  randomizerCommit : Randomizer -> RandomizerCommitment
  hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment
  proofCommitmentOfRandomizer : Randomizer -> StatementDigest
  openingAccepts :
    Transcript StatementDigest WitnessCommitment StatementDigest
      RandomizerCommitment Challenge Response Opening -> Prop
  openingRelation :
    Transcript StatementDigest WitnessCommitment StatementDigest
      RandomizerCommitment Challenge Response Opening ->
    Witness -> Randomizer -> Prop
  accountLinkProofBreak : Prop
  accepts_accountBacked_or_break :
    ∀ T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening,
      accountSlotWellFormed T.accountSlot ->
      A.verifierEquation T.proofCommitment T.challenge
        T.accountSlot.statementDigest T.response ->
      openingAccepts T.toTranscript ->
        AccountBackedByHiddenWitness A witnessCommit randomizerCommit
          proofCommitmentOfRandomizer hiddenLinkCommit openingRelation T ∨
        accountLinkProofBreak

namespace AccountCarriedHiddenLinkInterface

variable {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
  LinkCommitment Challenge Response Opening : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {accountSlotWellFormed :
  AccountCarriedSlot StatementDigest WitnessCommitment RandomizerCommitment
    LinkCommitment -> Prop}

def prechallengeWellFormed
    (_I :
      AccountCarriedHiddenLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A accountSlotWellFormed) :
    Prechallenge StatementDigest RandomizerCommitment -> Prop :=
  fun _ => True

theorem accepts_backed_or_break
    (I :
      AccountCarriedHiddenLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A accountSlotWellFormed)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening)
    (hSlot : accountSlotWellFormed T.accountSlot)
    (hEquation :
      A.verifierEquation T.proofCommitment T.challenge
        T.accountSlot.statementDigest T.response)
    (hOpening : I.openingAccepts T.toTranscript) :
    BackedByHiddenWitness A I.witnessCommit I.randomizerCommit
      I.proofCommitmentOfRandomizer I.openingRelation T.toTranscript ∨
      I.accountLinkProofBreak := by
  rcases I.accepts_accountBacked_or_break T hSlot hEquation hOpening with
    hbacked | hbreak
  · exact Or.inl hbacked.toBackedByHiddenWitness
  · exact Or.inr hbreak

theorem accepts_accountBacked_of_noBreak
    (I :
      AccountCarriedHiddenLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A accountSlotWellFormed)
    (hNoBreak : ¬ I.accountLinkProofBreak)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening)
    (hSlot : accountSlotWellFormed T.accountSlot)
    (hEquation :
      A.verifierEquation T.proofCommitment T.challenge
        T.accountSlot.statementDigest T.response)
    (hOpening : I.openingAccepts T.toTranscript) :
    AccountBackedByHiddenWitness A I.witnessCommit I.randomizerCommit
      I.proofCommitmentOfRandomizer I.hiddenLinkCommit I.openingRelation T := by
  rcases I.accepts_accountBacked_or_break T hSlot hEquation hOpening with
    hbacked | hbreak
  · exact hbacked
  · exact False.elim (hNoBreak hbreak)

theorem unbacked_acceptance_forces_break
    (I :
      AccountCarriedHiddenLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A accountSlotWellFormed)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening)
    (hSlot : accountSlotWellFormed T.accountSlot)
    (hEquation :
      A.verifierEquation T.proofCommitment T.challenge
        T.accountSlot.statementDigest T.response)
    (hOpening : I.openingAccepts T.toTranscript)
    (hUnbacked :
      ¬ AccountBackedByHiddenWitness A I.witnessCommit I.randomizerCommit
        I.proofCommitmentOfRandomizer I.hiddenLinkCommit I.openingRelation T) :
    I.accountLinkProofBreak := by
  rcases I.accepts_accountBacked_or_break T hSlot hEquation hOpening with
    hbacked | hbreak
  · exact False.elim (hUnbacked hbacked)
  · exact hbreak

theorem noBreak_rejects_unbacked_acceptance
    (I :
      AccountCarriedHiddenLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A accountSlotWellFormed)
    (hNoBreak : ¬ I.accountLinkProofBreak)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening)
    (hUnbacked :
      ¬ AccountBackedByHiddenWitness A I.witnessCommit I.randomizerCommit
        I.proofCommitmentOfRandomizer I.hiddenLinkCommit I.openingRelation T) :
    ¬ (accountSlotWellFormed T.accountSlot ∧
        A.verifierEquation T.proofCommitment T.challenge
          T.accountSlot.statementDigest T.response ∧
        I.openingAccepts T.toTranscript) := by
  intro hAccept
  exact hNoBreak
    (I.unbacked_acceptance_forces_break T hAccept.1 hAccept.2.1
      hAccept.2.2 hUnbacked)

end AccountCarriedHiddenLinkInterface

/-- Public verifier predicate for the account-carried stateful target. -/
def AccountCarriedVerifierAccepts
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop)
    (openingAccepts :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening -> Prop)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening) : Prop :=
  accountSlotWellFormed T.accountSlot ∧
  A.verifierEquation T.proofCommitment T.challenge
    T.accountSlot.statementDigest T.response ∧
  openingAccepts T.toTranscript

/--
Top-level account-carried soundness certificate.

This is the deployment theorem surface for the 256-byte stateful route.  It is
stronger than the generic committed-sigma certificate because the accepted
transcript must be backed by the consumed account slot's witness, randomizer,
and hidden-link commitments.
-/
structure AccountCarriedSoundnessCertificate
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (witnessCommit : Witness -> WitnessCommitment)
    (randomizerCommit : Randomizer -> RandomizerCommitment)
    (hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment)
    (proofCommitmentOfRandomizer : Randomizer -> StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop)
    (openingAccepts :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening -> Prop)
    (openingRelation :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening ->
      Witness -> Randomizer -> Prop)
    where
  breaks : SoundnessBreaks
  accepting_implies_accountBacked_or_break :
    ∀ T,
      AccountCarriedVerifierAccepts A accountSlotWellFormed openingAccepts T ->
        AccountBackedByHiddenWitness A witnessCommit randomizerCommit
          proofCommitmentOfRandomizer hiddenLinkCommit openingRelation T ∨
        breaks.any

namespace AccountCarriedSoundnessCertificate

variable {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
  LinkCommitment Challenge Response Opening : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {witnessCommit : Witness -> WitnessCommitment}
variable {randomizerCommit : Randomizer -> RandomizerCommitment}
variable {hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment}
variable {proofCommitmentOfRandomizer : Randomizer -> StatementDigest}
variable {accountSlotWellFormed :
  AccountCarriedSlot StatementDigest WitnessCommitment RandomizerCommitment
    LinkCommitment -> Prop}
variable {openingAccepts :
  Transcript StatementDigest WitnessCommitment StatementDigest
    RandomizerCommitment Challenge Response Opening -> Prop}
variable {openingRelation :
  Transcript StatementDigest WitnessCommitment StatementDigest
    RandomizerCommitment Challenge Response Opening ->
  Witness -> Randomizer -> Prop}

theorem accepted_accountBacked_or_break
    (C :
      AccountCarriedSoundnessCertificate A witnessCommit randomizerCommit
        hiddenLinkCommit proofCommitmentOfRandomizer accountSlotWellFormed
        openingAccepts openingRelation)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening)
    (hAccept :
      AccountCarriedVerifierAccepts A accountSlotWellFormed openingAccepts T) :
    AccountBackedByHiddenWitness A witnessCommit randomizerCommit
      proofCommitmentOfRandomizer hiddenLinkCommit openingRelation T ∨
      C.breaks.any :=
  C.accepting_implies_accountBacked_or_break T hAccept

theorem accepted_accountBacked_of_no_breaks
    (C :
      AccountCarriedSoundnessCertificate A witnessCommit randomizerCommit
        hiddenLinkCommit proofCommitmentOfRandomizer accountSlotWellFormed
        openingAccepts openingRelation)
    (hNoBreaks : ¬ C.breaks.any)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening)
    (hAccept :
      AccountCarriedVerifierAccepts A accountSlotWellFormed openingAccepts T) :
    AccountBackedByHiddenWitness A witnessCommit randomizerCommit
      proofCommitmentOfRandomizer hiddenLinkCommit openingRelation T := by
  rcases C.accepted_accountBacked_or_break T hAccept with hbacked | hbreak
  · exact hbacked
  · exact False.elim (hNoBreaks hbreak)

def fromHiddenLinkInterface
    (I :
      AccountCarriedHiddenLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A accountSlotWellFormed) :
    AccountCarriedSoundnessCertificate A I.witnessCommit I.randomizerCommit
      I.hiddenLinkCommit I.proofCommitmentOfRandomizer accountSlotWellFormed
      I.openingAccepts I.openingRelation where
  breaks :=
    { witnessCommitmentBindingBreak := False
      randomizerCommitmentBindingBreak := False
      openingProofBreak := I.accountLinkProofBreak
      fiatShamirFrameBreak := False
      postChallengeRepresentativeBreak := False }
  accepting_implies_accountBacked_or_break := by
    intro T hAccept
    rcases I.accepts_accountBacked_or_break T hAccept.1 hAccept.2.1
        hAccept.2.2 with hbacked | hbreak
    · exact Or.inl hbacked
    · exact Or.inr (SoundnessBreaks.any_of_openingProofBreak hbreak)

theorem fromHiddenLinkInterface_accepts_accountBacked_of_noBreak
    (I :
      AccountCarriedHiddenLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (Opening := Opening) A accountSlotWellFormed)
    (hNoBreak : ¬ I.accountLinkProofBreak)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening)
    (hAccept :
      AccountCarriedVerifierAccepts A accountSlotWellFormed I.openingAccepts T) :
    AccountBackedByHiddenWitness A I.witnessCommit I.randomizerCommit
      I.proofCommitmentOfRandomizer I.hiddenLinkCommit I.openingRelation T := by
  exact I.accepts_accountBacked_of_noBreak hNoBreak T hAccept.1 hAccept.2.1
    hAccept.2.2

end AccountCarriedSoundnessCertificate

/-- Opening payload split for account-carried designs. -/
structure AccountCarriedOpening (LocalOpening LinkProof : Type*) where
  localOpening : LocalOpening
  linkProof : LinkProof

namespace AccountCarriedOpening

def localTranscript
    {StatementDigest WitnessCommitment RandomizerCommitment Challenge Response
      LocalOpening LinkProof : Type*}
    (T :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof)) :
    Transcript StatementDigest WitnessCommitment StatementDigest
      RandomizerCommitment Challenge Response LocalOpening where
  slot := T.slot
  prechallenge := T.prechallenge
  challenge := T.challenge
  response := T.response
  opening := T.opening.localOpening

def localAccountTranscript
    {StatementDigest WitnessCommitment RandomizerCommitment LinkCommitment
      Challenge Response LocalOpening LinkProof : Type*}
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof)) :
    AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
      RandomizerCommitment LinkCommitment Challenge Response LocalOpening where
  accountSlot := T.accountSlot
  proofCommitment := T.proofCommitment
  challenge := T.challenge
  response := T.response
  opening := T.opening.localOpening

end AccountCarriedOpening

/--
Verifier predicate whose opening check is allowed to inspect the consumed
account slot.  This is the right shape for the hidden-link layer, because the
link proof must be checked against `hiddenLinkCommitment` in account state.
-/
def AccountCarriedVerifierAcceptsWithAccountOpening
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop)
    (accountOpeningAccepts :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response Opening -> Prop)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening) : Prop :=
  accountSlotWellFormed T.accountSlot ∧
  A.verifierEquation T.proofCommitment T.challenge
    T.accountSlot.statementDigest T.response ∧
  accountOpeningAccepts T

/--
Top-level account-carried soundness certificate for account-aware openings.

This is the deployment-facing version of `AccountCarriedSoundnessCertificate`
for the hidden-link design: the opening verifier is allowed to inspect the
consumed account slot, because the link proof is checked against account-carried
commitments.
-/
structure AccountCarriedAccountOpeningSoundnessCertificate
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response Opening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (witnessCommit : Witness -> WitnessCommitment)
    (randomizerCommit : Randomizer -> RandomizerCommitment)
    (hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment)
    (proofCommitmentOfRandomizer : Randomizer -> StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop)
    (accountOpeningAccepts :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response Opening -> Prop)
    (openingRelation :
      Transcript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment Challenge Response Opening ->
      Witness -> Randomizer -> Prop)
    where
  breaks : SoundnessBreaks
  accepting_implies_accountBacked_or_break :
    ∀ T,
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        accountOpeningAccepts T ->
        AccountBackedByHiddenWitness A witnessCommit randomizerCommit
          proofCommitmentOfRandomizer hiddenLinkCommit openingRelation T ∨
        breaks.any

namespace AccountCarriedAccountOpeningSoundnessCertificate

variable {Witness Randomizer StatementDigest WitnessCommitment
  RandomizerCommitment LinkCommitment Challenge Response Opening : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {witnessCommit : Witness -> WitnessCommitment}
variable {randomizerCommit : Randomizer -> RandomizerCommitment}
variable {hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment}
variable {proofCommitmentOfRandomizer : Randomizer -> StatementDigest}
variable {accountSlotWellFormed :
  AccountCarriedSlot StatementDigest WitnessCommitment RandomizerCommitment
    LinkCommitment -> Prop}
variable {accountOpeningAccepts :
  AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
    RandomizerCommitment LinkCommitment Challenge Response Opening -> Prop}
variable {openingRelation :
  Transcript StatementDigest WitnessCommitment StatementDigest
    RandomizerCommitment Challenge Response Opening ->
  Witness -> Randomizer -> Prop}

theorem accepted_accountBacked_or_break
    (C :
      AccountCarriedAccountOpeningSoundnessCertificate A witnessCommit
        randomizerCommit hiddenLinkCommit proofCommitmentOfRandomizer
        accountSlotWellFormed accountOpeningAccepts openingRelation)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening)
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A witnessCommit randomizerCommit
      proofCommitmentOfRandomizer hiddenLinkCommit openingRelation T ∨
      C.breaks.any :=
  C.accepting_implies_accountBacked_or_break T hAccept

theorem accepted_accountBacked_of_no_breaks
    (C :
      AccountCarriedAccountOpeningSoundnessCertificate A witnessCommit
        randomizerCommit hiddenLinkCommit proofCommitmentOfRandomizer
        accountSlotWellFormed accountOpeningAccepts openingRelation)
    (hNoBreaks : ¬ C.breaks.any)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment
        StatementDigest RandomizerCommitment LinkCommitment Challenge Response
        Opening)
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A witnessCommit randomizerCommit
      proofCommitmentOfRandomizer hiddenLinkCommit openingRelation T := by
  rcases C.accepted_accountBacked_or_break T hAccept with hbacked | hbreak
  · exact hbacked
  · exact False.elim (hNoBreaks hbreak)

end AccountCarriedAccountOpeningSoundnessCertificate

/--
Composed account-link proof system.

The local opening still checks the compact local/quotient transcript.  The link
proof is the new component: it may read the consumed account slot and must rule
out public post-challenge representatives unless `accountLinkProofBreak` occurs.
-/
structure AccountLinkProofSystem
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop)
    where
  witnessCommit : Witness -> WitnessCommitment
  randomizerCommit : Randomizer -> RandomizerCommitment
  hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment
  proofCommitmentOfRandomizer : Randomizer -> StatementDigest
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
  accountLinkProofBreak : Prop
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
        accountLinkProofBreak

namespace AccountLinkProofSystem

variable {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
  LinkCommitment Challenge Response LocalOpening LinkProof : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {accountSlotWellFormed :
  AccountCarriedSlot StatementDigest WitnessCommitment RandomizerCommitment
    LinkCommitment -> Prop}

def accountOpeningAccepts
    (S :
      AccountLinkProofSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof)) : Prop :=
  S.localOpeningAccepts (AccountCarriedOpening.localTranscript T.toTranscript) ∧
  S.linkProofAccepts T

theorem accepts_accountBacked_or_break_of_accepts
    (S :
      AccountLinkProofSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof))
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        S.accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A S.witnessCommit S.randomizerCommit
      S.proofCommitmentOfRandomizer S.hiddenLinkCommit
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T ∨
      S.accountLinkProofBreak :=
  S.accepts_accountBacked_or_break T hAccept.1 hAccept.2.1 hAccept.2.2.1
    hAccept.2.2.2

theorem accepts_accountBacked_of_noBreak
    (S :
      AccountLinkProofSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    (hNoBreak : ¬ S.accountLinkProofBreak)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof))
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        S.accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A S.witnessCommit S.randomizerCommit
      S.proofCommitmentOfRandomizer S.hiddenLinkCommit
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T := by
  rcases S.accepts_accountBacked_or_break_of_accepts T hAccept with
    hbacked | hbreak
  · exact hbacked
  · exact False.elim (hNoBreak hbreak)

def toAccountOpeningSoundnessCertificate
    (S :
      AccountLinkProofSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed) :
    AccountCarriedAccountOpeningSoundnessCertificate A S.witnessCommit
      S.randomizerCommit S.hiddenLinkCommit S.proofCommitmentOfRandomizer
      accountSlotWellFormed S.accountOpeningAccepts
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) where
  breaks :=
    { witnessCommitmentBindingBreak := False
      randomizerCommitmentBindingBreak := False
      openingProofBreak := S.accountLinkProofBreak
      fiatShamirFrameBreak := False
      postChallengeRepresentativeBreak := False }
  accepting_implies_accountBacked_or_break := by
    intro T hAccept
    rcases S.accepts_accountBacked_or_break_of_accepts T hAccept with
      hbacked | hbreak
    · exact Or.inl hbacked
    · exact Or.inr (SoundnessBreaks.any_of_openingProofBreak hbreak)

theorem toAccountOpeningSoundnessCertificate_accepts_accountBacked_of_noBreak
    (S :
      AccountLinkProofSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    (hNoBreak : ¬ S.accountLinkProofBreak)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof))
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        S.accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A S.witnessCommit S.randomizerCommit
      S.proofCommitmentOfRandomizer S.hiddenLinkCommit
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T := by
  exact S.accepts_accountBacked_of_noBreak hNoBreak T hAccept

end AccountLinkProofSystem

/--
Pre-response account backing system.

This is the part of the account-carried proof that should be discharged by the
local/account opening before the deterministic response-link equation is used.
It proves that the consumed account slot and local opening identify one hidden
`w,r`, but it intentionally does not prove
`response = responseOf r challenge w`.
-/
structure AccountPreResponseBackingSystem
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop)
    where
  witnessCommit : Witness -> WitnessCommitment
  randomizerCommit : Randomizer -> RandomizerCommitment
  hiddenLinkCommit : Witness -> Randomizer -> LinkCommitment
  proofCommitmentOfRandomizer : Randomizer -> StatementDigest
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
  preResponseBackingBreak : Prop
  accepts_preResponseBacked_or_break :
    ∀ T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof),
      accountSlotWellFormed T.accountSlot ->
      localOpeningAccepts (AccountCarriedOpening.localTranscript T.toTranscript) ->
        AccountBackedExceptResponse A witnessCommit randomizerCommit
          proofCommitmentOfRandomizer hiddenLinkCommit
          (fun Tfull w r =>
            localOpeningRelation
              (AccountCarriedOpening.localTranscript Tfull) w r) T ∨
        preResponseBackingBreak

namespace AccountPreResponseBackingSystem

variable {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
  LinkCommitment Challenge Response LocalOpening LinkProof : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {accountSlotWellFormed :
  AccountCarriedSlot StatementDigest WitnessCommitment RandomizerCommitment
    LinkCommitment -> Prop}

/--
Compose pre-response backing with deterministic response-link soundness.

This is the global proof adapter needed by the account-carried committed-sigma
route: local/account opening first backs the slot by the same hidden `w,r`; the
response-link theorem then turns the public sigma equation plus link acceptance
into the honest response equality.  The only remaining failures are the
pre-response backing break or the link-kernel break.
-/
def toAccountLinkProofSystem
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {linkKernelBreak : Prop}
    (hResponseLink :
      ResponseLinkSoundness A S.witnessCommit S.randomizerCommit
        S.proofCommitmentOfRandomizer S.hiddenLinkCommit
        (fun Tfull w r =>
          S.localOpeningRelation
            (AccountCarriedOpening.localTranscript Tfull) w r)
        S.linkProofAccepts linkKernelBreak) :
    AccountLinkProofSystem
      (Witness := Witness) (Randomizer := Randomizer)
      (StatementDigest := StatementDigest)
      (WitnessCommitment := WitnessCommitment)
      (RandomizerCommitment := RandomizerCommitment)
      (LinkCommitment := LinkCommitment)
      (Challenge := Challenge) (Response := Response)
      (LocalOpening := LocalOpening) (LinkProof := LinkProof)
      A accountSlotWellFormed where
  witnessCommit := S.witnessCommit
  randomizerCommit := S.randomizerCommit
  hiddenLinkCommit := S.hiddenLinkCommit
  proofCommitmentOfRandomizer := S.proofCommitmentOfRandomizer
  localOpeningAccepts := S.localOpeningAccepts
  linkProofAccepts := S.linkProofAccepts
  localOpeningRelation := S.localOpeningRelation
  accountLinkProofBreak := S.preResponseBackingBreak ∨ linkKernelBreak
  accepts_accountBacked_or_break := by
    intro T hSlot hEquation hLocal hLink
    rcases S.accepts_preResponseBacked_or_break T hSlot hLocal with
      hPre | hPreBreak
    · rcases
        hPre.accountBacked_or_linkKernelBreak hLink hResponseLink hEquation with
        hBacked | hKernelBreak
      · exact Or.inl hBacked
      · exact Or.inr (Or.inr hKernelBreak)
    · exact Or.inr (Or.inl hPreBreak)

def toAccountLinkProofSystemOfBoundedKernel
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {ResponseCommitment : Type*}
    {responseCommit : Response -> ResponseCommitment}
    {linkKernelBreak : Prop}
    (K : BoundedResponseLinkKernelCertificate A responseCommit linkKernelBreak)
    (hProofCommit :
      ∀ r, S.proofCommitmentOfRandomizer r = A.randomizerMap r)
    (hLinkEquation :
      ∀ T w r,
        T.accountSlot.statementDigest = A.witnessDigest w ->
        T.accountSlot.witnessCommitment = S.witnessCommit w ->
        T.accountSlot.randomizerCommitment = S.randomizerCommit r ->
        T.accountSlot.hiddenLinkCommitment = S.hiddenLinkCommit w r ->
        T.proofCommitment = S.proofCommitmentOfRandomizer r ->
        (fun Tfull w r =>
          S.localOpeningRelation
            (AccountCarriedOpening.localTranscript Tfull) w r) T.toTranscript w r ->
        S.linkProofAccepts T ->
        responseCommit T.response =
          responseCommit (A.responseOf r T.challenge w))
    (hDifferenceBox :
      ∀ T w r,
        T.accountSlot.statementDigest = A.witnessDigest w ->
        T.accountSlot.witnessCommitment = S.witnessCommit w ->
        T.accountSlot.randomizerCommitment = S.randomizerCommit r ->
        T.accountSlot.hiddenLinkCommitment = S.hiddenLinkCommit w r ->
        T.proofCommitment = S.proofCommitmentOfRandomizer r ->
        (fun Tfull w r =>
          S.localOpeningRelation
            (AccountCarriedOpening.localTranscript Tfull) w r) T.toTranscript w r ->
        A.verifierEquation T.proofCommitment T.challenge
          T.accountSlot.statementDigest T.response ->
        S.linkProofAccepts T ->
        K.inResponseDifferenceBox T.response
          (A.responseOf r T.challenge w)) :
    AccountLinkProofSystem
      (Witness := Witness) (Randomizer := Randomizer)
      (StatementDigest := StatementDigest)
      (WitnessCommitment := WitnessCommitment)
      (RandomizerCommitment := RandomizerCommitment)
      (LinkCommitment := LinkCommitment)
      (Challenge := Challenge) (Response := Response)
      (LocalOpening := LocalOpening) (LinkProof := LinkProof)
      A accountSlotWellFormed :=
  S.toAccountLinkProofSystem
    (K.responseLinkSoundness
      (witnessCommit := S.witnessCommit)
      (randomizerCommit := S.randomizerCommit)
      (proofCommitmentOfRandomizer := S.proofCommitmentOfRandomizer)
      (hiddenLinkCommit := S.hiddenLinkCommit)
      (openingRelation := fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r)
      (linkProofAccepts := S.linkProofAccepts)
      hProofCommit hLinkEquation hDifferenceBox)

def toAccountOpeningSoundnessCertificateOfBoundedKernel
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {ResponseCommitment : Type*}
    {responseCommit : Response -> ResponseCommitment}
    {linkKernelBreak : Prop}
    (K : BoundedResponseLinkKernelCertificate A responseCommit linkKernelBreak)
    (hProofCommit :
      ∀ r, S.proofCommitmentOfRandomizer r = A.randomizerMap r)
    (hLinkEquation :
      ∀ T w r,
        T.accountSlot.statementDigest = A.witnessDigest w ->
        T.accountSlot.witnessCommitment = S.witnessCommit w ->
        T.accountSlot.randomizerCommitment = S.randomizerCommit r ->
        T.accountSlot.hiddenLinkCommitment = S.hiddenLinkCommit w r ->
        T.proofCommitment = S.proofCommitmentOfRandomizer r ->
        (fun Tfull w r =>
          S.localOpeningRelation
            (AccountCarriedOpening.localTranscript Tfull) w r) T.toTranscript w r ->
        S.linkProofAccepts T ->
        responseCommit T.response =
          responseCommit (A.responseOf r T.challenge w))
    (hDifferenceBox :
      ∀ T w r,
        T.accountSlot.statementDigest = A.witnessDigest w ->
        T.accountSlot.witnessCommitment = S.witnessCommit w ->
        T.accountSlot.randomizerCommitment = S.randomizerCommit r ->
        T.accountSlot.hiddenLinkCommitment = S.hiddenLinkCommit w r ->
        T.proofCommitment = S.proofCommitmentOfRandomizer r ->
        (fun Tfull w r =>
          S.localOpeningRelation
            (AccountCarriedOpening.localTranscript Tfull) w r) T.toTranscript w r ->
        A.verifierEquation T.proofCommitment T.challenge
          T.accountSlot.statementDigest T.response ->
        S.linkProofAccepts T ->
        K.inResponseDifferenceBox T.response
          (A.responseOf r T.challenge w)) :
    AccountCarriedAccountOpeningSoundnessCertificate A S.witnessCommit
      S.randomizerCommit S.hiddenLinkCommit S.proofCommitmentOfRandomizer
      accountSlotWellFormed
      (S.toAccountLinkProofSystemOfBoundedKernel K hProofCommit hLinkEquation
        hDifferenceBox).accountOpeningAccepts
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) :=
  (S.toAccountLinkProofSystemOfBoundedKernel K hProofCommit hLinkEquation
    hDifferenceBox).toAccountOpeningSoundnessCertificate

theorem accepts_accountBacked_of_noBreaks_ofBoundedKernel
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {ResponseCommitment : Type*}
    {responseCommit : Response -> ResponseCommitment}
    {linkKernelBreak : Prop}
    (K : BoundedResponseLinkKernelCertificate A responseCommit linkKernelBreak)
    (hProofCommit :
      ∀ r, S.proofCommitmentOfRandomizer r = A.randomizerMap r)
    (hLinkEquation :
      ∀ T w r,
        T.accountSlot.statementDigest = A.witnessDigest w ->
        T.accountSlot.witnessCommitment = S.witnessCommit w ->
        T.accountSlot.randomizerCommitment = S.randomizerCommit r ->
        T.accountSlot.hiddenLinkCommitment = S.hiddenLinkCommit w r ->
        T.proofCommitment = S.proofCommitmentOfRandomizer r ->
        (fun Tfull w r =>
          S.localOpeningRelation
            (AccountCarriedOpening.localTranscript Tfull) w r) T.toTranscript w r ->
        S.linkProofAccepts T ->
        responseCommit T.response =
          responseCommit (A.responseOf r T.challenge w))
    (hDifferenceBox :
      ∀ T w r,
        T.accountSlot.statementDigest = A.witnessDigest w ->
        T.accountSlot.witnessCommitment = S.witnessCommit w ->
        T.accountSlot.randomizerCommitment = S.randomizerCommit r ->
        T.accountSlot.hiddenLinkCommitment = S.hiddenLinkCommit w r ->
        T.proofCommitment = S.proofCommitmentOfRandomizer r ->
        (fun Tfull w r =>
          S.localOpeningRelation
            (AccountCarriedOpening.localTranscript Tfull) w r) T.toTranscript w r ->
        A.verifierEquation T.proofCommitment T.challenge
          T.accountSlot.statementDigest T.response ->
        S.linkProofAccepts T ->
        K.inResponseDifferenceBox T.response
          (A.responseOf r T.challenge w))
    (hNoPreResponseBreak : ¬ S.preResponseBackingBreak)
    (hNoLinkKernelBreak : ¬ linkKernelBreak)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof))
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        (S.toAccountLinkProofSystemOfBoundedKernel K hProofCommit hLinkEquation
          hDifferenceBox).accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A S.witnessCommit S.randomizerCommit
      S.proofCommitmentOfRandomizer S.hiddenLinkCommit
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T := by
  let P :=
    S.toAccountLinkProofSystemOfBoundedKernel K hProofCommit hLinkEquation
      hDifferenceBox
  exact
    P.accepts_accountBacked_of_noBreak
      (by
        intro hBreak
        rcases hBreak with hPre | hKernel
        · exact hNoPreResponseBreak hPre
        · exact hNoLinkKernelBreak hKernel)
      T hAccept

/--
Named obligation bundle for the bounded-kernel account-carried deployment path.

A concrete instantiation should provide this object after choosing the
response-commitment representation.  It packages the bounded kernel certificate
with the byte-level bridge facts needed by
`toAccountOpeningSoundnessCertificateOfBoundedKernel`.
-/
structure BoundedKernelDeploymentObligations
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {ResponseCommitment : Type*}
    (responseCommit : Response -> ResponseCommitment) where
  linkKernelBreak : Prop
  kernelCertificate :
    BoundedResponseLinkKernelCertificate A responseCommit linkKernelBreak
  proofCommit_eq_randomizerMap :
    ∀ r, S.proofCommitmentOfRandomizer r = A.randomizerMap r
  linkEquation :
    ∀ T w r,
      T.accountSlot.statementDigest = A.witnessDigest w ->
      T.accountSlot.witnessCommitment = S.witnessCommit w ->
      T.accountSlot.randomizerCommitment = S.randomizerCommit r ->
      T.accountSlot.hiddenLinkCommitment = S.hiddenLinkCommit w r ->
      T.proofCommitment = S.proofCommitmentOfRandomizer r ->
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T.toTranscript w r ->
      S.linkProofAccepts T ->
      responseCommit T.response =
        responseCommit (A.responseOf r T.challenge w)
  differenceBox :
    ∀ T w r,
      T.accountSlot.statementDigest = A.witnessDigest w ->
      T.accountSlot.witnessCommitment = S.witnessCommit w ->
      T.accountSlot.randomizerCommitment = S.randomizerCommit r ->
      T.accountSlot.hiddenLinkCommitment = S.hiddenLinkCommit w r ->
      T.proofCommitment = S.proofCommitmentOfRandomizer r ->
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T.toTranscript w r ->
      A.verifierEquation T.proofCommitment T.challenge
        T.accountSlot.statementDigest T.response ->
      S.linkProofAccepts T ->
      kernelCertificate.inResponseDifferenceBox T.response
        (A.responseOf r T.challenge w)

namespace BoundedKernelDeploymentObligations

variable
  (S :
    AccountPreResponseBackingSystem
      (Witness := Witness) (Randomizer := Randomizer)
      (StatementDigest := StatementDigest)
      (WitnessCommitment := WitnessCommitment)
      (RandomizerCommitment := RandomizerCommitment)
      (LinkCommitment := LinkCommitment)
      (Challenge := Challenge) (Response := Response)
      (LocalOpening := LocalOpening) (LinkProof := LinkProof)
      A accountSlotWellFormed)

def toAccountLinkProofSystem
    {ResponseCommitment : Type*}
    {responseCommit : Response -> ResponseCommitment}
    (O : BoundedKernelDeploymentObligations S responseCommit) :
    AccountLinkProofSystem
      (Witness := Witness) (Randomizer := Randomizer)
      (StatementDigest := StatementDigest)
      (WitnessCommitment := WitnessCommitment)
      (RandomizerCommitment := RandomizerCommitment)
      (LinkCommitment := LinkCommitment)
      (Challenge := Challenge) (Response := Response)
      (LocalOpening := LocalOpening) (LinkProof := LinkProof)
      A accountSlotWellFormed :=
  S.toAccountLinkProofSystemOfBoundedKernel O.kernelCertificate
    O.proofCommit_eq_randomizerMap O.linkEquation O.differenceBox

def toAccountOpeningSoundnessCertificate
    {ResponseCommitment : Type*}
    {responseCommit : Response -> ResponseCommitment}
    (O : BoundedKernelDeploymentObligations S responseCommit) :
    AccountCarriedAccountOpeningSoundnessCertificate A S.witnessCommit
      S.randomizerCommit S.hiddenLinkCommit S.proofCommitmentOfRandomizer
      accountSlotWellFormed (O.toAccountLinkProofSystem S).accountOpeningAccepts
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) :=
  (O.toAccountLinkProofSystem S).toAccountOpeningSoundnessCertificate

theorem accepts_accountBacked_of_noBreaks
    {ResponseCommitment : Type*}
    {responseCommit : Response -> ResponseCommitment}
    (O : BoundedKernelDeploymentObligations S responseCommit)
    (hNoPreResponseBreak : ¬ S.preResponseBackingBreak)
    (hNoLinkKernelBreak : ¬ O.linkKernelBreak)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof))
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        (O.toAccountLinkProofSystem S).accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A S.witnessCommit S.randomizerCommit
      S.proofCommitmentOfRandomizer S.hiddenLinkCommit
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T := by
  exact
    S.accepts_accountBacked_of_noBreaks_ofBoundedKernel O.kernelCertificate
      O.proofCommit_eq_randomizerMap O.linkEquation O.differenceBox
      hNoPreResponseBreak hNoLinkKernelBreak T hAccept

end BoundedKernelDeploymentObligations

end AccountPreResponseBackingSystem

/--
Zero-signature-byte homomorphic account-link interface.

This captures the most plausible way to make the account-carried route concrete:
the consumed slot stores commitments to `w` and `r`, and the verifier can compute
a deterministic commitment of the public response `z`.  The public link equation

```text
Commit(z) = Commit(r) + c * Commit(w)
```

then becomes the `linkProofAccepts` check, with no extra link-proof bytes in the
signature.  This interface does not claim such commitments are easy or already
secure; the `linkEquationSoundness` field is the exact cryptographic obligation.
-/
structure HomomorphicAccountLinkInterface
    {Witness Randomizer StatementDigest LinkCommitment HiddenLinkCommitment
      Challenge Response LocalOpening : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest LinkCommitment LinkCommitment
        HiddenLinkCommitment -> Prop)
    where
  witnessCommit : Witness -> LinkCommitment
  randomizerCommit : Randomizer -> LinkCommitment
  hiddenLinkCommit : Witness -> Randomizer -> HiddenLinkCommitment
  proofCommitmentOfRandomizer : Randomizer -> StatementDigest
  responseCommit : Response -> LinkCommitment
  addCommit : LinkCommitment -> LinkCommitment -> LinkCommitment
  scaleCommit : Challenge -> LinkCommitment -> LinkCommitment
  localOpeningAccepts :
    Transcript StatementDigest LinkCommitment StatementDigest LinkCommitment
      Challenge Response LocalOpening -> Prop
  localOpeningRelation :
    Transcript StatementDigest LinkCommitment StatementDigest LinkCommitment
      Challenge Response LocalOpening ->
    Witness -> Randomizer -> Prop
  linkProofBreak : Prop
  honest_response_commitment :
    ∀ r c w,
      responseCommit (A.responseOf r c w) =
        addCommit (randomizerCommit r) (scaleCommit c (witnessCommit w))
  linkEquationSoundness :
    ∀ T :
      AccountCarriedTranscript StatementDigest LinkCommitment StatementDigest
        LinkCommitment HiddenLinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening Unit),
      accountSlotWellFormed T.accountSlot ->
      A.verifierEquation T.proofCommitment T.challenge
        T.accountSlot.statementDigest T.response ->
      localOpeningAccepts (AccountCarriedOpening.localTranscript T.toTranscript) ->
      responseCommit T.response =
        addCommit T.accountSlot.randomizerCommitment
          (scaleCommit T.challenge T.accountSlot.witnessCommitment) ->
        AccountBackedByHiddenWitness A witnessCommit randomizerCommit
          proofCommitmentOfRandomizer hiddenLinkCommit
          (fun Tfull w r =>
            localOpeningRelation
              (AccountCarriedOpening.localTranscript Tfull) w r) T ∨
        linkProofBreak

namespace HomomorphicAccountLinkInterface

variable {Witness Randomizer StatementDigest LinkCommitment HiddenLinkCommitment
  Challenge Response LocalOpening : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {accountSlotWellFormed :
  AccountCarriedSlot StatementDigest LinkCommitment LinkCommitment
    HiddenLinkCommitment -> Prop}

def linkProofAccepts
    (I :
      HomomorphicAccountLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (LinkCommitment := LinkCommitment)
        (HiddenLinkCommitment := HiddenLinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) A accountSlotWellFormed)
    (T :
      AccountCarriedTranscript StatementDigest LinkCommitment StatementDigest
        LinkCommitment HiddenLinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening Unit)) : Prop :=
  I.responseCommit T.response =
    I.addCommit T.accountSlot.randomizerCommitment
      (I.scaleCommit T.challenge T.accountSlot.witnessCommitment)

theorem honest_linkProofAccepts
    (I :
      HomomorphicAccountLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (LinkCommitment := LinkCommitment)
        (HiddenLinkCommitment := HiddenLinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) A accountSlotWellFormed)
    (r : Randomizer) (c : Challenge) (w : Witness)
    (localOpening : LocalOpening) :
    I.linkProofAccepts
      { accountSlot :=
          { statementDigest := A.witnessDigest w
            witnessCommitment := I.witnessCommit w
            randomizerCommitment := I.randomizerCommit r
            hiddenLinkCommitment := I.hiddenLinkCommit w r }
        proofCommitment := I.proofCommitmentOfRandomizer r
        challenge := c
        response := A.responseOf r c w
        opening := { localOpening := localOpening
                     linkProof := () } } := by
  exact I.honest_response_commitment r c w

/--
Degenerate homomorphic links that are only a public projection of the sigma
equation add no new acceptance constraint.

This is the guardrail for concrete instantiations: `linkEquationSoundness`
cannot be discharged by reusing the public relation map in another byte
encoding.  If response/account commitments are just homomorphic projections of
`L(z)`, `a`, and `digest`, then `linkProofAccepts` follows from the public
sigma equation alone.
-/
theorem linkProofAccepts_of_publicProjection
    (I :
      HomomorphicAccountLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (LinkCommitment := LinkCommitment)
        (HiddenLinkCommitment := HiddenLinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) A accountSlotWellFormed)
    (project : StatementDigest -> LinkCommitment)
    (hAdd :
      ∀ x y,
        project (A.add x y) =
          I.addCommit (project x) (project y))
    (hScale :
      ∀ c x,
        project (A.scale c x) =
          I.scaleCommit c (project x))
    {T :
      AccountCarriedTranscript StatementDigest LinkCommitment StatementDigest
        LinkCommitment HiddenLinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening Unit)}
    (hEquation :
      A.verifierEquation T.proofCommitment T.challenge
        T.accountSlot.statementDigest T.response)
    (hResponse :
      I.responseCommit T.response = project (A.relationMap T.response))
    (hRandomizer :
      T.accountSlot.randomizerCommitment = project T.proofCommitment)
    (hWitness :
      T.accountSlot.witnessCommitment =
        project T.accountSlot.statementDigest) :
    I.linkProofAccepts T := by
  unfold linkProofAccepts
  rw [hResponse, hEquation, hAdd, hScale, ← hRandomizer, ← hWitness]

def toAccountLinkProofSystem
    (I :
      HomomorphicAccountLinkInterface
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (LinkCommitment := LinkCommitment)
        (HiddenLinkCommitment := HiddenLinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) A accountSlotWellFormed) :
    AccountLinkProofSystem
      (Witness := Witness) (Randomizer := Randomizer)
      (StatementDigest := StatementDigest)
      (WitnessCommitment := LinkCommitment)
      (RandomizerCommitment := LinkCommitment)
      (LinkCommitment := HiddenLinkCommitment)
      (Challenge := Challenge) (Response := Response)
      (LocalOpening := LocalOpening) (LinkProof := Unit)
      A accountSlotWellFormed where
  witnessCommit := I.witnessCommit
  randomizerCommit := I.randomizerCommit
  hiddenLinkCommit := I.hiddenLinkCommit
  proofCommitmentOfRandomizer := I.proofCommitmentOfRandomizer
  localOpeningAccepts := I.localOpeningAccepts
  linkProofAccepts := I.linkProofAccepts
  localOpeningRelation := I.localOpeningRelation
  accountLinkProofBreak := I.linkProofBreak
  accepts_accountBacked_or_break := by
    intro T hSlot hEquation hLocal hLink
    exact I.linkEquationSoundness T hSlot hEquation hLocal hLink

end HomomorphicAccountLinkInterface

/-!
Concrete algebraic adapter for the existing full-response quotient operations.

This is intentionally only the sigma equation layer.  It reuses the
`FullResponseByteOpeningAlgebra` operations for

```text
L(r + c*w) = L(r) + c*L(w)
```

but it does not reuse the old representative verifier as the deployed
soundness theorem.  The committed-sigma opening layer must still prove that the
accepted response is backed by prechallenge witness/randomizer commitments.
-/
namespace FullResponseQuotient

open LocallyRecoverableSyndrome.LocalDeltaByteCommitment.N32Q12289

def statementAlgebra
    (A : FullResponseByteOpeningAlgebra)
    (statement : PublicOpeningStatement) :
    LinearSigmaAlgebra
      (ByteVector linearSigmaMaskedWitnessBytes)
      (ByteVector linearSigmaMaskedWitnessBytes)
      (ByteVector challengeBytes)
      (ByteVector linearSigmaMaskedWitnessBytes)
      (ByteVector linearSigmaProofCommitmentBytes) where
  relationMap := A.relationMapResponse statement
  randomizerMap := A.relationMapResponse statement
  witnessDigest := A.relationMapResponse statement
  add := A.addCommitment
  scale := A.scaleCommitment
  responseOf := fun randomizer challenge witness =>
    A.addResponse randomizer (A.scaleResponse challenge witness)
  honest_response_equation := by
    intro randomizer challenge witness
    exact A.response_linear statement randomizer witness challenge

theorem statementAlgebra_verifierEquation_iff_verifyFields
    (A : FullResponseByteOpeningAlgebra)
    (statement : PublicOpeningStatement)
    (proofCommitment : ByteVector linearSigmaProofCommitmentBytes)
    (maskedWitnessResponse : ByteVector linearSigmaMaskedWitnessBytes) :
    (statementAlgebra A statement).verifierEquation
        proofCommitment statement.challenge statement.digest
        maskedWitnessResponse ↔
      A.verifyFields statement proofCommitment maskedWitnessResponse := by
  rfl

theorem statementAlgebra_honest_response_equation
    (A : FullResponseByteOpeningAlgebra)
    (statement : PublicOpeningStatement)
    (randomizer witness : ByteVector linearSigmaMaskedWitnessBytes)
    (challengeBytesValue : ByteVector challengeBytes) :
    (statementAlgebra A statement).relationMap
        ((statementAlgebra A statement).responseOf
          randomizer challengeBytesValue witness) =
      (statementAlgebra A statement).add
        ((statementAlgebra A statement).randomizerMap randomizer)
        ((statementAlgebra A statement).scale challengeBytesValue
          ((statementAlgebra A statement).witnessDigest witness)) := by
  exact
    (statementAlgebra A statement).honest_response_equation
      randomizer challengeBytesValue witness

/--
Committed-sigma proof commitments for the current full-response algebra are
the relation-map image of the hidden randomizer.
-/
def proofCommitmentOfRandomizer
    (A : FullResponseByteOpeningAlgebra)
    (statement : PublicOpeningStatement)
    (randomizer : ByteVector linearSigmaMaskedWitnessBytes) :
    ByteVector linearSigmaProofCommitmentBytes :=
  A.relationMapResponse statement randomizer

theorem proofCommitmentOfRandomizer_eq_randomizerMap
    (A : FullResponseByteOpeningAlgebra)
    (statement : PublicOpeningStatement)
    (randomizer : ByteVector linearSigmaMaskedWitnessBytes) :
    proofCommitmentOfRandomizer A statement randomizer =
      (statementAlgebra A statement).randomizerMap randomizer := by
  rfl

end FullResponseQuotient

/--
Parameter-search cost model for the new architecture.

The search should optimize this object rather than the old representative
verifier.  `accountSlotBytes` is deliberately separated from
`signatureBytes`, because the stateful construction can move prechallenge
randomizer commitments into account state.
-/
structure ParameterSearchModel where
  publicKeyBytes : Nat
  accountSlotBytes : Nat
  witnessCommitmentBytes : Nat
  randomizerCommitmentBytes : Nat
  challengeBytes : Nat
  responseBytes : Nat
  openingBytes : Nat
  proofOverheadBytes : Nat
  classicalSecurityBits : Nat
  quantumSecurityBits : Nat
  rejectsPublicPostChallengeRepresentatives : Prop
  hasPrechallengeWitnessBoundCommitment : Prop

namespace ParameterSearchModel

def signatureBytes (M : ParameterSearchModel) : Nat :=
  M.challengeBytes + M.responseBytes + M.openingBytes + M.proofOverheadBytes

def meets128 (M : ParameterSearchModel) : Prop :=
  128 ≤ M.classicalSecurityBits ∧ 128 ≤ M.quantumSecurityBits

def structurallyAdmissible (M : ParameterSearchModel) : Prop :=
  M.meets128 ∧
  M.rejectsPublicPostChallengeRepresentatives ∧
  M.hasPrechallengeWitnessBoundCommitment

theorem structurallyAdmissible_meets128
    {M : ParameterSearchModel}
    (h : M.structurallyAdmissible) :
    M.meets128 :=
  h.1

end ParameterSearchModel

/--
Byte-level opening instantiation candidate.

This is the object the next parameter search should enumerate before running
lattice/security scoring.  It separates byte cost from structural admissibility:
candidates that reveal the randomizer or fail to instantiate
`PrechallengeRandomizerOpeningInterface` are rejected before byte ranking.
-/
structure OpeningInstantiationCandidate where
  name : String
  signatureOpeningBytes : Nat
  accountSlotBytes : Nat
  proverWorkUnits : Nat
  verifierWorkUnits : Nat
  revealsRandomizer : Prop
  instantiatesPrechallengeOpeningInterface : Prop
  postChallengeRepresentativeRejected : Prop
  witnessHidingArgument : Prop

namespace OpeningInstantiationCandidate

def structurallyAdmissible (C : OpeningInstantiationCandidate) : Prop :=
  ¬ C.revealsRandomizer ∧
  C.instantiatesPrechallengeOpeningInterface ∧
  C.postChallengeRepresentativeRejected ∧
  C.witnessHidingArgument

def totalStatefulByteCost (C : OpeningInstantiationCandidate) : Nat :=
  C.signatureOpeningBytes + C.accountSlotBytes

theorem structurallyAdmissible_not_revealRandomizer
    {C : OpeningInstantiationCandidate}
    (h : C.structurallyAdmissible) :
    ¬ C.revealsRandomizer :=
  h.1

theorem revealRandomizer_not_structurallyAdmissible
    {C : OpeningInstantiationCandidate}
    (hreveal : C.revealsRandomizer) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact h.1 hreveal

theorem missingInterface_not_structurallyAdmissible
    {C : OpeningInstantiationCandidate}
    (hmissing : ¬ C.instantiatesPrechallengeOpeningInterface) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact hmissing h.2.1

theorem postChallengeRepresentative_notRejected_not_structurallyAdmissible
    {C : OpeningInstantiationCandidate}
    (hmissing : ¬ C.postChallengeRepresentativeRejected) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact hmissing h.2.2.1

theorem missingWitnessHiding_not_structurallyAdmissible
    {C : OpeningInstantiationCandidate}
    (hmissing : ¬ C.witnessHidingArgument) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact hmissing h.2.2.2

def revealRandomizerCandidate
    (openingBytes accountBytes proverWork verifierWork : Nat) :
    OpeningInstantiationCandidate where
  name := "reveal-randomizer"
  signatureOpeningBytes := openingBytes
  accountSlotBytes := accountBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  revealsRandomizer := True
  instantiatesPrechallengeOpeningInterface := False
  postChallengeRepresentativeRejected := True
  witnessHidingArgument := False

theorem revealRandomizerCandidate_not_structurallyAdmissible
    (openingBytes accountBytes proverWork verifierWork : Nat) :
    ¬ (revealRandomizerCandidate openingBytes accountBytes proverWork verifierWork).structurallyAdmissible :=
  revealRandomizer_not_structurallyAdmissible
    (C := revealRandomizerCandidate openingBytes accountBytes proverWork verifierWork)
    True.intro

def bareHashRandomizerCandidate
    (openingBytes accountBytes proverWork verifierWork : Nat) :
    OpeningInstantiationCandidate where
  name := "bare-hash-randomizer-commitment"
  signatureOpeningBytes := openingBytes
  accountSlotBytes := accountBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  revealsRandomizer := False
  instantiatesPrechallengeOpeningInterface := False
  postChallengeRepresentativeRejected := False
  witnessHidingArgument := True

theorem bareHashRandomizerCandidate_not_structurallyAdmissible
    (openingBytes accountBytes proverWork verifierWork : Nat) :
    ¬ (bareHashRandomizerCandidate openingBytes accountBytes proverWork verifierWork).structurallyAdmissible :=
  missingInterface_not_structurallyAdmissible
    (C := bareHashRandomizerCandidate openingBytes accountBytes proverWork verifierWork)
    (by intro h; cases h)

def zOnlyOpeningCandidate
    (openingBytes accountBytes proverWork verifierWork : Nat) :
    OpeningInstantiationCandidate where
  name := "z-only-local-opening"
  signatureOpeningBytes := openingBytes
  accountSlotBytes := accountBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  revealsRandomizer := False
  instantiatesPrechallengeOpeningInterface := False
  postChallengeRepresentativeRejected := False
  witnessHidingArgument := True

theorem zOnlyOpeningCandidate_not_structurallyAdmissible
    (openingBytes accountBytes proverWork verifierWork : Nat) :
    ¬ (zOnlyOpeningCandidate openingBytes accountBytes proverWork verifierWork).structurallyAdmissible :=
  postChallengeRepresentative_notRejected_not_structurallyAdmissible
    (C := zOnlyOpeningCandidate openingBytes accountBytes proverWork verifierWork)
    (by intro h; cases h)

def abstractHiddenLinkProofCandidate
    (openingBytes accountBytes proverWork verifierWork : Nat)
    (hInterface hRejects hHiding : Prop) :
    OpeningInstantiationCandidate where
  name := "abstract-hidden-link-proof"
  signatureOpeningBytes := openingBytes
  accountSlotBytes := accountBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  revealsRandomizer := False
  instantiatesPrechallengeOpeningInterface := hInterface
  postChallengeRepresentativeRejected := hRejects
  witnessHidingArgument := hHiding

theorem abstractHiddenLinkProofCandidate_structurallyAdmissible
    {openingBytes accountBytes proverWork verifierWork : Nat}
    {hInterface hRejects hHiding : Prop}
    (hi : hInterface) (hr : hRejects) (hh : hHiding) :
    (abstractHiddenLinkProofCandidate
      openingBytes accountBytes proverWork verifierWork
      hInterface hRejects hHiding).structurallyAdmissible :=
  ⟨by intro h; exact h, hi, hr, hh⟩

end OpeningInstantiationCandidate

/--
Candidate class for the account-link proof component.

This is narrower than `OpeningInstantiationCandidate`: it models only the
`linkProofAccepts` side of `AccountLinkProofSystem`.  The local opening may still
be the existing local-delta/quotient opening; the link proof must be account-slot
aware and must prevent public post-challenge representatives from passing.
-/
structure LinkProofInstantiationCandidate where
  name : String
  signatureLinkProofBytes : Nat
  accountLinkBytes : Nat
  proverWorkUnits : Nat
  verifierWorkUnits : Nat
  revealsWitnessOrRandomizer : Prop
  localOpeningOnly : Prop
  accountSlotAware : Prop
  instantiatesAccountLinkProofSystem : Prop
  postChallengeRepresentativeRejected : Prop
  nonPublicProjectionOfSigmaEquation : Prop
  witnessHidingArgument : Prop

namespace LinkProofInstantiationCandidate

def structurallyAdmissible (C : LinkProofInstantiationCandidate) : Prop :=
  ¬ C.revealsWitnessOrRandomizer ∧
  ¬ C.localOpeningOnly ∧
  C.accountSlotAware ∧
  C.instantiatesAccountLinkProofSystem ∧
  C.postChallengeRepresentativeRejected ∧
  C.nonPublicProjectionOfSigmaEquation ∧
  C.witnessHidingArgument

def totalStateBytes (C : LinkProofInstantiationCandidate) : Nat :=
  C.signatureLinkProofBytes + C.accountLinkBytes

theorem reveal_not_structurallyAdmissible
    {C : LinkProofInstantiationCandidate}
    (hreveal : C.revealsWitnessOrRandomizer) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact h.1 hreveal

theorem localOpeningOnly_not_structurallyAdmissible
    {C : LinkProofInstantiationCandidate}
    (hlocal : C.localOpeningOnly) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact h.2.1 hlocal

theorem missingAccountSlot_not_structurallyAdmissible
    {C : LinkProofInstantiationCandidate}
    (hmissing : ¬ C.accountSlotAware) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact hmissing h.2.2.1

theorem missingProofSystem_not_structurallyAdmissible
    {C : LinkProofInstantiationCandidate}
    (hmissing : ¬ C.instantiatesAccountLinkProofSystem) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact hmissing h.2.2.2.1

theorem postChallengeRepresentative_notRejected_not_structurallyAdmissible
    {C : LinkProofInstantiationCandidate}
    (hmissing : ¬ C.postChallengeRepresentativeRejected) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact hmissing h.2.2.2.2.1

theorem missingWitnessHiding_not_structurallyAdmissible
    {C : LinkProofInstantiationCandidate}
    (hmissing : ¬ C.witnessHidingArgument) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact hmissing h.2.2.2.2.2.2

theorem publicProjection_not_structurallyAdmissible
    {C : LinkProofInstantiationCandidate}
    (hprojection : ¬ C.nonPublicProjectionOfSigmaEquation) :
    ¬ C.structurallyAdmissible := by
  intro h
  exact hprojection h.2.2.2.2.2.1

def revealWitnessOrRandomizerCandidate
    (signatureBytes accountBytes proverWork verifierWork : Nat) :
    LinkProofInstantiationCandidate where
  name := "reveal-witness-or-randomizer-link"
  signatureLinkProofBytes := signatureBytes
  accountLinkBytes := accountBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  revealsWitnessOrRandomizer := True
  localOpeningOnly := False
  accountSlotAware := True
  instantiatesAccountLinkProofSystem := False
  postChallengeRepresentativeRejected := True
  nonPublicProjectionOfSigmaEquation := True
  witnessHidingArgument := False

theorem revealWitnessOrRandomizerCandidate_not_structurallyAdmissible
    (signatureBytes accountBytes proverWork verifierWork : Nat) :
    ¬ (revealWitnessOrRandomizerCandidate signatureBytes accountBytes
      proverWork verifierWork).structurallyAdmissible :=
  reveal_not_structurallyAdmissible
    (C :=
      revealWitnessOrRandomizerCandidate signatureBytes accountBytes
        proverWork verifierWork)
    True.intro

def localOpeningOnlyCandidate
    (signatureBytes accountBytes proverWork verifierWork : Nat) :
    LinkProofInstantiationCandidate where
  name := "local-opening-only-link"
  signatureLinkProofBytes := signatureBytes
  accountLinkBytes := accountBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  revealsWitnessOrRandomizer := False
  localOpeningOnly := True
  accountSlotAware := False
  instantiatesAccountLinkProofSystem := False
  postChallengeRepresentativeRejected := False
  nonPublicProjectionOfSigmaEquation := False
  witnessHidingArgument := True

theorem localOpeningOnlyCandidate_not_structurallyAdmissible
    (signatureBytes accountBytes proverWork verifierWork : Nat) :
    ¬ (localOpeningOnlyCandidate signatureBytes accountBytes
      proverWork verifierWork).structurallyAdmissible :=
  localOpeningOnly_not_structurallyAdmissible
    (C :=
      localOpeningOnlyCandidate signatureBytes accountBytes proverWork
        verifierWork)
    True.intro

def bareAccountHashCandidate
    (signatureBytes accountBytes proverWork verifierWork : Nat) :
    LinkProofInstantiationCandidate where
  name := "bare-account-hash-link"
  signatureLinkProofBytes := signatureBytes
  accountLinkBytes := accountBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  revealsWitnessOrRandomizer := False
  localOpeningOnly := False
  accountSlotAware := True
  instantiatesAccountLinkProofSystem := False
  postChallengeRepresentativeRejected := False
  nonPublicProjectionOfSigmaEquation := True
  witnessHidingArgument := True

theorem bareAccountHashCandidate_not_structurallyAdmissible
    (signatureBytes accountBytes proverWork verifierWork : Nat) :
    ¬ (bareAccountHashCandidate signatureBytes accountBytes
      proverWork verifierWork).structurallyAdmissible :=
  missingProofSystem_not_structurallyAdmissible
    (C :=
      bareAccountHashCandidate signatureBytes accountBytes proverWork
        verifierWork)
    (by intro h; cases h)

def slotBoundFirstMessageOnlyCandidate
    (signatureBytes accountBytes proverWork verifierWork : Nat) :
    LinkProofInstantiationCandidate where
  name := "slot-bound-first-message-only"
  signatureLinkProofBytes := signatureBytes
  accountLinkBytes := accountBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  revealsWitnessOrRandomizer := False
  localOpeningOnly := False
  accountSlotAware := True
  instantiatesAccountLinkProofSystem := False
  postChallengeRepresentativeRejected := False
  nonPublicProjectionOfSigmaEquation := False
  witnessHidingArgument := True

theorem slotBoundFirstMessageOnlyCandidate_not_structurallyAdmissible
    (signatureBytes accountBytes proverWork verifierWork : Nat) :
    ¬ (slotBoundFirstMessageOnlyCandidate signatureBytes accountBytes
      proverWork verifierWork).structurallyAdmissible :=
  postChallengeRepresentative_notRejected_not_structurallyAdmissible
    (C :=
      slotBoundFirstMessageOnlyCandidate signatureBytes accountBytes proverWork
        verifierWork)
    (by intro h; cases h)

def publicProjectionHomomorphicLinkCandidate
    (signatureBytes accountBytes proverWork verifierWork : Nat) :
    LinkProofInstantiationCandidate where
  name := "public-projection-homomorphic-link"
  signatureLinkProofBytes := signatureBytes
  accountLinkBytes := accountBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  revealsWitnessOrRandomizer := False
  localOpeningOnly := False
  accountSlotAware := True
  instantiatesAccountLinkProofSystem := False
  postChallengeRepresentativeRejected := False
  nonPublicProjectionOfSigmaEquation := False
  witnessHidingArgument := True

theorem publicProjectionHomomorphicLinkCandidate_not_structurallyAdmissible
    (signatureBytes accountBytes proverWork verifierWork : Nat) :
    ¬ (publicProjectionHomomorphicLinkCandidate signatureBytes accountBytes
      proverWork verifierWork).structurallyAdmissible :=
  publicProjection_not_structurallyAdmissible
    (C :=
      publicProjectionHomomorphicLinkCandidate signatureBytes accountBytes
        proverWork verifierWork)
    (by intro h; cases h)

def abstractAccountLinkProofCandidate
    (signatureBytes accountBytes proverWork verifierWork : Nat)
    (hSystem hRejects hNonProjection hHiding : Prop) :
    LinkProofInstantiationCandidate where
  name := "abstract-account-link-proof"
  signatureLinkProofBytes := signatureBytes
  accountLinkBytes := accountBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  revealsWitnessOrRandomizer := False
  localOpeningOnly := False
  accountSlotAware := True
  instantiatesAccountLinkProofSystem := hSystem
  postChallengeRepresentativeRejected := hRejects
  nonPublicProjectionOfSigmaEquation := hNonProjection
  witnessHidingArgument := hHiding

theorem abstractAccountLinkProofCandidate_structurallyAdmissible
    {signatureBytes accountBytes proverWork verifierWork : Nat}
    {hSystem hRejects hNonProjection hHiding : Prop}
    (hs : hSystem) (hr : hRejects) (hn : hNonProjection) (hh : hHiding) :
    (abstractAccountLinkProofCandidate signatureBytes accountBytes
      proverWork verifierWork hSystem hRejects hNonProjection
      hHiding).structurallyAdmissible :=
  ⟨by intro h; exact h, by intro h; exact h, True.intro, hs, hr, hn, hh⟩

def fromBoundedKernelDeploymentObligations
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (_O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (signatureBytes accountBytes proverWork verifierWork : Nat)
    (hRejects hNonProjection hHiding : Prop) :
    LinkProofInstantiationCandidate :=
  abstractAccountLinkProofCandidate signatureBytes accountBytes proverWork
    verifierWork
    (Nonempty
      (AccountLinkProofSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed))
    hRejects hNonProjection hHiding

theorem fromBoundedKernelDeploymentObligations_structurallyAdmissible
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    {S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed}
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    {signatureBytes accountBytes proverWork verifierWork : Nat}
    {hRejects hNonProjection hHiding : Prop}
    (hr : hRejects) (hn : hNonProjection) (hh : hHiding) :
    (fromBoundedKernelDeploymentObligations S O signatureBytes accountBytes
      proverWork verifierWork hRejects hNonProjection
      hHiding).structurallyAdmissible :=
  abstractAccountLinkProofCandidate_structurallyAdmissible
    (hs := ⟨O.toAccountLinkProofSystem S⟩) hr hn hh

end LinkProofInstantiationCandidate

/-
Concrete byte frontier for the current `n = 32`, `q = 12289` stateful target.

This namespace does not claim a concrete hidden-link proof yet.  Its job is to
make the byte pressure precise: once a 32-byte prechallenge proof commitment is
bound into the Fiat-Shamir frame, the remaining space below a 256-byte
transaction depends almost entirely on the packed masked-response width and the
extra bytes needed by the hidden-link opening proof.
-/
namespace N32Q12289CommittedSigmaByteFrontier

open LocallyRecoverableSyndrome.LocalDeltaByteCommitment.N32Q12289

/-- Transaction bytes before adding the committed-sigma proof payload. -/
abbrev baseTransactionBytes : Nat := statefulSpendSignatureBytes

/-- The prechallenge first message `a = L(r)` carried by the signature. -/
abbrev proofCommitmentBytes : Nat := linearSigmaProofCommitmentBytes

/--
Committed-sigma proof payload:

```text
proof_commitment[32] || masked_response[maskedResponseBytes]
|| hidden_link_opening[hiddenLinkOpeningBytes]
```
-/
def proofBytes (maskedResponseBytes hiddenLinkOpeningBytes : Nat) : Nat :=
  proofCommitmentBytes + maskedResponseBytes + hiddenLinkOpeningBytes

def transactionBytes (maskedResponseBytes hiddenLinkOpeningBytes : Nat) : Nat :=
  statefulTransactionWithProofBytes
    (proofBytes maskedResponseBytes hiddenLinkOpeningBytes)

def hiddenLinkBudgetAt (cap maskedResponseBytes : Nat) : Nat :=
  cap - transactionBytes maskedResponseBytes 0

theorem baseTransactionBytes_eq :
    baseTransactionBytes = 187 := by
  native_decide

theorem proofCommitmentBytes_eq :
    proofCommitmentBytes = 32 := by
  native_decide

theorem transactionBytes_eq
    (maskedResponseBytes hiddenLinkOpeningBytes : Nat) :
    transactionBytes maskedResponseBytes hiddenLinkOpeningBytes =
      187 + 32 + maskedResponseBytes + hiddenLinkOpeningBytes := by
  unfold transactionBytes proofBytes proofCommitmentBytes
  rw [statefulTransactionWithProofBytes_eq]
  norm_num [linearSigmaProofCommitmentBytes]
  omega

/--
The current public sigma-equation path has the right algebraic equation but not
the right knowledge surface: the adapted public representative attack still
satisfies it, so it fails the committed-sigma structural gate.
-/
def currentPublicSigmaEquationCandidate : OpeningInstantiationCandidate where
  name := "current-public-sigma-equation"
  signatureOpeningBytes := proofBytes linearSigmaMaskedWitnessBytes 0
  accountSlotBytes := 0
  proverWorkUnits := 0
  verifierWorkUnits := 0
  revealsRandomizer := False
  instantiatesPrechallengeOpeningInterface := False
  postChallengeRepresentativeRejected := False
  witnessHidingArgument := True

theorem currentPublicSigmaEquation_transactionBytes_eq :
    transactionBytes linearSigmaMaskedWitnessBytes 0 = 260 := by
  native_decide

theorem currentPublicSigmaEquationCandidate_not_structurallyAdmissible :
    ¬ currentPublicSigmaEquationCandidate.structurallyAdmissible :=
  OpeningInstantiationCandidate.postChallengeRepresentative_notRejected_not_structurallyAdmissible
    (C := currentPublicSigmaEquationCandidate)
    (by intro h; cases h)

/--
After the AP2-004 challenge widening (23 algebraic + 4 hardening bytes), the
41-byte masked-response width no longer fits below a 256-byte transaction at
all: the zero-hidden-link payload already reaches 260 bytes.  Any sub-256
target therefore needs a narrower response or a different architecture; below
269 bytes at most nine hidden-link bytes remain.
-/
theorem response41_not_under256
    {hiddenLinkOpeningBytes : Nat} :
    ¬ transactionBytes 41 hiddenLinkOpeningBytes ≤ 256 := by
  rw [transactionBytes_eq]
  omega

theorem response41_under269_iff
    {hiddenLinkOpeningBytes : Nat} :
    transactionBytes 41 hiddenLinkOpeningBytes ≤ 269 ↔
      hiddenLinkOpeningBytes ≤ 9 := by
  rw [transactionBytes_eq]
  omega

theorem currentResponse_hiddenLinkBudget_under256_eq :
    hiddenLinkBudgetAt 256 linearSigmaMaskedWitnessBytes = 0 := by
  native_decide

theorem currentResponse_hiddenLinkBudget_under269_eq :
    hiddenLinkBudgetAt 269 linearSigmaMaskedWitnessBytes = 9 := by
  native_decide

/--
With the widened 27-byte hardened challenge, the 37-byte response target lands
exactly on 256 bytes with no extra hidden-link opening bytes: there is no
remaining sub-256 hidden-link budget on this width.
-/
abbrev fiberSaturatedResponseBytes : Nat := 37

theorem fiberSaturatedNoExtra_transactionBytes_eq :
    transactionBytes fiberSaturatedResponseBytes 0 = 256 := by
  native_decide

theorem fiberSaturated_under256_iff
    {hiddenLinkOpeningBytes : Nat} :
    transactionBytes fiberSaturatedResponseBytes hiddenLinkOpeningBytes ≤ 256 ↔
      hiddenLinkOpeningBytes = 0 := by
  rw [transactionBytes_eq]
  change 187 + 32 + 37 + hiddenLinkOpeningBytes ≤ 256 ↔
    hiddenLinkOpeningBytes = 0
  omega

theorem fiberSaturated_hiddenLinkBudget_under256_eq :
    hiddenLinkBudgetAt 256 fiberSaturatedResponseBytes = 0 := by
  native_decide

theorem fiberSaturated_hiddenLinkBudget_under269_eq :
    hiddenLinkBudgetAt 269 fiberSaturatedResponseBytes = 13 := by
  native_decide

/--
The 39-byte response target models the variant that keeps the local fiber-state
equality.  After the challenge widening it overshoots 256 bytes by two even
with no hidden-link bytes, so it no longer has a sub-256 deployment shape.
-/
abbrev fiberStatePreservingResponseBytes : Nat := 39

theorem fiberStatePreservingNoExtra_transactionBytes_eq :
    transactionBytes fiberStatePreservingResponseBytes 0 = 258 := by
  native_decide

theorem fiberStatePreserving_not_under256
    {hiddenLinkOpeningBytes : Nat} :
    ¬ transactionBytes fiberStatePreservingResponseBytes hiddenLinkOpeningBytes ≤
      256 := by
  rw [transactionBytes_eq]
  change ¬ 187 + 32 + 39 + hiddenLinkOpeningBytes ≤ 256
  omega

theorem fiberStatePreserving_hiddenLinkBudget_under256_eq :
    hiddenLinkBudgetAt 256 fiberStatePreservingResponseBytes = 0 := by
  native_decide

/--
This is not automatically deployable: it becomes admissible only after proving
the three hidden-link obligations supplied as arguments.  The byte theorem above
shows why this is the live low-byte target.
-/
def fiberSaturatedHiddenLinkCandidate
    (hiddenLinkOpeningBytes accountBytes proverWork verifierWork : Nat)
    (hInterface hRejects hHiding : Prop) :
    OpeningInstantiationCandidate :=
  OpeningInstantiationCandidate.abstractHiddenLinkProofCandidate
    (proofBytes fiberSaturatedResponseBytes hiddenLinkOpeningBytes)
    accountBytes proverWork verifierWork hInterface hRejects hHiding

theorem fiberSaturatedHiddenLinkCandidate_structurallyAdmissible
    {hiddenLinkOpeningBytes accountBytes proverWork verifierWork : Nat}
    {hInterface hRejects hHiding : Prop}
    (hi : hInterface) (hr : hRejects) (hh : hHiding) :
    (fiberSaturatedHiddenLinkCandidate hiddenLinkOpeningBytes accountBytes
      proverWork verifierWork hInterface hRejects hHiding).structurallyAdmissible :=
  OpeningInstantiationCandidate.abstractHiddenLinkProofCandidate_structurallyAdmissible
    hi hr hh

/--
Byte-level shape for the low-byte stateful route.

`signatureHiddenLinkBytes` are bytes added to the spend signature.  They are the
scarce resource.  `accountHiddenLinkBytes` are persistent slot/account bytes
used to carry commitments, roots, or other prechallenge evidence consumed with
the slot.  This lets the proof target say precisely what the blockchain design
is buying with state.
-/
structure AccountCarriedHiddenLinkInstantiation where
  name : String
  maskedResponseBytes : Nat
  signatureHiddenLinkBytes : Nat
  accountHiddenLinkBytes : Nat
  proverWorkUnits : Nat
  verifierWorkUnits : Nat
  openingInterfaceHolds : Prop
  postChallengeRepresentativeRejected : Prop
  witnessHidingArgument : Prop

namespace AccountCarriedHiddenLinkInstantiation

def proofBytes (I : AccountCarriedHiddenLinkInstantiation) : Nat :=
  N32Q12289CommittedSigmaByteFrontier.proofBytes
    I.maskedResponseBytes I.signatureHiddenLinkBytes

def transactionBytes (I : AccountCarriedHiddenLinkInstantiation) : Nat :=
  N32Q12289CommittedSigmaByteFrontier.transactionBytes
    I.maskedResponseBytes I.signatureHiddenLinkBytes

def toOpeningCandidate
    (I : AccountCarriedHiddenLinkInstantiation) :
    OpeningInstantiationCandidate :=
  OpeningInstantiationCandidate.abstractHiddenLinkProofCandidate
    I.proofBytes I.accountHiddenLinkBytes I.proverWorkUnits
    I.verifierWorkUnits I.openingInterfaceHolds
    I.postChallengeRepresentativeRejected I.witnessHidingArgument

def structurallyAdmissible
    (I : AccountCarriedHiddenLinkInstantiation) : Prop :=
  I.toOpeningCandidate.structurallyAdmissible

theorem structurallyAdmissible_of_obligations
    {I : AccountCarriedHiddenLinkInstantiation}
    (hi : I.openingInterfaceHolds)
    (hr : I.postChallengeRepresentativeRejected)
    (hh : I.witnessHidingArgument) :
    I.structurallyAdmissible :=
  OpeningInstantiationCandidate.abstractHiddenLinkProofCandidate_structurallyAdmissible
    hi hr hh

theorem signature_budget_under256_of_fiberSaturated
    {I : AccountCarriedHiddenLinkInstantiation}
    (hresponse :
      I.maskedResponseBytes =
        N32Q12289CommittedSigmaByteFrontier.fiberSaturatedResponseBytes) :
    I.transactionBytes ≤ 256 ↔ I.signatureHiddenLinkBytes = 0 := by
  unfold transactionBytes
  rw [hresponse]
  exact N32Q12289CommittedSigmaByteFrontier.fiberSaturated_under256_iff

theorem signature_budget_not_under256_of_fiberStatePreserving
    {I : AccountCarriedHiddenLinkInstantiation}
    (hresponse :
      I.maskedResponseBytes =
        N32Q12289CommittedSigmaByteFrontier.fiberStatePreservingResponseBytes) :
    ¬ I.transactionBytes ≤ 256 := by
  unfold transactionBytes
  rw [hresponse]
  exact N32Q12289CommittedSigmaByteFrontier.fiberStatePreserving_not_under256

end AccountCarriedHiddenLinkInstantiation

/--
The live target class after the byte-frontier result: a 37-byte quotient
response with the hidden-link evidence carried by account state and no extra
signature bytes.
-/
def accountCarriedFiberSaturatedTarget
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (hInterface hRejects hHiding : Prop) :
    AccountCarriedHiddenLinkInstantiation where
  name := "account-carried-fiber-saturated-hidden-link"
  maskedResponseBytes := fiberSaturatedResponseBytes
  signatureHiddenLinkBytes := 0
  accountHiddenLinkBytes := accountHiddenLinkBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  openingInterfaceHolds := hInterface
  postChallengeRepresentativeRejected := hRejects
  witnessHidingArgument := hHiding

theorem accountCarriedFiberSaturatedTarget_transactionBytes_eq
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (hInterface hRejects hHiding : Prop) :
    (accountCarriedFiberSaturatedTarget accountHiddenLinkBytes proverWork
      verifierWork hInterface hRejects hHiding).transactionBytes = 256 := by
  unfold AccountCarriedHiddenLinkInstantiation.transactionBytes
  exact fiberSaturatedNoExtra_transactionBytes_eq

theorem accountCarriedFiberSaturatedTarget_under256
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (hInterface hRejects hHiding : Prop) :
    (accountCarriedFiberSaturatedTarget accountHiddenLinkBytes proverWork
      verifierWork hInterface hRejects hHiding).transactionBytes ≤ 256 := by
  rw [accountCarriedFiberSaturatedTarget_transactionBytes_eq]

theorem accountCarriedFiberSaturatedTarget_structurallyAdmissible
    {accountHiddenLinkBytes proverWork verifierWork : Nat}
    {hInterface hRejects hHiding : Prop}
    (hi : hInterface) (hr : hRejects) (hh : hHiding) :
    (accountCarriedFiberSaturatedTarget accountHiddenLinkBytes proverWork
      verifierWork hInterface hRejects hHiding).structurallyAdmissible :=
  AccountCarriedHiddenLinkInstantiation.structurallyAdmissible_of_obligations
    hi hr hh

/--
A bare account-state hash/link commitment is not the hidden-link proof.  If the
serialized opening predicate has not been shown to instantiate
`AccountCarriedHiddenLinkInterface`, the candidate remains structurally rejected
even when it spends account bytes.
-/
def bareAccountHashLinkTarget
    (accountHiddenLinkBytes proverWork verifierWork : Nat) :
    AccountCarriedHiddenLinkInstantiation where
  name := "bare-account-hash-link"
  maskedResponseBytes := fiberSaturatedResponseBytes
  signatureHiddenLinkBytes := 0
  accountHiddenLinkBytes := accountHiddenLinkBytes
  proverWorkUnits := proverWork
  verifierWorkUnits := verifierWork
  openingInterfaceHolds := False
  postChallengeRepresentativeRejected := False
  witnessHidingArgument := True

theorem bareAccountHashLinkTarget_not_structurallyAdmissible
    (accountHiddenLinkBytes proverWork verifierWork : Nat) :
    ¬ (bareAccountHashLinkTarget accountHiddenLinkBytes proverWork
      verifierWork).structurallyAdmissible :=
  OpeningInstantiationCandidate.missingInterface_not_structurallyAdmissible
    (C :=
      (bareAccountHashLinkTarget accountHiddenLinkBytes proverWork
        verifierWork).toOpeningCandidate)
    (by intro h; cases h)

def accountCarriedLinkProofCandidate
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (hSystem hRejects hNonProjection hHiding : Prop) :
    LinkProofInstantiationCandidate :=
  LinkProofInstantiationCandidate.abstractAccountLinkProofCandidate
    0 accountHiddenLinkBytes proverWork verifierWork
    hSystem hRejects hNonProjection hHiding

theorem accountCarriedLinkProofCandidate_signatureBytes_eq
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (hSystem hRejects hNonProjection hHiding : Prop) :
    (accountCarriedLinkProofCandidate accountHiddenLinkBytes proverWork
      verifierWork hSystem hRejects hNonProjection
      hHiding).signatureLinkProofBytes = 0 := by
  rfl

theorem accountCarriedLinkProofCandidate_structurallyAdmissible
    {accountHiddenLinkBytes proverWork verifierWork : Nat}
    {hSystem hRejects hNonProjection hHiding : Prop}
    (hs : hSystem) (hr : hRejects) (hn : hNonProjection) (hh : hHiding) :
    (accountCarriedLinkProofCandidate accountHiddenLinkBytes proverWork
      verifierWork hSystem hRejects hNonProjection
      hHiding).structurallyAdmissible :=
  LinkProofInstantiationCandidate.abstractAccountLinkProofCandidate_structurallyAdmissible
    hs hr hn hh

def accountCarriedBoundedKernelLinkProofCandidate
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (hRejects hNonProjection hHiding : Prop) :
    LinkProofInstantiationCandidate :=
  LinkProofInstantiationCandidate.fromBoundedKernelDeploymentObligations S O
    0 accountHiddenLinkBytes proverWork verifierWork
    hRejects hNonProjection hHiding

theorem accountCarriedBoundedKernelLinkProofCandidate_signatureBytes_eq
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (hRejects hNonProjection hHiding : Prop) :
    (accountCarriedBoundedKernelLinkProofCandidate S O accountHiddenLinkBytes
      proverWork verifierWork hRejects hNonProjection
      hHiding).signatureLinkProofBytes = 0 := by
  rfl

theorem accountCarriedBoundedKernelLinkProofCandidate_structurallyAdmissible
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    {S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed}
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    {accountHiddenLinkBytes proverWork verifierWork : Nat}
    {hRejects hNonProjection hHiding : Prop}
    (hr : hRejects) (hn : hNonProjection) (hh : hHiding) :
    (accountCarriedBoundedKernelLinkProofCandidate S O accountHiddenLinkBytes
      proverWork verifierWork hRejects hNonProjection
      hHiding).structurallyAdmissible :=
  LinkProofInstantiationCandidate.fromBoundedKernelDeploymentObligations_structurallyAdmissible
    O hr hn hh

def accountCarriedBoundedKernelFiberSaturatedTarget
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (hRejects hHiding : Prop) :
    AccountCarriedHiddenLinkInstantiation :=
  accountCarriedFiberSaturatedTarget accountHiddenLinkBytes proverWork
    verifierWork
    (Nonempty
      (AccountCarriedAccountOpeningSoundnessCertificate A S.witnessCommit
        S.randomizerCommit S.hiddenLinkCommit S.proofCommitmentOfRandomizer
        accountSlotWellFormed (O.toAccountLinkProofSystem S).accountOpeningAccepts
        (fun Tfull w r =>
          S.localOpeningRelation
            (AccountCarriedOpening.localTranscript Tfull) w r)))
    hRejects hHiding

theorem accountCarriedBoundedKernelFiberSaturatedTarget_transactionBytes_eq
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (hRejects hHiding : Prop) :
    (accountCarriedBoundedKernelFiberSaturatedTarget S O
      accountHiddenLinkBytes proverWork verifierWork hRejects
      hHiding).transactionBytes = 256 := by
  exact
    accountCarriedFiberSaturatedTarget_transactionBytes_eq
      accountHiddenLinkBytes proverWork verifierWork
      (Nonempty
        (AccountCarriedAccountOpeningSoundnessCertificate A S.witnessCommit
          S.randomizerCommit S.hiddenLinkCommit S.proofCommitmentOfRandomizer
          accountSlotWellFormed
          (O.toAccountLinkProofSystem S).accountOpeningAccepts
          (fun Tfull w r =>
            S.localOpeningRelation
              (AccountCarriedOpening.localTranscript Tfull) w r)))
      hRejects hHiding

theorem accountCarriedBoundedKernelFiberSaturatedTarget_under256
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (hRejects hHiding : Prop) :
    (accountCarriedBoundedKernelFiberSaturatedTarget S O
      accountHiddenLinkBytes proverWork verifierWork hRejects
      hHiding).transactionBytes ≤ 256 := by
  rw [accountCarriedBoundedKernelFiberSaturatedTarget_transactionBytes_eq]

theorem accountCarriedBoundedKernelFiberSaturatedTarget_structurallyAdmissible
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    {S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed}
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    {accountHiddenLinkBytes proverWork verifierWork : Nat}
    {hRejects hHiding : Prop}
    (hr : hRejects) (hh : hHiding) :
    (accountCarriedBoundedKernelFiberSaturatedTarget S O
      accountHiddenLinkBytes proverWork verifierWork hRejects
      hHiding).structurallyAdmissible :=
  accountCarriedFiberSaturatedTarget_structurallyAdmissible
    ⟨O.toAccountOpeningSoundnessCertificate S⟩ hr hh

/--
Remaining structural gates for the bounded-kernel N32/q12289 target.

After `BoundedKernelDeploymentObligations` supplies the actual account-link
proof system, the byte-frontier row still needs these review facts: public
post-challenge representatives are rejected, the link is not a public projection
of the sigma equation, and the account-carried link evidence is witness hiding.
-/
structure BoundedKernelTargetStructuralGates where
  postChallengeRepresentativeRejected : Prop
  nonPublicProjectionOfSigmaEquation : Prop
  witnessHidingArgument : Prop

namespace BoundedKernelTargetStructuralGates

def allHold (G : BoundedKernelTargetStructuralGates) : Prop :=
  G.postChallengeRepresentativeRejected ∧
  G.nonPublicProjectionOfSigmaEquation ∧
  G.witnessHidingArgument

end BoundedKernelTargetStructuralGates

/--
Named break events for the three remaining structural gates.  This is the
review-facing version of `BoundedKernelTargetStructuralGates`: each abstract
gate must either be proved for the selected account commitment representation
or charged to one of these concrete audit buckets.
-/
structure BoundedKernelTargetStructuralBreaks where
  postChallengeRepresentativeBreak : Prop
  publicProjectionBreak : Prop
  witnessHidingBreak : Prop

namespace BoundedKernelTargetStructuralBreaks

def any (B : BoundedKernelTargetStructuralBreaks) : Prop :=
  B.postChallengeRepresentativeBreak ∨
  B.publicProjectionBreak ∨
  B.witnessHidingBreak

theorem any_of_postChallengeRepresentativeBreak
    {B : BoundedKernelTargetStructuralBreaks}
    (h : B.postChallengeRepresentativeBreak) : B.any :=
  Or.inl h

theorem any_of_publicProjectionBreak
    {B : BoundedKernelTargetStructuralBreaks}
    (h : B.publicProjectionBreak) : B.any :=
  Or.inr (Or.inl h)

theorem any_of_witnessHidingBreak
    {B : BoundedKernelTargetStructuralBreaks}
    (h : B.witnessHidingBreak) : B.any :=
  Or.inr (Or.inr h)

end BoundedKernelTargetStructuralBreaks

/--
Audit certificate for the remaining structural gates.

The fields are intentionally disjunctive: a concrete instantiation can discharge
each gate directly, or identify the exact break game that would be needed for a
counterexample.  The no-break theorem below turns this review object into the
plain `BoundedKernelTargetStructuralGates.allHold` fact consumed by the byte
frontier.
-/
structure BoundedKernelTargetStructuralReview where
  breaks : BoundedKernelTargetStructuralBreaks
  gates : BoundedKernelTargetStructuralGates
  representativeRejected_or_break :
    gates.postChallengeRepresentativeRejected ∨
      breaks.postChallengeRepresentativeBreak
  nonPublicProjection_or_break :
    gates.nonPublicProjectionOfSigmaEquation ∨ breaks.publicProjectionBreak
  witnessHiding_or_break :
    gates.witnessHidingArgument ∨ breaks.witnessHidingBreak

namespace BoundedKernelTargetStructuralReview

theorem allHold_of_noBreaks
    (R : BoundedKernelTargetStructuralReview)
    (hNoBreaks : ¬ R.breaks.any) :
    R.gates.allHold := by
  constructor
  · rcases R.representativeRejected_or_break with hGate | hBreak
    · exact hGate
    · exact False.elim
        (hNoBreaks
          (BoundedKernelTargetStructuralBreaks.any_of_postChallengeRepresentativeBreak
            hBreak))
  · constructor
    · rcases R.nonPublicProjection_or_break with hGate | hBreak
      · exact hGate
      · exact False.elim
          (hNoBreaks
            (BoundedKernelTargetStructuralBreaks.any_of_publicProjectionBreak
              hBreak))
    · rcases R.witnessHiding_or_break with hGate | hBreak
      · exact hGate
      · exact False.elim
          (hNoBreaks
            (BoundedKernelTargetStructuralBreaks.any_of_witnessHidingBreak
              hBreak))

end BoundedKernelTargetStructuralReview

def accountCarriedBoundedKernelLinkProofCandidateOfGates
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (G : BoundedKernelTargetStructuralGates) :
    LinkProofInstantiationCandidate :=
  accountCarriedBoundedKernelLinkProofCandidate S O accountHiddenLinkBytes
    proverWork verifierWork G.postChallengeRepresentativeRejected
    G.nonPublicProjectionOfSigmaEquation G.witnessHidingArgument

def accountCarriedBoundedKernelFiberSaturatedTargetOfGates
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (G : BoundedKernelTargetStructuralGates) :
    AccountCarriedHiddenLinkInstantiation :=
  accountCarriedBoundedKernelFiberSaturatedTarget S O accountHiddenLinkBytes
    proverWork verifierWork G.postChallengeRepresentativeRejected
    G.witnessHidingArgument

def accountCarriedBoundedKernelLinkProofCandidateOfReview
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (R : BoundedKernelTargetStructuralReview) :
    LinkProofInstantiationCandidate :=
  accountCarriedBoundedKernelLinkProofCandidateOfGates S O
    accountHiddenLinkBytes proverWork verifierWork R.gates

def accountCarriedBoundedKernelFiberSaturatedTargetOfReview
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (R : BoundedKernelTargetStructuralReview) :
    AccountCarriedHiddenLinkInstantiation :=
  accountCarriedBoundedKernelFiberSaturatedTargetOfGates S O
    accountHiddenLinkBytes proverWork verifierWork R.gates

theorem accountCarriedBoundedKernelLinkProofCandidateOfGates_structurallyAdmissible
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    {S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed}
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    {accountHiddenLinkBytes proverWork verifierWork : Nat}
    {G : BoundedKernelTargetStructuralGates}
    (hG : G.allHold) :
    (accountCarriedBoundedKernelLinkProofCandidateOfGates S O
      accountHiddenLinkBytes proverWork verifierWork G).structurallyAdmissible :=
  accountCarriedBoundedKernelLinkProofCandidate_structurallyAdmissible O
    hG.1 hG.2.1 hG.2.2

theorem accountCarriedBoundedKernelLinkProofCandidateOfReview_structurallyAdmissible
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    {S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed}
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    {accountHiddenLinkBytes proverWork verifierWork : Nat}
    {R : BoundedKernelTargetStructuralReview}
    (hNoBreaks : ¬ R.breaks.any) :
    (accountCarriedBoundedKernelLinkProofCandidateOfReview S O
      accountHiddenLinkBytes proverWork verifierWork R).structurallyAdmissible :=
  accountCarriedBoundedKernelLinkProofCandidateOfGates_structurallyAdmissible
    O (R.allHold_of_noBreaks hNoBreaks)

theorem accountCarriedBoundedKernelFiberSaturatedTargetOfGates_transactionBytes_eq
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (G : BoundedKernelTargetStructuralGates) :
    (accountCarriedBoundedKernelFiberSaturatedTargetOfGates S O
      accountHiddenLinkBytes proverWork verifierWork G).transactionBytes = 256 :=
  accountCarriedBoundedKernelFiberSaturatedTarget_transactionBytes_eq S O
    accountHiddenLinkBytes proverWork verifierWork
    G.postChallengeRepresentativeRejected G.witnessHidingArgument

theorem accountCarriedBoundedKernelFiberSaturatedTargetOfReview_transactionBytes_eq
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (R : BoundedKernelTargetStructuralReview) :
    (accountCarriedBoundedKernelFiberSaturatedTargetOfReview S O
      accountHiddenLinkBytes proverWork verifierWork R).transactionBytes = 256 :=
  accountCarriedBoundedKernelFiberSaturatedTargetOfGates_transactionBytes_eq
    S O accountHiddenLinkBytes proverWork verifierWork R.gates

theorem accountCarriedBoundedKernelFiberSaturatedTargetOfGates_structurallyAdmissible
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    {S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed}
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    {accountHiddenLinkBytes proverWork verifierWork : Nat}
    {G : BoundedKernelTargetStructuralGates}
    (hG : G.allHold) :
    (accountCarriedBoundedKernelFiberSaturatedTargetOfGates S O
      accountHiddenLinkBytes proverWork verifierWork G).structurallyAdmissible :=
  accountCarriedBoundedKernelFiberSaturatedTarget_structurallyAdmissible O
    hG.1 hG.2.2

theorem accountCarriedBoundedKernelFiberSaturatedTargetOfReview_structurallyAdmissible
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    {S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed}
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    {accountHiddenLinkBytes proverWork verifierWork : Nat}
    {R : BoundedKernelTargetStructuralReview}
    (hNoBreaks : ¬ R.breaks.any) :
    (accountCarriedBoundedKernelFiberSaturatedTargetOfReview S O
      accountHiddenLinkBytes proverWork verifierWork R).structurallyAdmissible :=
  accountCarriedBoundedKernelFiberSaturatedTargetOfGates_structurallyAdmissible
    O (R.allHold_of_noBreaks hNoBreaks)

/--
One reviewed account-link instantiation for the bounded-kernel N32/q12289
route.

This bundles the three objects that a concrete account commitment
representation must provide:

* a pre-response backing system for local/account openings;
* bounded-kernel deployment obligations for the response-link code;
* structural review gates for representative rejection, non-public projection,
  and witness hiding.
-/
structure ReviewedBoundedKernelAccountLinkInstantiation
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop)
    (responseCommit : Response -> ResponseCommitment) where
  preResponseSystem :
    AccountPreResponseBackingSystem
      (Witness := Witness) (Randomizer := Randomizer)
      (StatementDigest := StatementDigest)
      (WitnessCommitment := WitnessCommitment)
      (RandomizerCommitment := RandomizerCommitment)
      (LinkCommitment := LinkCommitment)
      (Challenge := Challenge) (Response := Response)
      (LocalOpening := LocalOpening) (LinkProof := LinkProof)
      A accountSlotWellFormed
  deployment :
    AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations
      preResponseSystem responseCommit
  structuralReview : BoundedKernelTargetStructuralReview

namespace ReviewedBoundedKernelAccountLinkInstantiation

variable {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
  LinkCommitment Challenge Response LocalOpening LinkProof
  ResponseCommitment : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {accountSlotWellFormed :
  AccountCarriedSlot StatementDigest WitnessCommitment RandomizerCommitment
    LinkCommitment -> Prop}
variable {responseCommit : Response -> ResponseCommitment}

def anyBreak
    (I :
      ReviewedBoundedKernelAccountLinkInstantiation (LocalOpening := LocalOpening)
        (LinkProof := LinkProof) A accountSlotWellFormed
        responseCommit) : Prop :=
  I.preResponseSystem.preResponseBackingBreak ∨
  I.deployment.linkKernelBreak ∨
  I.structuralReview.breaks.any

theorem noPreResponseBreak_of_noBreak
    {I :
      ReviewedBoundedKernelAccountLinkInstantiation (LocalOpening := LocalOpening)
        (LinkProof := LinkProof) A accountSlotWellFormed
        responseCommit}
    (hNoBreak : ¬ I.anyBreak) :
    ¬ I.preResponseSystem.preResponseBackingBreak := by
  intro h
  exact hNoBreak (Or.inl h)

theorem noLinkKernelBreak_of_noBreak
    {I :
      ReviewedBoundedKernelAccountLinkInstantiation (LocalOpening := LocalOpening)
        (LinkProof := LinkProof) A accountSlotWellFormed
        responseCommit}
    (hNoBreak : ¬ I.anyBreak) :
    ¬ I.deployment.linkKernelBreak := by
  intro h
  exact hNoBreak (Or.inr (Or.inl h))

theorem noStructuralBreaks_of_noBreak
    {I :
      ReviewedBoundedKernelAccountLinkInstantiation (LocalOpening := LocalOpening)
        (LinkProof := LinkProof) A accountSlotWellFormed
        responseCommit}
    (hNoBreak : ¬ I.anyBreak) :
    ¬ I.structuralReview.breaks.any := by
  intro h
  exact hNoBreak (Or.inr (Or.inr h))

def accountLinkProofSystem
    (I :
      ReviewedBoundedKernelAccountLinkInstantiation (LocalOpening := LocalOpening)
        (LinkProof := LinkProof) A accountSlotWellFormed
        responseCommit) :
    AccountLinkProofSystem
      (Witness := Witness) (Randomizer := Randomizer)
      (StatementDigest := StatementDigest)
      (WitnessCommitment := WitnessCommitment)
      (RandomizerCommitment := RandomizerCommitment)
      (LinkCommitment := LinkCommitment)
      (Challenge := Challenge) (Response := Response)
      (LocalOpening := LocalOpening) (LinkProof := LinkProof)
      A accountSlotWellFormed :=
  I.deployment.toAccountLinkProofSystem I.preResponseSystem

def target
    (I :
      ReviewedBoundedKernelAccountLinkInstantiation (LocalOpening := LocalOpening)
        (LinkProof := LinkProof) A accountSlotWellFormed
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat) :
    AccountCarriedHiddenLinkInstantiation :=
  accountCarriedBoundedKernelFiberSaturatedTargetOfReview
    I.preResponseSystem I.deployment accountHiddenLinkBytes proverWork
    verifierWork I.structuralReview

theorem target_transactionBytes_eq
    (I :
      ReviewedBoundedKernelAccountLinkInstantiation (LocalOpening := LocalOpening)
        (LinkProof := LinkProof) A accountSlotWellFormed
        responseCommit)
    (accountHiddenLinkBytes proverWork verifierWork : Nat) :
    (I.target accountHiddenLinkBytes proverWork verifierWork).transactionBytes =
      256 :=
  accountCarriedBoundedKernelFiberSaturatedTargetOfReview_transactionBytes_eq
    I.preResponseSystem I.deployment accountHiddenLinkBytes proverWork
    verifierWork I.structuralReview

theorem target_structurallyAdmissible_of_noBreak
    {I :
      ReviewedBoundedKernelAccountLinkInstantiation (LocalOpening := LocalOpening)
        (LinkProof := LinkProof) A accountSlotWellFormed
        responseCommit}
    (hNoBreak : ¬ I.anyBreak)
    {accountHiddenLinkBytes proverWork verifierWork : Nat} :
    (I.target accountHiddenLinkBytes proverWork verifierWork).structurallyAdmissible :=
  accountCarriedBoundedKernelFiberSaturatedTargetOfReview_structurallyAdmissible
    I.deployment (I.noStructuralBreaks_of_noBreak hNoBreak)

theorem accepts_accountBacked_of_noBreak
    {I :
      ReviewedBoundedKernelAccountLinkInstantiation (LocalOpening := LocalOpening)
        (LinkProof := LinkProof) A accountSlotWellFormed
        responseCommit}
    (hNoBreak : ¬ I.anyBreak)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof))
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        I.accountLinkProofSystem.accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A I.preResponseSystem.witnessCommit
      I.preResponseSystem.randomizerCommit
      I.preResponseSystem.proofCommitmentOfRandomizer
      I.preResponseSystem.hiddenLinkCommit
      (fun Tfull w r =>
        I.preResponseSystem.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T :=
  I.deployment.accepts_accountBacked_of_noBreaks I.preResponseSystem
    (I.noPreResponseBreak_of_noBreak hNoBreak)
    (I.noLinkKernelBreak_of_noBreak hNoBreak) T hAccept

theorem accepts_accountBacked_and_target_of_noBreak
    {I :
      ReviewedBoundedKernelAccountLinkInstantiation (LocalOpening := LocalOpening)
        (LinkProof := LinkProof) A accountSlotWellFormed
        responseCommit}
    (hNoBreak : ¬ I.anyBreak)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof))
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        I.accountLinkProofSystem.accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A I.preResponseSystem.witnessCommit
      I.preResponseSystem.randomizerCommit
      I.preResponseSystem.proofCommitmentOfRandomizer
      I.preResponseSystem.hiddenLinkCommit
      (fun Tfull w r =>
        I.preResponseSystem.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T ∧
    (I.target accountHiddenLinkBytes proverWork verifierWork).structurallyAdmissible ∧
    (I.target accountHiddenLinkBytes proverWork verifierWork).transactionBytes =
      256 := by
  constructor
  · exact I.accepts_accountBacked_of_noBreak hNoBreak T hAccept
  · constructor
    · exact I.target_structurallyAdmissible_of_noBreak hNoBreak
    · exact I.target_transactionBytes_eq accountHiddenLinkBytes proverWork verifierWork

end ReviewedBoundedKernelAccountLinkInstantiation

/--
Concrete account-link byte layout for the current N32/q12289 reviewed route.

The slot-side link state is modeled as

```text
witness commitment   : 32 bytes
randomizer commitment: 32 bytes
response-link code   :  4 bytes  -- two q-residues / coordinates [0, 3]
```

These are account/state bytes, not spend-signature bytes.
-/
abbrev reviewedAccountWitnessCommitmentBytes : Nat := sha256DigestBytes
abbrev reviewedAccountRandomizerCommitmentBytes : Nat := sha256DigestBytes
abbrev reviewedAccountResponseLinkCodeBytes : Nat := 4

def reviewedAccountHiddenLinkBytes : Nat :=
  reviewedAccountWitnessCommitmentBytes +
  reviewedAccountRandomizerCommitmentBytes +
  reviewedAccountResponseLinkCodeBytes

theorem reviewedAccountHiddenLinkBytes_eq :
    reviewedAccountHiddenLinkBytes = 68 := by
  native_decide

/--
Production-facing representation contract for the current account-carried
candidate.

This does not assert that the byte commitments are secure by definition.  It
ties a reviewed proof instantiation to the exact account-state byte layout that
the blockchain benchmark should price.
-/
structure ConcreteReviewedAccountCommitmentRepresentation
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    (A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest)
    (accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop)
    (responseCommit : Response -> ResponseCommitment) where
  instantiation :
    ReviewedBoundedKernelAccountLinkInstantiation
      (LocalOpening := LocalOpening) (LinkProof := LinkProof)
      A accountSlotWellFormed responseCommit
  witnessCommitmentBytes_eq :
    reviewedAccountWitnessCommitmentBytes = sha256DigestBytes
  randomizerCommitmentBytes_eq :
    reviewedAccountRandomizerCommitmentBytes = sha256DigestBytes
  responseLinkCodeBytes_eq :
    reviewedAccountResponseLinkCodeBytes = 4
  accountHiddenLinkBytes_eq :
    reviewedAccountHiddenLinkBytes =
      reviewedAccountWitnessCommitmentBytes +
      reviewedAccountRandomizerCommitmentBytes +
      reviewedAccountResponseLinkCodeBytes

namespace ConcreteReviewedAccountCommitmentRepresentation

variable {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
  LinkCommitment Challenge Response LocalOpening LinkProof
  ResponseCommitment : Type*}
variable {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
variable {accountSlotWellFormed :
  AccountCarriedSlot StatementDigest WitnessCommitment RandomizerCommitment
    LinkCommitment -> Prop}
variable {responseCommit : Response -> ResponseCommitment}

def anyBreak
    (C :
      ConcreteReviewedAccountCommitmentRepresentation
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed responseCommit) : Prop :=
  C.instantiation.anyBreak

def accountLinkProofSystem
    (C :
      ConcreteReviewedAccountCommitmentRepresentation
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed responseCommit) :
    AccountLinkProofSystem
      (Witness := Witness) (Randomizer := Randomizer)
      (StatementDigest := StatementDigest)
      (WitnessCommitment := WitnessCommitment)
      (RandomizerCommitment := RandomizerCommitment)
      (LinkCommitment := LinkCommitment)
      (Challenge := Challenge) (Response := Response)
      (LocalOpening := LocalOpening) (LinkProof := LinkProof)
      A accountSlotWellFormed :=
  C.instantiation.accountLinkProofSystem

def target
    (C :
      ConcreteReviewedAccountCommitmentRepresentation
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed responseCommit)
    (proverWork verifierWork : Nat) :
    AccountCarriedHiddenLinkInstantiation :=
  C.instantiation.target reviewedAccountHiddenLinkBytes proverWork verifierWork

theorem accountHiddenLinkBytes_eq_68 :
    reviewedAccountHiddenLinkBytes = 68 :=
  reviewedAccountHiddenLinkBytes_eq

theorem target_transactionBytes_eq
    (C :
      ConcreteReviewedAccountCommitmentRepresentation
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed responseCommit)
    (proverWork verifierWork : Nat) :
    (C.target proverWork verifierWork).transactionBytes = 256 :=
  C.instantiation.target_transactionBytes_eq reviewedAccountHiddenLinkBytes
    proverWork verifierWork

theorem target_signatureLinkBytes_eq
    (C :
      ConcreteReviewedAccountCommitmentRepresentation
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed responseCommit)
    (proverWork verifierWork : Nat) :
    (C.target proverWork verifierWork).signatureHiddenLinkBytes = 0 := by
  rfl

theorem target_structurallyAdmissible_of_noBreak
    {C :
      ConcreteReviewedAccountCommitmentRepresentation
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed responseCommit}
    (hNoBreak : ¬ C.anyBreak)
    {proverWork verifierWork : Nat} :
    (C.target proverWork verifierWork).structurallyAdmissible :=
  C.instantiation.target_structurallyAdmissible_of_noBreak hNoBreak

theorem target_accountHiddenLinkBytes_eq
    (C :
      ConcreteReviewedAccountCommitmentRepresentation
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed responseCommit)
    (proverWork verifierWork : Nat) :
    (C.target proverWork verifierWork).accountHiddenLinkBytes = 68 := by
  rw [target]
  exact reviewedAccountHiddenLinkBytes_eq

theorem target_onlinePlusAccountBytes_eq
    (C :
      ConcreteReviewedAccountCommitmentRepresentation
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed responseCommit)
    (proverWork verifierWork : Nat) :
    (C.target proverWork verifierWork).transactionBytes +
      (C.target proverWork verifierWork).accountHiddenLinkBytes = 324 := by
  rw [target_transactionBytes_eq C proverWork verifierWork,
    target_accountHiddenLinkBytes_eq C proverWork verifierWork]

theorem accepts_accountBacked_and_target_of_noBreak
    {C :
      ConcreteReviewedAccountCommitmentRepresentation
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed responseCommit}
    (hNoBreak : ¬ C.anyBreak)
    (proverWork verifierWork : Nat)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof))
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        C.accountLinkProofSystem.accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A
      C.instantiation.preResponseSystem.witnessCommit
      C.instantiation.preResponseSystem.randomizerCommit
      C.instantiation.preResponseSystem.proofCommitmentOfRandomizer
      C.instantiation.preResponseSystem.hiddenLinkCommit
      (fun Tfull w r =>
        C.instantiation.preResponseSystem.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T ∧
    (C.target proverWork verifierWork).structurallyAdmissible ∧
    (C.target proverWork verifierWork).transactionBytes = 256 := by
  exact
    C.instantiation.accepts_accountBacked_and_target_of_noBreak hNoBreak
      reviewedAccountHiddenLinkBytes proverWork verifierWork T hAccept

end ConcreteReviewedAccountCommitmentRepresentation

/--
Combined reviewed target theorem for the 256-byte account-carried row.

This is the deployment-facing shape produced by the current proof architecture:
bounded-kernel deployment obligations handle the accepting transcript, while the
structural review object handles the byte-frontier admissibility gates.  Under
the named no-break hypotheses, an accepting account-aware verifier transcript
is backed by the hidden account witness/randomizer and the advertised 256-byte
target is structurally admissible.
-/
theorem accountCarriedBoundedKernelReviewedTarget_accepts_accountBacked_and_structurallyAdmissible
    {Witness Randomizer StatementDigest WitnessCommitment RandomizerCommitment
      LinkCommitment Challenge Response LocalOpening LinkProof
      ResponseCommitment : Type*}
    {A : LinearSigmaAlgebra Witness Randomizer Challenge Response StatementDigest}
    {accountSlotWellFormed :
      AccountCarriedSlot StatementDigest WitnessCommitment
        RandomizerCommitment LinkCommitment -> Prop}
    (S :
      AccountPreResponseBackingSystem
        (Witness := Witness) (Randomizer := Randomizer)
        (StatementDigest := StatementDigest)
        (WitnessCommitment := WitnessCommitment)
        (RandomizerCommitment := RandomizerCommitment)
        (LinkCommitment := LinkCommitment)
        (Challenge := Challenge) (Response := Response)
        (LocalOpening := LocalOpening) (LinkProof := LinkProof)
        A accountSlotWellFormed)
    {responseCommit : Response -> ResponseCommitment}
    (O :
      AccountPreResponseBackingSystem.BoundedKernelDeploymentObligations S
        responseCommit)
    (R : BoundedKernelTargetStructuralReview)
    (hNoPreResponseBreak : ¬ S.preResponseBackingBreak)
    (hNoLinkKernelBreak : ¬ O.linkKernelBreak)
    (hNoStructuralBreaks : ¬ R.breaks.any)
    (accountHiddenLinkBytes proverWork verifierWork : Nat)
    (T :
      AccountCarriedTranscript StatementDigest WitnessCommitment StatementDigest
        RandomizerCommitment LinkCommitment Challenge Response
        (AccountCarriedOpening LocalOpening LinkProof))
    (hAccept :
      AccountCarriedVerifierAcceptsWithAccountOpening A accountSlotWellFormed
        (O.toAccountLinkProofSystem S).accountOpeningAccepts T) :
    AccountBackedByHiddenWitness A S.witnessCommit S.randomizerCommit
      S.proofCommitmentOfRandomizer S.hiddenLinkCommit
      (fun Tfull w r =>
        S.localOpeningRelation
          (AccountCarriedOpening.localTranscript Tfull) w r) T ∧
    (accountCarriedBoundedKernelFiberSaturatedTargetOfReview S O
      accountHiddenLinkBytes proverWork verifierWork R).structurallyAdmissible ∧
    (accountCarriedBoundedKernelFiberSaturatedTargetOfReview S O
      accountHiddenLinkBytes proverWork verifierWork R).transactionBytes = 256 := by
  constructor
  · exact
      O.accepts_accountBacked_of_noBreaks S hNoPreResponseBreak
        hNoLinkKernelBreak T hAccept
  · constructor
    · exact
        accountCarriedBoundedKernelFiberSaturatedTargetOfReview_structurallyAdmissible
          O hNoStructuralBreaks
    · exact
        accountCarriedBoundedKernelFiberSaturatedTargetOfReview_transactionBytes_eq
          S O accountHiddenLinkBytes proverWork verifierWork R

end N32Q12289CommittedSigmaByteFrontier

end CommittedSigma
end Descender
