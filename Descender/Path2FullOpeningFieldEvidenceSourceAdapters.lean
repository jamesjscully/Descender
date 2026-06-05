import Descender.Path2FullOpeningFieldEvidenceComposition
import Descender.Path2FullOpeningFieldEvidenceByteBridge
import Descender.Path2FullOpeningSourceConcrete
import Descender.Path2RoleRootConcrete

namespace Descender

/-!
# Path 2 Field-Evidence Source Adapters

This module connects the compatibility-indexed byte field-evidence source to
the concrete field certificates consumed by the source-concrete route.

The adapters remain compatibility-indexed on purpose: they require an already
available role-lift, assembly, and compatibility certificate chain.  They do
not turn the field-evidence object into generic pre-compatibility assembly
availability.
-/

namespace Path2FullOpeningFieldEvidenceSourceAdapters

open Path2NativeBoundary
open Path2FullOpeningFieldEvidence
open Path2FullOpeningFieldEvidenceByteBridge
open Path2FullOpeningFieldEvidenceComposition
open Path2FullOpeningSourceConcrete
open Path2CompatibilityConcrete

/--
Opening-to-role-lift witness needed to use source-route byte evidence in a
generic field-binding obligation.

The source-route byte adapter is keyed by the concrete role-lift certificate,
while `ReviewedFq12ConcreteAssemblyFieldBindingOrBreak` is keyed by the
semantic opening plus `reviewedFq12CertificateLiftsToOpening`. This source
names the missing implementation theorem: for the selected opening, recover the
valid role-lift certificate that carries exactly that opening, or export a
boundary break.
-/
def ReviewedFq12OpeningRoleLiftWitnessSource
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert
                opening ->
              (exists liftCert,
                P.reviewedFq12PayloadHasRoleLiftCertificate
                    b payload cert liftCert ∧
                  P.reviewedFq12RoleLiftCertificateValid
                    b payload cert liftCert ∧
                    liftCert.opening = opening) ∨
              (exists breakEvent,
                P.boundaryBreakFor b.context b breakEvent)

/--
Functionality of the selected certificate-to-opening lift relation.

`ReviewedFq12RoleRootBridgeCertificate` can recover a valid role-lift
certificate, and can prove that the opening carried by that certificate is a
valid lift.  To use that certificate for an already-given semantic opening,
the active model still needs this functionality theorem: the selected
certificate cannot lift to two different full native openings without a named
boundary break.
-/
def ReviewedFq12CertificateLiftOpeningFunctionality
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {left right : FullNativeRepresentativeOpening},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert left ->
              P.reviewedFq12CertificateLiftsToOpening b payload cert right ->
                left = right ∨
                  (exists breakEvent,
                    P.boundaryBreakFor b.context b breakEvent)

/--
Derive the opening-to-role-lift witness from the role-root bridge plus lift
functionality.

This replaces an opaque witness source with the sharper model theorem that the
selected certificate lift relation is functional.
-/
theorem opening_role_lift_witness_of_role_root_bridge_and_lift_functionality
    {P : Predicates}
    (Bridge : ReviewedFq12RoleRootBridgeCertificate P)
    (LiftFunctional : ReviewedFq12CertificateLiftOpeningFunctionality P) :
    ReviewedFq12OpeningRoleLiftWitnessSource P := by
  intro b payload cert opening hAccept hProjects hHas hValid hLifts
  rcases Bridge.roleLiftAvailability hAccept hProjects hHas hValid with
    hLift | hBreak
  · rcases hLift with ⟨liftCert, hHasLift, hLiftValid⟩
    have hBinds :
        ReviewedFq12RoleLiftCertificateBindsExtraction
          payload cert liftCert :=
      Bridge.roleLiftPayloadBinding hAccept hProjects hHas hValid
        hHasLift hLiftValid
    rcases
        Bridge.nativeRoleRootSoundness hAccept hProjects hHas hValid
          hHasLift hLiftValid hBinds with
      hOpens | hRootBreak
    · have hCarriedLifts :
          P.reviewedFq12CertificateLiftsToOpening
            b payload cert liftCert.opening :=
        Bridge.openedCertificateLifts hAccept hProjects hHas hValid
          hHasLift hLiftValid hBinds hOpens
      rcases
          LiftFunctional hAccept hProjects hHas hValid
            hCarriedLifts hLifts with
        hOpeningEq | hLiftBreak
      · exact Or.inl
          ⟨liftCert, hHasLift, hLiftValid, hOpeningEq⟩
      · exact Or.inr hLiftBreak
    · exact Or.inr hRootBreak
  · exact Or.inr hBreak

/--
Witness package needed to use source-route byte evidence for the assembly
field-binding predicate.

This is deliberately stronger than `ReviewedFq12OpeningRoleLiftWitnessSource`:
the active source-route byte frame is compatibility-indexed, so an assembly
field projection from that route also needs the compatibility certificate for
the constructed representative. This records the ordering constraint instead
of pretending that source-route evidence is a generic pre-compatibility
assembly source.
-/
def ReviewedFq12AssemblySourceRouteWitnessSource
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
                  (exists liftCert compatCert,
                    P.reviewedFq12PayloadHasRoleLiftCertificate
                        b payload cert liftCert ∧
                      P.reviewedFq12RoleLiftCertificateValid
                        b payload cert liftCert ∧
                        liftCert.opening = opening ∧
                          P.reviewedFq12HasExtractionCompatibilityCertificate
                            b payload cert liftCert.opening
                            (ReviewedFq12AssemblyConstructedRepresentative
                              assemblyCert)
                            compatCert ∧
                            P.reviewedFq12ExtractionCompatibilityCertificateValid
                              b payload cert liftCert.opening
                              (ReviewedFq12AssemblyConstructedRepresentative
                                assemblyCert)
                              compatCert) ∨
                    (exists breakEvent,
                      P.boundaryBreakFor b.context b breakEvent)

/--
Witness package needed to use source-route byte evidence for the generic
compatibility field-binding predicate.

The deployable compatibility obligation is stated over an arbitrary
representative `h`. The source-route byte certificate is stated over the
representative deterministically constructed by an assembly certificate. This
source names the implementation/model theorem that reconnects those views:
the semantic opening and representative in the compatibility premise must come
from a concrete role-lift and assembly certificate whose constructed
representative is exactly `h`, or a boundary break is exported.
-/
def ReviewedFq12CompatibilitySourceRouteWitnessSource
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
                            (exists liftCert assemblyCert,
                              P.reviewedFq12PayloadHasRoleLiftCertificate
                                  b payload cert liftCert ∧
                                P.reviewedFq12RoleLiftCertificateValid
                                  b payload cert liftCert ∧
                                  liftCert.opening = opening ∧
                                    P.reviewedFq12OpeningHasAssemblyCertificate
                                      b payload cert liftCert.opening
                                      assemblyCert ∧
                                      P.reviewedFq12AssemblyCertificateValid
                                        b payload cert liftCert.opening
                                        assemblyCert ∧
                                        ReviewedFq12AssemblyConstructedRepresentative
                                          assemblyCert = h) ∨
                              (exists breakEvent,
                                P.boundaryBreakFor b.context b breakEvent)

/--
Compatibility availability specialized to the deterministic representative
constructed by an already available assembly certificate.

This is the compatibility half of `ReviewedFq12AssemblySourceRouteWitnessSource`
with the role-lift lookup factored out. It remains post-assembly and
post-compatibility; it is not generic assembly availability.
-/
def ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource
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
                  (exists compatCert,
                    P.reviewedFq12HasExtractionCompatibilityCertificate
                      b payload cert opening
                      (ReviewedFq12AssemblyConstructedRepresentative
                        assemblyCert)
                      compatCert ∧
                      P.reviewedFq12ExtractionCompatibilityCertificateValid
                        b payload cert opening
                        (ReviewedFq12AssemblyConstructedRepresentative
                          assemblyCert)
                        compatCert) ∨
                    (exists breakEvent,
                      P.boundaryBreakFor b.context b breakEvent)

/--
Assembly representative lookup for a generic compatibility premise.

This is the assembly half of
`ReviewedFq12CompatibilitySourceRouteWitnessSource` with the role-lift lookup
factored out: the implementation/model must show that the representative in
the compatibility premise comes from an available deterministic assembly
certificate for the same opening.
-/
def ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource
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
                            (exists assemblyCert,
                              P.reviewedFq12OpeningHasAssemblyCertificate
                                b payload cert opening assemblyCert ∧
                                P.reviewedFq12AssemblyCertificateValid
                                  b payload cert opening assemblyCert ∧
                                  ReviewedFq12AssemblyConstructedRepresentative
                                    assemblyCert = h) ∨
                              (exists breakEvent,
                                P.boundaryBreakFor b.context b breakEvent)

/--
Representative-digest lookup needed to reconnect a generic compatibility
premise to the deterministic assembly representative.

`FullNativeOpeningMatchesRepresentative` deliberately names the opening role
coordinates but not the representative digest.  This source is the remaining
model theorem that an assembled/native-valid representative for the same
opening has the representative digest carried by the available assembly
certificate, or else exports a boundary break.
-/
def ReviewedFq12AssemblyRepresentativeDigestLookupSource
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {h : HiddenRepresentative}
      {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
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
                        P.reviewedFq12OpeningHasAssemblyCertificate
                            b payload cert opening assemblyCert ->
                          P.reviewedFq12AssemblyCertificateValid
                              b payload cert opening assemblyCert ->
                            ReviewedFq12AssemblyCertificateBindsOpening
                                payload cert opening assemblyCert ->
                              AssemblyCertificateMatchesRepresentative
                                  assemblyCert h ∨
                                (exists breakEvent,
                                  P.boundaryBreakFor b.context b breakEvent)

/--
Canonical representative-digest source for the selected implementation.

Rust owns the concrete hash/framing function.  The Lean obligation is not
injectivity of that hash, but that both sides of the active path use the same
canonical digest:

* the representative assembled from the full native opening carries the
  canonical digest for that boundary/opening tuple; and
* a valid assembly certificate that binds the same opening recomputes and
  carries that same canonical digest.

Instantiating this source with the selected Rust
`path2_reviewed_fq12_native_representative_digest_v1` frame is enough to derive
the generic representative-digest lookup above without assuming a
free-floating digest equality.
-/
structure ReviewedFq12RepresentativeCanonicalDigestSource
    (P : Predicates) where
  canonicalDigest :
    Boundary -> FullNativeRepresentativeOpening -> Nat
  representativeDigestCanonical :
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
                          h.representativeDigest =
                              canonicalDigest b opening ∨
                            (exists breakEvent,
                              P.boundaryBreakFor b.context b breakEvent)
  assemblyCertificateDigestCanonical :
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
                        payload cert opening assemblyCert ->
                      assemblyCert.representativeDigest =
                          canonicalDigest b opening ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)

/--
The canonical-digest source closes representative-digest lookup.

This is the proof-facing bridge for the Rust representative/canonicality
inspection: once the selected implementation is shown to recompute the same
canonical digest in representative assembly and in the assembly certificate,
the compatibility route can use `AssemblyCertificateMatchesRepresentative`
without a separate digest-equality assumption.
-/
theorem assembly_representative_digest_lookup_of_canonical_digest_source
    {P : Predicates}
    (canonicalSource :
      ReviewedFq12RepresentativeCanonicalDigestSource P) :
    ReviewedFq12AssemblyRepresentativeDigestLookupSource P := by
  intro b payload cert opening h assemblyCert hAccept hProjects hHas
    hValid hLifts hMatches hAssembles hFiber hLocal hDescent
    hHasAssembly hAssemblyValid hBinds
  rcases
      canonicalSource.representativeDigestCanonical hAccept hProjects
        hHas hValid hLifts hMatches hAssembles hFiber hLocal
        hDescent with
    hRepresentativeCanonical | hRepresentativeBreak
  · rcases
        canonicalSource.assemblyCertificateDigestCanonical hAccept
          hProjects hHas hValid hLifts hHasAssembly hAssemblyValid
          hBinds with
      hAssemblyCanonical | hAssemblyBreak
    · exact Or.inl
        (hRepresentativeCanonical.trans hAssemblyCanonical.symm)
    · exact Or.inr hAssemblyBreak
  · exact Or.inr hRepresentativeBreak

/--
Digest functionality for the full-native-opening assembly relation.

This is the uniqueness property that prevents two representatives with the
same opened role tuple, both accepted by the selected assembly relation and
native-local checks, from carrying different representative digests.
-/
def ReviewedFq12FullNativeOpeningRepresentativeDigestFunctionalitySource
    (P : Predicates) : Prop :=
  forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
      {cert : ReviewedFq12HiddenRelationExtractionCertificate}
      {opening : FullNativeRepresentativeOpening}
      {left right : HiddenRepresentative},
    P.reviewedFq12PayloadAccepts b payload ->
      P.reviewedFq12PayloadProjectsFrom b payload ->
        P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
          P.reviewedFq12ExtractionCertificateValid b payload cert ->
            P.reviewedFq12CertificateLiftsToOpening b payload cert opening ->
              FullNativeOpeningMatchesRepresentative opening left ->
                P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening left ->
                  InFiber P b.context b left ->
                    P.localValid b.context left ->
                      P.descentConsistent b.context left ->
                        FullNativeOpeningMatchesRepresentative opening right ->
                          P.fullNativeOpeningAssemblesRepresentative
                              b payload cert opening right ->
                            InFiber P b.context b right ->
                              P.localValid b.context right ->
                                P.descentConsistent b.context right ->
                                  left.representativeDigest =
                                      right.representativeDigest ∨
                                    (exists breakEvent,
                                      P.boundaryBreakFor
                                        b.context b breakEvent)

/--
Selected full-native-opening representative frame source.

This is a lower-level way to instantiate digest functionality: every
representative accepted by the selected full-native-opening assembly relation
and native-local checks carries the canonical representative digest for that
boundary/opening tuple.
-/
structure ReviewedFq12FullNativeOpeningRepresentativeFrameSource
    (P : Predicates) where
  canonicalDigest :
    Boundary -> FullNativeRepresentativeOpening -> Nat
  representativeDigestCanonical :
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
                          h.representativeDigest =
                              canonicalDigest b opening ∨
                            (exists breakEvent,
                              P.boundaryBreakFor b.context b breakEvent)

/--
Canonical representative obtained from a full native opening and a selected
representative-digest frame.

The role coordinates come from `FullNativeRepresentativeOpening`; only the
representative digest is supplied by the selected frame source.  This is the
concrete object the Rust/model path should expose for the
`fullNativeOpeningAssemblesRepresentative` predicate.
-/
def reviewedFq12FullNativeOpeningCanonicalRepresentative
    (canonicalDigest :
      Boundary -> FullNativeRepresentativeOpening -> Nat)
    (b : Boundary)
    (opening : FullNativeRepresentativeOpening) :
    HiddenRepresentative :=
  { representativeDigest := canonicalDigest b opening
    localSection := opening.localSectionDigest
    packedResidues := opening.packedResiduesDigest
    randomizer := opening.randomizerDigest
    selectedAccountProjection :=
      opening.selectedAccountProjectionDigest
    padSource := opening.padSourceDigest
    openingTuple := opening.openingTupleDigest
    projectionRelation := opening.projectionRelationDigest
    lifecycleSlot := opening.lifecycleSlotDigest
    traceWitness := opening.traceWitnessDigest
    gauge := opening.gaugeDigest }

/--
Selected full-native-opening canonical representative source.

This is lower-level than `ReviewedFq12FullNativeOpeningRepresentativeFrameSource`:
it states that the selected assembly predicate names the canonical
representative object for the boundary/opening tuple, or exports a boundary
break.
-/
structure ReviewedFq12FullNativeOpeningCanonicalRepresentativeSource
    (P : Predicates) where
  canonicalDigest :
    Boundary -> FullNativeRepresentativeOpening -> Nat
  assemblesCanonicalRepresentative :
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
                          h =
                              reviewedFq12FullNativeOpeningCanonicalRepresentative
                                canonicalDigest b opening ∨
                            (exists breakEvent,
                              P.boundaryBreakFor b.context b breakEvent)

/--
The canonical-representative source derives representative-frame canonicality.
-/
def representative_frame_source_of_canonical_representative_source
    {P : Predicates}
    (source :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeSource P) :
    ReviewedFq12FullNativeOpeningRepresentativeFrameSource P :=
  { canonicalDigest := source.canonicalDigest
    representativeDigestCanonical := by
      intro b payload cert opening h hAccept hProjects hHas hValid hLifts
        hMatches hAssembles hFiber hLocal hDescent
      rcases
          source.assemblesCanonicalRepresentative hAccept hProjects hHas
            hValid hLifts hMatches hAssembles hFiber hLocal hDescent with
        hEq | hBreak
      · rw [hEq]
        exact Or.inl rfl
      · exact Or.inr hBreak }

/--
Representative-frame canonicality plus the existing full-opening role-coordinate
match derives canonical representative construction.

This adapter is useful for Rust/model instantiation: if the implementation can
show that every accepted full-opening representative carries the selected
canonical digest, then `FullNativeOpeningMatchesRepresentative` supplies the
remaining role-coordinate fields needed to identify the whole canonical
representative object.
-/
def canonical_representative_source_of_representative_frame_source
    {P : Predicates}
    (source :
      ReviewedFq12FullNativeOpeningRepresentativeFrameSource P) :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeSource P :=
  { canonicalDigest := source.canonicalDigest
    assemblesCanonicalRepresentative := by
      intro b payload cert opening h hAccept hProjects hHas hValid hLifts
        hMatches hAssembles hFiber hLocal hDescent
      rcases
          source.representativeDigestCanonical hAccept hProjects hHas
            hValid hLifts hMatches hAssembles hFiber hLocal hDescent with
        hDigest | hBreak
      · rcases hMatches with
          ⟨hLocalSection, hPackedResidues, hRandomizer,
            hSelectedAccountProjection, hPadSource, hOpeningTuple,
            hProjectionRelation, hLifecycleSlot, hTraceWitness, hGauge⟩
        cases h
        simp [reviewedFq12FullNativeOpeningCanonicalRepresentative] at hDigest hLocalSection hPackedResidues hRandomizer hSelectedAccountProjection hPadSource hOpeningTuple hProjectionRelation hLifecycleSlot hTraceWitness hGauge ⊢
        exact
          Or.inl
            ⟨hDigest, hLocalSection, hPackedResidues, hRandomizer,
              hSelectedAccountProjection, hPadSource, hOpeningTuple,
              hProjectionRelation, hLifecycleSlot, hTraceWitness, hGauge⟩
      · exact Or.inr hBreak }

/--
A selected representative-frame source gives digest functionality for the
full-native-opening assembly relation.
-/
theorem representative_digest_functionality_of_representative_frame_source
    {P : Predicates}
    (frameSource :
      ReviewedFq12FullNativeOpeningRepresentativeFrameSource P) :
    ReviewedFq12FullNativeOpeningRepresentativeDigestFunctionalitySource P := by
  intro b payload cert opening left right hAccept hProjects hHas hValid
    hLifts hLeftMatches hLeftAssembles hLeftFiber hLeftLocal
    hLeftDescent hRightMatches hRightAssembles hRightFiber hRightLocal
    hRightDescent
  rcases
      frameSource.representativeDigestCanonical hAccept hProjects hHas
        hValid hLifts hLeftMatches hLeftAssembles hLeftFiber hLeftLocal
        hLeftDescent with
    hLeftCanonical | hLeftBreak
  · rcases
        frameSource.representativeDigestCanonical hAccept hProjects hHas
          hValid hLifts hRightMatches hRightAssembles hRightFiber
          hRightLocal hRightDescent with
      hRightCanonical | hRightBreak
    · exact Or.inl (hLeftCanonical.trans hRightCanonical.symm)
    · exact Or.inr hRightBreak
  · exact Or.inr hLeftBreak

