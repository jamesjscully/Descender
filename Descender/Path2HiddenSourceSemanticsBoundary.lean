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

end Path2HiddenSourceSemanticsBoundary
end Descender
