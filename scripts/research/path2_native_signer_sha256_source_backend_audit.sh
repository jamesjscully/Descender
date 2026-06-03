#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
crate_dir="$repo_root/tools/parameter-search"

cd "$crate_dir"
manifest_path="$(cargo metadata --format-version 1 --locked 2>/dev/null \
  | jq -r '.packages[] | select(.name=="sha2" and .version=="0.10.9") | .manifest_path')"

if [[ -z "$manifest_path" || "$manifest_path" == "null" ]]; then
  echo "sha2 0.10.9 manifest not found in locked cargo metadata" >&2
  exit 1
fi

sha2_dir="$(dirname "$manifest_path")"
sha256_rs="$sha2_dir/src/sha256.rs"
soft_rs="$sha2_dir/src/sha256/soft.rs"

for path in "$sha256_rs" "$soft_rs"; do
  if [[ ! -f "$path" ]]; then
    echo "missing sha2 source file: $path" >&2
    exit 1
  fi
done

if grep -q 'sha2-asm' Cargo.lock; then
  echo "sha2-asm appears in active lockfile" >&2
  exit 1
fi

sha256_source="$(cat "$sha256_rs")"
soft_source="$(cat "$soft_rs")"
feature_tree="$(cargo tree -p descender-parameter-search -e features --locked)"

if ! grep -q 'sha2 feature "force-soft"' <<<"$feature_tree"; then
  echo "active crate does not enable sha2 force-soft feature" >&2
  exit 1
fi

if ! grep -q 'cfg(feature = "force-soft")' <<<"$sha256_source"; then
  echo "sha256.rs does not expose force-soft backend branch" >&2
  exit 1
fi

if ! grep -q 'mod soft;' <<<"$sha256_source"; then
  echo "sha256.rs force-soft branch does not load soft backend" >&2
  exit 1
fi

if ! grep -q 'use soft::compress;' <<<"$sha256_source"; then
  echo "sha256.rs force-soft branch does not route through soft::compress" >&2
  exit 1
fi

if ! grep -Fq 'pub fn compress(state: &mut [u32; 8], blocks: &[[u8; 64]])' <<<"$soft_source"; then
  echo "soft SHA-256 backend compress signature not found" >&2
  exit 1
fi

if ! grep -q 'for block in blocks' <<<"$soft_source"; then
  echo "soft SHA-256 backend block loop not found" >&2
  exit 1
fi

if grep -Eq 'secret|key|branch.*message' <<<"$sha256_source$soft_source"; then
  echo "unexpected secret/key marker in sha2 backend source" >&2
  exit 1
fi

sha256_sha="$(sha256sum "$sha256_rs" | awk '{print $1}')"
soft_sha="$(sha256sum "$soft_rs" | awk '{print $1}')"

printf 'schema=Descender.Path2.NativeSignerSha256SourceBackendAudit.v1\n'
printf 'sha2_manifest=%s\n' "$manifest_path"
printf 'sha2_version=0.10.9\n'
printf 'sha2_asm_dependency_present=0\n'
printf 'force_soft_feature_enabled=1\n'
printf 'force_soft_source_branch_present=1\n'
printf 'runtime_x86_dispatch_selected=0\n'
printf 'soft_backend_selected=1\n'
printf 'soft_backend_block_loop_present=1\n'
printf 'sha256_rs_sha256=%s\n' "$sha256_sha"
printf 'sha256_soft_rs_sha256=%s\n' "$soft_sha"
printf 'constant_time_claim_complete=0\n'
