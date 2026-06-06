import Descender.Path2NativeAccountDesign
import Descender.Path2FullOpeningFieldEvidenceByteBridge
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
  hashByteDomainEvidenceDigest : ByteArray
  hashByteDomainEvidenceVerified : Bool
  hiddenSourceFrameDomainMatchesHashEvidence : Bool
  hiddenSourceStatementFrameUsesHashDomainField : Bool
  hiddenSourceStatementDigestUsesHashDomainFrame : Bool
  hiddenSourceProofDomainMatchesHashEvidence : Bool
  hiddenSourceProofTranscriptFrameUsesHashDomainField : Bool
  hiddenSourceProofTranscriptDigestUsesHashDomainFrame : Bool
  projectionCarrierDomainMatchesHashEvidence : Bool
  projectionCarrierBindingFrameUsesHashDomainField : Bool
  projectionCarrierBindingDigestSplitsToCheckAndTranscript : Bool
  accountWitnessOpeningDomainMatchesHashEvidence : Bool
  accountWitnessOpeningDigestUsesHashDomainFrame : Bool
  padSeedOpeningDomainMatchesHashEvidence : Bool
  padSeedOpeningDigestUsesHashDomainFrame : Bool
  padOpeningViewDomainMatchesHashEvidence : Bool
  padOpeningStatementDigestUsesHashDomainFrame : Bool
  directQueryBudgetSchemaMatches : Bool
  directQueryBudgetModeIsDirectSingleSlot : Bool
  directQueryBudgetExcludesQBatchFromActiveSymbols : Bool
  directQueryBudgetMarksQBatchInactive : Bool
  directQueryBudgetOptionalAggregationTermsQBatchOnly : Bool
  leanDirectQueryBudgetKeepsQBatchInactivePresent : Bool
  boundaryBaseEvidenceDigest : ByteArray
  boundaryBaseEvidenceVerified : Bool
  roleRootLiftEvidenceDigest : ByteArray
  roleRootLiftEvidenceVerified : Bool
  selectedSourceRouteEvidenceDigest : ByteArray
  selectedSourceRouteEvidenceVerified : Bool
  fieldLocalEvidenceDigest : ByteArray
  fieldLocalEvidenceVerified : Bool
  sharedDigestFrameWitnessEvidenceDigest : ByteArray
  sharedDigestFrameWitnessEvidenceVerified : Bool
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
  E.hashByteDomainEvidenceVerified = true ∧
  E.boundaryBaseEvidenceVerified = true ∧
  E.roleRootLiftEvidenceVerified = true ∧
  E.selectedSourceRouteEvidenceVerified = true ∧
  E.fieldLocalEvidenceVerified = true ∧
  E.sharedDigestFrameWitnessEvidenceVerified = true ∧
  E.reportDigestVerified = true ∧
  E.bridgeAuditValidated = true

/-- All non-closing five-source readiness facts recorded by the report hold. -/
def ReviewedFq12SelectedRustFiveSourceClosureEvidence.FactsAllTrue
    (E : ReviewedFq12SelectedRustFiveSourceClosureEvidence) : Prop :=
  E.selectedRustModelFiveSourceInputsReady = true ∧
  E.hashByteDomainEvidenceVerified = true ∧
  E.hiddenSourceFrameDomainMatchesHashEvidence = true ∧
  E.hiddenSourceStatementFrameUsesHashDomainField = true ∧
  E.hiddenSourceStatementDigestUsesHashDomainFrame = true ∧
  E.hiddenSourceProofDomainMatchesHashEvidence = true ∧
  E.hiddenSourceProofTranscriptFrameUsesHashDomainField = true ∧
  E.hiddenSourceProofTranscriptDigestUsesHashDomainFrame = true ∧
  E.projectionCarrierDomainMatchesHashEvidence = true ∧
  E.projectionCarrierBindingFrameUsesHashDomainField = true ∧
  E.projectionCarrierBindingDigestSplitsToCheckAndTranscript = true ∧
  E.accountWitnessOpeningDomainMatchesHashEvidence = true ∧
  E.accountWitnessOpeningDigestUsesHashDomainFrame = true ∧
  E.padSeedOpeningDomainMatchesHashEvidence = true ∧
  E.padSeedOpeningDigestUsesHashDomainFrame = true ∧
  E.padOpeningViewDomainMatchesHashEvidence = true ∧
  E.padOpeningStatementDigestUsesHashDomainFrame = true ∧
  E.directQueryBudgetSchemaMatches = true ∧
  E.directQueryBudgetModeIsDirectSingleSlot = true ∧
  E.directQueryBudgetExcludesQBatchFromActiveSymbols = true ∧
  E.directQueryBudgetMarksQBatchInactive = true ∧
  E.directQueryBudgetOptionalAggregationTermsQBatchOnly = true ∧
  E.leanDirectQueryBudgetKeepsQBatchInactivePresent = true ∧
  E.boundaryBaseEvidenceVerified = true ∧
  E.roleRootLiftEvidenceVerified = true ∧
  E.selectedSourceRouteEvidenceVerified = true ∧
  E.fieldLocalEvidenceVerified = true ∧
  E.sharedDigestFrameWitnessEvidenceVerified = true ∧
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
Lean mirror of the public base-boundary source evidence report.

This report records that the two non-byte AP2-N001 base premises have been
validated from the selected Rust/model boundary: accepted payload projection
from the native boundary and availability of extraction certificates.
-/
structure ReviewedFq12SelectedRustBoundaryBaseEvidenceReport where
  schemaMatches : Bool
  reportDigestVerified : Bool
  payloadProjectionLawPinned : Bool
  extractionCertificateAvailabilityPinned : Bool
  nativeBoundaryPayloadBindingPinned : Bool
  boundaryBaseNegativeControlsPassed : Bool

namespace ReviewedFq12SelectedRustBoundaryBaseEvidenceReport

/-- The public base-boundary report has been digest/schema checked. -/
def ReportVerified
    (E : ReviewedFq12SelectedRustBoundaryBaseEvidenceReport) :
    Prop :=
  E.schemaMatches = true ∧
    E.reportDigestVerified = true

/-- Every selected base-boundary component fact in the report holds. -/
def FactsAllTrue
    (E : ReviewedFq12SelectedRustBoundaryBaseEvidenceReport) :
    Prop :=
  E.payloadProjectionLawPinned = true ∧
    E.extractionCertificateAvailabilityPinned = true ∧
    E.nativeBoundaryPayloadBindingPinned = true ∧
    E.boundaryBaseNegativeControlsPassed = true

end ReviewedFq12SelectedRustBoundaryBaseEvidenceReport

/--
Proof-bearing selected Rust base-boundary source certificate.

The final AP2-N001 theorem should use this certificate to expose payload
projection and extraction-certificate availability instead of accepting those
base premises as independent raw assumptions.
-/
structure ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate
    (P : Predicates) where
  evidenceReport :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceReport
  evidenceDigest : ByteArray
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

namespace ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate

/-- Named constructor for the base-boundary evidence certificate. -/
def ofReportAndSources
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceReport)
    (evidenceDigest : ByteArray)
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
      ReviewedFq12ExtractionCertificateAvailability P) :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  evidenceReportVerifiedProof := evidenceReportVerifiedProof
  evidenceFactsAllTrueProof := evidenceFactsAllTrueProof
  payloadProjectsFrom := payloadProjectsFrom
  certificateAvailability := certificateAvailability

/-- Expose payload projection only after consuming report proofs. -/
def toPayloadProjectsFrom
    {P : Predicates}
    (Cert : ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P) :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload},
      P.reviewedFq12PayloadAccepts b payload ->
        payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
          payload.slotId = b.context.slotId ->
        P.reviewedFq12PayloadProjectsFrom b payload := by
  let _hReport : Cert.evidenceReport.ReportVerified :=
    Cert.evidenceReportVerifiedProof
  let _hFacts : Cert.evidenceReport.FactsAllTrue :=
    Cert.evidenceFactsAllTrueProof
  intro b payload hAccept hProjection
  exact Cert.payloadProjectsFrom hAccept hProjection

/-- Expose extraction-certificate availability only after report proofs. -/
def toCertificateAvailability
    {P : Predicates}
    (Cert : ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P) :
    ReviewedFq12ExtractionCertificateAvailability P :=
  let _hReport : Cert.evidenceReport.ReportVerified :=
    Cert.evidenceReportVerifiedProof
  let _hFacts : Cert.evidenceReport.FactsAllTrue :=
    Cert.evidenceFactsAllTrueProof
  Cert.certificateAvailability

end ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate

/--
The aggregate five-source report names the exact public evidence reports that
back each certificate consumed by the final AP2-N001 constructor.
-/
def ReviewedFq12SelectedRustFiveSourceClosureEvidence.SubEvidenceDigestsMatch
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (E : ReviewedFq12SelectedRustFiveSourceClosureEvidence)
    (boundaryBaseCertificate :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P)
    (roleRootLiftCertificate :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P)
    (selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P)
    (fieldLocalCertificate :
      ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P)
    (witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) : Prop :=
  E.boundaryBaseEvidenceDigest = boundaryBaseCertificate.evidenceDigest ∧
  E.roleRootLiftEvidenceDigest = roleRootLiftCertificate.evidenceDigest ∧
  E.selectedSourceRouteEvidenceDigest =
    selectedSourceRouteCertificate.evidenceDigest ∧
  E.fieldLocalEvidenceDigest = fieldLocalCertificate.evidenceDigest ∧
  E.sharedDigestFrameWitnessEvidenceDigest =
    witnessSourcesCertificate.evidenceDigest

/--
Concrete selected Rust/model five-source report value.

This is intentionally fail-closed: it gives the final theorem a named selected
report target without proving that the public Rust evidence has been imported
and verified in Lean.  The `ReportVerified` and `FactsAllTrue` proofs remain
separate selected definitions that must be supplied before the certificate can
close AP2-N001.
-/
def placeholderSelectedRustFiveSourceEvidenceReport :
    ReviewedFq12SelectedRustFiveSourceClosureEvidence where
  schemaMatches := false
  bridgeAuditDigest := ByteArray.mk #[]
  bridgeAuditDigestBound := false
  hashByteDomainEvidenceDigest := ByteArray.mk #[]
  hashByteDomainEvidenceVerified := false
  hiddenSourceFrameDomainMatchesHashEvidence := false
  hiddenSourceStatementFrameUsesHashDomainField := false
  hiddenSourceStatementDigestUsesHashDomainFrame := false
  hiddenSourceProofDomainMatchesHashEvidence := false
  hiddenSourceProofTranscriptFrameUsesHashDomainField := false
  hiddenSourceProofTranscriptDigestUsesHashDomainFrame := false
  projectionCarrierDomainMatchesHashEvidence := false
  projectionCarrierBindingFrameUsesHashDomainField := false
  projectionCarrierBindingDigestSplitsToCheckAndTranscript := false
  accountWitnessOpeningDomainMatchesHashEvidence := false
  accountWitnessOpeningDigestUsesHashDomainFrame := false
  padSeedOpeningDomainMatchesHashEvidence := false
  padSeedOpeningDigestUsesHashDomainFrame := false
  padOpeningViewDomainMatchesHashEvidence := false
  padOpeningStatementDigestUsesHashDomainFrame := false
  directQueryBudgetSchemaMatches := false
  directQueryBudgetModeIsDirectSingleSlot := false
  directQueryBudgetExcludesQBatchFromActiveSymbols := false
  directQueryBudgetMarksQBatchInactive := false
  directQueryBudgetOptionalAggregationTermsQBatchOnly := false
  leanDirectQueryBudgetKeepsQBatchInactivePresent := false
  boundaryBaseEvidenceDigest := ByteArray.mk #[]
  boundaryBaseEvidenceVerified := false
  roleRootLiftEvidenceDigest := ByteArray.mk #[]
  roleRootLiftEvidenceVerified := false
  selectedSourceRouteEvidenceDigest := ByteArray.mk #[]
  selectedSourceRouteEvidenceVerified := false
  fieldLocalEvidenceDigest := ByteArray.mk #[]
  fieldLocalEvidenceVerified := false
  sharedDigestFrameWitnessEvidenceDigest := ByteArray.mk #[]
  sharedDigestFrameWitnessEvidenceVerified := false
  reportDigestVerified := false
  bridgeAuditValidated := false
  selectedRustModelFiveSourceInputsReady := false
  rustPayloadProjectionLawPresent := false
  rustExtractionCertificateAvailabilityPresent := false
  roleRootLiftSourcePresent := false
  selectedSourceRouteByteSourcePresent := false
  assemblyFieldLocalNativeComponentsPresent := false
  compatibilityFieldExtractionComponentsPresent := false
  sharedDigestFrameWitnessByteSourceContractPresent := false
  activeFieldLocalDigestFrameWitnessByteComponentSurfacesPresent := false
  leanActiveFiveSourceBoundaryRoutePresent := false
  leanFullSignatureHiddenSourceHandoffPresent := false
  extractorTheoremInstantiated := false
  semanticsBridgeComplete := false

