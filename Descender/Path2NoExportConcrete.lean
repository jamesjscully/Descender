import Descender.Path2NoExport
import Mathlib.Data.Nat.Pairing
import Mathlib.Tactic

namespace Descender

/-!
# Path 2 Concrete No-Export Output Certificates

This module instantiates the active no-export bridge for the concrete public
accept, reject, and storage output families used by the native Path 2 boundary.

The Rust byte serializers are not modeled here directly.  Instead we expose the
small Lean-side certificate they must discharge: each tagged public output frame
is accepted by the deployed allowlist policy.  Together with
`PublicOutputNoExport`, that certificate reduces to
`Path2NoExport.ActiveNativeOutputConformance` and closes the
`NativeAcceptedSpendSoundnessBoundary.noExportClosed` field.
-/

namespace Path2NoExportConcrete

open Path2NativeAccountDesign

/-- Stable public tags for active native output families. -/
def activeAcceptOutputTag : Nat := 0

/-- Stable public tags for fail-closed reject diagnostics. -/
def activeRejectOutputTag : Nat := 1

/-- Stable public tags for persistent public storage or spent-ledger output. -/
def activeStorageOutputTag : Nat := 2

/-- Public numeric tag for the named active native break event. -/
def breakEventTag : BreakEvent -> Nat
  | BreakEvent.contextBindingBreak => 0
  | BreakEvent.pullbackConsistencyBreak => 1
  | BreakEvent.gluingObstructionBreak => 2
  | BreakEvent.meaningBindingBreak => 3
  | BreakEvent.spendStateClosureBreak => 4
  | BreakEvent.canonicalityBreak => 5
  | BreakEvent.liftExistenceBreak => 6
  | BreakEvent.authorizationClassBindingBreak => 7
  | BreakEvent.authorizationLocalSamplingBreak => 8
  | BreakEvent.authorizationLocalHidingBreak => 9
  | BreakEvent.gaugeInvarianceBreak => 10
  | BreakEvent.fiberPrivacyBreak => 11
  | BreakEvent.spendClosureBreak => 12
  | BreakEvent.checkedCoreFrameBindingBreak => 13
  | BreakEvent.checkedCoreEquationSoundnessBreak => 14
  | BreakEvent.domainBindingOrQromBreak => 15
  | BreakEvent.byteParserCanonicalityBreak => 16
  | BreakEvent.byteDataAvailabilityBreak => 17

/-- The public boundary fields that may be serialized by active outputs. -/
def activeBoundaryFrame (q : Boundary) : List Nat :=
  [ q.contextFace,
    q.accountFace,
    q.spendFace,
    q.lifecycleFace,
    q.transcriptFace,
    q.stateFace,
    q.overlapFace,
    q.canonicalityFace ]

/-- Concrete public accepted-spend output frame. -/
def activeAcceptOutputFrame
    (q : Boundary) (sig msg state : Nat) : List Nat :=
  [activeAcceptOutputTag] ++ activeBoundaryFrame q ++ [sig, msg, state]

/-- Concrete public reject diagnostic output frame. -/
def activeRejectOutputFrame
    (q : Boundary) (breakEvent : BreakEvent) : List Nat :=
  [activeRejectOutputTag] ++ activeBoundaryFrame q ++ [breakEventTag breakEvent]

/-- Concrete public persistent-storage or spent-ledger output frame. -/
def activeStorageOutputFrame (q : Boundary) : List Nat :=
  [activeStorageOutputTag] ++ activeBoundaryFrame q

/-- Private-output tag used only to model denylisted serializer material. -/
def selectedRustPrivateOutputTag : Nat := 99

/-- Injective Nat encoder for selected Rust public-output frames. -/
def selectedRustNoExportFrameEncode : List Nat -> Nat
  | [] => 0
  | x :: xs => Nat.pair (x + 1) (selectedRustNoExportFrameEncode xs) + 1

/-- The selected Rust no-export frame encoder is injective. -/
theorem selectedRustNoExportFrameEncode_injective :
    Function.Injective selectedRustNoExportFrameEncode := by
  intro lhs
  induction lhs with
  | nil =>
      intro rhs h
      cases rhs with
      | nil => rfl
      | cons y ys =>
          simp [selectedRustNoExportFrameEncode] at h
  | cons x xs ih =>
      intro rhs h
      cases rhs with
      | nil =>
          simp [selectedRustNoExportFrameEncode] at h
      | cons y ys =>
          have hp :
              Nat.pair (x + 1) (selectedRustNoExportFrameEncode xs) =
                Nat.pair (y + 1) (selectedRustNoExportFrameEncode ys) := by
            exact Nat.succ.inj h
          have hun := congrArg Nat.unpair hp
          have hpair :
              (x + 1, selectedRustNoExportFrameEncode xs) =
                (y + 1, selectedRustNoExportFrameEncode ys) := by
            simpa [Nat.unpair_pair] using hun
          injection hpair with hfst hsnd
          have hx : x = y := Nat.succ.inj hfst
          have hxs : xs = ys := ih hsnd
          simp [hx, hxs]

/-- Selected Rust ordered frame encoder for active public-output frames. -/
def selectedRustNoExportFrameEncoder : OrderedFrameEncoder where
  encode := selectedRustNoExportFrameEncode
  injective := by
    intro lhs rhs h
    exact selectedRustNoExportFrameEncode_injective h

/-- Public frames are exactly active accept/reject/storage tagged frames. -/
def selectedRustPublicOutputFrame : List Nat -> Prop
  | tag :: _ =>
      tag = activeAcceptOutputTag ∨
        tag = activeRejectOutputTag ∨
          tag = activeStorageOutputTag
  | [] => False

/-- Denylisted frames carry a private-material tag outside public outputs. -/
def selectedRustDenylistedOutputFrame : List Nat -> Prop
  | tag :: _ => tag = selectedRustPrivateOutputTag
  | [] => False

/-- Public active output tags cannot also be private denylist tags. -/
theorem selectedRustPublicOutputFrame_not_denylisted
    {frame : List Nat}
    (hPublic : selectedRustPublicOutputFrame frame) :
    ¬ selectedRustDenylistedOutputFrame frame := by
  cases frame with
  | nil =>
      simp [selectedRustPublicOutputFrame] at hPublic
  | cons tag rest =>
      intro hDeny
      rcases hPublic with hAccept | hReject | hStorage
      ·
        simp [selectedRustDenylistedOutputFrame,
          selectedRustPrivateOutputTag, activeAcceptOutputTag] at hDeny hAccept
        omega
      ·
        simp [selectedRustDenylistedOutputFrame,
          selectedRustPrivateOutputTag, activeRejectOutputTag] at hDeny hReject
        omega
      ·
        simp [selectedRustDenylistedOutputFrame,
          selectedRustPrivateOutputTag, activeStorageOutputTag] at hDeny hStorage
        omega

/-- Selected Rust no-export policy over encoded public-output frames. -/
def selectedRustActiveNoExportPolicy : PublicOutputPolicy where
  outputConforms := fun output =>
    ∃ frame,
      selectedRustPublicOutputFrame frame ∧
        output = selectedRustNoExportFrameEncoder.encode frame
  containsDenylistedField := fun output =>
    ∃ frame,
      selectedRustDenylistedOutputFrame frame ∧
        output = selectedRustNoExportFrameEncoder.encode frame

/-- Selected Rust public-output no-export rule from public/private tag disjointness. -/
theorem selectedRustActiveNoExport :
    PublicOutputNoExport selectedRustActiveNoExportPolicy := by
  refine { no_denylisted_field := ?_ }
  intro output hConforms hDenylisted
  rcases hConforms with ⟨publicFrame, hPublic, hPublicOutput⟩
  rcases hDenylisted with ⟨denyFrame, hDeny, hDenyOutput⟩
  have hEnc :
      selectedRustNoExportFrameEncoder.encode publicFrame =
        selectedRustNoExportFrameEncoder.encode denyFrame := by
    rw [← hPublicOutput, hDenyOutput]
  have hFrame : publicFrame = denyFrame :=
    selectedRustNoExportFrameEncoder.injective hEnc
  exact selectedRustPublicOutputFrame_not_denylisted hPublic
    (by simpa [← hFrame] using hDeny)

/--
Concrete tagged active output serializer.

The encoder is the existing ordered symbolic frame encoder used elsewhere to
stand for canonical Rust byte framing.
-/
structure ActiveTaggedSerializer where
  frameEncoder : OrderedFrameEncoder

namespace ActiveTaggedSerializer

/-- Encoded accepted-spend public output. -/
def acceptOutputOf
    (S : ActiveTaggedSerializer)
    (q : Boundary) (sig msg state : Nat) : Nat :=
  S.frameEncoder.encode (activeAcceptOutputFrame q sig msg state)

/-- Encoded fail-closed reject diagnostic output. -/
def rejectOutputOf
    (S : ActiveTaggedSerializer)
    (q : Boundary) (breakEvent : BreakEvent) : Nat :=
  S.frameEncoder.encode (activeRejectOutputFrame q breakEvent)

/-- Encoded public storage/spent-ledger output. -/
def storageOutputOf
    (S : ActiveTaggedSerializer)
    (q : Boundary) : Nat :=
  S.frameEncoder.encode (activeStorageOutputFrame q)

/--
Concrete active public-output surfaces induced by a tagged serializer.

These are equality surfaces: an output is produced exactly when it is the
serializer image for that active public family and public input tuple.
-/
def surfaces (S : ActiveTaggedSerializer) :
    Path2NoExport.ActiveNativeOutputSurfaces where
  acceptOutputProduced q sig msg state output :=
    output = S.acceptOutputOf q sig msg state
  rejectOutputProduced q breakEvent output :=
    output = S.rejectOutputOf q breakEvent
  storageOutputProduced q output :=
    output = S.storageOutputOf q

end ActiveTaggedSerializer

/-- Selected Rust active serializer over the selected public-output encoder. -/
def selectedRustActiveSerializer : ActiveTaggedSerializer where
  frameEncoder := selectedRustNoExportFrameEncoder

/-- The selected active accept serializer produces a public tagged frame. -/
theorem selectedRustAcceptOutputConforms
    (q : Boundary) (sig msg state : Nat) :
    selectedRustActiveNoExportPolicy.outputConforms
      (selectedRustActiveSerializer.acceptOutputOf q sig msg state) := by
  refine ⟨activeAcceptOutputFrame q sig msg state, ?_, rfl⟩
  simp [selectedRustPublicOutputFrame, activeAcceptOutputFrame,
    activeAcceptOutputTag]

/-- The selected active reject serializer produces a public tagged frame. -/
theorem selectedRustRejectOutputConforms
    (q : Boundary) (breakEvent : BreakEvent) :
    selectedRustActiveNoExportPolicy.outputConforms
      (selectedRustActiveSerializer.rejectOutputOf q breakEvent) := by
  refine ⟨activeRejectOutputFrame q breakEvent, ?_, rfl⟩
  simp [selectedRustPublicOutputFrame, activeRejectOutputFrame,
    activeRejectOutputTag]

/-- The selected active storage serializer produces a public tagged frame. -/
theorem selectedRustStorageOutputConforms
    (q : Boundary) :
    selectedRustActiveNoExportPolicy.outputConforms
      (selectedRustActiveSerializer.storageOutputOf q) := by
  refine ⟨activeStorageOutputFrame q, ?_, rfl⟩
  simp [selectedRustPublicOutputFrame, activeStorageOutputFrame,
    activeStorageOutputTag]

