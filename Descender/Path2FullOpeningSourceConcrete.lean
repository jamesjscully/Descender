import Descender.Path2FullOpeningSourceBridge
import Descender.Path2RoleRootBinding
import Descender.Path2AssemblyConstruction
import Descender.Path2AssemblyNativeValidity
import Descender.Path2CompatibilityConcrete

namespace Descender

/-!
# Path 2 Full-Opening Concrete Source Package

This module packages the non-aggregate source obligations for the active
reviewed-Fq12 full-opening route.  It is intentionally one layer above the
source bridge:

* assembly availability and binding must be generic, because assembly is what
  produces the representative used by compatibility;
* deterministic assembly construction and native validity are supplied for the
  representative constructed from the assembly certificate; and
* compatibility soundness is reduced to concrete same-representative extraction
  rules.

No aggregate full-opening bridge certificate is used as a source premise here.
-/

namespace Path2FullOpeningSourceConcrete

open Path2NativeBoundary
open Path2FullOpeningSourceBridge
open Path2CompatibilityConcrete
open Path2CompatibilitySoundness

/--
Concrete non-aggregate source for native representative assembly.

The `constructs` and `nativeValidity` fields are deliberately stated over the
deterministic representative constructed from the assembly certificate.  The
generic soundness surfaces are recovered by
`Path2AssemblyConstruction` and `Path2AssemblyNativeValidity`.
-/
structure ReviewedFq12ConcreteAssemblySource
    (P : Predicates) where
  availability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  binding :
    ReviewedFq12AssemblyCertificateBindingOrBreak P
  constructs :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativePredicates
                        P b payload cert opening assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)
  nativeValidity :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
                        P b assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)

/--
Concrete non-aggregate source for same-representative compatibility.

The availability and binding checks stay at the source-bridge surface.  The
soundness theorem is instantiated from concrete extraction-predicate rules, so
the compatibility proof does not appeal to an aggregate bridge object.
-/
structure ReviewedFq12ConcreteCompatibilitySource
    (P : Predicates) where
  availability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  binding :
    ReviewedFq12ExtractionCompatibilityCertificateBindingOrBreak P
  extractionRules :
    SameRepresentativeConcreteExtractionPredicates P

/--
Named availability certificate for assembly.

The witness `assemblyCert` is kept outside the structure so the structure
remains a compact Prop-valued proof object for the verifier/model to provide.
-/
structure ReviewedFq12AvailableAssemblyCertificate
    (P : Predicates)
    (b : Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    Prop where
  hasAssembly :
    P.reviewedFq12OpeningHasAssemblyCertificate
      b payload cert opening assemblyCert
  assemblyValid :
    P.reviewedFq12AssemblyCertificateValid
      b payload cert opening assemblyCert

/--
Named availability source for assembly certificates.
-/
def ReviewedFq12AssemblyAvailabilityOrBreak
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              (exists assemblyCert,
                ReviewedFq12AvailableAssemblyCertificate
                  P b payload cert opening assemblyCert) ∨
              (exists breakEvent, P.boundaryBreakFor b.context b breakEvent)

/--
Named assembly availability is exactly the availability surface consumed by
the source bridge.
-/
theorem assembly_availability_of_named_source
    (P : Predicates)
    (hAvailability : ReviewedFq12AssemblyAvailabilityOrBreak P) :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P := by
  intro b payload cert opening hAccept hProjects hHas hValid hLifts
  rcases
      hAvailability hAccept hProjects hHas hValid hLifts with
    hAssembly | hBreak
  · rcases hAssembly with ⟨assemblyCert, hAvailable⟩
    exact Or.inl
      ⟨assemblyCert, hAvailable.hasAssembly,
        hAvailable.assemblyValid⟩
  · exact Or.inr hBreak

/--
The base active-boundary assembly availability surface can be repackaged as
the named source used by the deployable P2FS-001 checklist.
-/
theorem named_assembly_availability_of_source
    (P : Predicates)
    (hAvailability :
      ReviewedFq12RepresentativeAssemblyCertificateAvailability P) :
    ReviewedFq12AssemblyAvailabilityOrBreak P := by
  intro b payload cert opening hAccept hProjects hHas hValid hLifts
  rcases
      hAvailability hAccept hProjects hHas hValid hLifts with
    hAssembly | hBreak
  · rcases hAssembly with
      ⟨assemblyCert, hHasAssembly, hAssemblyValid⟩
    exact Or.inl
      ⟨assemblyCert,
        { hasAssembly := hHasAssembly
          assemblyValid := hAssemblyValid }⟩
  · exact Or.inr hBreak

/--
Named availability certificate for compatibility.

The witness `compatCert` is kept outside the structure so the proof object
only records the has/valid predicates for that witness.
-/
structure ReviewedFq12AvailableCompatibilityCertificate
    (P : Predicates)
    (b : Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  hasCompatibility :
    P.reviewedFq12HasExtractionCompatibilityCertificate
      b payload cert opening h compatCert
  compatibilityValid :
    P.reviewedFq12ExtractionCompatibilityCertificateValid
      b payload cert opening h compatCert

/--
Named availability source for compatibility certificates.
-/
def ReviewedFq12CompatibilityAvailabilityOrBreak
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              FullNativeOpeningMatchesRepresentative opening h ->
                P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening h ->
                  InFiber P b.context b h ->
                    P.localValid b.context h ->
                      P.descentConsistent b.context h ->
                        (exists compatCert,
                          ReviewedFq12AvailableCompatibilityCertificate
                            P b payload cert opening h compatCert) ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)

/--
Named compatibility availability is exactly the availability surface consumed
by the source bridge.
-/
theorem compatibility_availability_of_named_source
    (P : Predicates)
    (hAvailability : ReviewedFq12CompatibilityAvailabilityOrBreak P) :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P := by
  intro b payload cert opening h hAccept hProjects hHas hValid hLifts
    hMatches hAssembles hFiber hLocal hDescent
  rcases
      hAvailability hAccept hProjects hHas hValid hLifts hMatches
        hAssembles hFiber hLocal hDescent with
    hCompat | hBreak
  · rcases hCompat with ⟨compatCert, hAvailable⟩
    exact Or.inl
      ⟨compatCert, hAvailable.hasCompatibility,
        hAvailable.compatibilityValid⟩
  · exact Or.inr hBreak

/--
The base active-boundary compatibility availability surface can be repackaged
as the named source used by the deployable P2FS-001 checklist.
-/
theorem named_compatibility_availability_of_source
    (P : Predicates)
    (hAvailability :
      ReviewedFq12ExtractionCompatibilityCertificateAvailability P) :
    ReviewedFq12CompatibilityAvailabilityOrBreak P := by
  intro b payload cert opening h hAccept hProjects hHas hValid hLifts
    hMatches hAssembles hFiber hLocal hDescent
  rcases
      hAvailability hAccept hProjects hHas hValid hLifts hMatches
        hAssembles hFiber hLocal hDescent with
    hCompat | hBreak
  · rcases hCompat with
      ⟨compatCert, hHasCompat, hCompatValid⟩
    exact Or.inl
      ⟨compatCert,
        { hasCompatibility := hHasCompat
          compatibilityValid := hCompatValid }⟩
  · exact Or.inr hBreak

/--
Named local predicate certificate for deterministic assembly.

This is propositionally equivalent to
`ReviewedFq12AssemblyConstructedRepresentativePredicates`, but exposes the two
implementation facts as named fields for verifier/model instantiation.
-/
structure ReviewedFq12AssemblyConstructedRepresentativeLocalCertificate
    (P : Predicates)
    (b : Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    Prop where
  assemblyCertificateAssembles :
    P.reviewedFq12AssemblyCertificateAssembles
      b payload cert opening assemblyCert
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
  fullOpeningAssembles :
    P.fullNativeOpeningAssemblesRepresentative
      b payload cert opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)

/-- Convert named deterministic assembly local facts to the existing pair. -/
theorem assembly_constructed_predicates_of_local_certificate
    {P : Predicates}
    {b : Boundary}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    (hLocal :
      ReviewedFq12AssemblyConstructedRepresentativeLocalCertificate
        P b payload cert opening assemblyCert) :
    ReviewedFq12AssemblyConstructedRepresentativePredicates
      P b payload cert opening assemblyCert :=
  ⟨hLocal.assemblyCertificateAssembles,
    hLocal.fullOpeningAssembles⟩

/-- Convert the existing pair to named deterministic assembly local facts. -/
theorem local_certificate_of_assembly_constructed_predicates
    {P : Predicates}
    {b : Boundary}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    (hPredicates :
      ReviewedFq12AssemblyConstructedRepresentativePredicates
        P b payload cert opening assemblyCert) :
    ReviewedFq12AssemblyConstructedRepresentativeLocalCertificate
      P b payload cert opening assemblyCert :=
  { assemblyCertificateAssembles := hPredicates.1
    fullOpeningAssembles := hPredicates.2 }

/--
Generic local deterministic assembly predicate source.

This is the non-compatibility-indexed fact still required after an assembly
certificate has been field-bound to an opening.
-/
def ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativeLocalCertificate
                        P b payload cert opening assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)