/-- Concrete selected Rust/model base-boundary report value, still fail-closed. -/
def placeholderSelectedRustBoundaryBaseEvidenceReport :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceReport where
  schemaMatches := false
  reportDigestVerified := false
  payloadProjectionLawPinned := false
  extractionCertificateAvailabilityPinned := false
  nativeBoundaryPayloadBindingPinned := false
  boundaryBaseNegativeControlsPassed := false

/-- Concrete selected Rust/model role-root/lift report value, still fail-closed. -/
def placeholderSelectedRustRoleRootLiftEvidenceReport :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceReport where
  schemaMatches := false
  reportDigestVerified := false
  concreteRoleRootFieldBindingPinned := false
  selectedLiftFunctionalityPinned := false
  roleRootBridgePinned := false
  roleRootNegativeControlsPassed := false

/-- Concrete selected Rust/model source-route report value, still fail-closed. -/
def placeholderSelectedRustSourceRouteEvidenceReport :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport where
  schemaMatches := false
  reportDigestVerified := false
  fieldFrameParserPinned := false
  routeFrameParserPinned := false
  sourceRouteAvailabilityPinned := false
  fieldByteCertificatePinned := false
  routeByteCertificatePinned := false
  fieldBindsPinned := false
  sourceRouteBindsPinned := false
  aggregateBridgeRejectionPinned := false
  sourceRouteNegativeControlsPassed := false

/-- Concrete selected Rust/model field/local report value, still fail-closed. -/
def placeholderSelectedRustFieldLocalEvidenceReport :
    ReviewedFq12SelectedRustFieldLocalEvidenceReport where
  schemaMatches := false
  reportDigestVerified := false
  assemblyAvailabilityPinned := false
  assemblyFieldBindingPinned := false
  assemblyLocalPredicatesPinned := false
  assemblyNativeValidityPinned := false
  compatibilityAvailabilityPinned := false
  compatibilityFieldBindingPinned := false
  compatibilityExtractionRulesPinned := false
  fieldLocalNegativeControlsPassed := false

/-- Concrete selected Rust/model shared-witness report value, still fail-closed. -/
def placeholderSelectedRustSharedFrameWitnessEvidenceReport :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport where
  schemaMatches := false
  reportDigestVerified := false
  canonicalWitnessParserPinned := false
  digestFrameWitnessParserPinned := false
  sharedDigestFramePinned := false
  canonicalWitnessByteCertificatePinned := false
  digestFrameWitnessByteCertificatePinned := false
  representativeDigestFrameSourcePinned := false
  canonicalWitnessVerifierPinned := false
  digestFrameWitnessVerifierPinned := false
  sharedDigestFrameNegativeControlsPassed := false

/--
Verifier-backed Rust/model five-source report imported from
`path2_selected_evidence_report_manifest`.

This is deliberately not the active `selectedRustFiveSourceEvidenceReport`
yet.  It proves the Rust report can inhabit the Lean report predicates while
leaving the selected closure path fail-closed until source records and
sub-evidence digest matching are imported.
-/
def verifierBackedSelectedRustFiveSourceEvidenceReport :
    ReviewedFq12SelectedRustFiveSourceClosureEvidence where
  schemaMatches := true
  bridgeAuditDigest := ByteArray.mk #[125, 229, 228, 169, 136, 54, 188,
    128, 138, 202, 39, 36, 89, 139, 221, 64, 253, 232, 8, 10, 39, 108, 49,
    63, 215, 193, 103, 72, 214, 142, 34, 53]
  bridgeAuditDigestBound := true
  hashByteDomainEvidenceDigest := ByteArray.mk #[23, 215, 160, 136, 150, 70,
    25, 35, 33, 177, 125, 177, 122, 119, 78, 13, 155, 114, 16, 80, 84,
    123, 251, 239, 42, 10, 247, 17, 206, 228, 133, 89]
  hashByteDomainEvidenceVerified := true
  hiddenSourceFrameDomainMatchesHashEvidence := true
  hiddenSourceStatementFrameUsesHashDomainField := true
  hiddenSourceStatementDigestUsesHashDomainFrame := true
  hiddenSourceProofDomainMatchesHashEvidence := true
  hiddenSourceProofTranscriptFrameUsesHashDomainField := true
  hiddenSourceProofTranscriptDigestUsesHashDomainFrame := true
  projectionCarrierDomainMatchesHashEvidence := true
  projectionCarrierBindingFrameUsesHashDomainField := true
  projectionCarrierBindingDigestSplitsToCheckAndTranscript := true
  accountWitnessOpeningDomainMatchesHashEvidence := true
  accountWitnessOpeningDigestUsesHashDomainFrame := true
  padSeedOpeningDomainMatchesHashEvidence := true
  padSeedOpeningDigestUsesHashDomainFrame := true
  padOpeningViewDomainMatchesHashEvidence := true
  padOpeningStatementDigestUsesHashDomainFrame := true
  directQueryBudgetSchemaMatches := true
  directQueryBudgetModeIsDirectSingleSlot := true
  directQueryBudgetExcludesQBatchFromActiveSymbols := true
  directQueryBudgetMarksQBatchInactive := true
  directQueryBudgetOptionalAggregationTermsQBatchOnly := true
  leanDirectQueryBudgetKeepsQBatchInactivePresent := true
  boundaryBaseEvidenceDigest := ByteArray.mk #[6, 242, 163, 38, 135, 152,
    231, 128, 79, 124, 169, 73, 131, 31, 237, 140, 69, 162, 33, 92, 67,
    23, 21, 246, 42, 194, 70, 101, 92, 198, 134, 169]
  boundaryBaseEvidenceVerified := true
  roleRootLiftEvidenceDigest := ByteArray.mk #[112, 34, 58, 36, 253, 212,
    82, 232, 205, 83, 113, 123, 199, 43, 87, 194, 42, 74, 217, 204, 35,
    246, 18, 154, 63, 26, 237, 166, 20, 147, 7, 43]
  roleRootLiftEvidenceVerified := true
  selectedSourceRouteEvidenceDigest := ByteArray.mk #[76, 112, 68, 95, 197,
    248, 203, 97, 213, 12, 109, 225, 214, 130, 133, 63, 42, 106, 59, 59,
    81, 82, 22, 111, 241, 35, 195, 73, 37, 227, 43, 127]
  selectedSourceRouteEvidenceVerified := true
  fieldLocalEvidenceDigest := ByteArray.mk #[169, 179, 188, 15, 137, 255,
    36, 143, 125, 232, 213, 13, 180, 213, 106, 216, 158, 75, 59, 46, 32,
    23, 118, 152, 244, 151, 38, 128, 245, 8, 172, 114]
  fieldLocalEvidenceVerified := true
  sharedDigestFrameWitnessEvidenceDigest := ByteArray.mk #[231, 221, 193,
    212, 120, 26, 89, 161, 44, 144, 83, 244, 222, 142, 228, 226, 178,
    182, 64, 109, 54, 74, 46, 221, 67, 139, 195, 204, 133, 122, 244, 215]
  sharedDigestFrameWitnessEvidenceVerified := true
  reportDigestVerified := true
  bridgeAuditValidated := true
  selectedRustModelFiveSourceInputsReady := true
  rustPayloadProjectionLawPresent := true
  rustExtractionCertificateAvailabilityPresent := true
  roleRootLiftSourcePresent := true
  selectedSourceRouteByteSourcePresent := true
  assemblyFieldLocalNativeComponentsPresent := true
  compatibilityFieldExtractionComponentsPresent := true
  sharedDigestFrameWitnessByteSourceContractPresent := true
  activeFieldLocalDigestFrameWitnessByteComponentSurfacesPresent := true
  leanActiveFiveSourceBoundaryRoutePresent := true
  leanFullSignatureHiddenSourceHandoffPresent := true
  extractorTheoremInstantiated := false
  semanticsBridgeComplete := false

/-- Verifier-backed base-boundary report imported from Rust evidence. -/
def verifierBackedSelectedRustBoundaryBaseEvidenceReport :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceReport where
  schemaMatches := true
  reportDigestVerified := true
  payloadProjectionLawPinned := true
  extractionCertificateAvailabilityPinned := true
  nativeBoundaryPayloadBindingPinned := true
  boundaryBaseNegativeControlsPassed := true

/-- Verifier-backed role-root/lift report imported from Rust evidence. -/
def verifierBackedSelectedRustRoleRootLiftEvidenceReport :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceReport where
  schemaMatches := true
  reportDigestVerified := true
  concreteRoleRootFieldBindingPinned := true
  selectedLiftFunctionalityPinned := true
  roleRootBridgePinned := true
  roleRootNegativeControlsPassed := true

/-- Verifier-backed source-route report imported from Rust evidence. -/
def verifierBackedSelectedRustSourceRouteEvidenceReport :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport where
  schemaMatches := true
  reportDigestVerified := true
  fieldFrameParserPinned := true
  routeFrameParserPinned := true
  sourceRouteAvailabilityPinned := true
  fieldByteCertificatePinned := true
  routeByteCertificatePinned := true
  fieldBindsPinned := true
  sourceRouteBindsPinned := true
  aggregateBridgeRejectionPinned := true
  sourceRouteNegativeControlsPassed := true

/-- Verifier-backed field/local report imported from Rust evidence. -/
def verifierBackedSelectedRustFieldLocalEvidenceReport :
    ReviewedFq12SelectedRustFieldLocalEvidenceReport where
  schemaMatches := true
  reportDigestVerified := true
  assemblyAvailabilityPinned := true
  assemblyFieldBindingPinned := true
  assemblyLocalPredicatesPinned := true
  assemblyNativeValidityPinned := true
  compatibilityAvailabilityPinned := true
  compatibilityFieldBindingPinned := true
  compatibilityExtractionRulesPinned := true
  fieldLocalNegativeControlsPassed := true

/-- Verifier-backed shared-frame witness report imported from Rust evidence. -/
def verifierBackedSelectedRustSharedFrameWitnessEvidenceReport :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport where
  schemaMatches := true
  reportDigestVerified := true
  canonicalWitnessParserPinned := true
  digestFrameWitnessParserPinned := true
  sharedDigestFramePinned := true
  canonicalWitnessByteCertificatePinned := true
  digestFrameWitnessByteCertificatePinned := true
  representativeDigestFrameSourcePinned := true
  canonicalWitnessVerifierPinned := true
  digestFrameWitnessVerifierPinned := true
  sharedDigestFrameNegativeControlsPassed := true

/-- The verifier-backed aggregate report satisfies the Lean verification predicate. -/
theorem verifierBackedSelectedRustFiveSourceEvidenceReport_reportVerified :
    verifierBackedSelectedRustFiveSourceEvidenceReport.ReportVerified := by
  simp [verifierBackedSelectedRustFiveSourceEvidenceReport,
    ReviewedFq12SelectedRustFiveSourceClosureEvidence.ReportVerified]

/-- The verifier-backed aggregate report satisfies all non-closing readiness facts. -/
theorem verifierBackedSelectedRustFiveSourceEvidenceReport_factsAllTrue :
    verifierBackedSelectedRustFiveSourceEvidenceReport.FactsAllTrue := by
  simp [verifierBackedSelectedRustFiveSourceEvidenceReport,
    ReviewedFq12SelectedRustFiveSourceClosureEvidence.FactsAllTrue]

/-- The verifier-backed base-boundary report satisfies the Lean predicates. -/
theorem verifierBackedSelectedRustBoundaryBaseEvidenceReport_ready :
    verifierBackedSelectedRustBoundaryBaseEvidenceReport.ReportVerified ∧
      verifierBackedSelectedRustBoundaryBaseEvidenceReport.FactsAllTrue := by
  simp [verifierBackedSelectedRustBoundaryBaseEvidenceReport,
    ReviewedFq12SelectedRustBoundaryBaseEvidenceReport.ReportVerified,
    ReviewedFq12SelectedRustBoundaryBaseEvidenceReport.FactsAllTrue]

/-- The verifier-backed role-root/lift report satisfies the Lean predicates. -/
theorem verifierBackedSelectedRustRoleRootLiftEvidenceReport_ready :
    verifierBackedSelectedRustRoleRootLiftEvidenceReport.ReportVerified ∧
      verifierBackedSelectedRustRoleRootLiftEvidenceReport.FactsAllTrue := by
  simp [verifierBackedSelectedRustRoleRootLiftEvidenceReport,
    ReviewedFq12SelectedRustRoleRootLiftEvidenceReport.ReportVerified,
    ReviewedFq12SelectedRustRoleRootLiftEvidenceReport.FactsAllTrue]

/-- The verifier-backed source-route report satisfies the Lean predicates. -/
theorem verifierBackedSelectedRustSourceRouteEvidenceReport_ready :
    verifierBackedSelectedRustSourceRouteEvidenceReport.ReportVerified ∧
      verifierBackedSelectedRustSourceRouteEvidenceReport.FactsAllTrue := by
  simp [verifierBackedSelectedRustSourceRouteEvidenceReport,
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport.ReportVerified,
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport.FactsAllTrue]