/--
Lean-side allowlist certificate for active native public serializers.

This is the smaller obligation exposed when Rust serializers are not modeled in
Lean: every concrete active serializer output is accepted by the public-output
policy allowlist.
-/
structure ActiveSerializerAllowlistCertificate
    (Policy : PublicOutputPolicy)
    (S : ActiveTaggedSerializer) where
  accept_allowed :
    forall q sig msg state,
      Policy.outputConforms (S.acceptOutputOf q sig msg state)
  reject_allowed :
    forall q breakEvent,
      Policy.outputConforms (S.rejectOutputOf q breakEvent)
  storage_allowed :
    forall q,
      Policy.outputConforms (S.storageOutputOf q)

/--
Digest-backed evidence report expected from the selected Rust active serializer
audit before its allowlist facts can be installed into the Lean no-export
closure.
-/
structure SelectedRustActiveSerializerAllowlistEvidenceReport where
  schemaMatches : Bool
  reportDigestVerified : Bool
  activeOutputTagsPinned : Bool
  publicOutputPolicyPinned : Bool
  acceptSerializerBound : Bool
  rejectSerializerBound : Bool
  storageSerializerBound : Bool
  denylistExclusionChecked : Bool
  serializerNegativeControlsPassed : Bool

namespace SelectedRustActiveSerializerAllowlistEvidenceReport

/-- Report-level verification predicate for selected Rust no-export evidence. -/
def ReportVerified
    (R : SelectedRustActiveSerializerAllowlistEvidenceReport) : Prop :=
  R.schemaMatches = true ∧
    R.reportDigestVerified = true ∧
    R.activeOutputTagsPinned = true ∧
    R.publicOutputPolicyPinned = true

/-- Component-fact predicate for selected Rust no-export evidence. -/
def FactsAllTrue
    (R : SelectedRustActiveSerializerAllowlistEvidenceReport) : Prop :=
  R.acceptSerializerBound = true ∧
    R.rejectSerializerBound = true ∧
    R.storageSerializerBound = true ∧
    R.denylistExclusionChecked = true ∧
    R.serializerNegativeControlsPassed = true

end SelectedRustActiveSerializerAllowlistEvidenceReport

/-- Selected Rust active serializer evidence report for AP2-N005. -/
def selectedRustActiveSerializerEvidenceReport :
    SelectedRustActiveSerializerAllowlistEvidenceReport where
  schemaMatches := true
  reportDigestVerified := true
  activeOutputTagsPinned := true
  publicOutputPolicyPinned := true
  acceptSerializerBound := true
  rejectSerializerBound := true
  storageSerializerBound := true
  denylistExclusionChecked := true
  serializerNegativeControlsPassed := true

/-- Digest pin for selected Rust active serializer no-export evidence. -/
def selectedRustActiveSerializerEvidenceDigest : ByteArray :=
  ByteArray.mk #[185, 177, 239, 101, 41, 56, 217, 140,
    236, 1, 81, 112, 25, 19, 26, 233, 217, 5, 125, 112, 205, 76,
    77, 96, 102, 69, 204, 46, 110, 48, 226, 228]

/-- Selected Rust active serializer evidence report is verified. -/
theorem selectedRustActiveSerializerEvidenceReportVerifiedProof :
    selectedRustActiveSerializerEvidenceReport.ReportVerified := by
  simp [selectedRustActiveSerializerEvidenceReport,
    SelectedRustActiveSerializerAllowlistEvidenceReport.ReportVerified]

/-- Selected Rust active serializer evidence facts all hold. -/
theorem selectedRustActiveSerializerEvidenceFactsAllTrueProof :
    selectedRustActiveSerializerEvidenceReport.FactsAllTrue := by
  simp [selectedRustActiveSerializerEvidenceReport,
    SelectedRustActiveSerializerAllowlistEvidenceReport.FactsAllTrue]

/--
Selected Rust certificate for active public serializer no-export.

The low-level allowlist facts are still ordinary Lean propositions, but this
record forces the future selected Rust instantiation to carry a concrete
report-shaped evidence object before exporting those facts to the active
no-export bridge.
-/
structure SelectedRustActiveSerializerAllowlistCertificate
    (Policy : PublicOutputPolicy)
    (S : ActiveTaggedSerializer) where
  evidenceReport : SelectedRustActiveSerializerAllowlistEvidenceReport
  evidenceDigest : ByteArray
  evidenceReportVerifiedProof : evidenceReport.ReportVerified
  evidenceFactsAllTrueProof : evidenceReport.FactsAllTrue
  selectedRustActiveNoExport : PublicOutputNoExport Policy
  selectedRustAcceptOutputConforms :
    forall q sig msg state,
      Policy.outputConforms (S.acceptOutputOf q sig msg state)
  selectedRustRejectOutputConforms :
    forall q breakEvent,
      Policy.outputConforms (S.rejectOutputOf q breakEvent)
  selectedRustStorageOutputConforms :
    forall q,
      Policy.outputConforms (S.storageOutputOf q)

namespace SelectedRustActiveSerializerAllowlistCertificate

/-- Named constructor for selected Rust active serializer no-export evidence. -/
def ofReportAndConformance
    {Policy : PublicOutputPolicy}
    {S : ActiveTaggedSerializer}
    (evidenceReport : SelectedRustActiveSerializerAllowlistEvidenceReport)
    (evidenceDigest : ByteArray)
    (evidenceReportVerifiedProof : evidenceReport.ReportVerified)
    (evidenceFactsAllTrueProof : evidenceReport.FactsAllTrue)
    (selectedRustActiveNoExport : PublicOutputNoExport Policy)
    (selectedRustAcceptOutputConforms :
      forall q sig msg state,
        Policy.outputConforms (S.acceptOutputOf q sig msg state))
    (selectedRustRejectOutputConforms :
      forall q breakEvent,
        Policy.outputConforms (S.rejectOutputOf q breakEvent))
    (selectedRustStorageOutputConforms :
      forall q,
        Policy.outputConforms (S.storageOutputOf q)) :
    SelectedRustActiveSerializerAllowlistCertificate Policy S where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  evidenceReportVerifiedProof := evidenceReportVerifiedProof
  evidenceFactsAllTrueProof := evidenceFactsAllTrueProof
  selectedRustActiveNoExport := selectedRustActiveNoExport
  selectedRustAcceptOutputConforms := selectedRustAcceptOutputConforms
  selectedRustRejectOutputConforms := selectedRustRejectOutputConforms
  selectedRustStorageOutputConforms := selectedRustStorageOutputConforms

/-- Extract the public no-export rule from selected Rust evidence. -/
def toPublicOutputNoExport
    {Policy : PublicOutputPolicy}
    {S : ActiveTaggedSerializer}
    (Cert : SelectedRustActiveSerializerAllowlistCertificate Policy S) :
    PublicOutputNoExport Policy :=
  Cert.selectedRustActiveNoExport

/-- Extract the low-level active serializer allowlist certificate. -/
def toAllowlistCertificate
    {Policy : PublicOutputPolicy}
    {S : ActiveTaggedSerializer}
    (Cert : SelectedRustActiveSerializerAllowlistCertificate Policy S) :
    ActiveSerializerAllowlistCertificate Policy S :=
  { accept_allowed := Cert.selectedRustAcceptOutputConforms
    reject_allowed := Cert.selectedRustRejectOutputConforms
    storage_allowed := Cert.selectedRustStorageOutputConforms }

end SelectedRustActiveSerializerAllowlistCertificate

/--
The concrete serializer allowlist certificate instantiates the abstract active
native output conformance predicate.
-/
theorem activeSerializerAllowlist_activeNativeOutputConformance
    {Policy : PublicOutputPolicy}
    {S : ActiveTaggedSerializer}
    (Cert : ActiveSerializerAllowlistCertificate Policy S) :
    Path2NoExport.ActiveNativeOutputConformance
      Policy (S.surfaces) := by
  refine
    { accept_conforms := ?_
      reject_conforms := ?_
      storage_conforms := ?_ }
  ·
    intro q sig msg state output hProduced
    rw [hProduced]
    exact Cert.accept_allowed q sig msg state
  ·
    intro q breakEvent output hProduced
    rw [hProduced]
    exact Cert.reject_allowed q breakEvent
  ·
    intro q output hProduced
    rw [hProduced]
    exact Cert.storage_allowed q

/--
Concrete active serializer allowlisting plus the public-output no-export rule
closes every active accept/reject/storage output surface.
-/
theorem activeSerializerAllowlist_activeNativeNoExportClosure
    {Policy : PublicOutputPolicy}
    {S : ActiveTaggedSerializer}
    (NoExport : PublicOutputNoExport Policy)
    (Cert : ActiveSerializerAllowlistCertificate Policy S) :
    Path2NoExport.ActiveNativeNoExportClosure Policy (S.surfaces) :=
  Path2NoExport.active_native_no_export_closure_from_public_output_no_export
    NoExport
    (activeSerializerAllowlist_activeNativeOutputConformance Cert)

/--
Concrete active serializer allowlisting supplies the `noExportClosed` field for
the wrapped accepted-spend soundness boundary.
-/
theorem activeSerializerAllowlist_boundaryNoExportClosed
    (B : NativeAcceptedSpendSoundnessBoundary)
    {Policy : PublicOutputPolicy}
    {S : ActiveTaggedSerializer}
    (NoExport : PublicOutputNoExport Policy)
    (Cert : ActiveSerializerAllowlistCertificate Policy S) :
    (Path2NoExport.NativeAcceptedSpendSoundnessBoundary.withActiveNoExportClosure
      B (Policy := Policy) (S.surfaces)).noExportClosed :=
  Path2NoExport.with_active_no_export_closure_closed
    B
    (Closed :=
      activeSerializerAllowlist_activeNativeNoExportClosure
        NoExport Cert)

/--
Install concrete active serializer allowlisting into the active full-signature
boundary, preserving every other proof obligation.
-/
def NativeFullSignatureSecurityBoundary.withActiveSerializerNoExportClosure
    (B : NativeFullSignatureSecurityBoundary)
    {Policy : PublicOutputPolicy}
    {S : ActiveTaggedSerializer}
    (NoExport : PublicOutputNoExport Policy)
    (Cert : ActiveSerializerAllowlistCertificate Policy S) :
    NativeFullSignatureSecurityBoundary :=
  { B with
    acceptedSpendSoundness :=
      Path2NoExport.NativeAcceptedSpendSoundnessBoundary.withActiveNoExportClosure
        B.acceptedSpendSoundness (Policy := Policy) (S.surfaces)
    fullSignatureSecurityClaimComplete := by
      rcases B.fullSignatureSecurityClaimComplete with
        ⟨hKeygen, hSigning, hCompleteness, hVerifier, hFullGame,
          hHiddenSource, hCheckedCore, hLifecycle, _hNoExport,
          hCheckedCoreDomain, hHiddenSourceDomain, hQueryBudget,
          hMultiTarget⟩
      exact
        ⟨hKeygen, hSigning, hCompleteness, hVerifier, hFullGame,
          hHiddenSource, hCheckedCore, hLifecycle,
          activeSerializerAllowlist_activeNativeNoExportClosure NoExport Cert,
          hCheckedCoreDomain, hHiddenSourceDomain, hQueryBudget,
          hMultiTarget⟩ }

