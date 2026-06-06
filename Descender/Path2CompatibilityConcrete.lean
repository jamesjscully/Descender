import Descender.Path2CompatibilitySoundness
import Descender.Path2NativeBoundary

namespace Descender

/-!
# Path 2 Compatibility Concrete Fields

This module factors the remaining same-representative compatibility assumption
through explicit digest-field certificates.  It does not use the aggregate
full-opening bridge availability or aggregate bridge binding as a source.
-/

namespace Path2CompatibilityConcrete

open Path2NativeBoundary
open Path2CompatibilitySoundness

/--
Concrete payload/extraction-certificate fields that a compatibility certificate
must copy before it can claim to refer to the same reviewed-Fq12 extraction
instance.
-/
structure CompatibilityPayloadExtractionFieldCertificate
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  payloadBindingDigest_eq :
    compatCert.payloadBindingDigest = cert.payloadBindingDigest
  extractionCertificateDigest_eq :
    compatCert.extractionCertificateDigest =
      cert.extractionCertificateDigest
  nativeBoundaryDigest_payload_eq :
    compatCert.nativeBoundaryDigest = payload.nativeBoundaryDigest
  nativeBoundaryDigest_cert_eq :
    compatCert.nativeBoundaryDigest = cert.nativeBoundaryDigest
  slotId_payload_eq :
    compatCert.slotId = payload.slotId
  slotId_cert_eq :
    compatCert.slotId = cert.slotId
  creationStatementDigest_payload_eq :
    compatCert.creationStatementDigest =
      payload.creationStatementDigest
  creationStatementDigest_cert_eq :
    compatCert.creationStatementDigest =
      cert.creationStatementDigest
  reviewedStatementDigest_payload_eq :
    compatCert.reviewedStatementDigest = payload.statementDigest
  reviewedStatementDigest_cert_eq :
    compatCert.reviewedStatementDigest = cert.reviewedStatementDigest

/--
Concrete full-opening and selected-projection fields that connect the
compatibility certificate to the selected native opening and reviewed
extraction certificate.
-/
structure CompatibilityOpeningProjectionFieldCertificate
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  fullOpeningDigest_eq :
    compatCert.fullOpeningDigest = opening.openingDigest
  selectedProjection_opening_eq :
    compatCert.selectedAccountProjectionDigest =
      opening.selectedAccountProjectionDigest
  selectedProjection_cert_eq :
    compatCert.selectedAccountProjectionDigest =
      cert.accountProjectionDigest

/--
Concrete representative fields that pin the compatibility certificate to the
assembled representative digest and the full opening's role coordinates.
-/
structure CompatibilityRepresentativeDigestFieldCertificate
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  representativeDigest_eq :
    compatCert.representativeDigest = h.representativeDigest
  nativeBoundaryDigest_eq :
    compatCert.nativeBoundaryDigest = h.nativeBoundaryDigest
  openingMatchesRepresentative :
    FullNativeOpeningMatchesRepresentative opening h

/--
The complete concrete same-representative field certificate consumed by the
field-level extraction predicate rules below.
-/
structure SameRepresentativeConcreteFieldCertificate
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  payloadExtractionFields :
    CompatibilityPayloadExtractionFieldCertificate payload cert compatCert
  openingProjectionFields :
    CompatibilityOpeningProjectionFieldCertificate cert opening compatCert
  representativeDigestFields :
    CompatibilityRepresentativeDigestFieldCertificate opening h compatCert

