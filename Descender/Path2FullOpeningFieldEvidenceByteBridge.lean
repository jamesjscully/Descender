import Descender.Path2FullOpeningFieldEvidencePredicates

namespace Descender

/-!
# Path 2 Full-Opening Field-Evidence Byte Bridge

This module names the byte/parser boundary for the Rust
`Path2ReviewedFq12FullOpeningFieldEvidenceV1` frame.  The hooks stay abstract:
the concrete parser, frame hash, and self-frame checks live in the production
reference implementation.  The theorem boundary here forces any instantiation
to prove that those byte hooks recover the pre-aggregate field-evidence
certificate, not the downstream aggregate bridge.
-/

namespace Path2FullOpeningFieldEvidenceByteBridge

open Path2NativeBoundary
open Path2FullOpeningFieldEvidence
open Path2FullOpeningFieldEvidencePredicates
open Path2CompatibilityConcrete

/--
Concrete parser hooks for the pre-aggregate full-opening field-evidence frame.

The three self-frame predicates correspond to the Rust frame checks for the
role root, full opening, and deterministic representative.  `domainSeparated`
is the Rust frame-domain guard for the field-evidence object itself.
-/
structure ReviewedFq12FullOpeningFieldEvidenceByteParser where
  parse : Nat -> ReviewedFq12FullOpeningFieldEvidenceCertificate
  frameDigest : ReviewedFq12FullOpeningFieldEvidenceCertificate -> Nat
  frameBytesCanonical : Nat -> Prop
  roleRootFrameValid :
    ReviewedFq12FullOpeningFieldEvidenceCertificate -> Prop
  fullOpeningFrameValid :
    ReviewedFq12FullOpeningFieldEvidenceCertificate -> Prop
  representativeFrameValid :
    ReviewedFq12FullOpeningFieldEvidenceCertificate -> Prop
  domainSeparated :
    Nat -> ReviewedFq12FullOpeningFieldEvidenceCertificate -> Prop

/--
Byte-level certificate for a parsed field-evidence frame.

