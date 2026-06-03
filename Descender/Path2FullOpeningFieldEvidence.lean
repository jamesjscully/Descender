import Descender.Path2AssemblyConstruction
import Descender.Path2CompatibilityConcrete
import Descender.Path2RoleRootConcrete

namespace Descender

/-!
# Path 2 Full-Opening Field Evidence Bridge

This module mirrors the Rust
`Path2ReviewedFq12FullOpeningFieldEvidenceV1` object at the theorem boundary.
It is deliberately pre-aggregate: the aggregate full-opening bridge is a
downstream audit handle, not a source for role-root, assembly, or compatibility
field binding.
-/

namespace Path2FullOpeningFieldEvidence

open Path2NativeBoundary
open Path2CompatibilityConcrete

/--
Lean mirror of the Rust pre-aggregate field evidence frame.

The digest fields are theorem-level handles for the Rust byte frame.  The
binding predicate below is what connects this flat frame to the semantic
payload, extraction, role-lift, assembly, and compatibility certificates.
-/
structure ReviewedFq12FullOpeningFieldEvidenceCertificate where
  payloadBindingDigest : Nat
  extractionCertificateDigest : Nat
  creationProofBindingDigest : Nat
  roleLiftCertificateDigest : Nat
  assemblyCertificateDigest : Nat
  compatibilityCertificateDigest : Nat
  nativeBoundaryDigest : Nat
  slotId : Nat
  creationStatementDigest : Nat
  reviewedStatementDigest : Nat
  expectedNativeRoleRootCommitment : Nat
  nativeRoleRootCommitment : Nat
  localSectionDigest : Nat
  packedResiduesDigest : Nat
  randomizerDigest : Nat
  selectedAccountProjectionDigest : Nat
  padSourceDigest : Nat
  openingTupleDigest : Nat
  projectionRelationDigest : Nat
  lifecycleSlotDigest : Nat
  traceWitnessDigest : Nat
  gaugeDigest : Nat
  fullOpeningDigest : Nat
  representativeDigest : Nat
  aggregateBridgeUsedAsSource : Bool
  fieldEvidenceDigest : Nat

/--
Lean mirror of the Rust
`Path2ReviewedFq12FullOpeningFieldSourceRouteV1` object.

This frame is intentionally flatter than `ReviewedFq12FullOpeningFieldEvidenceCertificate`.
It digest-binds the source ordering itself: assembly is a full-opening field
certificate, compatibility is a same-representative field certificate, and the
field-evidence object is only an adapter.  It still cannot replace the
detailed field-evidence binding predicate below.
-/
structure ReviewedFq12FullOpeningFieldSourceRouteCertificate where
  payloadBindingDigest : Nat
  extractionCertificateDigest : Nat
  creationProofBindingDigest : Nat
  roleLiftCertificateDigest : Nat
  assemblyCertificateDigest : Nat
  compatibilityCertificateDigest : Nat
  fieldEvidenceDigest : Nat
  nativeBoundaryDigest : Nat
  slotId : Nat
  nativeRoleRootCommitment : Nat
  fullOpeningDigest : Nat
  representativeDigest : Nat
  selectedAccountProjectionDigest : Nat
  assemblyIsFullOpeningFieldCertificate : Bool
  compatibilityIsSameRepresentativeFieldCertificate : Bool
  fieldEvidenceIsAdapterOnly : Bool
  aggregateBridgeUsedAsSource : Bool
  sourceRouteDigest : Nat

/--
Field-level binding contract for the Rust pre-aggregate evidence object.

