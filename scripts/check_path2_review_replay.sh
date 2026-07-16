#!/usr/bin/env bash
# Replay the fail-closed Path 2 direct-mode review package without running the
# full local build gate.
set -euo pipefail
cd "$(dirname "$0")/.."

: "${CARGO_BUILD_JOBS:=1}"
: "${CARGO_TARGET_DIR:=/tmp/descender-cargo-target}"
export CARGO_BUILD_JOBS
export CARGO_TARGET_DIR

tmp_files=()
cleanup() {
  if ((${#tmp_files[@]})); then
    rm -f "${tmp_files[@]}"
  fi
}
trap cleanup EXIT

run() {
  local label="$1"
  shift
  printf '== %s ==\n' "$label"
  "$@"
}

run_manifest_diff() {
  local label="$1"
  local expected="$2"
  shift 2
  local tmp
  tmp="$(mktemp)"
  tmp_files+=("$tmp")
  printf '== %s ==\n' "$label"
  "$@" > "$tmp"
  diff -u "$expected" "$tmp"
}

run "Lean direct QROM/action budget bridge" \
  lake env lean Descender/Path2HashBudget.lean

# selectedRustDirectQromActionTheoremLinkageReduction_v1
run "Lean direct spend theorem-linkage hooks" \
  lake env lean Descender/Path2FullSpendConcrete.lean

run_manifest_diff "account-creation frame KAT boundary manifest" \
  formal/kat/path2_account_creation_frame_kat_boundary_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_account_creation_frame_kat_manifest

run_manifest_diff "account-creation frame KAT vectors" \
  formal/kat/path2_account_creation_frame_kat_vectors_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_account_creation_frame_kat_vectors

run "account-creation frame KAT consistency" \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_account_creation_frame_kat_consistency

run_manifest_diff "direct QROM/action boundary manifest" \
  formal/kat/path2_direct_qrom_action_boundary_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_direct_qrom_action_boundary_manifest

run_manifest_diff "direct admission boundary manifest" \
  formal/kat/path2_direct_admission_boundary_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_direct_admission_boundary_manifest

run "direct admission boundary consistency" \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_direct_admission_boundary_consistency

run_manifest_diff "direct accepted spend tuple evidence manifest" \
  formal/kat/path2_direct_accepted_spend_tuple_evidence_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --features account-creation-stark-verifier \
    --bin path2_direct_accepted_spend_tuple_evidence_manifest

run_manifest_diff "direct accepted composition evidence manifest" \
  formal/kat/path2_direct_accepted_composition_evidence_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --features account-creation-stark-verifier \
    --bin path2_direct_accepted_composition_evidence_manifest

run_manifest_diff "direct theorem obligation vectors" \
  formal/kat/path2_direct_theorem_obligation_vectors_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_direct_theorem_obligation_vectors

run_manifest_diff "account-creation proof obligation vectors" \
  formal/kat/path2_account_creation_proof_obligation_vectors_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_account_creation_proof_obligation_vectors

run_manifest_diff "direct theorem boundary manifest" \
  formal/kat/path2_direct_theorem_boundary_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_direct_theorem_boundary_manifest

run_manifest_diff "account-creation proof boundary manifest" \
  formal/kat/path2_account_creation_proof_boundary_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_account_creation_proof_boundary_manifest

run_manifest_diff "direct lifecycle boundary manifest" \
  formal/kat/path2_direct_lifecycle_boundary_v1.json \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_direct_lifecycle_boundary_manifest

run "direct QROM/action boundary consistency" \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_direct_qrom_action_boundary_consistency

run "direct accepted spend tuple evidence consistency" \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --features account-creation-stark-verifier \
    --bin path2_direct_accepted_spend_tuple_evidence_consistency

run "direct accepted composition evidence consistency" \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --features account-creation-stark-verifier \
    --bin path2_direct_accepted_composition_evidence_consistency

run "direct theorem obligation vector consistency" \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_direct_theorem_obligation_vector_consistency

run "account-creation proof obligation vector consistency" \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_account_creation_proof_obligation_vector_consistency

run "boundary manifest consistency" \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_boundary_manifest_consistency

run "production preflight boundary consistency" \
  cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml \
    --bin path2_preflight_boundary_consistency

run "Path 2 leakage battery" \
  python3 scripts/check_path2_leakage_battery.py

run "documentation authority" \
  python3 scripts/check_doc_authority.py

run "tracker integrity" \
  python3 scripts/check_tracker_integrity.py

run "requirement traceability" \
  python3 scripts/check_requirement_traceability.py

run "active support imports" \
  python3 scripts/check_active_support_imports.py --max-unanchored 0

run "strict documentation classification" \
  python3 scripts/list_doc_classification_candidates.py --strict \
    --include-status-lines \
    --include-path2-research \
    --include-cryptanalysis-root \
    --include-all-markdown

run "review package manifest" \
  python3 scripts/check_path2_review_package.py
