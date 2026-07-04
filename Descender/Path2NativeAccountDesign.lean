import Mathlib.Data.Real.Basic
import Descender.Path2AcceptedSpendProbability

namespace Descender

/-!
# Path 2 Native Account Design Interface

This module records the theorem-level interface for the active native Path 2
account design.  It is intentionally structural: concrete encodings, q12289
obstruction algebra, checked-core implementation details, and cryptographic
hardness assumptions remain separate obligations.

The active design reads account creation as public obstruction vanishing over a
native boundary `q`.  Private signer state is a fiber over `q`, not a
pre-existing hidden witness that public account creation must reconstruct.
-/

namespace Path2NativeAccountDesign

/-- Semantic account meaning fixed by one admitted public boundary. -/
structure Meaning where
  accountNamespace : Nat
  spendAuthorizationClass : Nat
  lifecycleIdentity : Nat
  transcriptBindingSurface : Nat
  stateTransitionBoundary : Nat
  parameterContext : Nat
  deriving DecidableEq

/-- Pullback-organized public boundary faces for the active native account. -/
structure Boundary where
  contextFace : Nat
  accountFace : Nat
  spendFace : Nat
  lifecycleFace : Nat
  transcriptFace : Nat
  stateFace : Nat
  overlapFace : Nat
  canonicalityFace : Nat

/-- Public obstruction/check object for native account admission. -/
structure Obstruction where
  contextDefect : Nat
  pullbackDefect : Nat
  gluingDefect : Nat
  meaningDefect : Nat
  spendStateDefect : Nat
  canonicalityDefect : Nat
  deriving DecidableEq

/-- Private signer state over an admitted boundary. -/
structure PrivateFiber where
  liftCore : Nat
  authorizationLocal : Nat
  gauge : Nat
  randomizer : Nat

/-- Public action semantics of one accepted spend. -/
structure PublicSpendAction where
  meaning : Meaning
  message : Nat
  stateTransition : Nat
  lifecycle : Nat
  authorization : Nat

/-- Named structural breaks for the native account design. -/
inductive BreakEvent where
  | contextBindingBreak
  | pullbackConsistencyBreak
  | gluingObstructionBreak
  | meaningBindingBreak
  | spendStateClosureBreak
  | canonicalityBreak
  | liftExistenceBreak
  | authorizationClassBindingBreak
  | authorizationLocalSamplingBreak
  | authorizationLocalHidingBreak
  | gaugeInvarianceBreak
  | fiberPrivacyBreak
  | spendClosureBreak
  | checkedCoreFrameBindingBreak
  | checkedCoreEquationSoundnessBreak
  | domainBindingOrQromBreak
  | byteParserCanonicalityBreak
  | byteDataAvailabilityBreak
  deriving DecidableEq

/-- The zero obstruction value. -/
def zeroObstruction : Obstruction where
  contextDefect := 0
  pullbackDefect := 0
  gluingDefect := 0
  meaningDefect := 0
  spendStateDefect := 0
  canonicalityDefect := 0

/-- Public semantic meaning computed from a boundary. -/
structure MeaningProjection where
  meaningOfBoundary : Boundary -> Meaning

/-- Public obstruction map for native account admission. -/
structure ObstructionMap where
  obs : Boundary -> Obstruction

/-- Predicate package for the private fiber over a public boundary. -/
structure PrivateFiberPredicates where
  liftValid : Boundary -> PrivateFiber -> Prop
  authorizationCompatible : Boundary -> PrivateFiber -> Prop
  gaugeValid : Boundary -> PrivateFiber -> Prop
  randomizerValid : Boundary -> PrivateFiber -> Prop
  authClass : PrivateFiber -> Nat
  meaningOfFiber : Boundary -> PrivateFiber -> Meaning

/-- Public projections consumed by spend verification. -/
structure PublicProjections where
  pContext : Boundary -> Nat
  pAccount : Boundary -> Nat
  pSpend : Boundary -> Nat
  pLifecycle : Boundary -> Nat
  pTranscript : Boundary -> Nat
  pState : Boundary -> Nat

/-- Verification predicates over the native boundary. -/
structure VerifierPredicates where
  acceptCreate : Boundary -> Prop
  spendAccept : Boundary -> Nat -> Nat -> Nat -> Prop
  spendProduced : Boundary -> PrivateFiber -> Nat -> Nat -> Nat -> Prop
  breakFor : Boundary -> BreakEvent -> Prop

/-- Public view and simulation predicates for no-export obligations. -/
structure PublicViewPredicates where
  publicViewProduced : Boundary -> PrivateFiber -> Nat -> Nat -> Nat -> Nat -> Prop
  simulatableFromPublic : Boundary -> Nat -> Nat -> Nat -> Nat -> Prop

/-- Public output policy for allowlist/denylist no-export checks. -/
structure PublicOutputPolicy where
  outputConforms : Nat -> Prop
  containsDenylistedField : Nat -> Prop

/-- Honest wallet/keygen account creation relation. -/
structure HonestCreationPredicates where
  honestCreate : Nat -> Boundary -> PrivateFiber -> Prop

/-- Public semantic boundary predicate exposed by zero obstruction. -/
structure PublicBoundaryPredicates where
  publicBoundarySound : Boundary -> Prop

/--
Ordered symbolic frame encoder.

This models the Rust byte encoders as ordered, injective frame constructors
rather than commutative arithmetic over fields.
-/
structure OrderedFrameEncoder where
  encode : List Nat -> Nat
  injective :
    forall {lhs rhs : List Nat}, encode lhs = encode rhs -> lhs = rhs

/-- Equal ordered frame encodings recover the same ordered field list. -/
theorem ordered_frame_encode_injective
    (E : OrderedFrameEncoder)
    {lhs rhs : List Nat}
    (h : E.encode lhs = E.encode rhs) :
    lhs = rhs :=
  E.injective h

/-- Field-level context projection data used by `O_context` well-formedness. -/
structure ContextFields where
  nativeBoundaryVersion : Boundary -> Nat
  parameterSetId : Boundary -> Nat
  verifierFamilyId : Boundary -> Nat
  accountCreationDomain : Boundary -> Nat
  spendVerifierDomain : Boundary -> Nat
  chainApplicationDomain : Boundary -> Nat
  forkEpochScope : Boundary -> Nat
  aggregationMode : Boundary -> Nat

/-- Public validity predicates for the native context projection. -/
structure ContextPredicates where
  versionValid : Boundary -> Prop
  parameterSetValid : Boundary -> Prop
  verifierFamilyValid : Boundary -> Prop
  domainsCanonical : Boundary -> Prop
  aggregationModeValid : Boundary -> Prop

/-- A public boundary has zero native obstruction. -/
def ObstructionZero (O : ObstructionMap) (q : Boundary) : Prop :=
  O.obs q = zeroObstruction

/-- Executable or named-assumption checks for every obstruction component. -/
structure ObstructionComponentPredicates where
  contextChecked : Boundary -> Prop
  pullbackChecked : Boundary -> Prop
  gluingChecked : Boundary -> Prop
  meaningChecked : Boundary -> Prop
  spendStateChecked : Boundary -> Prop
  canonicalityChecked : Boundary -> Prop

/--
Zero obstruction is accepted only when every obstruction component has an
explicit check or named assumption behind it.
-/
def CheckedObstructionZero
    (O : ObstructionMap)
    (C : ObstructionComponentPredicates)
    (q : Boundary) : Prop :=
  ObstructionZero O q ∧
    C.contextChecked q ∧
      C.pullbackChecked q ∧
        C.gluingChecked q ∧
          C.meaningChecked q ∧
            C.spendStateChecked q ∧
              C.canonicalityChecked q

/-- Local validity of a private fiber point over a public boundary. -/
def LocalValid
    (O : ObstructionMap)
    (P : PrivateFiberPredicates)
    (q : Boundary) (h : PrivateFiber) : Prop :=
  ObstructionZero O q ∧
    P.liftValid q h ∧
      P.authorizationCompatible q h ∧
        P.gaugeValid q h ∧
          P.randomizerValid q h

/-- The private fiber point has the public meaning of its boundary. -/
def FiberMeaningConsistent
    (M : MeaningProjection)
    (P : PrivateFiberPredicates)
    (q : Boundary) (h : PrivateFiber) : Prop :=
  P.meaningOfFiber q h = M.meaningOfBoundary q

/-- First-pass safety equivalence: equality of semantic account meaning. -/
def SafeEquivalent
    (M : MeaningProjection)
    (q0 q1 : Boundary) : Prop :=
  M.meaningOfBoundary q0 = M.meaningOfBoundary q1

/-- Authorization-local material realizes the public spend class projected from `q`. -/
def AuthorizationClassMatches
    (P : PrivateFiberPredicates)
    (Proj : PublicProjections)
    (q : Boundary) (h : PrivateFiber) : Prop :=
  P.authClass h = Proj.pSpend q

/-- Authorization-local material is compatible with the specific public class of `q`. -/
def AuthorizationCompatibleOverClass
    (P : PrivateFiberPredicates)
    (Proj : PublicProjections)
    (q : Boundary) (h : PrivateFiber) : Prop :=
  P.authorizationCompatible q h ∧ AuthorizationClassMatches P Proj q h

/-- Spend acceptance semantics are determined by public projections of `q`. -/
def SpendActionFromBoundary
    (M : MeaningProjection)
    (Proj : PublicProjections)
    (q : Boundary) (msg _state : Nat)
    (action : PublicSpendAction) : Prop :=
  action.meaning = M.meaningOfBoundary q ∧
    action.message = msg ∧
      action.stateTransition = Proj.pState q ∧
        action.lifecycle = Proj.pLifecycle q ∧
          action.authorization = Proj.pSpend q

/-- The concrete spend action fields agree with the semantic meaning object. -/
def SpendActionMatchesMeaning
    (action : PublicSpendAction) : Prop :=
  action.authorization = action.meaning.spendAuthorizationClass ∧
    action.lifecycle = action.meaning.lifecycleIdentity ∧
      action.stateTransition = action.meaning.stateTransitionBoundary

/-- Projection completeness for consensus-facing account meaning. -/
structure ProjectionCompleteness
    (M : MeaningProjection)
    (Proj : PublicProjections) where
  account_eq :
    forall q, (M.meaningOfBoundary q).accountNamespace = Proj.pAccount q
  spend_eq :
    forall q, (M.meaningOfBoundary q).spendAuthorizationClass = Proj.pSpend q
  lifecycle_eq :
    forall q, (M.meaningOfBoundary q).lifecycleIdentity = Proj.pLifecycle q
  transcript_eq :
    forall q, (M.meaningOfBoundary q).transcriptBindingSurface = Proj.pTranscript q
  state_eq :
    forall q, (M.meaningOfBoundary q).stateTransitionBoundary = Proj.pState q
  context_eq :
    forall q, (M.meaningOfBoundary q).parameterContext = Proj.pContext q

/-- Effective descent: zero obstruction gives a valid private fiber point. -/
structure EffectiveDescent
    (O : ObstructionMap)
    (P : PrivateFiberPredicates) where
  lift_exists :
    forall {q}, ObstructionZero O q ->
      exists h, LocalValid O P q h

/-- Public effective descent: zero obstruction gives a coherent public boundary. -/
structure PublicEffectiveDescent
    (O : ObstructionMap)
    (B : PublicBoundaryPredicates) where
  public_sound :
    forall {q}, ObstructionZero O q ->
      B.publicBoundarySound q

/-- `O_context=0`-style well-formedness for context fields. -/
structure ContextWellFormedness
    (Ctxt : ContextPredicates) where
  context_wellformed :
    forall q,
      Ctxt.versionValid q ∧
        Ctxt.parameterSetValid q ∧
          Ctxt.verifierFamilyValid q ∧
            Ctxt.domainsCanonical q ∧
              Ctxt.aggregationModeValid q

/-- Authorization class binding for all compatible private authorization locals. -/
structure AuthorizationClassBinding
    (P : PrivateFiberPredicates)
    (Proj : PublicProjections) where
  auth_class_matches :
    forall {q h}, P.authorizationCompatible q h ->
      AuthorizationClassMatches P Proj q h

/-- Gauge/privacy surface for private fiber coordinates. -/
structure FiberPrivacy
    (M : MeaningProjection)
    (P : PrivateFiberPredicates) where
  gauge_preserves_meaning :
    forall {q h}, P.gaugeValid q h ->
      FiberMeaningConsistent M P q h
  randomizer_preserves_meaning :
    forall {q h}, P.randomizerValid q h ->
      FiberMeaningConsistent M P q h

/-- No-export soundness for public views derived from a private fiber point. -/
structure FiberNoExportSoundness
    (O : ObstructionMap)
    (P : PrivateFiberPredicates)
    (View : PublicViewPredicates)
    (V : VerifierPredicates) where
  public_view_simulatable_or_break :
    forall {q h sig msg state view},
      LocalValid O P q h ->
        View.publicViewProduced q h sig msg state view ->
          View.simulatableFromPublic q sig msg state view ∨
            exists breakEvent, V.breakFor q breakEvent

/-- A conforming public output contains no denylisted private field. -/
structure PublicOutputNoExport
    (Policy : PublicOutputPolicy) where
  no_denylisted_field :
    forall {output}, Policy.outputConforms output ->
      ¬ Policy.containsDenylistedField output

/-- Spend closure over the native public boundary. -/
structure SpendClosure
    (M : MeaningProjection)
    (Proj : PublicProjections)
    (V : VerifierPredicates) where
  spend_action :
    forall {q sig msg state},
      V.spendAccept q sig msg state ->
        exists action, SpendActionFromBoundary M Proj q msg state action

/-- Public evidence extracted from a spend proof reduction. -/
structure CheckedCoreSpendEvidence where
  proofCommitment : Nat
  checkedCoreFrame : Nat

/-- The canonical public action determined by a boundary and message. -/
def boundarySpendAction
    (M : MeaningProjection)
    (Proj : PublicProjections)
    (q : Boundary) (msg : Nat) : PublicSpendAction where
  meaning := M.meaningOfBoundary q
  message := msg
  stateTransition := Proj.pState q
  lifecycle := Proj.pLifecycle q
  authorization := Proj.pSpend q

/--
Checked-core spend reduction obligations.

This is the non-opaque route to `SpendClosure`: every accepted spend must
produce checked-core evidence and discharge the projection binding obligations
that make the resulting action a spend over the admitted native boundary.
-/
structure CheckedCoreSpendReduction
    (M : MeaningProjection)
    (Proj : PublicProjections)
    (V : VerifierPredicates) where
  frameEncoder : OrderedFrameEncoder
  checkedCoreAccepts :
    Boundary -> Nat -> Nat -> CheckedCoreSpendEvidence -> Prop
  evidence_of_spend :
    forall {q sig msg state},
      V.spendAccept q sig msg state ->
        exists evidence, checkedCoreAccepts q msg state evidence
  frame_bound_to_projections :
    forall {q msg state evidence},
      checkedCoreAccepts q msg state evidence ->
        evidence.checkedCoreFrame =
          frameEncoder.encode [
            Proj.pSpend q,
            Proj.pLifecycle q,
            Proj.pTranscript q,
            Proj.pState q,
            msg,
            state,
            evidence.proofCommitment
          ]
  authorization_class_bound :
    forall {q msg state evidence},
      checkedCoreAccepts q msg state evidence ->
        (boundarySpendAction M Proj q msg).authorization = Proj.pSpend q
  lifecycle_bound :
    forall {q msg state evidence},
      checkedCoreAccepts q msg state evidence ->
        (boundarySpendAction M Proj q msg).lifecycle = Proj.pLifecycle q
  state_transition_bound :
    forall {q msg state evidence},
      checkedCoreAccepts q msg state evidence ->
        (boundarySpendAction M Proj q msg).stateTransition = Proj.pState q
  replay_scope_bound :
    forall {q msg state evidence},
      checkedCoreAccepts q msg state evidence ->
        (boundarySpendAction M Proj q msg).lifecycle = Proj.pLifecycle q

/-- Checked-core or downstream spend frame is bound to projections of `q`. -/
structure CheckedCoreFrameBinding
    (Proj : PublicProjections) where
  frameEncoder : OrderedFrameEncoder
  frame : Boundary -> Nat -> Nat -> Nat -> Nat
  frame_binds :
    forall q msg state commitment,
      frame q msg state commitment =
        frameEncoder.encode [
          Proj.pSpend q,
          Proj.pLifecycle q,
          Proj.pTranscript q,
          Proj.pState q,
          msg,
          state,
          commitment
        ]

/-- Explicit field-level native checked-core challenge frame. -/
structure NativeCheckedCoreFrameFields where
  nativeBoundaryVersion : Boundary -> Nat
  parameterSetId : Boundary -> Nat
  verifierFamilyId : Boundary -> Nat
  accountCreationDomain : Boundary -> Nat
  spendVerifierDomain : Boundary -> Nat
  chainApplicationDomain : Boundary -> Nat
  forkEpochScope : Boundary -> Nat
  aggregationMode : Boundary -> Nat
  nativeBoundaryDigest : Boundary -> Nat
  accountDigest : Boundary -> Nat
  spendDigest : Boundary -> Nat
  lifecycleDigest : Boundary -> Nat
  transcriptDigest : Boundary -> Nat
  stateDigest : Boundary -> Nat
  messageDigest : Nat -> Nat
  currentStateDigest : Nat -> Nat
  proofCommitment : Nat -> Nat

/-- Native checked-core frame binding with explicit fields from the byte spec. -/
structure NativeCheckedCoreFrameBinding
    (F : NativeCheckedCoreFrameFields) where
  frameEncoder : OrderedFrameEncoder
  frame : Boundary -> Nat -> Nat -> Nat -> Nat
  frame_binds :
    forall q msg state commitment,
      frame q msg state commitment =
        frameEncoder.encode [
          F.nativeBoundaryVersion q,
          F.parameterSetId q,
          F.verifierFamilyId q,
          F.accountCreationDomain q,
          F.spendVerifierDomain q,
          F.chainApplicationDomain q,
          F.forkEpochScope q,
          F.aggregationMode q,
          F.nativeBoundaryDigest q,
          F.accountDigest q,
          F.spendDigest q,
          F.lifecycleDigest q,
          F.transcriptDigest q,
          F.stateDigest q,
          F.messageDigest msg,
          F.currentStateDigest state,
          F.proofCommitment commitment
        ]

/--
Field-level digest assembly for the active native boundary.

The boundary digest is intentionally acyclic: it is a function of the child
semantic projection digests, excluding the transcript digest. The transcript
projection then binds the resulting boundary digest.
-/
structure NativeProjectionDigestFields where
  contextDigest : Boundary -> Nat
  accountDigest : Boundary -> Nat
  spendDigest : Boundary -> Nat
  lifecycleDigest : Boundary -> Nat
  stateDigest : Boundary -> Nat
  nativeBoundaryDigest : Boundary -> Nat
  transcriptDigest : Boundary -> Nat

/-- Acyclic native-boundary digest assembly over non-transcript projections. -/
structure NativeBoundaryDigestAssembly
    (D : NativeProjectionDigestFields) where
  boundaryEncoder : OrderedFrameEncoder
  boundaryDigest : Boundary -> Nat
  boundary_binds :
    forall q,
      boundaryDigest q =
        boundaryEncoder.encode [
          D.contextDigest q,
          D.accountDigest q,
          D.spendDigest q,
          D.lifecycleDigest q,
          D.stateDigest q
        ]
  boundary_matches_field :
    forall q, D.nativeBoundaryDigest q = boundaryDigest q

/-- Transcript projection assembly after the boundary digest has been fixed. -/
structure NativeTranscriptDigestAssembly
    (D : NativeProjectionDigestFields) where
  transcriptEncoder : OrderedFrameEncoder
  transcriptDigest : Boundary -> Nat
  transcript_binds :
    forall q,
      transcriptDigest q =
        transcriptEncoder.encode [
          D.nativeBoundaryDigest q,
          D.contextDigest q,
          D.accountDigest q,
          D.spendDigest q,
          D.lifecycleDigest q,
          D.stateDigest q
        ]
  transcript_matches_field :
    forall q, D.transcriptDigest q = transcriptDigest q

/-- Combined acyclic projection assembly order used by the byte reference. -/
structure NativeAcyclicAssembly
    (D : NativeProjectionDigestFields) where
  boundary : NativeBoundaryDigestAssembly D
  transcript : NativeTranscriptDigestAssembly D

/--
The checked-core frame digest fields are exactly the assembled native
projection digest fields.
-/
structure NativeCheckedCoreFrameDigestBinding
    (D : NativeProjectionDigestFields)
    (F : NativeCheckedCoreFrameFields) where
  boundary_digest_bound :
    forall q, F.nativeBoundaryDigest q = D.nativeBoundaryDigest q
  account_digest_bound :
    forall q, F.accountDigest q = D.accountDigest q
  spend_digest_bound :
    forall q, F.spendDigest q = D.spendDigest q
  lifecycle_digest_bound :
    forall q, F.lifecycleDigest q = D.lifecycleDigest q
  transcript_digest_bound :
    forall q, F.transcriptDigest q = D.transcriptDigest q
  state_digest_bound :
    forall q, F.stateDigest q = D.stateDigest q

/-- Public verifier handles consumed by `path2_native_checked_core_accept_v1`. -/
structure NativeCheckedCoreVerifierHandleFields where
  checkedCoreVerifierHandleDigest : Boundary -> Nat
  proofByteLength : Nat -> Nat
  requiredProofByteLength : Nat
  proofBytesCanonical : Nat -> Prop
  challengeFromFrame : Nat -> Nat
  expectedResponseSyndrome : Nat -> Nat -> Nat -> Nat

/--
Concrete accepted checked-core evidence decoded from
`ProductionReviewedFq12AccountCarriedProof` plus its assembled native frame.
-/
structure NativeCheckedCoreSpendEvidence where
  proofCommitment : Nat
  checkedCoreFrame : Nat
  checkedCoreProofBytes : Nat
  checkedCoreVerifierHandleDigest : Nat
  decodedProofCommitment : Nat
  checkedCoreChallenge : Nat
  actualResponseSyndrome : Nat
  expectedResponseSyndrome : Nat

/-- Forget the Rust byte-level handles to the abstract checked-core evidence. -/
def NativeCheckedCoreSpendEvidence.toCheckedCoreSpendEvidence
    (evidence : NativeCheckedCoreSpendEvidence) : CheckedCoreSpendEvidence where
  proofCommitment := evidence.proofCommitment
  checkedCoreFrame := evidence.checkedCoreFrame

/--
Lean statement of the Rust `path2_native_checked_core_accept_v1` relation.

The relation is deliberately byte-facing: it requires the checked-core frame
assembled from the same native boundary/message/state/proof commitment, the
public verifier-handle digest selected by the boundary, canonical proof bytes
of the active length, proof-commitment agreement after decoding, challenge
derivation from the assembled frame, and syndrome equality.
-/
def Path2NativeCheckedCoreAcceptV1
    {F : NativeCheckedCoreFrameFields}
    (B : NativeCheckedCoreFrameBinding F)
    (H : NativeCheckedCoreVerifierHandleFields)
    (q : Boundary) (msg state : Nat)
    (evidence : NativeCheckedCoreSpendEvidence) : Prop :=
  evidence.checkedCoreFrame =
      B.frame q msg state evidence.proofCommitment ∧
    evidence.checkedCoreVerifierHandleDigest =
      H.checkedCoreVerifierHandleDigest q ∧
      H.proofByteLength evidence.checkedCoreProofBytes =
        H.requiredProofByteLength ∧
        H.proofBytesCanonical evidence.checkedCoreProofBytes ∧
          evidence.decodedProofCommitment = evidence.proofCommitment ∧
            evidence.checkedCoreChallenge =
              H.challengeFromFrame evidence.checkedCoreFrame ∧
              evidence.expectedResponseSyndrome =
                H.expectedResponseSyndrome
                  evidence.checkedCoreChallenge
                  evidence.checkedCoreVerifierHandleDigest
                  evidence.proofCommitment ∧
                evidence.actualResponseSyndrome =
                  evidence.expectedResponseSyndrome

/--
The Rust checked-core acceptor is the concrete implementation relation for the
abstract `CheckedCoreSpendReduction.checkedCoreAccepts` predicate.
-/
structure Path2NativeCheckedCoreAcceptV1Bridge
    (M : MeaningProjection)
    (Proj : PublicProjections)
    (V : VerifierPredicates)
    (F : NativeCheckedCoreFrameFields) where
  nativeFrameBinding : NativeCheckedCoreFrameBinding F
  verifierHandleFields : NativeCheckedCoreVerifierHandleFields
  reduction : CheckedCoreSpendReduction M Proj V
  checked_accepts_iff_native :
    forall {q msg state evidence},
      reduction.checkedCoreAccepts q msg state evidence ↔
        exists nativeEvidence,
          nativeEvidence.toCheckedCoreSpendEvidence = evidence ∧
            Path2NativeCheckedCoreAcceptV1
              nativeFrameBinding verifierHandleFields q msg state
                nativeEvidence

