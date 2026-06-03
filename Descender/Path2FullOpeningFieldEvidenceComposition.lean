import Descender.Path2FullOpeningFieldEvidenceByteBridge
import Descender.Path2AssemblyNativeValidity
import Descender.Path2CompatibilityConcrete

namespace Descender

/-!
# Path 2 Full-Opening Field-Evidence Composition

This module adapts the byte-backed field-evidence bridge to the concrete facts
consumed by the active P2FS-001 reductions.  It keeps proof ordering explicit:
the pre-aggregate field-evidence frame supplies role-root, assembly, and
compatibility field bindings; the aggregate bridge remains downstream.
-/

namespace Path2FullOpeningFieldEvidenceComposition

open Path2NativeBoundary
open Path2FullOpeningFieldEvidence
open Path2FullOpeningFieldEvidencePredicates
open Path2FullOpeningFieldEvidenceByteBridge
open Path2CompatibilitySoundness
open Path2CompatibilityConcrete

/--
The exact reduction facts produced by one pre-aggregate field-evidence frame.
-/
structure ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  roleLiftBindsExtraction :
    ReviewedFq12RoleLiftCertificateBindsExtraction
      payload cert liftCert
  assemblyPayloadBinding :
    ReviewedFq12AssemblyCertificateBindsOpening
      payload cert liftCert.opening assemblyCert
  assemblyFieldConstruction :
    ReviewedFq12AssemblyFieldConstructionCertificate
      liftCert.opening assemblyCert
  compatibilityPayloadBinding :
    ReviewedFq12ExtractionCompatibilityCertificateBinds
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert
  sameRepresentativeConcreteFields :
    SameRepresentativeConcreteFieldCertificate
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert
  nonRootOpeningFields :
    FullNativeOpeningNonRootRoleFieldEqualityCertificate
      liftCert.opening assemblyCert.opening

/--
The compatibility extraction facts obtained after deterministic assembly and
native validity have been supplied for the same field-evidence frame.
-/
structure ReviewedFq12FullOpeningFieldEvidenceCompatibilityExtracts
    (P : Predicates)
    (b : Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  compatibilityCertificateExtracts :
    P.reviewedFq12ExtractionCompatibilityCertificateExtracts
      b payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert
  extractionCertificateExtracts :
    P.reviewedFq12CertificateExtracts
      b payload cert
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
  payloadExtracts :
    P.reviewedFq12PayloadExtracts
      b payload
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)

/--
After field evidence gives the opening-to-assembly field construction, the
remaining assembly predicates and native-validity certificate give the
post-assembly native-fiber representative premise used by compatibility
soundness.
-/
theorem assembled_native_fiber_representative_of_field_evidence_local_facts
    {P : Predicates}
    {b : Boundary}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (facts :
      ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
        payload cert liftCert assemblyCert compatCert)
    (assemblyPredicates :
      ReviewedFq12AssemblyConstructedRepresentativePredicates
        P b payload cert liftCert.opening assemblyCert)
    (nativeValidity :
      ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
        P b assemblyCert) :
    AssembledNativeFiberRepresentative
      P b payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert) :=
  ⟨facts.assemblyFieldConstruction.matchesOpening,
    assemblyPredicates.2,
    nativeValidity.inFiber,
    nativeValidity.localValid,
    nativeValidity.descentConsistent⟩

