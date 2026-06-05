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
| AP2-001 | Field-local shared-frame selected-Rust source-route instantiation open | P2FS-001 is now split into non-aggregate role-root, assembly, compatibility, field-evidence, source-route, and local-source layers without using the aggregate full-opening bridge as a premise. Rust exposes pre-aggregate field-evidence and source-route frames with negative controls for aggregate-source misuse and stale/tampered digests. Lean exposes the direct native digest-frame endpoint `ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs` and theorem `reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_native_digest_frame_packaged_source_route_inputs`; that endpoint consumes `ReviewedFq12RoleRootFunctionalLiftSource`, `SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource`, independent `ReviewedFq12ConcreteAssemblyFieldBindingOrBreak`, `ReviewedFq12AssemblyCompatibilityLocalModelSource`, and `ReviewedFq12SelectedRustNativeRepresentativeDigestFrameSource`. Lean now also exposes the implementation-oriented field/local shared-frame package `ReviewedFq12SelectedRustFieldLocalSharedDigestFrameWitnessInputs` and theorem `reviewed_fq12_certificate_soundness_from_selected_rust_field_local_shared_digest_frame_witness_inputs`: selected source-route bytes supply field/source-route evidence; concrete field/local assembly and compatibility sources supply the direct binding and local model; and `ReviewedFq12SelectedRustSharedDigestFrameWitnessByteSources` pins the canonical representative witness source and assembly digest-frame witness source to one selected 11-input representative digest frame. The direct canonical/witness-byte, digest-frame witness, native-frame, shared-frame, and field/local packages remain adapters toward this active implementation target. Rust now has typed witnesses for both sides: `Path2ReviewedFq12NativeRepresentativeDigestFrameWitnessV1` validates the assembly-certificate side of the 11-input `path2_reviewed_fq12_native_representative_digest_v1` frame, and `Path2ReviewedFq12FullNativeOpeningCanonicalRepresentativeWitnessV1` validates the full-opening canonical representative side against the role-lift and assembly certificates. The Lean byte certificate now binds the canonical witness's native role-root commitment and full-opening digest to the opened tuple, matching the Rust frame. Both Rust witnesses reject self-consistent rebound assembly/opening drift, and the canonical witness now has explicit negative controls for native-role-root and full-opening-digest rebinding. The Rust fail-closed audit now source-inspects the selected hidden-relation extraction certificate, role-root/lift source, independent direct assembly binding source, local assembly/compatibility model source, field-evidence source, source-route source, source-route negative controls, the Rust-shaped canonical witness source, canonical-witness rebinding negative controls, and the shared representative digest frame instead of treating those booleans as unconditional. The remaining P2FS-001 proof work is to instantiate the five source packages consumed by the field-local shared-frame Lean theorem, while keeping source-route evidence compatibility-indexed rather than using it as generic pre-compatibility assembly evidence. | `P2FS-001` |
| AP2-002 | Byte bridge plus Rust parser evidence / downstream breaks open | Checked-core composition now has a Rust-named Lean relation `Path2NativeCheckedCoreAcceptV1`, a bridge into `CheckedCoreSpendReduction.checkedCoreAccepts`, and a theorem that accepted native evidence uses both the assembled native frame and the Lean reduction frame. `Descender/Path2CheckedCoreByteBridge.lean` adds parser/canonical-proof/frame/hash/syndrome certificates feeding the native accept relation. Rust now pins canonical proof length and rejects short, noncanonical, and decoded-commitment-mismatch proof bytes before acceptance. Remaining gaps are the formal Lean/Rust parser/hash binding and checked-core equation soundness as final named breaks. | `P2FS-002` |
| AP2-003 | Concrete carrier added / production game lift open | Lifecycle/replay controls have Rust local evidence for reserve/sign/consume, verify-and-consume replay rejection, spent-ledger stale-wallet blocking, reorg burns, prevalidated-slot consume-once, and selected-projection replay rejection. `Descender/Path2FullSpendGame.lean` states the direct-mode accepted-spend and full-forgery wrapper theorem; `Descender/Path2FullSpendConcrete.lean` adds a concrete carrier for prevalidated leaves, creation evidence, admission records, direct certificates, signature bodies, consume state, checked-core binding, rollback/reorg breaks, and concrete accepted-spend soundness. It now also adds `ConcreteFullSignatureExecutionSource`, the non-tautological production route from full-signature execution success to concrete verifier acceptance or a named break. The remaining gap is instantiating that source from the real EUF-CMA/blockchain implementation and numeric bound. | `P2FS-003`, `P2FS-013` |
| AP2-004 | Hash boundary props filled / widened deployment horizon in progress | Hash/QROM accounting has Rust budget/review KATs for direct-mode query caps, inactive batch terms, public native transcript assumptions, and domain/schema uniqueness. Deployment route decision is auxiliary Fq12 action bits: append canonical auxiliary algebraic challenge bits after the compact block/sign prefix, have the checked-core Fq12 action consume the whole longer challenge byte string, target 182 algebraic bits plus 32 hardening bits for `log2(T)=32`, and keep the 26-repetition compact local semantics unchanged for first closure. `Descender/Path2HashBudget.lean` and `Descender/Path2HashDomainAndMultiTarget.lean` are being updated from the old one-bit cap to the widened 182-bit QROM-discounted transcript term, 53-bit strict deployment margin, 32-bit active aggregate horizon, and 54-bit negative control. Rust native deployment preflight is being updated so QROM horizon completion is no longer the smallest productization gate once the widened constants are installed. | `P2FS-005` through `P2FS-008` |
| AP2-005 | Honest-flow and gate-separation evidence added / productization blockers pinned | Keygen, signing-oracle, and completeness are still deliberately not closed: Rust scope objects report signer/keygen productization and honest-constructor completion as false. Rust evidence shows the prevalidated honest constructor flow reaches selected-verifier public-boundary acceptance, while production acceptance remains fail-closed on the signer/verifier pipeline, CT, and storage gates; an additional test keeps those gates separate and prevents CT/storage non-claims from being hidden behind public-boundary acceptance. | `P2FS-009` through `P2FS-011` |
| AP2-006 | Public-boundary evidence present / byte-for-byte bridge open | Rust preflight and pipeline tests identify the selected Path 2 native/Fq12 verifier surface and keep it fail-closed. Public-boundary evidence, projection encoders, checked-core frame, serializers, consensus carrier, registry, hash review, signer/verifier evidence, and productization-gate separation tests are present, but the pipeline still reports no production acceptance and internal blockers for no-export serializer instantiation, signer/verifier pipeline, CT, and storage. The Lean relation is not yet a byte-for-byte theorem about that selected Rust verifier. | `P2FS-012` |
| AP2-007 | Concrete serializer allowlist bridge added / Rust binding open | Public native views, facade exports, replay rejection paths, state snapshots, and spent-ledger records have Rust no-selected-material checks. `Descender/Path2NoExport.lean` proves that accept/reject/storage serializer conformance supplies `noExportClosed`; `Descender/Path2NoExportConcrete.lean` adds tagged accept/reject/storage output frames and reduces a concrete serializer allowlist certificate to active no-export closure. Remaining work is binding that Lean serializer certificate to every concrete Rust accept/reject/storage output surface. | `P2FS-004` |

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