The `aggregateBridgeUnused` field is the proof-ordering guard: this evidence
is allowed to consume payload, extraction, creation-binding, role-lift,
assembly, and compatibility certificates, but not the aggregate bridge.
-/
structure ReviewedFq12FullOpeningFieldEvidenceBinds
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate)
    (fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate) :
    Prop where
  aggregateBridgeUnused :
    fieldCert.aggregateBridgeUsedAsSource = false
  payloadBindingDigest_cert_eq :
    fieldCert.payloadBindingDigest = cert.payloadBindingDigest
  payloadBindingDigest_lift_eq :
    fieldCert.payloadBindingDigest = liftCert.payloadBindingDigest
  payloadBindingDigest_assembly_eq :
    fieldCert.payloadBindingDigest = assemblyCert.payloadBindingDigest
  payloadBindingDigest_compat_eq :
    fieldCert.payloadBindingDigest = compatCert.payloadBindingDigest
  extractionCertificateDigest_cert_eq :
    fieldCert.extractionCertificateDigest =
      cert.extractionCertificateDigest
  extractionCertificateDigest_lift_eq :
    fieldCert.extractionCertificateDigest =
      liftCert.extractionCertificateDigest
  extractionCertificateDigest_assembly_eq :
    fieldCert.extractionCertificateDigest =
      assemblyCert.extractionCertificateDigest
  extractionCertificateDigest_compat_eq :
    fieldCert.extractionCertificateDigest =
      compatCert.extractionCertificateDigest
  creationProofBindingDigest_payload_eq :
    fieldCert.creationProofBindingDigest =
      payload.creationProofBindingDigest
  creationProofBindingDigest_lift_eq :
    fieldCert.creationProofBindingDigest =
      liftCert.creationProofBindingDigest
  creationProofBindingDigest_assembly_eq :
    fieldCert.creationProofBindingDigest =
      assemblyCert.creationProofBindingDigest
  roleLiftCertificateDigest_lift_eq :
    fieldCert.roleLiftCertificateDigest =
      liftCert.roleLiftCertificateDigest
  roleLiftCertificateDigest_assembly_eq :
    fieldCert.roleLiftCertificateDigest =
      assemblyCert.roleLiftCertificateDigest
  roleLiftCertificateDigest_compat_eq :
    fieldCert.roleLiftCertificateDigest =
      compatCert.roleLiftCertificateDigest
  assemblyCertificateDigest_assembly_eq :
    fieldCert.assemblyCertificateDigest =
      assemblyCert.assemblyCertificateDigest
  assemblyCertificateDigest_compat_eq :
    fieldCert.assemblyCertificateDigest =
      compatCert.assemblyCertificateDigest
  compatibilityCertificateDigest_compat_eq :
    fieldCert.compatibilityCertificateDigest =
      compatCert.compatibilityCertificateDigest
  nativeBoundaryDigest_payload_eq :
    fieldCert.nativeBoundaryDigest = payload.nativeBoundaryDigest
  nativeBoundaryDigest_cert_eq :
    fieldCert.nativeBoundaryDigest = cert.nativeBoundaryDigest
  nativeBoundaryDigest_lift_eq :
    fieldCert.nativeBoundaryDigest = liftCert.nativeBoundaryDigest
  nativeBoundaryDigest_assembly_eq :
    fieldCert.nativeBoundaryDigest = assemblyCert.nativeBoundaryDigest
  nativeBoundaryDigest_compat_eq :
    fieldCert.nativeBoundaryDigest = compatCert.nativeBoundaryDigest
  slotId_payload_eq :
    fieldCert.slotId = payload.slotId
  slotId_cert_eq :
    fieldCert.slotId = cert.slotId
  slotId_lift_eq :
    fieldCert.slotId = liftCert.slotId
  slotId_assembly_eq :
    fieldCert.slotId = assemblyCert.slotId
  slotId_compat_eq :
    fieldCert.slotId = compatCert.slotId
  creationStatementDigest_payload_eq :
    fieldCert.creationStatementDigest =
      payload.creationStatementDigest
  creationStatementDigest_cert_eq :
    fieldCert.creationStatementDigest =
      cert.creationStatementDigest
  creationStatementDigest_lift_eq :
    fieldCert.creationStatementDigest =
      liftCert.creationStatementDigest
  creationStatementDigest_assembly_eq :
    fieldCert.creationStatementDigest =
      assemblyCert.creationStatementDigest
  creationStatementDigest_compat_eq :
    fieldCert.creationStatementDigest =
      compatCert.creationStatementDigest
  reviewedStatementDigest_payload_eq :
    fieldCert.reviewedStatementDigest = payload.statementDigest
  reviewedStatementDigest_cert_eq :
    fieldCert.reviewedStatementDigest =
      cert.reviewedStatementDigest
  reviewedStatementDigest_lift_eq :
    fieldCert.reviewedStatementDigest =
      liftCert.reviewedStatementDigest
  reviewedStatementDigest_assembly_eq :
    fieldCert.reviewedStatementDigest =
      assemblyCert.reviewedStatementDigest
  reviewedStatementDigest_compat_eq :
    fieldCert.reviewedStatementDigest =
      compatCert.reviewedStatementDigest
  expectedNativeRoleRoot_eq :
    fieldCert.expectedNativeRoleRootCommitment =
      fieldCert.nativeRoleRootCommitment
  nativeRoleRootCommitment_lift_eq :
    fieldCert.nativeRoleRootCommitment =
      liftCert.nativeRoleRootCommitment
  nativeRoleRootCommitment_liftOpening_eq :
    fieldCert.nativeRoleRootCommitment =
      liftCert.opening.nativeRoleRootCommitment
  nativeRoleRootCommitment_assembly_eq :
    fieldCert.nativeRoleRootCommitment =
      assemblyCert.nativeRoleRootCommitment
  localSectionDigest_liftOpening_eq :
    fieldCert.localSectionDigest =
      liftCert.opening.localSectionDigest
  localSectionDigest_assembly_eq :
    fieldCert.localSectionDigest =
      assemblyCert.localSectionDigest
  packedResiduesDigest_liftOpening_eq :
    fieldCert.packedResiduesDigest =
      liftCert.opening.packedResiduesDigest
  packedResiduesDigest_assembly_eq :
    fieldCert.packedResiduesDigest =
      assemblyCert.packedResiduesDigest
  randomizerDigest_payload_eq :
    fieldCert.randomizerDigest = payload.randomizerCommitment
  randomizerDigest_liftOpening_eq :
    fieldCert.randomizerDigest =
      liftCert.opening.randomizerDigest
  randomizerDigest_assembly_eq :
    fieldCert.randomizerDigest =
      assemblyCert.randomizerDigest
  selectedAccountProjectionDigest_cert_eq :
    fieldCert.selectedAccountProjectionDigest =
      cert.accountProjectionDigest
  selectedAccountProjectionDigest_liftOpening_eq :
    fieldCert.selectedAccountProjectionDigest =
      liftCert.opening.selectedAccountProjectionDigest
  selectedAccountProjectionDigest_assembly_eq :
    fieldCert.selectedAccountProjectionDigest =
      assemblyCert.selectedAccountProjectionDigest
  selectedAccountProjectionDigest_compat_eq :
    fieldCert.selectedAccountProjectionDigest =
      compatCert.selectedAccountProjectionDigest
  padSourceDigest_payload_eq :
    fieldCert.padSourceDigest = payload.padSeedDigest
  padSourceDigest_liftOpening_eq :
    fieldCert.padSourceDigest =
      liftCert.opening.padSourceDigest
  padSourceDigest_assembly_eq :
    fieldCert.padSourceDigest =
      assemblyCert.padSourceDigest
  openingTupleDigest_payload_eq :
    fieldCert.openingTupleDigest = payload.openingCommitment
  openingTupleDigest_liftOpening_eq :
    fieldCert.openingTupleDigest =
      liftCert.opening.openingTupleDigest
  openingTupleDigest_assembly_eq :
    fieldCert.openingTupleDigest =
      assemblyCert.openingTupleDigest
  projectionRelationDigest_payload_eq :
    fieldCert.projectionRelationDigest =
      payload.projectionCarrierDigest
  projectionRelationDigest_liftOpening_eq :
    fieldCert.projectionRelationDigest =
      liftCert.opening.projectionRelationDigest
  projectionRelationDigest_assembly_eq :
    fieldCert.projectionRelationDigest =
      assemblyCert.projectionRelationDigest
  lifecycleSlotDigest_liftOpening_eq :
    fieldCert.lifecycleSlotDigest =
      liftCert.opening.lifecycleSlotDigest
  lifecycleSlotDigest_assembly_eq :
    fieldCert.lifecycleSlotDigest =
      assemblyCert.lifecycleSlotDigest
  traceWitnessDigest_liftOpening_eq :
    fieldCert.traceWitnessDigest =
      liftCert.opening.traceWitnessDigest
  traceWitnessDigest_assembly_eq :
    fieldCert.traceWitnessDigest =
      assemblyCert.traceWitnessDigest
  gaugeDigest_liftOpening_eq :
    fieldCert.gaugeDigest = liftCert.opening.gaugeDigest
  gaugeDigest_assembly_eq :
    fieldCert.gaugeDigest = assemblyCert.gaugeDigest
  fullOpeningDigest_liftOpening_eq :
    fieldCert.fullOpeningDigest =
      liftCert.opening.openingDigest
  fullOpeningDigest_assembly_eq :
    fieldCert.fullOpeningDigest =
      assemblyCert.fullOpeningDigest
  fullOpeningDigest_assemblyOpening_eq :
    fieldCert.fullOpeningDigest =
      assemblyCert.opening.openingDigest
  fullOpeningDigest_compat_eq :
    fieldCert.fullOpeningDigest =
      compatCert.fullOpeningDigest
  representativeDigest_assembly_eq :
    fieldCert.representativeDigest =
      assemblyCert.representativeDigest
  representativeDigest_compat_eq :
    fieldCert.representativeDigest =
      compatCert.representativeDigest
  assemblyOpening_eq :
    assemblyCert.opening = liftCert.opening