/--
Named lower-level obligations left after the byte-level checked-core acceptor
has accepted.
-/
structure Path2NativeCheckedCoreAcceptV1Soundness
    (V : VerifierPredicates)
    (F : NativeCheckedCoreFrameFields)
    (B : NativeCheckedCoreFrameBinding F)
    (H : NativeCheckedCoreVerifierHandleFields) where
  validCheckedCoreEvidence :
    Boundary -> Nat -> Nat -> NativeCheckedCoreSpendEvidence -> Prop
  accept_valid_or_named_break :
    forall {q msg state evidence},
      Path2NativeCheckedCoreAcceptV1 B H q msg state evidence ->
        validCheckedCoreEvidence q msg state evidence ∨
          V.breakFor q BreakEvent.checkedCoreEquationSoundnessBreak ∨
            V.breakFor q BreakEvent.checkedCoreFrameBindingBreak ∨
              V.breakFor q BreakEvent.domainBindingOrQromBreak ∨
                V.breakFor q BreakEvent.byteParserCanonicalityBreak

/-- Field-level public view surface emitted by the active native serializers. -/
structure NativePublicViewFields where
  accountView : Boundary -> Nat
  spendView : Boundary -> Nat -> Nat -> Nat -> Nat
  proofBytesDigest : Nat -> Nat
  proofByteLength : Nat -> Nat

/--
The active native account/spend public views are derived from assembled public
projection digests plus public message/state/proof handles.
-/
structure NativePublicViewAssembly
    (D : NativeProjectionDigestFields)
    (F : NativeCheckedCoreFrameFields)
    (V : NativePublicViewFields) where
  accountViewEncoder : OrderedFrameEncoder
  spendViewEncoder : OrderedFrameEncoder
  account_view_binds :
    forall q,
      V.accountView q =
        accountViewEncoder.encode [
          D.nativeBoundaryDigest q,
          D.contextDigest q,
          D.accountDigest q,
          D.spendDigest q,
          D.lifecycleDigest q,
          D.transcriptDigest q,
          D.stateDigest q
        ]
  spend_view_binds :
    forall q msg state proofBytes,
      V.spendView q msg state proofBytes =
        spendViewEncoder.encode [
          D.nativeBoundaryDigest q,
          F.messageDigest msg,
          F.currentStateDigest state,
          F.proofCommitment proofBytes,
          V.proofBytesDigest proofBytes,
          V.proofByteLength proofBytes
        ]

/-- Field-level projections used by the concrete first-pass `O_gluing` checks. -/
structure ConcreteGluingFields where
  spendStateClass : Boundary -> Nat
  stateTransitionClass : Boundary -> Nat
  spendAggregationMode : Boundary -> Nat
  transcriptAggregationMode : Boundary -> Nat
  contextAggregationMode : Boundary -> Nat
  spendVerifierFamily : Boundary -> Nat
  contextVerifierFamily : Boundary -> Nat
  spendDomainSeparator : Boundary -> Nat
  contextSpendDomain : Boundary -> Nat
  transcriptSpendBinding : Boundary -> Nat
  spendClassBinding : Boundary -> Nat
  transcriptContextBinding : Boundary -> Nat
  contextClassBinding : Boundary -> Nat

/-- Field-level state projection data used by `p_state(q)` canonicality. -/
structure StateProjectionFields where
  stateTransitionClass : Boundary -> Nat
  accountStateRootBefore : Boundary -> Nat
  admittedBoundaryId : Boundary -> Nat
  liveStateRequirement : Boundary -> Nat
  consumedSuccessorStateClass : Boundary -> Nat
  dataAvailabilityRequirement : Boundary -> Nat
  aggregationStateBoundary : Boundary -> Nat
  stateContextVersion : Boundary -> Nat
  stateContextParameterSet : Boundary -> Nat
  stateContextVerifierFamily : Boundary -> Nat
  stateContextSpendDomain : Boundary -> Nat
  stateContextChainApplicationDomain : Boundary -> Nat
  stateContextForkEpochScope : Boundary -> Nat
  stateContextAggregationMode : Boundary -> Nat

/--
Structural canonicality for `p_state(q)`: the transition class is a
deterministic function of the full state boundary and context.
-/
structure StateProjectionCanonicality
    (S : StateProjectionFields) where
  state_transition_class_eq :
    forall q,
      S.stateTransitionClass q =
        S.accountStateRootBefore q +
          S.admittedBoundaryId q +
            S.liveStateRequirement q +
              S.consumedSuccessorStateClass q +
                S.dataAvailabilityRequirement q +
                  S.aggregationStateBoundary q +
                    S.stateContextVersion q +
                      S.stateContextParameterSet q +
                        S.stateContextVerifierFamily q +
                          S.stateContextSpendDomain q +
                            S.stateContextChainApplicationDomain q +
                              S.stateContextForkEpochScope q +
                                S.stateContextAggregationMode q

/-- Concrete first-pass gluing checks for the edge-linked active encoding. -/
structure ConcreteGluingChecks
    (F : ConcreteGluingFields) where
  spend_state_edge :
    forall q, F.spendStateClass q = F.stateTransitionClass q
  spend_transcript_aggregation_edge :
    forall q, F.transcriptAggregationMode q = F.spendAggregationMode q
  spend_transcript_binding_edge :
    forall q, F.transcriptSpendBinding q = F.spendClassBinding q
  context_aggregation_pullback :
    forall q, F.contextAggregationMode q = F.spendAggregationMode q
  context_verifier_pullback :
    forall q, F.contextVerifierFamily q = F.spendVerifierFamily q
  context_domain_pullback :
    forall q, F.contextSpendDomain q = F.spendDomainSeparator q
  transcript_context_pullback :
    forall q, F.transcriptContextBinding q = F.contextClassBinding q

/-- Equality facts exported by the concrete gluing checks. -/
structure ConcreteGluingConsequences
    (F : ConcreteGluingFields)
    (q : Boundary) where
  state_class_bound :
    F.spendStateClass q = F.stateTransitionClass q
  spend_transcript_aggregation_bound :
    F.transcriptAggregationMode q = F.spendAggregationMode q
  context_spend_aggregation_bound :
    F.contextAggregationMode q = F.spendAggregationMode q
  transcript_spend_bound :
    F.transcriptSpendBinding q = F.spendClassBinding q
  verifier_family_bound :
    F.contextVerifierFamily q = F.spendVerifierFamily q
  spend_domain_bound :
    F.contextSpendDomain q = F.spendDomainSeparator q
  transcript_context_bound :
    F.transcriptContextBinding q = F.contextClassBinding q

/-- Combined certificate package for the active native account design. -/
structure NativeAccountDesignCertificate
    (M : MeaningProjection)
    (O : ObstructionMap)
    (CObs : ObstructionComponentPredicates)
    (P : PrivateFiberPredicates)
    (Proj : PublicProjections)
    (V : VerifierPredicates) where
  projectionCompleteness : ProjectionCompleteness M Proj
  effectiveDescent : EffectiveDescent O P
  authorizationClassBinding : AuthorizationClassBinding P Proj
  fiberPrivacy : FiberPrivacy M P
  checkedCoreSpendReduction : CheckedCoreSpendReduction M Proj V

/--
Consensus-facing public account admission theorem interface.

An accepted native boundary either has zero obstruction and is a coherent
public semantic boundary, or exposes a named structural break. This is the
compact safety-only account creation theorem; it does not require consensus to
extract private signer state.
-/
structure PublicAdmissionSoundness
    (O : ObstructionMap)
    (CObs : ObstructionComponentPredicates)
    (B : PublicBoundaryPredicates)
    (V : VerifierPredicates) where
  accepts_public_boundary_or_break :
    forall {q}, V.acceptCreate q ->
      (CheckedObstructionZero O CObs q ∧ B.publicBoundarySound q) ∨
        exists breakEvent, V.breakFor q breakEvent

/--
Honest wallet/keygen creation theorem interface.

An honest creator emits a public boundary together with signer-held private
state that is locally valid and realizes the public spend class.
-/
structure HonestCreationSoundness
    (O : ObstructionMap)
    (P : PrivateFiberPredicates)
    (Proj : PublicProjections)
    (C : HonestCreationPredicates) where
  honest_create_valid :
    forall {seed q h}, C.honestCreate seed q h ->
      ObstructionZero O q ∧
        LocalValid O P q h ∧
          AuthorizationClassMatches P Proj q h

/-- Certificate package for the default compact public-admission mode. -/
structure NativePublicAccountDesignCertificate
    (M : MeaningProjection)
    (O : ObstructionMap)
    (CObs : ObstructionComponentPredicates)
    (B : PublicBoundaryPredicates)
    (P : PrivateFiberPredicates)
    (Proj : PublicProjections)
    (V : VerifierPredicates)
    (View : PublicViewPredicates)
    (C : HonestCreationPredicates) where
  projectionCompleteness : ProjectionCompleteness M Proj
  publicEffectiveDescent : PublicEffectiveDescent O B
  publicAdmissionSoundness : PublicAdmissionSoundness O CObs B V
  honestCreationSoundness : HonestCreationSoundness O P Proj C
  authorizationClassBinding : AuthorizationClassBinding P Proj
  fiberPrivacy : FiberPrivacy M P
  fiberNoExportSoundness : FiberNoExportSoundness O P View V
  checkedCoreSpendReduction : CheckedCoreSpendReduction M Proj V

/--
Spendability-certified account admission theorem interface.

An accepted native boundary either has zero obstruction and a valid private
fiber point, or exposes a named structural break. This is stronger than the
default public admission theorem and should be used only when account creation
must reject unspendable public boundaries at consensus time.
-/
structure AccountAdmissionSoundness
    (O : ObstructionMap)
    (P : PrivateFiberPredicates)
    (V : VerifierPredicates) where
  accepts_zero_or_break :
    forall {q}, V.acceptCreate q ->
      (ObstructionZero O q ∧ exists h, LocalValid O P q h) ∨
        exists breakEvent, V.breakFor q breakEvent

theorem account_admission_or_break
    {O : ObstructionMap}
    {P : PrivateFiberPredicates}
    {V : VerifierPredicates}
    (S : AccountAdmissionSoundness O P V)
    {q : Boundary}
    (hAccept : V.acceptCreate q) :
    (ObstructionZero O q ∧ exists h, LocalValid O P q h) ∨
      exists breakEvent, V.breakFor q breakEvent :=
  S.accepts_zero_or_break hAccept

/-- Checked obstruction zero includes ordinary zero obstruction. -/
theorem obstruction_zero_of_checked
    {O : ObstructionMap}
    {CObs : ObstructionComponentPredicates}
    {q : Boundary}
    (hchecked : CheckedObstructionZero O CObs q) :
    ObstructionZero O q :=
  hchecked.1

/-- Checked obstruction zero exposes every checked obstruction component. -/
theorem checked_obstruction_components
    {O : ObstructionMap}
    {CObs : ObstructionComponentPredicates}
    {q : Boundary}
    (hchecked : CheckedObstructionZero O CObs q) :
    CObs.contextChecked q ∧
      CObs.pullbackChecked q ∧
        CObs.gluingChecked q ∧
          CObs.meaningChecked q ∧
            CObs.spendStateChecked q ∧
              CObs.canonicalityChecked q :=
  hchecked.2

/--
Public admission gives a coherent public boundary, or a named break.
-/
theorem public_admission_or_break
    {O : ObstructionMap}
    {CObs : ObstructionComponentPredicates}
    {B : PublicBoundaryPredicates}
    {V : VerifierPredicates}
    (S : PublicAdmissionSoundness O CObs B V)
    {q : Boundary}
    (hAccept : V.acceptCreate q) :
    (CheckedObstructionZero O CObs q ∧ B.publicBoundarySound q) ∨
      exists breakEvent, V.breakFor q breakEvent :=
  S.accepts_public_boundary_or_break hAccept

/--
Effective descent plus zero obstruction gives a local private fiber point.
-/
theorem local_valid_of_zero_obstruction
    {O : ObstructionMap}
    {P : PrivateFiberPredicates}
    (D : EffectiveDescent O P)
    {q : Boundary}
    (hzero : ObstructionZero O q) :
    exists h, LocalValid O P q h :=
  D.lift_exists hzero

/--
Public effective descent plus zero obstruction gives a coherent public boundary.
-/
theorem public_boundary_sound_of_zero_obstruction
    {O : ObstructionMap}
    {B : PublicBoundaryPredicates}
    (D : PublicEffectiveDescent O B)
    {q : Boundary}
    (hzero : ObstructionZero O q) :
    B.publicBoundarySound q :=
  D.public_sound hzero

/--
Context well-formedness exports validity of version, parameters, verifier
family, domains, and aggregation mode.
-/
theorem context_wellformed
    {Ctxt : ContextPredicates}
    (W : ContextWellFormedness Ctxt)
    (q : Boundary) :
    Ctxt.versionValid q ∧
      Ctxt.parameterSetValid q ∧
        Ctxt.verifierFamilyValid q ∧
          Ctxt.domainsCanonical q ∧
            Ctxt.aggregationModeValid q :=
  W.context_wellformed q

/--
Honest creation returns a locally valid private fiber over a zero-obstruction
public boundary, with authorization class fixed by `pSpend`.
-/
theorem honest_create_local_valid
    {O : ObstructionMap}
    {P : PrivateFiberPredicates}
    {Proj : PublicProjections}
    {C : HonestCreationPredicates}
    (S : HonestCreationSoundness O P Proj C)
    {seed : Nat} {q : Boundary} {h : PrivateFiber}
    (hcreate : C.honestCreate seed q h) :
    ObstructionZero O q ∧
      LocalValid O P q h ∧
        AuthorizationClassMatches P Proj q h :=
  S.honest_create_valid hcreate

/--
Compatible authorization-local material has the public spend class projected
from the native boundary.
-/
theorem auth_class_eq_pSpend
    {P : PrivateFiberPredicates}
    {Proj : PublicProjections}
    (A : AuthorizationClassBinding P Proj)
    {q : Boundary} {h : PrivateFiber}
    (hauth : P.authorizationCompatible q h) :
    P.authClass h = Proj.pSpend q :=
  A.auth_class_matches hauth

/--
Authorization class binding upgrades compatibility to compatibility over the
specific public spend class projected from `q`.
-/
theorem auth_compatible_over_class
    {P : PrivateFiberPredicates}
    {Proj : PublicProjections}
    (A : AuthorizationClassBinding P Proj)
    {q : Boundary} {h : PrivateFiber}
    (hauth : P.authorizationCompatible q h) :
    AuthorizationCompatibleOverClass P Proj q h :=
  ⟨hauth, A.auth_class_matches hauth⟩

/--
Honest creation returns authorization-local material compatible with the public
spend class projected from the emitted boundary.
-/
theorem honest_create_authorization_over_class
    {O : ObstructionMap}
    {P : PrivateFiberPredicates}
    {Proj : PublicProjections}
    {C : HonestCreationPredicates}
    (S : HonestCreationSoundness O P Proj C)
    {seed : Nat} {q : Boundary} {h : PrivateFiber}
    (hcreate : C.honestCreate seed q h) :
    AuthorizationCompatibleOverClass P Proj q h :=
  by
    exact ⟨(S.honest_create_valid hcreate).2.1.2.2.1,
      (S.honest_create_valid hcreate).2.2⟩

/--
A public view produced from a locally valid private fiber point is simulatable
from public inputs, or exposes a named break.
-/
theorem public_view_simulatable_or_break
    {O : ObstructionMap}
    {P : PrivateFiberPredicates}
    {View : PublicViewPredicates}
    {V : VerifierPredicates}
    (S : FiberNoExportSoundness O P View V)
    {q : Boundary} {h : PrivateFiber} {sig msg state view : Nat}
    (hvalid : LocalValid O P q h)
    (hview : View.publicViewProduced q h sig msg state view) :
    View.simulatableFromPublic q sig msg state view ∨
      exists breakEvent, V.breakFor q breakEvent :=
  S.public_view_simulatable_or_break hvalid hview

/--
Conforming public outputs satisfy the active no-export denylist policy.
-/
theorem public_output_no_export
    {Policy : PublicOutputPolicy}
    (N : PublicOutputNoExport Policy)
    {output : Nat}
    (hconforms : Policy.outputConforms output) :
    ¬ Policy.containsDenylistedField output :=
  N.no_denylisted_field hconforms

/--
Checked-core frame equality is a deterministic function of native projections,
message, state, and the prechallenge proof commitment.
-/
theorem checked_core_frame_eq
    {Proj : PublicProjections}
    (F : CheckedCoreFrameBinding Proj)
    (q : Boundary) (msg state commitment : Nat) :
    F.frame q msg state commitment =
      F.frameEncoder.encode [
        Proj.pSpend q,
        Proj.pLifecycle q,
        Proj.pTranscript q,
        Proj.pState q,
        msg,
        state,
        commitment
      ] :=
  F.frame_binds q msg state commitment

/--
The explicit native checked-core frame is determined by the concrete projection
and digest fields from `Path2NativeCheckedCoreFrameV1`.
-/
theorem native_checked_core_frame_eq
    {F : NativeCheckedCoreFrameFields}
    (B : NativeCheckedCoreFrameBinding F)
    (q : Boundary) (msg state commitment : Nat) :
    B.frame q msg state commitment =
      B.frameEncoder.encode [
        F.nativeBoundaryVersion q,
        F.parameterSetId q,
        F.verifierFamilyId q,
        F.accountCreationDomain q,
        F.spendVerifierDomain q,
        F.chainApplicationDomain q,
        F.forkEpochScope q,
        F.aggregationMode q,
        F.nativeBoundaryDigest q,
        F.accountDigest q,
        F.spendDigest q,
        F.lifecycleDigest q,
        F.transcriptDigest q,
        F.stateDigest q,
        F.messageDigest msg,
        F.currentStateDigest state,
        F.proofCommitment commitment
      ] :=
  B.frame_binds q msg state commitment

