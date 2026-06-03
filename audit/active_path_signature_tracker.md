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
| AP2-001 | Native digest-frame selected-Rust source-route instantiation open | P2FS-001 is now split into non-aggregate role-root, assembly, compatibility, field-evidence, source-route, and local-source layers without using the aggregate full-opening bridge as a premise. Rust exposes pre-aggregate field-evidence and source-route frames with negative controls for aggregate-source misuse and stale/tampered digests. Lean now exposes the active endpoint `ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs` and theorem `reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_native_digest_frame_packaged_source_route_inputs`; this endpoint consumes `ReviewedFq12RoleRootFunctionalLiftSource`, `SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource`, independent `ReviewedFq12ConcreteAssemblyFieldBindingOrBreak`, `ReviewedFq12AssemblyCompatibilityLocalModelSource`, and `ReviewedFq12SelectedRustNativeRepresentativeDigestFrameSource`. `ReviewedFq12SelectedRustFieldLocalPackagedSourceRouteInputs` and the shared-frame package remain derived adapters, not the cryptographic closure target. The remaining P2FS-001 proof work is to instantiate the native digest-frame source against `path2_reviewed_fq12_native_representative_digest_v1`, plus the already pinned direct assembly binding and local model, while keeping source-route evidence compatibility-indexed rather than using it as generic pre-compatibility assembly evidence. | `P2FS-001` |
| AP2-002 | Byte bridge plus Rust parser evidence / downstream breaks open | Checked-core composition now has a Rust-named Lean relation `Path2NativeCheckedCoreAcceptV1`, a bridge into `CheckedCoreSpendReduction.checkedCoreAccepts`, and a theorem that accepted native evidence uses both the assembled native frame and the Lean reduction frame. `Descender/Path2CheckedCoreByteBridge.lean` adds parser/canonical-proof/frame/hash/syndrome certificates feeding the native accept relation. Rust now pins canonical proof length and rejects short, noncanonical, and decoded-commitment-mismatch proof bytes before acceptance. Remaining gaps are the formal Lean/Rust parser/hash binding and checked-core equation soundness as final named breaks. | `P2FS-002` |
| AP2-003 | Concrete carrier added / production game lift open | Lifecycle/replay controls have Rust local evidence for reserve/sign/consume, verify-and-consume replay rejection, spent-ledger stale-wallet blocking, reorg burns, prevalidated-slot consume-once, and selected-projection replay rejection. `Descender/Path2FullSpendGame.lean` states the direct-mode accepted-spend and full-forgery wrapper theorem; `Descender/Path2FullSpendConcrete.lean` adds a concrete carrier for prevalidated leaves, creation evidence, admission records, direct certificates, signature bodies, consume state, checked-core binding, rollback/reorg breaks, and concrete accepted-spend soundness. It now also adds `ConcreteFullSignatureExecutionSource`, the non-tautological production route from full-signature execution success to concrete verifier acceptance or a named break. The remaining gap is instantiating that source from the real EUF-CMA/blockchain implementation and numeric bound. | `P2FS-003`, `P2FS-013` |
| AP2-004 | Hash boundary props filled / widened deployment horizon in progress | Hash/QROM accounting has Rust budget/review KATs for direct-mode query caps, inactive batch terms, public native transcript assumptions, and domain/schema uniqueness. Deployment route decision is auxiliary Fq12 action bits: append canonical auxiliary algebraic challenge bits after the compact block/sign prefix, have the checked-core Fq12 action consume the whole longer challenge byte string, target 182 algebraic bits plus 32 hardening bits for `log2(T)=32`, and keep the 26-repetition compact local semantics unchanged for first closure. `Descender/Path2HashBudget.lean` and `Descender/Path2HashDomainAndMultiTarget.lean` are being updated from the old one-bit cap to the widened 182-bit QROM-discounted transcript term, 53-bit strict deployment margin, 32-bit active aggregate horizon, and 54-bit negative control. Rust native deployment preflight is being updated so QROM horizon completion is no longer the smallest productization gate once the widened constants are installed. | `P2FS-005` through `P2FS-008` |
| AP2-005 | Honest-flow and gate-separation evidence added / productization blockers pinned | Keygen, signing-oracle, and completeness are still deliberately not closed: Rust scope objects report signer/keygen productization and honest-constructor completion as false. Rust evidence shows the prevalidated honest constructor flow reaches selected-verifier public-boundary acceptance, while production acceptance remains fail-closed on the signer/verifier pipeline, CT, and storage gates; an additional test keeps those gates separate and prevents CT/storage non-claims from being hidden behind public-boundary acceptance. | `P2FS-009` through `P2FS-011` |
| AP2-006 | Public-boundary evidence present / byte-for-byte bridge open | Rust preflight and pipeline tests identify the selected Path 2 native/Fq12 verifier surface and keep it fail-closed. Public-boundary evidence, projection encoders, checked-core frame, serializers, consensus carrier, registry, hash review, signer/verifier evidence, and productization-gate separation tests are present, but the pipeline still reports no production acceptance and internal blockers for signer/verifier pipeline, CT, and storage. The Lean relation is not yet a byte-for-byte theorem about that selected Rust verifier. | `P2FS-012` |
| AP2-007 | Concrete serializer allowlist bridge added / Rust binding open | Public native views, facade exports, replay rejection paths, state snapshots, and spent-ledger records have Rust no-selected-material checks. `Descender/Path2NoExport.lean` proves that accept/reject/storage serializer conformance supplies `noExportClosed`; `Descender/Path2NoExportConcrete.lean` adds tagged accept/reject/storage output frames and reduces a concrete serializer allowlist certificate to active no-export closure. Remaining work is binding that Lean serializer certificate to every concrete Rust accept/reject/storage output surface. | `P2FS-004` |

