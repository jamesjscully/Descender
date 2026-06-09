# Active Path Signature Audit Tracker

Date: 2026-06-02

Scope: initial independent Lean-only audit of the active path signature theorem surface. The first pass was based on theorem statements, definitions, and direct import dependencies, not on existing issue trackers or research notes. A later note review corrected which theorem surface is actually active for deployment.

## Active Target Lock

There is exactly one active deployable-signature target in this tracker:

**Path 2 native / reviewed Fq12 active-lane projection-carrier route**, as named by the Rust production preflight and represented in Lean by `NativeFullSignatureSecurityBoundary` in `Descender/Path2NativeAccountDesign.lean`.

The following are not active targets:

- Compact active-only six-replay endpoint in `Descender/SignatureTranscript.lean`; use only as retired-path / negative-control context.
- q12289 FS-core checked-systematic endpoint in `Descender/SignaturePublicViewWitness.lean`; use only as a reference proof shape or fallback evidence source unless deployment is explicitly switched back.

References to `q12289` inside the active route are parameter/modulus/reviewed-Fq12 component references, not a switch of the active theorem target back to the q12289 FS-core endpoint.

## Current Verdict

The Lean files compile, but the active path signature surface is not yet a closed cryptographic security proof.

There are multiple theorem surfaces, but only one active deployment target:

- Compact active-only six-replay path in `Descender/SignatureTranscript.lean`: proves a conditional extraction statement, but is now explicitly demoted to a negative-control/non-deployable path. Its no-breaker endpoint is contradictory in the unbounded Lean model and its sixth-root bound is not cryptographically meaningful.
- q12289 checked-systematic path in `Descender/SignaturePublicViewWitness.lean`: proves a cleaner transcript soundness bound, but is not the currently selected production verifier according to Rust deployment preflight. It remains useful as a reviewed theorem pattern/fallback evidence source unless deployment is changed back to q12289.
- Active production path: Rust preflight currently names the selected verifier as the Path 2 native / reviewed Fq12 active-lane projection-carrier route, still fail-closed pending hidden-source proof semantics, CT, storage, and review gates.

## Note Review Correction

The active deployment notes identify Path 2 native / reviewed Fq12, not the compact six-replay endpoint or the q12289 FS-core theorem, as the current deployable-signature target. Treat compact findings as retired-path/negative-control findings and q12289 findings as reference-shape or fallback findings unless deployment is explicitly switched back to q12289.

## Active Path Blockers

| ID | Status | Issue | Detailed tracker |
| --- | --- | --- | --- |
| AP2-001 | Closed for active hidden-source semantics bridge | P2FS-001 now has the selected Rust predicate-model certificate, source-record package, and final reviewed-Fq12 hidden-source semantics theorem. Rust evidence reports `predicate_model=true`, `extractor_theorem_instantiated=true`, and `semantics_bridge_complete=true`; the native deployment preflight no longer reports `PATH2_NATIVE_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_SEMANTICS`. Historical notes below still record the route that led to this closure. | `P2FS-001` |
| AP2-002 | Closed for active checked-core composition | `Descender/Path2CheckedCoreByteBridge.lean` now mirrors the selected checked-core byte evidence report, pins the report digest and active hash byte-domain evidence digest, defines the selected component-level acceptor, builds `SelectedRustCheckedCoreByteEvidenceCertificate.ofReportAndComponents`, and exposes `selectedRustCheckedCoreByteSource_instantiation_v1`. Rust source gates now accept the selected theorem and `checked_core_composition_review_complete=true`; the native deployment preflight no longer reports `PATH2_NATIVE_DEPLOYMENT_BLOCKER_CHECKED_CORE_COMPOSITION`. | `P2FS-002` |
| AP2-003 | Full-execution source instantiated / reduction certificate open | Lifecycle/replay controls have Rust local evidence for reserve/sign/consume, verify-and-consume replay rejection, spent-ledger stale-wallet blocking, reorg burns, prevalidated-slot consume-once, and selected-projection replay rejection. `Descender/Path2FullSpendGame.lean` states the direct-mode accepted-spend and full-forgery wrapper theorem; `Descender/Path2FullSpendConcrete.lean` adds a concrete carrier for prevalidated leaves, creation evidence, admission records, direct certificates, signature bodies, consume state, checked-core binding, rollback/reorg breaks, concrete accepted-spend soundness, and the selected Rust `ConcreteFullSignatureExecutionSource` instantiation. Rust preflight now marks `full_signature_execution_source_instantiated=true` and no longer reports `PATH2_NATIVE_DEPLOYMENT_BLOCKER_FULL_SIGNATURE_EXECUTION_SOURCE`. The remaining proof gap is the production full-signature reduction certificate, including the accepted-spend numeric bound and prerequisite closure facts. | `P2FS-003`, `P2FS-013` |
| AP2-004 | Hash boundary props filled / widened deployment horizon in progress | Hash/QROM accounting has Rust budget/review KATs for direct-mode query caps, inactive batch terms, public native transcript assumptions, and domain/schema uniqueness. Deployment route decision is auxiliary Fq12 action bits: append canonical auxiliary algebraic challenge bits after the compact block/sign prefix, have the checked-core Fq12 action consume the whole longer challenge byte string, target 182 algebraic bits plus 32 hardening bits for `log2(T)=32`, and keep the 26-repetition compact local semantics unchanged for first closure. `Descender/Path2HashBudget.lean` and `Descender/Path2HashDomainAndMultiTarget.lean` are being updated from the old one-bit cap to the widened 182-bit QROM-discounted transcript term, 53-bit strict deployment margin, 32-bit active aggregate horizon, and 54-bit negative control. Rust native deployment preflight is being updated so QROM horizon completion is no longer the smallest productization gate once the widened constants are installed. | `P2FS-005` through `P2FS-008` |
| AP2-005 | Honest-flow and gate-separation evidence added / productization blockers pinned | Keygen, signing-oracle, and completeness are still deliberately not closed: Rust scope objects report signer/keygen productization and honest-constructor completion as false. Rust evidence shows the prevalidated honest constructor flow reaches selected-verifier public-boundary acceptance, while production acceptance remains fail-closed on the signer/verifier pipeline, CT, and storage gates; an additional test keeps those gates separate and prevents CT/storage non-claims from being hidden behind public-boundary acceptance. | `P2FS-009` through `P2FS-011` |

AP2-001/AP2-002 status override: the active hidden-source semantics bridge and
active checked-core composition bridge are closed. The AP2-N003 full-execution
source gate is also closed; the next active proof blocker is the
accepted-spend numeric bound, followed by the full-signature reduction
certificate instantiation and remaining productization gates.
| AP2-006 | Public-boundary evidence present / byte-for-byte bridge open | Rust preflight and pipeline tests identify the selected Path 2 native/Fq12 verifier surface and keep it fail-closed. Public-boundary evidence, projection encoders, checked-core frame, serializers, consensus carrier, registry, hash review, signer/verifier evidence, and productization-gate separation tests are present, but the pipeline still reports no production acceptance and internal blockers for signer/verifier pipeline, CT, and storage after the proof-side blockers. The Lean relation is not yet a byte-for-byte theorem about that selected Rust verifier. | `P2FS-012` |
| AP2-007 | Closed for active no-export serializer source | Public native views, facade exports, replay rejection paths, state snapshots, and spent-ledger records have Rust no-selected-material checks. `Descender/Path2NoExport.lean` proves that accept/reject/storage serializer conformance supplies `noExportClosed`; `Descender/Path2NoExportConcrete.lean` now instantiates the selected active serializer theorem with injective frame encoding, public tags 0/1/2, a distinct private denylist tag, report/digest-bound serializer evidence, and the full-boundary no-export adapter. Rust preflight now has `no_export_serializer_instantiated=true` and no longer reports `PATH2_NATIVE_DEPLOYMENT_BLOCKER_NO_EXPORT_SERIALIZERS`. | `P2FS-004` |

AP2-N001 audit-gate note: `rust_concrete_role_root_field_binding_present` now has its own source-inspection helper, `path2_reviewed_fq12_concrete_role_root_field_binding_source_present_v1()`, instead of aliasing the broader role-root/lift helper. The helper checks the Lean `ReviewedFq12ConcreteRoleRootFieldBindingCertificate` package and the Rust field-level role-root/full-opening coordinate evidence plus role-root/opening-field negative controls.

AP2-N001 package-gate note: `selected_rust_canonical_byte_and_witness_package_surfaces_present` now requires the raw role-root side package directly: concrete role-root field binding plus selected certificate-lift opening functionality. The older `full_native_opening_role_lift_certificate_present` remains tracked separately, but it no longer stands in for `ReviewedFq12RoleRootFunctionalLiftSource`.

AP2-N001 active-target note: the hidden-source semantics bridge now has a digest-recorded active gate, `active_field_local_digest_frame_witness_byte_component_surfaces_present`, whose inputs mirror the raw-component theorem `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components`. The older source-route/package adapter booleans remain recorded as evidence, but bridge completion no longer treats them as separate active-path blockers.

AP2-N001 boundary-adapter note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now defines `ReviewedFq12HiddenSourceProofSemanticsClosed`, wraps it into `NativeAcceptedSpendSoundnessBoundary.hiddenSourceProofSemanticsClosed`, and proves `reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_digest_frame_witness_byte_components`. Rust source-inspects this as `lean_hidden_source_boundary_adapter_present`, so bridge completion now requires the raw active theorem and the full-boundary adapter without changing the fail-closed `extractor_theorem_instantiated` bit. The theorem's two base premises are also digest-recorded separately as `rust_payload_projection_law_present` and `rust_extraction_certificate_availability_present`; these checks pin the native-boundary digest/slot projection law and the extraction-certificate availability bridge rather than leaving them implicit inside the broader payload-interface flag.

AP2-N001 boundary theorem-window note: `rust_payload_projection_law_present` and `rust_extraction_certificate_availability_present` now share a Rust helper that slices only the raw active boundary theorem `reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_digest_frame_witness_byte_components`, ending before the following certificate-shaped theorem and later five-source records. A regression asserts that the window contains the payload-projection and certificate-availability premises but excludes `reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_digest_frame_witness_byte_component_inputs`, `ReviewedFq12SelectedRustModelFiveSourceClosureInputs`, and `ReviewedFq12SelectedRustFiveSourceClosureEvidence`, so later unrelated markers cannot satisfy these AP2-N001 readiness bits.

AP2-N001 base-premise spoofing note: the payload-projection-law and extraction-certificate-availability source gates now strip Lean comments and string literal contents from the raw active boundary theorem window before marker matching. Their corresponding Lean certificate/availability windows in `Path2NativeBoundary.lean` are also stripped before matching, so base AP2-N001 premise markers cannot be supplied by comments or string blobs in those Lean files.

AP2-N001 shared-frame note: `reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_shared_digest_frame_witness_sources` now destructures `ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources` and case-splits on both frame equalities before calling the lower-level canonical/assembly witness adapter. Rust source-inspects those `rcases`/`cases` lines in `rust_shared_digest_frame_witness_byte_source_contract_present`, so the canonical representative witness source and assembly digest-frame witness source cannot leave the shared-frame equality evidence as unused bookkeeping.

AP2-N001 raw-component gate note: `lean_selected_rust_field_local_digest_frame_witness_byte_component_endpoint_present_v1` now source-inspects the raw selected endpoint for the selected `digestFrame`, both witness byte source arguments, and both `canonical_digest_frame_eq` / `assembly_digest_frame_eq` equalities. This keeps the active raw-component theorem aligned with the shared-frame witness target rather than accepting a broad component list without same-frame evidence.

AP2-N001 raw-component window note: the same helper now slices only the concrete AP2-N001 raw-component window, starting at `ReviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputs` and ending before the older packaged source-route adapter. The window must contain the raw-component theorem `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components` and the named-input theorem `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_component_inputs`, so markers elsewhere in `Path2FullOpeningFieldEvidenceSourceAdapters.lean` no longer satisfy this active raw-component gate.

AP2-N001 raw-component spoofing note: `lean_selected_rust_field_local_digest_frame_witness_byte_component_endpoint_present_v1` now strips Lean comments and string literal contents before checking the active raw-component window. A regression fixture with every required AP2-N001 marker only in a block comment, and another with the markers only in a string literal, fails the gate; the real endpoint still passes.

AP2-N001 component-contract spoofing note: the active component gates now strip Lean comments and string literal contents before checking the role-root concrete package, source-adapter package, assembly field/local/native source package, compatibility field/extraction source package, selected source-route byte-source contract, source-route parser/frame certificate contract, and shared digest-frame witness contract. The hidden-source bridge audit still passes after stripping, confirming the AP2-N001 markers used by `active_field_local_digest_frame_witness_byte_component_surfaces_present` are present as executable Lean declarations rather than comments.

AP2-N001 selected-source-route evidence note: `Descender/Path2FullOpeningFieldEvidenceByteBridge.lean` now exposes `SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceReport` and `SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate`. The report records schema/digest verification, field and route parser pinning, source-route availability, field/route byte certificates, semantic binding facts, aggregate-bridge rejection, and negative controls. The final five-source constructor in `Path2HiddenSourceSemanticsBoundary.lean` now has `ofEvidenceAndSourceRouteCertificate`, which consumes that source-route certificate and calls `.toByteSource`; the Rust `extractor_theorem_instantiated` gate rejects the old raw-source-route constructor shape and requires `selectedRustSourceRouteByteSourceCertificate`.

AP2-N001 selected-source-route Rust mirror note: `tools/parameter-search/src/production_reference.rs` now exposes `Path2ReviewedFq12SelectedSourceRouteByteSourceEvidenceV1` with schema, frame, digest, all-facts, report-verification, and verifier functions mirroring the Lean selected source-route evidence report. The active selected-source-route contract gate now requires this Rust report surface and verifies a constructed report before accepting the source-route byte-source marker; the focused regression rejects tampered route-availability, schema-match, and digest-verified bits. This is evidence-surface hardening only: `extractor_theorem_instantiated` still remains false until the Lean/Rust theorem consumes the report into the certificate.

AP2-N001 shared-witness evidence note: `Descender/Path2FullOpeningFieldEvidenceSourceAdapters.lean` now exposes `ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceReport` and `ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate`. The report records schema/digest verification, canonical and digest-frame witness parser pinning, shared digest-frame equality, both byte certificates, representative digest-frame source pinning, verifier bindings, and negative controls. The final five-source constructor in `Path2HiddenSourceSemanticsBoundary.lean` now has `ofEvidenceAndSourceRouteAndWitnessCertificates`, which consumes both the source-route certificate and the shared-witness certificate; the Rust `extractor_theorem_instantiated` gate rejects the previous raw-witness-bundle constructor shape and requires `selectedRustSharedFrameWitnessSourcesCertificate`.

AP2-N001 shared-witness Rust mirror note: `tools/parameter-search/src/production_reference.rs` now exposes `Path2ReviewedFq12SharedDigestFrameWitnessEvidenceV1` with schema, frame, digest, all-facts, report-verification, and verifier functions mirroring the Lean shared digest-frame witness evidence report. The shared witness byte-source contract gate now requires this Rust report surface and verifies a constructed report before accepting the shared witness source marker; the focused regression rejects tampered shared-frame pinning, schema-match, and digest-verified bits. This keeps canonical/digest-frame witness evidence digest-bound without flipping `extractor_theorem_instantiated`.

AP2-N001 field/local evidence note: `Descender/Path2FullOpeningFieldEvidenceSourceAdapters.lean` now exposes `ReviewedFq12SelectedRustFieldLocalEvidenceReport` and `ReviewedFq12SelectedRustFieldLocalEvidenceCertificate`. The report records schema/digest verification, assembly availability, assembly field binding, local predicates, native validity, compatibility availability, compatibility field binding, extraction rules, and negative controls. The final five-source constructor in `Path2HiddenSourceSemanticsBoundary.lean` now has `ofEvidenceAndSourceRouteFieldLocalAndWitnessCertificates`, which consumes the source-route, field/local, and shared-witness certificates; the Rust `extractor_theorem_instantiated` gate rejects the previous raw assembly/compatibility constructor shape and requires `selectedRustFieldLocalSourceCertificate`.

AP2-N001 field/local Rust mirror note: `tools/parameter-search/src/production_reference.rs` now exposes `Path2ReviewedFq12FieldLocalEvidenceV1` with schema, frame, digest, all-facts, report-verification, and verifier functions mirroring the Lean field/local evidence report. The assembly/compatibility local-model gate now requires this Rust report surface and verifies a constructed report before accepting the local-model source marker; the focused regression rejects tampered assembly field binding, schema-match, and digest-verified bits. This keeps assembly and compatibility local evidence digest-bound without flipping `extractor_theorem_instantiated`.

AP2-N001 role-root/lift evidence note: `Descender/Path2FullOpeningFieldEvidenceSourceAdapters.lean` now exposes `ReviewedFq12SelectedRustRoleRootLiftEvidenceReport` and `ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate`. The report records schema/digest verification, concrete role-root field binding, selected lift functionality, role-root bridge pinning, and negative controls. The final five-source constructor in `Path2HiddenSourceSemanticsBoundary.lean` now has `ofEvidenceAndRoleRootSourceRouteFieldLocalAndWitnessCertificates`, which consumes the role-root/lift, source-route, field/local, and shared-witness certificates; the Rust `extractor_theorem_instantiated` gate rejects the previous raw role-root/lift constructor shape and requires `selectedRustRoleRootLiftSourceCertificate`.

AP2-N001 role-root/lift Rust mirror note: `tools/parameter-search/src/production_reference.rs` now exposes `Path2ReviewedFq12RoleRootLiftEvidenceV1` with schema, frame, digest, all-facts, report-verification, and verifier functions mirroring the Lean role-root/lift evidence report. The role-root/lift source gate now requires this Rust report surface and verifies a constructed report before accepting the combined role-root/lift source marker; the focused regression rejects tampered concrete-role-root binding, schema-match, and digest-verified bits. This hardens the role-root evidence surface without flipping `extractor_theorem_instantiated`.

AP2-N001 boundary-base evidence note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now exposes `ReviewedFq12SelectedRustBoundaryBaseEvidenceReport` and `ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate`. The report records schema/digest verification, payload projection law pinning, extraction-certificate availability pinning, native-boundary payload binding, and negative controls. The final five-source constructor now has `ofEvidenceAndBoundaryBaseRoleRootSourceRouteFieldLocalAndWitnessCertificates`, which consumes boundary-base, role-root/lift, source-route, field/local, and shared-witness certificates; the Rust `extractor_theorem_instantiated` gate rejects the previous raw payload-projection/extraction-availability constructor shape and requires `selectedRustBoundaryBaseCertificate`.

AP2-N001 boundary-base Rust mirror note: `tools/parameter-search/src/production_reference.rs` now exposes `Path2ReviewedFq12BoundaryBaseEvidenceV1` with schema, frame, digest, all-facts, report-verification, and verifier functions mirroring the Lean boundary-base evidence report. The five-source evidence source gate now requires this Rust report surface and verifies a constructed report before accepting the aggregate five-source evidence surface; the focused regression rejects tampered payload-projection, schema-match, and digest-verified bits. This is evidence-surface hardening only and does not flip `extractor_theorem_instantiated`.

AP2-N001 aggregate evidence-binding note: `ReviewedFq12SelectedRustFiveSourceClosureEvidence` and its Rust mirror now carry the verified report digests for boundary-base, role-root/lift, selected source-route, field/local, and shared digest-frame witness evidence. The aggregate Rust constructor verifies each sub-report before populating these digest fields, the aggregate frame hashes the digest fields and verification bits, and the focused regression rejects stale sub-report digests and cleared sub-report verification bits. This keeps the five public source-evidence identities explicit while `extractor_theorem_instantiated` remains fail-closed.

AP2-N001 aggregate/certificate digest-match note: the five per-source evidence certificates now carry certificate-level evidence digests, and `ReviewedFq12SelectedRustFiveSourceClosureEvidence.SubEvidenceDigestsMatch` requires the aggregate report's five digest fields to equal the boundary-base, role-root/lift, source-route, field/local, and shared-witness certificate digests. The final all-certificate constructor `ofEvidenceAndBoundaryBaseRoleRootSourceRouteFieldLocalAndWitnessCertificates` now requires this match proof, and the Rust final-theorem gate requires `selectedRustFiveSourceSubEvidenceDigestsMatchProof` in the accepted theorem shape. This prevents the final AP2-N001 constructor from consuming one aggregate report while silently sourcing different per-boundary certificates.

AP2-N001 aggregate certificate digest note: `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate` now carries an aggregate `evidenceDigest` for the five-source closure evidence report, and every final/staged constructor takes that digest immediately after the aggregate report. The Rust final-theorem gate now requires `selectedRustFiveSourceEvidenceDigest` in addition to the five sub-evidence digests and rejects `evidenceDigest := ByteArray.empty` record literals. This keeps AP2-N001 fail-closed unless the future hidden-source instantiation binds both the aggregate readiness report and each sub-certificate to public digest evidence.

AP2-N001 route-gate digest-match note: `lean_active_five_source_boundary_route_present` now source-inspects the aggregate sub-report digest fields, per-certificate `evidenceDigest` projections, `SubEvidenceDigestsMatch`, and `subEvidenceDigestsMatchProof`. The active route readiness bit therefore depends on the same aggregate/certificate digest-match shape that the future final theorem must consume.

AP2-N001 per-source certificate-constructor note: each per-source evidence certificate now has a named constructor: `ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate.ofReportAndSources`, `ReviewedFq12SelectedRustRoleRootLiftEvidenceCertificate.ofReportAndSource`, `SelectedRustReviewedFq12FullOpeningFieldSourceRouteEvidenceCertificate.ofReportAndSource`, `ReviewedFq12SelectedRustFieldLocalEvidenceCertificate.ofReportAndSources`, and `ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate.ofReportAndSources`. The Rust final-theorem gate now requires the accepted future theorem shape to build the five certificates through those constructors with explicit report, digest, report-verification proof, facts-all-true proof, and source arguments before calling the final all-certificate constructor.

AP2-N001 named-certificate note: `Descender/Path2FullOpeningFieldEvidenceSourceAdapters.lean` now packages the full raw AP2-N001 source checklist as `ReviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputs`, and `Descender/Path2HiddenSourceSemanticsBoundary.lean` exposes `reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_digest_frame_witness_byte_component_inputs`. The Rust boundary source gate checks this certificate-shaped theorem in addition to the older decomposed theorem, making the remaining `extractor_theorem_instantiated` obligation a single active-path certificate shape.

AP2-N001 source-to-certificate note: the source adapter now proves the existing five-source implementation package maps into the named AP2-N001 raw-component certificate. `reviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputsOfSources` extracts role-root/lift facts, selected source-route bytes, concrete assembly availability/binding/local/native facts, concrete compatibility availability/binding/extraction rules, and both shared-frame witness equalities into `ReviewedFq12SelectedRustFieldLocalDigestFrameWitnessByteComponentInputs`; `ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs.toByteComponentInputs` and the two `_via_component_inputs` theorems route the implementation package through that certificate. Rust source-inspects these names and field projections, so the active audit now ties the five-source wrapper to the exact raw certificate that must eventually be instantiated from Rust/model byte evidence.

AP2-N001 boundary five-source note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now exposes `reviewed_fq12_hidden_source_proof_semantics_closed_from_active_field_local_shared_digest_frame_witness_sources`, the production-facing boundary theorem that consumes the five selected Rust/model source records, constructs the named raw-component certificate, and then closes `ReviewedFq12HiddenSourceProofSemanticsClosed`. The Rust boundary source gate checks this theorem and the certificate constructor, so the remaining fail-closed bit is specifically the real instantiation of those five active source records.

AP2-N001 audit-field note: the hidden-source semantics bridge audit now digest-records `lean_active_five_source_boundary_route_present` separately from the broader `lean_hidden_source_boundary_adapter_present`. The field source-inspects the five-source boundary theorem window and is required for `semantics_bridge_complete`, with a fail-closed negative control in the Rust audit regression.

AP2-N001 five-source route-window note: `lean_active_five_source_boundary_route_present` now strips Lean comments and string literal contents, then inspects only the active five-source / accepted-boundary window from `ReviewedFq12SelectedRustModelFiveSourceClosureInputs` through `with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_five_source_inputs`. The full-signature handoff is checked by its own stripped window starting at `NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure`. A regression asserts the five-source route window excludes the full-signature adapter and that the full-signature handoff window excludes the model-input structure, preventing one broad end-of-file slice from satisfying both readiness bits.

AP2-N001 extractor-gate note: `extractor_theorem_instantiated` is no longer a naked hardcoded false. It is now computed by `path2_reviewed_fq12_active_five_source_extractor_theorem_instantiated_v1`, which source-inspects the active boundary module for the future theorem `reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_instantiation` and requires it to consume `ReviewedFq12SelectedRustModelFiveSourceClosureInputs` through `reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_inputs`. The helper currently returns false, so the bridge remains fail-closed, but the final hidden-source semantics blocker now names the exact theorem shape that must be added.

AP2-N001 model-input note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now exposes `ReviewedFq12SelectedRustModelFiveSourceClosureInputs`, a single record containing the payload projection law, extraction certificate availability, role-root/lift source, selected source-route bytes, concrete assembly source, concrete compatibility source, and shared-frame witness sources. The theorem `reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_inputs` closes hidden-source semantics from that record by the five-source boundary route. This is a target shape for the future concrete Rust/model instantiation theorem and does not flip `extractor_theorem_instantiated`.

AP2-N001 accepted-boundary note: `with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_five_source_inputs` now maps the same single model-input record into the `hiddenSourceProofSemanticsClosed` field of `NativeAcceptedSpendSoundnessBoundary.withReviewedFq12HiddenSourceClosure B P`. The active five-source source gate checks this theorem, so downstream full-signature boundary code can consume the same record rather than restating hidden-source semantics closure.

AP2-N001 full-boundary handoff note: `NativeFullSignatureSecurityBoundary.withReviewedFq12HiddenSourceClosure` now lifts reviewed-Fq12 hidden-source closure into the active full-signature boundary while preserving the remaining keygen, signing, completeness, verifier-relation, full-game, checked-core, lifecycle, no-export, hash-domain, query-budget, and multi-target conjuncts. The theorem `full_signature_with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_five_source_inputs` exposes the same selected Rust/model five-source record at the full-signature boundary. This is an adapter only; it does not flip `extractor_theorem_instantiated`.

AP2-N001 full-boundary audit note: the hidden-source semantics bridge audit now digest-records `lean_full_signature_hidden_source_handoff_present` separately. The bit source-inspects the full-signature adapter window, is required by `selected_rust_model_five_source_inputs_ready` and `semantics_bridge_complete`, and has a fail-closed regression mutation. This prevents the active proof burndown from treating accepted-spend hidden-source closure as automatically threaded into the full-signature theorem.

AP2-N001 model-readiness note: the hidden-source semantics audit now digest-records `selected_rust_model_five_source_inputs_ready`, the non-closing conjunction for the exact `ReviewedFq12SelectedRustModelFiveSourceClosureInputs` ingredients: payload projection law, extraction-certificate availability, active five-source Lean route, active field/local digest-frame component surfaces, role-root/lift source, assembly field/local/native source, compatibility field/extraction source, and shared digest-frame witness source. This bit being true does not complete deployment; `extractor_theorem_instantiated` still requires the exact final Lean instantiation theorem.

AP2-N001 native-preflight propagation note: `path2_native_deployment_preflight_v1` now carries `hidden_source_semantics_bridge_complete` from the active reviewed-Fq12 semantics bridge audit, includes it in `proof_side_public_boundary_preflight_complete`, emits `PATH2_NATIVE_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_SEMANTICS` while the extractor theorem remains absent, and makes the active verifier reject with `reject_active_native_hidden_source_semantics_open` before checked-core/full-execution/productization blockers. Signer/verifier evidence now reports AP2-N001 as the smallest remaining proof-side gate instead of hiding it behind AP2-N002.

AP2-N001 extractor-gate hardening note: `path2_reviewed_fq12_active_five_source_extractor_theorem_instantiated_v1` now rejects an abstract theorem that merely takes `ReviewedFq12SelectedRustModelFiveSourceClosureInputs` as a parameter. The future theorem must locally construct `selectedRustModelFiveSourceInputs` with named assignments for payload projection, extraction availability, role-root/lift, selected source route, assembly source, compatibility source, and shared witness sources before routing through `reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_inputs`.

AP2-N001 extractor-gate header note: the same gate now also rejects a theorem that locally constructs `selectedRustModelFiveSourceInputs` from separately quantified model-source parameters, or from a renamed whole-record parameter. The future theorem header cannot carry the whole input record, the concrete source types, or the seven model-input ingredients as assumptions; those facts must be supplied by concrete active Rust/model instantiation objects inside the theorem body.

AP2-N001 extractor-provenance note: the same gate now rejects placeholder-fed local record construction. The accepted future theorem shape must build `selectedRustModelFiveSourceInputs` from explicit selected-Rust source constructors (`selectedRustPayloadProjectionLaw`, `selectedRustExtractionCertificateAvailability`, `selectedRustRoleRootLiftSource`, `selectedRustSourceRouteByteSource`, `selectedRustAssemblySource`, `selectedRustCompatibilitySource`, and `selectedRustSharedFrameWitnessSources`) rather than arbitrary free identifiers in scope. The gate also rejects those constructor names if the active boundary module introduces them as `axiom`, `constant`, `opaque`, or module-level `variable` stubs.

AP2-N001 certificate-constructor note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now exposes `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSources`, the named constructor for the final selected Rust/model five-source certificate. The `extractor_theorem_instantiated` source gate no longer accepts a raw record literal for `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate`; it requires the final theorem to call this constructor with the verified five-source evidence report, the aggregate five-source evidence digest, the all-true proof, and each selected source package before deriving `toClosureInputs`. The same gate now strips comments/strings and rejects `sorry` or `admit` tokens in the active boundary module, so placeholder Lean proofs cannot flip the hidden-source semantics gate.

AP2-N001 five-source certificate note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now exposes `ReviewedFq12SelectedRustFiveSourceClosureEvidence`, `ReportVerified`, `FactsAllTrue`, and `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate`. The certificate carries a structured public evidence report plus proofs tied to that report, rather than arbitrary `Prop` fields, alongside the seven concrete model/source fields needed by `ReviewedFq12SelectedRustModelFiveSourceClosureInputs`; `ReportVerified` requires schema, explicit bridge-audit digest bytes, bridge-audit digest binding, report-digest verification, and bridge-validation bits, while `FactsAllTrue` covers the non-closing readiness facts. The Rust extractor gate now requires the future instantiation theorem to locally construct this certificate and call `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.toClosureInputs`; it rejects an assumed certificate, assumed report/proof bits, direct local input-record construction, inline all-true report construction, vacuous theorem-local verification, and normalized module-level stubs that make the report/facts evidence trivial. This does not flip `extractor_theorem_instantiated`, but it makes the remaining AP2-N001 closing theorem proof-bearing and report-shaped.

AP2-N001 proof-shell hardening note: the hidden-source extractor gate now rejects `abbrev` proof shells, in addition to `def`/`theorem` shells, for selected source proof names ending in `Proof` when the body is `True.intro`, `by trivial`, or `by exact True.intro`. A regression prepends `abbrev selectedRustFiveSourceEvidenceFactsAllTrueProof := True.intro` to the otherwise accepted theorem shape and confirms the module-level gate rejects it.

AP2-N001 hidden-source byte-domain evidence note: the five-source closure evidence report now carries the verified `Path2NativeHashByteDomainEvidenceV1` digest and explicit Rust facts that the reviewed-Fq12 hidden-source statement frame uses the active `hidden_source_frame_domain`. The Rust constructor verifies the shared hash-domain evidence, checks the length-delimited `domain` field at the start of `reviewed_fq12_hidden_source_statement_frame`, and checks that `reviewed_fq12_hidden_source_statement_digest` is SHA-256 over that same frame. `Descender/Path2HiddenSourceSemanticsBoundary.lean` mirrors these facts in `ReviewedFq12SelectedRustFiveSourceClosureEvidence.ReportVerified` / `FactsAllTrue`. This is active-path byte-binding hardening only; `extractor_theorem_instantiated` remains fail-closed until the selected Rust/model closure theorem consumes the evidence report and source certificates.