/--
Generic native-validity source for the deterministic assembly representative.

This gives a shorter name to the remaining native-fiber/local/descent
obligation.
-/
def ReviewedFq12AssemblyConstructedNativeValidityOrBreak
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
                        P b assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)

/--
Named local predicate source discharges the construction predicate pair used
by assembly construction soundness.
-/
theorem assembly_constructs_of_local_predicate_source
    (P : Predicates)
    (hLocal : ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak P) :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativePredicates
                        P b payload cert opening assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent) := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas hValid
    hLifts hHasAssembly hAssemblyValid hBinds
  rcases
      hLocal hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid hBinds with
    hLocalCert | hBreak
  · exact Or.inl
      (assembly_constructed_predicates_of_local_certificate hLocalCert)
  · exact Or.inr hBreak

/--
Concrete assembly field certificate.

This is the field-equality payload of
`ReviewedFq12AssemblyCertificateBindsOpening`, exposed as a named structure so
the implementation can target a small verifier certificate instead of a large
anonymous conjunction.
-/
structure ReviewedFq12AssemblyConcreteFieldCertificate
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    Prop where
  payloadBindingDigest_eq :
    assemblyCert.payloadBindingDigest = cert.payloadBindingDigest
  extractionCertificateDigest_eq :
    assemblyCert.extractionCertificateDigest =
      cert.extractionCertificateDigest
  creationProofBindingDigest_eq :
    assemblyCert.creationProofBindingDigest =
      payload.creationProofBindingDigest
  nativeBoundaryDigest_payload_eq :
    assemblyCert.nativeBoundaryDigest = payload.nativeBoundaryDigest
  nativeBoundaryDigest_cert_eq :
    assemblyCert.nativeBoundaryDigest = cert.nativeBoundaryDigest
  slotId_payload_eq :
    assemblyCert.slotId = payload.slotId
  slotId_cert_eq :
    assemblyCert.slotId = cert.slotId
  creationStatementDigest_payload_eq :
    assemblyCert.creationStatementDigest =
      payload.creationStatementDigest
  creationStatementDigest_cert_eq :
    assemblyCert.creationStatementDigest =
      cert.creationStatementDigest
  reviewedStatementDigest_payload_eq :
    assemblyCert.reviewedStatementDigest = payload.statementDigest
  reviewedStatementDigest_cert_eq :
    assemblyCert.reviewedStatementDigest = cert.reviewedStatementDigest
  nativeRoleRootCommitment_eq :
    assemblyCert.nativeRoleRootCommitment =
      opening.nativeRoleRootCommitment
  fullOpeningDigest_eq :
    assemblyCert.fullOpeningDigest = opening.openingDigest
  localSectionDigest_eq :
    assemblyCert.localSectionDigest = opening.localSectionDigest
  packedResiduesDigest_eq :
    assemblyCert.packedResiduesDigest = opening.packedResiduesDigest
  randomizerDigest_eq :
    assemblyCert.randomizerDigest = opening.randomizerDigest
  selectedAccountProjectionDigest_eq :
    assemblyCert.selectedAccountProjectionDigest =
      opening.selectedAccountProjectionDigest
  padSourceDigest_eq :
    assemblyCert.padSourceDigest = opening.padSourceDigest
  openingTupleDigest_eq :
    assemblyCert.openingTupleDigest = opening.openingTupleDigest
  projectionRelationDigest_eq :
    assemblyCert.projectionRelationDigest =
      opening.projectionRelationDigest
  lifecycleSlotDigest_eq :
    assemblyCert.lifecycleSlotDigest = opening.lifecycleSlotDigest
  traceWitnessDigest_eq :
    assemblyCert.traceWitnessDigest = opening.traceWitnessDigest
  gaugeDigest_eq :
    assemblyCert.gaugeDigest = opening.gaugeDigest
  opening_eq :
    assemblyCert.opening = opening