/--
Accepted Rust checked-core evidence instantiates the abstract Lean
`CheckedCoreSpendReduction.checkedCoreAccepts` predicate.
-/
theorem path2_native_checked_core_accepts_checkedCoreAccepts
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (B : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    {q : Boundary} {msg state : Nat}
    {nativeEvidence : NativeCheckedCoreSpendEvidence}
    (hAccept :
      Path2NativeCheckedCoreAcceptV1
        B.nativeFrameBinding B.verifierHandleFields q msg state
          nativeEvidence) :
    B.reduction.checkedCoreAccepts q msg state
      nativeEvidence.toCheckedCoreSpendEvidence :=
  (B.checked_accepts_iff_native).2
    ⟨nativeEvidence, rfl, hAccept⟩

/--
The accepted Rust checked-core frame is simultaneously the assembled native
`Path2NativeCheckedCoreFrameV1` frame and the frame consumed by the Lean
checked-core reduction.
-/
theorem path2_native_checked_core_frame_corresponds_to_checkedCore
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    (B : Path2NativeCheckedCoreAcceptV1Bridge M Proj V F)
    {q : Boundary} {msg state : Nat}
    {nativeEvidence : NativeCheckedCoreSpendEvidence}
    (hAccept :
      Path2NativeCheckedCoreAcceptV1
        B.nativeFrameBinding B.verifierHandleFields q msg state
          nativeEvidence) :
    nativeEvidence.checkedCoreFrame =
        B.nativeFrameBinding.frame q msg state
          nativeEvidence.proofCommitment ∧
      nativeEvidence.checkedCoreFrame =
        B.reduction.frameEncoder.encode [
          Proj.pSpend q,
          Proj.pLifecycle q,
          Proj.pTranscript q,
          Proj.pState q,
          msg,
          state,
          nativeEvidence.proofCommitment
        ] := by
  constructor
  · exact hAccept.1
  ·
    exact B.reduction.frame_bound_to_projections
      (path2_native_checked_core_accepts_checkedCoreAccepts B hAccept)

/--
After the Rust checked-core acceptor succeeds, failure of valid checked-core
evidence is charged only to named checked-core equation, frame-binding,
domain/QROM, or byte-parser breaks.
-/
theorem path2_native_checked_core_accept_valid_or_named_break
    {V : VerifierPredicates}
    {F : NativeCheckedCoreFrameFields}
    {B : NativeCheckedCoreFrameBinding F}
    {H : NativeCheckedCoreVerifierHandleFields}
    (S : Path2NativeCheckedCoreAcceptV1Soundness V F B H)
    {q : Boundary} {msg state : Nat}
    {evidence : NativeCheckedCoreSpendEvidence}
    (hAccept : Path2NativeCheckedCoreAcceptV1 B H q msg state evidence) :
    S.validCheckedCoreEvidence q msg state evidence ∨
      V.breakFor q BreakEvent.checkedCoreEquationSoundnessBreak ∨
        V.breakFor q BreakEvent.checkedCoreFrameBindingBreak ∨
          V.breakFor q BreakEvent.domainBindingOrQromBreak ∨
            V.breakFor q BreakEvent.byteParserCanonicalityBreak :=
  S.accept_valid_or_named_break hAccept

/-- The native boundary digest excludes the transcript digest. -/
theorem native_boundary_digest_acyclic_eq
    {D : NativeProjectionDigestFields}
    (A : NativeBoundaryDigestAssembly D)
    (q : Boundary) :
    D.nativeBoundaryDigest q =
      A.boundaryEncoder.encode [
        D.contextDigest q,
        D.accountDigest q,
        D.spendDigest q,
        D.lifecycleDigest q,
        D.stateDigest q
      ] := by
  rw [A.boundary_matches_field q, A.boundary_binds q]

/--
The transcript digest is assembled after the boundary digest and binds that
already-fixed boundary handle together with the child projection digests.
-/
theorem native_transcript_digest_binds_boundary_eq
    {D : NativeProjectionDigestFields}
    (A : NativeTranscriptDigestAssembly D)
    (q : Boundary) :
    D.transcriptDigest q =
      A.transcriptEncoder.encode [
        D.nativeBoundaryDigest q,
        D.contextDigest q,
        D.accountDigest q,
        D.spendDigest q,
        D.lifecycleDigest q,
        D.stateDigest q
      ] := by
  rw [A.transcript_matches_field q, A.transcript_binds q]

/-- Combined acyclic assembly order: child projections, boundary, transcript. -/
theorem native_acyclic_assembly_eqs
    {D : NativeProjectionDigestFields}
    (A : NativeAcyclicAssembly D)
    (q : Boundary) :
    D.nativeBoundaryDigest q =
        A.boundary.boundaryEncoder.encode [
          D.contextDigest q,
          D.accountDigest q,
          D.spendDigest q,
          D.lifecycleDigest q,
          D.stateDigest q
        ]
      ∧
    D.transcriptDigest q =
        A.transcript.transcriptEncoder.encode [
          D.nativeBoundaryDigest q,
          D.contextDigest q,
          D.accountDigest q,
          D.spendDigest q,
          D.lifecycleDigest q,
          D.stateDigest q
        ] := by
  constructor
  · exact native_boundary_digest_acyclic_eq A.boundary q
  · exact native_transcript_digest_binds_boundary_eq A.transcript q

/--
If checked-core frame digest fields are bound to the assembled native
projection digests, then the frame cannot use independent boundary/transcript
digest handles.
-/
theorem native_assembled_checked_core_frame_eq
    {D : NativeProjectionDigestFields}
    {F : NativeCheckedCoreFrameFields}
    (A : NativeAcyclicAssembly D)
    (Bind : NativeCheckedCoreFrameDigestBinding D F)
    (B : NativeCheckedCoreFrameBinding F)
    (q : Boundary) (msg state commitment : Nat) :
    B.frame q msg state commitment =
      B.frameEncoder.encode [
        F.nativeBoundaryVersion q,
        F.parameterSetId q,
        F.verifierFamilyId q,
        F.accountCreationDomain q,
        F.spendVerifierDomain q,
        F.chainApplicationDomain q,
        F.forkEpochScope q,
        F.aggregationMode q,
        A.boundary.boundaryEncoder.encode [
          D.contextDigest q,
          D.accountDigest q,
          D.spendDigest q,
          D.lifecycleDigest q,
          D.stateDigest q
        ],
        D.accountDigest q,
        D.spendDigest q,
        D.lifecycleDigest q,
        A.transcript.transcriptEncoder.encode [
          A.boundary.boundaryEncoder.encode [
            D.contextDigest q,
            D.accountDigest q,
            D.spendDigest q,
            D.lifecycleDigest q,
            D.stateDigest q
          ],
          D.contextDigest q,
          D.accountDigest q,
          D.spendDigest q,
          D.lifecycleDigest q,
          D.stateDigest q
        ],
        D.stateDigest q,
        F.messageDigest msg,
        F.currentStateDigest state,
        F.proofCommitment commitment
      ] := by
  rw [B.frame_binds q msg state commitment]
  rw [Bind.boundary_digest_bound q]
  rw [Bind.account_digest_bound q]
  rw [Bind.spend_digest_bound q]
  rw [Bind.lifecycle_digest_bound q]
  rw [Bind.transcript_digest_bound q]
  rw [Bind.state_digest_bound q]
  rw [native_boundary_digest_acyclic_eq A.boundary q]
  rw [native_transcript_digest_binds_boundary_eq A.transcript q]
  rw [native_boundary_digest_acyclic_eq A.boundary q]

/--
The active native public views expose only assembled public projection digests
and public message/state/proof handles.
-/
theorem native_public_views_assembled_eqs
    {D : NativeProjectionDigestFields}
    {F : NativeCheckedCoreFrameFields}
    {V : NativePublicViewFields}
    (A : NativeAcyclicAssembly D)
    (P : NativePublicViewAssembly D F V)
    (q : Boundary) (msg state proofBytes : Nat) :
    V.accountView q =
        P.accountViewEncoder.encode [
          A.boundary.boundaryEncoder.encode [
            D.contextDigest q,
            D.accountDigest q,
            D.spendDigest q,
            D.lifecycleDigest q,
            D.stateDigest q
          ],
          D.contextDigest q,
          D.accountDigest q,
          D.spendDigest q,
          D.lifecycleDigest q,
          A.transcript.transcriptEncoder.encode [
            A.boundary.boundaryEncoder.encode [
              D.contextDigest q,
              D.accountDigest q,
              D.spendDigest q,
              D.lifecycleDigest q,
              D.stateDigest q
            ],
            D.contextDigest q,
            D.accountDigest q,
            D.spendDigest q,
            D.lifecycleDigest q,
            D.stateDigest q
          ],
          D.stateDigest q
        ]
      ∧
    V.spendView q msg state proofBytes =
        P.spendViewEncoder.encode [
          A.boundary.boundaryEncoder.encode [
            D.contextDigest q,
            D.accountDigest q,
            D.spendDigest q,
            D.lifecycleDigest q,
            D.stateDigest q
          ],
          F.messageDigest msg,
          F.currentStateDigest state,
          F.proofCommitment proofBytes,
          V.proofBytesDigest proofBytes,
          V.proofByteLength proofBytes
        ] := by
  constructor
  · rw [P.account_view_binds q]
    rw [native_boundary_digest_acyclic_eq A.boundary q]
    rw [native_transcript_digest_binds_boundary_eq A.transcript q]
    rw [native_boundary_digest_acyclic_eq A.boundary q]
  · rw [P.spend_view_binds q msg state proofBytes]
    rw [native_boundary_digest_acyclic_eq A.boundary q]

/--
Concrete first-pass gluing checks export the equality facts needed by the
edge-linked spend/state and aggregation-mode encoding.
-/
theorem concrete_gluing_consequences
    {F : ConcreteGluingFields}
    (G : ConcreteGluingChecks F)
    (q : Boundary) :
    ConcreteGluingConsequences F q where
  state_class_bound := G.spend_state_edge q
  spend_transcript_aggregation_bound := G.spend_transcript_aggregation_edge q
  context_spend_aggregation_bound := G.context_aggregation_pullback q
  transcript_spend_bound := G.spend_transcript_binding_edge q
  verifier_family_bound := G.context_verifier_pullback q
  spend_domain_bound := G.context_domain_pullback q
  transcript_context_bound := G.transcript_context_pullback q

/--
The state transition class is fixed by the canonical state projection fields.
-/
theorem pstate_transition_class_unique
    {S : StateProjectionFields}
    (C : StateProjectionCanonicality S)
    (q : Boundary) :
    S.stateTransitionClass q =
      S.accountStateRootBefore q +
        S.admittedBoundaryId q +
          S.liveStateRequirement q +
            S.consumedSuccessorStateClass q +
              S.dataAvailabilityRequirement q +
                S.aggregationStateBoundary q +
                  S.stateContextVersion q +
                    S.stateContextParameterSet q +
                      S.stateContextVerifierFamily q +
                        S.stateContextSpendDomain q +
                          S.stateContextChainApplicationDomain q +
                            S.stateContextForkEpochScope q +
                              S.stateContextAggregationMode q :=
  C.state_transition_class_eq q

/--
An accepted spend is semantically determined by the native boundary projections.
-/
theorem spend_accept_has_boundary_action
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    (S : SpendClosure M Proj V)
    {q : Boundary} {sig msg state : Nat}
    (haccept : V.spendAccept q sig msg state) :
    exists action, SpendActionFromBoundary M Proj q msg state action :=
  S.spend_action haccept

/--
Checked-core reduction obligations derive spend closure, so downstream theorem
wrappers do not need to assume `SpendClosure` directly.
-/
theorem checked_core_reduction_implies_spend_closure
    {M : MeaningProjection}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    (R : CheckedCoreSpendReduction M Proj V) :
    SpendClosure M Proj V where
  spend_action := by
    intro q sig msg state hspend
    rcases R.evidence_of_spend hspend with ⟨evidence, hChecked⟩
    have _hFrame := R.frame_bound_to_projections hChecked
    have _hAuth := R.authorization_class_bound hChecked
    have _hLifecycle := R.lifecycle_bound hChecked
    have _hState := R.state_transition_bound hChecked
    have _hReplay := R.replay_scope_bound hChecked
    exact ⟨boundarySpendAction M Proj q msg, by
      simp [boundarySpendAction, SpendActionFromBoundary]⟩

/--
Projection completeness turns a projection-defined spend action into an action
whose authorization, lifecycle, and state fields are the corresponding
components of its semantic meaning.
-/
theorem spend_action_matches_meaning_of_projection_complete
    {M : MeaningProjection}
    {Proj : PublicProjections}
    (C : ProjectionCompleteness M Proj)
    {q : Boundary} {msg state : Nat} {action : PublicSpendAction}
    (hAction : SpendActionFromBoundary M Proj q msg state action) :
    SpendActionMatchesMeaning action := by
  rcases hAction with ⟨hMeaning, _hMsg, hState, hLifecycle, hAuthorization⟩
  constructor
  · rw [hAuthorization, hMeaning, C.spend_eq q]
  constructor
  · rw [hLifecycle, hMeaning, C.lifecycle_eq q]
  · rw [hState, hMeaning, C.state_eq q]

/--
Public account admission plus spend closure gives a boundary-determined spend
action whose action fields agree with semantic meaning, or exposes a named
admission break.
-/
theorem public_admission_spend_action_or_break
    {M : MeaningProjection}
    {O : ObstructionMap}
    {CObs : ObstructionComponentPredicates}
    {B : PublicBoundaryPredicates}
    {Proj : PublicProjections}
    {V : VerifierPredicates}
    (A : PublicAdmissionSoundness O CObs B V)
    (C : ProjectionCompleteness M Proj)
    (R : CheckedCoreSpendReduction M Proj V)
    {q : Boundary} {sig msg state : Nat}
    (hcreate : V.acceptCreate q)
    (hspend : V.spendAccept q sig msg state) :
    ((CheckedObstructionZero O CObs q ∧ B.publicBoundarySound q) ∧
      exists action,
        SpendActionFromBoundary M Proj q msg state action ∧
          SpendActionMatchesMeaning action) ∨
        exists breakEvent, V.breakFor q breakEvent :=
  match A.accepts_public_boundary_or_break hcreate with
  | Or.inl hpublic =>
      let S := checked_core_reduction_implies_spend_closure R
      let hAction := S.spend_action hspend
      Or.inl
        ⟨hpublic,
          hAction.elim
            (fun action hBoundary =>
              ⟨action, hBoundary,
                spend_action_matches_meaning_of_projection_complete C hBoundary⟩)⟩
  | Or.inr hbreak => Or.inr hbreak

/-- Cryptographic model selected for active native checked-core challenges. -/
inductive NativeHashAssumptionModel where
  | classicalRandomOracle
  | quantumRandomOracle
  | concreteSha256
  deriving DecidableEq, Repr

/--
Hash/query-budget boundary for the active Path 2 native verifier.

This is the Path 2 analogue of the q12289 hash-boundary record.  It is attached
to the deployed native/Fq12 verifier relation rather than the retired q12289
FS-core target.
-/
structure NativeHashBudgetBoundary where
  model : NativeHashAssumptionModel
  checkedCoreFrameDomainSeparated : Prop
  hiddenSourceFrameDomainSeparated : Prop
  queryBudget : Nat
  multiTargetBudget : Nat
  queryBudgetPriced : Prop
  multiTargetLossPriced : Prop

/--
Accepted-spend soundness boundary for the active Path 2 verifier.

This is intentionally abstract over the final proof family.  The existing
native-boundary theorems provide structural `or break` statements; a deployable
security claim must turn those statements into a probability bound for accepted
spend forgeries under named bad events.
-/
structure NativeAcceptedSpendSoundnessBoundary where
  acceptedSpendForgeryProb : ℝ
  acceptedSpendForgeryBound : ℝ
  productionAcceptedSpendForgeryProb : ℝ
  productionAcceptedSpendForgeryBound : ℝ
  acceptedSpendForgeryProbPinned :
    acceptedSpendForgeryProb = productionAcceptedSpendForgeryProb
  acceptedSpendForgeryBoundPinned :
    acceptedSpendForgeryBound = productionAcceptedSpendForgeryBound
  acceptedSpendForgery_le_bound :
    acceptedSpendForgeryProb <= acceptedSpendForgeryBound
  hiddenSourceProofSemanticsClosed : Prop
  checkedCoreCompositionClosed : Prop
  lifecycleReplayClosed : Prop
  noExportClosed : Prop

namespace NativeAcceptedSpendSoundnessBoundary

/-- The erased accepted-spend boundary exposes production probability pinning. -/
theorem acceptedSpendForgeryProb_eq_production
    (B : NativeAcceptedSpendSoundnessBoundary) :
    B.acceptedSpendForgeryProb = B.productionAcceptedSpendForgeryProb :=
  B.acceptedSpendForgeryProbPinned

/-- The erased accepted-spend boundary exposes production bound pinning. -/
theorem acceptedSpendForgeryBound_eq_production
    (B : NativeAcceptedSpendSoundnessBoundary) :
    B.acceptedSpendForgeryBound = B.productionAcceptedSpendForgeryBound :=
  B.acceptedSpendForgeryBoundPinned

/-- The erased accepted-spend boundary exposes the accepted-spend probability inequality. -/
theorem acceptedSpendForgeryProb_le_bound
    (B : NativeAcceptedSpendSoundnessBoundary) :
    B.acceptedSpendForgeryProb <= B.acceptedSpendForgeryBound :=
  B.acceptedSpendForgery_le_bound

end NativeAcceptedSpendSoundnessBoundary

/-- The all-zero 32-byte digest placeholder rejected by Rust evidence verifiers. -/
def selectedRustZeroDigest32 : ByteArray :=
  ByteArray.mk (Array.replicate 32 (0 : UInt8))

/--
Lean mirror of the selected Rust accepted-spend numeric evidence report.

The deployable theorem must bind the accepted-spend probability, bound,
inequality proof, and prerequisite closure facts to reviewed production
evidence before constructing the active accepted-spend boundary.
-/
structure SelectedRustAcceptedSpendSoundnessEvidenceReport where
  schemaMatches : Bool
  reportDigestVerified : Bool
  numericEvidenceDigestsVerified : Bool
  finiteTraceSupportDigest : ByteArray
  activeOutcomeCarrierDigest : ByteArray
  acceptedSpendEventSemanticsDigest : ByteArray
  activeBreakFamilyDigest : ByteArray
  namedBreakEventsDigest : ByteArray
  acceptedSpendUnionBoundPricingDigest : ByteArray
  acceptedSpendPricedBoundDigest : ByteArray
  targetSecurityBits : Nat
  selectedMultiTargetHorizonLog2 : Nat
  qromQuadraticLossBitsAtDeploymentBound : Nat
  transcriptTermBitsAfterQromLoss : Nat
  acceptedSpendPricedBoundSecurityBits : Nat
  acceptedSpendNumericBoundPriced : Bool
  finiteTraceSupportPriced : Bool
  activeOutcomeCarrierPinned : Bool
  acceptedSpendEventPinned : Bool
  activeBreakFamilyPinned : Bool
  namedBreakEventsPinned : Bool
  acceptedSpendUnionBoundPriced : Bool
  acceptedSpendEventBoundPriced : Bool
  namedBreakEventBoundsPriced : Bool
  breakPricingBudgetMeetsPricedBound : Bool
  acceptedSpendNumericBudgetMeetsTarget : Bool
  hiddenSourceClosureReady : Bool
  checkedCoreClosureReady : Bool
  lifecycleReplayClosureReady : Bool
  noExportClosureReady : Bool

/-- The accepted-spend numeric report has the expected schema and digest. -/
def SelectedRustAcceptedSpendSoundnessEvidenceReport.ReportVerified
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport) : Prop :=
  E.schemaMatches = true ∧
  E.reportDigestVerified = true ∧
  E.numericEvidenceDigestsVerified = true ∧
  E.finiteTraceSupportDigest ≠ selectedRustZeroDigest32 ∧
  E.activeOutcomeCarrierDigest ≠ selectedRustZeroDigest32 ∧
  E.acceptedSpendEventSemanticsDigest ≠ selectedRustZeroDigest32 ∧
  E.activeBreakFamilyDigest ≠ selectedRustZeroDigest32 ∧
  E.namedBreakEventsDigest ≠ selectedRustZeroDigest32 ∧
  E.acceptedSpendUnionBoundPricingDigest ≠ selectedRustZeroDigest32 ∧
  E.acceptedSpendPricedBoundDigest ≠ selectedRustZeroDigest32

/--
Proof object for the accepted-spend numeric evidence digest slots.

This keeps the verified-report proof from treating
`numericEvidenceDigestsVerified` as an opaque flag: every selected digest slot
must be carried with its own nonzero proof before the report can be accepted.
-/
structure SelectedRustAcceptedSpendNumericDigestSlotsCertificate
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport) where
  numericEvidenceDigestsVerifiedProof :
    E.numericEvidenceDigestsVerified = true
  finiteTraceSupportDigestNonzero :
    E.finiteTraceSupportDigest ≠ selectedRustZeroDigest32
  activeOutcomeCarrierDigestNonzero :
    E.activeOutcomeCarrierDigest ≠ selectedRustZeroDigest32
  acceptedSpendEventSemanticsDigestNonzero :
    E.acceptedSpendEventSemanticsDigest ≠ selectedRustZeroDigest32
  activeBreakFamilyDigestNonzero :
    E.activeBreakFamilyDigest ≠ selectedRustZeroDigest32
  namedBreakEventsDigestNonzero :
    E.namedBreakEventsDigest ≠ selectedRustZeroDigest32
  acceptedSpendUnionBoundPricingDigestNonzero :
    E.acceptedSpendUnionBoundPricingDigest ≠ selectedRustZeroDigest32
  acceptedSpendPricedBoundDigestNonzero :
    E.acceptedSpendPricedBoundDigest ≠ selectedRustZeroDigest32

namespace SelectedRustAcceptedSpendNumericDigestSlotsCertificate

/-- Convert the selected digest-slot certificate plus report pins into report verification. -/
theorem toReportVerified
    {E : SelectedRustAcceptedSpendSoundnessEvidenceReport}
    (C : SelectedRustAcceptedSpendNumericDigestSlotsCertificate E)
    (schemaMatchesProof : E.schemaMatches = true)
    (reportDigestVerifiedProof : E.reportDigestVerified = true) :
    E.ReportVerified :=
  ⟨schemaMatchesProof, reportDigestVerifiedProof,
    C.numericEvidenceDigestsVerifiedProof,
    C.finiteTraceSupportDigestNonzero,
    C.activeOutcomeCarrierDigestNonzero,
    C.acceptedSpendEventSemanticsDigestNonzero,
    C.activeBreakFamilyDigestNonzero,
    C.namedBreakEventsDigestNonzero,
    C.acceptedSpendUnionBoundPricingDigestNonzero,
    C.acceptedSpendPricedBoundDigestNonzero⟩

/-- The digest-slot certificate exposes the aggregate digest verification bit. -/
theorem numericEvidenceDigestsVerified
    {E : SelectedRustAcceptedSpendSoundnessEvidenceReport}
    (C : SelectedRustAcceptedSpendNumericDigestSlotsCertificate E) :
    E.numericEvidenceDigestsVerified = true :=
  C.numericEvidenceDigestsVerifiedProof

end SelectedRustAcceptedSpendNumericDigestSlotsCertificate

/-- A verified accepted-spend report has the selected schema. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_schemaMatches
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.ReportVerified) :
    E.schemaMatches = true := by
  exact h.1

/-- A verified accepted-spend report has its report digest checked. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_reportDigestVerified
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.ReportVerified) :
    E.reportDigestVerified = true := by
  exact h.2.1

/-- A verified accepted-spend report has its numeric evidence digests checked. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_numericEvidenceDigestsVerified
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.ReportVerified) :
    E.numericEvidenceDigestsVerified = true := by
  exact h.2.2.1

/-- A verified accepted-spend report has a nonzero finite-trace digest slot. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_finiteTraceSupportDigest_nonzero
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.ReportVerified) :
    E.finiteTraceSupportDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, hFinite, _, _, _, _, _, _⟩
  exact hFinite

/-- A verified accepted-spend report has a nonzero active-outcome digest slot. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_activeOutcomeCarrierDigest_nonzero
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.ReportVerified) :
    E.activeOutcomeCarrierDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, hOutcome, _, _, _, _, _⟩
  exact hOutcome

/-- A verified accepted-spend report has a nonzero accepted-event digest slot. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_acceptedSpendEventSemanticsDigest_nonzero
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.ReportVerified) :
    E.acceptedSpendEventSemanticsDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, _, hEvent, _, _, _, _⟩
  exact hEvent

/-- A verified accepted-spend report has a nonzero active-break digest slot. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_activeBreakFamilyDigest_nonzero
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.ReportVerified) :
    E.activeBreakFamilyDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, _, _, hBreak, _, _, _⟩
  exact hBreak

/-- A verified accepted-spend report has a nonzero named-break digest slot. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_namedBreakEventsDigest_nonzero
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.ReportVerified) :
    E.namedBreakEventsDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, _, _, _, hNamed, _, _⟩
  exact hNamed

/-- A verified accepted-spend report has a nonzero union-pricing digest slot. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_unionBoundPricingDigest_nonzero
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.ReportVerified) :
    E.acceptedSpendUnionBoundPricingDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, _, _, _, _, hUnion, _⟩
  exact hUnion

/-- A verified accepted-spend report has a nonzero priced-bound digest slot. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_pricedBoundDigest_nonzero
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.ReportVerified) :
    E.acceptedSpendPricedBoundDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, _, _, _, _, _, hPriced⟩
  exact hPriced

/-- All accepted-spend numeric and prerequisite closure facts are true. -/
def SelectedRustAcceptedSpendSoundnessEvidenceReport.FactsAllTrue
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport) : Prop :=
  E.acceptedSpendNumericBoundPriced = true ∧
  E.finiteTraceSupportPriced = true ∧
  E.activeOutcomeCarrierPinned = true ∧
  E.acceptedSpendEventPinned = true ∧
  E.activeBreakFamilyPinned = true ∧
  E.namedBreakEventsPinned = true ∧
  E.acceptedSpendUnionBoundPriced = true ∧
  E.acceptedSpendEventBoundPriced = true ∧
  E.namedBreakEventBoundsPriced = true ∧
  E.breakPricingBudgetMeetsPricedBound = true ∧
  E.acceptedSpendNumericBudgetMeetsTarget = true ∧
  E.targetSecurityBits = 128 ∧
  E.selectedMultiTargetHorizonLog2 = 32 ∧
  E.qromQuadraticLossBitsAtDeploymentBound = 32 ∧
  E.transcriptTermBitsAfterQromLoss = 182 ∧
  E.targetSecurityBits ≤ E.acceptedSpendPricedBoundSecurityBits ∧
  E.targetSecurityBits + E.selectedMultiTargetHorizonLog2 ≤
    E.transcriptTermBitsAfterQromLoss ∧
  E.acceptedSpendPricedBoundSecurityBits + E.selectedMultiTargetHorizonLog2 ≤
    E.transcriptTermBitsAfterQromLoss ∧
  E.hiddenSourceClosureReady = true ∧
  E.checkedCoreClosureReady = true ∧
  E.lifecycleReplayClosureReady = true ∧
  E.noExportClosureReady = true

/-- Structural accepted-spend evidence facts already selected from active sources. -/
def SelectedRustAcceptedSpendSoundnessEvidenceReport.StructuralFactsComplete
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport) : Prop :=
  E.finiteTraceSupportPriced = true ∧
  E.activeOutcomeCarrierPinned = true ∧
  E.acceptedSpendEventPinned = true ∧
  E.activeBreakFamilyPinned = true ∧
  E.namedBreakEventsPinned = true ∧
  E.acceptedSpendUnionBoundPriced = true ∧
  E.targetSecurityBits = 128 ∧
  E.selectedMultiTargetHorizonLog2 = 32 ∧
  E.qromQuadraticLossBitsAtDeploymentBound = 32 ∧
  E.transcriptTermBitsAfterQromLoss = 182 ∧
  E.targetSecurityBits + E.selectedMultiTargetHorizonLog2 ≤
    E.transcriptTermBitsAfterQromLoss ∧
  E.hiddenSourceClosureReady = true ∧
  E.checkedCoreClosureReady = true ∧
  E.lifecycleReplayClosureReady = true ∧
  E.noExportClosureReady = true

/-- Accepted-spend facts that still require production probability pricing. -/
def SelectedRustAcceptedSpendSoundnessEvidenceReport.PricingFactsComplete
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport) : Prop :=
  E.acceptedSpendNumericBoundPriced = true ∧
  E.acceptedSpendEventBoundPriced = true ∧
  E.namedBreakEventBoundsPriced = true ∧
  E.breakPricingBudgetMeetsPricedBound = true ∧
  E.acceptedSpendNumericBudgetMeetsTarget = true ∧
  E.targetSecurityBits ≤ E.acceptedSpendPricedBoundSecurityBits ∧
  E.acceptedSpendPricedBoundSecurityBits + E.selectedMultiTargetHorizonLog2 ≤
    E.transcriptTermBitsAfterQromLoss

/-- Accepted-spend report completeness is exactly structural evidence plus pricing evidence. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_iff_structural_and_pricing
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport) :
    E.FactsAllTrue ↔
      E.StructuralFactsComplete ∧ E.PricingFactsComplete := by
  constructor
  · intro h
    rcases h with
      ⟨hNumeric, hFinite, hOutcome, hEvent, hBreakFamily, hNamedBreaks,
        hUnion, hAcceptedBound, hNamedBreakBounds, hBreakBudget,
        hNumericBudget, hTarget, hHorizon, hQrom, hTranscript, hTargetBits,
        hActiveBudget, hClaimedBudget, hHidden, hChecked, hLifecycle,
        hNoExport⟩
    exact
      ⟨⟨hFinite, hOutcome, hEvent, hBreakFamily, hNamedBreaks, hUnion,
          hTarget, hHorizon, hQrom, hTranscript, hActiveBudget, hHidden,
          hChecked, hLifecycle, hNoExport⟩,
        ⟨hNumeric, hAcceptedBound, hNamedBreakBounds, hBreakBudget,
          hNumericBudget, hTargetBits, hClaimedBudget⟩⟩
  · intro h
    rcases h with
      ⟨⟨hFinite, hOutcome, hEvent, hBreakFamily, hNamedBreaks, hUnion,
          hTarget, hHorizon, hQrom, hTranscript, hActiveBudget, hHidden,
          hChecked, hLifecycle, hNoExport⟩,
        ⟨hNumeric, hAcceptedBound, hNamedBreakBounds, hBreakBudget,
          hNumericBudget, hTargetBits, hClaimedBudget⟩⟩
    exact
      ⟨hNumeric, hFinite, hOutcome, hEvent, hBreakFamily, hNamedBreaks,
        hUnion, hAcceptedBound, hNamedBreakBounds, hBreakBudget,
        hNumericBudget, hTarget, hHorizon, hQrom, hTranscript, hTargetBits,
        hActiveBudget, hClaimedBudget, hHidden, hChecked, hLifecycle,
        hNoExport⟩

/-- Assemble accepted-spend report completeness from the structural and pricing halves. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_of_structural_and_pricing
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (hStructural : E.StructuralFactsComplete)
    (hPricing : E.PricingFactsComplete) :
    E.FactsAllTrue :=
  (SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_iff_structural_and_pricing E).2
    ⟨hStructural, hPricing⟩

/-- A complete pricing half says the aggregate accepted-spend numeric bound is priced. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_numericBoundPriced
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.PricingFactsComplete) :
    E.acceptedSpendNumericBoundPriced = true :=
  h.1