AP2-N001 hidden-source proof-domain evidence note: `Path2NativeHashByteDomainEvidenceV1` now carries the companion `hidden_source_proof_domain` for `reviewed_fq12_hidden_source_proof_transcript_frame`, in addition to the statement-frame domain. The five-source closure report records Rust facts that this proof transcript domain is pinned to `REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_DOMAIN`, that the proof transcript frame starts with the corresponding length-delimited `domain` field, and that `reviewed_fq12_hidden_source_proof_transcript_digest` is SHA-256 over that same proof transcript frame. The Lean five-source report mirrors those facts, keeping statement-domain and proof-transcript-domain binding separate while preserving the fail-closed extractor theorem gate.

AP2-N001 projection-carrier byte-domain evidence note: `reviewed_fq12_state_assisted_projection_carrier_binding` now hashes a named `reviewed_fq12_state_assisted_projection_carrier_binding_frame`, so the carrier check bytes and transcript-binding bytes are traceable to a single domain-prefixed frame. The five-source closure report records Rust facts that this frame uses the `projection_carrier_domain` from `Path2NativeHashByteDomainEvidenceV1` and that the SHA-256 digest splits into the carrier's 3-byte check plus 29-byte transcript binding. Lean mirrors these carrier facts in the same five-source report; the extractor theorem remains fail-closed until it consumes the report and source certificates.

AP2-N001 opening/pad byte-domain evidence note: `Path2NativeHashByteDomainEvidenceV1` now also carries the account-witness opening, pad-seed opening, and public pad-opening-view domains. The account-witness opening digest and pad-seed opening digest now expose named frame helpers, and the five-source closure report records Rust facts that those helpers plus `public_pad_opening_statement_frame` start with their respective length-delimited domain fields and hash to the public digest values already consumed by the active projection-carrier path. Lean mirrors these opening/pad facts in the five-source report. This still does not flip `extractor_theorem_instantiated`; it makes the remaining theorem consume explicit byte-domain evidence instead of implicit serializer assumptions.

AP2-N001 direct/batch separation evidence note: `Path2ReviewedFq12FiveSourceClosureEvidenceV1` now consumes the active `Path2DirectQueryBudgetV1` direct-mode budget. The report records that the schema and `direct_single_slot` mode match, `q_batch` is absent from active query symbols, `q_batch` is explicitly inactive, optional aggregation terms remain `q_batch`-only, and the Lean direct-budget inactive theorem surface is present. Lean mirrors these facts in `ReviewedFq12SelectedRustFiveSourceClosureEvidence.FactsAllTrue`, and the Rust regression clears the `q_batch` inactive bit as a fail-closed negative control. The final selected Rust/model theorem remains the closing AP2-N001 blocker.

AP2-N001 certificate-route note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now exposes certificate-level hidden-source, accepted-spend, and full-signature adapters from `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate`. The future final theorem source gate now requires `reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_certificate P selectedRustModelFiveSourceCertificate` after constructing the all-certificate package, rather than treating the derived `ReviewedFq12SelectedRustModelFiveSourceClosureInputs` record as the primary closing route. This keeps the aggregate evidence report, sub-evidence digest match, and per-source certificates on the final theorem surface while preserving the fail-closed `extractor_theorem_instantiated` bit.

AP2-N001 concrete-definition blocker note: the hidden-source semantics bridge audit now separately records `selected_rust_model_concrete_definitions_present`. It is false on the active tree: the certificate route, source surfaces, fail-closed selected evidence-report definitions, and selected evidence-digest constants are present, but the concrete Lean `def`/`theorem` declarations for per-source proofs, source records, and sub-evidence digest match proof have not been added. The bit is included in the bridge-complete conjunction and digest frame, while `selected_rust_model_five_source_inputs_ready` remains the non-closing readiness bit for audited surfaces.

AP2-N001 concrete-definition checklist note: Rust now exposes `Path2SelectedRustModelConcreteDefinitionChecklistV1`, a digest-bound checklist for the concrete-definition categories needed by the final selected Rust/model theorem: evidence reports, evidence digests, report/facts proofs, the native-boundary predicate-model certificate, source records, and the sub-evidence digest-match proof. The checklist is carried by digest/verified bit in the hidden-source semantics bridge audit. This keeps the next proof task concrete without treating the absence of definitions as a failure of the already-audited route/evidence surfaces.

AP2-N001 checklist facade note: the curated `tools/production-reference` facade now exports the selected Rust/model concrete-definition checklist schema, type, constructor, frame, digest, and verifier. `path2_native_production_facade_exports_source_present_v1` pins those exports, and the facade crate test checks the current fail-closed state plus canonical checklist digest verification. This makes the concrete-definition blocker visible to deployability tooling without exposing selected-material helpers or changing the final theorem status.

AP2-N001 selected-report definition note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now defines the six selected Rust/model evidence report values named by the concrete-definition checklist. These report definitions are intentionally fail-closed, with verification and component bits false, so they do not supply `ReportVerified`, `FactsAllTrue`, selected source records, or the sub-evidence digest-match proof. The checklist records the report-definition category present while keeping AP2-N001 bridge completion false.

AP2-N001 report-definition checklist note: the concrete-definition checklist now requires each selected report declaration to mention its expected Lean report type. A name-only report fixture remains fail-closed, so the report category cannot be satisfied by aliases that do not expose the selected Rust/model evidence-report structure.

AP2-N001 selected-digest definition note: `tools/parameter-search/src/bin/path2_selected_evidence_digest_manifest.rs` now emits a source-controlled manifest for the six public selected evidence report digests, their all-facts predicates, and the closing status bits. `Path2HiddenSourceSemanticsBoundary.lean` mirrors those six digest values as `selectedRust*EvidenceDigest` byte-array definitions, and the hidden-source semantics bridge regression parses those Lean byte arrays and compares them to the Rust-computed aggregate/sub-evidence digests. The checklist now records report and digest categories present while keeping proof objects, source records, and the sub-evidence digest-match proof missing.

AP2-N001 proof-definition checklist note: the concrete-definition checklist no longer treats selected proof-object names alone as sufficient. The proof category now requires each `selectedRust*Proof` declaration window to mention the corresponding selected report and the exact `ReportVerified` or `FactsAllTrue` predicate, with a negative fixture showing untyped proof-name declarations fail the category. This keeps the remaining proof-object blocker tied to the evidence-report predicates the final certificate must consume.

AP2-N001 source-record checklist note: the concrete-definition checklist now requires selected source-record declarations to mention their expected source type or predicate: payload projection, extraction availability, role-root/lift source, source-route byte source, field/local assembly source, compatibility source, and shared-frame witness source. A name-only source-record fixture remains fail-closed, so the source-record category cannot be satisfied by aliases that do not expose the active source contract shape.

AP2-N001 sub-evidence-match checklist note: the concrete-definition checklist now requires `selectedRustFiveSourceSubEvidenceDigestsMatchProof` to mention `SubEvidenceDigestsMatch` and all five per-source certificate bindings. A name-only theorem fixture remains fail-closed, so the final digest-match category cannot be satisfied without exposing the aggregate-to-certificate binding shape the AP2-N001 constructor consumes.

AP2-N001 extractor-stub note: the Rust source gate now strips Lean comments and string literal contents before scanning multiline `axiom`, `constant`, and `opaque` declaration blocks for the selected Rust/model source names and evidence predicates, not just one-line declarations. It also treats vacuous `schemaMatches := true` report construction as a forbidden report stub marker. This closes a gate-bypass shape where the final extractor theorem could appear to consume concrete source names while those names were still introduced as multiline assumptions, and prevents comment/string-only stub text from poisoning the module scan.

AP2-N001 extractor-comment note: the future extractor theorem source gate now strips Lean line comments, block comments, and string literal contents before checking for the required certificate-construction markers. Regressions pin that comment-only or string-only occurrences of `selectedRustModelFiveSourceCertificate`, the evidence-report fields, source bindings, `toClosureInputs`, and the five-source closure theorem do not satisfy `extractor_theorem_instantiated`.

AP2-N001 extractor-aggregate-package note: the future extractor theorem source gate now rejects theorem headers that take newly invented aggregate five-source implementation/package/certificate parameters (for example `SelectedRustModelFiveSourceImplementation`) in addition to the already rejected closure input/certificate records and individual source parameters. A negative-control theorem shows such a package could otherwise feed the certificate fields while hiding the selected Rust/model source construction behind a single assumed object.

AP2-N001 five-source evidence note: Rust now exposes `Path2ReviewedFq12FiveSourceClosureEvidenceV1`, a public digest-bound evidence report over the selected model five-source readiness bits: payload projection, extraction-certificate availability, role-root/lift source, selected source-route byte source, assembly field/local/native source, compatibility field/extraction source, shared-frame witness source, active field/local byte-component surfaces, and the Lean five-source/full-signature handoff routes. The Rust report now carries the Lean-mirrored `ReportVerified` status bits: `schema_matches`, `bridge_audit_digest_bound`, `bridge_audit_validated`, and `report_digest_verified`; the supplied-report verifier requires the constructed report plus those status bits before returning the evidence digest. All readiness facts are exposed through a separate `all_facts_true` predicate; the `ReportVerified` fields, `extractor_theorem_instantiated`, and `semantics_bridge_complete` remain status fields outside that readiness predicate. This makes the report usable as future `ReportVerified` / `FactsAllTrue` input without creating a circular fail-closed dependency. The future extractor theorem source gate and the curated production facade now require this evidence surface.

AP2-N001 native-boundary semantics package note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now names the active target package as `ReviewedFq12SelectedRustNativeBoundarySemanticsPackage`, containing a `Path2NativeBoundary.Predicates` value and the selected source-record package for exactly that value. The module threads that package into accepted-spend closure, full-signature closure, and the full active-path forgery-bound theorem via `full_signature_active_path_forgery_bound_from_selected_rust_native_boundary_semantics_package`. Rust records separate manifest bits for the package target, hidden-source handoff, and bound handoff. This does not close AP2-N001: the manifest still reports no concrete native-boundary predicate model and no raw selected source records/certificate constructors.

AP2-N001 predicate-model source-target note: the active hidden-source boundary now has a fail-closed source-shaped target, `ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage`, which requires `ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource` plus selected source records over exactly `predicateModel.toPredicates`. This records that the full-spend concrete predicates are not the native-boundary predicate interpretation and prevents the final source records from floating over an arbitrary `Predicates` value. The predicate-model source also requires `payloadAcceptanceNotVacuous` and `boundaryBreaksNotUniversal`, ruling out the two most direct vacuous model closures. Rust manifest bits track this source target and its bound handoff separately from the still-false concrete predicate model import.

AP2-N001 evidence-frame note: `path2_reviewed_fq12_five_source_closure_evidence_v1_digest` now hashes an explicit length-delimited evidence frame instead of `serde_json` output. The frame binds schema, all Lean-mirrored `ReportVerified` bits, the bridge audit digest, all readiness facts, closing status bits, and cleanup policy in fixed order, is exported through the production facade, has source-gate checks for each frame label, and has a regression checking the digest equals `Sha256(frame)`.

AP2-N001 report/facts split note: Rust now exposes `path2_reviewed_fq12_five_source_closure_evidence_report_verified_v1` separately from `path2_reviewed_fq12_five_source_closure_evidence_all_facts_true`. Native preflight uses the report verifier for `five_source_closure_evidence_verified` and carries `five_source_closure_evidence_facts_all_true` as its own bit, matching the Lean `ReportVerified` / `FactsAllTrue` split. The stronger `verify_path2_reviewed_fq12_five_source_closure_evidence_v1` remains the conjunction for closure evidence.

AP2-N001 native-evidence propagation note: `Path2NativeDeploymentPreflightV1` now carries the five-source closure evidence digest plus verified/all-facts-true booleans, and `Path2NativeVerifierPipelineVerdictV1` / `Path2NativeSignerVerifierPipelineEvidenceV1` mirror those fields from preflight. This makes the proof-bearing AP2-N001 evidence visible in the active native verifier/signer trail while preserving the fail-closed `hidden_source_semantics_bridge_complete = false` production rejection until the extractor theorem is actually instantiated.

AP2-N001 component-window note: the Rust hidden-source semantics audit now hardens `rust_assembly_field_local_native_components_present` and `rust_compatibility_field_extraction_components_present` against broad-marker satisfaction. The assembly gate separately slices the native assembly constructor/verifier, the exact Rust assembly/full-opening field-binding function, the Lean `ReviewedFq12AssemblyAvailabilityLocalSource` structure, and its concrete-field-source forgetter. The compatibility gate separately slices the compatibility constructor/verifier, same-representative binding function, the Lean `ReviewedFq12CompatibilityAvailabilityExtractionRulesSource` structure, and its concrete-field-source forgetter. This keeps the AP2-N001 raw active theorem tied to the actual local-model and extraction-rule sources while the bridge remains fail-closed on `extractor_theorem_instantiated = false`.

AP2-R001 QROM-horizon note: the Rust preflight no longer treats `minimum_deployment_margin_bits > 1` as sufficient QROM/multi-target completion. `path2_auxiliary_fq12_qrom_horizon_complete_v1` now requires the selected auxiliary-Fq12 challenge shape exactly: 23 algebraic bytes, 4 hardening bytes, last algebraic-byte mask `0x3f`, 182 algebraic bits after masking, 214 total effective challenge bits, selected 32-bit aggregate horizon within the 53-bit deployment margin, and a 54-bit negative control outside that margin. `Descender/Path2HashBudget.lean` now mirrors the challenge-width facts directly in `Path2DirectQueryBudgetV1`.

AP2-R001 action-consumption note: `Descender/Path2HashBudget.lean` now exposes `AuxiliaryFq12ChallengeShapeComplete`, and `Descender/Path2HashDomainAndMultiTarget.lean` threads that fact through `ActiveHashDomainAndMultiTargetCertificate`. Rust regression `path2_checked_core_authorization_consumes_auxiliary_challenge_bytes_in_fq12_action` verifies the active checked-core authorization path consumes bytes outside the compact 130-bit block/sign prefix: flipping byte 17 or the first hardening byte leaves `decode_challenge` unchanged, changes `reviewed_fq12_challenge_action_vector`, changes the nonzero-witness expected Fq12 response syndrome, and a hardening-byte signature rebound is rejected by checked-core challenge binding. This addresses the route-specific risk that the widened bytes could have been transcript-only rather than part of the Fq12 action.

AP2-R001 action-collision note: the Rust direct budget/review object now serializes the Fq12 action-collision floor (`163` bits) plus the selected deployment residuals (`131` quantum bits and `130` classical bits after the union-safe query bounds), and `path2_auxiliary_fq12_qrom_horizon_complete_v1` requires those residuals to meet the 128-bit target. Lean mirrors this with `Fq12ActionCollisionBudgetPriced` in `Descender/Path2HashBudget.lean` and threads it through `ActiveHashDomainAndMultiTargetCertificate` in `Descender/Path2HashDomainAndMultiTarget.lean`.

AP2-R001 active-boundary source-gate note: `path2_auxiliary_fq12_qrom_horizon_complete_v1` now also requires `path2_active_native_hash_budget_boundary_source_present_v1`, which source-inspects the Lean direct-budget constants, auxiliary-Fq12 challenge shape, Fq12 action-collision budget, selected 32-bit aggregate horizon, 54-bit negative-control theorem, and `activeNativeHashBudgetBoundary` certificate window. This ties the Rust QROM/deployment-horizon preflight to the actual Lean native hash boundary instead of only checking Rust-side constants.

AP2-R001 source-gate spoofing note: `path2_active_native_hash_budget_boundary_source_present_v1` now strips Lean comments and string literal contents before scanning `Path2HashBudget.lean` and `Path2HashDomainAndMultiTarget.lean`. The gate uses declaration/code boundaries rather than doc-comment boundaries and the hash/QROM review regression rejects comment-only or string-only markers for the direct budget, Fq12 action budget, byte-domain instantiation, active multi-target certificate, and 54-bit negative-control theorem. This keeps the widened 32-bit horizon route from being marked present by documentation text while the real declarations are absent.

AP2-R001 exact active-declaration note: the active hash/QROM boundary source gate now requires exact Lean declaration names for the direct budget, auxiliary-Fq12 challenge shape, Fq12 action-collision budget, selected multi-target horizon, 54-bit negative control, byte-domain evidence route, active native hash boundary, and full-signature active-hash handoff. Regressions now reject suffixed active declarations such as `AuxiliaryFq12ChallengeShapeCompleteSpoof`, `activeLaneAccountHorizonV1Spoof`, and `full_signature_with_active_hash_boundary_propsSpoof`. This prevents the single active path from being satisfied by shadow or renamed QROM-boundary facts.

AP2-R001/AP2-R002 byte-domain note: `Descender/Path2HashDomainAndMultiTarget.lean` now instantiates `ActiveHashByteDomainInstantiation` as `activeHashByteDomainInstantiationV1`, giving theorem-side representatives for the active checked-core, hidden-source, projection-carrier, and account-lifecycle byte domains and proving `active_checkedCoreFrameByteDomainSeparated` / `active_hiddenSourceFrameByteDomainSeparated`. The byte-domain bridge is now wrapped by `ActiveHashByteDomainEvidenceReport` and `ActiveHashByteDomainEvidenceCertificate`, with a named evidence digest and constructor before `ActiveHashDomainAndMultiTargetCertificate` consumes `.toInstantiation`. The Rust QROM horizon source gate requires the evidence report/certificate/digest route and rejects an empty digest literal, making byte-domain separation part of a proof-bearing active Lean certificate instead of relying only on Rust registry uniqueness tests or a raw Nat-domain record.

AP2-R001/AP2-R002 Rust byte-domain evidence note: Rust now exports `Path2NativeHashByteDomainEvidenceV1` with schema `PATH2_NATIVE_HASH_BYTE_DOMAIN_EVIDENCE_SCHEMA`, pinned checked-core, hidden-source, projection-carrier, and lifecycle domain strings, registry membership, disjointness, duplicate-domain negative control, frame, digest, and verifier helpers through the production facade. `path2_auxiliary_fq12_qrom_horizon_complete_v1` now requires both the Lean active hash-boundary source gate and the Rust byte-domain evidence source/digest verifier before the QROM horizon is considered complete. This binds the active Lean byte-domain certificate to concrete Rust registry evidence; the remaining theorem work is tying the actual checked-core and hidden-source byte-frame constructors to those domains.

AP2-R001 full-boundary handoff note: `Descender/Path2HashDomainAndMultiTarget.lean` now exposes `NativeFullSignatureSecurityBoundary.withActiveHashBoundary`, which installs `activeNativeHashBudgetBoundary` into the full active Path 2 signature boundary while preserving every non-hash proof obligation. The theorem `full_signature_with_active_hash_boundary_props` exposes checked-core domain separation, hidden-source domain separation, query-budget pricing, and multi-target pricing after the handoff, and the Rust QROM horizon source gate requires this marker. This prevents the widened QROM/multi-target proof from remaining only a standalone hash-boundary fact.

AP2-R001/AP2-N003 full-reduction certificate note: `Descender/Path2NativeAccountDesign.lean` now exposes `SelectedRustFullSignatureReductionEvidenceReport` and `SelectedRustFullSignatureReductionCertificate`. The certificate route requires a report digest, production signature-probability pinning, equality from the exported signature-forgery probability to the accepted-spend numeric game's `successProb`, consumption of the full-execution source, consumption of the active hash boundary, all active query symbols priced, multi-target horizon priced, and proof witnesses for every native full-boundary proposition before constructing `NativeFullSignatureSecurityBoundary`. The certificate now exposes named projections from full-signature success to production equality, accepted-spend game equality, accepted-spend probability, accepted-spend bound, accepted-spend reported-security bits, target-security bits, the fixed active `2^-128` target, and the same fixed target after erasing to `NativeFullSignatureSecurityBoundary`; the boundary derives `signatureForgerySuccess_le_acceptedSpend` through the certificate projection instead of accepting a standalone inequality proof. Rust deployment preflight records both `full_signature_reduction_certificate_boundary_present` and the future `full_signature_reduction_certificate_instantiated` gate, includes both in the proof-side preflight conjunction and digest frame, exports `PATH2_NATIVE_DEPLOYMENT_BLOCKER_FULL_SIGNATURE_REDUCTION_CERTIFICATE`, and rejects comment/string marker spoofing, assumed certificate/full-boundary parameters, multiline selected-evidence stubs, and a missing `multiTargetLossPricedProof` witness in the focused regression. This remains fail-closed: `selectedRustFullSignatureReductionBoundary_instantiation_v1` is absent and must be instantiated from production EUF-CMA/blockchain evidence.

AP2-R001/AP2-N003 full-reduction exact-boundary note: the full-signature reduction certificate boundary source gate now requires exact Lean declarations for `SelectedRustFullSignatureReductionEvidenceReport`, its report/facts predicates, `SelectedRustFullSignatureReductionCertificate`, and the certificate constructor/projection functions. Regressions reject suffixed report or certificate structures, so the future final-reduction path cannot be marked boundary-present by a shadow certificate family.

AP2-R001/AP2-N003 full-reduction accepted-spend certificate note: `SelectedRustFullSignatureReductionCertificate` now consumes `SelectedRustAcceptedSpendSoundnessCertificate` directly and derives the erased `NativeAcceptedSpendSoundnessBoundary` via `toAcceptedSpendSoundnessBoundary`. The full-reduction source gate requires a named `selectedRustAcceptedSpendCertificate` in addition to the derived boundary projection, so the final theorem cannot forget the active numeric trace/outcome and break-family pins before constructing the full-signature boundary.

AP2-R001/AP2-N003 full-reduction accepted-spend probability note: `SelectedRustAcceptedSpendSoundnessCertificate` now exposes `successProb_le_acceptedSpendForgeryProb`, deriving packaged full-forgery success below the exported accepted-spend probability through the selected numeric source. `SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary` consumes that theorem instead of reaching directly into the raw priced certificate's union-bound proof, and the full-reduction source gate now requires the future final theorem to mention this certificate-level projection.

AP2-R001/AP2-N003 full-reduction report/digest binding note: `SelectedRustFullSignatureReductionCertificate` now mirrors the accepted-spend certificate's report/digest binding discipline with `evidenceReportBound` and `evidenceDigestBound` proof fields. `toNativeFullSignatureSecurityBoundary` consumes those proofs in both the probability inequality and complete-claim construction paths. The full-reduction source gate requires named selected report/digest binding propositions and proofs, and rejects `True` aliases for those bindings.

AP2-R001/AP2-N003 full-reduction evidence-digest note: `SelectedRustFullSignatureReductionEvidenceReport` now carries `reductionEvidenceDigestsVerified` plus public digest slots for production signature probability, signature-to-accepted-spend linkage, full-execution source consumption, active hash boundary consumption, query-budget pricing, multi-target pricing, and verifier-game binding. `ReportVerified` requires the digest-verification bit, and the full-reduction source gate requires the new report markers before accepting the future final theorem shape.

AP2-R001/AP2-N003 full-reduction Rust-evidence note: Rust now exposes `Path2NativeFullSignatureReductionEvidenceV1` with schema, length-delimited frame, digest, report-verification helper, all-facts helper, verifier, production-facade exports, and registry membership. The canonical report is intentionally fail-closed: schema/report/digest verification passes and the seven public digest slots are filled from active Lean source windows, but production signature probability pinning, signature-to-accepted-spend linkage, full-execution-source consumption, active hash-boundary consumption, query-symbol pricing, multi-target pricing, and verifier-game binding remain false. The final full-reduction instantiation gate now requires this Rust evidence source before scanning for `selectedRustFullSignatureReductionBoundary_instantiation_v1`.

AP2-R001/AP2-N003 full-reduction dependency-direction note: the future full-reduction instantiation gate now scans the combined `Path2NativeAccountDesign.lean` and `Path2FullSpendConcrete.lean` sources, rather than only the account-design file. A certificate-only theorem shape is rejected unless it explicitly consumes `concreteFullSignatureExecutionSource_instantiation_v1`, `full_signature_with_concrete_full_execution_source_lift`, `selectedRustAcceptedSpendSoundnessBoundary_instantiation_v1`, `SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary`, `activeNativeHashBudgetBoundary`, and `activeNativeHashBudgetBoundary_hashBoundaryProps`. This prevents the final reduction from closing in the upstream account-design layer with local shells instead of the active concrete execution source, accepted-spend numeric theorem, and active hash boundary.

AP2-R001/AP2-N003 deployment-order note: the active native preflight now ranks the accepted-spend numeric-bound instantiation before the final full-signature reduction instantiation once the reduction boundary surface is present. This matches the Lean dependency direction: `SelectedRustFullSignatureReductionCertificate` consumes the selected accepted-spend certificate and derives its accepted-spend inequality through that numeric source, so productization cannot treat the full-reduction instantiation as the next independent proof target while the numeric source remains open.

AP2-R001/AP2-N003 full-reduction alias hardening note: the future full-signature reduction gate now rejects whitespace-varied local/global aliases that set the selected signature-forgery probability to `0`, use `rfl`/`le_rfl` for the production pin and accepted-spend inequality, or define the keygen/signing/completeness/verifier/full-forgery model propositions as bare `True`. A negative control includes every required prerequisite marker and still fails when those selected names are local zero/`True` shells.

AP2-R001/AP2-N003 full-reduction proof-shell hardening note: the future full-signature reduction gate now also rejects global `def`/`abbrev` aliases that set the report/digest binding proofs, report-verified/facts proofs, keygen/signing/completeness/verifier/full-forgery model proofs, accepted-spend closure proofs, hash-domain proofs, query-budget proof, or multi-target proof to `True.intro`. A regression prepends those proof aliases to the otherwise accepted full-reduction theorem shape and confirms the gate stays open.

