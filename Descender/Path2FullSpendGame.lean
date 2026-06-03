namespace Descender

/-!
# Path 2 Full Spend Game Boundary

This module records the direct-mode full-spend lifecycle interface used by the
Path 2 proof plan.  It is deliberately abstract: concrete byte encodings,
checked-core equations, creation-relation soundness, registry implementations,
and chain rollback semantics are supplied by lower-level modules or deployment
models.

The key boundary is `account_creation_prevalidated_slot_authorization_or_break`:
an accepted direct Path 2 spend yields the exact prechallenge admitted leaf and
the exact consume-once state transition, unless the run exports a named break.
-/

namespace Path2FullSpendGame

universe u

/-- Type family for one direct-mode Path 2 full-spend game instance. -/
structure Types where
  PublicKey : Type u
  Message : Type u
  AccountId : Type u
  StateRoot : Type u
  SlotId : Type u
  SignatureBody : Type u
  PrevalidatedSlotLeaf : Type u
  AccountCreationEvidence : Type u
  DirectReplayableRelationCertificate : Type u
  AdmissionRecord : Type u
  ConsumeState : Type u
  ChallengeTime : Type u
  Digest : Type u

/--
The accepted spend tuple named by the direct-mode theorem target.

`accountStateRootBefore` is the root used for direct admission lookup.  The
`slotAuthorizationDigest` is the leaf-framed digest consumed by both checked-core
challenge binding and the one-use lifecycle transition.
-/
structure SpendTuple (T : Types.{u}) where
  publicKey : T.PublicKey
  message : T.Message
  accountId : T.AccountId
  accountStateRootBefore : T.StateRoot
  slotId : T.SlotId
  signatureBody : T.SignatureBody
  leaf : T.PrevalidatedSlotLeaf
  creationEvidence : T.AccountCreationEvidence
  directCertificate : T.DirectReplayableRelationCertificate
  admissionRecord : T.AdmissionRecord
  consumeStateBefore : T.ConsumeState
  consumeStateAfter : T.ConsumeState
  spendChallengeTime : T.ChallengeTime
  slotAuthorizationDigest : T.Digest

/-- Named bad events for the direct full-spend lifecycle lift. -/
inductive BreakEvent where
  | creationTimeHiddenSourceSoundnessBreak
  | prevalidationLeafBindingBreak
  | prevalidationRegistryBindingBreak
  | prevalidationDataAvailabilityBreak
  | postchallengeSlotCreationOrRewriteBreak
  | checkedCoreMessageBindingBreak
  | slotReplayOrRollbackBreak
  | qromOrDomainSeparationBreak
  | liveSlotLeakageOrKeygenBreak
  deriving DecidableEq

/--
Predicates supplied by a concrete direct-mode game implementation.

The fields mirror the audit boundary rather than a particular Rust encoding:
admission is scoped by `(account_id, account_state_root_before,
slot_authorization_digest)`, timing is relative to the checked-core challenge,
and consumption names the exact digest inserted into the successor lifecycle
state.
-/
structure Predicates (T : Types.{u}) where
  verifyPath2SpendAuthorizationAccepts : SpendTuple T -> Prop
  leafCanonicalAndLive : SpendTuple T -> Prop
  leafEvidenceCertificateBinding : SpendTuple T -> Prop
  slotAuthorizationDigestFramesLeaf : SpendTuple T -> Prop
  replayableCertificateDataAvailable : SpendTuple T -> Prop
  directAdmissionContains :
    T.AdmissionRecord ->
      T.AccountId -> T.StateRoot -> T.Digest -> T.PrevalidatedSlotLeaf -> Prop
  creationCertificateAcceptedBeforeChallenge :
    T.DirectReplayableRelationCertificate ->
      T.PrevalidatedSlotLeaf -> T.ChallengeTime -> Prop
  leafFixedBeforeChallenge :
    T.PrevalidatedSlotLeaf -> T.ChallengeTime -> Prop
  checkedCoreChallengeBindsAcceptedTuple : SpendTuple T -> Prop
  consumeOnceTransition :
    T.ConsumeState -> T.ConsumeState -> T.Digest -> Prop
  validConsumeSuccessor :
    T.ConsumeState -> T.ConsumeState -> Prop
  breakFor : SpendTuple T -> BreakEvent -> Prop

variable {T : Types.{u}}

/-- Some named direct-mode bad event occurred for this spend tuple. -/
def HasBreak (P : Predicates T) (spend : SpendTuple T) : Prop :=
  exists breakEvent, P.breakFor spend breakEvent

/-- The fixed leaf/certificate/digest object passed by local verifier checks. -/
def BoundPrevalidatedSlot (P : Predicates T) (spend : SpendTuple T) : Prop :=
  P.leafCanonicalAndLive spend ∧
    P.leafEvidenceCertificateBinding spend ∧
      P.slotAuthorizationDigestFramesLeaf spend ∧
        P.replayableCertificateDataAvailable spend

/--
The prechallenge admission fact needed by the accepted-spend reduction.