AP2-N001 certificate-constructor note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now exposes `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.ofEvidenceAndSources`, the named constructor for the final selected Rust/model five-source certificate. The `extractor_theorem_instantiated` source gate no longer accepts a raw record literal for `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate`; it requires the final theorem to call this constructor with the verified five-source evidence report, the all-true proof, and each selected source package before deriving `toClosureInputs`. The same gate now strips comments/strings and rejects `sorry` or `admit` tokens in the active boundary module, so placeholder Lean proofs cannot flip the hidden-source semantics gate.

AP2-N001 five-source certificate note: `Descender/Path2HiddenSourceSemanticsBoundary.lean` now exposes `ReviewedFq12SelectedRustFiveSourceClosureEvidence`, `ReportVerified`, `FactsAllTrue`, and `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate`. The certificate carries a structured public evidence report plus proofs tied to that report, rather than arbitrary `Prop` fields, alongside the seven concrete model/source fields needed by `ReviewedFq12SelectedRustModelFiveSourceClosureInputs`; `ReportVerified` requires schema, explicit bridge-audit digest bytes, bridge-audit digest binding, report-digest verification, and bridge-validation bits, while `FactsAllTrue` covers the non-closing readiness facts. The Rust extractor gate now requires the future instantiation theorem to locally construct this certificate and call `ReviewedFq12SelectedRustModelFiveSourceClosureCertificate.toClosureInputs`; it rejects an assumed certificate, assumed report/proof bits, direct local input-record construction, inline all-true report construction, vacuous theorem-local verification, and normalized module-level stubs that make the report/facts evidence trivial. This does not flip `extractor_theorem_instantiated`, but it makes the remaining AP2-N001 closing theorem proof-bearing and report-shaped.