Latest verification: `lake env lean Descender/Path2NativeBoundary.lean`, `lake env lean Descender/Path2NativeAccountDesign.lean`, `lake env lean Descender/Path2FullSpendGame.lean`, `lake env lean Descender/Path2FullSpendConcrete.lean`, `lake env lean Descender/Path2HashBudget.lean`, `lake env lean Descender/Path2NoExport.lean`, `lake env lean Descender/Path2NoExportConcrete.lean`, `lake env lean Descender/Path2RoleRootBinding.lean`, `lake env lean Descender/Path2RoleRootConcrete.lean`, `lake env lean Descender/Path2AssemblyConstruction.lean`, `lake env lean Descender/Path2AssemblyNativeValidity.lean`, `lake env lean Descender/Path2CompatibilitySoundness.lean`, `lake env lean Descender/Path2CompatibilityConcrete.lean`, `lake env lean Descender/Path2FullOpeningFieldEvidence.lean`, `lake build Descender.Path2FullOpeningFieldEvidence`, `lake env lean Descender/Path2FullOpeningFieldEvidencePredicates.lean`, `lake build Descender.Path2FullOpeningFieldEvidencePredicates`, `lake env lean Descender/Path2FullOpeningFieldEvidenceByteBridge.lean`, `lake build Descender.Path2FullOpeningFieldEvidenceByteBridge`, `lake env lean Descender/Path2FullOpeningFieldEvidenceComposition.lean`, `lake build Descender.Path2FullOpeningFieldEvidenceComposition`, `lake env lean Descender/Path2FullOpeningSourceBridge.lean`, `lake build Descender.Path2FullOpeningSourceBridge`, `lake env lean Descender/Path2FullOpeningSourceConcrete.lean`, `lake build Descender.Path2FullOpeningSourceConcrete`, `lake env lean Descender/Path2FullOpeningFieldEvidenceSourceAdapters.lean`, `lake build Descender.Path2FullOpeningFieldEvidenceSourceAdapters`, `lake env lean Descender/Path2CheckedCoreByteBridge.lean`, `lake env lean Descender/Path2HashDomainAndMultiTarget.lean`, and `lake env lean Descender.lean` pass after building the new modules. The latest Lean pass includes `ReviewedFq12DeployableHiddenSourceInputs`, `reviewed_fq12_certificate_soundness_from_deployable_hidden_source_inputs`, `ReviewedFq12AssemblySourceRouteWitnessSource`, `ReviewedFq12CompatibilitySourceRouteWitnessSource`, `ReviewedFq12SourceRouteDeployableHiddenSourceInputs`, `reviewed_fq12_certificate_soundness_from_source_route_deployable_inputs`, `ReviewedFq12SourceRouteLocalInstantiationInputs`, `reviewed_fq12_certificate_soundness_from_source_route_local_instantiation_inputs`, `source_route_byte_source_assembly_field_binding_of_route_witness`, `source_route_byte_source_compatibility_field_binding_of_route_witness`, `source_route_byte_source_compatibility_extraction_of_route_witness`, `ReviewedFq12FullOpeningFieldSourceRouteCertificate`, `ReviewedFq12FullOpeningFieldSourceRouteBytePredicateSource`, `source_route_byte_predicate_source_active_reduction_facts_or_named_break`, `source_route_byte_source_local_assembly_predicates_or_break`, `source_route_byte_source_local_native_validity_or_break`, `source_route_byte_source_local_compatibility_extracts_or_break`, `source_route_byte_source_concrete_field_certificates_or_named_break`, `source_route_byte_source_concrete_field_certificates_or_boundary_break`, `source_route_byte_source_assembly_concrete_fields_or_boundary_break`, `source_route_byte_source_compatibility_concrete_fields_or_boundary_break`, and `source_route_byte_source_concrete_compatibility_extraction_or_boundary_break`. Rust targeted checks pass: `cargo fmt --check` in `tools/parameter-search`; `cargo test path2_reviewed_fq12_full_native_opening_certificate_binds_all_roles --lib` now covers `Path2ReviewedFq12FullOpeningFieldEvidenceV1`, `Path2ReviewedFq12FullOpeningFieldSourceRouteV1`, direct payload role-field checks, aggregate-source rejection for both field evidence and source route, self-frame role-root/full-opening/representative tamper rejection, compatibility digest rebinding rejection, stale field-evidence/source-route digest rejection, schema mismatch rejection, and constructor-level assembly role-root/opening-field/source mismatch rejection; `cargo test production_domain_schema_registry_is_globally_disjoint --lib` covers the reviewed-Fq12 assembly, compatibility, field-evidence, source-route, and aggregate-bridge frame domains; `cargo test path2_reviewed_fq12_full_native_opening_certificate_rejects_role_root_rebinding --lib`, `cargo test path2_native_checked_core_accept_pins_canonical_proof_bytes_before_acceptance --lib`, `cargo test path2_native_signer_verifier_evidence_keeps_productization_gates_separate --lib`, `cargo test --lib path2_native_prevalidated_honest_flow_reaches_selected_verifier_public_acceptance`, `cargo test --lib path2_native_prevalidated_signer_execution_report_keeps_public_reject_outside_private_path`, and `cargo test --lib path2_native_verifier_pipeline_is_public_only_and_fail_closed_until_productized`. Prior targeted checks also passed for hidden-source fail-closed evidence, checked-core frame, boundary assembly, lifecycle/spent-slot/prevalidated-slot replay controls, no-export public surfaces, hash/QROM/domain review, signer/keygen scope, deployment preflight, and the `tools/production-reference` facade boundary. The Rust hidden-source bridge audit intentionally keeps `semantics_bridge_complete = false` until `extractor_theorem_instantiated` is supplied.

