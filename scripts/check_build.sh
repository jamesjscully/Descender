#!/usr/bin/env bash
# Local mirror of the CI build gate.  Run before pushing.
set -euo pipefail
cd "$(dirname "$0")/.."

echo "== Lean build =="
lake build

echo "== documentation authority check =="
python3 scripts/check_doc_authority.py

echo "== ADR immutable-section check =="
python3 scripts/check_adr_immutable_sections.py

echo "== process gate coverage check =="
python3 scripts/check_process_gate_coverage.py

echo "== documentation classification inventory =="
python3 scripts/list_doc_classification_candidates.py --strict --include-status-lines --include-path2-research --include-cryptanalysis-root --include-all-markdown

echo "== archive mixed-status audit =="
python3 scripts/check_archive_mixed_status.py --max-mixed 0

echo "== archive legacy-count audit =="
python3 scripts/check_archive_legacy_counts.py --max-non-active 10 --max ARCHIVED_TARGET=0 --max LEGACY_FORMAL=4 --max MIXED_STATUS=0 --max NEGATIVE_CONTROL=6 --max SUPERSEDED_DECISION=0 --max SUPERSEDED_TRACKER=0

echo "== cleanup queue check =="
python3 scripts/check_cleanup_queue.py

echo "== retired path absence check =="
python3 scripts/check_retired_path_absence.py

echo "== active support import audit =="
python3 scripts/check_active_support_imports.py --max-unanchored 0

echo "== active authority stale-term check =="
python3 scripts/check_active_authority_stale_terms.py

echo "== legacy code framing check =="
python3 scripts/check_legacy_code_framing.py

echo "== tool manifest framing check =="
python3 scripts/check_tool_manifest_framing.py

echo "== active source framing check =="
python3 scripts/check_active_source_framing.py

echo "== CI/local gate parity check =="
python3 scripts/check_ci_local_gate_parity.py

echo "== tracker integrity check =="
python3 scripts/check_tracker_integrity.py

echo "== requirement traceability check =="
python3 scripts/check_requirement_traceability.py

echo "== wire-format authority check =="
python3 scripts/check_wire_format_authority.py

echo "== active Path 2 Rust crate checks =="
RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-mask-prf/Cargo.toml --all-targets
RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-account-stark/Cargo.toml --all-targets
RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-account-verifier/Cargo.toml --all-targets
RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-recursion/Cargo.toml --all-targets

echo "== active Path 2 fast Rust tests =="
bash scripts/check_active_rust_fast_tests.sh

echo "== research automation routing check =="
python3 scripts/research/check_research_automation_routing.py

echo "== research queue pivot audit =="
python3 scripts/research/audit_task_pivot_alignment.py --strict-queue

echo "== semantic theorem audit =="
lake env lean --run scripts/audit_theorems.lean

echo "== Lean theorem statement drift check =="
git diff --exit-code audit/lean_theorem_statements.txt

echo "== Rust all-target check =="
cargo check --manifest-path tools/parameter-search/Cargo.toml --all-targets

payload_byte_model_manifest_tmp="$(mktemp)"
payload_kat_manifest_tmp="$(mktemp)"
payload_kat_vectors_tmp="$(mktemp)"
fri_qrom_residual_manifest_tmp="$(mktemp)"
proof_boundary_manifest_tmp="$(mktemp)"
direct_theorem_manifest_tmp="$(mktemp)"
trap 'rm -f "$payload_byte_model_manifest_tmp" "$payload_kat_manifest_tmp" "$payload_kat_vectors_tmp" "$fri_qrom_residual_manifest_tmp" "$proof_boundary_manifest_tmp" "$direct_theorem_manifest_tmp"' EXIT

echo "== account STARK payload byte model manifest check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_payload_byte_model_manifest > "$payload_byte_model_manifest_tmp"
diff -u formal/kat/path2_account_stark_payload_byte_model_v1.json "$payload_byte_model_manifest_tmp"

echo "== account STARK payload KAT boundary manifest check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_payload_kat_manifest > "$payload_kat_manifest_tmp"
diff -u formal/kat/path2_account_stark_payload_kat_boundary_v1.json "$payload_kat_manifest_tmp"

echo "== account STARK FRI/QROM residual manifest check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_fri_qrom_residual_manifest > "$fri_qrom_residual_manifest_tmp"
diff -u formal/kat/path2_account_stark_fri_qrom_residual_v1.json "$fri_qrom_residual_manifest_tmp"

echo "== account STARK FRI/QROM residual consistency check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_fri_qrom_residual_consistency

echo "== account-creation proof boundary manifest check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_creation_proof_boundary_manifest > "$proof_boundary_manifest_tmp"
diff -u formal/kat/path2_account_creation_proof_boundary_v1.json "$proof_boundary_manifest_tmp"

echo "== direct theorem boundary manifest check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_theorem_boundary_manifest > "$direct_theorem_manifest_tmp"
diff -u formal/kat/path2_direct_theorem_boundary_v1.json "$direct_theorem_manifest_tmp"

echo "== boundary manifest consistency check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_boundary_manifest_consistency

echo "== preflight boundary consistency check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_preflight_boundary_consistency

echo "== account STARK payload KAT vector corpus check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --features account-creation-stark-verifier --bin path2_account_stark_payload_kat_vectors > "$payload_kat_vectors_tmp"
diff -u formal/kat/path2_account_stark_payload_kat_vectors_v1.json "$payload_kat_vectors_tmp"

echo "== account STARK payload KAT vector consistency check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_payload_kat_vector_consistency

echo "== account STARK payload byte model consistency check =="
cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_payload_byte_model_consistency

echo "== legacy q12289 negative-control bin tests =="
cargo test --manifest-path tools/parameter-search/Cargo.toml --bin signature_q12289_local_delta_stateful

echo "== legacy KAT v0 byte-conformance =="
cargo run --manifest-path tools/parameter-search/Cargo.toml --bin signature_q12289_local_delta_stateful -- --kat-check

echo "== estimator/reference constant sync =="
python3 scripts/research/check_estimator_param_sync.py

echo "== QROM budget artifact regeneration =="
python3 scripts/research/qrom_challenge_budget_memo.py
python3 scripts/research/qrom_security_loss_review.py
python3 scripts/research/pre_third_party_qrom_frame_probe.py
python3 scripts/research/qrom_multitarget_margin.py

echo "== QROM budget/reference sync =="
python3 scripts/research/check_qrom_budget_sync.py

echo "== QROM generated artifact drift check =="
python3 scripts/research/check_qrom_generated_artifact_drift.py

echo "== Fq12 reference irreducibility check =="
python3 scripts/research/verify_fq12_irreducibility.py

echo "== checked-core concrete-security support smoke =="
python3 scripts/research/cryptanalysis_concrete_security_estimate.py

echo "All build gates passed."