/--
Concrete extraction rules applied to the deterministic representative named by
field evidence.
-/
theorem compatibility_extracts_of_field_evidence_local_facts
    (P : Predicates)
    (rules : SameRepresentativeConcreteExtractionPredicates P)
    {b : Boundary}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hLifts :
      P.reviewedFq12CertificateLiftsToOpening
        b payload cert liftCert.opening)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (facts :
      ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
        payload cert liftCert assemblyCert compatCert)
    (assemblyPredicates :
      ReviewedFq12AssemblyConstructedRepresentativePredicates
        P b payload cert liftCert.opening assemblyCert)
    (nativeValidity :
      ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
        P b assemblyCert) :
    ReviewedFq12FullOpeningFieldEvidenceCompatibilityExtracts
      P b payload cert liftCert assemblyCert compatCert := by
  have assembled :
      AssembledNativeFiberRepresentative
        P b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert) :=
    assembled_native_fiber_representative_of_field_evidence_local_facts
      facts assemblyPredicates nativeValidity
  exact
    { compatibilityCertificateExtracts :=
        rules.compatibilityCertificateExtracts
          hHasCompat hCompatValid facts.sameRepresentativeConcreteFields
      extractionCertificateExtracts :=
        rules.extractionCertificateExtracts
          hAccept hProjects hHasExtraction hExtractionValid hLifts
          assembled hHasCompat hCompatValid
          facts.sameRepresentativeConcreteFields
      payloadExtracts :=
        rules.payloadExtracts
          hAccept hProjects hHasExtraction hExtractionValid hLifts
          assembled hHasCompat hCompatValid
          facts.sameRepresentativeConcreteFields }

/--
Semantic field-evidence bindings are already exactly the active reduction
facts, just under a theorem-facing package name.
-/
theorem active_reduction_facts_of_field_evidence_semantic_bindings
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (S :
      ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
        payload cert liftCert assemblyCert compatCert) :
    ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
      payload cert liftCert assemblyCert compatCert :=
  { roleLiftBindsExtraction := S.roleLiftBindsExtraction
    assemblyPayloadBinding := S.assemblyBindsOpening
    assemblyFieldConstruction := S.assemblyFieldConstruction
    compatibilityPayloadBinding :=
      S.compatibilityBindsDeterministicRepresentative
    sameRepresentativeConcreteFields :=
      S.sameRepresentativeConcreteFields
    nonRootOpeningFields :=
      S.nonRootOpeningFieldsEqAssemblyOpening }

/--
A byte predicate source directly feeds the active P2FS-001 reduction facts, or
exports one of the named byte/parser residual breaks.
-/
theorem byte_predicate_source_active_reduction_facts_or_named_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    (exists frameBytes fieldCert,
      Source.hasByteFieldEvidence
          b payload cert liftCert assemblyCert compatCert frameBytes
          fieldCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          Parser frameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
            ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
              payload cert liftCert assemblyCert compatCert) ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      byte_predicate_source_semantic_bindings_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid
        hHasLift hLiftValid hHasAssembly hAssemblyValid hHasCompat
        hCompatValid with
    hField | hBreak
  · rcases hField with
      ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds, hSemantic⟩
    exact Or.inl
      ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds,
        active_reduction_facts_of_field_evidence_semantic_bindings
          hSemantic⟩
  · exact Or.inr hBreak

/--
A paired source-route byte predicate source also feeds the active P2FS-001
reduction facts, while preserving the explicit route frame as evidence that
the aggregate bridge was not used as source.
-/
theorem source_route_byte_predicate_source_active_reduction_facts_or_named_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    (exists fieldFrameBytes routeFrameBytes fieldCert routeCert,
      Source.hasByteFieldSourceRoute
          b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          routeFrameBytes fieldCert routeCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          FieldParser fieldFrameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
            RouteParser routeFrameBytes fieldCert routeCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
            ReviewedFq12FullOpeningFieldSourceRouteBinds
              payload cert liftCert assemblyCert compatCert fieldCert
              routeCert ∧
              ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
                payload cert liftCert assemblyCert compatCert) ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      source_route_byte_predicate_source_semantic_bindings_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hRoute | hBreak
  · rcases hRoute with
      ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
        hFieldByte, hRouteByte, hFieldBinds, hRouteBinds, hSemantic⟩
    exact Or.inl
      ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
        hFieldByte, hRouteByte, hFieldBinds, hRouteBinds,
        active_reduction_facts_of_field_evidence_semantic_bindings
          hSemantic⟩
  · exact Or.inr hBreak

/--
Byte-backed field evidence supplies the role-lift binding fact, or names the
byte/parser residual that prevents using the evidence.
-/
theorem byte_source_role_lift_binding_or_named_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    ReviewedFq12RoleLiftCertificateBindsExtraction
      payload cert liftCert ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hBreak
  · rcases hField with ⟨_frameBytes, _fieldCert, _hHasByte, _hByteCert,
      _hBinds, facts⟩
    exact Or.inl facts.roleLiftBindsExtraction
  · exact Or.inr hBreak

