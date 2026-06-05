import Descender.Path2FullSpendGame
import Descender.Path2NativeAccountDesign

namespace Descender

/-!
# Path 2 Full Spend Concrete Interface

This module instantiates the abstract full-spend game boundary with a small
native/account-shaped carrier.  It does not choose byte encodings, hash
functions, or a chain database.  Instead it names the concrete consensus
objects that those implementations must provide and packages the direct
accepted-spend theorem for that object language.
-/

namespace Path2FullSpendConcrete

/-- A prevalidated Path 2 slot leaf admitted before the checked-core challenge. -/
structure ConcretePrevalidatedSlotLeaf where
  boundary : Path2NativeAccountDesign.Boundary
  accountId : Nat
  accountStateRoot : Nat
  slotId : Nat
  publicKey : Nat
  accountNamespace : Nat
  authorizationClass : Nat
  lifecycleIdentity : Nat
  transcriptSurface : Nat
  stateTransitionBoundary : Nat
  creationTime : Nat
  frozenAt : Nat
  expiresAt : Nat
  creationEvidenceDigest : Nat
  leafDigest : Nat
  checkedCoreFrame : Nat

/-- Public account-creation evidence stored with the admitted native boundary. -/
structure ConcreteAccountCreationEvidence where
  boundary : Path2NativeAccountDesign.Boundary
  acceptedAt : Nat
  creationEvidenceDigest : Nat
  creationFrame : Nat

/-- Replayable direct-relation certificate carried by the prevalidated slot. -/
structure ConcreteDirectCertificate where
  boundary : Path2NativeAccountDesign.Boundary
  acceptedAt : Nat
  leafDigest : Nat
  relationFrame : Nat
  dataAvailable : Prop

/-- Direct admission namespace for prevalidated slot leaves. -/
structure ConcreteAdmissionRecord where
  contains :
    Nat -> Nat -> Nat -> ConcretePrevalidatedSlotLeaf -> Prop
  canonicalAt : Nat -> Prop

/-- Concrete signature/checking payload consumed by the direct verifier. -/
structure ConcreteSignatureBody where
  rawSignature : Nat
  proofCommitment : Nat
  checkedCoreFrame : Nat
  relationChallenge : Nat

/-- Consensus lifecycle state for consume-once slot authorization. -/
structure ConcreteConsumeState where
  stateNonce : Nat
  chainHeight : Nat
  forkEpoch : Nat
  rollbackBarrier : Nat
  consumed : Nat -> Prop
  canonical : Prop

/-- The Nat/native carrier used by the full-spend game boundary. -/
abbrev concreteTypes : Path2FullSpendGame.Types where
  PublicKey := Nat
  Message := Nat
  AccountId := Nat
  StateRoot := Nat
  SlotId := Nat
  SignatureBody := ConcreteSignatureBody
  PrevalidatedSlotLeaf := ConcretePrevalidatedSlotLeaf
  AccountCreationEvidence := ConcreteAccountCreationEvidence
  DirectReplayableRelationCertificate := ConcreteDirectCertificate
  AdmissionRecord := ConcreteAdmissionRecord
  ConsumeState := ConcreteConsumeState
  ChallengeTime := Nat
  Digest := Nat

abbrev ConcreteSpendTuple :=
  Path2FullSpendGame.SpendTuple concreteTypes

/--
Configuration supplied by an implementation of the native checked-core route.

The `leafFrame` field is the consensus leaf digest frame.  The checked-core
frame binding is the native active-lane projection carrier imported from
`Path2NativeAccountDesign`.
-/
structure ConcreteGameConfig where
  nativeMeaning : Path2NativeAccountDesign.MeaningProjection
  nativeProjections : Path2NativeAccountDesign.PublicProjections
  nativeObstruction : Path2NativeAccountDesign.ObstructionMap
  nativeChecks : Path2NativeAccountDesign.ObstructionComponentPredicates
  nativeVerifier : Path2NativeAccountDesign.VerifierPredicates
  checkedCoreFields : Path2NativeAccountDesign.NativeCheckedCoreFrameFields
  checkedCore :
    Path2NativeAccountDesign.NativeCheckedCoreFrameBinding checkedCoreFields
  leafFrameEncoder : Path2NativeAccountDesign.OrderedFrameEncoder
  leafFrame : ConcretePrevalidatedSlotLeaf -> Nat
  leafFrame_binds :
    forall leaf,
      leafFrame leaf =
        leafFrameEncoder.encode [
          leaf.accountId,
          leaf.accountStateRoot,
          leaf.slotId,
          leaf.publicKey,
          leaf.accountNamespace,
          leaf.authorizationClass,
          leaf.lifecycleIdentity,
          leaf.transcriptSurface,
          leaf.stateTransitionBoundary,
          leaf.creationTime,
          leaf.frozenAt,
          leaf.expiresAt,
          leaf.creationEvidenceDigest
        ]

