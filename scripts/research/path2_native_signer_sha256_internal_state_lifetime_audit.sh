#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
crate_dir="$repo_root/tools/parameter-search"
source_file="$crate_dir/src/production_reference.rs"

cd "$crate_dir"

extract_source_function() {
  local symbol="$1"
  awk -v symbol="$symbol" '
    index($0, "fn " symbol "(") { in_body = 1; depth = 0 }
    in_body {
      print
      for (idx = 1; idx <= length($0); idx++) {
        ch = substr($0, idx, 1)
        if (ch == "{") depth++
        if (ch == "}") depth--
      }
      if (depth == 0 && $0 ~ /}/) exit
    }
  ' "$source_file"
}

private_sha_source="$(extract_source_function 'path2_native_private_sha256_digest_v1')"
entropy_source="$(extract_source_function 'path2_native_signer_entropy_digest_v1')"
secret_source="$(extract_source_function 'path2_native_digest_secret_component_v1')"

if [[ -z "$private_sha_source" ]]; then
  echo "path2_native_private_sha256_digest_v1 source missing" >&2
  exit 1
fi

if [[ -z "$entropy_source" || -z "$secret_source" ]]; then
  echo "private signer digest source wrappers missing" >&2
  exit 1
fi

if ! grep -q 'MaybeUninit::<\[u8; PATH2_NATIVE_SIGNER_PRIVATE_SHA256_WORKSPACE_BYTES\]>::uninit();' <<<"$private_sha_source"; then
  echo "private SHA-256 workspace marker missing" >&2
  exit 1
fi

if ! grep -q 'ptr::write_volatile(workspace_ptr.add(idx), 0);' <<<"$private_sha_source"; then
  echo "private SHA-256 volatile workspace initialization marker missing" >&2
  exit 1
fi

if ! grep -q 'let mut schedule = \[0_u32; 64\];' <<<"$private_sha_source"; then
  echo "private SHA-256 schedule marker missing" >&2
  exit 1
fi

if ! grep -q 'let mut state = PATH2_NATIVE_PRIVATE_SHA256_IV_V1;' <<<"$private_sha_source"; then
  echo "private SHA-256 state marker missing" >&2
  exit 1
fi

if ! grep -q 'path2_native_zeroize_bytes_v1(workspace);' <<<"$private_sha_source"; then
  echo "private SHA-256 workspace zeroize marker missing" >&2
  exit 1
fi

if ! grep -q 'path2_native_zeroize_u32s_v1(&mut schedule);' <<<"$private_sha_source"; then
  echo "private SHA-256 schedule zeroize marker missing" >&2
  exit 1
fi

if ! grep -q 'path2_native_zeroize_u32s_v1(&mut state);' <<<"$private_sha_source"; then
  echo "private SHA-256 state zeroize marker missing" >&2
  exit 1
fi

if ! grep -q 'path2_native_private_sha256_digest_v1(frame.as_slice(), &mut digest)' <<<"$entropy_source"; then
  echo "entropy digest wrapper does not use private SHA-256" >&2
  exit 1
fi

if ! grep -q 'path2_native_private_sha256_digest_v1(frame.as_slice(), &mut digest)' <<<"$secret_source"; then
  echo "secret digest wrapper does not use private SHA-256" >&2
  exit 1
fi

if grep -q 'Sha256::digest(frame.as_slice())' <<<"$entropy_source$secret_source"; then
  echo "private digest wrapper still uses upstream Sha256::digest" >&2
  exit 1
fi

cargo rustc --release --lib -- --emit=asm >/dev/null

asm_file="$(find target/release/deps -maxdepth 1 -name 'descender_parameter_search-*.s' | sort | tail -n 1)"
if [[ -z "$asm_file" ]]; then
  echo "missing release assembly output" >&2
  exit 1
fi

extract_asm_function() {
  local symbol="$1"
  awk -v symbol="$symbol" '
    $0 ~ "^_ZN.*" symbol ".*:$" { in_body = 1 }
    in_body { print }
    in_body && /^\.Lfunc_end/ { exit }
  ' "$asm_file"
}

private_sha_body="$(extract_asm_function 'path2_native_private_sha256_digest_v1')"
entropy_body="$(extract_asm_function 'path2_native_signer_entropy_digest_v1')"
secret_body="$(extract_asm_function 'path2_native_digest_secret_component_v1')"

if [[ -z "$private_sha_body" ]]; then
  echo "path2_native_private_sha256_digest_v1 symbol missing from release assembly" >&2
  exit 1
fi

if [[ -z "$entropy_body" || -z "$secret_body" ]]; then
  echo "private signer digest wrapper symbol missing from release assembly" >&2
  exit 1
fi

combined_private_wrapper_body="$entropy_body
$secret_body"