/--
Byte-backed field evidence supplies the assembly binding fact, or names the
byte/parser residual that prevents using the evidence.
-/
theorem byte_source_assembly_binding_or_named_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    ReviewedFq12AssemblyCertificateBindsOpening
      payload cert liftCert.opening assemblyCert ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hBreak
  · rcases hField with ⟨_frameBytes, _fieldCert, _hHasByte, _hByteCert,
      _hBinds, facts⟩
    exact Or.inl facts.assemblyPayloadBinding
  · exact Or.inr hBreak

/--
Byte-backed field evidence supplies the compatibility binding fact for the
deterministic representative, or names the byte/parser residual that prevents
using the evidence.
-/
theorem byte_source_compatibility_binding_or_named_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    ReviewedFq12ExtractionCompatibilityCertificateBinds
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hBreak
  · rcases hField with ⟨_frameBytes, _fieldCert, _hHasByte, _hByteCert,
      _hBinds, facts⟩
    exact Or.inl facts.compatibilityPayloadBinding
  · exact Or.inr hBreak

/--
Byte-backed field evidence supplies the same-representative concrete field
certificate, or names the byte/parser residual that prevents using the
evidence.
-/
theorem byte_source_same_representative_fields_or_named_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    SameRepresentativeConcreteFieldCertificate
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hBreak
  · rcases hField with ⟨_frameBytes, _fieldCert, _hHasByte, _hByteCert,
      _hBinds, facts⟩
    exact Or.inl facts.sameRepresentativeConcreteFields
  · exact Or.inr hBreak

/--
Byte-backed field evidence supplies the non-root opening field equalities
between the role-lift opening and assembly opening, or names the byte/parser
residual that prevents using the evidence.
-/
theorem byte_source_nonroot_opening_fields_or_named_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    FullNativeOpeningNonRootRoleFieldEqualityCertificate
      liftCert.opening assemblyCert.opening ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hBreak
  · rcases hField with ⟨_frameBytes, _fieldCert, _hHasByte, _hByteCert,
      _hBinds, facts⟩
    exact Or.inl facts.nonRootOpeningFields
  · exact Or.inr hBreak

/--
Implementation-side local facts still needed after the byte field-evidence
source has supplied the active reduction fields.
-/
structure ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource
    (P : Predicates) where
  assemblyPredicates :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {liftCert : ReviewedFq12FullNativeOpeningCertificate}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening
                b payload cert liftCert.opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert liftCert.opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert liftCert.opening assemblyCert ->
                  ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
                      payload cert liftCert assemblyCert compatCert ->
                    ReviewedFq12AssemblyConstructedRepresentativePredicates
                        P b payload cert liftCert.opening assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)
  nativeValidity :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {liftCert : ReviewedFq12FullNativeOpeningCertificate}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening
                b payload cert liftCert.opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert liftCert.opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert liftCert.opening assemblyCert ->
                  ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
                      payload cert liftCert assemblyCert compatCert ->
                    ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
                        P b assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)
  extractionRules :
    SameRepresentativeConcreteExtractionPredicates P

/--
Byte-backed field evidence plus the local implementation source supplies the
deterministic assembly predicates for the constructed representative, or names
a boundary break.

This endpoint is compatibility-indexed on purpose: it uses the pre-aggregate
field-evidence object, whose Rust frame binds compatibility along with role
lift and assembly.  It should not be used as the generic source of assembly
availability before compatibility exists.
-/
theorem byte_source_local_assembly_predicates_or_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hLifts :
      P.reviewedFq12CertificateLiftsToOpening
        b payload cert liftCert.opening)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    (exists frameBytes fieldCert,
      Source.hasByteFieldEvidence
          b payload cert liftCert assemblyCert compatCert frameBytes
          fieldCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          Parser frameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
          ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
            payload cert liftCert assemblyCert compatCert ∧
          ReviewedFq12AssemblyConstructedRepresentativePredicates
            P b payload cert liftCert.opening assemblyCert) ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hByteBreak
  · rcases hField with
      ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds, facts⟩
    rcases
        Local.assemblyPredicates hAccept hProjects hHasExtraction
          hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
      assemblyPredicates | hBreak
    · exact Or.inl
        ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds, facts,
          assemblyPredicates⟩
    · exact Or.inr hBreak
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hByteBreak)

