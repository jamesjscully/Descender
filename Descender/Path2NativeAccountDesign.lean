import Mathlib.Data.Real.Basic

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

/--
Lean mirror of the selected Rust accepted-spend numeric evidence report.

The deployable theorem must bind the accepted-spend probability, bound,
inequality proof, and prerequisite closure facts to reviewed production
evidence before constructing the active accepted-spend boundary.
-/
structure SelectedRustAcceptedSpendSoundnessEvidenceReport where
  schemaMatches : Bool
  reportDigestVerified : Bool
  acceptedSpendNumericBoundPriced : Bool
  acceptedSpendInequalityProved : Bool
  hiddenSourceClosureReady : Bool
  checkedCoreClosureReady : Bool
  lifecycleReplayClosureReady : Bool
  noExportClosureReady : Bool

/-- The accepted-spend numeric report has the expected schema and digest. -/
def SelectedRustAcceptedSpendSoundnessEvidenceReport.ReportVerified
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport) : Prop :=
  E.schemaMatches = true ∧
  E.reportDigestVerified = true

/-- All accepted-spend numeric and prerequisite closure facts are true. -/
def SelectedRustAcceptedSpendSoundnessEvidenceReport.FactsAllTrue
    (E : SelectedRustAcceptedSpendSoundnessEvidenceReport) : Prop :=
  E.acceptedSpendNumericBoundPriced = true ∧
  E.acceptedSpendInequalityProved = true ∧
  E.hiddenSourceClosureReady = true ∧
  E.checkedCoreClosureReady = true ∧
  E.lifecycleReplayClosureReady = true ∧
  E.noExportClosureReady = true

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
  selectedRustAcceptedSpendBoundVerifiedProof :
    evidenceReport.ReportVerified
  selectedRustAcceptedSpendFactsAllTrueProof :
    evidenceReport.FactsAllTrue
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

/-- Named constructor for the selected Rust accepted-spend numeric certificate. -/
def SelectedRustAcceptedSpendSoundnessCertificate.ofReportAndBound
    (evidenceReport : SelectedRustAcceptedSpendSoundnessEvidenceReport)
    (evidenceDigest : ByteArray)
    (selectedRustAcceptedSpendBoundVerifiedProof :
      evidenceReport.ReportVerified)
    (selectedRustAcceptedSpendFactsAllTrueProof :
      evidenceReport.FactsAllTrue)
    (acceptedSpendForgeryProb : ℝ)
    (acceptedSpendForgeryBound : ℝ)
    (productionAcceptedSpendForgeryProb : ℝ)
    (productionAcceptedSpendForgeryBound : ℝ)
    (acceptedSpendForgeryProbPinned :
      acceptedSpendForgeryProb = productionAcceptedSpendForgeryProb)
    (acceptedSpendForgeryBoundPinned :
      acceptedSpendForgeryBound = productionAcceptedSpendForgeryBound)
    (acceptedSpendForgery_le_bound :
      acceptedSpendForgeryProb <= acceptedSpendForgeryBound)
    (hiddenSourceProofSemanticsClosed : Prop)
    (checkedCoreCompositionClosed : Prop)
    (lifecycleReplayClosed : Prop)
    (noExportClosed : Prop) :
    SelectedRustAcceptedSpendSoundnessCertificate where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  selectedRustAcceptedSpendBoundVerifiedProof :=
    selectedRustAcceptedSpendBoundVerifiedProof
  selectedRustAcceptedSpendFactsAllTrueProof :=
    selectedRustAcceptedSpendFactsAllTrueProof
  acceptedSpendForgeryProb := acceptedSpendForgeryProb
  acceptedSpendForgeryBound := acceptedSpendForgeryBound
  productionAcceptedSpendForgeryProb := productionAcceptedSpendForgeryProb
  productionAcceptedSpendForgeryBound := productionAcceptedSpendForgeryBound
  acceptedSpendForgeryProbPinned := acceptedSpendForgeryProbPinned
  acceptedSpendForgeryBoundPinned := acceptedSpendForgeryBoundPinned
  acceptedSpendForgery_le_bound := acceptedSpendForgery_le_bound
  hiddenSourceProofSemanticsClosed := hiddenSourceProofSemanticsClosed
  checkedCoreCompositionClosed := checkedCoreCompositionClosed
  lifecycleReplayClosed := lifecycleReplayClosed
  noExportClosed := noExportClosed

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
    let _hProbPinned :
        Cert.acceptedSpendForgeryProb =
          Cert.productionAcceptedSpendForgeryProb :=
      Cert.acceptedSpendForgeryProbPinned
    let _hBoundPinned :
        Cert.acceptedSpendForgeryBound =
          Cert.productionAcceptedSpendForgeryBound :=
      Cert.acceptedSpendForgeryBoundPinned
    exact Cert.acceptedSpendForgery_le_bound
  hiddenSourceProofSemanticsClosed :=
    Cert.hiddenSourceProofSemanticsClosed
  checkedCoreCompositionClosed :=
    Cert.checkedCoreCompositionClosed
  lifecycleReplayClosed :=
    Cert.lifecycleReplayClosed
  noExportClosed :=
    Cert.noExportClosed

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

namespace NativeFullSignatureSecurityBoundary

def fullSignatureSuccessProb
    (B : NativeFullSignatureSecurityBoundary) : ℝ :=
  B.signatureForgerySuccessProb

def activePathForgeryBound
    (B : NativeFullSignatureSecurityBoundary) : ℝ :=
  B.acceptedSpendSoundness.acceptedSpendForgeryBound

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

end Path2NativeAccountDesign
end Descender