/--
Selected Rust assembly-frame canonical digest source.

The concrete Rust frame is
`path2_reviewed_fq12_native_representative_digest_v1`: the Lean side keeps it
abstract as `canonicalDigest`, but requires every valid binding assembly
certificate to carry exactly that frame digest for the boundary/opening tuple.
-/
structure ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource
    (P : Predicates) where
  canonicalDigest :
    Boundary -> FullNativeRepresentativeOpening -> Nat
  assemblyCertificateDigestCanonical :
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
                        payload cert opening assemblyCert ->
                      assemblyCert.representativeDigest =
                          canonicalDigest b opening ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)

/--
Shared selected-Rust representative frame source.

This bundles the two active uses of
`path2_reviewed_fq12_native_representative_digest_v1` behind one
`canonicalDigest` function: representatives assembled from a full native opening
carry that digest, and valid binding assembly certificates recompute the same
digest.  It is the proof-facing shape that most directly matches the Rust
implementation.
-/
structure ReviewedFq12SelectedRustSharedRepresentativeFrameSource
    (P : Predicates) where
  canonicalDigest :
    Boundary -> FullNativeRepresentativeOpening -> Nat
  representativeDigestCanonical :
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
                          h.representativeDigest =
                              canonicalDigest b opening ∨
                            (exists breakEvent,
                              P.boundaryBreakFor b.context b breakEvent)
  assemblyCertificateDigestCanonical :
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
                        payload cert opening assemblyCert ->
                      assemblyCert.representativeDigest =
                          canonicalDigest b opening ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)

/--
The selected Rust native representative digest frame shape.

The argument order mirrors `path2_reviewed_fq12_native_representative_digest_v1`:
the active native boundary digest followed by the ten full-opening role digests.
-/
abbrev ReviewedFq12NativeRepresentativeDigestFrame :=
  Nat -> Nat -> Nat -> Nat -> Nat -> Nat -> Nat -> Nat -> Nat -> Nat -> Nat ->
    Nat

/-- Interpret the Rust-shaped digest frame as a boundary/opening digest. -/
def reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame)
    (b : Boundary)
    (opening : FullNativeRepresentativeOpening) : Nat :=
  digestFrame b.nativeBoundaryDigest opening.localSectionDigest
    opening.packedResiduesDigest opening.randomizerDigest
    opening.selectedAccountProjectionDigest opening.padSourceDigest
    opening.openingTupleDigest opening.projectionRelationDigest
    opening.lifecycleSlotDigest opening.traceWitnessDigest opening.gaugeDigest

/--
Lean mirror of a Rust byte-backed canonical full-opening representative
witness.

This object is about the representative assembled from the full native opening,
not the assembly-certificate-side digest-frame witness below.  It carries the
same selected digest frame and role coordinates needed to instantiate
`ReviewedFq12FullNativeOpeningCanonicalRepresentativeSource`.
-/
structure ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessCertificate where
  nativeBoundaryDigest : Nat
  nativeRoleRootCommitment : Nat
  fullOpeningDigest : Nat
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
  representativeDigest : Nat
  canonicalRepresentativeWitnessDigest : Nat

/-- Parser hooks for the Rust canonical full-opening representative witness. -/
structure ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser where
  parse :
    Nat ->
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessCertificate
  frameDigest :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessCertificate ->
      Nat
  frameBytesCanonical : Nat -> Prop
  domainSeparated :
    Nat ->
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessCertificate ->
        Prop

/--
Byte certificate for the canonical full-opening representative witness.

The representative object itself is still supplied by the semantic
`fullNativeOpeningAssemblesRepresentative` premise.  The byte certificate
provides the missing representative-digest equality against the selected Rust
digest frame; `FullNativeOpeningMatchesRepresentative` supplies the remaining
role-coordinate equalities.
-/
structure ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteCertificate
    (Parser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame)
    (b : Boundary)
    (opening : FullNativeRepresentativeOpening)
    (h : HiddenRepresentative)
    (frameBytes : Nat)
    (witness :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessCertificate) :
    Prop where
  canonical :
    Parser.frameBytesCanonical frameBytes
  parsed_eq :
    Parser.parse frameBytes = witness
  frame_digest_eq :
    Parser.frameDigest witness =
      witness.canonicalRepresentativeWitnessDigest
  domain_separated :
    Parser.domainSeparated frameBytes witness
  native_boundary_digest_eq :
    witness.nativeBoundaryDigest = b.nativeBoundaryDigest
  native_role_root_commitment_eq :
    witness.nativeRoleRootCommitment =
      opening.nativeRoleRootCommitment
  full_opening_digest_eq :
    witness.fullOpeningDigest = opening.openingDigest
  local_section_digest_eq :
    witness.localSectionDigest = opening.localSectionDigest
  packed_residues_digest_eq :
    witness.packedResiduesDigest = opening.packedResiduesDigest
  randomizer_digest_eq :
    witness.randomizerDigest = opening.randomizerDigest
  selected_account_projection_digest_eq :
    witness.selectedAccountProjectionDigest =
      opening.selectedAccountProjectionDigest
  pad_source_digest_eq :
    witness.padSourceDigest = opening.padSourceDigest
  opening_tuple_digest_eq :
    witness.openingTupleDigest = opening.openingTupleDigest
  projection_relation_digest_eq :
    witness.projectionRelationDigest =
      opening.projectionRelationDigest
  lifecycle_slot_digest_eq :
    witness.lifecycleSlotDigest = opening.lifecycleSlotDigest
  trace_witness_digest_eq :
    witness.traceWitnessDigest = opening.traceWitnessDigest
  gauge_digest_eq :
    witness.gaugeDigest = opening.gaugeDigest
  representative_digest_eq :
    h.representativeDigest =
      reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
        digestFrame b opening

/--
Selected-Rust byte source for the full-opening canonical representative.
-/
structure ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource
    (Parser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (P : Predicates) where
  digestFrame :
    ReviewedFq12NativeRepresentativeDigestFrame
  hasCanonicalRepresentativeWitness :
    Boundary ->
      ReviewedFq12HiddenSourcePayload ->
        ReviewedFq12HiddenRelationExtractionCertificate ->
          FullNativeRepresentativeOpening ->
            HiddenRepresentative ->
              Nat ->
                ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessCertificate ->
                  Prop
  canonicalRepresentativeWitnessAvailable :
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
                          (exists frameBytes witness,
                            hasCanonicalRepresentativeWitness
                                b payload cert opening h frameBytes
                                witness ∧
                              ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteCertificate
                                Parser digestFrame b opening h frameBytes
                                witness) ∨
                            (exists breakEvent,
                              P.boundaryBreakFor b.context b breakEvent)

/--
Rust-shaped source for the canonical representative witness.

The Rust constructor receives the concrete role-lift and assembly
certificates. This source keeps that shape, while the adapter below recovers
those certificates from the semantic opening using the active role-root,
assembly-availability, and direct assembly-binding sources.
-/
structure ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource
    (Parser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (P : Predicates) where
  digestFrame :
    ReviewedFq12NativeRepresentativeDigestFrame
  hasCanonicalRepresentativeWitnessFromCertificates :
    Boundary ->
      ReviewedFq12HiddenSourcePayload ->
        ReviewedFq12HiddenRelationExtractionCertificate ->
          ReviewedFq12FullNativeOpeningCertificate ->
            ReviewedFq12NativeRepresentativeAssemblyCertificate ->
              HiddenRepresentative ->
                Nat ->
                  ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessCertificate ->
                    Prop
  canonicalRepresentativeWitnessAvailableFromCertificates :
    forall {b : Boundary} {payload : ReviewedFq12HiddenSourcePayload}
        {cert : ReviewedFq12HiddenRelationExtractionCertificate}
        {opening : FullNativeRepresentativeOpening}
        {h : HiddenRepresentative}
        {liftCert : ReviewedFq12FullNativeOpeningCertificate}
        {assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate},
      P.reviewedFq12PayloadAccepts b payload ->
        P.reviewedFq12PayloadProjectsFrom b payload ->
          P.reviewedFq12PayloadHasExtractionCertificate b payload cert ->
            P.reviewedFq12ExtractionCertificateValid b payload cert ->
              P.reviewedFq12CertificateLiftsToOpening b payload cert
                  opening ->
                P.reviewedFq12PayloadHasRoleLiftCertificate
                    b payload cert liftCert ->
                  P.reviewedFq12RoleLiftCertificateValid
                      b payload cert liftCert ->
                    liftCert.opening = opening ->
                      P.reviewedFq12OpeningHasAssemblyCertificate
                          b payload cert opening assemblyCert ->
                        P.reviewedFq12AssemblyCertificateValid
                            b payload cert opening assemblyCert ->
                          ReviewedFq12AssemblyCertificateBindsOpening
                              payload cert opening assemblyCert ->
                            FullNativeOpeningMatchesRepresentative opening h ->
                              P.fullNativeOpeningAssemblesRepresentative
                                  b payload cert opening h ->
                                InFiber P b.context b h ->
                                  P.localValid b.context h ->
                                    P.descentConsistent b.context h ->
                                      (exists frameBytes witness,
                                        hasCanonicalRepresentativeWitnessFromCertificates
                                            b payload cert liftCert
                                            assemblyCert h frameBytes
                                            witness ∧
                                          ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteCertificate
                                            Parser digestFrame b opening h
                                            frameBytes witness) ∨
                                      (exists breakEvent,
                                        P.boundaryBreakFor b.context b
                                          breakEvent)

/--
The canonical-representative byte source instantiates the semantic canonical
representative source consumed by the active P2FS-001 route.
-/
def canonical_representative_source_of_canonical_representative_byte_source
    {Parser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource
        Parser P) :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeSource P :=
  { canonicalDigest :=
      reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
        Source.digestFrame
    assemblesCanonicalRepresentative := by
      intro b payload cert opening h hAccept hProjects hHas hValid hLifts
        hMatches hAssembles hFiber hLocal hDescent
      rcases
          Source.canonicalRepresentativeWitnessAvailable hAccept hProjects
            hHas hValid hLifts hMatches hAssembles hFiber hLocal hDescent with
        hWitness | hBreak
      · rcases hWitness with ⟨_frameBytes, _witness, _hHasWitness, hByte⟩
        have hDigest :
            h.representativeDigest =
              reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
                Source.digestFrame b opening :=
          hByte.representative_digest_eq
        rcases hMatches with
          ⟨hLocalSection, hPackedResidues, hRandomizer,
            hSelectedAccountProjection, hPadSource, hOpeningTuple,
            hProjectionRelation, hLifecycleSlot, hTraceWitness, hGauge⟩
        cases h
        simp [reviewedFq12FullNativeOpeningCanonicalRepresentative,
          reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest] at hDigest hLocalSection hPackedResidues hRandomizer hSelectedAccountProjection hPadSource hOpeningTuple hProjectionRelation hLifecycleSlot hTraceWitness hGauge ⊢
        exact
          Or.inl
            ⟨hDigest, hLocalSection, hPackedResidues, hRandomizer,
              hSelectedAccountProjection, hPadSource, hOpeningTuple,
              hProjectionRelation, hLifecycleSlot, hTraceWitness, hGauge⟩
      · exact Or.inr hBreak }

/--
Lean mirror of the Rust
`Path2ReviewedFq12NativeRepresentativeDigestFrameWitnessV1` object.

This is a byte-bound witness for the assembly-certificate side of
`path2_reviewed_fq12_native_representative_digest_v1`.  It does not claim
representative uniqueness; that remains the separate digest-functionality
source consumed by the active P2FS-001 wrapper.
-/
structure ReviewedFq12NativeRepresentativeDigestFrameWitnessCertificate where
  roleLiftCertificateDigest : Nat
  assemblyCertificateDigest : Nat
  nativeBoundaryDigest : Nat
  nativeRoleRootCommitment : Nat
  fullOpeningDigest : Nat
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
  representativeDigest : Nat
  digestFrameWitnessDigest : Nat

/--
Parser hooks for the Rust native representative digest-frame witness.
-/
structure ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser where
  parse : Nat -> ReviewedFq12NativeRepresentativeDigestFrameWitnessCertificate
  frameDigest :
    ReviewedFq12NativeRepresentativeDigestFrameWitnessCertificate -> Nat
  frameBytesCanonical : Nat -> Prop
  domainSeparated :
    Nat -> ReviewedFq12NativeRepresentativeDigestFrameWitnessCertificate ->
      Prop

/--
Byte certificate for the Rust digest-frame witness.

The equalities are intentionally only the data needed to discharge
`ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource`: the witness is
the parsed canonical frame, it is domain-separated, it binds the assembly
certificate digest, it binds the opening role digests, and it recomputes the
selected digest frame carried by the assembly certificate.
-/
structure ReviewedFq12NativeRepresentativeDigestFrameWitnessByteCertificate
    (Parser : ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (digestFrame : ReviewedFq12NativeRepresentativeDigestFrame)
    (b : Boundary)
    (opening : FullNativeRepresentativeOpening)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (frameBytes : Nat)
    (witness :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessCertificate) :
    Prop where
  canonical :
    Parser.frameBytesCanonical frameBytes
  parsed_eq :
    Parser.parse frameBytes = witness
  frame_digest_eq :
    Parser.frameDigest witness = witness.digestFrameWitnessDigest
  domain_separated :
    Parser.domainSeparated frameBytes witness
  role_lift_certificate_digest_eq :
    witness.roleLiftCertificateDigest =
      assemblyCert.roleLiftCertificateDigest
  assembly_certificate_digest_eq :
    witness.assemblyCertificateDigest =
      assemblyCert.assemblyCertificateDigest
  native_boundary_digest_eq :
    witness.nativeBoundaryDigest = b.nativeBoundaryDigest
  local_section_digest_eq :
    witness.localSectionDigest = opening.localSectionDigest
  packed_residues_digest_eq :
    witness.packedResiduesDigest = opening.packedResiduesDigest
  randomizer_digest_eq :
    witness.randomizerDigest = opening.randomizerDigest
  selected_account_projection_digest_eq :
    witness.selectedAccountProjectionDigest =
      opening.selectedAccountProjectionDigest
  pad_source_digest_eq :
    witness.padSourceDigest = opening.padSourceDigest
  opening_tuple_digest_eq :
    witness.openingTupleDigest = opening.openingTupleDigest
  projection_relation_digest_eq :
    witness.projectionRelationDigest = opening.projectionRelationDigest
  lifecycle_slot_digest_eq :
    witness.lifecycleSlotDigest = opening.lifecycleSlotDigest
  trace_witness_digest_eq :
    witness.traceWitnessDigest = opening.traceWitnessDigest
  gauge_digest_eq :
    witness.gaugeDigest = opening.gaugeDigest
  representative_digest_assembly_eq :
    witness.representativeDigest = assemblyCert.representativeDigest
  representative_digest_frame_eq :
    witness.representativeDigest =
      reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
        digestFrame b opening

/--
Selected-Rust byte source for the digest-frame witness.

This source is the Lean/Rust bridge for the typed Rust witness artifact: from a
valid binding assembly certificate, it produces the canonical parsed witness
or a named boundary break.
-/
structure ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
    (Parser : ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  digestFrame :
    ReviewedFq12NativeRepresentativeDigestFrame
  hasDigestFrameWitness :
    Boundary ->
      ReviewedFq12HiddenSourcePayload ->
        ReviewedFq12HiddenRelationExtractionCertificate ->
          FullNativeRepresentativeOpening ->
            ReviewedFq12NativeRepresentativeAssemblyCertificate ->
              Nat ->
                ReviewedFq12NativeRepresentativeDigestFrameWitnessCertificate ->
                  Prop
  digestFrameWitnessAvailable :
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
                        payload cert opening assemblyCert ->
                      (exists frameBytes witness,
                        hasDigestFrameWitness
                            b payload cert opening assemblyCert frameBytes
                            witness ∧
                          ReviewedFq12NativeRepresentativeDigestFrameWitnessByteCertificate
                            Parser digestFrame b opening assemblyCert
                            frameBytes witness) ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)

/--
The Rust digest-frame witness byte source instantiates the assembly-frame side
of the active representative digest route.
-/
def assembly_frame_source_of_digest_frame_witness_byte_source
    {Parser : ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
        Parser P) :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P :=
  { canonicalDigest :=
      reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
        Source.digestFrame
    assemblyCertificateDigestCanonical := by
      intro b payload cert opening assemblyCert hAccept hProjects hHas
        hValid hLifts hHasAssembly hAssemblyValid hBinds
      rcases
          Source.digestFrameWitnessAvailable hAccept hProjects hHas hValid
            hLifts hHasAssembly hAssemblyValid hBinds with
        hWitness | hBreak
      · rcases hWitness with
          ⟨_frameBytes, _witness, _hHasWitness, hByte⟩
        exact Or.inl
          (hByte.representative_digest_assembly_eq.symm.trans
            hByte.representative_digest_frame_eq)
      · exact Or.inr hBreak }

/--
Selected Rust native representative digest-frame source.

This is the concrete instantiation target for
`path2_reviewed_fq12_native_representative_digest_v1`: both accepted
representatives and binding assembly certificates carry the digest obtained by
applying the same 11-input frame to the native boundary/opening tuple.
-/
structure ReviewedFq12SelectedRustNativeRepresentativeDigestFrameSource
    (P : Predicates) where
  digestFrame :
    ReviewedFq12NativeRepresentativeDigestFrame
  representativeDigestCanonical :
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
                          h.representativeDigest =
                              reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
                                digestFrame b opening ∨
                            (exists breakEvent,
                              P.boundaryBreakFor b.context b breakEvent)
  assemblyCertificateDigestCanonical :
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
                        payload cert opening assemblyCert ->
                      assemblyCert.representativeDigest =
                          reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
                            digestFrame b opening ∨
                        (exists breakEvent,
                          P.boundaryBreakFor b.context b breakEvent)

/-- A Rust-shaped digest-frame source gives the shared representative frame. -/
def shared_representative_frame_source_of_native_digest_frame_source
    {P : Predicates}
    (source :
      ReviewedFq12SelectedRustNativeRepresentativeDigestFrameSource P) :
    ReviewedFq12SelectedRustSharedRepresentativeFrameSource P :=
  { canonicalDigest :=
      reviewedFq12NativeRepresentativeDigestFrameCanonicalDigest
        source.digestFrame
    representativeDigestCanonical :=
      source.representativeDigestCanonical
    assemblyCertificateDigestCanonical :=
      source.assemblyCertificateDigestCanonical }

/-- Forget the shared selected-Rust frame into the representative-frame source. -/
def representative_frame_source_of_shared_representative_frame_source
    {P : Predicates}
    (source :
      ReviewedFq12SelectedRustSharedRepresentativeFrameSource P) :
    ReviewedFq12FullNativeOpeningRepresentativeFrameSource P :=
  { canonicalDigest := source.canonicalDigest
    representativeDigestCanonical :=
      source.representativeDigestCanonical }

/-- Forget the shared selected-Rust frame into the assembly-frame source. -/
def assembly_frame_source_of_shared_representative_frame_source
    {P : Predicates}
    (source :
      ReviewedFq12SelectedRustSharedRepresentativeFrameSource P) :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P :=
  { canonicalDigest := source.canonicalDigest
    assemblyCertificateDigestCanonical :=
      source.assemblyCertificateDigestCanonical }

/-- A shared selected-Rust frame source gives the full canonical digest source. -/
def representative_canonical_digest_source_of_shared_representative_frame_source
    {P : Predicates}
    (source :
      ReviewedFq12SelectedRustSharedRepresentativeFrameSource P) :
    ReviewedFq12RepresentativeCanonicalDigestSource P :=
  { canonicalDigest := source.canonicalDigest
    representativeDigestCanonical :=
      source.representativeDigestCanonical
    assemblyCertificateDigestCanonical :=
      source.assemblyCertificateDigestCanonical }

/--
Assembly availability, direct assembly field binding, and representative-digest
lookup derive the generic compatibility-side assembly representative witness.
-/
theorem compatibility_assembly_representative_witness_of_assembly_availability_binding_and_digest_lookup
    {P : Predicates}
    (assemblyAvailability :
      ReviewedFq12RepresentativeAssemblyCertificateAvailability P)
    (assemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (digestLookup :
      ReviewedFq12AssemblyRepresentativeDigestLookupSource P) :
    ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource P := by
  intro b payload cert opening h compatCert hAccept hProjects hHas
    hValid hLifts hMatches hAssembles hFiber hLocal hDescent
    _hHasCompat _hCompatValid
  rcases
      assemblyAvailability hAccept hProjects hHas hValid hLifts with
    hAssembly | hAvailabilityBreak
  · rcases hAssembly with
      ⟨assemblyCert, hHasAssembly, hAssemblyValid⟩
    rcases
        assemblyBinding hAccept hProjects hHas hValid hLifts
          hHasAssembly hAssemblyValid with
      hFields | hBindingBreak
    · let hBinds :
          ReviewedFq12AssemblyCertificateBindsOpening
            payload cert opening assemblyCert :=
          binding_of_assembly_concrete_field_certificate hFields
      rcases
          digestLookup hAccept hProjects hHas hValid hLifts hMatches
            hAssembles hFiber hLocal hDescent hHasAssembly
            hAssemblyValid hBinds with
        hAssemblyMatches | hLookupBreak
      · have hEq :
          h = ReviewedFq12AssemblyConstructedRepresentative
            assemblyCert :=
          reviewed_fq12_assembled_representative_eq_constructed
            hBinds hMatches hAssemblyMatches
        exact Or.inl
          ⟨assemblyCert, hHasAssembly, hAssemblyValid, hEq.symm⟩
      · exact Or.inr hLookupBreak
    · exact Or.inr hBindingBreak
  · exact Or.inr hAvailabilityBreak

/--
Build the assembly source-route witness from separate role-lift and
constructed-representative compatibility witnesses.
-/
theorem assembly_source_route_witness_of_opening_and_constructed_compatibility_witness
    {P : Predicates}
    (OpeningWitness : ReviewedFq12OpeningRoleLiftWitnessSource P)
    (CompatibilityWitness :
      ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource P) :
    ReviewedFq12AssemblySourceRouteWitnessSource P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas
    hValid hLifts hHasAssembly hAssemblyValid
  rcases
      OpeningWitness hAccept hProjects hHas hValid hLifts with
    hLift | hBreak
  · rcases hLift with
      ⟨liftCert, hHasLift, hLiftValid, hOpeningEq⟩
    rcases
        CompatibilityWitness hAccept hProjects hHas hValid hLifts
          hHasAssembly hAssemblyValid with
      hCompat | hCompatBreak
    · rcases hCompat with
        ⟨compatCert, hHasCompat, hCompatValid⟩
      exact Or.inl
        ⟨liftCert, compatCert, hHasLift, hLiftValid, hOpeningEq,
          by simpa [hOpeningEq] using hHasCompat,
          by simpa [hOpeningEq] using hCompatValid⟩
    · exact Or.inr hCompatBreak
  · exact Or.inr hBreak

/--
Build the compatibility source-route witness from separate role-lift and
assembly-representative lookup witnesses.
-/
theorem compatibility_source_route_witness_of_opening_and_assembly_representative_witness
    {P : Predicates}
    (OpeningWitness : ReviewedFq12OpeningRoleLiftWitnessSource P)
    (AssemblyWitness :
      ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource P) :
    ReviewedFq12CompatibilitySourceRouteWitnessSource P := by
  intro b payload cert opening h compatCert hAccept hProjects hHas
    hValid hLifts hMatches hAssembles hFiber hLocal hDescent hHasCompat
    hCompatValid
  rcases
      OpeningWitness hAccept hProjects hHas hValid hLifts with
    hLift | hBreak
  · rcases hLift with
      ⟨liftCert, hHasLift, hLiftValid, hOpeningEq⟩
    rcases
        AssemblyWitness hAccept hProjects hHas hValid hLifts hMatches
          hAssembles hFiber hLocal hDescent hHasCompat hCompatValid with
      hAssembly | hAssemblyBreak
    · rcases hAssembly with
        ⟨assemblyCert, hHasAssembly, hAssemblyValid, hRepresentativeEq⟩
      exact Or.inl
        ⟨liftCert, assemblyCert, hHasLift, hLiftValid, hOpeningEq,
          by simpa [hOpeningEq] using hHasAssembly,
          by simpa [hOpeningEq] using hAssemblyValid,
          hRepresentativeEq⟩
    · exact Or.inr hAssemblyBreak
  · exact Or.inr hBreak

/--
Concrete field certificates recovered from one byte field-evidence frame for
the already-witnessed full-opening chain.
-/
structure ReviewedFq12FullOpeningFieldEvidenceConcreteFieldCertificates
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  assemblyFields :
    ReviewedFq12AssemblyConcreteFieldCertificate
      payload cert liftCert.opening assemblyCert
  compatibilityFields :
    SameRepresentativeConcreteFieldCertificate
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert

/--
Concrete source-shape certificates recovered for one already-witnessed
role-lift/assembly/compatibility chain.

This package is intentionally witnessed-chain scoped. It records what the
field-evidence adapter can safely supply after compatibility is already present,
without promoting those facts to generic assembly or compatibility sources.
-/
structure ReviewedFq12FullOpeningFieldEvidenceWitnessedConcreteSourceCertificates
    (P : Predicates)
    (b : Boundary)
    (payload : ReviewedFq12HiddenSourcePayload)
    (cert : ReviewedFq12HiddenRelationExtractionCertificate)
    (liftCert : ReviewedFq12FullNativeOpeningCertificate)
    (assemblyCert : ReviewedFq12NativeRepresentativeAssemblyCertificate)
    (compatCert : ReviewedFq12ExtractionCompatibilityCertificate) :
    Prop where
  activeFacts :
    ReviewedFq12FullOpeningFieldEvidenceActiveReductionFacts
      payload cert liftCert assemblyCert compatCert
  concreteFields :
    ReviewedFq12FullOpeningFieldEvidenceConcreteFieldCertificates
      payload cert liftCert assemblyCert compatCert
  localAssembly :
    ReviewedFq12AssemblyConstructedRepresentativeLocalCertificate
      P b payload cert liftCert.opening assemblyCert
  nativeValidity :
    ReviewedFq12AssemblyConstructedRepresentativeNativeValidityCertificate
      P b assemblyCert
  compatibilityExtraction :
    ReviewedFq12ConcreteCompatibilityExtractionCertificate
      P b payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert

/--
Byte field evidence supplies the concrete assembly and compatibility field
certificates for the witnessed chain, or names the byte/parser residual that
prevents using the evidence.
-/
theorem byte_source_concrete_field_certificates_or_named_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
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
    (exists frameBytes fieldCert,
      Source.hasByteFieldEvidence
          b payload cert liftCert assemblyCert compatCert frameBytes
          fieldCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          Parser frameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
            ReviewedFq12FullOpeningFieldEvidenceConcreteFieldCertificates
              payload cert liftCert assemblyCert compatCert) ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hBreak
  · rcases hField with
      ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds, facts⟩
    exact Or.inl
      ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds,
        { assemblyFields :=
            assembly_concrete_field_certificate_of_binding
              facts.assemblyPayloadBinding
          compatibilityFields := facts.sameRepresentativeConcreteFields }⟩
  · exact Or.inr hBreak

/--
Source-route byte evidence supplies the same concrete assembly and
compatibility field certificates, while also carrying the explicit route frame
that rules out aggregate-bridge sourcing.
-/
theorem source_route_byte_source_concrete_field_certificates_or_named_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
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
    (exists fieldFrameBytes routeFrameBytes fieldCert routeCert,
      Source.hasByteFieldSourceRoute
          b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          routeFrameBytes fieldCert routeCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          FieldParser fieldFrameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
            RouteParser routeFrameBytes fieldCert routeCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
            ReviewedFq12FullOpeningFieldSourceRouteBinds
              payload cert liftCert assemblyCert compatCert fieldCert
              routeCert ∧
              ReviewedFq12FullOpeningFieldEvidenceConcreteFieldCertificates
                payload cert liftCert assemblyCert compatCert) ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      source_route_byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hRoute | hBreak
  · rcases hRoute with
      ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
        hFieldByte, hRouteByte, hFieldBinds, hRouteBinds, facts⟩
    exact Or.inl
      ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
        hFieldByte, hRouteByte, hFieldBinds, hRouteBinds,
        { assemblyFields :=
            assembly_concrete_field_certificate_of_binding
              facts.assemblyPayloadBinding
          compatibilityFields := facts.sameRepresentativeConcreteFields }⟩
  · exact Or.inr hBreak

/--
Boundary-break version of the source-route concrete field-certificate adapter.

This keeps route byte/parser/domain residuals in the same ordinary
`exists breakEvent` shape consumed by later source packages, but remains
compatibility-indexed on the witnessed chain.
-/
theorem source_route_byte_source_concrete_field_certificates_or_boundary_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
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
    ReviewedFq12FullOpeningFieldEvidenceConcreteFieldCertificates
        payload cert liftCert assemblyCert compatCert ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      source_route_byte_source_concrete_field_certificates_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hRoute | hBreak
  · rcases hRoute with
      ⟨_fieldFrameBytes, _routeFrameBytes, _fieldCert, _routeCert,
        _hHasRoute, _hFieldByte, _hRouteByte, _hFieldBinds, _hRouteBinds,
        fields⟩
    exact Or.inl fields
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hBreak)

/--
Boundary-break version of the combined concrete field-certificate adapter.

This is the clean theorem-facing bridge for callers that want both concrete
field certificates from byte field evidence and do not care which byte/parser
residual caused failure.
-/
theorem byte_source_concrete_field_certificates_or_boundary_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
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
    ReviewedFq12FullOpeningFieldEvidenceConcreteFieldCertificates
        payload cert liftCert assemblyCert compatCert ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      byte_source_concrete_field_certificates_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hBreak
  · rcases hField with
      ⟨_frameBytes, _fieldCert, _hHasByte, _hByteCert, _hBinds,
        fields⟩
    exact Or.inl fields
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hBreak)

/--
Direct projection: source-route byte evidence supplies the concrete assembly
field certificate for the witnessed chain.
-/
theorem source_route_byte_source_assembly_concrete_fields_or_named_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
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
    ReviewedFq12AssemblyConcreteFieldCertificate
      payload cert liftCert.opening assemblyCert ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      source_route_byte_source_concrete_field_certificates_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hRoute | hBreak
  · rcases hRoute with
      ⟨_fieldFrameBytes, _routeFrameBytes, _fieldCert, _routeCert,
        _hHasRoute, _hFieldByte, _hRouteByte, _hFieldBinds, _hRouteBinds,
        fields⟩
    exact Or.inl fields.assemblyFields
  · exact Or.inr hBreak

/--
Boundary-break projection for assembly fields from source-route byte evidence.

This projection remains compatibility-indexed and must not be used as the
generic pre-compatibility assembly source.
-/
theorem source_route_byte_source_assembly_concrete_fields_or_boundary_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
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
    ReviewedFq12AssemblyConcreteFieldCertificate
      payload cert liftCert.opening assemblyCert ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      source_route_byte_source_assembly_concrete_fields_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hFields | hBreak
  · exact Or.inl hFields
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hBreak)

/--
Source-route byte evidence can instantiate the deployable assembly
field-binding input once the implementation supplies a witness connecting the
semantic opening in the generic obligation to the concrete role-lift
certificate consumed by the byte route.

This theorem is intentionally assembly-only. It does not promote the
compatibility-indexed source route to generic assembly availability, and it does
not solve compatibility binding for arbitrary representatives.
-/
theorem source_route_byte_source_assembly_field_binding_of_route_witness
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Witness : ReviewedFq12AssemblySourceRouteWitnessSource P)
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P) :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas
    hValid hLifts hHasAssembly hAssemblyValid
  rcases
      Witness hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid with
    hWitness | hBreak
  · rcases hWitness with
      ⟨liftCert, compatCert, hHasLift, hLiftValid, hOpeningEq,
        hHasCompat, hCompatValid⟩
    have hHasAssembly' :
        P.reviewedFq12OpeningHasAssemblyCertificate
          b payload cert liftCert.opening assemblyCert := by
      simpa [hOpeningEq] using hHasAssembly
    have hAssemblyValid' :
        P.reviewedFq12AssemblyCertificateValid
          b payload cert liftCert.opening assemblyCert := by
      simpa [hOpeningEq] using hAssemblyValid
    rcases
        source_route_byte_source_assembly_concrete_fields_or_boundary_break
          Source hAccept hProjects hHas hValid hHasLift hLiftValid
          hHasAssembly' hAssemblyValid' hHasCompat hCompatValid with
      hFields | hRouteBreak
    · exact Or.inl (by simpa [hOpeningEq] using hFields)
    · exact Or.inr hRouteBreak
  · exact Or.inr hBreak