/-- The verifier-backed field/local report satisfies the Lean predicates. -/
theorem verifierBackedSelectedRustFieldLocalEvidenceReport_ready :
    verifierBackedSelectedRustFieldLocalEvidenceReport.ReportVerified ∧
      verifierBackedSelectedRustFieldLocalEvidenceReport.FactsAllTrue := by
  simp [verifierBackedSelectedRustFieldLocalEvidenceReport,
    ReviewedFq12SelectedRustFieldLocalEvidenceReport.ReportVerified,
    ReviewedFq12SelectedRustFieldLocalEvidenceReport.FactsAllTrue]

/-- The verifier-backed shared-frame witness report satisfies the Lean predicates. -/
theorem verifierBackedSelectedRustSharedFrameWitnessEvidenceReport_ready :
    verifierBackedSelectedRustSharedFrameWitnessEvidenceReport.ReportVerified ∧
      verifierBackedSelectedRustSharedFrameWitnessEvidenceReport.FactsAllTrue := by
  simp [verifierBackedSelectedRustSharedFrameWitnessEvidenceReport,
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport.ReportVerified,
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport.FactsAllTrue]

/--
Active selected Rust/model five-source report.

This now points to the verifier-backed public Rust evidence report. It still
does not close AP2-N001 by itself: source records and the sub-evidence
digest-match proof remain separate selected definitions.
-/
def selectedRustFiveSourceEvidenceReport :
    ReviewedFq12SelectedRustFiveSourceClosureEvidence :=
  verifierBackedSelectedRustFiveSourceEvidenceReport

/-- Active selected Rust/model base-boundary report. -/
def selectedRustBoundaryBaseEvidenceReport :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceReport :=
  verifierBackedSelectedRustBoundaryBaseEvidenceReport

/-- Active selected Rust/model role-root/lift report. -/
def selectedRustRoleRootLiftEvidenceReport :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceReport :=
  verifierBackedSelectedRustRoleRootLiftEvidenceReport

/-- Active selected Rust/model source-route report. -/
def selectedRustSourceRouteEvidenceReport :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport :=
  verifierBackedSelectedRustSourceRouteEvidenceReport

/-- Active selected Rust/model field/local report. -/
def selectedRustFieldLocalEvidenceReport :
    ReviewedFq12SelectedRustFieldLocalEvidenceReport :=
  verifierBackedSelectedRustFieldLocalEvidenceReport

/-- Active selected Rust/model shared-frame witness report. -/
def selectedRustSharedFrameWitnessEvidenceReport :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport :=
  verifierBackedSelectedRustSharedFrameWitnessEvidenceReport

/-- Proof that the active selected aggregate report is verifier-backed. -/
theorem selectedRustFiveSourceEvidenceReportVerifiedProof :
    selectedRustFiveSourceEvidenceReport.ReportVerified := by
  simpa [selectedRustFiveSourceEvidenceReport] using
    verifierBackedSelectedRustFiveSourceEvidenceReport_reportVerified

/-- Proof that all non-closing active aggregate report facts hold. -/
theorem selectedRustFiveSourceEvidenceFactsAllTrueProof :
    selectedRustFiveSourceEvidenceReport.FactsAllTrue := by
  simpa [selectedRustFiveSourceEvidenceReport] using
    verifierBackedSelectedRustFiveSourceEvidenceReport_factsAllTrue

/-- Proof that the active base-boundary report is verifier-backed. -/
theorem selectedRustBoundaryBaseEvidenceReportVerifiedProof :
    selectedRustBoundaryBaseEvidenceReport.ReportVerified := by
  simpa [selectedRustBoundaryBaseEvidenceReport] using
    verifierBackedSelectedRustBoundaryBaseEvidenceReport_ready.1

/-- Proof that all active base-boundary report facts hold. -/
theorem selectedRustBoundaryBaseEvidenceFactsAllTrueProof :
    selectedRustBoundaryBaseEvidenceReport.FactsAllTrue := by
  simpa [selectedRustBoundaryBaseEvidenceReport] using
    verifierBackedSelectedRustBoundaryBaseEvidenceReport_ready.2

/-- Proof that the active role-root/lift report is verifier-backed. -/
theorem selectedRustRoleRootLiftEvidenceReportVerifiedProof :
    selectedRustRoleRootLiftEvidenceReport.ReportVerified := by
  simpa [selectedRustRoleRootLiftEvidenceReport] using
    verifierBackedSelectedRustRoleRootLiftEvidenceReport_ready.1

/-- Proof that all active role-root/lift report facts hold. -/
theorem selectedRustRoleRootLiftEvidenceFactsAllTrueProof :
    selectedRustRoleRootLiftEvidenceReport.FactsAllTrue := by
  simpa [selectedRustRoleRootLiftEvidenceReport] using
    verifierBackedSelectedRustRoleRootLiftEvidenceReport_ready.2

/-- Proof that the active source-route report is verifier-backed. -/
theorem selectedRustSourceRouteEvidenceReportVerifiedProof :
    selectedRustSourceRouteEvidenceReport.ReportVerified := by
  simpa [selectedRustSourceRouteEvidenceReport] using
    verifierBackedSelectedRustSourceRouteEvidenceReport_ready.1

/-- Proof that all active source-route report facts hold. -/
theorem selectedRustSourceRouteEvidenceFactsAllTrueProof :
    selectedRustSourceRouteEvidenceReport.FactsAllTrue := by
  simpa [selectedRustSourceRouteEvidenceReport] using
    verifierBackedSelectedRustSourceRouteEvidenceReport_ready.2

/-- Proof that the active field/local report is verifier-backed. -/
theorem selectedRustFieldLocalEvidenceReportVerifiedProof :
    selectedRustFieldLocalEvidenceReport.ReportVerified := by
  simpa [selectedRustFieldLocalEvidenceReport] using
    verifierBackedSelectedRustFieldLocalEvidenceReport_ready.1

/-- Proof that all active field/local report facts hold. -/
theorem selectedRustFieldLocalEvidenceFactsAllTrueProof :
    selectedRustFieldLocalEvidenceReport.FactsAllTrue := by
  simpa [selectedRustFieldLocalEvidenceReport] using
    verifierBackedSelectedRustFieldLocalEvidenceReport_ready.2

/-- Proof that the active shared-frame witness report is verifier-backed. -/
theorem selectedRustSharedFrameWitnessEvidenceReportVerifiedProof :
    selectedRustSharedFrameWitnessEvidenceReport.ReportVerified := by
  simpa [selectedRustSharedFrameWitnessEvidenceReport] using
    verifierBackedSelectedRustSharedFrameWitnessEvidenceReport_ready.1

/-- Proof that all active shared-frame witness report facts hold. -/
theorem selectedRustSharedFrameWitnessEvidenceFactsAllTrueProof :
    selectedRustSharedFrameWitnessEvidenceReport.FactsAllTrue := by
  simpa [selectedRustSharedFrameWitnessEvidenceReport] using
    verifierBackedSelectedRustSharedFrameWitnessEvidenceReport_ready.2

/--
The old placeholder five-source report is intentionally not proof-bearing.

These negative lemmas are sanity checks for the fail-closed boundary: the
future AP2-N001 theorem must replace the selected report/proof source with
verifier-backed evidence, not reuse this placeholder report as if it were
verified.
-/
theorem placeholderSelectedRustFiveSourceEvidenceReport_not_reportVerified :
    ¬ placeholderSelectedRustFiveSourceEvidenceReport.ReportVerified := by
  simp [placeholderSelectedRustFiveSourceEvidenceReport,
    ReviewedFq12SelectedRustFiveSourceClosureEvidence.ReportVerified]

theorem placeholderSelectedRustFiveSourceEvidenceReport_not_factsAllTrue :
    ¬ placeholderSelectedRustFiveSourceEvidenceReport.FactsAllTrue := by
  simp [placeholderSelectedRustFiveSourceEvidenceReport,
    ReviewedFq12SelectedRustFiveSourceClosureEvidence.FactsAllTrue]

/-- The current base-boundary report is intentionally fail-closed. -/
theorem placeholderSelectedRustBoundaryBaseEvidenceReport_not_reportVerified :
    ¬ placeholderSelectedRustBoundaryBaseEvidenceReport.ReportVerified := by
  simp [placeholderSelectedRustBoundaryBaseEvidenceReport,
    ReviewedFq12SelectedRustBoundaryBaseEvidenceReport.ReportVerified]

/-- The current base-boundary report does not prove its component facts. -/
theorem placeholderSelectedRustBoundaryBaseEvidenceReport_not_factsAllTrue :
    ¬ placeholderSelectedRustBoundaryBaseEvidenceReport.FactsAllTrue := by
  simp [placeholderSelectedRustBoundaryBaseEvidenceReport,
    ReviewedFq12SelectedRustBoundaryBaseEvidenceReport.FactsAllTrue]

/-- The current role-root/lift report is intentionally fail-closed. -/
theorem placeholderSelectedRustRoleRootLiftEvidenceReport_not_reportVerified :
    ¬ placeholderSelectedRustRoleRootLiftEvidenceReport.ReportVerified := by
  simp [placeholderSelectedRustRoleRootLiftEvidenceReport,
    ReviewedFq12SelectedRustRoleRootLiftEvidenceReport.ReportVerified]

/-- The current role-root/lift report does not prove its component facts. -/
theorem placeholderSelectedRustRoleRootLiftEvidenceReport_not_factsAllTrue :
    ¬ placeholderSelectedRustRoleRootLiftEvidenceReport.FactsAllTrue := by
  simp [placeholderSelectedRustRoleRootLiftEvidenceReport,
    ReviewedFq12SelectedRustRoleRootLiftEvidenceReport.FactsAllTrue]

/-- The current source-route report is intentionally fail-closed. -/
theorem placeholderSelectedRustSourceRouteEvidenceReport_not_reportVerified :
    ¬ placeholderSelectedRustSourceRouteEvidenceReport.ReportVerified := by
  simp [placeholderSelectedRustSourceRouteEvidenceReport,
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport.ReportVerified]

/-- The current source-route report does not prove its component facts. -/
theorem placeholderSelectedRustSourceRouteEvidenceReport_not_factsAllTrue :
    ¬ placeholderSelectedRustSourceRouteEvidenceReport.FactsAllTrue := by
  simp [placeholderSelectedRustSourceRouteEvidenceReport,
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport.FactsAllTrue]

/-- The current field/local report is intentionally fail-closed. -/
theorem placeholderSelectedRustFieldLocalEvidenceReport_not_reportVerified :
    ¬ placeholderSelectedRustFieldLocalEvidenceReport.ReportVerified := by
  simp [placeholderSelectedRustFieldLocalEvidenceReport,
    ReviewedFq12SelectedRustFieldLocalEvidenceReport.ReportVerified]

/-- The current field/local report does not prove its component facts. -/
theorem placeholderSelectedRustFieldLocalEvidenceReport_not_factsAllTrue :
    ¬ placeholderSelectedRustFieldLocalEvidenceReport.FactsAllTrue := by
  simp [placeholderSelectedRustFieldLocalEvidenceReport,
    ReviewedFq12SelectedRustFieldLocalEvidenceReport.FactsAllTrue]

/-- The current shared-frame witness report is intentionally fail-closed. -/
theorem placeholderSelectedRustSharedFrameWitnessEvidenceReport_not_reportVerified :
    ¬ placeholderSelectedRustSharedFrameWitnessEvidenceReport.ReportVerified := by
  simp [placeholderSelectedRustSharedFrameWitnessEvidenceReport,
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport.ReportVerified]

/-- The current shared-frame witness report does not prove its component facts. -/
theorem placeholderSelectedRustSharedFrameWitnessEvidenceReport_not_factsAllTrue :
    ¬ placeholderSelectedRustSharedFrameWitnessEvidenceReport.FactsAllTrue := by
  simp [placeholderSelectedRustSharedFrameWitnessEvidenceReport,
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport.FactsAllTrue]

/--
The current placeholder reports cannot supply the proof-bearing selected
definitions required by the concrete-definition checklist.

This theorem deliberately does not provide the selected proof definitions. It
records the opposite fact: any attempt to close AP2-N001 with proofs for the
current fail-closed reports is inconsistent.
-/
theorem selectedRustCurrentPlaceholderEvidenceProofDefinitions_impossible
    (hFiveReport :
      placeholderSelectedRustFiveSourceEvidenceReport.ReportVerified)
    (_hFiveFacts :
      placeholderSelectedRustFiveSourceEvidenceReport.FactsAllTrue)
    (_hBoundaryReport :
      placeholderSelectedRustBoundaryBaseEvidenceReport.ReportVerified)
    (_hBoundaryFacts :
      placeholderSelectedRustBoundaryBaseEvidenceReport.FactsAllTrue)
    (_hRoleRootReport :
      placeholderSelectedRustRoleRootLiftEvidenceReport.ReportVerified)
    (_hRoleRootFacts :
      placeholderSelectedRustRoleRootLiftEvidenceReport.FactsAllTrue)
    (_hSourceRouteReport :
      placeholderSelectedRustSourceRouteEvidenceReport.ReportVerified)
    (_hSourceRouteFacts :
      placeholderSelectedRustSourceRouteEvidenceReport.FactsAllTrue)
    (_hFieldLocalReport :
      placeholderSelectedRustFieldLocalEvidenceReport.ReportVerified)
    (_hFieldLocalFacts :
      placeholderSelectedRustFieldLocalEvidenceReport.FactsAllTrue)
    (_hSharedFrameReport :
      placeholderSelectedRustSharedFrameWitnessEvidenceReport.ReportVerified)
    (_hSharedFrameFacts :
      placeholderSelectedRustSharedFrameWitnessEvidenceReport.FactsAllTrue) :
    False := by
  exact placeholderSelectedRustFiveSourceEvidenceReport_not_reportVerified hFiveReport