/-- The leaf fields agree with the native public projections of its boundary. -/
def NativeProjectionCarrierBinding
    (C : ConcreteGameConfig)
    (leaf : ConcretePrevalidatedSlotLeaf) : Prop :=
  leaf.accountNamespace = C.nativeProjections.pAccount leaf.boundary ∧
    leaf.authorizationClass = C.nativeProjections.pSpend leaf.boundary ∧
      leaf.lifecycleIdentity = C.nativeProjections.pLifecycle leaf.boundary ∧
        leaf.transcriptSurface = C.nativeProjections.pTranscript leaf.boundary ∧
          leaf.stateTransitionBoundary = C.nativeProjections.pState leaf.boundary

/-- Account creation is accepted over the native zero-obstruction boundary. -/
def AccountCreationAccepted
    (C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple) : Prop :=
  spend.creationEvidence.boundary = spend.leaf.boundary ∧
    spend.creationEvidence.creationEvidenceDigest =
        spend.leaf.creationEvidenceDigest ∧
      Path2NativeAccountDesign.CheckedObstructionZero
        C.nativeObstruction C.nativeChecks spend.leaf.boundary ∧
        C.nativeVerifier.acceptCreate spend.leaf.boundary ∧
          spend.creationEvidence.acceptedAt ≤ (spend.spendChallengeTime : Nat)

/-- Local canonicality and liveness of the concrete prevalidated slot leaf. -/
def PrevalidatedSlotLeafCanonicalAndLive
    (C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple) : Prop :=
  spend.leaf.accountId = spend.accountId ∧
    spend.leaf.accountStateRoot = spend.accountStateRootBefore ∧
      spend.leaf.slotId = spend.slotId ∧
        spend.leaf.publicKey = spend.publicKey ∧
          spend.leaf.creationTime ≤ (spend.spendChallengeTime : Nat) ∧
            (spend.spendChallengeTime : Nat) ≤ spend.leaf.expiresAt ∧
              NativeProjectionCarrierBinding C spend.leaf

/--
The leaf, account-creation evidence, and replayable direct certificate name the
same boundary and digest.
-/
def LeafEvidenceCertificateBinding
    (C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple) : Prop :=
  AccountCreationAccepted C spend ∧
    spend.directCertificate.boundary = spend.leaf.boundary ∧
      spend.directCertificate.leafDigest = spend.leaf.leafDigest

/-- The slot authorization digest is exactly the ordered frame of the leaf. -/
def SlotAuthorizationDigestFramesLeaf
    (C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple) : Prop :=
  spend.slotAuthorizationDigest = C.leafFrame spend.leaf ∧
    C.leafFrame spend.leaf = spend.leaf.leafDigest

/-- Replayable direct certificate and admission record are available on chain. -/
def ReplayableCertificateDataAvailable
    (_C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple) : Prop :=
  spend.directCertificate.dataAvailable ∧
    spend.admissionRecord.canonicalAt (spend.spendChallengeTime : Nat)

/-- Direct namespace admission contains the leaf under the accepted digest. -/
def DirectAdmissionContains
    (record : ConcreteAdmissionRecord)
    (accountId accountRoot digest : Nat)
    (leaf : ConcretePrevalidatedSlotLeaf) : Prop :=
  record.contains accountId accountRoot digest leaf

/-- The replayable creation certificate was accepted before the challenge. -/
def CreationCertificateAcceptedBeforeChallenge
    (certificate : ConcreteDirectCertificate)
    (leaf : ConcretePrevalidatedSlotLeaf)
    (challengeTime : Nat) : Prop :=
  certificate.boundary = leaf.boundary ∧
    certificate.leafDigest = leaf.leafDigest ∧
      certificate.acceptedAt ≤ challengeTime