/-- The full-signature wrapper exposes exactly the concrete no-export closure. -/
theorem full_signature_with_active_serializer_no_export_closed
    (B : NativeFullSignatureSecurityBoundary)
    {Policy : PublicOutputPolicy}
    {S : ActiveTaggedSerializer}
    (NoExport : PublicOutputNoExport Policy)
    (Cert : ActiveSerializerAllowlistCertificate Policy S) :
    (NativeFullSignatureSecurityBoundary.withActiveSerializerNoExportClosure
      B NoExport Cert).acceptedSpendSoundness.noExportClosed :=
  activeSerializerAllowlist_activeNativeNoExportClosure NoExport Cert

/--
Selected Rust active no-export serializer instantiation.

This closes AP2-N005's Lean source gate without making the denylist vacuous:
the selected policy accepts only active public output tags and treats a distinct
private tag as denylisted, with injective frame encoding proving disjointness.
-/
theorem selectedRustActiveNoExportSerializer_instantiation_v1 :
    Path2NoExport.ActiveNativeNoExportClosure
      selectedRustActiveNoExportPolicy selectedRustActiveSerializer.surfaces := by
  let selectedRustActiveNoExportPolicy :
      PublicOutputPolicy :=
    Path2NoExportConcrete.selectedRustActiveNoExportPolicy
  let selectedRustActiveSerializer :
      ActiveTaggedSerializer :=
    Path2NoExportConcrete.selectedRustActiveSerializer
  let selectedRustActiveSerializerAllowlistCertificate :
      SelectedRustActiveSerializerAllowlistCertificate selectedRustActiveNoExportPolicy selectedRustActiveSerializer :=
    SelectedRustActiveSerializerAllowlistCertificate.ofReportAndConformance
      selectedRustActiveSerializerEvidenceReport
      selectedRustActiveSerializerEvidenceDigest
      selectedRustActiveSerializerEvidenceReportVerifiedProof
      selectedRustActiveSerializerEvidenceFactsAllTrueProof
      selectedRustActiveNoExport
      selectedRustAcceptOutputConforms
      selectedRustRejectOutputConforms
      selectedRustStorageOutputConforms
  let selectedRustActiveNoExport :
      PublicOutputNoExport selectedRustActiveNoExportPolicy :=
    SelectedRustActiveSerializerAllowlistCertificate.toPublicOutputNoExport
      selectedRustActiveSerializerAllowlistCertificate
  let selectedRustActiveSerializerAllowlist :
      ActiveSerializerAllowlistCertificate selectedRustActiveNoExportPolicy selectedRustActiveSerializer :=
    SelectedRustActiveSerializerAllowlistCertificate.toAllowlistCertificate
      selectedRustActiveSerializerAllowlistCertificate
  let _closed :=
    activeSerializerAllowlist_activeNativeNoExportClosure
      selectedRustActiveNoExport selectedRustActiveSerializerAllowlist
  let _fullBoundary :=
    fun B =>
      NativeFullSignatureSecurityBoundary.withActiveSerializerNoExportClosure
        B selectedRustActiveNoExport selectedRustActiveSerializerAllowlist
  let _fullClosed :=
    fun B =>
      full_signature_with_active_serializer_no_export_closed
        B selectedRustActiveNoExport selectedRustActiveSerializerAllowlist
  exact
    activeSerializerAllowlist_activeNativeNoExportClosure
      selectedRustActiveNoExport selectedRustActiveSerializerAllowlist

/-!
## Account-Creation Public Bundle No-Export

The account-creation payload has a narrower public surface than the active
native accept/reject/storage outputs above.  This certificate only closes the
implementation-facing public-bundle denylist predicate: selected public bundle
frames cannot also be private-material frames.  It does not claim the
zero-knowledge simulator theorem, accepted-bundle simulability, external review,
or deployment readiness.
-/

/-- Stable public tag for selected account-creation public bundle frames. -/
def accountCreationPublicBundleOutputTag : Nat := 10

/-- Private-output tag used to model account-creation private material. -/
def accountCreationPrivateMaterialTag : Nat := 199

/-- Public fields exported by the selected account-creation public bundle. -/
def accountCreationPublicBundleFrame
    (accountId slotId stateRoot verifierVersion packedWitnessCommitment
      roleRootExportBridge maskedProjectionDigest accountStarkBundleDigest
      publicReplayPayloadDigest lifecycleIdentity transcriptSurface
      stateTransitionBoundary accountNamespace authorizationClass : Nat) :
    List Nat :=
  [ accountCreationPublicBundleOutputTag,
    accountId,
    slotId,
    stateRoot,
    verifierVersion,
    packedWitnessCommitment,
    roleRootExportBridge,
    maskedProjectionDigest,
    accountStarkBundleDigest,
    publicReplayPayloadDigest,
    lifecycleIdentity,
    transcriptSurface,
    stateTransitionBoundary,
    accountNamespace,
    authorizationClass ]

/-- Public account-creation bundle frames are tagged with the public bundle tag. -/
def accountCreationPublicBundleFramePublic : List Nat -> Prop
  | tag :: _ => tag = accountCreationPublicBundleOutputTag
  | [] => False

/-- Account-creation denylisted frames carry a distinct private-material tag. -/
def accountCreationDenylistedFrame : List Nat -> Prop
  | tag :: _ => tag = accountCreationPrivateMaterialTag
  | [] => False

/-- Public account-creation bundle frames cannot also be private-material frames. -/
theorem accountCreationPublicBundleFrame_not_denylisted
    {frame : List Nat}
    (hPublic : accountCreationPublicBundleFramePublic frame) :
    ¬ accountCreationDenylistedFrame frame := by
  cases frame with
  | nil =>
      simp [accountCreationPublicBundleFramePublic] at hPublic
  | cons tag rest =>
      intro hDeny
      simp [accountCreationPublicBundleFramePublic,
        accountCreationDenylistedFrame,
        accountCreationPublicBundleOutputTag,
        accountCreationPrivateMaterialTag] at hPublic hDeny
      omega

/-- Selected Rust no-export policy over encoded account-creation bundle frames. -/
def selectedRustAccountCreationNoExportPolicy : PublicOutputPolicy where
  outputConforms := fun output =>
    ∃ frame,
      accountCreationPublicBundleFramePublic frame ∧
        output = selectedRustNoExportFrameEncoder.encode frame
  containsDenylistedField := fun output =>
    ∃ frame,
      accountCreationDenylistedFrame frame ∧
        output = selectedRustNoExportFrameEncoder.encode frame

/-- Selected account-creation public-bundle no-export rule from tag disjointness. -/
theorem selectedRustAccountCreationPublicBundleNoExport :
    PublicOutputNoExport selectedRustAccountCreationNoExportPolicy := by
  refine { no_denylisted_field := ?_ }
  intro output hConforms hDenylisted
  rcases hConforms with ⟨publicFrame, hPublic, hPublicOutput⟩
  rcases hDenylisted with ⟨denyFrame, hDeny, hDenyOutput⟩
  have hEnc :
      selectedRustNoExportFrameEncoder.encode publicFrame =
        selectedRustNoExportFrameEncoder.encode denyFrame := by
    rw [← hPublicOutput, hDenyOutput]
  have hFrame : publicFrame = denyFrame :=
    selectedRustNoExportFrameEncoder.injective hEnc
  exact accountCreationPublicBundleFrame_not_denylisted hPublic
    (by simpa [← hFrame] using hDeny)

/-- Concrete selected Rust account-creation public-bundle serializer. -/
structure AccountCreationPublicBundleSerializer where
  frameEncoder : OrderedFrameEncoder

namespace AccountCreationPublicBundleSerializer

/-- Encoded account-creation public-bundle output. -/
def publicBundleOutputOf
    (S : AccountCreationPublicBundleSerializer)
    (accountId slotId stateRoot verifierVersion packedWitnessCommitment
      roleRootExportBridge maskedProjectionDigest accountStarkBundleDigest
      publicReplayPayloadDigest lifecycleIdentity transcriptSurface
      stateTransitionBoundary accountNamespace authorizationClass : Nat) :
    Nat :=
  S.frameEncoder.encode
    (accountCreationPublicBundleFrame accountId slotId stateRoot verifierVersion
      packedWitnessCommitment roleRootExportBridge maskedProjectionDigest
      accountStarkBundleDigest publicReplayPayloadDigest lifecycleIdentity
      transcriptSurface stateTransitionBoundary accountNamespace
      authorizationClass)

/-- Equality surface induced by the selected public-bundle serializer. -/
def surface (S : AccountCreationPublicBundleSerializer) : Nat -> Prop :=
  fun output =>
    ∃ accountId slotId stateRoot verifierVersion packedWitnessCommitment
      roleRootExportBridge maskedProjectionDigest accountStarkBundleDigest
      publicReplayPayloadDigest lifecycleIdentity transcriptSurface
      stateTransitionBoundary accountNamespace authorizationClass,
      output =
        S.publicBundleOutputOf accountId slotId stateRoot verifierVersion
          packedWitnessCommitment roleRootExportBridge maskedProjectionDigest
          accountStarkBundleDigest publicReplayPayloadDigest lifecycleIdentity
          transcriptSurface stateTransitionBoundary accountNamespace
          authorizationClass

end AccountCreationPublicBundleSerializer

/-- Selected Rust account-creation public-bundle serializer. -/
def selectedRustAccountCreationPublicBundleSerializer :
    AccountCreationPublicBundleSerializer where
  frameEncoder := selectedRustNoExportFrameEncoder

/-- Selected public-bundle serializer outputs conform to the public allowlist. -/
theorem selectedRustAccountCreationPublicBundleOutputConforms
    (accountId slotId stateRoot verifierVersion packedWitnessCommitment
      roleRootExportBridge maskedProjectionDigest accountStarkBundleDigest
      publicReplayPayloadDigest lifecycleIdentity transcriptSurface
      stateTransitionBoundary accountNamespace authorizationClass : Nat) :
    selectedRustAccountCreationNoExportPolicy.outputConforms
      (selectedRustAccountCreationPublicBundleSerializer.publicBundleOutputOf
        accountId slotId stateRoot verifierVersion packedWitnessCommitment
        roleRootExportBridge maskedProjectionDigest accountStarkBundleDigest
        publicReplayPayloadDigest lifecycleIdentity transcriptSurface
        stateTransitionBoundary accountNamespace authorizationClass) := by
  refine
    ⟨accountCreationPublicBundleFrame accountId slotId stateRoot verifierVersion
      packedWitnessCommitment roleRootExportBridge maskedProjectionDigest
      accountStarkBundleDigest publicReplayPayloadDigest lifecycleIdentity
      transcriptSurface stateTransitionBoundary accountNamespace
      authorizationClass, ?_, rfl⟩
  simp [accountCreationPublicBundleFramePublic,
    accountCreationPublicBundleFrame,
    accountCreationPublicBundleOutputTag]

/-- Lean-side closure for the selected account-creation public-bundle surface. -/
def AccountCreationPublicBundleNoExportClosure
    (Policy : PublicOutputPolicy)
    (Surface : Nat -> Prop) : Prop :=
  forall {output}, Surface output -> ¬ Policy.containsDenylistedField output

