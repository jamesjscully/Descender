# Path 2 Native Full Signature Obligation Burn-Down

Date: 2026-06-02

Scope: active deployment path named by Rust production preflight: Path 2 native
/ reviewed Fq12 active-lane projection-carrier route.

## Current Position

`Descender/Path2NativeAccountDesign.lean` now contains the active full-signature
boundary:

- `NativeHashBudgetBoundary`
- `NativeAcceptedSpendSoundnessBoundary`
- `NativeFullSignatureSecurityBoundary`
- `NativeFullSignatureSecurityBoundary.fullSignatureSuccessProb_le_activePathForgeryBound`

The latest reduction batch adds standalone active-path wrappers:

- `Descender/Path2FullSpendGame.lean` for the direct accepted-spend and
  full-forgery lifecycle boundary.
- `Descender/Path2HashBudget.lean` for the executable direct query-budget mirror
  and `NativeHashBudgetBoundary.queryBudgetPriced` adapter.
- `Descender/Path2HashDomainAndMultiTarget.lean` for the active hash-domain
  registry and widened multi-target/account-horizon certificate.
- `Descender/Path2NoExport.lean` for public-output allowlist/denylist closure
  over active accept/reject/storage surfaces.
- `Descender/Path2RoleRootBinding.lean`,
  `Descender/Path2AssemblyConstruction.lean`, and
  `Descender/Path2CompatibilitySoundness.lean` for non-aggregate P2FS-001
  decomposition.
- `Descender/Path2RoleRootConcrete.lean`,
  `Descender/Path2AssemblyNativeValidity.lean`, and
  `Descender/Path2CompatibilityConcrete.lean` for field-level P2FS-001
  residuals.
- `Descender/Path2FullOpeningFieldEvidence.lean`,
  `Descender/Path2FullOpeningFieldEvidencePredicates.lean`, and
  `Descender/Path2FullOpeningFieldEvidenceByteBridge.lean` for the
  pre-aggregate field-evidence mirror, sidecar semantic bridge, byte
  parser/frame hook certificate, and byte-source-to-sidecar constructor.
- `Descender/Path2FullOpeningFieldEvidenceComposition.lean` for packaging
  byte-backed field evidence into the exact active reduction facts used by
  role-root, assembly, and compatibility reductions.
- `Descender/Path2FullOpeningSourceBridge.lean` for non-circular
  binding-or-break assembly/compatibility source obligations that derive
  extraction-certificate soundness without using the aggregate bridge as a
  premise.
- `Descender/Path2FullOpeningSourceConcrete.lean` for packaging the
  deployable non-aggregate sources: assembly availability, field-level
  assembly binding, deterministic assembly construction,
  assembled-representative native validity, compatibility availability,
  field-level compatibility binding, and concrete same-representative
  extraction rules.
- `Descender/Path2FullOpeningFieldEvidenceSourceAdapters.lean` for projecting
  concrete assembly and compatibility field certificates from byte field
  evidence for an already-witnessed role-lift/assembly/compatibility chain.
- `Descender/Path2CheckedCoreByteBridge.lean` for byte/parser certificates that
  imply the checked-core native accept relation.
- `Descender/Path2NoExportConcrete.lean` for tagged accept/reject/storage
  serializer allowlist certificates.
- `Descender/Path2FullSpendConcrete.lean` for the concrete carrier around
  prevalidated slot leaves, admission, checked-core binding, consume-once state,
  and rollback/reorg breaks.

This is the active replacement for trying to complete the q12289 FS-core
full-game proof. The q12289 theorem remains useful as a reference shape, but it
does not certify the current deployment verifier.

2026-06-05 hidden-source byte-domain update: the selected Rust five-source
closure evidence now consumes `Path2NativeHashByteDomainEvidenceV1` instead of
leaving the reviewed-Fq12 hidden-source statement frame domain implicit. The
Rust evidence carries the hash-domain evidence digest and verified bit, checks
that `hidden_source_frame_domain` equals
`REVIEWED_FQ12_HIDDEN_SOURCE_STATEMENT_DOMAIN`, checks that
`reviewed_fq12_hidden_source_statement_frame` starts with the length-delimited
`domain` field from that evidence, and checks that the hidden-source statement
digest is SHA-256 over the same frame. Lean mirrors these facts on
`ReviewedFq12SelectedRustFiveSourceClosureEvidence`. Follow-up in the same
thread added the companion `hidden_source_proof_domain` to
`Path2NativeHashByteDomainEvidenceV1` and binds
`reviewed_fq12_hidden_source_proof_transcript_frame` /
`reviewed_fq12_hidden_source_proof_transcript_digest` to
`REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_DOMAIN`. The projection-carrier binding was
then factored through `reviewed_fq12_state_assisted_projection_carrier_binding_frame`,
and the five-source evidence now checks that this frame uses the
`projection_carrier_domain` from the same hash byte-domain evidence and that
its digest splits into the carrier check bytes plus transcript-binding bytes.
The same evidence object now also carries the account-witness opening,
pad-seed opening, and pad-opening-view domains; the five-source evidence checks
the corresponding named frame helpers and public pad-opening frame hash to the
digests consumed by the active path. Follow-up direct-mode hardening now records
that the selected `Path2DirectQueryBudgetV1` schema/mode are active, `q_batch`
is excluded from active query symbols, `q_batch` is listed as inactive, optional
aggregation terms remain `q_batch`-only, and the Lean
`path2_direct_query_budget_keeps_q_batch_inactive` surface is present. This
narrows P2FS-006 to the final selected-Rust/model instantiation theorem; it
does not complete hidden-source semantics by itself.