/--
The existing compatibility binding predicate supplies exactly the concrete
field certificate needed by this module.
-/
theorem same_representative_concrete_field_certificate_of_binding
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {h : HiddenRepresentative}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hBind :
      ReviewedFq12ExtractionCompatibilityCertificateBinds
        payload cert opening h compatCert) :
    SameRepresentativeConcreteFieldCertificate
      payload cert opening h compatCert := by
  rcases hBind with
    ⟨hPayloadBinding, hExtractionDigest, hNativePayload, hNativeCert,
      hNativeOpening, hSlotPayload, hSlotCert, hCreationPayload, hCreationCert,
      hReviewedPayload, hReviewedCert, hFullOpening, hSelectedOpening,
      hSelectedCert, hRepresentative, hOpeningMatches⟩
  rcases hOpeningMatches with
    ⟨hRepresentativeNative, hOpeningMatchesTail⟩
  exact
    { payloadExtractionFields :=
        { payloadBindingDigest_eq := hPayloadBinding
          extractionCertificateDigest_eq := hExtractionDigest
          nativeBoundaryDigest_payload_eq := hNativePayload
          nativeBoundaryDigest_cert_eq := hNativeCert
          slotId_payload_eq := hSlotPayload
          slotId_cert_eq := hSlotCert
          creationStatementDigest_payload_eq := hCreationPayload
          creationStatementDigest_cert_eq := hCreationCert
          reviewedStatementDigest_payload_eq := hReviewedPayload
          reviewedStatementDigest_cert_eq := hReviewedCert }
      openingProjectionFields :=
        { fullOpeningDigest_eq := hFullOpening
          selectedProjection_opening_eq := hSelectedOpening
          selectedProjection_cert_eq := hSelectedCert }
      representativeDigestFields :=
        { representativeDigest_eq := hRepresentative
          nativeBoundaryDigest_eq :=
            hNativeOpening.trans hRepresentativeNative.symm
          openingMatchesRepresentative :=
            ⟨hRepresentativeNative, hOpeningMatchesTail⟩ } }

/--
Conversely, the concrete field certificate is precisely strong enough to
rebuild the existing compatibility binding predicate.
-/
theorem binding_of_same_representative_concrete_field_certificate
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {h : HiddenRepresentative}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hFields :
      SameRepresentativeConcreteFieldCertificate
        payload cert opening h compatCert) :
    ReviewedFq12ExtractionCompatibilityCertificateBinds
      payload cert opening h compatCert := by
  have hOpeningNative :
      h.nativeBoundaryDigest = opening.nativeBoundaryDigest :=
    hFields.representativeDigestFields.openingMatchesRepresentative.1
  exact
    ⟨hFields.payloadExtractionFields.payloadBindingDigest_eq,
      hFields.payloadExtractionFields.extractionCertificateDigest_eq,
      hFields.payloadExtractionFields.nativeBoundaryDigest_payload_eq,
      hFields.payloadExtractionFields.nativeBoundaryDigest_cert_eq,
      hFields.representativeDigestFields.nativeBoundaryDigest_eq.trans
        hOpeningNative,
      hFields.payloadExtractionFields.slotId_payload_eq,
      hFields.payloadExtractionFields.slotId_cert_eq,
      hFields.payloadExtractionFields.creationStatementDigest_payload_eq,
      hFields.payloadExtractionFields.creationStatementDigest_cert_eq,
      hFields.payloadExtractionFields.reviewedStatementDigest_payload_eq,
      hFields.payloadExtractionFields.reviewedStatementDigest_cert_eq,
      hFields.openingProjectionFields.fullOpeningDigest_eq,
      hFields.openingProjectionFields.selectedProjection_opening_eq,
      hFields.openingProjectionFields.selectedProjection_cert_eq,
      hFields.representativeDigestFields.representativeDigest_eq,
      hFields.representativeDigestFields.openingMatchesRepresentative⟩

/-- The concrete certificate exposes the representative digest equality. -/
theorem representative_digest_eq_of_concrete_field_certificate
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {h : HiddenRepresentative}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hFields :
      SameRepresentativeConcreteFieldCertificate
        payload cert opening h compatCert) :
    compatCert.representativeDigest = h.representativeDigest :=
  hFields.representativeDigestFields.representativeDigest_eq