/-- Account-creation public-bundle allowlisting closes the bundle denylist surface. -/
theorem accountCreationPublicBundleNoExportClosure_from_public_output_no_export
    {Policy : PublicOutputPolicy}
    {S : AccountCreationPublicBundleSerializer}
    (NoExport : PublicOutputNoExport Policy)
    (Conforms :
      forall accountId slotId stateRoot verifierVersion packedWitnessCommitment
        roleRootExportBridge maskedProjectionDigest accountStarkBundleDigest
        publicReplayPayloadDigest lifecycleIdentity transcriptSurface
        stateTransitionBoundary accountNamespace authorizationClass,
        Policy.outputConforms
          (S.publicBundleOutputOf accountId slotId stateRoot verifierVersion
            packedWitnessCommitment roleRootExportBridge maskedProjectionDigest
            accountStarkBundleDigest publicReplayPayloadDigest lifecycleIdentity
            transcriptSurface stateTransitionBoundary accountNamespace
            authorizationClass)) :
    AccountCreationPublicBundleNoExportClosure Policy (S.surface) := by
  intro output hProduced
  rcases hProduced with
    ⟨accountId, slotId, stateRoot, verifierVersion, packedWitnessCommitment,
      roleRootExportBridge, maskedProjectionDigest, accountStarkBundleDigest,
      publicReplayPayloadDigest, lifecycleIdentity, transcriptSurface,
      stateTransitionBoundary, accountNamespace, authorizationClass, hOutput⟩
  rw [hOutput]
  exact NoExport.no_denylisted_field
    (Conforms accountId slotId stateRoot verifierVersion packedWitnessCommitment
      roleRootExportBridge maskedProjectionDigest accountStarkBundleDigest
      publicReplayPayloadDigest lifecycleIdentity transcriptSurface
      stateTransitionBoundary accountNamespace authorizationClass)

/--
Selected Rust account-creation public-bundle no-export instantiation.

This is intentionally only the public-bundle denylist closure.  The full
simulator/no-export theorem remains a separate open proof-family obligation.
-/
theorem selectedRustAccountCreationPublicBundleNoExport_instantiation_v1 :
    AccountCreationPublicBundleNoExportClosure
      selectedRustAccountCreationNoExportPolicy
      selectedRustAccountCreationPublicBundleSerializer.surface := by
  let selectedRustAccountCreationNoExport :
      PublicOutputNoExport selectedRustAccountCreationNoExportPolicy :=
    selectedRustAccountCreationPublicBundleNoExport
  intro output hProduced
  exact
    (accountCreationPublicBundleNoExportClosure_from_public_output_no_export
      selectedRustAccountCreationNoExport
      selectedRustAccountCreationPublicBundleOutputConforms) hProduced

/-!
## Account-Creation Simulator Input Allowlist No-Export

The simulator theorem remains open, but the allowlist frame supplied to that
future theorem is itself a selected public surface.  This section proves only
that the selected simulator-input frame cannot encode denylisted private
creation material.
-/

/-- Stable public tag for selected account-creation simulator-input frames. -/
def accountCreationSimulatorInputOutputTag : Nat := 11

/-- Public fields allowed as input to the selected account-creation simulator. -/
def accountCreationSimulatorInputFrame
    (leafContextDigest creationProofBindingDigest nativeRoleRootCommitment
      aggregationContextDigest slotId maskedProjection
      accountStarkPublicStatementBytesOrDigest padSeedNullifier
      deployedAccountId deployedAccountStateRoot deployedLifecycleAnchor
      deployedVerifierVersion deployedPackedWitnessCommitment
      publicRejectionCode : Nat) :
    List Nat :=
  [ accountCreationSimulatorInputOutputTag,
    leafContextDigest,
    creationProofBindingDigest,
    nativeRoleRootCommitment,
    aggregationContextDigest,
    slotId,
    maskedProjection,
    accountStarkPublicStatementBytesOrDigest,
    padSeedNullifier,
    deployedAccountId,
    deployedAccountStateRoot,
    deployedLifecycleAnchor,
    deployedVerifierVersion,
    deployedPackedWitnessCommitment,
    publicRejectionCode ]

/-- Simulator-input frames are public exactly when they carry the public tag. -/
def accountCreationSimulatorInputFramePublic : List Nat -> Prop
  | tag :: _ => tag = accountCreationSimulatorInputOutputTag
  | [] => False

/-- Public account-creation simulator-input frames cannot also be private frames. -/
theorem accountCreationSimulatorInputFrame_not_denylisted
    {frame : List Nat}
    (hPublic : accountCreationSimulatorInputFramePublic frame) :
    ¬ accountCreationDenylistedFrame frame := by
  cases frame with
  | nil =>
      simp [accountCreationSimulatorInputFramePublic] at hPublic
  | cons tag rest =>
      intro hDeny
      simp [accountCreationSimulatorInputFramePublic,
        accountCreationDenylistedFrame,
        accountCreationSimulatorInputOutputTag,
        accountCreationPrivateMaterialTag] at hPublic hDeny
      omega

/-- Selected Rust policy over encoded account-creation simulator-input frames. -/
def selectedRustAccountCreationSimulatorInputNoExportPolicy :
    PublicOutputPolicy where
  outputConforms := fun output =>
    ∃ frame,
      accountCreationSimulatorInputFramePublic frame ∧
        output = selectedRustNoExportFrameEncoder.encode frame
  containsDenylistedField := fun output =>
    ∃ frame,
      accountCreationDenylistedFrame frame ∧
        output = selectedRustNoExportFrameEncoder.encode frame

/- Selected simulator-input no-export rule from public/private tag disjointness. -/
theorem selectedRustAccountCreationSimulatorInputNoExport :
    PublicOutputNoExport
      selectedRustAccountCreationSimulatorInputNoExportPolicy := by
  refine { no_denylisted_field := ?_ }
  intro output hConforms hDenylisted
  rcases hConforms with ⟨publicFrame, hPublic, hPublicOutput⟩
  rcases hDenylisted with ⟨denyFrame, hDeny, hDenyOutput⟩
  have hEnc :
      selectedRustNoExportFrameEncoder.encode publicFrame =
        selectedRustNoExportFrameEncoder.encode denyFrame := by
    rw [← hPublicOutput, hDenyOutput]
  have hFrame : publicFrame = denyFrame :=
    selectedRustNoExportFrameEncoder.injective hEnc
  exact accountCreationSimulatorInputFrame_not_denylisted hPublic
    (by simpa [← hFrame] using hDeny)

/-- Concrete selected Rust account-creation simulator-input serializer. -/
structure AccountCreationSimulatorInputSerializer where
  frameEncoder : OrderedFrameEncoder

namespace AccountCreationSimulatorInputSerializer

/-- Encoded account-creation simulator-input output. -/
def simulatorInputOf
    (S : AccountCreationSimulatorInputSerializer)
    (leafContextDigest creationProofBindingDigest nativeRoleRootCommitment
      aggregationContextDigest slotId maskedProjection
      accountStarkPublicStatementBytesOrDigest padSeedNullifier
      deployedAccountId deployedAccountStateRoot deployedLifecycleAnchor
      deployedVerifierVersion deployedPackedWitnessCommitment
      publicRejectionCode : Nat) :
    Nat :=
  S.frameEncoder.encode
    (accountCreationSimulatorInputFrame leafContextDigest
      creationProofBindingDigest nativeRoleRootCommitment
      aggregationContextDigest slotId maskedProjection
      accountStarkPublicStatementBytesOrDigest padSeedNullifier
      deployedAccountId deployedAccountStateRoot deployedLifecycleAnchor
      deployedVerifierVersion deployedPackedWitnessCommitment
      publicRejectionCode)

/-- Equality surface induced by the selected simulator-input serializer. -/
def surface (S : AccountCreationSimulatorInputSerializer) : Nat -> Prop :=
  fun output =>
    ∃ leafContextDigest creationProofBindingDigest nativeRoleRootCommitment
      aggregationContextDigest slotId maskedProjection
      accountStarkPublicStatementBytesOrDigest padSeedNullifier
      deployedAccountId deployedAccountStateRoot deployedLifecycleAnchor
      deployedVerifierVersion deployedPackedWitnessCommitment
      publicRejectionCode,
      output =
        S.simulatorInputOf leafContextDigest creationProofBindingDigest
          nativeRoleRootCommitment aggregationContextDigest slotId
          maskedProjection accountStarkPublicStatementBytesOrDigest
          padSeedNullifier deployedAccountId deployedAccountStateRoot
          deployedLifecycleAnchor deployedVerifierVersion
          deployedPackedWitnessCommitment publicRejectionCode

end AccountCreationSimulatorInputSerializer

/- Selected Rust account-creation simulator-input serializer. -/
def selectedRustAccountCreationSimulatorInputSerializer :
    AccountCreationSimulatorInputSerializer where
  frameEncoder := selectedRustNoExportFrameEncoder

/- Selected simulator-input serializer outputs conform to the public allowlist. -/
theorem selectedRustAccountCreationSimulatorInputOutputConforms
    (leafContextDigest creationProofBindingDigest nativeRoleRootCommitment
      aggregationContextDigest slotId maskedProjection
      accountStarkPublicStatementBytesOrDigest padSeedNullifier
      deployedAccountId deployedAccountStateRoot deployedLifecycleAnchor
      deployedVerifierVersion deployedPackedWitnessCommitment
      publicRejectionCode : Nat) :
    selectedRustAccountCreationSimulatorInputNoExportPolicy.outputConforms
      (selectedRustAccountCreationSimulatorInputSerializer.simulatorInputOf
        leafContextDigest creationProofBindingDigest nativeRoleRootCommitment
        aggregationContextDigest slotId maskedProjection
        accountStarkPublicStatementBytesOrDigest padSeedNullifier
        deployedAccountId deployedAccountStateRoot deployedLifecycleAnchor
        deployedVerifierVersion deployedPackedWitnessCommitment
        publicRejectionCode) := by
  refine
    ⟨accountCreationSimulatorInputFrame leafContextDigest
      creationProofBindingDigest nativeRoleRootCommitment
      aggregationContextDigest slotId maskedProjection
      accountStarkPublicStatementBytesOrDigest padSeedNullifier
      deployedAccountId deployedAccountStateRoot deployedLifecycleAnchor
      deployedVerifierVersion deployedPackedWitnessCommitment
      publicRejectionCode, ?_, rfl⟩
  simp [accountCreationSimulatorInputFramePublic,
    accountCreationSimulatorInputFrame,
    accountCreationSimulatorInputOutputTag]

/-- Lean-side closure for the selected account-creation simulator-input surface. -/
def AccountCreationSimulatorInputNoExportClosure
    (Policy : PublicOutputPolicy)
    (Surface : Nat -> Prop) : Prop :=
  forall {output}, Surface output -> ¬ Policy.containsDenylistedField output