This is intentionally only a byte/frame certificate.  Semantic agreement with
the payload, extraction, role-lift, assembly, and compatibility certificates is
carried by `ReviewedFq12FullOpeningFieldEvidenceBinds` below.
-/
structure ReviewedFq12FullOpeningFieldEvidenceByteCertificate
    (Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (frameBytes : Nat)
    (fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate) :
    Prop where
  canonical :
    Parser.frameBytesCanonical frameBytes
  parsed_eq :
    Parser.parse frameBytes = fieldCert
  frame_digest_eq :
    Parser.frameDigest fieldCert = fieldCert.fieldEvidenceDigest
  aggregate_bridge_unused :
    fieldCert.aggregateBridgeUsedAsSource = false
  role_root_frame_valid :
    Parser.roleRootFrameValid fieldCert
  full_opening_frame_valid :
    Parser.fullOpeningFrameValid fieldCert
  representative_frame_valid :
    Parser.representativeFrameValid fieldCert
  domain_separated :
    Parser.domainSeparated frameBytes fieldCert

/-- The byte certificate includes the pre-aggregate ordering guard. -/
theorem byte_certificate_aggregate_bridge_unused
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {frameBytes : Nat}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (C :
      ReviewedFq12FullOpeningFieldEvidenceByteCertificate
        Parser frameBytes fieldCert) :
    fieldCert.aggregateBridgeUsedAsSource = false :=
  C.aggregate_bridge_unused

/-- The byte certificate binds the parsed frame digest to the carried digest. -/
theorem byte_certificate_frame_digest_eq
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {frameBytes : Nat}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (C :
      ReviewedFq12FullOpeningFieldEvidenceByteCertificate
        Parser frameBytes fieldCert) :
    Parser.frameDigest fieldCert = fieldCert.fieldEvidenceDigest :=
  C.frame_digest_eq

/--
A byte-backed field-evidence object plus the semantic binding predicate.

This is the direct Lean mirror of the Rust V1 object after parsing and frame
checks have succeeded.
-/
structure ByteBackedFullOpeningFieldEvidence
    (Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (frameBytes : Nat)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate)
    (fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate) :
    Prop where
  byte_certificate :
    ReviewedFq12FullOpeningFieldEvidenceByteCertificate
      Parser frameBytes fieldCert
  binds :
    ReviewedFq12FullOpeningFieldEvidenceBinds
      payload cert liftCert assemblyCert compatCert fieldCert

/--
A byte-backed field-evidence object yields exactly the semantic bindings used
by the active P2FS-001 reductions.
-/
theorem semantic_bindings_of_byte_backed_field_evidence
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {frameBytes : Nat}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (C :
      ByteBackedFullOpeningFieldEvidence
        Parser frameBytes payload cert liftCert assemblyCert compatCert
        fieldCert) :
    ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
      payload cert liftCert assemblyCert compatCert :=
  semantic_bindings_of_full_opening_field_evidence_binds C.binds

/--
Byte-backed field evidence through the sidecar predicate bridge.

The sidecar bridge keeps the global `Predicates` record unchanged while still
allowing the concrete parser/frame certificate to be threaded into active-path
theorems.
-/
structure ByteBackedPredicateFieldEvidence
    (Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (P : Predicates)
    (Bridge : ReviewedFq12FullOpeningFieldEvidencePredicateBridge P)
    (b : Boundary)
    (frameBytes : Nat)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate)
    (fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate) :
    Prop where
  byte_certificate :
    ReviewedFq12FullOpeningFieldEvidenceByteCertificate
      Parser frameBytes fieldCert
  has_field_evidence :
    Bridge.hasFieldEvidence
      b payload cert liftCert assemblyCert compatCert fieldCert
  field_evidence_valid :
    Bridge.fieldEvidenceValid
      b payload cert liftCert assemblyCert compatCert fieldCert

/--
The byte-backed predicate bridge recovers the same semantic bindings as the
direct field-evidence binding predicate.
-/
theorem semantic_bindings_of_byte_backed_predicate_field_evidence
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    {Bridge : ReviewedFq12FullOpeningFieldEvidencePredicateBridge P}
    {b : Boundary}
    {frameBytes : Nat}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (C :
      ByteBackedPredicateFieldEvidence
        Parser P Bridge b frameBytes payload cert liftCert assemblyCert
        compatCert fieldCert) :
    ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
      payload cert liftCert assemblyCert compatCert :=
  semantic_bindings_of_valid_predicate_field_evidence
    P Bridge C.has_field_evidence C.field_evidence_valid

/--
Named residual exits for byte-level field-evidence instantiation.

These are intentionally restricted to data availability, field/domain
canonicality, payload binding, and hidden-source soundness.  The aggregate
bridge is not a residual source for this pre-aggregate object.
-/
def FieldEvidenceByteBridgeBreakAlternatives
    (P : Predicates) (b : Boundary) : Prop :=
  P.boundaryBreakFor b.context b BreakEvent.prevalidationDataAvailabilityBreak ∨
    P.boundaryBreakFor b.context b BreakEvent.reviewedFq12DomainSeparationBreak ∨
      P.boundaryBreakFor b.context b BreakEvent.reviewedFq12PayloadBindingBreak ∨
        P.boundaryBreakFor b.context b
          BreakEvent.reviewedFq12HiddenSourceSoundnessBreak

/-- A named byte-bridge residual is also a generic boundary break. -/
theorem exists_boundary_break_of_field_evidence_byte_break
    {P : Predicates} {b : Boundary}
    (hBreak : FieldEvidenceByteBridgeBreakAlternatives P b) :
    exists breakEvent, P.boundaryBreakFor b.context b breakEvent := by
  rcases hBreak with hData | hRest
  · exact ⟨BreakEvent.prevalidationDataAvailabilityBreak, hData⟩
  rcases hRest with hDomain | hRest
  · exact ⟨BreakEvent.reviewedFq12DomainSeparationBreak, hDomain⟩
  rcases hRest with hPayload | hHidden
  · exact ⟨BreakEvent.reviewedFq12PayloadBindingBreak, hPayload⟩
  · exact ⟨BreakEvent.reviewedFq12HiddenSourceSoundnessBreak, hHidden⟩

/--
Byte-level source facts sufficient to instantiate the sidecar implementation
predicate bridge.

The source theorem is what the Rust parser/frame implementation must supply:
from the already-accepted payload/extraction/lift/assembly/compatibility chain,
it either returns a concrete byte frame plus binding certificate, or one of the
named byte-bridge residual breaks.
-/
structure ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource
    (Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (P : Predicates) where
  hasByteFieldEvidence :
    Boundary ->
      ReviewedFq12HiddenSourcePayload ->
        ReviewedFq12HiddenRelationExtractionCertificate ->
          ReviewedFq12FullNativeOpeningCertificate ->
            ReviewedFq12NativeRepresentativeAssemblyCertificate ->
              ReviewedFq12ExtractionCompatibilityCertificate ->
                Nat ->
                  ReviewedFq12FullOpeningFieldEvidenceCertificate -> Prop
  byteFieldEvidenceAvailable :
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
            P.reviewedFq12PayloadHasRoleLiftCertificate
                b payload cert liftCert ->
              P.reviewedFq12RoleLiftCertificateValid
                  b payload cert liftCert ->
                P.reviewedFq12OpeningHasAssemblyCertificate
                    b payload cert liftCert.opening assemblyCert ->
                  P.reviewedFq12AssemblyCertificateValid
                      b payload cert liftCert.opening assemblyCert ->
                    P.reviewedFq12HasExtractionCompatibilityCertificate
                        b payload cert liftCert.opening
                        (ReviewedFq12AssemblyConstructedRepresentative
                          assemblyCert)
                        compatCert ->
                      P.reviewedFq12ExtractionCompatibilityCertificateValid
                          b payload cert liftCert.opening
                          (ReviewedFq12AssemblyConstructedRepresentative
                            assemblyCert)
                          compatCert ->
                        (exists frameBytes fieldCert,
                          hasByteFieldEvidence
                              b payload cert liftCert assemblyCert compatCert
                              frameBytes fieldCert ∧
                            ReviewedFq12FullOpeningFieldEvidenceByteCertificate
                              Parser frameBytes fieldCert ∧
                              ReviewedFq12FullOpeningFieldEvidenceBinds
                                payload cert liftCert assemblyCert compatCert
                                fieldCert) ∨
                        FieldEvidenceByteBridgeBreakAlternatives P b

/--
Convert byte-level implementation evidence into the existing sidecar predicate
bridge.
-/
def byte_predicate_source_to_sidecar_bridge
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P) :
    ReviewedFq12FullOpeningFieldEvidencePredicateBridge P :=
  { hasFieldEvidence := fun b payload cert liftCert assemblyCert compatCert
        fieldCert =>
      exists frameBytes,
        Source.hasByteFieldEvidence
          b payload cert liftCert assemblyCert compatCert frameBytes
          fieldCert ∧
          ReviewedFq12FullOpeningFieldEvidenceByteCertificate
            Parser frameBytes fieldCert
    fieldEvidenceValid := fun b payload cert liftCert assemblyCert
        compatCert fieldCert =>
      exists frameBytes,
        Source.hasByteFieldEvidence
          b payload cert liftCert assemblyCert compatCert frameBytes
          fieldCert ∧
          ReviewedFq12FullOpeningFieldEvidenceByteCertificate
            Parser frameBytes fieldCert ∧
            ReviewedFq12FullOpeningFieldEvidenceBinds
              payload cert liftCert assemblyCert compatCert fieldCert
    fieldEvidenceAvailable := by
      intro b payload cert liftCert assemblyCert compatCert hAccept
        hProjects hHasExtraction hExtractionValid hHasLift hLiftValid
        hHasAssembly hAssemblyValid hHasCompat hCompatValid
      rcases
          Source.byteFieldEvidenceAvailable hAccept hProjects
            hHasExtraction hExtractionValid hHasLift hLiftValid
            hHasAssembly hAssemblyValid hHasCompat hCompatValid with
        hByte | hBreak
      · rcases hByte with
          ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds⟩
        exact Or.inl
          ⟨fieldCert,
            ⟨frameBytes, hHasByte, hByteCert⟩,
            ⟨frameBytes, hHasByte, hByteCert, hBinds⟩⟩
      · exact Or.inr
          (exists_boundary_break_of_field_evidence_byte_break hBreak)
    fieldEvidenceBinds := by
      intro b payload cert liftCert assemblyCert compatCert fieldCert
        _ hValid
      rcases hValid with
        ⟨_frameBytes, _hHasByte, _hByteCert, hBinds⟩
      exact hBinds }

/--
Direct source theorem: a concrete byte source yields the semantic binding
package consumed by the active reductions, or one of the named byte residuals.
-/
theorem byte_predicate_source_semantic_bindings_or_named_break
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
            ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
              payload cert liftCert assemblyCert compatCert) ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      Source.byteFieldEvidenceAvailable hAccept hProjects hHasExtraction
        hExtractionValid hHasLift hLiftValid hHasAssembly hAssemblyValid
        hHasCompat hCompatValid with
    hByte | hBreak
  · rcases hByte with
      ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds⟩
    exact Or.inl
      ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds,
        semantic_bindings_of_full_opening_field_evidence_binds hBinds⟩
  · exact Or.inr hBreak

