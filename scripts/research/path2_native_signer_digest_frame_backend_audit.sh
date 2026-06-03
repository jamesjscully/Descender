#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
crate_dir="$repo_root/tools/parameter-search"

cd "$crate_dir"
cargo rustc --release --lib -- --emit=asm >/dev/null

asm_file="$(find target/release/deps -maxdepth 1 -name 'descender_parameter_search-*.s' | sort | tail -n 1)"
if [[ -z "$asm_file" ]]; then
  echo "missing release assembly output" >&2
  exit 1
fi

extract_function() {
  local symbol="$1"
  awk -v symbol="$symbol" '
    $0 ~ "^_ZN.*" symbol ".*:$" { in_body = 1 }
    in_body { print }
    in_body && /^\.Lfunc_end/ { exit }
  ' "$asm_file"
}

entropy_body="$(extract_function 'path2_native_signer_entropy_digest_v1')"
secret_body="$(extract_function 'path2_native_digest_secret_component_v1')"

if [[ -z "$entropy_body" ]]; then
  echo "path2_native_signer_entropy_digest_v1 symbol missing from release assembly" >&2
  exit 1
fi

if [[ -z "$secret_body" ]]; then
  echo "path2_native_digest_secret_component_v1 symbol missing from release assembly" >&2
  exit 1
fi

combined_body="$entropy_body
$secret_body"

if grep -Eq '__rust_alloc|__rust_dealloc|__rust_realloc|exchange_malloc|RawVec|alloc::raw_vec' <<<"$combined_body"; then
  echo "private signer digest frame constructor calls allocator path" >&2
  exit 1
fi

if ! grep -Eq 'subq[[:space:]]+\$504,[[:space:]]*%rsp' <<<"$entropy_body"; then
  echo "entropy digest constructor does not reserve expected fixed stack frame" >&2
  exit 1
fi

if ! grep -Eq 'subq[[:space:]]+\$504,[[:space:]]*%rsp' <<<"$secret_body"; then
  echo "secret digest constructor does not reserve expected fixed stack frame" >&2
  exit 1
fi

if ! grep -q 'memset' <<<"$combined_body"; then
  echo "fixed digest frame initialization marker missing" >&2
  exit 1
fi

entropy_zeroize_calls="$(grep -c 'callq.*path2_native_zeroize_signer_digest_frame_v1' <<<"$entropy_body" || true)"
secret_frame_zeroize_calls="$(grep -c 'callq.*path2_native_zeroize_signer_digest_frame_v1' <<<"$secret_body" || true)"
secret_seed_zeroize_calls="$(grep -c 'callq.*path2_native_zeroize_bytes_v1' <<<"$secret_body" || true)"
entropy_digest_zeroize_calls="$(grep -c 'callq.*path2_native_zeroize_bytes_v1' <<<"$entropy_body" || true)"

if [[ "$entropy_zeroize_calls" -ne 1 ]]; then
  echo "entropy digest constructor does not call fixed-frame zeroizer exactly once" >&2
  exit 1
fi

if [[ "$secret_frame_zeroize_calls" -ne 1 ]]; then
  echo "secret digest constructor does not call fixed-frame zeroizer exactly once" >&2
  exit 1
fi

if [[ "$entropy_digest_zeroize_calls" -ne 1 ]]; then
  echo "entropy digest constructor does not call local digest zeroizer exactly once" >&2
  exit 1
fi

if [[ "$secret_seed_zeroize_calls" -ne 2 ]]; then
  echo "secret digest constructor does not call seed and local digest zeroizers exactly once each" >&2
  exit 1
fi

entropy_private_sha256_calls="$(grep -c 'callq.*path2_native_private_sha256_digest_v1' <<<"$entropy_body" || true)"
secret_private_sha256_calls="$(grep -c 'callq.*path2_native_private_sha256_digest_v1' <<<"$secret_body" || true)"

if [[ "$entropy_private_sha256_calls" -ne 1 ]]; then
  echo "entropy digest constructor private SHA-256 call missing or duplicated" >&2
  exit 1
fi

if [[ "$secret_private_sha256_calls" -ne 1 ]]; then
  echo "secret digest constructor private SHA-256 call missing or duplicated" >&2
  exit 1
fi

if grep -q 'callq.*Digest.*digest' <<<"$combined_body"; then
  echo "private signer digest frame constructor still calls upstream Digest::digest" >&2
  exit 1
fi

entropy_sha="$(printf '%s' "$entropy_body" | sha256sum | awk '{print $1}')"
secret_sha="$(printf '%s' "$secret_body" | sha256sum | awk '{print $1}')"

printf 'schema=Descender.Path2.NativeSignerDigestFrameBackendAudit.v1\n'
printf 'asm_file=%s\n' "$asm_file"
printf 'entropy_digest_function=path2_native_signer_entropy_digest_v1\n'
printf 'entropy_fixed_stack_frame_bytes=504\n'
printf 'entropy_allocator_calls=0\n'
printf 'entropy_fixed_frame_zeroize_calls=%s\n' "$entropy_zeroize_calls"
printf 'entropy_local_digest_zeroize_calls=%s\n' "$entropy_digest_zeroize_calls"
printf 'entropy_private_sha256_calls=%s\n' "$entropy_private_sha256_calls"
printf 'entropy_upstream_digest_calls=0\n'
printf 'entropy_function_assembly_sha256=%s\n' "$entropy_sha"
printf 'secret_digest_function=path2_native_digest_secret_component_v1\n'
printf 'secret_fixed_stack_frame_bytes=504\n'
printf 'secret_allocator_calls=0\n'
printf 'secret_fixed_frame_zeroize_calls=%s\n' "$secret_frame_zeroize_calls"
printf 'secret_seed_and_local_digest_zeroize_calls=%s\n' "$secret_seed_zeroize_calls"
printf 'secret_private_sha256_calls=%s\n' "$secret_private_sha256_calls"
printf 'secret_upstream_digest_calls=0\n'
printf 'secret_function_assembly_sha256=%s\n' "$secret_sha"