/--
Byte-backed field evidence plus the local implementation source supplies the
native-validity certificate for the deterministic assembly representative, or
names a boundary break.
-/
theorem byte_source_local_native_validity_or_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hLifts :
      P.reviewedFq12CertificateLiftsToOpening
        b payload cert liftCert.opening)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    (exists frameBytes fieldCert,
      Source.hasByteFieldEvidence
          b payload cert liftCert assemblyCert compatCert frameBytes
          fieldCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          Parser frameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
          ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
            payload cert liftCert assemblyCert compatCert ∧
          ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
            P b assemblyCert) ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hByteBreak
  · rcases hField with
      ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds, facts⟩
    rcases
        Local.nativeValidity hAccept hProjects hHasExtraction
          hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
      nativeValidity | hBreak
    · exact Or.inl
        ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds, facts,
          nativeValidity⟩
    · exact Or.inr hBreak
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hByteBreak)

/--
Byte-backed field evidence plus local deterministic assembly/native/extraction
facts gives the compatibility extraction package for the constructed
representative, or a boundary break.
-/
theorem byte_source_local_compatibility_extracts_or_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hLifts :
      P.reviewedFq12CertificateLiftsToOpening
        b payload cert liftCert.opening)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    (exists frameBytes fieldCert,
      Source.hasByteFieldEvidence
          b payload cert liftCert assemblyCert compatCert frameBytes
          fieldCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          Parser frameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
            ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
              payload cert liftCert assemblyCert compatCert ∧
              ReviewedFq12FullOpeningFieldEvidenceCompatibilityExtracts
                P b payload cert liftCert assemblyCert compatCert) ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hByteBreak
  · rcases hField with
      ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds, facts⟩
    rcases
        Local.assemblyPredicates hAccept hProjects hHasExtraction
          hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
      assemblyPredicates | hBreak
    · rcases
          Local.nativeValidity hAccept hProjects hHasExtraction
            hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
        nativeValidity | hBreak
      · exact Or.inl
          ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds, facts,
            compatibility_extracts_of_field_evidence_local_facts
              P Local.extractionRules hAccept hProjects hHasExtraction
              hExtractionValid hLifts hHasCompat hCompatValid facts
              assemblyPredicates nativeValidity⟩
      · exact Or.inr hBreak
    · exact Or.inr hBreak
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hByteBreak)

/--
Source-route byte evidence plus the local implementation source supplies the
deterministic assembly predicates for the constructed representative, or names
a boundary break.

Like the field-evidence endpoint above, this remains compatibility-indexed and
does not supply generic pre-compatibility assembly availability.
-/
theorem source_route_byte_source_local_assembly_predicates_or_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hLifts :
      P.reviewedFq12CertificateLiftsToOpening
        b payload cert liftCert.opening)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    (exists fieldFrameBytes routeFrameBytes fieldCert routeCert,
      Source.hasByteFieldSourceRoute
          b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          routeFrameBytes fieldCert routeCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          FieldParser fieldFrameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
            RouteParser routeFrameBytes fieldCert routeCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
            ReviewedFq12FullOpeningFieldSourceRouteBinds
              payload cert liftCert assemblyCert compatCert fieldCert
              routeCert ∧
              ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
                payload cert liftCert assemblyCert compatCert ∧
              ReviewedFq12AssemblyConstructedRepresentativePredicates
                P b payload cert liftCert.opening assemblyCert) ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      source_route_byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hRoute | hByteBreak
  · rcases hRoute with
      ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
        hFieldByte, hRouteByte, hFieldBinds, hRouteBinds, facts⟩
    rcases
        Local.assemblyPredicates hAccept hProjects hHasExtraction
          hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
      assemblyPredicates | hBreak
    · exact Or.inl
        ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
          hFieldByte, hRouteByte, hFieldBinds, hRouteBinds, facts,
          assemblyPredicates⟩
    · exact Or.inr hBreak
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hByteBreak)

