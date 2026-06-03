import Descender.Path2AssemblyNativeValidity
import Descender.Path2FullOpeningFieldEvidence

namespace Descender

/-!
# Path 2 Full-Opening Field Evidence Predicates

This module makes the Rust/Lean field-evidence bridge usable without changing
the core `Predicates` record.  The sidecar bridge names implementation-level
availability, validity, and binding of
`Path2ReviewedFq12FullOpeningFieldEvidenceV1`; from those facts it derives the
semantic bindings consumed by the existing P2FS-001 reductions.

The bridge is intentionally post-certificate and pre-aggregate.  It does not
claim native `InFiber`/`localValid`/`descentConsistent`, and it does not use the
aggregate full-opening bridge.
-/

namespace Path2FullOpeningFieldEvidencePredicates

open Path2NativeBoundary
open Path2FullOpeningFieldEvidence
open Path2CompatibilityConcrete

/--
Semantic binding facts recovered from one valid field-evidence frame.
-/
structure ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  roleLiftBindsExtraction :
    ReviewedFq12RoleLiftCertificateBindsExtraction
      payload cert liftCert
  assemblyBindsOpening :
    ReviewedFq12AssemblyCertificateBindsOpening
      payload cert liftCert.opening assemblyCert
  assemblyFieldConstruction :
    ReviewedFq12AssemblyFieldConstructionCertificate
      liftCert.opening assemblyCert
  compatibilityBindsDeterministicRepresentative :
    ReviewedFq12ExtractionCompatibilityCertificateBinds
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert
  sameRepresentativeConcreteFields :
    SameRepresentativeConcreteFieldCertificate
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert
  nonRootOpeningFieldsEqAssemblyOpening :
    FullNativeOpeningNonRootRoleFieldEqualityCertificate
      liftCert.opening assemblyCert.opening

/--
The explicit field-evidence binding predicate immediately supplies the
semantic binding facts used elsewhere.
-/
theorem semantic_bindings_of_full_opening_field_evidence_binds
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (hBinds :
      ReviewedFq12FullOpeningFieldEvidenceBinds
        payload cert liftCert assemblyCert compatCert fieldCert) :
    ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
      payload cert liftCert assemblyCert compatCert :=
  { roleLiftBindsExtraction :=
      role_lift_binds_extraction_of_full_opening_field_evidence hBinds
    assemblyBindsOpening :=
      assembly_binds_opening_of_full_opening_field_evidence hBinds
    assemblyFieldConstruction :=
      assembly_field_construction_of_full_opening_field_evidence hBinds
    compatibilityBindsDeterministicRepresentative :=
      compatibility_binds_deterministic_representative_of_full_opening_field_evidence
        hBinds
    sameRepresentativeConcreteFields :=
      same_representative_concrete_fields_of_full_opening_field_evidence
        hBinds
    nonRootOpeningFieldsEqAssemblyOpening :=
      nonroot_opening_fields_eq_assembly_opening_of_full_opening_field_evidence
        hBinds }

/--
Sidecar implementation bridge for the Rust field-evidence frame.

Keeping this outside `Predicates` avoids broad churn in existing proof
interfaces while still making the concrete Rust frame available to the active
P2FS-001 reductions.
-/
structure ReviewedFq12FullOpeningFieldEvidencePredicateBridge
    (P : Predicates) where
  hasFieldEvidence :
    Boundary ->
      ReviewedFq12HiddenSourcePayload ->
        ReviewedFq12HiddenRelationExtractionCertificate ->
          ReviewedFq12FullNativeOpeningCertificate ->
            ReviewedFq12NativeRepresentativeAssemblyCertificate ->
              ReviewedFq12ExtractionCompatibilityCertificate ->
                ReviewedFq12FullOpeningFieldEvidenceCertificate -> Prop
  fieldEvidenceValid :
    Boundary ->
      ReviewedFq12HiddenSourcePayload ->
        ReviewedFq12HiddenRelationExtractionCertificate ->
          ReviewedFq12FullNativeOpeningCertificate ->
            ReviewedFq12NativeRepresentativeAssemblyCertificate ->
              ReviewedFq12ExtractionCompatibilityCertificate ->
                ReviewedFq12FullOpeningFieldEvidenceCertificate -> Prop
  fieldEvidenceAvailable :
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
                        (exists fieldCert,
                          hasFieldEvidence
                              b payload cert liftCert assemblyCert
                              compatCert fieldCert ∧
                            fieldEvidenceValid
                              b payload cert liftCert assemblyCert
                              compatCert fieldCert) ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)
  fieldEvidenceBinds :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {liftCert : ReviewedFq12FullNativeOpeningCertificate}
        {assemblyCert :
          ReviewedFq12NativeRepresentativeAssemblyCertificate}
        {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
        {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate},
      hasFieldEvidence b payload cert liftCert assemblyCert compatCert
          fieldCert ->
        fieldEvidenceValid b payload cert liftCert assemblyCert compatCert
            fieldCert ->
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert

/--
A valid implementation field-evidence frame yields all semantic field bindings,
or the implementation must expose a boundary break.
-/
theorem full_opening_field_evidence_semantic_bindings_or_break
    (P : Predicates)
    (B : ReviewedFq12FullOpeningFieldEvidencePredicateBridge P)
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
    (exists fieldCert,
      B.hasFieldEvidence b payload cert liftCert assemblyCert compatCert
          fieldCert ∧
        B.fieldEvidenceValid b payload cert liftCert assemblyCert
          compatCert fieldCert ∧
        ReviewedFq12FullOpeningFieldEvidenceBinds
          payload cert liftCert assemblyCert compatCert fieldCert ∧
        ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
          payload cert liftCert assemblyCert compatCert) ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      B.fieldEvidenceAvailable hAccept hProjects hHasExtraction
        hExtractionValid hHasLift hLiftValid hHasAssembly hAssemblyValid
        hHasCompat hCompatValid with
    hField | hBreak
  · rcases hField with ⟨fieldCert, hHasField, hFieldValid⟩
    have hBinds := B.fieldEvidenceBinds hHasField hFieldValid
    exact Or.inl
      ⟨fieldCert, hHasField, hFieldValid, hBinds,
        semantic_bindings_of_full_opening_field_evidence_binds hBinds⟩
  · exact Or.inr hBreak

/--
Once a field-evidence frame is already present and valid, recover the semantic
bindings directly.
-/
theorem semantic_bindings_of_valid_predicate_field_evidence
    (P : Predicates)
    (B : ReviewedFq12FullOpeningFieldEvidencePredicateBridge P)
    {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {liftCert : ReviewedFq12FullNativeOpeningCertificate}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {compatCert : ReviewedFq12ExtractionCompatibilityCertificate}
    {fieldCert : ReviewedFq12FullOpeningFieldEvidenceCertificate}
    (hHasField :
      B.hasFieldEvidence b payload cert liftCert assemblyCert compatCert
        fieldCert)
    (hFieldValid :
      B.fieldEvidenceValid b payload cert liftCert assemblyCert compatCert
        fieldCert) :
    ReviewedFq12FullOpeningFieldEvidenceSemanticBindings
      payload cert liftCert assemblyCert compatCert :=
  semantic_bindings_of_full_opening_field_evidence_binds
    (B.fieldEvidenceBinds hHasField hFieldValid)

end Path2FullOpeningFieldEvidencePredicates

end Descender
