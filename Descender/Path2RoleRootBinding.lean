import Descender.Path2NativeBoundary

namespace Descender
namespace Path2NativeBoundary

/-!
# Path 2 Role-Root Binding

This module isolates the non-aggregate role-root binding obligation used by
the active Path 2 native / reviewed-Fq12 route.

The aggregate full-opening bridge object is intentionally absent from these
interfaces.  The split below names the two remaining facts needed before the
assembly and compatibility layers:

* the accepted creation/payload binding fixes one native role root for the
  selected reviewed-Fq12 extraction certificate; and
* that native role root is binding for every coordinate of the carried full
  native opening.
-/

/--
Coordinate equality for the full native opening roles.

This is stronger than equality of the opened representative predicate: it pins
the digest handles that the later assembly layer consumes.
-/
def FullNativeOpeningRoleCoordinatesEqual
    (left right : FullNativeRepresentativeOpening) : Prop :=
  left.nativeBoundaryDigest = right.nativeBoundaryDigest ∧
  left.nativeRoleRootCommitment = right.nativeRoleRootCommitment ∧
    left.localSectionDigest = right.localSectionDigest ∧
      left.packedResiduesDigest = right.packedResiduesDigest ∧
        left.randomizerDigest = right.randomizerDigest ∧
          left.selectedAccountProjectionDigest =
            right.selectedAccountProjectionDigest ∧
            left.padSourceDigest = right.padSourceDigest ∧
              left.openingTupleDigest = right.openingTupleDigest ∧
                left.projectionRelationDigest =
                  right.projectionRelationDigest ∧
                  left.lifecycleSlotDigest =
                    right.lifecycleSlotDigest ∧
                    left.traceWitnessDigest =
                      right.traceWitnessDigest ∧
                      left.gaugeDigest = right.gaugeDigest ∧
                        left.openingDigest = right.openingDigest

/--
Opening-coordinate equality is enough to transport the semantic
representative-match predicate.
-/
theorem full_native_opening_matches_of_role_coordinates_eq
    {left right : FullNativeRepresentativeOpening}
    {h : HiddenRepresentative}
    (hEq : FullNativeOpeningRoleCoordinatesEqual left right)
    (hMatches : FullNativeOpeningMatchesRepresentative left h) :
    FullNativeOpeningMatchesRepresentative right h := by
  rcases hEq with
    ⟨hNativeBoundary, _hRoot, hLocal, hPacked, hRandomizer, hSelected, hPad, hTuple,
      hProjection, hLifecycle, hTrace, hGauge, _hDigest⟩
  rcases hMatches with
    ⟨mNativeBoundary, mLocal, mPacked, mRandomizer, mSelected, mPad, mTuple,
      mProjection, mLifecycle, mTrace, mGauge⟩
  exact
    ⟨mNativeBoundary.trans hNativeBoundary, mLocal.trans hLocal, mPacked.trans hPacked,
      mRandomizer.trans hRandomizer, mSelected.trans hSelected,
      mPad.trans hPad, mTuple.trans hTuple,
      mProjection.trans hProjection, mLifecycle.trans hLifecycle,
      mTrace.trans hTrace, mGauge.trans hGauge⟩

/--
Residual creation/payload binding assumption.

For one accepted payload and selected extraction certificate, two valid
role-lift certificates that both satisfy the non-aggregate field binding must
name the same native role root, or the verifier must export a named break.
-/
def ReviewedFq12CreationPayloadPinsNativeRoleRoot
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {left right : ReviewedFq12FullNativeOpeningCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12PayloadHasRoleLiftCertificate
                b payload cert left ->
              P.reviewedFq12RoleLiftCertificateValid
                  b payload cert left ->
                ReviewedFq12RoleLiftCertificateBindsExtraction
                    payload cert left ->
                  P.reviewedFq12PayloadHasRoleLiftCertificate
                      b payload cert right ->
                    P.reviewedFq12RoleLiftCertificateValid
                        b payload cert right ->
                      ReviewedFq12RoleLiftCertificateBindsExtraction
                          payload cert right ->
                        left.nativeRoleRootCommitment =
                          right.nativeRoleRootCommitment ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)

/--
Residual native-role-root binding assumption.

