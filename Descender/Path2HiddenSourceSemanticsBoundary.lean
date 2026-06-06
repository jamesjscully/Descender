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
  bridgeAuditDigest := ByteArray.mk #[235, 201, 246, 244, 129, 193, 253,
    124, 55, 52, 164, 100, 196, 139, 224, 135, 206, 179, 163, 37, 183,
    102, 176, 105, 158, 90, 208, 35, 245, 223, 0, 168]
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
  ByteArray.mk #[103, 232, 155, 220, 103, 72, 115, 10, 175, 64, 120, 122,
    82, 201, 135, 255, 218, 84, 155, 145, 158, 51, 31, 130, 79, 143, 164,
    152, 224, 163, 167, 233]

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
  evidenceReportBound :
    evidenceReport = selectedRustFiveSourceEvidenceReport
  evidenceDigestBound :
    evidenceDigest = selectedRustFiveSourceEvidenceDigest
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
    (evidenceReportBound :
      evidenceReport = selectedRustFiveSourceEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustFiveSourceEvidenceDigest)
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
  evidenceReportBound := evidenceReportBound
  evidenceDigestBound := evidenceDigestBound
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
    (evidenceReportBound :
      evidenceReport = selectedRustFiveSourceEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustFiveSourceEvidenceDigest)
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
    evidenceReport evidenceDigest evidenceReportBound evidenceDigestBound
    evidenceReportVerifiedProof evidenceFactsAllTrueProof
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
    (evidenceReportBound :
      evidenceReport = selectedRustFiveSourceEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustFiveSourceEvidenceDigest)
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
    evidenceReport evidenceDigest evidenceReportBound evidenceDigestBound
    evidenceReportVerifiedProof evidenceFactsAllTrueProof
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
    (evidenceReportBound :
      evidenceReport = selectedRustFiveSourceEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustFiveSourceEvidenceDigest)
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
    evidenceReport evidenceDigest evidenceReportBound evidenceDigestBound
    evidenceReportVerifiedProof evidenceFactsAllTrueProof
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
    (evidenceReportBound :
      evidenceReport = selectedRustFiveSourceEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustFiveSourceEvidenceDigest)
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
    evidenceReport evidenceDigest evidenceReportBound evidenceDigestBound
    evidenceReportVerifiedProof evidenceFactsAllTrueProof
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
    (evidenceReportBound :
      evidenceReport = selectedRustFiveSourceEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustFiveSourceEvidenceDigest)
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
    evidenceReport evidenceDigest evidenceReportBound evidenceDigestBound
    evidenceReportVerifiedProof evidenceFactsAllTrueProof
    boundaryBaseCertificate.toPayloadProjectsFrom
    boundaryBaseCertificate.toCertificateAvailability
    roleRootLiftCertificate.toRoleRootLiftSource
    selectedSourceRouteCertificate.toByteSource
    fieldLocalCertificate.toAssemblySource
    fieldLocalCertificate.toCompatibilitySource
    witnessSourcesCertificate.toWitnessSources

/--
Evidence/source binding for the active selected base-boundary report.

This is the object the Rust/model import must eventually supply before a
selected source can be treated as audited by a selected evidence report.
-/
structure ReviewedFq12SelectedRustBoundaryBaseEvidenceSourceBinding
    (P : Predicates) where
  evidenceReport :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceReport
  evidenceReportBound :
    evidenceReport = selectedRustBoundaryBaseEvidenceReport
  evidenceDigest : ByteArray
  evidenceDigestBound :
    evidenceDigest = selectedRustBoundaryBaseEvidenceDigest
  sourceBindingDigest : ByteArray
  sourceBindingDigestBound :
    sourceBindingDigest = evidenceDigest
  payloadProjectionLaw :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload},
      P.reviewedFq12PayloadAccepts b payload ->
        payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
          payload.slotId = b.context.slotId ->
        P.reviewedFq12PayloadProjectsFrom b payload
  extractionCertificateAvailability :
    ReviewedFq12ExtractionCertificateAvailability P

namespace ReviewedFq12SelectedRustBoundaryBaseEvidenceSourceBinding

/-- Named constructor for the selected base-boundary evidence/source binding. -/
def ofReportAndSources
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceReport)
    (evidenceDigest : ByteArray)
    (sourceBindingDigest : ByteArray)
    (evidenceReportBound :
      evidenceReport = selectedRustBoundaryBaseEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustBoundaryBaseEvidenceDigest)
    (sourceBindingDigestBound : sourceBindingDigest = evidenceDigest)
    (payloadProjectionLaw :
      forall {b : Path2NativeBoundary.Boundary}
          {payload : ReviewedFq12HiddenSourcePayload},
        P.reviewedFq12PayloadAccepts b payload ->
          payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
            payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload)
    (extractionCertificateAvailability :
      ReviewedFq12ExtractionCertificateAvailability P) :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceSourceBinding P where
  evidenceReport := evidenceReport
  evidenceReportBound := evidenceReportBound
  evidenceDigest := evidenceDigest
  evidenceDigestBound := evidenceDigestBound
  sourceBindingDigest := sourceBindingDigest
  sourceBindingDigestBound := sourceBindingDigestBound
  payloadProjectionLaw := payloadProjectionLaw
  extractionCertificateAvailability := extractionCertificateAvailability

/-- Certificate projection from the selected base-boundary evidence/source binding. -/
def toCertificate
    {P : Predicates}
    (Binding : ReviewedFq12SelectedRustBoundaryBaseEvidenceSourceBinding P) :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate P :=
  ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate.ofReportAndSources
    Binding.evidenceReport
    Binding.evidenceDigest
    (by
      rw [Binding.evidenceReportBound]
      exact selectedRustBoundaryBaseEvidenceReportVerifiedProof)
    (by
      rw [Binding.evidenceReportBound]
      exact selectedRustBoundaryBaseEvidenceFactsAllTrueProof)
    Binding.payloadProjectionLaw
    Binding.extractionCertificateAvailability

end ReviewedFq12SelectedRustBoundaryBaseEvidenceSourceBinding

/-- Evidence/source binding for the selected role-root/lift report. -/
structure ReviewedFq12SelectedRustRoleRootLiftEvidenceSourceBinding
    (P : Predicates) where
  evidenceReport :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceReport
  evidenceReportBound :
    evidenceReport = selectedRustRoleRootLiftEvidenceReport
  evidenceDigest : ByteArray
  evidenceDigestBound :
    evidenceDigest = selectedRustRoleRootLiftEvidenceDigest
  sourceBindingDigest : ByteArray
  sourceBindingDigestBound :
    sourceBindingDigest = evidenceDigest
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P

namespace ReviewedFq12SelectedRustRoleRootLiftEvidenceSourceBinding

/-- Named constructor for the selected role-root/lift evidence/source binding. -/
def ofReportAndSource
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceReport)
    (evidenceDigest : ByteArray)
    (sourceBindingDigest : ByteArray)
    (evidenceReportBound :
      evidenceReport = selectedRustRoleRootLiftEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustRoleRootLiftEvidenceDigest)
    (sourceBindingDigestBound : sourceBindingDigest = evidenceDigest)
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P) :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceSourceBinding P where
  evidenceReport := evidenceReport
  evidenceReportBound := evidenceReportBound
  evidenceDigest := evidenceDigest
  evidenceDigestBound := evidenceDigestBound
  sourceBindingDigest := sourceBindingDigest
  sourceBindingDigestBound := sourceBindingDigestBound
  roleRootLiftSource := roleRootLiftSource

/-- Certificate projection from the selected role-root/lift evidence/source binding. -/
def toCertificate
    {P : Predicates}
    (Binding : ReviewedFq12SelectedRustRoleRootLiftEvidenceSourceBinding P) :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate P :=
  ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate.ofReportAndSource
    Binding.evidenceReport
    Binding.evidenceDigest
    (by
      rw [Binding.evidenceReportBound]
      exact selectedRustRoleRootLiftEvidenceReportVerifiedProof)
    (by
      rw [Binding.evidenceReportBound]
      exact selectedRustRoleRootLiftEvidenceFactsAllTrueProof)
    Binding.roleRootLiftSource

end ReviewedFq12SelectedRustRoleRootLiftEvidenceSourceBinding

/-- Evidence/source binding for the selected source-route report. -/
structure ReviewedFq12SelectedRustSourceRouteEvidenceSourceBinding
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  evidenceReport :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport
  evidenceReportBound :
    evidenceReport = selectedRustSourceRouteEvidenceReport
  evidenceDigest : ByteArray
  evidenceDigestBound :
    evidenceDigest = selectedRustSourceRouteEvidenceDigest
  sourceBindingDigest : ByteArray
  sourceBindingDigestBound :
    sourceBindingDigest = evidenceDigest
  selectedSourceRouteByteSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P

namespace ReviewedFq12SelectedRustSourceRouteEvidenceSourceBinding

/-- Named constructor for the selected source-route evidence/source binding. -/
def ofReportAndSource
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (evidenceReport :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport)
    (evidenceDigest : ByteArray)
    (sourceBindingDigest : ByteArray)
    (evidenceReportBound :
      evidenceReport = selectedRustSourceRouteEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustSourceRouteEvidenceDigest)
    (sourceBindingDigestBound : sourceBindingDigest = evidenceDigest)
    (selectedSourceRouteByteSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustSourceRouteEvidenceSourceBinding
      FieldParser RouteParser P where
  evidenceReport := evidenceReport
  evidenceReportBound := evidenceReportBound
  evidenceDigest := evidenceDigest
  evidenceDigestBound := evidenceDigestBound
  sourceBindingDigest := sourceBindingDigest
  sourceBindingDigestBound := sourceBindingDigestBound
  selectedSourceRouteByteSource := selectedSourceRouteByteSource

/-- Certificate projection from the selected source-route evidence/source binding. -/
def toCertificate
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Binding :
      ReviewedFq12SelectedRustSourceRouteEvidenceSourceBinding
        FieldParser RouteParser P) :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
      FieldParser RouteParser P :=
  SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate.ofReportAndSource
    Binding.evidenceReport
    Binding.evidenceDigest
    (by
      rw [Binding.evidenceReportBound]
      exact selectedRustSourceRouteEvidenceReportVerifiedProof)
    (by
      rw [Binding.evidenceReportBound]
      exact selectedRustSourceRouteEvidenceFactsAllTrueProof)
    Binding.selectedSourceRouteByteSource

end ReviewedFq12SelectedRustSourceRouteEvidenceSourceBinding

/-- Evidence/source binding for the selected field/local report. -/
structure ReviewedFq12SelectedRustFieldLocalEvidenceSourceBinding
    (P : Predicates) where
  evidenceReport :
    ReviewedFq12SelectedRustFieldLocalEvidenceReport
  evidenceReportBound :
    evidenceReport = selectedRustFieldLocalEvidenceReport
  evidenceDigest : ByteArray
  evidenceDigestBound :
    evidenceDigest = selectedRustFieldLocalEvidenceDigest
  sourceBindingDigest : ByteArray
  sourceBindingDigestBound :
    sourceBindingDigest = evidenceDigest
  assemblySource :
    ReviewedFq12FieldConcreteAssemblyLocalSource P
  compatibilitySource :
    ReviewedFq12FieldConcreteCompatibilitySource P

namespace ReviewedFq12SelectedRustFieldLocalEvidenceSourceBinding

/-- Named constructor for the selected field/local evidence/source binding. -/
def ofReportAndSources
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustFieldLocalEvidenceReport)
    (evidenceDigest : ByteArray)
    (sourceBindingDigest : ByteArray)
    (evidenceReportBound :
      evidenceReport = selectedRustFieldLocalEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustFieldLocalEvidenceDigest)
    (sourceBindingDigestBound : sourceBindingDigest = evidenceDigest)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P) :
    ReviewedFq12SelectedRustFieldLocalEvidenceSourceBinding P where
  evidenceReport := evidenceReport
  evidenceReportBound := evidenceReportBound
  evidenceDigest := evidenceDigest
  evidenceDigestBound := evidenceDigestBound
  sourceBindingDigest := sourceBindingDigest
  sourceBindingDigestBound := sourceBindingDigestBound
  assemblySource := assemblySource
  compatibilitySource := compatibilitySource

/-- Certificate projection from the selected field/local evidence/source binding. -/
def toCertificate
    {P : Predicates}
    (Binding : ReviewedFq12SelectedRustFieldLocalEvidenceSourceBinding P) :
    ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P :=
  ReviewedFq12SelectedRustFieldLocalEvidenceCertificate.ofReportAndSources
    Binding.evidenceReport
    Binding.evidenceDigest
    (by
      rw [Binding.evidenceReportBound]
      exact selectedRustFieldLocalEvidenceReportVerifiedProof)
    (by
      rw [Binding.evidenceReportBound]
      exact selectedRustFieldLocalEvidenceFactsAllTrueProof)
    Binding.assemblySource
    Binding.compatibilitySource

end ReviewedFq12SelectedRustFieldLocalEvidenceSourceBinding

/-- Evidence/source binding for the selected shared-frame witness report. -/
structure ReviewedFq12SelectedRustSharedWitnessEvidenceSourceBinding
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  evidenceReport :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport
  evidenceReportBound :
    evidenceReport = selectedRustSharedFrameWitnessEvidenceReport
  evidenceDigest : ByteArray
  evidenceDigestBound :
    evidenceDigest = selectedRustSharedFrameWitnessEvidenceDigest
  sourceBindingDigest : ByteArray
  sourceBindingDigestBound :
    sourceBindingDigest = evidenceDigest
  sharedFrameWitnessSources :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
      CanonicalParser WitnessParser P

namespace ReviewedFq12SelectedRustSharedWitnessEvidenceSourceBinding