/-- A complete pricing half says the accepted-spend event component is priced. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_acceptedSpendEventBoundPriced
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.PricingFactsComplete) :
    E.acceptedSpendEventBoundPriced = true :=
  h.2.1

/-- A complete pricing half says each named-break event component is priced. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_namedBreakEventBoundsPriced
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.PricingFactsComplete) :
    E.namedBreakEventBoundsPriced = true :=
  h.2.2.1

/-- A complete pricing half says the component budget fits below the priced bound. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_breakPricingBudgetMeetsPricedBound
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.PricingFactsComplete) :
    E.breakPricingBudgetMeetsPricedBound = true :=
  h.2.2.2.1

/-- A complete pricing half says the selected numeric budget meets target. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_numericBudgetMeetsTarget
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.PricingFactsComplete) :
    E.acceptedSpendNumericBudgetMeetsTarget = true :=
  h.2.2.2.2.1

/-- A complete pricing half says reported priced-bound bits meet target bits. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_pricedBoundSecurityMeetsTarget
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.PricingFactsComplete) :
    E.targetSecurityBits ≤ E.acceptedSpendPricedBoundSecurityBits :=
  h.2.2.2.2.2.1

/-- A complete pricing half says claimed priced-bound bits fit the transcript budget. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_pricedBoundSecurityWithinTranscript
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.PricingFactsComplete) :
    E.acceptedSpendPricedBoundSecurityBits + E.selectedMultiTargetHorizonLog2 ≤
      E.transcriptTermBitsAfterQromLoss :=
  h.2.2.2.2.2.2

/-- A complete accepted-spend report says the selected numeric bound is priced. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_numericBoundPriced
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.acceptedSpendNumericBoundPriced = true := by
  rcases h with
    ⟨hNumericBoundPriced, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hNumericBoundPriced

/-- A complete accepted-spend report prices the finite trace support. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_finiteTraceSupportPriced
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.finiteTraceSupportPriced = true := by
  rcases h with
    ⟨_, hFiniteTraceSupportPriced, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hFiniteTraceSupportPriced

/-- A complete accepted-spend report pins the active outcome carrier. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeOutcomeCarrierPinned
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.activeOutcomeCarrierPinned = true := by
  rcases h with
    ⟨_, _, hActiveOutcomeCarrierPinned, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hActiveOutcomeCarrierPinned

/-- A complete accepted-spend report pins accepted-spend event semantics. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_acceptedSpendEventPinned
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.acceptedSpendEventPinned = true := by
  rcases h with
    ⟨_, _, _, hAcceptedSpendEventPinned, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hAcceptedSpendEventPinned

/-- A complete accepted-spend report pins the active break family. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeBreakFamilyPinned
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.activeBreakFamilyPinned = true := by
  rcases h with
    ⟨_, _, _, _, hActiveBreakFamilyPinned, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hActiveBreakFamilyPinned

/-- A complete accepted-spend report pins named-break event semantics. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_namedBreakEventsPinned
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.namedBreakEventsPinned = true := by
  rcases h with
    ⟨_, _, _, _, _, hNamedBreakEventsPinned, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hNamedBreakEventsPinned

/-- A complete accepted-spend report prices the aggregate accepted-spend union bound. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_unionBoundPriced
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.acceptedSpendUnionBoundPriced = true := by
  rcases h with
    ⟨_, _, _, _, _, _, hUnionPriced, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hUnionPriced

/-- A complete accepted-spend report prices the reduced accepted-spend event bound. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_acceptedSpendEventBoundPriced
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.acceptedSpendEventBoundPriced = true := by
  rcases h with
    ⟨_, _, _, _, _, _, _, hAcceptedSpendBoundPriced, _, _, _, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hAcceptedSpendBoundPriced

/-- A complete accepted-spend report prices the active named-break event bounds. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_namedBreakEventBoundsPriced
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.namedBreakEventBoundsPriced = true := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, hNamedBreakBoundsPriced, _, _, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hNamedBreakBoundsPriced

/-- A complete accepted-spend report checks the break-pricing budget against the priced bound. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_breakPricingBudgetMeetsPricedBound
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.breakPricingBudgetMeetsPricedBound = true := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, hBreakBudgetPriced, _, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hBreakBudgetPriced

/-- A complete accepted-spend report says the selected numeric budget meets target. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_numericBudgetMeetsTarget
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.acceptedSpendNumericBudgetMeetsTarget = true := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, hNumericBudgetMeetsTarget, _, _, _, _, _, _, _, _, _, _, _⟩
  exact hNumericBudgetMeetsTarget

/-- A complete accepted-spend report proves its priced-bound bits meet target. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityMeetsTarget
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.targetSecurityBits ≤ E.acceptedSpendPricedBoundSecurityBits := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, hPricedBoundBits, _, _, _, _, _, _⟩
  exact hPricedBoundBits

/-- A complete accepted-spend report pins the active target-security bits. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_targetSecurityBits
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.targetSecurityBits = 128 := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, hTarget, _, _, _, _, _, _, _, _, _, _⟩
  exact hTarget

/-- A complete accepted-spend report pins the selected QROM loss. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_qromQuadraticLossBits
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.qromQuadraticLossBitsAtDeploymentBound = 32 := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, _, _, hQromLoss, _, _, _, _, _, _, _, _⟩
  exact hQromLoss

/-- A complete accepted-spend report prices the selected active multi-target horizon. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeBudgetMarginPriced
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.targetSecurityBits + E.selectedMultiTargetHorizonLog2 ≤
      E.transcriptTermBitsAfterQromLoss := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, hBudget, _, _, _, _, _⟩
  exact hBudget

/-- A complete accepted-spend report caps claimed bound bits by the active transcript budget. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityWithinTranscript
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.acceptedSpendPricedBoundSecurityBits + E.selectedMultiTargetHorizonLog2 ≤
      E.transcriptTermBitsAfterQromLoss := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, hClaimedBudget, _, _, _, _⟩
  exact hClaimedBudget

/-- The active report constants leave 22 QROM-discounted transcript bits after target plus horizon. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeBudgetSlackBits
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.transcriptTermBitsAfterQromLoss -
      (E.targetSecurityBits + E.selectedMultiTargetHorizonLog2) = 22 := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, hTarget, hHorizon, _, hTranscript, _, _, _, _, _, _, _⟩
  rw [hTarget, hHorizon, hTranscript]

/-- A complete report caps accepted-spend priced-bound bits by the selected transcript budget. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityBits_le_transcriptCap
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.acceptedSpendPricedBoundSecurityBits ≤ 150 := by
  have hWithin :
      E.acceptedSpendPricedBoundSecurityBits +
        E.selectedMultiTargetHorizonLog2 ≤
          E.transcriptTermBitsAfterQromLoss :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityWithinTranscript
      E h
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, _, hHorizon, _, hTranscript, _, _, _, _, _, _, _⟩
  rw [hHorizon, hTranscript] at hWithin
  omega

/-- A complete accepted-spend report shows the reported bound is within target security. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_reportedSecurityBound_le_targetSecurityBound
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    ((2 : ℝ) ^ E.acceptedSpendPricedBoundSecurityBits)⁻¹ <=
      ((2 : ℝ) ^ E.targetSecurityBits)⁻¹ := by
  exact
    inv_pow_le_inv_pow_of_le (a := (2 : ℝ)) (by norm_num)
      (SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityMeetsTarget
        E h)

/-- A complete accepted-spend report records hidden-source closure readiness. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_hiddenSourceClosureReady
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.hiddenSourceClosureReady = true := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, hHiddenSource, _, _, _⟩
  exact hHiddenSource

/-- A complete accepted-spend report records checked-core closure readiness. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_checkedCoreClosureReady
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.checkedCoreClosureReady = true := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, hCheckedCore, _, _⟩
  exact hCheckedCore

/-- A complete accepted-spend report records lifecycle/replay closure readiness. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_lifecycleReplayClosureReady
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.lifecycleReplayClosureReady = true := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, hLifecycle, _⟩
  exact hLifecycle

/-- A complete accepted-spend report records no-export closure readiness. -/
theorem SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_noExportClosureReady
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (h : E.FactsAllTrue) :
    E.noExportClosureReady = true := by
  rcases h with
    ⟨_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, hNoExport⟩
  exact hNoExport

/-- Selected Rust accepted-spend numeric evidence report mirrored from the Rust verifier. -/
def selectedRustAcceptedSpendEvidenceReport :
    SelectedRustAcceptedSpendSoundnessEvidenceReport where
  schemaMatches := true
  reportDigestVerified := true
  numericEvidenceDigestsVerified := true
  finiteTraceSupportDigest :=
    ByteArray.mk #[32, 157, 78, 216, 213, 224, 135, 35, 199, 215, 72, 122,
      115, 180, 144, 197, 164, 25, 41, 200, 64, 208, 98, 6, 165, 57, 106,
      183, 194, 133, 147, 86]
  activeOutcomeCarrierDigest :=
    ByteArray.mk #[12, 106, 231, 51, 5, 206, 105, 148, 94, 250, 178, 31,
      184, 94, 226, 157, 26, 55, 237, 33, 21, 223, 183, 249, 34, 163, 53,
      42, 43, 237, 74, 214]
  acceptedSpendEventSemanticsDigest :=
    ByteArray.mk #[35, 65, 67, 124, 201, 208, 218, 210, 237, 210, 97, 133,
      169, 250, 226, 203, 205, 32, 97, 64, 113, 250, 85, 134, 122, 143,
      222, 95, 6, 221, 238, 15]
  activeBreakFamilyDigest :=
    ByteArray.mk #[111, 3, 153, 201, 118, 101, 3, 131, 125, 95, 74, 112,
      104, 81, 255, 126, 60, 29, 241, 180, 122, 212, 12, 39, 240, 46, 175,
      234, 226, 41, 148, 226]
  namedBreakEventsDigest :=
    ByteArray.mk #[239, 239, 12, 86, 194, 199, 4, 19, 215, 33, 190, 24,
      211, 27, 72, 24, 50, 159, 159, 35, 131, 125, 226, 200, 211, 19, 104,
      71, 30, 51, 137, 43]
  acceptedSpendUnionBoundPricingDigest :=
    ByteArray.mk #[158, 137, 218, 102, 81, 161, 239, 189, 139, 194, 27,
      129, 37, 31, 11, 141, 42, 153, 24, 66, 119, 154, 9, 2, 152, 158, 215,
      226, 152, 143, 72, 164]
  acceptedSpendPricedBoundDigest :=
    ByteArray.mk #[187, 178, 56, 246, 207, 80, 245, 41, 254, 45, 53, 219,
      78, 215, 69, 4, 188, 30, 50, 47, 15, 127, 87, 174, 48, 150, 229, 160,
      109, 162, 224, 74]
  targetSecurityBits := 128
  selectedMultiTargetHorizonLog2 := 32
  qromQuadraticLossBitsAtDeploymentBound := 32
  transcriptTermBitsAfterQromLoss := 182
  acceptedSpendPricedBoundSecurityBits := 128
  acceptedSpendNumericBoundPriced := false
  finiteTraceSupportPriced := true
  activeOutcomeCarrierPinned := true
  acceptedSpendEventPinned := true
  activeBreakFamilyPinned := true
  namedBreakEventsPinned := true
  acceptedSpendUnionBoundPriced := true
  acceptedSpendEventBoundPriced := false
  namedBreakEventBoundsPriced := false
  breakPricingBudgetMeetsPricedBound := false
  acceptedSpendNumericBudgetMeetsTarget := true
  hiddenSourceClosureReady := true
  checkedCoreClosureReady := true
  lifecycleReplayClosureReady := true
  noExportClosureReady := true

/-- Digest pin for the selected Rust accepted-spend numeric evidence report. -/
def selectedRustAcceptedSpendEvidenceDigest : ByteArray :=
  ByteArray.mk #[58, 41, 97, 234, 177, 225, 24, 187, 29, 73, 144, 50,
    225, 195, 151, 44, 234, 221, 153, 89, 136, 196, 214, 185, 75, 82,
    138, 5, 39, 121, 187, 218]

/-- Selected digest-slot certificate for the current accepted-spend numeric report. -/
def selectedRustAcceptedSpendNumericDigestSlotsCertificate :
    SelectedRustAcceptedSpendNumericDigestSlotsCertificate
      selectedRustAcceptedSpendEvidenceReport where
  numericEvidenceDigestsVerifiedProof := by native_decide
  finiteTraceSupportDigestNonzero := by native_decide
  activeOutcomeCarrierDigestNonzero := by native_decide
  acceptedSpendEventSemanticsDigestNonzero := by native_decide
  activeBreakFamilyDigestNonzero := by native_decide
  namedBreakEventsDigestNonzero := by native_decide
  acceptedSpendUnionBoundPricingDigestNonzero := by native_decide
  acceptedSpendPricedBoundDigestNonzero := by native_decide

/-- The selected accepted-spend report has verified public digest slots. -/
theorem selectedRustAcceptedSpendEvidenceReportVerifiedProof :
    selectedRustAcceptedSpendEvidenceReport.ReportVerified :=
  selectedRustAcceptedSpendNumericDigestSlotsCertificate.toReportVerified
    (by native_decide)
    (by native_decide)

/-- The current selected accepted-spend report is intentionally not proof-complete. -/
theorem selectedRustAcceptedSpendEvidenceReportFactsAllTrue_impossible :
    ¬ selectedRustAcceptedSpendEvidenceReport.FactsAllTrue := by
  simp [SelectedRustAcceptedSpendSoundnessEvidenceReport.FactsAllTrue,
    selectedRustAcceptedSpendEvidenceReport]

/-- The current selected accepted-spend report has the active target/horizon transcript headroom. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentBudgetCapacity :
    selectedRustAcceptedSpendEvidenceReport.targetSecurityBits +
        selectedRustAcceptedSpendEvidenceReport.selectedMultiTargetHorizonLog2 <=
      selectedRustAcceptedSpendEvidenceReport.transcriptTermBitsAfterQromLoss := by
  native_decide

/-- The current selected accepted-spend report carries the active 128-bit priced-bound claim. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentPricedBoundBitsActiveTarget :
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendPricedBoundSecurityBits = 128 := by
  native_decide

/-- The current selected accepted-spend report's priced-bound bits meet the active target. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentPricedBoundMeetsTarget :
    selectedRustAcceptedSpendEvidenceReport.targetSecurityBits <=
      selectedRustAcceptedSpendEvidenceReport.acceptedSpendPricedBoundSecurityBits := by
  native_decide

/-- The current selected accepted-spend report's priced-bound bits fit the transcript cap. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentPricedBoundWithinTranscript :
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendPricedBoundSecurityBits +
        selectedRustAcceptedSpendEvidenceReport.selectedMultiTargetHorizonLog2 <=
      selectedRustAcceptedSpendEvidenceReport.transcriptTermBitsAfterQromLoss := by
  native_decide

/-- The current selected accepted-spend report carries the structural numeric-source pins. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentStructuralFactsTrue :
    selectedRustAcceptedSpendEvidenceReport.finiteTraceSupportPriced = true ∧
    selectedRustAcceptedSpendEvidenceReport.activeOutcomeCarrierPinned = true ∧
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendEventPinned = true ∧
    selectedRustAcceptedSpendEvidenceReport.activeBreakFamilyPinned = true ∧
    selectedRustAcceptedSpendEvidenceReport.namedBreakEventsPinned = true ∧
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendUnionBoundPriced = true := by
  native_decide

/-- The current selected accepted-spend report completes the structural half. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentStructuralFactsComplete :
    selectedRustAcceptedSpendEvidenceReport.StructuralFactsComplete := by
  simp [SelectedRustAcceptedSpendSoundnessEvidenceReport.StructuralFactsComplete,
    selectedRustAcceptedSpendEvidenceReport]

/-- The current selected accepted-spend report is still missing production pricing facts. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentMissingPricingFactsFalse :
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendNumericBoundPriced = false ∧
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendEventBoundPriced = false ∧
    selectedRustAcceptedSpendEvidenceReport.namedBreakEventBoundsPriced = false ∧
    selectedRustAcceptedSpendEvidenceReport.breakPricingBudgetMeetsPricedBound = false := by
  native_decide

/-- The current selected accepted-spend report has not priced the aggregate numeric bound. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentNumericBoundPriced_false :
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendNumericBoundPriced = false :=
  selectedRustAcceptedSpendEvidenceReport_currentMissingPricingFactsFalse.1

/-- The current selected accepted-spend report has not priced the accepted-spend event component. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentAcceptedSpendEventBoundPriced_false :
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendEventBoundPriced = false :=
  selectedRustAcceptedSpendEvidenceReport_currentMissingPricingFactsFalse.2.1

/-- The current selected accepted-spend report has not priced the named-break components. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentNamedBreakEventBoundsPriced_false :
    selectedRustAcceptedSpendEvidenceReport.namedBreakEventBoundsPriced = false :=
  selectedRustAcceptedSpendEvidenceReport_currentMissingPricingFactsFalse.2.2.1

/-- The current selected accepted-spend report has not shown the component budget fits the priced bound. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentBreakPricingBudgetMeetsPricedBound_false :
    selectedRustAcceptedSpendEvidenceReport.breakPricingBudgetMeetsPricedBound = false :=
  selectedRustAcceptedSpendEvidenceReport_currentMissingPricingFactsFalse.2.2.2

/-- The current selected accepted-spend report has shown the numeric budget reaches target. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentNumericBudgetMeetsTarget_true :
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendNumericBudgetMeetsTarget = true := by
  native_decide

/--
Lean mirror of the current accepted-spend pricing diagnostic.

This is deliberately fail-closed: it records the hard pricing facts that the
selected report is still missing before a pricing witness can exist.
-/
structure SelectedRustAcceptedSpendPricingObligationGap where
  acceptedSpendNumericBoundPricedMissing :
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendNumericBoundPriced = false
  acceptedSpendEventBoundPricedMissing :
    selectedRustAcceptedSpendEvidenceReport.acceptedSpendEventBoundPriced = false
  namedBreakEventBoundsPricedMissing :
    selectedRustAcceptedSpendEvidenceReport.namedBreakEventBoundsPriced = false
  breakPricingBudgetMeetsPricedBoundMissing :
    selectedRustAcceptedSpendEvidenceReport.breakPricingBudgetMeetsPricedBound = false

/-- Current selected accepted-spend pricing obligations still missing in Lean. -/
def selectedRustAcceptedSpendEvidenceReport_currentPricingObligationGap :
    SelectedRustAcceptedSpendPricingObligationGap where
  acceptedSpendNumericBoundPricedMissing :=
    selectedRustAcceptedSpendEvidenceReport_currentNumericBoundPriced_false
  acceptedSpendEventBoundPricedMissing :=
    selectedRustAcceptedSpendEvidenceReport_currentAcceptedSpendEventBoundPriced_false
  namedBreakEventBoundsPricedMissing :=
    selectedRustAcceptedSpendEvidenceReport_currentNamedBreakEventBoundsPriced_false
  breakPricingBudgetMeetsPricedBoundMissing :=
    selectedRustAcceptedSpendEvidenceReport_currentBreakPricingBudgetMeetsPricedBound_false

/-- The current accepted-spend pricing gap blocks the selected pricing half. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentPricingGap_blocksPricingFactsComplete
    (Gap : SelectedRustAcceptedSpendPricingObligationGap) :
    ¬ selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete := by
  intro hPricing
  have hNumericBoundPriced :
      selectedRustAcceptedSpendEvidenceReport.acceptedSpendNumericBoundPriced = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.pricingFactsComplete_numericBoundPriced
      selectedRustAcceptedSpendEvidenceReport hPricing
  rw [Gap.acceptedSpendNumericBoundPricedMissing] at hNumericBoundPriced
  cases hNumericBoundPriced

/-- The current selected accepted-spend report cannot complete the pricing half. -/
theorem selectedRustAcceptedSpendEvidenceReport_currentPricingFactsComplete_impossible :
    ¬ selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete := by
  exact
    selectedRustAcceptedSpendEvidenceReport_currentPricingGap_blocksPricingFactsComplete
      selectedRustAcceptedSpendEvidenceReport_currentPricingObligationGap

/--
For the active accepted-spend report, the only remaining report-completeness
input is the production pricing half; the structural evidence is already
selected and proved for AP2-N003.
-/
theorem selectedRustAcceptedSpendEvidenceReportFactsAllTrue_of_currentPricingFactsComplete
    (hPricing :
      selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete) :
    selectedRustAcceptedSpendEvidenceReport.FactsAllTrue :=
  SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_of_structural_and_pricing
    selectedRustAcceptedSpendEvidenceReport
    selectedRustAcceptedSpendEvidenceReport_currentStructuralFactsComplete
    hPricing

/--
For the active accepted-spend report, completing `FactsAllTrue` is exactly the
remaining production pricing half; the structural half is already selected.
-/
theorem selectedRustAcceptedSpendEvidenceReport_currentFactsAllTrue_iff_pricingFactsComplete :
    selectedRustAcceptedSpendEvidenceReport.FactsAllTrue ↔
      selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete := by
  constructor
  · intro hFacts
    exact
      (SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_iff_structural_and_pricing
        selectedRustAcceptedSpendEvidenceReport).1 hFacts |>.2
  · exact selectedRustAcceptedSpendEvidenceReportFactsAllTrue_of_currentPricingFactsComplete

/--
Selected Rust/model certificate for the accepted-spend numeric soundness layer.

