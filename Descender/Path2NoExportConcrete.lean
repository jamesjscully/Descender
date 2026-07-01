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

end Path2NoExportConcrete
end Descender