workspace_zeroize_calls="$(grep -c 'callq.*path2_native_zeroize_bytes_v1' <<<"$private_sha_body" || true)"
u32_zeroize_calls="$(grep -c 'callq.*path2_native_zeroize_u32s_v1' <<<"$private_sha_body" || true)"
entropy_private_sha256_calls="$(grep -c 'callq.*path2_native_private_sha256_digest_v1' <<<"$entropy_body" || true)"
secret_private_sha256_calls="$(grep -c 'callq.*path2_native_private_sha256_digest_v1' <<<"$secret_body" || true)"
first_output_store_line="$(grep -nE 'movl[[:space:]]+%r?15d,[[:space:]]*\(%rax\)' <<<"$private_sha_body" | head -n 1 | cut -d: -f1 || true)"
first_zeroize_line="$(grep -n 'callq.*path2_native_zeroize_bytes_v1' <<<"$private_sha_body" | head -n 1 | cut -d: -f1 || true)"
last_zeroize_line="$(grep -n 'callq.*path2_native_zeroize_u32s_v1' <<<"$private_sha_body" | tail -n 1 | cut -d: -f1 || true)"
post_zeroize_tail="$(
  if [[ -n "$last_zeroize_line" ]]; then
    tail -n "+$((last_zeroize_line + 1))" <<<"$private_sha_body"
  fi
)"
post_zeroize_movs="$(grep -Ec '^[[:space:]]*mov' <<<"$post_zeroize_tail" || true)"

if [[ "$workspace_zeroize_calls" -ne 1 ]]; then
  echo "private SHA-256 release body does not call byte workspace zeroizer exactly once" >&2
  exit 1
fi

if [[ "$u32_zeroize_calls" -ne 2 ]]; then
  echo "private SHA-256 release body does not call u32 zeroizer exactly twice" >&2
  exit 1
fi

if [[ "$entropy_private_sha256_calls" -ne 1 || "$secret_private_sha256_calls" -ne 1 ]]; then
  echo "private signer wrappers do not call private SHA-256 exactly once" >&2
  exit 1
fi

if [[ -z "$first_output_store_line" || -z "$first_zeroize_line" || "$first_output_store_line" -ge "$first_zeroize_line" ]]; then
  echo "private SHA-256 release body does not write digest output before internal cleanup" >&2
  exit 1
fi

if ! grep -q 'leaq[[:space:]]*360(%rsp),[[:space:]]*%rdi' <<<"$private_sha_body"; then
  echo "private SHA-256 workspace zeroize stack region marker missing" >&2
  exit 1
fi

if ! grep -q 'leaq[[:space:]]*96(%rsp),[[:space:]]*%rdi' <<<"$private_sha_body"; then
  echo "private SHA-256 schedule zeroize stack region marker missing" >&2
  exit 1
fi

if ! grep -q 'leaq[[:space:]]*40(%rsp),[[:space:]]*%rdi' <<<"$private_sha_body"; then
  echo "private SHA-256 state zeroize stack region marker missing" >&2
  exit 1
fi

if [[ "$post_zeroize_movs" -ne 0 ]]; then
  echo "private SHA-256 release body has post-cleanup mov instructions before return" >&2
  exit 1
fi

if grep -Eq 'Digest.*digest|sha2::sha256::compress256|compress256' <<<"$combined_private_wrapper_body"; then
  echo "private signer wrappers still reference upstream digest/compress path" >&2
  exit 1
fi

if grep -Eq 'sha256rnds2|sha256msg|cpuid|x86_feature_detected|sha_ni|sha2_asm' <<<"$private_sha_body"; then
  echo "private SHA-256 release body contains SHA-NI/dispatch marker" >&2
  exit 1
fi

if grep -Eq '__rust_alloc|__rust_dealloc|__rust_realloc|exchange_malloc|RawVec|alloc::raw_vec' <<<"$private_sha_body"; then
  echo "private SHA-256 release body references allocator path" >&2
  exit 1
fi

private_source_sha="$(printf '%s' "$private_sha_source" | sha256sum | awk '{print $1}')"
private_asm_sha="$(printf '%s' "$private_sha_body" | sha256sum | awk '{print $1}')"

printf 'schema=Descender.Path2.NativeSignerSha256InternalStateLifetimeAudit.v1\n'
printf 'active_private_sha256_path=1\n'
printf 'upstream_private_sha256_digest_calls=0\n'
printf 'private_sha256_workspace_bytes=%s\n' '512'
printf 'private_sha256_schedule_words=64\n'
printf 'private_sha256_state_words=8\n'
printf 'private_sha256_workspace_zeroize_present=1\n'
printf 'private_sha256_schedule_zeroize_present=1\n'
printf 'private_sha256_state_zeroize_present=1\n'
printf 'internal_state_lifetime_source_cleanup_complete=1\n'
printf 'release_workspace_zeroize_calls=%s\n' "$workspace_zeroize_calls"
printf 'release_u32_zeroize_calls=%s\n' "$u32_zeroize_calls"
printf 'release_digest_output_written_before_internal_cleanup=1\n'
printf 'release_workspace_zeroize_stack_offset=360\n'
printf 'release_schedule_zeroize_stack_offset=96\n'
printf 'release_state_zeroize_stack_offset=40\n'
printf 'release_post_cleanup_mov_count=0\n'
printf 'entropy_private_sha256_calls=%s\n' "$entropy_private_sha256_calls"
printf 'secret_private_sha256_calls=%s\n' "$secret_private_sha256_calls"
printf 'release_allocator_calls=0\n'
printf 'release_sha_ni_dispatch_markers=0\n'
printf 'private_sha256_source_sha256=%s\n' "$private_source_sha"
printf 'private_sha256_assembly_sha256=%s\n' "$private_asm_sha"
printf 'release_internal_stack_lifetime_claim_complete=1\n'
printf 'release_residual_register_lifetime_claim_complete=0\n'
printf 'constant_time_claim_complete=0\n'