AP2-R001 QROM-review source-gate note: `path2_native_deployment_preflight_v1` now records `hash_qrom_review_present` from `path2_native_hash_qrom_review_source_present_v1` instead of checking only the review schema. The helper source-inspects the active hash/QROM review struct, review constructor, active native query-symbol list, ordinary-hash assumptions, hidden obligations not discharged by hashing, consensus-carrier byte accounting, and the public-only review regression. If this source boundary opens, deployment now gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_HASH_QROM_REVIEW` and verifier rejection `reject_active_native_hash_qrom_review_source_open`; the existing `PATH2_NATIVE_DEPLOYMENT_BLOCKER_QROM_TARGET_HORIZON` remains the stronger priced-horizon gate.

AP2-R001/AP2-R002 public-boundary preflight note: `proof_side_public_boundary_preflight_complete` is no longer a bare `true`. It is now the conjunction of the active source-gated public/proof boundary bits: projection encoders, checked-core frame, public serializers, consensus carrier, verifier preflight report, global domain registry, hash/QROM review, hidden-source semantics, checked-core composition, QROM horizon, full-signature execution source, full-signature reduction certificate boundary and instantiation, accepted-spend numeric bound, no-export serializer boundary, and no-export serializer instantiation. It intentionally excludes productization gates for signer/verifier pipeline, constant-time audit, storage hardening, and facade export curation, which remain separately tracked.

AP2-R001/AP2-R002 deployment-preflight evidence-frame note: `path2_native_deployment_preflight_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json` output. The frame binds every public/proof boundary bit, five-source closure evidence digest/verified/all-facts bits, facade and productization gates, byte-size limits, deployment-ready status, and ordered internal blocker list; it is exported through the production facade and covered by a `Sha256(frame)` regression.

AP2-R002 checked-core source-gate note: `path2_native_deployment_preflight_v1` no longer treats checked-core composition review as an unconditional `true`. It now separates `path2_native_checked_core_byte_bridge_source_present_v1`, which source-inspects `Descender/Path2CheckedCoreByteBridge.lean` for the selected Rust byte-source decomposition and source-inspects `path2_native_checked_core_accept_v1` for frame validation, canonical proof length/decode, proof-commitment equality, challenge derivation from the checked-core frame, actual/expected Fq12 syndrome computation, and sigma-equation equality, from the stronger `path2_native_checked_core_selected_rust_instantiation_present_v1` gate. The stronger gate requires a concrete selected-Rust instantiation theorem wiring the named parser/proof/frame/hash facts into `SelectedRustCheckedCoreByteSource`, and rejects `axiom`/`constant`/`opaque`/module-level `variable` stubs for those facts.

AP2-R002 checked-core instantiation-gate hardening note: `path2_native_checked_core_selected_rust_instantiation_present_v1` now strips Lean comments and string literal contents before scanning for the future `selectedRustCheckedCoreByteSource_instantiation_v1` theorem and normalized marker set. Its forbidden-stub detector also catches multiline `axiom`/`constant`/`opaque` declarations, module-level `variable` stubs, and vacuous `True`/`trivial` report-proof definitions while ignoring those markers inside comments or strings. The deployment preflight regression now includes comment/string spoofing and multiline-stub negative controls, so AP2-N002 cannot be closed by marker text that is not executable Lean source.

AP2-R002 closure note: `checked_core_composition_review_complete` still requires
both the checked-core byte-bridge source surface and the selected-Rust
checked-core instantiation gate, but AP2-N002 now satisfies both gates. Active
native deployment no longer reports
`PATH2_NATIVE_DEPLOYMENT_BLOCKER_CHECKED_CORE_COMPOSITION`; after AP2-N001 and
AP2-N002 closure the smallest active proof-side blocker is full-signature
execution source instantiation.

AP2-N002 component-target note: `Descender/Path2CheckedCoreByteBridge.lean` now exposes `SelectedRustCheckedCoreByteInstantiationInputs`, `selectedRustCheckedCoreByteSource_from_instantiation_inputs_valid_or_named_break`, and the selected theorem `selectedRustCheckedCoreByteSource_instantiation_v1`. The selected theorem fills the implementation-oriented package from report/digest-bound byte facts: selected acceptor acceptance, parser/native-handle agreement, proof length/canonicality, decoded proof commitment, parsed response syndrome, checked-core frame bytes, verifier handle equality, parser equality, and challenge/expected-vs-actual syndrome equality.

AP2-N002 Rust-evidence note: `path2_native_checked_core_byte_instantiation_evidence_v1` emits a public digest-only report for the selected Rust checked-core byte facts consumed by `SelectedRustCheckedCoreByteInstantiationInputs`. It records accepted-path parser/native-handle, proof length/canonicality, decoded commitment, response syndrome, frame byte roundtrip, verifier-handle/public-key binding, parser equality, challenge/expected-vs-actual syndrome equality, the verified `Path2NativeHashByteDomainEvidenceV1` digest/facts, checked-core frame-domain equality to that evidence object, checked-core frame bytes using that domain prefix, challenge and frame-digest derivation from that domain-prefixed frame, and negative controls for short, noncanonical, commitment-mismatch, and verifier-handle-mismatch inputs. The checked-core source-window audit now pins that report and accepts the concrete Lean selected instantiation theorem.

AP2-N002 Lean evidence-report note: `Descender/Path2CheckedCoreByteBridge.lean` now mirrors the Rust checked-core byte-instantiation evidence as `SelectedRustCheckedCoreByteEvidenceReport`, with concrete boolean fields for schema/digest verification, every selected byte-source component fact, the active hash byte-domain evidence digest/verified/all-facts binding, checked-core frame-domain/prefix/challenge/digest use of that evidence, and every negative control. `SelectedRustCheckedCoreByteEvidenceCertificate` now carries that report, the checked-core report digest, the hash byte-domain evidence digest, and proofs of `ReportVerified` and `FactsAllTrue`, instead of carrying arbitrary `evidenceReportVerified : Prop` and `evidenceFactsAllTrue : Prop` fields. The AP2-N002 instantiation gate now requires `selectedRustHashByteDomainEvidenceDigest` to be passed through `SelectedRustCheckedCoreByteEvidenceCertificate.ofReportAndComponents`, so the final theorem must route through a report-shaped public evidence object before it can build `SelectedRustCheckedCoreByteInstantiationInputs`.

AP2-N002 facade/evidence hardening note: the checked-core byte evidence report and digest helper are now exported through the curated `tools/production-reference` facade and pinned by `path2_native_production_facade_exports_source_present_v1`. The report's negative-control probes now use non-panicking rejection checks, so an unexpected accept is reported as a false evidence bit rather than crashing the public evidence constructor.

AP2-N002 supplied-evidence verifier note: `verify_path2_native_checked_core_byte_instantiation_evidence_v1` now recomputes the checked-core byte evidence report from the supplied projection bundle and proof bytes, rejects stale/tampered report fields, and returns the verified report digest. The verifier is exported through the curated facade and pinned by the checked-core source-window audit, giving downstream AP2-N002 consumers a concrete consistency check instead of trusting a standalone report digest.

AP2-N002 evidence-frame note: `path2_native_checked_core_byte_instantiation_evidence_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json` output. The frame binds the parser/native-handle, proof length/canonicality, decoded commitment, response syndrome, frame bytes, verifier handle, parser equality, hash/syndrome, negative-control facts, checked-core frame/proof/verifier/challenge digests, proof byte length, and cleanup policy in fixed order; it is exported through the production facade, pinned by the checked-core source gate, and covered by a `Sha256(frame)` regression.

AP2-N002 all-facts verifier note: `path2_native_checked_core_byte_instantiation_evidence_all_facts_true` now makes the checked-core evidence verifier reject a faithfully recomputed report unless every selected-Rust byte-source component fact and every negative control is true. The regression covers a public-key/verifier-handle rebinding case where checked-core proof generation can still produce a report, but `verifier_handle_eq` is false; verification now rejects it with an unsatisfied-component-fact error instead of returning a digest.

AP2-N002/P2FS-005 hash-domain consumption note: the checked-core byte evidence constructor now builds `Path2NativeHashByteDomainEvidenceV1`, verifies it with `verify_path2_native_hash_byte_domain_evidence_v1`, carries the resulting digest plus verified/all-facts bits in the checked-core evidence frame, and rejects tampering through the supplied-evidence verifier. It also records that `path2_native_checked_core_frame_v1_bytes` starts with the checked-core frame domain named by the evidence object, that `path2_native_checked_core_frame_v1_challenge` is the challenge hash of those exact domain-prefixed bytes, and that `checked_core_frame_digest` is SHA-256 over those same bytes. The checked-core source gate and future Lean instantiation shape now require this hash-domain digest and frame-domain facts, so AP2-N002 can no longer close independently of the active byte-domain evidence object.

AP2-N002 preflight verification note: `Path2NativeConsensusCarrierPreflightReportV1` now carries `checked_core_byte_instantiation_evidence_verified`. The accepted preflight path sets it only after constructing and verifying the checked-core byte evidence report with `verify_path2_native_checked_core_byte_instantiation_evidence_v1`; all carrier/context/bundle/DA reject paths leave both the digest absent and the verified bit false. This keeps public-boundary acceptance from silently degrading into an unchecked carried digest.

AP2-N002 verifier/signer propagation note: `Path2NativeVerifierPipelineVerdictV1` and `Path2NativeSignerVerifierPipelineEvidenceV1` now mirror `checked_core_byte_instantiation_evidence_verified`. The signer/verifier evidence constructor rejects if the verifier did not verify the checked-core report or if the verifier's checked-core evidence digest differs from the independently constructed signer-side digest, so the top-level public evidence can no longer carry a checked-core digest without the corresponding verified predicate.

AP2-N002 signer/verifier evidence binding note: `Path2NativeSignerVerifierPipelineEvidenceV1` now carries `checked_core_byte_instantiation_evidence_digest`, computed from `path2_native_checked_core_byte_instantiation_evidence_v1` over the same projection bundle and checked-core proof bytes used by the selected verifier path. The honest-flow regression independently recomputes that report digest, so public signer/verifier evidence is now bound to the checked-core byte-instantiation evidence surface while production remains fail-closed on the missing Lean instantiation theorem.

AP2-N002/AP2-N006 signer/verifier evidence-frame note: `path2_native_signer_verifier_pipeline_evidence_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json` output. The frame binds signer public output, carrier, checked-core proof, checked-core evidence digest/verified bit, five-source evidence digest/verified/all-facts bits, native boundary, verifier verdict status, smallest remaining gate, deployment blocker list, and explicit non-claims in fixed order; it is exported through the production facade, pinned by the signer/verifier source gate, and covered by a `Sha256(frame)` regression.

AP2-N002 verifier verdict binding note: `Path2NativeVerifierPipelineVerdictV1` now carries `checked_core_byte_instantiation_evidence_digest` when the public verifier boundary accepts, and leaves it absent when the public boundary rejects. The verifier regression recomputes the checked-core byte evidence digest for the accepted carrier and asserts the rejected carrier reports `None`, keeping the verifier output bound to AP2-N002 evidence without treating a failed public boundary as an instantiation witness.

AP2-N002 preflight binding note: `Path2NativeConsensusCarrierPreflightReportV1` now also carries `checked_core_byte_instantiation_evidence_digest` only on accepted consensus-carrier preflight reports. The preflight regression recomputes the same checked-core byte evidence digest for the accepted carrier and asserts every rejected carrier/bundle/DA/context report leaves the digest absent, binding the lowest public verifier report to AP2-N002 evidence without exporting a witness for rejected public boundaries.

AP2-N002 preflight fail-closed note: consensus-carrier preflight no longer sets `accepted = true` before checked-core byte evidence digest construction succeeds. If the carrier matches but the AP2-N002 evidence report/digest cannot be produced, preflight rejects with `reject_checked_core_byte_evidence_unavailable`; the verifier verdict now copies the preflight digest instead of recomputing it independently.

AP2-N002 source-gate hardening note: the concrete checked-core instantiation gate now rejects selected-Rust facts supplied as theorem parameters, in addition to module-level `axiom`/`constant`/`opaque`/`variable` stubs. This prevents `selectedRustCheckedCoreByteSource_instantiation_v1` from closing deployment by assuming the parser/frame/hash facts in its header instead of deriving them from concrete Rust/model definitions.

AP2-N002 component-input gate note: `path2_native_checked_core_selected_rust_instantiation_present_v1` now requires the future `selectedRustCheckedCoreByteSource_instantiation_v1` theorem to locally construct `selectedRustCheckedCoreInputs : SelectedRustCheckedCoreByteInstantiationInputs` and route through `selectedRustCheckedCoreByteSource_from_instantiation_inputs_v1`, with negative controls rejecting both direct `SelectedRustCheckedCoreByteSource` construction and an input record supplied as a theorem parameter. This keeps the final checked-core closure tied to the component-evidence package already audited by the public verifier/preflight reports.

AP2-N002 evidence-certificate bridge note: `Descender/Path2CheckedCoreByteBridge.lean` now exposes `SelectedRustCheckedCoreByteEvidenceCertificate`, carrying the checked-core public evidence report verification condition, proof of that condition, the all-component-facts-true condition, proof of that condition, and the individual parser/frame/hash/syndrome facts. `SelectedRustCheckedCoreByteEvidenceCertificate.toInstantiationInputs` is now the required route into `SelectedRustCheckedCoreByteInstantiationInputs`, and the Rust instantiation gate rejects an evidence certificate supplied as a theorem parameter, locally constructed component inputs that bypass the evidence certificate, theorem/header/stub assumptions for `selectedRustCheckedCoreEvidenceReportVerified`, `selectedRustCheckedCoreEvidenceReportVerifiedProof`, `selectedRustCheckedCoreEvidenceFactsAllTrue`, or `selectedRustCheckedCoreEvidenceFactsAllTrueProof`, and vacuous local `True`/`True.intro` verification construction. This makes the final AP2-N002 theorem shape consume proof-bearing verified public evidence instead of merely rebuilding the component package or naming verification propositions.

AP2-N002 bridge-assumption gate note: `path2_native_checked_core_selected_rust_instantiation_present_v1` now also rejects `selectedRustCheckedCoreByteSource_instantiation_v1` theorem headers that assume `Path2NativeCheckedCoreAcceptV1Bridge`, `Path2NativeCheckedCoreAcceptV1Soundness`, `SelectedRustCheckedCoreCompositionClosure`, `NativeAcceptedSpendSoundnessBoundary`, or `NativeFullSignatureSecurityBoundary`. This closes a shortcut where the selected byte-source theorem could build the checked-core evidence certificate locally while importing the native byte-to-accept bridge, equation-soundness boundary, or downstream full-boundary closure as a parameter.

AP2-R002 composition note: `Descender/Path2CheckedCoreByteBridge.lean` now exposes `selectedRustCheckedCoreByteSource_checkedCoreAccepts_or_named_break`, composing `SelectedRustCheckedCoreByteSource` with `Path2NativeCheckedCoreAcceptV1Bridge` directly into the abstract `CheckedCoreSpendReduction.checkedCoreAccepts` predicate or the same named byte/parser/frame/domain residual breaks. The Rust source gate now checks for this theorem, so downstream active-path proof code can consume the selected Rust source endpoint without manually stitching together the byte certificate and native V1 bridge.

AP2-R002 checked-core validity note: `Descender/Path2CheckedCoreByteBridge.lean` now also exposes `selectedRustCheckedCoreByteSource_valid_or_named_break`, composing the selected Rust byte source through `Path2NativeCheckedCoreAcceptV1Bridge` and `Path2NativeCheckedCoreAcceptV1Soundness`. The endpoint yields valid checked-core evidence or one of the named byte data, byte parser, checked-core frame, domain/QROM, or checked-core equation soundness breaks. The Rust source gate now checks for this validity endpoint and the widened `CheckedCoreByteAndEquationBreakAlternatives` residual set.

AP2-R002 full-boundary handoff note: `SelectedRustCheckedCoreCompositionClosure` now packages the selected Rust/model checked-core closure proposition, `NativeAcceptedSpendSoundnessBoundary.withSelectedRustCheckedCoreClosure` installs it into `checkedCoreCompositionClosed`, and `NativeFullSignatureSecurityBoundary.withSelectedRustCheckedCoreClosure` threads the same closure to the active full-signature boundary. The Rust source gate now requires the accepted-spend and full-signature checked-core adapters, so AP2-N002 cannot be counted as closed by a local byte bridge alone. This remains a boundary handoff only; the final `selectedRustCheckedCoreByteSource_instantiation_v1` production theorem is still absent.

AP2-N002 checked-core evidence-constructor note: `SelectedRustCheckedCoreByteEvidenceCertificate` now carries an `evidenceDigest`, and `SelectedRustCheckedCoreByteEvidenceCertificate.ofReportAndComponents` is the named constructor for the selected checked-core report, digest, report-verification proof, all-facts proof, and parser/frame/hash component facts. The certificate also carries `evidenceSelectedRustProductionCheckedCoreAcceptorAccepts` and `evidenceSelectedRustCheckedCoreAcceptorPinnedProof`, forcing the selected acceptor predicate used by all parser/frame/hash implications to equal the named production checked-core acceptor. The selected-Rust instantiation gate now requires the future theorem to use this constructor, the production acceptor, and the pinning proof; rejects anonymous evidence-certificate record literals; and has a negative control rejecting a constructor-built theorem whose selected and production acceptors are both `fun _ _ _ _ => False`. This keeps AP2-N002 aligned with the report/digest-bound style used by AP2-N001 while preventing vacuous checked-core component implications.

AP2-N002 checked-core proof-shell hardening note: the selected checked-core source gate now rejects `abbrev` aliases that set the report-verified or facts-all-true proof names to `True.intro`, and global `def`/`abbrev` aliases that set the selected or production checked-core acceptor to `fun _ _ _ _ => False`. Regressions prepend those aliases to the otherwise accepted checked-core theorem shape and keep the gate open.

AP2-R002 source-window note: `path2_native_checked_core_byte_bridge_source_present_v1` now slices the Lean checked-core byte-source record, checked-core-accept endpoint, valid-or-break endpoint, and native V1 soundness structure separately. The gate requires `Path2NativeCheckedCoreAcceptV1Bridge`, `SelectedRustCheckedCoreByteSource`, `Path2NativeCheckedCoreAcceptV1Soundness`, `path2_native_checked_core_accept_valid_or_named_break`, and each explicit byte/parser/frame/domain/equation residual conversion in the endpoint window, instead of accepting broad markers anywhere in `Path2CheckedCoreByteBridge.lean`.

AP2-R002 projection-encoder source-gate note: `path2_native_deployment_preflight_v1` now records `projection_encoders_present` from `path2_native_projection_encoder_source_boundary_present_v1` instead of a bare boolean. The helper source-inspects the active Rust context/account/spend/lifecycle/state/transcript projection encoders, public-key and authorization-class digest encoders, projection preimage DA framing, native boundary digest framing, stable-layout tests, mutation-cascade tests, private-sentinel negative controls, and DA frame roundtrip test. If this source boundary opens, deployment now gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_PROJECTION_ENCODERS` and verifier rejection `reject_active_native_projection_encoder_source_open`.

AP2-R002 checked-core-frame source-gate note: `path2_native_deployment_preflight_v1` now records `checked_core_frame_present` from `path2_native_checked_core_frame_source_boundary_present_v1` instead of a bare boolean. The helper source-inspects the active Rust checked-core frame structure, context validator, canonical byte encoder/parser, challenge/digest derivation, golden-vector roundtrip, mutation-to-challenge tests, invalid-context rejection, and private-handle exclusion. If this source boundary opens, deployment now gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_CHECKED_CORE_FRAME` and verifier rejection `reject_active_native_checked_core_frame_source_open`.

AP2-R002 consensus-carrier source-gate note: `path2_native_deployment_preflight_v1` now records `consensus_carrier_present` from `path2_native_consensus_carrier_source_boundary_present_v1` instead of a body-size equality. The helper source-inspects the active Rust consensus-carrier constructor, context validator, canonical serializer/parser/digest, bundle verifier, canonical/mutation tests, and bundle-mismatch tests. If this source boundary opens, deployment now gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_CONSENSUS_CARRIER` and verifier rejection `reject_active_native_consensus_carrier_source_open`.

AP2-R002 verifier-preflight source-gate note: `path2_native_deployment_preflight_v1` now records `verifier_preflight_report_present` from `path2_native_verifier_preflight_report_source_present_v1` instead of a bare boolean. The consensus-carrier source gate now owns carrier construction/serialization/bundle verification, while this helper source-inspects the preflight report struct, accepted/rejection-code computation, carrier/boundary digest recomputation, projection-DA mismatch handling, and public-only report tests. If this source boundary opens, deployment now gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_VERIFIER_PREFLIGHT_REPORT` and verifier rejection `reject_active_native_verifier_preflight_report_source_open`.

AP2-R002 verifier-preflight evidence-frame note: `path2_native_consensus_carrier_preflight_report_v1_digest` now hashes an explicit length-delimited frame instead of relying on `serde_json`. The frame binds accepted/rejection status, optional carrier/recomputed-boundary/checked-core evidence digests with presence bits, the checked-core evidence verified bit, every public carrier digest field, public state root, obstruction flags digest, and projection-preimage DA digest; it is exported through the production facade, pinned by the verifier-preflight source gate, and covered by a `Sha256(frame)` regression.

AP2-R002 verifier-verdict evidence-frame note: `path2_native_verifier_pipeline_verdict_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json`. The frame binds public-boundary/production acceptance, rejection code, optional consensus-carrier and checked-core evidence digests with presence bits, checked-core/five-source verified status, native boundary digest, and framed digests of the embedded consensus-carrier preflight report and deployment preflight report; it is exported through the production facade and covered by a `Sha256(frame)` regression.

AP2-R002 domain-registry source-gate note: `path2_native_deployment_preflight_v1` now records `global_domain_registry_present` from `path2_native_global_domain_registry_source_present_v1` instead of checking for one consensus-carrier label. The helper source-inspects the production domain/schema registry, the active native domain/public-schema uniqueness test, and the global disjointness test for the active `PATH2_NATIVE_`/`PATH2_PUBLIC_NATIVE_` registry surface. If this source boundary opens, deployment now gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_GLOBAL_DOMAIN_REGISTRY` and verifier rejection `reject_active_native_global_domain_registry_source_open`.

AP2-R002/AP2-R006 public-view serializer source-gate note: `path2_native_deployment_preflight_v1` now records `public_view_serializers_present` from `path2_native_public_view_serializer_source_boundary_present_v1`. The helper source-inspects the active Rust account-view and spend-view serializers, JSON encoders, context/proof-length validation, digest-only proof export, allowlisted account fields, no-private-material checks, mutation tests, and bad-context/bad-length rejection. If this source boundary opens, deployment now gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_PUBLIC_VIEW_SERIALIZERS` and verifier rejection `reject_active_native_public_view_serializer_source_open`.

AP2-R002 facade source-gate note: `path2_native_deployment_preflight_v1` now records `facade_exports_present` from `path2_native_production_facade_exports_source_present_v1`. The helper source-inspects `tools/production-reference/src/lib.rs` for the curated active Path 2 carrier/preflight/verifier/public-view exports, the active source-gate blocker constants, and the no-glob/no-selected-material negative-control test. If this boundary opens, deployment now gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_FACADE_EXPORTS` and verifier rejection `reject_active_native_facade_exports_open`.

AP2-N006 signer/verifier productization note: `path2_native_deployment_preflight_v1` no longer sets `signer_verifier_pipeline_present` from a literal `false`. It now calls `path2_native_signer_verifier_pipeline_productized_v1`, which requires source-inspected signer public-output/evidence code and tests plus completed honest-constructor and keygen productization scopes. The source evidence is present, but the honest-constructor scope remains `complete: false` and keygen scope remains `productized: false`, so deployment still records `PATH2_NATIVE_DEPLOYMENT_BLOCKER_SIGNER_VERIFIER_PIPELINE`. The verifier currently rejects earlier at `reject_active_native_hidden_source_semantics_open` until AP2-N001 closes.

AP2-N006/AP2-N003/AP2-N005 blocker propagation note: `path2_native_signer_verifier_pipeline_source_present_v1` now requires signer/verifier evidence to propagate `hidden_source_semantics_bridge_complete`, `full_signature_execution_source_instantiated`, `accepted_spend_numeric_bound_instantiated`, `no_export_serializer_instantiated`, and the AP2-N001/AP2-N003/AP2-N005 blocker constants alongside checked-core, QROM, signer/verifier, CT, and storage gates. `smallest_remaining_productization_gate` is now sourced by `path2_native_smallest_remaining_deployment_gate_v1`, which checks AP2-N001 first, AP2-N002/QROM next, then AP2-N003, AP2-N005 no-export, and only then signer/CT/storage productization. A synthetic regression pins that if hidden-source, checked-core, QROM, full-execution, and accepted-spend are closed but no-export remains open, signer/verifier evidence reports `PATH2_NATIVE_DEPLOYMENT_BLOCKER_NO_EXPORT_SERIALIZERS` before `PATH2_NATIVE_DEPLOYMENT_BLOCKER_SIGNER_VERIFIER_PIPELINE`.

AP2-N006 signer public-output readiness note: `path2_native_signer_pipeline_public_output_v1` now sets `signer_pipeline_ready` from `deployment_preflight.internal_deployment_ready`, not from the narrower signer/verifier and CT bits alone. The public-output source gate and regression require this equality and require the current checked-core and full-execution blockers to remain visible in the embedded preflight, so signer public output cannot become "ready" while AP2-N002/AP2-N003 or storage remain open.

AP2-N006 signer public-output evidence-frame note: `path2_native_signer_pipeline_public_output_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json`. The frame binds signer readiness, rejection code, canonical consensus carrier bytes, carried carrier digest, checked-core proof digest/length, projection-preimage DA digest, native boundary digest, transcript digest, and the framed deployment-preflight digest; it is exported through the production facade, pinned by the signer/verifier source gate, and covered by a `Sha256(frame)` regression.

AP2-N006 signer execution-report evidence-frame note: `path2_native_signer_execution_report_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json`. The internal frame binds constructor/public-boundary/production status, rejection code, optional signer-output/carrier/native-boundary/checked-core proof digests with presence bits, cleanup policy, fixed entropy-q attempt count, and the framed deployment-preflight digest; success and rejection regressions cover `Sha256(frame)`, and downstream CT/cleanup reports continue to consume this digest without treating it as a production readiness claim.

AP2-N006 CT stage-report evidence-frame note: `path2_native_signer_constant_time_stage_report_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json`. The frame binds audit/constant-time claim status, fixed public schedule requirement, ordered stage labels, success/rejection execution-report digests, public rejection-code status, cleanup policy, CT blocker, and the framed deployment-preflight digest; the CT stage regression covers `Sha256(frame)` while keeping `audit_complete = false`.

AP2-N006 CT q-sampler source-audit evidence-frame note: `path2_native_signer_ct_source_audit_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json`. The frame binds the audited function name, source/backend audit status, fixed-attempt/fallback/mask-select/no-modulo/no-exhaustion/fixed-byte-load facts, source-window digest, CT blocker, and explicit non-claims; the q-sampler source-audit regression covers `Sha256(frame)`.

AP2-N006 SHA-256 signer audit evidence-frame note: `path2_native_signer_sha256_boundary_audit_v1_digest` and `path2_native_signer_sha256_call_schedule_audit_v1_digest` now hash explicit length-delimited frames instead of `serde_json`. The frames bind the source-boundary/call-schedule completion bits, backend CT non-claim, domain/label/public-binding/owned-entropy/counter facts, fixed digest-call counts, source digests, CT blocker, and explicit non-claims; both SHA-256 audit regressions cover `Sha256(frame)`.

AP2-N006 rejection-cleanup evidence-frame note: `path2_native_signer_rejection_cleanup_audit_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json`. The frame binds audit status, public error-surface status, zeroization-claim status, success/constructor/entropy report digests, rejection-code presence and values, public-only/no-private-material facts, CT blocker, framed deployment-preflight digest, and explicit non-claims; the cleanup regression covers `Sha256(frame)` while keeping the zeroization claim incomplete.

AP2-N006 CT boundary/routing evidence-frame note: `path2_native_signer_os_entropy_boundary_audit_v1_digest`, `path2_native_signer_runtime_cleanup_audit_v1_digest`, and `path2_native_signer_production_routing_audit_v1_digest` now hash explicit length-delimited frames instead of `serde_json`. The frames bind OS entropy ordering/sanitization facts, runtime cleanup and fixed-frame zeroization facts, prevalidated production entrypoint routing facts, source-window digests, CT blocker status, and explicit non-claims; all three focused regressions cover `Sha256(frame)` while keeping backend RNG health, allocator inspection, and whole-signer timing claims open.

AP2-N006 CT evidence-ledger evidence-frame note: `path2_native_signer_ct_evidence_ledger_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json`. The frame binds every source/backend evidence pin, smoke-only/open CT claim bit, backend-script bundle digest, CT blocker, ordered remaining blockers, and explicit non-claims; the ledger regression covers `Sha256(frame)` while keeping final CT completion false until target-host execution, backend constant-time, and residual-lifetime evidence are closed.

AP2-N006 productization-scope evidence-frame note: `path2_native_honest_constructor_scope_v1_digest`, `path2_native_signer_keygen_productization_scope_v1_digest`, `path2_native_constant_time_audit_scope_v1_digest`, and `path2_native_storage_lifecycle_hardening_scope_v1_digest` now hash explicit length-delimited frames instead of `serde_json`. The frames bind each scope's completion/productization bit, blocker, ordered artifact/harness lists, and explicit non-claims; the focused scope regressions cover `Sha256(frame)` while preserving the signer/verifier, CT, and storage blockers.

AP2-N006 storage profile evidence-frame note: `production_target_chain_database_adapter_profile_digest` now hashes an explicit length-delimited frame instead of `serde_json` output, and the local/SQLite canonical-node KV stores consume that digest for committed roots and profile metadata. The frame binds the database adapter schema, chain id, engine, transaction semantics, trusted-anchor placement, KV namespace/value schemas, public-key identity rule, spent-key shape, and external storage review digest; a focused regression covers `Sha256(frame)` and profile-field mutation.

AP2-N006 storage persistence codec note: the target-chain KV value encoders (`lifecycle`, `spent_slots`, `trusted_anchor`) now write explicit length-delimited frames instead of JSON, while the decoders retain JSON fallback for existing reference stores. The framed codec binds lifecycle snapshot sequence/reservations/account-slot records, spent-slot ledger records including optional pad-seed digests, and trusted-anchor records; the focused storage codec regression covers framed round trips plus legacy JSON fallback. Remaining `serde_json::to_vec` occurrences in `production_reference.rs` are test-only legacy/fixture writers.

AP2-N006 constant-time productization note: `path2_native_deployment_preflight_v1` no longer sets `constant_time_audit_complete` from a literal `false`. It now calls `path2_native_constant_time_audit_complete_v1`, which requires source-inspected CT evidence ledger/scope code and tests plus completed CT scope and ledger claims. The source evidence and backend-script pins are present, but the ledger still marks timing evidence as smoke-only and leaves backend CT, whole-signer timing, register/internal hash lifetime, target-host bundle execution, and final CT completion open, so deployment still gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_CONSTANT_TIME`.

AP2-N006 storage productization note: `path2_native_deployment_preflight_v1` no longer sets `storage_lifecycle_hardening_complete` from a literal `false`. It now calls `path2_native_storage_lifecycle_hardening_complete_v1`, which requires source-inspected storage/lifecycle hardening scope code and tests plus `hardening_complete`. The active scope records consensus storage, lifecycle, data-availability, restore/reorg artifacts, and required harnesses, but it remains `hardening_complete: false`, so deployment still gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_STORAGE`.

AP2-R004 full-execution source-gate note: `path2_native_deployment_preflight_v1` now records `full_signature_execution_source_boundary_present`, sourced by `path2_full_signature_execution_source_boundary_present_v1`. The helper source-inspects `Descender/Path2FullSpendConcrete.lean` for `ConcreteFullSignatureExecutionSource`, the execution-backed full-game predicate package, the execution-backed `FullForgeryToAcceptedSpendSoundness`, and `concrete_fullExecutionImpliesAcceptedSpendForgery`. This keeps production readiness tied to the non-tautological execution-success bridge rather than the convenience carrier where full-signature success is definitionally verifier acceptance.

AP2-N003 full-execution source closure note: `path2_native_deployment_preflight_v1` still separates `full_signature_execution_source_boundary_present` from `full_signature_execution_source_instantiated`, but the selected theorem `concreteFullSignatureExecutionSource_instantiation_v1` is now present in `Descender/Path2FullSpendConcrete.lean`. It locally constructs `selectedRustFullExecutionSource : ConcreteFullSignatureExecutionSource` through `SelectedRustFullExecutionSourceCertificate.ofReportAndExecutionSource`, pins the selected execution predicate to `selectedRustProductionExecutionSucceeds`, proves execution success implies concrete verifier acceptance or a named break, and threads the source through the concrete full-execution lift/full-boundary handoff. Rust preflight now advances past `PATH2_NATIVE_DEPLOYMENT_BLOCKER_FULL_SIGNATURE_EXECUTION_SOURCE`; the next AP2-N003 blocker is `PATH2_NATIVE_DEPLOYMENT_BLOCKER_FULL_SIGNATURE_REDUCTION_CERTIFICATE`.

AP2-N003 execution-certificate bridge note: `Descender/Path2FullSpendConcrete.lean` now exposes `SelectedRustFullExecutionSourceCertificate`, carrying report-shaped production execution evidence, proofs that the report is verified and all execution facts are true, the selected Rust production execution-success predicate, and the proof that success implies concrete verifier acceptance or a named break. `SelectedRustFullExecutionSourceCertificate.toExecutionSource` is now the required route into `ConcreteFullSignatureExecutionSource`; the Rust instantiation gate rejects a source certificate supplied as a theorem parameter, direct local construction of `ConcreteFullSignatureExecutionSource`, theorem/header/stub assumptions for the selected execution-success or verification facts/proofs, multiline `axiom`/`constant`/`opaque`/`variable` stubs, comment/string marker spoofing, and vacuous certificate construction that sets the evidence report or proofs to `True`. This closes the proof-shape hole where the previous certificate named a verification proposition without requiring Lean evidence for it.

AP2-N003 execution evidence-report note: `SelectedRustFullExecutionSourceCertificate` now carries a concrete `SelectedRustFullExecutionEvidenceReport` plus proofs of `ReportVerified` and `FactsAllTrue`, rather than an arbitrary `selectedRustFullExecutionSourceVerified : Prop`. The report records schema/digest verification, production execution surface verification, execution-success predicate pinning, success-to-verifier-accept-or-break coverage, concrete game config pinning, non-tautological execution-source use, and named-break exhaustiveness. The AP2-N003 instantiation gate now requires `evidenceReport := selectedRustFullExecutionEvidenceReport`, keeping the future production execution theorem tied to a report-shaped evidence object.

AP2-N003 execution constructor hardening note: `SelectedRustFullExecutionSourceCertificate` now also carries an `evidenceDigest`, and `SelectedRustFullExecutionSourceCertificate.ofReportAndExecutionSource` is the named Lean constructor for the full-execution source certificate. The Rust future-instantiation gate requires that constructor plus `selectedRustFullExecutionEvidenceDigest`, rejects theorem/header/declaration assumptions of the digest alongside the report and source facts, and no longer accepts a raw certificate record literal as the positive AP2-N003 shape. This keeps the lifecycle/full-execution theorem fail-closed until the selected production execution evidence is digest-bound and constructor-built.

AP2-N003 execution predicate-pinning note: `SelectedRustFullExecutionSourceCertificate` now carries both `selectedRustExecutionSucceeds` and `selectedRustProductionExecutionSucceeds`, plus `selectedRustExecutionPredicatePinnedProof` equating them. The Rust instantiation gate requires the production predicate and pinning proof markers, rejects header/declaration assumptions of those facts, and has a negative control for a constructor-built theorem that sets both execution predicates to `fun _ => False`. This prevents the future full-execution theorem from closing by proving a vacuous success-implies-acceptance bridge instead of binding the selected predicate to the production full-signature execution surface.

AP2-N003 full-execution proof-shell hardening note: `path2_full_signature_execution_source_instantiated_v1` now rejects global `def`/`abbrev` aliases that set `selectedRustFullExecutionSourceVerifiedProof` or `selectedRustFullExecutionFactsAllTrueProof` to `True.intro`, and aliases that set `selectedRustExecutionSucceeds` or `selectedRustProductionExecutionSucceeds` to `fun _ => False`. Regressions prepend those aliases to the otherwise accepted full-execution theorem shape and confirm the gate stays open.

AP2-N003 full-boundary assumption gate note: `path2_full_signature_execution_source_instantiated_v1` now rejects `concreteFullSignatureExecutionSource_instantiation_v1` theorem headers that assume `NativeFullSignatureSecurityBoundary`, `NativeAcceptedSpendSoundnessBoundary`, `SelectedRustAcceptedSpendSoundnessCertificate`, or `Path2FullSpendGame.FullForgeryToAcceptedSpendSoundness`. This prevents the execution-source instantiation from satisfying the required full-boundary adapter markers by importing the active full-signature boundary, accepted-spend numeric package, or full-forgery soundness package as a parameter.

AP2-N003 accepted-spend numeric-bound preflight note: `path2_native_deployment_preflight_v1` now has `accepted_spend_numeric_bound_instantiated`, sourced by the future `selectedRustAcceptedSpendSoundnessBoundary_instantiation_v1` theorem. The gate strips Lean comments and string literals before scanning, requires a locally constructed `SelectedRustAcceptedSpendSoundnessCertificate` with proof-bearing selected-Rust bound verification, and rejects assumed boundary/certificate/bound facts, multiline declaration/variable stubs, comment/string marker spoofing, plus vacuous `True` verification. Until that theorem is present, production readiness and `proof_side_public_boundary_preflight_complete` remain false with `PATH2_NATIVE_DEPLOYMENT_BLOCKER_ACCEPTED_SPEND_NUMERIC_BOUND`, preventing arbitrary `acceptedSpendForgeryBound` / `acceptedSpendForgery_le_bound` fields from closing the final full-signature theorem.

AP2-N003 accepted-spend evidence-report note: `SelectedRustAcceptedSpendSoundnessCertificate` now carries a concrete `SelectedRustAcceptedSpendSoundnessEvidenceReport` plus proofs of `ReportVerified` and `FactsAllTrue`, rather than an arbitrary `selectedRustAcceptedSpendBoundVerified : Prop`. The report records schema/digest verification, accepted-spend numeric-bound pricing, finite trace-support pricing, active outcome-carrier pinning, active break-family pinning, accepted-spend union-bound pricing, and the four prerequisite accepted-spend closure facts. The numeric-bound instantiation gate now requires these report-field markers plus `evidenceReport := selectedRustAcceptedSpendEvidenceReport`, so the future theorem must bind the selected probability/bound route to report-shaped evidence before constructing `NativeAcceptedSpendSoundnessBoundary`.