Additional current Lean source-adapter coverage includes
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
bindings. It is not the active cryptographic closure target.
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
Rust hidden-source semantics audit now digest-binds the narrowed source-route
surface with `lean_source_route_local_instantiation_endpoint_present`,
`lean_boundary_availability_concrete_role_root_source_route_endpoint_present`,
`lean_factored_boundary_availability_concrete_role_root_source_route_endpoint_present`,
`lean_selected_rust_source_route_byte_source_endpoint_present`,
`lean_selected_rust_factored_concrete_local_source_route_endpoint_present`,
`lean_selected_rust_factored_concrete_local_functional_lift_source_route_endpoint_present`,
`lean_selected_rust_concrete_local_functional_lift_source_route_endpoint_present`,
`lean_selected_rust_packaged_source_route_endpoint_present`,
`lean_selected_rust_direct_assembly_native_digest_frame_packaged_source_route_endpoint_present`,
`lean_selected_rust_field_local_packaged_source_route_adapter_present`,
`rust_concrete_role_root_field_binding_present`,
`rust_certificate_lift_opening_functionality_present`,
`rust_assembly_representative_digest_lookup_source_present`,
`rust_representative_canonical_digest_source_present`,
`rust_full_native_opening_canonical_representative_source_present`,
`rust_full_native_opening_representative_frame_source_present`,
`rust_representative_digest_functionality_source_present`,
`rust_assembly_representative_frame_source_present`,
`rust_shared_representative_frame_source_present`,
`rust_native_representative_digest_frame_source_present`,
`rust_constructed_assembly_compatibility_witness_source_present`,
`rust_compatibility_route_witness_source_present`,
`rust_assembly_compatibility_local_model_source_present`,
`rust_full_opening_field_evidence_present`,
`rust_full_opening_source_route_present`, and
`rust_source_route_negative_controls_present`, while still keeping
`extractor_theorem_instantiated = false` and
`semantics_bridge_complete = false`. The production hidden-source semantics
gate/readiness report now also carry the bridge-audit digest and the
`semantics_bridge_complete` bit, so the deployability chain is digest-bound to
the narrowed active-path audit and remains blocked independently of the
acceptance-obligation bitmap. The top-level production preflight now also
reports the hidden-source semantics bridge audit as present and the bridge as
incomplete, keeping the hidden-source semantics blocker visible in the active
deployment surface.