The active full-signature theorem is only cryptographically meaningful after
the real selected verifier/model supplies the accepted-spend probability,
deployment bound, and proof of the inequality.  This certificate keeps that
numeric bound from being silently supplied as an arbitrary boundary field.
-/
structure SelectedRustAcceptedSpendSoundnessCertificate where
  evidenceReport : SelectedRustAcceptedSpendSoundnessEvidenceReport
  evidenceDigest : ByteArray
  evidenceReportBound :
    evidenceReport = selectedRustAcceptedSpendEvidenceReport
  evidenceReportBoundProof :
    evidenceReport = selectedRustAcceptedSpendEvidenceReport
  evidenceDigestBound :
    evidenceDigest = selectedRustAcceptedSpendEvidenceDigest
  evidenceDigestBoundProof :
    evidenceDigest = selectedRustAcceptedSpendEvidenceDigest
  selectedRustAcceptedSpendBoundVerifiedProof :
    evidenceReport.ReportVerified
  selectedRustAcceptedSpendFactsAllTrueProof :
    evidenceReport.FactsAllTrue
  numericSource :
    Path2AcceptedSpendProbability.SelectedAcceptedSpendNumericSource
  acceptedSpendForgeryProb : ℝ
  acceptedSpendForgeryBound : ℝ
  productionAcceptedSpendForgeryProb : ℝ
  productionAcceptedSpendForgeryBound : ℝ
  acceptedSpendForgeryProbMatchesUnionBound :
    acceptedSpendForgeryProb = numericSource.pricedCertificate.unionBound
  acceptedSpendForgeryBoundMatchesPriced :
    acceptedSpendForgeryBound = numericSource.pricedCertificate.pricedBound
  acceptedSpendPricedBoundSecurityPriced :
    numericSource.pricedCertificate.pricedBound <=
      ((2 : ℝ) ^ evidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹
  acceptedSpendForgeryProbPinned :
    acceptedSpendForgeryProb = productionAcceptedSpendForgeryProb
  acceptedSpendForgeryBoundPinned :
    acceptedSpendForgeryBound = productionAcceptedSpendForgeryBound
  hiddenSourceProofSemanticsClosed : Prop
  checkedCoreCompositionClosed : Prop
  lifecycleReplayClosed : Prop
  noExportClosed : Prop
  hiddenSourceProofSemanticsClosedProof :
    hiddenSourceProofSemanticsClosed
  checkedCoreCompositionClosedProof :
    checkedCoreCompositionClosed
  lifecycleReplayClosedProof :
    lifecycleReplayClosed
  noExportClosedProof :
    noExportClosed

/-- Named constructor for the selected Rust accepted-spend numeric certificate. -/
def SelectedRustAcceptedSpendSoundnessCertificate.ofReportAndBound
    (evidenceReport : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (evidenceDigest : ByteArray)
    (evidenceReportBound :
      evidenceReport = selectedRustAcceptedSpendEvidenceReport)
    (evidenceReportBoundProof :
      evidenceReport = selectedRustAcceptedSpendEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustAcceptedSpendEvidenceDigest)
    (evidenceDigestBoundProof :
      evidenceDigest = selectedRustAcceptedSpendEvidenceDigest)
    (selectedRustAcceptedSpendBoundVerifiedProof :
      evidenceReport.ReportVerified)
    (selectedRustAcceptedSpendFactsAllTrueProof :
      evidenceReport.FactsAllTrue)
    (numericSource :
      Path2AcceptedSpendProbability.SelectedAcceptedSpendNumericSource)
    (acceptedSpendForgeryProb : ℝ)
    (acceptedSpendForgeryBound : ℝ)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbMatchesUnionBound :
      acceptedSpendForgeryProb = numericSource.pricedCertificate.unionBound)
    (acceptedSpendForgeryBoundMatchesPriced :
      acceptedSpendForgeryBound = numericSource.pricedCertificate.pricedBound)
    (acceptedSpendPricedBoundSecurityPriced :
      numericSource.pricedCertificate.pricedBound <=
        ((2 : ℝ) ^ evidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹)
    (acceptedSpendForgeryProbPinned :
      acceptedSpendForgeryProb = productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      acceptedSpendForgeryBound = productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    SelectedRustAcceptedSpendSoundnessCertificate where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  evidenceReportBound := evidenceReportBound
  evidenceReportBoundProof := evidenceReportBoundProof
  evidenceDigestBound := evidenceDigestBound
  evidenceDigestBoundProof := evidenceDigestBoundProof
  selectedRustAcceptedSpendBoundVerifiedProof :=
    selectedRustAcceptedSpendBoundVerifiedProof
  selectedRustAcceptedSpendFactsAllTrueProof :=
    selectedRustAcceptedSpendFactsAllTrueProof
  numericSource := numericSource
  acceptedSpendForgeryProb := acceptedSpendForgeryProb
  acceptedSpendForgeryBound := acceptedSpendForgeryBound
  productionAcceptedSpendForgeryProb := productionAcceptedSpendForgeryProb
  productionAcceptedSpendForgeryBound := productionAcceptedSpendForgeryBound
  acceptedSpendForgeryProbMatchesUnionBound :=
    acceptedSpendForgeryProbMatchesUnionBound
  acceptedSpendForgeryBoundMatchesPriced := acceptedSpendForgeryBoundMatchesPriced
  acceptedSpendPricedBoundSecurityPriced :=
    acceptedSpendPricedBoundSecurityPriced
  acceptedSpendForgeryProbPinned := acceptedSpendForgeryProbPinned
  acceptedSpendForgeryBoundPinned := acceptedSpendForgeryBoundPinned
  hiddenSourceProofSemanticsClosed := hiddenSourceProofSemanticsClosed
  checkedCoreCompositionClosed := checkedCoreCompositionClosed
  lifecycleReplayClosed := lifecycleReplayClosed
  noExportClosed := noExportClosed
  hiddenSourceProofSemanticsClosedProof :=
    hiddenSourceProofSemanticsClosedProof
  checkedCoreCompositionClosedProof := checkedCoreCompositionClosedProof
  lifecycleReplayClosedProof := lifecycleReplayClosedProof
  noExportClosedProof := noExportClosedProof

/--
Active AP2-N003 constructor for the selected accepted-spend certificate.  The
selected report supplies verification and structural facts; the remaining hard
input is the production pricing half.
-/
def SelectedRustAcceptedSpendSoundnessCertificate.ofSelectedReportAndBoundPricingFacts
    (selectedRustAcceptedSpendPricingFactsCompleteProof :
      selectedRustAcceptedSpendEvidenceReport.PricingFactsComplete)
    (numericSource :
      Path2AcceptedSpendProbability.SelectedAcceptedSpendNumericSource)
    (acceptedSpendForgeryProb : ℝ)
    (acceptedSpendForgeryBound : ℝ)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbMatchesUnionBound :
      acceptedSpendForgeryProb = numericSource.pricedCertificate.unionBound)
    (acceptedSpendForgeryBoundMatchesPriced :
      acceptedSpendForgeryBound = numericSource.pricedCertificate.pricedBound)
    (acceptedSpendPricedBoundSecurityPriced :
      numericSource.pricedCertificate.pricedBound <=
        ((2 : ℝ) ^
          selectedRustAcceptedSpendEvidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹)
    (acceptedSpendForgeryProbPinned :
      acceptedSpendForgeryProb = productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      acceptedSpendForgeryBound = productionAcceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop)
    (hiddenSourceProofSemanticsClosedProof :
      hiddenSourceProofSemanticsClosed)
    (checkedCoreCompositionClosedProof :
      checkedCoreCompositionClosed)
    (lifecycleReplayClosedProof :
      lifecycleReplayClosed)
    (noExportClosedProof :
      noExportClosed) :
    SelectedRustAcceptedSpendSoundnessCertificate :=
  SelectedRustAcceptedSpendSoundnessCertificate.ofReportAndBound
    selectedRustAcceptedSpendEvidenceReport
    selectedRustAcceptedSpendEvidenceDigest
    rfl
    rfl
    rfl
    rfl
    selectedRustAcceptedSpendEvidenceReportVerifiedProof
    (selectedRustAcceptedSpendEvidenceReportFactsAllTrue_of_currentPricingFactsComplete
      selectedRustAcceptedSpendPricingFactsCompleteProof)
    numericSource
    acceptedSpendForgeryProb
    acceptedSpendForgeryBound
    productionAcceptedSpendForgeryProb
    productionAcceptedSpendForgeryBound
    acceptedSpendForgeryProbMatchesUnionBound
    acceptedSpendForgeryBoundMatchesPriced
    acceptedSpendPricedBoundSecurityPriced
    acceptedSpendForgeryProbPinned
    acceptedSpendForgeryBoundPinned
    hiddenSourceProofSemanticsClosed
    checkedCoreCompositionClosed
    lifecycleReplayClosed
    noExportClosed
    hiddenSourceProofSemanticsClosedProof
    checkedCoreCompositionClosedProof
    lifecycleReplayClosedProof
    noExportClosedProof

/-- No selected accepted-spend certificate exists for the current fail-closed report. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.currentSelectedReport_impossible
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) : False :=
  selectedRustAcceptedSpendEvidenceReportFactsAllTrue_impossible (by
    rw [← Cert.evidenceReportBoundProof]
    exact Cert.selectedRustAcceptedSpendFactsAllTrueProof)

/-- The selected accepted-spend certificate carries hidden-source closure. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.hiddenSourceProofSemanticsClosed_holds
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.hiddenSourceProofSemanticsClosed :=
  Cert.hiddenSourceProofSemanticsClosedProof

/-- The selected accepted-spend certificate carries checked-core closure. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.checkedCoreCompositionClosed_holds
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.checkedCoreCompositionClosed :=
  Cert.checkedCoreCompositionClosedProof

/-- The selected accepted-spend certificate carries lifecycle/replay closure. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.lifecycleReplayClosed_holds
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.lifecycleReplayClosed :=
  Cert.lifecycleReplayClosedProof

/-- The selected accepted-spend certificate carries no-export closure. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.noExportClosed_holds
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.noExportClosed :=
  Cert.noExportClosedProof

/-- The priced numeric certificate validated by the selected accepted-spend source. -/
def SelectedRustAcceptedSpendSoundnessCertificate.pricedNumericCertificate
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Path2AcceptedSpendProbability.PricedAcceptedSpendNumericCertificate :=
  Cert.numericSource.pricedCertificate

/-- The component budget carried by the selected accepted-spend certificate. -/
def SelectedRustAcceptedSpendSoundnessCertificate.componentBudget
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) : ℝ :=
  Cert.pricedNumericCertificate.acceptedSpendBound +
    (@Finset.univ Cert.pricedNumericCertificate.Break
      Cert.pricedNumericCertificate.breakFintype).sum
        Cert.pricedNumericCertificate.breakBound

/-- The exported accepted-spend probability is the selected numeric source's union bound. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryProb_eq_unionBound
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryProb =
      Cert.pricedNumericCertificate.unionBound :=
  Cert.acceptedSpendForgeryProbMatchesUnionBound

/-- The exported accepted-spend bound is the selected numeric source's priced bound. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryBound_eq_pricedBound
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryBound =
      Cert.pricedNumericCertificate.pricedBound :=
  Cert.acceptedSpendForgeryBoundMatchesPriced

/-- The exported accepted-spend probability is pinned to the production value. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryProb_eq_production
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryProb =
      Cert.productionAcceptedSpendForgeryProb :=
  Cert.acceptedSpendForgeryProbPinned

/-- The exported accepted-spend bound is pinned to the production value. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryBound_eq_production
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryBound =
      Cert.productionAcceptedSpendForgeryBound :=
  Cert.acceptedSpendForgeryBoundPinned

/-- The selected accepted-spend certificate proves its exported probability bound. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryProb_le_bound
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryProb <= Cert.acceptedSpendForgeryBound := by
  let _hVerified : Cert.evidenceReport.ReportVerified :=
    Cert.selectedRustAcceptedSpendBoundVerifiedProof
  let _hFacts : Cert.evidenceReport.FactsAllTrue :=
    Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hSchemaMatches :
      Cert.evidenceReport.schemaMatches = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_schemaMatches
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendBoundVerifiedProof
  let _hReportDigestVerified :
      Cert.evidenceReport.reportDigestVerified = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_reportDigestVerified
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendBoundVerifiedProof
  let _hNumericEvidenceDigestsVerified :
      Cert.evidenceReport.numericEvidenceDigestsVerified = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_numericEvidenceDigestsVerified
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendBoundVerifiedProof
  let _hReportBound :
      Cert.evidenceReport = selectedRustAcceptedSpendEvidenceReport :=
    Cert.evidenceReportBoundProof
  let _hDigestBound :
      Cert.evidenceDigest = selectedRustAcceptedSpendEvidenceDigest :=
    Cert.evidenceDigestBoundProof
  let _hPricedBoundBitsMeetTarget :
      Cert.evidenceReport.targetSecurityBits ≤
        Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityMeetsTarget
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hActiveBudgetMarginPriced :
      Cert.evidenceReport.targetSecurityBits +
        Cert.evidenceReport.selectedMultiTargetHorizonLog2 ≤
          Cert.evidenceReport.transcriptTermBitsAfterQromLoss :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeBudgetMarginPriced
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hActiveBudgetSlack :
      Cert.evidenceReport.transcriptTermBitsAfterQromLoss -
        (Cert.evidenceReport.targetSecurityBits +
          Cert.evidenceReport.selectedMultiTargetHorizonLog2) = 22 :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeBudgetSlackBits
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hClaimedBoundWithinTranscript :
      Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits +
        Cert.evidenceReport.selectedMultiTargetHorizonLog2 ≤
          Cert.evidenceReport.transcriptTermBitsAfterQromLoss :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityWithinTranscript
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hClaimedBoundWithinTranscriptCap :
      Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits ≤ 150 :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityBits_le_transcriptCap
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hNumericBoundPriced :
      Cert.evidenceReport.acceptedSpendNumericBoundPriced = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_numericBoundPriced
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hFiniteTraceSupportPriced :
      Cert.evidenceReport.finiteTraceSupportPriced = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_finiteTraceSupportPriced
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hActiveOutcomeCarrierPinned :
      Cert.evidenceReport.activeOutcomeCarrierPinned = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeOutcomeCarrierPinned
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hAcceptedSpendEventPinned :
      Cert.evidenceReport.acceptedSpendEventPinned = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_acceptedSpendEventPinned
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hActiveBreakFamilyPinned :
      Cert.evidenceReport.activeBreakFamilyPinned = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeBreakFamilyPinned
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hNamedBreakEventsPinned :
      Cert.evidenceReport.namedBreakEventsPinned = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_namedBreakEventsPinned
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hUnionBoundPriced :
      Cert.evidenceReport.acceptedSpendUnionBoundPriced = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_unionBoundPriced
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hAcceptedSpendEventBoundPriced :
      Cert.evidenceReport.acceptedSpendEventBoundPriced = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_acceptedSpendEventBoundPriced
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hNamedBreakEventBoundsPriced :
      Cert.evidenceReport.namedBreakEventBoundsPriced = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_namedBreakEventBoundsPriced
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hBreakPricingBudgetMeetsPricedBound :
      Cert.evidenceReport.breakPricingBudgetMeetsPricedBound = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_breakPricingBudgetMeetsPricedBound
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hNumericBudgetMeetsTarget :
      Cert.evidenceReport.acceptedSpendNumericBudgetMeetsTarget = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_numericBudgetMeetsTarget
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hQromQuadraticLossBits :
      Cert.evidenceReport.qromQuadraticLossBitsAtDeploymentBound = 32 :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_qromQuadraticLossBits
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hHiddenSourceClosureReady :
      Cert.evidenceReport.hiddenSourceClosureReady = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_hiddenSourceClosureReady
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hCheckedCoreClosureReady :
      Cert.evidenceReport.checkedCoreClosureReady = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_checkedCoreClosureReady
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hLifecycleReplayClosureReady :
      Cert.evidenceReport.lifecycleReplayClosureReady = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_lifecycleReplayClosureReady
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hNoExportClosureReady :
      Cert.evidenceReport.noExportClosureReady = true :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_noExportClosureReady
        Cert.evidenceReport
        Cert.selectedRustAcceptedSpendFactsAllTrueProof
  let _hPricedBoundSecurity :
      Cert.pricedNumericCertificate.pricedBound <=
        ((2 : ℝ) ^
          Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹ :=
    Cert.acceptedSpendPricedBoundSecurityPriced
  let _hProbPinned :
      Cert.acceptedSpendForgeryProb =
        Cert.productionAcceptedSpendForgeryProb :=
    Cert.acceptedSpendForgeryProbPinned
  let _hBoundPinned :
      Cert.acceptedSpendForgeryBound =
        Cert.productionAcceptedSpendForgeryBound :=
    Cert.acceptedSpendForgeryBoundPinned
  calc
    Cert.acceptedSpendForgeryProb =
        Cert.pricedNumericCertificate.unionBound :=
      Cert.acceptedSpendForgeryProb_eq_unionBound
    _ <= Cert.pricedNumericCertificate.pricedBound :=
      Cert.numericSource.unionBound_le_priced
    _ = Cert.acceptedSpendForgeryBound :=
      Cert.acceptedSpendForgeryBound_eq_pricedBound.symm

/-- The selected accepted-spend certificate exposes its union-to-component-budget route. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryProb_le_componentBudget
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryProb <= Cert.componentBudget := by
  calc
    Cert.acceptedSpendForgeryProb =
        Cert.pricedNumericCertificate.unionBound :=
      Cert.acceptedSpendForgeryProb_eq_unionBound
    _ <= Cert.componentBudget := by
      simpa [SelectedRustAcceptedSpendSoundnessCertificate.componentBudget] using
        Cert.numericSource.unionBound_le_componentBudget

/-- The selected accepted-spend certificate exposes its component budget below the exported bound. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.componentBudget_le_acceptedSpendForgeryBound
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.componentBudget <= Cert.acceptedSpendForgeryBound := by
  calc
    Cert.componentBudget <= Cert.pricedNumericCertificate.pricedBound := by
      simpa [SelectedRustAcceptedSpendSoundnessCertificate.componentBudget] using
        Cert.numericSource.breakBudget_le_pricedBound
    _ = Cert.acceptedSpendForgeryBound :=
      Cert.acceptedSpendForgeryBound_eq_pricedBound.symm

/-- The selected accepted-spend certificate bounds packaged success by the component budget. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.successProb_le_componentBudget
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.pricedNumericCertificate.successProb <= Cert.componentBudget := by
  simpa [SelectedRustAcceptedSpendSoundnessCertificate.componentBudget] using
    Cert.numericSource.successProb_le_componentBudget

/-- The selected accepted-spend certificate exports a nonnegative probability. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryProb_nonneg
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    0 <= Cert.acceptedSpendForgeryProb := by
  calc
    0 <= Cert.pricedNumericCertificate.unionBound :=
      Cert.pricedNumericCertificate.unionBound_nonneg
    _ = Cert.acceptedSpendForgeryProb :=
      Cert.acceptedSpendForgeryProb_eq_unionBound.symm

/-- The selected accepted-spend certificate exports a nonnegative bound. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryBound_nonneg
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    0 <= Cert.acceptedSpendForgeryBound := by
  calc
    0 <= Cert.pricedNumericCertificate.pricedBound :=
      Cert.pricedNumericCertificate.pricedBound_nonneg
    _ = Cert.acceptedSpendForgeryBound :=
      Cert.acceptedSpendForgeryBound_eq_pricedBound.symm

/-- The selected accepted-spend certificate prices its exported bound by reported security bits. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryBound_le_reportedSecurity
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryBound <=
      ((2 : ℝ) ^ Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹ := by
  calc
    Cert.acceptedSpendForgeryBound =
        Cert.pricedNumericCertificate.pricedBound :=
      Cert.acceptedSpendForgeryBound_eq_pricedBound
    _ <= ((2 : ℝ) ^
          Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹ :=
      Cert.acceptedSpendPricedBoundSecurityPriced

/-- The selected accepted-spend certificate prices accepted-spend probability by reported security bits. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryProb_le_reportedSecurity
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryProb <=
      ((2 : ℝ) ^ Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹ :=
  le_trans
    Cert.acceptedSpendForgeryProb_le_bound
    Cert.acceptedSpendForgeryBound_le_reportedSecurity

/-- The selected accepted-spend certificate prices its bound by the target security bits. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryBound_le_targetSecurity
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryBound <=
      ((2 : ℝ) ^ Cert.evidenceReport.targetSecurityBits)⁻¹ :=
  le_trans
    Cert.acceptedSpendForgeryBound_le_reportedSecurity
    (SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_reportedSecurityBound_le_targetSecurityBound
      Cert.evidenceReport
      Cert.selectedRustAcceptedSpendFactsAllTrueProof)

/-- The active selected accepted-spend certificate prices its bound by the fixed 128-bit target. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryBound_le_active128BitTarget
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryBound <= ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  have hTarget :
      Cert.evidenceReport.targetSecurityBits = 128 :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_targetSecurityBits
      Cert.evidenceReport
      Cert.selectedRustAcceptedSpendFactsAllTrueProof
  simpa [hTarget] using Cert.acceptedSpendForgeryBound_le_targetSecurity

/-- The active selected accepted-spend certificate prices its component budget by the fixed target. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.componentBudget_le_active128BitTarget
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.componentBudget <= ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  le_trans
    Cert.componentBudget_le_acceptedSpendForgeryBound
    Cert.acceptedSpendForgeryBound_le_active128BitTarget

/-- The selected accepted-spend certificate prices accepted-spend probability by target bits. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryProb_le_targetSecurity
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryProb <=
      ((2 : ℝ) ^ Cert.evidenceReport.targetSecurityBits)⁻¹ :=
  le_trans
    Cert.acceptedSpendForgeryProb_le_reportedSecurity
    (SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_reportedSecurityBound_le_targetSecurityBound
      Cert.evidenceReport
      Cert.selectedRustAcceptedSpendFactsAllTrueProof)

/-- The active selected accepted-spend certificate reaches the fixed 128-bit target. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryProb_le_active128BitTarget
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.acceptedSpendForgeryProb <= ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  have hTarget :
      Cert.evidenceReport.targetSecurityBits = 128 :=
    SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_targetSecurityBits
      Cert.evidenceReport
      Cert.selectedRustAcceptedSpendFactsAllTrueProof
  simpa [hTarget] using Cert.acceptedSpendForgeryProb_le_targetSecurity

/-- The active selected accepted-spend certificate prices packaged success by the fixed target. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.successProb_le_active128BitTarget
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.pricedNumericCertificate.successProb <= ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  le_trans
    Cert.successProb_le_componentBudget
    Cert.componentBudget_le_active128BitTarget

/-- Convert selected Rust accepted-spend numeric evidence into the boundary. -/
def SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    NativeAcceptedSpendSoundnessBoundary where
  acceptedSpendForgeryProb :=
    Cert.acceptedSpendForgeryProb
  acceptedSpendForgeryBound :=
    Cert.acceptedSpendForgeryBound
  productionAcceptedSpendForgeryProb :=
    Cert.productionAcceptedSpendForgeryProb
  productionAcceptedSpendForgeryBound :=
    Cert.productionAcceptedSpendForgeryBound
  acceptedSpendForgeryProbPinned :=
    Cert.acceptedSpendForgeryProbPinned
  acceptedSpendForgeryBoundPinned :=
    Cert.acceptedSpendForgeryBoundPinned
  acceptedSpendForgery_le_bound := by
    let _hVerified : Cert.evidenceReport.ReportVerified :=
      Cert.selectedRustAcceptedSpendBoundVerifiedProof
    let _hFacts : Cert.evidenceReport.FactsAllTrue :=
      Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hSchemaMatches :
        Cert.evidenceReport.schemaMatches = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_schemaMatches
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendBoundVerifiedProof
    let _hReportDigestVerified :
        Cert.evidenceReport.reportDigestVerified = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_reportDigestVerified
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendBoundVerifiedProof
    let _hNumericEvidenceDigestsVerified :
        Cert.evidenceReport.numericEvidenceDigestsVerified = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.reportVerified_numericEvidenceDigestsVerified
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendBoundVerifiedProof
    let _hReportBound :
        Cert.evidenceReport = selectedRustAcceptedSpendEvidenceReport :=
      Cert.evidenceReportBoundProof
    let _hDigestBound :
        Cert.evidenceDigest = selectedRustAcceptedSpendEvidenceDigest :=
      Cert.evidenceDigestBoundProof
    let _hPricedBoundBitsMeetTarget :
        Cert.evidenceReport.targetSecurityBits ≤
          Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityMeetsTarget
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hActiveBudgetMarginPriced :
        Cert.evidenceReport.targetSecurityBits +
          Cert.evidenceReport.selectedMultiTargetHorizonLog2 ≤
            Cert.evidenceReport.transcriptTermBitsAfterQromLoss :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeBudgetMarginPriced
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hActiveBudgetSlack :
        Cert.evidenceReport.transcriptTermBitsAfterQromLoss -
          (Cert.evidenceReport.targetSecurityBits +
            Cert.evidenceReport.selectedMultiTargetHorizonLog2) = 22 :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeBudgetSlackBits
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hClaimedBoundWithinTranscript :
        Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits +
          Cert.evidenceReport.selectedMultiTargetHorizonLog2 ≤
            Cert.evidenceReport.transcriptTermBitsAfterQromLoss :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityWithinTranscript
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hClaimedBoundWithinTranscriptCap :
        Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits ≤ 150 :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityBits_le_transcriptCap
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hNumericBoundPriced :
        Cert.evidenceReport.acceptedSpendNumericBoundPriced = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_numericBoundPriced
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hFiniteTraceSupportPriced :
        Cert.evidenceReport.finiteTraceSupportPriced = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_finiteTraceSupportPriced
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hActiveOutcomeCarrierPinned :
        Cert.evidenceReport.activeOutcomeCarrierPinned = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeOutcomeCarrierPinned
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hAcceptedSpendEventPinned :
        Cert.evidenceReport.acceptedSpendEventPinned = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_acceptedSpendEventPinned
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hActiveBreakFamilyPinned :
        Cert.evidenceReport.activeBreakFamilyPinned = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_activeBreakFamilyPinned
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hNamedBreakEventsPinned :
        Cert.evidenceReport.namedBreakEventsPinned = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_namedBreakEventsPinned
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hUnionBoundPriced :
        Cert.evidenceReport.acceptedSpendUnionBoundPriced = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_unionBoundPriced
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hAcceptedSpendEventBoundPriced :
        Cert.evidenceReport.acceptedSpendEventBoundPriced = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_acceptedSpendEventBoundPriced
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hNamedBreakEventBoundsPriced :
        Cert.evidenceReport.namedBreakEventBoundsPriced = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_namedBreakEventBoundsPriced
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hBreakPricingBudgetMeetsPricedBound :
        Cert.evidenceReport.breakPricingBudgetMeetsPricedBound = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_breakPricingBudgetMeetsPricedBound
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hNumericBudgetMeetsTarget :
        Cert.evidenceReport.acceptedSpendNumericBudgetMeetsTarget = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_numericBudgetMeetsTarget
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hQromQuadraticLossBits :
        Cert.evidenceReport.qromQuadraticLossBitsAtDeploymentBound = 32 :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_qromQuadraticLossBits
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hHiddenSourceClosureReady :
        Cert.evidenceReport.hiddenSourceClosureReady = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_hiddenSourceClosureReady
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hCheckedCoreClosureReady :
        Cert.evidenceReport.checkedCoreClosureReady = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_checkedCoreClosureReady
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hLifecycleReplayClosureReady :
        Cert.evidenceReport.lifecycleReplayClosureReady = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_lifecycleReplayClosureReady
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hNoExportClosureReady :
        Cert.evidenceReport.noExportClosureReady = true :=
      SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_noExportClosureReady
          Cert.evidenceReport
          Cert.selectedRustAcceptedSpendFactsAllTrueProof
    let _hPricedBoundSecurity :
        Cert.pricedNumericCertificate.pricedBound <=
          ((2 : ℝ) ^
            Cert.evidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹ :=
      Cert.acceptedSpendPricedBoundSecurityPriced
    let _hProbPinned :
        Cert.acceptedSpendForgeryProb =
          Cert.productionAcceptedSpendForgeryProb :=
      Cert.acceptedSpendForgeryProbPinned
    let _hBoundPinned :
        Cert.acceptedSpendForgeryBound =
          Cert.productionAcceptedSpendForgeryBound :=
      Cert.acceptedSpendForgeryBoundPinned
    exact Cert.acceptedSpendForgeryProb_le_bound
  hiddenSourceProofSemanticsClosed :=
    Cert.hiddenSourceProofSemanticsClosed
  checkedCoreCompositionClosed :=
    Cert.checkedCoreCompositionClosed
  lifecycleReplayClosed :=
    Cert.lifecycleReplayClosed
  noExportClosed :=
    Cert.noExportClosed

/-- The erased accepted-spend boundary from the selected certificate still reaches the fixed target. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryProb_le_active128BitTarget
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.toAcceptedSpendSoundnessBoundary.acceptedSpendForgeryProb <=
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Cert.acceptedSpendForgeryProb_le_active128BitTarget

/-- The erased accepted-spend boundary from the selected certificate keeps the exported bound at the fixed target. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_le_active128BitTarget
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.toAcceptedSpendSoundnessBoundary.acceptedSpendForgeryBound <=
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Cert.acceptedSpendForgeryBound_le_active128BitTarget

/-- The erased accepted-spend boundary keeps the exported probability nonnegative. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryProb_nonneg
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    0 <= Cert.toAcceptedSpendSoundnessBoundary.acceptedSpendForgeryProb :=
  Cert.acceptedSpendForgeryProb_nonneg

/-- The erased accepted-spend boundary keeps the exported bound nonnegative. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_nonneg
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    0 <= Cert.toAcceptedSpendSoundnessBoundary.acceptedSpendForgeryBound :=
  Cert.acceptedSpendForgeryBound_nonneg

/-- The selected accepted-spend certificate bounds packaged success by the exported accepted-spend probability. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.successProb_le_acceptedSpendForgeryProb
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.pricedNumericCertificate.successProb <=
      Cert.acceptedSpendForgeryProb := by
  calc
    Cert.pricedNumericCertificate.successProb <=
        Cert.pricedNumericCertificate.unionBound :=
      Cert.numericSource.successProb_le_unionBound
    _ = Cert.acceptedSpendForgeryProb :=
      Cert.acceptedSpendForgeryProbMatchesUnionBound.symm

/-- The selected accepted-spend certificate keeps packaged success probability at most one. -/
theorem SelectedRustAcceptedSpendSoundnessCertificate.successProb_le_one
    (Cert : SelectedRustAcceptedSpendSoundnessCertificate) :
    Cert.pricedNumericCertificate.successProb <= 1 :=
  Cert.pricedNumericCertificate.successProb_le_one

/--
Full-signature boundary for the currently selected Path 2 native/Fq12 route.

Constructing this record is the active deployable-signature proof target.  It
can only prove a final full-signature bound after the production keygen,
signing oracle, verifier implementation bridge, hash budget, and full-forgery
to accepted-spend reduction have been supplied for the Path 2 native verifier.
-/
structure NativeFullSignatureSecurityBoundary where
  acceptedSpendSoundness : NativeAcceptedSpendSoundnessBoundary
  hashBoundary : NativeHashBudgetBoundary
  keygenDistributionModeled : Prop
  signingOracleModeled : Prop
  completenessModeled : Prop
  verifierRelationMatchesImplementation : Prop
  fullForgeryImpliesAcceptedSpendForgery : Prop
  signatureForgerySuccessProb : ℝ
  signatureForgerySuccess_le_acceptedSpend :
    signatureForgerySuccessProb <=
      acceptedSpendSoundness.acceptedSpendForgeryProb
  fullSignatureSecurityClaimComplete :
    keygenDistributionModeled ∧
      signingOracleModeled ∧
      completenessModeled ∧
      verifierRelationMatchesImplementation ∧
      fullForgeryImpliesAcceptedSpendForgery ∧
      acceptedSpendSoundness.hiddenSourceProofSemanticsClosed ∧
      acceptedSpendSoundness.checkedCoreCompositionClosed ∧
      acceptedSpendSoundness.lifecycleReplayClosed ∧
      acceptedSpendSoundness.noExportClosed ∧
      hashBoundary.checkedCoreFrameDomainSeparated ∧
      hashBoundary.hiddenSourceFrameDomainSeparated ∧
      hashBoundary.queryBudgetPriced ∧
      hashBoundary.multiTargetLossPriced

/--
Lean mirror of the selected Rust full-signature reduction evidence report.

The final deployable theorem must bind the signature-forgery probability,
accepted-spend reduction inequality, and active hash/QROM budget to reviewed
production evidence before constructing `NativeFullSignatureSecurityBoundary`.
-/
structure SelectedRustFullSignatureReductionEvidenceReport where
  schemaMatches : Bool
  reportDigestVerified : Bool
  reductionEvidenceDigestsVerified : Bool
  productionSignatureProbabilityDigest : ByteArray
  signatureAcceptedSpendLinkDigest : ByteArray
  fullExecutionSourceDigest : ByteArray
  activeHashBoundaryDigest : ByteArray
  queryBudgetPricingDigest : ByteArray
  multiTargetPricingDigest : ByteArray
  verifierGameBindingDigest : ByteArray
  productionSignatureProbabilityPinned : Bool
  signatureSuccessMatchesAcceptedSpendGame : Bool
  acceptedSpendProbabilityLinked : Bool
  fullExecutionSourceConsumed : Bool
  activeHashBoundaryConsumed : Bool
  allActiveQuerySymbolsPriced : Bool
  multiTargetHorizonPriced : Bool
  verifierGameMatchesProductionExecution : Bool

/-- The full-signature reduction report has the expected schema and digest. -/
def SelectedRustFullSignatureReductionEvidenceReport.ReportVerified
    (E : SelectedRustFullSignatureReductionEvidenceReport) : Prop :=
  E.schemaMatches = true ∧
  E.reportDigestVerified = true ∧
  E.reductionEvidenceDigestsVerified = true ∧
  E.productionSignatureProbabilityDigest ≠ selectedRustZeroDigest32 ∧
  E.signatureAcceptedSpendLinkDigest ≠ selectedRustZeroDigest32 ∧
  E.fullExecutionSourceDigest ≠ selectedRustZeroDigest32 ∧
  E.activeHashBoundaryDigest ≠ selectedRustZeroDigest32 ∧
  E.queryBudgetPricingDigest ≠ selectedRustZeroDigest32 ∧
  E.multiTargetPricingDigest ≠ selectedRustZeroDigest32 ∧
  E.verifierGameBindingDigest ≠ selectedRustZeroDigest32

/-- A verified full-signature reduction report has the selected schema. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.reportVerified_schemaMatches
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ReportVerified) :
    E.schemaMatches = true := by
  exact h.1

/-- A verified full-signature reduction report has its report digest checked. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.reportVerified_reportDigestVerified
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ReportVerified) :
    E.reportDigestVerified = true := by
  exact h.2.1

/-- A verified full-signature reduction report has its evidence digests checked. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.reportVerified_reductionEvidenceDigestsVerified
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ReportVerified) :
    E.reductionEvidenceDigestsVerified = true := by
  exact h.2.2.1

/-- A verified full-signature reduction report has a nonzero production-probability digest. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.reportVerified_productionSignatureProbabilityDigest_nonzero
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ReportVerified) :
    E.productionSignatureProbabilityDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, hProbability, _, _, _, _, _, _⟩
  exact hProbability

/-- A verified full-signature reduction report has a nonzero accepted-spend-link digest. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.reportVerified_signatureAcceptedSpendLinkDigest_nonzero
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ReportVerified) :
    E.signatureAcceptedSpendLinkDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, hLink, _, _, _, _, _⟩
  exact hLink

/-- A verified full-signature reduction report has a nonzero execution-source digest. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.reportVerified_fullExecutionSourceDigest_nonzero
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ReportVerified) :
    E.fullExecutionSourceDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, _, hExecution, _, _, _, _⟩
  exact hExecution

