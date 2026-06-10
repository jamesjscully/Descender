#!/usr/bin/env bash
# Local mirror of the CI build gate.  Run before pushing.
set -euo pipefail
cd "$(dirname "$0")/.."

echo "== lake build =="
lake build

echo "== semantic theorem audit =="
lake env lean --run scripts/audit_theorems.lean

echo "== cargo check --all-targets =="
cargo check --manifest-path tools/parameter-search/Cargo.toml --all-targets

echo "All build gates passed."