## 2026-06-01 Parallel AP2 Batch

| Work item | Result | Still open |
| --- | --- | --- |
| AP2-N001 | `ReviewedFq12FullOpeningBridgeCertificate` now consumes assembly construction and assembled-native-validity soundness separately, deriving the old assembly soundness surface by theorem. Field evidence now has a Rust object, Lean mirror, sidecar predicate bridge, byte parser/frame hook bridge, and byte-source constructor for recovering the non-aggregate binding facts. Rust also exposes `Path2ReviewedFq12FullOpeningFieldSourceRouteV1`, which digest-binds the concrete source ordering: assembly full-opening field certificate, compatibility same-representative field certificate, and adapter-only field evidence before the aggregate bridge exists. Lean now mirrors that source route with `ReviewedFq12FullOpeningFieldSourceRouteCertificate`, route byte-parser/source structures, `source_route_byte_predicate_source_active_reduction_facts_or_named_break`, and `source_route_byte_source_concrete_field_certificates_or_named_break`. `Path2FullOpeningSourceBridge.lean` adds binding-or-break wrappers for assembly and compatibility and derives extraction-certificate soundness without aggregate-bridge premises. `Path2FullOpeningSourceConcrete.lean` now packages the deployable non-aggregate route as `ReviewedFq12DeployableHiddenSourceInputs` and adds reverse availability bridges from the base active-boundary availability surfaces. `Path2FullOpeningFieldEvidenceByteBridge.lean` now exposes `SelectedRustReviewedFq12FullOpeningFieldSourceRouteByteSource`, and `Path2FullOpeningFieldEvidenceSourceAdapters.lean` now packages the selected-Rust active route as `ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs`, deriving constructed-assembly compatibility from independent direct assembly binding plus the local model and deriving both representative-frame and assembly-frame canonicality from the 11-input selected Rust native representative digest frame rather than the aggregate bridge. The theorem `reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_native_digest_frame_packaged_source_route_inputs` is the current source-route P2FS-001 endpoint. The shared-frame package is a derived adapter; the field/local packaged theorem remains only an adapter for already-existing concrete records. | Instantiate `ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs` from the Rust verifier/model: role-root/lift source, selected Rust source-route byte source, independent direct assembly field binding, assembly/compatibility local-model source, and native representative digest-frame source for `path2_reviewed_fq12_native_representative_digest_v1`. |
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
| AP2-R003 | High | Instantiate `ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs` for the active Path 2 native / reviewed-Fq12 route: `ReviewedFq12RoleRootFunctionalLiftSource`, selected Rust field/source-route byte source, independent `ReviewedFq12ConcreteAssemblyFieldBindingOrBreak`, `ReviewedFq12AssemblyCompatibilityLocalModelSource`, and `ReviewedFq12SelectedRustNativeRepresentativeDigestFrameSource`. This wrapper derives constructed-assembly compatibility from direct assembly binding plus the local model, derives canonical representative construction from representative-frame digest canonicality plus `FullNativeOpeningMatchesRepresentative`, derives assembly-frame canonicality from the same 11-input selected Rust digest frame, and converts through the shared-frame adapter to the active `ReviewedFq12SelectedRustPackagedSourceRouteInputs` endpoint using independent direct assembly binding for compatibility-route construction. Selected source-route bytes remain the parser/frame bridge for an already witnessed active route; they are not used to source the direct assembly-binding premise. Started 2026-06-02: rechecked the source-route shape and Rust regressions; no aggregate-bridge source use found in the active field-evidence/source-route path, role-root rebinding tests still pass, and Lean exposes `reviewed_fq12_certificate_soundness_from_selected_rust_direct_assembly_native_digest_frame_packaged_source_route_inputs` as the narrowest current direct-assembly wrapper theorem over the exact packaged selected-Rust source-route closure endpoint. Rust inspection found canonical representative digest recomputation in the assembly certificate and field-evidence frames, compatibility/source-route propagation from the assembly digest, a representative-digest-only mutation negative control, and an omission negative control showing every active representative-digest input changes `path2_reviewed_fq12_native_representative_digest_v1`. The latest regression also mutates only the source-route `representative_digest`, recomputes the route digest, and confirms verifier reconstruction rejects it. The Rust fail-closed audit now digest-binds `lean_selected_rust_direct_assembly_native_digest_frame_packaged_source_route_endpoint_present`, `rust_native_representative_digest_frame_source_present`, `lean_selected_rust_direct_assembly_shared_representative_frame_packaged_source_route_endpoint_present`, `rust_shared_representative_frame_source_present`, `rust_representative_canonical_digest_source_present`, `rust_full_native_opening_canonical_representative_source_present`, `rust_full_native_opening_representative_frame_source_present`, `rust_representative_digest_functionality_source_present`, `rust_assembly_representative_frame_source_present`, `rust_constructed_assembly_compatibility_witness_source_present`, and `rust_independent_direct_assembly_binding_source_present`. Direct binding is backed by `path2_reviewed_fq12_assembly_binds_full_opening_fields_v1`; the regression asserts the valid assembly/full-opening pair binds and a local-section mutation fails before source-route acceptance. The remaining hard compatibility-route work is therefore to instantiate the native representative digest-frame source against `path2_reviewed_fq12_native_representative_digest_v1`, plus the already pinned direct assembly binding and local model. | `AP2-001`, `P2FS-001` |
| AP2-R004 | High | Bind `Path2FullSpendConcrete` to the real account creation, slot admission, spend verification, state transition, rollback model, and numeric full-forgery reduction. Started 2026-06-02: added `ConcreteFullSignatureExecutionSource` so production full-signature success is no longer required to be definitionally equal to `ConcreteSpendVerificationAccepts`; production must prove success implies verifier acceptance or a named break. | `AP2-003`, `P2FS-003`, `P2FS-013` |
| AP2-R005 | Medium | Close signer/keygen/completeness productization: honest constructor complete, signer/verifier pipeline present, CT/storage blockers retired, and honest flow verifies under the selected byte verifier in production mode. | `AP2-005`, `AP2-006`, `P2FS-009`, `P2FS-010`, `P2FS-011` |
| AP2-R006 | Medium | Bind `Path2NoExportConcrete` tagged output frames to every concrete Rust accept/reject/storage serializer. | `AP2-007`, `P2FS-004` |

## Near-Term Work Queue

| ID | Priority | Work item | Closes |
| --- | --- | --- | --- |
| AP2-N001 | Highest by proof dependency | Instantiate `ReviewedFq12SelectedRustDirectAssemblyNativeDigestFramePackagedSourceRouteInputs` from the Rust verifier/model: role-root/lift source, selected Rust source-route byte source, independent direct assembly field binding, assembly/compatibility local-model source, and the 11-input native representative digest-frame source for `path2_reviewed_fq12_native_representative_digest_v1`. This derives constructed-assembly compatibility from direct binding plus the local model and derives both representative-frame and assembly-frame canonicality from one Rust-shaped digest frame, while keeping selected source-route bytes compatibility-indexed and the aggregate bridge downstream only. | `AP2-001` |
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
