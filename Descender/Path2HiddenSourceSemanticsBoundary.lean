import Descender.Path2NativeAccountDesign
import Descender.Path2FullOpeningFieldEvidenceSourceAdapters

namespace Descender

/-!
# Path 2 Hidden-Source Semantics Boundary

This module connects the active reviewed-Fq12 extraction-certificate theorem to
the full-signature boundary field `hiddenSourceProofSemanticsClosed`.

The proof remains parameterized by the concrete Rust/model source records.  It
does not close deployment by itself; it names the exact proposition that the
accepted-spend boundary should carry once those active records are supplied.
-/

namespace Path2HiddenSourceSemanticsBoundary

open Path2NativeAccountDesign
open Path2NativeBoundary
open Path2FullOpeningFieldEvidenceByteBridge
open Path2FullOpeningSourceConcrete
open Path2CompatibilityConcrete
open Path2FullOpeningFieldEvidenceSourceAdapters

/--
The active hidden-source semantics closure proposition for reviewed-Fq12.

The accepted-spend boundary only stores a `Prop`, so we package the existing
certificate object as an existential proposition.  The certificate still
contains the payload projection law and payload soundness theorem used by the
native boundary layer.
-/
def ReviewedFq12HiddenSourceProofSemanticsClosed
    (P : Predicates) : Prop :=
  exists _C : ReviewedFq12HiddenSourceSemanticsCertificate P, True

/--
Use a reviewed-Fq12 hidden-source closure proposition as the
`hiddenSourceProofSemanticsClosed` field of an accepted-spend boundary.
-/
def NativeAcceptedSpendSoundnessBoundary.withReviewedFq12HiddenSourceClosure
    (B : NativeAcceptedSpendSoundnessBoundary)
    (P : Predicates) :
    NativeAcceptedSpendSoundnessBoundary :=
  { B with
    hiddenSourceProofSemanticsClosed :=
      ReviewedFq12HiddenSourceProofSemanticsClosed P }

/-- The wrapped accepted-spend boundary exposes exactly the reviewed-Fq12 closure. -/
theorem with_reviewed_fq12_hidden_source_closure_closed
    (B : NativeAcceptedSpendSoundnessBoundary)
    {P : Predicates}
    (Closed : ReviewedFq12HiddenSourceProofSemanticsClosed P) :
    (NativeAcceptedSpendSoundnessBoundary.withReviewedFq12HiddenSourceClosure
      B P).hiddenSourceProofSemanticsClosed :=
  Closed

/--
Certificate availability plus extraction-certificate soundness closes the
hidden-source semantics proposition.
-/
theorem reviewed_fq12_hidden_source_proof_semantics_closed_from_extraction_soundness
    (P : Predicates)
    (payloadProjectsFrom :
      forall {b : Path2NativeBoundary.Boundary}
          {payload : ReviewedFq12HiddenSourcePayload},
        P.reviewedFq12PayloadAccepts b payload ->
          payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
            payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload)
    (certificateAvailability :
      ReviewedFq12ExtractionCertificateAvailability P)
    (certificateSoundness :
      ReviewedFq12ExtractionCertificateSoundness P) :
    ReviewedFq12HiddenSourceProofSemanticsClosed P :=
  let C : ReviewedFq12HiddenSourceSemanticsCertificate P :=
    { payloadProjectsFrom := payloadProjectsFrom
      payloadSoundness :=
        reviewed_fq12_payload_soundness_from_extraction_certificate
          P certificateAvailability certificateSoundness }
  ⟨C, trivial⟩

/--
Active AP2-N001 boundary closure from the raw selected Rust/model source
components.