/-- Named constructor for the selected shared-witness evidence/source binding. -/
def ofReportAndSources
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (evidenceReport :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport)
    (evidenceDigest : ByteArray)
    (sourceBindingDigest : ByteArray)
    (evidenceReportBound :
      evidenceReport = selectedRustSharedFrameWitnessEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustSharedFrameWitnessEvidenceDigest)
    (sourceBindingDigestBound : sourceBindingDigest = evidenceDigest)
    (sharedFrameWitnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustSharedWitnessEvidenceSourceBinding
      CanonicalParser WitnessParser P where
  evidenceReport := evidenceReport
  evidenceReportBound := evidenceReportBound
  evidenceDigest := evidenceDigest
  evidenceDigestBound := evidenceDigestBound
  sourceBindingDigest := sourceBindingDigest
  sourceBindingDigestBound := sourceBindingDigestBound
  sharedFrameWitnessSources := sharedFrameWitnessSources

/-- Certificate projection from the selected shared-witness evidence/source binding. -/
def toCertificate
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Binding :
      ReviewedFq12SelectedRustSharedWitnessEvidenceSourceBinding
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
      CanonicalParser WitnessParser P :=
  ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate.ofReportAndSources
    Binding.evidenceReport
    Binding.evidenceDigest
    (by
      rw [Binding.evidenceReportBound]
      exact selectedRustSharedFrameWitnessEvidenceReportVerifiedProof)
    (by
      rw [Binding.evidenceReportBound]
      exact selectedRustSharedFrameWitnessEvidenceFactsAllTrueProof)
    Binding.sharedFrameWitnessSources

end ReviewedFq12SelectedRustSharedWitnessEvidenceSourceBinding

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
  boundaryBaseEvidenceSourceBinding :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceSourceBinding P
  roleRootLiftEvidenceSourceBinding :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceSourceBinding P
  sourceRouteEvidenceSourceBinding :
    ReviewedFq12SelectedRustSourceRouteEvidenceSourceBinding
      FieldParser RouteParser P
  fieldLocalEvidenceSourceBinding :
    ReviewedFq12SelectedRustFieldLocalEvidenceSourceBinding P
  sharedWitnessEvidenceSourceBinding :
    ReviewedFq12SelectedRustSharedWitnessEvidenceSourceBinding
      CanonicalParser WitnessParser P
  payloadProjectionLaw :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload},
      P.reviewedFq12PayloadAccepts b payload ->
        payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
          payload.slotId = b.context.slotId ->
        P.reviewedFq12PayloadProjectsFrom b payload
  payloadProjectionLawBound :
    (payloadProjectionLaw :
      forall {b : Path2NativeBoundary.Boundary}
          {payload : ReviewedFq12HiddenSourcePayload},
        P.reviewedFq12PayloadAccepts b payload ->
          payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
            payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload) =
      (boundaryBaseEvidenceSourceBinding.payloadProjectionLaw :
        forall {b : Path2NativeBoundary.Boundary}
            {payload : ReviewedFq12HiddenSourcePayload},
          P.reviewedFq12PayloadAccepts b payload ->
            payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
              payload.slotId = b.context.slotId ->
            P.reviewedFq12PayloadProjectsFrom b payload)
  extractionCertificateAvailability :
    ReviewedFq12ExtractionCertificateAvailability P
  extractionCertificateAvailabilityBound :
    (extractionCertificateAvailability :
      ReviewedFq12ExtractionCertificateAvailability P) =
      (boundaryBaseEvidenceSourceBinding.extractionCertificateAvailability :
        ReviewedFq12ExtractionCertificateAvailability P)
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  roleRootLiftSourceBound :
    roleRootLiftSource =
      roleRootLiftEvidenceSourceBinding.roleRootLiftSource
  selectedSourceRouteByteSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  selectedSourceRouteByteSourceBound :
    selectedSourceRouteByteSource =
      sourceRouteEvidenceSourceBinding.selectedSourceRouteByteSource
  assemblySource :
    ReviewedFq12FieldConcreteAssemblyLocalSource P
  assemblySourceBound :
    assemblySource = fieldLocalEvidenceSourceBinding.assemblySource
  compatibilitySource :
    ReviewedFq12FieldConcreteCompatibilitySource P
  compatibilitySourceBound :
    compatibilitySource = fieldLocalEvidenceSourceBinding.compatibilitySource
  sharedFrameWitnessSources :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
      CanonicalParser WitnessParser P
  sharedFrameWitnessSourcesBound :
    sharedFrameWitnessSources =
      sharedWitnessEvidenceSourceBinding.sharedFrameWitnessSources

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
    (boundaryBaseEvidenceSourceBinding :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceSourceBinding P)
    (roleRootLiftEvidenceSourceBinding :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceSourceBinding P)
    (sourceRouteEvidenceSourceBinding :
      ReviewedFq12SelectedRustSourceRouteEvidenceSourceBinding
        FieldParser RouteParser P)
    (fieldLocalEvidenceSourceBinding :
      ReviewedFq12SelectedRustFieldLocalEvidenceSourceBinding P)
    (sharedWitnessEvidenceSourceBinding :
      ReviewedFq12SelectedRustSharedWitnessEvidenceSourceBinding
        CanonicalParser WitnessParser P)
    (payloadProjectionLaw :
      forall {b : Path2NativeBoundary.Boundary}
          {payload : ReviewedFq12HiddenSourcePayload},
        P.reviewedFq12PayloadAccepts b payload ->
          payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
            payload.slotId = b.context.slotId ->
          P.reviewedFq12PayloadProjectsFrom b payload)
    (payloadProjectionLawBound :
      (payloadProjectionLaw :
        forall {b : Path2NativeBoundary.Boundary}
            {payload : ReviewedFq12HiddenSourcePayload},
          P.reviewedFq12PayloadAccepts b payload ->
            payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
              payload.slotId = b.context.slotId ->
            P.reviewedFq12PayloadProjectsFrom b payload) =
        (boundaryBaseEvidenceSourceBinding.payloadProjectionLaw :
          forall {b : Path2NativeBoundary.Boundary}
              {payload : ReviewedFq12HiddenSourcePayload},
            P.reviewedFq12PayloadAccepts b payload ->
              payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
                payload.slotId = b.context.slotId ->
              P.reviewedFq12PayloadProjectsFrom b payload))
    (extractionCertificateAvailability :
      ReviewedFq12ExtractionCertificateAvailability P)
    (extractionCertificateAvailabilityBound :
      (extractionCertificateAvailability :
        ReviewedFq12ExtractionCertificateAvailability P) =
        (boundaryBaseEvidenceSourceBinding.extractionCertificateAvailability :
          ReviewedFq12ExtractionCertificateAvailability P))
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (roleRootLiftSourceBound :
      roleRootLiftSource =
        roleRootLiftEvidenceSourceBinding.roleRootLiftSource)
    (selectedSourceRouteByteSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (selectedSourceRouteByteSourceBound :
      selectedSourceRouteByteSource =
        sourceRouteEvidenceSourceBinding.selectedSourceRouteByteSource)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (assemblySourceBound :
      assemblySource = fieldLocalEvidenceSourceBinding.assemblySource)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (compatibilitySourceBound :
      compatibilitySource = fieldLocalEvidenceSourceBinding.compatibilitySource)
    (sharedFrameWitnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P)
    (sharedFrameWitnessSourcesBound :
      sharedFrameWitnessSources =
        sharedWitnessEvidenceSourceBinding.sharedFrameWitnessSources) :
    ReviewedFq12SelectedRustModelSourceRecordPackage
      FieldParser RouteParser CanonicalParser WitnessParser P where
  boundaryBaseEvidenceSourceBinding := boundaryBaseEvidenceSourceBinding
  roleRootLiftEvidenceSourceBinding := roleRootLiftEvidenceSourceBinding
  sourceRouteEvidenceSourceBinding := sourceRouteEvidenceSourceBinding
  fieldLocalEvidenceSourceBinding := fieldLocalEvidenceSourceBinding
  sharedWitnessEvidenceSourceBinding := sharedWitnessEvidenceSourceBinding
  payloadProjectionLaw := payloadProjectionLaw
  payloadProjectionLawBound := payloadProjectionLawBound
  extractionCertificateAvailability := extractionCertificateAvailability
  extractionCertificateAvailabilityBound := extractionCertificateAvailabilityBound
  roleRootLiftSource := roleRootLiftSource
  roleRootLiftSourceBound := roleRootLiftSourceBound
  selectedSourceRouteByteSource := selectedSourceRouteByteSource
  selectedSourceRouteByteSourceBound := selectedSourceRouteByteSourceBound
  assemblySource := assemblySource
  assemblySourceBound := assemblySourceBound
  compatibilitySource := compatibilitySource
  compatibilitySourceBound := compatibilitySourceBound
  sharedFrameWitnessSources := sharedFrameWitnessSources
  sharedFrameWitnessSourcesBound := sharedFrameWitnessSourcesBound

/--
Construct the selected source-record package from explicit evidence/source
bindings.

This is the preferred shape for the final AP2-N001 import: the seven source
records are not free-standing assumptions, but projections of the five public
report binding objects.
-/
def ofSourceBindings
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (boundaryBaseEvidenceSourceBinding :
      ReviewedFq12SelectedRustBoundaryBaseEvidenceSourceBinding P)
    (roleRootLiftEvidenceSourceBinding :
      ReviewedFq12SelectedRustRoleRootLiftEvidenceSourceBinding P)
    (sourceRouteEvidenceSourceBinding :
      ReviewedFq12SelectedRustSourceRouteEvidenceSourceBinding
        FieldParser RouteParser P)
    (fieldLocalEvidenceSourceBinding :
      ReviewedFq12SelectedRustFieldLocalEvidenceSourceBinding P)
    (sharedWitnessEvidenceSourceBinding :
      ReviewedFq12SelectedRustSharedWitnessEvidenceSourceBinding
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustModelSourceRecordPackage
      FieldParser RouteParser CanonicalParser WitnessParser P where
  boundaryBaseEvidenceSourceBinding := boundaryBaseEvidenceSourceBinding
  roleRootLiftEvidenceSourceBinding := roleRootLiftEvidenceSourceBinding
  sourceRouteEvidenceSourceBinding := sourceRouteEvidenceSourceBinding
  fieldLocalEvidenceSourceBinding := fieldLocalEvidenceSourceBinding
  sharedWitnessEvidenceSourceBinding := sharedWitnessEvidenceSourceBinding
  payloadProjectionLaw :=
    boundaryBaseEvidenceSourceBinding.payloadProjectionLaw
  payloadProjectionLawBound := rfl
  extractionCertificateAvailability :=
    boundaryBaseEvidenceSourceBinding.extractionCertificateAvailability
  extractionCertificateAvailabilityBound := rfl
  roleRootLiftSource :=
    roleRootLiftEvidenceSourceBinding.roleRootLiftSource
  roleRootLiftSourceBound := rfl
  selectedSourceRouteByteSource :=
    sourceRouteEvidenceSourceBinding.selectedSourceRouteByteSource
  selectedSourceRouteByteSourceBound := rfl
  assemblySource :=
    fieldLocalEvidenceSourceBinding.assemblySource
  assemblySourceBound := rfl
  compatibilitySource :=
    fieldLocalEvidenceSourceBinding.compatibilitySource
  compatibilitySourceBound := rfl
  sharedFrameWitnessSources :=
    sharedWitnessEvidenceSourceBinding.sharedFrameWitnessSources
  sharedFrameWitnessSourcesBound := rfl

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
  Package.boundaryBaseEvidenceSourceBinding.toCertificate

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
  Package.roleRootLiftEvidenceSourceBinding.toCertificate

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
  Package.sourceRouteEvidenceSourceBinding.toCertificate

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
  Package.fieldLocalEvidenceSourceBinding.toCertificate

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
  Package.sharedWitnessEvidenceSourceBinding.toCertificate

/-- The package's shared witness source is exactly the certificate projection. -/
theorem sharedFrameWitnessSourcesFromCertificate
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
    Package.sharedFrameWitnessSources =
      Package.sharedFrameWitnessSourcesCertificate.toWitnessSources := by
  rw [Package.sharedFrameWitnessSourcesBound]
  rfl

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
    Package.boundaryBaseEvidenceSourceBinding.evidenceDigestBound
    Package.roleRootLiftEvidenceSourceBinding.evidenceDigestBound
    Package.sourceRouteEvidenceSourceBinding.evidenceDigestBound
    Package.fieldLocalEvidenceSourceBinding.evidenceDigestBound
    Package.sharedWitnessEvidenceSourceBinding.evidenceDigestBound

/--
Proof-bearing five-source certificate derived from the selected source-record
package.

This makes the aggregate evidence report and sub-evidence digest match part of
the package closure route, not a side object that the final theorem can build
and then ignore.
-/
def fiveSourceClosureCertificate
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
    ReviewedFq12SelectedRustModelFiveSourceClosureCertificate
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndBoundaryBaseRoleRootSourceRouteFieldLocalAndWitnessCertificates
    selectedRustFiveSourceEvidenceReport
    selectedRustFiveSourceEvidenceDigest
    rfl
    rfl
    selectedRustFiveSourceEvidenceReportVerifiedProof
    selectedRustFiveSourceEvidenceFactsAllTrueProof
    Package.boundaryBaseCertificate
    Package.roleRootLiftSourceCertificate
    Package.sourceRouteByteSourceCertificate
    Package.fieldLocalSourceCertificate
    Package.sharedFrameWitnessSourcesCertificate
    Package.subEvidenceDigestsMatch

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
  witnessSources := Package.sharedFrameWitnessSourcesCertificate.toWitnessSources

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
    Package.sharedFrameWitnessSourcesCertificate.toWitnessSources

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
    Package.sharedFrameWitnessSourcesCertificate.toWitnessSources

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
Active AP2-N001 boundary closure from the selected source records plus the
report-bound shared witness evidence certificate.

This is the preferred certificate route for the source-record package: the
shared witness source bundle is exposed only by the public shared-witness
evidence certificate.
-/
theorem reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_shared_digest_frame_witness_certificate
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
    (witnessSourcesCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12HiddenSourceProofSemanticsClosed P :=
  reviewed_fq12_hidden_source_proof_semantics_closed_from_extraction_soundness
    P payloadProjectsFrom certificateAvailability
    (reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_certificate
      P roleRootLiftSource selectedSource assemblySource compatibilitySource
      witnessSourcesCertificate)

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
  reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_shared_digest_frame_witness_certificate
    P Package.payloadProjectionLaw Package.extractionCertificateAvailability
    Package.roleRootLiftSource Package.selectedSourceRouteByteSource
    Package.assemblySource Package.compatibilitySource
    Package.sharedFrameWitnessSourcesCertificate

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
Law bundle for the representative-producing reviewed-Fq12 provider subfamily.

These are the obligations that make the five hard extraction predicates usable
by the native hidden-source soundness theorem. They are separated from the
full predicate model so the active AP2-N001 route can close this high-risk
slice before the remaining native-boundary predicates are fully instantiated.
-/
structure ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelLaws
    (P : Predicates) where
  certificateExtractionImpliesPayloadExtraction :
    ReviewedFq12CertificateExtractionImpliesPayloadExtraction P
  payloadExtractionInBoundaryFiber :
    ReviewedFq12PayloadExtractionInBoundaryFiber P
  payloadExtractionNativeValid :
    ReviewedFq12PayloadExtractionNativeValid P
  assemblyCertificateImpliesFullNativeAssembly :
    ReviewedFq12AssemblyCertificateImpliesFullNativeAssemblyTarget P
  compatibilityExtractionImpliesReviewedExtraction :
    ReviewedFq12CompatibilityExtractionImpliesReviewedExtractionTarget P

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
  nativeBoundaryPredicateFieldSourcePackagePresent : Bool
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
                E.nativeBoundaryPredicateFieldSourcePackagePresent = true ∧
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
  nativeBoundaryPredicateRecordComplete := true
  nativeBoundaryPredicateFieldSourcePackagePresent := true
  activeVerifierRoutePinned := true
  predicateNegativeControlsPassed := true
  concretePredicateModelPresent := false
  certifiedSemanticsSourcePackageTargetPresent := true

/-- Concrete digest of the public Rust native-boundary predicate-model report. -/
def selectedRustNativeBoundaryPredicateModelEvidenceDigest : ByteArray :=
  ByteArray.mk #[121, 74, 79, 56, 100, 199, 183, 150, 246, 30, 129, 223,
    148, 114, 63, 30, 64, 15, 223, 166, 82, 105, 72, 0, 177, 201, 4, 36,
    118, 68, 164, 126]

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
Structured field-source package for the selected Rust native-boundary predicate
interpretation.

This is a target shape, not a selected implementation. It keeps the eventual
`Path2NativeBoundary.Predicates` import auditable field-by-field before the
proof forgets it into the large native-boundary predicate record.
-/
structure ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources where
  selectedBoundaryOf :
    Context -> HiddenRepresentative -> Path2NativeBoundary.Boundary
  selectedLocalValid :
    Context -> HiddenRepresentative -> Prop
  selectedDescentConsistent :
    Context -> HiddenRepresentative -> Prop
  selectedNormRangeValid :
    Path2NativeBoundary.Boundary -> HiddenRepresentative -> Prop
  selectedBlindingNoiseBounded :
    Path2NativeBoundary.Boundary -> BlindingNoisePolicy -> HiddenRepresentative -> Prop
  selectedSafeEquivalent :
    Context -> HiddenRepresentative -> HiddenRepresentative -> Prop
  selectedBoundaryBreakFor :
    Context -> Path2NativeBoundary.Boundary -> Path2NativeBoundary.BreakEvent -> Prop
  selectedAccountCreateAccepts :
    Context -> Path2NativeBoundary.Boundary -> Prop
  selectedAdmitted :
    Context -> Path2NativeBoundary.Boundary -> Prop
  selectedParentRootBindsPrivateRoles :
    Path2NativeBoundary.Boundary -> ColumnProfile -> Prop
  selectedRoleRestrictionsAuthenticate :
    Path2NativeBoundary.Boundary -> ColumnProfile -> Prop
  selectedNoExportHolds :
    Path2NativeBoundary.Boundary -> TransparentPublicView -> ColumnProfile -> Prop
  selectedRoleAuthenticationMaterialSimulatable :
    Path2NativeBoundary.Boundary -> TransparentPublicView -> ColumnProfile -> Prop
  selectedMaskFreshnessHolds :
    Path2NativeBoundary.Boundary -> TransparentPublicView -> ColumnProfile -> Prop
  selectedQromScheduleBound :
    Path2NativeBoundary.Boundary -> TransparentPublicView -> ColumnProfile -> Prop
  selectedQromExtractorRewindBound :
    Path2NativeBoundary.Boundary -> TransparentPublicView -> ColumnProfile -> Prop
  selectedRangeRowsBindRepresentative :
    Path2NativeBoundary.Boundary ->
      TransparentPublicView ->
        ColumnProfile ->
          HybridBase128RangePolicy -> HiddenRepresentative -> Prop
  selectedRangeRowsBindTraceRepresentative :
    Path2NativeBoundary.Boundary ->
      TransparentPublicView ->
        ColumnProfile ->
          HybridBase128RangePolicy ->
            ExtractedTrace -> HiddenRepresentative -> Prop
  selectedBlindingRowsBoundRepresentative :
    Path2NativeBoundary.Boundary ->
      TransparentPublicView ->
        ColumnProfile -> BlindingNoisePolicy -> HiddenRepresentative -> Prop
  selectedBlindingRowsBoundTraceRepresentative :
    Path2NativeBoundary.Boundary ->
      TransparentPublicView ->
        ColumnProfile ->
          BlindingNoisePolicy ->
            ExtractedTrace -> HiddenRepresentative -> Prop
  selectedTransparentAccepts :
    Path2NativeBoundary.Boundary -> TransparentPublicView -> Prop
  selectedTransparentExtractsTrace :
    Path2NativeBoundary.Boundary -> TransparentPublicView -> ExtractedTrace -> Prop
  selectedTransparentExtracts :
    Path2NativeBoundary.Boundary -> TransparentPublicView -> HiddenRepresentative -> Prop
  selectedReviewedFq12PayloadAccepts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload -> Prop
  selectedReviewedFq12PayloadProjectsFrom :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload -> Prop
  selectedReviewedFq12PayloadHasExtractionCertificate :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate -> Prop
  selectedReviewedFq12ExtractionCertificateValid :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate -> Prop
  selectedReviewedFq12CertificateExtracts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        HiddenRepresentative -> Prop
  selectedReviewedFq12CertificateLiftsToOpening :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> Prop
  selectedReviewedFq12PayloadHasRoleLiftCertificate :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate -> Prop
  selectedReviewedFq12RoleLiftCertificateValid :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate -> Prop
  selectedReviewedFq12RoleLiftCertificateOpens :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate ->
          FullNativeRepresentativeOpening -> Prop
  selectedFullNativeOpeningAssemblesRepresentative :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative -> Prop
  selectedReviewedFq12OpeningHasAssemblyCertificate :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate -> Prop
  selectedReviewedFq12AssemblyCertificateValid :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate -> Prop
  selectedReviewedFq12AssemblyCertificateAssembles :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            HiddenRepresentative -> Prop
  selectedReviewedFq12HasExtractionCompatibilityCertificate :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop
  selectedReviewedFq12ExtractionCompatibilityCertificateValid :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop
  selectedReviewedFq12ExtractionCompatibilityCertificateExtracts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop
  selectedReviewedFq12HasFullOpeningBridgeDigestCertificate :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            ReviewedFq12ExtractionCompatibilityCertificate ->
              ReviewedFq12FullOpeningBridgeDigestCertificate -> Prop
  selectedReviewedFq12FullOpeningBridgeDigestCertificateValid :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            ReviewedFq12ExtractionCompatibilityCertificate ->
              ReviewedFq12FullOpeningBridgeDigestCertificate -> Prop
  selectedReviewedFq12PayloadExtracts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload -> HiddenRepresentative -> Prop
  selectedTraceOpensBoundary :
    Path2NativeBoundary.Boundary -> ColumnProfile -> ExtractedTrace -> Prop
  selectedTracesSafetyEquivalent :
    Path2NativeBoundary.Boundary -> ColumnProfile -> ExtractedTrace -> ExtractedTrace -> Prop
  selectedMaskedOpeningsConsistent :
    Path2NativeBoundary.Boundary -> TransparentPublicView -> ColumnProfile -> ExtractedTrace -> Prop
  selectedTraceRelationValid :
    Path2NativeBoundary.Boundary -> ColumnProfile -> ExtractedTrace -> Prop
  selectedTraceAssemblesRepresentative :
    ExtractedTrace -> HiddenRepresentative -> Prop
  selectedExtractedRolesComplete :
    Path2NativeBoundary.Boundary -> ColumnProfile -> HiddenRepresentative -> Prop
  selectedTraceAssemblyRolesComplete :
    Path2NativeBoundary.Boundary -> ColumnProfile -> ExtractedTrace -> HiddenRepresentative -> Prop
  selectedTraceAssemblyDerivedRolesBound :
    Path2NativeBoundary.Boundary -> ColumnProfile -> ExtractedTrace -> HiddenRepresentative -> Prop
  selectedTraceAssemblyInFiber :
    Path2NativeBoundary.Boundary -> ColumnProfile -> ExtractedTrace -> HiddenRepresentative -> Prop
  selectedTraceAssemblyLocalValid :
    Path2NativeBoundary.Boundary -> ColumnProfile -> ExtractedTrace -> HiddenRepresentative -> Prop
  selectedTraceAssemblyDescentConsistent :
    Path2NativeBoundary.Boundary -> ColumnProfile -> ExtractedTrace -> HiddenRepresentative -> Prop
  selectedSlotAuthorizationOf :
    Path2NativeBoundary.Boundary -> SlotAuthorization
  selectedSpendAccepts :
    Context -> SlotAuthorization -> Prop
  selectedSpendWitnessValid :
    Context -> HiddenRepresentative -> SlotAuthorization -> Prop

/-- Selected native-boundary projection from context and hidden representative. -/
def selectedBoundaryOf
    (c : Context) (h : HiddenRepresentative) :
    Path2NativeBoundary.Boundary where
  boundaryVersion := c.profileVersionDigest
  context := c
  parentSectionRoot := h.localSection
  roleProfileDigest := h.packedResidues
  roleHandles :=
    { packedWitnessHandle := h.packedResidues
      randomizerHandle := h.randomizer
      padSourceHandle := h.padSource
      openingTupleHandle := h.openingTuple
      projectionRelationHandle := h.projectionRelation }
  maskedProjection := fun lane =>
    h.selectedAccountProjection + h.randomizer + lane.val
  liveStatusMarker := h.lifecycleSlot
  creationStatementDigest := h.openingTuple
  nativeBoundaryDigest := h.representativeDigest

/-- Selected local-validity provider for the native representative. -/
def selectedLocalValid
    (c : Context) (h : HiddenRepresentative) : Prop :=
  h.lifecycleSlot = c.slotId ∧
    h.selectedAccountProjection = c.accountId

/-- Selected descent-consistency provider for the native representative. -/
def selectedDescentConsistent
    (c : Context) (h : HiddenRepresentative) : Prop :=
  h.openingTuple = c.lifecycleAnchor ∧
    h.projectionRelation = c.aggregationContextDigest

/-- Selected norm/range provider for the native representative. -/
def selectedNormRangeValid
    (b : Path2NativeBoundary.Boundary)
    (h : HiddenRepresentative) : Prop :=
  h.representativeDigest = b.nativeBoundaryDigest ∧
    h.localSection = b.parentSectionRoot ∧
      h.packedResidues = b.roleProfileDigest

/-- Selected blinding/noise provider for the native representative. -/
def selectedBlindingNoiseBounded
    (_b : Path2NativeBoundary.Boundary)
    (policy : BlindingNoisePolicy)
    (h : HiddenRepresentative) : Prop :=
  policy.policyId = selectedBlindingNoisePolicyId ∧
    h.padSource <= policy.padSourceBound ∧
      h.projectionRelation <= policy.projectionRelationBound ∧
        h.openingTuple <= policy.openingTupleBound

/-- Selected safety-equivalence provider for native representatives. -/
def selectedSafeEquivalent
    (c : Context)
    (left right : HiddenRepresentative) : Prop :=
  selectedBoundaryOf c left = selectedBoundaryOf c right ∧
    left.lifecycleSlot = right.lifecycleSlot ∧
      left.traceWitness = right.traceWitness

/-- Selected native-boundary break provider. -/
def selectedBoundaryBreakFor
    (c : Context)
    (b : Path2NativeBoundary.Boundary)
    (breakEvent : Path2NativeBoundary.BreakEvent) : Prop :=
  breakEvent = Path2NativeBoundary.BreakEvent.boundaryBindingBreak ∧
    b.context = c

/-- Selected account-creation acceptance provider. -/
def selectedAccountCreateAccepts
    (c : Context) (b : Path2NativeBoundary.Boundary) : Prop :=
  b.context = c ∧
    b.nativeBoundaryDigest = b.creationStatementDigest

/-- Selected native-boundary admission provider. -/
def selectedAdmitted
    (c : Context) (b : Path2NativeBoundary.Boundary) : Prop :=
  selectedAccountCreateAccepts c b ∧
    b.liveStatusMarker = c.slotId

/-- Selected slot-authorization projection. -/
def selectedSlotAuthorizationOf
    (b : Path2NativeBoundary.Boundary) : SlotAuthorization where
  accountId := b.context.accountId
  publicKeyIdentity := b.context.publicKeyIdentity
  slotId := b.context.slotId
  authorizationDigest := b.creationStatementDigest
  nativeBoundaryDigest := b.nativeBoundaryDigest

/-- Selected spend acceptance provider. -/
def selectedSpendAccepts
    (c : Context) (authorization : SlotAuthorization) : Prop :=
  authorization.accountId = c.accountId ∧
    authorization.publicKeyIdentity = c.publicKeyIdentity ∧
      authorization.slotId = c.slotId

/-- Selected spend-witness validity provider. -/
def selectedSpendWitnessValid
    (c : Context)
    (h : HiddenRepresentative)
    (authorization : SlotAuthorization) : Prop :=
  selectedSpendAccepts c authorization ∧
    authorization.nativeBoundaryDigest =
      (selectedBoundaryOf c h).nativeBoundaryDigest

/-- Selected parent-root/private-role binding provider. -/
def selectedParentRootBindsPrivateRoles
    (b : Path2NativeBoundary.Boundary)
    (profile : ColumnProfile) : Prop :=
  profile.profileId = b.roleProfileDigest ∧
    profile.relationProfile.relationId = b.roleProfileDigest

/-- Selected role-restriction authentication provider. -/
def selectedRoleRestrictionsAuthenticate
    (b : Path2NativeBoundary.Boundary)
    (profile : ColumnProfile) : Prop :=
  selectedParentRootBindsPrivateRoles b profile ∧
    TraceRelationCoversNativeBoundary profile.relationProfile

/-- Selected no-export provider for the transparent public view. -/
def selectedNoExportHolds
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (profile : ColumnProfile) : Prop :=
  TransparentViewProjectsFrom b view ∧
    view.columnProfile = profile.profileId ∧
      profile.relationKnowledgeMode =
        RelationKnowledgeMode.fullWitnessRelationKnowledge

/-- Selected role-authentication simulation provider. -/
def selectedRoleAuthenticationMaterialSimulatable
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (profile : ColumnProfile) : Prop :=
  TransparentViewProjectsFrom b view ∧
    view.columnProfile = profile.profileId ∧
      profile.extractionMode = TraceExtractionMode.fullWitnessExtractable

/-- Selected mask-freshness provider. -/
def selectedMaskFreshnessHolds
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (_profile : ColumnProfile) : Prop :=
  TransparentViewProjectsFrom b view ∧
    view.queryIndex < view.queryCount

/-- Selected QROM schedule-bound provider. -/
def selectedQromScheduleBound
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (profile : ColumnProfile) : Prop :=
  TransparentViewProjectsFrom b view ∧
    view.queryCount <= profile.relationProfile.rowCount + view.traceRows

/-- Selected QROM extractor-rewind bound provider. -/
def selectedQromExtractorRewindBound
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (profile : ColumnProfile) : Prop :=
  selectedQromScheduleBound b view profile ∧
    TraceRelationMeetsSelectedNativeBoundaryRowFloor
      profile.relationProfile

/-- Selected range-row representative binding provider. -/
def selectedRangeRowsBindRepresentative
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (profile : ColumnProfile)
    (policy : HybridBase128RangePolicy)
    (h : HiddenRepresentative) : Prop :=
  TransparentViewProjectsFrom b view ∧
    UsesSelectedHybridBase128RangePolicy profile policy ∧
      selectedNormRangeValid b h

/-- Selected range-row trace representative binding provider. -/
def selectedRangeRowsBindTraceRepresentative
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (profile : ColumnProfile)
    (policy : HybridBase128RangePolicy)
    (trace : ExtractedTrace)
    (h : HiddenRepresentative) : Prop :=
  selectedRangeRowsBindRepresentative b view profile policy h ∧
    trace.parentSectionRoot = b.parentSectionRoot ∧
      trace.roleProfileDigest = b.roleProfileDigest

/-- Selected blinding-row representative bound provider. -/
def selectedBlindingRowsBoundRepresentative
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (_profile : ColumnProfile)
    (policy : BlindingNoisePolicy)
    (h : HiddenRepresentative) : Prop :=
  TransparentViewProjectsFrom b view ∧
    UsesSelectedBlindingNoisePolicy policy ∧
      selectedBlindingNoiseBounded b policy h

/-- Selected blinding-row trace representative bound provider. -/
def selectedBlindingRowsBoundTraceRepresentative
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (profile : ColumnProfile)
    (policy : BlindingNoisePolicy)
    (trace : ExtractedTrace)
    (h : HiddenRepresentative) : Prop :=
  selectedBlindingRowsBoundRepresentative b view profile policy h ∧
    trace.blindingWitness = h.padSource

/-- Selected transparent-proof acceptance provider. -/
def selectedTransparentAccepts
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView) : Prop :=
  TransparentViewProjectsFrom b view ∧
    view.queryIndex < view.queryCount

/-- Selected transparent trace-extraction provider. -/
def selectedTransparentExtractsTrace
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (trace : ExtractedTrace) : Prop :=
  selectedTransparentAccepts b view ∧
    trace.parentSectionRoot = b.parentSectionRoot ∧
      trace.roleProfileDigest = b.roleProfileDigest ∧
        trace.roleHandles = b.roleHandles

/-- Selected transparent representative-extraction provider. -/
def selectedTransparentExtracts
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (h : HiddenRepresentative) : Prop :=
  selectedTransparentAccepts b view ∧
    h.representativeDigest = b.nativeBoundaryDigest

/-- Selected trace-opening provider. -/
def selectedTraceOpensBoundary
    (b : Path2NativeBoundary.Boundary)
    (profile : ColumnProfile)
    (trace : ExtractedTrace) : Prop :=
  selectedParentRootBindsPrivateRoles b profile ∧
    trace.parentSectionRoot = b.parentSectionRoot ∧
      trace.roleProfileDigest = b.roleProfileDigest ∧
        trace.roleHandles = b.roleHandles

/-- Selected trace safety-equivalence provider. -/
def selectedTracesSafetyEquivalent
    (b : Path2NativeBoundary.Boundary)
    (_profile : ColumnProfile)
    (left right : ExtractedTrace) : Prop :=
  left.parentSectionRoot = b.parentSectionRoot ∧
    right.parentSectionRoot = b.parentSectionRoot ∧
      left.roleHandles = right.roleHandles

/-- Selected masked-opening consistency provider. -/
def selectedMaskedOpeningsConsistent
    (b : Path2NativeBoundary.Boundary)
    (view : TransparentPublicView)
    (_profile : ColumnProfile)
    (trace : ExtractedTrace) : Prop :=
  TransparentViewProjectsFrom b view ∧
    trace.maskWitness = view.transparentTranscriptDigest

/-- Selected trace-relation validity provider. -/
def selectedTraceRelationValid
    (b : Path2NativeBoundary.Boundary)
    (profile : ColumnProfile)
    (trace : ExtractedTrace) : Prop :=
  selectedTraceOpensBoundary b profile trace ∧
    TraceRelationCoversNativeBoundary profile.relationProfile ∧
      TraceRelationMeetsSelectedNativeBoundaryClassFloors
        profile.relationProfile

/-- Selected trace-to-representative assembly provider. -/
def selectedTraceAssemblesRepresentative
    (trace : ExtractedTrace)
    (h : HiddenRepresentative) : Prop :=
  trace.traceWitness = h.traceWitness ∧
    trace.rangeWitness = h.packedResidues ∧
      trace.blindingWitness = h.padSource

/-- Selected extracted-role completeness provider. -/
def selectedExtractedRolesComplete
    (b : Path2NativeBoundary.Boundary)
    (_profile : ColumnProfile)
    (h : HiddenRepresentative) : Prop :=
  h.localSection = b.parentSectionRoot ∧
    h.packedResidues = b.roleProfileDigest ∧
      h.lifecycleSlot = b.context.slotId

/-- Selected trace-assembly role completeness provider. -/
def selectedTraceAssemblyRolesComplete
    (b : Path2NativeBoundary.Boundary)
    (profile : ColumnProfile)
    (trace : ExtractedTrace)
    (h : HiddenRepresentative) : Prop :=
  selectedTraceRelationValid b profile trace ∧
    selectedTraceAssemblesRepresentative trace h

/-- Selected trace-assembly derived-role binding provider. -/
def selectedTraceAssemblyDerivedRolesBound
    (b : Path2NativeBoundary.Boundary)
    (_profile : ColumnProfile)
    (trace : ExtractedTrace)
    (h : HiddenRepresentative) : Prop :=
  trace.roleHandles = b.roleHandles ∧
    h.localSection = trace.parentSectionRoot ∧
      h.packedResidues = trace.roleProfileDigest

/-- Selected trace-assembly fiber provider. -/
def selectedTraceAssemblyInFiber
    (b : Path2NativeBoundary.Boundary)
    (profile : ColumnProfile)
    (trace : ExtractedTrace)
    (h : HiddenRepresentative) : Prop :=
  selectedTraceAssemblyRolesComplete b profile trace h ∧
    selectedBoundaryOf b.context h = b

/-- Selected trace-assembly local-validity provider. -/
def selectedTraceAssemblyLocalValid
    (b : Path2NativeBoundary.Boundary)
    (_profile : ColumnProfile)
    (_trace : ExtractedTrace)
    (h : HiddenRepresentative) : Prop :=
  selectedLocalValid b.context h

/-- Selected trace-assembly descent-consistency provider. -/
def selectedTraceAssemblyDescentConsistent
    (b : Path2NativeBoundary.Boundary)
    (_profile : ColumnProfile)
    (_trace : ExtractedTrace)
    (h : HiddenRepresentative) : Prop :=
  selectedDescentConsistent b.context h

/--
Reviewed-Fq12/full-opening provider slice of the selected Rust
native-boundary predicate interpretation.

This is the highest-risk active Path 2 predicate family because it is where
payload acceptance, extraction certificates, role-lift openings, native
representative assembly, compatibility, and bridge-digest binding all become
one concrete relation. Keeping it as a named slice lets the proof track that
family before the complete native-boundary predicate record is assembled.
-/
structure ReviewedFq12SelectedRustNativeBoundaryReviewedFq12PredicateFieldSources where
  selectedReviewedFq12PayloadAccepts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload -> Prop
  selectedReviewedFq12PayloadProjectsFrom :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload -> Prop
  selectedReviewedFq12PayloadHasExtractionCertificate :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate -> Prop
  selectedReviewedFq12ExtractionCertificateValid :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate -> Prop
  selectedReviewedFq12CertificateExtracts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        HiddenRepresentative -> Prop
  selectedReviewedFq12CertificateLiftsToOpening :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> Prop
  selectedReviewedFq12PayloadHasRoleLiftCertificate :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate -> Prop
  selectedReviewedFq12RoleLiftCertificateValid :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate -> Prop
  selectedReviewedFq12RoleLiftCertificateOpens :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate ->
          FullNativeRepresentativeOpening -> Prop
  selectedFullNativeOpeningAssemblesRepresentative :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative -> Prop
  selectedReviewedFq12OpeningHasAssemblyCertificate :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate -> Prop
  selectedReviewedFq12AssemblyCertificateValid :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate -> Prop
  selectedReviewedFq12AssemblyCertificateAssembles :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            HiddenRepresentative -> Prop
  selectedReviewedFq12HasExtractionCompatibilityCertificate :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop
  selectedReviewedFq12ExtractionCompatibilityCertificateValid :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop
  selectedReviewedFq12ExtractionCompatibilityCertificateExtracts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop
  selectedReviewedFq12HasFullOpeningBridgeDigestCertificate :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            ReviewedFq12ExtractionCompatibilityCertificate ->
              ReviewedFq12FullOpeningBridgeDigestCertificate -> Prop
  selectedReviewedFq12FullOpeningBridgeDigestCertificateValid :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        ReviewedFq12FullNativeOpeningCertificate ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            ReviewedFq12ExtractionCompatibilityCertificate ->
              ReviewedFq12FullOpeningBridgeDigestCertificate -> Prop
  selectedReviewedFq12PayloadExtracts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload -> HiddenRepresentative -> Prop

/--
Representative-producing reviewed-Fq12 provider slice of the selected Rust
native-boundary predicate interpretation.

These are the active Path 2 fields that cannot be sourced from aggregate
bridge-digest validity alone: they must be backed by the canonical
representative/shared digest-frame witness route.
-/
structure ReviewedFq12SelectedRustNativeBoundaryReviewedFq12HardExtractionPredicateFieldSources where
  selectedFullNativeOpeningAssemblesRepresentative :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative -> Prop
  selectedReviewedFq12AssemblyCertificateAssembles :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            HiddenRepresentative -> Prop
  selectedReviewedFq12ExtractionCompatibilityCertificateExtracts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop
  selectedReviewedFq12CertificateExtracts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        HiddenRepresentative -> Prop
  selectedReviewedFq12PayloadExtracts :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload -> HiddenRepresentative -> Prop

namespace ReviewedFq12SelectedRustNativeBoundaryReviewedFq12PredicateFieldSources

/--
Project the reviewed-Fq12 provider family to the representative-producing
subfamily that remains the hard cryptographic closure target.
-/
def toHardExtractionPredicateFieldSources
    (Sources :
      ReviewedFq12SelectedRustNativeBoundaryReviewedFq12PredicateFieldSources) :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12HardExtractionPredicateFieldSources where
  selectedFullNativeOpeningAssemblesRepresentative :=
    Sources.selectedFullNativeOpeningAssemblesRepresentative
  selectedReviewedFq12AssemblyCertificateAssembles :=
    Sources.selectedReviewedFq12AssemblyCertificateAssembles
  selectedReviewedFq12ExtractionCompatibilityCertificateExtracts :=
    Sources.selectedReviewedFq12ExtractionCompatibilityCertificateExtracts
  selectedReviewedFq12CertificateExtracts :=
    Sources.selectedReviewedFq12CertificateExtracts
  selectedReviewedFq12PayloadExtracts :=
    Sources.selectedReviewedFq12PayloadExtracts

end ReviewedFq12SelectedRustNativeBoundaryReviewedFq12PredicateFieldSources

/--
Extract the representative-producing reviewed-Fq12 provider subfamily directly
from a native-boundary predicate interpretation.
-/
def reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
    (P : Predicates) :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12HardExtractionPredicateFieldSources where
  selectedFullNativeOpeningAssemblesRepresentative :=
    P.fullNativeOpeningAssemblesRepresentative
  selectedReviewedFq12AssemblyCertificateAssembles :=
    P.reviewedFq12AssemblyCertificateAssembles
  selectedReviewedFq12ExtractionCompatibilityCertificateExtracts :=
    P.reviewedFq12ExtractionCompatibilityCertificateExtracts
  selectedReviewedFq12CertificateExtracts :=
    P.reviewedFq12CertificateExtracts
  selectedReviewedFq12PayloadExtracts :=
    P.reviewedFq12PayloadExtracts

/--
Self-contained hard-extraction slice of a selected native-boundary predicate
model.

This package ties the five representative-producing reviewed-Fq12 predicates
to the exact native-boundary predicate interpretation whose laws justify using
them in hidden-source soundness.
-/
structure ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelPackage where
  predicates : Predicates
  fieldSources :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12HardExtractionPredicateFieldSources
  fieldSourcesFromPredicates :
    fieldSources =
      reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates predicates
  laws :
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelLaws
      predicates

/--
Digest-frame-based full-native-opening assembly provider.

This is a concrete non-circular provider constructor, not the final selected
global provider. It pins the representative digest to the active native
representative digest-frame shape and uses the full native opening for every
role coordinate.
-/
def reviewedFq12FullNativeOpeningAssemblesRepresentativeFromDigestFrame
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame)
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative) : Prop :=
  payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
    payload.slotId = b.context.slotId ∧
      cert.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
        cert.slotId = b.context.slotId ∧
          cert.openingCommitment = opening.openingTupleDigest ∧
            h =
              reviewedFq12FullNativeOpeningCanonicalRepresentative
                (reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
                  digestFrame)
                b opening