This combines the direct namespace lookup with the timing-game rule: the same
leaf and direct certificate were fixed before the checked-core challenge for the
accepted spend.
-/
def AdmittedPrechallengeLeaf (P : Predicates T) (spend : SpendTuple T) : Prop :=
  P.directAdmissionContains spend.admissionRecord spend.accountId
      spend.accountStateRootBefore spend.slotAuthorizationDigest spend.leaf ∧
    P.creationCertificateAcceptedBeforeChallenge spend.directCertificate
      spend.leaf spend.spendChallengeTime ∧
      P.leafFixedBeforeChallenge spend.leaf spend.spendChallengeTime

/-- The exact digest is consumed once and yields a valid successor state. -/
def ConsumeOnceAcceptedTransition
    (P : Predicates T) (spend : SpendTuple T) : Prop :=
  P.consumeOnceTransition spend.consumeStateBefore spend.consumeStateAfter
      spend.slotAuthorizationDigest ∧
    P.validConsumeSuccessor spend.consumeStateBefore spend.consumeStateAfter

/--
Accepted-spend conclusion used by the full-signature lift.

The checked-core binding conjunct keeps the accepted message/account/root/slot
surface attached to the same digest that admission and lifecycle consume.
-/
def AcceptedSpendConclusion (P : Predicates T) (spend : SpendTuple T) : Prop :=
  AdmittedPrechallengeLeaf P spend ∧
    P.checkedCoreChallengeBindsAcceptedTuple spend ∧
      ConsumeOnceAcceptedTransition P spend

/--
Composable soundness assumptions for the direct accepted-spend verifier.

Each field is intentionally phrased as an `or break` boundary so concrete
instances can charge failure to the corresponding named event without this file
depending on a registry, hash, reorg, or checked-core implementation.
-/
structure DirectAcceptedSpendSoundness (P : Predicates T) where
  bound_slot_or_break :
    forall {spend},
      P.verifyPath2SpendAuthorizationAccepts spend ->
        BoundPrevalidatedSlot P spend ∨ HasBreak P spend
  admitted_prechallenge_or_break :
    forall {spend},
      BoundPrevalidatedSlot P spend ->
        AdmittedPrechallengeLeaf P spend ∨ HasBreak P spend
  checked_core_binds_or_break :
    forall {spend},
      P.verifyPath2SpendAuthorizationAccepts spend ->
        P.checkedCoreChallengeBindsAcceptedTuple spend ∨ HasBreak P spend
  consume_once_or_break :
    forall {spend},
      P.verifyPath2SpendAuthorizationAccepts spend ->
        ConsumeOnceAcceptedTransition P spend ∨ HasBreak P spend

/--
Direct-mode theorem target for AP2-N003/P2FS-003/P2FS-013.

If the direct Path 2 spend verifier accepts, then the spend is backed by the
same admitted prechallenge leaf and by the exact consume-once successor
transition, or the run exports a named break.
-/
theorem account_creation_prevalidated_slot_authorization_or_break
    {P : Predicates T}
    (S : DirectAcceptedSpendSoundness P)
    {spend : SpendTuple T}
    (hAccept : P.verifyPath2SpendAuthorizationAccepts spend) :
    AcceptedSpendConclusion P spend ∨ HasBreak P spend := by
  rcases S.bound_slot_or_break hAccept with hBound | hBreak
  · rcases S.admitted_prechallenge_or_break hBound with hAdmitted | hBreak
    · rcases S.checked_core_binds_or_break hAccept with hChecked | hBreak
      · rcases S.consume_once_or_break hAccept with hConsume | hBreak
        · exact Or.inl ⟨hAdmitted, hChecked, hConsume⟩
        · exact Or.inr hBreak
      · exact Or.inr hBreak
    · exact Or.inr hBreak
  · exact Or.inr hBreak

/-- A full-signature forgery run names the spend tuple it tries to realize. -/
structure FullForgeryRun (T : Types.{u}) where
  spend : SpendTuple T

/--
Full-game predicates kept abstract from concrete adversary and blockchain
schedulers.
-/
structure FullGamePredicates (T : Types.{u}) extends Predicates T where
  fullSignatureForgerySucceeds : FullForgeryRun T -> Prop

/--
Boundary from arbitrary full-signature success to the accepted-spend event, or
to a priced full-game bad event.
-/
structure FullForgeryToAcceptedSpendSoundness (P : FullGamePredicates T) where
  success_implies_accepted_spend_or_break :
    forall {run},
      P.fullSignatureForgerySucceeds run ->
        P.verifyPath2SpendAuthorizationAccepts run.spend ∨ HasBreak P.toPredicates run.spend

/--
Full-game lift: any successful direct-mode full-signature forgery gives the
accepted-spend conclusion for its spend tuple, or a named break.
-/
theorem fullForgeryImpliesAcceptedSpendForgery
    {P : FullGamePredicates T}
    (F : FullForgeryToAcceptedSpendSoundness P)
    (S : DirectAcceptedSpendSoundness P.toPredicates)
    {run : FullForgeryRun T}
    (hSuccess : P.fullSignatureForgerySucceeds run) :
    AcceptedSpendConclusion P.toPredicates run.spend ∨
      HasBreak P.toPredicates run.spend := by
  rcases F.success_implies_accepted_spend_or_break hSuccess with hAccept | hBreak
  · exact account_creation_prevalidated_slot_authorization_or_break S hAccept
  · exact Or.inr hBreak

end Path2FullSpendGame
end Descender