/--
Binding contract for the Rust source-route object.

The route proves ordering and digest agreement, while the detailed
field-evidence binding remains the source of concrete role/assembly/
compatibility field equalities.
-/
structure ReviewedFq12FullOpeningFieldSourceRouteBinds
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate)
    (fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate)
    (routeCert : ReviewedFq12FullOpeningFieldSourceRouteCertificate) :
    Prop where
  assembly_source_present :
    routeCert.assemblyIsFullOpeningFieldCertificate = true
  compatibility_source_present :
    routeCert.compatibilityIsSameRepresentativeFieldCertificate = true
  field_evidence_adapter_only :
    routeCert.fieldEvidenceIsAdapterOnly = true
  aggregate_bridge_unused :
    routeCert.aggregateBridgeUsedAsSource = false
  payloadBindingDigest_cert_eq :
    routeCert.payloadBindingDigest = cert.payloadBindingDigest
  payloadBindingDigest_lift_eq :
    routeCert.payloadBindingDigest = liftCert.payloadBindingDigest
  payloadBindingDigest_assembly_eq :
    routeCert.payloadBindingDigest = assemblyCert.payloadBindingDigest
  payloadBindingDigest_compat_eq :
    routeCert.payloadBindingDigest = compatCert.payloadBindingDigest
  extractionCertificateDigest_cert_eq :
    routeCert.extractionCertificateDigest =
      cert.extractionCertificateDigest
  extractionCertificateDigest_lift_eq :
    routeCert.extractionCertificateDigest =
      liftCert.extractionCertificateDigest
  extractionCertificateDigest_assembly_eq :
    routeCert.extractionCertificateDigest =
      assemblyCert.extractionCertificateDigest
  extractionCertificateDigest_compat_eq :
    routeCert.extractionCertificateDigest =
      compatCert.extractionCertificateDigest
  creationProofBindingDigest_payload_eq :
    routeCert.creationProofBindingDigest =
      payload.creationProofBindingDigest
  creationProofBindingDigest_lift_eq :
    routeCert.creationProofBindingDigest =
      liftCert.creationProofBindingDigest
  creationProofBindingDigest_assembly_eq :
    routeCert.creationProofBindingDigest =
      assemblyCert.creationProofBindingDigest
  roleLiftCertificateDigest_lift_eq :
    routeCert.roleLiftCertificateDigest =
      liftCert.roleLiftCertificateDigest
  roleLiftCertificateDigest_assembly_eq :
    routeCert.roleLiftCertificateDigest =
      assemblyCert.roleLiftCertificateDigest
  roleLiftCertificateDigest_compat_eq :
    routeCert.roleLiftCertificateDigest =
      compatCert.roleLiftCertificateDigest
  assemblyCertificateDigest_assembly_eq :
    routeCert.assemblyCertificateDigest =
      assemblyCert.assemblyCertificateDigest
  assemblyCertificateDigest_compat_eq :
    routeCert.assemblyCertificateDigest =
      compatCert.assemblyCertificateDigest
  compatibilityCertificateDigest_compat_eq :
    routeCert.compatibilityCertificateDigest =
      compatCert.compatibilityCertificateDigest
  fieldEvidenceDigest_eq :
    routeCert.fieldEvidenceDigest = fieldCert.fieldEvidenceDigest
  nativeBoundaryDigest_payload_eq :
    routeCert.nativeBoundaryDigest = payload.nativeBoundaryDigest
  nativeBoundaryDigest_cert_eq :
    routeCert.nativeBoundaryDigest = cert.nativeBoundaryDigest
  nativeBoundaryDigest_lift_eq :
    routeCert.nativeBoundaryDigest = liftCert.nativeBoundaryDigest
  nativeBoundaryDigest_assembly_eq :
    routeCert.nativeBoundaryDigest = assemblyCert.nativeBoundaryDigest
  nativeBoundaryDigest_compat_eq :
    routeCert.nativeBoundaryDigest = compatCert.nativeBoundaryDigest
  slotId_payload_eq :
    routeCert.slotId = payload.slotId
  slotId_cert_eq :
    routeCert.slotId = cert.slotId
  slotId_lift_eq :
    routeCert.slotId = liftCert.slotId
  slotId_assembly_eq :
    routeCert.slotId = assemblyCert.slotId
  slotId_compat_eq :
    routeCert.slotId = compatCert.slotId
  nativeRoleRootCommitment_lift_eq :
    routeCert.nativeRoleRootCommitment =
      liftCert.nativeRoleRootCommitment
  nativeRoleRootCommitment_liftOpening_eq :
    routeCert.nativeRoleRootCommitment =
      liftCert.opening.nativeRoleRootCommitment
  nativeRoleRootCommitment_assembly_eq :
    routeCert.nativeRoleRootCommitment =
      assemblyCert.nativeRoleRootCommitment
  fullOpeningDigest_liftOpening_eq :
    routeCert.fullOpeningDigest =
      liftCert.opening.openingDigest
  fullOpeningDigest_assembly_eq :
    routeCert.fullOpeningDigest =
      assemblyCert.fullOpeningDigest
  fullOpeningDigest_compat_eq :
    routeCert.fullOpeningDigest =
      compatCert.fullOpeningDigest
  representativeDigest_assembly_eq :
    routeCert.representativeDigest =
      assemblyCert.representativeDigest
  representativeDigest_compat_eq :
    routeCert.representativeDigest =
      compatCert.representativeDigest
  selectedAccountProjectionDigest_cert_eq :
    routeCert.selectedAccountProjectionDigest =
      cert.accountProjectionDigest
  selectedAccountProjectionDigest_liftOpening_eq :
    routeCert.selectedAccountProjectionDigest =
      liftCert.opening.selectedAccountProjectionDigest
  selectedAccountProjectionDigest_assembly_eq :
    routeCert.selectedAccountProjectionDigest =
      assemblyCert.selectedAccountProjectionDigest
  selectedAccountProjectionDigest_compat_eq :
    routeCert.selectedAccountProjectionDigest =
      compatCert.selectedAccountProjectionDigest