/-- The prevalidated slot leaf was frozen before the checked-core challenge. -/
def LeafFixedBeforeChallenge
    (leaf : ConcretePrevalidatedSlotLeaf)
    (challengeTime : Nat) : Prop :=
  leaf.frozenAt ≤ challengeTime

/--
Checked-core binding of message, state, native boundary, proof commitment, and
the accepted spend tuple to one frame.
-/
def CheckedCoreChallengeBindsAcceptedTuple
    (C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple) : Prop :=
  C.nativeVerifier.spendAccept spend.leaf.boundary
      spend.signatureBody.rawSignature
      spend.message
      spend.consumeStateBefore.stateNonce ∧
    spend.signatureBody.checkedCoreFrame =
      C.checkedCore.frame spend.leaf.boundary
        spend.message
        spend.consumeStateBefore.stateNonce
        spend.signatureBody.proofCommitment ∧
      spend.leaf.checkedCoreFrame = spend.signatureBody.checkedCoreFrame ∧
        spend.leaf.accountId = spend.accountId ∧
          spend.leaf.accountStateRoot = spend.accountStateRootBefore ∧
            spend.leaf.slotId = spend.slotId ∧
              spend.leaf.publicKey = spend.publicKey

/-- The exact digest is absent before the transition and present after it. -/
def ConsumeOnceTransition
    (before after : ConcreteConsumeState)
    (digest : Nat) : Prop :=
  before.canonical ∧
    ¬ before.consumed digest ∧
      after.consumed digest ∧
        after.chainHeight = before.chainHeight + 1 ∧
          after.forkEpoch = before.forkEpoch ∧
            after.rollbackBarrier = before.rollbackBarrier

/-- The successor state remains in the same canonical lifecycle lane. -/
def ValidConsumeSuccessor
    (before after : ConcreteConsumeState) : Prop :=
  before.canonical ∧
    after.canonical ∧
      after.chainHeight = before.chainHeight + 1 ∧
        after.forkEpoch = before.forkEpoch ∧
          after.rollbackBarrier = before.rollbackBarrier

/-- Concrete rollback/reorg condition exported as the slot replay break. -/
def RollbackOrReorgBreak
    (spend : ConcreteSpendTuple) : Prop :=
  ¬ spend.consumeStateAfter.canonical ∨
    spend.consumeStateAfter.forkEpoch ≠ spend.consumeStateBefore.forkEpoch ∨
      spend.consumeStateAfter.rollbackBarrier ≠
        spend.consumeStateBefore.rollbackBarrier

/-- Concrete bad-event classifier for this instantiation layer. -/
def ConcreteBreakFor
    (_C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple)
    (breakEvent : Path2FullSpendGame.BreakEvent) : Prop :=
  match breakEvent with
  | Path2FullSpendGame.BreakEvent.creationTimeHiddenSourceSoundnessBreak =>
      ¬ AccountCreationAccepted _C spend
  | Path2FullSpendGame.BreakEvent.prevalidationLeafBindingBreak =>
      ¬ PrevalidatedSlotLeafCanonicalAndLive _C spend
  | Path2FullSpendGame.BreakEvent.prevalidationRegistryBindingBreak =>
      ¬ (DirectAdmissionContains spend.admissionRecord spend.accountId
          spend.accountStateRootBefore spend.slotAuthorizationDigest spend.leaf ∧
        CreationCertificateAcceptedBeforeChallenge spend.directCertificate
          spend.leaf (spend.spendChallengeTime : Nat) ∧
          LeafFixedBeforeChallenge spend.leaf (spend.spendChallengeTime : Nat))
  | Path2FullSpendGame.BreakEvent.prevalidationDataAvailabilityBreak =>
      ¬ ReplayableCertificateDataAvailable _C spend
  | Path2FullSpendGame.BreakEvent.postchallengeSlotCreationOrRewriteBreak =>
      ¬ LeafFixedBeforeChallenge spend.leaf (spend.spendChallengeTime : Nat)
  | Path2FullSpendGame.BreakEvent.checkedCoreMessageBindingBreak =>
      ¬ CheckedCoreChallengeBindsAcceptedTuple _C spend
  | Path2FullSpendGame.BreakEvent.slotReplayOrRollbackBreak =>
      RollbackOrReorgBreak spend
  | Path2FullSpendGame.BreakEvent.qromOrDomainSeparationBreak =>
      False
  | Path2FullSpendGame.BreakEvent.liveSlotLeakageOrKeygenBreak =>
      False