AP2-N003 accepted-spend constructor hardening note: `NativeAcceptedSpendSoundnessBoundary` and `SelectedRustAcceptedSpendSoundnessCertificate` now carry named production probability/bound fields plus equality proofs pinning the exported accepted-spend probability and deployment bound to those production values. The selected certificate also carries `evidenceDigest` and exposes the named constructor `SelectedRustAcceptedSpendSoundnessCertificate.ofReportAndBound`. The numeric-bound future-instantiation gate requires that constructor, `selectedRustAcceptedSpendEvidenceDigest`, `selectedRustProductionAcceptedSpendForgeryProb`, `selectedRustProductionAcceptedSpendForgeryBound`, the probability/bound pin proofs, and the four named prerequisite closure inputs `selectedRustAcceptedSpendHiddenSourceClosed`, `selectedRustAcceptedSpendCheckedCoreClosed`, `selectedRustAcceptedSpendLifecycleClosed`, and `selectedRustAcceptedSpendNoExportClosed`; rejects theorem/header/declaration assumptions of those inputs with the report and bound facts; and no longer treats a raw certificate record literal, constructor with `True` closure arguments, or zero probability/bound placeholders as the positive AP2-N003 numeric-bound shape. This keeps the probability-bound handoff fail-closed until the selected accepted-spend evidence report, digest, production numeric values, inequality proof, and prerequisite closure facts are bound together.

AP2-N003 accepted-spend dependency-direction note: the accepted-spend numeric-bound instantiation gate now scans the combined account-design, hidden-source boundary, checked-core byte bridge, full-spend concrete, and concrete no-export sources. A numeric-certificate-only theorem shape is rejected unless it also consumes `reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_certificate`, `selectedRustCheckedCoreByteSource_instantiation_v1`, `accepted_spend_with_selected_rust_checked_core_closure_closed`, `concreteFullSignatureExecutionSource_instantiation_v1`, `concrete_fullExecutionAcceptedSpendLift`, `selectedRustActiveNoExportSerializer_instantiation_v1`, and `activeSerializerAllowlist_boundaryNoExportClosed`. This prevents accepted-spend soundness from closing in the upstream numeric layer with local prerequisite propositions instead of the active hidden-source, checked-core, lifecycle, and no-export closures.

AP2-N003 accepted-spend alias hardening note: the numeric-bound instantiation gate now rejects whitespace-varied local or global aliases that define the selected accepted-spend probability/bound as `0`, pin proofs as `rfl`/`le_rfl`, evidence digests as `ByteArray.empty`, or prerequisite closure names as bare `True`. The regression covers both theorem-local `let` aliases and top-level `def` aliases before the theorem. This keeps the gate open until a future theorem consumes real production accepted-spend numeric evidence, rather than a zero-bound shell hidden behind selected names.

AP2-N003 accepted-spend probability-layer note: `Descender/Path2AcceptedSpendProbability.lean` now supplies `AcceptedSpendNumericGame`, `successProb_le_acceptedSpendOrBreakUnionBound`, `AcceptedSpendBreakPricingCertificate`, `PricedAcceptedSpendNumericCertificate`, and the derived `unionBound_le_priced` proof. The pricing certificate records the accepted-spend bound, one bound per active named break, and the final budget inequality from their finite sum to the production-priced bound; `PricedAcceptedSpendNumericCertificate.unionBound_le_priced` is derived from that package rather than stored as one opaque real inequality. The source gate now requires those markers, so the active numeric theorem cannot close solely by constructing `SelectedRustAcceptedSpendSoundnessCertificate` from raw `ℝ` fields.

AP2-N003 accepted-spend pricing-evidence split note: the Lean/Rust accepted-spend evidence report now separates aggregate union-bound pricing from the three constructive pricing subclaims required by `AcceptedSpendBreakPricingCertificate`: `acceptedSpendEventBoundPriced`, `namedBreakEventBoundsPriced`, and `breakPricingBudgetMeetsPricedBound` (mirrored in Rust as `accepted_spend_event_bound_priced`, `named_break_event_bounds_priced`, and `break_pricing_budget_meets_priced_bound`). The canonical report remains fail-closed with all three false, so AP2-N003 still requires real production-derived accepted-spend and named-break bounds plus the finite-sum budget check before the selected theorem can instantiate.

AP2-N003 accepted-spend pricing-projection note: `SelectedRustAcceptedSpendSoundnessEvidenceReport` now exposes named `FactsAllTrue` projection theorems for aggregate union-bound pricing, accepted-spend event-bound pricing, named-break bound pricing, and break-pricing budget closure. `SelectedRustAcceptedSpendSoundnessCertificate.acceptedSpendForgeryProb_le_bound` and the boundary erasure route consume those projections, and the Rust source gate exact-checks the projection theorem names. This keeps the future theorem from hiding the constructive pricing subclaims inside one opaque `FactsAllTrue` proof term.

AP2-N003 accepted-spend readiness-projection note: the same report now exposes and the certificate route consumes named projections for numeric-bound pricing, finite trace support, active outcome-carrier pinning, accepted-spend event pinning, active break-family pinning, named-break event pinning, numeric-budget target readiness, selected QROM loss, and the hidden-source/checked-core/lifecycle/no-export prerequisite closure readiness bits. The source gate exact-checks those theorem names, so the future selected proof must surface every active-path readiness fact rather than only passing an opaque `FactsAllTrue` witness.

AP2-N003 accepted-spend report-verification projection note: `SelectedRustAcceptedSpendSoundnessEvidenceReport.ReportVerified` now has named projections for schema match, report-digest verification, and numeric-evidence digest verification. The selected accepted-spend certificate consumes them and the source gate exact-checks the projection names, so the future theorem cannot hide report identity/digest binding inside an opaque `ReportVerified` proof alone.

AP2-N003 accepted-spend component-bound note: `PricedAcceptedSpendNumericCertificate` and `SelectedAcceptedSpendNumericSource` now expose theorem projections for the accepted-spend component bound, per-break component bounds, and finite component-budget-to-priced-bound inequality. The accepted-spend source gate requires and exact-checks those projection names, keeping the constructive `AcceptedSpendBreakPricingCertificate` evidence visible after the selected numeric source is packaged.

AP2-N003 concrete component-bound note: `SelectedRustConcreteAcceptedSpendNumericCertificate` now mirrors the accepted-spend component-bound, per-break component-bound, and break-budget projection theorems before erasing to the generic selected numeric source. The accepted-spend source gate exact-checks these concrete-wrapper projections, so the final theorem must expose constructive pricing through the active outcome carrier and break-family wrapper.

AP2-N003 accepted-spend certificate-surface note: `SelectedRustAcceptedSpendSoundnessCertificate` now carries `SelectedAcceptedSpendNumericSource` plus proofs that the exported accepted-spend probability and bound match that source's priced certificate union expression and priced bound. `toAcceptedSpendSoundnessBoundary` derives `acceptedSpendForgery_le_bound` from the packaged `unionBound_le_priced` proof through the source's priced certificate; the selected constructor no longer accepts a standalone `selectedRustAcceptedSpendForgeryLeBound` inequality proof. The source gate now requires the numeric source, concrete wrapper route, and match proofs.

AP2-N003 finite-trace/outcome split note: `AcceptedSpendNumericGame` now samples a finite `Trace` support and observes an arbitrary `Outcome` through `observe`. `PricedAcceptedSpendNumericCertificate` carries `traceFintype` rather than requiring `Fintype Outcome`, and the accepted-spend source gate requires the trace-support and observation-map markers. This keeps bounded-game probability accounting finite without falsely requiring the selected concrete full-forgery run carrier to be finite.

AP2-N003 active-break-family note: `PricedAcceptedSpendNumericCertificate` now pins its finite break index to `Path2FullSpendGame.BreakEvent` through `activeBreakFamilyPinned`, with theorem `breakFamily_eq_activeFullSpendBreakEvent`. The accepted-spend source gate requires those markers, preventing the future numeric instantiation from pricing an implementation-chosen or proof-convenient break family instead of the active full-spend break family.

AP2-N003 active-outcome-carrier note: `Descender/Path2FullSpendConcrete.lean` now exposes `selectedRustAcceptedSpendNumericOutcome` as the concrete `Path2FullSpendGame.FullForgeryRun concreteTypes` carrier and wraps the priced numeric certificate in `SelectedRustConcreteAcceptedSpendNumericCertificate`. The wrapper pins the priced certificate's outcome carrier to the selected concrete full-forgery run type and provides a named forgetful map back to `PricedAcceptedSpendNumericCertificate`; the accepted-spend source gate now requires those markers before a future numeric theorem can close.

AP2-N003 accepted-spend event-semantics note: `PricedAcceptedSpendNumericCertificate` now exposes stored-instance accessors for observed outcomes, accepted-spend events, and named break events. `SelectedRustConcreteAcceptedSpendNumericCertificate` must prove that every priced accepted-spend event over an observed trace implies the active `Path2FullSpendGame.AcceptedSpendConclusion` on the selected concrete run, and that every priced named-break event implies the active concrete `breakFor` predicate after transporting the break index through the pinned `Path2FullSpendGame.BreakEvent` equality. The source gate now requires the report bits and active-soundness theorem markers for these predicate pins, so a future numeric theorem cannot price arbitrary Boolean events over the right carrier and break-index type.

AP2-N003 accepted-spend Rust-evidence note: `tools/parameter-search/src/production_reference.rs` now exposes `Path2NativeAcceptedSpendNumericEvidenceV1` with a length-delimited frame, digest, report-verification helper, all-facts helper, verifier, and production-facade exports. The current canonical report is intentionally fail-closed: schema/digest verification passes and already-closed hidden-source, checked-core, lifecycle, and no-export closure bits are true, but numeric-bound pricing, finite trace support, active outcome/event/break pins, and union-bound pricing remain false. A regression proves that flipping those readiness bits in a copied report does not satisfy `all_facts_true` or verification, and the accepted-spend numeric instantiation gate now first requires this Rust evidence source to be present before scanning for the future Lean theorem.

AP2-N003 accepted-spend evidence-digest note: the accepted-spend numeric evidence report now carries public digest commitments for finite trace support, active outcome carrier, accepted-spend event semantics, active break family, named break event semantics, union-bound pricing, and the priced bound. `SelectedRustAcceptedSpendSoundnessEvidenceReport` mirrors these digest fields and has an explicit `numericEvidenceDigestsVerified` bit in `ReportVerified`; Rust mirrors that as `numeric_evidence_digests_verified`. The source gate requires their Lean/Rust markers before the future numeric theorem can close. Report verification rejects zero digest placeholders, digest tampering, and a false digest-verification bit, so the future proof route has concrete public commitments to bind instead of only Boolean readiness flags.

AP2-N003 accepted-spend source-window digest note: the canonical Rust accepted-spend numeric evidence now fills its seven digest slots from source windows in `Path2AcceptedSpendProbability.lean`, `Path2FullSpendConcrete.lean`, and `Path2FullSpendGame.lean` instead of hashing placeholder strings. The source windows cover the priced certificate/finite trace support, active full-forgery outcome carrier, concrete accepted-spend event semantics, active break-family enumeration, named-break event semantics, union-bound expression, and priced-bound theorem surface. Readiness bits remain false until real numeric pricing is instantiated.

AP2-N003 accepted-spend numeric-budget note: the accepted-spend numeric evidence report now carries public numeric budget fields for target security bits, selected multi-target horizon, QROM loss, transcript term after QROM loss, and the claimed priced-bound security bits, plus `acceptedSpendNumericBudgetMeetsTarget`. `FactsAllTrue` pins the active constants `targetSecurityBits = 128`, `selectedMultiTargetHorizonLog2 = 32`, `qromQuadraticLossBitsAtDeploymentBound = 32`, and `transcriptTermBitsAfterQromLoss = 182`, then requires the target predicate and the inequalities `targetSecurityBits ≤ acceptedSpendPricedBoundSecurityBits`, `targetSecurityBits + selectedMultiTargetHorizonLog2 ≤ transcriptTermBitsAfterQromLoss`, and `acceptedSpendPricedBoundSecurityBits + selectedMultiTargetHorizonLog2 ≤ transcriptTermBitsAfterQromLoss`; Rust mirrors those checks in `path2_native_accepted_spend_numeric_evidence_all_facts_true`. The current canonical Rust evidence keeps the priced-bound bits at `0` and the target predicate false, so it remains fail-closed.

AP2-N003 accepted-spend transcript-cap note: `SelectedRustAcceptedSpendSoundnessEvidenceReport.factsAllTrue_pricedBoundSecurityBits_le_transcriptCap` now names the selected-budget ceiling `acceptedSpendPricedBoundSecurityBits ≤ 150`, derived from the 182-bit QROM-discounted transcript term and 32-bit active multi-target horizon. The accepted-spend certificate path consumes this cap and the Rust source gate requires the projection, so a future theorem cannot silently overclaim accepted-spend priced-bound bits beyond the active transcript budget.

AP2-N003 accepted-spend budget-arithmetic note: `SelectedRustAcceptedSpendSoundnessEvidenceReport` now exposes theorem facts for priced-bound bits meeting target, the selected 32-bit active multi-target horizon fitting inside the 182-bit QROM-discounted transcript term, the claimed priced-bound bits also fitting inside that transcript term with the selected horizon, and the resulting 22-bit target arithmetic slack. `SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary` consumes these facts before deriving the boundary, and the accepted-spend instantiation source gate requires the future selected theorem to consume them too. This narrows the remaining numeric-bound gap to the real-valued accepted-spend probability source and priced-bound inequality.

AP2-N003 accepted-spend priced-bound real-value note: `SelectedRustAcceptedSpendSoundnessCertificate` now carries `acceptedSpendPricedBoundSecurityPriced`, proving the actual real-valued `numericSource.pricedCertificate.pricedBound` is at most `((2 : ℝ) ^ evidenceReport.acceptedSpendPricedBoundSecurityBits)⁻¹`. The numeric-bound source gate requires a named `selectedRustAcceptedSpendPricedBoundSecurityPriced` proof and rejects `le_rfl` aliases, so the security-bit report cannot float independently of the priced probability bound.

AP2-N003 accepted-spend equality-proof hardening note: the numeric-bound source gate now rejects local or global `rfl` aliases for `selectedRustAcceptedSpendForgeryProbMatchesUnionBound` and `selectedRustAcceptedSpendForgeryBoundMatchesPriced`. The future theorem must supply nontrivial selected proofs that the exported accepted-spend probability/bound are the concrete numeric source's union bound and priced bound, rather than making the selected values definitionally equal by local shelling.

AP2-N003 accepted-spend report/digest binding note: `SelectedRustAcceptedSpendSoundnessCertificate` now carries proof fields for both `evidenceReportBound` and `evidenceDigestBound`, and `toAcceptedSpendSoundnessBoundary` consumes those proofs before deriving the accepted-spend inequality. The numeric-bound source gate requires named `selectedRustAcceptedSpendEvidenceReportBound` and `selectedRustAcceptedSpendEvidenceDigestBound` propositions plus proofs, and rejects `True` aliases for those bindings. This prevents a future theorem from passing an arbitrary report/digest pair alongside otherwise valid numeric fields.

AP2-N003 accepted-spend report-proof hardening note: the numeric-bound source gate now also rejects global `def`/`abbrev` aliases that set `selectedRustAcceptedSpendEvidenceReportBoundProof`, `selectedRustAcceptedSpendEvidenceDigestBoundProof`, `selectedRustAcceptedSpendBoundVerifiedProof`, or `selectedRustAcceptedSpendFactsAllTrueProof` to `True.intro`. Regressions prepend those aliases to the otherwise accepted theorem shape and confirm the active accepted-spend gate stays open, so report/digest binding, report verification, and all-facts closure must come from the selected evidence report rather than proof-name shells.

AP2-N003 numeric-source handoff note: `Path2AcceptedSpendProbability.lean` now exposes `SelectedAcceptedSpendNumericSource`, a proof-bearing generic source package for a priced accepted-spend numeric certificate. `Path2FullSpendConcrete.lean` maps `SelectedRustConcreteAcceptedSpendNumericCertificate` into that package with `toSelectedAcceptedSpendNumericSource`, and `SelectedRustAcceptedSpendSoundnessCertificate` now consumes this numeric source rather than a bare priced certificate. This closes the previous type-surface gap where the Rust source gate required concrete wrapper markers, but the Lean accepted-spend certificate itself could still be built from an arbitrary generic priced certificate.

AP2-N003 selected-source probability-bound note: `SelectedAcceptedSpendNumericSource` now exposes theorem projections for accepted-spend probability below the priced bound, full-forgery success below the union bound, full-forgery success below the priced bound, and union-bound pricing. `SelectedRustAcceptedSpendSoundnessCertificate.toAcceptedSpendSoundnessBoundary` now uses the selected source's union-bound pricing proof, and the source gate requires future theorem text to consume these selected-source projections.

AP2-N003 certificate inequality note: `SelectedRustAcceptedSpendSoundnessCertificate` now exposes `acceptedSpendForgeryProb_le_bound`, deriving the exported accepted-spend probability inequality from the selected numeric source, report/digest witnesses, active budget facts, production pinning, and priced-bound equality. `toAcceptedSpendSoundnessBoundary` consumes that theorem, and the source gate requires the future selected theorem to mention it before erasing to `NativeAcceptedSpendSoundnessBoundary`.

AP2-N003 reported-security theorem note: `SelectedRustAcceptedSpendSoundnessCertificate` now exposes `acceptedSpendForgeryBound_le_reportedSecurity`, `acceptedSpendForgeryProb_le_reportedSecurity`, target-security variants, `acceptedSpendForgeryProb_le_active128BitTarget`, and the same fixed-target theorem after erasing to `NativeAcceptedSpendSoundnessBoundary`, tying the exported bound and probability to `((2 : ℝ) ^ acceptedSpendPricedBoundSecurityBits)⁻¹`, then to the pinned `targetSecurityBits = 128`. The source gate requires the future theorem to consume these certificate projections, so the reported security bits and final active target remain theorem-visible after the selected numeric source is packaged.

AP2-N003 certificate equality-projection note: `SelectedRustAcceptedSpendSoundnessCertificate` now exposes named theorem projections for the exported probability equaling the selected numeric source union bound, the exported bound equaling the selected priced bound, and both exported values matching production values. The certificate inequality proofs consume these projections, and the source gate requires future theorem text to mention them instead of relying only on raw record-field equalities.

AP2-N003 exact-projection scanner note: the accepted-spend numeric-bound and full-reduction instantiation gates now exact-check qualified Lean identifier references for their certificate projection markers. Focused regressions replace the active `...acceptedSpendForgeryProb_le_active128BitTarget` and `...signatureForgerySuccessProb_le_active128BitTarget` projection names with suffixed spoof names and keep both gates open, so substring matches cannot satisfy the final target-bound route.

AP2-N003 numeric-source consumption note: the accepted-spend numeric-bound source gate now requires the future theorem to construct `selectedRustConcreteAcceptedSpendNumericSourceCertificate` with `SelectedRustConcreteAcceptedSpendNumericSourceCertificate.ofReportAndConcreteCertificate`, then bind `selectedRustConcreteAcceptedSpendNumericCertificate` and `selectedRustAcceptedSpendNumericSource` through that source certificate's concrete and selected projections. A regression rejects theorem text that merely mentions the concrete conversion under another name while passing a generic source into `SelectedRustAcceptedSpendSoundnessCertificate.ofReportAndBound`.

AP2-N003 concrete probability-bound note: `SelectedRustConcreteAcceptedSpendNumericCertificate` now exposes theorem projections for accepted-spend probability below the priced bound, full-forgery success below the union bound, and full-forgery success below the priced bound. The numeric-bound source gate requires the future selected theorem to consume those concrete-wrapper projections, keeping the probability-bound route tied to the active full-forgery outcome carrier before the generic priced certificate is erased.

AP2-N003 concrete numeric-source hardening note: the accepted-spend numeric-bound source gate now rejects global `def`/`abbrev` record literals for `selectedRustConcreteAcceptedSpendNumericCertificateSource`, `selectedRustConcreteAcceptedSpendNumericSourceCertificate`, or `selectedRustConcreteAcceptedSpendNumericCertificate`. The focused regression prepends such a source to the otherwise accepted theorem shape and keeps the gate open, so the concrete active-carrier wrapper must come from a named production-derived source route rather than a top-level anonymous certificate literal.

AP2-N003 local concrete-source hardening note: the same gate now also rejects theorem-local `let selectedRustConcreteAcceptedSpendNumericCertificateSource := { ... }` and `let selectedRustConcreteAcceptedSpendNumericSourceCertificate := { ... }` / tuple-literal source construction. The regressions insert local concrete numeric source literals immediately before the active bindings and confirm the gate remains open, so the future proof cannot fabricate the production-derived numeric source inside the final theorem body.

AP2-N003 report-bound concrete numeric-source note: `Descender/Path2FullSpendConcrete.lean` now exposes `SelectedRustConcreteAcceptedSpendNumericSourceCertificate`, carrying the accepted-spend evidence report, digest, report/digest binding proofs, `ReportVerified`, `FactsAllTrue`, and the concrete active-carrier numeric certificate. The source gate requires the future selected theorem to construct this source certificate and extract both the concrete wrapper and generic selected source from it, tying the accepted-spend numeric handoff to the same report/digest route as the selected soundness certificate.

AP2-N003 concrete source split-constructor note: `SelectedRustConcreteAcceptedSpendNumericSourceCertificate` now exposes `ofReportAndConcreteCertificateSplitFacts` and the active selected-report constructor `ofSelectedReportAndConcreteCertificatePricingFacts`. The selected constructor uses the current selected report verification and structural facts internally, so the future accepted-spend numeric source package only has to provide the hard `PricingFactsComplete` proof plus the concrete numeric certificate and digest bindings.

AP2-N003 concrete pricing-witness note: `SelectedRustConcreteAcceptedSpendPricingWitness` now packages the concrete accepted-spend numeric certificate, the real priced-bound security inequality, the selected report pricing bits, and all digest bindings to the selected public evidence report. It derives both the active `PricingFactsComplete` proof and the report-bound concrete numeric source certificate, so the accepted-spend hard half is now a single proof object tying real probability pricing to the report bits.

AP2-N003 concrete numeric digest-binding note: `SelectedRustConcreteAcceptedSpendNumericCertificate` now carries concrete digest bytes for each accepted-spend numeric evidence slot: finite trace support, active outcome carrier, accepted-spend event semantics, active break family, named break events, union-bound pricing, and priced-bound pricing. `SelectedRustConcreteAcceptedSpendNumericSourceCertificate` now requires equality proofs that those certificate digest bytes match the corresponding accepted-spend evidence-report digest slots, and exposes projection theorems for all seven equalities. The source gate requires the future theorem to pass named equality proofs, exact-checks the projection theorem references, and rejects header/declaration/local trivial aliases for those witnesses. This does not instantiate production hash computation yet, but it prevents the final theorem shape from carrying a verified report beside an unrelated concrete numeric certificate.

AP2-N003 accepted-spend report digest-presence note: `SelectedRustAcceptedSpendSoundnessEvidenceReport.ReportVerified` now includes nonzero `ByteArray` requirements for all seven accepted-spend numeric digest slots, matching the Rust verifier's fail-closed rejection of placeholder digest slots. The source gate exact-checks the seven new `reportVerified_*_nonzero` projection theorems in the future accepted-spend theorem shape, so report verification cannot be represented in Lean solely by three true Boolean flags.

AP2-N003 concrete digest-presence projection note: `SelectedRustConcreteAcceptedSpendNumericSourceCertificate` now derives concrete-certificate digest nonzero theorems from the report-verification nonzero facts plus the seven report/certificate digest equalities. The accepted-spend source gate exact-checks those source-certificate projections, so the future theorem must pull digest presence through to the concrete active-carrier numeric certificate rather than only proving report-side digest presence.

AP2-N003 source-bound fact projection note: `SelectedRustConcreteAcceptedSpendNumericSourceCertificate` now exposes report-fact projections for the accepted-spend numeric bound, finite trace support, active outcome carrier, accepted-spend event, active break family, named break events, union-bound pricing, accepted-spend component pricing, named-break component pricing, break-pricing budget, and target budget. The accepted-spend numeric-bound source gate exact-checks those source-certificate projection names, so the future theorem must pull the remaining numeric readiness facts through the same report-bound concrete source package rather than only mentioning report-level `FactsAllTrue` projections. A focused regression replaces the concrete-source union-bound fact with the report-level projection and confirms the gate remains open.

AP2-N003 numeric sign/probability-range note: `AcceptedSpendNumericGame`, `AcceptedSpendBreakPricingCertificate`, `PricedAcceptedSpendNumericCertificate`, and `SelectedAcceptedSpendNumericSource` now expose nonnegativity theorems for the full-signature success probability, accepted-spend union bound, accepted-spend component bound, each named-break component bound, and final priced bound. The actual event probabilities also carry explicit `≤ 1` facts for success, accepted-spend, and named-break events through the generic game, priced certificate, and selected source. `SelectedRustConcreteAcceptedSpendNumericCertificate` and `SelectedRustConcreteAcceptedSpendNumericSourceCertificate` project the same sign facts plus those actual-probability `≤ 1` facts through the active concrete carrier/source route. `SelectedRustAcceptedSpendSoundnessCertificate` proves the exported accepted-spend probability and bound remain nonnegative after erasure to `NativeAcceptedSpendSoundnessBoundary`, and exposes packaged success probability `≤ 1`; `SelectedRustFullSignatureReductionCertificate` proves selected/production signature-forgery success probabilities plus the erased `NativeFullSignatureSecurityBoundary.signatureForgerySuccessProb` are nonnegative and at most one. The accepted-spend and full-reduction source gates exact-check those theorem names, so the future pricing package cannot expose only upper-bound inequalities while leaving probability/budget signs implicit or losing actual probability-range facts before the concrete/full-boundary handoff. This does not claim every derived union/priced budget is `≤ 1`; it preserves the range facts only for real event probabilities.

AP2-N003 selected evidence binding note: the accepted-spend numeric and full-signature reduction Lean certificates now bind their evidence reports and evidence digests by equality to concrete selected Rust evidence report/digest constants, rather than carrying arbitrary `Prop` binding slots. The selected AP2-N003 reports mirror the Rust public evidence objects and remain fail-closed on `FactsAllTrue`, while `ReportVerified` is provable from concrete nonzero digest slots. This turns the report/digest binding from scanner-only discipline into a type-level requirement for future theorem constructors.

AP2-N003 current-report sanity note: Lean now proves `selectedRustAcceptedSpendEvidenceReportFactsAllTrue_impossible` and `selectedRustFullSignatureReductionFactsAllTrue_impossible`, making the current selected public AP2-N003 reports explicitly verified-but-not-complete. Lean also exposes certificate-level contradictions for the current selected constants: `SelectedRustAcceptedSpendSoundnessCertificate.currentSelectedReport_impossible`, `SelectedRustFullSignatureReductionCertificate.currentSelectedReport_impossible`, and `SelectedRustConcreteAcceptedSpendNumericSourceCertificate.currentSelectedReport_impossible`. Closing AP2-N003 therefore requires updating the selected Rust evidence facts/proofs, not reusing the current report constants as complete evidence or wrapping them in a selected certificate.

AP2-N003 blocker classification note: Lean now exposes diagnostic facts separating active budget capacity and structural selected evidence from missing production probability evidence. The current accepted-spend report proves the target plus 32-bit multi-target horizon fits inside the 182-bit QROM-discounted transcript term, carries the finite trace support, active outcome/event/break-family pins, named-break pin, and accepted-spend union-bound pricing as true, but still proves the selected priced-bound bits are `0`, below the 128-bit target, and that accepted-spend event component pricing, named-break component pricing, break-pricing budget, aggregate numeric-bound pricing, and target budget remain false. The current full-reduction report now carries full-execution-source consumption, active hash-boundary consumption, and multi-target horizon pricing as true, while production signature-probability pinning, signature-success equality to the accepted-spend game, accepted-spend probability linkage, all active query-symbol pricing in the final expression, and verifier-game binding to production execution remain false. This keeps the priority clear: the remaining redesign-risk items are the real accepted-spend event/named-break probability pricing and final production reduction/linkage facts, not the already-selected structural pins.

AP2-N003 completeness-split theorem note: `SelectedRustAcceptedSpendSoundnessEvidenceReport` now factors `FactsAllTrue` into `StructuralFactsComplete ∧ PricingFactsComplete`, and proves the iff as `factsAllTrue_iff_structural_and_pricing`. `SelectedRustFullSignatureReductionEvidenceReport` likewise factors `FactsAllTrue` into `StructuralFactsComplete ∧ ProductionFactsComplete`, with `factsAllTrue_iff_structural_and_production`. This makes the remaining proof target explicit: accepted-spend closure needs the pricing half, while full-reduction closure needs the production half after the structural half is already selected.

AP2-N003 current-half obstruction note: Lean now proves the current selected accepted-spend report satisfies `StructuralFactsComplete` but cannot satisfy `PricingFactsComplete`; it also proves the current selected full-reduction report satisfies `StructuralFactsComplete` but cannot satisfy `ProductionFactsComplete`. This pins the active blocker to the missing pricing and production halves, rather than to report parsing, digest presence, QROM capacity, or structural source consumption.

AP2-N003 hard-half projection note: the split halves now expose named Lean projections. `PricingFactsComplete` projects the aggregate numeric-bound pricing bit, accepted-spend component pricing, named-break component pricing, break-budget pricing, numeric budget target bit, target-security inequality, and transcript-cap inequality. `ProductionFactsComplete` projects production signature-probability pinning, signature-to-accepted-spend-game equality, accepted-spend probability linkage, all-active-query-symbol pricing, and verifier-game-to-production-execution binding. Lean also exposes `factsAllTrue_of_structural_and_pricing` and `factsAllTrue_of_structural_and_production`, so the final theorem can prove the hard half separately and then reassemble the monolithic constructor input without unpacking tuple positions.

AP2-N003 current-report hard-half handoff note: Lean now exposes `selectedRustAcceptedSpendEvidenceReportFactsAllTrue_of_currentPricingFactsComplete` and `selectedRustFullSignatureReductionEvidenceReportFactsAllTrue_of_currentProductionFactsComplete`. These lemmas reuse the already-proved current structural halves for the selected AP2-N003 reports, so future closing work can supply only `PricingFactsComplete` for accepted spend and `ProductionFactsComplete` for the full reduction before reassembling the monolithic `FactsAllTrue` inputs.

AP2-N003 certificate-level hard-half constructor note: `SelectedRustAcceptedSpendSoundnessCertificate.ofSelectedReportAndBoundPricingFacts` and `SelectedRustFullSignatureReductionCertificate.ofSelectedReportAndReductionProductionFacts` now route the active selected reports through the hard-half proofs directly. Once production supplies `PricingFactsComplete` and `ProductionFactsComplete`, the certificate constructors obtain report verification and structural facts from the selected constants instead of requiring theorem code to hand over opaque monolithic `FactsAllTrue` witnesses.

AP2-N003 full-reduction production-witness note: `SelectedRustFullSignatureReductionProductionWitness` now packages the selected accepted-spend certificate, active hash boundary, production signature-probability pin, accepted-spend-game equality, keygen/signing/completeness/verifier/execution obligations, and the selected full-reduction production report bits. It derives both `ProductionFactsComplete` and the selected full-signature reduction certificate, tying the full-reduction hard half to the actual reduction data.

AP2-N003 source-gate witness-route note: the Rust future-theorem scanners now require the accepted-spend theorem shape to mention `SelectedRustConcreteAcceptedSpendPricingWitness` and its pricing/source projections, and the full-reduction theorem shape to mention `SelectedRustFullSignatureReductionProductionWitness`, its `ProductionFactsComplete` projection, its certificate projection, and the selected hard-half constructor. This keeps the witness routes on the required proof path.

AP2-N003 witness-projection note: the accepted-spend pricing witness now exposes the concrete accepted-spend component bound, named-break component bounds, component-budget bound, and full-success-to-priced-bound inequality. The full-reduction production witness now exposes the production probability pin, accepted-spend-game equality, accepted-spend certificate probability bound, query-budget pricing, and multi-target pricing. The Rust source gates require these projection names so future theorem text cannot ignore the witness payload.