/--
Either the byte-backed field-evidence object is present, or an instantiation
must export one of the named residual breaks above.
-/
def ByteBackedFieldEvidenceOrBreak
    (Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (P : Predicates)
    (b : Boundary)
    (frameBytes : Nat)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate)
    (fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate) :
    Prop :=
  ByteBackedFullOpeningFieldEvidence
    Parser frameBytes payload cert liftCert assemblyCert compatCert
    fieldCert ∨
    FieldEvidenceByteBridgeBreakAlternatives P b

/--
Discharge a byte-backed field-evidence object into semantic bindings, or expose
the named byte/parser residual.
-/
theorem byte_backed_field_evidence_semantic_or_named_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    {b : Boundary}
    {frameBytes : Nat}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (h :
      ByteBackedFieldEvidenceOrBreak
        Parser P b frameBytes payload cert liftCert assemblyCert compatCert
        fieldCert) :
    ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
      payload cert liftCert assemblyCert compatCert ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  cases h with
  | inl C =>
      exact Or.inl
        (semantic_bindings_of_byte_backed_field_evidence C)
  | inr hBreak =>
      exact Or.inr hBreak

/--
Concrete parser hooks for the Rust
`Path2ReviewedFq12FullOpeningFieldSourceRouteV1` frame.

This frame is a digest-bound ordering certificate.  It deliberately lives next
to the field-evidence parser, because the source route is only meaningful when
paired with the detailed field-evidence binding object.
-/
structure ReviewedFq12FullOpeningFieldSourceRouteByteParser where
  parse : Nat -> ReviewedFq12FullOpeningFieldSourceRouteCertificate
  frameDigest : ReviewedFq12FullOpeningFieldSourceRouteCertificate -> Nat
  frameBytesCanonical : Nat -> Prop
  domainSeparated :
    Nat -> ReviewedFq12FullOpeningFieldSourceRouteCertificate -> Prop