/--
Digest-frame-based assembly-certificate assembly provider.

The assembly certificate must field-bind to the opening, its representative
digest must equal the selected digest-frame value, and the hidden
representative must be the canonical representative for the same boundary and
opening.
-/
def reviewedFq12AssemblyCertificateAssemblesFromDigestFrame
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame)
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (h : HiddenRepresentative) : Prop :=
  ReviewedFq12AssemblyCertificateBindsOpening payload cert opening
      assemblyCert ∧
    assemblyCert.representativeDigest =
        reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
          digestFrame b opening ∧
      reviewedFq12FullNativeOpeningAssemblesRepresentativeFromDigestFrame
        digestFrame b payload cert opening h

/--
Digest-frame-based compatibility extraction provider.

The compatibility certificate must bind the same payload, extraction
certificate, opening, and canonical representative.
-/
def reviewedFq12ExtractionCompatibilityCertificateExtractsFromDigestFrame
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame)
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) : Prop :=
  ReviewedFq12ExtractionCompatibilityCertificateBinds
      payload cert opening h compatCert ∧
    reviewedFq12FullNativeOpeningAssemblesRepresentativeFromDigestFrame
      digestFrame b payload cert opening h

/--
Digest-frame-based certificate extraction provider.

An extraction certificate extracts a representative when it has a full native
opening whose canonical representative is that representative.
-/
def reviewedFq12CertificateExtractsFromDigestFrame
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame)
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (h : HiddenRepresentative) : Prop :=
  exists opening,
    reviewedFq12FullNativeOpeningAssemblesRepresentativeFromDigestFrame
      digestFrame b payload cert opening h

