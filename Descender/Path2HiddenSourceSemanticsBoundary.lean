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
def selectedRustFiveSourceEvidenceReport :
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
def selectedRustBoundaryBaseEvidenceReport :
    ReviewedFq12SelectedRustBoundaryBaseEvidenceReport where
  schemaMatches := false
  reportDigestVerified := false
  payloadProjectionLawPinned := false
  extractionCertificateAvailabilityPinned := false
  nativeBoundaryPayloadBindingPinned := false
  boundaryBaseNegativeControlsPassed := false

/-- Concrete selected Rust/model role-root/lift report value, still fail-closed. -/
def selectedRustRoleRootLiftEvidenceReport :
    ReviewedFq12SelectedRustRoleRootLiftEvidenceReport where
  schemaMatches := false
  reportDigestVerified := false
  concreteRoleRootFieldBindingPinned := false
  selectedLiftFunctionalityPinned := false
  roleRootBridgePinned := false
  roleRootNegativeControlsPassed := false

/-- Concrete selected Rust/model source-route report value, still fail-closed. -/
def selectedRustSourceRouteEvidenceReport :
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
def selectedRustFieldLocalEvidenceReport :
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
def selectedRustSharedFrameWitnessEvidenceReport :
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

/-- Concrete digest of the public Rust five-source evidence report. -/
def selectedRustFiveSourceEvidenceDigest : ByteArray :=
  ByteArray.mk #[11, 182, 157, 92, 3, 36, 31, 145, 79, 21, 67, 87, 94, 61,
    179, 222, 29, 139, 250, 234, 209, 76, 36, 53, 55, 85, 235, 149, 81, 89,
    229, 30]

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