/-- A verified full-signature reduction report has a nonzero hash-boundary digest. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.reportVerified_activeHashBoundaryDigest_nonzero
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ReportVerified) :
    E.activeHashBoundaryDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, _, _, hHash, _, _, _⟩
  exact hHash

/-- A verified full-signature reduction report has a nonzero query-pricing digest. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.reportVerified_queryBudgetPricingDigest_nonzero
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ReportVerified) :
    E.queryBudgetPricingDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, _, _, _, hQuery, _, _⟩
  exact hQuery

/-- A verified full-signature reduction report has a nonzero multi-target digest. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.reportVerified_multiTargetPricingDigest_nonzero
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ReportVerified) :
    E.multiTargetPricingDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, _, _, _, _, hMultiTarget, _⟩
  exact hMultiTarget

/-- A verified full-signature reduction report has a nonzero verifier-game digest. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.reportVerified_verifierGameBindingDigest_nonzero
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ReportVerified) :
    E.verifierGameBindingDigest ≠ selectedRustZeroDigest32 := by
  rcases h with ⟨_, _, _, _, _, _, _, _, _, hVerifierGame⟩
  exact hVerifierGame

/-- Every production full-signature reduction fact recorded by the report holds. -/
def SelectedRustFullSignatureReductionEvidenceReport.FactsAllTrue
    (E : SelectedRustFullSignatureReductionEvidenceReport) : Prop :=
  E.productionSignatureProbabilityPinned = true ∧
  E.signatureSuccessMatchesAcceptedSpendGame = true ∧
  E.acceptedSpendProbabilityLinked = true ∧
  E.fullExecutionSourceConsumed = true ∧
  E.activeHashBoundaryConsumed = true ∧
  E.allActiveQuerySymbolsPriced = true ∧
  E.multiTargetHorizonPriced = true ∧
  E.verifierGameMatchesProductionExecution = true

/-- Full-reduction structural evidence already selected from active sources. -/
def SelectedRustFullSignatureReductionEvidenceReport.StructuralFactsComplete
    (E : SelectedRustFullSignatureReductionEvidenceReport) : Prop :=
  E.fullExecutionSourceConsumed = true ∧
  E.activeHashBoundaryConsumed = true ∧
  E.multiTargetHorizonPriced = true

/-- Full-reduction facts that still require the production model/reduction theorem. -/
def SelectedRustFullSignatureReductionEvidenceReport.ProductionFactsComplete
    (E : SelectedRustFullSignatureReductionEvidenceReport) : Prop :=
  E.productionSignatureProbabilityPinned = true ∧
  E.signatureSuccessMatchesAcceptedSpendGame = true ∧
  E.acceptedSpendProbabilityLinked = true ∧
  E.allActiveQuerySymbolsPriced = true ∧
  E.verifierGameMatchesProductionExecution = true

/-- Full-reduction completeness is exactly structural evidence plus production facts. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_iff_structural_and_production
    (E : SelectedRustFullSignatureReductionEvidenceReport) :
    E.FactsAllTrue ↔
      E.StructuralFactsComplete ∧ E.ProductionFactsComplete := by
  constructor
  · intro h
    rcases h with
      ⟨hProbability, hAcceptedGame, hAcceptedLinked, hExecution, hHash,
        hQuery, hMultiTarget, hVerifierGame⟩
    exact
      ⟨⟨hExecution, hHash, hMultiTarget⟩,
        ⟨hProbability, hAcceptedGame, hAcceptedLinked, hQuery,
          hVerifierGame⟩⟩
  · intro h
    rcases h with
      ⟨⟨hExecution, hHash, hMultiTarget⟩,
        ⟨hProbability, hAcceptedGame, hAcceptedLinked, hQuery,
          hVerifierGame⟩⟩
    exact
      ⟨hProbability, hAcceptedGame, hAcceptedLinked, hExecution, hHash,
        hQuery, hMultiTarget, hVerifierGame⟩

/-- Assemble full-reduction report completeness from the structural and production halves. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_of_structural_and_production
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (hStructural : E.StructuralFactsComplete)
    (hProduction : E.ProductionFactsComplete) :
    E.FactsAllTrue :=
  (SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_iff_structural_and_production E).2
    ⟨hStructural, hProduction⟩

/-- A complete production half pins the production signature probability. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.productionFactsComplete_productionSignatureProbabilityPinned
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ProductionFactsComplete) :
    E.productionSignatureProbabilityPinned = true :=
  h.1

/-- A complete production half links signature success to the accepted-spend game. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.productionFactsComplete_signatureSuccessMatchesAcceptedSpendGame
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ProductionFactsComplete) :
    E.signatureSuccessMatchesAcceptedSpendGame = true :=
  h.2.1

/-- A complete production half links accepted-spend probability. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.productionFactsComplete_acceptedSpendProbabilityLinked
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ProductionFactsComplete) :
    E.acceptedSpendProbabilityLinked = true :=
  h.2.2.1

/-- A complete production half prices every active final query symbol. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.productionFactsComplete_allActiveQuerySymbolsPriced
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ProductionFactsComplete) :
    E.allActiveQuerySymbolsPriced = true :=
  h.2.2.2.1

/-- A complete production half binds the verifier game to production execution. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.productionFactsComplete_verifierGameMatchesProductionExecution
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.ProductionFactsComplete) :
    E.verifierGameMatchesProductionExecution = true :=
  h.2.2.2.2

/-- A complete full-reduction report pins the production signature probability. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_productionSignatureProbabilityPinned
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.FactsAllTrue) :
    E.productionSignatureProbabilityPinned = true := by
  exact h.1

/-- A complete full-reduction report links signature success to the accepted-spend game. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_signatureSuccessMatchesAcceptedSpendGame
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.FactsAllTrue) :
    E.signatureSuccessMatchesAcceptedSpendGame = true := by
  exact h.2.1

/-- A complete full-reduction report links accepted-spend probability. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_acceptedSpendProbabilityLinked
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.FactsAllTrue) :
    E.acceptedSpendProbabilityLinked = true := by
  exact h.2.2.1

/-- A complete full-reduction report consumes the concrete full-execution source. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_fullExecutionSourceConsumed
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.FactsAllTrue) :
    E.fullExecutionSourceConsumed = true := by
  exact h.2.2.2.1

/-- A complete full-reduction report consumes the active hash boundary. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_activeHashBoundaryConsumed
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.FactsAllTrue) :
    E.activeHashBoundaryConsumed = true := by
  exact h.2.2.2.2.1

/-- A complete full-reduction report prices every active query symbol. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_allActiveQuerySymbolsPriced
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.FactsAllTrue) :
    E.allActiveQuerySymbolsPriced = true := by
  exact h.2.2.2.2.2.1

/-- A complete full-reduction report prices the active multi-target horizon. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_multiTargetHorizonPriced
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.FactsAllTrue) :
    E.multiTargetHorizonPriced = true := by
  exact h.2.2.2.2.2.2.1

/-- A complete full-reduction report binds the verifier game to production execution. -/
theorem SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_verifierGameMatchesProductionExecution
    (E : SelectedRustFullSignatureReductionEvidenceReport)
    (h : E.FactsAllTrue) :
    E.verifierGameMatchesProductionExecution = true := by
  exact h.2.2.2.2.2.2.2

/-- Selected Rust full-signature reduction evidence report mirrored from Rust. -/
def selectedRustFullSignatureReductionEvidenceReport :
    SelectedRustFullSignatureReductionEvidenceReport where
  schemaMatches := true
  reportDigestVerified := true
  reductionEvidenceDigestsVerified := true
  productionSignatureProbabilityDigest :=
    ByteArray.mk #[114, 146, 77, 165, 33, 44, 170, 41, 250, 236, 206, 234,
      206, 157, 223, 86, 31, 15, 198, 160, 150, 219, 237, 210, 63, 213,
      129, 35, 213, 140, 1, 116]
  signatureAcceptedSpendLinkDigest :=
    ByteArray.mk #[252, 138, 172, 20, 49, 172, 16, 33, 64, 39, 154, 202,
      217, 151, 221, 248, 175, 192, 246, 131, 12, 234, 239, 28, 121, 62,
      222, 206, 65, 175, 240, 71]
  fullExecutionSourceDigest :=
    ByteArray.mk #[255, 181, 157, 198, 240, 129, 76, 113, 9, 35, 168, 216,
      204, 48, 31, 224, 238, 132, 237, 162, 151, 134, 201, 91, 85, 7, 117,
      202, 68, 28, 244, 206]
  activeHashBoundaryDigest :=
    ByteArray.mk #[200, 31, 58, 59, 106, 52, 143, 78, 67, 6, 238, 92, 210,
      79, 245, 60, 179, 190, 228, 249, 236, 166, 174, 245, 192, 185, 107,
      85, 200, 81, 100, 237]
  queryBudgetPricingDigest :=
    ByteArray.mk #[233, 21, 108, 53, 33, 254, 79, 99, 213, 214, 194, 144,
      224, 47, 136, 20, 99, 117, 203, 37, 102, 235, 60, 203, 106, 71, 197,
      88, 221, 87, 35, 87]
  multiTargetPricingDigest :=
    ByteArray.mk #[24, 85, 71, 199, 201, 152, 191, 90, 105, 103, 38, 92,
      211, 167, 99, 194, 113, 134, 176, 46, 139, 245, 67, 45, 191, 5, 96,
      105, 186, 237, 13, 142]
  verifierGameBindingDigest :=
    ByteArray.mk #[129, 23, 122, 9, 55, 148, 45, 226, 53, 192, 28, 253,
      247, 168, 27, 252, 28, 141, 61, 68, 57, 195, 236, 209, 241, 245, 134,
      178, 226, 155, 117, 233]
  productionSignatureProbabilityPinned := false
  signatureSuccessMatchesAcceptedSpendGame := false
  acceptedSpendProbabilityLinked := false
  fullExecutionSourceConsumed := true
  activeHashBoundaryConsumed := true
  allActiveQuerySymbolsPriced := false
  multiTargetHorizonPriced := true
  verifierGameMatchesProductionExecution := true

/-- Digest pin for the selected Rust full-signature reduction evidence report. -/
def selectedRustFullSignatureReductionEvidenceDigest : ByteArray :=
  ByteArray.mk #[65, 129, 52, 192, 126, 122, 172, 232, 192, 29, 252, 16,
    28, 17, 209, 81, 101, 244, 26, 145, 214, 144, 191, 46, 180, 14, 170,
    81, 16, 97, 161, 239]

/-- The selected full-reduction report has verified public digest slots. -/
theorem selectedRustFullSignatureReductionVerifiedProof :
    selectedRustFullSignatureReductionEvidenceReport.ReportVerified := by
  simp [SelectedRustFullSignatureReductionEvidenceReport.ReportVerified,
    selectedRustFullSignatureReductionEvidenceReport, selectedRustZeroDigest32]
  repeat constructor <;> native_decide

/-- The current selected full-reduction report is intentionally not proof-complete. -/
theorem selectedRustFullSignatureReductionFactsAllTrue_impossible :
    ¬ selectedRustFullSignatureReductionEvidenceReport.FactsAllTrue := by
  simp [SelectedRustFullSignatureReductionEvidenceReport.FactsAllTrue,
    selectedRustFullSignatureReductionEvidenceReport]

/-- The current selected full-reduction report carries existing structural consumption facts. -/
theorem selectedRustFullSignatureReductionEvidenceReport_currentStructuralFactsTrue :
    selectedRustFullSignatureReductionEvidenceReport.fullExecutionSourceConsumed = true ∧
    selectedRustFullSignatureReductionEvidenceReport.activeHashBoundaryConsumed = true ∧
    selectedRustFullSignatureReductionEvidenceReport.multiTargetHorizonPriced = true := by
  native_decide

/-- The current selected full-reduction report completes the structural half. -/
theorem selectedRustFullSignatureReductionEvidenceReport_currentStructuralFactsComplete :
    selectedRustFullSignatureReductionEvidenceReport.StructuralFactsComplete := by
  simp [SelectedRustFullSignatureReductionEvidenceReport.StructuralFactsComplete,
    selectedRustFullSignatureReductionEvidenceReport]

/-- The current selected full-reduction report is still missing four production reduction facts. -/
theorem selectedRustFullSignatureReductionEvidenceReport_currentMissingReductionFactsFalse :
    selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned = false ∧
    selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame = false ∧
    selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked = false ∧
    selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced = false := by
  native_decide

/-- The current selected full-reduction report has not pinned production signature probability. -/
theorem selectedRustFullSignatureReductionEvidenceReport_currentProductionSignatureProbabilityPinned_false :
    selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned = false :=
  selectedRustFullSignatureReductionEvidenceReport_currentMissingReductionFactsFalse.1

/-- The current selected full-reduction report has not linked signature success to the accepted-spend game. -/
theorem selectedRustFullSignatureReductionEvidenceReport_currentSignatureSuccessMatchesAcceptedSpendGame_false :
    selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame = false :=
  selectedRustFullSignatureReductionEvidenceReport_currentMissingReductionFactsFalse.2.1

/-- The current selected full-reduction report has not linked accepted-spend probability. -/
theorem selectedRustFullSignatureReductionEvidenceReport_currentAcceptedSpendProbabilityLinked_false :
    selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked = false :=
  selectedRustFullSignatureReductionEvidenceReport_currentMissingReductionFactsFalse.2.2.1

/-- The current selected full-reduction report has not priced every active final query symbol. -/
theorem selectedRustFullSignatureReductionEvidenceReport_currentAllActiveQuerySymbolsPriced_false :
    selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced = false :=
  selectedRustFullSignatureReductionEvidenceReport_currentMissingReductionFactsFalse.2.2.2

/--
The current selected full-reduction report now binds the verifier game to the
selected production execution source.
-/
theorem selectedRustFullSignatureReductionEvidenceReport_currentVerifierGameMatchesProductionExecution_true :
    selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution = true := by
  native_decide

/--
Lean mirror of the current full-reduction production diagnostic.

This records the hard production facts that remain false before the selected
full-signature reduction certificate can be instantiated.
-/
structure SelectedRustFullSignatureReductionProductionObligationGap where
  productionSignatureProbabilityPinnedMissing :
    selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned = false
  signatureSuccessMatchesAcceptedSpendGameMissing :
    selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame = false
  acceptedSpendProbabilityLinkedMissing :
    selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked = false
  allActiveQuerySymbolsPricedMissing :
    selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced = false

/-- Current selected full-reduction production obligations still missing in Lean. -/
def selectedRustFullSignatureReductionEvidenceReport_currentProductionObligationGap :
    SelectedRustFullSignatureReductionProductionObligationGap where
  productionSignatureProbabilityPinnedMissing :=
    selectedRustFullSignatureReductionEvidenceReport_currentProductionSignatureProbabilityPinned_false
  signatureSuccessMatchesAcceptedSpendGameMissing :=
    selectedRustFullSignatureReductionEvidenceReport_currentSignatureSuccessMatchesAcceptedSpendGame_false
  acceptedSpendProbabilityLinkedMissing :=
    selectedRustFullSignatureReductionEvidenceReport_currentAcceptedSpendProbabilityLinked_false
  allActiveQuerySymbolsPricedMissing :=
    selectedRustFullSignatureReductionEvidenceReport_currentAllActiveQuerySymbolsPriced_false

/-- The current full-reduction production gap blocks the selected production half. -/
theorem selectedRustFullSignatureReductionEvidenceReport_currentProductionGap_blocksProductionFactsComplete
    (Gap : SelectedRustFullSignatureReductionProductionObligationGap) :
    ¬ selectedRustFullSignatureReductionEvidenceReport.ProductionFactsComplete := by
  intro hProduction
  have hProductionPinned :
      selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned = true :=
    SelectedRustFullSignatureReductionEvidenceReport.productionFactsComplete_productionSignatureProbabilityPinned
      selectedRustFullSignatureReductionEvidenceReport hProduction
  rw [Gap.productionSignatureProbabilityPinnedMissing] at hProductionPinned
  cases hProductionPinned

/-- The current selected full-reduction report cannot complete the production half. -/
theorem selectedRustFullSignatureReductionEvidenceReport_currentProductionFactsComplete_impossible :
    ¬ selectedRustFullSignatureReductionEvidenceReport.ProductionFactsComplete := by
  exact
    selectedRustFullSignatureReductionEvidenceReport_currentProductionGap_blocksProductionFactsComplete
      selectedRustFullSignatureReductionEvidenceReport_currentProductionObligationGap

/--
For the active full-signature reduction report, the only remaining
report-completeness input is the production reduction half; the structural
source-consumption evidence is already selected and proved for AP2-N003.
-/
theorem selectedRustFullSignatureReductionEvidenceReportFactsAllTrue_of_currentProductionFactsComplete
    (hProduction :
      selectedRustFullSignatureReductionEvidenceReport.ProductionFactsComplete) :
    selectedRustFullSignatureReductionEvidenceReport.FactsAllTrue :=
  SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_of_structural_and_production
    selectedRustFullSignatureReductionEvidenceReport
    selectedRustFullSignatureReductionEvidenceReport_currentStructuralFactsComplete
    hProduction