/--
Byte-level certificate for the parsed source-route frame.

The boolean fields mirror the Rust self-checks:
assembly source present, compatibility source present, adapter-only field
evidence, and no aggregate bridge used as source.
-/
structure ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (routeFrameBytes : Nat)
    (fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate)
    (routeCert : ReviewedFq12FullOpeningFieldSourceRouteCertificate) :
    Prop where
  canonical :
    RouteParser.frameBytesCanonical routeFrameBytes
  parsed_eq :
    RouteParser.parse routeFrameBytes = routeCert
  frame_digest_eq :
    RouteParser.frameDigest routeCert = routeCert.sourceRouteDigest
  field_evidence_digest_eq :
    routeCert.fieldEvidenceDigest = fieldCert.fieldEvidenceDigest
  assembly_source_present :
    routeCert.assemblyIsFullOpeningFieldCertificate = true
  compatibility_source_present :
    routeCert.compatibilityIsSameRepresentativeFieldCertificate = true
  field_evidence_adapter_only :
    routeCert.fieldEvidenceIsAdapterOnly = true
  aggregate_bridge_unused :
    routeCert.aggregateBridgeUsedAsSource = false
  domain_separated :
    RouteParser.domainSeparated routeFrameBytes routeCert

/--
A byte-backed source route paired with the detailed byte-backed field-evidence
object.
-/
structure ByteBackedFullOpeningFieldSourceRoute
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (fieldFrameBytes routeFrameBytes : Nat)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate)
    (fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate)
    (routeCert : ReviewedFq12FullOpeningFieldSourceRouteCertificate) :
    Prop where
  field_byte_certificate :
    ReviewedFq12FullOpeningFieldEvidenceByteCertificate
      FieldParser fieldFrameBytes fieldCert
  source_route_byte_certificate :
    ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
      RouteParser routeFrameBytes fieldCert routeCert
  field_binds :
    ReviewedFq12FullOpeningFieldEvidenceBinds
      payload cert liftCert assemblyCert compatCert fieldCert
  source_route_binds :
    ReviewedFq12FullOpeningFieldSourceRouteBinds
      payload cert liftCert assemblyCert compatCert fieldCert routeCert