/--
Digest-frame-based payload extraction provider.

Payload extraction is sourced from a certificate extraction over the same
boundary and payload. Native fiber/local/descent validity are still separate
laws supplied by the selected predicate model.
-/
def reviewedFq12PayloadExtractsFromDigestFrame
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame)
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (h : HiddenRepresentative) : Prop :=
  exists cert,
    reviewedFq12CertificateExtractsFromDigestFrame
      digestFrame b payload cert h

/--
Concrete hard-extraction provider slice obtained from one native
representative digest frame.
-/
def reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame) :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12HardExtractionPredicateFieldSources where
  selectedFullNativeOpeningAssemblesRepresentative :=
    reviewedFq12FullNativeOpeningAssemblesRepresentativeFromDigestFrame
      digestFrame
  selectedReviewedFq12AssemblyCertificateAssembles :=
    reviewedFq12AssemblyCertificateAssemblesFromDigestFrame digestFrame
  selectedReviewedFq12ExtractionCompatibilityCertificateExtracts :=
    reviewedFq12ExtractionCompatibilityCertificateExtractsFromDigestFrame
      digestFrame
  selectedReviewedFq12CertificateExtracts :=
    reviewedFq12CertificateExtractsFromDigestFrame digestFrame
  selectedReviewedFq12PayloadExtracts :=
    reviewedFq12PayloadExtractsFromDigestFrame digestFrame

/--
The digest-frame assembly-certificate provider immediately implies the
digest-frame full-opening assembly provider for the same representative.
-/
theorem reviewedFq12DigestFrameAssemblyCertificateImpliesFullNativeAssembly
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame)
    {b : Path2NativeBoundary.Boundary}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {h : HiddenRepresentative}
    (hAssembles :
      (reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame
        digestFrame).selectedReviewedFq12AssemblyCertificateAssembles
          b payload cert opening assemblyCert h) :
    (reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame
      digestFrame).selectedFullNativeOpeningAssemblesRepresentative
        b payload cert opening h := by
  exact hAssembles.2.2

/--
The digest-frame certificate extraction provider immediately implies the
digest-frame payload extraction provider for the same representative.
-/
theorem reviewedFq12DigestFrameCertificateExtractionImpliesPayloadExtraction
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame)
    {b : Path2NativeBoundary.Boundary}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {h : HiddenRepresentative}
    (hExtracts :
      (reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame
        digestFrame).selectedReviewedFq12CertificateExtracts
          b payload cert h) :
    (reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame
      digestFrame).selectedReviewedFq12PayloadExtracts b payload h := by
  exact ⟨cert, hExtracts⟩

/--
Certificate-shaped route from the active shared digest-frame witness evidence
into the hard-extraction provider slice.

This consumes the report-verified shared witness certificate before exposing
the digest frame. It keeps the representative-producing provider route tied to
the active selected Rust shared-frame evidence, rather than to a naked frame
parameter.
-/
def reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12HardExtractionPredicateFieldSources :=
  reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame
    Cert.toWitnessSources.digestFrame

/--
The shared-witness certificate route is definitionally the digest-frame
provider route for the certificate's report-verified shared frame.
-/
theorem reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate_eq
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
        Cert =
      reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame
        Cert.toWitnessSources.digestFrame := rfl

/--
Selected payload-acceptance provider for the reviewed-Fq12 native-boundary
model.

Acceptance pins the payload to the native boundary digest and slot. The
payload statement/proof bytes remain checked by the payload/extraction and
source-record evidence layers.
-/
def selectedReviewedFq12PayloadAccepts
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload) : Prop :=
  payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
    payload.slotId = b.context.slotId

/--
Selected payload-projection provider for the reviewed-Fq12 native-boundary
model.

Projection uses the same boundary/slot pin as acceptance and leaves the
field-level source-route byte proof as the source of the concrete coordinate
projection facts.
-/
def selectedReviewedFq12PayloadProjectsFrom
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload) : Prop :=
  payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
    payload.slotId = b.context.slotId

/-- Selected extraction-certificate availability provider. -/
def selectedReviewedFq12PayloadHasExtractionCertificate
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate) : Prop :=
  selectedReviewedFq12PayloadAccepts b payload ∧
    ReviewedFq12ExtractionCertificateBindsPayload payload cert

/-- Selected extraction-certificate validity provider. -/
def selectedReviewedFq12ExtractionCertificateValid
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate) : Prop :=
  selectedReviewedFq12PayloadProjectsFrom b payload ∧
    ReviewedFq12ExtractionCertificateBindsPayload payload cert

/-- Selected certificate-to-full-opening lift provider. -/
def selectedReviewedFq12CertificateLiftsToOpening
    (_b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening) : Prop :=
  ReviewedFq12ExtractionCertificateBindsPayload payload cert ∧
    cert.openingCommitment = opening.openingTupleDigest ∧
      cert.accountProjectionDigest =
        opening.selectedAccountProjectionDigest ∧
        cert.padSeedDigest = opening.padSourceDigest ∧
          payload.randomizerCommitment = opening.randomizerDigest

/-- Selected role-lift certificate availability provider. -/
def selectedReviewedFq12PayloadHasRoleLiftCertificate
    (_b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate) : Prop :=
  ReviewedFq12RoleLiftCertificateBindsExtraction payload cert liftCert

/-- Selected role-lift certificate validity provider. -/
def selectedReviewedFq12RoleLiftCertificateValid
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate) : Prop :=
  selectedReviewedFq12PayloadAccepts b payload ∧
    ReviewedFq12RoleLiftCertificateBindsExtraction payload cert liftCert

/-- Selected role-lift opening provider. -/
def selectedReviewedFq12RoleLiftCertificateOpens
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (opening : FullNativeRepresentativeOpening) : Prop :=
  selectedReviewedFq12CertificateLiftsToOpening b payload cert opening ∧
    ReviewedFq12RoleLiftCertificateBindsExtraction payload cert liftCert ∧
      liftCert.opening = opening