/-- Bound concrete prevalidated slot package. -/
def BoundConcretePrevalidatedSlot
    (C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple) : Prop :=
  PrevalidatedSlotLeafCanonicalAndLive C spend ∧
    LeafEvidenceCertificateBinding C spend ∧
      SlotAuthorizationDigestFramesLeaf C spend ∧
        ReplayableCertificateDataAvailable C spend

/-- Concrete prechallenge admission package consumed by the abstract game. -/
def AdmittedConcretePrechallengeLeaf
    (_C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple) : Prop :=
  DirectAdmissionContains spend.admissionRecord spend.accountId
      spend.accountStateRootBefore spend.slotAuthorizationDigest spend.leaf ∧
    CreationCertificateAcceptedBeforeChallenge spend.directCertificate
      spend.leaf (spend.spendChallengeTime : Nat) ∧
      LeafFixedBeforeChallenge spend.leaf (spend.spendChallengeTime : Nat)

/-- Concrete consume-once successor package. -/
def ConcreteConsumeOnceAcceptedTransition
    (_C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple) : Prop :=
  ConsumeOnceTransition spend.consumeStateBefore spend.consumeStateAfter
      spend.slotAuthorizationDigest ∧
    ValidConsumeSuccessor spend.consumeStateBefore spend.consumeStateAfter

/--
Concrete direct verifier acceptance.  Implementations may refine this predicate
into executable byte checks, but the theorem-level carrier requires exactly
these named facts or an exported bad event.
-/
def ConcreteSpendVerificationAccepts
    (C : ConcreteGameConfig)
    (spend : ConcreteSpendTuple) : Prop :=
  BoundConcretePrevalidatedSlot C spend ∧
    AdmittedConcretePrechallengeLeaf C spend ∧
      CheckedCoreChallengeBindsAcceptedTuple C spend ∧
        ConcreteConsumeOnceAcceptedTransition C spend

/-- Full-game predicates for the concrete native carrier. -/
def concretePredicates
    (C : ConcreteGameConfig) :
    Path2FullSpendGame.Predicates concreteTypes where
  verifyPath2SpendAuthorizationAccepts := ConcreteSpendVerificationAccepts C
  leafCanonicalAndLive := PrevalidatedSlotLeafCanonicalAndLive C
  leafEvidenceCertificateBinding := LeafEvidenceCertificateBinding C
  slotAuthorizationDigestFramesLeaf := SlotAuthorizationDigestFramesLeaf C
  replayableCertificateDataAvailable := ReplayableCertificateDataAvailable C
  directAdmissionContains := DirectAdmissionContains
  creationCertificateAcceptedBeforeChallenge :=
    CreationCertificateAcceptedBeforeChallenge
  leafFixedBeforeChallenge := LeafFixedBeforeChallenge
  checkedCoreChallengeBindsAcceptedTuple :=
    CheckedCoreChallengeBindsAcceptedTuple C
  consumeOnceTransition := ConsumeOnceTransition
  validConsumeSuccessor := ValidConsumeSuccessor
  breakFor := ConcreteBreakFor C

/-- The concrete carrier supplies the abstract direct accepted-spend soundness. -/
def concreteDirectAcceptedSpendSoundness
    (C : ConcreteGameConfig) :
    Path2FullSpendGame.DirectAcceptedSpendSoundness
      (concretePredicates C) where
  bound_slot_or_break := by
    intro spend hAccept
    exact Or.inl hAccept.1
  admitted_prechallenge_or_break := by
    intro spend _hBound
    classical
    by_cases hAdmitted :
        Path2FullSpendGame.AdmittedPrechallengeLeaf
          (concretePredicates C) spend
    · exact Or.inl hAdmitted
    · exact Or.inr ⟨
        Path2FullSpendGame.BreakEvent.prevalidationRegistryBindingBreak,
        by
          dsimp [Path2FullSpendGame.HasBreak, concretePredicates,
            ConcreteBreakFor, AdmittedConcretePrechallengeLeaf,
            DirectAdmissionContains, CreationCertificateAcceptedBeforeChallenge,
            LeafFixedBeforeChallenge] at *
          exact hAdmitted
      ⟩
  checked_core_binds_or_break := by
    intro spend hAccept
    exact Or.inl hAccept.2.2.1
  consume_once_or_break := by
    intro spend hAccept
    exact Or.inl hAccept.2.2.2

