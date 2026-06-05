import Descender.Path2NoExport

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

end Path2NoExportConcrete
end Descender