This theorem is the Lean-side target corresponding to the Rust
`active_field_local_digest_frame_witness_byte_component_surfaces_present` gate.
Supplying these premises gives the `hiddenSourceProofSemanticsClosed`
proposition; the Rust deployment audit must still remain fail-closed until the
premises are instantiated for the concrete implementation.
-/
theorem reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_digest_frame_witness_byte_components
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (payloadProjectsFrom :
      forall {b : Path2NativeBoundary.Boundary}
          {payload : ReviewedFq12HiddenSourcePayload},
        P.reviewedFq12PayloadAccepts b payload ->
          payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
            payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload)
    (certificateAvailability :
      ReviewedFq12ExtractionCertificateAvailability P)
    (roleRootFields :
      ReviewedFq12ConcreteRoleRootFieldBindingCertificate P)
    (liftOpeningFunctionality :
      ReviewedFq12CertificateLiftOpeningFunctionality P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (assemblyAvailability :
      ReviewedFq12RepresentativeAssemblyCertificateAvailability P)
    (assemblyFieldBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (assemblyLocalPredicates :
      ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak P)
    (assemblyNativeValidity :
      ReviewedFq12AssemblyConstructedNativeValidityOrBreak P)
    (compatibilityAvailability :
      ReviewedFq12ExtractionCompatibilityCertificateAvailability P)
    (compatibilityFieldBinding :
      ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak P)
    (compatibilityExtractionRules :
      SameRepresentativeConcreteExtractionPredicates P)
    (digestFrame :
      ReviewedFq12NativeRepresentativeDigestFrame)
    (canonicalRepresentativeWitnessByteSource :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource
        CanonicalParser P)
    (canonical_digest_frame_eq :
      canonicalRepresentativeWitnessByteSource.digestFrame = digestFrame)
    (nativeDigestFrameWitnessByteSource :
      ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
        WitnessParser P)
    (assembly_digest_frame_eq :
      nativeDigestFrameWitnessByteSource.digestFrame = digestFrame) :
    ReviewedFq12HiddenSourceProofSemanticsClosed P :=
  reviewed_fq12_hidden_source_proof_semantics_closed_from_extraction_soundness
    P payloadProjectsFrom certificateAvailability
    (reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components
      P roleRootFields liftOpeningFunctionality selectedSource
      assemblyAvailability assemblyFieldBinding assemblyLocalPredicates
      assemblyNativeValidity compatibilityAvailability compatibilityFieldBinding
      compatibilityExtractionRules digestFrame
      canonicalRepresentativeWitnessByteSource canonical_digest_frame_eq
      nativeDigestFrameWitnessByteSource assembly_digest_frame_eq)

/--
Active AP2-N001 boundary closure from the named raw selected Rust/model source
component package.

This is the certificate-shaped version of
`reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_digest_frame_witness_byte_components`.
It keeps the production boundary aligned with the single Rust audit gate that
must eventually flip `extractor_theorem_instantiated`.
-/
theorem reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_digest_frame_witness_byte_component_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (payloadProjectsFrom :
      forall {b : Path2NativeBoundary.Boundary}
          {payload : ReviewedFq12HiddenSourcePayload},
        P.reviewedFq12PayloadAccepts b payload ->
          payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
            payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload)
    (certificateAvailability :
      ReviewedFq12ExtractionCertificateAvailability P)
    (I :
      ReviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputs
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12HiddenSourceProofSemanticsClosed P :=
  reviewed_fq12_hidden_source_proof_semantics_closed_from_extraction_soundness
    P payloadProjectsFrom certificateAvailability
    (reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_component_inputs
      P I)

/--
Single record for the concrete model inputs needed by the active five-source
hidden-source boundary route.

This is intentionally not the final Rust/model instantiation theorem.  It is a
target shape for that theorem: once the implementation supplies this record,
the boundary closes by the existing five-source route.
-/
structure ReviewedFq12SelectedRustModelFiveSourceClosureInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  payloadProjectsFrom :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload},
      P.reviewedFq12PayloadAccepts b payload ->
        payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
          payload.slotId = b.context.slotId ->
        P.reviewedFq12PayloadProjectsFrom b payload
  certificateAvailability :
    ReviewedFq12ExtractionCertificateAvailability P
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  assemblySource :
    ReviewedFq12FieldConcreteAssemblyLocalSource P
  compatibilitySource :
    ReviewedFq12FieldConcreteCompatibilitySource P
  witnessSources :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
      CanonicalParser WitnessParser P

/--
Lean mirror of the public five-source closure evidence report.

The final extractor theorem should consume proofs tied to this report shape,
not arbitrary propositions. The status bits are recorded with the report but
are intentionally not part of `FactsAllTrue`: they describe whether the final
extractor theorem has been instantiated, while the readiness facts are the
evidence consumed by that future theorem.
-/
structure ReviewedFq12SelectedRustFiveSourceClosureEvidence where
  schemaMatches : Bool
  bridgeAuditDigest : ByteArray
  bridgeAuditDigestBound : Bool
  reportDigestVerified : Bool
  bridgeAuditValidated : Bool
  selectedRustModelFiveSourceInputsReady : Bool
  rustPayloadProjectionLawPresent : Bool
  rustExtractionCertificateAvailabilityPresent : Bool
  roleRootLiftSourcePresent : Bool
  selectedSourceRouteByteSourcePresent : Bool
  assemblyFieldLocalNativeComponentsPresent : Bool
  compatibilityFieldExtractionComponentsPresent : Bool
  sharedDigestFrameWitnessByteSourceContractPresent : Bool
  activeFieldLocalDigestFrameWitnessByteComponentSurfacesPresent : Bool
  leanActiveFiveSourceBoundaryRoutePresent : Bool
  leanFullSignatureHiddenSourceHandoffPresent : Bool
  extractorTheoremInstantiated : Bool
  semanticsBridgeComplete : Bool

/-- The report has been validated against the public Rust evidence surface. -/
def ReviewedFq12SelectedRustFiveSourceClosureEvidence.ReportVerified
    (E : ReviewedFq12SelectedRustFiveSourceClosureEvidence) : Prop :=
  E.schemaMatches = true ∧
  E.bridgeAuditDigestBound = true ∧
  E.reportDigestVerified = true ∧
  E.bridgeAuditValidated = true

/-- All non-closing five-source readiness facts recorded by the report hold. -/
def ReviewedFq12SelectedRustFiveSourceClosureEvidence.FactsAllTrue
    (E : ReviewedFq12SelectedRustFiveSourceClosureEvidence) : Prop :=
  E.selectedRustModelFiveSourceInputsReady = true ∧
  E.rustPayloadProjectionLawPresent = true ∧
  E.rustExtractionCertificateAvailabilityPresent = true ∧
  E.roleRootLiftSourcePresent = true ∧
  E.selectedSourceRouteByteSourcePresent = true ∧
  E.assemblyFieldLocalNativeComponentsPresent = true ∧
  E.compatibilityFieldExtractionComponentsPresent = true ∧
  E.sharedDigestFrameWitnessByteSourceContractPresent = true ∧
  E.activeFieldLocalDigestFrameWitnessByteComponentSurfacesPresent = true ∧
  E.leanActiveFiveSourceBoundaryRoutePresent = true ∧
  E.leanFullSignatureHiddenSourceHandoffPresent = true

/--
Proof-bearing selected Rust/model certificate for the active five-source
hidden-source closure input record.

The final AP2-N001 instantiation theorem should construct this certificate from
reviewed implementation/model evidence, then derive the closure inputs through
`toClosureInputs`.  This keeps the preflight gate from accepting a theorem that
locally assembles the seven source fields without also carrying a structured,
verified public evidence report and proof that all report readiness facts are
true.
-/
structure ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  evidenceReport :
    ReviewedFq12SelectedRustFiveSourceClosureEvidence
  evidenceReportVerifiedProof :
    evidenceReport.ReportVerified
  evidenceFactsAllTrueProof :
    evidenceReport.FactsAllTrue
  payloadProjectsFrom :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload},
      P.reviewedFq12PayloadAccepts b payload ->
        payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
          payload.slotId = b.context.slotId ->
        P.reviewedFq12PayloadProjectsFrom b payload
  certificateAvailability :
    ReviewedFq12ExtractionCertificateAvailability P
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  assemblySource :
    ReviewedFq12FieldConcreteAssemblyLocalSource P
  compatibilitySource :
    ReviewedFq12FieldConcreteCompatibilitySource P
  witnessSources :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
      CanonicalParser WitnessParser P

/-- Convert a proof-bearing selected Rust/model certificate into closure inputs. -/
def ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.toClosureInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Cert :
      ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustModelFiveSourceClosureInputs
      FieldParser RouteParser CanonicalParser WitnessParser P where
  payloadProjectsFrom := by
    let _hReport : Cert.evidenceReport.ReportVerified :=
      Cert.evidenceReportVerifiedProof
    let _hFacts : Cert.evidenceReport.FactsAllTrue :=
      Cert.evidenceFactsAllTrueProof
    intro b payload hAccept hProjection
    exact Cert.payloadProjectsFrom hAccept hProjection
  certificateAvailability :=
    Cert.certificateAvailability
  roleRootLiftSource :=
    Cert.roleRootLiftSource
  selectedSource :=
    Cert.selectedSource
  assemblySource :=
    Cert.assemblySource
  compatibilitySource :=
    Cert.compatibilitySource
  witnessSources :=
    Cert.witnessSources

/--
Named constructor for the final selected Rust/model five-source certificate.

The implementation instantiation should route through this constructor rather
than assembling the record anonymously, so the Rust fail-closed gate can require
the proof-bearing evidence report and every selected source field at one named
surface.
-/
def ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustFiveSourceClosureEvidence)
    (evidenceReportVerifiedProof :
      evidenceReport.ReportVerified)
    (evidenceFactsAllTrueProof :
      evidenceReport.FactsAllTrue)
    (payloadProjectsFrom :
      forall {b : Path2NativeBoundary.Boundary}
          {payload : ReviewedFq12HiddenSourcePayload},
        P.reviewedFq12PayloadAccepts b payload ->
          payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
            payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload)
    (certificateAvailability :
      ReviewedFq12ExtractionCertificateAvailability P)
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (witnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
      FieldParser RouteParser CanonicalParser WitnessParser P where
  evidenceReport := evidenceReport
  evidenceReportVerifiedProof := evidenceReportVerifiedProof
  evidenceFactsAllTrueProof := evidenceFactsAllTrueProof
  payloadProjectsFrom := payloadProjectsFrom
  certificateAvailability := certificateAvailability
  roleRootLiftSource := roleRootLiftSource
  selectedSource := selectedSource
  assemblySource := assemblySource
  compatibilitySource := compatibilitySource
  witnessSources := witnessSources

/--
Active AP2-N001 boundary closure from the five selected Rust/model source
records.

This is the production-facing route from the source package the Rust audit
tracks into the named raw-component certificate and then into hidden-source
proof semantics closure.
-/
theorem reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_shared_digest_frame_witness_sources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (payloadProjectsFrom :
      forall {b : Path2NativeBoundary.Boundary}
          {payload : ReviewedFq12HiddenSourcePayload},
        P.reviewedFq12PayloadAccepts b payload ->
          payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
            payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload)
    (certificateAvailability :
      ReviewedFq12ExtractionCertificateAvailability P)
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (witnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P) :
    ReviewedFq12HiddenSourceProofSemanticsClosed P :=
  reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_digest_frame_witness_byte_component_inputs
    P payloadProjectsFrom certificateAvailability
    (reviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputsOfSources
      roleRootLiftSource selectedSource assemblySource compatibilitySource
      witnessSources)

/--
Active AP2-N001 boundary closure from the single concrete model input record.
-/
theorem reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustModelFiveSourceClosureInputs
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12HiddenSourceProofSemanticsClosed P :=
  reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_shared_digest_frame_witness_sources
    P I.payloadProjectsFrom I.certificateAvailability
    I.roleRootLiftSource I.selectedSource I.assemblySource
    I.compatibilitySource I.witnessSources

/--
The accepted-spend boundary field closes from the same single concrete model
input record.
-/
theorem with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_five_source_inputs
    (B : NativeAcceptedSpendSoundnessBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustModelFiveSourceClosureInputs
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    (NativeAcceptedSpendSoundnessBoundary.withReviewedFq12HiddenSourceClosure
      B P).hiddenSourceProofSemanticsClosed :=
  with_reviewed_fq12_hidden_source_closure_closed
    B
    (reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_inputs
      P I)

/--
Use reviewed-Fq12 hidden-source closure inside the full active Path 2 signature
boundary, preserving every other full-signature proof obligation.
-/
def NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
    (B : NativeFullSignatureSecurityBoundary)
    (P : Predicates)
    (Closed : ReviewedFq12HiddenSourceProofSemanticsClosed P) :
    NativeFullSignatureSecurityBoundary :=
  { B with
    acceptedSpendSoundness :=
      NativeAcceptedSpendSoundnessBoundary.withReviewedFq12HiddenSourceClosure
        B.acceptedSpendSoundness P
    fullSignatureSecurityClaimComplete := by
      rcases B.fullSignatureSecurityClaimComplete with
        ⟨hKeygen, hSigning, hCompleteness, hVerifier, hFullGame,
          _hHiddenSource, hCheckedCore, hLifecycle, hNoExport,
          hCheckedCoreDomain, hHiddenSourceDomain, hQueryBudget,
          hMultiTarget⟩
      exact
        ⟨hKeygen, hSigning, hCompleteness, hVerifier, hFullGame,
          Closed, hCheckedCore, hLifecycle, hNoExport,
          hCheckedCoreDomain, hHiddenSourceDomain, hQueryBudget,
          hMultiTarget⟩ }

/-- The full-signature wrapper exposes exactly the supplied reviewed-Fq12 closure. -/
theorem full_signature_with_reviewed_fq12_hidden_source_closure_closed
    (B : NativeFullSignatureSecurityBoundary)
    {P : Predicates}
    (Closed : ReviewedFq12HiddenSourceProofSemanticsClosed P) :
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B P Closed).acceptedSpendSoundness.hiddenSourceProofSemanticsClosed :=
  Closed

/--
The full-signature boundary can consume the same selected Rust/model five-source
record used by the accepted-spend boundary adapter.
-/
theorem full_signature_with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_five_source_inputs
    (B : NativeFullSignatureSecurityBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustModelFiveSourceClosureInputs
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B P
      (reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_inputs
        P I)).acceptedSpendSoundness.hiddenSourceProofSemanticsClosed :=
  full_signature_with_reviewed_fq12_hidden_source_closure_closed
    B
    (reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_inputs
      P I)

end Path2HiddenSourceSemanticsBoundary
end Descender