AP2-N001 extractor-stub note: the Rust source gate now strips Lean comments and string literal contents before scanning multiline `axiom`, `constant`, and `opaque` declaration blocks for the selected Rust/model source names and evidence predicates, not just one-line declarations. It also treats vacuous `schemaMatches := true` report construction as a forbidden report stub marker. This closes a gate-bypass shape where the final extractor theorem could appear to consume concrete source names while those names were still introduced as multiline assumptions, and prevents comment/string-only stub text from poisoning the module scan.

AP2-N001 extractor-comment note: the future extractor theorem source gate now strips Lean line comments, block comments, and string literal contents before checking for the required certificate-construction markers. Regressions pin that comment-only or string-only occurrences of `selectedRustModelFiveSourceCertificate`, the evidence-report fields, source bindings, `toClosureInputs`, and the five-source closure theorem do not satisfy `extractor_theorem_instantiated`.

AP2-N001 extractor-aggregate-package note: the future extractor theorem source gate now rejects theorem headers that take newly invented aggregate five-source implementation/package/certificate parameters (for example `SelectedRustModelFiveSourceImplementation`) in addition to the already rejected closure input/certificate records and individual source parameters. A negative-control theorem shows such a package could otherwise feed the certificate fields while hiding the selected Rust/model source construction behind a single assumed object.

AP2-N001 five-source evidence note: Rust now exposes `Path2ReviewedFq12FiveSourceClosureEvidenceV1`, a public digest-bound evidence report over the selected model five-source readiness bits: payload projection, extraction-certificate availability, role-root/lift source, selected source-route byte source, assembly field/local/native source, compatibility field/extraction source, shared-frame witness source, active field/local byte-component surfaces, and the Lean five-source/full-signature handoff routes. The Rust report now carries the Lean-mirrored `ReportVerified` status bits: `schema_matches`, `bridge_audit_digest_bound`, `bridge_audit_validated`, and `report_digest_verified`; the supplied-report verifier requires the constructed report plus those status bits before returning the evidence digest. All readiness facts are exposed through a separate `all_facts_true` predicate; the `ReportVerified` fields, `extractor_theorem_instantiated`, and `semantics_bridge_complete` remain status fields outside that readiness predicate. This makes the report usable as future `ReportVerified` / `FactsAllTrue` input without creating a circular fail-closed dependency. The future extractor theorem source gate and the curated production facade now require this evidence surface.

AP2-N001 evidence-frame note: `path2_reviewed_fq12_five_source_closure_evidence_v1_digest` now hashes an explicit length-delimited evidence frame instead of `serde_json` output. The frame binds schema, all Lean-mirrored `ReportVerified` bits, the bridge audit digest, all readiness facts, closing status bits, and cleanup policy in fixed order, is exported through the production facade, has source-gate checks for each frame label, and has a regression checking the digest equals `Sha256(frame)`.

AP2-N001 report/facts split note: Rust now exposes `path2_reviewed_fq12_five_source_closure_evidence_report_verified_v1` separately from `path2_reviewed_fq12_five_source_closure_evidence_all_facts_true`. Native preflight uses the report verifier for `five_source_closure_evidence_verified` and carries `five_source_closure_evidence_facts_all_true` as its own bit, matching the Lean `ReportVerified` / `FactsAllTrue` split. The stronger `verify_path2_reviewed_fq12_five_source_closure_evidence_v1` remains the conjunction for closure evidence.

AP2-N001 native-evidence propagation note: `Path2NativeDeploymentPreflightV1` now carries the five-source closure evidence digest plus verified/all-facts-true booleans, and `Path2NativeVerifierPipelineVerdictV1` / `Path2NativeSignerVerifierPipelineEvidenceV1` mirror those fields from preflight. This makes the proof-bearing AP2-N001 evidence visible in the active native verifier/signer trail while preserving the fail-closed `hidden_source_semantics_bridge_complete = false` production rejection until the extractor theorem is actually instantiated.