/-- Source-route binding carries the pre-bridge ordering guard. -/
theorem aggregate_bridge_unused_of_full_opening_source_route_binds
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    {routeCert : ReviewedFq12FullOpeningFieldSourceRouteCertificate}
    (h :
      ReviewedFq12FullOpeningFieldSourceRouteBinds
        payload cert liftCert assemblyCert compatCert fieldCert routeCert) :
    routeCert.aggregateBridgeUsedAsSource = false :=
  h.aggregate_bridge_unused

/--
The pre-aggregate field evidence supplies the role-lift extraction binding.
-/
theorem role_lift_binds_extraction_of_full_opening_field_evidence
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (h :
      ReviewedFq12FullOpeningFieldEvidenceBinds
        payload cert liftCert assemblyCert compatCert fieldCert) :
    ReviewedFq12RoleLiftCertificateBindsExtraction
      payload cert liftCert := by
  exact
    ⟨h.payloadBindingDigest_lift_eq.symm.trans
        h.payloadBindingDigest_cert_eq,
      h.extractionCertificateDigest_lift_eq.symm.trans
        h.extractionCertificateDigest_cert_eq,
      h.creationProofBindingDigest_lift_eq.symm.trans
        h.creationProofBindingDigest_payload_eq,
      h.nativeBoundaryDigest_lift_eq.symm.trans
        h.nativeBoundaryDigest_payload_eq,
      h.nativeBoundaryDigest_lift_eq.symm.trans
        h.nativeBoundaryDigest_cert_eq,
      h.slotId_lift_eq.symm.trans h.slotId_payload_eq,
      h.slotId_lift_eq.symm.trans h.slotId_cert_eq,
      h.creationStatementDigest_lift_eq.symm.trans
        h.creationStatementDigest_payload_eq,
      h.creationStatementDigest_lift_eq.symm.trans
        h.creationStatementDigest_cert_eq,
      h.reviewedStatementDigest_lift_eq.symm.trans
        h.reviewedStatementDigest_payload_eq,
      h.reviewedStatementDigest_lift_eq.symm.trans
        h.reviewedStatementDigest_cert_eq,
      h.nativeRoleRootCommitment_liftOpening_eq.symm.trans
        h.nativeRoleRootCommitment_lift_eq,
      h.selectedAccountProjectionDigest_liftOpening_eq.symm.trans
        h.selectedAccountProjectionDigest_cert_eq,
      h.randomizerDigest_liftOpening_eq.symm.trans
        h.randomizerDigest_payload_eq,
      h.openingTupleDigest_liftOpening_eq.symm.trans
        h.openingTupleDigest_payload_eq,
      h.padSourceDigest_liftOpening_eq.symm.trans
        h.padSourceDigest_payload_eq,
      h.projectionRelationDigest_liftOpening_eq.symm.trans
        h.projectionRelationDigest_payload_eq⟩