2026-06-05 certificate-route update: the active boundary now has direct
certificate-level adapters from
`ReviewedFq12SelectedRustModelFiveSourceClosureCertificate` into hidden-source
closure, accepted-spend closure, and the full-signature handoff. The Rust
future-theorem source gate now requires the selected Rust/model instantiation
to construct the all-certificate package and close through
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_certificate`,
keeping the aggregate report, sub-evidence digest match, and per-source
certificates on the final proof surface. This is still a target-shape
hardening; the concrete selected-source definitions and final theorem remain
open.

2026-06-05 native-semantics package handoff update: the hidden-source boundary
now names `ReviewedFq12SelectedRustNativeBoundarySemanticsPackage`, a package
containing the active `Path2NativeBoundary.Predicates` instance plus the
selected Rust/model source-record package for exactly that predicate set. Lean
threads that package through accepted-spend closure, full-signature closure,
and now
`full_signature_active_path_forgery_bound_from_selected_rust_native_boundary_semantics_package`,
which applies the existing active-path forgery-bound theorem to the wrapped
full-signature boundary. This is a target-shape reduction only. It does not
construct the native-boundary predicate model or any of the seven raw selected
source records, and the Rust manifest keeps those gaps fail-closed.

2026-06-05 predicate-model source-target update: Lean now also names
`ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource` and
`ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage`. This is the
non-vacuous target shape for AP2-N001: first import the active selected Rust
native-boundary predicate interpretation, then supply the seven selected source
records over exactly `predicateModel.toPredicates`, then forget that source
package into the existing semantics package. The Rust manifest records this
target and its accepted-spend/full-signature/bound handoffs separately from
`native_boundary_concrete_predicate_model_present`, which remains false.
The source target now also carries anti-vacuity obligations:
`payloadAcceptanceNotVacuous` rules out the always-false reviewed-Fq12 payload
acceptance predicate, and `boundaryBreaksNotUniversal` rules out closing every
semantic implication through a universal break predicate. The selected evidence
manifest records these as separate subcheck bits, so target-shape readiness and
anti-vacuity readiness remain independently visible.
The predicate-model target also now records concrete binding obligations:
`ReviewedFq12PayloadAcceptanceMatchesBoundary` ties accepted reviewed-Fq12
payloads to the native boundary digest and slot, while
`ReviewedFq12ExtractionCertificateMatchesPayload` ties extraction certificates
to the same boundary and payload statement digests.
`ReviewedFq12ExtractionCertificateValidityMatchesPayload` applies the same
binding discipline to the separately consumed certificate-validity predicate.
`ReviewedFq12CertificateExtractionImpliesPayloadExtraction` ties certificate
extraction to payload extraction for the same hidden representative, preventing
the two extraction predicates from drifting apart in the selected model.
`ReviewedFq12PayloadExtractionInBoundaryFiber` then requires payload extraction
to land in the native boundary fiber for that boundary.
`ReviewedFq12PayloadExtractionNativeValid` additionally requires extracted
payload representatives to satisfy the native local-valid and
descent-consistent predicates for the boundary context.
`ReviewedFq12RoleLiftValidityBindsExtractionTarget` requires valid role-lift
certificates to satisfy the native bridge relation
`ReviewedFq12RoleLiftCertificateBindsExtraction`, tying full-native openings
back to the same selected extraction certificate and payload.
`ReviewedFq12RoleLiftOpeningImpliesCertificateLiftTarget` then ties the
role-lift opening predicate to `reviewedFq12CertificateLiftsToOpening` for the
same full native opening.
`ReviewedFq12AssemblyValidityBindsOpeningTarget` requires valid native assembly
certificates to bind to the opened tuple, and
`ReviewedFq12AssemblyCertificateImpliesFullNativeAssemblyTarget` ties
assembly-certificate assembly to `fullNativeOpeningAssemblesRepresentative`.
`ReviewedFq12CompatibilityValidityBindsTarget` requires valid compatibility
certificates to bind to the same payload, extraction certificate, opening, and
representative, while
`ReviewedFq12CompatibilityExtractionImpliesReviewedExtractionTarget` ties
compatibility-certificate extraction to both reviewed-Fq12 extraction
predicates.
`ReviewedFq12FullOpeningBridgeDigestValidityBindsTarget` requires the
aggregate full-opening bridge digest certificate validity predicate to enforce
the digest binding across payload, extraction, role-lift, assembly, and
compatibility certificates.
The predicate-model source no longer carries the six generic `Prop` evidence
labels for active-model pinning and modeled surfaces. The selected evidence
manifest now reports
`native_boundary_predicate_model_remaining_generic_evidence_label_count = 0`,
so AP2-N001 is blocked on the real concrete active
`Path2NativeBoundary.Predicates` interpretation, or an equivalent theorem
package for exactly the predicate model consumed by the hidden-source
boundary.
Follow-up target shaping now names that replacement object explicitly:
`ReviewedFq12SelectedRustNativeBoundaryPredicateModelEvidenceReport` records
the public predicate-model report bits, and
`ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate` carries the
report/digest proofs, the concrete `Predicates` value, and every non-vacuity
and binding obligation currently required by the predicate-model source. Its
`.toSource` adapter is the intended handoff into
`ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage`. This still
does not close AP2-N001; it makes the missing concrete Rust predicate-model
certificate the next named import rather than a collection of bare `Prop`
labels.
The route is now wired into
`ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage`, which
contains the predicate-model certificate and the selected source-record package
for `predicateModelCertificate.toSource.toPredicates`. That certified package
has accepted-spend, full-signature, and active-forgery-bound handoffs. The
manifest exposes these as the certified semantics-source package target,
handoff, and bound-handoff bits, all while
`native_boundary_concrete_predicate_model_present` remains false.
Rust now also emits
`Path2ReviewedFq12NativeBoundaryPredicateModelEvidenceV1`, a schema/digest
verified report for this predicate-model surface. It is intentionally
fail-closed: report verification succeeds, but `all_facts` is false because
`concrete_predicate_model_present`, `active_boundary_predicate_model_pinned`,
and the derived pinned model facts remain false until the concrete active
`Path2NativeBoundary.Predicates` interpretation is imported.
Lean mirrors that public report as
`selectedRustNativeBoundaryPredicateModelEvidenceReport`, records
`selectedRustNativeBoundaryPredicateModelEvidenceDigest`, and proves the report
verification predicate. It also proves the current `FactsAllTrue` obligation is
impossible, so the predicate-model certificate category remains open rather
than being closed by a report-only artifact.
The concrete-definition checklist now exposes this split directly:
`predicate_model_report_surface_definitions_present = true`, while
`predicate_model_facts_proof_definition_present = false`,
`predicate_model_certificate_definition_present = false`, and the aggregate
`predicate_model_certificate_definitions_present = false`.

2026-06-05 concrete-definition blocker update: the Rust bridge audit now has a
separate `selected_rust_model_concrete_definitions_present` status bit. It
currently stays false, even while the five-source route/evidence surfaces are
ready, because the active Lean module does not yet define the selected Rust
five-source evidence reports, evidence digests, per-source proof objects,
source records, or the aggregate/sub-evidence digest-match proof as concrete
`def`/`theorem` declarations. This bit is part of
`semantics_bridge_complete` and is digest-bound in the audit frame, but it is
not treated as a non-closing readiness fact.

2026-06-05 concrete-definition checklist update: the concrete-definition
blocker is now backed by
`Path2SelectedRustModelConcreteDefinitionChecklistV1`. The checklist is
digest-bound in the bridge audit and splits the final-definition work into
six categories: selected Rust five-source evidence reports, evidence digests,
report/facts proof objects, the native-boundary predicate-model certificate,
concrete source records, and the aggregate sub-evidence digest-match proof.
The final theorem target is now mechanically visible.

2026-06-05 certified predicate-model theorem-gate update: the AP2-N001 final
theorem scanner no longer accepts the stale shape that stops at constructing
`ReviewedFq12SelectedRustModelFiveSourceClosureCertificate` and applying the
old hidden-source certificate adapter. A future accepted theorem must also
route through
`ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage`, using
`selectedRustNativeBoundaryPredicateModelCertificate` and
`selectedRustModelSourceRecordPackage`, expose the package
`.hiddenSourceClosure`, and name the certified-package full-signature handoff
plus
`full_signature_active_path_forgery_bound_from_selected_rust_native_boundary_certified_semantics_source_package`.
The source gate rejects predicate-model evidence/certificates/source packages
as theorem assumptions, rejects vacuous local predicate-model report/proof
construction, and rejects omission of the active-bound handoff. AP2-N001
therefore remains fail-closed until the concrete native-boundary predicate
model certificate and selected source-record package are genuinely imported.

2026-06-05 predicate-model certificate constructor update:
`ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate` now exposes
named constructor `ofReportAndModel` for the public predicate-model
report/digest/proofs plus every concrete native-boundary model obligation. The
concrete-definition checklist requires
`selectedRustNativeBoundaryPredicateModelCertificate` to use that constructor;
a certificate definition with the right name and type but without the
constructor no longer completes the predicate-model certificate category. This
keeps the final native-boundary semantics package from being sourced by an
anonymous certificate record.
The checklist now requires that certificate constructor body to name every
selected model-obligation argument: non-vacuity, payload/boundary binding,
certificate/payload binding, payload extraction consistency, role-lift,
assembly, compatibility, and full-opening bridge digest binding. Omitting one
of those obligations is a negative-control failure for
`predicate_model_certificate_definition_present`.

2026-06-05 source-record package constructor update:
`ReviewedFq12SelectedRustModelSourceRecordPackage` now exposes named
constructor `ofSources` for the seven selected Rust/model source records. The
concrete-definition checklist now treats `selectedRustModelSourceRecordPackage`
as part of `source_record_definitions` and requires it to be built through
`ofSources`; a same-name/same-type anonymous source-record package no longer
completes that category. This makes the package consumed by the certified
native-boundary semantics route constructor-bound, not merely type-shaped.
`ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource.toPredicates` is
now prefix-bound to `Model.predicates`; an alias return plus an unused
`Model.predicates` mention no longer satisfies the source-target subcheck.
The certified semantics source-package target bit is now scoped to the named
structure/method declarations, with `hiddenSourceClosure` prefix-bound to
`(Package.toSemanticsSourcePackage).hiddenSourceClosure`; alias-returning
closures and unrelated marker placement are rejected.
The old current-report impossible guards have been retired: the active
predicate-model report is now fact-complete, and the selected
predicate-model certificate is constructed from the fixed-point source.
The concrete and source-shaped semantics package target bits are now scoped
the same way: concrete closure must start from the selected source-record
package theorem, and source-package projection must start from
`Package.predicateModel.toPredicates` and `Package.sourceRecords`.
The predicate-model certificate constructor check is now prefix-bound on
`selectedRustNativeBoundaryPredicateModelCertificate` itself; an alias-bound
certificate plus an unused `ofReportAndModel` mention no longer completes the
predicate-model certificate category.

2026-06-05 source-record predicate-binding update: the concrete-definition
checklist now requires every selected source-record definition and certificate
definition to be over
`selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates`.
A complete-looking source package over a generic `P` no longer completes
`source_record_definitions`, preventing source records from drifting away from
the certified active native-boundary predicate model.
The final theorem gate now also requires the theorem body itself to construct
`selectedRustModelSourceRecordPackage` via
`ReviewedFq12SelectedRustModelSourceRecordPackage.ofSources`; merely naming a
pre-existing source-record package no longer satisfies the AP2-N001 theorem
source scanner.
The source-record/package constructor checks now require constructor-prefixed
definition bodies, so an alias-bound `selectedRustModelSourceRecordPackage`
with an unused `ofSources` mention no longer completes the checklist or
source-record subcheck.
The five selected evidence certificate constructor-bound bits in the
source-record subcheck now use the same prefix-body rule, preventing alias-bound
certificates from reducing the missing source-record count.
The same gate now rejects a final hidden-source theorem over generic
`P : Predicates`; the accepted target must conclude
`ReviewedFq12HiddenSourceProofSemanticsClosed
selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates`.

2026-06-05 certified semantics package constructor update:
`ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage` now
exposes named constructor `ofCertificateAndSourceRecords` for the
constructor-bound predicate-model certificate and constructor-bound selected
source-record package. The final AP2-N001 theorem gate requires this
constructor before accepting `.hiddenSourceClosure` or the certified-package
full-signature bound handoff; an anonymous certified semantics package no
longer satisfies the theorem source scanner.
The same scanner now requires the final theorem to `exact` the certified
package's `.hiddenSourceClosure`; an unused local mention of that closure no
longer satisfies the final AP2-N001 gate.
It also requires the certified package binding itself to call
`ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage.ofCertificateAndSourceRecords`;
an alias-bound package plus an unused constructor mention is rejected.

2026-06-05 sub-evidence digest-match route update: the concrete-definition
checklist now requires `selectedRustFiveSourceSubEvidenceDigestsMatchProof` to
derive from `selectedRustModelSourceRecordPackage.subEvidenceDigestsMatch`.
A same-type proof with the right theorem name but without the constructor-bound
source-record package reduction no longer completes
`sub_evidence_digest_match_proof_present`.
This check is now prefix-bound on the proof body, so an alias-bound proof plus
an unused package-proof mention is also rejected.
The source-record package now also has
`toFieldLocalSharedDigestFrameWitnessInputs`, and the source-record subcheck
requires that reduction surface. This pins the package to the preferred
field/local shared-frame AP2-N001 theorem target in addition to the aggregate
five-source and byte-component projections.
The final theorem scanner now requires the future accepted theorem to bind the
same projection as `selectedRustFieldLocalSharedDigestFrameWitnessInputs`;
dropping that binding is a negative-control failure.
The same scanner now rejects theorem-header assumptions for the predicate-model
obligation proofs and local `let` rebinding of
`selectedRustNativeBoundaryPredicateModelCertificate`, keeping the final route
on the selected certified predicate model rather than a local certificate
stub.
Lean now constructs the proof-bearing selected predicate-model certificate
from the fact-complete predicate-model report and fixed-point source. The
package-level blocker has moved from predicate-model import to the final
extractor theorem/semantics bridge instantiation.
The concrete predicate-model detector now requires the full reviewed-Fq12
native-boundary predicate surface, not only accept/project/certificate-valid
fields. A partial selected `Path2NativeBoundary.Predicates` record remains
fail-closed, so `native_boundary_concrete_predicate_model_present` only turns
true for a model that exposes the role-lift, assembly, compatibility,
full-opening bridge digest, payload extraction, and break predicates as well.
The detector is declaration-body scoped to
`selectedRustNativeBoundaryPredicates`, so nearby or unrelated marker text no
longer contributes to the concrete predicate-model import bit.
The predicate-model evidence report now mirrors the Rust frame's
`concrete_predicate_model_present` and
`certified_semantics_source_package_target_present` facts in Lean, and
`FactsAllTrue` requires both. The report remains verified but not
fact-complete until the concrete native-boundary predicate model is imported.
The selected evidence manifest now prints every predicate-model all-facts
input, making the remaining false pins visible directly in the deployment
artifact instead of only through the hashed evidence frame.
The Rust source audit now checks the predicate-model source, evidence report,
certificate, and `toSource` adapter in their named Lean declaration windows,
with a split-marker regression so unrelated declarations cannot satisfy the
target bits.
The selected source-record completion bit is now also gated by the concrete
native-boundary predicate model. A fixture with all source-record definitions
but no concrete `selectedRustNativeBoundaryPredicates` model no longer reports
`all_selected_source_records_present`; the positive fixture must include both
the full active predicate model and the source-record package.
The concrete-definition checklist now has the same dependency: the selected
predicate-model certificate must call `ofReportAndModel` with
`selectedRustNativeBoundaryPredicates`, and the predicate-certificate/all-
definition bits still require the full concrete model detector. A generic `P`
certificate can no longer satisfy the active checklist.
The concrete predicate-model detector now requires the full
`Path2NativeBoundary.Predicates` record surface, including account/admission,
transparent-view, trace-assembly, slot-authorization, and spend predicates. A
reviewed-Fq12-only selected record is no longer enough to flip the concrete
model bit.
It also requires concrete declarations for every selected predicate-field
provider consumed by `selectedRustNativeBoundaryPredicates`. The manifest now
exposes `native_boundary_predicate_model_field_sources_defined_present`; a
field-complete record with dangling provider identifiers remains non-closing.
That provider-definition requirement is now included in the digest-bound
predicate-model evidence report by making
`nativeBoundaryPredicateRecordComplete` require both the native predicate record
surface and the selected provider definitions. The current Lean report/digest
has that fact false, matching Rust.
The source audit now also rejects vacuous selected predicate-field provider
bodies, such as defining `selectedReviewedFq12PayloadAccepts` as a
constant-false predicate. The manifest exposes this as
`native_boundary_predicate_model_forbidden_vacuous_field_sources_present`, and
the digest-bound predicate-model negative-control bit requires it to be false.
The same bit now also rejects direct constant-lambda provider bodies across
the selected provider surface, including simple parenthesized wrappers and
active account-admission placeholders such as
`selectedAdmitted := fun _ _ => True`.
It now also rejects the obvious syntactic vacuity escape hatch: a field-complete
`selectedRustNativeBoundaryPredicates` whose critical break, reviewed-Fq12
certificate/extraction, payload-extraction, or spend predicates are filled by
constant `True`/`False` lambdas. The manifest surfaces this as
`native_boundary_predicate_model_forbidden_vacuous_literals_present`; the
digest-bound predicate-model report includes it in
`predicate_negative_controls_passed`, and the concrete-model bit requires it
to be false.
Concrete predicate-model completion now also requires the selected predicates
to be projected from the named
`selectedRustNativeBoundaryPredicateFieldSources` package. A raw
field-complete `Path2NativeBoundary.Predicates` literal is rejected, and the
same vacuous-lambda detector scans the package body as well as the projected
predicate model. The manifest exposes the package-route bit as
`native_boundary_predicate_model_field_source_package_present`.
The selected predicate-model certificate now carries that same package route:
the certificate stores the selected field-source package, records
`predicates = predicateFieldSources.toPredicates`, and the named constructor
takes the field-source package before the model obligations. The Rust checklist
rejects a selected certificate built directly from the raw
`selectedRustNativeBoundaryPredicates` record, and
`nativeBoundaryPredicateRecordComplete` now requires both concrete provider
definitions and the package-route bit.
The route is now certificate-gated: Lean defines
`ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate`,
whose constructor binds the selected report/digest, report verification, the
package-present report fact, and the field-source package. The predicate-model
certificate consumes this field-source certificate rather than the raw package,
and the manifest exposes
`native_boundary_predicate_field_source_certificate_target_present`.
The package-route bit is now also a standalone selected predicate-model
evidence fact, mirrored in Lean as
`nativeBoundaryPredicateFieldSourcePackagePresent` and in Rust/manifest output
as `native_boundary_predicate_field_source_package_present`. The predicate
evidence digest was updated so the current false package-route fact is
explicitly public and hash-bound.
The same detector now rejects module-level `axiom`, `constant`, `opaque`, or
`variable` stubs for the selected predicate-field providers used by
`selectedRustNativeBoundaryPredicates`. The manifest exposes this as
`native_boundary_predicate_model_stub_field_sources_present`, and the
digest-bound negative-control bit also requires it to be false.
That stub bit now also covers obvious `selected...Value` dependency stubs for
the selected providers, preventing a concrete provider declaration from simply
aliasing an axiom/constant placeholder with the same selected-provider stem.
The provider-completeness detector now accepts only concrete `def`/`abbrev`
value declarations for those selected predicate-field providers. A
theorem-shaped replacement for a critical reviewed-Fq12 provider leaves both
the provider-completeness bit and concrete active predicate-model bit false.
It also exposes
`native_boundary_predicate_model_incomplete_proof_terms_present`; `sorry` or
`admit` tokens outside comments/strings now block concrete-model completion
and fail the digest-bound negative-control bit.
The focused bridge regression now parses the Lean
`selectedRustNativeBoundaryPredicateModelEvidenceDigest` byte-array constant
and compares it to the Rust-computed predicate-model evidence digest, matching
the existing six selected source-evidence digest checks.
It now also parses the boolean fields in
`selectedRustNativeBoundaryPredicateModelEvidenceReport` and compares them to
the Rust predicate-model evidence frame, so the visible fail-closed blocker
bits cannot drift independently of the digest-bound Rust evidence.

2026-06-05 concrete-definition checklist facade update: the production-reference
facade now exports the checklist schema, type, constructor, frame, digest, and
verifier, and the Path 2 deployment facade source gate requires those exports.
The facade crate test checks that the current public checklist remains
canonical and fail-closed.
This exposes the next proof blocker to deployment tooling while keeping
selected-source helpers out of the curated facade.

2026-06-05 selected-report definition update: Lean now defines the six selected
Rust/model evidence report values named by the checklist. They are explicit
fail-closed report values: verification and component bits remain false, and no
`ReportVerified`, `FactsAllTrue`, selected source-record, or sub-evidence
digest-match proof is supplied. This preserves the AP2-N001 fail-closed bridge
status.

2026-06-05 report-definition checklist hardening: the concrete-definition
checklist now requires each selected report declaration to mention its expected
Lean report type. A name-only report fixture remains fail-closed, keeping the
report category tied to the selected Rust/model evidence-report structures.

2026-06-05 selected-evidence digest update: `tools/parameter-search` now has
`path2_selected_evidence_digest_manifest`, a source-controlled digest manifest
binary for the six public selected evidence reports. Lean mirrors the emitted
digests as concrete `selectedRust*EvidenceDigest` byte-array definitions in
`Path2HiddenSourceSemanticsBoundary.lean`; the hidden-source bridge regression
now parses those Lean constants and checks them against the Rust-computed
aggregate and sub-evidence digests. The concrete-definition checklist now has
report and digest categories present, with proof objects, source records, and
the aggregate sub-evidence digest-match proof still missing.

2026-06-05 proof-definition checklist hardening: the concrete-definition
checklist now requires selected proof declarations to name both their selected
report and the exact `ReportVerified` or `FactsAllTrue` predicate. A regression
fixture with all proof names but no report/predicate type shape still leaves
the proof-object category false. This keeps the remaining proof blocker from
being satisfied by untyped aliases.

2026-06-05 source-record checklist hardening: the concrete-definition checklist
now requires selected source-record declarations to mention the active source
type or predicate they provide: payload projection, extraction availability,
role-root/lift, source-route byte source, field/local assembly,
compatibility, and shared-frame witness source. A name-only source-record
fixture remains fail-closed, keeping the source-record category tied to the
final certificate's expected source contracts.

2026-06-05 sub-evidence match checklist hardening: the concrete-definition
checklist now requires `selectedRustFiveSourceSubEvidenceDigestsMatchProof` to
name `SubEvidenceDigestsMatch` and the five boundary-base, role-root/lift,
source-route, field/local, and shared-witness certificates. A name-only theorem
fixture remains fail-closed, keeping the aggregate digest-match proof tied to
the constructor surface that consumes it.

2026-06-02 update: `Path2FullOpeningSourceConcrete.lean` now packages the
deployable non-aggregate theorem route as `ReviewedFq12DeployableHiddenSourceInputs`. The remaining P2FS-001 proof inputs are
no longer an abstract source-bridge instantiation; they are the concrete
role-root bridge, named assembly availability,
`ReviewedFq12ConcreteAssemblyFieldBindingOrBreak`, deterministic assembly
predicate facts, native-fiber/local/descent facts for the constructed
representative, compatibility availability,
`ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak`, and concrete
same-representative extraction rules. The compatibility-indexed field-evidence
byte source remains a separate Rust parser/digest bridge, not the generic
source of pre-compatibility assembly. The concrete source package also includes
strict-source adapters and field-or-break adapters: exact assembly and
compatibility binding verifier theorems are sufficient to instantiate the
binding-or-break bridge surface, while assembly and compatibility binding can
also be supplied through `ReviewedFq12AssemblyConcreteFieldCertificate` and
`SameRepresentativeConcreteFieldCertificate` field certificates. The active theorem route is
`reviewed_fq12_certificate_soundness_from_deployable_hidden_source_inputs`, over a single `ReviewedFq12DeployableHiddenSourceInputs` package:
assembly availability is supplied by
`ReviewedFq12AssemblyAvailabilityOrBreak`, assembly binding by
`ReviewedFq12ConcreteAssemblyFieldBindingOrBreak`, deterministic assembly
local facts by `ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak`,
native validity by `ReviewedFq12AssemblyConstructedNativeValidityOrBreak`,
compatibility availability by `ReviewedFq12CompatibilityAvailabilityOrBreak`,
compatibility binding by
`ReviewedFq12ConcreteCompatibilityFieldBindingOrBreak`, and extraction by
`ReviewedFq12ConcreteCompatibilityExtractionOrBreak`.
`Path2FullOpeningFieldEvidenceSourceAdapters.lean` separately projects the
same concrete assembly and compatibility field certificates from byte field
evidence for an already-witnessed chain; this remains a compatibility-indexed
parser/digest adapter, not generic assembly availability. It now also names
`ReviewedFq12OpeningRoleLiftWitnessSource`,
`ReviewedFq12AssemblySourceRouteWitnessSource`,
`ReviewedFq12CompatibilitySourceRouteWitnessSource`,
`source_route_byte_source_assembly_field_binding_of_route_witness`,
`source_route_byte_source_compatibility_field_binding_of_route_witness`,
`source_route_byte_source_compatibility_extraction_of_route_witness`, and
`ReviewedFq12SourceRouteDeployableHiddenSourceInputs`, making the extra route
witnesses explicit: source-route binding requires the selected role-lift
certificate for the semantic opening plus assembly/compatibility witnesses
connecting the generic representative to the constructed representative.
Rust now also exposes `Path2ReviewedFq12FullOpeningFieldSourceRouteV1`, a
digest-bound source-route certificate over the same ordering: the assembly
certificate binds the full-opening fields, the compatibility certificate binds
the same representative fields, and the field evidence is only an adapter with
`aggregate_bridge_used_as_source = false`. The targeted regression now pins
this route before constructing the aggregate bridge and rejects aggregate-source
misuse, stale source-route digests, bad assembly field sources, and bad
compatibility field sources. The Rust production domain registry now also
includes the reviewed-Fq12 assembly, compatibility, field-evidence,
source-route, and aggregate-bridge frame domains, covered by the global
disjointness regression.
Lean now mirrors this Rust route with
`ReviewedFq12FullOpeningFieldSourceRouteCertificate`, byte-parser/source
structures, `source_route_byte_predicate_source_active_reduction_facts_or_named_break`,
and `source_route_byte_source_concrete_field_certificates_or_named_break`.
The byte bridge also adds
`field_evidence_byte_source_of_source_route_byte_source`, which lets the
stronger paired source-route theorem feed callers that only need the plain
field-evidence byte-source interface while retaining the route frame inside the
derived predicate.
It also exposes ordinary-boundary-break projections for combined, assembly-only,
and compatibility-only field certificates:
`source_route_byte_source_concrete_field_certificates_or_boundary_break`,
`source_route_byte_source_assembly_concrete_fields_or_boundary_break`, and
`source_route_byte_source_compatibility_concrete_fields_or_boundary_break`.
The route-aware local endpoints now also expose ordinary assembly predicate,
native-validity, and compatibility-extraction facts:
`source_route_byte_source_local_assembly_predicates_or_break`,
`source_route_byte_source_local_native_validity_or_break`, and
`source_route_byte_source_local_compatibility_extracts_or_break`. Source
adapters now package local assembly predicates and native validity into the
concrete-source certificate shapes with
`byte_source_concrete_local_assembly_predicates_or_boundary_break`,
`source_route_byte_source_concrete_local_assembly_predicates_or_boundary_break`,
`byte_source_concrete_native_validity_or_boundary_break`, and
`source_route_byte_source_concrete_native_validity_or_boundary_break`. The
plain byte-field source-adapter theorem
`byte_source_concrete_compatibility_extraction_or_boundary_break` and the
source-route boundary theorem
`source_route_byte_source_concrete_compatibility_extraction_or_boundary_break`
turn local compatibility extraction facts into the concrete compatibility
extraction certificate for the witnessed deterministic representative, or
export the ordinary boundary break. The witnessed-chain package
`ReviewedFq12FullOpeningFieldEvidenceWitnessedConcreteSourceCertificates`
groups the resulting concrete fields, named local assembly facts, native
validity, and concrete compatibility extraction certificate for one
already-witnessed role-lift/assembly/compatibility chain; the plain byte and
source-route constructors are
`byte_source_witnessed_concrete_source_certificates_or_boundary_break` and
`source_route_byte_source_witnessed_concrete_source_certificates_or_boundary_break`.
The source-route deployable package
`ReviewedFq12SourceRouteDeployableHiddenSourceInputs` now derives
`ReviewedFq12AssemblyConstructedLocalPredicatesOrBreak` via
`source_route_byte_source_assembly_local_predicates_of_route_witness`,
`ReviewedFq12AssemblyConstructedNativeValidityOrBreak` via
`source_route_byte_source_assembly_native_validity_of_route_witness`, and
`ReviewedFq12ConcreteCompatibilityExtractionOrBreak` via
`source_route_byte_source_compatibility_extraction_of_route_witness`, so the
route plus local predicate source supplies source-route field bindings,
deterministic assembly local/native facts, and compatibility extraction while
role-root plus assembly/compatibility availability remain explicit.
`ReviewedFq12SourceRouteLocalInstantiationInputs` is now an intermediate
source-route package. The boundary-availability Lean-side package before the
Rust verifier/model theorem is
`ReviewedFq12BoundaryAvailabilityConcreteRoleRootSourceRouteInputs`, which
repackages base active-boundary assembly/compatibility availability and
concrete role-root field binding into the source-route endpoint. It still
requires the byte predicate source, local predicate source, and route witnesses
for the selected parser/frame/digest implementation.
`ReviewedFq12FactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs`
now splits those route witnesses into an opening-to-role-lift witness, a
constructed-assembly compatibility witness, and a compatibility
assembly-representative witness.
`SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource` now names the
selected Rust source-route byte-source decomposition contract, and
`ReviewedFq12SelectedRustFactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs`
replaces the generic byte predicate source. The compact-local endpoint is
`ReviewedFq12SelectedRustFactoredConcreteLocalSourceRouteInputs`, which also
replaces the free local predicate source with compact assembly
availability/local/native and compatibility availability/extraction-rules
records.
`ReviewedFq12SelectedRustFactoredConcreteLocalFunctionalLiftSourceRouteInputs`
derives the opening-to-role-lift
witness from the role-root bridge plus
`ReviewedFq12CertificateLiftOpeningFunctionality`, so the remaining lift
blocker is functionality of the selected certificate lift relation rather than
an opaque witness source.
`ReviewedFq12SelectedRustConcreteLocalFunctionalLiftSourceRouteInputs` keeps
that functional-lift derivation and bundles the two compatibility-side route
witnesses into `ReviewedFq12CompatibilityRouteWitnessSource`.
`ReviewedFq12SelectedRustPackagedSourceRouteInputs` groups role-root/lift facts as
`ReviewedFq12RoleRootFunctionalLiftSource` and assembly/compatibility local
facts as `ReviewedFq12AssemblyCompatibilityLocalModelSource`.
That packaged endpoint is now an adapter under the field-local shared-frame
source theorem
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_sources`,
which is the active P2FS-001 closure target. The older field/local packaged
record remains a convenience adapter that consumes already-named concrete
field/local assembly and compatibility source records and forgets their
redundant field-binding facts, because selected source-route bytes supply
those bindings. The Rust hidden-source semantics audit now digest-binds the
packaged endpoint plus the field/local shared-frame endpoint presence
with source-route local-instantiation, boundary-availability/concrete-role-root
source-route endpoint, factored boundary-availability/concrete-role-root
source-route endpoint, selected-Rust source-route byte-source endpoint,
selected-Rust compact-local endpoint, selected-Rust functional-lift endpoint,
selected-Rust witnessed functional-lift endpoint, selected-Rust packaged
endpoint, selected-Rust field/local packaged adapter,
concrete role-root field binding, certificate lift-opening functionality,
assembly representative digest lookup source, compatibility route witness
source, assembly/compatibility local-model source, field-evidence, source-route, and
source-route negative-control flags; the targeted Rust test now also rejects
removing the narrowed Lean endpoint flag, the factored Lean endpoint flag, the
selected-Rust source-route endpoint flag, the compact-local endpoint flag, or
the witnessed functional-lift endpoint flag, the packaged endpoint flag, the
field/local packaged adapter flag, the lift-functionality flag, the
assembly representative digest lookup flag, the compatibility route witness
source flag, or the assembly/compatibility local-model source flag. It remains
fail-closed because `extractor_theorem_instantiated` and
`semantics_bridge_complete` are still false. The production hidden-source
semantics gate and readiness report now carry the bridge-audit digest and
`semantics_bridge_complete` bit, so deployability is tied to the narrowed audit
surface and cannot pass merely by satisfying the acceptance-obligation bitmap.
The assembly/compatibility local-model, constructed-assembly compatibility
witness, and compatibility route witness source flags are now backed by
`path2_reviewed_fq12_assembly_compatibility_local_model_source_present_v1`,
which source-inspects the active assembly constructor/verifier, compatibility
constructor/verifier, same-representative predicate, and source-route
constructor/frame/verifier.
The field evidence, source-route, and source-route negative-control flags are
also source-inspected by
`path2_reviewed_fq12_full_opening_field_evidence_source_present_v1`,
`path2_reviewed_fq12_full_opening_source_route_source_present_v1`, and
`path2_reviewed_fq12_source_route_negative_controls_present_v1`.
The selected hidden-relation extraction-certificate flag is source-inspected by
`path2_reviewed_fq12_hidden_relation_extraction_certificate_source_present_v1`,
covering payload digest checks, statement/opening/pad binding,
account-witness opening-frame recomputation, certificate framing, and the
statement-rebinding regression.
The upstream hidden-source payload binding is now source-inspected by
`path2_reviewed_fq12_hidden_source_payload_binding_source_present_v1`; the audit
uses it for native-boundary payload binding, slot/opening binding, projection
carrier binding, the accepting wrapper path through extraction, and the active
rebinding controls. That source check requires the constructor to assemble the
native boundary, bind the prevalidated leaf to the projection bundle, recompute
the reviewed creation statement and opening commitment, digest the carrier and
checked-core frame/proof bytes, frame every payload field, and reject rebound
payloads by verifier reconstruction.
The creation-statement bridge flag is now source-inspected by
`path2_creation_statement_bridge_source_present_v1`, covering the non-circular
creation statement derived from the prevalidated leaf plus reviewed pad/witness
commitments, the creation-statement frame labels, the creation-proof binding and
native role-root inputs, and regressions that message/signature rebinding does
not change the creation digest while pad and proof-binding field mutations do.
The active no-partial-export negative-control flag is now source-inspected by
`path2_reviewed_fq12_no_partial_export_negative_controls_source_present_v1`,
covering the selected-material reject-path assertion, helper-visibility guard,
wrong-pad and malformed-opening public errors, opaque carrier-export rejection,
and the single accepting-certificate export wrapper. This is a scoped active
negative-control check; the full serializer allowlist theorem remains tracked
under P2FS-004.
The Lean payload-interface flag is now source-inspected by
`lean_path2_reviewed_fq12_payload_interface_pinned_v1`, covering
`ReviewedFq12HiddenSourcePayload`,
`ReviewedFq12HiddenRelationExtractionCertificate`, the active payload/certificate
predicates in `Path2NativeBoundary.lean`, the extraction availability and
soundness definitions, and the matching Rust payload-binding and extraction
certificate structs. The active hidden-source semantics-bridge audit now has no
unconditional `true` assignments inside the P2FS-001 source-route core; remaining
presence booleans are productization/preflight surfaces tracked separately.
The Lean source-route endpoint flags are now source-inspected by
`lean_path2_reviewed_fq12_source_route_endpoint_surface_present_v1`, covering
the source-route local-instantiation theorem, boundary/factored selected-Rust
wrappers, compact/local functional-lift wrappers, packaged source-route wrapper,
direct-assembly representative/shared/native-frame wrappers, and the field/local
adapter theorem.
The top-level production preflight also reports that audit as present while
keeping the bridge incomplete, so the hidden-source semantics blocker remains
visible in the active deployment surface.
That keeps the source-route frame as ordering evidence while the detailed
field-evidence binding remains the source of concrete field equalities. The
assembly projection is still post-compatibility only and must not be used as
generic pre-compatibility assembly availability or binding.