/--
For the active full-reduction report, completing `FactsAllTrue` is exactly the
remaining production reduction half; the structural source-consumption half is
already selected.
-/
theorem selectedRustFullSignatureReductionEvidenceReport_currentFactsAllTrue_iff_productionFactsComplete :
    selectedRustFullSignatureReductionEvidenceReport.FactsAllTrue ↔
      selectedRustFullSignatureReductionEvidenceReport.ProductionFactsComplete := by
  constructor
  · intro hFacts
    exact
      (SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_iff_structural_and_production
        selectedRustFullSignatureReductionEvidenceReport).1 hFacts |>.2
  · exact selectedRustFullSignatureReductionEvidenceReportFactsAllTrue_of_currentProductionFactsComplete

/--
Selected Rust/model certificate for the full-signature reduction layer.

This certificate is the non-raw route into `NativeFullSignatureSecurityBoundary`.
It keeps the production signature success probability, the inequality into the
accepted-spend game, and the active hash/QROM budget in one reviewed package.
-/
structure SelectedRustFullSignatureReductionCertificate where
  evidenceReport : SelectedRustFullSignatureReductionEvidenceReport
  evidenceDigest : ByteArray
  evidenceReportBound :
    evidenceReport = selectedRustFullSignatureReductionEvidenceReport
  evidenceReportBoundProof :
    evidenceReport = selectedRustFullSignatureReductionEvidenceReport
  evidenceDigestBound :
    evidenceDigest = selectedRustFullSignatureReductionEvidenceDigest
  evidenceDigestBoundProof :
    evidenceDigest = selectedRustFullSignatureReductionEvidenceDigest
  selectedRustFullSignatureReductionVerifiedProof :
    evidenceReport.ReportVerified
  selectedRustFullSignatureReductionFactsAllTrueProof :
    evidenceReport.FactsAllTrue
  acceptedSpendCertificate : SelectedRustAcceptedSpendSoundnessCertificate
  hashBoundary : NativeHashBudgetBoundary
  signatureForgerySuccessProb : ℝ
  productionSignatureForgerySuccessProb : ℝ
  signatureForgerySuccessProbPinned :
    signatureForgerySuccessProb = productionSignatureForgerySuccessProb
  signatureForgerySuccessMatchesAcceptedSpendGame :
    signatureForgerySuccessProb =
      acceptedSpendCertificate.pricedNumericCertificate.successProb
  keygenDistributionModeled : Prop
  keygenDistributionModeledProof : keygenDistributionModeled
  signingOracleModeled : Prop
  signingOracleModeledProof : signingOracleModeled
  completenessModeled : Prop
  completenessModeledProof : completenessModeled
  verifierRelationMatchesImplementation : Prop
  verifierRelationMatchesImplementationProof :
    verifierRelationMatchesImplementation
  fullForgeryImpliesAcceptedSpendForgery : Prop
  fullForgeryImpliesAcceptedSpendForgeryProof :
    fullForgeryImpliesAcceptedSpendForgery
  hiddenSourceProofSemanticsClosedProof :
    acceptedSpendCertificate.toAcceptedSpendSoundnessBoundary.hiddenSourceProofSemanticsClosed
  checkedCoreCompositionClosedProof :
    acceptedSpendCertificate.toAcceptedSpendSoundnessBoundary.checkedCoreCompositionClosed
  lifecycleReplayClosedProof :
    acceptedSpendCertificate.toAcceptedSpendSoundnessBoundary.lifecycleReplayClosed
  noExportClosedProof :
    acceptedSpendCertificate.toAcceptedSpendSoundnessBoundary.noExportClosed
  checkedCoreFrameDomainSeparatedProof :
    hashBoundary.checkedCoreFrameDomainSeparated
  hiddenSourceFrameDomainSeparatedProof :
    hashBoundary.hiddenSourceFrameDomainSeparated
  queryBudgetPricedProof :
    hashBoundary.queryBudgetPriced
  multiTargetLossPricedProof :
    hashBoundary.multiTargetLossPriced

/-- Named constructor for the selected Rust full-signature reduction certificate. -/
def SelectedRustFullSignatureReductionCertificate.ofReportAndReduction
    (evidenceReport : SelectedRustFullSignatureReductionEvidenceReport)
    (evidenceDigest : ByteArray)
    (evidenceReportBound :
      evidenceReport = selectedRustFullSignatureReductionEvidenceReport)
    (evidenceReportBoundProof :
      evidenceReport = selectedRustFullSignatureReductionEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustFullSignatureReductionEvidenceDigest)
    (evidenceDigestBoundProof :
      evidenceDigest = selectedRustFullSignatureReductionEvidenceDigest)
    (selectedRustFullSignatureReductionVerifiedProof :
      evidenceReport.ReportVerified)
    (selectedRustFullSignatureReductionFactsAllTrueProof :
      evidenceReport.FactsAllTrue)
    (acceptedSpendCertificate : SelectedRustAcceptedSpendSoundnessCertificate)
    (hashBoundary : NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        acceptedSpendCertificate.pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced) :
    SelectedRustFullSignatureReductionCertificate where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  evidenceReportBound := evidenceReportBound
  evidenceReportBoundProof := evidenceReportBoundProof
  evidenceDigestBound := evidenceDigestBound
  evidenceDigestBoundProof := evidenceDigestBoundProof
  selectedRustFullSignatureReductionVerifiedProof :=
    selectedRustFullSignatureReductionVerifiedProof
  selectedRustFullSignatureReductionFactsAllTrueProof :=
    selectedRustFullSignatureReductionFactsAllTrueProof
  acceptedSpendCertificate := acceptedSpendCertificate
  hashBoundary := hashBoundary
  signatureForgerySuccessProb := signatureForgerySuccessProb
  productionSignatureForgerySuccessProb :=
    productionSignatureForgerySuccessProb
  signatureForgerySuccessProbPinned :=
    signatureForgerySuccessProbPinned
  signatureForgerySuccessMatchesAcceptedSpendGame :=
    signatureForgerySuccessMatchesAcceptedSpendGame
  keygenDistributionModeled := keygenDistributionModeled
  keygenDistributionModeledProof := keygenDistributionModeledProof
  signingOracleModeled := signingOracleModeled
  signingOracleModeledProof := signingOracleModeledProof
  completenessModeled := completenessModeled
  completenessModeledProof := completenessModeledProof
  verifierRelationMatchesImplementation :=
    verifierRelationMatchesImplementation
  verifierRelationMatchesImplementationProof :=
    verifierRelationMatchesImplementationProof
  fullForgeryImpliesAcceptedSpendForgery :=
    fullForgeryImpliesAcceptedSpendForgery
  fullForgeryImpliesAcceptedSpendForgeryProof :=
    fullForgeryImpliesAcceptedSpendForgeryProof
  hiddenSourceProofSemanticsClosedProof :=
    acceptedSpendCertificate.hiddenSourceProofSemanticsClosedProof
  checkedCoreCompositionClosedProof :=
    acceptedSpendCertificate.checkedCoreCompositionClosedProof
  lifecycleReplayClosedProof :=
    acceptedSpendCertificate.lifecycleReplayClosedProof
  noExportClosedProof :=
    acceptedSpendCertificate.noExportClosedProof
  checkedCoreFrameDomainSeparatedProof :=
    checkedCoreFrameDomainSeparatedProof
  hiddenSourceFrameDomainSeparatedProof :=
    hiddenSourceFrameDomainSeparatedProof
  queryBudgetPricedProof := queryBudgetPricedProof
  multiTargetLossPricedProof := multiTargetLossPricedProof

/--
Active AP2-N003 constructor for the selected full-signature reduction
certificate.  The selected report supplies verification and structural facts;
the remaining hard input is the production reduction half.
-/
def SelectedRustFullSignatureReductionCertificate.ofSelectedReportAndReductionProductionFacts
    (selectedRustFullSignatureReductionProductionFactsCompleteProof :
      selectedRustFullSignatureReductionEvidenceReport.ProductionFactsComplete)
    (acceptedSpendCertificate : SelectedRustAcceptedSpendSoundnessCertificate)
    (hashBoundary : NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        acceptedSpendCertificate.pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced) :
    SelectedRustFullSignatureReductionCertificate :=
  SelectedRustFullSignatureReductionCertificate.ofReportAndReduction
    selectedRustFullSignatureReductionEvidenceReport
    selectedRustFullSignatureReductionEvidenceDigest
    rfl
    rfl
    rfl
    rfl
    _root_.Descender.Path2NativeAccountDesign.selectedRustFullSignatureReductionVerifiedProof
    (selectedRustFullSignatureReductionEvidenceReportFactsAllTrue_of_currentProductionFactsComplete
      selectedRustFullSignatureReductionProductionFactsCompleteProof)
    acceptedSpendCertificate
    hashBoundary
    signatureForgerySuccessProb
    productionSignatureForgerySuccessProb
    signatureForgerySuccessProbPinned
    signatureForgerySuccessMatchesAcceptedSpendGame
    keygenDistributionModeled
    keygenDistributionModeledProof
    signingOracleModeled
    signingOracleModeledProof
    completenessModeled
    completenessModeledProof
    verifierRelationMatchesImplementation
    verifierRelationMatchesImplementationProof
    fullForgeryImpliesAcceptedSpendForgery
    fullForgeryImpliesAcceptedSpendForgeryProof
    checkedCoreFrameDomainSeparatedProof
    hiddenSourceFrameDomainSeparatedProof
    queryBudgetPricedProof
    multiTargetLossPricedProof

/--
Production-reduction witness for the active selected full-signature report.

This packages the report production bits together with the actual probability
pinning, accepted-spend-game equality, active hash boundary, and production
verifier/execution obligations consumed by the selected full-reduction
certificate constructor.
-/
structure SelectedRustFullSignatureReductionProductionWitness where
  acceptedSpendCertificate : SelectedRustAcceptedSpendSoundnessCertificate
  hashBoundary : NativeHashBudgetBoundary
  signatureForgerySuccessProb : ℝ
  productionSignatureForgerySuccessProb : ℝ
  signatureForgerySuccessProbPinned :
    signatureForgerySuccessProb = productionSignatureForgerySuccessProb
  signatureForgerySuccessMatchesAcceptedSpendGame :
    signatureForgerySuccessProb =
      acceptedSpendCertificate.pricedNumericCertificate.successProb
  keygenDistributionModeled : Prop
  keygenDistributionModeledProof : keygenDistributionModeled
  signingOracleModeled : Prop
  signingOracleModeledProof : signingOracleModeled
  completenessModeled : Prop
  completenessModeledProof : completenessModeled
  verifierRelationMatchesImplementation : Prop
  verifierRelationMatchesImplementationProof :
    verifierRelationMatchesImplementation
  fullForgeryImpliesAcceptedSpendForgery : Prop
  fullForgeryImpliesAcceptedSpendForgeryProof :
    fullForgeryImpliesAcceptedSpendForgery
  checkedCoreFrameDomainSeparatedProof :
    hashBoundary.checkedCoreFrameDomainSeparated
  hiddenSourceFrameDomainSeparatedProof :
    hashBoundary.hiddenSourceFrameDomainSeparated
  queryBudgetPricedProof :
    hashBoundary.queryBudgetPriced
  multiTargetLossPricedProof :
    hashBoundary.multiTargetLossPriced
  productionSignatureProbabilityPinnedProof :
    selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned = true
  signatureSuccessMatchesAcceptedSpendGameProof :
    selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame = true
  acceptedSpendProbabilityLinkedProof :
    selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked = true
  allActiveQuerySymbolsPricedProof :
    selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced = true
  verifierGameMatchesProductionExecutionProof :
    selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution = true

/--
Named constructor for the selected full-reduction production witness.

This deliberately keeps the five hard production report bits as separate inputs
instead of accepting only a bundled `ProductionFactsComplete` proof.
-/
def SelectedRustFullSignatureReductionProductionWitness.ofAcceptedSpendAndReportBits
    (acceptedSpendCertificate : SelectedRustAcceptedSpendSoundnessCertificate)
    (hashBoundary : NativeHashBudgetBoundary)
    (signatureForgerySuccessProb : ℝ)
    (productionSignatureForgerySuccessProb : ℝ)
    (signatureForgerySuccessProbPinned :
      signatureForgerySuccessProb = productionSignatureForgerySuccessProb)
    (signatureForgerySuccessMatchesAcceptedSpendGame :
      signatureForgerySuccessProb =
        acceptedSpendCertificate.pricedNumericCertificate.successProb)
    (keygenDistributionModeled : Prop)
    (keygenDistributionModeledProof : keygenDistributionModeled)
    (signingOracleModeled : Prop)
    (signingOracleModeledProof : signingOracleModeled)
    (completenessModeled : Prop)
    (completenessModeledProof : completenessModeled)
    (verifierRelationMatchesImplementation : Prop)
    (verifierRelationMatchesImplementationProof :
      verifierRelationMatchesImplementation)
    (fullForgeryImpliesAcceptedSpendForgery : Prop)
    (fullForgeryImpliesAcceptedSpendForgeryProof :
      fullForgeryImpliesAcceptedSpendForgery)
    (checkedCoreFrameDomainSeparatedProof :
      hashBoundary.checkedCoreFrameDomainSeparated)
    (hiddenSourceFrameDomainSeparatedProof :
      hashBoundary.hiddenSourceFrameDomainSeparated)
    (queryBudgetPricedProof :
      hashBoundary.queryBudgetPriced)
    (multiTargetLossPricedProof :
      hashBoundary.multiTargetLossPriced)
    (productionSignatureProbabilityPinnedProof :
      selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned = true)
    (signatureSuccessMatchesAcceptedSpendGameProof :
      selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame = true)
    (acceptedSpendProbabilityLinkedProof :
      selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked = true)
    (allActiveQuerySymbolsPricedProof :
      selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced = true)
    (verifierGameMatchesProductionExecutionProof :
      selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution = true) :
    SelectedRustFullSignatureReductionProductionWitness where
  acceptedSpendCertificate := acceptedSpendCertificate
  hashBoundary := hashBoundary
  signatureForgerySuccessProb := signatureForgerySuccessProb
  productionSignatureForgerySuccessProb := productionSignatureForgerySuccessProb
  signatureForgerySuccessProbPinned := signatureForgerySuccessProbPinned
  signatureForgerySuccessMatchesAcceptedSpendGame :=
    signatureForgerySuccessMatchesAcceptedSpendGame
  keygenDistributionModeled := keygenDistributionModeled
  keygenDistributionModeledProof := keygenDistributionModeledProof
  signingOracleModeled := signingOracleModeled
  signingOracleModeledProof := signingOracleModeledProof
  completenessModeled := completenessModeled
  completenessModeledProof := completenessModeledProof
  verifierRelationMatchesImplementation := verifierRelationMatchesImplementation
  verifierRelationMatchesImplementationProof :=
    verifierRelationMatchesImplementationProof
  fullForgeryImpliesAcceptedSpendForgery := fullForgeryImpliesAcceptedSpendForgery
  fullForgeryImpliesAcceptedSpendForgeryProof :=
    fullForgeryImpliesAcceptedSpendForgeryProof
  checkedCoreFrameDomainSeparatedProof :=
    checkedCoreFrameDomainSeparatedProof
  hiddenSourceFrameDomainSeparatedProof :=
    hiddenSourceFrameDomainSeparatedProof
  queryBudgetPricedProof := queryBudgetPricedProof
  multiTargetLossPricedProof := multiTargetLossPricedProof
  productionSignatureProbabilityPinnedProof :=
    productionSignatureProbabilityPinnedProof
  signatureSuccessMatchesAcceptedSpendGameProof :=
    signatureSuccessMatchesAcceptedSpendGameProof
  acceptedSpendProbabilityLinkedProof :=
    acceptedSpendProbabilityLinkedProof
  allActiveQuerySymbolsPricedProof := allActiveQuerySymbolsPricedProof
  verifierGameMatchesProductionExecutionProof :=
    verifierGameMatchesProductionExecutionProof

namespace SelectedRustFullSignatureReductionProductionWitness

/-- The selected production witness supplies the active report production half. -/
theorem productionFactsComplete
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    SelectedRustFullSignatureReductionEvidenceReport.ProductionFactsComplete
      selectedRustFullSignatureReductionEvidenceReport :=
  ⟨W.productionSignatureProbabilityPinnedProof,
    W.signatureSuccessMatchesAcceptedSpendGameProof,
    W.acceptedSpendProbabilityLinkedProof,
    W.allActiveQuerySymbolsPricedProof,
    W.verifierGameMatchesProductionExecutionProof⟩

/-- The production witness exposes the active signature-probability pinning bit. -/
theorem productionSignatureProbabilityPinned
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    selectedRustFullSignatureReductionEvidenceReport.productionSignatureProbabilityPinned = true :=
  W.productionSignatureProbabilityPinnedProof

/-- The production witness exposes the signature-success/accepted-spend game link bit. -/
theorem signatureSuccessMatchesAcceptedSpendGame
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    selectedRustFullSignatureReductionEvidenceReport.signatureSuccessMatchesAcceptedSpendGame = true :=
  W.signatureSuccessMatchesAcceptedSpendGameProof

/-- The production witness exposes the accepted-spend probability linkage bit. -/
theorem acceptedSpendProbabilityLinked
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    selectedRustFullSignatureReductionEvidenceReport.acceptedSpendProbabilityLinked = true :=
  W.acceptedSpendProbabilityLinkedProof

/-- The production witness exposes active query-symbol pricing. -/
theorem allActiveQuerySymbolsPriced
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    selectedRustFullSignatureReductionEvidenceReport.allActiveQuerySymbolsPriced = true :=
  W.allActiveQuerySymbolsPricedProof

/-- The production witness exposes the verifier-game/production-execution binding bit. -/
theorem verifierGameMatchesProductionExecution
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    selectedRustFullSignatureReductionEvidenceReport.verifierGameMatchesProductionExecution = true :=
  W.verifierGameMatchesProductionExecutionProof

/-- The production witness pins selected signature success to the production probability. -/
theorem signatureForgerySuccessProb_eq_production
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.signatureForgerySuccessProb = W.productionSignatureForgerySuccessProb :=
  W.signatureForgerySuccessProbPinned

/-- The production witness identifies selected signature success with the accepted-spend game. -/
theorem signatureForgerySuccessProb_eq_acceptedSpendGame
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.signatureForgerySuccessProb =
      W.acceptedSpendCertificate.pricedNumericCertificate.successProb :=
  W.signatureForgerySuccessMatchesAcceptedSpendGame

/-- The production witness carries the accepted-spend certificate probability bound. -/
theorem acceptedSpendCertificate_prob_le_bound
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.acceptedSpendCertificate.acceptedSpendForgeryProb <=
      W.acceptedSpendCertificate.acceptedSpendForgeryBound :=
  W.acceptedSpendCertificate.acceptedSpendForgeryProb_le_bound

/-- The production witness carries the selected key-generation model obligation. -/
theorem keygenDistributionModeled_holds
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.keygenDistributionModeled :=
  W.keygenDistributionModeledProof

/-- The production witness carries the selected signing-oracle model obligation. -/
theorem signingOracleModeled_holds
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.signingOracleModeled :=
  W.signingOracleModeledProof

/-- The production witness carries the selected completeness model obligation. -/
theorem completenessModeled_holds
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.completenessModeled :=
  W.completenessModeledProof

/-- The production witness carries the selected verifier relation obligation. -/
theorem verifierRelationMatchesImplementation_holds
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.verifierRelationMatchesImplementation :=
  W.verifierRelationMatchesImplementationProof

/-- The production witness carries the full-forgery-to-accepted-spend obligation. -/
theorem fullForgeryImpliesAcceptedSpendForgery_holds
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.fullForgeryImpliesAcceptedSpendForgery :=
  W.fullForgeryImpliesAcceptedSpendForgeryProof

/-- The production witness carries checked-core frame domain separation. -/
theorem checkedCoreFrameDomainSeparated
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.hashBoundary.checkedCoreFrameDomainSeparated :=
  W.checkedCoreFrameDomainSeparatedProof

/-- The production witness carries hidden-source frame domain separation. -/
theorem hiddenSourceFrameDomainSeparated
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.hashBoundary.hiddenSourceFrameDomainSeparated :=
  W.hiddenSourceFrameDomainSeparatedProof

/-- The production witness carries active query-budget pricing through its hash boundary. -/
theorem queryBudgetPriced
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.hashBoundary.queryBudgetPriced :=
  W.queryBudgetPricedProof

/-- The production witness carries active multi-target-loss pricing through its hash boundary. -/
theorem multiTargetLossPriced
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.hashBoundary.multiTargetLossPriced :=
  W.multiTargetLossPricedProof

/-- Build the selected full-reduction certificate from the production witness. -/
def toFullSignatureReductionCertificate
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    SelectedRustFullSignatureReductionCertificate :=
  SelectedRustFullSignatureReductionCertificate.ofSelectedReportAndReductionProductionFacts
    (productionFactsComplete W)
    W.acceptedSpendCertificate
    W.hashBoundary
    W.signatureForgerySuccessProb
    W.productionSignatureForgerySuccessProb
    W.signatureForgerySuccessProbPinned
    W.signatureForgerySuccessMatchesAcceptedSpendGame
    W.keygenDistributionModeled
    W.keygenDistributionModeledProof
    W.signingOracleModeled
    W.signingOracleModeledProof
    W.completenessModeled
    W.completenessModeledProof
    W.verifierRelationMatchesImplementation
    W.verifierRelationMatchesImplementationProof
    W.fullForgeryImpliesAcceptedSpendForgery
    W.fullForgeryImpliesAcceptedSpendForgeryProof
    W.checkedCoreFrameDomainSeparatedProof
    W.hiddenSourceFrameDomainSeparatedProof
    W.queryBudgetPricedProof
    W.multiTargetLossPricedProof

/-- No selected production witness exists for the current fail-closed full-reduction report. -/
theorem currentSelectedReport_impossible
    (W : SelectedRustFullSignatureReductionProductionWitness) : False :=
  selectedRustFullSignatureReductionEvidenceReport_currentProductionFactsComplete_impossible
    (productionFactsComplete W)

end SelectedRustFullSignatureReductionProductionWitness

/-- No selected full-signature certificate exists for the current fail-closed report. -/
theorem SelectedRustFullSignatureReductionCertificate.currentSelectedReport_impossible
    (Cert : SelectedRustFullSignatureReductionCertificate) : False :=
  selectedRustFullSignatureReductionFactsAllTrue_impossible (by
    rw [← Cert.evidenceReportBoundProof]
    exact Cert.selectedRustFullSignatureReductionFactsAllTrueProof)

/-- The selected full-reduction certificate carries the key-generation model obligation. -/
theorem SelectedRustFullSignatureReductionCertificate.keygenDistributionModeled_holds
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.keygenDistributionModeled :=
  Cert.keygenDistributionModeledProof

/-- The selected full-reduction certificate carries the signing-oracle model obligation. -/
theorem SelectedRustFullSignatureReductionCertificate.signingOracleModeled_holds
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.signingOracleModeled :=
  Cert.signingOracleModeledProof

/-- The selected full-reduction certificate carries the completeness model obligation. -/
theorem SelectedRustFullSignatureReductionCertificate.completenessModeled_holds
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.completenessModeled :=
  Cert.completenessModeledProof

/-- The selected full-reduction certificate carries the verifier relation obligation. -/
theorem SelectedRustFullSignatureReductionCertificate.verifierRelationMatchesImplementation_holds
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.verifierRelationMatchesImplementation :=
  Cert.verifierRelationMatchesImplementationProof

/-- The selected full-reduction certificate carries the full-forgery-to-accepted-spend obligation. -/
theorem SelectedRustFullSignatureReductionCertificate.fullForgeryImpliesAcceptedSpendForgery_holds
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.fullForgeryImpliesAcceptedSpendForgery :=
  Cert.fullForgeryImpliesAcceptedSpendForgeryProof

/-- The selected full-reduction certificate carries checked-core frame domain separation. -/
theorem SelectedRustFullSignatureReductionCertificate.checkedCoreFrameDomainSeparated
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.hashBoundary.checkedCoreFrameDomainSeparated :=
  Cert.checkedCoreFrameDomainSeparatedProof

/-- The selected full-reduction certificate carries hidden-source frame domain separation. -/
theorem SelectedRustFullSignatureReductionCertificate.hiddenSourceFrameDomainSeparated
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.hashBoundary.hiddenSourceFrameDomainSeparated :=
  Cert.hiddenSourceFrameDomainSeparatedProof

/-- The selected full-reduction certificate carries active query-budget pricing. -/
theorem SelectedRustFullSignatureReductionCertificate.queryBudgetPriced
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.hashBoundary.queryBudgetPriced :=
  Cert.queryBudgetPricedProof