AP2-N003 pricing-witness route note: `SelectedRustConcreteAcceptedSpendPricingWitness` now has direct adapters to the selected accepted-spend certificate and erased accepted-spend boundary. The accepted-spend source gate requires those adapters, `SelectedRustConcreteAcceptedSpendNumericSourceCertificate.ofSelectedReportAndConcreteCertificatePricingFacts`, and `SelectedRustAcceptedSpendSoundnessCertificate.ofSelectedReportAndBoundPricingFacts`, so the future theorem must route through the selected-report hard-half constructors rather than only mentioning the generic report-and-bound constructors.

AP2-N003 pricing-witness target note: `SelectedRustConcreteAcceptedSpendPricingWitness` now derives concrete priced-bound-to-target, priced-bound-to-active-128-bit, and success-probability-to-active-128-bit projections from its real priced-bound inequality plus the selected report pricing half. The accepted-spend source gate requires these names so the eventual pricing witness must expose the actual 128-bit conclusion before certificate erasure.

AP2-N003 union-decomposition note: `AcceptedSpendNumericGame`, `PricedAcceptedSpendNumericCertificate`, `SelectedAcceptedSpendNumericSource`, `SelectedRustConcreteAcceptedSpendNumericCertificate`, `SelectedRustConcreteAcceptedSpendNumericSourceCertificate`, and `SelectedRustConcreteAcceptedSpendPricingWitness` now expose the named equality that the accepted-spend union expression is the accepted-spend probability plus the finite sum of named-break probabilities. The accepted-spend source gate requires these identifiers, so the remaining concrete pricing proof must target the exact active accepted-plus-break decomposition rather than an opaque aggregate event.

AP2-N003 component-budget note: `PricedAcceptedSpendNumericCertificate`, `SelectedAcceptedSpendNumericSource`, `SelectedRustConcreteAcceptedSpendNumericCertificate`, `SelectedRustConcreteAcceptedSpendNumericSourceCertificate`, and `SelectedRustConcreteAcceptedSpendPricingWitness` now expose `unionBound_le_componentBudget` and `successProb_le_componentBudget`. The accepted-spend source gate requires those projections, so the final pricing proof must explicitly pass through component-wise accepted-spend and named-break bounds before using the final priced-bound budget.

AP2-N003 witness fail-closed note: Lean now proves the current selected accepted-spend report cannot produce `SelectedRustConcreteAcceptedSpendPricingWitness`, and the current selected full-reduction report cannot produce `SelectedRustFullSignatureReductionProductionWitness`. The Rust source gates require these impossibility theorem names, so retiring the fail-closed state must happen by changing the selected evidence facts and not by bypassing the witness objects.

AP2-N003 pricing-witness active-target note: `SelectedRustConcreteAcceptedSpendPricingWitness` now projects `componentBudget_le_active128BitTarget` and `unionBound_le_active128BitTarget` in addition to the priced-bound and success-probability active-target facts. The accepted-spend source gate requires these projections, so the final witness must expose the fixed-target consequence at each budget layer: component sum, union expression, and success probability.

AP2-N003 component-probability target note: the accepted-spend pricing layer now exposes component-budget inclusion for the accepted-spend bound and each named-break bound, plus derived bounds from the actual accepted-spend probability and each actual named-break probability to the component budget. `SelectedRustConcreteAcceptedSpendPricingWitness` further derives `acceptedSpendProb_le_active128BitTarget` and `breakProb_le_active128BitTarget` from the selected pricing half. The active source gate exact-checks these theorem names, so the remaining production pricing package must prove not just an aggregate budget but the concrete active accepted-spend event and every named-break probability against the 128-bit target route.

AP2-N003 accepted-spend certificate component-route note: `SelectedRustAcceptedSpendSoundnessCertificate` now names its carried component budget and projects the exported accepted-spend probability through that component budget before the priced bound. It also derives component-budget, exported-bound, and packaged-success fixed 128-bit target facts. The accepted-spend source gate requires these names, so the full-reduction handoff cannot collapse the accepted-spend certificate to only an opaque final priced inequality.

AP2-N003 full-reduction component-route note: `SelectedRustFullSignatureReductionCertificate` and `SelectedRustFullSignatureReductionProductionWitness` now expose `signatureForgerySuccessProb_le_componentBudget`, routing selected full-signature success through the accepted-spend certificate's component budget before the exported accepted-spend bound and fixed target. The full-reduction source gate requires these names, so the production reduction path must keep the accepted-spend component decomposition visible through the final signature theorem surface.

AP2-N003 production-witness obligation projection note: `SelectedRustFullSignatureReductionProductionWitness` now exposes named projections for the selected key-generation, signing-oracle, completeness, verifier-relation, full-forgery-to-accepted-spend, checked-core domain-separation, and hidden-source domain-separation obligations. The full-reduction source gate requires these projection names along with query-budget and multi-target pricing, so the final production witness cannot carry the model obligations only as anonymous record fields.

AP2-N003 full-reduction certificate obligation projection note: `SelectedRustFullSignatureReductionCertificate` now also exposes named projections for the selected key-generation, signing-oracle, completeness, verifier-relation, full-forgery-to-accepted-spend, checked-core/hidden-source domain separation, query-budget, and multi-target obligations. The full-reduction gate requires these certificate-level names, keeping the same production-model obligations visible after the witness is converted into the selected certificate.

AP2-N003 production-witness route note: `SelectedRustFullSignatureReductionProductionWitness` now projects selected signature success through the accepted-spend probability, the active 128-bit target, and the erased native full-signature boundary. The full-reduction source gate requires these witness-level projections as well as the selected full-reduction certificate constructor, keeping the final AP2-N003 theorem tied to the production witness instead of only to certificate-level theorem names.

AP2-N003 selected-report stale-digest guard note: the Rust test `path2_native_ap2_n003_lean_selected_reports_match_rust_evidence` now parses the Lean selected accepted-spend numeric and full-signature reduction report constants, checks every digest/boolean/numeric field against the current Rust public evidence reports, and checks the Lean selected evidence digest constants against the Rust report-frame hashes. This catches source-window digest drift before a stale Lean selected report can satisfy the type-level binding route.

AP2-N003 full-reduction report digest-presence note: `SelectedRustFullSignatureReductionEvidenceReport.ReportVerified` now mirrors the same digest-presence requirement for the seven full-signature reduction evidence slots: production signature probability, signature-to-accepted-spend link, full-execution source, active hash boundary, query-budget pricing, multi-target pricing, and verifier-game binding. `SelectedRustFullSignatureReductionCertificate.toNativeFullSignatureSecurityBoundary` consumes those projection theorems, and the full-reduction source gate exact-checks them in the future selected theorem shape.

AP2-N003 full-reduction facts-projection note: `SelectedRustFullSignatureReductionEvidenceReport` now exposes named `FactsAllTrue` projection theorems for production signature probability pinning, signature-success-to-accepted-spend-game linkage, accepted-spend probability linkage, full-execution source consumption, active hash-boundary consumption, all-active-query-symbol pricing, multi-target horizon pricing, and verifier-game-to-production-execution binding. The full-reduction certificate erasure consumes these projections, and the source gate exact-checks them, so the final reduction theorem cannot hide the reduction facts inside one opaque all-true witness.

AP2-N003 active-break finiteness note: `Path2FullSpendGame.BreakEvent` now has a manual `Fintype` instance over the active named full-spend break constructors. This lets the accepted-spend numeric game use the actual active break family as the finite union-bound index instead of an abstract placeholder.

AP2-R004 full-boundary handoff note: `Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary.withConcreteFullExecutionSource` now installs `ConcreteFullExecutionAcceptedSpendLift C Source` into the active full-signature boundary's `fullForgeryImpliesAcceptedSpendForgery` field while preserving the other proof obligations. The AP2-N003 selected theorem consumes this adapter and `full_signature_with_concrete_full_execution_source_lift`, so the concrete execution-source theorem is no longer only a standalone full-spend-game result. The remaining production work is the full-signature reduction certificate and accepted-spend numeric-bound package, not the execution-source gate itself.

AP2-R006/AP2-N005 no-export source-gate note: `path2_native_deployment_preflight_v1` now records `no_export_serializer_boundary_present`, sourced by `path2_no_export_serializer_boundary_present_v1`. The helper source-inspects the Lean active accept/reject/storage conformance and closure theorem, the concrete tagged serializer allowlist certificate, and Rust public-output negative controls for verifier verdicts and storage/lifecycle scope. This keeps the no-export boundary tied to the active serializer allowlist path rather than a broad `public_view_serializers_present = true` claim.

AP2-N005 no-export instantiation closure note: `path2_native_deployment_preflight_v1` separately records `no_export_serializer_instantiated`, sourced by `selectedRustActiveNoExportSerializer_instantiation_v1`. `Descender/Path2NoExportConcrete.lean` now closes that source gate without using a vacuous denylist: it defines an injective selected serializer frame encoder, treats active accept/reject/storage tags 0/1/2 as public output frames, treats a distinct private tag as denylisted, proves public/private tag disjointness, and builds `SelectedRustActiveSerializerAllowlistCertificate.ofReportAndConformance` from the report/digest-backed selected serializer evidence. The theorem derives the no-export rule and allowlist through `toPublicOutputNoExport` and `toAllowlistCertificate`, then routes through `activeSerializerAllowlist_activeNativeNoExportClosure`, `NativeFullSignatureSecurityBoundary.withActiveSerializerNoExportClosure`, and `full_signature_with_active_serializer_no_export_closed`. The source gate still rejects theorem headers that assume the full-signature boundary, accepted-spend boundary, public no-export proof, serializer, policy, allowlist certificate, or selected serializer conformance facts; rejects `axiom`/`constant`/`opaque`/`variable` stubs for selected no-export facts even when declarations are multiline; and rejects locally fabricated all-true evidence reports or raw certificate literals. Rust deployment preflight now reports `no_export_serializer_instantiated=true` and no longer records `PATH2_NATIVE_DEPLOYMENT_BLOCKER_NO_EXPORT_SERIALIZERS`; the synthetic regression that forces this bit false still confirms no-export would gate before signer/CT/storage productization.

AP2-N005 no-export proof-shell hardening note: the no-export instantiation gate now rejects global `def`/`abbrev` aliases that set `selectedRustActiveSerializerEvidenceReportVerifiedProof` or `selectedRustActiveSerializerEvidenceFactsAllTrueProof` to `True.intro`. A regression prepends those aliases to the otherwise accepted no-export theorem shape and confirms the gate stays open.

AP2-N005 Rust evidence note: `Path2NativeNoExportSerializerEvidenceV1` mirrors the Lean selected-Rust report shape for active serializer no-export. The report hashes serialized accept, reject, and storage outputs, pins the active output schemas, checks accept/reject/storage serializer surface markers, screens every output for denylisted private material, and records a negative-control result. It has explicit frame/digest/all-facts/verify functions exported through `tools/production-reference`, and the regression rejects digest tampering, false component facts, and a private-marker accept output. The Lean selected theorem now consumes the corresponding report/digest route into `SelectedRustActiveSerializerAllowlistCertificate`, closing AP2-N005's active source gate.

AP2-R006 full-boundary handoff note: `NativeFullSignatureSecurityBoundary.withActiveSerializerNoExportClosure` now installs the concrete active serializer allowlist/no-export closure into the active full-signature boundary's `acceptedSpendSoundness.noExportClosed` field while preserving the other proof obligations. The no-export source gate requires this adapter and `full_signature_with_active_serializer_no_export_closed`, so P2FS-004 closure is threaded through the same active full-signature theorem surface rather than stopping at the accepted-spend wrapper.

Latest verification: `lake env lean Descender/Path2NativeBoundary.lean`, `lake env lean Descender/Path2NativeAccountDesign.lean`, `lake env lean Descender/Path2FullSpendGame.lean`, `lake env lean Descender/Path2FullSpendConcrete.lean`, `lake env lean Descender/Path2HashBudget.lean`, `lake env lean Descender/Path2NoExport.lean`, `lake env lean Descender/Path2NoExportConcrete.lean`, `lake env lean Descender/Path2RoleRootBinding.lean`, `lake env lean Descender/Path2RoleRootConcrete.lean`, `lake env lean Descender/Path2AssemblyConstruction.lean`, `lake env lean Descender/Path2AssemblyNativeValidity.lean`, `lake env lean Descender/Path2CompatibilitySoundness.lean`, `lake env lean Descender/Path2CompatibilityConcrete.lean`, `lake env lean Descender/Path2FullOpeningFieldEvidence.lean`, `lake build Descender.Path2FullOpeningFieldEvidence`, `lake env lean Descender/Path2FullOpeningFieldEvidencePredicates.lean`, `lake build Descender.Path2FullOpeningFieldEvidencePredicates`, `lake env lean Descender/Path2FullOpeningFieldEvidenceByteBridge.lean`, `lake build Descender.Path2FullOpeningFieldEvidenceByteBridge`, `lake env lean Descender/Path2FullOpeningFieldEvidenceComposition.lean`, `lake build Descender.Path2FullOpeningFieldEvidenceComposition`, `lake env lean Descender/Path2FullOpeningSourceBridge.lean`, `lake build Descender.Path2FullOpeningSourceBridge`, `lake env lean Descender/Path2FullOpeningSourceConcrete.lean`, `lake build Descender.Path2FullOpeningSourceConcrete`, `lake env lean Descender/Path2FullOpeningFieldEvidenceSourceAdapters.lean`, `lake build Descender.Path2FullOpeningFieldEvidenceSourceAdapters`, `lake env lean Descender/Path2CheckedCoreByteBridge.lean`, `lake env lean Descender/Path2HashDomainAndMultiTarget.lean`, and `lake env lean Descender.lean` pass after building the new modules. The latest Lean pass includes `ReviewedFq12DeployableHiddenSourceInputs`, `reviewed_fq12_certificate_soundness_from_deployable_hidden_source_inputs`, `ReviewedFq12AssemblySourceRouteWitnessSource`, `ReviewedFq12CompatibilitySourceRouteWitnessSource`, `ReviewedFq12SourceRouteDeployableHiddenSourceInputs`, `reviewed_fq12_certificate_soundness_from_source_route_deployable_inputs`, `ReviewedFq12SourceRouteLocalInstantiationInputs`, `reviewed_fq12_certificate_soundness_from_source_route_local_instantiation_inputs`, `source_route_byte_source_assembly_field_binding_of_route_witness`, `source_route_byte_source_compatibility_field_binding_of_route_witness`, `source_route_byte_source_compatibility_extraction_of_route_witness`, `ReviewedFq12FullOpeningFieldSourceRouteCertificate`, `ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource`, `source_route_byte_predicate_source_active_reduction_facts_or_named_break`, `source_route_byte_source_local_assembly_predicates_or_break`, `source_route_byte_source_local_native_validity_or_break`, `source_route_byte_source_local_compatibility_extracts_or_break`, `source_route_byte_source_concrete_field_certificates_or_named_break`, `source_route_byte_source_concrete_field_certificates_or_boundary_break`, `source_route_byte_source_assembly_concrete_fields_or_boundary_break`, `source_route_byte_source_compatibility_concrete_fields_or_boundary_break`, and `source_route_byte_source_concrete_compatibility_extraction_or_boundary_break`. Rust targeted checks pass: `cargo fmt --check` in `tools/parameter-search`; `cargo test path2_reviewed_fq12_full_native_opening_certificate_binds_all_roles --lib` now covers `Path2ReviewedFq12FullOpeningFieldEvidenceV1`, `Path2ReviewedFq12FullOpeningFieldSourceRouteV1`, direct payload role-field checks, aggregate-source rejection for both field evidence and source route, self-frame role-root/full-opening/representative tamper rejection, compatibility digest rebinding rejection, stale field-evidence/source-route digest rejection, schema mismatch rejection, and constructor-level assembly role-root/opening-field/source mismatch rejection; `cargo test production_domain_schema_registry_is_globally_disjoint --lib` covers the reviewed-Fq12 assembly, compatibility, field-evidence, source-route, and aggregate-bridge frame domains; `cargo test path2_reviewed_fq12_full_native_opening_certificate_rejects_role_root_rebinding --lib`, `cargo test path2_native_checked_core_accept_pins_canonical_proof_bytes_before_acceptance --lib`, `cargo test path2_native_signer_verifier_evidence_keeps_productization_gates_separate --lib`, `cargo test --lib path2_native_prevalidated_honest_flow_reaches_selected_verifier_public_acceptance`, `cargo test --lib path2_native_prevalidated_signer_execution_report_keeps_public_reject_outside_private_path`, and `cargo test --lib path2_native_verifier_pipeline_is_public_only_and_fail_closed_until_productized`. Prior targeted checks also passed for hidden-source fail-closed evidence, checked-core frame, boundary assembly, lifecycle/spent-slot/prevalidated-slot replay controls, no-export public surfaces, hash/QROM/domain review, signer/keygen scope, deployment preflight, and the `tools/production-reference` facade boundary. The Rust hidden-source bridge audit intentionally keeps `semantics_bridge_complete = false` until `extractor_theorem_instantiated` is supplied.

Additional current Lean source-adapter coverage includes
`shared_representative_frame_source_of_assembly_frame_functionality_binding_and_local_model`,
`ReviewedFq12SelectedRustDirectAssemblyDigestFrameWitnessPackagedSourceRouteInputs`,
`ReviewedFq12SelectedRustDirectAssemblyDigestFrameWitnessPackagedSourceRouteInputs.toSharedRepresentativeFramePackagedInputs`,
`reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_digest_frame_witness_packaged_source_route_inputs`,
`ReviewedFq12NativeRepresentativeDigestFrameWitnessCertificate`,
`ReviewedFq12NativeRepresentativeDigestFrameWitnessByteParser`,
`ReviewedFq12NativeRepresentativeDigestFrameWitnessByteCertificate`,
`ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource`,
`assembly_frame_source_of_digest_frame_witness_byte_source`,
`ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessCertificate`,
`ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteParser`,
`ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteCertificate`,
`ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource`,
`canonical_representative_source_of_canonical_representative_byte_source`,
`ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeWitnessBytePackagedSourceRouteInputs`,
`ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeWitnessBytePackagedSourceRouteInputs.toDigestFrameWitnessPackagedInputs`,
`reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_witness_byte_packaged_source_route_inputs`,
`ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputs`,
`ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputs.toCanonicalRepresentativeWitnessBytePackagedInputs`,
`reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_byte_and_witness_packaged_source_route_inputs`,
`byte_source_concrete_local_assembly_predicates_or_boundary_break`,
`source_route_byte_source_concrete_local_assembly_predicates_or_boundary_break`,
`byte_source_concrete_native_validity_or_boundary_break`,
`source_route_byte_source_concrete_native_validity_or_boundary_break`, and
`byte_source_concrete_compatibility_extraction_or_boundary_break`, the plain
byte-field sibling of the source-route compatibility-extraction adapter.
Source-route local deployable coverage now includes
`source_route_byte_source_assembly_local_predicates_of_route_witness`,
`source_route_byte_source_assembly_native_validity_of_route_witness`,
`source_route_byte_source_compatibility_extraction_of_route_witness`,
`ReviewedFq12SourceRouteDeployableHiddenSourceInputs`, and
`reviewed_fq12_certificate_soundness_from_source_route_deployable_inputs`,
so the route plus local predicate source can supply field bindings,
deterministic assembly local/native facts, and compatibility extraction while
leaving role-root plus assembly/compatibility availability explicit.
`ReviewedFq12SourceRouteLocalInstantiationInputs`
and
`reviewed_fq12_certificate_soundness_from_source_route_local_instantiation_inputs`
now name the source-route package before the Rust parser/model theorem. The
first boundary-availability wrapper is
`ReviewedFq12BoundaryAvailabilityConcreteRoleRootSourceRouteInputs` with
`reviewed_fq12_certificate_soundness_from_boundary_availability_concrete_role_root_source_route_inputs`:
it consumes concrete role-root field binding, base active-boundary
assembly/compatibility availability, the source-route byte predicate source,
local predicate source, and route witnesses.
The factored endpoint
`ReviewedFq12FactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs`
and
`reviewed_fq12_certificate_soundness_from_factored_boundary_availability_concrete_role_root_source_route_inputs`
split those route witnesses into `ReviewedFq12OpeningRoleLiftWitnessSource`,
`ReviewedFq12ConstructedAssemblyCompatibilityWitnessSource`, and
`ReviewedFq12CompatibilityAssemblyRepresentativeWitnessSource`.
The selected-Rust endpoint
`ReviewedFq12SelectedRustFactoredBoundaryAvailabilityConcreteRoleRootSourceRouteInputs`
and
`reviewed_fq12_certificate_soundness_from_selected_rust_factored_boundary_availability_concrete_role_root_source_route_inputs`
replace the generic source-route byte predicate source with
`SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource`, whose contract
decomposes the chosen Rust field-evidence/source-route frames into byte
certificates plus semantic binding predicates.
The compact-local endpoint
`ReviewedFq12SelectedRustFactoredConcreteLocalSourceRouteInputs`
and
`reviewed_fq12_certificate_soundness_from_selected_rust_factored_concrete_local_source_route_inputs`
replace the free local predicate source with compact assembly
availability/local/native and compatibility availability/extraction-rules
records.
The functional-lift endpoint
`ReviewedFq12SelectedRustFactoredConcreteLocalFunctionalLiftSourceRouteInputs`
and
`reviewed_fq12_certificate_soundness_from_selected_rust_factored_concrete_local_functional_lift_source_route_inputs`
replace the raw opening-to-role-lift witness with
`ReviewedFq12CertificateLiftOpeningFunctionality`, deriving the witness from
the role-root bridge plus functionality of the selected certificate lift
relation.
The witnessed functional-lift endpoint
`ReviewedFq12SelectedRustConcreteLocalFunctionalLiftSourceRouteInputs` and
`reviewed_fq12_certificate_soundness_from_selected_rust_concrete_local_functional_lift_source_route_inputs`
keep the functional-lift derivation and replace the two loose
compatibility-side route witnesses with one
`ReviewedFq12CompatibilityRouteWitnessSource`.
The packaged endpoint
`ReviewedFq12SelectedRustPackagedSourceRouteInputs` and
`reviewed_fq12_certificate_soundness_from_selected_rust_packaged_source_route_inputs`
groups role-root/lift facts as
`ReviewedFq12RoleRootFunctionalLiftSource` and assembly/compatibility local
facts as `ReviewedFq12AssemblyCompatibilityLocalModelSource`, leaving four
explicit source packages.
The field/local packaged adapter
`ReviewedFq12SelectedRustFieldLocalPackagedSourceRouteInputs` and
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_packaged_source_route_inputs`
is an instantiation convenience: it consumes the already-named concrete
field/local assembly and compatibility source records and forgets their
redundant field-binding facts, because selected source-route bytes supply those
bindings. It is not the active cryptographic closure target; the active target
is now the field-local shared-frame source theorem
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_sources`.
The adapter theorem
`constructed_compatibility_witness_of_assembly_field_binding_and_local_model`
now shows that direct `ReviewedFq12ConcreteAssemblyFieldBindingOrBreak` plus
`ReviewedFq12AssemblyCompatibilityLocalModelSource` derives the constructed
compatibility half of `ReviewedFq12CompatibilityRouteWitnessSource`. This is a
non-active convenience route: selected source-route byte binding is
compatibility-indexed and cannot supply that direct pre-compatibility assembly
binding without circularity.
Rust semantics-audit negative controls now also reject removing the narrowed
Lean endpoint flag, the factored Lean endpoint flag, the selected-Rust
source-route byte-source endpoint flag, the compact-local endpoint flag, the
functional-lift endpoint flag, the witnessed functional-lift endpoint flag, the
packaged endpoint flag, the field/local packaged adapter flag, or the
assembly/compatibility local-model source flag.
The local-model Rust flags are now source-inspected rather than blanket
constants: `path2_reviewed_fq12_assembly_compatibility_local_model_source_present_v1`
checks the assembly constructor/verifier, compatibility constructor/verifier,
same-representative predicate, and full-opening source-route constructor/frame
against the active route assumptions.
The field/source-route Rust flags are also source-inspected:
`path2_reviewed_fq12_full_opening_field_evidence_source_present_v1`,
`path2_reviewed_fq12_full_opening_source_route_source_present_v1`, and
`path2_reviewed_fq12_source_route_negative_controls_present_v1` pin field
evidence construction/framing/verification, route construction/framing/verification,
and the active stale-digest, aggregate-source, and rebinding regressions.
The audit now also carries
`rust_selected_source_route_byte_source_contract_present`, backed by
`path2_reviewed_fq12_selected_source_route_byte_source_contract_present_v1`,
which ties the Lean
`SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource` contract to the
Rust field-evidence/source-route constructors, frame labels, verifier
reconstruction, and source-route negative controls. This keeps the raw
AP2-N001 component theorem from treating generic route-source evidence as the
selected byte-source contract.
The parser/frame part of that contract is now visible separately as
`rust_source_route_byte_parser_frame_certificates_present`, backed by
`path2_reviewed_fq12_source_route_byte_parser_frame_certificates_present_v1`.
It source-inspects the Lean field-evidence and source-route byte parser
certificates, the Rust field-evidence/source-route frame self-checks, the
SHA-256 frame digest functions, verifier reconstruction, and the stale/tamper
negative controls. This keeps byte canonicality and self-frame validation from
being hidden behind the broader selected-source-route contract bit.
The selected extraction-certificate flag is now source-inspected by
`path2_reviewed_fq12_hidden_relation_extraction_certificate_source_present_v1`,
which pins payload digest checks, statement/opening/pad binding,
account-witness opening-frame recomputation, certificate framing, and the
statement-rebinding regression.
The hidden-source payload binding itself is now source-inspected by
`path2_reviewed_fq12_hidden_source_payload_binding_source_present_v1`; the Rust
audit uses it to back the native-boundary payload-binding, slot/opening,
projection-carrier, accepting-wrapper, and rebinding-control flags. The source
check requires native-boundary assembly, leaf/bundle equality checks, reviewed
statement and opening recomputation, carrier and checked-core digest binding,
payload frame labels, verifier reconstruction, and the native-boundary rebound
regression.
The creation-statement bridge flag is now source-inspected by
`path2_creation_statement_bridge_source_present_v1`, which pins the non-circular
creation statement, creation-statement frame, creation-proof binding/native
role-root inputs, and regressions showing message/signature rebinding does not
become the Path 2 creation digest while pad/proof-binding field mutations remain
digest-visible.
The active no-partial-export negative-control flag is now source-inspected by
`path2_reviewed_fq12_no_partial_export_negative_controls_source_present_v1`,
which pins selected-material reject-path assertions, helper-visibility checks,
wrong-pad and malformed-opening public errors, opaque carrier-export rejection,
and the single accepting-certificate export wrapper. This records the active
negative controls only; the full Rust serializer allowlist bridge remains
P2FS-004.
The Lean payload-interface flag is now source-inspected by
`lean_path2_reviewed_fq12_payload_interface_pinned_v1`, which pins
`ReviewedFq12HiddenSourcePayload`,
`ReviewedFq12HiddenRelationExtractionCertificate`, the active
payload/certificate predicates, extraction availability/soundness definitions,
and matching Rust payload-binding/extraction-certificate structs. The active
hidden-source semantics-bridge audit no longer uses unconditional booleans
inside the P2FS-001 source-route core; remaining `true` presence flags are
productization/preflight surfaces outside this audit.
The hidden-source boundary theorem's base premises are now tracked explicitly.
`rust_payload_projection_law_present` source-inspects the Lean
`ReviewedFq12HiddenSourceSemanticsCertificate` projection law plus the active
boundary theorem premise, and checks the Rust payload-binding constructor,
verifier, and regression that bind the accepted payload to the assembled native
boundary digest and slot. `rust_extraction_certificate_availability_present`
source-inspects `ReviewedFq12ExtractionCertificateAvailability`, the active
boundary theorem premise, and the Rust extraction-certificate constructor,
frame, and accepted-tuple regression. These fields are now required by
`semantics_bridge_complete`, so the bridge cannot appear closed merely because
the broad payload interface exists.
The Lean source-route endpoint flags are now source-inspected by
`lean_path2_reviewed_fq12_source_route_endpoint_surface_present_v1`, which pins
the base source-route closure theorem, boundary/factored wrappers, selected-Rust
source-route byte-source wrapper, compact/local functional-lift wrappers,
packaged route, direct-assembly representative/shared/native-frame wrappers, and
the field/local adapter theorem.
Rust hidden-source semantics audit now digest-binds the raw active AP2-N001
surface as
`active_field_local_digest_frame_witness_byte_component_surfaces_present`.
That gate mirrors
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components`:
concrete role-root field binding, selected lift functionality, selected
source-route byte evidence, field/local assembly and compatibility components,
and the canonical plus assembly witness byte sources pinned to one
representative digest frame. Older source-route/package adapter booleans remain
digest-recorded as supporting evidence, but `semantics_bridge_complete` no
longer treats them as separate active-path blockers. The bridge still keeps
`extractor_theorem_instantiated = false` and
`semantics_bridge_complete = false`; the production hidden-source semantics
gate/readiness report carries the bridge-audit digest and completion bit, so
the deployability chain remains fail-closed on the active theorem-instantiation
obligation.

The bridge audit still hash-binds the AP2-N001 byte-shaped convenience package
surface. `selected_rust_canonical_byte_and_witness_package_surfaces_present`
now requires the raw role-root side package directly, but it remains a
supporting surface gate, not a theorem-instantiation claim;
`extractor_theorem_instantiated` is the final hidden-source semantics blocker.
The Rust audit now also carries
`rust_full_native_opening_canonical_representative_witness_byte_source_present`
separately, so the Rust-shaped canonical witness source required by the latest
Lean endpoint is visible as its own audited item rather than hidden inside the
broader canonical representative source flag.
It also carries
`rust_canonical_and_digest_witnesses_share_native_frame_present`, which
source-inspects that the canonical representative witness and assembly
digest-frame witness recompute the representative digest from the same
`path2_reviewed_fq12_native_representative_digest_v1` argument list.
The shared witness-byte package is now tracked separately as
`rust_shared_digest_frame_witness_byte_source_contract_present`, backed by
`path2_reviewed_fq12_shared_digest_frame_witness_byte_source_contract_present_v1`.
That helper pins the Lean
`ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources` package and its
two digest-frame equalities to the Rust canonical representative witness and
assembly digest-frame witness constructors, frames, and verifier
reconstruction. This keeps the raw-component theorem's shared-frame equalities
from being hidden behind the broader "same native frame" source check.
The helper now also requires the shared-frame theorem to destructure the
witness package and consume the stored canonical and assembly digest-frame
equalities with explicit Lean `cases`, preventing a regression where both
witness byte sources are merely packaged beside one digest frame while the
proof route ignores the equality evidence.
The representative-digest functionality flag is also source-inspected now:
`rust_representative_digest_functionality_source_present` is backed by
`path2_reviewed_fq12_representative_digest_functionality_source_present_v1`,
which pins the Lean
`ReviewedFq12FullNativeOpeningRepresentativeDigestFunctionalitySource`
uniqueness theorem to the selected Rust shared-frame witness contract rather
than aliasing the broader canonical-representative source.
The canonical-digest and assembly-digest-lookup flags are source-inspected now
as well. `rust_representative_canonical_digest_source_present` checks the Lean
`ReviewedFq12RepresentativeCanonicalDigestSource` and shared-frame adapter
markers against the Rust shared witness-frame contract, and
`rust_assembly_representative_digest_lookup_source_present` checks
`assembly_representative_digest_lookup_of_canonical_digest_source` rather than
aliasing the canonical representative flag.
The shared-representative-frame flag is source-inspected now too:
`rust_shared_representative_frame_source_present` checks the Lean
`ReviewedFq12SelectedRustSharedRepresentativeFrameSource` record, its
representative/assembly projections, and the direct-assembly shared-frame
endpoint instead of only conjoining the two separate frame-source flags.
The full-native-opening representative-frame flag is also split from the
canonical-representative flag:
`rust_full_native_opening_representative_frame_source_present` now checks
`representative_frame_source_of_canonical_representative_source` and the
matching canonical witness gates rather than aliasing the broader canonical
representative source.
The constructed-compatibility and compatibility-route witness flags are now
source-inspected separately from the broad local-model helper.
`rust_constructed_assembly_compatibility_witness_source_present` checks
`constructed_compatibility_witness_of_assembly_field_binding_and_local_model`
against direct assembly binding plus assembly/compatibility component helpers.
`rust_compatibility_route_witness_source_present` checks
`compatibility_route_witness_source_of_assembly_field_binding_local_model_and_digest_lookup`,
including the representative-digest lookup source, instead of aliasing the
constructed-compatibility flag.
`rust_assembly_compatibility_local_model_source_present` is also split back to
the broad local-model helper
`path2_reviewed_fq12_assembly_compatibility_local_model_source_present_v1`,
so local-model presence is not implied solely by the constructed-compatibility
adapter.
The audit now also carries a dedicated marker bit for
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_sources`,
so the active five-source endpoint is checked directly rather than only through
the broader canonical/witness package surface.
Lean now also exposes
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_sources`,
which opens the shared witness package into the concrete Rust-facing
obligations: the selected representative digest frame, the canonical witness
byte source, the assembly digest-frame witness byte source, and the two digest
frame equalities.
The raw-component theorem
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components`
then expands the remaining source records into the exact role-root/lift,
source-route byte, assembly field/local/native, compatibility field/extraction,
and shared witness byte obligations.
The Rust audit tracks this theorem with its own component-endpoint bit, so the
bridge gate is no longer satisfied merely by the older packaged or shared-frame
markers.
The raw-component audit now also separates
`rust_certificate_lift_opening_functionality_present` from the role-root field
binding flag: it source-inspects the deterministic full-opening lift
constructor/verifier and the opening-field rebinding regressions instead of
aliasing the broader role-root helper.
It also separates assembly field/local/native components and compatibility
field/extraction components from the broader local-model helper, so the raw
component theorem's assembly and compatibility premises are visible as distinct
hash-bound audit flags.