/--
Concrete direct-mode theorem: an accepted native Path 2 spend has the admitted
prechallenge leaf and exact consume-once transition, or exports a named break.
-/
theorem concrete_account_creation_prevalidated_slot_authorization_or_break
    (C : ConcreteGameConfig)
    {spend : ConcreteSpendTuple}
    (hAccept : ConcreteSpendVerificationAccepts C spend) :
    Path2FullSpendGame.AcceptedSpendConclusion
        (concretePredicates C) spend ∨
      Path2FullSpendGame.HasBreak (concretePredicates C) spend :=
  Path2FullSpendGame.account_creation_prevalidated_slot_authorization_or_break
    (concreteDirectAcceptedSpendSoundness C) hAccept

/-- A concrete full-signature run succeeds when its concrete verifier accepts. -/
def ConcreteFullSignatureForgerySucceeds
    (C : ConcreteGameConfig)
    (run : Path2FullSpendGame.FullForgeryRun concreteTypes) : Prop :=
  ConcreteSpendVerificationAccepts C run.spend

/-- Concrete full-game predicate package for the native carrier. -/
def concreteFullGamePredicates
    (C : ConcreteGameConfig) :
    Path2FullSpendGame.FullGamePredicates concreteTypes where
  toPredicates := concretePredicates C
  fullSignatureForgerySucceeds := ConcreteFullSignatureForgerySucceeds C

/--
Non-tautological source for production full-signature execution.

`ConcreteFullSignatureForgerySucceeds` above remains the minimal theorem carrier,
where full-game success is identified with concrete verifier acceptance.  A
production instantiation should instead provide this source: its success
predicate can model byte parsing, account admission, chain scheduling, rollback
policy, and verifier routing independently, and must prove that any successful
run reaches the concrete accepted-spend predicate or exports a named break.
-/
structure ConcreteFullSignatureExecutionSource
    (C : ConcreteGameConfig) where
  executionSucceeds :
    Path2FullSpendGame.FullForgeryRun concreteTypes -> Prop
  success_implies_verifier_accept_or_break :
    forall {run},
      executionSucceeds run ->
      ConcreteSpendVerificationAccepts C run.spend ∨
          Path2FullSpendGame.HasBreak (concretePredicates C) run.spend

/--
Lean mirror of the public selected-Rust full-execution evidence report.

The final AP2-N003 instantiation should bind this report to the production
EUF-CMA/blockchain execution surface, then use the verified facts to construct
the execution source consumed by the concrete full-game lift.
-/
structure SelectedRustFullExecutionEvidenceReport where
  schemaMatches : Bool
  reportDigestVerified : Bool
  productionExecutionSurfaceVerified : Bool
  executionSuccessPredicatePinned : Bool
  successImpliesVerifierAcceptOrBreak : Bool
  concreteGameConfigPinned : Bool
  nonTautologicalExecutionSource : Bool
  namedBreaksExhaustive : Bool

/-- The production full-execution report has the expected schema and digest. -/
def SelectedRustFullExecutionEvidenceReport.ReportVerified
    (E : SelectedRustFullExecutionEvidenceReport) : Prop :=
  E.schemaMatches = true ∧
  E.reportDigestVerified = true

/-- All production full-execution source facts recorded by the report hold. -/
def SelectedRustFullExecutionEvidenceReport.FactsAllTrue
    (E : SelectedRustFullExecutionEvidenceReport) : Prop :=
  E.productionExecutionSurfaceVerified = true ∧
  E.executionSuccessPredicatePinned = true ∧
  E.successImpliesVerifierAcceptOrBreak = true ∧
  E.concreteGameConfigPinned = true ∧
  E.nonTautologicalExecutionSource = true ∧
  E.namedBreaksExhaustive = true

/--
Selected Rust/model evidence certificate for production full-signature
execution.

