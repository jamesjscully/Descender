#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
crate_dir="$repo_root/tools/parameter-search"

cd "$crate_dir"
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

if grep -q 'sha2::sha256::x86::digest_blocks$' <<<"$symbol_index"; then
  echo "sha2 x86 SHA-NI digest_blocks symbol present despite force-soft policy" >&2
  exit 1
fi

if grep -q 'sha2::sha256::x86::shani_cpuid::STORAGE$' <<<"$symbol_index"; then
  echo "sha2 shani cpuid storage symbol present despite force-soft policy" >&2
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
  echo "failed to extract sha2 compression release assembly" >&2
  exit 1
fi

if grep -Eq '__rust_alloc|__rust_dealloc|__rust_realloc|memset|bzero|__mem' <<<"$compress_body"; then
  echo "sha2 compression backend unexpectedly calls allocator or memory routine" >&2
  exit 1
fi

if grep -Eq 'sha256rnds2|sha256msg1|sha256msg2' <<<"$compress_body"; then
  echo "SHA-NI instruction present despite force-soft policy" >&2
  exit 1
fi

if grep -Eq 'shani_cpuid|cpufeature|cpuid' <<<"$compress_body"; then
  echo "CPU feature dispatch marker present despite force-soft policy" >&2
  exit 1
fi

if ! grep -Eq 'test[[:space:]]+%rdx,%rdx|test[[:space:]]+%r[a-z0-9]+,%r[a-z0-9]+' <<<"$compress_body"; then
  echo "soft compress public block-count guard missing" >&2
  exit 1
fi

compress_sha="$(printf '%s' "$compress_body" | sha256sum | awk '{print $1}')"

printf 'schema=Descender.Path2.NativeSignerSha256ReleaseBackendAudit.v1\n'
printf 'rlib=%s\n' "$rlib"
printf 'object=%s\n' "$sha256_obj"
printf 'compress256_symbol_present=1\n'
printf 'force_soft_release_backend_selected=1\n'
printf 'x86_digest_blocks_symbol_present=0\n'
printf 'shani_cpuid_storage_present=0\n'
printf 'allocator_or_memroutine_calls=0\n'
printf 'public_cpufeature_dispatch_present=0\n'
printf 'sha256rnds2_count=0\n'
printf 'sha256msg1_count=0\n'
printf 'sha256msg2_count=0\n'
printf 'soft_public_block_count_guard_present=1\n'
printf 'compress256_assembly_sha256=%s\n' "$compress_sha"
printf 'constant_time_claim_complete=0\n'
