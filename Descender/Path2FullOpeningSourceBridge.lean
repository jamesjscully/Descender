import Descender.Path2NativeBoundary

namespace Descender

/-!
# Path 2 Full-Opening Source Bridge

This module factors the active full-opening route through source-level
binding-or-break obligations.  The older bridge package requires successful
field binding for assembly and compatibility certificates.  For the active
path, those byte/source predicates are more honest as local checks that either
produce the field binding or export a named boundary break.

Keeping this layer separate prevents the aggregate full-opening bridge from
being used as the source of assembly or compatibility soundness.
-/

namespace Path2FullOpeningSourceBridge

open Path2NativeBoundary

/--
Source-level assembly binding check.

Given accepted payload/extraction evidence, a lifted opening, and an available
valid assembly certificate, the source check either binds the certificate to
the opened tuple or names the boundary break.
-/
def ReviewedFq12AssemblyCertificateBindingOrBreak
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              P.reviewedFq12OpeningHasAssemblyCertificate
                  b payload cert opening assemblyCert ->
                P.reviewedFq12AssemblyCertificateValid
                    b payload cert opening assemblyCert ->
                  ReviewedFq12AssemblyCertificateBindsOpening
                      payload cert opening assemblyCert ∨
                    (exists breakEvent,
                      P.boundaryBreakFor b.context b breakEvent)

/--
Assembly-certificate availability and soundness instantiate native assembly
when the binding check may itself return a named break.
-/
theorem reviewed_fq12_full_native_opening_assembly_from_certificate_or_break_binding
    (P : Predicates)
    (hAvail : ReviewedFq12RepresentativeAssemblyCertificateAvailability P)
    (hSound : ReviewedFq12RepresentativeAssemblyCertificateSoundness P)
    (hBind : ReviewedFq12AssemblyCertificateBindingOrBreak P) :
    ReviewedFq12FullNativeOpeningAssemblySoundness P := by
  intro b payload cert opening hAccept hProjects hHas hValid hLifts
  rcases hAvail hAccept hProjects hHas hValid hLifts with
    hAssemblyCert | hBreak
  · rcases hAssemblyCert with ⟨assemblyCert, hHasAssembly, hAssemblyValid⟩
    rcases
        hBind hAccept hProjects hHas hValid hLifts hHasAssembly
          hAssemblyValid with
      hBinds | hBreak
    · rcases
          hSound hAccept hProjects hHas hValid hLifts hHasAssembly
            hAssemblyValid hBinds with
        hAssembled | hBreak
      · rcases hAssembled with
          ⟨h, hMatches, _hAssemblyCertMatches,
            _hAssemblyCertAssembles, hAssembles,
            hFiber, hLocal, hDescent⟩
        exact Or.inl
          ⟨h, hMatches, hAssembles, hFiber, hLocal, hDescent⟩
      · exact Or.inr hBreak
    · exact Or.inr hBreak
  · exact Or.inr hBreak

/--
Source-level extraction-compatibility binding check.

Once assembly has produced a native representative in the boundary fiber, a
valid compatibility certificate must either bind the reviewed-Fq12 extraction
relations to that representative or export a named break.
-/
def ReviewedFq12ExtractionCompatibilityCertificateBindingOrBreak
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
                                payload cert opening h compatCert ∨
                              (exists breakEvent,
                                P.boundaryBreakFor b.context b breakEvent)

/--
Compatibility-certificate availability and soundness instantiate extraction
compatibility when the binding check may itself return a named break.
-/
theorem reviewed_fq12_extraction_compatibility_from_certificate_or_break_binding
    (P : Predicates)
    (hAvail : ReviewedFq12ExtractionCompatibilityCertificateAvailability P)
    (hSound : ReviewedFq12ExtractionCompatibilityCertificateSoundness P)
    (hBind :
      ReviewedFq12ExtractionCompatibilityCertificateBindingOrBreak P) :
    ReviewedFq12FullNativeOpeningExtractionCompatibility P := by
  intro b payload cert opening h hAccept hProjects hHas hValid hLifts
    hMatches hAssembles hFiber hLocal hDescent
  rcases
      hAvail hAccept hProjects hHas hValid hLifts hMatches hAssembles
        hFiber hLocal hDescent with
    hCompatCert | hBreak
  · rcases hCompatCert with ⟨compatCert, hHasCompat, hCompatValid⟩
    rcases
        hBind hAccept hProjects hHas hValid hLifts hMatches hAssembles
          hFiber hLocal hDescent hHasCompat hCompatValid with
      hBinds | hBreak
    · rcases
          hSound hAccept hProjects hHas hValid hLifts hMatches hAssembles
            hFiber hLocal hDescent hHasCompat hCompatValid hBinds with
        hExtracts | hBreak
      · rcases hExtracts with
          ⟨_hCompatExtracts, hCertExtracts, hPayloadExtracts⟩
        exact Or.inl ⟨hCertExtracts, hPayloadExtracts⟩
      · exact Or.inr hBreak
    · exact Or.inr hBreak
  · exact Or.inr hBreak

/--
Active source bridge for the reviewed-Fq12 full-opening route.

This is the non-circular certificate package: role-root evidence supplies the
opening, assembly evidence supplies the native representative, and
compatibility evidence binds the reviewed-Fq12 extraction relations to that
assembled representative.  Assembly and compatibility field checks are allowed
to fail only by naming a boundary break.
-/
structure ReviewedFq12FullOpeningSourceBridgeCertificate
    (P : Predicates) where
  roleRootBridge :
    ReviewedFq12RoleRootBridgeCertificate P
  assemblyAvailability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  assemblyBinding :
    ReviewedFq12AssemblyCertificateBindingOrBreak P
  assemblyConstructionSoundness :
    ReviewedFq12RepresentativeAssemblyConstructionSoundness P
  assembledRepresentativeNativeValiditySoundness :
    ReviewedFq12AssembledRepresentativeNativeValiditySoundness P
  compatibilityAvailability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  compatibilityBinding :
    ReviewedFq12ExtractionCompatibilityCertificateBindingOrBreak P
  compatibilitySoundness :
    ReviewedFq12ExtractionCompatibilityCertificateSoundness P

/--
The active source bridge discharges reviewed-Fq12 extraction-certificate
soundness without using the aggregate full-opening bridge as a premise.
-/
theorem reviewed_fq12_certificate_soundness_from_full_opening_source_bridge
    (P : Predicates)
    (C : ReviewedFq12FullOpeningSourceBridgeCertificate P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_role_lift P
    (reviewed_fq12_selected_role_lift_from_role_root_bridge
      P C.roleRootBridge)
    (reviewed_fq12_full_native_opening_soundness_from_assembly
      P
      (reviewed_fq12_full_native_opening_assembly_from_certificate_or_break_binding
        P C.assemblyAvailability
        (reviewed_fq12_representative_assembly_soundness_from_parts
          P C.assemblyConstructionSoundness
          C.assembledRepresentativeNativeValiditySoundness)
        C.assemblyBinding)
      (reviewed_fq12_extraction_compatibility_from_certificate_or_break_binding
        P C.compatibilityAvailability C.compatibilitySoundness
        C.compatibilityBinding))

end Path2FullOpeningSourceBridge

end Descender