/-- Selected assembly-certificate availability provider. -/
def selectedReviewedFq12OpeningHasAssemblyCertificate
    (_b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    Prop :=
  ReviewedFq12AssemblyCertificateBindsOpening
    payload cert opening assemblyCert

/-- Selected assembly-certificate validity provider. -/
def selectedReviewedFq12AssemblyCertificateValid
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    Prop :=
  selectedReviewedFq12PayloadAccepts b payload ∧
    ReviewedFq12AssemblyCertificateBindsOpening
      payload cert opening assemblyCert

/-- Selected extraction-compatibility certificate availability provider. -/
def selectedReviewedFq12HasExtractionCompatibilityCertificate
    (_b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop :=
  ReviewedFq12ExtractionCompatibilityCertificateBinds
    payload cert opening h compatCert

/-- Selected extraction-compatibility certificate validity provider. -/
def selectedReviewedFq12ExtractionCompatibilityCertificateValid
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop :=
  selectedReviewedFq12PayloadAccepts b payload ∧
    ReviewedFq12ExtractionCompatibilityCertificateBinds
      payload cert opening h compatCert

/-- Selected full-opening bridge-digest certificate availability provider. -/
def selectedReviewedFq12HasFullOpeningBridgeDigestCertificate
    (_b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate)
    (bridgeCert : ReviewedFq12FullOpeningBridgeDigestCertificate) : Prop :=
  ReviewedFq12FullOpeningBridgeDigestCertificateBinds
    payload cert liftCert assemblyCert compatCert bridgeCert

/-- Selected full-opening bridge-digest certificate validity provider. -/
def selectedReviewedFq12FullOpeningBridgeDigestCertificateValid
    (b : Path2NativeBoundary.Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate)
    (bridgeCert : ReviewedFq12FullOpeningBridgeDigestCertificate) : Prop :=
  selectedReviewedFq12PayloadAccepts b payload ∧
    ReviewedFq12FullOpeningBridgeDigestCertificateBinds
      payload cert liftCert assemblyCert compatCert bridgeCert

/--
Selected hard full-opening assembly provider, sourced from the active shared
digest-frame witness certificate.

This is not a global predicate-model instantiation.  The certificate parameter
is the proof object that the final selected source-record package must supply.
-/
def selectedFullNativeOpeningAssemblesRepresentative
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative -> Prop :=
  (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
    Cert).selectedFullNativeOpeningAssemblesRepresentative

/--
Selected hard assembly-certificate provider, sourced from the active shared
digest-frame witness certificate.
-/
def selectedReviewedFq12AssemblyCertificateAssembles
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening ->
          ReviewedFq12NativeRepresentativeAssemblyCertificate ->
            HiddenRepresentative -> Prop :=
  (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
    Cert).selectedReviewedFq12AssemblyCertificateAssembles

/--
Selected hard compatibility-extraction provider, sourced from the active shared
digest-frame witness certificate.
-/
def selectedReviewedFq12ExtractionCompatibilityCertificateExtracts
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        FullNativeRepresentativeOpening -> HiddenRepresentative ->
          ReviewedFq12ExtractionCompatibilityCertificate -> Prop :=
  (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
    Cert).selectedReviewedFq12ExtractionCompatibilityCertificateExtracts

/--
Selected hard certificate-extraction provider, sourced from the active shared
digest-frame witness certificate.
-/
def selectedReviewedFq12CertificateExtracts
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      ReviewedFq12HiddenRelationExtractionCertificate ->
        HiddenRepresentative -> Prop :=
  (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
    Cert).selectedReviewedFq12CertificateExtracts

/--
Selected hard payload-extraction provider, sourced from the active shared
digest-frame witness certificate.
-/
def selectedReviewedFq12PayloadExtracts
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    Path2NativeBoundary.Boundary -> ReviewedFq12HiddenSourcePayload ->
      HiddenRepresentative -> Prop :=
  (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
    Cert).selectedReviewedFq12PayloadExtracts

/--
Selected native-boundary predicate field-source package.

The broad core, transparent/trace, and non-representative reviewed-Fq12 fields
are concrete declarations in this module.  The five representative-producing
reviewed-Fq12 fields are supplied through the active shared-witness certificate,
because they depend on the canonical representative digest frame.
-/
def selectedRustNativeBoundaryPredicateFieldSources
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (sharedWitnessCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources where
  selectedBoundaryOf := selectedBoundaryOf
  selectedLocalValid := selectedLocalValid
  selectedDescentConsistent := selectedDescentConsistent
  selectedNormRangeValid := selectedNormRangeValid
  selectedBlindingNoiseBounded := selectedBlindingNoiseBounded
  selectedSafeEquivalent := selectedSafeEquivalent
  selectedBoundaryBreakFor := selectedBoundaryBreakFor
  selectedAccountCreateAccepts := selectedAccountCreateAccepts
  selectedAdmitted := selectedAdmitted
  selectedParentRootBindsPrivateRoles :=
    selectedParentRootBindsPrivateRoles
  selectedRoleRestrictionsAuthenticate :=
    selectedRoleRestrictionsAuthenticate
  selectedNoExportHolds := selectedNoExportHolds
  selectedRoleAuthenticationMaterialSimulatable :=
    selectedRoleAuthenticationMaterialSimulatable
  selectedMaskFreshnessHolds := selectedMaskFreshnessHolds
  selectedQromScheduleBound := selectedQromScheduleBound
  selectedQromExtractorRewindBound := selectedQromExtractorRewindBound
  selectedRangeRowsBindRepresentative :=
    selectedRangeRowsBindRepresentative
  selectedRangeRowsBindTraceRepresentative :=
    selectedRangeRowsBindTraceRepresentative
  selectedBlindingRowsBoundRepresentative :=
    selectedBlindingRowsBoundRepresentative
  selectedBlindingRowsBoundTraceRepresentative :=
    selectedBlindingRowsBoundTraceRepresentative
  selectedTransparentAccepts := selectedTransparentAccepts
  selectedTransparentExtractsTrace := selectedTransparentExtractsTrace
  selectedTransparentExtracts := selectedTransparentExtracts
  selectedReviewedFq12PayloadAccepts := selectedReviewedFq12PayloadAccepts
  selectedReviewedFq12PayloadProjectsFrom :=
    selectedReviewedFq12PayloadProjectsFrom
  selectedReviewedFq12PayloadHasExtractionCertificate :=
    selectedReviewedFq12PayloadHasExtractionCertificate
  selectedReviewedFq12ExtractionCertificateValid :=
    selectedReviewedFq12ExtractionCertificateValid
  selectedReviewedFq12CertificateExtracts :=
    selectedReviewedFq12CertificateExtracts sharedWitnessCertificate
  selectedReviewedFq12CertificateLiftsToOpening :=
    selectedReviewedFq12CertificateLiftsToOpening
  selectedReviewedFq12PayloadHasRoleLiftCertificate :=
    selectedReviewedFq12PayloadHasRoleLiftCertificate
  selectedReviewedFq12RoleLiftCertificateValid :=
    selectedReviewedFq12RoleLiftCertificateValid
  selectedReviewedFq12RoleLiftCertificateOpens :=
    selectedReviewedFq12RoleLiftCertificateOpens
  selectedFullNativeOpeningAssemblesRepresentative :=
    selectedFullNativeOpeningAssemblesRepresentative
      sharedWitnessCertificate
  selectedReviewedFq12OpeningHasAssemblyCertificate :=
    selectedReviewedFq12OpeningHasAssemblyCertificate
  selectedReviewedFq12AssemblyCertificateValid :=
    selectedReviewedFq12AssemblyCertificateValid
  selectedReviewedFq12AssemblyCertificateAssembles :=
    selectedReviewedFq12AssemblyCertificateAssembles
      sharedWitnessCertificate
  selectedReviewedFq12HasExtractionCompatibilityCertificate :=
    selectedReviewedFq12HasExtractionCompatibilityCertificate
  selectedReviewedFq12ExtractionCompatibilityCertificateValid :=
    selectedReviewedFq12ExtractionCompatibilityCertificateValid
  selectedReviewedFq12ExtractionCompatibilityCertificateExtracts :=
    selectedReviewedFq12ExtractionCompatibilityCertificateExtracts
      sharedWitnessCertificate
  selectedReviewedFq12HasFullOpeningBridgeDigestCertificate :=
    selectedReviewedFq12HasFullOpeningBridgeDigestCertificate
  selectedReviewedFq12FullOpeningBridgeDigestCertificateValid :=
    selectedReviewedFq12FullOpeningBridgeDigestCertificateValid
  selectedReviewedFq12PayloadExtracts :=
    selectedReviewedFq12PayloadExtracts sharedWitnessCertificate
  selectedTraceOpensBoundary := selectedTraceOpensBoundary
  selectedTracesSafetyEquivalent := selectedTracesSafetyEquivalent
  selectedMaskedOpeningsConsistent := selectedMaskedOpeningsConsistent
  selectedTraceRelationValid := selectedTraceRelationValid
  selectedTraceAssemblesRepresentative :=
    selectedTraceAssemblesRepresentative
  selectedExtractedRolesComplete := selectedExtractedRolesComplete
  selectedTraceAssemblyRolesComplete :=
    selectedTraceAssemblyRolesComplete
  selectedTraceAssemblyDerivedRolesBound :=
    selectedTraceAssemblyDerivedRolesBound
  selectedTraceAssemblyInFiber := selectedTraceAssemblyInFiber
  selectedTraceAssemblyLocalValid := selectedTraceAssemblyLocalValid
  selectedTraceAssemblyDescentConsistent :=
    selectedTraceAssemblyDescentConsistent
  selectedSlotAuthorizationOf := selectedSlotAuthorizationOf
  selectedSpendAccepts := selectedSpendAccepts
  selectedSpendWitnessValid := selectedSpendWitnessValid

/--
Build the hard-extraction law bundle for a selected predicate model once its
five hard fields have been proved equal to the shared-witness certificate
route.

The digest-frame provider gives the certificate/payload, assembly/full-opening,
and compatibility/reviewed-extraction implications structurally. The remaining
inputs are exactly the native-boundary facts that payload extraction lands in
the boundary fiber and satisfies local/descent validity.
-/
theorem reviewedFq12HardExtractionLawsOfSharedWitnessCertificate
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P)
    (hFields :
      reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert =
        reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates P)
    (hFiber : ReviewedFq12PayloadExtractionInBoundaryFiber P)
    (hNative : ReviewedFq12PayloadExtractionNativeValid P) :
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelLaws P where
  certificateExtractionImpliesPayloadExtraction := by
    intro b payload cert h hExtracts
    have hExtractsFromShared :
        (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert).selectedReviewedFq12CertificateExtracts
            b payload cert h := by
      rw [hFields]
      exact hExtracts
    have hPayloadFromShared :
        (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert).selectedReviewedFq12PayloadExtracts b payload h := by
      exact
        reviewedFq12DigestFrameCertificateExtractionImpliesPayloadExtraction
          Cert.toWitnessSources.digestFrame hExtractsFromShared
    have hPayloadOfPredicates :
        (reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
          P).selectedReviewedFq12PayloadExtracts b payload h := by
      rw [← hFields]
      exact hPayloadFromShared
    simpa [reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates]
      using hPayloadOfPredicates
  payloadExtractionInBoundaryFiber := hFiber
  payloadExtractionNativeValid := hNative
  assemblyCertificateImpliesFullNativeAssembly := by
    intro b payload cert opening assemblyCert h hAssembles
    have hAssemblesFromShared :
        (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert).selectedReviewedFq12AssemblyCertificateAssembles
            b payload cert opening assemblyCert h := by
      rw [hFields]
      exact hAssembles
    have hFullFromShared :
        (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert).selectedFullNativeOpeningAssemblesRepresentative
            b payload cert opening h := by
      exact
        reviewedFq12DigestFrameAssemblyCertificateImpliesFullNativeAssembly
          Cert.toWitnessSources.digestFrame hAssemblesFromShared
    have hFullOfPredicates :
        (reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
          P).selectedFullNativeOpeningAssemblesRepresentative
            b payload cert opening h := by
      rw [← hFields]
      exact hFullFromShared
    simpa [reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates]
      using hFullOfPredicates
  compatibilityExtractionImpliesReviewedExtraction := by
    intro b payload cert opening h compatCert hCompat
    have hCompatFromShared :
        (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert).selectedReviewedFq12ExtractionCompatibilityCertificateExtracts
            b payload cert opening h compatCert := by
      rw [hFields]
      exact hCompat
    have hCertFromShared :
        (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert).selectedReviewedFq12CertificateExtracts
            b payload cert h := by
      exact ⟨opening, hCompatFromShared.2⟩
    have hPayloadFromShared :
        (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert).selectedReviewedFq12PayloadExtracts b payload h := by
      exact
        reviewedFq12DigestFrameCertificateExtractionImpliesPayloadExtraction
          Cert.toWitnessSources.digestFrame hCertFromShared
    constructor
    · have hCertOfPredicates :
          (reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
            P).selectedReviewedFq12CertificateExtracts
              b payload cert h := by
        rw [← hFields]
        exact hCertFromShared
      simpa [reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates]
        using hCertOfPredicates
    · have hPayloadOfPredicates :
          (reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
            P).selectedReviewedFq12PayloadExtracts b payload h := by
        rw [← hFields]
        exact hPayloadFromShared
      simpa [reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates]
        using hPayloadOfPredicates

/--
Native-validity source for the remaining hard-extraction payload laws.

After the shared-witness hard fields are selected, payload extraction unpacks
to a full native opening whose canonical representative is the extracted
hidden representative. This source is exactly the remaining native-boundary
evidence needed for that full-opening assembly witness.
-/
structure ReviewedFq12SharedWitnessFullOpeningNativeValiditySource
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) where
  fullOpeningAssemblyInBoundaryFiber :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {h : HiddenRepresentative},
      (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
        Cert).selectedFullNativeOpeningAssemblesRepresentative
          b payload cert opening h ->
        InFiber P b.context b h
  fullOpeningAssemblyNativeValid :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {h : HiddenRepresentative},
      (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
        Cert).selectedFullNativeOpeningAssemblesRepresentative
          b payload cert opening h ->
        P.localValid b.context h ∧
          P.descentConsistent b.context h

/--
The shared-witness full-opening native-validity source discharges the two
payload-extraction native laws once the selected hard fields are proved equal
to the shared-witness hard fields.
-/
theorem reviewedFq12PayloadExtractionNativeLawsOfSharedWitnessFullOpeningSource
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P)
    (hFields :
      reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert =
        reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates P)
    (nativeSource :
      ReviewedFq12SharedWitnessFullOpeningNativeValiditySource Cert) :
    ReviewedFq12PayloadExtractionInBoundaryFiber P ∧
      ReviewedFq12PayloadExtractionNativeValid P := by
  constructor
  · intro b payload h hPayload
    have hPayloadFromShared :
        (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert).selectedReviewedFq12PayloadExtracts b payload h := by
      rw [hFields]
      exact hPayload
    rcases hPayloadFromShared with ⟨cert, hCertExtracts⟩
    rcases hCertExtracts with ⟨opening, hFullOpening⟩
    exact nativeSource.fullOpeningAssemblyInBoundaryFiber hFullOpening
  · intro b payload h hPayload
    have hPayloadFromShared :
        (reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert).selectedReviewedFq12PayloadExtracts b payload h := by
      rw [hFields]
      exact hPayload
    rcases hPayloadFromShared with ⟨cert, hCertExtracts⟩
    rcases hCertExtracts with ⟨opening, hFullOpening⟩
    exact nativeSource.fullOpeningAssemblyNativeValid hFullOpening

/--
Full hard-extraction laws from the shared-witness field equality and the
remaining full-opening native-validity source.
-/
theorem reviewedFq12HardExtractionLawsOfSharedWitnessFullOpeningSource
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P)
    (hFields :
      reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          Cert =
        reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates P)
    (nativeSource :
      ReviewedFq12SharedWitnessFullOpeningNativeValiditySource Cert) :
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelLaws P := by
  have hNativeLaws :=
    reviewedFq12PayloadExtractionNativeLawsOfSharedWitnessFullOpeningSource
      Cert hFields nativeSource
  exact
    reviewedFq12HardExtractionLawsOfSharedWitnessCertificate
      Cert hFields hNativeLaws.1 hNativeLaws.2

/--
Canonical full-opening native-validity source for the shared-witness route.

This is the implementation-facing form of the remaining native-validity
obligation: prove fiber/local/descent for the exact canonical representative
computed from the report-verified shared digest frame and the opened native
roles.
-/
structure ReviewedFq12SharedWitnessCanonicalFullOpeningNativeValiditySource
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) where
  canonicalFullOpeningInBoundaryFiber :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening},
      payload.nativeBoundaryDigest = b.nativeBoundaryDigest ->
        payload.slotId = b.context.slotId ->
          cert.nativeBoundaryDigest = b.nativeBoundaryDigest ->
            cert.slotId = b.context.slotId ->
              cert.openingCommitment = opening.openingTupleDigest ->
                InFiber P b.context b
                  (reviewedFq12FullNativeOpeningCanonicalRepresentative
                    (reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
                      Cert.toWitnessSources.digestFrame)
                    b opening)
  canonicalFullOpeningNativeValid :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening},
      payload.nativeBoundaryDigest = b.nativeBoundaryDigest ->
        payload.slotId = b.context.slotId ->
          cert.nativeBoundaryDigest = b.nativeBoundaryDigest ->
            cert.slotId = b.context.slotId ->
              cert.openingCommitment = opening.openingTupleDigest ->
                P.localValid b.context
                    (reviewedFq12FullNativeOpeningCanonicalRepresentative
                      (reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
                        Cert.toWitnessSources.digestFrame)
                      b opening) ∧
                  P.descentConsistent b.context
                    (reviewedFq12FullNativeOpeningCanonicalRepresentative
                      (reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
                        Cert.toWitnessSources.digestFrame)
                      b opening)

/--
The canonical full-opening native-validity source derives the full-opening
native-validity source used by the hard law bundle.
-/
def reviewedFq12SharedWitnessFullOpeningNativeValiditySourceOfCanonicalSource
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P)
    (canonicalSource :
      ReviewedFq12SharedWitnessCanonicalFullOpeningNativeValiditySource
        Cert) :
    ReviewedFq12SharedWitnessFullOpeningNativeValiditySource Cert where
  fullOpeningAssemblyInBoundaryFiber := by
    intro b payload cert opening h hAssembles
    dsimp [
      reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate,
      reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame,
      reviewedFq12FullNativeOpeningAssemblesRepresentativeFromDigestFrame
    ] at hAssembles
    rcases hAssembles with
      ⟨hPayloadBoundary, hPayloadSlot, hCertBoundary, hCertSlot,
        hOpeningCommitment, hEq⟩
    rw [hEq]
    exact
      canonicalSource.canonicalFullOpeningInBoundaryFiber
        hPayloadBoundary hPayloadSlot hCertBoundary hCertSlot
        hOpeningCommitment
  fullOpeningAssemblyNativeValid := by
    intro b payload cert opening h hAssembles
    dsimp [
      reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate,
      reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame,
      reviewedFq12FullNativeOpeningAssemblesRepresentativeFromDigestFrame
    ] at hAssembles
    rcases hAssembles with
      ⟨hPayloadBoundary, hPayloadSlot, hCertBoundary, hCertSlot,
        hOpeningCommitment, hEq⟩
    rw [hEq]
    exact
      canonicalSource.canonicalFullOpeningNativeValid
        hPayloadBoundary hPayloadSlot hCertBoundary hCertSlot
        hOpeningCommitment