/--
Direct projection: byte field evidence supplies the concrete assembly field
certificate for the witnessed chain.
-/
theorem byte_source_assembly_concrete_fields_or_named_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
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
    ReviewedFq12AssemblyConcreteFieldCertificate
      payload cert liftCert.opening assemblyCert ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      byte_source_concrete_field_certificates_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hBreak
  · rcases hField with
      ⟨_frameBytes, _fieldCert, _hHasByte, _hByteCert, _hBinds, fields⟩
    exact Or.inl fields.assemblyFields
  · exact Or.inr hBreak

/--
Boundary-break projection for assembly fields.

This is the form consumed by source-level field-binding obligations: byte
parser/domain residuals are converted into ordinary boundary breaks.
-/
theorem byte_source_assembly_concrete_fields_or_boundary_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
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
    ReviewedFq12AssemblyConcreteFieldCertificate
      payload cert liftCert.opening assemblyCert ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      byte_source_assembly_concrete_fields_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hFields | hBreak
  · exact Or.inl hFields
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hBreak)

/--
Direct projection: source-route byte evidence supplies the concrete
compatibility field certificate for the witnessed deterministic representative.
-/
theorem source_route_byte_source_compatibility_concrete_fields_or_named_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
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
    SameRepresentativeConcreteFieldCertificate
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert ∨
      FieldEvidenceByteBridgeBreakAlternatives P b := by
  rcases
      source_route_byte_source_concrete_field_certificates_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hRoute | hBreak
  · rcases hRoute with
      ⟨_fieldFrameBytes, _routeFrameBytes, _fieldCert, _routeCert,
        _hHasRoute, _hFieldByte, _hRouteByte, _hFieldBinds, _hRouteBinds,
        fields⟩
    exact Or.inl fields.compatibilityFields
  · exact Or.inr hBreak

/--
Boundary-break projection for compatibility fields from source-route byte
evidence.
-/
theorem source_route_byte_source_compatibility_concrete_fields_or_boundary_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
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
    SameRepresentativeConcreteFieldCertificate
      payload cert liftCert.opening
      (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
      compatCert ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      source_route_byte_source_compatibility_concrete_fields_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hFields | hBreak
  · exact Or.inl hFields
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hBreak)

/--
Source-route byte evidence can instantiate the deployable compatibility
field-binding input once the implementation supplies a witness connecting the
generic representative in the compatibility premise to the deterministic
representative constructed by a concrete assembly certificate.

This theorem records the remaining ordering dependency explicitly: the route
evidence is still compatibility-indexed and cannot act as a source of generic
compatibility availability.
-/
theorem source_route_byte_source_compatibility_field_binding_of_route_witness
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Witness : ReviewedFq12CompatibilitySourceRouteWitnessSource P)
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P) :
    ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak P := by
  intro b payload cert opening h compatCert hAccept hProjects hHas
    hValid hLifts hMatches hAssembles hFiber hLocal hDescent hHasCompat
    hCompatValid
  rcases
      Witness hAccept hProjects hHas hValid hLifts hMatches hAssembles
        hFiber hLocal hDescent hHasCompat hCompatValid with
    hWitness | hBreak
  · rcases hWitness with
      ⟨liftCert, assemblyCert, hHasLift, hLiftValid, hOpeningEq,
        hHasAssembly, hAssemblyValid, hRepresentativeEq⟩
    have hHasCompat' :
        P.reviewedFq12HasExtractionCompatibilityCertificate
          b payload cert liftCert.opening
          (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
          compatCert := by
      simpa [hOpeningEq, hRepresentativeEq] using hHasCompat
    have hCompatValid' :
        P.reviewedFq12ExtractionCompatibilityCertificateValid
          b payload cert liftCert.opening
          (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
          compatCert := by
      simpa [hOpeningEq, hRepresentativeEq] using hCompatValid
    rcases
        source_route_byte_source_compatibility_concrete_fields_or_boundary_break
          Source hAccept hProjects hHas hValid hHasLift hLiftValid
          hHasAssembly hAssemblyValid hHasCompat' hCompatValid' with
      hFields | hRouteBreak
    · exact Or.inl (by simpa [hOpeningEq, hRepresentativeEq] using hFields)
    · exact Or.inr hRouteBreak
  · exact Or.inr hBreak

/--
Source-route field-binding package.

This is the exact part of `ReviewedFq12DeployableHiddenSourceInputs` that the
compatibility-indexed byte route can provide once the implementation supplies
the required role-lift/assembly witnesses. It intentionally provides only field
bindings; availability, local/native validity, and extraction rules remain
separate non-route sources.
-/
structure ReviewedFq12SourceRouteFieldBindingInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  source :
    ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
      FieldParser RouteParser P
  assemblyWitness :
    ReviewedFq12AssemblySourceRouteWitnessSource P
  compatibilityWitness :
    ReviewedFq12CompatibilitySourceRouteWitnessSource P

/-- Assembly field binding supplied by the source-route field-binding package. -/
def ReviewedFq12SourceRouteFieldBindingInputs.toAssemblyFieldBinding
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SourceRouteFieldBindingInputs
        FieldParser RouteParser P) :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P :=
  source_route_byte_source_assembly_field_binding_of_route_witness
    I.assemblyWitness I.source

/-- Compatibility field binding supplied by the source-route package. -/
def ReviewedFq12SourceRouteFieldBindingInputs.toCompatibilityFieldBinding
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SourceRouteFieldBindingInputs
        FieldParser RouteParser P) :
    ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak P :=
  source_route_byte_source_compatibility_field_binding_of_route_witness
    I.compatibilityWitness I.source

/--
Source-route byte evidence plus local implementation facts can instantiate the
generic deterministic assembly-local input once the implementation supplies the
same assembly route witness used for field binding.