/-- The concrete assembly field certificate rebuilds the binding predicate. -/
theorem binding_of_assembly_concrete_field_certificate
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    (hFields :
      ReviewedFq12AssemblyConcreteFieldCertificate
        payload cert opening assemblyCert) :
    ReviewedFq12AssemblyCertificateBindsOpening
      payload cert opening assemblyCert :=
  ⟨hFields.payloadBindingDigest_eq,
    hFields.extractionCertificateDigest_eq,
    hFields.creationProofBindingDigest_eq,
    hFields.nativeBoundaryDigest_payload_eq,
    hFields.nativeBoundaryDigest_cert_eq,
    hFields.slotId_payload_eq,
    hFields.slotId_cert_eq,
    hFields.creationStatementDigest_payload_eq,
    hFields.creationStatementDigest_cert_eq,
    hFields.reviewedStatementDigest_payload_eq,
    hFields.reviewedStatementDigest_cert_eq,
    hFields.nativeRoleRootCommitment_eq,
    hFields.fullOpeningDigest_eq,
    hFields.localSectionDigest_eq,
    hFields.packedResiduesDigest_eq,
    hFields.randomizerDigest_eq,
    hFields.selectedAccountProjectionDigest_eq,
    hFields.padSourceDigest_eq,
    hFields.openingTupleDigest_eq,
    hFields.projectionRelationDigest_eq,
    hFields.lifecycleSlotDigest_eq,
    hFields.traceWitnessDigest_eq,
    hFields.gaugeDigest_eq,
    hFields.opening_eq⟩

/--
The binding predicate also exposes the concrete assembly field certificate,
which lets callers move between the named field package and the existing
conjunction without loss.
-/
theorem assembly_concrete_field_certificate_of_binding
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    (hBinds :
      ReviewedFq12AssemblyCertificateBindsOpening
        payload cert opening assemblyCert) :
    ReviewedFq12AssemblyConcreteFieldCertificate
      payload cert opening assemblyCert := by
  rcases hBinds with
    ⟨hPayloadBinding, hExtraction, hCreationProof, hNativePayload,
      hNativeCert, hSlotPayload, hSlotCert, hCreationPayload,
      hCreationCert, hReviewedPayload, hReviewedCert, hRoleRoot,
      hFullOpening, hLocalSection, hPackedResidues, hRandomizer,
      hSelectedAccountProjection, hPadSource, hOpeningTuple,
      hProjectionRelation, hLifecycleSlot, hTraceWitness, hGauge,
      hOpeningEq⟩
  exact
    { payloadBindingDigest_eq := hPayloadBinding
      extractionCertificateDigest_eq := hExtraction
      creationProofBindingDigest_eq := hCreationProof
      nativeBoundaryDigest_payload_eq := hNativePayload
      nativeBoundaryDigest_cert_eq := hNativeCert
      slotId_payload_eq := hSlotPayload
      slotId_cert_eq := hSlotCert
      creationStatementDigest_payload_eq := hCreationPayload
      creationStatementDigest_cert_eq := hCreationCert
      reviewedStatementDigest_payload_eq := hReviewedPayload
      reviewedStatementDigest_cert_eq := hReviewedCert
      nativeRoleRootCommitment_eq := hRoleRoot
      fullOpeningDigest_eq := hFullOpening
      localSectionDigest_eq := hLocalSection
      packedResiduesDigest_eq := hPackedResidues
      randomizerDigest_eq := hRandomizer
      selectedAccountProjectionDigest_eq := hSelectedAccountProjection
      padSourceDigest_eq := hPadSource
      openingTupleDigest_eq := hOpeningTuple
      projectionRelationDigest_eq := hProjectionRelation
      lifecycleSlotDigest_eq := hLifecycleSlot
      traceWitnessDigest_eq := hTraceWitness
      gaugeDigest_eq := hGauge
      opening_eq := hOpeningEq }

/--
Strict assembly binding verifier.

This is the Rust-facing shape when a valid assembly certificate is expected to
bind exactly.  The surrounding source bridge lifts this into the
bind-or-named-break interface.
-/
def ReviewedFq12StrictAssemblyCertificateBinding
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                    payload cert opening assemblyCert

/--
Strict assembly field verifier.

This narrower Rust-facing target only needs to expose the concrete field
certificate; the theorem-side binding predicate is rebuilt from those fields.
-/
def ReviewedFq12StrictAssemblyFieldBinding
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyConcreteFieldCertificate
                    payload cert opening assemblyCert

/--
Strict compatibility binding verifier.

