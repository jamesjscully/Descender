import Descender.Path2NativeBoundary

namespace Descender
namespace Path2NativeBoundary

/-!
# Path 2 Assembly Construction Boundary

This module isolates the non-aggregate construction step for the active Path 2
native / reviewed-Fq12 route.  The aggregate full-opening bridge and extraction
compatibility certificate are intentionally not used here: a valid assembly
frame first names the concrete hidden representative obtained from the opened
role coordinates and the assembly certificate's representative digest.
-/

/--
The hidden representative named directly by a native representative assembly
certificate.

The coordinates are the certificate's opened role-coordinate fields; the
representative digest is the certificate's own representative digest.  The
separate binding theorem below is what aligns these certificate fields with a
particular `FullNativeRepresentativeOpening`.
-/
def ReviewedFq12AssemblyConstructedRepresentative
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    HiddenRepresentative :=
  { nativeBoundaryDigest := assemblyCert.nativeBoundaryDigest
    representativeDigest := assemblyCert.representativeDigest
    localSection := assemblyCert.localSectionDigest
    packedResidues := assemblyCert.packedResiduesDigest
    randomizer := assemblyCert.randomizerDigest
    selectedAccountProjection :=
      assemblyCert.selectedAccountProjectionDigest
    padSource := assemblyCert.padSourceDigest
    openingTuple := assemblyCert.openingTupleDigest
    projectionRelation := assemblyCert.projectionRelationDigest
    lifecycleSlot := assemblyCert.lifecycleSlotDigest
    traceWitness := assemblyCert.traceWitnessDigest
    gauge := assemblyCert.gaugeDigest }

/--
Pure field-level certificate that the deterministic assembly representative is
the representative with the opened role coordinates and assembly digest.
-/
structure ReviewedFq12AssemblyFieldConstructionCertificate
    (opening : FullNativeRepresentativeOpening)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    Prop where
  matchesOpening :
    FullNativeOpeningMatchesRepresentative opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
  matchesAssemblyDigest :
    AssemblyCertificateMatchesRepresentative assemblyCert
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)

/--
The representative constructed from the assembly certificate always has the
certificate's representative digest.
-/
theorem reviewed_fq12_assembly_constructed_representative_matches_certificate
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    AssemblyCertificateMatchesRepresentative assemblyCert
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert) := by
  rfl

/--
If the assembly certificate is field-bound to an opening, the deterministic
representative constructed from the certificate has exactly the opening's role
coordinates.
-/
theorem reviewed_fq12_assembly_constructed_representative_matches_opening
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    (hBinds :
      ReviewedFq12AssemblyCertificateBindsOpening
        payload cert opening assemblyCert) :
    FullNativeOpeningMatchesRepresentative opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert) := by
  rcases hBinds with
    ⟨_hPayloadBinding, _hExtraction, _hCreationProof, _hNativePayload,
      _hNativeCert, _hSlotPayload, _hSlotCert, _hCreationPayload,
      _hCreationCert, _hReviewedPayload, _hReviewedCert, hNativeOpening,
      _hRoleRoot,
      _hFullOpening, hLocalSection, hPackedResidues, hRandomizer,
      hSelectedAccountProjection, hPadSource, hOpeningTuple,
      hProjectionRelation, hLifecycleSlot, hTraceWitness, hGauge,
      _hOpeningEq⟩
  exact
    ⟨hNativeOpening, hLocalSection, hPackedResidues, hRandomizer,
      hSelectedAccountProjection, hPadSource, hOpeningTuple,
      hProjectionRelation, hLifecycleSlot, hTraceWitness, hGauge⟩

/--
The existing bind contract is enough to produce the pure field-level
construction certificate for the deterministic representative.
-/
theorem reviewed_fq12_assembly_field_construction_from_binds
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    (hBinds :
      ReviewedFq12AssemblyCertificateBindsOpening
        payload cert opening assemblyCert) :
    ReviewedFq12AssemblyFieldConstructionCertificate opening assemblyCert :=
  { matchesOpening :=
      reviewed_fq12_assembly_constructed_representative_matches_opening
        hBinds
    matchesAssemblyDigest :=
      reviewed_fq12_assembly_constructed_representative_matches_certificate
        assemblyCert }

/--
The only predicate-level facts still needed after field construction: the
assembly frame and full-opening assembly relation both accept the deterministic
representative.
-/
def ReviewedFq12AssemblyConstructedRepresentativePredicates
    (P : Predicates)
    (b : Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (opening : FullNativeRepresentativeOpening)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    Prop :=
  P.reviewedFq12AssemblyCertificateAssembles
      b payload cert opening assemblyCert
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert) ∧
    P.fullNativeOpeningAssemblesRepresentative
      b payload cert opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)

/--
Single-certificate reduction for the non-aggregate construction obligation.

After the concrete field equalities in
`ReviewedFq12AssemblyCertificateBindsOpening`, callers only need to prove that
the two local assembly predicates hold for the deterministic representative
constructed from the assembly certificate, or export a boundary break.
-/
theorem reviewed_fq12_representative_assembly_construction_soundness_from_constructed_representative
    (P : Predicates)
    (hConstructs :
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
                          P.boundaryBreakFor b.context b breakEvent)) :
    ReviewedFq12RepresentativeAssemblyConstructionSoundness P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas hValid
    hLifts hHasAssembly hAssemblyValid hBinds
  rcases
      hConstructs hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid hBinds with
    hConstructed | hBreak
  · rcases hConstructed with
      ⟨hAssemblyCertAssembles, hOpeningAssembles⟩
    exact Or.inl
      ⟨ReviewedFq12AssemblyConstructedRepresentative assemblyCert,
        reviewed_fq12_assembly_constructed_representative_matches_opening
          hBinds,
        reviewed_fq12_assembly_constructed_representative_matches_certificate
          assemblyCert,
        hAssemblyCertAssembles,
        hOpeningAssembles⟩
  · exact Or.inr hBreak

end Path2NativeBoundary
end Descender