This is still post-compatibility: the witness carries a compatible extraction
certificate for the constructed representative, so the route is not being used
as generic assembly availability.
-/
theorem source_route_byte_source_assembly_local_predicates_of_route_witness
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Witness : ReviewedFq12AssemblySourceRouteWitnessSource P)
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P) :
    ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas
    hValid hLifts hHasAssembly hAssemblyValid _hFields
  rcases
      Witness hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid with
    hWitness | hBreak
  · rcases hWitness with
      ⟨liftCert, compatCert, hHasLift, hLiftValid, hOpeningEq,
        hHasCompat, hCompatValid⟩
    have hHasAssembly' :
        P.reviewedFq12OpeningHasAssemblyCertificate
          b payload cert liftCert.opening assemblyCert := by
      simpa [hOpeningEq] using hHasAssembly
    have hAssemblyValid' :
        P.reviewedFq12AssemblyCertificateValid
          b payload cert liftCert.opening assemblyCert := by
      simpa [hOpeningEq] using hAssemblyValid
    have hLifts' :
        P.reviewedFq12CertificateLiftsToOpening
          b payload cert liftCert.opening := by
      simpa [hOpeningEq] using hLifts
    rcases
        source_route_byte_source_local_assembly_predicates_or_break
          Source Local hAccept hProjects hHas hValid hHasLift hLiftValid
          hLifts' hHasAssembly' hAssemblyValid' hHasCompat hCompatValid with
      hPredicates | hRouteBreak
    · rcases hPredicates with
        ⟨_fieldFrameBytes, _routeFrameBytes, _fieldCert, _routeCert,
          _hHasRoute, _hFieldByte, _hRouteByte, _hFieldBinds,
          _hRouteBinds, _facts, predicates⟩
      let localCert :
          ReviewedFq12AssemblyConstructedRepresentativeLocalCertificate
            P b payload cert liftCert.opening assemblyCert :=
        local_certificate_of_assembly_constructed_predicates predicates
      exact Or.inl (by simpa [hOpeningEq] using localCert)
    · exact Or.inr hRouteBreak
  · exact Or.inr hBreak

/--
Source-route byte evidence plus local implementation facts can instantiate the
generic deterministic assembly-native-validity input once the implementation
supplies the same assembly route witness used for field binding.
-/
theorem source_route_byte_source_assembly_native_validity_of_route_witness
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Witness : ReviewedFq12AssemblySourceRouteWitnessSource P)
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P) :
    ReviewedFq12AssemblyConstructedNativeValidityOrBreak P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas
    hValid hLifts hHasAssembly hAssemblyValid _hFields
  rcases
      Witness hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid with
    hWitness | hBreak
  · rcases hWitness with
      ⟨liftCert, compatCert, hHasLift, hLiftValid, hOpeningEq,
        hHasCompat, hCompatValid⟩
    have hHasAssembly' :
        P.reviewedFq12OpeningHasAssemblyCertificate
          b payload cert liftCert.opening assemblyCert := by
      simpa [hOpeningEq] using hHasAssembly
    have hAssemblyValid' :
        P.reviewedFq12AssemblyCertificateValid
          b payload cert liftCert.opening assemblyCert := by
      simpa [hOpeningEq] using hAssemblyValid
    have hLifts' :
        P.reviewedFq12CertificateLiftsToOpening
          b payload cert liftCert.opening := by
      simpa [hOpeningEq] using hLifts
    rcases
        source_route_byte_source_local_native_validity_or_break
          Source Local hAccept hProjects hHas hValid hHasLift hLiftValid
          hLifts' hHasAssembly' hAssemblyValid' hHasCompat hCompatValid with
      hValidity | hRouteBreak
    · rcases hValidity with
        ⟨_fieldFrameBytes, _routeFrameBytes, _fieldCert, _routeCert,
          _hHasRoute, _hFieldByte, _hRouteByte, _hFieldBinds,
          _hRouteBinds, _facts, nativeValidity⟩
      exact Or.inl nativeValidity
    · exact Or.inr hRouteBreak
  · exact Or.inr hBreak

/-- Assembly local predicates supplied by the source-route package plus locals. -/
def ReviewedFq12SourceRouteFieldBindingInputs.toAssemblyLocalPredicates
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SourceRouteFieldBindingInputs
        FieldParser RouteParser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P) :
    ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak P :=
  source_route_byte_source_assembly_local_predicates_of_route_witness
    I.assemblyWitness I.source Local

/-- Assembly native validity supplied by the source-route package plus locals. -/
def ReviewedFq12SourceRouteFieldBindingInputs.toAssemblyNativeValidity
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SourceRouteFieldBindingInputs
        FieldParser RouteParser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P) :
    ReviewedFq12AssemblyConstructedNativeValidityOrBreak P :=
  source_route_byte_source_assembly_native_validity_of_route_witness
    I.assemblyWitness I.source Local

/--
Plain byte field evidence plus local implementation facts packages all
post-compatibility concrete source-shape certificates for the witnessed chain.

This is the adapter-facing bundle for callers that need the concrete field
certificates, named local assembly facts, native validity, and compatibility
extraction certificate together. It is still witnessed-chain scoped and does
not instantiate the generic assembly or compatibility source packages.
-/
theorem byte_source_witnessed_concrete_source_certificates_or_boundary_break
    {Parser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldEvidenceBytePredicateSource Parser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P)
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
    (hLifts :
      P.reviewedFq12CertificateLiftsToOpening
        b payload cert liftCert.opening)
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
    (exists frameBytes fieldCert,
      Source.hasByteFieldEvidence
          b payload cert liftCert assemblyCert compatCert frameBytes
          fieldCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          Parser frameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
            ReviewedFq12FullOpeningFieldEvidenceWitnessedConcreteSourceCertificates
              P b payload cert liftCert assemblyCert compatCert) ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hField | hByteBreak
  · rcases hField with
      ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds, facts⟩
    rcases
        Local.assemblyPredicates hAccept hProjects hHasExtraction
          hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
      assemblyPredicates | hBreak
    · rcases
          Local.nativeValidity hAccept hProjects hHasExtraction
            hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
        nativeValidity | hBreak
      · let extracts :=
          compatibility_extracts_of_field_evidence_local_facts
            P Local.extractionRules hAccept hProjects hHasExtraction
            hExtractionValid hLifts hHasCompat hCompatValid facts
            assemblyPredicates nativeValidity
        exact Or.inl
          ⟨frameBytes, fieldCert, hHasByte, hByteCert, hBinds,
            { activeFacts := facts
              concreteFields :=
                { assemblyFields :=
                    assembly_concrete_field_certificate_of_binding
                      facts.assemblyPayloadBinding
                  compatibilityFields :=
                    facts.sameRepresentativeConcreteFields }
              localAssembly :=
                local_certificate_of_assembly_constructed_predicates
                  assemblyPredicates
              nativeValidity := nativeValidity
              compatibilityExtraction :=
                { compatibilityCertificateExtracts :=
                    extracts.compatibilityCertificateExtracts
                  extractionCertificateExtracts :=
                    extracts.extractionCertificateExtracts
                  payloadExtracts := extracts.payloadExtracts } }⟩
      · exact Or.inr hBreak
    · exact Or.inr hBreak
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hByteBreak)

/--
Source-route byte evidence plus local implementation facts packages all
post-compatibility concrete source-shape certificates for the witnessed chain,
while preserving the explicit route frame that rules out aggregate sourcing.
-/
theorem source_route_byte_source_witnessed_concrete_source_certificates_or_boundary_break
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P)
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
    (hLifts :
      P.reviewedFq12CertificateLiftsToOpening
        b payload cert liftCert.opening)
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
    (exists fieldFrameBytes routeFrameBytes fieldCert routeCert,
      Source.hasByteFieldSourceRoute
          b payload cert liftCert assemblyCert compatCert fieldFrameBytes
          routeFrameBytes fieldCert routeCert ∧
        ReviewedFq12FullOpeningFieldEvidenceByteCertificate
          FieldParser fieldFrameBytes fieldCert ∧
          ReviewedFq12FullOpeningFieldSourceRouteByteCertificate
            RouteParser routeFrameBytes fieldCert routeCert ∧
          ReviewedFq12FullOpeningFieldEvidenceBinds
            payload cert liftCert assemblyCert compatCert fieldCert ∧
            ReviewedFq12FullOpeningFieldSourceRouteBinds
              payload cert liftCert assemblyCert compatCert fieldCert
              routeCert ∧
              ReviewedFq12FullOpeningFieldEvidenceWitnessedConcreteSourceCertificates
                P b payload cert liftCert assemblyCert compatCert) ∨
      (exists breakEvent, P.boundaryBreakFor b.context b breakEvent) := by
  rcases
      source_route_byte_predicate_source_active_reduction_facts_or_named_break
        Source hAccept hProjects hHasExtraction hExtractionValid hHasLift
        hLiftValid hHasAssembly hAssemblyValid hHasCompat hCompatValid with
    hRoute | hByteBreak
  · rcases hRoute with
      ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert, hHasRoute,
        hFieldByte, hRouteByte, hFieldBinds, hRouteBinds, facts⟩
    rcases
        Local.assemblyPredicates hAccept hProjects hHasExtraction
          hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
      assemblyPredicates | hBreak
    · rcases
          Local.nativeValidity hAccept hProjects hHasExtraction
            hExtractionValid hLifts hHasAssembly hAssemblyValid facts with
        nativeValidity | hBreak
      · let extracts :=
          compatibility_extracts_of_field_evidence_local_facts
            P Local.extractionRules hAccept hProjects hHasExtraction
            hExtractionValid hLifts hHasCompat hCompatValid facts
            assemblyPredicates nativeValidity
        exact Or.inl
          ⟨fieldFrameBytes, routeFrameBytes, fieldCert, routeCert,
            hHasRoute, hFieldByte, hRouteByte, hFieldBinds, hRouteBinds,
            { activeFacts := facts
              concreteFields :=
                { assemblyFields :=
                    assembly_concrete_field_certificate_of_binding
                      facts.assemblyPayloadBinding
                  compatibilityFields :=
                    facts.sameRepresentativeConcreteFields }
              localAssembly :=
                local_certificate_of_assembly_constructed_predicates
                  assemblyPredicates
              nativeValidity := nativeValidity
              compatibilityExtraction :=
                { compatibilityCertificateExtracts :=
                    extracts.compatibilityCertificateExtracts
                  extractionCertificateExtracts :=
                    extracts.extractionCertificateExtracts
                  payloadExtracts := extracts.payloadExtracts } }⟩
      · exact Or.inr hBreak
    · exact Or.inr hBreak
  · exact Or.inr
      (exists_boundary_break_of_field_evidence_byte_break hByteBreak)

/--
Source-route byte evidence plus local implementation facts can instantiate the
deployable compatibility-extraction input once the implementation supplies the
same route witness used for compatibility field binding.

The generic extraction premise already includes a same-representative field
certificate. This theorem does not use that premise as the source of fields:
the paired source-route byte frame re-derives the compatibility fields for the
witnessed deterministic representative, then the local extraction rules supply
the concrete extraction certificate.
-/
theorem source_route_byte_source_compatibility_extraction_of_route_witness
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (Witness : ReviewedFq12CompatibilitySourceRouteWitnessSource P)
    (Source :
      ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
        FieldParser RouteParser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P) :
    ReviewedFq12ConcreteCompatibilityExtractionOrBreak P := by
  intro b payload cert opening h compatCert hAccept hProjects hHas
    hValid hLifts hMatches hAssembles hFiber hLocal hDescent hHasCompat
    hCompatValid _hFields
  rcases
      Witness hAccept hProjects hHas hValid hLifts hMatches hAssembles
        hFiber hLocal hDescent hHasCompat hCompatValid with
    hWitness | hBreak
  · rcases hWitness with
      ⟨liftCert, assemblyCert, hHasLift, hLiftValid, hOpeningEq,
        hHasAssembly, hAssemblyValid, hRepresentativeEq⟩
    have hHasCompat' :
        P.reviewedFq12HasExtractionCompatibilityCertificate
          b payload cert liftCert.opening
          (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
          compatCert := by
      simpa [hOpeningEq, hRepresentativeEq] using hHasCompat
    have hCompatValid' :
        P.reviewedFq12ExtractionCompatibilityCertificateValid
          b payload cert liftCert.opening
          (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
          compatCert := by
      simpa [hOpeningEq, hRepresentativeEq] using hCompatValid
    have hLifts' :
        P.reviewedFq12CertificateLiftsToOpening
          b payload cert liftCert.opening := by
      simpa [hOpeningEq] using hLifts
    rcases
        source_route_byte_source_local_compatibility_extracts_or_break
          Source Local hAccept hProjects hHas hValid hHasLift hLiftValid
          hLifts' hHasAssembly hAssemblyValid hHasCompat' hCompatValid' with
      hExtracts | hRouteBreak
    · rcases hExtracts with
        ⟨_fieldFrameBytes, _routeFrameBytes, _fieldCert, _routeCert,
          _hHasRoute, _hFieldByte, _hRouteByte, _hFieldBinds,
          _hRouteBinds, _facts, extracts⟩
      let extractionCert :
          ReviewedFq12ConcreteCompatibilityExtractionCertificate
            P b payload cert liftCert.opening
            (ReviewedFq12AssemblyConstructedRepresentative assemblyCert)
            compatCert :=
        { compatibilityCertificateExtracts :=
            extracts.compatibilityCertificateExtracts
          extractionCertificateExtracts :=
            extracts.extractionCertificateExtracts
          payloadExtracts := extracts.payloadExtracts }
      have extractionCert' :
          ReviewedFq12ConcreteCompatibilityExtractionCertificate
            P b payload cert opening h compatCert := by
        simpa [hOpeningEq, hRepresentativeEq] using extractionCert
      exact Or.inl extractionCert'
    · exact Or.inr hRouteBreak
  · exact Or.inr hBreak

/-- Compatibility extraction supplied by the source-route package plus locals. -/
def ReviewedFq12SourceRouteFieldBindingInputs.toCompatibilityExtraction
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SourceRouteFieldBindingInputs
        FieldParser RouteParser P)
    (Local :
      ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P) :
    ReviewedFq12ConcreteCompatibilityExtractionOrBreak P :=
  source_route_byte_source_compatibility_extraction_of_route_witness
    I.compatibilityWitness I.source Local

/--
Deployable source-route input package for P2FS-001.

This record separates what the source route supplies from what must still come
from non-route verifier/model sources. The source route supplies assembly and
compatibility field bindings, and paired local evidence supplies deterministic
assembly local/native facts plus compatibility extraction. Availability remains
explicit. The aggregate bridge remains absent.
-/
structure ReviewedFq12SourceRouteDeployableHiddenSourceInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootBinding :
    ReviewedFq12NonAggregateRoleRootBindingCertificate P
  assemblyAvailability :
    ReviewedFq12AssemblyAvailabilityOrBreak P
  routeBindings :
    ReviewedFq12SourceRouteFieldBindingInputs
      FieldParser RouteParser P
  compatibilityAvailability :
    ReviewedFq12CompatibilityAvailabilityOrBreak P
  localPredicateSource :
    ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P

/-- Convert source-route deployable inputs to the active hidden-source package. -/
def ReviewedFq12SourceRouteDeployableHiddenSourceInputs.toDeployableInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SourceRouteDeployableHiddenSourceInputs
        FieldParser RouteParser P) :
    ReviewedFq12DeployableHiddenSourceInputs P :=
  { roleRootBinding := I.roleRootBinding
    assemblyAvailability := I.assemblyAvailability
    assemblyFieldBinding := I.routeBindings.toAssemblyFieldBinding
    assemblyLocalPredicates :=
      I.routeBindings.toAssemblyLocalPredicates I.localPredicateSource
    assemblyNativeValidity :=
      I.routeBindings.toAssemblyNativeValidity I.localPredicateSource
    compatibilityAvailability := I.compatibilityAvailability
    compatibilityFieldBinding := I.routeBindings.toCompatibilityFieldBinding
    compatibilityExtraction :=
      I.routeBindings.toCompatibilityExtraction I.localPredicateSource }

/--
P2FS-001 closure from the source-route deployable package. This theorem keeps
the route witness and the non-route sources visible while reusing the active
deployable endpoint.
-/
theorem reviewed_fq12_certificate_soundness_from_source_route_deployable_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SourceRouteDeployableHiddenSourceInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_deployable_hidden_source_inputs
    P I.toDeployableInputs

/--
Raw source-route/local instantiation inputs.

This is the last Lean-side package before the Rust verifier/model theorem:
callers must still provide the byte predicate source, local predicate source, and
route witnesses for the selected parser/frame/digest implementation. The record
only performs the non-circular packaging into the deployable P2FS-001 endpoint.
-/
structure ReviewedFq12SourceRouteLocalInstantiationInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootBinding :
    ReviewedFq12NonAggregateRoleRootBindingCertificate P
  assemblyAvailability :
    ReviewedFq12AssemblyAvailabilityOrBreak P
  source :
    ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
      FieldParser RouteParser P
  assemblyWitness :
    ReviewedFq12AssemblySourceRouteWitnessSource P
  compatibilityWitness :
    ReviewedFq12CompatibilitySourceRouteWitnessSource P
  compatibilityAvailability :
    ReviewedFq12CompatibilityAvailabilityOrBreak P
  localPredicateSource :
    ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P

/-- Extract the field-binding package from raw source-route instantiation inputs. -/
def ReviewedFq12SourceRouteLocalInstantiationInputs.toRouteBindings
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SourceRouteLocalInstantiationInputs
        FieldParser RouteParser P) :
    ReviewedFq12SourceRouteFieldBindingInputs FieldParser RouteParser P :=
  { source := I.source
    assemblyWitness := I.assemblyWitness
    compatibilityWitness := I.compatibilityWitness }

/-- Package raw source-route/local inputs into the deployable endpoint. -/
def ReviewedFq12SourceRouteLocalInstantiationInputs.toDeployableInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SourceRouteLocalInstantiationInputs
        FieldParser RouteParser P) :
    ReviewedFq12SourceRouteDeployableHiddenSourceInputs
      FieldParser RouteParser P :=
  { roleRootBinding := I.roleRootBinding
    assemblyAvailability := I.assemblyAvailability
    routeBindings := I.toRouteBindings
    compatibilityAvailability := I.compatibilityAvailability
    localPredicateSource := I.localPredicateSource }

/--
P2FS-001 closure from raw source-route/local instantiation inputs. The remaining
external obligation is exactly to instantiate this record for the Rust
parser/frame/digest implementation.
-/
theorem reviewed_fq12_certificate_soundness_from_source_route_local_instantiation_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SourceRouteLocalInstantiationInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_source_route_deployable_inputs
    P I.toDeployableInputs

/--
Concrete role-root/source-route instantiation inputs.

This is the next narrower active P2FS-001 shape for the Rust verifier/model:
role-root binding is supplied by the field-level non-aggregate package from
`Path2RoleRootConcrete`, while the source-route/local adapters still derive
field binding, deterministic assembly local/native facts, and compatibility
extraction. Assembly and compatibility availability remain explicit non-route
sources.
-/
structure ReviewedFq12ConcreteRoleRootSourceRouteLocalInstantiationInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootFields :
    ReviewedFq12ConcreteRoleRootFieldBindingCertificate P
  assemblyAvailability :
    ReviewedFq12AssemblyAvailabilityOrBreak P
  source :
    ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
      FieldParser RouteParser P
  assemblyWitness :
    ReviewedFq12AssemblySourceRouteWitnessSource P
  compatibilityWitness :
    ReviewedFq12CompatibilitySourceRouteWitnessSource P
  compatibilityAvailability :
    ReviewedFq12CompatibilityAvailabilityOrBreak P
  localPredicateSource :
    ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P

/--
Convert concrete role-root/source-route inputs to the source-route local
instantiation package.
-/
def ReviewedFq12ConcreteRoleRootSourceRouteLocalInstantiationInputs.toInstantiationInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12ConcreteRoleRootSourceRouteLocalInstantiationInputs
        FieldParser RouteParser P) :
    ReviewedFq12SourceRouteLocalInstantiationInputs
      FieldParser RouteParser P :=
  { roleRootBinding :=
      reviewed_fq12_nonaggregate_role_root_binding_from_concrete_fields
        P I.roleRootFields
    assemblyAvailability := I.assemblyAvailability
    source := I.source
    assemblyWitness := I.assemblyWitness
    compatibilityWitness := I.compatibilityWitness
    compatibilityAvailability := I.compatibilityAvailability
    localPredicateSource := I.localPredicateSource }