/--
Source-route byte evidence plus the local implementation source supplies the
native-validity certificate for the deterministic assembly representative, or
names a boundary break.
-/
theorem source_route_byte_source_local_native_validity_or_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hLifts :
      P.reviewedFq12CertificateLiftsToOpening
        b payload cert liftCert.opening)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    (exists fieldFrameBytes routeFrameBytes fieldCert routeCert,
      Source.hasByteFieldSourceRoute
          b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          routeFrameBytes fieldCert routeCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          FieldParser fieldFrameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
            RouteParser routeFrameBytes fieldCert routeCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
            ReviewedFq12FullOpeningFieldSourceRouteBinds
              payload cert liftCert assemblyCert compatCert fieldCert
              routeCert ∧
              ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
                payload cert liftCert assemblyCert compatCert ∧
              ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
                P b assemblyCert) ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      source_route_byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hRoute | hByteBreak
  · rcases hRoute with
      ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
        hFieldByte, hRouteByte, hFieldBinds, hRouteBinds, facts⟩
    rcases
        Local.nativeValidity hAccept hProjects hHasExtraction
          hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
      nativeValidity | hBreak
    · exact Or.inl
        ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
          hFieldByte, hRouteByte, hFieldBinds, hRouteBinds, facts,
          nativeValidity⟩
    · exact Or.inr hBreak
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hByteBreak)

/--
Source-route byte evidence plus local deterministic assembly/native/extraction
facts gives the compatibility extraction package for the constructed
representative, or a boundary break.
-/
theorem source_route_byte_source_local_compatibility_extracts_or_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHasExtraction :
      P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hExtractionValid :
      P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLift :
      P.reviewedFq12PayloadHasRoleLiftCertificate
        b payload cert liftCert)
    (hLiftValid :
      P.reviewedFq12RoleLiftCertificateValid
        b payload cert liftCert)
    (hLifts :
      P.reviewedFq12CertificateLiftsToOpening
        b payload cert liftCert.opening)
    (hHasAssembly :
      P.reviewedFq12OpeningHasAssemblyCertificate
        b payload cert liftCert.opening assemblyCert)
    (hAssemblyValid :
      P.reviewedFq12AssemblyCertificateValid
        b payload cert liftCert.opening assemblyCert)
    (hHasCompat :
      P.reviewedFq12HasExtractionCompatibilityCertificate
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert)
    (hCompatValid :
      P.reviewedFq12ExtractionCompatibilityCertificateValid
        b payload cert liftCert.opening
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
        compatCert) :
    (exists fieldFrameBytes routeFrameBytes fieldCert routeCert,
      Source.hasByteFieldSourceRoute
          b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          routeFrameBytes fieldCert routeCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          FieldParser fieldFrameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
            RouteParser routeFrameBytes fieldCert routeCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
            ReviewedFq12FullOpeningFieldSourceRouteBinds
              payload cert liftCert assemblyCert compatCert fieldCert
              routeCert ∧
              ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
                payload cert liftCert assemblyCert compatCert ∧
              ReviewedFq12FullOpeningFieldEvidenceCompatibilityExtracts
                P b payload cert liftCert assemblyCert compatCert) ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      source_route_byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hRoute | hByteBreak
  · rcases hRoute with
      ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
        hFieldByte, hRouteByte, hFieldBinds, hRouteBinds, facts⟩
    rcases
        Local.assemblyPredicates hAccept hProjects hHasExtraction
          hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
      assemblyPredicates | hBreak
    · rcases
          Local.nativeValidity hAccept hProjects hHasExtraction
            hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
        nativeValidity | hBreak
      · exact Or.inl
          ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
            hFieldByte, hRouteByte, hFieldBinds, hRouteBinds, facts,
            compatibility_extracts_of_field_evidence_local_facts
              P Local.extractionRules hAccept hProjects hHasExtraction
              hExtractionValid hLifts hHasCompat hCompatValid facts
              assemblyPredicates nativeValidity⟩
      · exact Or.inr hBreak
    · exact Or.inr hBreak
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hByteBreak)

end Path2FullOpeningFieldEvidenceComposition
end Descender