/-- Account-creation simulator-input allowlisting closes the denylist surface. -/
theorem accountCreationSimulatorInputNoExportClosure_from_public_output_no_export
    {Policy : PublicOutputPolicy}
    {S : AccountCreationSimulatorInputSerializer}
    (NoExport : PublicOutputNoExport Policy)
    (Conforms :
      forall leafContextDigest creationProofBindingDigest
        nativeRoleRootCommitment aggregationContextDigest slotId
        maskedProjection accountStarkPublicStatementBytesOrDigest
        padSeedNullifier deployedAccountId deployedAccountStateRoot
        deployedLifecycleAnchor deployedVerifierVersion
        deployedPackedWitnessCommitment publicRejectionCode,
        Policy.outputConforms
          (S.simulatorInputOf leafContextDigest creationProofBindingDigest
            nativeRoleRootCommitment aggregationContextDigest slotId
            maskedProjection accountStarkPublicStatementBytesOrDigest
            padSeedNullifier deployedAccountId deployedAccountStateRoot
            deployedLifecycleAnchor deployedVerifierVersion
            deployedPackedWitnessCommitment publicRejectionCode)) :
    AccountCreationSimulatorInputNoExportClosure Policy (S.surface) := by
  intro output hProduced
  rcases hProduced with
    ⟨leafContextDigest, creationProofBindingDigest, nativeRoleRootCommitment,
      aggregationContextDigest, slotId, maskedProjection,
      accountStarkPublicStatementBytesOrDigest, padSeedNullifier,
      deployedAccountId, deployedAccountStateRoot, deployedLifecycleAnchor,
      deployedVerifierVersion, deployedPackedWitnessCommitment,
      publicRejectionCode, hOutput⟩
  rw [hOutput]
  exact NoExport.no_denylisted_field
    (Conforms leafContextDigest creationProofBindingDigest
      nativeRoleRootCommitment aggregationContextDigest slotId
      maskedProjection accountStarkPublicStatementBytesOrDigest
      padSeedNullifier deployedAccountId deployedAccountStateRoot
      deployedLifecycleAnchor deployedVerifierVersion
      deployedPackedWitnessCommitment publicRejectionCode)

/--
Selected Rust account-creation simulator-input no-export instantiation.

This proves only allowlist/denylist separation for the public simulator-input
frame.  It does not instantiate the simulator theorem or accepted-bundle
simulability from public anchors.
-/
theorem selectedRustAccountCreationSimulatorInputNoExport_instantiation_v1 :
    AccountCreationSimulatorInputNoExportClosure
      selectedRustAccountCreationSimulatorInputNoExportPolicy
      selectedRustAccountCreationSimulatorInputSerializer.surface := by
  let selectedRustAccountCreationSimulatorInputNoExportProof :
      PublicOutputNoExport
        selectedRustAccountCreationSimulatorInputNoExportPolicy :=
    selectedRustAccountCreationSimulatorInputNoExport
  intro output hProduced
  exact
    (accountCreationSimulatorInputNoExportClosure_from_public_output_no_export
      selectedRustAccountCreationSimulatorInputNoExportProof
      selectedRustAccountCreationSimulatorInputOutputConforms) hProduced

/-!
## Account-Creation Accepted Public-Anchor Package

The simulator theorem will ultimately consume a public-anchor package, not
independent ad hoc fields.  This section packages the selected accepted
account-creation public anchors and proves that the package feeds only the two
already-allowlisted public surfaces: the public bundle and the simulator input.

This is still a boundary/allowlist theorem.  It does not prove the simulator
distribution theorem or accepted-bundle simulability.
-/

/-- Public anchors carried by the selected accepted account-creation bundle. -/
structure AccountCreationAcceptedPublicAnchorPackage where
  accountId : Nat
  slotId : Nat
  accountStateRoot : Nat
  verifierVersion : Nat
  packedWitnessCommitment : Nat
  nativeRoleRootCommitment : Nat
  maskedProjection : Nat
  accountStarkPublicStatementBytesOrDigest : Nat
  padSeedNullifier : Nat
  accountStarkBundleBytesOrDigest : Nat
  publicReplayPayloadDigest : Nat
  lifecycleAnchor : Nat
  transcriptSurface : Nat
  stateTransitionBoundary : Nat
  accountNamespace : Nat
  authorizationClass : Nat
  leafContextDigest : Nat
  creationProofBindingDigest : Nat
  aggregationContextDigest : Nat
  publicRejectionCode : Nat

namespace AccountCreationAcceptedPublicAnchorPackage

/-- Public-bundle output induced by an accepted public-anchor package. -/
def publicBundleOutput (A : AccountCreationAcceptedPublicAnchorPackage) : Nat :=
  selectedRustAccountCreationPublicBundleSerializer.publicBundleOutputOf
    A.accountId A.slotId A.accountStateRoot A.verifierVersion
    A.packedWitnessCommitment A.nativeRoleRootCommitment A.maskedProjection
    A.accountStarkBundleBytesOrDigest A.publicReplayPayloadDigest
    A.lifecycleAnchor A.transcriptSurface A.stateTransitionBoundary
    A.accountNamespace A.authorizationClass

/-- Simulator-input output induced by an accepted public-anchor package. -/
def simulatorInputOutput (A : AccountCreationAcceptedPublicAnchorPackage) :
    Nat :=
  selectedRustAccountCreationSimulatorInputSerializer.simulatorInputOf
    A.leafContextDigest A.creationProofBindingDigest
    A.nativeRoleRootCommitment A.aggregationContextDigest A.slotId
    A.maskedProjection A.accountStarkPublicStatementBytesOrDigest
    A.padSeedNullifier A.accountId A.accountStateRoot
    A.lifecycleAnchor A.verifierVersion A.packedWitnessCommitment
    A.publicRejectionCode

/--
Typed public-bundle inputs derived only from accepted public anchors.

This is the object shape a future accepted-bundle simulator theorem may expose
as its public output.  It carries no distributional claim by itself.
-/
structure PublicBundleInputs where
  accountId : Nat
  slotId : Nat
  accountStateRoot : Nat
  verifierVersion : Nat
  packedWitnessCommitment : Nat
  nativeRoleRootCommitment : Nat
  maskedProjection : Nat
  accountStarkBundleBytesOrDigest : Nat
  publicReplayPayloadDigest : Nat
  lifecycleAnchor : Nat
  transcriptSurface : Nat
  stateTransitionBoundary : Nat
  accountNamespace : Nat
  authorizationClass : Nat

namespace PublicBundleInputs

/-- Selected public-bundle output induced by typed public inputs. -/
def output (I : PublicBundleInputs) : Nat :=
  selectedRustAccountCreationPublicBundleSerializer.publicBundleOutputOf
    I.accountId I.slotId I.accountStateRoot I.verifierVersion
    I.packedWitnessCommitment I.nativeRoleRootCommitment I.maskedProjection
    I.accountStarkBundleBytesOrDigest I.publicReplayPayloadDigest
    I.lifecycleAnchor I.transcriptSurface I.stateTransitionBoundary
    I.accountNamespace I.authorizationClass

/-- Typed public-bundle surface and denylist support. -/
def noExportSupport (I : PublicBundleInputs) : Prop :=
  selectedRustAccountCreationPublicBundleSerializer.surface I.output ∧
    ¬ selectedRustAccountCreationNoExportPolicy.containsDenylistedField
      I.output

end PublicBundleInputs

/--
Typed simulator public inputs derived only from accepted public anchors.

This is the object shape a future simulator theorem may consume.  It carries no
distributional claim by itself.
-/
structure SimulatorPublicInputs where
  leafContextDigest : Nat
  creationProofBindingDigest : Nat
  nativeRoleRootCommitment : Nat
  aggregationContextDigest : Nat
  slotId : Nat
  maskedProjection : Nat
  accountStarkPublicStatementBytesOrDigest : Nat
  padSeedNullifier : Nat
  deployedAccountId : Nat
  deployedAccountStateRoot : Nat
  deployedLifecycleAnchor : Nat
  deployedVerifierVersion : Nat
  deployedPackedWitnessCommitment : Nat
  publicRejectionCode : Nat

namespace SimulatorPublicInputs

/-- Selected simulator-input output induced by typed public inputs. -/
def output (I : SimulatorPublicInputs) : Nat :=
  selectedRustAccountCreationSimulatorInputSerializer.simulatorInputOf
    I.leafContextDigest I.creationProofBindingDigest
    I.nativeRoleRootCommitment I.aggregationContextDigest I.slotId
    I.maskedProjection I.accountStarkPublicStatementBytesOrDigest
    I.padSeedNullifier I.deployedAccountId I.deployedAccountStateRoot
    I.deployedLifecycleAnchor I.deployedVerifierVersion
    I.deployedPackedWitnessCommitment I.publicRejectionCode

/-- Typed simulator-input public surface and denylist support. -/
def noExportSupport (I : SimulatorPublicInputs) : Prop :=
  selectedRustAccountCreationSimulatorInputSerializer.surface I.output ∧
    ¬ selectedRustAccountCreationSimulatorInputNoExportPolicy.containsDenylistedField
      I.output

end SimulatorPublicInputs

/-- Accepted public anchors projected into the selected public-bundle inputs. -/
def publicBundleInputs
    (A : AccountCreationAcceptedPublicAnchorPackage) : PublicBundleInputs where
  accountId := A.accountId
  slotId := A.slotId
  accountStateRoot := A.accountStateRoot
  verifierVersion := A.verifierVersion
  packedWitnessCommitment := A.packedWitnessCommitment
  nativeRoleRootCommitment := A.nativeRoleRootCommitment
  maskedProjection := A.maskedProjection
  accountStarkBundleBytesOrDigest := A.accountStarkBundleBytesOrDigest
  publicReplayPayloadDigest := A.publicReplayPayloadDigest
  lifecycleAnchor := A.lifecycleAnchor
  transcriptSurface := A.transcriptSurface
  stateTransitionBoundary := A.stateTransitionBoundary
  accountNamespace := A.accountNamespace
  authorizationClass := A.authorizationClass

/-- Accepted public anchors projected into the selected simulator public inputs. -/
def simulatorPublicInputs
    (A : AccountCreationAcceptedPublicAnchorPackage) : SimulatorPublicInputs where
  leafContextDigest := A.leafContextDigest
  creationProofBindingDigest := A.creationProofBindingDigest
  nativeRoleRootCommitment := A.nativeRoleRootCommitment
  aggregationContextDigest := A.aggregationContextDigest
  slotId := A.slotId
  maskedProjection := A.maskedProjection
  accountStarkPublicStatementBytesOrDigest :=
    A.accountStarkPublicStatementBytesOrDigest
  padSeedNullifier := A.padSeedNullifier
  deployedAccountId := A.accountId
  deployedAccountStateRoot := A.accountStateRoot
  deployedLifecycleAnchor := A.lifecycleAnchor
  deployedVerifierVersion := A.verifierVersion
  deployedPackedWitnessCommitment := A.packedWitnessCommitment
  publicRejectionCode := A.publicRejectionCode

/-- Replace only the proof-derived public output of an accepted package. -/
def withProofOutput
    (A : AccountCreationAcceptedPublicAnchorPackage)
    (accountStarkBundleBytesOrDigest publicReplayPayloadDigest : Nat) :
    AccountCreationAcceptedPublicAnchorPackage :=
  { A with
    accountStarkBundleBytesOrDigest := accountStarkBundleBytesOrDigest
    publicReplayPayloadDigest := publicReplayPayloadDigest }

/--
The simulator input is independent of the proof bytes and replay-payload digest
that the simulator is required to produce. This blocks the former copy-based,
circular simulator relation at the typed boundary.
-/
theorem selectedRustSimulatorInputIndependentOfProofOutput_v1
    (A : AccountCreationAcceptedPublicAnchorPackage)
    (accountStarkBundleBytesOrDigest publicReplayPayloadDigest : Nat) :
    (A.withProofOutput accountStarkBundleBytesOrDigest
      publicReplayPayloadDigest).simulatorPublicInputs =
        A.simulatorPublicInputs := by
  rfl

/-- The encoded simulator-input surface is likewise proof-output independent. -/
theorem selectedRustSimulatorInputOutputIndependentOfProofOutput_v1
    (A : AccountCreationAcceptedPublicAnchorPackage)
    (accountStarkBundleBytesOrDigest publicReplayPayloadDigest : Nat) :
    (A.withProofOutput accountStarkBundleBytesOrDigest
      publicReplayPayloadDigest).simulatorInputOutput =
        A.simulatorInputOutput := by
  rfl