/-- The current base-boundary placeholder report cannot certify a source. -/
theorem selectedRustBoundaryBaseCertificate_not_from_current_placeholder_report
    {P : Predicates}
    (Cert : ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P)
    (hReport :
      Cert.evidenceReport = placeholderSelectedRustBoundaryBaseEvidenceReport) :
    False := by
  exact placeholderSelectedRustBoundaryBaseEvidenceReport_not_reportVerified
    (hReport ▸ Cert.evidenceReportVerifiedProof)

/-- The current role-root/lift placeholder report cannot certify a source. -/
theorem selectedRustRoleRootLiftCertificate_not_from_current_placeholder_report
    {P : Predicates}
    (Cert : ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P)
    (hReport :
      Cert.evidenceReport = placeholderSelectedRustRoleRootLiftEvidenceReport) :
    False := by
  exact placeholderSelectedRustRoleRootLiftEvidenceReport_not_reportVerified
    (hReport ▸ Cert.evidenceReportVerifiedProof)

/-- The current source-route placeholder report cannot certify a byte source. -/
theorem selectedRustSourceRouteCertificate_not_from_current_placeholder_report
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Cert :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P)
    (hReport :
      Cert.evidenceReport = placeholderSelectedRustSourceRouteEvidenceReport) :
    False := by
  exact placeholderSelectedRustSourceRouteEvidenceReport_not_reportVerified
    (hReport ▸ Cert.evidenceReportVerifiedProof)

/-- The current field/local placeholder report cannot certify sources. -/
theorem selectedRustFieldLocalCertificate_not_from_current_placeholder_report
    {P : Predicates}
    (Cert : ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P)
    (hReport :
      Cert.evidenceReport = placeholderSelectedRustFieldLocalEvidenceReport) :
    False := by
  exact placeholderSelectedRustFieldLocalEvidenceReport_not_reportVerified
    (hReport ▸ Cert.evidenceReportVerifiedProof)

/-- The current shared-witness placeholder report cannot certify sources. -/
theorem selectedRustSharedFrameWitnessCertificate_not_from_current_placeholder_report
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P)
    (hReport :
      Cert.evidenceReport = placeholderSelectedRustSharedFrameWitnessEvidenceReport) :
    False := by
  exact placeholderSelectedRustSharedFrameWitnessEvidenceReport_not_reportVerified
    (hReport ▸ Cert.evidenceReportVerifiedProof)

/--
The current placeholder reports cannot be the report sources for the component
certificates required by the selected Rust/model source-record package.

This records the source-record side of the fail-closed checklist: the future
definitions must be backed by real selected evidence certificates, not
certificates over the placeholder reports above.
-/
theorem selectedRustCurrentPlaceholderSourceRecordCertificates_impossible
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (boundaryBaseCertificate :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P)
    (roleRootLiftCertificate :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P)
    (selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P)
    (fieldLocalCertificate :
      ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P)
    (witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P)
    (hBoundaryReport :
      boundaryBaseCertificate.evidenceReport =
        placeholderSelectedRustBoundaryBaseEvidenceReport)
    (_hRoleRootReport :
      roleRootLiftCertificate.evidenceReport =
        placeholderSelectedRustRoleRootLiftEvidenceReport)
    (_hSourceRouteReport :
      selectedSourceRouteCertificate.evidenceReport =
        placeholderSelectedRustSourceRouteEvidenceReport)
    (_hFieldLocalReport :
      fieldLocalCertificate.evidenceReport =
        placeholderSelectedRustFieldLocalEvidenceReport)
    (_hWitnessReport :
      witnessSourcesCertificate.evidenceReport =
        placeholderSelectedRustSharedFrameWitnessEvidenceReport) :
    False := by
  exact selectedRustBoundaryBaseCertificate_not_from_current_placeholder_report
    boundaryBaseCertificate hBoundaryReport

/--
Any sub-evidence match proof for the current placeholder aggregate report would
force every consumed certificate digest to be empty.

The public Rust evidence digests below are non-empty concrete byte arrays, so
the future AP2-N001 instantiation must replace this placeholder aggregate
report with verifier-backed selected evidence before the digest-match proof can
be useful.
-/
theorem selectedRustFiveSourceEvidenceReport_subEvidenceDigestsMatch_forces_empty_certificate_digests
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    {boundaryBaseCertificate :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P}
    {roleRootLiftCertificate :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P}
    {selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P}
    {fieldLocalCertificate :
      ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P}
    {witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P}
    (hMatch :
      placeholderSelectedRustFiveSourceEvidenceReport.SubEvidenceDigestsMatch
        boundaryBaseCertificate roleRootLiftCertificate
        selectedSourceRouteCertificate fieldLocalCertificate
        witnessSourcesCertificate) :
    boundaryBaseCertificate.evidenceDigest = ByteArray.mk #[] ∧
      roleRootLiftCertificate.evidenceDigest = ByteArray.mk #[] ∧
      selectedSourceRouteCertificate.evidenceDigest = ByteArray.mk #[] ∧
      fieldLocalCertificate.evidenceDigest = ByteArray.mk #[] ∧
      witnessSourcesCertificate.evidenceDigest = ByteArray.mk #[] := by
  rcases hMatch with
    ⟨hBoundary, hRoleRoot, hSourceRoute, hFieldLocal, hWitness⟩
  simp [placeholderSelectedRustFiveSourceEvidenceReport] at hBoundary hRoleRoot hSourceRoute hFieldLocal hWitness
  exact
    ⟨hBoundary.symm, hRoleRoot.symm, hSourceRoute.symm,
      hFieldLocal.symm, hWitness.symm⟩

/-- Concrete digest of the public Rust five-source evidence report. -/
def selectedRustFiveSourceEvidenceDigest : ByteArray :=
  ByteArray.mk #[220, 228, 8, 138, 204, 164, 23, 230, 150, 121, 152, 216,
    114, 107, 80, 137, 22, 61, 113, 253, 255, 108, 27, 5, 206, 53, 252,
    131, 208, 110, 197, 145]

/-- Concrete digest of the public Rust base-boundary evidence report. -/
def selectedRustBoundaryBaseEvidenceDigest : ByteArray :=
  ByteArray.mk #[6, 242, 163, 38, 135, 152, 231, 128, 79, 124, 169, 73,
    131, 31, 237, 140, 69, 162, 33, 92, 67, 23, 21, 246, 42, 194, 70, 101,
    92, 198, 134, 169]

/-- Concrete digest of the public Rust role-root/lift evidence report. -/
def selectedRustRoleRootLiftEvidenceDigest : ByteArray :=
  ByteArray.mk #[112, 34, 58, 36, 253, 212, 82, 232, 205, 83, 113, 123,
    199, 43, 87, 194, 42, 74, 217, 204, 35, 246, 18, 154, 63, 26, 237,
    166, 20, 147, 7, 43]

/-- Concrete digest of the public Rust selected source-route evidence report. -/
def selectedRustSourceRouteEvidenceDigest : ByteArray :=
  ByteArray.mk #[76, 112, 68, 95, 197, 248, 203, 97, 213, 12, 109, 225,
    214, 130, 133, 63, 42, 106, 59, 59, 81, 82, 22, 111, 241, 35, 195,
    73, 37, 227, 43, 127]

/-- Concrete digest of the public Rust field/local evidence report. -/
def selectedRustFieldLocalEvidenceDigest : ByteArray :=
  ByteArray.mk #[169, 179, 188, 15, 137, 255, 36, 143, 125, 232, 213, 13,
    180, 213, 106, 216, 158, 75, 59, 46, 32, 23, 118, 152, 244, 151, 38,
    128, 245, 8, 172, 114]

/-- Concrete digest of the public Rust shared-frame witness evidence report. -/
def selectedRustSharedFrameWitnessEvidenceDigest : ByteArray :=
  ByteArray.mk #[231, 221, 193, 212, 120, 26, 89, 161, 44, 144, 83, 244,
    222, 142, 228, 226, 178, 182, 64, 109, 54, 74, 46, 221, 67, 139, 195,
    204, 133, 122, 244, 215]

/-- The selected public Rust evidence digests are concrete non-empty digests. -/
theorem selectedRustEvidenceDigests_nonempty :
    selectedRustFiveSourceEvidenceDigest ≠ ByteArray.mk #[] ∧
      selectedRustBoundaryBaseEvidenceDigest ≠ ByteArray.mk #[] ∧
      selectedRustRoleRootLiftEvidenceDigest ≠ ByteArray.mk #[] ∧
      selectedRustSourceRouteEvidenceDigest ≠ ByteArray.mk #[] ∧
      selectedRustFieldLocalEvidenceDigest ≠ ByteArray.mk #[] ∧
      selectedRustSharedFrameWitnessEvidenceDigest ≠ ByteArray.mk #[] := by
  decide

/--
The verifier-backed active aggregate report digest-matches any five selected
source certificates whose certificate digests are the selected public Rust
component evidence digests.

This does not construct the selected source certificates. It isolates the
remaining `sub_evidence_digest_match_proof` work to proving that those future
certificates carry the selected component digests.
-/
theorem selectedRustFiveSourceEvidenceReport_subEvidenceDigestsMatch_of_selected_certificate_digests
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    {boundaryBaseCertificate :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P}
    {roleRootLiftCertificate :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P}
    {selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P}
    {fieldLocalCertificate :
      ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P}
    {witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P}
    (hBoundaryDigest :
      boundaryBaseCertificate.evidenceDigest =
        selectedRustBoundaryBaseEvidenceDigest)
    (hRoleRootDigest :
      roleRootLiftCertificate.evidenceDigest =
        selectedRustRoleRootLiftEvidenceDigest)
    (hSourceRouteDigest :
      selectedSourceRouteCertificate.evidenceDigest =
        selectedRustSourceRouteEvidenceDigest)
    (hFieldLocalDigest :
      fieldLocalCertificate.evidenceDigest =
        selectedRustFieldLocalEvidenceDigest)
    (hWitnessDigest :
      witnessSourcesCertificate.evidenceDigest =
        selectedRustSharedFrameWitnessEvidenceDigest) :
    selectedRustFiveSourceEvidenceReport.SubEvidenceDigestsMatch
      boundaryBaseCertificate roleRootLiftCertificate
      selectedSourceRouteCertificate fieldLocalCertificate
      witnessSourcesCertificate := by
  exact
    ⟨by
      simpa [selectedRustFiveSourceEvidenceReport,
        verifierBackedSelectedRustFiveSourceEvidenceReport,
        selectedRustBoundaryBaseEvidenceDigest] using hBoundaryDigest.symm,
    by
      simpa [selectedRustFiveSourceEvidenceReport,
        verifierBackedSelectedRustFiveSourceEvidenceReport,
        selectedRustRoleRootLiftEvidenceDigest] using hRoleRootDigest.symm,
    by
      simpa [selectedRustFiveSourceEvidenceReport,
        verifierBackedSelectedRustFiveSourceEvidenceReport,
        selectedRustSourceRouteEvidenceDigest] using hSourceRouteDigest.symm,
    by
      simpa [selectedRustFiveSourceEvidenceReport,
        verifierBackedSelectedRustFiveSourceEvidenceReport,
        selectedRustFieldLocalEvidenceDigest] using hFieldLocalDigest.symm,
    by
      simpa [selectedRustFiveSourceEvidenceReport,
        verifierBackedSelectedRustFiveSourceEvidenceReport,
        selectedRustSharedFrameWitnessEvidenceDigest] using hWitnessDigest.symm⟩

/--
The current placeholder aggregate report cannot digest-match certificates whose
certificate digests are the selected public Rust evidence digests.