/-- The concrete certificate exposes the selected projection equality. -/
theorem selected_projection_eq_extraction_certificate_of_concrete_fields
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {h : HiddenRepresentative}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    (hFields :
      SameRepresentativeConcreteFieldCertificate
        payload cert opening h compatCert) :
    compatCert.selectedAccountProjectionDigest =
      cert.accountProjectionDigest :=
  hFields.openingProjectionFields.selectedProjection_cert_eq

/--
Concrete field-level rules for the three extraction predicates needed by
`SameRepresentativeFieldCompatibilitySoundness`.

These are deliberately stated over the concrete digest certificate above.  The
rules may be implemented by the active verifier/extractor without appealing to
the aggregate full-opening bridge.
-/
structure SameRepresentativeConcreteExtractionPredicates
    (P : Predicates) :
    Prop where
  compatibilityCertificateExtracts :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {h : HiddenRepresentative}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
      P.reviewedFq12HasExtractionCompatibilityCertificate
          b payload cert opening h compatCert ->
        P.reviewedFq12ExtractionCompatibilityCertificateValid
            b payload cert opening h compatCert ->
          SameRepresentativeConcreteFieldCertificate
              payload cert opening h compatCert ->
            P.reviewedFq12ExtractionCompatibilityCertificateExtracts
              b payload cert opening h compatCert
  extractionCertificateExtracts :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {h : HiddenRepresentative}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
        P.reviewedFq12PayloadProjectsFrom b payload ->
          P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
            P.reviewedFq12ExtractionCertificateValid b payload cert ->
              P.reviewedFq12CertificateLiftsToOpening
                  b payload cert opening ->
                AssembledNativeFiberRepresentative
                    P b payload cert opening h ->
                  P.reviewedFq12HasExtractionCompatibilityCertificate
                      b payload cert opening h compatCert ->
                    P.reviewedFq12ExtractionCompatibilityCertificateValid
                        b payload cert opening h compatCert ->
                      SameRepresentativeConcreteFieldCertificate
                          payload cert opening h compatCert ->
                        P.reviewedFq12CertificateExtracts b payload cert h
  payloadExtracts :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {h : HiddenRepresentative}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
        P.reviewedFq12PayloadProjectsFrom b payload ->
          P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
            P.reviewedFq12ExtractionCertificateValid b payload cert ->
              P.reviewedFq12CertificateLiftsToOpening
                  b payload cert opening ->
                AssembledNativeFiberRepresentative
                    P b payload cert opening h ->
                  P.reviewedFq12HasExtractionCompatibilityCertificate
                      b payload cert opening h compatCert ->
                    P.reviewedFq12ExtractionCompatibilityCertificateValid
                        b payload cert opening h compatCert ->
                      SameRepresentativeConcreteFieldCertificate
                          payload cert opening h compatCert ->
                        P.reviewedFq12PayloadExtracts b payload h

/--
Explicit concrete field extraction rules discharge the same-representative
field compatibility assumption.
-/
theorem same_representative_field_compatibility_soundness_from_concrete_predicates
    (P : Predicates)
    (hConcrete :
      SameRepresentativeConcreteExtractionPredicates P) :
    SameRepresentativeFieldCompatibilitySoundness P := by
  intro b payload cert opening h compatCert
    hAccept hProjects hHasExtraction hExtractionValid hLifts
    hAssembled hHasCompat hCompatValid hCompatBinds
  have hFields :
      SameRepresentativeConcreteFieldCertificate
        payload cert opening h compatCert :=
    same_representative_concrete_field_certificate_of_binding hCompatBinds
  exact Or.inl
    ⟨hConcrete.compatibilityCertificateExtracts
        hHasCompat hCompatValid hFields,
      hConcrete.extractionCertificateExtracts
        hAccept hProjects hHasExtraction hExtractionValid hLifts
        hAssembled hHasCompat hCompatValid hFields,
      hConcrete.payloadExtracts
        hAccept hProjects hHasExtraction hExtractionValid hLifts
        hAssembled hHasCompat hCompatValid hFields⟩

end Path2CompatibilityConcrete

end Descender