The final AP2-N003 instantiation must prove that the real production execution
surface has been verified, then expose the execution-success predicate and the
bridge from success into concrete verifier acceptance or a named break. This
record prevents the deployment-closing theorem from constructing
`ConcreteFullSignatureExecutionSource` without naming the production evidence
it depends on.
-/
structure SelectedRustFullExecutionSourceCertificate
    (C : ConcreteGameConfig) where
  evidenceReport : SelectedRustFullExecutionEvidenceReport
  evidenceDigest : ByteArray
  selectedRustFullExecutionSourceVerifiedProof :
    evidenceReport.ReportVerified
  selectedRustFullExecutionFactsAllTrueProof :
    evidenceReport.FactsAllTrue
  selectedRustExecutionSucceeds :
    Path2FullSpendGame.FullForgeryRun concreteTypes -> Prop
  selectedRustSuccessImpliesVerifierAcceptOrBreak :
    forall {run},
      selectedRustExecutionSucceeds run ->
      ConcreteSpendVerificationAccepts C run.spend ∨
          Path2FullSpendGame.HasBreak (concretePredicates C) run.spend

/-- Named constructor for the selected Rust full-execution evidence certificate. -/
def SelectedRustFullExecutionSourceCertificate.ofReportAndExecutionSource
    {C : ConcreteGameConfig}
    (evidenceReport : SelectedRustFullExecutionEvidenceReport)
    (evidenceDigest : ByteArray)
    (selectedRustFullExecutionSourceVerifiedProof :
      evidenceReport.ReportVerified)
    (selectedRustFullExecutionFactsAllTrueProof :
      evidenceReport.FactsAllTrue)
    (selectedRustExecutionSucceeds :
      Path2FullSpendGame.FullForgeryRun concreteTypes -> Prop)
    (selectedRustSuccessImpliesVerifierAcceptOrBreak :
      forall {run},
        selectedRustExecutionSucceeds run ->
          ConcreteSpendVerificationAccepts C run.spend ∨
            Path2FullSpendGame.HasBreak (concretePredicates C) run.spend) :
    SelectedRustFullExecutionSourceCertificate C where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  selectedRustFullExecutionSourceVerifiedProof :=
    selectedRustFullExecutionSourceVerifiedProof
  selectedRustFullExecutionFactsAllTrueProof :=
    selectedRustFullExecutionFactsAllTrueProof
  selectedRustExecutionSucceeds := selectedRustExecutionSucceeds
  selectedRustSuccessImpliesVerifierAcceptOrBreak :=
    selectedRustSuccessImpliesVerifierAcceptOrBreak

/-- Convert selected Rust production execution evidence into the theorem source. -/
def SelectedRustFullExecutionSourceCertificate.toExecutionSource
    {C : ConcreteGameConfig}
    (Cert : SelectedRustFullExecutionSourceCertificate C) :
    ConcreteFullSignatureExecutionSource C where
  executionSucceeds := by
    let _hVerified : Cert.evidenceReport.ReportVerified :=
      Cert.selectedRustFullExecutionSourceVerifiedProof
    let _hFacts : Cert.evidenceReport.FactsAllTrue :=
      Cert.selectedRustFullExecutionFactsAllTrueProof
    exact Cert.selectedRustExecutionSucceeds
  success_implies_verifier_accept_or_break :=
    Cert.selectedRustSuccessImpliesVerifierAcceptOrBreak

/-- Full-game predicates using a production execution source instead of the tautological carrier. -/
def concreteFullGamePredicatesFromExecution
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C) :
    Path2FullSpendGame.FullGamePredicates concreteTypes where
  toPredicates := concretePredicates C
  fullSignatureForgerySucceeds := Source.executionSucceeds

/-- A production execution source supplies the full-forgery-to-accepted-spend boundary. -/
def concreteFullForgeryToAcceptedSpendSoundnessFromExecution
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C) :
    Path2FullSpendGame.FullForgeryToAcceptedSpendSoundness
      (concreteFullGamePredicatesFromExecution C Source) where
  success_implies_accepted_spend_or_break := by
    intro run hSuccess
    exact Source.success_implies_verifier_accept_or_break hSuccess

/--
Production full-game lift using an explicit execution source.