/--
P2FS-001 closure from concrete role-root plus source-route/local
instantiation inputs.
-/
theorem reviewed_fq12_certificate_soundness_from_concrete_role_root_source_route_local_instantiation_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12ConcreteRoleRootSourceRouteLocalInstantiationInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_source_route_local_instantiation_inputs
    P I.toInstantiationInputs

/--
Boundary-availability/concrete-role-root source-route inputs.

This boundary-availability wrapper consumes the base active boundary
availability surfaces from `Path2NativeBoundary`, concrete role-root field
binding from `Path2RoleRootConcrete`, and the source-route byte plus local
predicate witnesses. The named availability records are recovered by
equivalence, not by an additional cryptographic assumption.
-/
structure ReviewedFq12BoundaryAvailabilityConcreteRoleRootSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootFields :
    ReviewedFq12ConcreteRoleRootFieldBindingCertificate P
  assemblyAvailability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  source :
    ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
      FieldParser RouteParser P
  assemblyWitness :
    ReviewedFq12AssemblySourceRouteWitnessSource P
  compatibilityWitness :
    ReviewedFq12CompatibilitySourceRouteWitnessSource P
  compatibilityAvailability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  localPredicateSource :
    ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P

/--
Convert base boundary-availability/concrete-role-root source-route inputs into
the current concrete role-root source-route package.
-/
def ReviewedFq12BoundaryAvailabilityConcreteRoleRootSourceRouteInputs.toConcreteRoleRootInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12BoundaryAvailabilityConcreteRoleRootSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ConcreteRoleRootSourceRouteLocalInstantiationInputs
      FieldParser RouteParser P :=
  { roleRootFields := I.roleRootFields
    assemblyAvailability :=
      named_assembly_availability_of_source P I.assemblyAvailability
    source := I.source
    assemblyWitness := I.assemblyWitness
    compatibilityWitness := I.compatibilityWitness
    compatibilityAvailability :=
      named_compatibility_availability_of_source
        P I.compatibilityAvailability
    localPredicateSource := I.localPredicateSource }

/--
P2FS-001 closure from base active-boundary availability, concrete role-root
field binding, and source-route/local implementation evidence.
-/
theorem reviewed_fq12_certificate_soundness_from_boundary_availability_concrete_role_root_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12BoundaryAvailabilityConcreteRoleRootSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_concrete_role_root_source_route_local_instantiation_inputs
    P I.toConcreteRoleRootInputs

/--
Boundary-availability/concrete-role-root source-route inputs with factored
route witnesses.

Compared with
`ReviewedFq12BoundaryAvailabilityConcreteRoleRootSourceRouteInputs`, this
record splits the broad route witnesses into:

* an opening-to-role-lift witness;
* compatibility availability for the deterministic assembly representative;
* assembly lookup for the representative appearing in a compatibility premise.

This is easier to instantiate and audit against the Rust verifier/model.
-/
structure ReviewedFq12FactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootFields :
    ReviewedFq12ConcreteRoleRootFieldBindingCertificate P
  assemblyAvailability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  source :
    ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource
      FieldParser RouteParser P
  openingWitness :
    ReviewedFq12OpeningRoleLiftWitnessSource P
  constructedCompatibilityWitness :
    ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource P
  compatibilityAssemblyWitness :
    ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource P
  compatibilityAvailability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  localPredicateSource :
    ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P

/--
Convert factored route witnesses to the current narrow source-route package.
-/
def ReviewedFq12FactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs.toBoundaryInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12FactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12BoundaryAvailabilityConcreteRoleRootSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootFields := I.roleRootFields
    assemblyAvailability := I.assemblyAvailability
    source := I.source
    assemblyWitness :=
      assembly_source_route_witness_of_opening_and_constructed_compatibility_witness
        I.openingWitness I.constructedCompatibilityWitness
    compatibilityWitness :=
      compatibility_source_route_witness_of_opening_and_assembly_representative_witness
        I.openingWitness I.compatibilityAssemblyWitness
    compatibilityAvailability := I.compatibilityAvailability
    localPredicateSource := I.localPredicateSource }

/--
P2FS-001 closure from the factored source-route witness package.
-/
theorem reviewed_fq12_certificate_soundness_from_factored_boundary_availability_concrete_role_root_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12FactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_boundary_availability_concrete_role_root_source_route_inputs
    P I.toBoundaryInputs

/--
Selected-Rust/factored source-route inputs for the active P2FS-001 endpoint.

This is the Rust verifier/model-facing shape after the byte layer has been
selected: instead of accepting an arbitrary source-route byte predicate
source, it consumes a
`SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource`, whose
contract decomposes the chosen Rust field-evidence/source-route frames into
byte certificates and semantic binding predicates.  The remaining fields are
the non-byte model obligations that are still intentionally visible in the
active path.
-/
structure ReviewedFq12SelectedRustFactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootFields :
    ReviewedFq12ConcreteRoleRootFieldBindingCertificate P
  assemblyAvailability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  openingWitness :
    ReviewedFq12OpeningRoleLiftWitnessSource P
  constructedCompatibilityWitness :
    ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource P
  compatibilityAssemblyWitness :
    ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource P
  compatibilityAvailability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  localPredicateSource :
    ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P

/--
Forget the selected-Rust byte source into the factored AP2-N001 package.
-/
def ReviewedFq12SelectedRustFactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs.toFactoredInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustFactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12FactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootFields := I.roleRootFields
    assemblyAvailability := I.assemblyAvailability
    source := I.selectedSource.toBytePredicateSource
    openingWitness := I.openingWitness
    constructedCompatibilityWitness := I.constructedCompatibilityWitness
    compatibilityAssemblyWitness := I.compatibilityAssemblyWitness
    compatibilityAvailability := I.compatibilityAvailability
    localPredicateSource := I.localPredicateSource }

/--
P2FS-001 closure from the selected-Rust source-route byte-source package.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_factored_boundary_availability_concrete_role_root_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustFactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_factored_boundary_availability_concrete_role_root_source_route_inputs
    P I.toFactoredInputs

/--
Compact assembly source for the selected-Rust route.

The source-route byte frame supplies assembly field binding, so this package
only carries assembly availability plus the deterministic local/native facts
needed after a concrete assembly certificate has already been field-bound.
-/
structure ReviewedFq12AssemblyAvailabilityLocalSource
    (P : Predicates) where
  availability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  localPredicates :
    ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak P
  nativeValidity :
    ReviewedFq12AssemblyConstructedNativeValidityOrBreak P

/--
Compact compatibility source for the selected-Rust route.

The source-route byte frame supplies same-representative field binding, so this
package carries compatibility availability plus the concrete extraction rules.
-/
structure ReviewedFq12CompatibilityAvailabilityExtractionRulesSource
    (P : Predicates) where
  availability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  extractionRules :
    SameRepresentativeConcreteExtractionPredicates P

/--
Direct assembly field binding plus the compact local model derives the
constructed-representative compatibility witness.

This adapter is intentionally not used to make selected source-route bytes
pre-compatibility evidence.  It shows the non-circular route available when a
model supplies assembly field binding independently of the compatibility-indexed
source-route frame: field binding gives the constructed representative's
opening match, the assembly local/native sources give assembled native-fiber
facts, and compatibility availability supplies the compatibility certificate.
-/
theorem constructed_compatibility_witness_of_assembly_field_binding_and_local_model
    {P : Predicates}
    (assemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (assemblySource : ReviewedFq12AssemblyAvailabilityLocalSource P)
    (compatibilitySource :
      ReviewedFq12CompatibilityAvailabilityExtractionRulesSource P) :
    ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas
    hValid hLifts hHasAssembly hAssemblyValid
  rcases
      assemblyBinding hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid with
    hFields | hBindingBreak
  · let hBinds :
        ReviewedFq12AssemblyCertificateBindsOpening
          payload cert opening assemblyCert :=
        binding_of_assembly_concrete_field_certificate hFields
    rcases
        assemblySource.localPredicates
          hAccept hProjects hHas hValid hLifts hHasAssembly
          hAssemblyValid hBinds with
      hLocal | hLocalBreak
    · rcases
        assemblySource.nativeValidity
          hAccept hProjects hHas hValid hLifts hHasAssembly
          hAssemblyValid hBinds with
      hNative | hNativeBreak
      · let hMatches :
          FullNativeOpeningMatchesRepresentative opening
            (ReviewedFq12AssemblyConstructedRepresentative assemblyCert) :=
          reviewed_fq12_assembly_constructed_representative_matches_opening
            hBinds
        rcases
            compatibilitySource.availability
              hAccept hProjects hHas hValid hLifts hMatches
              hLocal.fullOpeningAssembles hNative.inFiber hNative.localValid
              hNative.descentConsistent with
          hCompat | hCompatBreak
        · rcases hCompat with ⟨compatCert, hHasCompat, hCompatValid⟩
          exact Or.inl ⟨compatCert, hHasCompat, hCompatValid⟩
        · exact Or.inr hCompatBreak
      · exact Or.inr hNativeBreak
    · exact Or.inr hLocalBreak
  · exact Or.inr hBindingBreak

/--
Derive the compatibility-indexed local predicate source from the compact local
source records.  The field binding premise comes from the active reduction
facts recovered from the selected Rust field/source-route frame.
-/
def local_predicate_source_of_availability_local_sources
    (P : Predicates)
    (assembly : ReviewedFq12AssemblyAvailabilityLocalSource P)
    (compatibility :
      ReviewedFq12CompatibilityAvailabilityExtractionRulesSource P) :
    ReviewedFq12FullOpeningFieldEvidenceLocalPredicateSource P :=
  { assemblyPredicates := by
      intro b payload cert liftCert assemblyCert compatCert hAccept
        hProjects hHasExtraction hExtractionValid hLifts hHasAssembly
        hAssemblyValid facts
      rcases
          assembly.localPredicates hAccept hProjects hHasExtraction
            hExtractionValid hLifts hHasAssembly hAssemblyValid
            facts.assemblyPayloadBinding with
        hLocal | hBreak
      · exact Or.inl
          (assembly_constructed_predicates_of_local_certificate hLocal)
      · exact Or.inr hBreak
    nativeValidity := by
      intro b payload cert liftCert assemblyCert compatCert hAccept
        hProjects hHasExtraction hExtractionValid hLifts hHasAssembly
        hAssemblyValid facts
      exact
        assembly.nativeValidity hAccept hProjects hHasExtraction
          hExtractionValid hLifts hHasAssembly hAssemblyValid
          facts.assemblyPayloadBinding
    extractionRules := compatibility.extractionRules }

/--
Selected-Rust/factored source-route inputs with compact local source records.

This is narrower than
`ReviewedFq12SelectedRustFactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs`:
assembly and compatibility availability are bundled with the local/native and
extraction-rule facts they naturally support, and the free local-predicate
source field disappears.
-/
structure ReviewedFq12SelectedRustFactoredConcreteLocalSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootFields :
    ReviewedFq12ConcreteRoleRootFieldBindingCertificate P
  assemblySource :
    ReviewedFq12AssemblyAvailabilityLocalSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  openingWitness :
    ReviewedFq12OpeningRoleLiftWitnessSource P
  constructedCompatibilityWitness :
    ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource P
  compatibilityAssemblyWitness :
    ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource P
  compatibilitySource :
    ReviewedFq12CompatibilityAvailabilityExtractionRulesSource P

/--
Convert compact local source-route inputs to the selected-Rust/factored
endpoint.
-/
def ReviewedFq12SelectedRustFactoredConcreteLocalSourceRouteInputs.toSelectedRustFactoredInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustFactoredConcreteLocalSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustFactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootFields := I.roleRootFields
    assemblyAvailability := I.assemblySource.availability
    selectedSource := I.selectedSource
    openingWitness := I.openingWitness
    constructedCompatibilityWitness := I.constructedCompatibilityWitness
    compatibilityAssemblyWitness := I.compatibilityAssemblyWitness
    compatibilityAvailability := I.compatibilitySource.availability
    localPredicateSource :=
      local_predicate_source_of_availability_local_sources
        P I.assemblySource I.compatibilitySource }

/--
P2FS-001 closure from the selected-Rust source-route package with compact
local source records.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_factored_concrete_local_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustFactoredConcreteLocalSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_factored_boundary_availability_concrete_role_root_source_route_inputs
    P I.toSelectedRustFactoredInputs

/--
Selected-Rust/factored compact local inputs with functional certificate lifts.

This is narrower than
`ReviewedFq12SelectedRustFactoredConcreteLocalSourceRouteInputs`: the
opening-to-role-lift witness is derived from the role-root bridge already
inside `roleRootFields`, plus the explicit theorem that the selected
certificate lift relation is functional.
-/
structure ReviewedFq12SelectedRustFactoredConcreteLocalFunctionalLiftSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootFields :
    ReviewedFq12ConcreteRoleRootFieldBindingCertificate P
  liftOpeningFunctionality :
    ReviewedFq12CertificateLiftOpeningFunctionality P
  assemblySource :
    ReviewedFq12AssemblyAvailabilityLocalSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  constructedCompatibilityWitness :
    ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource P
  compatibilityAssemblyWitness :
    ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource P
  compatibilitySource :
    ReviewedFq12CompatibilityAvailabilityExtractionRulesSource P

/--
Convert the functional-lift compact package to the current compact endpoint.
-/
def ReviewedFq12SelectedRustFactoredConcreteLocalFunctionalLiftSourceRouteInputs.toConcreteLocalInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustFactoredConcreteLocalFunctionalLiftSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustFactoredConcreteLocalSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootFields := I.roleRootFields
    assemblySource := I.assemblySource
    selectedSource := I.selectedSource
    openingWitness :=
      opening_role_lift_witness_of_role_root_bridge_and_lift_functionality
        I.roleRootFields.roleRootBridge I.liftOpeningFunctionality
    constructedCompatibilityWitness := I.constructedCompatibilityWitness
    compatibilityAssemblyWitness := I.compatibilityAssemblyWitness
    compatibilitySource := I.compatibilitySource }

/--
P2FS-001 closure from the selected-Rust compact source-route package with
functional certificate lifts.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_factored_concrete_local_functional_lift_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustFactoredConcreteLocalFunctionalLiftSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_factored_concrete_local_source_route_inputs
    P I.toConcreteLocalInputs

/--
Compatibility-side route witnesses remaining after the opening witness is
derived from the role-root bridge and certificate-lift functionality.

This source keeps the two post-assembly/post-compatibility model obligations
paired: one supplies compatibility for the deterministic assembly
representative, and the other reconnects a generic compatibility premise to
the deterministic assembly representative.  It is not a generic source of
assembly availability.
-/
structure ReviewedFq12CompatibilityRouteWitnessSource
    (P : Predicates) where
  constructedCompatibilityWitness :
    ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource P
  compatibilityAssemblyWitness :
    ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource P

/--
Recover constructed-representative compatibility from a full assembly
source-route witness by forgetting the role-lift certificate it also carries.
-/
theorem constructed_compatibility_witness_of_assembly_source_route_witness
    {P : Predicates}
    (Witness : ReviewedFq12AssemblySourceRouteWitnessSource P) :
    ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource P := by
  intro b payload cert opening assemblyCert hAccept hProjects hHas
    hValid hLifts hHasAssembly hAssemblyValid
  rcases
      Witness hAccept hProjects hHas hValid hLifts hHasAssembly
        hAssemblyValid with
    hRoute | hBreak
  · rcases hRoute with
      ⟨_liftCert, compatCert, _hHasLift, _hLiftValid, hOpeningEq,
        hHasCompat, hCompatValid⟩
    exact Or.inl
      ⟨compatCert,
        by simpa [hOpeningEq] using hHasCompat,
        by simpa [hOpeningEq] using hCompatValid⟩
  · exact Or.inr hBreak

/--
Recover the assembly-representative lookup from a full compatibility
source-route witness by forgetting the role-lift certificate it also carries.
-/
theorem assembly_representative_witness_of_compatibility_source_route_witness
    {P : Predicates}
    (Witness : ReviewedFq12CompatibilitySourceRouteWitnessSource P) :
    ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource P := by
  intro b payload cert opening h compatCert hAccept hProjects hHas
    hValid hLifts hMatches hAssembles hFiber hLocal hDescent hHasCompat
    hCompatValid
  rcases
      Witness hAccept hProjects hHas hValid hLifts hMatches hAssembles
        hFiber hLocal hDescent hHasCompat hCompatValid with
    hRoute | hBreak
  · rcases hRoute with
      ⟨_liftCert, assemblyCert, _hHasLift, _hLiftValid, hOpeningEq,
        hHasAssembly, hAssemblyValid, hRepresentativeEq⟩
    exact Or.inl
      ⟨assemblyCert,
        by simpa [hOpeningEq] using hHasAssembly,
        by simpa [hOpeningEq] using hAssemblyValid,
        hRepresentativeEq⟩
  · exact Or.inr hBreak

/--
Package the compatibility-side route witness source from full source-route
witnesses.
-/
def compatibility_route_witness_source_of_source_route_witnesses
    {P : Predicates}
    (AssemblyWitness : ReviewedFq12AssemblySourceRouteWitnessSource P)
    (CompatibilityWitness :
      ReviewedFq12CompatibilitySourceRouteWitnessSource P) :
    ReviewedFq12CompatibilityRouteWitnessSource P :=
  { constructedCompatibilityWitness :=
      constructed_compatibility_witness_of_assembly_source_route_witness
        AssemblyWitness
    compatibilityAssemblyWitness :=
      assembly_representative_witness_of_compatibility_source_route_witness
        CompatibilityWitness }

/--
Selected-Rust compact local inputs with functional certificate lifts and one
compatibility-side route-witness source.

This narrows
`ReviewedFq12SelectedRustFactoredConcreteLocalFunctionalLiftSourceRouteInputs`
by replacing its two loose compatibility route witnesses with a single audited
route/model source obligation.
-/
structure ReviewedFq12SelectedRustConcreteLocalFunctionalLiftSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootFields :
    ReviewedFq12ConcreteRoleRootFieldBindingCertificate P
  liftOpeningFunctionality :
    ReviewedFq12CertificateLiftOpeningFunctionality P
  assemblySource :
    ReviewedFq12AssemblyAvailabilityLocalSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  compatibilityRouteWitnesses :
    ReviewedFq12CompatibilityRouteWitnessSource P
  compatibilitySource :
    ReviewedFq12CompatibilityAvailabilityExtractionRulesSource P

/--
Forget the bundled compatibility route-witness source into the previous
functional-lift compact endpoint.
-/
def ReviewedFq12SelectedRustConcreteLocalFunctionalLiftSourceRouteInputs.toFactoredFunctionalLiftInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustConcreteLocalFunctionalLiftSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustFactoredConcreteLocalFunctionalLiftSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootFields := I.roleRootFields
    liftOpeningFunctionality := I.liftOpeningFunctionality
    assemblySource := I.assemblySource
    selectedSource := I.selectedSource
    constructedCompatibilityWitness :=
      I.compatibilityRouteWitnesses.constructedCompatibilityWitness
    compatibilityAssemblyWitness :=
      I.compatibilityRouteWitnesses.compatibilityAssemblyWitness
    compatibilitySource := I.compatibilitySource }

/--
P2FS-001 closure from the selected-Rust compact source-route package with
functional certificate lifts and bundled compatibility route witnesses.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_concrete_local_functional_lift_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustConcreteLocalFunctionalLiftSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_factored_concrete_local_functional_lift_source_route_inputs
    P I.toFactoredFunctionalLiftInputs