/--
The same evidence supplies the native representative assembly binding for the
opening carried by the role-lift certificate.
-/
theorem assembly_binds_opening_of_full_opening_field_evidence
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (h :
      ReviewedFq12FullOpeningFieldEvidenceBinds
        payload cert liftCert assemblyCert compatCert fieldCert) :
    ReviewedFq12AssemblyCertificateBindsOpening
      payload cert liftCert.opening assemblyCert := by
  exact
    ⟨h.payloadBindingDigest_assembly_eq.symm.trans
        h.payloadBindingDigest_cert_eq,
      h.extractionCertificateDigest_assembly_eq.symm.trans
        h.extractionCertificateDigest_cert_eq,
      h.creationProofBindingDigest_assembly_eq.symm.trans
        h.creationProofBindingDigest_payload_eq,
      h.nativeBoundaryDigest_assembly_eq.symm.trans
        h.nativeBoundaryDigest_payload_eq,
      h.nativeBoundaryDigest_assembly_eq.symm.trans
        h.nativeBoundaryDigest_cert_eq,
      h.slotId_assembly_eq.symm.trans h.slotId_payload_eq,
      h.slotId_assembly_eq.symm.trans h.slotId_cert_eq,
      h.creationStatementDigest_assembly_eq.symm.trans
        h.creationStatementDigest_payload_eq,
      h.creationStatementDigest_assembly_eq.symm.trans
        h.creationStatementDigest_cert_eq,
      h.reviewedStatementDigest_assembly_eq.symm.trans
        h.reviewedStatementDigest_payload_eq,
      h.reviewedStatementDigest_assembly_eq.symm.trans
        h.reviewedStatementDigest_cert_eq,
      h.nativeRoleRootCommitment_assembly_eq.symm.trans
        h.nativeRoleRootCommitment_liftOpening_eq,
      h.fullOpeningDigest_assembly_eq.symm.trans
        h.fullOpeningDigest_liftOpening_eq,
      h.localSectionDigest_assembly_eq.symm.trans
        h.localSectionDigest_liftOpening_eq,
      h.packedResiduesDigest_assembly_eq.symm.trans
        h.packedResiduesDigest_liftOpening_eq,
      h.randomizerDigest_assembly_eq.symm.trans
        h.randomizerDigest_liftOpening_eq,
      h.selectedAccountProjectionDigest_assembly_eq.symm.trans
        h.selectedAccountProjectionDigest_liftOpening_eq,
      h.padSourceDigest_assembly_eq.symm.trans
        h.padSourceDigest_liftOpening_eq,
      h.openingTupleDigest_assembly_eq.symm.trans
        h.openingTupleDigest_liftOpening_eq,
      h.projectionRelationDigest_assembly_eq.symm.trans
        h.projectionRelationDigest_liftOpening_eq,
      h.lifecycleSlotDigest_assembly_eq.symm.trans
        h.lifecycleSlotDigest_liftOpening_eq,
      h.traceWitnessDigest_assembly_eq.symm.trans
        h.traceWitnessDigest_liftOpening_eq,
      h.gaugeDigest_assembly_eq.symm.trans h.gaugeDigest_liftOpening_eq,
      h.assemblyOpening_eq⟩