### P2FS-001 Packaged Active-Endpoint Blockers

The active target for hidden-source proof semantics is
`ReviewedFq12SelectedRustPackagedSourceRouteInputs`,
proved by
`reviewed_fq12_certificate_soundness_from_selected_rust_packaged_source_route_inputs`.
The remaining blockers are deliberately model-facing, not alternate-path
trackers:

| Blocker | Why it remains real |
|---|---|
| Role-root/lift source | `ReviewedFq12RoleRootFunctionalLiftSource` still needs concrete role-root field binding plus selected-certificate lift-opening functionality. The selected source-route byte source is a single witnessed-chain parser/digest bridge; it does not prove global pairwise role-root/opening uniqueness or functionality of `reviewedFq12CertificateLiftsToOpening`. |
| Selected Rust source-route byte source | The implementation still needs the Lean/Rust theorem that the selected Rust field-evidence/source-route bytes instantiate `SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource`. Rust now source-inspects the relevant constructors and regressions: field evidence verifies chain/role/root/opening/representative agreement, frames reject aggregate-source use and recompute role-root/full-opening/representative digests, source-route construction requires direct assembly binding, same-representative compatibility, and adapter-only field evidence, and tests pin stale digest, aggregate-source, and field-rebinding rejection. |
| Assembly/compatibility local-model source | The endpoint consumes `ReviewedFq12AssemblyCompatibilityLocalModelSource`: assembly availability/local/native facts plus compatibility availability/extraction rules. It deliberately does not require the redundant field-binding facts carried by the older concrete field/local records, because selected source-route bytes supply binding. Rust now source-inspects the corresponding active route: assembly construction verifies the full opening, checks native/projection/role agreement, recomputes the representative digest, compatibility construction checks certificate-chain/digest/field agreement, and the source-route constructor requires direct assembly binding, same-representative compatibility, and adapter-only field evidence. |
| Compatibility route witness source | `ReviewedFq12CompatibilityRouteWitnessSource` bundles constructed-assembly compatibility plus compatibility-side assembly representative lookup. New adapters `constructed_compatibility_witness_of_assembly_field_binding_and_local_model` and `compatibility_route_witness_source_of_assembly_field_binding_local_model_and_digest_lookup` show that direct `ReviewedFq12ConcreteAssemblyFieldBindingOrBreak`, the local-model source, and `ReviewedFq12AssemblyRepresentativeDigestLookupSource` are enough to build the full route-witness package. The digest lookup is still a real Lean/Rust bridge blocker: `FullNativeOpeningMatchesRepresentative` omits `representativeDigest`, so the model must prove that a generic compatibility representative has the digest of the available assembly certificate. Lean first narrowed that to the selected Rust native representative digest frame: `ReviewedFq12SelectedRustNativeRepresentativeDigestFrameSource` supplies both representative-frame canonicality and selected assembly-frame recomputation from the 11-input `path2_reviewed_fq12_native_representative_digest_v1` shape. Lean now further exposes the Rust-witness-shaped route `ReviewedFq12SelectedRustDirectAssemblyDigestFrameWitnessPackagedSourceRouteInputs`, whose theorem `reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_digest_frame_witness_packaged_source_route_inputs` consumes `ReviewedFq12SelectedRustAssemblyRepresentativeFrameSource` for the assembly side and derives representative canonicality from `ReviewedFq12FullNativeOpeningRepresentativeDigestFunctionalitySource`, direct assembly binding, and the local model. The new `ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource` and `assembly_frame_source_of_digest_frame_witness_byte_source` adapter instantiate the assembly-frame premise from the typed Rust witness shape without claiming representative uniqueness. The current P2FS-001 implementation route is the field-local shared-frame wrapper: it derives direct assembly binding from `ReviewedFq12FieldConcreteAssemblyLocalSource`, derives the local model from the concrete assembly/compatibility field sources, and uses `ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources` to require the canonical representative witness source and assembly digest-frame witness source to share one selected representative digest frame. Selected source-route bytes remain the parser/frame bridge for an already witnessed active route and are not used to source the direct assembly-binding premise. The Rust fail-closed audit now source-inspects and pins `lean_selected_rust_direct_assembly_digest_frame_witness_packaged_source_route_endpoint_present`, and also pins `lean_selected_rust_direct_assembly_native_digest_frame_packaged_source_route_endpoint_present`, `rust_native_representative_digest_frame_source_present`, `lean_selected_rust_direct_assembly_shared_representative_frame_packaged_source_route_endpoint_present`, `rust_shared_representative_frame_source_present`, `rust_independent_direct_assembly_binding_source_present`, `rust_constructed_assembly_compatibility_witness_source_present`, `rust_compatibility_route_witness_source_present`, `rust_assembly_compatibility_local_model_source_present`, `rust_full_opening_field_evidence_present`, `rust_full_opening_source_route_present`, `rust_source_route_negative_controls_present`, canonical-witness rebinding negative controls, and the shared-frame witness audit flags; direct binding is backed by `path2_reviewed_fq12_assembly_binds_full_opening_fields_v1`, while the local model/route flags are backed by source inspection of the assembly constructor, compatibility constructor, field-evidence constructor/frame/verifier, source-route constructor/frame/verifier, same-representative predicate, active negative controls, canonical witness constructor/frame/verifier, canonical witness native-role-root/full-opening rebinding tests, and digest-frame witness constructor/frame/verifier. Regressions assert the valid assembly/full-opening pair binds and a local-section mutation fails before source-route acceptance. Direct Rust inspection found that `path2_reviewed_fq12_native_representative_digest_v1` canonically recomputes the digest from `native_boundary_digest` plus the role tuple, assembly certificate framing rejects a stale representative digest, field evidence recomputes the same digest, and compatibility/source-route frames carry the assembly digest. Rust now also exposes `Path2ReviewedFq12NativeRepresentativeDigestFrameWitnessV1`, whose constructor verifies the role-lift digest, assembly certificate digest, direct assembly/full-opening field binding, and representative digest recomputation before producing a domain-separated witness digest. Negative controls now mutate only `assembly_cert.representative_digest` while leaving role fields unchanged, mutate only the source-route `representative_digest` and require verifier reconstruction to reject, independently mutate each representative-digest input to check that every active boundary/role digest affects `path2_reviewed_fq12_native_representative_digest_v1`, reject canonical witness native-role-root/full-opening rebinding even with recomputed witness digests, and reject a self-consistent rebound assembly certificate whose local-section field no longer matches the full-opening certificate. This points to instantiating the field-local shared-frame Lean package, not a protocol redesign, unless some non-inspected production path bypasses these constructors. Selected source-route byte binding remains compatibility-indexed and cannot supply this direct pre-compatibility assembly binding without circularity. |

## Redesign-Risk Priority Order

This queue is ordered by likelihood of exposing a real issue that forces a
parameter or route redesign, not by theorem dependency order.

| Priority | Obligations | Why this comes first | Current blue-team result | Next move |
|---|---|---|---|---|
| 1 | P2FS-008, P2FS-007 | The old one-bit active multi-target horizon invalidated deployment security even if all structural proofs closed. | Started 2026-06-02. Re-ran `scripts/research/qrom_multitarget_margin.py`; the old artifact failed closed above `log2(T)=1`. Redesign route chosen 2026-06-02: auxiliary Fq12 action bits. The first closure target keeps 26 compact repetitions, raises the algebraic challenge frame to 23 bytes with mask `0x3f`, keeps 4 hardening bytes, and prices 182 algebraic / 214 total challenge bits for `log2(T)=32`. Rust/Lean update in progress: QROM-discounted transcript term `182`, strict deployment margin `53`, active aggregate horizon `32`, over-budget negative control `54`, transaction signature `258`, and transaction plus hidden-source proof `290`. | Finish implementing and proving the auxiliary-Fq12 widened challenge budget. |
| 2 | P2FS-012, P2FS-002, P2FS-005, P2FS-006 | A byte-frame or implementation mismatch can certify the wrong verifier and can also invalidate the QROM/domain model. | Started 2026-06-02. Checked-core is now narrowed by `SelectedRustCheckedCoreByteSource` in `Path2CheckedCoreByteBridge.lean`, which decomposes the selected Rust acceptor into the exact Lean byte-certificate fields. Hash domains are now narrowed by `ActiveHashByteDomainInstantiation` in `Path2HashDomainAndMultiTarget.lean`, and the active bridge is wrapped by `ActiveHashByteDomainEvidenceCertificate` with report/digest evidence before `ActiveHashDomainAndMultiTargetCertificate` consumes `.toInstantiation`. The checked-core Rust evidence report now also consumes `Path2NativeHashByteDomainEvidenceV1`, verifies it, carries its digest/facts through the checked-core evidence frame and Lean certificate, and records that the checked-core frame, frame digest, and challenge use the checked-core domain named by that evidence object. Targeted Rust tests passed for canonical proof rejection, shared assembly-frame derivation, proof-generation/expected-syndrome agreement, active native domain uniqueness, global domain/schema disjointness, hash-budget source-gate comment/string spoofing rejection, and checked-core/domain-evidence frame-prefix/digest/challenge binding. | Instantiate `SelectedRustCheckedCoreByteSource` with the concrete Rust parser/frame/hash functions against the checked-core evidence report/digest package, then continue this item with hidden-source frame bytes and parser canonicality. |
| 3 | P2FS-001 | Hidden-source role-root/full-opening binding is the deepest cryptographic assumption and a plausible route-level redesign trigger. | Started in this ordering pass. Interface narrowed to `ReviewedFq12SelectedRustPackagedSourceRouteInputs`; concrete Rust/model instantiation open. Rechecked the active field-evidence/source-route shape: Lean keeps the source route compatibility-indexed and Rust rejects aggregate-source use; full-opening role-binding and role-root rebinding regressions pass. The latest Lean reduction adds the field-local shared-frame wrapper, direct five-source theorem, direct witness-byte-source theorem, and raw-component theorem, which derive direct assembly binding from concrete assembly fields, derive the local model from assembly/compatibility components, consume the Rust-shaped canonical representative witness and assembly digest-frame witness sources, and require both witness sources to share one selected representative digest frame. Rust now exposes typed witnesses for both sides, the independent direct assembly field-binding helper, and source-inspected local assembly/compatibility model, field-evidence, source-route, negative-control, canonical-witness, and shared-frame audit flags. | Instantiate `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components` from role-root/lift components, selected Rust source-route byte source, assembly field/local/native components, compatibility field/extraction components, the selected representative digest frame, canonical witness byte source, assembly digest-frame witness byte source, and the two frame equalities. |
| 4 | P2FS-003, P2FS-013 | Lifecycle semantics can force redesign if blockchain success does not match accepted-spend semantics. | Started 2026-06-02. `Path2FullSpendConcrete.lean` now has `ConcreteFullSignatureExecutionSource`, a non-tautological production execution source requiring full-signature success to imply concrete verifier acceptance or a named break, and the selected Rust instantiation `concreteFullSignatureExecutionSource_instantiation_v1`. The AP2-N003 source gate now closes through `SelectedRustFullExecutionSourceCertificate.ofReportAndExecutionSource` with `selectedRustFullExecutionEvidenceDigest`, pins the selected execution predicate to `selectedRustProductionExecutionSucceeds`, and threads the source through the concrete full-execution lift/full-boundary handoff. Rust preflight no longer reports `PATH2_NATIVE_DEPLOYMENT_BLOCKER_FULL_SIGNATURE_EXECUTION_SOURCE`; the active proof blocker is now the full-signature reduction certificate, plus its accepted-spend numeric-bound dependency. | Instantiate `selectedRustFullSignatureReductionBoundary_instantiation_v1` from the selected full-execution source, active hash boundary, all active query-symbol pricing, multi-target pricing, production signature-forgery probability pinning, and the accepted-spend numeric bound. |
| 5 | P2FS-009, P2FS-010, P2FS-011 | Keygen/signer/completeness can force productization redesign, but the proof route can be evaluated after the verifier and budget surfaces are known. | Productization blockers intentionally remain active. | Model keygen/signing/completeness over the selected verifier after signer/verifier pipeline, CT, and storage gates are retired. |
| 6 | P2FS-004 | No-export is deployment-critical, but less likely than the above to force algebraic parameter redesign. | Closed for the active selected serializer source. Lean now proves public/private tag disjointness for the selected active serializer and Rust preflight no longer reports `PATH2_NATIVE_DEPLOYMENT_BLOCKER_NO_EXPORT_SERIALIZERS`. | Keep the synthetic reopen-ordering regression; reopen only if future concrete serializers expose selected hidden material or require output fields outside the selected public tags. |

## Active Obligations

| ID | Boundary field | Current status | Next proof move | Redesign trigger |
|---|---|---|---|---|
| P2FS-001 | `acceptedSpendSoundness.hiddenSourceProofSemanticsClosed` | Interface closed; concrete instantiation narrowed to non-aggregate role-root, assembly, compatibility, field-evidence, source-route, and compact local-source layers. Rust has pre-aggregate field-evidence/source-route frames with negative controls for aggregate-source misuse, stale digests, route-source misuse, and role-root/full-opening/representative rebinding. Lean now exposes `ReviewedFq12SelectedRustPackagedSourceRouteInputs` and theorem `reviewed_fq12_certificate_soundness_from_selected_rust_packaged_source_route_inputs`, packaging role-root/lift facts, selected Rust source-route bytes, assembly/compatibility local-model facts, and compatibility-side route witnesses as explicit source packages. It also exposes `ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputs`, which derives compatibility-route witnesses from independent direct assembly binding, the local model, canonical representative byte construction, and the Rust-facing assembly representative frame witness byte source. The representative/canonicality issue is now narrowed to instantiating `ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource` for the selected Rust/model assembly predicate, not to reusing source-route bytes as assembly evidence and not to relying on the assembly witness for representative uniqueness. Rust now has typed witness APIs for both the canonical representative side and the assembly digest-frame side; regressions verify valid witnesses, stale representative-digest rejection, stale witness-digest rejection, and rebound assembly/opening drift rejection. The Lean canonical representative byte certificate now binds native role root and full-opening digest to the opened tuple, matching the Rust witness frame. The Rust fail-closed audit now source-inspects the role-root/lift source, independent direct assembly binding source, local assembly/compatibility model source, field-evidence source, source-route source, source-route negative controls, the Rust-shaped canonical witness byte source, and the shared native digest frame used by both witness sources. It also hash-binds a composite AP2-N001 package-surface gate covering the role-root/lift source, selected source-route byte endpoint, direct assembly source, local-model source, canonical representative source, canonical representative witness byte source, native representative digest-frame source, and Rust digest-frame witness API. Lean now also exposes `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_inputs`, `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_sources`, `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_sources`, and `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components`, which consume the concrete field/local assembly and compatibility components, derive the direct assembly binding/local model internally, and use canonical/assembly witness byte sources pinned to one selected representative digest frame. The new canonical representative witness domain is in the global disjoint-domain registry. The canonical-representative/witness-byte, digest-frame witness, native-frame, shared-frame, and field/local packaged theorems are tracked as convenience adapters only. `extractor_theorem_instantiated` remains false, so the Rust bridge audit and deployment preflight stay fail-closed. | Instantiate the components consumed by `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components`: concrete role-root field binding; selected certificate-lift functionality; selected Rust source-route byte source; assembly availability, direct assembly field binding, local predicates, and native validity; compatibility availability, same-representative field binding, and extraction rules; selected representative digest frame; Rust-shaped canonical witness byte source for `Path2ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessV1`; `ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource` for `Path2ReviewedFq12NativeRepresentativeDigestFrameWitnessV1`; and the two frame equalities. Use the aggregate verifier object only as a downstream audit bridge, not as the source of assembly soundness. Track operational private-fiber digests under P2FS-003/P2FS-010/P2FS-004. | Redesign to Option 2 private lifecycle root if any operational private-fiber digest becomes verifier-facing, affects accepted hidden-source semantics, or is needed to prove lifecycle/signing-oracle/no-export. |