/--
Role-root and lift-functionality source for the selected Rust route.

This groups the two role-root-side model obligations that must be discharged
before source-route bytes can be used for the semantic opening: concrete
role-root/full-opening field binding and functionality of the selected
certificate lift relation.
-/
structure ReviewedFq12RoleRootFunctionalLiftSource
    (P : Predicates) where
  roleRootFields :
    ReviewedFq12ConcreteRoleRootFieldBindingCertificate P
  liftOpeningFunctionality :
    ReviewedFq12CertificateLiftOpeningFunctionality P

/--
Local model source for assembly and compatibility.

This groups the non-byte local model facts left after selected source-route
bytes supply concrete field binding: assembly availability/local/native facts
and compatibility availability/extraction rules.
-/
structure ReviewedFq12AssemblyCompatibilityLocalModelSource
    (P : Predicates) where
  assemblySource :
    ReviewedFq12AssemblyAvailabilityLocalSource P
  compatibilitySource :
    ReviewedFq12CompatibilityAvailabilityExtractionRulesSource P

/--
Convert the Rust-shaped canonical representative witness source into the
semantic byte source consumed by AP2-N001.

The conversion is the point where the proof recovers the concrete role-lift
certificate for the semantic opening, obtains the assembly certificate for
that opening, and proves the assembly certificate binds the same opening before
calling the Rust-shaped witness source.
-/
def canonical_representative_byte_source_of_witness_source_and_model
    {Parser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {P : Predicates}
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (directAssemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (localModelSource :
      ReviewedFq12AssemblyCompatibilityLocalModelSource P)
    (Source :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource
        Parser P) :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource
      Parser P :=
  let openingWitness :
      ReviewedFq12OpeningRoleLiftWitnessSource P :=
    opening_role_lift_witness_of_role_root_bridge_and_lift_functionality
      roleRootLiftSource.roleRootFields.roleRootBridge
      roleRootLiftSource.liftOpeningFunctionality
  { digestFrame := Source.digestFrame
    hasCanonicalRepresentativeWitness :=
      fun b payload cert _opening h frameBytes witness =>
        exists liftCert assemblyCert,
          Source.hasCanonicalRepresentativeWitnessFromCertificates
            b payload cert liftCert assemblyCert h frameBytes witness
    canonicalRepresentativeWitnessAvailable := by
      intro b payload cert opening h hAccept hProjects hHas hValid hLifts
        hMatches hAssembles hFiber hLocal hDescent
      rcases
          openingWitness hAccept hProjects hHas hValid hLifts with
        hLift | hLiftBreak
      · rcases hLift with
          ⟨liftCert, hHasLift, hLiftValid, hOpeningEq⟩
        rcases
            localModelSource.assemblySource.availability
              hAccept hProjects hHas hValid hLifts with
          hAssembly | hAssemblyBreak
        · rcases hAssembly with
            ⟨assemblyCert, hHasAssembly, hAssemblyValid⟩
          rcases
              directAssemblyBinding hAccept hProjects hHas hValid hLifts
                hHasAssembly hAssemblyValid with
            hBindsFields | hBindingBreak
          · let hBinds :
                ReviewedFq12AssemblyCertificateBindsOpening
                  payload cert opening assemblyCert :=
              binding_of_assembly_concrete_field_certificate hBindsFields
            rcases
                Source.canonicalRepresentativeWitnessAvailableFromCertificates
                  hAccept hProjects hHas hValid hLifts hHasLift hLiftValid
                  hOpeningEq hHasAssembly hAssemblyValid hBinds hMatches
                  hAssembles hFiber hLocal hDescent with
              hWitness | hWitnessBreak
            · rcases hWitness with
                ⟨frameBytes, witness, hHasWitness, hByte⟩
              exact Or.inl
                ⟨frameBytes, witness,
                  ⟨liftCert, assemblyCert, hHasWitness⟩, hByte⟩
            · exact Or.inr hWitnessBreak
          · exact Or.inr hBindingBreak
        · exact Or.inr hAssemblyBreak
      · exact Or.inr hLiftBreak }

/--
Derive the representative canonical-digest source from lower-level selected
Rust/model facts.

The representative side uses assembly availability to obtain the deterministic
assembly certificate for the opening, selected source-route/field binding to
bind that certificate to the opening, local/native assembly facts for the
constructed representative, and digest functionality to identify the generic
representative's digest with the constructed representative's digest.  The
assembly side is exactly the selected Rust assembly-frame recomputation check.
-/
def representative_canonical_digest_source_of_frame_functionality_binding_and_local_model
    {P : Predicates}
    (assemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (localModel : ReviewedFq12AssemblyCompatibilityLocalModelSource P)
    (digestFunctionality :
      ReviewedFq12FullNativeOpeningRepresentativeDigestFunctionalitySource P)
    (frameSource :
      ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P) :
    ReviewedFq12RepresentativeCanonicalDigestSource P :=
  { canonicalDigest := frameSource.canonicalDigest
    representativeDigestCanonical := by
      intro b payload cert opening h hAccept hProjects hHas hValid hLifts
        hMatches hAssembles hFiber hLocal hDescent
      rcases
          localModel.assemblySource.availability
            hAccept hProjects hHas hValid hLifts with
        hAssembly | hAvailabilityBreak
      · rcases hAssembly with
          ⟨assemblyCert, hHasAssembly, hAssemblyValid⟩
        rcases
            assemblyBinding hAccept hProjects hHas hValid hLifts
              hHasAssembly hAssemblyValid with
          hFields | hBindingBreak
        · let hBinds :
              ReviewedFq12AssemblyCertificateBindsOpening
                payload cert opening assemblyCert :=
              binding_of_assembly_concrete_field_certificate hFields
          rcases
              localModel.assemblySource.localPredicates
                hAccept hProjects hHas hValid hLifts hHasAssembly
                hAssemblyValid hBinds with
            hConstructedLocal | hLocalBreak
          · rcases
                localModel.assemblySource.nativeValidity
                  hAccept hProjects hHas hValid hLifts hHasAssembly
                  hAssemblyValid hBinds with
              hConstructedNative | hNativeBreak
            · let constructed :
                  HiddenRepresentative :=
                ReviewedFq12AssemblyConstructedRepresentative
                  assemblyCert
              have hConstructedMatches :
                  FullNativeOpeningMatchesRepresentative
                    opening constructed :=
                reviewed_fq12_assembly_constructed_representative_matches_opening
                  hBinds
              have hConstructedAssembles :
                  P.fullNativeOpeningAssemblesRepresentative
                    b payload cert opening constructed :=
                hConstructedLocal.fullOpeningAssembles
              have hConstructedDigest :
                  constructed.representativeDigest =
                    assemblyCert.representativeDigest := by
                rfl
              rcases
                  digestFunctionality hAccept hProjects hHas hValid hLifts
                    hMatches hAssembles hFiber hLocal hDescent
                    hConstructedMatches hConstructedAssembles
                    hConstructedNative.inFiber
                    hConstructedNative.localValid
                    hConstructedNative.descentConsistent with
                hDigestEq | hFunctionalityBreak
              · rcases
                    frameSource.assemblyCertificateDigestCanonical
                      hAccept hProjects hHas hValid hLifts hHasAssembly
                      hAssemblyValid hBinds with
                  hAssemblyCanonical | hFrameBreak
                · exact Or.inl
                    (hDigestEq.trans
                      (hConstructedDigest.trans hAssemblyCanonical))
                · exact Or.inr hFrameBreak
              · exact Or.inr hFunctionalityBreak
            · exact Or.inr hNativeBreak
          · exact Or.inr hLocalBreak
        · exact Or.inr hBindingBreak
      · exact Or.inr hAvailabilityBreak
    assemblyCertificateDigestCanonical := by
      intro b payload cert opening assemblyCert hAccept hProjects hHas
        hValid hLifts hHasAssembly hAssemblyValid hBinds
      exact
        frameSource.assemblyCertificateDigestCanonical
          hAccept hProjects hHas hValid hLifts hHasAssembly hAssemblyValid
          hBinds }

/--
Build the shared selected-Rust representative frame from the byte-witness-shaped
assembly frame plus the existing representative digest-functionality/local-model
argument.

The Rust `Path2ReviewedFq12NativeRepresentativeDigestFrameWitnessV1` witnesses
the assembly-certificate side of `path2_reviewed_fq12_native_representative_digest_v1`.
The representative side is still derived by comparing an arbitrary assembled
representative with the deterministically constructed assembly representative.
-/
def shared_representative_frame_source_of_assembly_frame_functionality_binding_and_local_model
    {P : Predicates}
    (assemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (localModel : ReviewedFq12AssemblyCompatibilityLocalModelSource P)
    (digestFunctionality :
      ReviewedFq12FullNativeOpeningRepresentativeDigestFunctionalitySource P)
    (frameSource :
      ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P) :
    ReviewedFq12SelectedRustSharedRepresentativeFrameSource P :=
  let representativeSource :
      ReviewedFq12RepresentativeCanonicalDigestSource P :=
    representative_canonical_digest_source_of_frame_functionality_binding_and_local_model
      assemblyBinding localModel digestFunctionality frameSource
  { canonicalDigest := frameSource.canonicalDigest
    representativeDigestCanonical :=
      representativeSource.representativeDigestCanonical
    assemblyCertificateDigestCanonical :=
      frameSource.assemblyCertificateDigestCanonical }

/--
Build the full compatibility route-witness source from direct assembly field
binding, the compact local model, and the remaining representative-lookup
witness.

This is a non-active convenience adapter: the active selected source-route
package keeps `ReviewedFq12CompatibilityRouteWitnessSource` explicit because
selected route-byte binding is compatibility-indexed and cannot supply the
direct pre-compatibility assembly binding consumed here.
-/
def compatibility_route_witness_source_of_assembly_field_binding_local_model_and_representative_witness
    {P : Predicates}
    (assemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (localModel : ReviewedFq12AssemblyCompatibilityLocalModelSource P)
    (AssemblyWitness :
      ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource P) :
    ReviewedFq12CompatibilityRouteWitnessSource P :=
  { constructedCompatibilityWitness :=
      constructed_compatibility_witness_of_assembly_field_binding_and_local_model
        assemblyBinding localModel.assemblySource
        localModel.compatibilitySource
    compatibilityAssemblyWitness := AssemblyWitness }

/--
Build the full compatibility route-witness source from direct assembly field
binding, the compact local model, and the representative-digest lookup source.

This is the strongest non-circular adapter currently available for the
compatibility route witness package.  It still requires direct assembly field
binding before compatibility is selected; selected source-route byte binding is
not sufficient because it is only available for an already witnessed
compatibility-indexed chain.
-/
def compatibility_route_witness_source_of_assembly_field_binding_local_model_and_digest_lookup
    {P : Predicates}
    (assemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (localModel : ReviewedFq12AssemblyCompatibilityLocalModelSource P)
    (digestLookup :
      ReviewedFq12AssemblyRepresentativeDigestLookupSource P) :
    ReviewedFq12CompatibilityRouteWitnessSource P :=
  { constructedCompatibilityWitness :=
      constructed_compatibility_witness_of_assembly_field_binding_and_local_model
        assemblyBinding localModel.assemblySource
        localModel.compatibilitySource
    compatibilityAssemblyWitness :=
      compatibility_assembly_representative_witness_of_assembly_availability_binding_and_digest_lookup
        localModel.assemblySource.availability assemblyBinding digestLookup }

/--
Forget redundant field-binding facts from the existing field/local concrete
assembly source into the local-model source used by the selected Rust route.
The selected source-route byte frame supplies field binding separately.
-/
def assembly_availability_local_source_of_field_concrete_assembly_local_source
    {P : Predicates}
    (A : ReviewedFq12FieldConcreteAssemblyLocalSource P) :
    ReviewedFq12AssemblyAvailabilityLocalSource P :=
  { availability := A.availability
    localPredicates := A.localPredicates
    nativeValidity := A.nativeValidity }

/--
Forget redundant field-binding facts from the existing field-level
compatibility source into the local-model source used by the selected Rust
route. The selected source-route byte frame supplies same-representative field
binding separately.
-/
def compatibility_availability_extraction_rules_source_of_field_concrete_compatibility_source
    {P : Predicates}
    (C : ReviewedFq12FieldConcreteCompatibilitySource P) :
    ReviewedFq12CompatibilityAvailabilityExtractionRulesSource P :=
  { availability := C.availability
    extractionRules := C.extractionRules }

/--
Build the packaged local model source from the existing concrete field/local
source records.
-/
def assembly_compatibility_local_model_source_of_field_sources
    {P : Predicates}
    (A : ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (C : ReviewedFq12FieldConcreteCompatibilitySource P) :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P :=
  { assemblySource :=
      assembly_availability_local_source_of_field_concrete_assembly_local_source
        A
    compatibilitySource :=
      compatibility_availability_extraction_rules_source_of_field_concrete_compatibility_source
        C }

/--
Lean mirror of the public field/local source evidence report.

This report records that the selected Rust/model local field sources pin the
assembly availability, assembly binding, local/native assembly facts,
compatibility availability, same-representative binding, extraction rules, and
negative controls before the final AP2-N001 certificate may expose the raw
assembly and compatibility source records.
-/
structure ReviewedFq12SelectedRustFieldLocalEvidenceReport where
  schemaMatches : Bool
  reportDigestVerified : Bool
  assemblyAvailabilityPinned : Bool
  assemblyFieldBindingPinned : Bool
  assemblyLocalPredicatesPinned : Bool
  assemblyNativeValidityPinned : Bool
  compatibilityAvailabilityPinned : Bool
  compatibilityFieldBindingPinned : Bool
  compatibilityExtractionRulesPinned : Bool
  fieldLocalNegativeControlsPassed : Bool

namespace ReviewedFq12SelectedRustFieldLocalEvidenceReport

/-- The public field/local source report has been digest/schema checked. -/
def ReportVerified
    (E : ReviewedFq12SelectedRustFieldLocalEvidenceReport) :
    Prop :=
  E.schemaMatches = true ∧
    E.reportDigestVerified = true

/-- Every selected field/local component fact in the report holds. -/
def FactsAllTrue
    (E : ReviewedFq12SelectedRustFieldLocalEvidenceReport) :
    Prop :=
  E.assemblyAvailabilityPinned = true ∧
    E.assemblyFieldBindingPinned = true ∧
    E.assemblyLocalPredicatesPinned = true ∧
    E.assemblyNativeValidityPinned = true ∧
    E.compatibilityAvailabilityPinned = true ∧
    E.compatibilityFieldBindingPinned = true ∧
    E.compatibilityExtractionRulesPinned = true ∧
    E.fieldLocalNegativeControlsPassed = true

end ReviewedFq12SelectedRustFieldLocalEvidenceReport

/--
Proof-bearing selected Rust field/local source certificate.

The final AP2-N001 theorem should use this certificate to expose the assembly
and compatibility source records, preventing the concrete local-model facts
from entering the five-source closure as independent raw assumptions.
-/
structure ReviewedFq12SelectedRustFieldLocalEvidenceCertificate
    (P : Predicates) where
  evidenceReport :
    ReviewedFq12SelectedRustFieldLocalEvidenceReport
  evidenceReportVerifiedProof :
    evidenceReport.ReportVerified
  evidenceFactsAllTrueProof :
    evidenceReport.FactsAllTrue
  assemblySource :
    ReviewedFq12FieldConcreteAssemblyLocalSource P
  compatibilitySource :
    ReviewedFq12FieldConcreteCompatibilitySource P

namespace ReviewedFq12SelectedRustFieldLocalEvidenceCertificate

/-- Expose the assembly source only after consuming report proofs. -/
def toAssemblySource
    {P : Predicates}
    (Cert : ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P) :
    ReviewedFq12FieldConcreteAssemblyLocalSource P :=
  let _hReport : Cert.evidenceReport.ReportVerified :=
    Cert.evidenceReportVerifiedProof
  let _hFacts : Cert.evidenceReport.FactsAllTrue :=
    Cert.evidenceFactsAllTrueProof
  Cert.assemblySource

/-- Expose the compatibility source only after consuming report proofs. -/
def toCompatibilitySource
    {P : Predicates}
    (Cert : ReviewedFq12SelectedRustFieldLocalEvidenceCertificate P) :
    ReviewedFq12FieldConcreteCompatibilitySource P :=
  let _hReport : Cert.evidenceReport.ReportVerified :=
    Cert.evidenceReportVerifiedProof
  let _hFacts : Cert.evidenceReport.FactsAllTrue :=
    Cert.evidenceFactsAllTrueProof
  Cert.compatibilitySource

end ReviewedFq12SelectedRustFieldLocalEvidenceCertificate

/--
Selected-Rust source-route inputs with explicit role-root/lift, local-model,
and compatibility-route source packages.

This is the narrowest active P2FS-001 wrapper: it leaves four implementation
or model sources visible, rather than loose fields, while still deriving the
same selected-Rust concrete local functional-lift endpoint.
-/
structure ReviewedFq12SelectedRustPackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  compatibilityRouteWitnesses :
    ReviewedFq12CompatibilityRouteWitnessSource P

/-- Forget packaged sources into the selected-Rust functional-lift endpoint. -/
def ReviewedFq12SelectedRustPackagedSourceRouteInputs.toConcreteLocalFunctionalLiftInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustPackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustConcreteLocalFunctionalLiftSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootFields := I.roleRootLiftSource.roleRootFields
    liftOpeningFunctionality :=
      I.roleRootLiftSource.liftOpeningFunctionality
    assemblySource := I.localModelSource.assemblySource
    selectedSource := I.selectedSource
    compatibilityRouteWitnesses := I.compatibilityRouteWitnesses
    compatibilitySource := I.localModelSource.compatibilitySource }

/--
P2FS-001 closure from the selected-Rust packaged source-route inputs.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustPackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_concrete_local_functional_lift_source_route_inputs
    P I.toConcreteLocalFunctionalLiftInputs

/--
Selected-Rust packaged source-route inputs with canonical representative
digest evidence exposed instead of a prebuilt compatibility route witness.

This is the first instantiation-oriented wrapper for the active package that
matches the deployable Rust shape: source-route bytes and an assembly
source-route witness supply direct assembly field binding; the local model
supplies assembly/compatibility availability and extraction; the canonical
digest source supplies the representative-digest lookup needed to reconnect a
generic compatibility representative to the deterministic assembly
certificate.
-/
structure ReviewedFq12SelectedRustCanonicalDigestPackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  assemblySourceRouteWitness :
    ReviewedFq12AssemblySourceRouteWitnessSource P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  representativeCanonicalDigestSource :
    ReviewedFq12RepresentativeCanonicalDigestSource P

/--
Build the active packaged endpoint from canonical representative-digest
evidence and the selected Rust source-route byte source.
-/
def ReviewedFq12SelectedRustCanonicalDigestPackagedSourceRouteInputs.toPackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustCanonicalDigestPackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustPackagedSourceRouteInputs
      FieldParser RouteParser P :=
  let assemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P :=
    source_route_byte_source_assembly_field_binding_of_route_witness
      I.assemblySourceRouteWitness I.selectedSource.toBytePredicateSource
  let digestLookup :
      ReviewedFq12AssemblyRepresentativeDigestLookupSource P :=
    assembly_representative_digest_lookup_of_canonical_digest_source
      I.representativeCanonicalDigestSource
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    localModelSource := I.localModelSource
    compatibilityRouteWitnesses :=
      compatibility_route_witness_source_of_assembly_field_binding_local_model_and_digest_lookup
        assemblyBinding I.localModelSource digestLookup }

/--
P2FS-001 closure from selected-Rust packaged inputs backed by canonical
representative-digest evidence.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_canonical_digest_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustCanonicalDigestPackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_packaged_source_route_inputs
    P I.toPackagedInputs

/--
Selected-Rust packaged source-route inputs using the lowest currently exposed
representative-digest obligations.

This wrapper replaces the prebuilt
`ReviewedFq12RepresentativeCanonicalDigestSource` with the two concrete facts
that should be instantiated from the selected Rust/model path:

* digest functionality of the full-native-opening assembly relation; and
* the selected Rust assembly representative frame recomputing
  `path2_reviewed_fq12_native_representative_digest_v1`.
-/
structure ReviewedFq12SelectedRustFrameCanonicalDigestPackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  assemblySourceRouteWitness :
    ReviewedFq12AssemblySourceRouteWitnessSource P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  representativeDigestFunctionality :
    ReviewedFq12FullNativeOpeningRepresentativeDigestFunctionalitySource P
  assemblyRepresentativeFrameSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P

/--
Convert frame/functionality canonical-digest inputs into the canonical-digest
packaged source-route wrapper.
-/
def ReviewedFq12SelectedRustFrameCanonicalDigestPackagedSourceRouteInputs.toCanonicalDigestPackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustFrameCanonicalDigestPackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustCanonicalDigestPackagedSourceRouteInputs
      FieldParser RouteParser P :=
  let assemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P :=
    source_route_byte_source_assembly_field_binding_of_route_witness
      I.assemblySourceRouteWitness I.selectedSource.toBytePredicateSource
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    assemblySourceRouteWitness := I.assemblySourceRouteWitness
    localModelSource := I.localModelSource
    representativeCanonicalDigestSource :=
      representative_canonical_digest_source_of_frame_functionality_binding_and_local_model
        assemblyBinding I.localModelSource I.representativeDigestFunctionality
        I.assemblyRepresentativeFrameSource }

/--
P2FS-001 closure from selected-Rust packaged inputs backed by selected
assembly-frame canonicality and digest functionality.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_frame_canonical_digest_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustFrameCanonicalDigestPackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_canonical_digest_packaged_source_route_inputs
    P I.toCanonicalDigestPackagedInputs

/--
Selected-Rust packaged source-route inputs using representative and assembly
frame sources directly.

This is the lowest exposed representative/canonicality wrapper in the active
source-route chain: the representative frame source derives digest
functionality, and the assembly frame source proves the selected Rust assembly
certificate recomputes the same active representative digest frame.
-/
structure ReviewedFq12SelectedRustRepresentativeFramePackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  assemblySourceRouteWitness :
    ReviewedFq12AssemblySourceRouteWitnessSource P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  representativeFrameSource :
    ReviewedFq12FullNativeOpeningRepresentativeFrameSource P
  assemblyRepresentativeFrameSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P

/--
Convert direct representative/assembly frame sources into the frame-canonical
packaged source-route wrapper.
-/
def ReviewedFq12SelectedRustRepresentativeFramePackagedSourceRouteInputs.toFrameCanonicalDigestPackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustRepresentativeFramePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustFrameCanonicalDigestPackagedSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    assemblySourceRouteWitness := I.assemblySourceRouteWitness
    localModelSource := I.localModelSource
    representativeDigestFunctionality :=
      representative_digest_functionality_of_representative_frame_source
        I.representativeFrameSource
    assemblyRepresentativeFrameSource :=
      I.assemblyRepresentativeFrameSource }

/--
P2FS-001 closure from selected-Rust packaged inputs backed directly by
representative and assembly frame sources.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_representative_frame_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustRepresentativeFramePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_frame_canonical_digest_packaged_source_route_inputs
    P I.toFrameCanonicalDigestPackagedInputs

/--
Selected-Rust packaged source-route inputs using canonical representative
construction directly.

This is the lowest exposed representative/canonicality wrapper: the
full-native-opening predicate must assemble the canonical representative object
for the boundary/opening tuple, and the assembly certificate must recompute the
selected Rust representative digest frame.
-/
structure ReviewedFq12SelectedRustCanonicalRepresentativePackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  assemblySourceRouteWitness :
    ReviewedFq12AssemblySourceRouteWitnessSource P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  canonicalRepresentativeSource :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeSource P
  assemblyRepresentativeFrameSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P

/--
Convert canonical-representative inputs into the representative-frame package.
-/
def ReviewedFq12SelectedRustCanonicalRepresentativePackagedSourceRouteInputs.toRepresentativeFramePackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustCanonicalRepresentativePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustRepresentativeFramePackagedSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    assemblySourceRouteWitness := I.assemblySourceRouteWitness
    localModelSource := I.localModelSource
    representativeFrameSource :=
      representative_frame_source_of_canonical_representative_source
        I.canonicalRepresentativeSource
    assemblyRepresentativeFrameSource :=
      I.assemblyRepresentativeFrameSource }

/--
P2FS-001 closure from selected-Rust packaged inputs backed directly by
canonical representative construction and assembly-frame recomputation.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_canonical_representative_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustCanonicalRepresentativePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_representative_frame_packaged_source_route_inputs
    P I.toRepresentativeFramePackagedInputs

/--
Factored selected-Rust canonical-representative package.

This replaces the raw `ReviewedFq12AssemblySourceRouteWitnessSource` with the
two non-circular ingredients that produce it: role-root/lift source and
constructed-assembly compatibility witness.  The selected source-route bytes
remain compatibility-indexed; this wrapper only makes the witness construction
explicit.
-/
structure ReviewedFq12SelectedRustFactoredCanonicalRepresentativePackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  constructedCompatibilityWitness :
    ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  canonicalRepresentativeSource :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeSource P
  assemblyRepresentativeFrameSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P

/--
Convert factored canonical-representative inputs into the current canonical
representative package.
-/
def ReviewedFq12SelectedRustFactoredCanonicalRepresentativePackagedSourceRouteInputs.toCanonicalRepresentativePackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustFactoredCanonicalRepresentativePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustCanonicalRepresentativePackagedSourceRouteInputs
      FieldParser RouteParser P :=
  let openingWitness :
      ReviewedFq12OpeningRoleLiftWitnessSource P :=
    opening_role_lift_witness_of_role_root_bridge_and_lift_functionality
      I.roleRootLiftSource.roleRootFields.roleRootBridge
      I.roleRootLiftSource.liftOpeningFunctionality
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    assemblySourceRouteWitness :=
      assembly_source_route_witness_of_opening_and_constructed_compatibility_witness
        openingWitness I.constructedCompatibilityWitness
    localModelSource := I.localModelSource
    canonicalRepresentativeSource := I.canonicalRepresentativeSource
    assemblyRepresentativeFrameSource :=
      I.assemblyRepresentativeFrameSource }

/--
P2FS-001 closure from the factored canonical-representative selected-Rust
package.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_factored_canonical_representative_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustFactoredCanonicalRepresentativePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_canonical_representative_packaged_source_route_inputs
    P I.toCanonicalRepresentativePackagedInputs

/--
Direct-assembly-binding canonical-representative package.

This is the non-circular way to remove
`ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource` as a primitive
source: the model must supply direct assembly field binding independently of the
compatibility-indexed selected source-route bytes.  The selected bytes still
bind the active field/source-route frame after the route witness is constructed;
they are not reused as pre-compatibility assembly evidence.
-/
structure ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativePackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  directAssemblyBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  canonicalRepresentativeSource :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeSource P
  assemblyRepresentativeFrameSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P

/--
Convert direct assembly binding into the factored canonical-representative
package by deriving constructed-assembly compatibility from the local model.
-/
def ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativePackagedSourceRouteInputs.toFactoredCanonicalRepresentativePackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustFactoredCanonicalRepresentativePackagedSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    constructedCompatibilityWitness :=
      constructed_compatibility_witness_of_assembly_field_binding_and_local_model
        I.directAssemblyBinding I.localModelSource.assemblySource
        I.localModelSource.compatibilitySource
    localModelSource := I.localModelSource
    canonicalRepresentativeSource := I.canonicalRepresentativeSource
    assemblyRepresentativeFrameSource :=
      I.assemblyRepresentativeFrameSource }