Once creation/payload binding has identified the same native role root, the
root binding for the role-lift object pins the entire full native opening
coordinate tuple, or exports a named break.
-/
def ReviewedFq12NativeRoleRootPinsFullOpeningRoles
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {left right : ReviewedFq12FullNativeOpeningCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12PayloadHasRoleLiftCertificate
                b payload cert left ->
              P.reviewedFq12RoleLiftCertificateValid
                  b payload cert left ->
                ReviewedFq12RoleLiftCertificateBindsExtraction
                    payload cert left ->
                  P.reviewedFq12RoleLiftCertificateOpens
                      b payload cert left left.opening ->
                    P.reviewedFq12PayloadHasRoleLiftCertificate
                        b payload cert right ->
                      P.reviewedFq12RoleLiftCertificateValid
                          b payload cert right ->
                        ReviewedFq12RoleLiftCertificateBindsExtraction
                            payload cert right ->
                          P.reviewedFq12RoleLiftCertificateOpens
                              b payload cert right right.opening ->
                            left.nativeRoleRootCommitment =
                              right.nativeRoleRootCommitment ->
                              FullNativeOpeningRoleCoordinatesEqual
                                left.opening right.opening ∨
                              (exists breakEvent,
                                P.boundaryBreakFor b.context b breakEvent)

/--
The non-aggregate role-root binding package.

This extends the existing role-root bridge with only the two additional
pinning assumptions above. It does not depend on, mention, or derive anything
from the aggregate full-opening bridge digest certificate.
-/
structure ReviewedFq12NonAggregateRoleRootBindingCertificate
    (P : Predicates) where
  roleRootBridge :
    ReviewedFq12RoleRootBridgeCertificate P
  creationPayloadPinsNativeRoleRoot :
    ReviewedFq12CreationPayloadPinsNativeRoleRoot P
  nativeRoleRootPinsFullOpeningRoles :
    ReviewedFq12NativeRoleRootPinsFullOpeningRoles P

/--
The existing role-root bridge still supplies the selected-certificate lift
surface; the extra fields in
`ReviewedFq12NonAggregateRoleRootBindingCertificate` are reserved for the
stronger full-role pinning theorem below.
-/
theorem reviewed_fq12_selected_role_lift_from_nonaggregate_role_root_binding
    (P : Predicates)
    (C : ReviewedFq12NonAggregateRoleRootBindingCertificate P) :
    ReviewedFq12SelectedCertificateRoleLiftSoundness P :=
  reviewed_fq12_selected_role_lift_from_role_root_bridge P C.roleRootBridge

/--
Main non-aggregate pinning theorem.

For the same accepted payload and selected extraction certificate, the
role-root bridge opens both valid role-lift certificates, creation/payload
binding first forces the same native role root, and native-role-root binding
then forces equality of every full-opening role coordinate.
-/
theorem reviewed_fq12_role_root_bridge_pins_full_native_opening_roles
    (P : Predicates)
    (C : ReviewedFq12NonAggregateRoleRootBindingCertificate P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {left right : ReviewedFq12FullNativeOpeningCertificate}
    (hAccept : P.reviewedFq12PayloadAccepts b payload)
    (hProjects : P.reviewedFq12PayloadProjectsFrom b payload)
    (hHas : P.reviewedFq12PayloadHasExtractionCertificate b payload cert)
    (hValid : P.reviewedFq12ExtractionCertificateValid b payload cert)
    (hHasLeft :
      P.reviewedFq12PayloadHasRoleLiftCertificate b payload cert left)
    (hValidLeft :
      P.reviewedFq12RoleLiftCertificateValid b payload cert left)
    (hHasRight :
      P.reviewedFq12PayloadHasRoleLiftCertificate b payload cert right)
    (hValidRight :
      P.reviewedFq12RoleLiftCertificateValid b payload cert right) :
    FullNativeOpeningRoleCoordinatesEqual left.opening right.opening ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  have hBindsLeft :=
    C.roleRootBridge.roleLiftPayloadBinding hAccept hProjects hHas hValid
      hHasLeft hValidLeft
  have hBindsRight :=
    C.roleRootBridge.roleLiftPayloadBinding hAccept hProjects hHas hValid
      hHasRight hValidRight
  rcases
      C.roleRootBridge.nativeRoleRootSoundness hAccept hProjects hHas
        hValid hHasLeft hValidLeft hBindsLeft with
    hOpensLeft | hBreak
  · rcases
        C.roleRootBridge.nativeRoleRootSoundness hAccept hProjects hHas
          hValid hHasRight hValidRight hBindsRight with
      hOpensRight | hBreak
    · rcases
          C.creationPayloadPinsNativeRoleRoot hAccept hProjects hHas
            hValid hHasLeft hValidLeft hBindsLeft hHasRight hValidRight
            hBindsRight with
        hSameRoot | hBreak
      · exact
          C.nativeRoleRootPinsFullOpeningRoles hAccept hProjects hHas
            hValid hHasLeft hValidLeft hBindsLeft hOpensLeft hHasRight
            hValidRight hBindsRight hOpensRight hSameRoot
      · exact Or.inr hBreak
    · exact Or.inr hBreak
  · exact Or.inr hBreak

end Path2NativeBoundary
end Descender