This is the Rust-facing shape when a valid compatibility certificate is
expected to bind exactly to the assembled representative.
-/
def ReviewedFq12StrictCompatibilityCertificateBinding
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative}
      {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              FullNativeOpeningMatchesRepresentative opening h ->
                P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening h ->
                  InFiber P b.context b h ->
                    P.localValid b.context h ->
                      P.descentConsistent b.context h ->
                        P.reviewedFq12HasExtractionCompatibilityCertificate
                            b payload cert opening h compatCert ->
                          P.reviewedFq12ExtractionCompatibilityCertificateValid
                              b payload cert opening h compatCert ->
                            ReviewedFq12ExtractionCompatibilityCertificateBinds
                              payload cert opening h compatCert

/--
Strict compatibility field verifier.

This is narrower than the full binding predicate: it only has to expose the
same-representative concrete field certificate from
`Path2CompatibilityConcrete`.
-/
def ReviewedFq12StrictCompatibilityFieldBinding
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative}
      {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              FullNativeOpeningMatchesRepresentative opening h ->
                P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening h ->
                  InFiber P b.context b h ->
                    P.localValid b.context h ->
                      P.descentConsistent b.context h ->
                        P.reviewedFq12HasExtractionCompatibilityCertificate
                            b payload cert opening h compatCert ->
                          P.reviewedFq12ExtractionCompatibilityCertificateValid
                              b payload cert opening h compatCert ->
                            SameRepresentativeConcreteFieldCertificate
                              payload cert opening h compatCert

/--
A strict assembly binding verifier is a valid binding-or-break source with no
break on the successful verifier path.
-/
theorem assembly_binding_or_break_of_strict_binding
    (P : Predicates)
    (hStrict : ReviewedFq12StrictAssemblyCertificateBinding P) :
    ReviewedFq12AssemblyCertificateBindingOrBreak P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas
    hValid hLifts hHasAssembly hAssemblyValid
  exact Or.inl
    (hStrict hAccept hProjects hHas hValid hLifts hHasAssembly
      hAssemblyValid)

/--
Concrete assembly field certificates imply strict assembly binding.
-/
theorem strict_assembly_binding_of_field_binding
    (P : Predicates)
    (hFields : ReviewedFq12StrictAssemblyFieldBinding P) :
    ReviewedFq12StrictAssemblyCertificateBinding P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas
    hValid hLifts hHasAssembly hAssemblyValid
  exact
    binding_of_assembly_concrete_field_certificate
      (hFields hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid)

/--
Concrete field-level assembly binding source.

This is weaker than a strict theorem-side assembly binding: the implementation
only has to supply the concrete assembly field certificate, or export a named
break.  The theorem-side binding predicate is rebuilt from those fields.
-/
def ReviewedFq12ConcreteAssemblyFieldBindingOrBreak
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyConcreteFieldCertificate
                      payload cert opening assemblyCert ∨
                    (exists breakEvent,
                      P.boundaryBreakFor b.context b breakEvent)

/--
Concrete assembly fields instantiate the source-bridge assembly
binding-or-break obligation.
-/
theorem assembly_binding_or_break_of_concrete_field_binding
    (P : Predicates)
    (hFields : ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P) :
    ReviewedFq12AssemblyCertificateBindingOrBreak P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas
    hValid hLifts hHasAssembly hAssemblyValid
  rcases
      hFields hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid with
    hConcrete | hBreak
  · exact Or.inl
      (binding_of_assembly_concrete_field_certificate hConcrete)
  · exact Or.inr hBreak

/--
A strict compatibility binding verifier is a valid binding-or-break source
with no break on the successful verifier path.
-/
theorem compatibility_binding_or_break_of_strict_binding
    (P : Predicates)
    (hStrict : ReviewedFq12StrictCompatibilityCertificateBinding P) :
    ReviewedFq12ExtractionCompatibilityCertificateBindingOrBreak P := by
  intro b payload cert opening h compatCert hAccept hProjects hHas hValid
    hLifts hMatches hAssembles hFiber hLocal hDescent hHasCompat
    hCompatValid
  exact Or.inl
    (hStrict hAccept hProjects hHas hValid hLifts hMatches hAssembles
      hFiber hLocal hDescent hHasCompat hCompatValid)

/--
Concrete same-representative field certificates imply the strict compatibility
binding predicate.
-/
theorem strict_compatibility_binding_of_field_binding
    (P : Predicates)
    (hFields : ReviewedFq12StrictCompatibilityFieldBinding P) :
    ReviewedFq12StrictCompatibilityCertificateBinding P := by
  intro b payload cert opening h compatCert hAccept hProjects hHas hValid
    hLifts hMatches hAssembles hFiber hLocal hDescent hHasCompat
    hCompatValid
  exact
    binding_of_same_representative_concrete_field_certificate
      (hFields hAccept hProjects hHas hValid hLifts hMatches hAssembles
        hFiber hLocal hDescent hHasCompat hCompatValid)

/--
Concrete field-level compatibility binding source.

This is weaker than a strict theorem-side compatibility binding: the
implementation only has to supply the same-representative concrete field
certificate, or export a named break.  The theorem-side binding predicate is
rebuilt from those fields by `Path2CompatibilityConcrete`.
-/
def ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative}
      {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              FullNativeOpeningMatchesRepresentative opening h ->
                P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening h ->
                  InFiber P b.context b h ->
                    P.localValid b.context h ->
                      P.descentConsistent b.context h ->
                        P.reviewedFq12HasExtractionCompatibilityCertificate
                            b payload cert opening h compatCert ->
                          P.reviewedFq12ExtractionCompatibilityCertificateValid
                              b payload cert opening h compatCert ->
                            SameRepresentativeConcreteFieldCertificate
                                payload cert opening h compatCert ∨
                              (exists breakEvent,
                                P.boundaryBreakFor b.context b breakEvent)

/--
Concrete same-representative fields instantiate the source-bridge
compatibility binding-or-break obligation.
-/
theorem compatibility_binding_or_break_of_concrete_field_binding
    (P : Predicates)
    (hFields :
      ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak P) :
    ReviewedFq12ExtractionCompatibilityCertificateBindingOrBreak P := by
  intro b payload cert opening h compatCert hAccept hProjects hHas hValid
    hLifts hMatches hAssembles hFiber hLocal hDescent hHasCompat
    hCompatValid
  rcases
      hFields hAccept hProjects hHas hValid hLifts hMatches hAssembles
        hFiber hLocal hDescent hHasCompat hCompatValid with
    hConcrete | hBreak
  · exact Or.inl
      (binding_of_same_representative_concrete_field_certificate hConcrete)
  · exact Or.inr hBreak

/--
Strict-source package for assembly.  This is usually the closest match to the
implementation verifier: availability, exact binding, deterministic assembly
predicates, and native validity.
-/
structure ReviewedFq12StrictConcreteAssemblySource
    (P : Predicates) where
  availability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  binding :
    ReviewedFq12StrictAssemblyCertificateBinding P
  constructs :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativePredicates
                        P b payload cert opening assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)
  nativeValidity :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
                        P b assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)