/--
P2FS-001 closure from direct assembly binding, canonical representative
construction, and the selected Rust source-route byte source.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_factored_canonical_representative_packaged_source_route_inputs
    P I.toFactoredCanonicalRepresentativePackagedInputs

/--
Direct-assembly representative-frame package.

This is the lowest direct-assembly wrapper for the selected Rust/model route:
direct assembly field binding derives constructed compatibility, the
representative frame source supplies the canonical representative digest, and
`FullNativeOpeningMatchesRepresentative` supplies the role-coordinate equality
needed to derive the canonical representative object.
-/
structure ReviewedFq12SelectedRustDirectAssemblyRepresentativeFramePackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  directAssemblyBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  representativeFrameSource :
    ReviewedFq12FullNativeOpeningRepresentativeFrameSource P
  assemblyRepresentativeFrameSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P

/--
Convert direct assembly binding plus representative-frame canonicality into the
direct-assembly canonical-representative package.
-/
def ReviewedFq12SelectedRustDirectAssemblyRepresentativeFramePackagedSourceRouteInputs.toDirectAssemblyCanonicalRepresentativePackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustDirectAssemblyRepresentativeFramePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativePackagedSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    directAssemblyBinding := I.directAssemblyBinding
    localModelSource := I.localModelSource
    canonicalRepresentativeSource :=
      canonical_representative_source_of_representative_frame_source
        I.representativeFrameSource
    assemblyRepresentativeFrameSource :=
      I.assemblyRepresentativeFrameSource }

/--
P2FS-001 closure from direct assembly binding, selected representative-frame
canonicality, selected assembly-frame recomputation, and source-route bytes.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_representative_frame_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustDirectAssemblyRepresentativeFramePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_packaged_source_route_inputs
    P I.toDirectAssemblyCanonicalRepresentativePackagedInputs

/--
Direct-assembly package with a single shared selected-Rust representative frame.

This is the closest current Lean target to the Rust implementation: one
canonical digest frame feeds both full-native-opening representative canonicality
and assembly-certificate recomputation.
-/
structure ReviewedFq12SelectedRustDirectAssemblySharedRepresentativeFramePackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  directAssemblyBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  sharedRepresentativeFrameSource :
    ReviewedFq12SelectedRustSharedRepresentativeFrameSource P

/--
Convert a shared representative frame source into the direct-assembly
representative-frame package.
-/
def ReviewedFq12SelectedRustDirectAssemblySharedRepresentativeFramePackagedSourceRouteInputs.toDirectAssemblyRepresentativeFramePackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustDirectAssemblySharedRepresentativeFramePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustDirectAssemblyRepresentativeFramePackagedSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    directAssemblyBinding := I.directAssemblyBinding
    localModelSource := I.localModelSource
    representativeFrameSource :=
      representative_frame_source_of_shared_representative_frame_source
        I.sharedRepresentativeFrameSource
    assemblyRepresentativeFrameSource :=
      assembly_frame_source_of_shared_representative_frame_source
        I.sharedRepresentativeFrameSource }

/--
Convert the shared-frame package directly to the active packaged source-route
endpoint.

This avoids recovering assembly binding from selected source-route bytes for
compatibility-route construction: independent direct assembly binding supplies
that premise, while selected bytes remain the parser/frame bridge for the
already witnessed active route.
-/
def ReviewedFq12SelectedRustDirectAssemblySharedRepresentativeFramePackagedSourceRouteInputs.toPackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustDirectAssemblySharedRepresentativeFramePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustPackagedSourceRouteInputs
      FieldParser RouteParser P :=
  let constructedCompatibilityWitness :
      ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource P :=
    constructed_compatibility_witness_of_assembly_field_binding_and_local_model
      I.directAssemblyBinding I.localModelSource.assemblySource
      I.localModelSource.compatibilitySource
  let digestLookup :
      ReviewedFq12AssemblyRepresentativeDigestLookupSource P :=
    assembly_representative_digest_lookup_of_canonical_digest_source
      (representative_canonical_digest_source_of_shared_representative_frame_source
        I.sharedRepresentativeFrameSource)
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    localModelSource := I.localModelSource
    compatibilityRouteWitnesses :=
      { constructedCompatibilityWitness :=
          constructedCompatibilityWitness
        compatibilityAssemblyWitness :=
          compatibility_assembly_representative_witness_of_assembly_availability_binding_and_digest_lookup
            I.localModelSource.assemblySource.availability
            I.directAssemblyBinding digestLookup } }

/--
P2FS-001 closure from direct assembly binding, a single shared selected Rust
representative frame, and source-route bytes.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_shared_representative_frame_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustDirectAssemblySharedRepresentativeFramePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_packaged_source_route_inputs
    P I.toPackagedInputs

/--
Direct-assembly package with the selected Rust native representative digest frame.

This is the narrowest proof-facing active-route target: the representative frame
is not an arbitrary boundary/opening digest function, but the 11-input
`path2_reviewed_fq12_native_representative_digest_v1` shape over the native
boundary digest and ten full-opening role digests.
-/
structure ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  directAssemblyBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  nativeDigestFrameSource :
    ReviewedFq12SelectedRustNativeRepresentativeDigestFrameSource P

/--
Convert the Rust-shaped native digest frame package into the shared-frame
package used by the active selected-Rust source-route closure theorem.
-/
def ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs.toSharedRepresentativeFramePackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustDirectAssemblySharedRepresentativeFramePackagedSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    directAssemblyBinding := I.directAssemblyBinding
    localModelSource := I.localModelSource
    sharedRepresentativeFrameSource :=
      shared_representative_frame_source_of_native_digest_frame_source
        I.nativeDigestFrameSource }

/--
P2FS-001 closure from direct assembly binding, the selected Rust native
representative digest frame, and source-route bytes.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_native_digest_frame_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_shared_representative_frame_packaged_source_route_inputs
    P I.toSharedRepresentativeFramePackagedInputs

/--
Direct-assembly package using the Rust digest-frame witness shape.

This is the instantiation-oriented target after adding
`Path2ReviewedFq12NativeRepresentativeDigestFrameWitnessV1` in Rust: the byte
witness supplies the assembly-frame recomputation source, while representative
canonicality is derived from digest functionality, direct assembly binding, and
the local model.
-/
structure ReviewedFq12SelectedRustDirectAssemblyDigestFrameWitnessPackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  directAssemblyBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  representativeDigestFunctionality :
    ReviewedFq12FullNativeOpeningRepresentativeDigestFunctionalitySource P
  nativeDigestFrameWitnessSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource P

/--
Convert the Rust digest-frame witness package into the shared-frame package used
by the active selected-Rust source-route closure theorem.
-/
def ReviewedFq12SelectedRustDirectAssemblyDigestFrameWitnessPackagedSourceRouteInputs.toSharedRepresentativeFramePackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustDirectAssemblyDigestFrameWitnessPackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustDirectAssemblySharedRepresentativeFramePackagedSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    directAssemblyBinding := I.directAssemblyBinding
    localModelSource := I.localModelSource
    sharedRepresentativeFrameSource :=
      shared_representative_frame_source_of_assembly_frame_functionality_binding_and_local_model
        I.directAssemblyBinding I.localModelSource
        I.representativeDigestFunctionality
        I.nativeDigestFrameWitnessSource }

/--
P2FS-001 closure from direct assembly binding, the Rust digest-frame witness
source, representative digest functionality, and source-route bytes.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_digest_frame_witness_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustDirectAssemblyDigestFrameWitnessPackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_shared_representative_frame_packaged_source_route_inputs
    P I.toSharedRepresentativeFramePackagedInputs

/--
Direct-assembly package using the Rust digest-frame witness bytes plus a
canonical full-opening representative source.

This is one step closer to the concrete Rust/model instantiation than the raw
digest-functionality package: the byte source discharges the assembly-frame
side, while canonical representative construction discharges representative
digest functionality.
-/
structure ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeWitnessBytePackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  directAssemblyBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  canonicalRepresentativeSource :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeSource P
  nativeDigestFrameWitnessByteSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
      WitnessParser P

/--
Convert canonical representative plus Rust witness bytes into the
digest-functionality/witness package.
-/
def ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeWitnessBytePackagedSourceRouteInputs.toDigestFrameWitnessPackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeWitnessBytePackagedSourceRouteInputs
        FieldParser RouteParser WitnessParser P) :
    ReviewedFq12SelectedRustDirectAssemblyDigestFrameWitnessPackagedSourceRouteInputs
      FieldParser RouteParser P :=
  let representativeFrameSource :
      ReviewedFq12FullNativeOpeningRepresentativeFrameSource P :=
    representative_frame_source_of_canonical_representative_source
      I.canonicalRepresentativeSource
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    directAssemblyBinding := I.directAssemblyBinding
    localModelSource := I.localModelSource
    representativeDigestFunctionality :=
      representative_digest_functionality_of_representative_frame_source
        representativeFrameSource
    nativeDigestFrameWitnessSource :=
      assembly_frame_source_of_digest_frame_witness_byte_source
        I.nativeDigestFrameWitnessByteSource }

/--
P2FS-001 closure from canonical representative construction, Rust digest-frame
witness bytes, direct assembly binding, and source-route bytes.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_witness_byte_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeWitnessBytePackagedSourceRouteInputs
        FieldParser RouteParser WitnessParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_digest_frame_witness_packaged_source_route_inputs
    P I.toDigestFrameWitnessPackagedInputs

/--
Fully byte-shaped direct-assembly package: selected source-route bytes, a
canonical representative byte source, and Rust digest-frame witness bytes.
-/
structure ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  directAssemblyBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  localModelSource :
    ReviewedFq12AssemblyCompatibilityLocalModelSource P
  canonicalRepresentativeByteSource :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource
      CanonicalParser P
  nativeDigestFrameWitnessByteSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
      WitnessParser P

/--
Convert the fully byte-shaped package into the canonical-representative
witness-byte package by discharging the canonical representative source from
the byte source.
-/
def ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputs.toCanonicalRepresentativeWitnessBytePackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputs
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeWitnessBytePackagedSourceRouteInputs
      FieldParser RouteParser WitnessParser P :=
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    directAssemblyBinding := I.directAssemblyBinding
    localModelSource := I.localModelSource
    canonicalRepresentativeSource :=
      canonical_representative_source_of_canonical_representative_byte_source
        I.canonicalRepresentativeByteSource
    nativeDigestFrameWitnessByteSource :=
      I.nativeDigestFrameWitnessByteSource }

/--
P2FS-001 closure from byte-shaped selected Rust source-route evidence,
canonical representative evidence, and digest-frame witness evidence.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_byte_and_witness_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputs
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_witness_byte_packaged_source_route_inputs
    P I.toCanonicalRepresentativeWitnessBytePackagedInputs

/--
Assemble the fully byte-shaped active source-route package from its six
concrete source records.

