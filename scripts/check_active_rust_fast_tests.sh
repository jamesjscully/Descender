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

echo "== active fast test: direct spend rejection state no-mutation =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_single_slot_spend_rejections_preserve_consume_once_state

echo "== active fast test: direct public result class enum =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_public_result_class_v1_names_match_consensus_spec

echo "== active fast test: direct malformed signature rejection KATs =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_single_slot_spend_malformed_signature_kats_are_panic_free

echo "== active fast test: direct public object rejection KATs =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_single_slot_spend_public_object_rejection_kats_are_panic_free

echo "== active fast test: direct public result wrapper =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --lib path2_direct_single_slot_spend_public_result_wrapper_hides_internal_diagnostics

echo "== active fast test: recursion config scaffold smoke =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-recursion/Cargo.toml phase1_scaffold_wired_to_leaf_config
