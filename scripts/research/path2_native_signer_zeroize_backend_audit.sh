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

bytes_body="$(extract_function 'path2_native_zeroize_bytes_v1')"
u16s_body="$(extract_function 'path2_native_zeroize_u16s_v1')"
u32s_body="$(extract_function 'path2_native_zeroize_u32s_v1')"
digest_frame_body="$(extract_function 'path2_native_zeroize_signer_digest_frame_v1')"

if [[ -z "$bytes_body" ]]; then
  echo "path2_native_zeroize_bytes_v1 symbol missing from release assembly" >&2
  exit 1
fi

if [[ -z "$u16s_body" ]]; then
  echo "path2_native_zeroize_u16s_v1 symbol missing from release assembly" >&2
  exit 1
fi

if [[ -z "$u32s_body" ]]; then
  echo "path2_native_zeroize_u32s_v1 symbol missing from release assembly" >&2
  exit 1
fi

if [[ -z "$digest_frame_body" ]]; then
  echo "path2_native_zeroize_signer_digest_frame_v1 symbol missing from release assembly" >&2
  exit 1
fi

byte_zero_stores="$(grep -Ec '^[[:space:]]*movb[[:space:]]+\$0,' <<<"$bytes_body" || true)"
word_zero_stores="$(grep -Ec '^[[:space:]]*movw[[:space:]]+\$0,' <<<"$u16s_body" || true)"
u32_zero_stores="$(grep -Ec '^[[:space:]]*movl[[:space:]]+\$0,' <<<"$u32s_body" || true)"
digest_frame_zero_stores="$(grep -Ec '^[[:space:]]*movb[[:space:]]+\$0,' <<<"$digest_frame_body" || true)"
byte_barriers="$(grep -c '#MEMBARRIER' <<<"$bytes_body" || true)"
word_barriers="$(grep -c '#MEMBARRIER' <<<"$u16s_body" || true)"
u32_barriers="$(grep -c '#MEMBARRIER' <<<"$u32s_body" || true)"
digest_frame_barriers="$(grep -c '#MEMBARRIER' <<<"$digest_frame_body" || true)"

if [[ "$byte_zero_stores" -ne 9 ]]; then
  echo "expected dynamic byte zeroizer to expose one peel store plus eight unrolled stores, found $byte_zero_stores" >&2
  exit 1
fi

if [[ "$word_zero_stores" -ne 12 ]]; then
  echo "expected 12 u16 zero stores, found $word_zero_stores" >&2
  exit 1
fi

if [[ "$u32_zero_stores" -ne 9 ]]; then
  echo "expected dynamic u32 zeroizer to expose one peel store plus eight unrolled stores, found $u32_zero_stores" >&2
  exit 1
fi

if [[ "$byte_barriers" -lt 1 || "$word_barriers" -lt 1 || "$u32_barriers" -lt 1 ]]; then
  echo "compiler fence marker missing from zeroize assembly" >&2
  exit 1
fi

if [[ "$digest_frame_zero_stores" -ne 8 || "$digest_frame_barriers" -lt 1 ]]; then
  echo "fixed signer digest frame zeroize helper does not expose eight unrolled volatile zero stores plus fence" >&2
  exit 1
fi

if ! grep -Eq 'cmpq[[:space:]]+\$455,' <<<"$digest_frame_body"; then
  echo "fixed signer digest frame zeroize helper does not pin a 448-byte loop bound" >&2
  exit 1
fi

if grep -Eq 'memset|bzero|__mem' <<<"$bytes_body$u16s_body$u32s_body$digest_frame_body"; then
  echo "zeroize helper lowered to external memory routine" >&2
  exit 1
fi

if ! grep -Eq 'jne[[:space:]]+\.LBB' <<<"$bytes_body"; then
  echo "dynamic byte zeroizer loop branch marker missing" >&2
  exit 1
fi

if ! grep -Eq 'jne[[:space:]]+\.LBB' <<<"$u32s_body"; then
  echo "dynamic u32 zeroizer loop branch marker missing" >&2
  exit 1
fi

if grep -Eq '^[[:space:]]*j[a-z]+[[:space:]]' <<<"$u16s_body"; then
  echo "conditional branch found in fixed-size u16 zeroize helper assembly" >&2
  exit 1
fi

bytes_sha="$(printf '%s' "$bytes_body" | sha256sum | awk '{print $1}')"
u16s_sha="$(printf '%s' "$u16s_body" | sha256sum | awk '{print $1}')"
u32s_sha="$(printf '%s' "$u32s_body" | sha256sum | awk '{print $1}')"
digest_frame_sha="$(printf '%s' "$digest_frame_body" | sha256sum | awk '{print $1}')"
printf 'schema=Descender.Path2.NativeSignerZeroizeBackendAudit.v1\n'
printf 'asm_file=%s\n' "$asm_file"
printf 'byte_zeroize_function=path2_native_zeroize_bytes_v1\n'
printf 'byte_zero_stores=%s\n' "$byte_zero_stores"
printf 'byte_zeroize_dynamic_public_length_loop=1\n'
printf 'byte_compiler_fence_present=1\n'
printf 'byte_function_assembly_sha256=%s\n' "$bytes_sha"
printf 'u16_zeroize_function=path2_native_zeroize_u16s_v1\n'
printf 'u16_zero_stores=%s\n' "$word_zero_stores"
printf 'u16_compiler_fence_present=1\n'
printf 'u16_function_assembly_sha256=%s\n' "$u16s_sha"
printf 'u32_zeroize_function=path2_native_zeroize_u32s_v1\n'
printf 'u32_zero_stores=%s\n' "$u32_zero_stores"
printf 'u32_zeroize_dynamic_public_length_loop=1\n'
printf 'u32_compiler_fence_present=1\n'
printf 'u32_function_assembly_sha256=%s\n' "$u32s_sha"
printf 'digest_frame_zeroize_function=path2_native_zeroize_signer_digest_frame_v1\n'
printf 'digest_frame_zero_stores=%s\n' "$digest_frame_zero_stores"
printf 'digest_frame_loop_bound_bytes=448\n'
printf 'digest_frame_compiler_fence_present=1\n'
printf 'digest_frame_function_assembly_sha256=%s\n' "$digest_frame_sha"