/--
Proof-bearing hard-extraction predicate-model certificate sourced by the
active shared digest-frame witness certificate.

The nontrivial field is `fieldSourcesFromSharedWitnessCertificate`: it is the
future proof that the selected native-boundary predicate model's five
representative-producing fields are exactly the fields derived from the
report-verified shared witness certificate.
-/
structure ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (predicates : Path2NativeBoundary.Predicates) where
  sharedWitnessCertificate :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
      CanonicalParser WitnessParser predicates
  fieldSourcesFromSharedWitnessCertificate :
    reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
        sharedWitnessCertificate =
      reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates predicates
  laws :
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelLaws
      predicates

namespace ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate

/--
Construct the shared-witness hard-extraction predicate-model certificate from
the field equality and the canonical native-validity source.
-/
def ofCanonicalNativeSource
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {predicates : Path2NativeBoundary.Predicates}
    (sharedWitnessCertificate :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser predicates)
    (fieldSourcesFromSharedWitnessCertificate :
      reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
          sharedWitnessCertificate =
        reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
          predicates)
    (canonicalNativeSource :
      ReviewedFq12SharedWitnessCanonicalFullOpeningNativeValiditySource
        sharedWitnessCertificate) :
    @ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate
      CanonicalParser
      WitnessParser
      predicates where
  sharedWitnessCertificate := sharedWitnessCertificate
  fieldSourcesFromSharedWitnessCertificate :=
    fieldSourcesFromSharedWitnessCertificate
  laws :=
    reviewedFq12HardExtractionLawsOfSharedWitnessFullOpeningSource
      sharedWitnessCertificate
      fieldSourcesFromSharedWitnessCertificate
      (reviewedFq12SharedWitnessFullOpeningNativeValiditySourceOfCanonicalSource
        sharedWitnessCertificate canonicalNativeSource)

/--
Forget the shared-witness hard-extraction certificate into the generic hard
predicate-model package.
-/
def toHardExtractionPredicateModelPackage
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {predicates : Path2NativeBoundary.Predicates}
    (Cert :
      @ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate
        CanonicalParser
        WitnessParser
        predicates) :
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelPackage where
  predicates := predicates
  fieldSources :=
    reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
      Cert.sharedWitnessCertificate
  fieldSourcesFromPredicates := Cert.fieldSourcesFromSharedWitnessCertificate
  laws := Cert.laws

end ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate

namespace ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources

/--
Project the complete selected predicate-source record to the reviewed-Fq12
provider family that must eventually be backed by concrete Rust relation
definitions.
-/
def toReviewedFq12PredicateFieldSources
    (Sources :
      ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources) :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12PredicateFieldSources where
  selectedReviewedFq12PayloadAccepts :=
    Sources.selectedReviewedFq12PayloadAccepts
  selectedReviewedFq12PayloadProjectsFrom :=
    Sources.selectedReviewedFq12PayloadProjectsFrom
  selectedReviewedFq12PayloadHasExtractionCertificate :=
    Sources.selectedReviewedFq12PayloadHasExtractionCertificate
  selectedReviewedFq12ExtractionCertificateValid :=
    Sources.selectedReviewedFq12ExtractionCertificateValid
  selectedReviewedFq12CertificateExtracts :=
    Sources.selectedReviewedFq12CertificateExtracts
  selectedReviewedFq12CertificateLiftsToOpening :=
    Sources.selectedReviewedFq12CertificateLiftsToOpening
  selectedReviewedFq12PayloadHasRoleLiftCertificate :=
    Sources.selectedReviewedFq12PayloadHasRoleLiftCertificate
  selectedReviewedFq12RoleLiftCertificateValid :=
    Sources.selectedReviewedFq12RoleLiftCertificateValid
  selectedReviewedFq12RoleLiftCertificateOpens :=
    Sources.selectedReviewedFq12RoleLiftCertificateOpens
  selectedFullNativeOpeningAssemblesRepresentative :=
    Sources.selectedFullNativeOpeningAssemblesRepresentative
  selectedReviewedFq12OpeningHasAssemblyCertificate :=
    Sources.selectedReviewedFq12OpeningHasAssemblyCertificate
  selectedReviewedFq12AssemblyCertificateValid :=
    Sources.selectedReviewedFq12AssemblyCertificateValid
  selectedReviewedFq12AssemblyCertificateAssembles :=
    Sources.selectedReviewedFq12AssemblyCertificateAssembles
  selectedReviewedFq12HasExtractionCompatibilityCertificate :=
    Sources.selectedReviewedFq12HasExtractionCompatibilityCertificate
  selectedReviewedFq12ExtractionCompatibilityCertificateValid :=
    Sources.selectedReviewedFq12ExtractionCompatibilityCertificateValid
  selectedReviewedFq12ExtractionCompatibilityCertificateExtracts :=
    Sources.selectedReviewedFq12ExtractionCompatibilityCertificateExtracts
  selectedReviewedFq12HasFullOpeningBridgeDigestCertificate :=
    Sources.selectedReviewedFq12HasFullOpeningBridgeDigestCertificate
  selectedReviewedFq12FullOpeningBridgeDigestCertificateValid :=
    Sources.selectedReviewedFq12FullOpeningBridgeDigestCertificateValid
  selectedReviewedFq12PayloadExtracts :=
    Sources.selectedReviewedFq12PayloadExtracts

/--
Project the complete selected predicate-source record to the
representative-producing reviewed-Fq12 subfamily.
-/
def toReviewedFq12HardExtractionPredicateFieldSources
    (Sources :
      ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources) :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12HardExtractionPredicateFieldSources :=
  Sources.toReviewedFq12PredicateFieldSources.toHardExtractionPredicateFieldSources

/--
The selected predicate field-source package uses the supplied shared-witness
certificate for exactly its five hard reviewed-Fq12 extraction fields.
-/
theorem selectedHardFieldsFromSharedWitnessCertificate
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    (selectedRustNativeBoundaryPredicateFieldSources
        Cert).toReviewedFq12HardExtractionPredicateFieldSources =
      reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
        Cert := rfl

/-- Forget the selected field-source package into the native boundary record. -/
def toPredicates
    (Sources :
      ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources) :
    Path2NativeBoundary.Predicates where
  boundaryOf := Sources.selectedBoundaryOf
  localValid := Sources.selectedLocalValid
  descentConsistent := Sources.selectedDescentConsistent
  normRangeValid := Sources.selectedNormRangeValid
  blindingNoiseBounded := Sources.selectedBlindingNoiseBounded
  safeEquivalent := Sources.selectedSafeEquivalent
  boundaryBreakFor := Sources.selectedBoundaryBreakFor
  accountCreateAccepts := Sources.selectedAccountCreateAccepts
  admitted := Sources.selectedAdmitted
  parentRootBindsPrivateRoles := Sources.selectedParentRootBindsPrivateRoles
  roleRestrictionsAuthenticate :=
    Sources.selectedRoleRestrictionsAuthenticate
  noExportHolds := Sources.selectedNoExportHolds
  roleAuthenticationMaterialSimulatable :=
    Sources.selectedRoleAuthenticationMaterialSimulatable
  maskFreshnessHolds := Sources.selectedMaskFreshnessHolds
  qromScheduleBound := Sources.selectedQromScheduleBound
  qromExtractorRewindBound := Sources.selectedQromExtractorRewindBound
  rangeRowsBindRepresentative :=
    Sources.selectedRangeRowsBindRepresentative
  rangeRowsBindTraceRepresentative :=
    Sources.selectedRangeRowsBindTraceRepresentative
  blindingRowsBoundRepresentative :=
    Sources.selectedBlindingRowsBoundRepresentative
  blindingRowsBoundTraceRepresentative :=
    Sources.selectedBlindingRowsBoundTraceRepresentative
  transparentAccepts := Sources.selectedTransparentAccepts
  transparentExtractsTrace := Sources.selectedTransparentExtractsTrace
  transparentExtracts := Sources.selectedTransparentExtracts
  reviewedFq12PayloadAccepts :=
    Sources.selectedReviewedFq12PayloadAccepts
  reviewedFq12PayloadProjectsFrom :=
    Sources.selectedReviewedFq12PayloadProjectsFrom
  reviewedFq12PayloadHasExtractionCertificate :=
    Sources.selectedReviewedFq12PayloadHasExtractionCertificate
  reviewedFq12ExtractionCertificateValid :=
    Sources.selectedReviewedFq12ExtractionCertificateValid
  reviewedFq12CertificateExtracts :=
    Sources.selectedReviewedFq12CertificateExtracts
  reviewedFq12CertificateLiftsToOpening :=
    Sources.selectedReviewedFq12CertificateLiftsToOpening
  reviewedFq12PayloadHasRoleLiftCertificate :=
    Sources.selectedReviewedFq12PayloadHasRoleLiftCertificate
  reviewedFq12RoleLiftCertificateValid :=
    Sources.selectedReviewedFq12RoleLiftCertificateValid
  reviewedFq12RoleLiftCertificateOpens :=
    Sources.selectedReviewedFq12RoleLiftCertificateOpens
  fullNativeOpeningAssemblesRepresentative :=
    Sources.selectedFullNativeOpeningAssemblesRepresentative
  reviewedFq12OpeningHasAssemblyCertificate :=
    Sources.selectedReviewedFq12OpeningHasAssemblyCertificate
  reviewedFq12AssemblyCertificateValid :=
    Sources.selectedReviewedFq12AssemblyCertificateValid
  reviewedFq12AssemblyCertificateAssembles :=
    Sources.selectedReviewedFq12AssemblyCertificateAssembles
  reviewedFq12HasExtractionCompatibilityCertificate :=
    Sources.selectedReviewedFq12HasExtractionCompatibilityCertificate
  reviewedFq12ExtractionCompatibilityCertificateValid :=
    Sources.selectedReviewedFq12ExtractionCompatibilityCertificateValid
  reviewedFq12ExtractionCompatibilityCertificateExtracts :=
    Sources.selectedReviewedFq12ExtractionCompatibilityCertificateExtracts
  reviewedFq12HasFullOpeningBridgeDigestCertificate :=
    Sources.selectedReviewedFq12HasFullOpeningBridgeDigestCertificate
  reviewedFq12FullOpeningBridgeDigestCertificateValid :=
    Sources.selectedReviewedFq12FullOpeningBridgeDigestCertificateValid
  reviewedFq12PayloadExtracts :=
    Sources.selectedReviewedFq12PayloadExtracts
  traceOpensBoundary := Sources.selectedTraceOpensBoundary
  tracesSafetyEquivalent := Sources.selectedTracesSafetyEquivalent
  maskedOpeningsConsistent := Sources.selectedMaskedOpeningsConsistent
  traceRelationValid := Sources.selectedTraceRelationValid
  traceAssemblesRepresentative := Sources.selectedTraceAssemblesRepresentative
  extractedRolesComplete := Sources.selectedExtractedRolesComplete
  traceAssemblyRolesComplete := Sources.selectedTraceAssemblyRolesComplete
  traceAssemblyDerivedRolesBound :=
    Sources.selectedTraceAssemblyDerivedRolesBound
  traceAssemblyInFiber := Sources.selectedTraceAssemblyInFiber
  traceAssemblyLocalValid := Sources.selectedTraceAssemblyLocalValid
  traceAssemblyDescentConsistent :=
    Sources.selectedTraceAssemblyDescentConsistent
  slotAuthorizationOf := Sources.selectedSlotAuthorizationOf
  spendAccepts := Sources.selectedSpendAccepts
  spendWitnessValid := Sources.selectedSpendWitnessValid