/--
The pre-aggregate evidence yields the pure deterministic assembly construction
certificate consumed by the construction reduction.
-/
theorem assembly_field_construction_of_full_opening_field_evidence
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (h :
      ReviewedFq12FullOpeningFieldEvidenceBinds
        payload cert liftCert assemblyCert compatCert fieldCert) :
    ReviewedFq12AssemblyFieldConstructionCertificate
      liftCert.opening assemblyCert :=
  reviewed_fq12_assembly_field_construction_from_binds
    (assembly_binds_opening_of_full_opening_field_evidence h)

/--
The pre-aggregate field evidence binds the compatibility certificate to the
same deterministic representative named by the assembly certificate.
-/
theorem compatibility_binds_deterministic_representative_of_full_opening_field_evidence
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (h :
      ReviewedFq12FullOpeningFieldEvidenceBinds
        payload cert liftCert assemblyCert compatCert fieldCert) :
    ReviewedFq12ExtractionCompatibilityCertificateBinds
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert := by
  exact
    ⟨h.payloadBindingDigest_compat_eq.symm.trans
        h.payloadBindingDigest_cert_eq,
      h.extractionCertificateDigest_compat_eq.symm.trans
        h.extractionCertificateDigest_cert_eq,
      h.nativeBoundaryDigest_compat_eq.symm.trans
        h.nativeBoundaryDigest_payload_eq,
      h.nativeBoundaryDigest_compat_eq.symm.trans
        h.nativeBoundaryDigest_cert_eq,
      h.slotId_compat_eq.symm.trans h.slotId_payload_eq,
      h.slotId_compat_eq.symm.trans h.slotId_cert_eq,
      h.creationStatementDigest_compat_eq.symm.trans
        h.creationStatementDigest_payload_eq,
      h.creationStatementDigest_compat_eq.symm.trans
        h.creationStatementDigest_cert_eq,
      h.reviewedStatementDigest_compat_eq.symm.trans
        h.reviewedStatementDigest_payload_eq,
      h.reviewedStatementDigest_compat_eq.symm.trans
        h.reviewedStatementDigest_cert_eq,
      h.fullOpeningDigest_compat_eq.symm.trans
        h.fullOpeningDigest_liftOpening_eq,
      h.selectedAccountProjectionDigest_compat_eq.symm.trans
        h.selectedAccountProjectionDigest_liftOpening_eq,
      h.selectedAccountProjectionDigest_compat_eq.symm.trans
        h.selectedAccountProjectionDigest_cert_eq,
      h.representativeDigest_compat_eq.symm.trans
        h.representativeDigest_assembly_eq,
      reviewed_fq12_assembly_constructed_representative_matches_opening
        (assembly_binds_opening_of_full_opening_field_evidence h)⟩

