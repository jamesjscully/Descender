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

soft_rs="$(dirname "$manifest_path")/src/sha256/soft.rs"
if [[ ! -f "$soft_rs" ]]; then
  echo "missing sha2 soft backend source file" >&2
  exit 1
fi

soft_source="$(cat "$soft_rs")"
if ! grep -q 'for block in blocks' <<<"$soft_source"; then
  echo "soft SHA-256 backend public block loop missing" >&2
  exit 1
fi

if ! grep -q 'block.chunks_exact(4)' <<<"$soft_source"; then
  echo "soft SHA-256 backend fixed 4-byte chunk loop missing" >&2
  exit 1
fi

if ! grep -q 'sha256_digest_block_u32(&mut state_cpy, &block_u32)' <<<"$soft_source"; then
  echo "soft SHA-256 backend fixed block compression call missing" >&2
  exit 1
fi

if grep -Eq 'if[[:space:]]|match[[:space:]]|while[[:space:]]' <<<"$soft_source"; then
  echo "unexpected source-level conditional in soft SHA-256 backend" >&2
  exit 1
fi

rm -f target/release/deps/libsha2-*.rlib target/release/deps/libsha2-*.rmeta
cargo build --release --lib >/dev/null

rlib="$(find target/release/deps -maxdepth 1 -name 'libsha2-*.rlib' | sort | tail -n 1)"
if [[ -z "$rlib" ]]; then
  echo "compiled sha2 rlib missing" >&2
  exit 1
fi

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
(
  cd "$tmp_dir"
  ar x "$crate_dir/$rlib"
)

symbol_index="$(
  cd "$tmp_dir"
  for obj in *.o; do
    nm -C "$obj" | sed "s|^|$obj |"
  done
)"

sha256_obj="$(awk '/ T sha2::sha256::compress256$/ { print $1; exit }' <<<"$symbol_index")"
if [[ -z "$sha256_obj" ]]; then
  echo "sha2::sha256::compress256 symbol missing from compiled rlib" >&2
  exit 1
fi

objdump_body="$(cd "$tmp_dir" && objdump -d -C "$sha256_obj")"
compress_body="$(
  awk '
    /^0+[[:xdigit:]]* <sha2::sha256::compress256>:/ { in_body = 1 }
    in_body { print }
    in_body && /^Disassembly of section/ && !/^Disassembly of section .*compress256/ { exit }
  ' <<<"$objdump_body"
)"

if [[ -z "$compress_body" ]]; then
  echo "failed to extract soft SHA-256 compression assembly" >&2
  exit 1
fi

conditional_branches="$(
  grep -E '^[[:space:]]*[[:xdigit:]]+:[[:space:]].*\bj[a-z]+\b' <<<"$compress_body" \
    | grep -Evc '\bjmp\b' || true
)"
unconditional_jumps="$(grep -Ec '^[[:space:]]*[[:xdigit:]]+:[[:space:]].*\bjmp\b' <<<"$compress_body" || true)"
calls="$(grep -Ec '^[[:space:]]*[[:xdigit:]]+:[[:space:]].*\bcall' <<<"$compress_body" || true)"
cmovs="$(grep -Ec '^[[:space:]]*[[:xdigit:]]+:[[:space:]].*\bcmov' <<<"$compress_body" || true)"
sets="$(grep -Ec '^[[:space:]]*[[:xdigit:]]+:[[:space:]].*\bset[a-z]+' <<<"$compress_body" || true)"

if [[ "$conditional_branches" -ne 2 ]]; then
  echo "unexpected conditional branch count in soft SHA-256 assembly: $conditional_branches" >&2
  exit 1
fi

if [[ "$unconditional_jumps" -ne 1 ]]; then
  echo "unexpected unconditional jump count in soft SHA-256 assembly: $unconditional_jumps" >&2
  exit 1
fi

if [[ "$calls" -ne 0 || "$cmovs" -ne 0 || "$sets" -ne 0 ]]; then
  echo "unexpected calls/cmov/setcc in soft SHA-256 assembly" >&2
  exit 1
fi

if grep -Eq '(rol|ror|shl|shr|sal|sar)[[:space:]]+%cl' <<<"$compress_body"; then
  echo "variable shift count found in soft SHA-256 assembly" >&2
  exit 1
fi

if grep -Eq '__rust_alloc|__rust_dealloc|__rust_realloc|memset|bzero|__mem|sha256rnds2|sha256msg1|sha256msg2|cpuid|shani_cpuid' <<<"$compress_body"; then
  echo "unexpected allocator/memory/SHA-NI/CPU-dispatch marker in soft SHA-256 assembly" >&2
  exit 1
fi

soft_source_sha="$(sha256sum "$soft_rs" | awk '{print $1}')"
compress_sha="$(printf '%s' "$compress_body" | sha256sum | awk '{print $1}')"

printf 'schema=Descender.Path2.NativeSignerSha256SoftCtShapeAudit.v1\n'
printf 'sha2_manifest=%s\n' "$manifest_path"
printf 'sha2_version=0.10.9\n'
printf 'force_soft_release_backend_selected=1\n'
printf 'source_public_block_loop_present=1\n'
printf 'source_fixed_chunk_loop_present=1\n'
printf 'source_unexpected_conditionals=0\n'
printf 'assembly_conditional_branches=%s\n' "$conditional_branches"
printf 'assembly_unconditional_jumps=%s\n' "$unconditional_jumps"
printf 'assembly_calls=0\n'
printf 'assembly_cmovs=0\n'
printf 'assembly_setcc=0\n'
printf 'assembly_variable_shifts=0\n'
printf 'assembly_allocator_mem_sha_ni_dispatch_markers=0\n'
printf 'sha256_soft_rs_sha256=%s\n' "$soft_source_sha"
printf 'compress256_assembly_sha256=%s\n' "$compress_sha"
printf 'constant_time_claim_complete=0\n'