/--
The shared-witness hard fields are the hard-field projection of the selected
native-boundary predicate record built from that certificate.
-/
theorem selectedHardFieldsFromSharedWitnessCertificate_toPredicates
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
        Cert =
      reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
        (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates :=
  rfl

end ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources

/--
Proof-bearing selected Rust native-boundary predicate field-source package.

This certificate keeps the raw provider package from becoming an untracked
assumption: the package route must be backed by the public selected
predicate-model report/digest surface before the larger predicate-model
certificate can consume it.
-/
structure ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate
    where
  evidenceReport :
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport
  evidenceDigest : ByteArray
  evidenceReportBound :
    evidenceReport = selectedRustNativeBoundaryPredicateModelEvidenceReport
  evidenceDigestBound :
    evidenceDigest = selectedRustNativeBoundaryPredicateModelEvidenceDigest
  evidenceReportVerifiedProof :
    evidenceReport.ReportVerified
  fieldSourcePackagePresentProof :
    evidenceReport.nativeBoundaryPredicateFieldSourcePackagePresent = true
  predicateFieldSources :
    ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources

namespace ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate

/--
Named constructor for the selected predicate field-source package certificate.
-/
def ofReportAndSources
    (evidenceReport :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport)
    (evidenceDigest : ByteArray)
    (evidenceReportBound :
      evidenceReport = selectedRustNativeBoundaryPredicateModelEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustNativeBoundaryPredicateModelEvidenceDigest)
    (evidenceReportVerifiedProof :
      evidenceReport.ReportVerified)
    (fieldSourcePackagePresentProof :
      evidenceReport.nativeBoundaryPredicateFieldSourcePackagePresent = true)
    (predicateFieldSources :
      ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources) :
    ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  evidenceReportBound := evidenceReportBound
  evidenceDigestBound := evidenceDigestBound
  evidenceReportVerifiedProof := evidenceReportVerifiedProof
  fieldSourcePackagePresentProof := fieldSourcePackagePresentProof
  predicateFieldSources := predicateFieldSources

/--
Project a proof-bearing field-source package certificate to the reviewed-Fq12
provider family.
-/
def toReviewedFq12PredicateFieldSources
    (Cert :
      ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate) :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12PredicateFieldSources :=
  Cert.predicateFieldSources.toReviewedFq12PredicateFieldSources

/--
Project a proof-bearing field-source package certificate to the
representative-producing reviewed-Fq12 provider subfamily.
-/
def toReviewedFq12HardExtractionPredicateFieldSources
    (Cert :
      ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate) :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12HardExtractionPredicateFieldSources :=
  Cert.predicateFieldSources.toReviewedFq12HardExtractionPredicateFieldSources

end ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate

/-- Canonical semantic context used only to witness non-empty selected predicates. -/
def selectedRustCanonicalContext : Context where
  profileVersionDigest := 0
  verifierVersionDigest := 0
  accountId := 0
  publicKeyIdentity := 0
  slotId := 0
  accountStateRootBefore := 0
  lifecycleAnchor := 0
  aggregationContextDigest := 0

/-- Canonical semantic boundary used only to witness non-empty selected predicates. -/
def selectedRustCanonicalBoundary : Path2NativeBoundary.Boundary where
  boundaryVersion := 0
  context := selectedRustCanonicalContext
  parentSectionRoot := 0
  roleProfileDigest := 0
  roleHandles := {
    packedWitnessHandle := 0
    randomizerHandle := 0
    padSourceHandle := 0
    openingTupleHandle := 0
    projectionRelationHandle := 0
  }
  maskedProjection := fun _ => 0
  liveStatusMarker := 0
  creationStatementDigest := 0
  nativeBoundaryDigest := 0

/-- Canonical semantic payload used only to witness non-empty selected predicates. -/
def selectedRustCanonicalPayload : ReviewedFq12HiddenSourcePayload where
  statementDigest := 0
  creationStatementDigest := 0
  creationProofBindingDigest := 0
  randomizerCommitment := 0
  openingCommitment := 0
  padSeedDigest := 0
  projectionCarrierDigest := 0
  checkedCoreTranscriptDigest := 0
  proofDigest := 0
  nativeBoundaryDigest := 0
  slotId := 0

/-- The selected payload-acceptance predicate is not empty. -/
theorem selectedRustPayloadAcceptanceNotVacuous
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ¬ (forall b payload,
      ¬ (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates.reviewedFq12PayloadAccepts
          b payload) := by
  intro hEmpty
  have hAccepts :
      (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates.reviewedFq12PayloadAccepts
        selectedRustCanonicalBoundary selectedRustCanonicalPayload := by
    simp [
      ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources.toPredicates,
      selectedRustNativeBoundaryPredicateFieldSources,
      selectedReviewedFq12PayloadAccepts,
      selectedRustCanonicalBoundary,
      selectedRustCanonicalPayload,
      selectedRustCanonicalContext
    ]
  exact hEmpty selectedRustCanonicalBoundary selectedRustCanonicalPayload hAccepts

/-- The selected boundary-break predicate is not the universal break relation. -/
theorem selectedRustBoundaryBreaksNotUniversal
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ¬ (forall c b breakEvent,
      (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates.boundaryBreakFor
          c b breakEvent) := by
  intro hUniversal
  have hBreak :=
    hUniversal selectedRustCanonicalContext selectedRustCanonicalBoundary
      Path2NativeBoundary.BreakEvent.descentSoundnessBreak
  simp [
    ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources.toPredicates,
    selectedRustNativeBoundaryPredicateFieldSources,
    selectedBoundaryBreakFor
  ] at hBreak

/-- The selected payload-acceptance model projects back to the same boundary. -/
theorem selectedRustPayloadAcceptanceMatchesBoundary
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12PayloadAcceptanceMatchesBoundary
      (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates := by
  intro b payload hAccepts
  simpa [
    ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources.toPredicates,
    selectedRustNativeBoundaryPredicateFieldSources,
    selectedReviewedFq12PayloadAccepts
  ] using hAccepts

/--
The selected extraction-certificate availability predicate binds certificates
to the same native boundary and accepted payload.
-/
theorem selectedRustExtractionCertificateMatchesPayload
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12ExtractionCertificateMatchesPayload
      (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates := by
  intro b payload cert hHasCert
  rcases hHasCert with ⟨hAccepts, hBind⟩
  rcases hAccepts with ⟨hPayloadDigest, hPayloadSlot⟩
  rcases hBind with
    ⟨hCertDigest, hCertSlot, hCreation, hReviewed, _hOpening, _hPad⟩
  exact ⟨hCertDigest.trans hPayloadDigest, hCertSlot.trans hPayloadSlot,
    hCreation, hReviewed⟩

/--
The selected extraction-certificate validity predicate binds certificates to
the same native boundary and accepted payload.
-/
theorem selectedRustExtractionCertificateValidityMatchesPayload
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12ExtractionCertificateValidityMatchesPayload
      (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates := by
  intro b payload cert hValid
  rcases hValid with ⟨hProjects, hBind⟩
  rcases hProjects with ⟨hPayloadDigest, hPayloadSlot⟩
  rcases hBind with
    ⟨hCertDigest, hCertSlot, hCreation, hReviewed, _hOpening, _hPad⟩
  exact ⟨hCertDigest.trans hPayloadDigest, hCertSlot.trans hPayloadSlot,
    hCreation, hReviewed⟩

/-- Selected role-lift validity exposes the role-lift extraction binding. -/
theorem selectedRustRoleLiftValidityBindsExtraction
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12RoleLiftValidityBindsExtractionTarget
      (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates := by
  intro b payload cert liftCert hValid
  exact hValid.2

/-- Selected role-lift opening entails the certificate-to-opening lift relation. -/
theorem selectedRustRoleLiftOpeningImpliesCertificateLift
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12RoleLiftOpeningImpliesCertificateLiftTarget
      (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates := by
  intro b payload cert liftCert opening hOpens
  exact hOpens.1

/-- Selected assembly validity exposes the assembly/opening binding. -/
theorem selectedRustAssemblyValidityBindsOpening
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12AssemblyValidityBindsOpeningTarget
      (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates := by
  intro b payload cert opening assemblyCert hValid
  exact hValid.2

/-- Selected compatibility validity exposes the compatibility binding. -/
theorem selectedRustCompatibilityValidityBinds
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12CompatibilityValidityBindsTarget
      (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates := by
  intro b payload cert opening h compatCert hValid
  exact hValid.2

/-- Selected full-opening bridge validity exposes the aggregate bridge binding. -/
theorem selectedRustFullOpeningBridgeDigestValidityBinds
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Path2NativeBoundary.Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12FullOpeningBridgeDigestValidityBindsTarget
      (selectedRustNativeBoundaryPredicateFieldSources Cert).toPredicates := by
  intro b payload cert liftCert assemblyCert compatCert bridgeCert hValid
  exact hValid.2

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

/--
Project the predicate-model source to the laws governing the
representative-producing reviewed-Fq12 provider subfamily.
-/
def toHardExtractionPredicateModelLaws
    (Model : ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource) :
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelLaws
      Model.predicates where
  certificateExtractionImpliesPayloadExtraction :=
    Model.certificateExtractionImpliesPayloadExtraction
  payloadExtractionInBoundaryFiber :=
    Model.payloadExtractionInBoundaryFiber
  payloadExtractionNativeValid :=
    Model.payloadExtractionNativeValid
  assemblyCertificateImpliesFullNativeAssembly :=
    Model.assemblyCertificateImpliesFullNativeAssembly
  compatibilityExtractionImpliesReviewedExtraction :=
    Model.compatibilityExtractionImpliesReviewedExtraction

/--
Project the predicate-model source to the complete hard-extraction provider
package over the same predicate interpretation.
-/
def toHardExtractionPredicateModelPackage
    (Model : ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource) :
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelPackage where
  predicates := Model.predicates
  fieldSources :=
    reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
      Model.predicates
  fieldSourcesFromPredicates := rfl
  laws := Model.toHardExtractionPredicateModelLaws

end ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource

/--
Selected Rust predicate fixed-point source.

This is the non-circular handoff shape for the final model import: it supplies
field sources, a shared-witness certificate over exactly the predicates
obtained from those field sources, a proof that the selected package built
from that certificate has the same predicate record, and the canonical
native-validity source for the same certificate.
-/
structure ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    where
  predicateFieldSources :
    ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources
  sharedWitnessCertificate :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
      CanonicalParser WitnessParser predicateFieldSources.toPredicates
  predicatesFromSharedWitness :
    (selectedRustNativeBoundaryPredicateFieldSources
      sharedWitnessCertificate).toPredicates =
      predicateFieldSources.toPredicates
  canonicalNativeSource :
    ReviewedFq12SharedWitnessCanonicalFullOpeningNativeValiditySource
      sharedWitnessCertificate

namespace ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource

/-- The selected hard fields of a fixed-point source are the shared-witness fields. -/
theorem hardFieldSourcesFromSharedWitness
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Source :
      @ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource
        CanonicalParser WitnessParser) :
    reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate
        Source.sharedWitnessCertificate =
      reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
        Source.predicateFieldSources.toPredicates := by
  exact
    ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources.selectedHardFieldsFromSharedWitnessCertificate_toPredicates
      Source.sharedWitnessCertificate |>.trans
        (congrArg reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
          Source.predicatesFromSharedWitness)

/--
Build the hard-extraction certificate from a fixed-point source and its
canonical native-validity witness.
-/
def toHardExtractionSharedWitnessPredicateModelCertificate
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Source :
      @ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource
        CanonicalParser WitnessParser) :
    @ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate
      CanonicalParser
      WitnessParser
      Source.predicateFieldSources.toPredicates :=
  ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate.ofCanonicalNativeSource
    Source.sharedWitnessCertificate
    Source.hardFieldSourcesFromSharedWitness
    Source.canonicalNativeSource

/--
Forget a fixed-point source into the predicate-model source consumed by the
full selected Rust certificate.
-/
def toPredicateModelSource
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Source :
      @ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource
        CanonicalParser WitnessParser) :
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource where
  predicates := Source.predicateFieldSources.toPredicates
  payloadAcceptanceNotVacuous := by
    exact Source.predicatesFromSharedWitness ▸
      selectedRustPayloadAcceptanceNotVacuous Source.sharedWitnessCertificate
  boundaryBreaksNotUniversal := by
    exact Source.predicatesFromSharedWitness ▸
      selectedRustBoundaryBreaksNotUniversal Source.sharedWitnessCertificate
  payloadAcceptanceMatchesBoundary := by
    exact Source.predicatesFromSharedWitness ▸
      selectedRustPayloadAcceptanceMatchesBoundary Source.sharedWitnessCertificate
  extractionCertificateMatchesPayload := by
    exact Source.predicatesFromSharedWitness ▸
      selectedRustExtractionCertificateMatchesPayload
        Source.sharedWitnessCertificate
  extractionCertificateValidityMatchesPayload := by
    exact Source.predicatesFromSharedWitness ▸
      selectedRustExtractionCertificateValidityMatchesPayload
        Source.sharedWitnessCertificate
  certificateExtractionImpliesPayloadExtraction :=
    Source.toHardExtractionSharedWitnessPredicateModelCertificate.laws.certificateExtractionImpliesPayloadExtraction
  payloadExtractionInBoundaryFiber :=
    Source.toHardExtractionSharedWitnessPredicateModelCertificate.laws.payloadExtractionInBoundaryFiber
  payloadExtractionNativeValid :=
    Source.toHardExtractionSharedWitnessPredicateModelCertificate.laws.payloadExtractionNativeValid
  roleLiftValidityBindsExtraction := by
    exact Source.predicatesFromSharedWitness ▸
      selectedRustRoleLiftValidityBindsExtraction Source.sharedWitnessCertificate
  roleLiftOpeningImpliesCertificateLift := by
    exact Source.predicatesFromSharedWitness ▸
      selectedRustRoleLiftOpeningImpliesCertificateLift
        Source.sharedWitnessCertificate
  assemblyValidityBindsOpening := by
    exact Source.predicatesFromSharedWitness ▸
      selectedRustAssemblyValidityBindsOpening Source.sharedWitnessCertificate
  assemblyCertificateImpliesFullNativeAssembly :=
    Source.toHardExtractionSharedWitnessPredicateModelCertificate.laws.assemblyCertificateImpliesFullNativeAssembly
  compatibilityValidityBinds := by
    exact Source.predicatesFromSharedWitness ▸
      selectedRustCompatibilityValidityBinds Source.sharedWitnessCertificate
  compatibilityExtractionImpliesReviewedExtraction :=
    Source.toHardExtractionSharedWitnessPredicateModelCertificate.laws.compatibilityExtractionImpliesReviewedExtraction
  fullOpeningBridgeDigestValidityBinds := by
    exact Source.predicatesFromSharedWitness ▸
      selectedRustFullOpeningBridgeDigestValidityBinds
        Source.sharedWitnessCertificate

end ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource

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
  evidenceReportBound :
    evidenceReport = selectedRustNativeBoundaryPredicateModelEvidenceReport
  evidenceDigestBound :
    evidenceDigest = selectedRustNativeBoundaryPredicateModelEvidenceDigest
  evidenceReportVerifiedProof :
    evidenceReport.ReportVerified
  evidenceFactsAllTrueProof :
    evidenceReport.FactsAllTrue
  predicateFieldSourceCertificate :
    ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate
  predicateFieldSources :
    ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources
  predicateFieldSourcesFromCertificate :
    predicateFieldSources =
      predicateFieldSourceCertificate.predicateFieldSources
  predicates : Predicates
  predicatesFromFieldSources :
    predicates = predicateFieldSources.toPredicates
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
    (evidenceReportBound :
      evidenceReport = selectedRustNativeBoundaryPredicateModelEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustNativeBoundaryPredicateModelEvidenceDigest)
    (evidenceReportVerifiedProof :
      evidenceReport.ReportVerified)
    (evidenceFactsAllTrueProof :
      evidenceReport.FactsAllTrue)
    (predicateFieldSourceCertificate :
      ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate)
    (payloadAcceptanceNotVacuous :
      ¬ (forall b payload,
        ¬ predicateFieldSourceCertificate.predicateFieldSources.toPredicates.reviewedFq12PayloadAccepts
            b payload))
    (boundaryBreaksNotUniversal :
      ¬ (forall c b breakEvent,
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates.boundaryBreakFor
          c b breakEvent))
    (payloadAcceptanceMatchesBoundary :
      ReviewedFq12PayloadAcceptanceMatchesBoundary
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (extractionCertificateMatchesPayload :
      ReviewedFq12ExtractionCertificateMatchesPayload
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (extractionCertificateValidityMatchesPayload :
      ReviewedFq12ExtractionCertificateValidityMatchesPayload
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (certificateExtractionImpliesPayloadExtraction :
      ReviewedFq12CertificateExtractionImpliesPayloadExtraction
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (payloadExtractionInBoundaryFiber :
      ReviewedFq12PayloadExtractionInBoundaryFiber
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (payloadExtractionNativeValid :
      ReviewedFq12PayloadExtractionNativeValid
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (roleLiftValidityBindsExtraction :
      ReviewedFq12RoleLiftValidityBindsExtractionTarget
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (roleLiftOpeningImpliesCertificateLift :
      ReviewedFq12RoleLiftOpeningImpliesCertificateLiftTarget
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (assemblyValidityBindsOpening :
      ReviewedFq12AssemblyValidityBindsOpeningTarget
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (assemblyCertificateImpliesFullNativeAssembly :
      ReviewedFq12AssemblyCertificateImpliesFullNativeAssemblyTarget
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (compatibilityValidityBinds :
      ReviewedFq12CompatibilityValidityBindsTarget
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (compatibilityExtractionImpliesReviewedExtraction :
      ReviewedFq12CompatibilityExtractionImpliesReviewedExtractionTarget
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates)
    (fullOpeningBridgeDigestValidityBinds :
      ReviewedFq12FullOpeningBridgeDigestValidityBindsTarget
        predicateFieldSourceCertificate.predicateFieldSources.toPredicates) :
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  evidenceReportBound := evidenceReportBound
  evidenceDigestBound := evidenceDigestBound
  evidenceReportVerifiedProof := evidenceReportVerifiedProof
  evidenceFactsAllTrueProof := evidenceFactsAllTrueProof
  predicateFieldSourceCertificate := predicateFieldSourceCertificate
  predicateFieldSources := predicateFieldSourceCertificate.predicateFieldSources
  predicateFieldSourcesFromCertificate := rfl
  predicates := predicateFieldSourceCertificate.predicateFieldSources.toPredicates
  predicatesFromFieldSources := rfl
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
Construct a predicate-model certificate from public report proofs, the
field-source package certificate, and the selected fixed-point source.

This is the constructor shape the final Rust/model import should use: the
fixed-point source derives all model laws, while this certificate still
requires the public report/digest and `FactsAllTrue` proofs.
-/
def ofReportAndFixedPointSource
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (evidenceReport :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport)
    (evidenceDigest : ByteArray)
    (evidenceReportBound :
      evidenceReport = selectedRustNativeBoundaryPredicateModelEvidenceReport)
    (evidenceDigestBound :
      evidenceDigest = selectedRustNativeBoundaryPredicateModelEvidenceDigest)
    (evidenceReportVerifiedProof :
      evidenceReport.ReportVerified)
    (evidenceFactsAllTrueProof :
      evidenceReport.FactsAllTrue)
    (predicateFieldSourceCertificate :
      ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate)
    (fixedPointSource :
      @ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource
        CanonicalParser WitnessParser)
    (fieldSourcesBound :
      predicateFieldSourceCertificate.predicateFieldSources =
        fixedPointSource.predicateFieldSources) :
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate :=
  ofReportAndModel
    evidenceReport
    evidenceDigest
    evidenceReportBound
    evidenceDigestBound
    evidenceReportVerifiedProof
    evidenceFactsAllTrueProof
    predicateFieldSourceCertificate
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.payloadAcceptanceNotVacuous)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.boundaryBreaksNotUniversal)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.payloadAcceptanceMatchesBoundary)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.extractionCertificateMatchesPayload)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.extractionCertificateValidityMatchesPayload)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.certificateExtractionImpliesPayloadExtraction)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.payloadExtractionInBoundaryFiber)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.payloadExtractionNativeValid)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.roleLiftValidityBindsExtraction)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.roleLiftOpeningImpliesCertificateLift)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.assemblyValidityBindsOpening)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.assemblyCertificateImpliesFullNativeAssembly)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.compatibilityValidityBinds)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.compatibilityExtractionImpliesReviewedExtraction)
    (by
      rw [fieldSourcesBound]
      exact fixedPointSource.toPredicateModelSource.fullOpeningBridgeDigestValidityBinds)

/--
Project a proof-bearing predicate-model certificate to the reviewed-Fq12
provider family that backs its native-boundary predicate interpretation.
-/
def toReviewedFq12PredicateFieldSources
    (Cert :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate) :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12PredicateFieldSources :=
  Cert.predicateFieldSourceCertificate.toReviewedFq12PredicateFieldSources

/--
Project a proof-bearing predicate-model certificate to the
representative-producing reviewed-Fq12 provider subfamily.
-/
def toReviewedFq12HardExtractionPredicateFieldSources
    (Cert :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate) :
    ReviewedFq12SelectedRustNativeBoundaryReviewedFq12HardExtractionPredicateFieldSources :=
  Cert.predicateFieldSourceCertificate.toReviewedFq12HardExtractionPredicateFieldSources

/--
Project a proof-bearing predicate-model certificate to the laws governing the
representative-producing reviewed-Fq12 provider subfamily.
-/
def toHardExtractionPredicateModelLaws
    (Cert :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate) :
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelLaws
      Cert.predicates where
  certificateExtractionImpliesPayloadExtraction :=
    Cert.certificateExtractionImpliesPayloadExtraction
  payloadExtractionInBoundaryFiber :=
    Cert.payloadExtractionInBoundaryFiber
  payloadExtractionNativeValid :=
    Cert.payloadExtractionNativeValid
  assemblyCertificateImpliesFullNativeAssembly :=
    Cert.assemblyCertificateImpliesFullNativeAssembly
  compatibilityExtractionImpliesReviewedExtraction :=
    Cert.compatibilityExtractionImpliesReviewedExtraction

/--
Project a proof-bearing predicate-model certificate to the complete
hard-extraction provider package over the certificate's predicate
interpretation.
-/
def toHardExtractionPredicateModelPackage
    (Cert :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate) :
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelPackage where
  predicates := Cert.predicates
  fieldSources :=
    reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates
      Cert.predicates
  fieldSourcesFromPredicates := rfl
  laws := Cert.toHardExtractionPredicateModelLaws

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
Route a full predicate-model certificate through a shared-witness
hard-extraction certificate over the same predicate interpretation.
-/
def hardExtractionSharedWitnessPackageForPredicateModelCertificate
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (ModelCert :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate)
    (HardCert :
      @ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate
        CanonicalParser
        WitnessParser
        ModelCert.predicates) :
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelPackage :=
  HardCert.toHardExtractionPredicateModelPackage

/--
The shared-witness hard package associated with a full predicate-model
certificate has the same hard provider fields as the certificate's generic
hard-package projection.

This is the key consistency fact the final predicate-model route must consume:
the shared-witness hard slice is not a parallel interpretation; it is the
representative-producing reviewed-Fq12 slice of the selected predicate model.
-/
theorem hardExtractionSharedWitnessPackageFieldsMatchPredicateModelCertificate
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (ModelCert :
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate)
    (HardCert :
      @ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate
        CanonicalParser
        WitnessParser
        ModelCert.predicates) :
    (hardExtractionSharedWitnessPackageForPredicateModelCertificate
      ModelCert HardCert).fieldSources =
      ModelCert.toHardExtractionPredicateModelPackage.fieldSources := by
  dsimp [
    hardExtractionSharedWitnessPackageForPredicateModelCertificate,
    ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate.toHardExtractionPredicateModelPackage,
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate.toHardExtractionPredicateModelPackage
  ]
  exact HardCert.fieldSourcesFromSharedWitnessCertificate

/--
The selected predicate-model report now records that the field-source package
surface is present.

This is still not a full predicate-model certificate: the remaining model-law
and concrete-model pins in `FactsAllTrue` stay false.
-/
theorem selectedRustNativeBoundaryPredicateFieldSourcePackagePresentProof :
    selectedRustNativeBoundaryPredicateModelEvidenceReport.nativeBoundaryPredicateFieldSourcePackagePresent =
      true := by
  simp [selectedRustNativeBoundaryPredicateModelEvidenceReport]

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
      ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate) :
    False := by
  exact selectedRustNativeBoundaryPredicateModelEvidenceFactsAllTrue_impossible
    (by simpa [Cert.evidenceReportBound] using Cert.evidenceFactsAllTrueProof)

/-
Conditional selected Rust source-record instantiation.

These definitions name the exact source-record objects the final Rust/model
import must provide. They are parameterized by a proof-bearing predicate-model
certificate and by the seven raw source records, so they do not assert that the
current fail-closed report can build those objects.
-/
section SelectedRustConditionalSourceRecords

variable {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
variable {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
variable {CanonicalParser :
  ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
variable {WitnessParser :
  ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}

variable
  (selectedRustNativeBoundaryPredicateModelCertificate :
    ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate)

abbrev selectedRustCertifiedNativeBoundaryPredicates : Predicates :=
  selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates

variable
  (selectedRustPayloadProjectionLawValue :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload},
      Predicates.reviewedFq12PayloadAccepts
        selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates
        b payload ->
        payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
          payload.slotId = b.context.slotId ->
        Predicates.reviewedFq12PayloadProjectsFrom
          selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates
          b payload)
  (selectedRustExtractionCertificateAvailabilityValue :
    ReviewedFq12ExtractionCertificateAvailability
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates)
  (selectedRustRoleRootLiftSourceValue :
    ReviewedFq12RoleRootFunctionalLiftSource
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates)
  (selectedRustSourceRouteByteSourceValue :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates)
  (selectedRustAssemblySourceValue :
    ReviewedFq12FieldConcreteAssemblyLocalSource
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates)
  (selectedRustCompatibilitySourceValue :
    ReviewedFq12FieldConcreteCompatibilitySource
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates)
  (selectedRustSharedFrameWitnessSourcesValue :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
      CanonicalParser WitnessParser
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates)

def selectedRustBoundaryBaseSourceBindingDigest : ByteArray :=
  selectedRustBoundaryBaseEvidenceDigest

def selectedRustRoleRootLiftSourceBindingDigest : ByteArray :=
  selectedRustRoleRootLiftEvidenceDigest

def selectedRustSourceRouteSourceBindingDigest : ByteArray :=
  selectedRustSourceRouteEvidenceDigest

def selectedRustFieldLocalSourceBindingDigest : ByteArray :=
  selectedRustFieldLocalEvidenceDigest

def selectedRustSharedWitnessSourceBindingDigest : ByteArray :=
  selectedRustSharedFrameWitnessEvidenceDigest

def selectedRustBoundaryBaseEvidenceSourceBinding :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceSourceBinding
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  ReviewedFq12SelectedRustBoundaryBaseEvidenceSourceBinding.ofReportAndSources
    selectedRustBoundaryBaseEvidenceReport
    selectedRustBoundaryBaseEvidenceDigest
    selectedRustBoundaryBaseSourceBindingDigest
    rfl
    rfl
    rfl
    selectedRustPayloadProjectionLawValue
    selectedRustExtractionCertificateAvailabilityValue

def selectedRustRoleRootLiftEvidenceSourceBinding :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceSourceBinding
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  ReviewedFq12SelectedRustRoleRootLiftEvidenceSourceBinding.ofReportAndSource
    selectedRustRoleRootLiftEvidenceReport
    selectedRustRoleRootLiftEvidenceDigest
    selectedRustRoleRootLiftSourceBindingDigest
    rfl
    rfl
    rfl
    selectedRustRoleRootLiftSourceValue

def selectedRustSourceRouteEvidenceSourceBinding :
    ReviewedFq12SelectedRustSourceRouteEvidenceSourceBinding
      FieldParser RouteParser
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  ReviewedFq12SelectedRustSourceRouteEvidenceSourceBinding.ofReportAndSource
    selectedRustSourceRouteEvidenceReport
    selectedRustSourceRouteEvidenceDigest
    selectedRustSourceRouteSourceBindingDigest
    rfl
    rfl
    rfl
    selectedRustSourceRouteByteSourceValue

def selectedRustFieldLocalEvidenceSourceBinding :
    ReviewedFq12SelectedRustFieldLocalEvidenceSourceBinding
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  ReviewedFq12SelectedRustFieldLocalEvidenceSourceBinding.ofReportAndSources
    selectedRustFieldLocalEvidenceReport
    selectedRustFieldLocalEvidenceDigest
    selectedRustFieldLocalSourceBindingDigest
    rfl
    rfl
    rfl
    selectedRustAssemblySourceValue
    selectedRustCompatibilitySourceValue

def selectedRustSharedWitnessEvidenceSourceBinding :
    ReviewedFq12SelectedRustSharedWitnessEvidenceSourceBinding
      CanonicalParser WitnessParser
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  ReviewedFq12SelectedRustSharedWitnessEvidenceSourceBinding.ofReportAndSources
    selectedRustSharedFrameWitnessEvidenceReport
    selectedRustSharedFrameWitnessEvidenceDigest
    selectedRustSharedWitnessSourceBindingDigest
    rfl
    rfl
    rfl
    selectedRustSharedFrameWitnessSourcesValue

def selectedRustPayloadProjectionLaw :
    forall {b : Path2NativeBoundary.Boundary}
        {payload : ReviewedFq12HiddenSourcePayload},
      Predicates.reviewedFq12PayloadAccepts
        selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates
        b payload ->
        payload.nativeBoundaryDigest = b.nativeBoundaryDigest ∧
          payload.slotId = b.context.slotId ->
        Predicates.reviewedFq12PayloadProjectsFrom
          selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates
          b payload :=
  (selectedRustBoundaryBaseEvidenceSourceBinding
    selectedRustNativeBoundaryPredicateModelCertificate
    selectedRustPayloadProjectionLawValue
    selectedRustExtractionCertificateAvailabilityValue).payloadProjectionLaw

def selectedRustExtractionCertificateAvailability :
    ReviewedFq12ExtractionCertificateAvailability
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  (selectedRustBoundaryBaseEvidenceSourceBinding
    selectedRustNativeBoundaryPredicateModelCertificate
    selectedRustPayloadProjectionLawValue
    selectedRustExtractionCertificateAvailabilityValue).extractionCertificateAvailability

def selectedRustRoleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  (selectedRustRoleRootLiftEvidenceSourceBinding
    selectedRustNativeBoundaryPredicateModelCertificate
    selectedRustRoleRootLiftSourceValue).roleRootLiftSource

def selectedRustSourceRouteByteSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  (selectedRustSourceRouteEvidenceSourceBinding
    selectedRustNativeBoundaryPredicateModelCertificate
    selectedRustSourceRouteByteSourceValue).selectedSourceRouteByteSource

def selectedRustAssemblySource :
    ReviewedFq12FieldConcreteAssemblyLocalSource
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  (selectedRustFieldLocalEvidenceSourceBinding
    selectedRustNativeBoundaryPredicateModelCertificate
    selectedRustAssemblySourceValue
    selectedRustCompatibilitySourceValue).assemblySource

def selectedRustCompatibilitySource :
    ReviewedFq12FieldConcreteCompatibilitySource
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  (selectedRustFieldLocalEvidenceSourceBinding
    selectedRustNativeBoundaryPredicateModelCertificate
    selectedRustAssemblySourceValue
    selectedRustCompatibilitySourceValue).compatibilitySource

def selectedRustSharedFrameWitnessSources :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
      CanonicalParser WitnessParser
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  (selectedRustSharedWitnessEvidenceSourceBinding
    selectedRustNativeBoundaryPredicateModelCertificate
    selectedRustSharedFrameWitnessSourcesValue).sharedFrameWitnessSources

def selectedRustBoundaryBaseCertificate :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate.ofReportAndSources
    selectedRustBoundaryBaseEvidenceReport
    selectedRustBoundaryBaseEvidenceDigest
    selectedRustBoundaryBaseEvidenceReportVerifiedProof
    selectedRustBoundaryBaseEvidenceFactsAllTrueProof
    (selectedRustPayloadProjectionLaw
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustPayloadProjectionLawValue
      selectedRustExtractionCertificateAvailabilityValue)
    (selectedRustExtractionCertificateAvailability
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustPayloadProjectionLawValue
      selectedRustExtractionCertificateAvailabilityValue)

def selectedRustRoleRootLiftSourceCertificate :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate.ofReportAndSource
    selectedRustRoleRootLiftEvidenceReport
    selectedRustRoleRootLiftEvidenceDigest
    selectedRustRoleRootLiftEvidenceReportVerifiedProof
    selectedRustRoleRootLiftEvidenceFactsAllTrueProof
    (selectedRustRoleRootLiftSource
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustRoleRootLiftSourceValue)

def selectedRustSourceRouteByteSourceCertificate :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
      FieldParser RouteParser
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate.ofReportAndSource
    selectedRustSourceRouteEvidenceReport
    selectedRustSourceRouteEvidenceDigest
    selectedRustSourceRouteEvidenceReportVerifiedProof
    selectedRustSourceRouteEvidenceFactsAllTrueProof
    (selectedRustSourceRouteByteSource
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustSourceRouteByteSourceValue)

def selectedRustFieldLocalSourceCertificate :
    ReviewedFq12SelectedRustFieldLocalEvidenceCertificate
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  ReviewedFq12SelectedRustFieldLocalEvidenceCertificate.ofReportAndSources
    selectedRustFieldLocalEvidenceReport
    selectedRustFieldLocalEvidenceDigest
    selectedRustFieldLocalEvidenceReportVerifiedProof
    selectedRustFieldLocalEvidenceFactsAllTrueProof
    (selectedRustAssemblySource
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustAssemblySourceValue
      selectedRustCompatibilitySourceValue)
    (selectedRustCompatibilitySource
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustAssemblySourceValue
      selectedRustCompatibilitySourceValue)

def selectedRustSharedFrameWitnessSourcesCertificate :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
      CanonicalParser WitnessParser
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate.ofReportAndSources
    selectedRustSharedFrameWitnessEvidenceReport
    selectedRustSharedFrameWitnessEvidenceDigest
    selectedRustSharedFrameWitnessEvidenceReportVerifiedProof
    selectedRustSharedFrameWitnessEvidenceFactsAllTrueProof
    (selectedRustSharedFrameWitnessSources
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustSharedFrameWitnessSourcesValue)

def selectedRustModelSourceRecordPackage :
    ReviewedFq12SelectedRustModelSourceRecordPackage
      FieldParser RouteParser CanonicalParser WitnessParser
      selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates :=
  ReviewedFq12SelectedRustModelSourceRecordPackage.ofSourceBindings
    (selectedRustBoundaryBaseEvidenceSourceBinding
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustPayloadProjectionLawValue
      selectedRustExtractionCertificateAvailabilityValue)
    (selectedRustRoleRootLiftEvidenceSourceBinding
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustRoleRootLiftSourceValue)
    (selectedRustSourceRouteEvidenceSourceBinding
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustSourceRouteByteSourceValue)
    (selectedRustFieldLocalEvidenceSourceBinding
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustAssemblySourceValue
      selectedRustCompatibilitySourceValue)
    (selectedRustSharedWitnessEvidenceSourceBinding
      selectedRustNativeBoundaryPredicateModelCertificate
      selectedRustSharedFrameWitnessSourcesValue)

theorem selectedRustFiveSourceSubEvidenceDigestsMatchProof :
    selectedRustFiveSourceEvidenceReport.SubEvidenceDigestsMatch
      (selectedRustBoundaryBaseCertificate
        selectedRustNativeBoundaryPredicateModelCertificate
        selectedRustPayloadProjectionLawValue
        selectedRustExtractionCertificateAvailabilityValue)
      (selectedRustRoleRootLiftSourceCertificate
        selectedRustNativeBoundaryPredicateModelCertificate
        selectedRustRoleRootLiftSourceValue)
      (selectedRustSourceRouteByteSourceCertificate
        selectedRustNativeBoundaryPredicateModelCertificate
        selectedRustSourceRouteByteSourceValue)
      (selectedRustFieldLocalSourceCertificate
        selectedRustNativeBoundaryPredicateModelCertificate
        selectedRustAssemblySourceValue
        selectedRustCompatibilitySourceValue)
      (selectedRustSharedFrameWitnessSourcesCertificate
        selectedRustNativeBoundaryPredicateModelCertificate
        selectedRustSharedFrameWitnessSourcesValue) :=
  (selectedRustModelSourceRecordPackage
    selectedRustNativeBoundaryPredicateModelCertificate
    selectedRustPayloadProjectionLawValue
    selectedRustExtractionCertificateAvailabilityValue
    selectedRustRoleRootLiftSourceValue
    selectedRustSourceRouteByteSourceValue
    selectedRustAssemblySourceValue
    selectedRustCompatibilitySourceValue
    selectedRustSharedFrameWitnessSourcesValue).subEvidenceDigestsMatch

end SelectedRustConditionalSourceRecords

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
  reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_source_record_package
    Package.predicateModelCertificate.toSource.toPredicates
    Package.sourceRecords

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
        FieldParser RouteParser CanonicalParser WitnessParser) :
    False :=
  selectedRustNativeBoundaryPredicateModelCertificate_from_current_report_impossible
    Package.predicateModelCertificate

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
The certified full-signature active-path bound cannot be instantiated from the
current fail-closed selected predicate-model report.

This records the blocker at the same layer as the active bound handoff: the
selected predicate-model report is verified, but its concrete-model facts are
false until the native-boundary predicate interpretation is imported.
-/
theorem full_signature_active_path_forgery_bound_from_current_predicate_report_impossible
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (Package :
      ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage
        FieldParser RouteParser CanonicalParser WitnessParser) :
    False :=
  selectedRustNativeBoundaryCertifiedSemanticsSourcePackage_from_current_report_impossible
    Package

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