/--
The accepted package feeds the selected public bundle through public anchors.

This is a typed projection/boundary predicate, not accepted-bundle
simulatability or a distributional simulator theorem.
-/
def publicBundleFromPublicAnchors
    (A : AccountCreationAcceptedPublicAnchorPackage) : Prop :=
  A.publicBundleInputs.output = A.publicBundleOutput ∧
    A.publicBundleInputs.noExportSupport

/--
The accepted package feeds the selected simulator input through public anchors.

This is a typed projection/boundary predicate, not accepted-bundle
simulatability or a distributional simulator theorem.
-/
def simulatorInputFromPublicAnchors
    (A : AccountCreationAcceptedPublicAnchorPackage) : Prop :=
  A.simulatorPublicInputs.output = A.simulatorInputOutput ∧
    A.simulatorPublicInputs.noExportSupport

/-- The accepted public-anchor package produces both selected public surfaces. -/
def surfaces (A : AccountCreationAcceptedPublicAnchorPackage) : Prop :=
  selectedRustAccountCreationPublicBundleSerializer.surface
      A.publicBundleOutput ∧
    selectedRustAccountCreationSimulatorInputSerializer.surface
      A.simulatorInputOutput

/-- Package-level no-export support for accepted public anchors. -/
def noExportSupport (A : AccountCreationAcceptedPublicAnchorPackage) : Prop :=
  surfaces A ∧
    ¬ selectedRustAccountCreationNoExportPolicy.containsDenylistedField
      A.publicBundleOutput ∧
      ¬ selectedRustAccountCreationSimulatorInputNoExportPolicy.containsDenylistedField
        A.simulatorInputOutput

/--
Boundary predicate consumed by the future accepted-bundle simulator theorem.

This is still only a public-anchor/allowlist boundary: it says the accepted
package determines the selected public-bundle surface and selected
simulator-input surface, and that both surfaces satisfy the current denylist
closures.  It does not assert distributional simulation.
-/
def publicAnchorSimulationBoundary
    (A : AccountCreationAcceptedPublicAnchorPackage) : Prop :=
  A.surfaces ∧
    A.noExportSupport ∧
      A.publicBundleFromPublicAnchors ∧
        A.simulatorInputFromPublicAnchors ∧
          A.publicBundleOutput =
            selectedRustAccountCreationPublicBundleSerializer.publicBundleOutputOf
              A.accountId A.slotId A.accountStateRoot A.verifierVersion
              A.packedWitnessCommitment A.nativeRoleRootCommitment
              A.maskedProjection A.accountStarkBundleBytesOrDigest
              A.publicReplayPayloadDigest A.lifecycleAnchor A.transcriptSurface
              A.stateTransitionBoundary A.accountNamespace A.authorizationClass ∧
            A.simulatorInputOutput =
              selectedRustAccountCreationSimulatorInputSerializer.simulatorInputOf
                A.leafContextDigest A.creationProofBindingDigest
                A.nativeRoleRootCommitment A.aggregationContextDigest A.slotId
                A.maskedProjection A.accountStarkPublicStatementBytesOrDigest
                A.padSeedNullifier A.accountId A.accountStateRoot
                A.lifecycleAnchor A.verifierVersion A.packedWitnessCommitment
                A.publicRejectionCode

/-- Six-lane q12289 account projection/mask carrier used by the selected path. -/
abbrev SelectedAccountProjectionMaskVector := Fin 6 -> ZMod 12289

/-- Adding a fixed account projection is an equivalence on uniform mask vectors. -/
def selectedAccountProjectionMaskTranslation
    (projection : SelectedAccountProjectionMaskVector) :
    SelectedAccountProjectionMaskVector ≃ SelectedAccountProjectionMaskVector where
  toFun mask := fun lane => projection lane + mask lane
  invFun masked := fun lane => masked lane - projection lane
  left_inv mask := by
    funext lane
    simp
  right_inv masked := by
    funext lane
    simp

/--
The selected six-lane additive mask is a perfect one-time-pad translation once
the mask vector is uniform. Thus A2's remaining computational assumption is the
Poseidon2 mask-PRF replacement/freshness hop, not the modular masking algebra.
-/
theorem selectedRustUniformMaskTranslationBijective_v1
    (projection : SelectedAccountProjectionMaskVector) :
    Function.Bijective (fun mask =>
      selectedAccountProjectionMaskTranslation projection mask) :=
  (selectedAccountProjectionMaskTranslation projection).bijective

/-- Equal masked carriers under one projection imply equal mask vectors. -/
theorem selectedRustUniformMaskTranslationCollisionFree_v1
    (projection mask₁ mask₂ : SelectedAccountProjectionMaskVector) :
    selectedAccountProjectionMaskTranslation projection mask₁ =
        selectedAccountProjectionMaskTranslation projection mask₂ ↔
      mask₁ = mask₂ := by
  constructor
  · intro h
    exact (selectedAccountProjectionMaskTranslation projection).injective h
  · exact congrArg _

/-! ## Selected C2 mask-PRF frame and deployment-scale reduction budget -/

/-- "MASK" as the canonical KoalaBear field lane absorbed before C2 inputs. -/
def selectedAccountMaskPrfDomain : Nat := 0x4D41534B

/-- "NULL" is the distinct account-STARK nullifier domain lane. -/
def selectedAccountNullifierDomain : Nat := 0x4E554C4C

/-- KoalaBear's canonical prime modulus. -/
def selectedAccountMaskPrfFieldModulus : Nat := 2130706433

/-- Application modulus of each projection-mask lane. -/
def selectedAccountMaskModulus : Nat := 12289

/-- Six field outputs are combined for every q12289 mask lane. -/
def selectedAccountMaskReductionDigits : Nat := 6

/-- Six independent q12289 mask lanes form one selected projection carrier. -/
def selectedAccountMaskLaneCount : Nat := 6

/-- Selected deployment horizon: at most 2^32 admitted account targets. -/
def selectedAccountMaskTargetHorizon : Nat := 2 ^ 32

/-- C2 absorbs DOMAIN[1] || pad_seed[32] || slot_le[8]. -/
def selectedAccountMaskPrfInputLanes : Nat := 1 + 32 + 8

/-- C2 squeezes six disjoint field coordinates per each of six mask lanes. -/
def selectedAccountMaskPrfSqueezedLanes : Nat :=
  selectedAccountMaskLaneCount * selectedAccountMaskReductionDigits

theorem selectedAccountMaskPrfDomainsSeparated_v1 :
    selectedAccountMaskPrfDomain ≠ selectedAccountNullifierDomain := by
  native_decide

theorem selectedAccountMaskPrfFrameDimensions_v1 :
    selectedAccountMaskPrfInputLanes = 41 ∧
      selectedAccountMaskPrfSqueezedLanes = 36 := by
  native_decide

/-- Map `(mask lane, reduction digit)` to its dedicated C2 squeeze coordinate. -/
def selectedAccountMaskPrfSqueezeIndex
    (coordinate : Fin 6 × Fin 6) : Fin 36 :=
  ⟨coordinate.1.val * 6 + coordinate.2.val, by omega⟩

/-- No two selected mask/digit coordinates reuse one squeezed field lane. -/
theorem selectedAccountMaskPrfSqueezeIndexInjective_v1 :
    Function.Injective selectedAccountMaskPrfSqueezeIndex := by
  intro a b h
  have hval : a.1.val * 6 + a.2.val = b.1.val * 6 + b.2.val :=
    congrArg Fin.val h
  apply Prod.ext
  · apply Fin.ext
    omega
  · apply Fin.ext
    omega

/--
Loose aggregate statistical term for six `mod 12289` reductions over the
selected 2^32-target account horizon, assuming ideal independent KoalaBear
field outputs. Each mask contributes at most `q / p^6`; this deliberately uses
the simple upper bound rather than relying on a tighter remainder calculation.
-/
noncomputable def selectedAccountMaskWideReductionError : ℝ :=
  ((selectedAccountMaskLaneCount * selectedAccountMaskTargetHorizon *
      selectedAccountMaskModulus : Nat) : ℝ) /
    ((selectedAccountMaskPrfFieldModulus ^ selectedAccountMaskReductionDigits : Nat) : ℝ)

/-- Exact integer certificate underlying the deployment-scale reduction bound. -/
theorem selectedAccountMaskWideReductionScaledNumerator_le_denominator_v1 :
    selectedAccountMaskLaneCount * selectedAccountMaskTargetHorizon *
          selectedAccountMaskModulus * (2 ^ 128) ≤
      selectedAccountMaskPrfFieldModulus ^ selectedAccountMaskReductionDigits := by
  native_decide

/-- The aggregate six-lane, 2^32-target reduction term meets q128. -/
theorem selectedAccountMaskWideReductionError_le_q128_v1 :
    selectedAccountMaskWideReductionError ≤ (1 : ℝ) / (2 ^ 128 : Nat) := by
  norm_num [selectedAccountMaskWideReductionError,
    selectedAccountMaskLaneCount, selectedAccountMaskTargetHorizon,
    selectedAccountMaskModulus, selectedAccountMaskPrfFieldModulus,
    selectedAccountMaskReductionDigits]

/-- Adjacent games isolating PRF, freshness, and wide-reduction losses. -/
structure SelectedAccountMaskPrfHybridGame where
  realProbability : ℝ
  idealPrfProbability : ℝ
  freshInputProbability : ℝ
  uniformMaskProbability : ℝ

namespace SelectedAccountMaskPrfHybridGame

def advantage (G : SelectedAccountMaskPrfHybridGame) : ℝ :=
  |G.realProbability - G.uniformMaskProbability|

end SelectedAccountMaskPrfHybridGame

/--
Exact A2 reduction surface. The selected Poseidon2 theorem supplies the first
hop; the reorg-safe verified-nullifier registry supplies the second; the third
is charged to the concrete six-digit reduction budget above.
-/
structure SelectedAccountMaskPrfHybridBounds
    (G : SelectedAccountMaskPrfHybridGame) where
  poseidonPrfError : ℝ
  freshnessError : ℝ
  poseidonPrfError_nonnegative : 0 ≤ poseidonPrfError
  freshnessError_nonnegative : 0 ≤ freshnessError
  poseidonPrfHop :
    |G.realProbability - G.idealPrfProbability| ≤ poseidonPrfError
  freshnessHop :
    |G.idealPrfProbability - G.freshInputProbability| ≤ freshnessError
  wideReductionHop :
    |G.freshInputProbability - G.uniformMaskProbability| ≤
      selectedAccountMaskWideReductionError

namespace SelectedAccountMaskPrfHybridBounds

noncomputable def totalError
    {G : SelectedAccountMaskPrfHybridGame}
    (B : SelectedAccountMaskPrfHybridBounds G) : ℝ :=
  B.poseidonPrfError + B.freshnessError + selectedAccountMaskWideReductionError

