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

private_sha_body="$(extract_function 'path2_native_private_sha256_digest_v1')"
entropy_body="$(extract_function 'path2_native_signer_entropy_digest_v1')"
secret_body="$(extract_function 'path2_native_digest_secret_component_v1')"

if [[ -z "$private_sha_body" || -z "$entropy_body" || -z "$secret_body" ]]; then
  echo "required signer digest symbols missing from release assembly" >&2
  exit 1
fi

line_no() {
  local pattern="$1"
  local body="$2"
  grep -nE "$pattern" <<<"$body" | head -n 1 | cut -d: -f1 || true
}

tail_after_last() {
  local pattern="$1"
  local body="$2"
  local line
  line="$(grep -nE "$pattern" <<<"$body" | tail -n 1 | cut -d: -f1 || true)"
  if [[ -n "$line" ]]; then
    tail -n "+$((line + 1))" <<<"$body"
  fi
}

success_epilogue_after_last() {
  local pattern="$1"
  local body="$2"
  local line
  line="$(grep -nE "$pattern" <<<"$body" | tail -n 1 | cut -d: -f1 || true)"
  if [[ -n "$line" ]]; then
    tail -n "+$((line + 1))" <<<"$body" | awk '{ print } /^[[:space:]]*retq$/ { exit }'
  fi
}

private_first_output="$(line_no 'movl[[:space:]]+%r?15d,[[:space:]]*\(%rax\)' "$private_sha_body")"
private_first_zeroize="$(line_no 'callq.*path2_native_zeroize_bytes_v1' "$private_sha_body")"
private_post_cleanup_tail="$(success_epilogue_after_last 'callq.*path2_native_zeroize_u32s_v1' "$private_sha_body")"
private_post_cleanup_movs="$(grep -Ec '^[[:space:]]*mov' <<<"$private_post_cleanup_tail" || true)"

if [[ -z "$private_first_output" || -z "$private_first_zeroize" || "$private_first_output" -ge "$private_first_zeroize" ]]; then
  echo "private SHA output is not stored before private SHA cleanup" >&2
  exit 1
fi

if [[ "$private_post_cleanup_movs" -ne 0 ]]; then
  echo "private SHA callee has mov instructions after final cleanup call" >&2
  exit 1
fi

check_wrapper() {
  local name="$1"
  local body="$2"
  local expected_byte_zeroizers="$3"
  local first_output_store
  local first_frame_zeroize
  local final_byte_zeroize
  local post_cleanup_tail
  local post_cleanup_movs
  local xmm_digest_copy_count

  first_output_store="$(line_no 'movups[[:space:]]+%xmm[01],[[:space:]]*(16)?\(%r(ax|14)\)' "$body")"
  first_frame_zeroize="$(line_no 'callq.*path2_native_zeroize_signer_digest_frame_v1' "$body")"
  final_byte_zeroize="$(grep -nE 'callq.*path2_native_zeroize_bytes_v1' <<<"$body" | tail -n 1 | cut -d: -f1 || true)"
  post_cleanup_tail="$(success_epilogue_after_last 'callq.*path2_native_zeroize_bytes_v1' "$body")"
  post_cleanup_movs="$(grep -Ec '^[[:space:]]*mov' <<<"$post_cleanup_tail" || true)"
  xmm_digest_copy_count="$(grep -Ec '^[[:space:]]*movaps[[:space:]]+[0-9]+[(]%rsp[)],[[:space:]]*%xmm[01]' <<<"$body" || true)"
  byte_zeroizers="$(grep -Ec 'callq.*path2_native_zeroize_bytes_v1' <<<"$body" || true)"

  if [[ -z "$first_output_store" || -z "$first_frame_zeroize" || "$first_output_store" -ge "$first_frame_zeroize" ]]; then
    echo "$name does not copy digest output before frame cleanup" >&2
    exit 1
  fi

  if [[ -z "$final_byte_zeroize" || "$final_byte_zeroize" -le "$first_output_store" ]]; then
    echo "$name does not zeroize local digest after output copy" >&2
    exit 1
  fi

  if [[ "$post_cleanup_movs" -ne 0 ]]; then
    echo "$name has mov instructions after final byte cleanup" >&2
    exit 1
  fi

  if [[ "$xmm_digest_copy_count" -ne 2 ]]; then
    echo "$name does not expose the expected two XMM digest-copy loads" >&2
    exit 1
  fi

  if [[ "$byte_zeroizers" -ne "$expected_byte_zeroizers" ]]; then
    echo "$name byte-zeroizer count mismatch: expected $expected_byte_zeroizers found $byte_zeroizers" >&2
    exit 1
  fi
}

check_wrapper "entropy digest wrapper" "$entropy_body" 1
check_wrapper "secret digest wrapper" "$secret_body" 2

private_sha="$(printf '%s' "$private_sha_body" | sha256sum | awk '{print $1}')"
entropy_sha="$(printf '%s' "$entropy_body" | sha256sum | awk '{print $1}')"
secret_sha="$(printf '%s' "$secret_body" | sha256sum | awk '{print $1}')"

printf 'schema=Descender.Path2.NativeSignerRegisterResidualAudit.v1\n'
printf 'asm_file=%s\n' "$asm_file"
printf 'private_sha_output_before_cleanup=1\n'
printf 'private_sha_post_cleanup_mov_count=0\n'
printf 'entropy_wrapper_output_before_cleanup=1\n'
printf 'entropy_wrapper_local_digest_zeroized_after_output_copy=1\n'
printf 'entropy_wrapper_post_cleanup_mov_count=0\n'
printf 'secret_wrapper_output_before_cleanup=1\n'
printf 'secret_wrapper_seed_and_local_digest_zeroized=1\n'
printf 'secret_wrapper_post_cleanup_mov_count=0\n'
printf 'wrapper_xmm_digest_copy_registers_observed=1\n'
printf 'general_register_scrub_complete=0\n'
printf 'simd_register_scrub_complete=0\n'
printf 'register_residual_lifetime_claim_complete=0\n'
printf 'private_sha_assembly_sha256=%s\n' "$private_sha"
printf 'entropy_wrapper_assembly_sha256=%s\n' "$entropy_sha"
printf 'secret_wrapper_assembly_sha256=%s\n' "$secret_sha"
printf 'constant_time_claim_complete=0\n'
