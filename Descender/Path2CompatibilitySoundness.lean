import Descender.Path2NativeBoundary

namespace Descender

/-!
# Path 2 Compatibility Certificate Soundness

This module isolates the AP2-001/P2FS-001 same-representative compatibility
step after native assembly has already produced a representative in the
native fiber. It deliberately does not use the aggregate full-opening bridge as
the source of compatibility.
-/

namespace Path2CompatibilitySoundness

open Path2NativeBoundary

/--
The representative premise available after native assembly has produced the
same representative named by the full opening and placed it in the native
boundary fiber.
-/
def AssembledNativeFiberRepresentative
    (P : Predicates)
    (b : Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative) : Prop :=
  FullNativeOpeningMatchesRepresentative opening h ∧
    P.fullNativeOpeningAssemblesRepresentative b payload cert opening h ∧
      InFiber P b.context b h ∧
        P.localValid b.context h ∧
          P.descentConsistent b.context h

/--
Remaining compatibility assumption after native assembly:

* the reviewed-Fq12 payload/certificate premises identify the active statement;
* the compatibility certificate is present and valid;
* the compatibility certificate has the field-level binding to the selected
  payload, extraction certificate, opening, and assembled representative; and
* the assembled representative is already known to be in the native fiber.

Under exactly those assumptions, the compatibility certificate may conclude
that all extraction predicates name that same representative, or it must export
a named boundary break.
-/
def SameRepresentativeFieldCompatibilitySoundness
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
              AssembledNativeFiberRepresentative P b payload cert opening h ->
                P.reviewedFq12HasExtractionCompatibilityCertificate
                    b payload cert opening h compatCert ->
                  P.reviewedFq12ExtractionCompatibilityCertificateValid
                      b payload cert opening h compatCert ->
                    ReviewedFq12ExtractionCompatibilityCertificateBinds
                        payload cert opening h compatCert ->
                      (P.reviewedFq12ExtractionCompatibilityCertificateExtracts
                          b payload cert opening h compatCert ∧
                        P.reviewedFq12CertificateExtracts
                          b payload cert h ∧
                        P.reviewedFq12PayloadExtracts b payload h) ∨
                      (exists breakEvent,
                        P.boundaryBreakFor b.context b breakEvent)

/--
Compatibility-certificate soundness reduces to the field-level compatibility
binding plus the post-assembly native-fiber representative premise.

The remaining assumption is named explicitly by
`SameRepresentativeFieldCompatibilitySoundness`; no aggregate full-opening
bridge availability or aggregate bridge binding is used here.
-/
theorem reviewed_fq12_extraction_compatibility_certificate_soundness
    (P : Predicates)
    (hSameRepresentative :
      SameRepresentativeFieldCompatibilitySoundness P) :
    ReviewedFq12ExtractionCompatibilityCertificateSoundness P := by
  intro b payload cert opening h compatCert
    hAccept hProjects hHasExtraction hExtractionValid hLifts
    hOpeningMatches hAssembles hFiber hLocal hDescent
    hHasCompat hCompatValid hCompatBinds
  exact
    hSameRepresentative hAccept hProjects hHasExtraction hExtractionValid
      hLifts
      ⟨hOpeningMatches, hAssembles, hFiber, hLocal, hDescent⟩
      hHasCompat hCompatValid hCompatBinds

end Path2CompatibilitySoundness

end Descender