P2FS-001 concrete-definition blocker note: Lean now includes
`selectedRustCurrentPlaceholderEvidenceProofDefinitions_impossible`, proving
that the retained fail-closed placeholder evidence reports cannot supply the
proof-bearing report/facts definitions required by the concrete-definition
checklist. This is now a negative control only: the active selected
report/proof names have moved to verifier-backed aliases, while source records
and digest-match proofs remain missing.
Lean also includes
`selectedRustCurrentPlaceholderSourceRecordCertificates_impossible`, proving
that selected source-record certificates over the current placeholder component
reports are inconsistent. This documents the `source_record_definitions` gap
without flipping it.
The concrete-definition checklist now requires each selected source-certificate
definition to call its named constructor with the selected report, digest,
report-verification proof, facts proof, and raw source arguments. A regression
fixture where `selectedRustBoundaryBaseCertificate` is replaced by an anonymous
certificate value leaves `source_record_definitions_present = false`, so the
category cannot be satisfied by a type-correct but unbound certificate literal.
`selectedRustCurrentPlaceholderSubEvidenceDigestMatchProof_impossible` now
names the matching blocker for `sub_evidence_digest_match_proof`: the
placeholder aggregate report cannot digest-match selected public Rust component
evidence digests.
Lean also proves
`selectedRustFiveSourceEvidenceReport_subEvidenceDigestsMatch_of_selected_certificate_digests`.
For the active verifier-backed aggregate report, any five selected source
certificates whose `evidenceDigest` fields equal the selected component digest
constants satisfy `SubEvidenceDigestsMatch`. This does not close the checklist
yet, because `selectedRustFiveSourceSubEvidenceDigestsMatchProof` still depends
on importing the selected source certificates themselves.
Rust now has `path2_selected_evidence_report_manifest`, a source utility that
prints the full selected evidence reports. Its first run confirms all six
report/fact sets verify, while `extractor_theorem_instantiated` and
`semantics_bridge_complete` remain false. That makes verifier-backed Lean report
replacement the next safe step, separate from source-record/digest-match
closure.
Lean now imports those report values under `verifierBacked...` names and proves
their `ReportVerified`/`FactsAllTrue` predicates. The active `selectedRust...`
report names now alias those verifier-backed values; source-record
certificates, aggregate/sub-certificate digest matching, and aggregate digest
self-reference remain fail-closed.
Rust now source-checks the ByteArray fields inside the verifier-backed aggregate
Lean report, not only the standalone selected digest constants. The regression
parses `verifierBackedSelectedRustFiveSourceEvidenceReport` and compares its
bridge audit digest, hash-domain evidence digest, and five sub-evidence digests
against the Rust-computed evidence values. This closes the stale-record-field
blind spot without changing the remaining source-record/digest-match blockers.
The same regression now compares all verifier-backed aggregate and sub-report
boolean fields against the corresponding Rust evidence reports, so the imported
Lean records cannot silently drift in either byte fields or fact flags.
The concrete source-record route should target
`ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputs`.
The first proof object to instantiate is
`ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource`: it must turn
the Rust canonical witness/frame evidence into Lean byte certificates for
accepted semantic openings. The existing digest-frame witness source only
discharges assembly-frame recomputation and cannot replace the representative
canonicality source.
`path2_selected_evidence_digest_manifest` now emits `source_record_subcheck`.
The current subcheck reports `missing_selected_source_record_count = 13`: the
seven raw selected source definitions, five constructor-bound selected
source-certificate definitions, and the constructor-bound
`selectedRustModelSourceRecordPackage` definition are all still absent. The
same subcheck reports the canonical-witness-to-byte-source adapter and
byte/witness packaged route endpoint as present, so the next work is concrete
selected source import, not another route theorem.
Lean now provides
`ReviewedFq12SelectedRustModelSourceRecordPackage`, which packages the seven
raw selected source records before final aggregate-digest closure. The package
projections construct all five selected source certificates with the named
report/digest/proof/source constructors, and
`ReviewedFq12SelectedRustModelSourceRecordPackage.subEvidenceDigestsMatch`
derives the aggregate sub-evidence match from selected component digest
equalities. The manifest now reports both package-surface booleans true, while
the thirteen concrete selected source/package bindings remain absent.
The same package now has direct reductions to the active proof inputs:
`toFiveSourceClosureInputs`,
`toByteComponentInputs`, and
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_source_record_package`.
The manifest reports these three package-reduction checks true, so once the
seven raw selected source records exist, the closure path is mechanical. It
does not change the fail-closed status because the package itself is still not
instantiated.
Lean now also exposes the package handoff all the way to the boundary fields:
`with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_source_record_package`
and
`full_signature_with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_source_record_package`.
The focused Rust audit gate requires these package-handoff markers. The only
remaining non-mechanical step is still the concrete native-boundary semantics
package/source-record import.
Boundary-base source audit: report verification is not the payload projection
law or extraction-certificate availability theorem. The active file currently
has verifier-backed boundary-base evidence, but no
`selectedRustPayloadProjectionLaw`, no
`selectedRustExtractionCertificateAvailability`, and no constructor-bound
`selectedRustBoundaryBaseCertificate`. The manifest now exposes this exact
state as `boundary_base_report_only_not_source_record_closure = true`, and the
Rust regression checks that a report-only fixture remains fail-closed while a
fixture with the raw sources plus named certificate constructor clears the
flag.
Role-root/lift source audit: the same issue holds for the next selected source
category. The verifier-backed role-root/lift report is present, but the active
file still has no `selectedRustRoleRootLiftSource` and no constructor-bound
`selectedRustRoleRootLiftSourceCertificate`. This source must supply
`ReviewedFq12ConcreteRoleRootFieldBindingCertificate` plus
`ReviewedFq12CertificateLiftOpeningFunctionality`; report verification alone
does not prove either semantic obligation. The manifest now exposes
`role_root_lift_report_only_not_source_record_closure = true`, with a fixture
regression pinning the fail-closed distinction.
All-five source-record audit: the source-record subcheck now exposes matching
report-only guards for boundary-base, role-root/lift, source-route,
field/local, and shared-witness evidence. The active manifest reports all five
guards true, while `missing_selected_source_record_count = 13`. This is the
current proof-completeness frontier: verified evidence reports are in Lean,
but the raw source records (`selectedRustPayloadProjectionLaw`,
`selectedRustExtractionCertificateAvailability`,
`selectedRustRoleRootLiftSource`, `selectedRustSourceRouteByteSource`,
`selectedRustAssemblySource`, `selectedRustCompatibilitySource`, and
`selectedRustSharedFrameWitnessSources`) plus their five named source
certificates plus the selected source-record package definition still have to
be imported from the active Rust/model route.
Predicate-model audit: the concrete full-spend package currently visible in
Lean instantiates `Path2FullSpendGame.Predicates`; it does not instantiate the
`Path2NativeBoundary.Predicates` record used by P2FS-001. The selected source
records are semantic theorems over that native-boundary predicate record. The
next constructive input is therefore a concrete active native-boundary
predicate interpretation, or an equivalent theorem package that supplies the
seven selected source records for the exact `P` used by the hidden-source
boundary.
The selected evidence manifest now makes this machine-checkable:
`native_boundary_predicates_record_present = true`,
`native_boundary_semantics_package_target_present = true`,
`full_spend_concrete_predicates_are_separate_model = true`,
`native_boundary_concrete_predicate_model_present = false`, and
`selected_source_records_blocked_on_native_boundary_predicate_model = true`.
The missing proof object is therefore the concrete native-boundary semantics
package for the active verifier, not another evidence digest import.
Lean now names the target package:
`ReviewedFq12SelectedRustNativeBoundarySemanticsPackage`, carrying the
concrete native-boundary `Predicates` interpretation and the selected
source-record package tied to that exact interpretation.
That target now reaches both boundary handoff surfaces, and the manifest
reports `native_boundary_semantics_package_handoff_present = true`. The
remaining missing object is still the concrete native-boundary semantics
package itself.

P2FS-001 latest hardening: `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSources` is now the named Lean constructor for the final selected Rust/model certificate. The certificate now carries an aggregate five-source `evidenceDigest`, and the Rust `extractor_theorem_instantiated` gate requires the future final theorem to use that constructor with the verified five-source evidence report, aggregate evidence digest, all-true facts proof, and selected source packages before deriving `toClosureInputs`; a raw certificate record literal, empty digest literal, `sorry`, or `admit` no longer satisfies the AP2-N001 source gate.

P2FS-001 selected source-route byte-source hardening: `SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport` and `SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate` now wrap the selected Rust field-evidence/source-route byte source. The final five-source certificate constructor has a source-route-certificate variant, and the Rust `extractor_theorem_instantiated` gate now requires `selectedRustSourceRouteByteSourceCertificate` rather than a bare `selectedRustSourceRouteByteSource`, so the selected parser/frame/binding/negative-control evidence must be public and proof-bearing before AP2-N001 can close.

P2FS-001 selected source-route Rust mirror hardening: `Path2ReviewedFq12SelectedSourceRouteByteSourceEvidenceV1` now mirrors the Lean selected source-route evidence report on the Rust side, including schema/frame/digest functions, report verification, all-facts checking, and a verifier. The selected source-route contract gate requires that Rust evidence surface and checks a constructed report before accepting the active source-route byte-source marker. The new focused regression pins successful digest verification and rejects tampered route-availability, schema-match, and digest-verified bits. This keeps the Rust evidence public and digest-bound, but it does not flip `extractor_theorem_instantiated`.

P2FS-001 shared digest-frame witness hardening: `ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport` and `ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate` now wrap the canonical-representative witness and assembly digest-frame witness bundle. The final five-source certificate constructor has a combined source-route/witness-certificate variant, and the Rust `extractor_theorem_instantiated` gate now requires `selectedRustSharedFrameWitnessSourcesCertificate` rather than a bare `selectedRustSharedFrameWitnessSources`, so the shared-frame equality, parser/frame/verifier bindings, and witness negative controls must be public and proof-bearing before AP2-N001 can close.

P2FS-001 shared-witness Rust mirror hardening: `Path2ReviewedFq12SharedDigestFrameWitnessEvidenceV1` now mirrors the Lean shared digest-frame witness evidence report on the Rust side, including schema/frame/digest functions, report verification, all-facts checking, and a verifier. The shared witness byte-source contract gate requires that Rust evidence surface and checks a constructed report before accepting the shared witness source marker. The new focused regression pins successful digest verification and rejects tampered shared-frame pinning, schema-match, and digest-verified bits. This keeps canonical/digest-frame witness evidence public and digest-bound, but it does not flip `extractor_theorem_instantiated`.

P2FS-001 field/local source hardening: `ReviewedFq12SelectedRustFieldLocalEvidenceReport` and `ReviewedFq12SelectedRustFieldLocalEvidenceCertificate` now wrap the assembly and compatibility local source records. The final five-source certificate constructor has a source-route/field-local/witness certificate variant, and the Rust `extractor_theorem_instantiated` gate now requires `selectedRustFieldLocalSourceCertificate` rather than bare `selectedRustAssemblySource` and `selectedRustCompatibilitySource`, so assembly availability/binding/local/native facts and compatibility availability/binding/extraction facts must be public and proof-bearing before AP2-N001 can close.

P2FS-001 field/local Rust mirror hardening: `Path2ReviewedFq12FieldLocalEvidenceV1` now mirrors the Lean field/local evidence report on the Rust side, including schema/frame/digest functions, report verification, all-facts checking, and a verifier. The assembly/compatibility local-model gate requires that Rust evidence surface and checks a constructed report before accepting the local-model source marker. The new focused regression pins successful digest verification and rejects tampered assembly field binding, schema-match, and digest-verified bits. This keeps the assembly/compatibility local evidence public and digest-bound, but it does not flip `extractor_theorem_instantiated`.

P2FS-001 role-root/lift source hardening: `ReviewedFq12SelectedRustRoleRootLiftEvidenceReport` and `ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate` now wrap the concrete role-root field binding and selected lift functionality source. The final five-source certificate constructor has a role-root/source-route/field-local/witness certificate variant, and the Rust `extractor_theorem_instantiated` gate now requires `selectedRustRoleRootLiftSourceCertificate` rather than bare `selectedRustRoleRootLiftSource`, so the native role-root binding and selected lift-functionality facts must be public and proof-bearing before AP2-N001 can close.

P2FS-001 role-root/lift Rust mirror hardening: `Path2ReviewedFq12RoleRootLiftEvidenceV1` now mirrors the Lean role-root/lift evidence report on the Rust side, including schema/frame/digest functions, report verification, all-facts checking, and a verifier. The role-root/lift source gate requires that Rust evidence surface and checks a constructed report before accepting the combined role-root/lift source marker. The new focused regression pins successful digest verification and rejects tampered concrete-role-root binding, schema-match, and digest-verified bits. This keeps the role-root/lift evidence public and digest-bound, but it does not flip `extractor_theorem_instantiated`.

P2FS-001 boundary-base source hardening: `ReviewedFq12SelectedRustBoundaryBaseEvidenceReport` and `ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate` now wrap the payload projection law and extraction-certificate availability source. The final five-source certificate constructor has a boundary-base/role-root/source-route/field-local/witness certificate variant, and the Rust `extractor_theorem_instantiated` gate now requires `selectedRustBoundaryBaseCertificate` rather than bare `selectedRustPayloadProjectionLaw` and `selectedRustExtractionCertificateAvailability`, so AP2-N001's final source boundary is certificate-only.

P2FS-001 boundary-base Rust mirror hardening: `Path2ReviewedFq12BoundaryBaseEvidenceV1` now mirrors the Lean boundary-base evidence report on the Rust side, including schema/frame/digest functions, report verification, all-facts checking, and a verifier. The five-source evidence source gate requires that Rust evidence surface and checks a constructed report before accepting the aggregate five-source evidence surface. The new focused regression pins successful digest verification and rejects tampered payload-projection, schema-match, and digest-verified bits. This keeps the base-boundary evidence public and digest-bound, but it does not flip `extractor_theorem_instantiated`.

P2FS-001 aggregate evidence-binding hardening: the public five-source closure evidence report now digest-binds the five prerequisite evidence reports themselves: boundary-base, role-root/lift, selected source-route, field/local, and shared digest-frame witness. Rust verifies each sub-report before emitting the aggregate report, includes each sub-report digest and verification bit in the aggregate evidence frame, and rejects stale sub-report digests or cleared verification bits. This is still a proof-surface hardening step only; the final `extractor_theorem_instantiated` theorem remains absent.

P2FS-001 aggregate/certificate digest-match hardening: the aggregate five-source certificate now carries its own evidence digest, the five sub-evidence certificates carry evidence digests, and the final all-certificate AP2-N001 constructor requires `ReviewedFq12SelectedRustFiveSourceClosureEvidence.SubEvidenceDigestsMatch`, proving that the aggregate five-source report names exactly the five certificate reports it consumes. The Rust final-theorem source gate now requires `selectedRustFiveSourceEvidenceDigest` and `selectedRustFiveSourceSubEvidenceDigestsMatchProof` in the accepted theorem shape, so a future close cannot pair an undigested aggregate readiness report with mismatched source certificates.

P2FS-001 per-source certificate-constructor hardening: the boundary-base, role-root/lift, source-route, field/local, and shared-witness evidence certificates now each have named constructors that take the public report, report digest, report-verification proof, all-facts proof, and underlying source records. The future AP2-N001 theorem gate requires these constructors before the final all-certificate constructor, preventing anonymous certificate records from hiding mismatched or unverified evidence.

P2FS-001 proof-shell hardening: the hidden-source extractor gate now rejects
`abbrev` proof shells, in addition to `def`/`theorem` shells, for selected
source proof names ending in `Proof` when the body is `True.intro`,
`by trivial`, or `by exact True.intro`. A regression prepends such an alias to
the otherwise accepted theorem shape and keeps the gate open.

P2FS-001 selected evidence manifest status: `path2_selected_evidence_digest_manifest`
now emits the concrete-definition checklist next to the six selected evidence
digests. The current active output verifies all six Rust evidence reports and
all component facts, but keeps AP2-N001 fail-closed:
`extractor_theorem_instantiated = false`,
`semantics_bridge_complete = false`, checklist digest
`f1dc786697672712b03cbce6ab97124ac520d8cac569c6ffe5566270782ca3bd`, and
missing categories `predicate_model_certificate_definitions`,
`source_record_definitions`, `source_record_evidence_source_bindings`, and
`sub_evidence_digest_match_proof`. The
selected Lean report/proof definitions
now alias verifier-backed Rust evidence, so the highest-risk remaining proof
work is to import selected source-record certificates, bind each imported
source record to the public evidence that claims to certify it, and prove the
aggregate five-source report's sub-digests match those five certificates.
The active source scanner now preserves UTF-8 Lean symbols during
comment/string stripping, so conjunction-heavy propositions are checked against
the actual stripped Lean source rather than byte-corrupted marker text. The
early reviewed-Fq12 extraction-chain target gates for payload acceptance,
extraction-certificate payload binding, extraction-certificate validity,
certificate-to-payload extraction consistency, payload fiber membership, and
payload native validity now require the named definition body to start with
the concrete quantified obligation. Regression fixtures alias the live
payload-acceptance and payload-native-valid target bodies while placing the old
markers in unused examples; both fixtures fail closed. The current manifest
still reports all six target bits true, while `predicate_model = false`,
`missing_selected_source_record_count = 13`, and the three missing definition
categories remain the real AP2-N001 blockers.
The same declaration-scoped body-prefix treatment now covers the downstream
role-lift, assembly, compatibility, and aggregate full-opening bridge target
obligations that feed `full_opening_and_assembly_model_pinned` and
`compatibility_and_extraction_model_pinned`. Regression fixtures alias the
role-lift opening and compatibility-extraction target bodies while moving the
old markers into unused examples; both now fail closed, while the active
manifest still reports those scoped target bits true on the live Lean source.
The accepted-spend/full-signature handoff and active-bound handoff gates for
the semantics package, source-shaped package, and certified source package are
now declaration-scoped as well. Each gate checks the named theorem header and
requires the theorem body to call the corresponding closure or
`fullSignatureSuccessProb_le_activePathForgeryBound` theorem with the package's
actual predicates and `hiddenSourceClosure`. A regression fixture aliases the
semantics-package active-bound theorem body while moving the old markers into
an unused example; the handoff bit now fails closed under that mutation.
The selected source-record package surface and reductions are now scoped to
their named structure, constructor, projection definitions, sub-evidence
theorem, and hidden-source closure theorem. The scanner requires the actual
package projection bodies to consume the package fields in order, and a
regression fixture aliases the package-to-hidden-source-closure theorem while
placing the old markers in an unused example; the reduction bit now fails
closed under that mutation.
The payload-acceptance and boundary-break antivacuity target checks are also
scoped to the actual `ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource`
field types. A regression fixture aliases the payload antivacuity field type;
the corresponding target bit now fails closed unless the source record carries
the concrete non-vacuity proposition.
The concrete-definition checklist now has an explicit
`source_record_evidence_source_bindings_present` category. This keeps
`all_definitions_present` false even for a synthetic fixture that supplies
arbitrary source terms and constructor-shaped certificates, unless the proof
also supplies selected evidence/source binding definitions for the boundary
base, role-root/lift, source-route, field/local, and shared-witness source
families. Current selected digests are checklist
`f1dc786697672712b03cbce6ab97124ac520d8cac569c6ffe5566270782ca3bd` and
five-source evidence
`67e89bdc6748730aaf40787a52c987ffda549b919e331f824f8fa498e0a3a7e9`.
The binding category is now constructor-gated as well: Lean provides named
constructors for the five selected evidence/source binding families, and the
Rust checklist requires selected binding definitions to use those constructors
rather than anonymous record literals.
The module-level final theorem gate now requires the full concrete-definition
checklist before `extractor_theorem_instantiated` can become true. A
constructor-shaped theorem snippet without the binding definitions, selected
predicate-model certificate, source-record definitions, or sub-evidence
digest-match proof remains accepted only by the body-shape scanner and is
rejected by the real module gate, so the public AP2-N001 status cannot advance
ahead of the selected definition import work.
Lean now exposes concrete source-binding object types for the five selected
evidence families and a
`ReviewedFq12SelectedRustModelSourceRecordPackage.ofSourceBindings`
constructor that derives the seven source-record fields from those binding
objects. The Rust package-surface gate requires that binding-backed
constructor, and the source-binding checklist requires future selected binding
definitions to use the concrete Lean binding types. The manifest remains
fail-closed with the same four missing categories, so this adds the proof
shape without silently importing any selected sources.
The selected source-record definition gate and final theorem body-shape gate
now require the binding-backed `ofSourceBindings` constructor rather than the
raw `ofSources` constructor. This closes the route where binding definitions
could exist unused while the package consumed independent raw source terms.
Each Lean source-binding object now also carries an `evidenceReportBound`
proof tying its report field to the active selected report constant consumed
by the certificate path. The Rust package-surface gate pins those fields, so a
future binding-backed source package cannot be typed against unrelated report
objects while still satisfying the AP2-N001 surface checks.
The selected source-binding definition checklist now requires structure-literal
bodies that set the active selected report, `evidenceReportBound := rfl`, a
named source-binding digest, and the selected source fields. Header-only
binding declarations no longer satisfy the module-level final theorem gate.
That same category now also requires the five named source-binding digest
definitions to exist as `ByteArray` definitions and treats those digest names
as forbidden source stubs if they are declared by `axiom`, `constant`, or
`opaque`.
The predicate-model certificate category now also requires named selected
obligation definitions for every native-boundary predicate target consumed by
`ofReportAndModel`. The certificate body alone is no longer enough; each
selected proof must expose the target proposition over
`selectedRustNativeBoundaryPredicates`, and those names are blocked as
`axiom`/`constant`/`opaque` stubs.
The source-record definition category now requires every raw selected source
definition to be the projection of its corresponding evidence/source binding
object. This keeps the certificate route and the binding-backed package route
definitionally aligned instead of allowing independent raw source terms to
coexist with an unrelated `ofSourceBindings` package.
The final theorem body-shape scanner now rejects local same-name
reconstruction of the five selected source certificates and
`selectedRustModelSourceRecordPackage`. The future theorem may build the
aggregate five-source certificate and certified semantics handoff from those
objects, but the per-source certificates and source-record package must be the
selected global definitions that the concrete-definition checklist already
checks.
Each selected evidence/source binding object now also carries the selected
component `evidenceDigest` plus an `evidenceDigestBound` equality proof. The
source-binding checklist requires future binding definitions to fill those
fields with the selected component evidence digest and `rfl`, so report-bound
source records are also digest-bound to the same public evidence constants
consumed by the source certificates.
The five source-binding digest constants are now scanner-bound to the same
selected evidence digest constants. A negative-control fixture that points the
boundary-base source-binding digest at a different digest leaves
`source_record_evidence_source_bindings_present = false`.
The predicate-model report target gate now requires the Lean `FactsAllTrue`
definition to consume every public report bit, including extraction,
full-opening/assembly, compatibility, native-break, record-completeness,
active-verifier-route, negative-control, concrete-model, and certified-package
target facts. Dropping one fact from `FactsAllTrue` is a source-gate failure,
so the future facts proof cannot certify a narrower predicate-model report
than the Rust evidence frame records.
The aggregate digest import also exposed a source-hash circularity constraint:
the current Rust aggregate evidence digest is
`146570f815d27ca8b79ad52480f51a9bb7fb55c892e17a34fdeb9b0050cd1367`, and the
current bridge-audit digest inside that report is
`ae4a87958c5c0479df16f77b3e8467870dbdbec3e90c6410bcb79f4fac0c0e54`, but
hard-coding either value back into the Lean source would change the source
hash and therefore change the report. The source-record import route must
break that self-reference instead of treating the aggregate digest literal as
a stable Lean constant.

P2FS-001 selected report fail-closed sanity: Lean now proves that every current
selected AP2-N001 report placeholder fails both `ReportVerified` and
`FactsAllTrue`. This does not close any missing checklist category; it prevents
the named fail-closed report definitions from being mistaken for usable
evidence and keeps the next work item focused on replacing them with
verifier-backed selected proof/source imports.

P2FS-001 concrete-definition checklist hardening: selected report/proof/source
marker checks now examine only the declaration header/type before `:=` or
`where`. A source-record spoof with required AP2-N001 type names only in
definition bodies fails the checklist, so the remaining source-record category
cannot be closed by body marker placement; the selected declarations must carry
the active proof/source types at their boundary.

P2FS-001 source-certificate checklist hardening: the existing
`source_record_definitions_present` category now requires the five selected
certificate declarations consumed by the final AP2-N001 constructor, not just
the raw source records. A complete fixture without
`selectedRustBoundaryBaseCertificate`,
`selectedRustRoleRootLiftSourceCertificate`,
`selectedRustSourceRouteByteSourceCertificate`,
`selectedRustFieldLocalSourceCertificate`, and
`selectedRustSharedFrameWitnessSourcesCertificate` no longer counts as
definition-complete. The active manifest state is unchanged: the category is
still false, and the missing-count remains three.

P2FS-001 sub-evidence proof hardening: the
`selectedRustFiveSourceSubEvidenceDigestsMatchProof` checklist gate now
requires the proof header/type to name `selectedRustFiveSourceEvidenceReport`
as the aggregate report. A spoofed proof over `otherFiveSourceEvidenceReport`
with the correct five certificate names fails the checklist, so the future
digest-match proof must bind the selected aggregate report, not just any
five-source report-shaped value.

P2FS-001 placeholder aggregate digest sanity: Lean now proves that any
`SubEvidenceDigestsMatch` proof against the current
`selectedRustFiveSourceEvidenceReport` placeholder forces all five consumed
certificate evidence digests to be empty byte arrays. Since the selected Rust
evidence digest constants are concrete non-zero arrays, the future
digest-match proof cannot legitimately reuse the placeholder aggregate report;
it must be derived from verifier-backed selected evidence.

P2FS-001 final-theorem endpoint hardening: the source gate regression now
accepts only the production-facing certificate endpoint for the final
five-source theorem. The positive fixture calls
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_certificate`
with the constructed `selectedRustModelFiveSourceCertificate`; constructing
that certificate and then detouring through `toClosureInputs` plus the raw
input theorem is an explicit negative fixture. This keeps the final route
certificate-facing all the way into hidden-source closure.