/--
Strict assembly source where binding is supplied as concrete assembly fields.
-/
structure ReviewedFq12StrictConcreteAssemblyFieldSource
    (P : Predicates) where
  availability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  fieldBinding :
    ReviewedFq12StrictAssemblyFieldBinding P
  constructs :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativePredicates
                        P b payload cert opening assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)
  nativeValidity :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
                        P b assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)

/-- Convert the strict assembly package to the general concrete source. -/
def concrete_assembly_source_of_strict
    (P : Predicates)
    (A : ReviewedFq12StrictConcreteAssemblySource P) :
    ReviewedFq12ConcreteAssemblySource P :=
  { availability := A.availability
    binding := assembly_binding_or_break_of_strict_binding P A.binding
    constructs := A.constructs
    nativeValidity := A.nativeValidity }

/-- Convert the field-certificate assembly source to the strict source. -/
def strict_assembly_source_of_field_source
    (P : Predicates)
    (A : ReviewedFq12StrictConcreteAssemblyFieldSource P) :
    ReviewedFq12StrictConcreteAssemblySource P :=
  { availability := A.availability
    binding := strict_assembly_binding_of_field_binding P A.fieldBinding
    constructs := A.constructs
    nativeValidity := A.nativeValidity }

/--
Field-source package for assembly.  This is the field-certificate route:
availability, assembly field binding-or-break, deterministic assembly
predicates, and native validity.
-/
structure ReviewedFq12FieldConcreteAssemblySource
    (P : Predicates) where
  availability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  fieldBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  constructs :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativePredicates
                        P b payload cert opening assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)
  nativeValidity :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ->
                    ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
                        P b assemblyCert ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)

/--
Compact field-level assembly source using the named local/native obligations.
-/
structure ReviewedFq12FieldConcreteAssemblyLocalSource
    (P : Predicates) where
  availability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  fieldBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  localPredicates :
    ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak P
  nativeValidity :
    ReviewedFq12AssemblyConstructedNativeValidityOrBreak P

/--
Compact field-level assembly source using named availability plus named
local/native obligations.
-/
structure ReviewedFq12NamedFieldConcreteAssemblyLocalSource
    (P : Predicates) where
  availability :
    ReviewedFq12AssemblyAvailabilityOrBreak P
  fieldBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  localPredicates :
    ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak P
  nativeValidity :
    ReviewedFq12AssemblyConstructedNativeValidityOrBreak P

/--
Convert the named-availability assembly source to the current compact
field/local source.
-/
def field_concrete_assembly_local_source_of_named
    (P : Predicates)
    (A : ReviewedFq12NamedFieldConcreteAssemblyLocalSource P) :
    ReviewedFq12FieldConcreteAssemblyLocalSource P :=
  { availability :=
      assembly_availability_of_named_source P A.availability
    fieldBinding := A.fieldBinding
    localPredicates := A.localPredicates
    nativeValidity := A.nativeValidity }

/--
Convert the compact field-level assembly local source to the general
field-level assembly source consumed by the deployable theorem.
-/
def field_concrete_assembly_source_of_local_source
    (P : Predicates)
    (A : ReviewedFq12FieldConcreteAssemblyLocalSource P) :
    ReviewedFq12FieldConcreteAssemblySource P :=
  { availability := A.availability
    fieldBinding := A.fieldBinding
    constructs :=
      assembly_constructs_of_local_predicate_source P A.localPredicates
    nativeValidity := A.nativeValidity }

/-- Convert the field-level assembly package to the general source. -/
def concrete_assembly_source_of_field_binding
    (P : Predicates)
    (A : ReviewedFq12FieldConcreteAssemblySource P) :
    ReviewedFq12ConcreteAssemblySource P :=
  { availability := A.availability
    binding :=
      assembly_binding_or_break_of_concrete_field_binding
        P A.fieldBinding
    constructs := A.constructs
    nativeValidity := A.nativeValidity }

/--
Strict-source package for compatibility.  This is the exact-binding verifier
plus the concrete extraction rules.
-/
structure ReviewedFq12StrictConcreteCompatibilitySource
    (P : Predicates) where
  availability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  binding :
    ReviewedFq12StrictCompatibilityCertificateBinding P
  extractionRules :
    SameRepresentativeConcreteExtractionPredicates P

/-- Convert the strict compatibility package to the general concrete source. -/
def concrete_compatibility_source_of_strict
    (P : Predicates)
    (C : ReviewedFq12StrictConcreteCompatibilitySource P) :
    ReviewedFq12ConcreteCompatibilitySource P :=
  { availability := C.availability
    binding := compatibility_binding_or_break_of_strict_binding P C.binding
    extractionRules := C.extractionRules }

/--
Strict compatibility source where binding is supplied as concrete
same-representative fields rather than the assembled full binding predicate.
-/
structure ReviewedFq12StrictConcreteCompatibilityFieldSource
    (P : Predicates) where
  availability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  fieldBinding :
    ReviewedFq12StrictCompatibilityFieldBinding P
  extractionRules :
    SameRepresentativeConcreteExtractionPredicates P