AP2-N001 component-window note: the Rust hidden-source semantics audit now hardens `rust_assembly_field_local_native_components_present` and `rust_compatibility_field_extraction_components_present` against broad-marker satisfaction. The assembly gate separately slices the native assembly constructor/verifier, the exact Rust assembly/full-opening field-binding function, the Lean `ReviewedFq12AssemblyAvailabilityLocalSource` structure, and its concrete-field-source forgetter. The compatibility gate separately slices the compatibility constructor/verifier, same-representative binding function, the Lean `ReviewedFq12CompatibilityAvailabilityExtractionRulesSource` structure, and its concrete-field-source forgetter. This keeps the AP2-N001 raw active theorem tied to the actual local-model and extraction-rule sources while the bridge remains fail-closed on `extractor_theorem_instantiated = false`.

AP2-R001 QROM-horizon note: the Rust preflight no longer treats `minimum_deployment_margin_bits > 1` as sufficient QROM/multi-target completion. `path2_auxiliary_fq12_qrom_horizon_complete_v1` now requires the selected auxiliary-Fq12 challenge shape exactly: 23 algebraic bytes, 4 hardening bytes, last algebraic-byte mask `0x3f`, 182 algebraic bits after masking, 214 total effective challenge bits, selected 32-bit aggregate horizon within the 53-bit deployment margin, and a 54-bit negative control outside that margin. `Descender/Path2HashBudget.lean` now mirrors the challenge-width facts directly in `Path2DirectQueryBudgetV1`.

AP2-R001 action-consumption note: `Descender/Path2HashBudget.lean` now exposes `AuxiliaryFq12ChallengeShapeComplete`, and `Descender/Path2HashDomainAndMultiTarget.lean` threads that fact through `ActiveHashDomainAndMultiTargetCertificate`. Rust regression `path2_checked_core_authorization_consumes_auxiliary_challenge_bytes_in_fq12_action` verifies the active checked-core authorization path consumes bytes outside the compact 130-bit block/sign prefix: flipping byte 17 or the first hardening byte leaves `decode_challenge` unchanged, changes `reviewed_fq12_challenge_action_vector`, changes the nonzero-witness expected Fq12 response syndrome, and a hardening-byte signature rebound is rejected by checked-core challenge binding. This addresses the route-specific risk that the widened bytes could have been transcript-only rather than part of the Fq12 action.

AP2-R001 action-collision note: the Rust direct budget/review object now serializes the Fq12 action-collision floor (`163` bits) plus the selected deployment residuals (`131` quantum bits and `130` classical bits after the union-safe query bounds), and `path2_auxiliary_fq12_qrom_horizon_complete_v1` requires those residuals to meet the 128-bit target. Lean mirrors this with `Fq12ActionCollisionBudgetPriced` in `Descender/Path2HashBudget.lean` and threads it through `ActiveHashDomainAndMultiTargetCertificate` in `Descender/Path2HashDomainAndMultiTarget.lean`.

AP2-R001 active-boundary source-gate note: `path2_auxiliary_fq12_qrom_horizon_complete_v1` now also requires `path2_active_native_hash_budget_boundary_source_present_v1`, which source-inspects the Lean direct-budget constants, auxiliary-Fq12 challenge shape, Fq12 action-collision budget, selected 32-bit aggregate horizon, 54-bit negative-control theorem, and `activeNativeHashBudgetBoundary` certificate window. This ties the Rust QROM/deployment-horizon preflight to the actual Lean native hash boundary instead of only checking Rust-side constants.

AP2-R001/AP2-R002 byte-domain note: `Descender/Path2HashDomainAndMultiTarget.lean` now instantiates `ActiveHashByteDomainInstantiation` as `activeHashByteDomainInstantiationV1`, giving theorem-side representatives for the active checked-core, hidden-source, projection-carrier, and account-lifecycle byte domains and proving `active_checkedCoreFrameByteDomainSeparated` / `active_hiddenSourceFrameByteDomainSeparated`. `ActiveHashDomainAndMultiTargetCertificate` carries this byte-domain instantiation and byte-separation facts, and the Rust QROM horizon source gate now requires those markers. This makes byte-domain separation part of the active Lean certificate instead of relying only on Rust registry uniqueness tests.