Lean now also exposes
`reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_byte_and_witness_sources`,
a direct AP2-N001 theorem over the six required selected Rust/model source
records. This removes package assembly as proof work; the remaining proof
obligation is to instantiate the six records, with the canonical representative
byte source as the first high-risk sub-constructor.

For that first sub-constructor, Lean now has
`ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource`, a
Rust-shaped source that is allowed to start from the concrete role-lift and
assembly certificates used by
`Path2ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessV1`. The
adapter `canonical_representative_byte_source_of_witness_source_and_model`
recovers those concrete certificates from the active role-root/lift source,
direct assembly binding, and local assembly availability, then exports the
semantic `ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource`.
The theorem
`reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_canonical_representative_witness_and_digest_frame_sources`
now consumes that Rust-shaped canonical witness source directly.
Lean now also exposes
`ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources` and
`reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_shared_digest_frame_witness_sources`;
this is the preferred active instantiation target because it requires the
canonical witness source and assembly digest-frame witness source to be pinned
to the same selected 11-input representative digest frame.
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_inputs`
then specializes that target to the existing concrete field/local assembly and
compatibility source records, deriving the direct assembly binding and local
model internally. The direct source-record theorem
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_sources`
is the exact AP2-N001 closure target for the five records still needing
concrete Rust/model instantiation.

## 2026-06-01 Parallel AP2 Batch

| Work item | Result | Still open |
| --- | --- | --- |
| AP2-N001 | `ReviewedFq12FullOpeningBridgeCertificate` now consumes assembly construction and assembled-native-validity soundness separately, deriving the old assembly soundness surface by theorem. Field evidence now has a Rust object, Lean mirror, sidecar predicate bridge, byte parser/frame hook bridge, and byte-source constructor for recovering the non-aggregate binding facts. Rust also exposes `Path2ReviewedFq12FullOpeningFieldSourceRouteV1`, which digest-binds the concrete source ordering: assembly full-opening field certificate, compatibility same-representative field certificate, and adapter-only field evidence before the aggregate bridge exists. Lean now mirrors that source route with `ReviewedFq12FullOpeningFieldSourceRouteCertificate`, route byte-parser/source structures, `source_route_byte_predicate_source_active_reduction_facts_or_named_break`, and `source_route_byte_source_concrete_field_certificates_or_named_break`. `Path2FullOpeningSourceBridge.lean` adds binding-or-break wrappers for assembly and compatibility and derives extraction-certificate soundness without aggregate-bridge premises. `Path2FullOpeningSourceConcrete.lean` now packages the deployable non-aggregate route as `ReviewedFq12DeployableHiddenSourceInputs` and adds reverse availability bridges from the base active-boundary availability surfaces. `Path2FullOpeningFieldEvidenceByteBridge.lean` now exposes `SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource`, and `Path2FullOpeningFieldEvidenceSourceAdapters.lean` now packages the selected-Rust active route as `ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs`, deriving direct assembly binding and the local model from concrete field/local source records, consuming the Rust-shaped canonical representative witness byte source plus the Rust-shaped assembly representative frame witness byte source, and requiring both witness sources to share one representative digest frame. The theorem `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_sources` is the current source-route P2FS-001 endpoint. The direct canonical/witness-byte, digest-frame witness, native-frame, shared-frame, field/local input, and older package theorems are adapters leading into this implementation target. | Instantiate `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_sources` from the Rust verifier/model: role-root/lift source, selected Rust source-route byte source, concrete field/local assembly source, concrete compatibility field source, and the shared-frame package containing `ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessByteSource` for `Path2ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessV1` plus `ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource` for `Path2ReviewedFq12NativeRepresentativeDigestFrameWitnessV1`. |
| AP2-N002 | Added `Path2NativeCheckedCoreAcceptV1`, `Path2NativeCheckedCoreAcceptV1Bridge`, frame-correspondence theorem, and named equation/frame/domain/parser breaks. | Instantiate the bridge byte-for-byte against Rust parser/frame/verifier behavior and discharge downstream checked-core equation soundness. |
| AP2-N003 | Added `Path2FullSpendGame.lean` with accepted-spend conclusion and full-forgery-to-accepted-spend wrapper. | Instantiate the abstract game with the real account creation, slot admission, spend verification, state transition, and rollback model. |
| AP2-N004 | Added `Path2HashBudget.lean`, proving inactive `q_batch` and adapting the executable direct budget to `NativeHashBudgetBoundary.queryBudgetPriced`; widened route update is moving this from the old one-bit margin to the 182-bit QROM-discounted transcript term. | Price the selected 32-bit multi-target/account horizon and prove concrete domain separation for checked-core and hidden-source frames. |
| AP2-N005 | Added `Path2NoExport.lean`, lifting public-output conformance into active accept/reject/storage no-export closure. | Prove the concrete Rust serializers instantiate the accept/reject/storage conformance predicates. |
| AP2-N006 | Added Rust `Path2NativeSignerVerifierPipelineEvidenceV1` and a test that prevalidated honest construction reaches selected-verifier public-boundary acceptance while production remains fail-closed. | Retire signer/verifier pipeline, CT, and storage productization blockers before claiming production acceptance or completeness. |

## 2026-06-01 Parallel AP2 Batch 2

| Work item | Result | Still open |
| --- | --- | --- |
| AP2-N007 | Added `Path2RoleRootBinding.lean`, isolating non-aggregate role-root pinning with `ReviewedFq12CreationPayloadPinsNativeRoleRoot` and `ReviewedFq12NativeRoleRootPinsFullOpeningRoles`. | Instantiate both assumptions against the concrete Rust creation/payload binding and native role-root commitment. |
| AP2-N008 | Added `Path2AssemblyConstruction.lean`, deterministically constructing a `HiddenRepresentative` from the assembly certificate and reducing construction soundness to local predicate facts or a boundary break. | Prove the concrete assembly predicates hold for the deterministic representative and separately close assembled-native-validity. |
| AP2-N009 | Added `Path2CompatibilitySoundness.lean`, reducing compatibility-certificate soundness to same-representative field compatibility plus an assembled native-fiber representative premise. | Instantiate same-representative field compatibility from the concrete compatibility certificate and assembly output. |
| AP2-N010 | Added `Path2CheckedCoreByteBridge.lean`, giving parser/canonical-proof/frame/hash/syndrome certificates that imply `Path2NativeCheckedCoreAcceptV1` and then `checkedCoreAccepts`. | Bind these certificates to the real Rust parser, canonical byte format, frame bytes, and checked-core equation verifier. |
| AP2-N011 | Added `Path2HashDomainAndMultiTarget.lean`, filling checked-core domain separation, hidden-source domain separation, query-budget pricing, and multi-target pricing. The widened route update pins the selected 32-bit active horizon and keeps an over-budget negative control above the new margin. | Prove the widened budget and byte-frame binding against the active Rust constants. |
| AP2-N012 | Added a Rust test pinning that public-boundary acceptance does not hide signer/verifier, CT, or storage productization blockers. | Retire those gates with real productization evidence before claiming production acceptance or completeness. |

## 2026-06-01 Parallel AP2 Batch 3

| Work item | Result | Still open |
| --- | --- | --- |
| AP2-N013 | Added `Path2RoleRootConcrete.lean`, reducing role-root residuals to pairwise native-role-root equality and non-root full-opening field equality certificates. Rust now materializes those equalities in `Path2ReviewedFq12FullOpeningFieldEvidenceV1`, including canonical role-root recomputation, direct payload role-field checks, and self-framed full-opening digest checks. The Rust regression rejects both self-frame role-root tampering and constructor-level assembly role-root mismatch. `Path2FullOpeningFieldEvidence.lean` mirrors the object, `Path2FullOpeningFieldEvidenceByteBridge.lean` names the byte source contract, and `Path2FullOpeningFieldEvidenceComposition.lean` packages field evidence into the role-lift binding plus non-root opening equality needed by active role-root reductions. | Supply the concrete Rust byte source theorem, then use the active reduction facts package to feed the active role-root field premises. |
| AP2-N014 | Added `Path2AssemblyNativeValidity.lean`, reducing assembled-native validity to `InFiber`, `localValid`, and `descentConsistent` for the deterministic assembly representative. | Prove those native-validity facts for the real assembled representative. |
| AP2-N015 | Added `Path2CompatibilityConcrete.lean`, making compatibility binding equivalent to concrete payload/extraction/opening/projection/representative field certificates and reducing same-representative soundness to concrete extraction-predicate rules. Rust `Path2ReviewedFq12FullOpeningFieldEvidenceV1` now materializes the compatibility payload/extraction/opening/projection/representative field equalities before the aggregate bridge is available, rejects compatibility digest rebinding, and rejects constructor-level assembly opening-field mismatch. The field-evidence composition module now packages byte-backed evidence into compatibility binding plus `SameRepresentativeConcreteFieldCertificate`, then derives the compatibility extraction triple for the deterministic assembly representative from local assembly/native-validity facts plus `SameRepresentativeConcreteExtractionPredicates`. | Instantiate the concrete extraction-predicate rules from the real compatibility verifier and supply the Rust field-evidence byte source theorem. |
| AP2-N016 | Added Rust checked-core proof evidence test pinning canonical proof length and reject-before-accept behavior for short, noncanonical, and commitment-mismatched proof bytes. | Connect this Rust evidence to `Path2CheckedCoreByteBridge` with a Lean/Rust byte-parser theorem. |
| AP2-N017 | Added `Path2NoExportConcrete.lean`, reducing tagged accept/reject/storage serializer allowlisting to active no-export closure. | Bind tagged Lean frames to concrete Rust serializers and all active outputs. |
| AP2-N018 | Added `Path2FullSpendConcrete.lean`, instantiating the full-spend abstract game with concrete carrier records and direct/full-forgery soundness wrappers. `ConcreteFullSignatureExecutionSource` now separates production execution success from concrete verifier acceptance. | Connect the execution source and carrier to the actual account creation, admission registry, verifier, chain state, rollback model, and numeric EUF-CMA bound. |

## Redesign-Risk Work Queue

This queue is ordered by likelihood of exposing a real issue that forces a
parameter or route redesign.

| ID | Priority | Work item | Closes |
| --- | --- | --- | --- |
| AP2-R001 | Highest | Resolve the QROM/multi-target deployment horizon before claiming active-path deployment security. Route decision recorded 2026-06-02: use auxiliary Fq12 action bits, not compact coefficient widening. Target constants are 23 algebraic challenge bytes plus 4 hardening bytes, last algebraic-byte mask `0x3f`, 182 algebraic challenge bits, and 214 total challenge bits. Keep 26 repetitions for first closure, giving `258` transaction bytes / `290` bytes with hidden-source proof. The active implementation/proof update is now replacing the old one-bit fail-closed budget with the widened 32-bit aggregate horizon while preserving explicit QROM/direct-query and Fq12 action-collision obligations. | `P2FS-007`, `P2FS-008` |
| AP2-R002 | High | Prove the selected Rust verifier/pipeline instantiates the Lean active relation byte-for-byte, including canonical signature/proof bytes, checked-core frame bytes, hidden-source frame bytes, parser canonicality, domain registry, and actual hash inputs. Started 2026-06-02: `Path2CheckedCoreByteBridge.lean` now exposes `SelectedRustCheckedCoreByteSource`, the Lean-facing contract for decomposing `path2_native_checked_core_accept_v1` into parser/native-handle agreement, proof length/canonicality, decoded commitment, parsed response syndrome, frame bytes, verifier handle, challenge, and expected/actual syndrome equality. `Path2HashDomainAndMultiTarget.lean` now exposes `ActiveHashByteDomainInstantiation`, the contract binding concrete Rust byte domains to the active Lean tag registry. Targeted Rust evidence passed for canonical proof rejection, assembly-frame derivation, proof-generation/expected-syndrome agreement, and active/native plus global domain-schema disjointness. | `P2FS-012`, `P2FS-002`, `P2FS-005`, `P2FS-006` |
| AP2-R003 | High | Instantiate `ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs` for the active Path 2 native / reviewed-Fq12 route: `ReviewedFq12RoleRootFunctionalLiftSource`, selected Rust field/source-route byte source, concrete field/local assembly source, concrete compatibility field source, and the shared-frame witness source package. This wrapper derives direct assembly binding from the concrete assembly source, derives the local model from the concrete assembly/compatibility sources, consumes the Rust-shaped assembly-frame recomputation witness, derives representative canonicality from the Rust-shaped canonical witness byte source, and pins both witness sides to one representative digest frame. Selected source-route bytes remain the parser/frame bridge for an already witnessed active route; they are not used to source the direct assembly-binding premise. Started 2026-06-02: rechecked the source-route shape and Rust regressions; no aggregate-bridge source use found in the active field-evidence/source-route path, role-root rebinding tests still pass, and Lean now exposes `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_inputs` as the current implementation-oriented closure theorem over the exact packaged selected-Rust source-route closure endpoint. Rust now has typed witness APIs for the canonical representative side and assembly digest-frame side; regressions verify both valid witnesses, stale representative-digest rejection, stale witness-digest rejection, and rebound assembly/opening drift rejection. The new canonical representative witness domain is in the global disjoint-domain registry. The Rust fail-closed audit now source-inspects the selected hidden-relation extraction certificate, local assembly/compatibility model route, field evidence, source route, negative controls, Rust-shaped canonical witness source, and shared representative digest frame: assembly construction verifies the full opening, checks native/projection/role fields, recomputes the representative digest, compatibility construction verifies chain/digest/field agreement, field evidence rejects aggregate-source use and recomputes role-root/full-opening/representative digests, and the source-route constructor requires direct assembly binding, same-representative compatibility, and adapter-only field evidence. The audit also exposes a shared-frame AP2-N001 surface gate for the field-local package. The remaining hard compatibility-route work is to instantiate those five source packages from Rust/model byte evidence, with the shared-frame package carrying both witness-source records. | `AP2-001`, `P2FS-001` |
| AP2-R004 | High | Bind `Path2FullSpendConcrete` to the real account creation, slot admission, spend verification, state transition, rollback model, and numeric full-forgery reduction. Started 2026-06-02: added `ConcreteFullSignatureExecutionSource` so production full-signature success is no longer required to be definitionally equal to `ConcreteSpendVerificationAccepts`; production must prove success implies verifier acceptance or a named break. | `AP2-003`, `P2FS-003`, `P2FS-013` |
| AP2-R005 | Medium | Close signer/keygen/completeness productization: honest constructor complete, signer/verifier pipeline present, CT/storage blockers retired, and honest flow verifies under the selected byte verifier in production mode. | `AP2-005`, `AP2-006`, `P2FS-009`, `P2FS-010`, `P2FS-011` |
| AP2-R006 | Medium | Bind `Path2NoExportConcrete` tagged output frames to every concrete Rust accept/reject/storage serializer. | `AP2-007`, `P2FS-004` |

## Near-Term Work Queue

| ID | Priority | Work item | Closes |
| --- | --- | --- | --- |
| AP2-N001 | Highest by proof dependency | Instantiate `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_digest_frame_witness_byte_components` from the Rust verifier/model: concrete role-root field binding, selected certificate-lift functionality, selected Rust source-route byte source, assembly availability, direct assembly field binding, assembly local predicates, assembly native validity, compatibility availability, same-representative field binding, compatibility extraction rules, the selected representative digest frame, the Rust-shaped canonical representative witness byte source, `ReviewedFq12SelectedRustAssemblyRepresentativeFrameWitnessByteSource` for `Path2ReviewedFq12NativeRepresentativeDigestFrameWitnessV1`, and equalities pinning both witness sources to that frame. Rust now exposes the digest frame as a typed witness, and the local assembly/compatibility model, field-evidence, source-route, source-route negative-control, canonical witness, canonical-witness rebinding negative-control, and shared-frame audit flags are source-inspected, so the Lean/Rust bridge should consume byte evidence rather than relying on broad source-window inspection. Lean now exposes the raw-component theorem as the most implementation-oriented AP2-N001 target. This derives direct assembly binding and the local model internally, derives assembly-frame canonicality from the Rust-shaped witness byte source, derives representative digest functionality from canonical representative witness construction, and keeps selected source-route bytes compatibility-indexed with the aggregate bridge downstream only. | `AP2-001` |
| AP2-N002 | High | Instantiate `SelectedRustCheckedCoreByteSource`, `Path2CheckedCoreByteBridge`, and `Path2NativeCheckedCoreAcceptV1Bridge` against Rust `path2_native_checked_core_accept_v1` bytes/parser/hash behavior, then prove downstream checked-core equation soundness or named breaks. | `AP2-002` |
| AP2-N003 | High | Instantiate `selectedRustFullSignatureReductionBoundary_instantiation_v1` from the selected full-execution source, active hash boundary, all active query-symbol pricing, multi-target pricing, production signature-forgery probability pinning, and the accepted-spend numeric bound. | `AP2-003`, `P2FS-013` |
| AP2-N004 | High | Replace/thread `activeNativeHashBudgetBoundary` with the auxiliary-Fq12 widened challenge budget: prove the 182-bit algebraic action, 214-bit transcript budget, `log2(T)=32` multi-target pricing, Fq12 action-collision floor, canonical byte mask, and full-byte frame binding. | `AP2-004` |
| AP2-N005 | Closed | Selected no-export serializer instantiation is now present and preflight no longer reports `PATH2_NATIVE_DEPLOYMENT_BLOCKER_NO_EXPORT_SERIALIZERS`; keep the synthetic reopen-ordering regression as a guard. | `AP2-007` |
| AP2-N006 | Medium | Close signer/keygen/completeness productization: honest constructor complete, signer/verifier pipeline present, CT/storage blockers retired, and honest flow verifies under the selected byte verifier in production mode. | `AP2-005`, `AP2-006` |

## 2026-06-03 Active Path Native-Boundary Update

The active target for the current account-creation payload work is the Path 2
`zk_masked_trace_iop_native_boundary` route, not the older public-linear/SIS
commitment shape. A downgrade remained possible at the theorem surface: a
column profile could be compatible with the boundary while a private role was
authenticated by a public raw hidden leaf or a public linear commitment. Those
modes are dead for the active private-role route because they either export
hidden material or rely on the full-rank public-linear surface already rejected
by the rank/hiding probe.

This pass adds the active gate in Lean and Rust:

- `Descender/Path2NativeBoundary.lean` now has `RoleAuthenticationMode`,
  `ColumnProfileEntry.authenticationMode`,
  `ColumnProfilePrivateAuthModesSafe`, and a strengthened
  `ColumnProfileCompatibleWithBoundary`.
- `tools/parameter-search/src/production_reference.rs` now binds
  `Path2TwoViewColumnProfileEntryV1.authentication_mode` into
  `path2_two_view_column_profile_v1_digest`.
- The active two-view profile verifier rejects `public_path_to_raw_leaf`,
  `public_linear_commitment`, and unknown authentication modes before the
  transparent fail-closed proof handoff.

This does not instantiate the proof-family verifier. The remaining blocker is
to prove that the allowed modes actually correspond to parent-root role
subcommitment authentication, simulator-only consistency, and extractor-only
hidden role recovery in the concrete transparent proof system.

