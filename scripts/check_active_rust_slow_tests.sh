#!/usr/bin/env bash
# Manual active Path 2 Rust slow tests. Not part of the default local/CI gate.
set -euo pipefail
cd "$(dirname "$0")/.."

echo "== active slow test: account STARK roundtrip and binding integration =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-stark/Cargo.toml --test roundtrip
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-stark/Cargo.toml --test context_binding
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-stark/Cargo.toml --test redteam_attacks

echo "== active slow test: parameter-search account STARK admission boundary =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/parameter-search/Cargo.toml --features account-creation-stark-verifier path2_account_stark_creation_roundtrip_and_binds

echo "== active slow test: empirical hiding mask channel battery =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-stark/Cargo.toml --test hiding_mask_channel

echo "== active slow test: deployed verifier binding integration =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-account-verifier/Cargo.toml --test deployed_binding

echo "== active slow test: recursion verifier-AIR suite =="
RUSTFLAGS="-D warnings" cargo test --manifest-path tools/descender-recursion/Cargo.toml