AP2-R001 QROM-review source-gate note: `path2_native_deployment_preflight_v1` now records `hash_qrom_review_present` from `path2_native_hash_qrom_review_source_present_v1` instead of checking only the review schema. The helper source-inspects the active hash/QROM review struct, review constructor, active native query-symbol list, ordinary-hash assumptions, hidden obligations not discharged by hashing, consensus-carrier byte accounting, and the public-only review regression. If this source boundary opens, deployment now gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_HASH_QROM_REVIEW` and verifier rejection `reject_active_native_hash_qrom_review_source_open`; the existing `PATH2_NATIVE_DEPLOYMENT_BLOCKER_QROM_TARGET_HORIZON` remains the stronger priced-horizon gate.

AP2-R001/AP2-R002 public-boundary preflight note: `proof_side_public_boundary_preflight_complete` is no longer a bare `true`. It is now the conjunction of the active source-gated public/proof boundary bits: projection encoders, checked-core frame, public serializers, consensus carrier, verifier preflight report, global domain registry, hash/QROM review, hidden-source semantics, checked-core composition, QROM horizon, full-signature execution source, accepted-spend numeric bound, no-export serializer boundary, and no-export serializer instantiation. It intentionally excludes productization gates for signer/verifier pipeline, constant-time audit, storage hardening, and facade export curation, which remain separately tracked.

AP2-R001/AP2-R002 deployment-preflight evidence-frame note: `path2_native_deployment_preflight_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json` output. The frame binds every public/proof boundary bit, five-source closure evidence digest/verified/all-facts bits, facade and productization gates, byte-size limits, deployment-ready status, and ordered internal blocker list; it is exported through the production facade and covered by a `Sha256(frame)` regression.

AP2-R002 checked-core source-gate note: `path2_native_deployment_preflight_v1` no longer treats checked-core composition review as an unconditional `true`. It now separates `path2_native_checked_core_byte_bridge_source_present_v1`, which source-inspects `Descender/Path2CheckedCoreByteBridge.lean` for the selected Rust byte-source decomposition and source-inspects `path2_native_checked_core_accept_v1` for frame validation, canonical proof length/decode, proof-commitment equality, challenge derivation from the checked-core frame, actual/expected Fq12 syndrome computation, and sigma-equation equality, from the stronger `path2_native_checked_core_selected_rust_instantiation_present_v1` gate. The stronger gate requires a concrete selected-Rust instantiation theorem wiring the named parser/proof/frame/hash facts into `SelectedRustCheckedCoreByteSource`, and rejects `axiom`/`constant`/`opaque`/module-level `variable` stubs for those facts.

AP2-R002 checked-core instantiation-gate hardening note: `path2_native_checked_core_selected_rust_instantiation_present_v1` now strips Lean comments and string literal contents before scanning for the future `selectedRustCheckedCoreByteSource_instantiation_v1` theorem and normalized marker set. Its forbidden-stub detector also catches multiline `axiom`/`constant`/`opaque` declarations, module-level `variable` stubs, and vacuous `True`/`trivial` report-proof definitions while ignoring those markers inside comments or strings. The deployment preflight regression now includes comment/string spoofing and multiline-stub negative controls, so AP2-N002 cannot be closed by marker text that is not executable Lean source.

AP2-R002 fail-closed instantiation note: `checked_core_composition_review_complete` now requires both the checked-core byte-bridge source surface and the selected-Rust checked-core instantiation gate. Since AP2-N002 is still open, active native deployment gets `PATH2_NATIVE_DEPLOYMENT_BLOCKER_CHECKED_CORE_COMPOSITION`, but AP2-N001 hidden-source semantics is now checked first in the native preflight and remains the smallest active proof-side blocker until its bridge is instantiated.

AP2-N002 component-target note: `Descender/Path2CheckedCoreByteBridge.lean` now exposes `SelectedRustCheckedCoreByteInstantiationInputs` and `selectedRustCheckedCoreByteSource_from_instantiation_inputs_valid_or_named_break`. This is the implementation-oriented package AP2-N002 must fill from Rust/model byte facts: selected acceptor acceptance, parser/native-handle agreement, proof length/canonicality, decoded proof commitment, parsed response syndrome, checked-core frame bytes, verifier handle equality, parser equality, and challenge/expected-vs-actual syndrome equality. The Rust source-window audit requires this target, but deployment remains blocked because the concrete selected-Rust instantiation theorem is still absent.

AP2-N002 Rust-evidence note: `path2_native_checked_core_byte_instantiation_evidence_v1` now emits a public digest-only report for the selected Rust checked-core byte facts consumed by `SelectedRustCheckedCoreByteInstantiationInputs`. It records accepted-path parser/native-handle, proof length/canonicality, decoded commitment, response syndrome, frame byte roundtrip, verifier-handle/public-key binding, parser equality, challenge/expected-vs-actual syndrome equality, and negative controls for short, noncanonical, commitment-mismatch, and verifier-handle-mismatch inputs. The checked-core source-window audit pins this report and its regression, but AP2-N002 still requires translating the report/facts into the concrete Lean instantiation theorem.

AP2-N002 Lean evidence-report note: `Descender/Path2CheckedCoreByteBridge.lean` now mirrors the Rust checked-core byte-instantiation evidence as `SelectedRustCheckedCoreByteEvidenceReport`, with concrete boolean fields for schema/digest verification, every selected byte-source component fact, and every negative control. `SelectedRustCheckedCoreByteEvidenceCertificate` now carries that report plus proofs of `ReportVerified` and `FactsAllTrue`, instead of carrying arbitrary `evidenceReportVerified : Prop` and `evidenceFactsAllTrue : Prop` fields. The AP2-N002 instantiation gate now requires `evidenceReport := selectedRustCheckedCoreEvidenceReport`, so the final theorem must route through a report-shaped public evidence object before it can build `SelectedRustCheckedCoreByteInstantiationInputs`.

AP2-N002 facade/evidence hardening note: the checked-core byte evidence report and digest helper are now exported through the curated `tools/production-reference` facade and pinned by `path2_native_production_facade_exports_source_present_v1`. The report's negative-control probes now use non-panicking rejection checks, so an unexpected accept is reported as a false evidence bit rather than crashing the public evidence constructor.

AP2-N002 supplied-evidence verifier note: `verify_path2_native_checked_core_byte_instantiation_evidence_v1` now recomputes the checked-core byte evidence report from the supplied projection bundle and proof bytes, rejects stale/tampered report fields, and returns the verified report digest. The verifier is exported through the curated facade and pinned by the checked-core source-window audit, giving downstream AP2-N002 consumers a concrete consistency check instead of trusting a standalone report digest.

AP2-N002 evidence-frame note: `path2_native_checked_core_byte_instantiation_evidence_v1_digest` now hashes an explicit length-delimited frame instead of `serde_json` output. The frame binds the parser/native-handle, proof length/canonicality, decoded commitment, response syndrome, frame bytes, verifier handle, parser equality, hash/syndrome, negative-control facts, checked-core frame/proof/verifier/challenge digests, proof byte length, and cleanup policy in fixed order; it is exported through the production facade, pinned by the checked-core source gate, and covered by a `Sha256(frame)` regression.

AP2-N002 all-facts verifier note: `path2_native_checked_core_byte_instantiation_evidence_all_facts_true` now makes the checked-core evidence verifier reject a faithfully recomputed report unless every selected-Rust byte-source component fact and every negative control is true. The regression covers a public-key/verifier-handle rebinding case where checked-core proof generation can still produce a report, but `verifier_handle_eq` is false; verification now rejects it with an unsatisfied-component-fact error instead of returning a digest.

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

AP2-N003 fail-closed preflight note: `path2_native_deployment_preflight_v1` now separates `full_signature_execution_source_boundary_present` from `full_signature_execution_source_instantiated`. The boundary theorem remains present, but production readiness and `proof_side_public_boundary_preflight_complete` now require the instantiation bit, and the current preflight records `PATH2_NATIVE_DEPLOYMENT_BLOCKER_FULL_SIGNATURE_EXECUTION_SOURCE`. The future instantiation gate strips Lean comments and string literals before scanning, looks for `concreteFullSignatureExecutionSource_instantiation_v1` locally constructing `selectedRustFullExecutionSource : ConcreteFullSignatureExecutionSource`, and rejects a theorem that takes `ConcreteFullSignatureExecutionSource` or `ConcreteGameConfig` in binder headers. The header check is binder-aware, so it rejects assumed inputs without rejecting the selected concrete config named in the theorem result.

AP2-N003 execution-certificate bridge note: `Descender/Path2FullSpendConcrete.lean` now exposes `SelectedRustFullExecutionSourceCertificate`, carrying report-shaped production execution evidence, proofs that the report is verified and all execution facts are true, the selected Rust production execution-success predicate, and the proof that success implies concrete verifier acceptance or a named break. `SelectedRustFullExecutionSourceCertificate.toExecutionSource` is now the required route into `ConcreteFullSignatureExecutionSource`; the Rust instantiation gate rejects a source certificate supplied as a theorem parameter, direct local construction of `ConcreteFullSignatureExecutionSource`, theorem/header/stub assumptions for the selected execution-success or verification facts/proofs, multiline `axiom`/`constant`/`opaque`/`variable` stubs, comment/string marker spoofing, and vacuous certificate construction that sets the evidence report or proofs to `True`. This closes the proof-shape hole where the previous certificate named a verification proposition without requiring Lean evidence for it.

AP2-N003 execution evidence-report note: `SelectedRustFullExecutionSourceCertificate` now carries a concrete `SelectedRustFullExecutionEvidenceReport` plus proofs of `ReportVerified` and `FactsAllTrue`, rather than an arbitrary `selectedRustFullExecutionSourceVerified : Prop`. The report records schema/digest verification, production execution surface verification, execution-success predicate pinning, success-to-verifier-accept-or-break coverage, concrete game config pinning, non-tautological execution-source use, and named-break exhaustiveness. The AP2-N003 instantiation gate now requires `evidenceReport := selectedRustFullExecutionEvidenceReport`, keeping the future production execution theorem tied to a report-shaped evidence object.

AP2-N003 full-boundary assumption gate note: `path2_full_signature_execution_source_instantiated_v1` now rejects `concreteFullSignatureExecutionSource_instantiation_v1` theorem headers that assume `NativeFullSignatureSecurityBoundary`, `NativeAcceptedSpendSoundnessBoundary`, `SelectedRustAcceptedSpendSoundnessCertificate`, or `Path2FullSpendGame.FullForgeryToAcceptedSpendSoundness`. This prevents the execution-source instantiation from satisfying the required full-boundary adapter markers by importing the active full-signature boundary, accepted-spend numeric package, or full-forgery soundness package as a parameter.

AP2-N003 accepted-spend numeric-bound preflight note: `path2_native_deployment_preflight_v1` now has `accepted_spend_numeric_bound_instantiated`, sourced by the future `selectedRustAcceptedSpendSoundnessBoundary_instantiation_v1` theorem. The gate strips Lean comments and string literals before scanning, requires a locally constructed `SelectedRustAcceptedSpendSoundnessCertificate` with proof-bearing selected-Rust bound verification, and rejects assumed boundary/certificate/bound facts, multiline declaration/variable stubs, comment/string marker spoofing, plus vacuous `True` verification. Until that theorem is present, production readiness and `proof_side_public_boundary_preflight_complete` remain false with `PATH2_NATIVE_DEPLOYMENT_BLOCKER_ACCEPTED_SPEND_NUMERIC_BOUND`, preventing arbitrary `acceptedSpendForgeryBound` / `acceptedSpendForgery_le_bound` fields from closing the final full-signature theorem.

AP2-N003 accepted-spend evidence-report note: `SelectedRustAcceptedSpendSoundnessCertificate` now carries a concrete `SelectedRustAcceptedSpendSoundnessEvidenceReport` plus proofs of `ReportVerified` and `FactsAllTrue`, rather than an arbitrary `selectedRustAcceptedSpendBoundVerified : Prop`. The report records schema/digest verification, numeric bound pricing, inequality proof readiness, and the four prerequisite accepted-spend closure facts. The numeric-bound instantiation gate now requires `evidenceReport := selectedRustAcceptedSpendEvidenceReport`, so the future theorem must bind the selected probability/bound inequality to report-shaped evidence before constructing `NativeAcceptedSpendSoundnessBoundary`.

AP2-R004 full-boundary handoff note: `Path2NativeAccountDesign.NativeFullSignatureSecurityBoundary.withConcreteFullExecutionSource` now installs `ConcreteFullExecutionAcceptedSpendLift C Source` into the active full-signature boundary's `fullForgeryImpliesAcceptedSpendForgery` field while preserving the other proof obligations. The source gate requires this adapter and `full_signature_with_concrete_full_execution_source_lift`, so the concrete execution-source theorem is no longer only a standalone full-spend-game result. The real production source is still open.

AP2-R006/AP2-N005 no-export source-gate note: `path2_native_deployment_preflight_v1` now records `no_export_serializer_boundary_present`, sourced by `path2_no_export_serializer_boundary_present_v1`. The helper source-inspects the Lean active accept/reject/storage conformance and closure theorem, the concrete tagged serializer allowlist certificate, and Rust public-output negative controls for verifier verdicts and storage/lifecycle scope. This keeps the no-export boundary tied to the active serializer allowlist path rather than a broad `public_view_serializers_present = true` claim.

AP2-N005 no-export instantiation gate note: `path2_native_deployment_preflight_v1` now separately records `no_export_serializer_instantiated`, sourced by the future `selectedRustActiveNoExportSerializer_instantiation_v1` theorem. `Descender/Path2NoExportConcrete.lean` now exposes `SelectedRustActiveSerializerAllowlistEvidenceReport` and `SelectedRustActiveSerializerAllowlistCertificate`, so selected Rust serializer no-export closure must carry schema/digest verification, active tag and policy pinning, accept/reject/storage serializer binding, denylist exclusion, and negative-control facts before exporting `PublicOutputNoExport` and the low-level `ActiveSerializerAllowlistCertificate`. The source gate strips Lean comments and string literals before scanning, requires local construction of that selected Rust report-backed certificate, derives the no-export rule and allowlist through `toPublicOutputNoExport` and `toAllowlistCertificate`, then routes through `activeSerializerAllowlist_activeNativeNoExportClosure`, `NativeFullSignatureSecurityBoundary.withActiveSerializerNoExportClosure`, and `full_signature_with_active_serializer_no_export_closed`. It rejects theorem headers that assume the full-signature boundary, accepted-spend boundary, public no-export proof, serializer, policy, allowlist certificate, or selected serializer conformance facts; rejects `axiom`/`constant`/`opaque`/`variable` stubs for selected no-export facts even when declarations are multiline; and rejects locally fabricated all-true evidence reports. Negative controls cover assumed boundary/certificate inputs, comment-spoofed markers, string-spoofed markers, multiline declaration stubs, and vacuous report construction. The real source remains fail-closed, so deployment now records `PATH2_NATIVE_DEPLOYMENT_BLOCKER_NO_EXPORT_SERIALIZERS` until the concrete Rust accept/reject/storage serializer theorem is supplied.

AP2-N005 Rust evidence note: `Path2NativeNoExportSerializerEvidenceV1` now mirrors the Lean selected-Rust report shape for active serializer no-export. The report hashes serialized accept, reject, and storage outputs, pins the active output schemas, checks accept/reject/storage serializer surface markers, screens every output for denylisted private material, and records a negative-control result. It has explicit frame/digest/all-facts/verify functions exported through `tools/production-reference`, and the regression rejects digest tampering, false component facts, and a private-marker accept output. This is evidence only; `no_export_serializer_instantiated` remains false until `selectedRustActiveNoExportSerializer_instantiation_v1` consumes the evidence in Lean.

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
| AP2-N003 | High | Instantiate `ConcreteFullSignatureExecutionSource` and bind `Path2FullSpendConcrete` to the real account creation, slot admission, spend verification, state transition, rollback model, and numeric full-forgery reduction. | `AP2-003`, `P2FS-013` |
| AP2-N004 | High | Replace/thread `activeNativeHashBudgetBoundary` with the auxiliary-Fq12 widened challenge budget: prove the 182-bit algebraic action, 214-bit transcript budget, `log2(T)=32` multi-target pricing, Fq12 action-collision floor, canonical byte mask, and full-byte frame binding. | `AP2-004` |
| AP2-N005 | Medium | Bind `Path2NoExportConcrete` tagged output frames to every concrete Rust accept/reject/storage serializer. | `AP2-007` |
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