/--
The same binding can be consumed by the concrete compatibility field package.
-/
theorem same_representative_concrete_fields_of_full_opening_field_evidence
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (h :
      ReviewedFq12FullOpeningFieldEvidenceBinds
        payload cert liftCert assemblyCert compatCert fieldCert) :
    SameRepresentativeConcreteFieldCertificate
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert :=
  same_representative_concrete_field_certificate_of_binding
    (compatibility_binds_deterministic_representative_of_full_opening_field_evidence h)

/--
The field evidence also gives the non-root opening equality between the
role-lift opening and the assembly certificate's carried opening.
-/
theorem nonroot_opening_fields_eq_assembly_opening_of_full_opening_field_evidence
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (h :
      ReviewedFq12FullOpeningFieldEvidenceBinds
        payload cert liftCert assemblyCert compatCert fieldCert) :
    FullNativeOpeningNonRootRoleFieldEqualityCertificate
      liftCert.opening assemblyCert.opening := by
  rw [h.assemblyOpening_eq]
  exact
    { localSectionDigest_eq := rfl
      packedResiduesDigest_eq := rfl
      randomizerDigest_eq := rfl
      selectedAccountProjectionDigest_eq := rfl
      padSourceDigest_eq := rfl
      openingTupleDigest_eq := rfl
      projectionRelationDigest_eq := rfl
      lifecycleSlotDigest_eq := rfl
      traceWitnessDigest_eq := rfl
      gaugeDigest_eq := rfl
      openingDigest_eq := rfl }

end Path2FullOpeningFieldEvidence

end Descender