This is the proof-level counterpart to the fail-closed manifest state: the
named selected digest constants are non-empty, while the placeholder aggregate
report would force every matched certificate digest to be empty.
-/
theorem selectedRustFiveSourceEvidenceReport_subEvidenceDigestsMatch_incompatible_with_selected_certificate_digests
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    {boundaryBaseCertificate :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P}
    {roleRootLiftCertificate :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P}
    {selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P}
    {fieldLocalCertificate :
      ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P}
    {witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P}
    (hMatch :
      placeholderSelectedRustFiveSourceEvidenceReport.SubEvidenceDigestsMatch
        boundaryBaseCertificate roleRootLiftCertificate
        selectedSourceRouteCertificate fieldLocalCertificate
        witnessSourcesCertificate)
    (hBoundaryDigest :
      boundaryBaseCertificate.evidenceDigest =
        selectedRustBoundaryBaseEvidenceDigest)
    (hRoleRootDigest :
      roleRootLiftCertificate.evidenceDigest =
        selectedRustRoleRootLiftEvidenceDigest)
    (hSourceRouteDigest :
      selectedSourceRouteCertificate.evidenceDigest =
        selectedRustSourceRouteEvidenceDigest)
    (hFieldLocalDigest :
      fieldLocalCertificate.evidenceDigest =
        selectedRustFieldLocalEvidenceDigest)
    (hWitnessDigest :
      witnessSourcesCertificate.evidenceDigest =
        selectedRustSharedFrameWitnessEvidenceDigest) :
    False := by
  rcases
      selectedRustFiveSourceEvidenceReport_subEvidenceDigestsMatch_forces_empty_certificate_digests
        hMatch with
    ⟨hBoundaryEmpty, hRoleRootEmpty, hSourceRouteEmpty, hFieldLocalEmpty,
      hWitnessEmpty⟩
  have hSelectedCertificateDigestsEmpty :
      selectedRustBoundaryBaseEvidenceDigest = ByteArray.mk #[] ∧
        selectedRustRoleRootLiftEvidenceDigest = ByteArray.mk #[] ∧
        selectedRustSourceRouteEvidenceDigest = ByteArray.mk #[] ∧
        selectedRustFieldLocalEvidenceDigest = ByteArray.mk #[] ∧
        selectedRustSharedFrameWitnessEvidenceDigest = ByteArray.mk #[] := by
    exact
      ⟨hBoundaryDigest.symm.trans hBoundaryEmpty,
        hRoleRootDigest.symm.trans hRoleRootEmpty,
        hSourceRouteDigest.symm.trans hSourceRouteEmpty,
        hFieldLocalDigest.symm.trans hFieldLocalEmpty,
        hWitnessDigest.symm.trans hWitnessEmpty⟩
  exact selectedRustEvidenceDigests_nonempty.2.1
    hSelectedCertificateDigestsEmpty.1

/--
The current placeholder aggregate report cannot supply the checklist's
sub-evidence digest-match proof for certificates carrying the selected public
Rust evidence digests.

This is the named blocker for the `sub_evidence_digest_match_proof` manifest
category. The future proof must import verifier-backed aggregate evidence whose
sub-digests equal the selected component evidence digests.
-/
theorem selectedRustCurrentPlaceholderSubEvidenceDigestMatchProof_impossible
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    {boundaryBaseCertificate :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P}
    {roleRootLiftCertificate :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P}
    {selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P}
    {fieldLocalCertificate :
      ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P}
    {witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P}
    (hMatch :
      placeholderSelectedRustFiveSourceEvidenceReport.SubEvidenceDigestsMatch
        boundaryBaseCertificate roleRootLiftCertificate
        selectedSourceRouteCertificate fieldLocalCertificate
        witnessSourcesCertificate)
    (hBoundaryDigest :
      boundaryBaseCertificate.evidenceDigest =
        selectedRustBoundaryBaseEvidenceDigest)
    (hRoleRootDigest :
      roleRootLiftCertificate.evidenceDigest =
        selectedRustRoleRootLiftEvidenceDigest)
    (hSourceRouteDigest :
      selectedSourceRouteCertificate.evidenceDigest =
        selectedRustSourceRouteEvidenceDigest)
    (hFieldLocalDigest :
      fieldLocalCertificate.evidenceDigest =
        selectedRustFieldLocalEvidenceDigest)
    (hWitnessDigest :
      witnessSourcesCertificate.evidenceDigest =
        selectedRustSharedFrameWitnessEvidenceDigest) :
    False := by
  exact
    selectedRustFiveSourceEvidenceReport_subEvidenceDigestsMatch_incompatible_with_selected_certificate_digests
      hMatch hBoundaryDigest hRoleRootDigest hSourceRouteDigest
      hFieldLocalDigest hWitnessDigest

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
  evidenceDigest : ByteArray
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

/--
No final selected Rust/model certificate can carry the current placeholder
aggregate evidence report.