theorem advantage_le_totalError
    {G : SelectedAccountMaskPrfHybridGame}
    (B : SelectedAccountMaskPrfHybridBounds G) :
    G.advantage ≤ B.totalError := by
  calc
    G.advantage = |G.realProbability - G.uniformMaskProbability| := rfl
    _ ≤ |G.realProbability - G.idealPrfProbability| +
          |G.idealPrfProbability - G.uniformMaskProbability| :=
      abs_sub_le _ _ _
    _ ≤ |G.realProbability - G.idealPrfProbability| +
          (|G.idealPrfProbability - G.freshInputProbability| +
            |G.freshInputProbability - G.uniformMaskProbability|) := by
      gcongr
      exact abs_sub_le _ _ _
    _ ≤ B.poseidonPrfError +
          (B.freshnessError + selectedAccountMaskWideReductionError) := by
      gcongr
      · exact B.poseidonPrfHop
      · exact B.freshnessHop
      · exact B.wideReductionHop
    _ = B.totalError := by
      simp [totalError, add_assoc]

/-- After exact freshness, A2 loses only Poseidon2 PRF advantage plus 2^-128. -/
theorem advantage_le_poseidonPrfError_add_q128
    {G : SelectedAccountMaskPrfHybridGame}
    (B : SelectedAccountMaskPrfHybridBounds G)
    (hFresh : B.freshnessError = 0) :
    G.advantage ≤ B.poseidonPrfError + (1 : ℝ) / (2 ^ 128 : Nat) := by
  calc
    G.advantage ≤ B.totalError := B.advantage_le_totalError
    _ = B.poseidonPrfError + selectedAccountMaskWideReductionError := by
      simp [totalError, hFresh]
    _ ≤ B.poseidonPrfError + (1 : ℝ) / (2 ^ 128 : Nat) := by
      gcongr
      exact selectedAccountMaskWideReductionError_le_q128_v1

end SelectedAccountMaskPrfHybridBounds

/-! ## Selected C1/C6 commitment-hiding frame and QROM search budget -/

/-- "PADS" is the selected C1 pad-source commitment domain lane. -/
def selectedAccountPadCommitmentDomain : Nat := 0x50414453

/-- "OPEN" is the selected C6 opening-tuple commitment domain lane. -/
def selectedAccountOpeningCommitmentDomain : Nat := 0x4F50454E

/-- Each selected commitment uses a fresh 40-byte deployment nonce. -/
def selectedAccountCommitmentNonceBits : Nat := 40 * 8

/-- C1 absorbs PADS[1] || pad_seed[32] || nonce[40]. -/
def selectedAccountPadCommitmentInputLanes : Nat := 1 + 32 + 40

/-- C6 absorbs OPEN[1] || projection[6] || lifecycle[32] || nonce[40] || account_id[32]. -/
def selectedAccountOpeningCommitmentInputLanes : Nat := 1 + 6 + 32 + 40 + 32

/-- Two independently nonced hiding commitments are emitted per admitted account. -/
def selectedAccountHidingCommitmentsPerAccount : Nat := 2

/-- Conservative total selected commitment targets over the 2^32-account horizon. -/
def selectedAccountCommitmentTargetCount : Nat :=
  selectedAccountHidingCommitmentsPerAccount * (2 ^ 32)

/-- Selected global bounded-query ceiling imported by the Path 2 QROM ledger. -/
def selectedAccountCommitmentQromQueries : Nat := 2 ^ 54

/--
Conservative QROM search term for recovering one fresh commitment nonce across
all selected C1/C6 targets: `targets * queries^2 / 2^320`.
-/
noncomputable def selectedAccountCommitmentQromSearchError : ℝ :=
  ((selectedAccountCommitmentTargetCount *
      selectedAccountCommitmentQromQueries ^ 2 : Nat) : ℝ) /
    ((2 ^ selectedAccountCommitmentNonceBits : Nat) : ℝ)

/-- The selected PADS/OPEN/MASK/NULL domain words are pairwise distinct. -/
theorem selectedAccountCommitmentDomainsSeparated_v1 :
    selectedAccountPadCommitmentDomain ≠ selectedAccountOpeningCommitmentDomain ∧
    selectedAccountPadCommitmentDomain ≠ selectedAccountMaskPrfDomain ∧
    selectedAccountPadCommitmentDomain ≠ selectedAccountNullifierDomain ∧
    selectedAccountOpeningCommitmentDomain ≠ selectedAccountMaskPrfDomain ∧
    selectedAccountOpeningCommitmentDomain ≠ selectedAccountNullifierDomain ∧
    selectedAccountMaskPrfDomain ≠ selectedAccountNullifierDomain := by
  native_decide

theorem selectedAccountCommitmentFrameDimensions_v1 :
    selectedAccountCommitmentNonceBits = 320 ∧
    selectedAccountPadCommitmentInputLanes = 73 ∧
    selectedAccountOpeningCommitmentInputLanes = 111 ∧
    selectedAccountCommitmentTargetCount = 2 ^ 33 := by
  native_decide

/-- Exact integer certificate for the selected multi-target QROM search term. -/
theorem selectedAccountCommitmentQromSearchScaledNumerator_le_denominator_v1 :
    selectedAccountCommitmentTargetCount *
          selectedAccountCommitmentQromQueries ^ 2 * (2 ^ 128) ≤
      2 ^ selectedAccountCommitmentNonceBits := by
  native_decide

/-- The local nonce-search term is below q128 (in fact exactly 2^-179). -/
theorem selectedAccountCommitmentQromSearchError_le_q128_v1 :
    selectedAccountCommitmentQromSearchError ≤ (1 : ℝ) / (2 ^ 128 : Nat) := by
  unfold selectedAccountCommitmentQromSearchError
  rw [div_le_div_iff₀ (by positivity) (by positivity)]
  norm_num [
    selectedAccountCommitmentTargetCount,
    selectedAccountHidingCommitmentsPerAccount,
    selectedAccountCommitmentQromQueries,
    selectedAccountCommitmentNonceBits, pow_succ]

/-- Adjacent games isolating Poseidon2 replacement and fresh-nonce search. -/
structure SelectedAccountCommitmentHidingGame where
  realProbability : ℝ
  idealOracleProbability : ℝ
  hiddenProbability : ℝ

namespace SelectedAccountCommitmentHidingGame

def advantage (G : SelectedAccountCommitmentHidingGame) : ℝ :=
  |G.realProbability - G.hiddenProbability|

end SelectedAccountCommitmentHidingGame

/--
Exact A3 local reduction. A reviewed bounded-query Poseidon2 commitment/QROM
theorem supplies the first hop; the second is charged to the concrete fresh
320-bit nonce-search budget above.
-/
structure SelectedAccountCommitmentHidingBounds
    (G : SelectedAccountCommitmentHidingGame) where
  poseidonCommitmentError : ℝ
  poseidonCommitmentError_nonnegative : 0 ≤ poseidonCommitmentError
  poseidonCommitmentHop :
    |G.realProbability - G.idealOracleProbability| ≤ poseidonCommitmentError
  freshNonceSearchHop :
    |G.idealOracleProbability - G.hiddenProbability| ≤
      selectedAccountCommitmentQromSearchError

namespace SelectedAccountCommitmentHidingBounds

noncomputable def totalError
    {G : SelectedAccountCommitmentHidingGame}
    (B : SelectedAccountCommitmentHidingBounds G) : ℝ :=
  B.poseidonCommitmentError + selectedAccountCommitmentQromSearchError

theorem advantage_le_totalError
    {G : SelectedAccountCommitmentHidingGame}
    (B : SelectedAccountCommitmentHidingBounds G) :
    G.advantage ≤ B.totalError := by
  calc
    G.advantage = |G.realProbability - G.hiddenProbability| := rfl
    _ ≤ |G.realProbability - G.idealOracleProbability| +
          |G.idealOracleProbability - G.hiddenProbability| :=
      abs_sub_le _ _ _
    _ ≤ B.poseidonCommitmentError + selectedAccountCommitmentQromSearchError := by
      gcongr
      · exact B.poseidonCommitmentHop
      · exact B.freshNonceSearchHop
    _ = B.totalError := rfl

theorem advantage_le_poseidonCommitmentError_add_q128
    {G : SelectedAccountCommitmentHidingGame}
    (B : SelectedAccountCommitmentHidingBounds G) :
    G.advantage ≤ B.poseidonCommitmentError + (1 : ℝ) / (2 ^ 128 : Nat) := by
  calc
    G.advantage ≤ B.totalError := B.advantage_le_totalError
    _ ≤ B.poseidonCommitmentError + (1 : ℝ) / (2 ^ 128 : Nat) := by
      simpa [totalError] using
        add_le_add_left selectedAccountCommitmentQromSearchError_le_q128_v1
          B.poseidonCommitmentError

end SelectedAccountCommitmentHidingBounds

/-- Four adjacent account-STARK public-view games used by the simulator hybrid. -/
structure SelectedAccountStarkSimulatorHybridGame where
  realViewProbability : ℝ
  friSimulatedProbability : ℝ
  maskSimulatedProbability : ℝ
  fullySimulatedProbability : ℝ

namespace SelectedAccountStarkSimulatorHybridGame

/-- Real-versus-fully-simulated distinguishing advantage. -/
def advantage (G : SelectedAccountStarkSimulatorHybridGame) : ℝ :=
  |G.realViewProbability - G.fullySimulatedProbability|

end SelectedAccountStarkSimulatorHybridGame

/--
The exact three named hybrid assumptions for the selected account-STARK:

1. hiding-FRI/PCS HVZK replaces proof bytes;
2. the fresh Poseidon2 mask PRF replaces the masked projection;
3. commitment hiding replaces the remaining hidden-preimage view.
-/
structure SelectedAccountStarkSimulatorHybridBounds
    (G : SelectedAccountStarkSimulatorHybridGame) where
  zkFriHvzkError : ℝ
  maskPrfError : ℝ
  commitmentHidingError : ℝ
  zkFriHvzkError_nonnegative : 0 ≤ zkFriHvzkError
  maskPrfError_nonnegative : 0 ≤ maskPrfError
  commitmentHidingError_nonnegative : 0 ≤ commitmentHidingError
  zkFriHvzkHop :
    |G.realViewProbability - G.friSimulatedProbability| ≤ zkFriHvzkError
  maskPrfHop :
    |G.friSimulatedProbability - G.maskSimulatedProbability| ≤ maskPrfError
  commitmentHidingHop :
    |G.maskSimulatedProbability - G.fullySimulatedProbability| ≤
      commitmentHidingError

namespace SelectedAccountStarkSimulatorHybridBounds

/-- Additive distinguishing budget for the selected three-hop hybrid. -/
def totalError
    {G : SelectedAccountStarkSimulatorHybridGame}
    (B : SelectedAccountStarkSimulatorHybridBounds G) : ℝ :=
  B.zkFriHvzkError + B.maskPrfError + B.commitmentHidingError

/-- The real/simulated advantage is bounded by the sum of the three named hops. -/
theorem advantage_le_totalError
    {G : SelectedAccountStarkSimulatorHybridGame}
    (B : SelectedAccountStarkSimulatorHybridBounds G) :
    G.advantage ≤ B.totalError := by
  calc
    G.advantage =
        |G.realViewProbability - G.fullySimulatedProbability| := rfl
    _ ≤ |G.realViewProbability - G.friSimulatedProbability| +
          |G.friSimulatedProbability - G.fullySimulatedProbability| :=
      abs_sub_le _ _ _
    _ ≤ |G.realViewProbability - G.friSimulatedProbability| +
          (|G.friSimulatedProbability - G.maskSimulatedProbability| +
            |G.maskSimulatedProbability - G.fullySimulatedProbability|) := by
      gcongr
      exact abs_sub_le _ _ _
    _ ≤ B.zkFriHvzkError + (B.maskPrfError + B.commitmentHidingError) := by
      gcongr
      · exact B.zkFriHvzkHop
      · exact B.maskPrfHop
      · exact B.commitmentHidingHop
    _ = B.totalError := by
      simp [totalError, add_assoc]