/--
The source-route byte object is not a replacement for field-evidence
semantics; paired with detailed field-evidence binding it recovers the same
semantic binding package.
-/
theorem semantic_bindings_of_byte_backed_source_route
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {fieldFrameBytes routeFrameBytes : Nat}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    {routeCert : ReviewedFq12FullOpeningFieldSourceRouteCertificate}
    (C :
      ByteBackedFullOpeningFieldSourceRoute
        FieldParser RouteParser fieldFrameBytes routeFrameBytes payload cert
        liftCert assemblyCert compatCert fieldCert routeCert) :
    ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
      payload cert liftCert assemblyCert compatCert :=
  semantic_bindings_of_full_opening_field_evidence_binds C.field_binds

/--
The paired source-route and field-evidence byte source expected from the Rust
implementation.

The route byte theorem is still compatibility-indexed: it starts from an
already available role-lift, assembly, and compatibility certificate chain and
then returns the two pre-aggregate frames that witness the concrete source
ordering.
-/
structure ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  hasByteFieldSourceRoute :
    Boundary ->
      ReviewedFq12HiddenSourcePayload ->
        ReviewedFq12HiddenRelationExtractionCertificate ->
          ReviewedFq12FullNativeOpeningCertificate ->
            ReviewedFq12NativeRepresentativeAssemblyCertificate ->
              ReviewedFq12ExtractionCompatibilityCertificate ->
                Nat ->
                  Nat ->
                    ReviewedFq12FullOpeningFieldEvidenceCertificate ->
                      ReviewedFq12FullOpeningFieldSourceRouteCertificate ->
                        Prop
  byteFieldSourceRouteAvailable :
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
            P.reviewedFq12PayloadHasRoleLiftCertificate
                b payload cert liftCert ->
              P.reviewedFq12RoleLiftCertificateValid
                  b payload cert liftCert ->
                P.reviewedFq12OpeningHasAssemblyCertificate
                    b payload cert liftCert.opening assemblyCert ->
                  P.reviewedFq12AssemblyCertificateValid
                      b payload cert liftCert.opening assemblyCert ->
                    P.reviewedFq12HasExtractionCompatibilityCertificate
                        b payload cert liftCert.opening
                        (ReviewedFq12AssemblyConstructedRepresentative
                          assemblyCert)
                        compatCert ->
                      P.reviewedFq12ExtractionCompatibilityCertificateValid
                          b payload cert liftCert.opening
                          (ReviewedFq12AssemblyConstructedRepresentative
                            assemblyCert)
                          compatCert ->
                        (exists fieldFrameBytes routeFrameBytes fieldCert
                            routeCert,
                          hasByteFieldSourceRoute
                              b payload cert liftCert assemblyCert compatCert
                              fieldFrameBytes routeFrameBytes fieldCert
                              routeCert ∧
                            ReviewedFq12FullOpeningFieldEvidenceByteCertificate
                              FieldParser fieldFrameBytes fieldCert ∧
                              ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
                                RouteParser routeFrameBytes fieldCert
                                routeCert ∧
                              ReviewedFq12FullOpeningFieldEvidenceBinds
                                payload cert liftCert assemblyCert compatCert
                                fieldCert ∧
                              ReviewedFq12FullOpeningFieldSourceRouteBinds
                                payload cert liftCert assemblyCert compatCert
                                fieldCert routeCert) ∨
                        FieldEvidenceByteBridgeBreakAlternatives P b