This is the theorem shape needed to make the concrete full-signature theorem
cryptographically closed: production success is no longer definitionally equal
to accepted spend; the source has to prove that bridge or name a break.
-/
theorem concrete_fullExecutionImpliesAcceptedSpendForgery
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C)
    {run : Path2FullSpendGame.FullForgeryRun concreteTypes}
    (hSuccess : Source.executionSucceeds run) :
    Path2FullSpendGame.AcceptedSpendConclusion
        (concretePredicates C) run.spend ∨
      Path2FullSpendGame.HasBreak (concretePredicates C) run.spend :=
  Path2FullSpendGame.fullForgeryImpliesAcceptedSpendForgery
    (concreteFullForgeryToAcceptedSpendSoundnessFromExecution C Source)
    (concreteDirectAcceptedSpendSoundness C)
    hSuccess

/-- Proposition installed into the active full-signature boundary by a concrete execution source. -/
def ConcreteFullExecutionAcceptedSpendLift
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C) : Prop :=
  forall {run : Path2FullSpendGame.FullForgeryRun concreteTypes},
    Source.executionSucceeds run ->
      Path2FullSpendGame.AcceptedSpendConclusion
          (concretePredicates C) run.spend ∨
        Path2FullSpendGame.HasBreak (concretePredicates C) run.spend

/-- A concrete execution source supplies the full-game lift proposition. -/
theorem concrete_fullExecutionAcceptedSpendLift
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C) :
    ConcreteFullExecutionAcceptedSpendLift C Source := by
  intro run hSuccess
  exact concrete_fullExecutionImpliesAcceptedSpendForgery C Source hSuccess

/--
Install a concrete full-signature execution source into the active native
full-signature security boundary, preserving every other proof obligation.
-/
def Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary.withConcreteFullExecutionSource
    (B : Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C) :
    Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary :=
  { B with
    fullForgeryImpliesAcceptedSpendForgery :=
      ConcreteFullExecutionAcceptedSpendLift C Source
    fullSignatureSecurityClaimComplete := by
      rcases B.fullSignatureSecurityClaimComplete with
        ⟨hKeygen, hSigning, hCompleteness, hVerifier, _hFullGame,
          hHiddenSource, hCheckedCore, hLifecycle, hNoExport,
          hCheckedCoreDomain, hHiddenSourceDomain, hQueryBudget,
          hMultiTarget⟩
      exact
        ⟨hKeygen, hSigning, hCompleteness, hVerifier,
          concrete_fullExecutionAcceptedSpendLift C Source,
          hHiddenSource, hCheckedCore, hLifecycle, hNoExport,
          hCheckedCoreDomain, hHiddenSourceDomain, hQueryBudget,
          hMultiTarget⟩ }

/-- The full-signature wrapper exposes exactly the concrete execution lift. -/
theorem full_signature_with_concrete_full_execution_source_lift
    (B : Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary)
    (C : ConcreteGameConfig)
    (Source : ConcreteFullSignatureExecutionSource C) :
    (Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary.withConcreteFullExecutionSource
      B C Source).fullForgeryImpliesAcceptedSpendForgery :=
  concrete_fullExecutionAcceptedSpendLift C Source

/-- Full-game success supplies an accepted spend for the concrete carrier. -/
def concreteFullForgeryToAcceptedSpendSoundness
    (C : ConcreteGameConfig) :
    Path2FullSpendGame.FullForgeryToAcceptedSpendSoundness
      (concreteFullGamePredicates C) where
  success_implies_accepted_spend_or_break := by
    intro run hSuccess
    exact Or.inl hSuccess

/-- Concrete full-game lift to the accepted-spend conclusion. -/
theorem concrete_fullForgeryImpliesAcceptedSpendForgery
    (C : ConcreteGameConfig)
    {run : Path2FullSpendGame.FullForgeryRun concreteTypes}
    (hSuccess : ConcreteFullSignatureForgerySucceeds C run) :
    Path2FullSpendGame.AcceptedSpendConclusion
        (concretePredicates C) run.spend ∨
      Path2FullSpendGame.HasBreak (concretePredicates C) run.spend :=
  Path2FullSpendGame.fullForgeryImpliesAcceptedSpendForgery
    (concreteFullForgeryToAcceptedSpendSoundness C)
    (concreteDirectAcceptedSpendSoundness C)
    hSuccess

end Path2FullSpendConcrete
end Descender