The same pass also tightened relation-row coverage for the active
native-boundary trace. `TraceRelationCoversNativeBoundary` now requires
explicit `openingTupleAccountBindingRow` and
`checkedCoreResidueCompatibilityRow` classes in addition to the older q-field,
range, mask, role-restriction, lifecycle/context, and bookkeeping classes. This
prevents the active relation-knowledge assumption from being discharged by a
profile that includes the obsolete masked core and generic lifecycle rows while
omitting the two account/spend handoff groups most likely to force redesign.
The Rust proof-family interface mirror now binds
`opening_tuple_account_binding_rows_required` and
`checked_core_residue_compatibility_rows_required` into
`path2_native_boundary_trace_proof_family_interface_v1_digest`, which is in
turn bound into the active transparent proof-system id through the capability
digest. Negative controls reject clearing either bit before verifier handoff.
The zero-knowledge/no-export assumption is also split now:
`NativeBoundaryTraceZeroKnowledge` requires `NoExportSoundness`,
`RoleAuthenticationMaterialNoExportSoundness`, and `MaskFreshnessSoundness`.
The Rust proof-family interface and capability digests bind corresponding
simulator/freshness flags, with negative controls for clearing them. This keeps
“no raw openings” from being treated as a complete hiding theorem.
The same active proof-family interface now binds the extractor-side
blinding/noise policy to policy id `1` and centered bound `2048`; Lean requires
`UsesSelectedBlindingNoisePolicy(policy)` before deriving
`BlindingNoiseBounded`, and Rust negative controls reject policy or bound drift.
`BlindingNoiseBoundSoundness` is also decomposed now:
`BlindingNoiseBoundRowSoundness` plus `BlindingNoiseRowsToBoundSoundness`
compose back into the full blinding/noise bound endpoint.
The knowledge side is also split more aggressively now:
`NativeBoundaryTraceKnowledgeSoundness` explicitly includes parent-trace
commitment extraction, double-trace binding, extracted-trace relation
knowledge, masked-opening consistency, and the composed parent-trace extraction
contract. The Rust proof-family interface binds matching requirement bits plus
QROM extractor rewind accounting and double-trace binding-break export.
Lean now separates QROM schedule binding from
`NativeBoundaryTraceQromExtractorRewindSoundness`; the active endpoint returns
both `qromScheduleBound` and `qromExtractorRewindBound` before range/blinding
closure.
Lean also now consumes `ParentTraceDoubleBindingSoundness` inside the
commitment/consistency/relation combiner for `ParentTraceExtractionSoundness`,
so the selected-profile atomic endpoint no longer merely carries the
double-binding theorem as an unused parameter.
Lean now also requires `UsesSelectedTraceKnowledgeModes(profile)` for the
parent-trace extraction contract, aligning the proof target with the Rust
profile's `fullTraceExtractable` and `fullWitnessRelationKnowledge` modes.
That selected trace-mode gate now also requires
`TraceRelationMeetsSelectedNativeBoundaryRowFloor`, with Lean and Rust both
pinning the active working native-boundary relation floor to `176` rows.
Lean also requires `TraceRelationMeetsSelectedNativeBoundaryClassFloors`, and
Rust digest-binds `native_boundary_relation_class_floors_required = true`, so
relation knowledge cannot close with all surplus rows assigned to irrelevant
classes.
The final Lean endpoint now routes through `ParentTraceExtractionSoundness` and
`TraceAssemblySoundness` directly; the old `TwoViewExtractionSoundness` surface
is no longer enough to discharge `transparent_view_full_interface_or_break`.
`TraceAssemblySoundness` is now decomposed into representative assembly
existence, role completeness, derived lifecycle/statement roles,
boundary-fiber projection, local validity, and descent consistency, with a
composition theorem back to the endpoint.
Lean now mirrors the selected active Rust column profile as
`SelectedActiveColumnProfile`, including hidden/masked column sets `101..109`
and `201..209`, and proves `selected_active_column_profile_role_complete`, so
the active profile map discharges `ProfileRoleComplete` for trace assembly.
Rust now binds `trace_assembly_derived_roles_required = true`, matching Lean's
`TraceAssemblyDerivedRolesSoundness` gate.
The preferred full-interface closure theorem is now
`transparent_view_full_interface_from_selected_profile_atomic_gates_or_break`,
which composes the selected active profile, parent-trace commitment extraction,
masked-opening consistency, relation-knowledge, decomposed assembly, same-trace
hybrid range rows, range row soundness, same-trace blinding/noise rows, and
blinding/noise row obligations. The Rust proof-family interface now binds
`lean_atomic_full_interface_endpoint_present = true`,
`lean_selected_profile_atomic_endpoint_present = true`,
`lean_account_admission_adapter_endpoint_present = true`, plus the decomposed
assembly/range/blinding row subclaim flags into its digest. Rust now also has a
direct pre-verifier selected-view bridge,
`verify_path2_direct_creation_transparent_descent_selected_view_bridge`, which
checks the active transparent public view/profile/query schedule, interface
policy flags, leaf and aggregation bindings, commitment API, and transparent
payload digest before the full payload boundary returns the intentional
proof-verifier-not-implemented blocker. This records the account-create-to-view
adapter needed by `AccountCreateSelectedTransparentViewSoundness` without
claiming that the concrete transparent IOP verifier exists.
The active range/SIS gate now also pins `selectedSisRoleMatrixRows = 64` and the
Rust proof-family interface binds `sis_role_matrix_rows = 64`; the existing
sanity probe rejects the 48-row `opening_tuple` case under the selected `2048`
blinding bound.
The norm/range gate now consumes
`UsesSelectedHybridBase128RangePolicy(profile, rangePolicy)`, which fixes
semantic policy id `1`, witness policy id `2`, private role rows `64`, digit
counts `12/12/16`, range rows `40`, reconstruction rows `20`, range witnesses
`40`, digit maximum `127`, extractor-hidden semantic range entries, and
range-private witness entries; Rust rejects range-policy id drift and range
witness visibility drift in the active two-view column profile.
`NormRangeSoundness` is no longer only monolithic: Lean now exposes
`HybridBase128RangeRowSoundness` and
`HybridBase128RangeRowsToNormRangeSoundness`, with a composition theorem back
to the full endpoint.
`TraceRangeRowsBindRepresentativeSoundness` ties those selected range rows to
the same extracted trace that assembles the hidden representative.
`BlindingNoiseBoundSoundness` is similarly decomposed into
`BlindingNoiseBoundRowSoundness` and `BlindingNoiseRowsToBoundSoundness`, with
`TraceBlindingRowsBoundRepresentativeSoundness` tying selected blinding/noise
bound rows to the same extracted trace.
Route clarity is now recorded in
`research/path2_proof_family_route_clarity.md`: the active transparent route is
viable only as an argument of knowledge for the full hidden native-boundary
trace with simulator-safe masked openings. Sampled/proximity-only Merkle/FRI
does not imply the needed extraction theorem.
Parallel route synthesis is now recorded in
`research/path2_parallel_proof_route_synthesis.md`: the transparent IOP remains
the active theorem-heavy baseline, MPC-in-the-head is the cleanest theorem
fallback with byte/verifier risk, descent/SIS is only viable as a private-ZK
replacement probe, and native image membership is a protocol-redesign probe.
The next two proof-family probes are now concrete:
`research/path2_transparent_iop_knowledge_theorem_probe.md` states the exact
transparent IOP-of-knowledge theorem surface, while
`research/path2_mith_full_witness_probe.md` gives the parallel MITH
full-witness theorem, optimistic wide-challenge byte floor, and conservative
64-round extrapolation. The current MITH read is theorem-clean but likely
byte-heavy unless challenge-width/QROM accounting cuts repetitions.
Lean now names the transparent account-admission bridge as
`AccountCreateSelectedTransparentViewSoundness` and proves
`selected_transparent_view_atomic_gates_imply_account_admission`, so the
selected transparent endpoint composes into `AccountAdmissionSoundness` once
accepted account creation supplies the selected view/profile/policies.
Rust now digest-binds that Lean account-admission adapter through
`lean_account_admission_adapter_endpoint_present`.
The legacy reviewed-Fq12 production preflight report no longer treats its
hidden-source callback, readiness-report/verdict, deployability assertion,
active-lane projection-carrier, pad-seed/spent-digest invariant, and
fail-closed deployment-verifier presence bits as unconditional literals. Those
bits are now derived from source-inspection gates over the active reviewed-Fq12
callback boundaries, readiness/deployability report constructors and
validators, selected projection-carrier tuple verifier, masked projection
candidate verifier, soundness certificate, digest-only storage/spent-ledger
regressions, and no-selected-material negative controls. Its final
`reviewed_fq12_deployment_verifier_present` bit is also derived from a
productization helper that requires hidden-source semantics closure and rejects
the current fail-closed verifier body. This is report-surface hardening only:
the active target remains the Path 2 native / reviewed-Fq12 active-lane
projection-carrier route, and production deployment remains fail-closed until
the hidden-source semantics, signer/verifier productization, constant-time,
storage, and external-review gates are closed.
The selected AP2-N001 evidence digest manifest now also emits the concrete
definition checklist digest and missing categories. Current manifest output
confirms that all six selected Rust evidence reports verify and have all facts
true, while the Lean closure remains fail-closed with
`extractor_theorem_instantiated = false`,
`semantics_bridge_complete = false`, checklist digest
`f1dc786697672712b03cbce6ab97124ac520d8cac569c6ffe5566270782ca3bd`, and
exactly four missing active categories:
`predicate_model_certificate_definitions`, `source_record_definitions`, and
`source_record_evidence_source_bindings`, and
`sub_evidence_digest_match_proof`. The selected Lean report/proof definitions
now alias verifier-backed Rust evidence, so `evidence_proof_definitions` is no
longer a blocker. The remaining proof-completeness work is to import the
selected source-record certificates, bind each imported source record to the
public evidence that claims to certify it, and prove that the aggregate
five-source report's sub-digests match the five component certificates,
without reviving any inactive path.
The current selected aggregate evidence digest is
`146570f815d27ca8b79ad52480f51a9bb7fb55c892e17a34fdeb9b0050cd1367`, and the
current bridge-audit digest embedded in the Rust-generated aggregate report is
`ae4a87958c5c0479df16f77b3e8467870dbdbec3e90c6410bcb79f4fac0c0e54`.
This exposed a closure-design constraint: naively hard-coding the aggregate
evidence digest or bridge-audit digest back into the Lean file changes the
Lean source that the bridge audit hashes, which in turn changes the Rust
aggregate digest. Treat those aggregate literals as externally witnessed
source-record data until the source-record import route breaks that
self-reference; do not chase them as if they were stable proof constants.
Lean now also states negative sanity lemmas for the current selected AP2-N001
report placeholders: each selected report is proved not to satisfy
`ReportVerified` and not to satisfy `FactsAllTrue`. These lemmas are not the
future proof objects and intentionally do not reduce the checklist missing
count; they make explicit that the present Lean report definitions are
fail-closed targets, not usable evidence.
The concrete-definition checklist now inspects only the selected declaration
header/type before `:=` or `where` when looking for report/proof/source markers.
A body-only source-record spoof fixture with every required AP2-N001 source
type marker in the definition bodies still leaves
`source_record_definitions_present = false`, so future source-record closure
must put the active types on the selected declarations themselves.
The same source-record category now also requires the five selected
proof-bearing source certificates:
`selectedRustBoundaryBaseCertificate`,
`selectedRustRoleRootLiftSourceCertificate`,
`selectedRustSourceRouteByteSourceCertificate`,
`selectedRustFieldLocalSourceCertificate`, and
`selectedRustSharedFrameWitnessSourcesCertificate`. This closes a checklist
blind spot where a synthetic complete fixture could define raw sources and
mention certificate names in the digest-match theorem without declaring the
certificate values that the final AP2-N001 route consumes.
The sub-evidence digest-match proof category now also requires the theorem
header/type to mention `selectedRustFiveSourceEvidenceReport`. A regression
fixture with the right certificate names but `otherFiveSourceEvidenceReport`
fails the checklist, so `selectedRustFiveSourceSubEvidenceDigestsMatchProof`
cannot satisfy the concrete-definition gate for the wrong aggregate report.
Lean now proves
`selectedRustFiveSourceEvidenceReport_subEvidenceDigestsMatch_forces_empty_certificate_digests`:
any `SubEvidenceDigestsMatch` proof against the current placeholder aggregate
report forces all five consumed certificate `evidenceDigest` fields to be
`selectedRustZeroDigest32`. This makes the placeholder/non-zero Rust digest mismatch
explicit in the proof layer without introducing any future proof-object names.
The final extractor-gate regression now treats the certificate endpoint as the
only accepted production-facing route: the positive fixture calls
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_certificate`
directly with `selectedRustModelFiveSourceCertificate`, while the older route
that derives `toClosureInputs` and calls the input theorem is an explicit
negative fixture. This keeps the source gate aligned with the proof-bearing
certificate boundary rather than accepting a syntactic detour through raw
inputs.
The evidence-digest definition checklist now requires each selected digest
declaration to carry `ByteArray` in its header/type. Untyped digest-name
definitions no longer satisfy `evidence_digest_definitions_present`; the real
Lean selected digest constants continue to pass, and the manifest digest/state
remain unchanged.
The evidence-proof definition checklist now requires the exact selected report
predicate in each proof declaration header/type, for example
`selectedRustFiveSourceEvidenceReport.ReportVerified`, instead of accepting
the selected report name and predicate name as independent substrings. A
regression fixture where a theorem mentions `selectedRustBoundaryBaseEvidenceReport`
in an unrelated premise but proves `otherBoundaryBaseEvidenceReport.ReportVerified`
fails `evidence_proof_definitions_present`, so future AP2-N001 proof imports
must prove the selected report predicates directly.
The sub-evidence digest-match checklist now follows the same exact-predicate
rule: the proof declaration header/type must contain
`selectedRustFiveSourceEvidenceReport.SubEvidenceDigestsMatch`. A split-marker
fixture that mentions `selectedRustFiveSourceEvidenceReport` in an unrelated
premise while proving `otherFiveSourceEvidenceReport.SubEvidenceDigestsMatch`
fails the category, so the digest-match proof must bind the selected aggregate
report directly.
Lean now also proves `selectedRustEvidenceDigests_nonzero` and
`selectedRustFiveSourceEvidenceReport_subEvidenceDigestsMatch_incompatible_with_selected_certificate_digests`.
Together with the earlier placeholder digest-match lemma, this shows that the
current placeholder aggregate report cannot match certificates carrying the
selected public Rust digest constants; replacing the placeholder with
verifier-backed selected evidence is a real proof obligation, not a naming
cleanup.
Lean now also proves
`selectedRustModelFiveSourceCertificate_not_from_current_placeholder_report`:
no final selected Rust/model five-source certificate can carry the current
placeholder aggregate report, because the certificate contains a
`ReportVerified` proof and the placeholder report is already proved not
verified.
Lean now extends the same sanity check to all five per-source certificates:
boundary-base, role-root/lift, selected source-route, field/local, and
shared-witness certificates cannot carry their current placeholder selected
reports. Each certificate exposes sources only behind `ReportVerified`, while
each placeholder report is proved not verified.
The concrete-definition checklist now treats declaration markers as ordered
header/type markers. The payload projection law source gate specifically
requires `P.reviewedFq12PayloadAccepts` before
`P.reviewedFq12PayloadProjectsFrom`; a synthetic complete fixture with the
payload theorem reversed still has its report/digest/proof and digest-match
categories present, but fails `source_record_definitions_present`.
The AP2-N001 anti-stub filter now also rejects vacuous selected source-route
evidence report/proof declarations. An all-true
`selectedRustSourceRouteEvidenceReport` with trivial report-verification and
facts-all-true proof definitions is treated as a forbidden source stub, matching
the existing aggregate, boundary-base, role-root/lift, field/local, and
shared-witness vacuity checks.
The final extractor theorem source gate now requires each local selected
certificate variable to be bound directly by its named constructor. A regression
where `ReviewedFq12SelectedRustBoundaryBaseEvidenceCertificate.ofReportAndSources`
is present but assigned to a differently named local, while
`selectedRustBoundaryBaseCertificate` appears later, fails the final theorem
gate. This blocks constructor-name laundering in the future AP2-N001 theorem
shape.
The same final theorem gate now also requires each constructor call to consume
the selected report, digest, proof objects, and source records in the intended
order. A regression where the boundary-base certificate constructor is called
directly but with `otherBoundaryBaseEvidenceReport` instead of
`selectedRustBoundaryBaseEvidenceReport` fails the gate.
Selected declaration matching now checks an identifier boundary after the
requested name. A synthetic otherwise-complete fixture where
`selectedRustFiveSourceEvidenceReport` is replaced by
`selectedRustFiveSourceEvidenceReportSpoof` fails
`evidence_report_definitions_present`, so suffixed declarations cannot satisfy
the AP2-N001 selected-definition checklist by prefix.
The final AP2-N001 theorem source gate now applies the same exact-name boundary
to the future instantiation theorem. An otherwise-valid constructor fixture with
the theorem declaration renamed to
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_instantiationSpoof`
is rejected, so a suffixed theorem cannot satisfy `extractor_theorem_instantiated`
by prefix.
The final theorem scanner now checks every candidate theorem prefix in the
active boundary source and lets the exact-name helper reject suffixed
candidates. A regression with a suffixed spoof before an exact valid theorem
still accepts the exact theorem, avoiding a fail-closed false negative when
nearby diagnostic/spoof declarations exist.
Active full-signature reduction gate note: the
`selectedRustFullSignatureReductionBoundary_instantiation_v1` scanner now uses
the same exact theorem-name and all-candidate pattern. A suffixed theorem name
is rejected, while a suffixed spoof before an exact valid theorem still lets the
exact theorem satisfy the gate. This keeps the downstream active deployability
gate robust without changing its current fail-closed status.
Active full-execution source gate note: the
`concreteFullSignatureExecutionSource_instantiation_v1` scanner now also uses
exact theorem-name matching across all candidates. A suffixed spoof is rejected,
but a suffixed spoof before an exact valid theorem still allows the real
full-execution source instantiation to satisfy the gate.
Active accepted-spend numeric-bound gate note: the
`selectedRustAcceptedSpendSoundnessBoundary_instantiation_v1` scanner now uses
the same exact theorem-name and all-candidate pattern. This keeps a suffixed
theorem spoof from closing the active accepted-spend bound gate, while allowing
a later exact selected theorem to satisfy the future source closure.
Active no-export serializer gate note: the
`selectedRustActiveNoExportSerializer_instantiation_v1` scanner now also uses
exact theorem-name matching across all candidates, so a suffixed no-export
theorem cannot close the active serializer gate and a later exact theorem
remains reachable.
Active checked-core selected-source gate note: the
`selectedRustCheckedCoreByteSource_instantiation_v1` scanner now uses exact
theorem-name matching across all candidates. A suffixed checked-core source
theorem cannot satisfy the active composition gate or mask a later exact
selected Rust byte-source instantiation.
Active full-execution boundary note: the
`concrete_fullExecutionImpliesAcceptedSpendForgery` boundary-presence check now
requires an exact theorem declaration name in the active Lean source. A suffixed
boundary theorem cannot make the full-execution source surface appear present.
Active no-export boundary note: the
`active_native_no_export_closure_from_public_output_no_export` boundary check
now also requires an exact theorem declaration name before the active serializer
boundary can be marked present.
Active checked-core boundary note: the checked-core byte bridge source gate now
requires exact endpoint theorem names for
`selectedRustCheckedCoreByteSource_checkedCoreAccepts_or_named_break` and
`selectedRustCheckedCoreByteSource_valid_or_named_break`, so suffixed endpoint
theorems cannot make the active byte bridge appear present.
Active concrete-definition blocker note: Lean now states
`selectedRustCurrentPlaceholderEvidenceProofDefinitions_impossible`, proving
that the retained fail-closed placeholder evidence reports cannot supply the
proof-bearing definitions required by the checklist. This is now a negative
control only: the active selected report/proof names have moved to
verifier-backed aliases.
Active source-record blocker note: Lean now also states
`selectedRustCurrentPlaceholderSourceRecordCertificates_impossible`, proving
that source certificates over the current placeholder component reports cannot
populate the selected Rust/model source-record package.
The concrete-definition checklist now also requires the five selected
source-certificate definitions to be built by their named constructors with
the selected report, digest, report-verification proof, facts proof, and raw
source arguments. A regression fixture that replaces
`selectedRustBoundaryBaseCertificate` with an anonymous certificate value keeps
`source_record_definitions_present = false`, so this category cannot be closed
by type-correct but unbound certificate literals.
Active sub-evidence blocker note: Lean now states
`selectedRustCurrentPlaceholderSubEvidenceDigestMatchProof_impossible`, proving
that the current placeholder aggregate report cannot provide the checklist's
sub-evidence digest-match proof for certificates carrying the selected public
Rust evidence digests.
Lean also states
`selectedRustFiveSourceEvidenceReport_subEvidenceDigestsMatch_of_selected_certificate_digests`:
the active verifier-backed aggregate report digest-matches any five selected
source certificates whose `evidenceDigest` fields equal the selected component
digests. This reduces the remaining sub-evidence proof work to certificate
construction and per-certificate digest equality; it intentionally does not
create `selectedRustFiveSourceSubEvidenceDigestsMatchProof` while the selected
certificates are still absent.
Active evidence import note: `path2_selected_evidence_report_manifest` now
prints the full selected Rust evidence report values, not only digests. The
manifest shows all six report/fact sets verify while
`extractor_theorem_instantiated` and `semantics_bridge_complete` remain false,
which is the next safe import target for replacing Lean placeholder reports
without flipping final closure.
Active Lean evidence import note: `Path2HiddenSourceSemanticsBoundary.lean`
now contains verifier-backed report values under `verifierBacked...` names, plus
Lean proofs that those imported reports satisfy `ReportVerified` and
`FactsAllTrue`. The active `selectedRust...` report names now alias those
verifier-backed values; final closure remains fail-closed because the selected
source-record certificates, the aggregate/sub-certificate digest-match proof,
and the aggregate digest self-reference are still unresolved.
The Rust focused bridge regression now parses the ByteArray fields inside
`verifierBackedSelectedRustFiveSourceEvidenceReport` and checks them against the
Rust evidence frame. This binds the imported bridge audit digest, hash-domain
evidence digest, and five sub-evidence digests at the record-field level, not
only through the public selected digest constants.
It also compares all verifier-backed aggregate and sub-report boolean fields to
their Rust evidence records, so source-audit coverage includes the fact flags
that Lean imports before the final source-record/digest-match closure.
Active source-record route note: the remaining constructive import target is
the selected Rust direct-assembly/canonical-representative package
`ReviewedFq12SelectedRustDirectAssemblyCanonicalRepresentativeByteAndWitnessPackagedSourceRouteInputs`.
The first nontrivial sub-constructor is
`ReviewedFq12FullNativeOpeningCanonicalRepresentativeByteSource`: Rust has the
canonical witness/frame evidence, but Lean still needs the selected parser
source that turns accepted semantic openings into canonical representative
witness byte certificates. The assembly digest-frame witness source proves
assembly-frame recomputation; it cannot by itself prove representative
canonicality or uniqueness.
The selected evidence digest manifest now includes `source_record_subcheck`.
Current output reports `missing_selected_source_record_count = 12`: all seven
raw selected source definitions and all five constructor-bound source
certificate definitions are absent. It also reports
`canonical_witness_to_byte_source_adapter_present = true` and
`byte_and_witness_packaged_route_endpoint_present = true`, confirming that the
Lean adapter route exists and the remaining work is importing concrete
selected source records, not inventing another endpoint.
Lean now also exposes
`ReviewedFq12SelectedRustModelSourceRecordPackage`, a generic package for the
seven raw selected source records. Its projections build the five selected
source certificates by their named constructors, and
`ReviewedFq12SelectedRustModelSourceRecordPackage.subEvidenceDigestsMatch`
derives the aggregate sub-evidence match from the selected component digests.
The manifest reports `source_record_package_surface_present = true` and
`package_sub_evidence_match_reduction_present = true`; this reduces the
remaining source-record work to importing the raw selected sources themselves,
while leaving aggregate evidence-digest closure separate.
The package now also projects to the active closure/input surfaces:
`toFiveSourceClosureInputs` builds the five-source closure input record,
`toByteComponentInputs` builds the field-local shared-frame byte-component
input record, and
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_source_record_package`
routes the package directly into hidden-source semantics closure. The manifest
reports all three package-reduction booleans true. This is still fail-closed:
the theorem consumes a genuine source-record package and does not construct
one from evidence reports.
The source-record package also now reaches both handoff surfaces:
`with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_source_record_package`
for accepted-spend soundness and
`full_signature_with_reviewed_fq12_hidden_source_closure_from_selected_rust_model_source_record_package`
for the active full-signature boundary. The Rust focused audit gate requires
these markers, so the package route cannot regress back to loose input-record
handoff without being detected.
Boundary-base source check: a direct scan of the active Lean path shows the
selected boundary-base report and its `ReportVerified`/`FactsAllTrue` proofs
are evidence only. They do not construct
`selectedRustPayloadProjectionLaw`,
`selectedRustExtractionCertificateAvailability`, or
`selectedRustBoundaryBaseCertificate`; those are semantic selected-source
records over the concrete `Predicates` interpretation. The Rust subcheck now
emits `boundary_base_report_only_not_source_record_closure = true` for the
current active file, with a regression showing the flag clears only when the
raw source terms and named boundary-base certificate constructor are present.
Role-root/lift source check: the selected role-root/lift report likewise
verifies only the public evidence bits. It does not construct
`selectedRustRoleRootLiftSource` or
`selectedRustRoleRootLiftSourceCertificate`; the source itself is the pair of
semantic obligations `ReviewedFq12ConcreteRoleRootFieldBindingCertificate` and
`ReviewedFq12CertificateLiftOpeningFunctionality`. The source-record subcheck
now emits
`role_root_lift_report_only_not_source_record_closure = true` for the active
file and has a fixture proving that report/proof imports alone do not satisfy
the selected source gate.
All selected source categories now have the same explicit report-only guard.
The active manifest reports true for
`boundary_base_report_only_not_source_record_closure`,
`role_root_lift_report_only_not_source_record_closure`,
`source_route_report_only_not_source_record_closure`,
`field_local_report_only_not_source_record_closure`, and
`shared_witness_report_only_not_source_record_closure`. This confirms the
current state precisely: all five selected evidence reports/proofs are
imported and verified, but none of the seven raw selected source records or
five constructor-bound source certificates exists yet.
Concrete predicate-model check: the existing concrete full-spend package
`Path2FullSpendConcrete.concretePredicates` instantiates
`Path2FullSpendGame.Predicates`, not `Path2NativeBoundary.Predicates`. The
seven remaining selected source records are all propositions over a
`Path2NativeBoundary.Predicates` value, so the next import cannot be another
evidence report. It must either provide the concrete native-boundary predicate
interpretation for the active Rust verifier, or provide a theorem package
whose fields are exactly the seven source records for the `P` consumed by the
hidden-source boundary theorem.
The selected evidence manifest now exposes this predicate-model blocker
directly: `native_boundary_predicates_record_present = true`,
`native_boundary_semantics_package_target_present = true`,
`full_spend_concrete_predicates_are_separate_model = true`,
`native_boundary_concrete_predicate_model_present = false`, and
`selected_source_records_blocked_on_native_boundary_predicate_model = true`.
The generic predicate-model source labels have been removed, so
`native_boundary_predicate_model_remaining_generic_evidence_label_count = 0`
and the predicate negative-control bit is no longer held open by placeholder
`Prop` fields.
The target route now names the proof-bearing replacement surface:
`native_boundary_predicate_model_evidence_report_target_present = true`,
`native_boundary_predicate_model_certificate_target_present = true`, and
`native_boundary_predicate_model_certificate_to_source_present = true`.
The certificate is the intended import object for the concrete active Rust
native-boundary predicate interpretation before source records are supplied.
That route is now packaged as
`ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage`, and
the manifest reports certified-package target, handoff, and bound-handoff bits
as true. It still remains conditional on importing the concrete certificate
and source records.
Rust now emits the matching
`Path2ReviewedFq12NativeBoundaryPredicateModelEvidenceV1` report. The report
is digest/schema verified in the manifest but remains fail-closed:
`predicate_model` all-facts is false because no concrete native-boundary
predicate model is present.
Lean now mirrors that report and digest as
`selectedRustNativeBoundaryPredicateModelEvidenceReport` and
`selectedRustNativeBoundaryPredicateModelEvidenceDigest`, with a proof that
the current report is verified and a proof that `FactsAllTrue` is impossible.
The concrete-definition checklist records this as a completed predicate-model
report surface but keeps the facts-proof and certificate sub-bits false.
This pins the current failure mode as a missing concrete native-boundary
semantics import, not a missing byte/digest evidence report.
Lean now names that import target as
`ReviewedFq12SelectedRustNativeBoundarySemanticsPackage`: it contains the
concrete `Path2NativeBoundary.Predicates` value plus the selected source-record
package for that exact predicate interpretation, and exposes
`.hiddenSourceClosure`.
The predicate-model source projection is now prefix-bound as well:
`ReviewedFq12SelectedRustNativeBoundaryPredicateModelSource.toPredicates` must
return `Model.predicates`; an alias return with an unused `Model.predicates`
mention no longer satisfies the source-target subcheck.
The certified semantics source-package target bit now inspects the named
structure and methods directly, and requires the certified package
`hiddenSourceClosure` body to start from
`(Package.toSemanticsSourcePackage).hiddenSourceClosure`; unrelated marker
placement or an alias-returning closure no longer satisfies that evidence bit.
The older concrete/source-shaped semantics package target bits are now
declaration-scoped too: the concrete package closure must start from the
selected source-record package theorem, and the source-shaped package
`toSemanticsPackage` projection must start from
`Package.predicateModel.toPredicates` plus `Package.sourceRecords`.
The same semantics package now has accepted-spend and full-signature handoff
theorems, and the manifest reports
`native_boundary_semantics_package_handoff_present = true`. This means the
path from the future concrete native-boundary semantics package to the active
full-signature boundary is now mechanical; the package itself remains absent.
The AP2-N001 final theorem source gate now requires that future closure to
route through
`ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage`, not
just through the older five-source certificate. The accepted synthetic theorem
shape must name the predicate-model evidence report/digest/proofs, consume
`selectedRustNativeBoundaryPredicateModelCertificate`, bind
`sourceRecords := selectedRustModelSourceRecordPackage`, expose
`.hiddenSourceClosure`, and call both the certified-package full-signature
handoff and
`full_signature_active_path_forgery_bound_from_selected_rust_native_boundary_certified_semantics_source_package`.
Negative controls reject an assumed or wrong predicate-model certificate, raw
predicate-model/source-record package stubs, and omission of the active-bound
handoff. This keeps `extractor_theorem_instantiated` from closing on the stale
five-source-only route.
The final AP2-N001 scanner now also requires the theorem to return
`selectedRustNativeBoundaryCertifiedSemanticsSourcePackage.hiddenSourceClosure`
through the final `exact`; merely mentioning that closure in an unused local
binding is not enough to instantiate the extractor theorem gate.
It also requires the local
`selectedRustNativeBoundaryCertifiedSemanticsSourcePackage` binding itself to
be constructed by
`ReviewedFq12SelectedRustNativeBoundaryCertifiedSemanticsSourcePackage.ofCertificateAndSourceRecords`;
mentioning that constructor in an unused local while binding the package to an
alias no longer closes the final gate.
`ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate` now also has
named constructor `ofReportAndModel`, and the concrete-definition checklist
requires `selectedRustNativeBoundaryPredicateModelCertificate` to be built
through it. A same-name/same-type anonymous predicate-model certificate no
longer completes the predicate-model certificate category.
That constructor check is now prefix-bound on the certificate definition body:
an alias-bound `selectedRustNativeBoundaryPredicateModelCertificate` plus an
unused `ofReportAndModel` mention no longer completes the predicate-model
certificate category.
The predicate-model certificate checklist now also requires the constructor
body to name every selected native-boundary model obligation argument,
including payload/boundary binding, extraction consistency, role-lift,
assembly, compatibility, and full-opening bridge digest binding. A certificate
body missing one of those obligations no longer completes the category.
The selected source-record package is now constructor-bound too:
`ReviewedFq12SelectedRustModelSourceRecordPackage.ofSources` is the required
route for `selectedRustModelSourceRecordPackage`, and an anonymous package with
the right name/type no longer completes `source_record_definitions`.
The source-record/package constructor checks now require the relevant
definition bodies to start with their named constructors; an alias-bound
`selectedRustModelSourceRecordPackage` plus an unused `ofSources` mention is
rejected by both the concrete-definition checklist and source-record subcheck.
The same prefix-body rule now applies to the five selected evidence
certificate constructor-bound bits in the source-record subcheck, so an
alias-bound certificate plus an unused constructor mention does not reduce the
missing selected-source-record count.
The source-record subcheck now counts that selected package definition as part
of `all_selected_source_records_present`, so the seven raw source records and
five source-certificate constructors cannot be reported complete unless the
final package consumed by AP2-N001 is also present.
Those source records must now be typed over
`selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates`; a
generic-`P` source package no longer completes the source-record category.
The final AP2-N001 theorem gate also requires
`selectedRustModelSourceRecordPackage` to be constructed through
`ReviewedFq12SelectedRustModelSourceRecordPackage.ofSources`.
It now rejects a final theorem over generic `P : Predicates`; the conclusion
must target the certified predicate model exposed by
`selectedRustNativeBoundaryPredicateModelCertificate.toSource.toPredicates`.
The certified native-boundary semantics package now has constructor
`ofCertificateAndSourceRecords`, and the final AP2-N001 theorem gate requires
that constructor before it accepts `.hiddenSourceClosure` or the
certified-package active-bound handoff.
The aggregate sub-evidence digest-match proof is now route-bound:
`selectedRustFiveSourceSubEvidenceDigestsMatchProof` must derive from
`selectedRustModelSourceRecordPackage.subEvidenceDigestsMatch`, not from an
anonymous same-type proof.
That digest-match proof check is now prefix-bound on the proof body; an
alias-bound proof plus an unused mention of
`selectedRustModelSourceRecordPackage.subEvidenceDigestsMatch` no longer
completes the sub-evidence category.
The selected source-record package also now projects directly to
`ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs` through
`toFieldLocalSharedDigestFrameWitnessInputs`, and the Rust source-record
subcheck records that reduction. This keeps the active source package tied to
the implementation-oriented field/local shared-frame theorem surface, while
leaving the concrete selected source definitions fail-closed.
The final AP2-N001 theorem gate now requires the accepted theorem body to bind
that projection as `selectedRustFieldLocalSharedDigestFrameWitnessInputs`;
omitting the projection no longer satisfies the scanner.
It also rejects theorem-header assumptions for the individual predicate-model
obligations and rejects local `let` shadowing of
`selectedRustNativeBoundaryPredicateModelCertificate`, so the final theorem
must consume the selected certified predicate model rather than rebuilding it
inside the theorem body.
The selected predicate-model report is now fact-complete for the active source:
Rust reports `predicate_model = true`, the concrete definition checklist has
all categories present, and the old current-report impossibility guards have
been retired. The final theorem still must consume the selected certified
predicate model rather than rebuilding a local certificate.
The `native_boundary_concrete_predicate_model_present` detector now requires
the full active native-boundary reviewed-Fq12 predicate surface, including
certificate extraction, role-lift, assembly, compatibility, full-opening bridge
digest, payload extraction, and break predicates. A partial selected
`Path2NativeBoundary.Predicates` stub no longer flips the predicate-model bit.
The detector is now also scoped to the body of the actual
`selectedRustNativeBoundaryPredicates` declaration: unrelated declarations in
the same Lean module cannot contribute missing predicate-field markers.
Concrete predicate-model completion now requires the active model to route
through the named field-source package:
`selectedRustNativeBoundaryPredicateFieldSources` must package the selected
predicate-field providers, and `selectedRustNativeBoundaryPredicates` must be
the package's `toPredicates` projection. A raw field-complete
`Path2NativeBoundary.Predicates` literal no longer satisfies the concrete
model gate, and the obvious vacuous-lambda detector also scans the field-source
package body. The manifest exposes the package-route bit as
`native_boundary_predicate_model_field_source_package_present`.
The proof-bearing predicate-model certificate is now package-routed too:
`ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate` carries the
field-source package and records that its `predicates` value is the package's
`toPredicates` projection. Its named constructor takes
`selectedRustNativeBoundaryPredicateFieldSources`, not a raw
`selectedRustNativeBoundaryPredicates` value, and the Rust checklist rejects
the raw-predicate constructor route. The digest-bound
`nativeBoundaryPredicateRecordComplete` fact now also requires the package
route bit, so provider declarations alone cannot close the predicate-record
fact.
That package-route fact is now independently digest-bound in the selected
predicate-model evidence report as
`nativeBoundaryPredicateFieldSourcePackagePresent` /
`native_boundary_predicate_field_source_package_present`. The current report
and manifest keep it false, and the selected predicate-model evidence digest is
updated to bind the extra false fact rather than hiding it inside
`nativeBoundaryPredicateRecordComplete`.
The Lean predicate-model evidence report now mirrors the Rust report's
concrete-model and certified-semantics-package status fields as
`concretePredicateModelPresent` and
`certifiedSemanticsSourcePackageTargetPresent`, and `FactsAllTrue` requires
both. The current selected report remains impossible because the concrete
predicate model is still absent.
The selected evidence manifest now emits every predicate-model all-facts input
from the Rust evidence frame, not only the digest, top-level model bit, and
negative-control bit. This makes the active blocker inspectable without
changing the fail-closed status.
The Rust source audit now scopes predicate-model source/report/certificate
target detection to the named Lean declarations, with a split-marker negative
fixture. Unrelated declarations can no longer contribute missing fields to
these target bits.
The selected source-record completion bit is now explicitly gated on the
concrete native-boundary predicate model. A fixture with every source-record
marker but no concrete `selectedRustNativeBoundaryPredicates` model remains
non-closing; only the combined concrete-model plus source-record fixture closes
the source-record subcheck. This prevents source-record package markers from
floating over an absent active predicate interpretation.
The concrete-definition checklist is gated the same way: the predicate-model
certificate must pass `selectedRustNativeBoundaryPredicates` to
`ofReportAndModel`, and `predicate_model_certificate_definitions_present` /
`all_definitions_present` remain false unless the full concrete predicate model
is also detected. This blocks a generic `P` certificate fixture from closing
the active definition checklist.
The concrete native predicate-model detector now requires the full
`Path2NativeBoundary.Predicates` field surface, including account/admission,
transparent-view, trace-assembly, slot-authorization, and spend predicates, not
only the reviewed-Fq12 certificate chain.
It also requires concrete declarations for every selected predicate-field
provider consumed by `selectedRustNativeBoundaryPredicates`; a field-complete
record with dangling provider identifiers no longer counts as the active
predicate model. The manifest exposes this as
`native_boundary_predicate_model_field_sources_defined_present`.
That provider-definition requirement is now folded into the digest-bound
predicate-model evidence report through `nativeBoundaryPredicateRecordComplete`;
the current selected Lean report and digest mirror Rust with that bit false.
The source audit now also rejects vacuous selected predicate-field provider
bodies, such as a provider definition of
`selectedReviewedFq12PayloadAccepts` as a constant-false predicate. The manifest
exposes this as
`native_boundary_predicate_model_forbidden_vacuous_field_sources_present`, and
the digest-bound predicate-model negative-control bit requires it to be false.
That rejection now applies generically to direct constant-lambda provider
bodies (`True`/`False` or `fun _ ... => True/False`) across the selected
predicate-field provider surface, including simple parenthesized wrappers. A
regression fixture covers `selectedAdmitted := fun _ _ => True` and
`selectedAdmitted := (fun _ _ => True)`, so the active account-admission leg
cannot be closed by an always-true placeholder outside the reviewed-Fq12 list.
It now also rejects the most direct vacuous literal model shapes inside
`selectedRustNativeBoundaryPredicates`: universal break predicates,
always-false reviewed-Fq12 payload acceptance, and constant-true reviewed-Fq12
certificate/extraction/spend predicates. The manifest exposes this as
`native_boundary_predicate_model_forbidden_vacuous_literals_present`, and the
digest-bound predicate-model report folds it into
`predicate_negative_controls_passed`, so a field-complete record still cannot
count as the concrete active model if its critical predicates are filled by
constant placeholders.
It also rejects module-level `axiom`, `constant`, `opaque`, or `variable`
stubs for the selected predicate-field providers used by
`selectedRustNativeBoundaryPredicates`. The manifest exposes this as
`native_boundary_predicate_model_stub_field_sources_present`, and the
digest-bound negative-control bit requires it to be false.
The same stub bit now also rejects obvious `selected...Value` dependency stubs
for those providers, so a provider definition such as
`selectedAdmitted := selectedAdmittedValue` cannot be completed by an axiom or
constant named `selectedAdmittedValue`.
Those selected predicate-field providers must now be concrete `def`/`abbrev`
value declarations; theorem- or instance-shaped declarations no longer satisfy
the provider-completeness gate. A regression fixture replaces a critical
reviewed-Fq12 provider with a theorem declaration and confirms that both
`native_boundary_predicate_model_field_sources_defined_present` and the
concrete-model bit remain false.
The same subcheck now exposes
`native_boundary_predicate_model_incomplete_proof_terms_present`; `sorry` or
`admit` tokens outside comments/strings block concrete-model completion and
the digest-bound negative-control bit.
The focused bridge regression now also parses
`selectedRustNativeBoundaryPredicateModelEvidenceDigest` from Lean and compares
it to the Rust-computed predicate-model evidence digest, so the predicate-model
report cannot silently drift away from the selected public manifest digest.
It now also parses every boolean field in
`selectedRustNativeBoundaryPredicateModelEvidenceReport` and compares the Lean
record against `Path2ReviewedFq12NativeBoundaryPredicateModelEvidenceV1`. This
keeps the current fail-closed predicate-model report byte/digest binding and
its exposed blocker bits synchronized with Rust evidence.
The active Path 2 source scanner now preserves UTF-8 Lean symbols while
stripping comments and strings, so declaration-scoped gates can safely match
Lean propositions containing `∧` without marker corruption. The early
reviewed-Fq12 extraction-chain target checks for payload acceptance,
extraction-certificate payload binding, extraction-certificate validity,
certificate-to-payload extraction consistency, payload fiber membership, and
payload native validity now require the named target definition body to start
with the concrete quantified obligation. Regression fixtures alias the real
payload-acceptance and payload-native-valid targets while moving the old
markers into unused examples; both must now fail closed. The current manifest
still reports these six target bits true for the active Lean source, while the
actual closure blockers remain unchanged:
`predicate_model = false`, `missing_selected_source_record_count = 13`, and
missing categories `predicate_model_certificate_definitions`,
`source_record_definitions`, `source_record_evidence_source_bindings`, and
`sub_evidence_digest_match_proof`.
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
The selected evidence/source binding definitions now have named constructors
for all five families, and the Rust checklist requires selected binding
definitions to use those constructors with the selected evidence report,
evidence digest, source-binding digest, digest-bound proof, and source
projection. Anonymous binding record literals no longer satisfy
`source_record_evidence_source_bindings_present`.
The module-level final theorem gate now also requires this source-binding
category before `extractor_theorem_instantiated` can become true. A
constructor-shaped theorem snippet without the binding definitions remains
accepted only by the body-shape scanner and is rejected by the real module
gate, so the public AP2-N001 status cannot advance ahead of the selected
evidence/source binding work.
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
The module-level final theorem gate now requires the full concrete-definition
checklist, not only the evidence/source-binding subcategory, before
`extractor_theorem_instantiated` can become true. Its body-shape scanner also
rejects local same-name reconstruction of the five selected source
certificates and `selectedRustModelSourceRecordPackage`, so the final theorem
must consume the selected global certificate/package definitions checked by
the checklist rather than shadowing them with locally rebuilt objects.
Each selected evidence/source binding object now also carries
`evidenceDigest` and `evidenceDigestBound`, and the Rust checklist requires
future binding definitions to set those fields to the selected component
evidence digest with `rfl`. This keeps report-bound source records tied to
the same public digest constants consumed by the selected source certificates,
instead of allowing an arbitrary source-binding digest to be the only digest
field on the binding object.
The source-binding digest constants themselves are now body-bound aliases of
the selected component evidence digests. A binding definition whose
`sourceBindingDigest` name points at a different digest no longer satisfies
`source_record_evidence_source_bindings_present`.
Each selected evidence/source binding now also carries
`sourceBindingDigestBound`, proving its public binding digest equals the
binding object's selected evidence digest. The Rust body-shape checklist
requires `sourceBindingDigestBound := rfl` for every future selected binding,
and a fixture that omits the field no longer satisfies
`source_record_evidence_source_bindings_present`.
The selected source-record package now derives an explicit
`fiveSourceClosureCertificate` from the package's constructor-bound per-source
certificates and `Package.subEvidenceDigestsMatch`. The package-to-hidden-source
closure theorem routes through that certificate's `toClosureInputs`, so the
aggregate five-source report and sub-evidence digest proof are proof-critical
for package closure rather than a final-theorem side construction. The Rust
source gate requires this package-derived certificate route and rejects aliases
for either the certificate constructor or the package closure reduction.
The proof-critical aggregate five-source certificate now also carries
`evidenceReportBound` and `evidenceDigestBound`, and every selected
five-source constructor route must pass `rfl rfl` after the aggregate evidence
digest. This prevents the aggregate certificate from being built over an
unrelated report/digest pair while still using selected per-source
certificates.
The selected source-record package now retains the five selected
evidence/source binding objects and carries equality witnesses tying every raw
source projection back to the corresponding binding. The package can no longer
erase the selected public-evidence bindings after construction and then feed an
unrelated raw payload-projection, availability, route, local, or witness source
into the aggregate closure.
The final AP2-N001 theorem gate now also rejects raw
`ReviewedFq12SelectedRustNativeBoundarySemanticsPackage` and source-shaped
`ReviewedFq12SelectedRustNativeBoundarySemanticsSourcePackage` closure routes.
The final proof must close through
`selectedRustNativeBoundaryCertifiedSemanticsSourcePackage.hiddenSourceClosure`,
so the predicate-model certificate remains on the active closure path.
The same current-report blocker is now propagated at the full-signature bound
layer:
`full_signature_active_path_forgery_bound_from_current_predicate_report_impossible`
reduces any certified semantics package back to the selected predicate-model
certificate impossibility, so the active bound cannot be claimed from the
current fail-closed predicate evidence.
Lean now exposes
`ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources`, a structured
field-source package for the eventual selected native-boundary predicate
interpretation, plus its canonical `toPredicates` projection. The source audit
requires this target shape and checks that critical break, reviewed-Fq12,
trace/fiber, slot-authorization, and spend predicates are routed through the
package. This does not close the predicate model: the selected concrete field
source definitions remain absent and the manifest must stay fail-closed.
The predicate-model certificate now has the same selected public-evidence
discipline: it carries `evidenceReportBound` and `evidenceDigestBound` fields
tying its report and digest to
`selectedRustNativeBoundaryPredicateModelEvidenceReport` and
`selectedRustNativeBoundaryPredicateModelEvidenceDigest`. The selected
certificate constructor/checklist requires `rfl rfl` for those equalities, and
the fail-closed current-report theorem uses `Cert.evidenceReportBound`
internally rather than relying on an external report-equality premise.
The predicate-model evidence-report target gate now requires the Lean
`FactsAllTrue` definition to include every public predicate-model fact:
extraction, full-opening/assembly, compatibility, break-model,
record-completeness, verifier-route, negative-control, concrete-model, and
certified-package target bits in addition to payload/model pins. A weakened
`FactsAllTrue` definition that drops one fact no longer satisfies the report
target bit.
The selected evidence/source binding route now projects the five package-level
source certificates from the binding objects themselves:
`Package.boundaryBaseEvidenceSourceBinding.toCertificate`,
`Package.roleRootLiftEvidenceSourceBinding.toCertificate`,
`Package.sourceRouteEvidenceSourceBinding.toCertificate`,
`Package.fieldLocalEvidenceSourceBinding.toCertificate`, and
`Package.sharedWitnessEvidenceSourceBinding.toCertificate`. The package
sub-evidence digest proof now consumes each binding's `evidenceDigestBound`
field rather than relying on definitional coincidence. The Rust source audit
requires this package-certificate route and still reports the active manifest
fail-closed with the same four missing concrete categories:
`predicate_model_certificate_definitions`, `source_record_definitions`,
`source_record_evidence_source_bindings`, and
`sub_evidence_digest_match_proof`.
The future final AP2-N001 theorem gate now mirrors that same route: its
synthetic positive shape binds `selectedRustModelFiveSourceCertificate` from
`selectedRustModelSourceRecordPackage.fiveSourceClosureCertificate` and then
derives the side hidden-source closure through
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_certificate`.
An alias-bound theorem that manually rebuilds the certificate while merely
mentioning the package projection is rejected, so the final theorem cannot use
the package-derived certificate as a decorative marker.
The certified semantics package's own `hiddenSourceClosure` now calls
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_source_record_package`
directly on `Package.predicateModelCertificate.toSource.toPredicates` and
`Package.sourceRecords`. The source audit rejects an alias body that only
mentions that reduction, so the certified package closure no longer detours
through the older source-shaped semantics adapter.
The current-report predicate-model blocker is closed. The active Lean source now
defines `selectedRustNativeBoundaryPredicates` as the selected field-source
package projection, builds
`selectedRustNativeBoundaryPredicateFieldSourcePackageCertificate`, and builds
`selectedRustNativeBoundaryPredicateModelCertificate` from the fixed-point
source and fact-complete predicate-model report. The remaining AP2-N001 blocker
is no longer source-record availability; it is the final extractor theorem and
semantics bridge instantiation.
Scanner hygiene update: the concrete definition checklist now computes
`native_boundary_concrete_predicate_model_present` through a direct active Lean
source marker check rather than recursively consulting the source-record
subcheck. This preserves the current fail-closed predicate-model status while
making the checklist independent from future changes to the subcheck report
shape.
Selected source-record bottleneck: Rust evidence exists for the source-route,
canonical representative witness, native digest-frame witness, direct assembly
binding, and local model, but Lean has not instantiated
`selectedRustSharedFrameWitnessSources`. That source record is the first
reviewed-Fq12 subroute item to build because it must prove the representative
canonicality side and cannot be replaced by assembly-frame recomputation alone.
Lean now also exposes
`reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_certificate`,
which consumes
`ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate.toWitnessSources`
instead of raw shared witness sources. The Rust source audit requires this
wrapper before accepting the shared digest-frame witness endpoint surface.
The source-record package hidden-source closure now calls the corresponding
certificate-level boundary theorem with
`Package.sharedFrameWitnessSourcesCertificate`; it no longer unwraps the
package through five-source closure inputs solely to pass raw `I.witnessSources`.
Package projections now route shared witness inputs through
`Package.sharedFrameWitnessSourcesCertificate.toWitnessSources`, with
`sharedFrameWitnessSourcesFromCertificate` recording equality to the raw package
field. The scanner requires this certificate projection in the package-to-input
reductions.
The source-record subcheck now splits the missing selected native-boundary
predicate providers into three families. Current active manifest state has all
three absent: core/admission-spend providers, transparent/trace/hash-row
providers, and reviewed-Fq12/full-opening providers
(`native_boundary_predicate_model_missing_provider_family_count = 3`).
The active Lean target now also names the reviewed-Fq12/full-opening provider
family separately as
`ReviewedFq12SelectedRustNativeBoundaryReviewedFq12PredicateFieldSources`, with
`ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSources.toReviewedFq12PredicateFieldSources`
projecting it from the full predicate-source record. The manifest reports
`native_boundary_predicate_model_reviewed_fq12_provider_family_target_present =
true`, but the actual reviewed-Fq12 provider fields remain undefined and
fail-closed.
The source audit now also guards against a circular provider shortcut:
reviewed-Fq12 selected predicate definitions are not allowed to count as
concrete if they reach into the downstream source-record/source-route proof
stack. Those theorems are `P`-parameterized and already assume the predicate
fields, so they can discharge laws for an independently defined provider but
cannot define the provider. The active manifest reports the circular-dependency
bit false.
Mapping the reviewed-Fq12 provider fields against Rust shows that payload,
extraction, role-lift, assembly, compatibility, and bridge-digest validity each
have verifier functions, but the fields that produce a native
`HiddenRepresentative` still depend on the canonical representative/shared
digest-frame witness route. See the Path 2 burndown for the field-by-field
mapping.
The manifest now exposes those five representative/extraction provider
obligations as their own target/source subchecks:
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_provider_family_target_present`
is `true`, meaning the Lean proof surface names the five-field hard slice, and
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_provider_fields_defined_present`.
is currently `false`, independently of the broader reviewed-Fq12 provider
family bit. The positive source-audit fixture asserts that the implementation
bit only flips when all five named providers are actually defined.
The Lean boundary now also projects both the full reviewed-Fq12 provider
family and the five-field hard-extraction subfamily through
`ReviewedFq12SelectedRustNativeBoundaryPredicateFieldSourcePackageCertificate`
and `ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate`, so
downstream source-record construction can consume the proof-bearing certificate
surface instead of reaching into raw provider records. The Rust source audit
pins that projection with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_provider_certificate_projection_present`.
The proof side now also names the corresponding hard-extraction law bundle,
`ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelLaws`, with
source/certificate projections pinned by
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_provider_law_bundle_target_present`.
This separates "define the five predicates" from "prove their native
extraction/assembly laws" for the next implementation pass.
The Lean target also packages those together as
`ReviewedFq12SelectedRustNativeBoundaryHardExtractionPredicateModelPackage`;
the manifest pins that with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_provider_package_target_present`.
The circular-provider guard now also rejects selected reviewed-Fq12 provider
definitions that reach into this hard-extraction package, its law bundle, or
the package/law projections; the regression covers
`selectedFullNativeOpeningAssemblesRepresentative` directly.
The proof side now has digest-frame-based constructors for the hard provider
slice:
`reviewedFq12HardExtractionPredicateFieldSourcesFromDigestFrame` builds the
five representative/extraction predicates from one native representative
digest frame, with immediate lemmas for assembly-implies-full-assembly and
certificate-extraction-implies-payload-extraction. The manifest tracks this as
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_digest_frame_provider_constructors_present`.
That constructor now also has a certificate-shaped active route:
`reviewedFq12HardExtractionPredicateFieldSourcesFromSharedWitnessCertificate`
consumes the report-verified
`ReviewedFq12SelectedRustSharedDigestFrameWitnessEvidenceCertificate` and
projects its shared digest frame into the hard-provider constructor. The
manifest tracks this as
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_shared_witness_certificate_route_present`.
The next target shape is also named:
`ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate`
packages the shared-witness certificate route, the equality proof that those
five fields are exactly the selected predicate model's hard fields, and the
hard-extraction law bundle. The manifest tracks this target with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_shared_witness_model_certificate_target_present`.
Lean now also reduces the hard-extraction law burden:
`reviewedFq12HardExtractionLawsOfSharedWitnessCertificate` builds the hard law
bundle from the shared-witness field equality plus only
`ReviewedFq12PayloadExtractionInBoundaryFiber` and
`ReviewedFq12PayloadExtractionNativeValid`. The manifest tracks this with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_shared_witness_law_reduction_present`.
The remaining native-validity source is now explicit:
`ReviewedFq12SharedWitnessFullOpeningNativeValiditySource` only has to prove
fiber/local/descent for the full-opening assembly witness exposed by
shared-witness payload extraction, and
`reviewedFq12HardExtractionLawsOfSharedWitnessFullOpeningSource` turns that
source plus the field equality into the full hard law bundle. The manifest
tracks this with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_full_opening_native_source_target_present`.
That native source has been reduced one step further to the exact canonical
representative:
`ReviewedFq12SharedWitnessCanonicalFullOpeningNativeValiditySource` asks for
fiber/local/descent of
`reviewedFq12FullNativeOpeningCanonicalRepresentative` under the
report-verified shared digest frame, and
`reviewedFq12SharedWitnessFullOpeningNativeValiditySourceOfCanonicalSource`
turns it into the full-opening native source. The manifest tracks this with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_canonical_native_source_target_present`.
The shared-witness hard model certificate now has a constructor,
`ReviewedFq12SelectedRustNativeBoundaryHardExtractionSharedWitnessPredicateModelCertificate.ofCanonicalNativeSource`,
that builds the certificate from the shared-witness certificate, the hard-field
equality, and the canonical native-validity source, deriving the law bundle
internally. The manifest tracks this with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_model_certificate_constructor_present`.
The hard route is now bridged back to the full predicate-model certificate:
`hardExtractionSharedWitnessPackageForPredicateModelCertificate` and
`hardExtractionSharedWitnessPackageFieldsMatchPredicateModelCertificate` show
that a shared-witness hard certificate over `ModelCert.predicates` has the same
hard provider fields as `ModelCert.toHardExtractionPredicateModelPackage`. The
manifest tracks this with
`native_boundary_predicate_model_reviewed_fq12_hard_extraction_predicate_model_bridge_present`.
The active Lean boundary now defines the selected native-boundary predicate
provider surface: core/admission/spend fields, transparent trace/hash-row
fields, the reviewed-Fq12 non-representative fields, and the five
shared-witness-backed hard extraction fields. `selectedRustNativeBoundaryPredicateFieldSources`
packages those definitions, with the representative-producing fields still
parameterized by the report-verified shared-witness certificate. The manifest
now reports `native_boundary_predicate_model_missing_provider_family_count = 0`,
`native_boundary_predicate_model_field_sources_defined_present = true`,
`native_boundary_predicate_model_field_source_package_present = true`, and
`native_boundary_predicate_record_complete = true`, with no vacuous/stub,
incomplete-proof, or circular-provider flags. This does not close AP2-N001:
`predicate_model = false`, the pinned model-law booleans remain false, the
current-report impossibility now lives at the full predicate-model certificate
rather than the field-source package, and the selected source-record definition
categories remain missing.
The active Lean boundary now also has a conditional selected source-record
instantiation route. `selectedRustModelSourceRecordPackage` is built from the
five selected evidence-source bindings, and
`selectedRustFiveSourceSubEvidenceDigestsMatchProof` is projected from that
package, all parameterized by a proof-bearing
`ReviewedFq12SelectedRustNativeBoundaryPredicateModelCertificate` plus the
seven raw source records it must certify. Lean accepts the block, and the
manifest now reports `missing_selected_source_record_count = 0` with every
source-record definition and certificate-constructor binding present. This is
not cryptographic closure: `all_selected_source_records_present` remains false
because `native_boundary_concrete_predicate_model_present = false`, and the
broader concrete-definition checklist still rejects the conditional route as a
non-concrete import rather than treating parameterized source records as real
Rust/model evidence.
The next native-boundary model slice now has direct Lean proofs instead of
remaining as opaque model facts: the selected field-source package proves
payload-acceptance non-vacuity, boundary-break non-universality, payload
acceptance matching the native boundary, extraction-certificate availability
and validity matching the accepted payload, role-lift validity binding
extraction, assembly validity binding the opening, compatibility validity
binding the representative, and aggregate full-opening bridge validity binding
the bridge certificate. The selected role-lift opening predicate was tightened
so opening now carries the certificate-to-opening lift relation, and the
role-lift-opening-to-certificate-lift law is proved directly from that
relation. These are definitional/projection facts over
`selectedRustNativeBoundaryPredicateFieldSources Cert`; they do not instantiate
the predicate-model certificate. The remaining hard blocker in this cluster is
the shared-witness/canonical native-validity source needed to finish the hard
extraction law bundle and then build the concrete predicate-model certificate
from real report evidence.
The selected hard-field equality is now also proved in both useful forms:
`selectedHardFieldsFromSharedWitnessCertificate` states that the selected
field-source package uses the supplied shared-witness certificate for exactly
the five hard reviewed-Fq12 extraction fields, and
`selectedHardFieldsFromSharedWitnessCertificate_toPredicates` gives the
constructor-facing equality against
`reviewedFq12HardExtractionPredicateFieldSourcesOfPredicates`. The remaining
design point is no longer the hard-field equality itself. Lean now names the
fixed-point/import route as
`ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource`: it carries
the selected field-source package, a shared-witness certificate over that
package's predicate record, predicate-level equality to the selected
shared-witness-sourced package, and the canonical native-validity source.
`toPredicateModelSource` reduces that fixed-point source to the full
predicate-model source target, deriving the hard extraction law bundle through
the shared-witness certificate. The predicate-model certificate also now has
`ofReportAndFixedPointSource`, which builds the proof-bearing certificate from
the public report/digest proofs, the field-source package certificate, and the
fixed-point source instead of requiring every model law as an independent
argument. The remaining concrete blocker is supplying that fixed-point source
and `FactsAllTrue` report proof from real Rust/model evidence.
The canonical native-validity input is now decomposed further:
`ReviewedFq12SharedWitnessCanonicalFullOpeningNativeValidityEquationSource`
splits the remaining obligation into three source facts over the exact
canonical representative: `boundaryOf` reconstructs the accepted boundary,
`localValid` holds, and `descentConsistent` holds.
`reviewedFq12SharedWitnessCanonicalFullOpeningNativeValiditySourceOfEquations`
and `ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource.ofCanonicalNativeEquations`
lift those three facts through the fixed-point route. The remaining evidence
import is therefore concrete and narrow: prove those three canonical equations
for the selected Rust predicate record and provide the report `FactsAllTrue`
certificate.
That three-equation import is now reduced to field-level Rust/model facts:
`ReviewedFq12SharedWitnessCanonicalFullOpeningNativeValidityFieldEquationSource`
separates the predicate identification facts
(`boundaryOf = selectedBoundaryOf`, `localValid = selectedLocalValid`,
`descentConsistent = selectedDescentConsistent`) from concrete equality checks
over the accepted boundary and full opening. The adapter
`reviewedFq12SharedWitnessCanonicalFullOpeningNativeValidityEquationSourceOfFieldEquations`
proves the three canonical native-validity equations from those fields by
computation. `ReviewedFq12SelectedRustNativeBoundaryPredicateFixedPointSource.ofCanonicalNativeFieldEquations`
now plugs that field-level source directly into the fixed-point constructor.
The fixed-point equality also now projects the three selected-predicate
identification facts directly:
`boundaryOf_eq_selected_of_predicatesFromSharedWitness`,
`localValid_eq_selected_of_predicatesFromSharedWitness`, and
`descentConsistent_eq_selected_of_predicatesFromSharedWitness`. This is still
fail-closed: the concrete import must supply the boundary-version, role-handle,
masked-projection, representative-digest, local lifecycle-slot, and descent
lifecycle/aggregation equalities from the active Rust verifier/evidence, plus
the report `FactsAllTrue` certificate. The earlier highest-risk local-account
mismatch is resolved by retargeting `selectedLocalValid` to lifecycle locality
only. The previous predicate required
`opening.selectedAccountProjectionDigest = b.context.accountId`, but Rust sets
`selected_account_projection_digest` from the extraction certificate's
account-projection lanes while `account_id` is the public account identity
digest. That equality is not enforced by the active verifier and should not be
part of the model; account binding is carried by native boundary reconstruction
and the account projection digest instead.
Continuing the same field audit exposed a broader coordinate mismatch: Rust
sets `lifecycle_slot_digest = assembly.p_lifecycle_digest`,
`opening_tuple_digest = creation_binding.opening_tuple_commitment`, and
`projection_relation_digest = creation_binding.projection_relation_commitment`.
The selected Lean model no longer treats local/descent validity as proof of
unbacked context equalities: `selectedLocalValid` and
`selectedDescentConsistent` now record native-field self-consistency at this
representative layer, with public slot binding carried by payload/certificate
acceptance and opened role coordinates carried by boundary reconstruction plus
the full-opening evidence. Boundary reconstruction still asks for the opened
lifecycle and opening tuple to match the selected boundary fields
(`liveStatusMarker` and `creationStatementDigest`), so the remaining coordinate
audit is narrowed to whether those selected boundary fields are the right Lean
names for the Rust native-boundary frame objects.
The same principle now applies to account creation/admission:
`selectedAccountCreateAccepts` no longer asserts
`nativeBoundaryDigest = creationStatementDigest`, and `selectedAdmitted` no
longer asserts `liveStatusMarker = slotId`. Those equalities are not Rust facts;
context identity remains in the admission predicate, while native digest and
slot binding stay in the payload/certificate acceptance route.
The digest-identity instance has now been repaired in the Lean model:
`HiddenRepresentative` and `FullNativeRepresentativeOpening` carry
`nativeBoundaryDigest` separately from `representativeDigest`,
`selectedBoundaryOf` sets `Boundary.nativeBoundaryDigest` from the native digest,
and the role-lift, assembly, compatibility, bridge, and canonical
full-opening predicates bind the native digest across the opening/certificate
chain. The former `representativeDigest_eq` field equation has been replaced by
`nativeBoundaryDigest_eq`; the representative digest remains the Rust frame hash
`path2_reviewed_fq12_native_representative_digest_v1(native_boundary_digest,
...)` and is no longer modeled as the native boundary digest itself. This closes
the explicit representative/native digest conflation; the coordinate retargeting
above remains the next active proof-completeness blocker.
Tooling note: the Rust source scanner's direct-constant-lambda check now clamps
source windows to a UTF-8 character boundary before slicing; the prior byte
slice could panic when a window ended inside Lean's `∧` token. The broad
fail-closed regression was updated for the new provider-complete state but is
currently too slow after the larger package body; the verified gates for this
iteration are Lean compilation, manifest output, and `git diff --check`.
P2FS-001 / AP2-N001 is now closed for the active hidden-source semantics layer:
Lean builds the selected predicate-model certificate, selected source-record
package, and final theorem
`reviewed_fq12_hidden_source_proof_semantics_closed_from_selected_rust_model_five_source_instantiation`.
The manifest reports `predicate_model = true`,
`extractor_theorem_instantiated = true`, and
`semantics_bridge_complete = true`; the prevalidated verifier now rejects at
checked-core composition rather than hidden-source semantics.
AP2-N003 concrete numeric-source adapter note: `Path2FullSpendConcrete.lean`
now exposes
`SelectedRustConcreteAcceptedSpendNumericSourceCertificate.toSelectedRustAcceptedSpendSoundnessCertificate`
and `.toSelectedRustAcceptedSpendSoundnessBoundary`. These adapters build the
accepted-spend certificate/boundary only from the report-bound concrete numeric
source while still requiring the real production probability pins,
priced-bound security inequality, `FactsAllTrue`, and closure propositions.
The Rust accepted-spend numeric-bound source gate now exact-checks these
adapter names and rejects suffixed/spoofed variants. The selected
accepted-spend report digest was refreshed for the expanded source window, but
the report remains verified-only and intentionally not `FactsAllTrue`.
Follow-up AP2-N003 accepted-spend proof-carrying note: the selected
accepted-spend certificate now carries proofs for the four closure propositions
it names: hidden-source semantics, checked-core composition, lifecycle/replay,
and no-export. The concrete numeric-source adapters require those proofs before
they can build the selected accepted-spend certificate or erased boundary. The
Rust source gate now requires the proof names and rejects local/global
`True.intro` aliases for them, so the numeric-bound instantiation cannot close
by naming closure propositions without supplying evidence.
AP2-N003 full-reduction closure dependency note: the selected full-signature
reduction constructor no longer accepts a second set of hidden-source,
checked-core, lifecycle/replay, or no-export closure proof arguments. It derives
those proof fields from the consumed `SelectedRustAcceptedSpendSoundnessCertificate`
itself. The full-reduction source gate now expects the future theorem to project
`selectedRustAcceptedSpendCertificate.hiddenSourceProofSemanticsClosedProof`,
`.checkedCoreCompositionClosedProof`, `.lifecycleReplayClosedProof`, and
`.noExportClosedProof`, keeping closure ownership at the accepted-spend layer.

## Findings

| ID | Severity | Status | Finding | Primary references |
| --- | --- | --- | --- | --- |
| APS-001 | Critical | Closed for active path | The compact final certificate requires `noKeygenBreak : forall breaker, not K.BreaksKeygen breaker`, but `N32ProductBlockSecretRecoveryGame` also includes `buildBreaker_sound`, so a breaker exists by applying `buildBreaker` to the real secret. The code now exposes this as `exists_keygenBreaker` / `no_keygenBreaker_false`, and the compact endpoint is documented as an uninhabitable contradiction certificate rather than a security theorem. | `Descender/SignatureTranscript.lean:4233`, `Descender/SignatureTranscript.lean:4264`, `Descender/SignatureTranscript.lean:5267` |
| APS-002 | Critical | Closed for active path | The compact six-replay theorem gives `(forgerSuccess - 2^-133)^6 <= failureUnionBound`. With the certified coverage bound `16 * (2 * (17/32)^6)^7`, the direct residual advantage is about `2^-4.55`, not cryptographic security. The compact path is therefore dead for deployable security unless redesigned with a new replay/coverage budget. | `Descender/SignatureTranscript.lean:3484`, `Descender/SignatureTranscript.lean:3640`, `Descender/SignatureTranscript.lean:5296` |
| APS-003 | High | Closed for active path | The compact active extractor assumes a shared mask across replayed accepted challenges. Lean explicitly records that different masks introduce a mask-delta term. Because the compact six-replay endpoint is now non-deployable, this remains a redesign obligation only if the compact path is revived. | `Descender/SignatureTranscript.lean:3118`, `Descender/SignatureTranscript.lean:3216`, `Descender/BlockChallengeSoundness.lean:117` |
| APS-004 | High | Explicit boundary added / not discharged | Fiat-Shamir/hash agreement is assumed through fields such as `recomputedChallenge_matches_actual`, `oracleChallenge_matches_actual`, and hash-frame agreement. This is now named by `Q12289HashAssumptionModel`, `Q12289HashFrameAssumptionBoundary`, and `Q12289FSCoreHashAssumptionBoundary`; it is still not a proved SHA-256/RO/QROM reduction. | `Descender/SignatureTranscript.lean:2115`, `Descender/SignatureTranscript.lean:2306`, `Descender/SignaturePublicViewWitness.lean:3646`, `Descender/SignaturePublicViewWitness.lean:3673` |
| APS-005 | Medium | Redirected for active deployment | The q12289 checked path proves accepted-transcript soundness, not full signature security. `N32Q12289FullSignatureSecurityBoundary` now records the missing EUF-CMA/signing/keygen/completeness/implementation/hash-budget obligations, requires a numeric `signatureForgerySuccess_le_acceptedTranscript` reduction, and proves a full-signature `<= 2^-133` bound only from that boundary. But current Rust preflight selects the Path 2 native / reviewed Fq12 verifier, so q12289 full-game closure would certify the wrong verifier unless deployment is changed. The active replacement boundary is `NativeFullSignatureSecurityBoundary`; see `audit/path2_native_full_signature_obligation_burndown.md`. | `Descender/SignaturePublicViewWitness.lean:3736`, `Descender/SignaturePublicViewWitness.lean:5228`, `Descender/SignaturePublicViewWitness.lean:5310`, `Descender/SignaturePublicViewWitness.lean:5398`, `Descender/Path2NativeAccountDesign.lean:1334`, `tools/parameter-search/src/production_reference.rs:22` |

## Progress Checklist

| Task ID | Status | Goal | Blocks / Notes |
| --- | --- | --- | --- |
| APS-T001 | Done | Decide which theorem path is intended to be the active production path: compact six-replay, q12289 checked-systematic, stateful/local-delta, or a replacement. | Active proof work should proceed on Path 2 native / reviewed Fq12. q12289 is reference/fallback evidence unless deployment changes; compact six-replay is negative-control only. |
| APS-T002 | Done | Fix or retire the compact final certificate. | APS-001 closed for active path by exposing contradiction in Lean and demoting the endpoint. |
| APS-T003 | Done | Recompute the compact coverage/replay budget if compact extraction remains in scope. | APS-002 closed for active path by retiring compact from deployable security; revive only with a redesigned replay/coverage theorem. |
| APS-T004 | Done for active path | Internalize same-mask/commitment binding or charge mask-delta failures to an explicit structural failure event. | APS-003 is compact-path-only after retirement. Reopen if compact extraction is revived. |
| APS-T005 | In progress | Replace hash/random-oracle agreement fields with a precise assumption/reduction boundary. | Boundary records added in Lean with model, query budget, multi-target budget, and domain separation fields. Still needs the actual selected model and budget proof/reduction. |
| APS-T006 | Redirected | Lift q12289 checked-systematic transcript soundness to a full signature security game, if that path is selected. | q12289 is not the selected deployment verifier. `N32Q12289FullSignatureSecurityBoundary` remains as a reference shape, but active proof work should create the analogous boundary for the Path 2 native / reviewed Fq12 verifier relation. |
| APS-T009 | In progress | Close the active Path 2 native / reviewed Fq12 full-signature boundary. | `NativeFullSignatureSecurityBoundary` now names the active obligations and proves `fullSignatureSuccessProb_le_activePathForgeryBound` from the boundary. P2FS-001 now has the Option 2 role-root route wired in: Rust `Path2CreationProofBindingV1` carries `gauge_commitment`, `randomizer_commitment`, and `native_role_root_commitment`, `Path2ReviewedFq12HiddenSourcePayloadBindingV1` pins the creation-proof binding digest plus reviewed-statement randomizer, and `Path2ReviewedFq12FullNativeOpeningCertificateV1` binds role lift to that root. Red-team test `path2_reviewed_fq12_full_native_opening_certificate_rejects_role_root_rebinding` now rejects the previous alternate-role tuple for the same accepted payload/extraction certificate, and the full-opening test now rejects a randomizer-rebound creation binding. Option 1 is viable for P2FS-001 operational-private fields: Rust confirms changing `authorization_local_digest`, private-fiber `randomizer_digest`, and `wallet_recovery_digest` leaves the full-opening certificate unchanged/valid, and Lean records them as `OperationalPrivateFiberDigests` ignored by hidden-source opening semantics. Rust now has the non-aggregate field certificate `Path2ReviewedFq12FullOpeningFieldEvidenceV1` plus the downstream aggregate `Path2ReviewedFq12FullOpeningBridgeCertificateV1`; the former verifies role-root recomputation, full-opening fields, compatibility fields, representative digest, direct payload role-field agreement, and rejects aggregate-bridge sourcing, while the latter remains a final audit handle. Lean mirrors this with `Path2FullOpeningFieldEvidence.lean`, adds the sidecar semantic bridge in `Path2FullOpeningFieldEvidencePredicates.lean`, adds byte parser/frame hooks in `Path2FullOpeningFieldEvidenceByteBridge.lean`, packages local composition in `Path2FullOpeningFieldEvidenceComposition.lean`, and adds `Path2FullOpeningSourceBridge.lean` so assembly/compatibility bindings can return named breaks while still deriving extraction-certificate soundness. Current proof ordering is explicit: the aggregate bridge is a final audit object, while role-root binding, assembly construction, assembled-representative native validity, and compatibility-certificate soundness remain the non-aggregate obligations needed to avoid circularity. See `audit/path2_native_full_signature_obligation_burndown.md`. |
| APS-T007 | Done for compact recovery game | Add non-vacuity sanity lemmas/tests for certificate structures. | `N32ProductBlockSecretRecoveryGame.exists_keygenBreaker` and `no_keygenBreaker_false` prevent the compact no-breaker package from passing as a live security theorem. More sanity lemmas may be useful for future certificate families. |
| APS-T008 | Not started | Re-run Lean and update this tracker after each proof change. | Baseline files checked during audit: `SignatureTranscript.lean`, `SignaturePublicViewWitness.lean`, `BlockChallengeSoundness.lean`. |

## Working Rule

For future updates, do not mark a finding closed just because an assumption field was renamed or repackaged. Close it only when the relevant theorem either proves the obligation, explicitly prices it in a security bound, or is documented as a final external assumption with a concrete game and budget.