The certificate contains a proof of `ReportVerified`; the placeholder report is
proved not to satisfy that predicate.
-/
theorem selectedRustModelFiveSourceCertificate_not_from_current_placeholder_report
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Cert :
      ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
        FieldParser RouteParser CanonicalParser WitnessParser P)
    (hReport :
      Cert.evidenceReport = placeholderSelectedRustFiveSourceEvidenceReport) :
    False := by
  exact placeholderSelectedRustFiveSourceEvidenceReport_not_reportVerified
    (hReport ▸ Cert.evidenceReportVerifiedProof)

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
    (evidenceDigest : ByteArray)
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
  evidenceDigest := evidenceDigest
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
Named constructor for the final selected Rust/model five-source certificate
when the selected source-route byte source itself is supplied by a public
evidence-backed certificate.
-/
def ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSourceRouteCertificate
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustFiveSourceClosureEvidence)
    (evidenceDigest : ByteArray)
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
    (selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (witnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSources
    evidenceReport evidenceDigest evidenceReportVerifiedProof evidenceFactsAllTrueProof
    payloadProjectsFrom certificateAvailability roleRootLiftSource
    selectedSourceRouteCertificate.toByteSource assemblySource compatibilitySource
    witnessSources

/--
Named constructor for the final selected Rust/model five-source certificate
when both byte-shaped source records are supplied by public evidence-backed
certificates.
-/
def ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSourceRouteAndWitnessCertificates
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustFiveSourceClosureEvidence)
    (evidenceDigest : ByteArray)
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
    (selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSources
    evidenceReport evidenceDigest evidenceReportVerifiedProof evidenceFactsAllTrueProof
    payloadProjectsFrom certificateAvailability roleRootLiftSource
    selectedSourceRouteCertificate.toByteSource assemblySource compatibilitySource
    witnessSourcesCertificate.toWitnessSources

/--
Named constructor for the final selected Rust/model five-source certificate
when the byte-shaped and field/local source records are supplied by public
evidence-backed certificates.
-/
def ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSourceRouteFieldLocalAndWitnessCertificates
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustFiveSourceClosureEvidence)
    (evidenceDigest : ByteArray)
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
    (selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P)
    (fieldLocalCertificate :
      ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P)
    (witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSources
    evidenceReport evidenceDigest evidenceReportVerifiedProof evidenceFactsAllTrueProof
    payloadProjectsFrom certificateAvailability roleRootLiftSource
    selectedSourceRouteCertificate.toByteSource
    fieldLocalCertificate.toAssemblySource
    fieldLocalCertificate.toCompatibilitySource
    witnessSourcesCertificate.toWitnessSources

/--
Named constructor for the final selected Rust/model five-source certificate
when the role-root/lift source and all byte/local source records are supplied
by public evidence-backed certificates.
-/
def ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndRoleRootSourceRouteFieldLocalAndWitnessCertificates
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustFiveSourceClosureEvidence)
    (evidenceDigest : ByteArray)
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
    (roleRootLiftCertificate :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P)
    (selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P)
    (fieldLocalCertificate :
      ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P)
    (witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSources
    evidenceReport evidenceDigest evidenceReportVerifiedProof evidenceFactsAllTrueProof
    payloadProjectsFrom certificateAvailability
    roleRootLiftCertificate.toRoleRootLiftSource
    selectedSourceRouteCertificate.toByteSource
    fieldLocalCertificate.toAssemblySource
    fieldLocalCertificate.toCompatibilitySource
    witnessSourcesCertificate.toWitnessSources

/--
Named constructor for the final selected Rust/model five-source certificate
when every AP2-N001 source record is supplied by a public evidence-backed
certificate.
-/
def ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndBoundaryBaseRoleRootSourceRouteFieldLocalAndWitnessCertificates
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustFiveSourceClosureEvidence)
    (evidenceDigest : ByteArray)
    (evidenceReportVerifiedProof :
      evidenceReport.ReportVerified)
    (evidenceFactsAllTrueProof :
      evidenceReport.FactsAllTrue)
    (boundaryBaseCertificate :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P)
    (roleRootLiftCertificate :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P)
    (selectedSourceRouteCertificate :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P)
    (fieldLocalCertificate :
      ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P)
    (witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    evidenceReport.SubEvidenceDigestsMatch
      boundaryBaseCertificate roleRootLiftCertificate
      selectedSourceRouteCertificate fieldLocalCertificate
      witnessSourcesCertificate ->
    ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  fun subEvidenceDigestsMatchProof =>
  let _hSubEvidenceDigestsMatch :
      evidenceReport.SubEvidenceDigestsMatch
        boundaryBaseCertificate roleRootLiftCertificate
        selectedSourceRouteCertificate fieldLocalCertificate
        witnessSourcesCertificate :=
    subEvidenceDigestsMatchProof
  ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSources
    evidenceReport evidenceDigest evidenceReportVerifiedProof evidenceFactsAllTrueProof
    boundaryBaseCertificate.toPayloadProjectsFrom
    boundaryBaseCertificate.toCertificateAvailability
    roleRootLiftCertificate.toRoleRootLiftSource
    selectedSourceRouteCertificate.toByteSource
    fieldLocalCertificate.toAssemblySource
    fieldLocalCertificate.toCompatibilitySource
    witnessSourcesCertificate.toWitnessSources

/--
Selected Rust/model source-record package before final aggregate-digest
closure.

This is the proof-facing shape of the remaining `source_record_definitions`
work.  It contains exactly the seven raw selected source records that must be
imported from the active Rust/model route.  The namespace projections below
show that, once these raw records exist, the five public evidence certificates
and the aggregate sub-evidence digest match are constructor consequences.
-/
structure ReviewedFq12SelectedRustModelSourceRecordPackage
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  payloadProjectionLaw :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload},
      P.reviewedFq12PayloadAccepts b payload ->
        payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
          payload.slotId = b.context.slotId ->
        P.reviewedFq12PayloadProjectsFrom b payload
  extractionCertificateAvailability :
    ReviewedFq12ExtractionCertificateAvailability P
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSourceRouteByteSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  assemblySource :
    ReviewedFq12FieldConcreteAssemblyLocalSource P
  compatibilitySource :
    ReviewedFq12FieldConcreteCompatibilitySource P
  sharedFrameWitnessSources :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
      CanonicalParser WitnessParser P

namespace ReviewedFq12SelectedRustModelSourceRecordPackage

/--
Named constructor for the selected Rust/model source-record package.

The final AP2-N001 route should import the seven selected source records
through this explicit package constructor before deriving certificates or
hidden-source closure.
-/
def ofSources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (payloadProjectionLaw :
      forall {b : Path2NativeBoundary.Boundary}
          {payload : ReviewedFq12HiddenSourcePayload},
        P.reviewedFq12PayloadAccepts b payload ->
          payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
            payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload)
    (extractionCertificateAvailability :
      ReviewedFq12ExtractionCertificateAvailability P)
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSourceRouteByteSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (sharedFrameWitnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustModelSourceRecordPackage
      FieldParser RouteParser CanonicalParser WitnessParser P where
  payloadProjectionLaw := payloadProjectionLaw
  extractionCertificateAvailability := extractionCertificateAvailability
  roleRootLiftSource := roleRootLiftSource
  selectedSourceRouteByteSource := selectedSourceRouteByteSource
  assemblySource := assemblySource
  compatibilitySource := compatibilitySource
  sharedFrameWitnessSources := sharedFrameWitnessSources

/-- Constructor-bound boundary/base certificate from selected raw sources. -/
def boundaryBaseCertificate
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P :=
  ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate.ofReportAndSources
    selectedRustBoundaryBaseEvidenceReport
    selectedRustBoundaryBaseEvidenceDigest
    selectedRustBoundaryBaseEvidenceReportVerifiedProof
    selectedRustBoundaryBaseEvidenceFactsAllTrueProof
    Package.payloadProjectionLaw
    Package.extractionCertificateAvailability

/-- Constructor-bound role-root/lift certificate from selected raw sources. -/
def roleRootLiftSourceCertificate
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P :=
  ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate.ofReportAndSource
    selectedRustRoleRootLiftEvidenceReport
    selectedRustRoleRootLiftEvidenceDigest
    selectedRustRoleRootLiftEvidenceReportVerifiedProof
    selectedRustRoleRootLiftEvidenceFactsAllTrueProof
    Package.roleRootLiftSource

/-- Constructor-bound source-route byte-source certificate. -/
def sourceRouteByteSourceCertificate
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
      FieldParser RouteParser P :=
  SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate.ofReportAndSource
    selectedRustSourceRouteEvidenceReport
    selectedRustSourceRouteEvidenceDigest
    selectedRustSourceRouteEvidenceReportVerifiedProof
    selectedRustSourceRouteEvidenceFactsAllTrueProof
    Package.selectedSourceRouteByteSource

/-- Constructor-bound field/local certificate. -/
def fieldLocalSourceCertificate
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P :=
  ReviewedFq12SelectedRustFieldLocalEvidenceCertificate.ofReportAndSources
    selectedRustFieldLocalEvidenceReport
    selectedRustFieldLocalEvidenceDigest
    selectedRustFieldLocalEvidenceReportVerifiedProof
    selectedRustFieldLocalEvidenceFactsAllTrueProof
    Package.assemblySource
    Package.compatibilitySource

/-- Constructor-bound shared-frame witness certificate. -/
def sharedFrameWitnessSourcesCertificate
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
      CanonicalParser WitnessParser P :=
  ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate.ofReportAndSources
    selectedRustSharedFrameWitnessEvidenceReport
    selectedRustSharedFrameWitnessEvidenceDigest
    selectedRustSharedFrameWitnessEvidenceReportVerifiedProof
    selectedRustSharedFrameWitnessEvidenceFactsAllTrueProof
    Package.sharedFrameWitnessSources

/--
The selected aggregate report digest-matches the constructor-bound certificates
derived from a selected source-record package.
-/
theorem subEvidenceDigestsMatch
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    selectedRustFiveSourceEvidenceReport.SubEvidenceDigestsMatch
      Package.boundaryBaseCertificate
      Package.roleRootLiftSourceCertificate
      Package.sourceRouteByteSourceCertificate
      Package.fieldLocalSourceCertificate
      Package.sharedFrameWitnessSourcesCertificate :=
  selectedRustFiveSourceEvidenceReport_subEvidenceDigestsMatch_of_selected_certificate_digests
    (by rfl) (by rfl) (by rfl) (by rfl) (by rfl)

/--
Forget a selected source-record package into the active five-source closure
input record.

This is only a projection. It does not instantiate any selected source; it
keeps the future Rust/model import surface as one package while exposing the
existing closure theorem shape.
-/
def toFiveSourceClosureInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustModelFiveSourceClosureInputs
      FieldParser RouteParser CanonicalParser WitnessParser P where
  payloadProjectsFrom := Package.payloadProjectionLaw
  certificateAvailability :=
    Package.extractionCertificateAvailability
  roleRootLiftSource := Package.roleRootLiftSource
  selectedSource := Package.selectedSourceRouteByteSource
  assemblySource := Package.assemblySource
  compatibilitySource := Package.compatibilitySource
  witnessSources := Package.sharedFrameWitnessSources

/--
Forget a selected source-record package into the active byte-component input
record used by the field-local shared-frame theorem.

This confirms that the package fields are exactly the implementation-oriented
AP2-N001 component surface: role-root/lift, selected source-route bytes,
assembly local source, compatibility local source, and shared-frame witness
sources.
-/
def toByteComponentInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputs
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  reviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputsOfSources
    Package.roleRootLiftSource
    Package.selectedSourceRouteByteSource
    Package.assemblySource
    Package.compatibilitySource
    Package.sharedFrameWitnessSources

/--
Forget a selected source-record package into the preferred field/local
shared-frame input record.

This records the active implementation-oriented AP2-N001 route directly:
the source package supplies role-root/lift, selected source-route bytes,
assembly and compatibility local sources, and the shared digest-frame witness
bundle consumed by
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_inputs`.
-/
def toFieldLocalSharedDigestFrameWitnessInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  reviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputsOfSources
    Package.roleRootLiftSource
    Package.selectedSourceRouteByteSource
    Package.assemblySource
    Package.compatibilitySource
    Package.sharedFrameWitnessSources

end ReviewedFq12SelectedRustModelSourceRecordPackage

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
Active AP2-N001 boundary closure from the selected source-record package.

This is the same fail-closed route as the raw-source theorem above, with the
seven selected sources kept in the package shape tracked by the Rust manifest.
It should only be used after the concrete selected package is genuinely
imported; evidence reports alone do not construct such a package.
-/
theorem reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_source_record_package
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12HiddenSourceProofSemanticsClosed P :=
  reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_shared_digest_frame_witness_sources
    P Package.payloadProjectionLaw
    Package.extractionCertificateAvailability
    Package.roleRootLiftSource
    Package.selectedSourceRouteByteSource
    Package.assemblySource
    Package.compatibilitySource
    Package.sharedFrameWitnessSources

/--
Concrete predicate-model obligation for accepted reviewed-Fq12 payloads.

Acceptance must be tied to the native boundary being proven, not merely to a
free-standing reviewed-Fq12 statement.
-/
def ReviewedFq12PayloadAcceptanceMatchesBoundary
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload},
    P.reviewedFq12PayloadAccepts b payload ->
      payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
        payload.slotId = b.context.slotId

/--
Concrete predicate-model obligation for selected extraction certificates.

The certificate relation must bind back to the same native boundary and the
same reviewed/creation statement digests carried by the accepted payload.
-/
def ReviewedFq12ExtractionCertificateMatchesPayload
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate},
    P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
      cert.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
        cert.slotId = b.context.slotId ∧
          cert.creationStatementDigest = payload.creationStatementDigest ∧
            cert.reviewedStatementDigest = payload.statementDigest

/--
Concrete predicate-model obligation for valid selected extraction
certificates.

The validity predicate is consumed separately from certificate availability, so
it must carry the same native-boundary and payload binding discipline.
-/
def ReviewedFq12ExtractionCertificateValidityMatchesPayload
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate},
    P.reviewedFq12ExtractionCertificateValid b payload cert ->
      cert.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
        cert.slotId = b.context.slotId ∧
          cert.creationStatementDigest = payload.creationStatementDigest ∧
            cert.reviewedStatementDigest = payload.statementDigest

/--
Concrete predicate-model obligation connecting certificate extraction to
payload extraction.

The native soundness statement consumes both predicates for the same hidden
representative, so the selected model must not let certificate extraction drift
away from the payload extraction relation it is supposed to justify.
-/
def ReviewedFq12CertificateExtractionImpliesPayloadExtraction
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {h : HiddenRepresentative},
    P.reviewedFq12CertificateExtracts b payload cert h ->
      P.reviewedFq12PayloadExtracts b payload h

/--
Concrete predicate-model obligation that payload extraction lands in the native
boundary fiber.

This prevents a selected model from satisfying payload extraction with a hidden
representative unrelated to the boundary whose digest/slot the payload claims.
-/
def ReviewedFq12PayloadExtractionInBoundaryFiber
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {h : HiddenRepresentative},
    P.reviewedFq12PayloadExtracts b payload h ->
      InFiber P b.context b h

/--
Concrete predicate-model obligation that payload extraction satisfies the
native local and descent predicates for the boundary context.

This keeps the selected extraction relation aligned with the native validity
conditions consumed by `ReviewedFq12HiddenSourcePayloadSoundness`.
-/
def ReviewedFq12PayloadExtractionNativeValid
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {h : HiddenRepresentative},
    P.reviewedFq12PayloadExtracts b payload h ->
      P.localValid b.context h ∧
        P.descentConsistent b.context h

/--
Concrete predicate-model obligation for valid role-lift certificates.

The role-lift validity predicate must bind the full native opening certificate
back to the selected extraction certificate and payload that produced it.
-/
def ReviewedFq12RoleLiftValidityBindsExtractionTarget
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {liftCert : ReviewedFq12FullNativeOpeningCertificate},
    P.reviewedFq12RoleLiftCertificateValid b payload cert liftCert ->
      ReviewedFq12RoleLiftCertificateBindsExtraction payload cert liftCert

/--
Concrete predicate-model obligation connecting role-lift opening to the
certificate-lift relation consumed by downstream native-opening obligations.
-/
def ReviewedFq12RoleLiftOpeningImpliesCertificateLiftTarget
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {liftCert : ReviewedFq12FullNativeOpeningCertificate}
      {opening : FullNativeRepresentativeOpening},
    P.reviewedFq12RoleLiftCertificateOpens
        b payload cert liftCert opening ->
      P.reviewedFq12CertificateLiftsToOpening b payload cert opening

/--
Concrete predicate-model obligation for valid native assembly certificates.

Assembly-certificate validity must bind the certificate to the same payload,
extraction certificate, and full native opening that it is claimed to assemble.
-/
def ReviewedFq12AssemblyValidityBindsOpeningTarget
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
    P.reviewedFq12AssemblyCertificateValid
        b payload cert opening assemblyCert ->
      ReviewedFq12AssemblyCertificateBindsOpening
        payload cert opening assemblyCert

/--
Concrete predicate-model obligation connecting assembly-certificate assembly to
full-native-opening assembly for the same hidden representative.
-/
def ReviewedFq12AssemblyCertificateImpliesFullNativeAssemblyTarget
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
      {h : HiddenRepresentative},
    P.reviewedFq12AssemblyCertificateAssembles
        b payload cert opening assemblyCert h ->
      P.fullNativeOpeningAssemblesRepresentative b payload cert opening h

/--
Concrete predicate-model obligation for valid extraction-compatibility
certificates.

Compatibility-certificate validity must bind the certificate to the same
payload, extraction certificate, full native opening, and assembled hidden
representative.
-/
def ReviewedFq12CompatibilityValidityBindsTarget
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative}
      {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
    P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert opening h compatCert ->
      ReviewedFq12ExtractionCompatibilityCertificateBinds
        payload cert opening h compatCert

/--
Concrete predicate-model obligation connecting compatibility-certificate
extraction to the two reviewed-Fq12 extraction predicates for the same hidden
representative.
-/
def ReviewedFq12CompatibilityExtractionImpliesReviewedExtractionTarget
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative}
      {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
    P.reviewedFq12ExtractionCompatibilityCertificateExtracts
        b payload cert opening h compatCert ->
      P.reviewedFq12CertificateExtracts b payload cert h ∧
        P.reviewedFq12PayloadExtracts b payload h

/--
Concrete predicate-model obligation for valid aggregate full-opening bridge
digest certificates.

The aggregate bridge object is only meaningful when its validity predicate
enforces the byte-level digest binding across payload, extraction, role-lift,
assembly, and compatibility certificates.
-/
def ReviewedFq12FullOpeningBridgeDigestValidityBindsTarget
    (P : Predicates) : Prop :=
  forall {b : Path2NativeBoundary.Boundary}
      {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {liftCert : ReviewedFq12FullNativeOpeningCertificate}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
      {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
      {bridgeCert : ReviewedFq12FullOpeningBridgeDigestCertificate},
    P.reviewedFq12FullOpeningBridgeDigestCertificateValid
        b payload cert liftCert assemblyCert compatCert bridgeCert ->
      ReviewedFq12FullOpeningBridgeDigestCertificateBinds
        payload cert liftCert assemblyCert compatCert bridgeCert

/--
Public evidence report for the selected Rust native-boundary predicate model.

This report is the structured replacement target for the remaining generic
model labels.  It still does not construct the predicate interpretation; a
closing proof must provide a certificate tying this report to the concrete
`Path2NativeBoundary.Predicates` value used by the active verifier.
-/
structure ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport
    where
  schemaMatches : Bool
  reportDigestVerified : Bool
  activeBoundaryPredicateModelPinned : Bool
  payloadAcceptanceModelPinned : Bool
  extractionCertificateModelPinned : Bool
  fullOpeningAndAssemblyModelPinned : Bool
  compatibilityAndExtractionModelPinned : Bool
  nativeBoundaryBreakModelPinned : Bool
  nativeBoundaryPredicateRecordComplete : Bool
  activeVerifierRoutePinned : Bool
  predicateNegativeControlsPassed : Bool
  concretePredicateModelPresent : Bool
  certifiedSemanticsSourcePackageTargetPresent : Bool

namespace ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport

/-- The selected predicate-model report has been schema/digest checked. -/
def ReportVerified
    (E :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport) :
    Prop :=
  E.schemaMatches = true ∧
    E.reportDigestVerified = true

/-- All public selected predicate-model facts recorded by the report hold. -/
def FactsAllTrue
    (E :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport) :
    Prop :=
  E.activeBoundaryPredicateModelPinned = true ∧
    E.payloadAcceptanceModelPinned = true ∧
      E.extractionCertificateModelPinned = true ∧
        E.fullOpeningAndAssemblyModelPinned = true ∧
          E.compatibilityAndExtractionModelPinned = true ∧
            E.nativeBoundaryBreakModelPinned = true ∧
              E.nativeBoundaryPredicateRecordComplete = true ∧
                E.activeVerifierRoutePinned = true ∧
                  E.predicateNegativeControlsPassed = true ∧
                    E.concretePredicateModelPresent = true ∧
                      E.certifiedSemanticsSourcePackageTargetPresent = true

end ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport

/--
Selected Rust native-boundary predicate-model report mirrored from the public
Rust evidence.

This report is verified but intentionally not fact-complete: the concrete
native-boundary predicate model has not been imported yet.
-/
def selectedRustNativeBoundaryPredicateModelEvidenceReport :
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport where
  schemaMatches := true
  reportDigestVerified := true
  activeBoundaryPredicateModelPinned := false
  payloadAcceptanceModelPinned := false
  extractionCertificateModelPinned := false
  fullOpeningAndAssemblyModelPinned := false
  compatibilityAndExtractionModelPinned := false
  nativeBoundaryBreakModelPinned := false
  nativeBoundaryPredicateRecordComplete := false
  activeVerifierRoutePinned := true
  predicateNegativeControlsPassed := true
  concretePredicateModelPresent := false
  certifiedSemanticsSourcePackageTargetPresent := true

/-- Concrete digest of the public Rust native-boundary predicate-model report. -/
def selectedRustNativeBoundaryPredicateModelEvidenceDigest : ByteArray :=
  ByteArray.mk #[127, 52, 141, 26, 78, 247, 222, 18, 63, 134, 176, 207,
    88, 251, 221, 115, 73, 21, 181, 123, 98, 63, 121, 253, 200, 102, 198,
    30, 58, 155, 238, 74]

/-- The selected predicate-model report is schema/digest verified. -/
theorem selectedRustNativeBoundaryPredicateModelEvidenceReportVerifiedProof :
    selectedRustNativeBoundaryPredicateModelEvidenceReport.ReportVerified := by
  simp [selectedRustNativeBoundaryPredicateModelEvidenceReport,
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport.ReportVerified]

/--
The current selected predicate-model report cannot supply the facts proof
needed by the final certificate.
-/
theorem selectedRustNativeBoundaryPredicateModelEvidenceFactsAllTrue_impossible :
    ¬ selectedRustNativeBoundaryPredicateModelEvidenceReport.FactsAllTrue := by
  intro hFacts
  exact Bool.false_eq_true.mp hFacts.1

/--
Selected Rust native-boundary predicate-model import target.

The full-spend concrete predicates are not this object: they live over concrete
spend tuples.  The hidden-source boundary needs a predicate interpretation over
`Path2NativeBoundary.Boundary`, reviewed-Fq12 payloads, extraction
certificates, full native openings, assembly certificates, compatibility
certificates, transparent views, traces, and slot authorizations.  This source
record names that active interpretation without defining it by vacuous
acceptance or universal breaks.
-/
structure ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource where
  predicates : Predicates
  payloadAcceptanceNotVacuous :
    ¬ (forall b payload, ¬ predicates.reviewedFq12PayloadAccepts b payload)
  boundaryBreaksNotUniversal :
    ¬ (forall c b breakEvent, predicates.boundaryBreakFor c b breakEvent)
  payloadAcceptanceMatchesBoundary :
    ReviewedFq12PayloadAcceptanceMatchesBoundary predicates
  extractionCertificateMatchesPayload :
    ReviewedFq12ExtractionCertificateMatchesPayload predicates
  extractionCertificateValidityMatchesPayload :
    ReviewedFq12ExtractionCertificateValidityMatchesPayload predicates
  certificateExtractionImpliesPayloadExtraction :
    ReviewedFq12CertificateExtractionImpliesPayloadExtraction predicates
  payloadExtractionInBoundaryFiber :
    ReviewedFq12PayloadExtractionInBoundaryFiber predicates
  payloadExtractionNativeValid :
    ReviewedFq12PayloadExtractionNativeValid predicates
  roleLiftValidityBindsExtraction :
    ReviewedFq12RoleLiftValidityBindsExtractionTarget predicates
  roleLiftOpeningImpliesCertificateLift :
    ReviewedFq12RoleLiftOpeningImpliesCertificateLiftTarget predicates
  assemblyValidityBindsOpening :
    ReviewedFq12AssemblyValidityBindsOpeningTarget predicates
  assemblyCertificateImpliesFullNativeAssembly :
    ReviewedFq12AssemblyCertificateImpliesFullNativeAssemblyTarget predicates
  compatibilityValidityBinds :
    ReviewedFq12CompatibilityValidityBindsTarget predicates
  compatibilityExtractionImpliesReviewedExtraction :
    ReviewedFq12CompatibilityExtractionImpliesReviewedExtractionTarget
      predicates
  fullOpeningBridgeDigestValidityBinds :
    ReviewedFq12FullOpeningBridgeDigestValidityBindsTarget predicates

namespace ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource

/-- The predicate model consumed by the selected source-record package. -/
def toPredicates
    (Model : ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource) :
    Predicates :=
  Model.predicates

end ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource

/--
Proof-bearing selected Rust native-boundary predicate-model certificate.

This is the object a cryptographically closed AP2-N001 route should import
before constructing the semantics source package: public report/digest proofs,
the concrete predicate interpretation, and every non-vacuity/binding obligation
needed by the hidden-source boundary.
-/
structure ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate
    where
  evidenceReport :
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport
  evidenceDigest : ByteArray
  evidenceReportVerifiedProof :
    evidenceReport.ReportVerified
  evidenceFactsAllTrueProof :
    evidenceReport.FactsAllTrue
  predicates : Predicates
  payloadAcceptanceNotVacuous :
    ¬ (forall b payload, ¬ predicates.reviewedFq12PayloadAccepts b payload)
  boundaryBreaksNotUniversal :
    ¬ (forall c b breakEvent, predicates.boundaryBreakFor c b breakEvent)
  payloadAcceptanceMatchesBoundary :
    ReviewedFq12PayloadAcceptanceMatchesBoundary predicates
  extractionCertificateMatchesPayload :
    ReviewedFq12ExtractionCertificateMatchesPayload predicates
  extractionCertificateValidityMatchesPayload :
    ReviewedFq12ExtractionCertificateValidityMatchesPayload predicates
  certificateExtractionImpliesPayloadExtraction :
    ReviewedFq12CertificateExtractionImpliesPayloadExtraction predicates
  payloadExtractionInBoundaryFiber :
    ReviewedFq12PayloadExtractionInBoundaryFiber predicates
  payloadExtractionNativeValid :
    ReviewedFq12PayloadExtractionNativeValid predicates
  roleLiftValidityBindsExtraction :
    ReviewedFq12RoleLiftValidityBindsExtractionTarget predicates
  roleLiftOpeningImpliesCertificateLift :
    ReviewedFq12RoleLiftOpeningImpliesCertificateLiftTarget predicates
  assemblyValidityBindsOpening :
    ReviewedFq12AssemblyValidityBindsOpeningTarget predicates
  assemblyCertificateImpliesFullNativeAssembly :
    ReviewedFq12AssemblyCertificateImpliesFullNativeAssemblyTarget predicates
  compatibilityValidityBinds :
    ReviewedFq12CompatibilityValidityBindsTarget predicates
  compatibilityExtractionImpliesReviewedExtraction :
    ReviewedFq12CompatibilityExtractionImpliesReviewedExtractionTarget
      predicates
  fullOpeningBridgeDigestValidityBinds :
    ReviewedFq12FullOpeningBridgeDigestValidityBindsTarget predicates

namespace ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate

/--
Named constructor for the selected Rust native-boundary predicate-model
certificate.

The final AP2-N001 import should build the predicate model through this
public report/digest/proof surface and the concrete model obligations, not by
supplying an anonymous certificate record.
-/
def ofReportAndModel
    (evidenceReport :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport)
    (evidenceDigest : ByteArray)
    (evidenceReportVerifiedProof :
      evidenceReport.ReportVerified)
    (evidenceFactsAllTrueProof :
      evidenceReport.FactsAllTrue)
    (predicates : Predicates)
    (payloadAcceptanceNotVacuous :
      ¬ (forall b payload, ¬ predicates.reviewedFq12PayloadAccepts b payload))
    (boundaryBreaksNotUniversal :
      ¬ (forall c b breakEvent, predicates.boundaryBreakFor c b breakEvent))
    (payloadAcceptanceMatchesBoundary :
      ReviewedFq12PayloadAcceptanceMatchesBoundary predicates)
    (extractionCertificateMatchesPayload :
      ReviewedFq12ExtractionCertificateMatchesPayload predicates)
    (extractionCertificateValidityMatchesPayload :
      ReviewedFq12ExtractionCertificateValidityMatchesPayload predicates)
    (certificateExtractionImpliesPayloadExtraction :
      ReviewedFq12CertificateExtractionImpliesPayloadExtraction predicates)
    (payloadExtractionInBoundaryFiber :
      ReviewedFq12PayloadExtractionInBoundaryFiber predicates)
    (payloadExtractionNativeValid :
      ReviewedFq12PayloadExtractionNativeValid predicates)
    (roleLiftValidityBindsExtraction :
      ReviewedFq12RoleLiftValidityBindsExtractionTarget predicates)
    (roleLiftOpeningImpliesCertificateLift :
      ReviewedFq12RoleLiftOpeningImpliesCertificateLiftTarget predicates)
    (assemblyValidityBindsOpening :
      ReviewedFq12AssemblyValidityBindsOpeningTarget predicates)
    (assemblyCertificateImpliesFullNativeAssembly :
      ReviewedFq12AssemblyCertificateImpliesFullNativeAssemblyTarget predicates)
    (compatibilityValidityBinds :
      ReviewedFq12CompatibilityValidityBindsTarget predicates)
    (compatibilityExtractionImpliesReviewedExtraction :
      ReviewedFq12CompatibilityExtractionImpliesReviewedExtractionTarget
        predicates)
    (fullOpeningBridgeDigestValidityBinds :
      ReviewedFq12FullOpeningBridgeDigestValidityBindsTarget predicates) :
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  evidenceReportVerifiedProof := evidenceReportVerifiedProof
  evidenceFactsAllTrueProof := evidenceFactsAllTrueProof
  predicates := predicates
  payloadAcceptanceNotVacuous := payloadAcceptanceNotVacuous
  boundaryBreaksNotUniversal := boundaryBreaksNotUniversal
  payloadAcceptanceMatchesBoundary := payloadAcceptanceMatchesBoundary
  extractionCertificateMatchesPayload := extractionCertificateMatchesPayload
  extractionCertificateValidityMatchesPayload :=
    extractionCertificateValidityMatchesPayload
  certificateExtractionImpliesPayloadExtraction :=
    certificateExtractionImpliesPayloadExtraction
  payloadExtractionInBoundaryFiber := payloadExtractionInBoundaryFiber
  payloadExtractionNativeValid := payloadExtractionNativeValid
  roleLiftValidityBindsExtraction := roleLiftValidityBindsExtraction
  roleLiftOpeningImpliesCertificateLift :=
    roleLiftOpeningImpliesCertificateLift
  assemblyValidityBindsOpening := assemblyValidityBindsOpening
  assemblyCertificateImpliesFullNativeAssembly :=
    assemblyCertificateImpliesFullNativeAssembly
  compatibilityValidityBinds := compatibilityValidityBinds
  compatibilityExtractionImpliesReviewedExtraction :=
    compatibilityExtractionImpliesReviewedExtraction
  fullOpeningBridgeDigestValidityBinds :=
    fullOpeningBridgeDigestValidityBinds

/--
Forget a proof-bearing predicate-model certificate into the existing source
record target consumed by the semantics source package.
-/
def toSource
    (Cert :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate) :
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource where
  predicates := Cert.predicates
  payloadAcceptanceNotVacuous := Cert.payloadAcceptanceNotVacuous
  boundaryBreaksNotUniversal := Cert.boundaryBreaksNotUniversal
  payloadAcceptanceMatchesBoundary :=
    Cert.payloadAcceptanceMatchesBoundary
  extractionCertificateMatchesPayload :=
    Cert.extractionCertificateMatchesPayload
  extractionCertificateValidityMatchesPayload :=
    Cert.extractionCertificateValidityMatchesPayload
  certificateExtractionImpliesPayloadExtraction :=
    Cert.certificateExtractionImpliesPayloadExtraction
  payloadExtractionInBoundaryFiber :=
    Cert.payloadExtractionInBoundaryFiber
  payloadExtractionNativeValid :=
    Cert.payloadExtractionNativeValid
  roleLiftValidityBindsExtraction :=
    Cert.roleLiftValidityBindsExtraction
  roleLiftOpeningImpliesCertificateLift :=
    Cert.roleLiftOpeningImpliesCertificateLift
  assemblyValidityBindsOpening :=
    Cert.assemblyValidityBindsOpening
  assemblyCertificateImpliesFullNativeAssembly :=
    Cert.assemblyCertificateImpliesFullNativeAssembly
  compatibilityValidityBinds := Cert.compatibilityValidityBinds
  compatibilityExtractionImpliesReviewedExtraction :=
    Cert.compatibilityExtractionImpliesReviewedExtraction
  fullOpeningBridgeDigestValidityBinds :=
    Cert.fullOpeningBridgeDigestValidityBinds

end ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate

/--
The current selected predicate-model report cannot back a proof-bearing
predicate-model certificate.

This is the Lean-level fail-closed guard matching the manifest state: the
public report is schema/digest verified, but its concrete model facts are
false until the selected Rust native-boundary predicate interpretation is
imported.
-/
theorem selectedRustNativeBoundaryPredicateModelCertificate_from_current_report_impossible
    (Cert :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate)
    (hReport :
      Cert.evidenceReport =
        selectedRustNativeBoundaryPredicateModelEvidenceReport) :
    False := by
  exact selectedRustNativeBoundaryPredicateModelEvidenceFactsAllTrue_impossible
    (by simpa [hReport] using Cert.evidenceFactsAllTrueProof)

/--
Concrete native-boundary semantics import target for the selected Rust path.

This is the next non-mechanical object the active proof needs: not merely
verified evidence reports, but the actual native-boundary predicate
interpretation and the seven selected source records for that exact
interpretation.
-/
structure ReviewedFq12SelectedRustNativeBoundarySemanticsPackage
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser) where
  predicates : Predicates
  sourceRecords :
    ReviewedFq12SelectedRustModelSourceRecordPackage
      FieldParser RouteParser CanonicalParser WitnessParser predicates

/--
Source-shaped native-boundary semantics import target.

This variant keeps the predicate-model source explicit, then requires the
selected source records over exactly `predicateModel.toPredicates`.
-/
structure ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser) where
  predicateModel : ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource
  sourceRecords :
    ReviewedFq12SelectedRustModelSourceRecordPackage
      FieldParser RouteParser CanonicalParser WitnessParser
        predicateModel.toPredicates

/--
Certificate-shaped native-boundary semantics import target.

This is the preferred active AP2-N001 import shape: a proof-bearing selected
Rust predicate-model certificate, followed by source records for exactly the
predicate interpretation exposed through that certificate.
-/
structure ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser) where
  predicateModelCertificate :
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate
  sourceRecords :
    ReviewedFq12SelectedRustModelSourceRecordPackage
      FieldParser RouteParser CanonicalParser WitnessParser
        predicateModelCertificate.toSource.toPredicates

namespace ReviewedFq12SelectedRustNativeBoundarySemanticsPackage

/-- Hidden-source closure from a concrete native-boundary semantics package. -/
def hiddenSourceClosure
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Semantics :
      ReviewedFq12SelectedRustNativeBoundarySemanticsPackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    ReviewedFq12HiddenSourceProofSemanticsClosed Semantics.predicates :=
  reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_source_record_package
    Semantics.predicates
    Semantics.sourceRecords

end ReviewedFq12SelectedRustNativeBoundarySemanticsPackage

namespace ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage

/--
Forget the explicit predicate-model source into the existing native semantics
package.
-/
def toSemanticsPackage
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    ReviewedFq12SelectedRustNativeBoundarySemanticsPackage
      FieldParser RouteParser CanonicalParser WitnessParser where
  predicates := Package.predicateModel.toPredicates
  sourceRecords := Package.sourceRecords

/-- Hidden-source closure supplied by the source-shaped semantics package. -/
def hiddenSourceClosure
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    ReviewedFq12HiddenSourceProofSemanticsClosed
      Package.predicateModel.toPredicates :=
  (Package.toSemanticsPackage).hiddenSourceClosure

end ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage

namespace ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage

/--
Named constructor for the certificate-shaped native-boundary semantics package.

This keeps the final AP2-N001 import route explicit: a proof-bearing
predicate-model certificate and the selected source-record package for exactly
the predicate interpretation exposed by that certificate.
-/
def ofCertificateAndSourceRecords
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (predicateModelCertificate :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate)
    (sourceRecords :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser
          predicateModelCertificate.toSource.toPredicates) :
    ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage
      FieldParser RouteParser CanonicalParser WitnessParser where
  predicateModelCertificate := predicateModelCertificate
  sourceRecords := sourceRecords

/--
Forget the predicate-model certificate into the older source-shaped semantics
package.  This keeps existing closure reductions reusable while making the
preferred import boundary certificate-shaped.
-/
def toSemanticsSourcePackage
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage
      FieldParser RouteParser CanonicalParser WitnessParser where
  predicateModel := Package.predicateModelCertificate.toSource
  sourceRecords := Package.sourceRecords

/--
Hidden-source closure supplied by the certificate-shaped semantics source
package.
-/
def hiddenSourceClosure
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    ReviewedFq12HiddenSourceProofSemanticsClosed
      Package.predicateModelCertificate.toSource.toPredicates :=
  (Package.toSemanticsSourcePackage).hiddenSourceClosure

end ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage

/--
The certificate-shaped semantics package cannot be backed by the current
fail-closed selected predicate-model report.

This propagates the predicate-certificate blocker to the exact package shape
consumed by the final AP2-N001 route.
-/
theorem selectedRustNativeBoundaryCertifiedSemanticsSourcePackage_from_current_report_impossible
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser)
    (hReport :
      Package.predicateModelCertificate.evidenceReport =
        selectedRustNativeBoundaryPredicateModelEvidenceReport) :
    False :=
  selectedRustNativeBoundaryPredicateModelCertificate_from_current_report_impossible
    Package.predicateModelCertificate hReport

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
Active AP2-N001 boundary closure from the proof-bearing selected Rust/model
five-source certificate.

This keeps the public evidence report and per-source certificates on the
closure surface until the final selected Rust/model theorem explicitly derives
the concrete input record through `toClosureInputs`.
-/
theorem reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_certificate
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (Cert :
      ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12HiddenSourceProofSemanticsClosed P :=
  reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_inputs
    P
    (ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.toClosureInputs
      Cert)

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
The accepted-spend boundary field closes from the selected source-record
package.
-/
theorem with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_source_record_package
    (B : NativeAcceptedSpendSoundnessBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    (NativeAcceptedSpendSoundnessBoundary.withReviewedFq12HiddenSourceClosure
      B P).hiddenSourceProofSemanticsClosed :=
  with_reviewed_fq12_hidden_source_closure_closed
    B
    (reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_source_record_package
      P Package)

/--
The accepted-spend boundary field closes from a concrete native-boundary
semantics package.
-/
theorem with_reviewed_fq12_hidden_source_closure_from_selected_rust_native_boundary_semantics_package
    (B : NativeAcceptedSpendSoundnessBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Semantics :
      ReviewedFq12SelectedRustNativeBoundarySemanticsPackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    (NativeAcceptedSpendSoundnessBoundary.withReviewedFq12HiddenSourceClosure
      B Semantics.predicates).hiddenSourceProofSemanticsClosed :=
  with_reviewed_fq12_hidden_source_closure_closed
    B
    Semantics.hiddenSourceClosure

/--
The accepted-spend boundary field closes from a source-shaped native-boundary
semantics package.
-/
theorem with_reviewed_fq12_hidden_source_closure_from_selected_rust_native_boundary_semantics_source_package
    (B : NativeAcceptedSpendSoundnessBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    (NativeAcceptedSpendSoundnessBoundary.withReviewedFq12HiddenSourceClosure
      B Package.predicateModel.toPredicates).hiddenSourceProofSemanticsClosed :=
  with_reviewed_fq12_hidden_source_closure_closed
    B
    Package.hiddenSourceClosure

/--
The accepted-spend boundary field closes from a certificate-shaped
native-boundary semantics package.
-/
theorem with_reviewed_fq12_hidden_source_closure_from_selected_rust_native_boundary_certified_semantics_source_package
    (B : NativeAcceptedSpendSoundnessBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    (NativeAcceptedSpendSoundnessBoundary.withReviewedFq12HiddenSourceClosure
      B Package.predicateModelCertificate.toSource.toPredicates
        ).hiddenSourceProofSemanticsClosed :=
  with_reviewed_fq12_hidden_source_closure_closed
    B
    Package.hiddenSourceClosure

/--
The accepted-spend boundary field closes from the proof-bearing selected
Rust/model five-source certificate.
-/
theorem with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_five_source_certificate
    (B : NativeAcceptedSpendSoundnessBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (Cert :
      ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    (NativeAcceptedSpendSoundnessBoundary.withReviewedFq12HiddenSourceClosure
      B P).hiddenSourceProofSemanticsClosed :=
  with_reviewed_fq12_hidden_source_closure_closed
    B
    (reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_certificate
      P Cert)

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

/--
The full-signature boundary can consume the selected source-record package
directly.
-/
theorem full_signature_with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_source_record_package
    (B : NativeFullSignatureSecurityBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (Package :
      ReviewedFq12SelectedRustModelSourceRecordPackage
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B P
      (reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_source_record_package
        P Package)).acceptedSpendSoundness.hiddenSourceProofSemanticsClosed :=
  full_signature_with_reviewed_fq12_hidden_source_closure_closed
    B
    (reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_source_record_package
      P Package)

/--
The full-signature boundary can consume a concrete native-boundary semantics
package directly.
-/
theorem full_signature_with_reviewed_fq12_hidden_source_closure_from_selected_rust_native_boundary_semantics_package
    (B : NativeFullSignatureSecurityBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Semantics :
      ReviewedFq12SelectedRustNativeBoundarySemanticsPackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B Semantics.predicates
      Semantics.hiddenSourceClosure).acceptedSpendSoundness.hiddenSourceProofSemanticsClosed :=
  full_signature_with_reviewed_fq12_hidden_source_closure_closed
    B
    Semantics.hiddenSourceClosure

/--
The full-signature boundary can consume a source-shaped native-boundary
semantics package directly.
-/
theorem full_signature_with_reviewed_fq12_hidden_source_closure_from_selected_rust_native_boundary_semantics_source_package
    (B : NativeFullSignatureSecurityBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B Package.predicateModel.toPredicates
      Package.hiddenSourceClosure).acceptedSpendSoundness.hiddenSourceProofSemanticsClosed :=
  full_signature_with_reviewed_fq12_hidden_source_closure_closed
    B
    Package.hiddenSourceClosure

/--
The full-signature boundary can consume a certificate-shaped native-boundary
semantics source package directly.
-/
theorem full_signature_with_reviewed_fq12_hidden_source_closure_from_selected_rust_native_boundary_certified_semantics_source_package
    (B : NativeFullSignatureSecurityBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B Package.predicateModelCertificate.toSource.toPredicates
      Package.hiddenSourceClosure).acceptedSpendSoundness.hiddenSourceProofSemanticsClosed :=
  full_signature_with_reviewed_fq12_hidden_source_closure_closed
    B
    Package.hiddenSourceClosure

/--
Once the active native-boundary semantics package is supplied, the existing
full-signature theorem yields the active Path 2 forgery bound for the wrapped
boundary.  This is still conditional on constructing the package; it does not
introduce source records or close deployment by report evidence alone.
-/
theorem full_signature_active_path_forgery_bound_from_selected_rust_native_boundary_semantics_package
    (B : NativeFullSignatureSecurityBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Semantics :
      ReviewedFq12SelectedRustNativeBoundarySemanticsPackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B Semantics.predicates
      Semantics.hiddenSourceClosure).fullSignatureSuccessProb <=
      (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
        B Semantics.predicates
        Semantics.hiddenSourceClosure).activePathForgeryBound :=
  NativeFullSignatureSecurityBoundary.fullSignatureSuccessProb_le_activePathForgeryBound
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B Semantics.predicates
      Semantics.hiddenSourceClosure)

/--
The source-shaped native-boundary semantics package also reaches the active
Path 2 forgery bound.  This remains conditional on importing the predicate
model source and the selected source records for that exact model.
-/
theorem full_signature_active_path_forgery_bound_from_selected_rust_native_boundary_semantics_source_package
    (B : NativeFullSignatureSecurityBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B Package.predicateModel.toPredicates
      Package.hiddenSourceClosure).fullSignatureSuccessProb <=
      (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
        B Package.predicateModel.toPredicates
        Package.hiddenSourceClosure).activePathForgeryBound :=
  NativeFullSignatureSecurityBoundary.fullSignatureSuccessProb_le_activePathForgeryBound
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B Package.predicateModel.toPredicates
      Package.hiddenSourceClosure)

/--
The certificate-shaped native-boundary semantics source package also reaches
the active Path 2 forgery bound.
-/
theorem full_signature_active_path_forgery_bound_from_selected_rust_native_boundary_certified_semantics_source_package
    (B : NativeFullSignatureSecurityBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B Package.predicateModelCertificate.toSource.toPredicates
      Package.hiddenSourceClosure).fullSignatureSuccessProb <=
      (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
        B Package.predicateModelCertificate.toSource.toPredicates
        Package.hiddenSourceClosure).activePathForgeryBound :=
  NativeFullSignatureSecurityBoundary.fullSignatureSuccessProb_le_activePathForgeryBound
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B Package.predicateModelCertificate.toSource.toPredicates
      Package.hiddenSourceClosure)

/--
The full-signature boundary can consume the proof-bearing selected Rust/model
five-source certificate directly.
-/
theorem full_signature_with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_five_source_certificate
    (B : NativeFullSignatureSecurityBoundary)
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (Cert :
      ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    (NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure
      B P
      (reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_certificate
        P Cert)).acceptedSpendSoundness.hiddenSourceProofSemanticsClosed :=
  full_signature_with_reviewed_fq12_hidden_source_closure_closed
    B
    (reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_certificate
      P Cert)

end Path2HiddenSourceSemanticsBoundary
end Descender