/--
Convert the field-certificate compatibility source to the strict source.
-/
def strict_compatibility_source_of_field_source
    (P : Predicates)
    (C : ReviewedFq12StrictConcreteCompatibilityFieldSource P) :
    ReviewedFq12StrictConcreteCompatibilitySource P :=
  { availability := C.availability
    binding := strict_compatibility_binding_of_field_binding P C.fieldBinding
    extractionRules := C.extractionRules }

/--
Field-source package for compatibility.  This is the field-certificate route:
availability, same-representative field binding-or-break, and concrete
extraction rules.
-/
structure ReviewedFq12FieldConcreteCompatibilitySource
    (P : Predicates) where
  availability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  fieldBinding :
    ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak P
  extractionRules :
    SameRepresentativeConcreteExtractionPredicates P

/--
Named compatibility extraction result for one concrete same-representative
field certificate.
-/
structure ReviewedFq12ConcreteCompatibilityExtractionCertificate
    (P : Predicates)
    (b : Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  compatibilityCertificateExtracts :
    P.reviewedFq12ExtractionCompatibilityCertificateExtracts
      b payload cert opening h compatCert
  extractionCertificateExtracts :
    P.reviewedFq12CertificateExtracts b payload cert h
  payloadExtracts :
    P.reviewedFq12PayloadExtracts b payload h

/--
Compatibility extraction source that may export a named break.

This is the natural verifier-facing form: after assembly has already produced
the native-fiber representative and the compatibility certificate has supplied
same-representative concrete fields, extraction either succeeds for all three
predicates or names a boundary break.
-/
def ReviewedFq12ConcreteCompatibilityExtractionOrBreak
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative}
      {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              FullNativeOpeningMatchesRepresentative opening h ->
                P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening h ->
                  InFiber P b.context b h ->
                    P.localValid b.context h ->
                      P.descentConsistent b.context h ->
                        P.reviewedFq12HasExtractionCompatibilityCertificate
                            b payload cert opening h compatCert ->
                          P.reviewedFq12ExtractionCompatibilityCertificateValid
                              b payload cert opening h compatCert ->
                            SameRepresentativeConcreteFieldCertificate
                              payload cert opening h compatCert ->
                              ReviewedFq12ConcreteCompatibilityExtractionCertificate
                                P b payload cert opening h compatCert ∨
                            (exists breakEvent,
                              P.boundaryBreakFor b.context b breakEvent)

/--
An extraction-or-break source discharges compatibility certificate soundness.
-/
theorem compatibility_soundness_of_extraction_source
    (P : Predicates)
    (hExtraction :
      ReviewedFq12ConcreteCompatibilityExtractionOrBreak P) :
    ReviewedFq12ExtractionCompatibilityCertificateSoundness P := by
  intro b payload cert opening h compatCert hAccept hProjects
    hHasExtraction hExtractionValid hLifts hOpeningMatches hAssembles
    hFiber hLocal hDescent hHasCompat hCompatValid hCompatBinds
  have hFields :
      SameRepresentativeConcreteFieldCertificate
        payload cert opening h compatCert :=
    same_representative_concrete_field_certificate_of_binding hCompatBinds
  rcases
      hExtraction hAccept hProjects hHasExtraction hExtractionValid hLifts
        hOpeningMatches hAssembles hFiber hLocal hDescent hHasCompat
        hCompatValid hFields with
    hExtracts | hBreak
  · exact Or.inl
      ⟨hExtracts.compatibilityCertificateExtracts,
        hExtracts.extractionCertificateExtracts,
        hExtracts.payloadExtracts⟩
  · exact Or.inr hBreak

/--
Field-level compatibility source with extraction-or-break semantics.
-/
structure ReviewedFq12FieldConcreteCompatibilityExtractionSource
    (P : Predicates) where
  availability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  fieldBinding :
    ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak P
  extraction :
    ReviewedFq12ConcreteCompatibilityExtractionOrBreak P

/--
Field-level compatibility extraction source using named availability.
-/
structure ReviewedFq12NamedFieldConcreteCompatibilityExtractionSource
    (P : Predicates) where
  availability :
    ReviewedFq12CompatibilityAvailabilityOrBreak P
  fieldBinding :
    ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak P
  extraction :
    ReviewedFq12ConcreteCompatibilityExtractionOrBreak P

/--
Convert the named-availability compatibility extraction source to the current
field-level extraction source.
-/
def field_concrete_compatibility_extraction_source_of_named
    (P : Predicates)
    (C : ReviewedFq12NamedFieldConcreteCompatibilityExtractionSource P) :
    ReviewedFq12FieldConcreteCompatibilityExtractionSource P :=
  { availability :=
      compatibility_availability_of_named_source P C.availability
    fieldBinding := C.fieldBinding
    extraction := C.extraction }

/-- Convert the field-level compatibility package to the general source. -/
def concrete_compatibility_source_of_field_binding
    (P : Predicates)
    (C : ReviewedFq12FieldConcreteCompatibilitySource P) :
    ReviewedFq12ConcreteCompatibilitySource P :=
  { availability := C.availability
    binding :=
      compatibility_binding_or_break_of_concrete_field_binding
        P C.fieldBinding
    extractionRules := C.extractionRules }

/--
The assembly source discharges the construction half of assembly soundness.
-/
theorem assembly_construction_soundness_of_concrete_source
    (P : Predicates)
    (A : ReviewedFq12ConcreteAssemblySource P) :
    ReviewedFq12RepresentativeAssemblyConstructionSoundness P :=
  reviewed_fq12_representative_assembly_construction_soundness_from_constructed_representative
    P A.constructs

/--
The assembly source discharges the native-validity half of assembly soundness.
-/
theorem assembled_native_validity_soundness_of_concrete_source
    (P : Predicates)
    (A : ReviewedFq12ConcreteAssemblySource P) :
    ReviewedFq12AssembledRepresentativeNativeValiditySoundness P :=
  reviewed_fq12_assembled_representative_native_validity_soundness_from_constructed
    P A.nativeValidity

/--
Concrete same-representative extraction rules discharge compatibility
certificate soundness.
-/
theorem compatibility_soundness_of_concrete_source
    (P : Predicates)
    (C : ReviewedFq12ConcreteCompatibilitySource P) :
    ReviewedFq12ExtractionCompatibilityCertificateSoundness P :=
  reviewed_fq12_extraction_compatibility_certificate_soundness P
    (same_representative_field_compatibility_soundness_from_concrete_predicates
      P C.extractionRules)

/--
Concrete non-aggregate sources instantiate the active full-opening source
bridge certificate.
-/
def source_bridge_certificate_of_concrete_sources
    (P : Predicates)
    (roleRootBridge : ReviewedFq12RoleRootBridgeCertificate P)
    (assembly : ReviewedFq12ConcreteAssemblySource P)
    (compatibility : ReviewedFq12ConcreteCompatibilitySource P) :
    ReviewedFq12FullOpeningSourceBridgeCertificate P :=
  { roleRootBridge := roleRootBridge
    assemblyAvailability := assembly.availability
    assemblyBinding := assembly.binding
    assemblyConstructionSoundness :=
      assembly_construction_soundness_of_concrete_source P assembly
    assembledRepresentativeNativeValiditySoundness :=
      assembled_native_validity_soundness_of_concrete_source P assembly
    compatibilityAvailability := compatibility.availability
    compatibilityBinding := compatibility.binding
    compatibilitySoundness :=
      compatibility_soundness_of_concrete_source P compatibility }

/--
End-to-end extraction-certificate soundness from concrete non-aggregate
sources.  This is the theorem-facing deployable route: role-root bridge first,
assembly second, compatibility third, aggregate bridge never as a source.
-/
theorem reviewed_fq12_certificate_soundness_from_concrete_sources
    (P : Predicates)
    (roleRootBridge : ReviewedFq12RoleRootBridgeCertificate P)
    (assembly : ReviewedFq12ConcreteAssemblySource P)
    (compatibility : ReviewedFq12ConcreteCompatibilitySource P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_full_opening_source_bridge P
    (source_bridge_certificate_of_concrete_sources
      P roleRootBridge assembly compatibility)

/--
Concrete sources can also consume the stronger non-aggregate role-root binding
package.  The extraction soundness theorem only needs its `roleRootBridge`
field, while the extra native-role-root pinning facts remain available to the
caller as the deployable full-opening uniqueness obligation.
-/
def source_bridge_certificate_of_nonaggregate_role_root_and_concrete_sources
    (P : Predicates)
    (roleRootBinding :
      ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    (assembly : ReviewedFq12ConcreteAssemblySource P)
    (compatibility : ReviewedFq12ConcreteCompatibilitySource P) :
    ReviewedFq12FullOpeningSourceBridgeCertificate P :=
  source_bridge_certificate_of_concrete_sources
    P roleRootBinding.roleRootBridge assembly compatibility

/--
Source bridge certificate for the field/local assembly route and the
compatibility extraction-or-break route.
-/
def source_bridge_certificate_of_field_local_extraction_sources
    (P : Predicates)
    (roleRootBinding :
      ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    (assembly : ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibility :
      ReviewedFq12FieldConcreteCompatibilityExtractionSource P) :
    ReviewedFq12FullOpeningSourceBridgeCertificate P :=
  let assemblySource :=
    concrete_assembly_source_of_field_binding P
      (field_concrete_assembly_source_of_local_source P assembly)
  { roleRootBridge := roleRootBinding.roleRootBridge
    assemblyAvailability := assemblySource.availability
    assemblyBinding := assemblySource.binding
    assemblyConstructionSoundness :=
      assembly_construction_soundness_of_concrete_source P assemblySource
    assembledRepresentativeNativeValiditySoundness :=
      assembled_native_validity_soundness_of_concrete_source P assemblySource
    compatibilityAvailability := compatibility.availability
    compatibilityBinding :=
      compatibility_binding_or_break_of_concrete_field_binding
        P compatibility.fieldBinding
    compatibilitySoundness :=
      compatibility_soundness_of_extraction_source P
        compatibility.extraction }

/--
End-to-end extraction-certificate soundness from the full non-aggregate
role-root binding package plus concrete assembly and compatibility sources.
-/
theorem reviewed_fq12_certificate_soundness_from_nonaggregate_role_root_and_concrete_sources
    (P : Predicates)
    (roleRootBinding :
      ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    (assembly : ReviewedFq12ConcreteAssemblySource P)
    (compatibility : ReviewedFq12ConcreteCompatibilitySource P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_full_opening_source_bridge P
    (source_bridge_certificate_of_nonaggregate_role_root_and_concrete_sources
      P roleRootBinding assembly compatibility)

/--
Deployable theorem for the common strict-verifier case: valid assembly and
compatibility certificates bind exactly, while construction/native-validity and
extraction rules remain explicit local verifier facts.
-/
theorem reviewed_fq12_certificate_soundness_from_strict_concrete_sources
    (P : Predicates)
    (roleRootBinding :
      ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    (assembly : ReviewedFq12StrictConcreteAssemblySource P)
    (compatibility : ReviewedFq12StrictConcreteCompatibilitySource P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_nonaggregate_role_root_and_concrete_sources
    P roleRootBinding
    (concrete_assembly_source_of_strict P assembly)
    (concrete_compatibility_source_of_strict P compatibility)

/--
Deployable theorem variant where compatibility binding is supplied by concrete
same-representative field certificates.
-/
theorem reviewed_fq12_certificate_soundness_from_strict_concrete_field_sources
    (P : Predicates)
    (roleRootBinding :
      ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    (assembly : ReviewedFq12StrictConcreteAssemblySource P)
    (compatibility :
      ReviewedFq12StrictConcreteCompatibilityFieldSource P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_strict_concrete_sources
    P roleRootBinding assembly
    (strict_compatibility_source_of_field_source P compatibility)

/--
Deployable theorem variant where both assembly and compatibility binding are
supplied by concrete field certificates.
-/
theorem reviewed_fq12_certificate_soundness_from_strict_field_sources
    (P : Predicates)
    (roleRootBinding :
      ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    (assembly : ReviewedFq12StrictConcreteAssemblyFieldSource P)
    (compatibility :
      ReviewedFq12StrictConcreteCompatibilityFieldSource P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_strict_concrete_field_sources
    P roleRootBinding
    (strict_assembly_source_of_field_source P assembly)
    compatibility

/--
Deployable theorem variant where assembly and compatibility binding may be
supplied as concrete field certificates or named breaks.
-/
theorem reviewed_fq12_certificate_soundness_from_field_binding_sources
    (P : Predicates)
    (roleRootBinding :
      ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    (assembly : ReviewedFq12FieldConcreteAssemblySource P)
    (compatibility : ReviewedFq12FieldConcreteCompatibilitySource P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_nonaggregate_role_root_and_concrete_sources
    P roleRootBinding
    (concrete_assembly_source_of_field_binding P assembly)
    (concrete_compatibility_source_of_field_binding P compatibility)

/--
Deployable theorem variant with the remaining assembly obligations named at
their natural granularity: availability, field binding, deterministic local
assembly predicates, and native validity.
-/
theorem reviewed_fq12_certificate_soundness_from_field_local_sources
    (P : Predicates)
    (roleRootBinding :
      ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    (assembly : ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibility : ReviewedFq12FieldConcreteCompatibilitySource P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_field_binding_sources
    P roleRootBinding
    (field_concrete_assembly_source_of_local_source P assembly)
    compatibility

/--
Narrowest source route when compatibility extraction itself may return a named
break.  This matches the source bridge's proof shape most directly.
-/
theorem reviewed_fq12_certificate_soundness_from_field_local_extraction_sources
    (P : Predicates)
    (roleRootBinding :
      ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    (assembly : ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibility :
      ReviewedFq12FieldConcreteCompatibilityExtractionSource P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_full_opening_source_bridge P
    (source_bridge_certificate_of_field_local_extraction_sources
      P roleRootBinding assembly compatibility)

/--
Fully named deployable source route.  This is the narrowest theorem-facing
shape after factoring availability, field binding, local assembly, native
validity, and compatibility extraction into explicit verifier/model sources.
-/
theorem reviewed_fq12_certificate_soundness_from_named_field_local_extraction_sources
    (P : Predicates)
    (roleRootBinding :
      ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    (assembly : ReviewedFq12NamedFieldConcreteAssemblyLocalSource P)
    (compatibility :
      ReviewedFq12NamedFieldConcreteCompatibilityExtractionSource P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_field_local_extraction_sources
    P roleRootBinding
    (field_concrete_assembly_local_source_of_named P assembly)
    (field_concrete_compatibility_extraction_source_of_named
      P compatibility)

/--
Deployable hidden-source input package for the active Path 2 native /
reviewed-Fq12 route.

This record is the non-circular checklist for P2FS-001. It names generic
assembly and compatibility sources separately from the compatibility-indexed
byte field-evidence/source-route adapters:

* role-root binding supplies the full native opening;
* assembly availability, field binding, deterministic local predicates, and
  native validity supply the representative;
* compatibility availability, same-representative field binding, and extraction
  supply the reviewed-Fq12 extraction facts.

The aggregate bridge is not a field of this record and cannot be used as a
source for assembly or compatibility soundness.
-/
structure ReviewedFq12DeployableHiddenSourceInputs
    (P : Predicates) where
  roleRootBinding :
    ReviewedFq12NonAggregateRoleRootBindingCertificate P
  assemblyAvailability :
    ReviewedFq12AssemblyAvailabilityOrBreak P
  assemblyFieldBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  assemblyLocalPredicates :
    ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak P
  assemblyNativeValidity :
    ReviewedFq12AssemblyConstructedNativeValidityOrBreak P
  compatibilityAvailability :
    ReviewedFq12CompatibilityAvailabilityOrBreak P
  compatibilityFieldBinding :
    ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak P
  compatibilityExtraction :
    ReviewedFq12ConcreteCompatibilityExtractionOrBreak P

/-- Extract the named assembly source consumed by the active source theorem. -/
def ReviewedFq12DeployableHiddenSourceInputs.toAssemblySource
    {P : Predicates}
    (I : ReviewedFq12DeployableHiddenSourceInputs P) :
    ReviewedFq12NamedFieldConcreteAssemblyLocalSource P :=
  { availability := I.assemblyAvailability
    fieldBinding := I.assemblyFieldBinding
    localPredicates := I.assemblyLocalPredicates
    nativeValidity := I.assemblyNativeValidity }

/-- Extract the named compatibility source consumed by the active theorem. -/
def ReviewedFq12DeployableHiddenSourceInputs.toCompatibilitySource
    {P : Predicates}
    (I : ReviewedFq12DeployableHiddenSourceInputs P) :
    ReviewedFq12NamedFieldConcreteCompatibilityExtractionSource P :=
  { availability := I.compatibilityAvailability
    fieldBinding := I.compatibilityFieldBinding
    extraction := I.compatibilityExtraction }

/--
Deployable hidden-source closure theorem from the exact non-circular input
package. This is the active target for closing P2FS-001.
-/
theorem reviewed_fq12_certificate_soundness_from_deployable_hidden_source_inputs
    (P : Predicates)
    (I : ReviewedFq12DeployableHiddenSourceInputs P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_named_field_local_extraction_sources
    P I.roleRootBinding I.toAssemblySource I.toCompatibilitySource

end Path2FullOpeningSourceConcrete

end Descender
