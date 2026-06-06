import Descender.Path2AssemblyConstruction
import Descender.Path2NativeBoundary

namespace Descender
namespace Path2NativeBoundary

/-!
# Path 2 Assembly Native Validity Boundary

This module isolates the native-validity half of P2FS-001 for the deterministic
representative constructed from a reviewed-Fq12 native assembly certificate.
-/

/--
The smallest native-validity certificate needed after deterministic assembly:
the constructed representative is in the native boundary fiber and satisfies
the local and descent predicates.
-/
structure ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
    (P : Predicates)
    (b : Boundary)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate) :
    Prop where
  inFiber :
    InFiber P b.context b
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
  localValid :
    P.localValid b.context
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
  descentConsistent :
    P.descentConsistent b.context
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)

/--
Expose the three native-validity facts carried by the constructed
representative certificate.
-/
theorem reviewed_fq12_assembly_constructed_representative_native_validity_facts
    {P : Predicates}
    {b : Boundary}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    (hNative :
      ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
        P b assemblyCert) :
    InFiber P b.context b
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert) ∧
      P.localValid b.context
        (ReviewedFq12AssemblyConstructedRepresentative assemblyCert) ∧
        P.descentConsistent b.context
          (ReviewedFq12AssemblyConstructedRepresentative assemblyCert) :=
  ⟨hNative.inFiber, hNative.localValid, hNative.descentConsistent⟩

/--
Any representative accepted by the assembled-native validity interface and
matching both the full opening and assembly representative digest is the
deterministic representative constructed from the assembly certificate.
-/
theorem reviewed_fq12_assembled_representative_eq_constructed
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {h : HiddenRepresentative}
    (hBinds :
      ReviewedFq12AssemblyCertificateBindsOpening
        payload cert opening assemblyCert)
    (hOpeningMatches : FullNativeOpeningMatchesRepresentative opening h)
    (hAssemblyMatches :
      AssemblyCertificateMatchesRepresentative assemblyCert h) :
    h = ReviewedFq12AssemblyConstructedRepresentative assemblyCert := by
  rcases hOpeningMatches with
    ⟨hNativeBoundary, hLocalSection, hPackedResidues, hRandomizer,
      hSelectedAccountProjection, hPadSource, hOpeningTuple,
      hProjectionRelation, hLifecycleSlot, hTraceWitness, hGauge⟩
  rcases hBinds with
    ⟨_hPayloadBinding, _hExtraction, _hCreationProof, _hNativePayload,
      _hNativeCert, _hSlotPayload, _hSlotCert, _hCreationPayload,
      _hCreationCert, _hReviewedPayload, _hReviewedCert, hAssemblyNativeOpening,
      _hRoleRoot,
      _hFullOpening, hAssemblyLocalSection, hAssemblyPackedResidues,
      hAssemblyRandomizer, hAssemblySelectedAccountProjection,
      hAssemblyPadSource, hAssemblyOpeningTuple,
      hAssemblyProjectionRelation, hAssemblyLifecycleSlot,
      hAssemblyTraceWitness, hAssemblyGauge, _hOpeningEq⟩
  cases h
  simp [ReviewedFq12AssemblyConstructedRepresentative, AssemblyCertificateMatchesRepresentative] at hAssemblyMatches hNativeBoundary hLocalSection hPackedResidues hRandomizer hSelectedAccountProjection hPadSource hOpeningTuple hProjectionRelation hLifecycleSlot hTraceWitness hGauge ⊢
  exact
    ⟨hNativeBoundary.trans hAssemblyNativeOpening.symm,
      hAssemblyMatches,
      hLocalSection.trans hAssemblyLocalSection.symm,
      hPackedResidues.trans hAssemblyPackedResidues.symm,
      hRandomizer.trans hAssemblyRandomizer.symm,
      hSelectedAccountProjection.trans
        hAssemblySelectedAccountProjection.symm,
      hPadSource.trans hAssemblyPadSource.symm,
      hOpeningTuple.trans hAssemblyOpeningTuple.symm,
      hProjectionRelation.trans hAssemblyProjectionRelation.symm,
      hLifecycleSlot.trans hAssemblyLifecycleSlot.symm,
      hTraceWitness.trans hAssemblyTraceWitness.symm,
      hGauge.trans hAssemblyGauge.symm⟩

/--
Native validity for any assembled representative follows from native validity
of the deterministic representative constructed from the assembly certificate.
-/
theorem reviewed_fq12_assembled_representative_native_validity_from_constructed
    {P : Predicates}
    {b : Boundary}
    {payload : ReviewedFq12HiddenSourcePayload}
    {cert : ReviewedFq12HiddenRelationExtractionCertificate}
    {opening : FullNativeRepresentativeOpening}
    {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate}
    {h : HiddenRepresentative}
    (hBinds :
      ReviewedFq12AssemblyCertificateBindsOpening
        payload cert opening assemblyCert)
    (hOpeningMatches : FullNativeOpeningMatchesRepresentative opening h)
    (hAssemblyMatches :
      AssemblyCertificateMatchesRepresentative assemblyCert h)
    (hNative :
      ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
        P b assemblyCert) :
    InFiber P b.context b h ∧
      P.localValid b.context h ∧
        P.descentConsistent b.context h := by
  have hEq :=
    reviewed_fq12_assembled_representative_eq_constructed
      hBinds hOpeningMatches hAssemblyMatches
  rw [hEq]
  exact
    reviewed_fq12_assembly_constructed_representative_native_validity_facts
      hNative

/--
Wrapper for the native-validity half of reviewed-Fq12 assembly soundness.

Callers only need to establish the three explicit native facts for the
deterministic representative constructed from the assembly certificate, or
export a boundary break.
-/
theorem reviewed_fq12_assembled_representative_native_validity_soundness_from_constructed
    (P : Predicates)
    (hNative :
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
                          P.boundaryBreakFor b.context b breakEvent)) :
    ReviewedFq12AssembledRepresentativeNativeValiditySoundness P := by
  intro b payload cert opening assemblyCert h hAccept hProjects hHas
    hValid hLifts hHasAssembly hAssemblyValid hBinds hOpeningMatches
    hAssemblyMatches _hAssemblyCertAssembles _hOpeningAssembles
  rcases
      hNative hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid hBinds with
    hConstructedNative | hBreak
  · exact Or.inl
      (reviewed_fq12_assembled_representative_native_validity_from_constructed
        hBinds hOpeningMatches hAssemblyMatches hConstructedNative)
  · exact Or.inr hBreak

end Path2NativeBoundary
end Descender