/--
Selected Rust source-route byte source for the active reviewed-Fq12 route.

This is the Rust-facing decomposition contract for the paired
`Path2ReviewedFq12FullOpeningFieldEvidenceV1` and
`Path2ReviewedFq12FullOpeningFieldSourceRouteV1` frames.  A future
instantiation must expose the selected Rust source-route relation and prove,
for every accepted route frame, the two byte certificates plus the semantic
field-evidence and source-route binding predicates.  That keeps the source
route as compatibility-indexed ordering evidence and prevents an opaque Rust
accept predicate from satisfying the Lean endpoint by itself.
-/
structure SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  rustSourceRouteFrame :
    Boundary ->
      ReviewedFq12HiddenSourcePayload ->
        ReviewedFq12HiddenRelationExtractionCertificate ->
          ReviewedFq12FullNativeOpeningCertificate ->
            ReviewedFq12NativeRepresentativeAssemblyCertificate ->
              ReviewedFq12ExtractionCompatibilityCertificate ->
                Nat ->
                  Nat ->
                    ReviewedFq12FullOpeningFieldEvidenceCertificate ->
                      ReviewedFq12FullOpeningFieldSourceRouteCertificate ->
                        Prop
  sourceRouteAvailable :
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
            P.reviewedFq12PayloadHasRoleLiftCertificate
                b payload cert liftCert ->
              P.reviewedFq12RoleLiftCertificateValid
                  b payload cert liftCert ->
                P.reviewedFq12OpeningHasAssemblyCertificate
                    b payload cert liftCert.opening assemblyCert ->
                  P.reviewedFq12AssemblyCertificateValid
                      b payload cert liftCert.opening assemblyCert ->
                    P.reviewedFq12HasExtractionCompatibilityCertificate
                        b payload cert liftCert.opening
                        (ReviewedFq12AssemblyConstructedRepresentative
                          assemblyCert)
                        compatCert ->
                      P.reviewedFq12ExtractionCompatibilityCertificateValid
                          b payload cert liftCert.opening
                          (ReviewedFq12AssemblyConstructedRepresentative
                            assemblyCert)
                          compatCert ->
                        (exists fieldFrameBytes routeFrameBytes fieldCert
                            routeCert,
                          rustSourceRouteFrame
                              b payload cert liftCert assemblyCert compatCert
                              fieldFrameBytes routeFrameBytes fieldCert
                              routeCert) ∨
                        FieldEvidenceByteBridgeBreakAlternatives P b
  fieldByteCertificate :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {liftCert : ReviewedFq12FullNativeOpeningCertificate}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
        {fieldFrameBytes routeFrameBytes : Nat}
        {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
        {routeCert : ReviewedFq12FullOpeningFieldSourceRouteCertificate},
      rustSourceRouteFrame
          b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          routeFrameBytes fieldCert routeCert ->
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          FieldParser fieldFrameBytes fieldCert
  routeByteCertificate :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {liftCert : ReviewedFq12FullNativeOpeningCertificate}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
        {fieldFrameBytes routeFrameBytes : Nat}
        {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
        {routeCert : ReviewedFq12FullOpeningFieldSourceRouteCertificate},
      rustSourceRouteFrame
          b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          routeFrameBytes fieldCert routeCert ->
        ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
          RouteParser routeFrameBytes fieldCert routeCert
  fieldBinds :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {liftCert : ReviewedFq12FullNativeOpeningCertificate}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
        {fieldFrameBytes routeFrameBytes : Nat}
        {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
        {routeCert : ReviewedFq12FullOpeningFieldSourceRouteCertificate},
      rustSourceRouteFrame
          b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          routeFrameBytes fieldCert routeCert ->
        ReviewedFq12FullOpeningFieldEvidenceBinds
          payload cert liftCert assemblyCert compatCert fieldCert
  sourceRouteBinds :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {liftCert : ReviewedFq12FullNativeOpeningCertificate}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
        {fieldFrameBytes routeFrameBytes : Nat}
        {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
        {routeCert : ReviewedFq12FullOpeningFieldSourceRouteCertificate},
      rustSourceRouteFrame
          b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          routeFrameBytes fieldCert routeCert ->
        ReviewedFq12FullOpeningFieldSourceRouteBinds
          payload cert liftCert assemblyCert compatCert fieldCert routeCert

/--
Lean mirror of the public selected source-route byte-source evidence report.

This report is the AP2-N001 analogue of the checked-core byte evidence report:
it records that the selected Rust field-evidence/source-route byte path pins the
two parsers, availability theorem, semantic binding facts, aggregate-source
rejection, and negative controls before a proof may expose the byte source to
the five-source hidden-source closure theorem.
-/
structure SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport where
  schemaMatches : Bool
  reportDigestVerified : Bool
  fieldFrameParserPinned : Bool
  routeFrameParserPinned : Bool
  sourceRouteAvailabilityPinned : Bool
  fieldByteCertificatePinned : Bool
  routeByteCertificatePinned : Bool
  fieldBindsPinned : Bool
  sourceRouteBindsPinned : Bool
  aggregateBridgeRejectionPinned : Bool
  sourceRouteNegativeControlsPassed : Bool

namespace SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport

/-- The public source-route byte evidence report has been digest/schema checked. -/
def ReportVerified
    (E : SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport) :
    Prop :=
  E.schemaMatches = true ∧
    E.reportDigestVerified = true

/-- Every selected source-route byte-source component fact recorded by the report holds. -/
def FactsAllTrue
    (E : SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport) :
    Prop :=
  E.fieldFrameParserPinned = true ∧
    E.routeFrameParserPinned = true ∧
    E.sourceRouteAvailabilityPinned = true ∧
    E.fieldByteCertificatePinned = true ∧
    E.routeByteCertificatePinned = true ∧
    E.fieldBindsPinned = true ∧
    E.sourceRouteBindsPinned = true ∧
    E.aggregateBridgeRejectionPinned = true ∧
    E.sourceRouteNegativeControlsPassed = true

end SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport

/--
Proof-bearing selected Rust source-route byte-source certificate.

The final AP2-N001 theorem should construct this certificate from public Rust
evidence and then expose the underlying byte source through `toByteSource`.
That prevents the five-source closure theorem from accepting a bare source
record whose parser, frame, binding, and negative-control evidence has not been
named at the proof boundary.
-/
structure SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  evidenceReport :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport
  evidenceDigest : ByteArray
  evidenceReportVerifiedProof :
    evidenceReport.ReportVerified
  evidenceFactsAllTrueProof :
    evidenceReport.FactsAllTrue
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P

namespace SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate

/-- Named constructor for the selected source-route evidence certificate. -/
def ofReportAndSource
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (evidenceReport :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport)
    (evidenceDigest : ByteArray)
    (evidenceReportVerifiedProof :
      evidenceReport.ReportVerified)
    (evidenceFactsAllTrueProof :
      evidenceReport.FactsAllTrue)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P) :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
      FieldParser RouteParser P where
  evidenceReport := evidenceReport
  evidenceDigest := evidenceDigest
  evidenceReportVerifiedProof := evidenceReportVerifiedProof
  evidenceFactsAllTrueProof := evidenceFactsAllTrueProof
  selectedSource := selectedSource

/-- Expose the selected byte source only after consuming the report proofs. -/
def toByteSource
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Cert :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate
        FieldParser RouteParser P) :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P :=
  let _hReport : Cert.evidenceReport.ReportVerified :=
    Cert.evidenceReportVerifiedProof
  let _hFacts : Cert.evidenceReport.FactsAllTrue :=
    Cert.evidenceFactsAllTrueProof
  Cert.selectedSource

end SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate

/--
Forget the selected-Rust wrapper into the generic source-route byte predicate
source consumed by the AP2-N001 adapters.
-/
def SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource.toBytePredicateSource
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P) :
    ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
      FieldParser RouteParser P :=
  { hasByteFieldSourceRoute := Source.rustSourceRouteFrame
    byteFieldSourceRouteAvailable := by
      intro b payload cert liftCert assemblyCert compatCert hAccept
        hProjects hHasExtraction hExtractionValid hHasLift hLiftValid
        hHasAssembly hAssemblyValid hHasCompat hCompatValid
      rcases
          Source.sourceRouteAvailable hAccept hProjects hHasExtraction
            hExtractionValid hHasLift hLiftValid hHasAssembly
            hAssemblyValid hHasCompat hCompatValid with
        hRoute | hBreak
      · rcases hRoute with
          ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert,
            hFrame⟩
        exact Or.inl
          ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert,
            hFrame, Source.fieldByteCertificate hFrame,
            Source.routeByteCertificate hFrame, Source.fieldBinds hFrame,
            Source.sourceRouteBinds hFrame⟩
      · exact Or.inr hBreak }

/--
A paired source-route byte source is also a plain field-evidence byte source.

The derived `hasByteFieldEvidence` predicate retains the route frame as
additional evidence, so this adapter forgets no ordering check in the proof
object even though callers only consume the field-evidence interface.
-/
def field_evidence_byte_source_of_source_route_byte_source
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P) :
    ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource FieldParser P :=
  { hasByteFieldEvidence :=
      fun b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          fieldCert =>
        exists routeFrameBytes routeCert,
          Source.hasByteFieldSourceRoute
              b payload cert liftCert assemblyCert compatCert
              fieldFrameBytes routeFrameBytes fieldCert routeCert ∧
            ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
              RouteParser routeFrameBytes fieldCert routeCert ∧
              ReviewedFq12FullOpeningFieldSourceRouteBinds
                payload cert liftCert assemblyCert compatCert fieldCert
                routeCert
    byteFieldEvidenceAvailable := by
      intro b payload cert liftCert assemblyCert compatCert hAccept
        hProjects hHasExtraction hExtractionValid hHasLift hLiftValid
        hHasAssembly hAssemblyValid hHasCompat hCompatValid
      rcases
          Source.byteFieldSourceRouteAvailable hAccept hProjects
            hHasExtraction hExtractionValid hHasLift hLiftValid
            hHasAssembly hAssemblyValid hHasCompat hCompatValid with
        hRoute | hBreak
      · rcases hRoute with
          ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert,
            hHasRoute, hFieldByte, hRouteByte, hFieldBinds,
            hRouteBinds⟩
        exact Or.inl
          ⟨fieldFrameBytes, fieldCert,
            ⟨routeFrameBytes, routeCert, hHasRoute, hRouteByte,
              hRouteBinds⟩,
            hFieldByte, hFieldBinds⟩
      · exact Or.inr hBreak }

/--
Direct source-route theorem: the Rust paired source-route/field-evidence
source yields semantic bindings or a named byte/parser residual.
-/
theorem source_route_byte_predicate_source_semantic_bindings_or_named_break
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
              ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
                payload cert liftCert assemblyCert compatCert) ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      Source.byteFieldSourceRouteAvailable hAccept hProjects
        hHasExtraction hExtractionValid hHasLift hLiftValid hHasAssembly
        hAssemblyValid hHasCompat hCompatValid with
    hByte | hBreak
  · rcases hByte with
      ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
        hFieldByte, hRouteByte, hFieldBinds, hRouteBinds⟩
    exact Or.inl
      ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
        hFieldByte, hRouteByte, hFieldBinds, hRouteBinds,
        semantic_bindings_of_full_opening_field_evidence_binds hFieldBinds⟩
  · exact Or.inr hBreak

end Path2FullOpeningFieldEvidenceByteBridge
end Descender