/-- The selected full-reduction certificate carries active multi-target-loss pricing. -/
theorem SelectedRustFullSignatureReductionCertificate.multiTargetLossPriced
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.hashBoundary.multiTargetLossPriced :=
  Cert.multiTargetLossPricedProof

/-- The selected full-signature success probability is pinned to production. -/
theorem SelectedRustFullSignatureReductionCertificate.signatureForgerySuccessProb_eq_production
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.signatureForgerySuccessProb =
      Cert.productionSignatureForgerySuccessProb :=
  Cert.signatureForgerySuccessProbPinned

/-- The selected full-signature success probability is the accepted-spend game success. -/
theorem SelectedRustFullSignatureReductionCertificate.signatureForgerySuccessProb_eq_acceptedSpendGame
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.signatureForgerySuccessProb =
      Cert.acceptedSpendCertificate.pricedNumericCertificate.successProb :=
  Cert.signatureForgerySuccessMatchesAcceptedSpendGame

/-- The selected full-signature reduction reaches the exported accepted-spend probability. -/
theorem SelectedRustFullSignatureReductionCertificate.signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.signatureForgerySuccessProb <=
      Cert.acceptedSpendCertificate.acceptedSpendForgeryProb := by
  calc
    Cert.signatureForgerySuccessProb =
        Cert.acceptedSpendCertificate.pricedNumericCertificate.successProb :=
      Cert.signatureForgerySuccessProb_eq_acceptedSpendGame
    _ <= Cert.acceptedSpendCertificate.acceptedSpendForgeryProb :=
      Cert.acceptedSpendCertificate.successProb_le_acceptedSpendForgeryProb

/-- The selected full-signature reduction reaches the accepted-spend component budget. -/
theorem SelectedRustFullSignatureReductionCertificate.signatureForgerySuccessProb_le_componentBudget
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.signatureForgerySuccessProb <=
      Cert.acceptedSpendCertificate.componentBudget := by
  calc
    Cert.signatureForgerySuccessProb =
        Cert.acceptedSpendCertificate.pricedNumericCertificate.successProb :=
      Cert.signatureForgerySuccessProb_eq_acceptedSpendGame
    _ <= Cert.acceptedSpendCertificate.componentBudget :=
      Cert.acceptedSpendCertificate.successProb_le_componentBudget

/-- The selected full-signature reduction reaches the exported accepted-spend bound. -/
theorem SelectedRustFullSignatureReductionCertificate.signatureForgerySuccessProb_le_acceptedSpendBound
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.signatureForgerySuccessProb <=
      Cert.acceptedSpendCertificate.acceptedSpendForgeryBound :=
  le_trans
    Cert.signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    Cert.acceptedSpendCertificate.acceptedSpendForgeryProb_le_bound

/-- The selected full-signature success probability is nonnegative. -/
theorem SelectedRustFullSignatureReductionCertificate.signatureForgerySuccessProb_nonneg
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    0 <= Cert.signatureForgerySuccessProb := by
  calc
    0 <= Cert.acceptedSpendCertificate.pricedNumericCertificate.successProb :=
      Cert.acceptedSpendCertificate.pricedNumericCertificate.successProb_nonneg
    _ = Cert.signatureForgerySuccessProb :=
      Cert.signatureForgerySuccessProb_eq_acceptedSpendGame.symm

/-- The selected production full-signature success probability is nonnegative. -/
theorem SelectedRustFullSignatureReductionCertificate.productionSignatureForgerySuccessProb_nonneg
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    0 <= Cert.productionSignatureForgerySuccessProb := by
  rw [← Cert.signatureForgerySuccessProbPinned]
  exact Cert.signatureForgerySuccessProb_nonneg

/-- The selected full-signature success probability is at most one. -/
theorem SelectedRustFullSignatureReductionCertificate.signatureForgerySuccessProb_le_one
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.signatureForgerySuccessProb <= 1 := by
  calc
    Cert.signatureForgerySuccessProb =
        Cert.acceptedSpendCertificate.pricedNumericCertificate.successProb :=
      Cert.signatureForgerySuccessProb_eq_acceptedSpendGame
    _ <= 1 :=
      Cert.acceptedSpendCertificate.pricedNumericCertificate.successProb_le_one

/-- The selected production full-signature success probability is at most one. -/
theorem SelectedRustFullSignatureReductionCertificate.productionSignatureForgerySuccessProb_le_one
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.productionSignatureForgerySuccessProb <= 1 := by
  rw [← Cert.signatureForgerySuccessProbPinned]
  exact Cert.signatureForgerySuccessProb_le_one

/-- The selected full-signature reduction is priced by the accepted-spend reported security bits. -/
theorem SelectedRustFullSignatureReductionCertificate.signatureForgerySuccessProb_le_reportedSecurity
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.signatureForgerySuccessProb <=
      ((2 : ℝ) ^
        Cert.acceptedSpendCertificate.evidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹ :=
  le_trans
    Cert.signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    Cert.acceptedSpendCertificate.acceptedSpendForgeryProb_le_reportedSecurity

/-- The selected full-signature reduction reaches the accepted-spend target-security bound. -/
theorem SelectedRustFullSignatureReductionCertificate.signatureForgerySuccessProb_le_targetSecurity
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.signatureForgerySuccessProb <=
      ((2 : ℝ) ^
        Cert.acceptedSpendCertificate.evidenceReport.targetSecurityBits)⁻¹ :=
  le_trans
    Cert.signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    Cert.acceptedSpendCertificate.acceptedSpendForgeryProb_le_targetSecurity

/-- The active selected full-signature reduction reaches the fixed 128-bit target. -/
theorem SelectedRustFullSignatureReductionCertificate.signatureForgerySuccessProb_le_active128BitTarget
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.signatureForgerySuccessProb <= ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  le_trans
    Cert.signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    Cert.acceptedSpendCertificate.acceptedSpendForgeryProb_le_active128BitTarget

/-- Convert selected Rust full-signature reduction evidence into the boundary. -/
def SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    NativeFullSignatureSecurityBoundary where
  acceptedSpendSoundness :=
    Cert.acceptedSpendCertificate.toAcceptedSpendSoundnessBoundary
  hashBoundary := Cert.hashBoundary
  keygenDistributionModeled := Cert.keygenDistributionModeled
  signingOracleModeled := Cert.signingOracleModeled
  completenessModeled := Cert.completenessModeled
  verifierRelationMatchesImplementation :=
    Cert.verifierRelationMatchesImplementation
  fullForgeryImpliesAcceptedSpendForgery :=
    Cert.fullForgeryImpliesAcceptedSpendForgery
  signatureForgerySuccessProb := Cert.signatureForgerySuccessProb
  signatureForgerySuccess_le_acceptedSpend := by
    let _hVerified : Cert.evidenceReport.ReportVerified :=
      Cert.selectedRustFullSignatureReductionVerifiedProof
    let _hFacts : Cert.evidenceReport.FactsAllTrue :=
      Cert.selectedRustFullSignatureReductionFactsAllTrueProof
    let _hSchema :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_schemaMatches
        Cert.evidenceReport _hVerified
    let _hReportDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_reportDigestVerified
        Cert.evidenceReport _hVerified
    let _hReductionDigests :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_reductionEvidenceDigestsVerified
        Cert.evidenceReport _hVerified
    let _hProductionProbabilityDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_productionSignatureProbabilityDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hAcceptedSpendLinkDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_signatureAcceptedSpendLinkDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hFullExecutionDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_fullExecutionSourceDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hHashBoundaryDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_activeHashBoundaryDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hQueryPricingDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_queryBudgetPricingDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hMultiTargetDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_multiTargetPricingDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hVerifierGameDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_verifierGameBindingDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hReportProbabilityPinned :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_productionSignatureProbabilityPinned
        Cert.evidenceReport _hFacts
    let _hReportAcceptedSpendGame :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_signatureSuccessMatchesAcceptedSpendGame
        Cert.evidenceReport _hFacts
    let _hReportAcceptedSpendLinked :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_acceptedSpendProbabilityLinked
        Cert.evidenceReport _hFacts
    let _hReportExecutionSource :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_fullExecutionSourceConsumed
        Cert.evidenceReport _hFacts
    let _hReportHashBoundary :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_activeHashBoundaryConsumed
        Cert.evidenceReport _hFacts
    let _hReportQuerySymbols :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_allActiveQuerySymbolsPriced
        Cert.evidenceReport _hFacts
    let _hReportMultiTarget :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_multiTargetHorizonPriced
        Cert.evidenceReport _hFacts
    let _hReportVerifierGame :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_verifierGameMatchesProductionExecution
        Cert.evidenceReport _hFacts
    let _hReportBound :
        Cert.evidenceReport = selectedRustFullSignatureReductionEvidenceReport :=
      Cert.evidenceReportBoundProof
    let _hDigestBound :
        Cert.evidenceDigest = selectedRustFullSignatureReductionEvidenceDigest :=
      Cert.evidenceDigestBoundProof
    let _hSigPinned :
        Cert.signatureForgerySuccessProb =
          Cert.productionSignatureForgerySuccessProb :=
      Cert.signatureForgerySuccessProbPinned
    change
      Cert.signatureForgerySuccessProb <=
        Cert.acceptedSpendCertificate.acceptedSpendForgeryProb
    exact Cert.signatureForgerySuccessProb_le_acceptedSpendForgeryProb
  fullSignatureSecurityClaimComplete := by
    let _hVerified : Cert.evidenceReport.ReportVerified :=
      Cert.selectedRustFullSignatureReductionVerifiedProof
    let _hFacts : Cert.evidenceReport.FactsAllTrue :=
      Cert.selectedRustFullSignatureReductionFactsAllTrueProof
    let _hSchema :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_schemaMatches
        Cert.evidenceReport _hVerified
    let _hReportDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_reportDigestVerified
        Cert.evidenceReport _hVerified
    let _hReductionDigests :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_reductionEvidenceDigestsVerified
        Cert.evidenceReport _hVerified
    let _hProductionProbabilityDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_productionSignatureProbabilityDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hAcceptedSpendLinkDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_signatureAcceptedSpendLinkDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hFullExecutionDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_fullExecutionSourceDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hHashBoundaryDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_activeHashBoundaryDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hQueryPricingDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_queryBudgetPricingDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hMultiTargetDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_multiTargetPricingDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hVerifierGameDigest :=
      SelectedRustFullSignatureReductionEvidenceReport.reportVerified_verifierGameBindingDigest_nonzero
        Cert.evidenceReport _hVerified
    let _hReportProbabilityPinned :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_productionSignatureProbabilityPinned
        Cert.evidenceReport _hFacts
    let _hReportAcceptedSpendGame :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_signatureSuccessMatchesAcceptedSpendGame
        Cert.evidenceReport _hFacts
    let _hReportAcceptedSpendLinked :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_acceptedSpendProbabilityLinked
        Cert.evidenceReport _hFacts
    let _hReportExecutionSource :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_fullExecutionSourceConsumed
        Cert.evidenceReport _hFacts
    let _hReportHashBoundary :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_activeHashBoundaryConsumed
        Cert.evidenceReport _hFacts
    let _hReportQuerySymbols :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_allActiveQuerySymbolsPriced
        Cert.evidenceReport _hFacts
    let _hReportMultiTarget :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_multiTargetHorizonPriced
        Cert.evidenceReport _hFacts
    let _hReportVerifierGame :=
      SelectedRustFullSignatureReductionEvidenceReport.factsAllTrue_verifierGameMatchesProductionExecution
        Cert.evidenceReport _hFacts
    let _hReportBound :
        Cert.evidenceReport = selectedRustFullSignatureReductionEvidenceReport :=
      Cert.evidenceReportBoundProof
    let _hDigestBound :
        Cert.evidenceDigest = selectedRustFullSignatureReductionEvidenceDigest :=
      Cert.evidenceDigestBoundProof
    exact
      ⟨Cert.keygenDistributionModeledProof,
        Cert.signingOracleModeledProof,
        Cert.completenessModeledProof,
        Cert.verifierRelationMatchesImplementationProof,
        Cert.fullForgeryImpliesAcceptedSpendForgeryProof,
        Cert.hiddenSourceProofSemanticsClosedProof,
        Cert.checkedCoreCompositionClosedProof,
        Cert.lifecycleReplayClosedProof,
        Cert.noExportClosedProof,
        Cert.checkedCoreFrameDomainSeparatedProof,
        Cert.hiddenSourceFrameDomainSeparatedProof,
        Cert.queryBudgetPricedProof,
        Cert.multiTargetLossPricedProof⟩

/-- The erased full-signature boundary from the selected certificate still reaches the fixed target. -/
theorem SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.toNativeFullSignatureSecurityBoundary.signatureForgerySuccessProb <=
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  Cert.signatureForgerySuccessProb_le_active128BitTarget

/-- The erased full-signature boundary keeps the signature success probability nonnegative. -/
theorem SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    0 <= Cert.toNativeFullSignatureSecurityBoundary.signatureForgerySuccessProb :=
  Cert.signatureForgerySuccessProb_nonneg

namespace NativeFullSignatureSecurityBoundary

def fullSignatureSuccessProb
    (B : NativeFullSignatureSecurityBoundary) : ℝ :=
  B.signatureForgerySuccessProb

def activePathForgeryBound
    (B : NativeFullSignatureSecurityBoundary) : ℝ :=
  B.acceptedSpendSoundness.acceptedSpendForgeryBound

/-- The native full-signature boundary exposes its key-generation model obligation. -/
theorem keygenDistributionModeled_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.keygenDistributionModeled := by
  rcases B.fullSignatureSecurityClaimComplete with ⟨hKeygen, _⟩
  exact hKeygen

/-- The native full-signature boundary exposes its signing-oracle model obligation. -/
theorem signingOracleModeled_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.signingOracleModeled := by
  rcases B.fullSignatureSecurityClaimComplete with ⟨_, hSigning, _⟩
  exact hSigning

/-- The native full-signature boundary exposes its completeness model obligation. -/
theorem completenessModeled_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.completenessModeled := by
  rcases B.fullSignatureSecurityClaimComplete with ⟨_, _, hCompleteness, _⟩
  exact hCompleteness

/-- The native full-signature boundary exposes its verifier relation obligation. -/
theorem verifierRelationMatchesImplementation_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.verifierRelationMatchesImplementation := by
  rcases B.fullSignatureSecurityClaimComplete with ⟨_, _, _, hVerifier, _⟩
  exact hVerifier

/-- The native full-signature boundary exposes full-forgery reduction to accepted spend. -/
theorem fullForgeryImpliesAcceptedSpendForgery_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.fullForgeryImpliesAcceptedSpendForgery := by
  rcases B.fullSignatureSecurityClaimComplete with ⟨_, _, _, _, hFullForgery, _⟩
  exact hFullForgery

/-- The native full-signature boundary exposes hidden-source closure after erasure. -/
theorem hiddenSourceProofSemanticsClosed_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.acceptedSpendSoundness.hiddenSourceProofSemanticsClosed := by
  rcases B.fullSignatureSecurityClaimComplete with ⟨_, _, _, _, _, hHidden, _⟩
  exact hHidden

/-- The native full-signature boundary exposes checked-core closure after erasure. -/
theorem checkedCoreCompositionClosed_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.acceptedSpendSoundness.checkedCoreCompositionClosed := by
  rcases B.fullSignatureSecurityClaimComplete with ⟨_, _, _, _, _, _, hChecked, _⟩
  exact hChecked

/-- The native full-signature boundary exposes lifecycle/replay closure after erasure. -/
theorem lifecycleReplayClosed_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.acceptedSpendSoundness.lifecycleReplayClosed := by
  rcases B.fullSignatureSecurityClaimComplete with ⟨_, _, _, _, _, _, _, hLifecycle, _⟩
  exact hLifecycle

/-- The native full-signature boundary exposes no-export closure after erasure. -/
theorem noExportClosed_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.acceptedSpendSoundness.noExportClosed := by
  rcases B.fullSignatureSecurityClaimComplete with ⟨_, _, _, _, _, _, _, _, hNoExport, _⟩
  exact hNoExport

/-- The native full-signature boundary exposes checked-core frame domain separation. -/
theorem checkedCoreFrameDomainSeparated_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.hashBoundary.checkedCoreFrameDomainSeparated := by
  rcases B.fullSignatureSecurityClaimComplete with ⟨_, _, _, _, _, _, _, _, _, hCheckedDomain, _⟩
  exact hCheckedDomain

/-- The native full-signature boundary exposes hidden-source frame domain separation. -/
theorem hiddenSourceFrameDomainSeparated_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.hashBoundary.hiddenSourceFrameDomainSeparated := by
  rcases B.fullSignatureSecurityClaimComplete with
    ⟨_, _, _, _, _, _, _, _, _, _, hHiddenDomain, _⟩
  exact hHiddenDomain

/-- The native full-signature boundary exposes active query-budget pricing. -/
theorem queryBudgetPriced_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.hashBoundary.queryBudgetPriced := by
  rcases B.fullSignatureSecurityClaimComplete with
    ⟨_, _, _, _, _, _, _, _, _, _, _, hQueryBudget, _⟩
  exact hQueryBudget

/-- The native full-signature boundary exposes active multi-target-loss pricing. -/
theorem multiTargetLossPriced_holds
    (B : NativeFullSignatureSecurityBoundary) :
    B.hashBoundary.multiTargetLossPriced := by
  rcases B.fullSignatureSecurityClaimComplete with
    ⟨_, _, _, _, _, _, _, _, _, _, _, _, hMultiTarget⟩
  exact hMultiTarget

/-- The native full-signature boundary exposes its reduction to accepted-spend probability. -/
theorem signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    (B : NativeFullSignatureSecurityBoundary) :
    B.signatureForgerySuccessProb <=
      B.acceptedSpendSoundness.acceptedSpendForgeryProb :=
  B.signatureForgerySuccess_le_acceptedSpend

/--
The active Path 2 full-signature bound follows only after the native full-game
boundary supplies a reduction to accepted-spend soundness and the accepted-spend
soundness layer supplies its own bound.
-/
theorem fullSignatureSuccessProb_le_activePathForgeryBound
    (B : NativeFullSignatureSecurityBoundary) :
    B.fullSignatureSuccessProb <= B.activePathForgeryBound :=
  le_trans B.signatureForgerySuccess_le_acceptedSpend
    B.acceptedSpendSoundness.acceptedSpendForgery_le_bound

end NativeFullSignatureSecurityBoundary

/-- The erased full-signature boundary keeps selected signature success at most one. -/
theorem SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.toNativeFullSignatureSecurityBoundary.signatureForgerySuccessProb <= 1 :=
  Cert.signatureForgerySuccessProb_le_one

/-- The erased full-signature boundary preserves the accepted-spend reduction. -/
theorem SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.toNativeFullSignatureSecurityBoundary.signatureForgerySuccessProb <=
      Cert.toNativeFullSignatureSecurityBoundary.acceptedSpendSoundness.acceptedSpendForgeryProb :=
  NativeFullSignatureSecurityBoundary.signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    Cert.toNativeFullSignatureSecurityBoundary

/-- The erased full-signature boundary exposes the active Path 2 bound handoff. -/
theorem SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.toNativeFullSignatureSecurityBoundary.fullSignatureSuccessProb <=
      Cert.toNativeFullSignatureSecurityBoundary.activePathForgeryBound :=
  NativeFullSignatureSecurityBoundary.fullSignatureSuccessProb_le_activePathForgeryBound
    Cert.toNativeFullSignatureSecurityBoundary

/--
The erased full-signature boundary keeps the active Path 2 bound at the fixed
accepted-spend 128-bit target.
-/
theorem SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.toNativeFullSignatureSecurityBoundary.activePathForgeryBound <=
      ((2 : ℝ) ^ (128 : Nat))⁻¹ := by
  simpa [NativeFullSignatureSecurityBoundary.activePathForgeryBound] using
    SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary_acceptedSpendForgeryBound_le_active128BitTarget
      Cert.acceptedSpendCertificate

/--
The erased full-signature boundary reaches the fixed active target through the
active Path 2 bound handoff.
-/
theorem SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget
    (Cert : SelectedRustFullSignatureReductionCertificate) :
    Cert.toNativeFullSignatureSecurityBoundary.fullSignatureSuccessProb <=
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  le_trans
    (Cert.toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound)
    (Cert.toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget)

namespace SelectedRustFullSignatureReductionProductionWitness

/-- The production witness reaches the exported accepted-spend probability. -/
theorem signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.signatureForgerySuccessProb <=
      W.acceptedSpendCertificate.acceptedSpendForgeryProb :=
  (toFullSignatureReductionCertificate W).signatureForgerySuccessProb_le_acceptedSpendForgeryProb

/-- The production witness reaches the accepted-spend component budget. -/
theorem signatureForgerySuccessProb_le_componentBudget
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.signatureForgerySuccessProb <=
      W.acceptedSpendCertificate.componentBudget :=
  (toFullSignatureReductionCertificate W).signatureForgerySuccessProb_le_componentBudget

/-- The production witness reaches the active 128-bit target. -/
theorem signatureForgerySuccessProb_le_active128BitTarget
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.signatureForgerySuccessProb <= ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  (toFullSignatureReductionCertificate W).signatureForgerySuccessProb_le_active128BitTarget

/-- The production witness keeps selected signature success nonnegative. -/
theorem signatureForgerySuccessProb_nonneg
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    0 <= W.signatureForgerySuccessProb :=
  (toFullSignatureReductionCertificate W).signatureForgerySuccessProb_nonneg

/-- The production witness keeps selected signature success at most one. -/
theorem signatureForgerySuccessProb_le_one
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.signatureForgerySuccessProb <= 1 :=
  (toFullSignatureReductionCertificate W).signatureForgerySuccessProb_le_one

/-- The production witness keeps production signature success nonnegative. -/
theorem productionSignatureForgerySuccessProb_nonneg
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    0 <= W.productionSignatureForgerySuccessProb :=
  (toFullSignatureReductionCertificate W).productionSignatureForgerySuccessProb_nonneg

/-- The production witness keeps production signature success at most one. -/
theorem productionSignatureForgerySuccessProb_le_one
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    W.productionSignatureForgerySuccessProb <= 1 :=
  (toFullSignatureReductionCertificate W).productionSignatureForgerySuccessProb_le_one

/-- Erase the selected production witness directly into the native full-signature boundary. -/
def toNativeFullSignatureSecurityBoundary
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    NativeFullSignatureSecurityBoundary :=
  (toFullSignatureReductionCertificate W).toNativeFullSignatureSecurityBoundary

/-- The erased full-signature boundary from the production witness reaches the active target. -/
theorem toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    (toNativeFullSignatureSecurityBoundary W).signatureForgerySuccessProb <=
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  (toFullSignatureReductionCertificate W).toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_active128BitTarget

/-- The erased boundary from the production witness keeps signature success nonnegative. -/
theorem toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    0 <= (toNativeFullSignatureSecurityBoundary W).signatureForgerySuccessProb :=
  (toFullSignatureReductionCertificate W).toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_nonneg

/-- The erased boundary from the production witness keeps signature success at most one. -/
theorem toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    (toNativeFullSignatureSecurityBoundary W).signatureForgerySuccessProb <= 1 :=
  (toFullSignatureReductionCertificate W).toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_one

/-- The erased boundary from the production witness preserves the accepted-spend reduction. -/
theorem toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_acceptedSpendForgeryProb
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    (toNativeFullSignatureSecurityBoundary W).signatureForgerySuccessProb <=
      (toNativeFullSignatureSecurityBoundary W).acceptedSpendSoundness.acceptedSpendForgeryProb :=
  (toFullSignatureReductionCertificate W).toNativeFullSignatureSecurityBoundary_signatureForgerySuccessProb_le_acceptedSpendForgeryProb

/-- The erased boundary from the production witness exposes the active Path 2 bound handoff. -/
theorem toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    (toNativeFullSignatureSecurityBoundary W).fullSignatureSuccessProb <=
      (toNativeFullSignatureSecurityBoundary W).activePathForgeryBound :=
  (toFullSignatureReductionCertificate W).toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_activePathForgeryBound

/-- The erased boundary from the production witness keeps the active Path 2 bound at the fixed target. -/
theorem toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    (toNativeFullSignatureSecurityBoundary W).activePathForgeryBound <=
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  (toFullSignatureReductionCertificate W).toNativeFullSignatureSecurityBoundary_activePathForgeryBound_le_active128BitTarget

/-- The erased boundary from the production witness reaches the fixed active target. -/
theorem toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget
    (W : SelectedRustFullSignatureReductionProductionWitness) :
    (toNativeFullSignatureSecurityBoundary W).fullSignatureSuccessProb <=
      ((2 : ℝ) ^ (128 : Nat))⁻¹ :=
  (toFullSignatureReductionCertificate W).toNativeFullSignatureSecurityBoundary_fullSignatureSuccessProb_le_active128BitTarget

end SelectedRustFullSignatureReductionProductionWitness

end Path2NativeAccountDesign
end Descender