P2FS-001 typed-digest checklist hardening: selected evidence digest definitions
now count only when their declaration header/type contains `ByteArray`.
Name-only digest definitions fail the concrete-definition checklist, while the
active Lean digest constants remain accepted and the manifest state is
unchanged. This keeps the digest inputs byte-array-shaped before the future
proof objects and source certificates consume them.

P2FS-001 exact proof-predicate checklist hardening: selected evidence proof
definitions now count only when their declaration header/type contains the
exact selected report predicate, such as
`selectedRustBoundaryBaseEvidenceReport.ReportVerified`. A split-marker spoof
where the theorem mentions the selected report in a separate premise but proves
`otherBoundaryBaseEvidenceReport.ReportVerified` fails the proof-definition
category, so the future verifier-backed proof imports must close the selected
report predicates themselves rather than merely mentioning their names.

P2FS-001 exact sub-evidence predicate checklist hardening: the digest-match
proof category now requires
`selectedRustFiveSourceEvidenceReport.SubEvidenceDigestsMatch` in the
declaration header/type. A split-marker spoof that mentions the selected
aggregate report separately while proving
`otherFiveSourceEvidenceReport.SubEvidenceDigestsMatch` fails, keeping the
future aggregate/certificate digest-match proof pinned to the selected
five-source report.

P2FS-001 selected-digest impossibility sanity: Lean now proves the selected
public Rust evidence digest constants are non-zero and that the current
placeholder `selectedRustFiveSourceEvidenceReport` cannot digest-match
certificates carrying those selected digest constants. This makes the
placeholder/certificate mismatch explicit in the theorem layer: AP2-N001 must
import verifier-backed selected evidence rather than deriving digest-match
closure from the fail-closed placeholder report.

P2FS-001 placeholder-certificate impossibility sanity: Lean now proves no
`ReviewedFq12SelectedRustModelFiveSourceClosureCertificate` can carry the
current placeholder aggregate report. Such a certificate contains a
`ReportVerified` proof, while the placeholder report is proved not verified.
This pins the remaining proof work to real selected report import, not to
reusing the fail-closed named report value.

P2FS-001 per-source placeholder-certificate sanity: Lean now proves the current
boundary-base, role-root/lift, selected source-route, field/local, and
shared-witness placeholder reports cannot be carried by their corresponding
selected evidence certificates. Every source certificate exposes its source
only behind `ReportVerified`, and every current placeholder report is proved
not verified.

P2FS-001 ordered source-declaration checklist hardening: selected declaration
marker matching is now ordered within the declaration header/type. The payload
projection law gate requires `P.reviewedFq12PayloadAccepts` before
`P.reviewedFq12PayloadProjectsFrom`; a synthetic otherwise-complete fixture
with the implication reversed keeps the report/digest/proof and digest-match
categories present but fails `source_record_definitions_present`.

P2FS-001 source-route anti-stub hardening: the forbidden-source-stub scan now
rejects vacuous selected source-route evidence reports and trivial
source-route report/facts proof declarations. This fills the remaining
per-source vacuity check so the selected source-route report is treated like
the aggregate, boundary-base, role-root/lift, field/local, and shared-witness
reports.

P2FS-001 predicate-model certificate binding hardening: the selected native
boundary predicate-model certificate now carries proof fields binding its
`evidenceReport` and `evidenceDigest` to the selected public
`selectedRustNativeBoundaryPredicateModelEvidenceReport` and
`selectedRustNativeBoundaryPredicateModelEvidenceDigest` constants. The named
constructor requires those equalities explicitly, the selected concrete
definition gate requires `rfl rfl` at the report/digest boundary, and the
fail-closed current-report theorem derives impossibility through the
certificate's internal `evidenceReportBound` field rather than through an
externally supplied report equality. This keeps AP2-N001 blocked on importing
the real active `Path2NativeBoundary.Predicates` model while closing another
alias/rebind route around the public predicate-model evidence object.

P2FS-001 source-binding digest-proof hardening: each selected
evidence/source binding now also carries `sourceBindingDigestBound`, proving
that the public binding digest field is definitionally the same byte array as
the selected evidence digest carried by the binding. The future selected
binding definitions must set this with `rfl`; the source audit rejects a
binding body that has the digest fields but omits the equality proof. This
keeps `sourceBindingDigest` from remaining audit-only bookkeeping detached
from the proof object consumed by `ofSourceBindings`.

P2FS-001 package-certificate closure hardening: the selected source-record
package now exposes `fiveSourceClosureCertificate`, built from the package's
constructor-bound per-source certificates and `Package.subEvidenceDigestsMatch`.
The source-record package closure theorem derives closure through that
certificate's `toClosureInputs`, so the aggregate five-source report and
sub-evidence digest match proof are consumed by the actual closure route rather
than merely built beside it. The Rust source audit requires this route and has
negative controls for aliasing either the package-derived certificate
constructor or the package closure reduction.

P2FS-001 aggregate certificate evidence binding: the proof-critical aggregate
`ReviewedFq12SelectedRustModelFiveSourceClosureCertificate` now carries
`evidenceReportBound` and `evidenceDigestBound`, and every selected constructor
route must pass `rfl rfl` after `selectedRustFiveSourceEvidenceDigest`. This
matches the predicate-model and source-binding evidence discipline and rules
out aggregate certificate construction over an unrelated public report/digest
pair.

P2FS-001 source-record package binding: the selected source-record package now
stores the five selected evidence/source binding objects and has equality
witnesses from each exported raw source projection back to those bindings. This
keeps the package from becoming a laundering point where selected evidence
bindings are consumed and then discarded before hidden-source closure.

P2FS-001 package certificate projection hardening: the five package-level
source certificates now project from the stored evidence/source binding
objects through `.toCertificate`, and
`ReviewedFq12SelectedRustModelSourceRecordPackage.subEvidenceDigestsMatch`
uses the five binding `evidenceDigestBound` proofs as the digest-match
arguments. The Rust source audit now requires those package certificate bodies
and those digest-bound arguments. The manifest remains fail-closed with
`source_record_package_surface_present = true`,
`package_sub_evidence_match_reduction_present = true`, and the same four
missing concrete categories.

P2FS-001 final-theorem package-certificate route: the future
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_instantiation`
gate now requires `selectedRustModelFiveSourceCertificate` to be assigned from
`selectedRustModelSourceRecordPackage.fiveSourceClosureCertificate` and to be
consumed by the certificate-level hidden-source closure theorem. A theorem that
manually rebuilds the five-source certificate and only mentions the package
projection no longer satisfies the gate.

P2FS-001 certified-package direct closure: the certificate-shaped native
boundary semantics package now derives `hiddenSourceClosure` directly from
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_source_record_package`
over `Package.sourceRecords`, instead of detouring through
`toSemanticsSourcePackage`. The Rust source audit requires that direct body and
keeps the alias-returning negative control.

P2FS-001 closure update: Lean now names the selected native-boundary predicate
projection, the proof-bearing field-source package certificate, the full
selected predicate-model certificate, the selected source-record package, and
the final selected-Rust extractor theorem
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_instantiation`.
Rust evidence now reports `predicate_model = true`,
`extractor_theorem_instantiated = true`, `semantics_bridge_complete = true`,
all selected source records present, and the concrete definition checklist
complete. The active hidden-source semantics bridge is closed; the next live
deployment proof blocker is checked-core composition.

P2FS-001 certified closure route guard: the future final theorem gate now
rejects raw native-boundary semantics package closure and source-shaped
semantics package closure markers. The active final proof must close through
`selectedRustNativeBoundaryCertifiedSemanticsSourcePackage.hiddenSourceClosure`,
keeping the selected predicate-model certificate on the closure route.
The current-report blocker is also recorded at the full-signature bound layer:
`full_signature_active_path_forgery_bound_from_current_predicate_report_impossible`
shows that a certified semantics package over the current fail-closed selected
predicate evidence would contradict the predicate-model certificate guard.
The selected native-boundary predicate interpretation now has a structured
target package:
`ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources.toPredicates`.
The source audit requires this package to route the critical break,
reviewed-Fq12, trace/fiber, slot-authorization, and spend predicates into
`Path2NativeBoundary.Predicates`, while still keeping concrete selected field
source definitions absent and fail-closed.

P2FS-001 final-theorem constructor-binding hardening: the future extractor
theorem gate now requires each selected certificate local to be bound directly
by its named constructor. A constructor laundering fixture where the
boundary-base constructor is present but assigned to a differently named local,
while `selectedRustBoundaryBaseCertificate` is mentioned later, is rejected.
The gate now also requires the constructor arguments to occur in the selected
report/digest/proof/source order for every per-source certificate and for the
aggregate certificate; a direct boundary-base constructor call with
`otherBoundaryBaseEvidenceReport` in the selected report position is rejected.

P2FS-001 selected-declaration name-boundary hardening: source inspection now
requires an identifier boundary after each selected declaration name. An
otherwise-complete fixture with
`selectedRustFiveSourceEvidenceReportSpoof` instead of
`selectedRustFiveSourceEvidenceReport` fails the report-definition category,
so prefix-matched declarations cannot close AP2-N001 checklist categories.

P2FS-001 final-theorem name-boundary hardening: the future final extractor
theorem gate now requires an identifier boundary after
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_instantiation`.
An otherwise-valid constructor fixture with the theorem name suffixed by
`Spoof` is rejected, so prefix-matched theorem declarations cannot flip
`extractor_theorem_instantiated`.
The scanner now evaluates all candidate theorem-prefix matches and accepts a
later exact theorem even if a suffixed spoof appears first, preventing a
fail-closed false negative once diagnostic or negative-control declarations sit
near the real AP2-N001 theorem.

