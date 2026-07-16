#!/usr/bin/env bash
# Fast active Path 2 Rust tests for the default local/CI gate.
set -euo pipefail
cd "$(dirname "$0")/.."

echo "== active fast test: signer-side mask PRF =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-mask-prf/Cargo.toml

echo "== active fast test: account STARK mask-PRF equivalence =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-stark/Cargo.toml --test mask_prf_equivalence

echo "== active fast test: account STARK context soundness smoke =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-stark/Cargo.toml --test soundness_context_binding
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-stark/Cargo.toml --test context_binding completeness_context_matches

echo "== active fast test: verifier export bridge smoke =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-verifier/Cargo.toml role_root_export_digest_is_domain_separated_and_ordered
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-verifier/Cargo.toml public_wire_field_elements_must_be_canonical
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-verifier/Cargo.toml creation_nullifier_bytes_are_canonical_and_version_independent

echo "== active fast test: direct spend rejection state no-mutation =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_single_slot_spend_rejections_preserve_consume_once_state

echo "== active fast test: direct accepted tuple evidence =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_single_slot_spend_tuple_evidence_pins_accepted_tuple

echo "== active fast test: sovereign direct admission state and data availability =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_sovereign_direct_admission_

echo "== active fast test: verified account-STARK block admission rollback =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --features account-creation-stark-verifier --lib path2_account_stark_creation_roundtrip_and_binds

echo "== active fast test: account-creation accepted proof-boundary evidence =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --features account-creation-stark-verifier --lib path2_account_creation_accepted_proof_boundary_evidence_pins_selected_payload

echo "== active fast test: direct accepted composition evidence =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --features account-creation-stark-verifier --lib path2_direct_accepted_composition_evidence_binds_creation_and_spend

echo "== active fast test: direct public result class enum =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_public_result_class_v1_names_match_consensus_spec

echo "== active fast test: direct malformed signature rejection KATs =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_single_slot_spend_malformed_signature_kats_are_panic_free

echo "== active fast test: direct public object rejection KATs =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_single_slot_spend_public_object_rejection_kats_are_panic_free

echo "== active fast test: direct public result wrapper =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_single_slot_spend_public_result_wrapper_hides_internal_diagnostics

echo "== active fast test: exhaustive direct public rejection classes =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_public_rejection_classes_are_exhaustive_and_state_preserving

echo "== active fast test: direct consensus-visible public object mutations =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_all_consensus_visible_public_object_mutations_are_panic_free

echo "== active fast test: sovereign direct client rejection mapping =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_sovereign_direct_client_mapping_is_exhaustive_and_state_preserving

echo "== active fast test: sanitized callback boundary evidence =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib sanitized_callback_wrappers_do_not_echo_secret_errors
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib sanitized_callback_wrappers_have_bounded_invocation_shape
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib callback_boundary_classification_marks_only_sanitized_paths_deployment_facing
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib deployment_callback_boundary_gate_rejects_diagnostic_paths
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib default_deployment_callback_profile_is_complete_and_sanitized

echo "== active fast test: storage failure fail-closed evidence =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib chain_state_file_stores_lifecycle_save_failure_is_fail_closed_after_ledger_persist

echo "== active fast test: entropy failure boundary evidence =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_native_os_entropy_boundary_audit_pins_sanitized_source_boundary

echo "== active fast test: recursion config scaffold smoke =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-recursion/Cargo.toml phase1_scaffold_wired_to_leaf_config
