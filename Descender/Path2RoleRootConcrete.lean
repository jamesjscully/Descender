import Descender.Path2RoleRootBinding
import Descender.Path2NativeBoundary

namespace Descender
namespace Path2NativeBoundary

/-!
# Path 2 Concrete Role-Root Field Binding

This module lowers the two non-aggregate role-root residuals from
`Path2RoleRootBinding` to pairwise field-level certificates.

The aggregate full-opening bridge object is intentionally not used here.  The
remaining assumptions are direct commitments/equality certificates over the
role-lift certificate fields that the active Path 2 route already exposes.
-/

/--
Pairwise certificate that the accepted creation/payload binding fixes one
native role root for the selected reviewed-Fq12 extraction certificate.
-/
structure ReviewedFq12CreationPayloadNativeRoleRootFieldEqualityCertificate
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (left right : ReviewedFq12FullNativeOpeningCertificate) where
  nativeRoleRootCommitment_eq :
    left.nativeRoleRootCommitment = right.nativeRoleRootCommitment

/--
Concrete field-binding premise for creation/payload native-role-root
uniqueness.

Compared with `ReviewedFq12CreationPayloadPinsNativeRoleRoot`, this premise is
smaller: it only consumes the two explicit extraction-binding facts and emits
a pairwise field equality certificate.  Boundary acceptance/projection
premises and break export remain outside this concrete equality step.
-/
def ReviewedFq12CreationPayloadNativeRoleRootFieldBinding : Prop :=
  forall {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {left right : ReviewedFq12FullNativeOpeningCertificate},
    ReviewedFq12RoleLiftCertificateBindsExtraction payload cert left ->
      ReviewedFq12RoleLiftCertificateBindsExtraction payload cert right ->
        ReviewedFq12CreationPayloadNativeRoleRootFieldEqualityCertificate
          payload cert left right

/--
The concrete creation/payload native-role-root field binding discharges the
corresponding non-aggregate residual.
-/
theorem reviewed_fq12_creation_payload_pins_native_role_root_of_field_binding
    (P : Predicates)
    (hFieldBinding :
      ReviewedFq12CreationPayloadNativeRoleRootFieldBinding) :
    ReviewedFq12CreationPayloadPinsNativeRoleRoot P := by
  intro b payload cert left right _hAccept _hProjects _hHas _hValid
    _hHasLeft _hValidLeft hBindsLeft _hHasRight _hValidRight hBindsRight
  exact Or.inl
    (hFieldBinding hBindsLeft hBindsRight).nativeRoleRootCommitment_eq

/--
The root field carried in a full native opening is pinned to the enclosing
role-lift certificate root by the existing extraction-binding predicate.
-/
theorem reviewed_fq12_opening_native_role_root_eq_of_binds
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    (hBinds :
      ReviewedFq12RoleLiftCertificateBindsExtraction
        payload cert liftCert) :
    liftCert.opening.nativeRoleRootCommitment =
      liftCert.nativeRoleRootCommitment := by
  rcases hBinds with
    ⟨_hPayloadBinding, _hExtraction, _hCreationProof,
      _hNativeBoundaryPayload, _hNativeBoundaryCert, _hSlotPayload,
      _hSlotCert, _hCreationStatementPayload, _hCreationStatementCert,
      _hReviewedStatementPayload, _hReviewedStatementCert, hRoot,
      _hSelected, _hRandomizer, _hOpeningTuple, _hPad,
      _hProjectionRelation⟩
  exact hRoot

/--
Field-level equality certificate for every full native opening role except the
native role root.  The root coordinate is obtained from the enclosing role-lift
binding plus the already-known certificate root equality.
-/
structure FullNativeOpeningNonRootRoleFieldEqualityCertificate
    (left right : FullNativeRepresentativeOpening) where
  localSectionDigest_eq :
    left.localSectionDigest = right.localSectionDigest
  packedResiduesDigest_eq :
    left.packedResiduesDigest = right.packedResiduesDigest
  randomizerDigest_eq :
    left.randomizerDigest = right.randomizerDigest
  selectedAccountProjectionDigest_eq :
    left.selectedAccountProjectionDigest =
      right.selectedAccountProjectionDigest
  padSourceDigest_eq :
    left.padSourceDigest = right.padSourceDigest
  openingTupleDigest_eq :
    left.openingTupleDigest = right.openingTupleDigest
  projectionRelationDigest_eq :
    left.projectionRelationDigest = right.projectionRelationDigest
  lifecycleSlotDigest_eq :
    left.lifecycleSlotDigest = right.lifecycleSlotDigest
  traceWitnessDigest_eq :
    left.traceWitnessDigest = right.traceWitnessDigest
  gaugeDigest_eq :
    left.gaugeDigest = right.gaugeDigest
  openingDigest_eq :
    left.openingDigest = right.openingDigest

/--
Reassemble the coordinate predicate used by the role-root binding layer from
the root equality and concrete non-root field equalities.
-/
theorem full_native_opening_role_coordinates_eq_of_nonroot_fields
    {left right : FullNativeRepresentativeOpening}
    (hRoot :
      left.nativeRoleRootCommitment =
        right.nativeRoleRootCommitment)
    (C :
      FullNativeOpeningNonRootRoleFieldEqualityCertificate
        left right) :
    FullNativeOpeningRoleCoordinatesEqual left right :=
  ⟨hRoot, C.localSectionDigest_eq, C.packedResiduesDigest_eq,
    C.randomizerDigest_eq, C.selectedAccountProjectionDigest_eq,
    C.padSourceDigest_eq, C.openingTupleDigest_eq,
    C.projectionRelationDigest_eq, C.lifecycleSlotDigest_eq,
    C.traceWitnessDigest_eq, C.gaugeDigest_eq, C.openingDigest_eq⟩

/--
Concrete field-binding premise for native-role-root opening uniqueness.

Compared with `ReviewedFq12NativeRoleRootPinsFullOpeningRoles`, this premise is
smaller: it assumes only the two explicit extraction bindings, the two opening
facts, and the already-proven certificate root equality, then returns concrete
non-root field equalities.
-/
def ReviewedFq12NativeRoleRootNonRootFieldBinding
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {left right : ReviewedFq12FullNativeOpeningCertificate},
    ReviewedFq12RoleLiftCertificateBindsExtraction payload cert left ->
      P.reviewedFq12RoleLiftCertificateOpens
          b payload cert left left.opening ->
        ReviewedFq12RoleLiftCertificateBindsExtraction payload cert right ->
          P.reviewedFq12RoleLiftCertificateOpens
              b payload cert right right.opening ->
            left.nativeRoleRootCommitment =
              right.nativeRoleRootCommitment ->
              FullNativeOpeningNonRootRoleFieldEqualityCertificate
                left.opening right.opening

/--
The concrete native-role-root field binding discharges the corresponding
non-aggregate residual.
-/
theorem reviewed_fq12_native_role_root_pins_full_opening_roles_of_field_binding
    (P : Predicates)
    (hFieldBinding :
      ReviewedFq12NativeRoleRootNonRootFieldBinding P) :
    ReviewedFq12NativeRoleRootPinsFullOpeningRoles P := by
  intro b payload cert left right _hAccept _hProjects _hHas _hValid
    _hHasLeft _hValidLeft hBindsLeft hOpensLeft
    _hHasRight _hValidRight hBindsRight hOpensRight hSameRoot
  have hLeftOpeningRoot :
      left.opening.nativeRoleRootCommitment =
        left.nativeRoleRootCommitment :=
    reviewed_fq12_opening_native_role_root_eq_of_binds hBindsLeft
  have hRightOpeningRoot :
      right.opening.nativeRoleRootCommitment =
        right.nativeRoleRootCommitment :=
    reviewed_fq12_opening_native_role_root_eq_of_binds hBindsRight
  have hOpeningRoot :
      left.opening.nativeRoleRootCommitment =
        right.opening.nativeRoleRootCommitment :=
    hLeftOpeningRoot.trans (hSameRoot.trans hRightOpeningRoot.symm)
  exact Or.inl
    (full_native_opening_role_coordinates_eq_of_nonroot_fields hOpeningRoot
      (hFieldBinding hBindsLeft hOpensLeft hBindsRight hOpensRight
        hSameRoot))

/--
Concrete non-aggregate role-root package.  It combines the existing
role-root bridge with the smaller field-level premises above.
-/
structure ReviewedFq12ConcreteRoleRootFieldBindingCertificate
    (P : Predicates) where
  roleRootBridge :
    ReviewedFq12RoleRootBridgeCertificate P
  creationPayloadNativeRoleRootBinding :
    ReviewedFq12CreationPayloadNativeRoleRootFieldBinding
  nativeRoleRootNonRootFieldBinding :
    ReviewedFq12NativeRoleRootNonRootFieldBinding P

/--
The concrete field-binding package instantiates the non-aggregate role-root
binding certificate without using the aggregate full-opening bridge object.
-/
theorem reviewed_fq12_nonaggregate_role_root_binding_from_concrete_fields
    (P : Predicates)
    (C : ReviewedFq12ConcreteRoleRootFieldBindingCertificate P) :
    ReviewedFq12NonAggregateRoleRootBindingCertificate P where
  roleRootBridge := C.roleRootBridge
  creationPayloadPinsNativeRoleRoot :=
    reviewed_fq12_creation_payload_pins_native_role_root_of_field_binding
      P C.creationPayloadNativeRoleRootBinding
  nativeRoleRootPinsFullOpeningRoles :=
    reviewed_fq12_native_role_root_pins_full_opening_roles_of_field_binding
      P C.nativeRoleRootNonRootFieldBinding

end Path2NativeBoundary
end Descender