P2FS-013 full-signature reduction scanner hardening: the
`selectedRustFullSignatureReductionBoundary_instantiation_v1` gate now uses the
same exact theorem-name and all-candidate scan pattern. A suffixed theorem is
rejected, while a suffixed spoof before an exact theorem no longer creates a
false negative for the future full-signature reduction certificate
instantiation.
The full-reduction instantiation gate now also scans the combined
`Path2NativeAccountDesign.lean` and `Path2FullSpendConcrete.lean` sources and
rejects a certificate-only theorem shape unless it consumes the concrete
full-execution source theorem, the concrete full-execution lift, the accepted
spend numeric-bound instantiation theorem, the accepted-spend certificate
conversion, `activeNativeHashBudgetBoundary`, and
`activeNativeHashBudgetBoundary_hashBoundaryProps`. This keeps the final
reduction from closing in the upstream account-design layer with local shells
instead of the active concrete execution source, numeric accepted-spend package,
and active hash boundary.
The full-reduction gate now also rejects local/global zero or bare-`True`
aliases for the selected signature probability, production pin, accepted-spend
inequality, and core full-signature modeling propositions, even when every
active prerequisite marker is present. This remains a fail-closed scanner
hardening; it does not instantiate the production full-reduction theorem.
The full-reduction boundary source gate now requires exact Lean declarations
for the selected reduction evidence report, report/facts predicates,
certificate structure, and certificate constructor/projection functions.
`SelectedRustFullSignatureReductionCertificate` now also exposes theorem
projections from full-signature success to the production probability, the
accepted-spend game success, the exported accepted-spend probability, the
exported accepted-spend bound, and the accepted-spend reported-security-bit
bound, then to target-security bits and the fixed active `2^-128` target. The
full-reduction source gate requires the future selected theorem to consume those
projections, so the final theorem cannot stop at a raw record conversion without
making the final target-security claim theorem-visible. The fixed target now
also has explicit projections after erasing the certificates to
`NativeAcceptedSpendSoundnessBoundary` and `NativeFullSignatureSecurityBoundary`,
so the theorem-visible target bound survives the public boundary conversion.
The AP2-N003 accepted-spend and full-reduction instantiation gates now
exact-check qualified Lean identifier references for these certificate
projection markers. Regressions replace the active 128-bit target projection
names with suffixed spoof names and keep both gates open, preventing substring
matches from satisfying the final target-bound route.
Regressions reject suffixed report/certificate structures, keeping the final
active reduction boundary tied to the named certificate family rather than a
shadow source shape.
The accepted-spend certificate now exposes
`successProb_le_acceptedSpendForgeryProb`, deriving packaged full-forgery
success below the exported accepted-spend probability through the selected
numeric source. The full-reduction boundary conversion consumes that projection,
and the source gate requires the future final theorem to mention it instead of
reaching directly into the raw priced certificate's union-bound proof.
The same gate now also rejects global `def`/`abbrev` aliases that set the
report/digest binding proofs, report-verified/facts proofs,
keygen/signing/completeness/verifier/full-forgery model proofs,
accepted-spend closure proofs, hash-domain proofs, query-budget proof, or
multi-target proof to `True.intro`; a regression prepends those proof aliases
to the otherwise accepted full-reduction theorem shape and keeps the gate open.
`SelectedRustFullSignatureReductionCertificate` now also carries
`evidenceReportBound` and `evidenceDigestBound` proof fields, consumed by
`toNativeFullSignatureSecurityBoundary` in both the probability inequality and
complete-claim construction paths. The full-reduction source gate requires
named selected report/digest binding propositions and proofs, and rejects
`True` aliases for those bindings.
`SelectedRustFullSignatureReductionEvidenceReport` now also carries
`reductionEvidenceDigestsVerified` plus public digest slots for production
signature probability, signature-to-accepted-spend linkage, full-execution
source consumption, active hash boundary consumption, query-budget pricing,
multi-target pricing, and verifier-game binding. `ReportVerified` requires the
digest-verification bit, and the full-reduction source gate requires these
markers before accepting the future final theorem shape.
Rust now exposes `Path2NativeFullSignatureReductionEvidenceV1` with schema,
length-delimited frame, digest, report-verification helper, all-facts helper,
verifier, production-facade exports, and registry membership. The canonical
report is intentionally fail-closed: schema/report/digest verification passes
and the seven public digest slots are filled from active Lean source windows,
but production signature probability pinning, signature-to-accepted-spend
linkage, full-execution-source consumption, active hash-boundary consumption,
query-symbol pricing, multi-target pricing, and verifier-game binding remain
false. The final full-reduction instantiation gate now requires this Rust
evidence source before scanning for
`selectedRustFullSignatureReductionBoundary_instantiation_v1`.
The same all-candidate exact-name scan now applies to
`concreteFullSignatureExecutionSource_instantiation_v1`, so full-execution
source closure also rejects suffixed theorem spoofs without making a later
exact theorem unreachable.
The full-execution source gate also rejects global `def`/`abbrev` aliases that
set the selected execution report proofs to `True.intro` or set the selected
execution-success predicates to `fun _ => False`; regressions prepend those
aliases to the otherwise accepted full-execution theorem shape and keep the
gate open.
`selectedRustAcceptedSpendSoundnessBoundary_instantiation_v1` now follows the
same pattern for the active accepted-spend numeric-bound gate: suffixed theorem
spoofs reject, and later exact selected declarations remain reachable.
The accepted-spend numeric-bound gate now scans the account-design,
hidden-source boundary, checked-core byte bridge, full-spend concrete, and
concrete no-export sources together. It rejects a numeric-certificate-only
theorem unless the source also consumes the active hidden-source closure,
selected checked-core instantiation and accepted-spend wrapper, concrete
full-execution source and lift, and selected no-export serializer closure.
This keeps the numeric handoff dependent on the active closure proofs rather
than locally named prerequisite propositions.
The accepted-spend numeric-bound gate now also rejects local or global aliases
that hide zero probability/bound values, `rfl`/`le_rfl` pins, empty evidence
digests, or bare `True` prerequisite closures behind the selected
accepted-spend names. This is a fail-closed scanner hardening only: the active
numeric-bound theorem remains open until production accepted-spend evidence
provides non-vacuous numeric values and the corresponding inequality proof.
`Path2AcceptedSpendProbability.lean` now adds the finite numeric layer for this
handoff: `AcceptedSpendNumericGame` proves that full-signature success
probability is bounded by the union over the reduced accepted-spend event and
the finite family of named break events, while
`AcceptedSpendBreakPricingCertificate` records the accepted-spend bound, one
bound per active named break, and the final finite-sum budget inequality into
the production-priced upper bound. `PricedAcceptedSpendNumericCertificate`
derives `unionBound_le_priced` from that constructive break-pricing package
rather than carrying a monolithic real inequality. The accepted-spend source
gate now requires this layer, so future closure must consume the probability
game and priced-bound package rather than only raw `ℝ` fields.
The accepted-spend evidence report now mirrors that split with separate
fail-closed bits for pricing the accepted-spend event bound, pricing the named
break-event bounds, and proving the break-pricing budget fits the production
priced bound; the aggregate `acceptedSpendUnionBoundPriced` bit is no longer
the only pricing readiness signal.
Those bits also have named `FactsAllTrue` projection theorems that are consumed
by the selected accepted-spend certificate and exact-checked by the Rust source
gate, so the final theorem must expose each constructive pricing subclaim on
the active proof route.
The remaining report readiness bits now have the same treatment: finite trace
support, active outcome carrier, accepted-spend event semantics, active
break-family, named-break semantics, numeric-budget readiness, selected QROM
loss, and prerequisite closure readiness are all exposed as named projections
consumed by the selected certificate and required by the source gate.
Report verification is similarly split into named projections for schema match,
report-digest verification, and numeric-evidence digest verification, so the
selected certificate cannot consume report identity as an opaque proof blob.
The priced certificate and selected numeric source now additionally expose
component-bound projections for the accepted-spend event, each named break
event, and the finite component-budget inequality into the priced bound; these
projection names are also required by the source gate.
The concrete active-carrier wrapper mirrors those component-bound projections,
so the future theorem must expose them before erasing the active outcome
carrier and break-family pins into the generic selected numeric source.
The numeric source now pins event semantics, not only carrier types.
`PricedAcceptedSpendNumericCertificate` exposes stored-instance accessors for
observed outcomes, accepted-spend events, and named break events, and
`SelectedRustConcreteAcceptedSpendNumericCertificate` must prove that priced
accepted-spend Boolean events imply the active concrete
`AcceptedSpendConclusion`, while priced named-break Boolean events imply the
active concrete `breakFor` predicate after transporting through the pinned
`Path2FullSpendGame.BreakEvent` equality. The accepted-spend source gate
requires these markers, preventing a future theorem from pricing arbitrary
predicates over the selected run carrier.
Rust now has a matching public evidence carrier,
`Path2NativeAcceptedSpendNumericEvidenceV1`, with length-delimited frame,
digest, report-verification helper, all-facts helper, verifier, and production
facade exports. The canonical report remains fail-closed: prerequisite closure
bits that are already closed are true, while numeric pricing, finite trace
support, active event pins, and union-bound pricing are false. A regression
checks that flipping those readiness bits in a copied report does not satisfy
the all-facts helper or verifier. The accepted-spend numeric instantiation gate
now requires this Rust evidence source before scanning for the future Lean
theorem.
That evidence now carries digest commitments for finite trace support, active
outcome carrier, accepted-spend event semantics, active break family, named
break event semantics, union-bound pricing, and the priced bound. The Lean
report mirrors those digest fields with an explicit
`numericEvidenceDigestsVerified` report-verification bit, mirrored in Rust as
`numeric_evidence_digests_verified`. Report verification rejects zero digest
placeholders, digest tampering, and a false digest-verification bit. This keeps
the future numeric theorem from closing against uncommitted Boolean flags
alone.
`SelectedRustAcceptedSpendSoundnessCertificate` also now carries
`evidenceReportBound` and `evidenceDigestBound` proof fields, consumed by
`toAcceptedSpendSoundnessBoundary`. The source gate requires named selected
report/digest binding propositions and proofs, and rejects `True` aliases for
those bindings, so the future accepted-spend theorem cannot attach arbitrary
report/digest bytes to otherwise plausible numeric fields.
The probability game now separates finite trace support from the observed
full-forgery outcome: `Trace` has `traceFintype`, while `Outcome` is reached
through `observe` and need not be finite. This is required for the active
Nat/native concrete run carrier, which should be observed from bounded finite
execution traces rather than treated as a finite type itself.
`SelectedRustAcceptedSpendSoundnessCertificate` now carries the
`SelectedAcceptedSpendNumericSource` package plus equality proofs from the
exported probability/bound to the source certificate's union expression and
priced bound. The conversion to `NativeAcceptedSpendSoundnessBoundary` derives
`acceptedSpendForgery_le_bound` from the packaged `unionBound_le_priced` proof
through that numeric source; a standalone raw inequality proof is no longer an
input to the selected constructor.
`PricedAcceptedSpendNumericCertificate` now also pins its break index to the
active full-spend break family `Path2FullSpendGame.BreakEvent`; the source gate
requires the corresponding projection and theorem markers, so the future
numeric theorem cannot price a different finite bad-event index while claiming
to close the active path.
`Path2FullSpendConcrete.lean` now also names the active numeric outcome carrier
as `selectedRustAcceptedSpendNumericOutcome = Path2FullSpendGame.FullForgeryRun
concreteTypes` and wraps the priced certificate in
`SelectedRustConcreteAcceptedSpendNumericCertificate`. The source gate requires
that wrapper and its forgetful map, so a future theorem cannot price an
unrelated finite outcome space while claiming to bound the selected concrete
full-forgery game.
The concrete wrapper now maps into the generic
`SelectedAcceptedSpendNumericSource` package through
`toSelectedAcceptedSpendNumericSource`, and the accepted-spend certificate
consumes that source package instead of a bare priced certificate. This keeps
the concrete carrier proof on the Lean certificate route, not only in Rust
scanner text.
The selected numeric-source package now exposes its own accepted-spend and
full-forgery probability-bound projections plus union-bound pricing, and the
accepted-spend certificate-to-boundary conversion consumes the selected source's
union-bound pricing proof instead of reaching directly into the raw priced
certificate.
The accepted-spend certificate now also exposes
`acceptedSpendForgeryProb_le_bound`, packaging the exported probability
inequality before conversion to `NativeAcceptedSpendSoundnessBoundary`; the
source gate requires the future selected theorem to consume this certificate
projection.
It also exposes `acceptedSpendForgeryBound_le_reportedSecurity` and
`acceptedSpendForgeryProb_le_reportedSecurity`, target-security variants, and
`acceptedSpendForgeryProb_le_active128BitTarget`, keeping both the reported
`acceptedSpendPricedBoundSecurityBits` and the final pinned 128-bit target
theorem-visible after the selected numeric source is packaged.
The certificate now also exposes named equality projections for exported
probability-to-union-bound, exported-bound-to-priced-bound, and production-value
pinning; the source gate requires the future selected theorem to consume those
projection names rather than relying only on raw record fields.
The concrete wrapper also exposes selected theorem projections for accepted-
spend probability below the priced bound, full-forgery success below the union
bound, and full-forgery success below the priced bound. The numeric-bound source
gate requires the future selected theorem to consume these concrete-wrapper
facts before erasing to the generic priced certificate.
The accepted-spend numeric evidence report now carries public budget numbers
for the target security bits, selected multi-target horizon, QROM loss,
QROM-discounted transcript term, and claimed priced-bound security bits, plus
`acceptedSpendNumericBudgetMeetsTarget`. `FactsAllTrue` pins the active
constants `targetSecurityBits = 128`, `selectedMultiTargetHorizonLog2 = 32`,
`qromQuadraticLossBitsAtDeploymentBound = 32`, and
`transcriptTermBitsAfterQromLoss = 182`, then requires the target predicate
plus the numeric inequalities
`targetSecurityBits ≤ acceptedSpendPricedBoundSecurityBits` and
`targetSecurityBits + selectedMultiTargetHorizonLog2 ≤
transcriptTermBitsAfterQromLoss`, and
`acceptedSpendPricedBoundSecurityBits + selectedMultiTargetHorizonLog2 ≤
transcriptTermBitsAfterQromLoss`; Rust mirrors those checks in the evidence
`all_facts_true` helper. This prevents over-reporting claimed priced-bound
bits beyond the active QROM-discounted transcript budget. The canonical Rust
evidence still has priced-bound security bits `0` and the target predicate
false, so it remains fail-closed.
Lean now exposes report-derived theorem facts that the accepted-spend
priced-bound bits meet target, the selected 32-bit multi-target horizon fits
inside the 182-bit QROM-discounted transcript term, the claimed priced-bound
bits plus that horizon also fit inside the transcript term, and the resulting
target arithmetic slack is 22 bits. It also names the selected transcript cap
`acceptedSpendPricedBoundSecurityBits ≤ 150` via
`factsAllTrue_pricedBoundSecurityBits_le_transcriptCap`. The accepted-spend
certificate-to-boundary conversion consumes those facts directly, and the
numeric-bound source gate requires the future selected theorem to consume them
too. This leaves the real-valued accepted-spend probability source and
priced-bound inequality as the remaining numeric gap.
`SelectedRustAcceptedSpendSoundnessCertificate` also carries
`acceptedSpendPricedBoundSecurityPriced`, proving the actual real-valued
`numericSource.pricedCertificate.pricedBound` is at most
`((2 : ℝ) ^ evidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹`. The
source gate requires the named selected proof and rejects `le_rfl` aliases, so
reported security bits cannot remain detached from the probability bound.
The source gate also rejects local/global `rfl` aliases for
`selectedRustAcceptedSpendForgeryProbMatchesUnionBound` and
`selectedRustAcceptedSpendForgeryBoundMatchesPriced`, keeping the exported
accepted-spend probability and bound tied to selected evidence rather than
local definitional shells.
The source gate also rejects global `def`/`abbrev` aliases that set
`selectedRustAcceptedSpendEvidenceReportBoundProof`,
`selectedRustAcceptedSpendEvidenceDigestBoundProof`,
`selectedRustAcceptedSpendBoundVerifiedProof`, or
`selectedRustAcceptedSpendFactsAllTrueProof` to `True.intro`; regressions
prepend those aliases to the otherwise accepted theorem shape and keep the
active numeric-bound gate open.
The accepted-spend numeric-bound source gate now requires the consumed
`selectedRustAcceptedSpendNumericSource` binding to come from
`SelectedRustConcreteAcceptedSpendNumericSourceCertificate.toSelectedAcceptedSpendNumericSource`
on a named `selectedRustConcreteAcceptedSpendNumericSourceCertificate`; the
concrete wrapper must also be extracted through
`SelectedRustConcreteAcceptedSpendNumericSourceCertificate.toConcreteAcceptedSpendNumericCertificate`.
A regression rejects the previous shape where the concrete conversion was
merely mentioned under another local name while `ofReportAndBound` received a
generic selected numeric source.
The same gate now rejects global `def`/`abbrev` record literals for
`selectedRustConcreteAcceptedSpendNumericCertificateSource`,
`selectedRustConcreteAcceptedSpendNumericSourceCertificate`, or
`selectedRustConcreteAcceptedSpendNumericCertificate`; regressions prepend such
sources to the otherwise accepted theorem shape and keep the gate open, so the
concrete active-carrier wrapper must come from a named production-derived
source route. It now also rejects theorem-local
`let selectedRustConcreteAcceptedSpendNumericCertificateSource := { ... }` and
`let selectedRustConcreteAcceptedSpendNumericSourceCertificate := { ... }`
construction immediately before the active bindings, so the final theorem
cannot fabricate that source in its body.
`SelectedRustConcreteAcceptedSpendNumericSourceCertificate` now carries the
accepted-spend evidence report, digest, report/digest binding proofs,
`ReportVerified`, `FactsAllTrue`, and the concrete active-carrier numeric
certificate. The remaining proof work is to construct that object from real
production accepted-spend numeric evidence and then price the accepted-spend and
named-break events through it.
The concrete numeric certificate now carries seven digest bytes for finite
trace support, active outcome carrier, accepted-spend event semantics, active
break family, named break events, union-bound pricing, and priced-bound pricing.
The source certificate requires equality proofs tying those concrete-certificate
digests to the corresponding evidence-report digest slots, and the source gate
requires named witnesses plus source-certificate projection theorem references
for each slot. This still leaves the production hash computation open, but the
final theorem shape can no longer put a verified report beside an unrelated
concrete numeric certificate without exposing the missing digest equalities.
`SelectedRustAcceptedSpendSoundnessEvidenceReport.ReportVerified` now also
requires all seven accepted-spend numeric digest slots to be nonzero
`ByteArray`s, and the source gate exact-checks the seven corresponding
`reportVerified_*_nonzero` projection theorems. This mirrors the Rust
verifier's rejection of placeholder digest slots and prevents the Lean report
verification route from collapsing to three Boolean flags.
The concrete numeric source certificate now derives nonzero digest facts for
the concrete certificate itself from those report-side facts plus the seven
report/certificate digest equalities. The accepted-spend source gate
exact-checks these source-certificate projections, so digest presence is carried
through to the active-carrier numeric certificate.
The concrete source certificate now also derives the remaining accepted-spend
numeric readiness facts from its own `evidenceFactsAllTrueProof`: numeric bound
priced, finite trace support priced, active outcome carrier pinned,
accepted-spend event pinned, active break family pinned, named-break events
pinned, union bound priced, accepted-spend component bound priced, named-break
component bounds priced, break-pricing budget below the priced bound, and
numeric budget meets target. The accepted-spend source gate exact-checks those
source-certificate projection names, so the future theorem must pull the
numeric facts through the report-bound concrete source package, not only
through standalone report-level projections. A focused source-gate regression
replaces the concrete-source union-bound fact projection with the report-level
projection and confirms that this no longer satisfies the active accepted-spend
numeric-bound gate.
The generic accepted-spend pricing layer now also exposes nonnegativity
theorems for the full-signature success probability, accepted-spend union
bound, accepted-spend component bound, each named-break component bound, and the
final priced bound at the
`AcceptedSpendNumericGame`, `AcceptedSpendBreakPricingCertificate`,
`PricedAcceptedSpendNumericCertificate`, and `SelectedAcceptedSpendNumericSource`
layers. `SelectedRustConcreteAcceptedSpendNumericCertificate` and
`SelectedRustConcreteAcceptedSpendNumericSourceCertificate` now project those
sign facts through the active concrete carrier/source route too, and
`SelectedRustAcceptedSpendSoundnessCertificate` proves the exported
accepted-spend probability and bound remain nonnegative after erasure to
`NativeAcceptedSpendSoundnessBoundary`. `SelectedRustFullSignatureReductionCertificate`
also proves selected and production signature-forgery success probabilities are
nonnegative, and the erased `NativeFullSignatureSecurityBoundary` preserves
nonnegativity of `signatureForgerySuccessProb`. The accepted-spend and
full-reduction source gates exact-check those theorem names, so the future
pricing package must expose nonnegative probabilities and budgets rather than
only upper-bound inequalities, and it must preserve those sign facts through
the concrete source and full-boundary handoffs.
The accepted-spend numeric and full-signature reduction certificates now bind
their evidence reports and digests by equality to concrete selected Rust
report/digest constants, rather than by arbitrary `Prop` slots. The selected
AP2-N003 reports mirror the Rust public evidence reports and are verified on
schema/digest/nonzero slots while remaining fail-closed on `FactsAllTrue`.
Future theorem constructors therefore have a type-level selected-evidence
identity obligation, not only scanner-enforced binding names.
Lean also proves the current selected accepted-spend numeric and full-reduction
reports cannot satisfy `FactsAllTrue`. These sanity guards keep the current
verified-but-incomplete public reports separate from the future complete
selected evidence values needed to close AP2-N003.
The same fail-closed distinction is now stated at the certificate boundary:
`SelectedRustAcceptedSpendSoundnessCertificate.currentSelectedReport_impossible`,
`SelectedRustFullSignatureReductionCertificate.currentSelectedReport_impossible`,
and
`SelectedRustConcreteAcceptedSpendNumericSourceCertificate.currentSelectedReport_impossible`
show that no current selected accepted-spend, full-reduction, or concrete
numeric-source certificate can exist while the selected report constants remain
in their present fail-closed state.
The Rust regression `path2_native_ap2_n003_lean_selected_reports_match_rust_evidence`
now parses those Lean selected report constants and compares every
digest/boolean/numeric field plus the selected report-frame digest constants
against the current Rust AP2-N003 evidence objects. This prevents stale Lean
selected reports after source-window digest churn.
`SelectedRustFullSignatureReductionEvidenceReport.ReportVerified` now applies
the same digest-presence requirement to the seven full-reduction evidence slots:
production signature probability, signature-to-accepted-spend link,
full-execution source, active hash boundary, query-budget pricing,
multi-target pricing, and verifier-game binding. The full-reduction certificate
erasure consumes those projections, and the future selected theorem gate
exact-checks them.
The full-reduction report also now exposes named `FactsAllTrue` projections for
each production reduction fact: signature probability pinning, accepted-spend
game linkage, accepted-spend probability linkage, full-execution source
consumption, active hash-boundary consumption, active query-symbol pricing,
multi-target horizon pricing, and verifier-game binding. The full-reduction
certificate erasure consumes those projections and the source gate exact-checks
them, so those facts cannot remain hidden inside one opaque all-true witness.
`Path2FullSpendGame.BreakEvent` now has a manual finite instance over the
active full-spend named breaks, so the numeric game can index its union bound by
the actual active break family.
The canonical Rust accepted-spend numeric evidence now fills its seven digest
slots from source windows instead of placeholder labels: finite trace support
and union/priced-bound logic from `Path2AcceptedSpendProbability.lean`, active
outcome and event semantics from `Path2FullSpendConcrete.lean`, and active
break-family enumeration from `Path2FullSpendGame.lean`. These are source
commitments only; readiness remains false until the production numeric pricing
theorem is instantiated.
`SelectedRustFullSignatureReductionCertificate` now consumes the proof-bearing
`SelectedRustAcceptedSpendSoundnessCertificate` directly and derives the erased
accepted-spend boundary internally. The full-reduction source gate requires a
named `selectedRustAcceptedSpendCertificate`, preventing the final theorem from
discarding the active numeric trace/outcome and bad-event pins before building
the full-signature boundary.
`selectedRustActiveNoExportSerializer_instantiation_v1` now follows the same
exact-name/all-candidate pattern for the active no-export serializer gate.
The full-execution boundary-presence check now also requires an exact
`concrete_fullExecutionImpliesAcceptedSpendForgery` theorem declaration name in
the active Lean source.
The no-export boundary-presence check now similarly requires an exact
`active_native_no_export_closure_from_public_output_no_export` theorem
declaration name.