This is the direct byte-shaped AP2-N001 adapter: once the selected Rust/model
proof supplies these six records, it feeds the active hidden-source semantics
theorem without using the aggregate verifier object as an assembly premise.
The field/local shared-frame wrapper below is the preferred selected
implementation target because it consumes the concrete field/local source
records directly and pins both witness byte sources to one representative
digest frame.
-/
def reviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputsOfSources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (directAssemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (localModelSource :
      ReviewedFq12AssemblyCompatibilityLocalModelSource P)
    (canonicalRepresentativeByteSource :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource
        CanonicalParser P)
    (nativeDigestFrameWitnessByteSource :
      ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
        WitnessParser P) :
    ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputs
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  { roleRootLiftSource := roleRootLiftSource
    selectedSource := selectedSource
    directAssemblyBinding := directAssemblyBinding
    localModelSource := localModelSource
    canonicalRepresentativeByteSource := canonicalRepresentativeByteSource
    nativeDigestFrameWitnessByteSource :=
      nativeDigestFrameWitnessByteSource }

/--
P2FS-001 closure from the six selected Rust/model AP2-N001 source records.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_byte_and_witness_sources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (directAssemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (localModelSource :
      ReviewedFq12AssemblyCompatibilityLocalModelSource P)
    (canonicalRepresentativeByteSource :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource
        CanonicalParser P)
    (nativeDigestFrameWitnessByteSource :
      ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
        WitnessParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_byte_and_witness_packaged_source_route_inputs
    P
    (reviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputsOfSources
      roleRootLiftSource selectedSource directAssemblyBinding localModelSource
      canonicalRepresentativeByteSource nativeDigestFrameWitnessByteSource)

/--
P2FS-001 closure from the Rust-shaped canonical representative witness source
and the Rust digest-frame witness byte source.

This is the Rust-witness-shaped AP2-N001 adapter: the canonical-representative
side may be proved in the same shape as
`Path2ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessV1`, while
the theorem derives the semantic canonical byte source internally.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_witness_and_digest_frame_sources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (directAssemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (localModelSource :
      ReviewedFq12AssemblyCompatibilityLocalModelSource P)
    (canonicalRepresentativeWitnessByteSource :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource
        CanonicalParser P)
    (nativeDigestFrameWitnessByteSource :
      ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
        WitnessParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_byte_and_witness_sources
    P roleRootLiftSource selectedSource directAssemblyBinding localModelSource
    (canonical_representative_byte_source_of_witness_source_and_model
      roleRootLiftSource directAssemblyBinding localModelSource
      canonicalRepresentativeWitnessByteSource)
    nativeDigestFrameWitnessByteSource

/--
Canonical and assembly-side Rust witness byte sources pinned to one selected
native representative digest frame.

Both Rust witness artifacts are byte-level views of the same
`path2_reviewed_fq12_native_representative_digest_v1` frame. The equalities are
kept in the source package so the concrete selected Rust instantiation cannot
silently feed the representative side and the assembly side from different
digest-frame definitions.
-/
structure ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  digestFrame :
    ReviewedFq12NativeRepresentativeDigestFrame
  canonicalRepresentativeWitnessByteSource :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource
      CanonicalParser P
  canonical_digest_frame_eq :
    canonicalRepresentativeWitnessByteSource.digestFrame = digestFrame
  nativeDigestFrameWitnessByteSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
      WitnessParser P
  assembly_digest_frame_eq :
    nativeDigestFrameWitnessByteSource.digestFrame = digestFrame

/--
Lean mirror of the public shared digest-frame witness evidence report.

The active AP2-N001 theorem consumes one Rust representative digest frame
through two byte witnesses: the canonical-representative side and the assembly
representative-frame side.  This report records that the selected Rust
evidence pinned both parsers, both byte certificates, the shared digest-frame
equality checks, verifier bindings, and negative controls before the raw
witness source bundle may be exposed to the five-source closure certificate.
-/
structure ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport where
  schemaMatches : Bool
  reportDigestVerified : Bool
  canonicalWitnessParserPinned : Bool
  digestFrameWitnessParserPinned : Bool
  sharedDigestFramePinned : Bool
  canonicalWitnessByteCertificatePinned : Bool
  digestFrameWitnessByteCertificatePinned : Bool
  representativeDigestFrameSourcePinned : Bool
  canonicalWitnessVerifierPinned : Bool
  digestFrameWitnessVerifierPinned : Bool
  sharedDigestFrameNegativeControlsPassed : Bool

namespace ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport

/-- The public shared witness report has been digest/schema checked. -/
def ReportVerified
    (E : ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport) :
    Prop :=
  E.schemaMatches = true ∧
    E.reportDigestVerified = true

/-- Every shared digest-frame witness component fact in the report holds. -/
def FactsAllTrue
    (E : ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport) :
    Prop :=
  E.canonicalWitnessParserPinned = true ∧
    E.digestFrameWitnessParserPinned = true ∧
    E.sharedDigestFramePinned = true ∧
    E.canonicalWitnessByteCertificatePinned = true ∧
    E.digestFrameWitnessByteCertificatePinned = true ∧
    E.representativeDigestFrameSourcePinned = true ∧
    E.canonicalWitnessVerifierPinned = true ∧
    E.digestFrameWitnessVerifierPinned = true ∧
    E.sharedDigestFrameNegativeControlsPassed = true

end ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport

/--
Proof-bearing selected Rust shared digest-frame witness certificate.

This keeps the AP2-N001 final theorem from accepting an arbitrary
`ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources` value.  The
future Rust/model instantiation must first supply public evidence and proofs
that the evidence report was verified and all component checks passed.
-/
structure ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  evidenceReport :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport
  evidenceReportVerifiedProof :
    evidenceReport.ReportVerified
  evidenceFactsAllTrueProof :
    evidenceReport.FactsAllTrue
  witnessSources :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
      CanonicalParser WitnessParser P

namespace ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate

/-- Expose the shared witness sources only after consuming report proofs. -/
def toWitnessSources
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (Cert :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
      CanonicalParser WitnessParser P :=
  let _hReport : Cert.evidenceReport.ReportVerified :=
    Cert.evidenceReportVerifiedProof
  let _hFacts : Cert.evidenceReport.FactsAllTrue :=
    Cert.evidenceFactsAllTrueProof
  Cert.witnessSources

end ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate

/--
P2FS-001 closure from selected Rust witness byte sources that are explicitly
pinned to the same representative digest frame.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_shared_digest_frame_witness_sources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (directAssemblyBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (localModelSource :
      ReviewedFq12AssemblyCompatibilityLocalModelSource P)
    (witnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P) :
    ReviewedFq12ExtractionCertificateSoundness P := by
  rcases witnessSources with
    ⟨digestFrame, canonicalRepresentativeWitnessByteSource,
      canonical_digest_frame_eq, nativeDigestFrameWitnessByteSource,
      assembly_digest_frame_eq⟩
  let canonicalRepresentativeWitnessByteSourceOnSharedFrame :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource
        CanonicalParser P :=
    { digestFrame := digestFrame
      hasCanonicalRepresentativeWitnessFromCertificates :=
        canonicalRepresentativeWitnessByteSource.hasCanonicalRepresentativeWitnessFromCertificates
      canonicalRepresentativeWitnessAvailableFromCertificates := by
        cases canonical_digest_frame_eq
        exact
          canonicalRepresentativeWitnessByteSource.canonicalRepresentativeWitnessAvailableFromCertificates }
  let nativeDigestFrameWitnessByteSourceOnSharedFrame :
      ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
        WitnessParser P :=
    { digestFrame := digestFrame
      hasDigestFrameWitness :=
        nativeDigestFrameWitnessByteSource.hasDigestFrameWitness
      digestFrameWitnessAvailable := by
        cases assembly_digest_frame_eq
        exact nativeDigestFrameWitnessByteSource.digestFrameWitnessAvailable }
  intro b payload cert hAccept hProjects hHas hValid
  exact
    reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_witness_and_digest_frame_sources
      P roleRootLiftSource selectedSource directAssemblyBinding localModelSource
      canonicalRepresentativeWitnessByteSourceOnSharedFrame
      nativeDigestFrameWitnessByteSourceOnSharedFrame
      hAccept hProjects hHas hValid

/--
Selected-Rust source-route inputs using existing concrete field/local sources
and witness byte sources pinned to one representative digest frame.

This is the most implementation-oriented AP2-N001 wrapper: the assembly field
source supplies the direct assembly binding, the assembly/compatibility field
sources supply the local model, and the shared witness package supplies both
representative byte witnesses under one digest frame.
-/
structure ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  assemblySource :
    ReviewedFq12FieldConcreteAssemblyLocalSource P
  compatibilitySource :
    ReviewedFq12FieldConcreteCompatibilitySource P
  witnessSources :
    ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
      CanonicalParser WitnessParser P

/--
Assemble the selected implementation-oriented P2FS-001 package from the five
source records that still need concrete Rust/model instantiation.
-/
def reviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputsOfSources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (witnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  { roleRootLiftSource := roleRootLiftSource
    selectedSource := selectedSource
    assemblySource := assemblySource
    compatibilitySource := compatibilitySource
    witnessSources := witnessSources }

/--
P2FS-001 closure from the concrete field/local sources and shared-frame witness
sources used by the selected Rust implementation.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_shared_digest_frame_witness_sources
    P I.roleRootLiftSource I.selectedSource I.assemblySource.fieldBinding
    (assembly_compatibility_local_model_source_of_field_sources
      I.assemblySource I.compatibilitySource)
    I.witnessSources

/--
P2FS-001 closure from the five selected Rust/model source records that make up
the field-local shared-frame implementation target.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_sources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (witnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_inputs
    P
    (reviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputsOfSources
      roleRootLiftSource selectedSource assemblySource compatibilitySource
      witnessSources)

/--
P2FS-001 closure from the field/local source records plus the two Rust witness
byte sources and explicit proof that they use the same selected representative
digest frame.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_sources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (digestFrame :
      ReviewedFq12NativeRepresentativeDigestFrame)
    (canonicalRepresentativeWitnessByteSource :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource
        CanonicalParser P)
    (canonical_digest_frame_eq :
      canonicalRepresentativeWitnessByteSource.digestFrame = digestFrame)
    (nativeDigestFrameWitnessByteSource :
      ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
        WitnessParser P)
    (assembly_digest_frame_eq :
      nativeDigestFrameWitnessByteSource.digestFrame = digestFrame) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_sources
    P roleRootLiftSource selectedSource assemblySource compatibilitySource
    { digestFrame := digestFrame
      canonicalRepresentativeWitnessByteSource :=
        canonicalRepresentativeWitnessByteSource
      canonical_digest_frame_eq := canonical_digest_frame_eq
      nativeDigestFrameWitnessByteSource :=
        nativeDigestFrameWitnessByteSource
      assembly_digest_frame_eq := assembly_digest_frame_eq }

/--
Named AP2-N001 raw component package.

This record is the proof-side shape of the active Rust hidden-source semantics
audit gate: every field corresponds to one concrete Rust/model source surface
or to the same-frame witness evidence required by the selected representative
digest route.
-/
structure ReviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser)
    (WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser)
    (P : Predicates) where
  roleRootFields :
    ReviewedFq12ConcreteRoleRootFieldBindingCertificate P
  liftOpeningFunctionality :
    ReviewedFq12CertificateLiftOpeningFunctionality P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  assemblyAvailability :
    ReviewedFq12RepresentativeAssemblyCertificateAvailability P
  assemblyFieldBinding :
    ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P
  assemblyLocalPredicates :
    ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak P
  assemblyNativeValidity :
    ReviewedFq12AssemblyConstructedNativeValidityOrBreak P
  compatibilityAvailability :
    ReviewedFq12ExtractionCompatibilityCertificateAvailability P
  compatibilityFieldBinding :
    ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak P
  compatibilityExtractionRules :
    SameRepresentativeConcreteExtractionPredicates P
  digestFrame :
    ReviewedFq12NativeRepresentativeDigestFrame
  canonicalRepresentativeWitnessByteSource :
    ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource
      CanonicalParser P
  canonical_digest_frame_eq :
    canonicalRepresentativeWitnessByteSource.digestFrame = digestFrame
  nativeDigestFrameWitnessByteSource :
    ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
      WitnessParser P
  assembly_digest_frame_eq :
    nativeDigestFrameWitnessByteSource.digestFrame = digestFrame

/--
Assemble the named AP2-N001 raw component package from the five selected
Rust/model source records.

This is the proof-side certificate constructor that matches the active Rust
audit shape: role-root/lift source, selected source-route bytes, concrete
assembly source, concrete compatibility source, and the shared-frame witness
source package.
-/
def reviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputsOfSources
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (witnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputs
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  { roleRootFields := roleRootLiftSource.roleRootFields
    liftOpeningFunctionality :=
      roleRootLiftSource.liftOpeningFunctionality
    selectedSource := selectedSource
    assemblyAvailability := assemblySource.availability
    assemblyFieldBinding := assemblySource.fieldBinding
    assemblyLocalPredicates := assemblySource.localPredicates
    assemblyNativeValidity := assemblySource.nativeValidity
    compatibilityAvailability := compatibilitySource.availability
    compatibilityFieldBinding := compatibilitySource.fieldBinding
    compatibilityExtractionRules := compatibilitySource.extractionRules
    digestFrame := witnessSources.digestFrame
    canonicalRepresentativeWitnessByteSource :=
      witnessSources.canonicalRepresentativeWitnessByteSource
    canonical_digest_frame_eq := witnessSources.canonical_digest_frame_eq
    nativeDigestFrameWitnessByteSource :=
      witnessSources.nativeDigestFrameWitnessByteSource
    assembly_digest_frame_eq := witnessSources.assembly_digest_frame_eq }

/--
Forget the existing five-source implementation package into the named AP2-N001
raw component package.
-/
def ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs.toByteComponentInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputs
      FieldParser RouteParser CanonicalParser WitnessParser P :=
  reviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputsOfSources
    I.roleRootLiftSource I.selectedSource I.assemblySource
    I.compatibilitySource I.witnessSources

/--
P2FS-001 closure from the raw source components remaining in the selected
Rust/model route. This is the concrete checklist behind the field-local
shared-frame theorem: role-root/lift facts, selected source-route bytes,
assembly field/local/native facts, compatibility field/extraction facts, and
the two witness byte sources pinned to one digest frame.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (roleRootFields :
      ReviewedFq12ConcreteRoleRootFieldBindingCertificate P)
    (liftOpeningFunctionality :
      ReviewedFq12CertificateLiftOpeningFunctionality P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (assemblyAvailability :
      ReviewedFq12RepresentativeAssemblyCertificateAvailability P)
    (assemblyFieldBinding :
      ReviewedFq12ConcreteAssemblyFieldBindingOrBreak P)
    (assemblyLocalPredicates :
      ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak P)
    (assemblyNativeValidity :
      ReviewedFq12AssemblyConstructedNativeValidityOrBreak P)
    (compatibilityAvailability :
      ReviewedFq12ExtractionCompatibilityCertificateAvailability P)
    (compatibilityFieldBinding :
      ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak P)
    (compatibilityExtractionRules :
      SameRepresentativeConcreteExtractionPredicates P)
    (digestFrame :
      ReviewedFq12NativeRepresentativeDigestFrame)
    (canonicalRepresentativeWitnessByteSource :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource
        CanonicalParser P)
    (canonical_digest_frame_eq :
      canonicalRepresentativeWitnessByteSource.digestFrame = digestFrame)
    (nativeDigestFrameWitnessByteSource :
      ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource
        WitnessParser P)
    (assembly_digest_frame_eq :
      nativeDigestFrameWitnessByteSource.digestFrame = digestFrame) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_sources
    P
    { roleRootFields := roleRootFields
      liftOpeningFunctionality := liftOpeningFunctionality }
    selectedSource
    { availability := assemblyAvailability
      fieldBinding := assemblyFieldBinding
      localPredicates := assemblyLocalPredicates
      nativeValidity := assemblyNativeValidity }
    { availability := compatibilityAvailability
      fieldBinding := compatibilityFieldBinding
      extractionRules := compatibilityExtractionRules }
    digestFrame canonicalRepresentativeWitnessByteSource
    canonical_digest_frame_eq nativeDigestFrameWitnessByteSource
    assembly_digest_frame_eq

/--
P2FS-001 closure from the named AP2-N001 raw component package.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_component_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputs
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components
    P I.roleRootFields I.liftOpeningFunctionality I.selectedSource
    I.assemblyAvailability I.assemblyFieldBinding I.assemblyLocalPredicates
    I.assemblyNativeValidity I.compatibilityAvailability
    I.compatibilityFieldBinding I.compatibilityExtractionRules I.digestFrame
    I.canonicalRepresentativeWitnessByteSource I.canonical_digest_frame_eq
    I.nativeDigestFrameWitnessByteSource I.assembly_digest_frame_eq

/--
P2FS-001 closure from the five-source implementation package through the named
AP2-N001 raw component certificate.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_inputs_via_component_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs
        FieldParser RouteParser CanonicalParser WitnessParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_component_inputs
    P I.toByteComponentInputs

/--
P2FS-001 closure from the five selected Rust/model sources through the named
AP2-N001 raw component certificate.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_sources_via_component_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {CanonicalParser :
      ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser}
    {WitnessParser :
      ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser}
    (P : Predicates)
    (roleRootLiftSource :
      ReviewedFq12RoleRootFunctionalLiftSource P)
    (selectedSource :
      SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
        FieldParser RouteParser P)
    (assemblySource :
      ReviewedFq12FieldConcreteAssemblyLocalSource P)
    (compatibilitySource :
      ReviewedFq12FieldConcreteCompatibilitySource P)
    (witnessSources :
      ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources
        CanonicalParser WitnessParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_component_inputs
    P
    (reviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputsOfSources
      roleRootLiftSource selectedSource assemblySource compatibilitySource
      witnessSources)

/--
Selected-Rust packaged source-route adapter using the existing concrete
field/local source records for the local model.

This is an instantiation-oriented convenience wrapper around
`ReviewedFq12SelectedRustPackagedSourceRouteInputs`; it keeps selected
source-route byte evidence responsible for field binding, while accepting the
already-named concrete field/local source records as the local-model evidence.
It is not the active cryptographic closure target: the active target is the
field-local shared-frame source theorem below, which consumes the concrete
field/local records and pins the canonical and assembly witness byte sources
to one selected representative digest frame.
-/
structure ReviewedFq12SelectedRustFieldLocalPackagedSourceRouteInputs
    (FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser)
    (RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser)
    (P : Predicates) where
  roleRootLiftSource :
    ReviewedFq12RoleRootFunctionalLiftSource P
  selectedSource :
    SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource
      FieldParser RouteParser P
  assemblySource :
    ReviewedFq12FieldConcreteAssemblyLocalSource P
  compatibilitySource :
    ReviewedFq12FieldConcreteCompatibilitySource P
  compatibilityRouteWitnesses :
    ReviewedFq12CompatibilityRouteWitnessSource P

/-- Convert existing field/local concrete source records to packaged inputs. -/
def ReviewedFq12SelectedRustFieldLocalPackagedSourceRouteInputs.toPackagedInputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    {P : Predicates}
    (I :
      ReviewedFq12SelectedRustFieldLocalPackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12SelectedRustPackagedSourceRouteInputs
      FieldParser RouteParser P :=
  { roleRootLiftSource := I.roleRootLiftSource
    selectedSource := I.selectedSource
    localModelSource :=
      assembly_compatibility_local_model_source_of_field_sources
        I.assemblySource I.compatibilitySource
    compatibilityRouteWitnesses := I.compatibilityRouteWitnesses }

/--
P2FS-001 closure from selected-Rust packaged inputs backed by existing
field/local concrete source records. This is a convenience adapter theorem;
the active endpoint remains
`reviewed_fq12_certificate_soundness_from_selected_rust_packaged_source_route_inputs`.
-/
theorem reviewed_fq12_certificate_soundness_from_selected_rust_field_local_packaged_source_route_inputs
    {FieldParser : ReviewedFq12FullOpeningFieldEvidenceByteParser}
    {RouteParser : ReviewedFq12FullOpeningFieldSourceRouteByteParser}
    (P : Predicates)
    (I :
      ReviewedFq12SelectedRustFieldLocalPackagedSourceRouteInputs
        FieldParser RouteParser P) :
    ReviewedFq12ExtractionCertificateSoundness P :=
  reviewed_fq12_certificate_soundness_from_selected_rust_packaged_source_route_inputs
    P I.toPackagedInputs

end Path2FullOpeningFieldEvidenceSourceAdapters

end Descender