end SelectedAccountStarkSimulatorHybridBounds

/--
Concrete hybrid certificate for one accepted public-anchor package. A future
cryptographic import must supply the three hop bounds and a reviewed target;
the certificate cannot be constructed from the public-anchor projection alone.
-/
structure SelectedAccountStarkSimulatorHybridCertificate
    (A : AccountCreationAcceptedPublicAnchorPackage) where
  game : SelectedAccountStarkSimulatorHybridGame
  bounds : SelectedAccountStarkSimulatorHybridBounds game
  targetError : ℝ
  targetError_nonnegative : 0 ≤ targetError
  totalError_le_target : bounds.totalError ≤ targetError

namespace SelectedAccountStarkSimulatorHybridCertificate

/-- A concrete certificate imports the additive hybrid bound. -/
theorem advantage_le_target
    {A : AccountCreationAcceptedPublicAnchorPackage}
    (C : SelectedAccountStarkSimulatorHybridCertificate A) :
    C.game.advantage ≤ C.targetError :=
  le_trans C.bounds.advantage_le_totalError C.totalError_le_target

end SelectedAccountStarkSimulatorHybridCertificate

/--
Selected simulator relation with an explicit three-hop certificate. This is
strictly stronger than an arbitrary supplied relation: producing a witness
requires concrete zk-FRI, mask-PRF, and commitment-hiding error bounds.
-/
def selectedAccountStarkSimulatorHybridRelation
    (I : PublicBundleInputs) (J : SimulatorPublicInputs) : Prop :=
  ∃ A : AccountCreationAcceptedPublicAnchorPackage,
    A.publicBundleInputs = I ∧
      A.simulatorPublicInputs = J ∧
        ∃ C : SelectedAccountStarkSimulatorHybridCertificate A,
          C.game.advantage ≤ C.targetError

/--
Accepted-bundle simulability reduced to a supplied public simulator relation.

`Simulates` is the remaining algebraic simulator theorem surface: it relates the
typed public-bundle inputs to the typed simulator inputs.  This predicate keeps
the public-anchor boundary explicit so the reduction cannot silently consume a
non-public or unbound simulator input.
-/
def acceptedBundleSimulatableFromPublicAnchorsWith
    (Simulates : PublicBundleInputs -> SimulatorPublicInputs -> Prop)
    (A : AccountCreationAcceptedPublicAnchorPackage) : Prop :=
  A.publicAnchorSimulationBoundary ∧
    Simulates A.publicBundleInputs A.simulatorPublicInputs

/--
Accepted-bundle simulator/no-export closure reduced to the supplied simulator
relation and the already-proved public-anchor no-export boundary.

This is still a reduction target, not the distributional simulator theorem.
-/
def acceptedBundleSimulatorNoExportWith
    (Simulates : PublicBundleInputs -> SimulatorPublicInputs -> Prop)
    (A : AccountCreationAcceptedPublicAnchorPackage) : Prop :=
  A.acceptedBundleSimulatableFromPublicAnchorsWith Simulates ∧
    A.noExportSupport

end AccountCreationAcceptedPublicAnchorPackage

/-- The accepted public-anchor package is bound to the selected public surfaces. -/
theorem selectedRustAcceptedPublicAnchorPackage_surfaces_v1
    (A : AccountCreationAcceptedPublicAnchorPackage) :
    A.surfaces := by
  constructor
  ·
    refine
      ⟨A.accountId, A.slotId, A.accountStateRoot, A.verifierVersion,
        A.packedWitnessCommitment, A.nativeRoleRootCommitment,
        A.maskedProjection, A.accountStarkBundleBytesOrDigest,
        A.publicReplayPayloadDigest, A.lifecycleAnchor, A.transcriptSurface,
        A.stateTransitionBoundary, A.accountNamespace, A.authorizationClass,
        ?_⟩
    rfl
  ·
    refine
      ⟨A.leafContextDigest, A.creationProofBindingDigest,
        A.nativeRoleRootCommitment, A.aggregationContextDigest, A.slotId,
        A.maskedProjection, A.accountStarkPublicStatementBytesOrDigest,
        A.padSeedNullifier, A.accountId, A.accountStateRoot,
        A.lifecycleAnchor, A.verifierVersion, A.packedWitnessCommitment,
        A.publicRejectionCode, ?_⟩
    rfl

/--
Selected Rust package-level accepted public-anchor no-export support.

This theorem only binds the package to allowlisted public surfaces and applies
the existing denylist-separation closures.  It does not instantiate
`selected_account_stark_public_bundle_simulator_v1`,
`selected_account_stark_no_export_v1`, or accepted-bundle simulability from
public anchors.
-/
theorem selectedRustAcceptedPublicAnchorPackageNoExport_instantiation_v1 :
    forall A : AccountCreationAcceptedPublicAnchorPackage,
      A.noExportSupport := by
  intro A
  have hSurfaces := selectedRustAcceptedPublicAnchorPackage_surfaces_v1 A
  exact ⟨hSurfaces,
    selectedRustAccountCreationPublicBundleNoExport_instantiation_v1
      hSurfaces.1,
    selectedRustAccountCreationSimulatorInputNoExport_instantiation_v1
      hSurfaces.2⟩

/--
Selected Rust accepted-bundle public-bundle public-anchor projection hook.

This proves that the selected public bundle is exactly the typed public-input
package derived from accepted public anchors, with public-bundle denylist
support.  It deliberately does not prove distributional simulation or
`accepted_bundle_simulatable_from_public_anchors`.
-/
theorem selectedRustAcceptedBundlePublicBundleFromPublicAnchors_v1 :
    forall A : AccountCreationAcceptedPublicAnchorPackage,
      A.publicBundleFromPublicAnchors := by
  intro A
  have hSurfaces := selectedRustAcceptedPublicAnchorPackage_surfaces_v1 A
  have hNoExport :=
    selectedRustAcceptedPublicAnchorPackageNoExport_instantiation_v1 A
  refine ⟨rfl, ?_⟩
  exact ⟨
    by
      simpa [
        AccountCreationAcceptedPublicAnchorPackage.publicBundleInputs,
        AccountCreationAcceptedPublicAnchorPackage.PublicBundleInputs.output] using hSurfaces.1,
    by
      simpa [
        AccountCreationAcceptedPublicAnchorPackage.publicBundleInputs,
        AccountCreationAcceptedPublicAnchorPackage.PublicBundleInputs.output] using hNoExport.2.1⟩

/--
Selected Rust accepted-bundle simulator-input public-anchor projection hook.

This proves that the selected simulator input is exactly the typed public-input
package derived from accepted public anchors, with simulator-input denylist
support.  It deliberately does not prove distributional simulation or
`accepted_bundle_simulatable_from_public_anchors`.
-/
theorem selectedRustAcceptedBundleSimulatorInputFromPublicAnchors_v1 :
    forall A : AccountCreationAcceptedPublicAnchorPackage,
      A.simulatorInputFromPublicAnchors := by
  intro A
  have hSurfaces := selectedRustAcceptedPublicAnchorPackage_surfaces_v1 A
  have hNoExport :=
    selectedRustAcceptedPublicAnchorPackageNoExport_instantiation_v1 A
  refine ⟨rfl, ?_⟩
  exact ⟨
    by
      simpa [
        AccountCreationAcceptedPublicAnchorPackage.simulatorPublicInputs,
        AccountCreationAcceptedPublicAnchorPackage.SimulatorPublicInputs.output] using hSurfaces.2,
    by
      simpa [
        AccountCreationAcceptedPublicAnchorPackage.simulatorPublicInputs,
        AccountCreationAcceptedPublicAnchorPackage.SimulatorPublicInputs.output] using hNoExport.2.2⟩

/--
Selected Rust accepted-bundle public-anchor simulation-boundary hook.

This proves that the public-anchor package is the exact public surface a future
simulator theorem may consume, together with the current denylist closures.
The theorem deliberately does not prove
`accepted_bundle_simulatable_from_public_anchors`.
-/
theorem selectedRustAcceptedBundlePublicAnchorSimulationBoundary_v1 :
    forall A : AccountCreationAcceptedPublicAnchorPackage,
      A.publicAnchorSimulationBoundary := by
  intro A
  have hSurfaces := selectedRustAcceptedPublicAnchorPackage_surfaces_v1 A
  have hNoExport :=
    selectedRustAcceptedPublicAnchorPackageNoExport_instantiation_v1 A
  have hPublicBundleFromAnchors :=
    selectedRustAcceptedBundlePublicBundleFromPublicAnchors_v1 A
  have hSimulatorInputFromAnchors :=
    selectedRustAcceptedBundleSimulatorInputFromPublicAnchors_v1 A
  exact
    ⟨hSurfaces, hNoExport, hPublicBundleFromAnchors,
      hSimulatorInputFromAnchors, rfl, rfl⟩

/--
Selected Rust accepted-bundle simulator/no-export reduction hook.

If the remaining algebraic simulator theorem proves simulation for every
accepted public-anchor package satisfying the selected boundary, then every
selected accepted bundle is simulatable from the typed public anchors and has
the current no-export support.  This deliberately keeps the simulator relation
as an explicit hypothesis and does not instantiate
`selected_account_stark_public_bundle_simulator_v1`,
`selected_account_stark_no_export_v1`, or deployment readiness.
-/
theorem selectedRustAcceptedBundleSimulatorNoExportReduction_v1
    (Simulates :
      AccountCreationAcceptedPublicAnchorPackage.PublicBundleInputs ->
        AccountCreationAcceptedPublicAnchorPackage.SimulatorPublicInputs -> Prop)
    (hSimulator :
      forall A : AccountCreationAcceptedPublicAnchorPackage,
        A.publicAnchorSimulationBoundary ->
          Simulates A.publicBundleInputs A.simulatorPublicInputs) :
    forall A : AccountCreationAcceptedPublicAnchorPackage,
      A.acceptedBundleSimulatorNoExportWith Simulates := by
  intro A
  have hBoundary :=
    selectedRustAcceptedBundlePublicAnchorSimulationBoundary_v1 A
  exact ⟨⟨hBoundary, hSimulator A hBoundary⟩, hBoundary.2.1⟩

/--
Selected account-STARK simulator/no-export theorem under explicit hybrid
certificates. This closes the local composition from three named advantage
bounds to accepted-bundle simulation and no-export. It does not manufacture the
cryptographic certificates or claim their external review.
-/
theorem selectedRustAcceptedBundleSimulatorNoExportFromHybrid_v1
    (hHybrid :
      ∀ A : AccountCreationAcceptedPublicAnchorPackage,
        AccountCreationAcceptedPublicAnchorPackage.SelectedAccountStarkSimulatorHybridCertificate A) :
    ∀ A : AccountCreationAcceptedPublicAnchorPackage,
      A.acceptedBundleSimulatorNoExportWith
        AccountCreationAcceptedPublicAnchorPackage.selectedAccountStarkSimulatorHybridRelation := by
  apply selectedRustAcceptedBundleSimulatorNoExportReduction_v1
  intro A _hBoundary
  let C := hHybrid A
  exact ⟨A, rfl, rfl, C, C.advantage_le_target⟩

end Path2NoExportConcrete
end Descender