| P2FS-002 | `acceptedSpendSoundness.checkedCoreCompositionClosed` | Closed for the active checked-core composition bridge. Lean now has `Path2NativeCheckedCoreAcceptV1`, `Path2NativeCheckedCoreAcceptV1Bridge`, byte-certificate theorems into `CheckedCoreSpendReduction.checkedCoreAccepts`, `SelectedRustCheckedCoreByteEvidenceReport`, digest pins for checked-core and hash byte-domain evidence, a component-level selected acceptor, `SelectedRustCheckedCoreByteEvidenceCertificate.ofReportAndComponents`, and `selectedRustCheckedCoreByteSource_instantiation_v1`. Rust source gates accept the selected theorem, reject suffixed/spoofed/vacuous variants, and the native preflight reports `checked_core_composition_review_complete=true` with no `PATH2_NATIVE_DEPLOYMENT_BLOCKER_CHECKED_CORE_COMPOSITION`. Targeted tests continue to cover proof length, canonicality, commitment/syndrome mismatch rejection, frame-prefix/digest/challenge binding, and active hash byte-domain evidence consumption. | No active AP2-N002 proof blocker remains. Continue with P2FS-003/P2FS-013: instantiate the full-signature execution source and production accepted-spend reduction. | Redesign only if later production execution binding shows the concrete checked-core verifier can accept a proof whose frame/proof/handle/hash components do not satisfy the component-level relation now used by the selected source. |

P2FS-002 checked-core evidence-constructor hardening: `SelectedRustCheckedCoreByteEvidenceCertificate` now carries the public evidence digest and active hash byte-domain evidence digest, and has named constructor `ofReportAndComponents`, taking the checked-core evidence report, both digests, verification proof, all-facts proof, parser/frame/hash component facts, a named production checked-core acceptor, and a proof that the selected acceptor equals that production acceptor. The future selected-Rust checked-core theorem gate now requires this constructor and rejects anonymous certificate record literals or constructor-built `False` acceptors, so the checked-core byte-source instantiation must be report/digest-bound, hash-domain-evidence-bound, and production-predicate-pinned before AP2-N002 can close.

P2FS-002 checked-core proof-shell hardening: the selected checked-core source
gate now rejects `abbrev` aliases that set the report proof names to
`True.intro`, and global aliases that set the selected or production checked
core acceptor to `fun _ _ _ _ => False`. Regressions prepend those aliases to
the otherwise accepted checked-core theorem shape and keep the gate open.

P2FS-002 closure note: AP2-N002 is now closed on the active path. The selected
checked-core theorem constructs the required evidence certificate from the
report/digest-bound component relation, and Rust preflight advances past
checked-core composition and full-signature execution-source instantiation to
the full-signature reduction certificate blocker.

| P2FS-003 | `acceptedSpendSoundness.lifecycleReplayClosed` | Full-spend theorem boundary and concrete carrier stated; selected full-execution source instantiated; production reduction certificate open. Rust tests cover reservation monotonicity, reserve/sign/consume, stale reservation rejection, verify-and-consume replay rejection, spent-ledger stale-wallet blocking, reorg burns, public-only lifecycle/spent-ledger serialization, prevalidated-slot consume-once, admission namespace duplicate rejection, and selected-projection replay rejection. `Descender/Path2FullSpendGame.lean` defines `AcceptedSpendConclusion`, `account_creation_prevalidated_slot_authorization_or_break`, and a full-forgery wrapper requiring the same prechallenge admitted leaf, checked-core tuple binding, and exact consume-once successor transition or a named break. `Path2FullSpendConcrete.lean` adds concrete carrier records for prevalidated leaves, creation evidence, admission records, direct certificates, signature bodies, consume state, checked-core binding, consume-once transition, rollback/reorg breaks, concrete accepted-spend soundness, `ConcreteFullSignatureExecutionSource`, `SelectedRustFullExecutionEvidenceReport`, and `concreteFullSignatureExecutionSource_instantiation_v1`. Rust source gates now accept the selected theorem and preflight advances past `PATH2_NATIVE_DEPLOYMENT_BLOCKER_FULL_SIGNATURE_EXECUTION_SOURCE`; with the reduction boundary shape present, the smallest active proof blocker is now `PATH2_NATIVE_DEPLOYMENT_BLOCKER_ACCEPTED_SPEND_NUMERIC_BOUND`. | Close the accepted-spend numeric-bound package and prerequisite closure facts first, then instantiate `selectedRustFullSignatureReductionBoundary_instantiation_v1` from that certificate, the selected full-execution source, active hash boundary, all active query-symbol pricing, and production EUF-CMA/blockchain evidence. | Redesign if target-chain lifecycle semantics can make a spent/revealed/burned/rollback-ambiguous slot spendable again without a named break, or if the production full-forgery event cannot be reduced to the selected execution-success predicate. |
| P2FS-004 | `acceptedSpendSoundness.noExportClosed` | Closed for the active selected serializer source. Lean has `FiberNoExportSoundness`, `PublicOutputNoExport`, `NoExportSoundness`, no-export-or-break theorem hooks, `Path2NoExport.ActiveNativeOutputConformance`, and `Path2NoExportConcrete.ActiveSerializerAllowlistCertificate`, which reduces tagged accept/reject/storage public-output frames to `ActiveNativeNoExportClosure` and `NativeAcceptedSpendSoundnessBoundary.noExportClosed`. `Path2NoExportConcrete` now defines a selected injective serializer frame encoder, public output tags 0/1/2, a distinct private denylist tag, and `selectedRustActiveNoExport`, proving public active frames cannot be denylisted. The selected theorem `selectedRustActiveNoExportSerializer_instantiation_v1` builds `SelectedRustActiveSerializerAllowlistCertificate.ofReportAndConformance` from report/digest-backed selected serializer evidence, extracts `PublicOutputNoExport` and the low-level allowlist through the certificate projections, and routes through the active no-export closure plus full-boundary adapter. Rust mirrors the report shape as `Path2NativeNoExportSerializerEvidenceV1`; focused regressions cover digest tampering, false component facts, private-marker output rejection, comment/string/stub spoofing, vacuous reports, and the synthetic case where reopening no-export gates before signer/CT/storage. Native deployment preflight now reports `no_export_serializer_instantiated=true` and no longer lists `PATH2_NATIVE_DEPLOYMENT_BLOCKER_NO_EXPORT_SERIALIZERS`. | No active P2FS-004 proof blocker remains. Continue with the full-signature reduction certificate and accepted-spend numeric-bound theorem, which can now consume no-export as a closed prerequisite. | Reopen/redesign only if a future concrete serializer exposes selected hidden material or if the production output schema needs a public field not covered by the selected public tags. |

P2FS-004 no-export proof-shell hardening: the no-export instantiation gate now
rejects global `def`/`abbrev` aliases that set the selected active serializer
report proof names to `True.intro`. A regression prepends those aliases to the
otherwise accepted no-export theorem shape and keeps the gate open.
P2FS-007/P2FS-008 exact active-boundary gate hardening: the QROM/hash-boundary
source gate now requires exact Lean declaration names for the active direct
budget, auxiliary-Fq12 challenge shape, Fq12 action-collision budget, selected
multi-target horizon, 54-bit negative control, byte-domain evidence route,
active native hash boundary, and full-signature active-hash handoff.
Regressions reject suffixed declarations such as
`AuxiliaryFq12ChallengeShapeCompleteSpoof`, `activeLaneAccountHorizonV1Spoof`,
and `full_signature_with_active_hash_boundary_propsSpoof`, so shadow or renamed
facts cannot satisfy the single active Path 2 QROM boundary gate.
| P2FS-005 | `hashBoundary.checkedCoreFrameDomainSeparated` | Lean active-domain certificate added; concrete byte-frame bridge still open but now report/digest-shaped and Rust-evidence-backed. Rust `Path2NativeCheckedCoreFrameV1` uses an explicit domain-prefixed canonical byte frame; tests cover golden-vector roundtrip, invalid context rejection, mutation-induced challenge changes, exclusion of legacy private handles, and active native domain/schema uniqueness. `Path2HashDomainAndMultiTarget.lean` pins disjoint active tags, fills the checked-core domain-separated proposition for the native hash boundary, names `ActiveHashByteDomainInstantiation`, and wraps it in `ActiveHashByteDomainEvidenceCertificate` before the active hash certificate consumes `.toInstantiation`. Rust now exports `Path2NativeHashByteDomainEvidenceV1` with pinned `PATH2_NATIVE_CHECKED_CORE_FRAME_DOMAIN`, hidden-source statement, projection-carrier binding, and lifecycle domains; frame/digest/verifier helpers; registry membership; disjointness; and duplicate-domain negative control. The QROM horizon gate requires this Rust evidence verifier as well as the Lean certificate source, and the checked-core byte-instantiation evidence now verifies and carries the same domain-evidence digest plus frame-prefix/digest/challenge-use facts. | Translate the checked-core report's domain-prefix/digest/challenge facts into the selected-Rust Lean theorem, then prove byte/frame correspondence to `CheckedCoreSpendReduction.frameEncoder`. | Redesign if checked-core hash frame overlaps another domain, omits a spend-binding field, or becomes cyclic. |
| P2FS-006 | `hashBoundary.hiddenSourceFrameDomainSeparated` | Lean active-domain certificate added; concrete hidden-source frame bridge still open but now report/digest-shaped and Rust-evidence-backed. Rust and docs include Path 2 domain/schema registry checks, direct-vs-batch mode separation, native hash/QROM review, and hidden-source/pad/carrier public transcript assumptions. `Path2HashDomainAndMultiTarget.lean` pins disjoint active tags, fills the hidden-source domain-separated proposition for the native hash boundary, names `ActiveHashByteDomainInstantiation`, and wraps it in `ActiveHashByteDomainEvidenceCertificate` for the concrete byte-domain registry. Rust `Path2NativeHashByteDomainEvidenceV1` pins the selected hidden-source frame to `REVIEWED_FQ12_HIDDEN_SOURCE_STATEMENT_DOMAIN` and proves it disjoint from checked-core, projection-carrier, and lifecycle domains before QROM horizon completion. | Tie the hidden-source statement/proof/opening/pad/carrier byte constructors and direct/batch mode separation to `Path2NativeHashByteDomainEvidenceV1.hidden_source_frame_domain` and companion domains in the selected-Rust theorem. | Redesign if any hidden-source frame shares a challenge/commitment domain ambiguously or if direct/batch domains can be replayed across modes. |
| P2FS-007 | `hashBoundary.queryBudgetPriced` | Lean budget mirror and native hash-boundary adapter added; final full-forgery reduction open. The widened route updates `path2_direct_query_budget_v1` to the selected auxiliary-Fq12 challenge accounting: 128-bit target, `2^16` quantum hash-query cap, `2^33` classical cap, 32-bit QROM quadratic loss, 182-bit transcript term after QROM loss, 53-bit strict deployment margin, and inactive `q_batch`. `Descender/Path2HashBudget.lean` mirrors those constants and packages the query pricing together with domain and multi-target props as `activeNativeHashBudgetBoundary`; `NativeFullSignatureSecurityBoundary.withActiveHashBoundary` now threads those props into the full active signature boundary. `SelectedRustFullSignatureReductionCertificate` now also requires `activeHashBoundaryConsumed`, `allActiveQuerySymbolsPriced`, `queryBudgetPricedProof`, and certificate-level projections from full-signature success through accepted-spend probability/bound to reported security before constructing the full boundary; Rust preflight has a fail-closed `full_signature_reduction_certificate_instantiated` gate for the absent `selectedRustFullSignatureReductionBoundary_instantiation_v1` theorem. | Instantiate the full-signature reduction certificate from production EUF-CMA/blockchain evidence, proving the numeric expression against the widened challenge and every active direct-mode query symbol rather than relying on raw boundary fields. | Redesign if any additional loss consumes the widened deployment margin or if the Fq12 action does not consume the full canonical challenge bytes. |
| P2FS-008 | `hashBoundary.multiTargetLossPriced` | `Path2HashDomainAndMultiTarget.lean` defines an explicit `ActiveMultiTargetHorizon`; the widened route now instantiates an active-lane horizon with account log2 = 32, spend log2 = 0, message log2 = 0, and aggregate target log2 = 32, with a 54-bit over-budget negative control. The selected route is auxiliary Fq12 action bits, not compact coefficient widening: first closure keeps 26 repetitions and targets `log2(T)=32`. The active hash-boundary props are now available through the full-signature handoff and through `SelectedRustFullSignatureReductionCertificate`, which requires `multiTargetHorizonPriced` and `multiTargetLossPricedProof` before constructing the full boundary; Rust rejects future instantiation shortcuts that assume the full certificate or full boundary. | Instantiate the full-signature reduction certificate with multi-target pricing for the widened action, including the Fq12 action-collision floor and canonical byte-frame binding. | Parameter redesign if account/network scale leaves insufficient margin after QROM and theorem composition losses. |
| P2FS-009 | `keygenDistributionModeled` | Productization blocker pinned; theorem open. Lean has `HonestCreationSoundness` and helper theorems for honest creation returning zero obstruction, local validity, and authorization-class compatibility. Rust scope KATs intentionally report the signer/keygen productization scope as not productized, with secret ownership, entropy, constructor, zeroization, and harness requirements pinned. Public prevalidation accepts valid projections and rejects invalid projections before private entropy. | Prove the production keygen/account-creation distribution emits native boundaries with valid private fibers and compatible authorization classes, except named sampler/RNG/constructor breaks; then flip productization only after the scope no longer reports the blocker. | Redesign if keygen can emit boundaries outside the native/Fq12 soundness relation or if secret ownership/zeroization cannot be made compatible with verifier-facing evidence. |
| P2FS-010 | `signingOracleModeled` | Signer evidence present; oracle theorem open. Rust tests pin public rejection cleanup, prevalidated signer execution reports, SHA-256 signer boundary evidence, CT evidence ledger separation, and no private reject output; the CT ledger still explicitly separates pinned evidence from open whole-signer timing claims. | Model the signing oracle over reserved live/prevalidated slots and prove every signing result is either a valid accepted spend candidate over the active frame or a public failure, with no secret-dependent output and named CT/RNG/sampler breaks. | Redesign if signer behavior cannot be represented without leaking/reusing hidden source material or if CT/productization blockers cannot be closed. |
| P2FS-011 | `completenessModeled` | Honest public-boundary flow and gate-separation evidence added; production completeness still blocked. Rust rejects randomizer mismatch and pins the private-fiber bridge scope. `Path2NativeSignerVerifierPipelineEvidenceV1` shows prevalidated honest construction reaches selected-verifier public-boundary acceptance, while production acceptance remains fail-closed and the current smallest named gate is `PATH2_NATIVE_DEPLOYMENT_BLOCKER_ACCEPTED_SPEND_NUMERIC_BOUND`. Gate ordering is now factored through `path2_native_smallest_remaining_deployment_gate_v1`; a synthetic test still confirms that no-export would be reported before signer/CT/storage productization if the no-export instantiation bit reopened. | Prove honest create plus honest prevalidated signing plus valid live state verifies under the selected Path 2 native/Fq12 production verifier and consumes the intended slot, after signer/verifier pipeline, CT, and storage gates are retired. | Redesign if the honest flow requires verifier inputs outside the active public boundary or if constructor/verifier frame equality cannot be made exact. |
| P2FS-012 | `verifierRelationMatchesImplementation` | Fail-closed implementation evidence plus honest public-boundary and gate-separation evidence present; byte-for-byte Lean bridge open. Rust preflight names the selected Path 2 native/Fq12 route and keeps deployment not ready. `path2_native_deployment_preflight_v1` reports public-boundary preflight, projection encoders, checked-core frame, public serializers, consensus carrier, verifier report, registry, hash review, facade exports, and no-export serializer instantiation present. `Path2NativeSignerVerifierPipelineEvidenceV1` additionally confirms the honest prevalidated signer output reaches selected-verifier public-boundary acceptance, but production still reports `production_accepted = false` with full-reduction, accepted-spend numeric, signer/verifier pipeline, CT, and storage blockers. Checked-core now has a Lean selected-Rust source contract, but the full verifier/pipeline byte-for-byte theorem remains open. | Instantiate the checked-core selected-Rust source, then add executable evidence bridges for signature bytes, carrier bytes, hidden-source frame domains, parser canonicality, public preflight, fail-closed production acceptance, and negative tests. | Redesign if the implementation path diverges from the Lean Path 2 native relation or if production acceptance requires fields outside the Lean public boundary. |
| P2FS-013 | `fullForgeryImpliesAcceptedSpendForgery` and `signatureForgerySuccess_le_acceptedSpend` | Abstract full-game lift and concrete carrier stated; production EUF-CMA/blockchain instantiation open. `Descender/Path2FullSpendGame.lean` states `account_creation_prevalidated_slot_authorization_or_break` and `fullForgeryImpliesAcceptedSpendForgery`. `Descender/Path2FullSpendConcrete.lean` instantiates those wrappers over native/account-shaped records for account creation evidence, prevalidated admission, direct certificates, checked-core binding, consume-once state, rollback/reorg breaks, and concrete verifier acceptance. It now adds `ConcreteFullSignatureExecutionSource` plus `concrete_fullExecutionImpliesAcceptedSpendForgery`, avoiding reliance on the tautological `ConcreteFullSignatureForgerySucceeds = ConcreteSpendVerificationAccepts` path for production closure. `Path2NativeAccountDesign.lean` now also requires `NativeAcceptedSpendSoundnessBoundary` and `SelectedRustAcceptedSpendSoundnessCertificate` to carry production probability/bound fields and equality proofs pinning the exported accepted-spend probability and bound to those production values, plus `SelectedRustAcceptedSpendSoundnessEvidenceReport` before installing accepted-spend probability/bound evidence. `SelectedRustFullSignatureReductionCertificate` adds the full-signature probability/reduction layer: report digest, production signature probability pinning, equality to the accepted-spend numeric game's `successProb`, full-execution source consumption, active hash-budget consumption, and proof witnesses for every full-boundary proposition before constructing `NativeFullSignatureSecurityBoundary`; the final `signatureForgerySuccess_le_acceptedSpend` boundary field is derived from `successProb_le_unionBound`. The source gates strip comments/strings and reject spoofing, missing witnesses, assumed full-certificate/full-boundary parameters, and multiline selected-evidence stubs, but `selectedRustFullSignatureReductionBoundary_instantiation_v1` is absent and no theorem currently instantiates these certificates from the production game. | Bind `ConcreteFullSignatureExecutionSource`, `SelectedRustAcceptedSpendSoundnessEvidenceReport`, and `SelectedRustFullSignatureReductionCertificate` to the production EUF-CMA/blockchain spend-forgery game over account creation, signing, prevalidated slot admission, spend verification, state transition, and reorg/rollback handling; prove every successful forgery induces an accepted-spend event or one of the priced bad events, then pin signature success to the accepted-spend numeric game's `successProb`. | Redesign if the blockchain notion of success includes outcomes not captured by accepted spend, or if postchallenge slot creation/rewrite cannot be excluded by the timing game. |

## Immediate Priority

By proof dependency, P2FS-001 remains the deepest active obligation. By
redesign-risk ordering, P2FS-008/P2FS-007 still come first, but the old
`log2(T) <= 1` horizon failure is no longer the active route assumption. The
selected route is the auxiliary-Fq12 widened challenge plan: 26 compact
repetitions, 23 algebraic challenge bytes with mask `0x3f`, 182 QROM-discounted
algebraic challenge bits, 214 total challenge bits, strict margin 53, and an
active aggregate horizon of `log2(T)=32`. The remaining work is to prove the
full-forgery query-budget expression and multi-target pricing over that widened
action, including the Fq12 action-collision floor and canonical byte-frame
binding. Deployment remains fail-closed until those pricing theorems and the
downstream proof gates close.

After the widened-budget pricing proof pass, return to P2FS-001. The abstract
theorem shape is now in Lean. `Descender/Path2NativeBoundary.lean` defines
`ReviewedFq12HiddenSourcePayload`,
`ReviewedFq12HiddenSourceSemanticsCertificate`, and
`reviewed_fq12_hidden_source_semantics_closed_or_break`, which takes an
accepted reviewed-Fq12 hidden-source payload plus native boundary digest/slot
agreement and returns:

```text
valid_native_boundary_representative
  OR hidden_source_soundness_break
  OR checked_core_binding_break
  OR domain/qrom_break
```

The next concrete move is to supply the byte predicate source theorem for the
Rust reviewed-Fq12 verifier, generate the sidecar bridge from it, and use the
source/local predicate facts to instantiate
`ReviewedFq12AssemblyCertificateBindingOrBreak` and
`ReviewedFq12ExtractionCompatibilityCertificateBindingOrBreak`. The Rust
bridge audit keeps this distinction explicit:
the current implementation evidence supports binding/no-export claims, carries
the native boundary digest through a concrete payload binding, and emits a
selected hidden-relation extraction certificate for accepted tuples. Lean now
packages the active bridge as `ReviewedFq12ExtractionBridgeCertificate` and
derives the older `ReviewedFq12HiddenSourceSemanticsCertificate` from it.
The certificate-to-native-representative step is decomposed again: selected
evidence must first lift to a `FullNativeRepresentativeOpening`, then that
opening must assemble into the hidden representative. This confirms the core
constraint: the current selected certificate does not itself contain all native
private roles.

Predicate-provider status: the active Lean source still has only the target
`ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources` package shape and
no concrete selected `Path2NativeBoundary.Predicates` implementation. The Rust
scanner now computes the concrete predicate-model presence bit directly for the
concrete-definition checklist instead of deriving it through the source-record
subcheck, so checklist fail-closed status does not depend on that subcheck's
future report layout.

Selected source-record status: Rust has concrete source-route, canonical
representative witness, native representative digest-frame witness, direct
assembly binding, and local-model evidence. The Lean source-record package has
not yet instantiated `selectedRustSharedFrameWitnessSources`; that is the first
reviewed-Fq12 subroute source record most likely to expose a real issue because
it must combine the canonical representative byte source with the native
digest-frame witness source rather than merely recomputing the assembly frame.
The proof route now has a certificate-consuming wrapper,
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_certificate`,
so the eventual source record can expose shared witness sources through
`ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate.toWitnessSources`
instead of passing the raw witness package directly.
The source-record package closure uses the same discipline: it now routes
through `Package.sharedFrameWitnessSourcesCertificate` and the certificate-level
hidden-source theorem, rather than detouring through raw five-source closure
inputs.
The package-to-input projections now also use
`Package.sharedFrameWitnessSourcesCertificate.toWitnessSources`; Lean records
the equality back to the raw package field with
`sharedFrameWitnessSourcesFromCertificate`, and the Rust scanner requires this
certificate projection shape.
The source-record subcheck now exposes the predicate-provider gap by family:
core/admission-spend, transparent/trace/hash-row, and reviewed-Fq12/full-opening
provider fields. The current active manifest reports all three families absent,
so the first real construction remains a selected native-boundary predicate
provider package, not another report-only certificate wrapper.
The reviewed-Fq12/full-opening family now has its own Lean target slice,
`ReviewedFq12SelectedRustNativeBoundaryReviewedFq12PredicateFieldSources`, plus
the projection
`ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources.toReviewedFq12PredicateFieldSources`.
The manifest distinguishes that target from implementation:
`native_boundary_predicate_model_reviewed_fq12_provider_family_target_present =
true`, while
`native_boundary_predicate_model_reviewed_fq12_provider_fields_defined_present =
false` and the missing provider-family count remains 3. That keeps the
highest-risk provider family explicit without treating target-shaping work as a
concrete selected predicate implementation.
The scanner also now rejects a likely circular shortcut for this family: a
selected reviewed-Fq12 predicate provider cannot be counted as concrete if its
definition reaches into the downstream source-record/source-route theorem stack
(`ReviewedFq12SelectedRustModelSourceRecordPackage`,
`ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs`, or the
`reviewed_fq12_certificate_soundness_from_selected_rust...` theorems). Those
proofs are parameterized by `P` and already assume the predicate fields, so
they can validate laws for a chosen provider but cannot define the provider
itself. The current manifest reports
`native_boundary_predicate_model_reviewed_fq12_provider_circular_source_dependency_present =
false`.

Reviewed-Fq12 provider mapping from the active Rust verifier surface:
`selectedReviewedFq12PayloadAccepts` and
`selectedReviewedFq12PayloadProjectsFrom` must be tied to a successful
`verify_path2_reviewed_fq12_hidden_source_payload_binding_v1` plus native
boundary digest/slot agreement, not merely digest equality. Extraction
certificate availability/validity maps to
`verify_path2_reviewed_fq12_hidden_relation_extraction_certificate_v1`;
role-lift fields map to
`verify_path2_reviewed_fq12_full_native_opening_certificate_v1`; assembly
certificate fields map to
`verify_path2_reviewed_fq12_native_representative_assembly_certificate_v1`;
compatibility fields map to
`verify_path2_reviewed_fq12_extraction_compatibility_certificate_v1`; and
aggregate bridge-digest fields map to
`verify_path2_reviewed_fq12_full_opening_bridge_certificate_v1`. The remaining
hard provider fields are the ones that turn those certificates into the native
`HiddenRepresentative` relation:
`selectedFullNativeOpeningAssemblesRepresentative`,
`selectedReviewedFq12AssemblyCertificateAssembles`,
`selectedReviewedFq12ExtractionCompatibilityCertificateExtracts`,
`selectedReviewedFq12CertificateExtracts`, and
`selectedReviewedFq12PayloadExtracts`. Those are exactly where the canonical
representative and shared digest-frame witness source records are needed; they
cannot be supplied by the aggregate bridge verifier alone.
The Rust source-record subcheck now reports this hard subgroup separately as
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_provider_family_target_present`
and
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_provider_fields_defined_present`.
The current active source reports the Lean target-slice bit as `true` and the
concrete provider-definition bit as `false`; the scanner regression's complete
native-boundary-model fixture reports the provider-definition bit as `true`.
This keeps the highest-risk redesign trigger visible while the broader
reviewed-Fq12 provider family remains unimplemented.
Lean now projects both the full reviewed-Fq12 provider family and this
five-field hard subgroup through the field-source package certificate and the
predicate-model certificate. That keeps the next AP2-N001 construction aimed at
the proof-bearing certificate boundary rather than raw provider-record access.
The Rust source audit pins those projections with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_provider_certificate_projection_present`.
The Lean boundary now also names
`ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelLaws`, and
the Rust source audit pins the source/certificate projections with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_provider_law_bundle_target_present`.
That splits AP2-N001's hard subgroup into concrete provider definitions versus
the laws needed to consume them in hidden-source soundness.
The paired package
`ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelPackage`
now ties the five projected providers to those laws for one predicate
interpretation, and the manifest pins it with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_provider_package_target_present`.
The Rust circular-provider guard now treats that hard package, its law bundle,
the `reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates` projection,
and the hard package/law projection functions as forbidden provider sources.
The regression now checks `selectedFullNativeOpeningAssemblesRepresentative`
against that shortcut directly.
Lean now also exposes the first concrete non-circular provider-construction
route: `reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame` builds
the five hard predicates from a selected native representative digest frame.
This pins representative digest canonicality for full opening assembly,
assembly-certificate assembly, compatibility extraction, certificate
extraction, and payload extraction, while leaving native fiber/local/descent
laws to the selected source records. The manifest tracks the route with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_digest_frame_provider_constructors_present`.
The route now also has a certificate-shaped entry point:
`reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate`
first consumes the active shared digest-frame witness evidence certificate and
then derives the hard-provider slice from the certificate's shared frame. The
manifest tracks this with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_shared_witness_certificate_route_present`,
while the selected provider fields themselves remain intentionally fail-closed.
The proof target now also names the final hard-provider selection certificate:
`ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate`
must prove that the certificate-derived hard-provider slice equals
`reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates predicates` and
must supply the hard-extraction law bundle for the same predicate model. This
is tracked by
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_shared_witness_model_certificate_target_present`.
The hard law bundle is no longer opaque in this route:
`reviewedFq12HardExtractionLawsOfSharedWitnessCertificate` derives the
certificate-to-payload, assembly-to-full-opening, and
compatibility-to-reviewed-extraction laws from the shared-witness/digest-frame
provider structurally. After the five-field equality is proved, the only
remaining hard-law inputs are payload extraction in the boundary fiber and
payload extraction native local/descent validity. The manifest tracks this as
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_shared_witness_law_reduction_present`.
Those two remaining inputs are now shaped as a full-opening native-validity
source:
`ReviewedFq12SharedWitnessFullOpeningNativeValiditySource` proves
fiber/local/descent for each full-opening assembly witness unpacked from
shared-witness payload extraction, and
`reviewedFq12HardExtractionLawsOfSharedWitnessFullOpeningSource` derives the
full hard law bundle from that source. The manifest tracks this as
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_full_opening_native_source_target_present`.
The source is now reduced to an exact canonical-representative target:
`ReviewedFq12SharedWitnessCanonicalFullOpeningNativeValiditySource` proves
fiber/local/descent for
`reviewedFq12FullNativeOpeningCanonicalRepresentative` computed from the
report-verified shared digest frame, and
`reviewedFq12SharedWitnessFullOpeningNativeValiditySourceOfCanonicalSource`
lifts it to the full-opening native source. The manifest tracks this as
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_canonical_native_source_target_present`.
The canonical target now has a field-level adapter:
`ReviewedFq12SharedWitnessCanonicalFullOpeningNativeValidityFieldEquationSource`
lists the exact selected-predicate and boundary/opening equalities required by
the active Rust/model import, and
`reviewedFq12SharedWitnessCanonicalFullOpeningNativeValidityEquationSourceOfFieldEquations`
derives the three canonical native-validity equations from that list.
`ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource.ofCanonicalNativeFieldEquations`
feeds this field-level source directly into the fixed-point route, and the
fixed-point equality now projects the three selected-predicate identification
facts directly. The predicate-model report and certificate route are now
fact-complete; the remaining import is the final theorem that consumes the
selected predicate-model certificate and selected source-record package.
The likely redesign-forcing local-account mismatch has been removed from the
Lean model. `selectedLocalValid` now records only native-field
self-consistency at the representative layer; the previous equality between
the selected account-projection digest and `Context.accountId` was not true of
the active Rust route, where the selected projection digest comes from
extracted account-projection lanes and `account_id` is the public account
identity digest. Public slot binding remains in payload/certificate acceptance,
and account binding remains in the native boundary/account projection
reconstruction rather than this local validity predicate.
The same retarget removed the unbacked descent equalities:
`selectedDescentConsistent` no longer asks full-opening digest commitments
(`p_lifecycle_digest`, opening commitment, projection-carrier digest) to equal
scalar/context coordinates (`slotId`, `lifecycleAnchor`,
`aggregationContextDigest`). It now records self-consistency of the opened
native commitment fields while boundary reconstruction still binds the opened
lifecycle/opening tuple to the selected boundary fields, so the remaining
model-alignment question is whether those selected boundary field names match
the Rust native-boundary frame objects.
Account/admission predicates were retargeted the same way:
`selectedAccountCreateAccepts` only checks context identity, and
`selectedAdmitted` delegates to that account-creation predicate. The removed
`nativeBoundaryDigest = creationStatementDigest` and `liveStatusMarker = slotId`
equalities were not active Rust facts; native digest and slot binding are
handled by payload/certificate acceptance.
The primary concrete digest-identity example is now repaired: Rust keeps
`native_boundary_digest` and `representative_digest` separate, with the latter
computed from the former plus opening fields. Lean now carries both fields in
`HiddenRepresentative` / `FullNativeRepresentativeOpening`, and
`selectedBoundaryOf` writes `Boundary.nativeBoundaryDigest` from the native
digest rather than the representative digest.
The hard model certificate route now has a constructor:
`ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate.ofCanonicalNativeSource`
uses the shared-witness certificate, the field equality, and the canonical
native-validity source to build the certificate and derive the law bundle
internally. The manifest tracks this as
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_model_certificate_constructor_present`.
The hard package is now explicitly tied back to the full predicate-model
certificate: `hardExtractionSharedWitnessPackageForPredicateModelCertificate`
routes a `ModelCert` through a shared-witness hard certificate over
`ModelCert.predicates`, and
`hardExtractionSharedWitnessPackageFieldsMatchPredicateModelCertificate`
proves its hard fields match
`ModelCert.toHardExtractionPredicateModelPackage.fieldSources`. The manifest
tracks this as
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_predicate_model_bridge_present`.
AP2-N003 concrete numeric-source adapter note: the accepted-spend numeric route
now has a direct adapter from the report-bound concrete numeric source to the
selected accepted-spend soundness certificate and erased boundary:
`SelectedRustConcreteAcceptedSpendNumericSourceCertificate.toSelectedRustAcceptedSpendSoundnessCertificate`
and `.toSelectedRustAcceptedSpendSoundnessBoundary`. This keeps the future
accepted-spend numeric-bound instantiation from manually assembling
`SelectedRustAcceptedSpendSoundnessCertificate` after merely mentioning a
concrete source. The adapter still requires the real production probability
pins, priced-bound security inequality, all selected report facts, and the
closed hidden-source/checked-core/lifecycle/no-export propositions. The Rust
source gate now requires these adapter names exactly and rejects spoofed
suffixes. The selected accepted-spend event-semantics source digest and report
digest were refreshed to reflect the new adapter window; the report remains
verified but not `FactsAllTrue`.
AP2-N003 accepted-spend closure-proof note: the selected accepted-spend
certificate now carries proof fields for the hidden-source, checked-core,
lifecycle/replay, and no-export closure propositions. The concrete
numeric-source-to-certificate and numeric-source-to-boundary adapters require
those proofs. The Rust accepted-spend numeric-bound instantiation gate requires
the proof names and rejects local/global `True.intro` proof aliases, keeping
the accepted-spend numeric route from advancing with proposition names alone.
AP2-N003 full-reduction closure dependency note: the full-signature reduction
constructor now derives its accepted-spend closure proofs from the consumed
`SelectedRustAcceptedSpendSoundnessCertificate` instead of taking standalone
closure proof arguments. The source gate's positive shape now projects
`selectedRustAcceptedSpendCertificate.hiddenSourceProofSemanticsClosedProof`,
`.checkedCoreCompositionClosedProof`, `.lifecycleReplayClosedProof`, and
`.noExportClosedProof` before constructing the full-reduction certificate. This
removes the remaining duplicate closure-witness handoff between accepted-spend
numeric closure and full-signature reduction.
