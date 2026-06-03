#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
crate_dir="$repo_root/tools/parameter-search"
source_file="$crate_dir/src/production_reference.rs"

cd "$crate_dir"

private_source="$(
  awk '
    index($0, "fn path2_native_private_sha256_digest_v1(") { in_body = 1; depth = 0 }
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
)"

if [[ -z "$private_source" ]]; then
  echo "path2_native_private_sha256_digest_v1 source missing" >&2
  exit 1
fi

for marker in \
  'assert!' \
  'MaybeUninit::<[u8; PATH2_NATIVE_SIGNER_PRIVATE_SHA256_WORKSPACE_BYTES]>::uninit();' \
  'ptr::write_volatile(workspace_ptr.add(idx), 0);' \
  'ptr::read_volatile(data.as_ptr().add(idx))' \
  'for block in workspace[..padded_len].chunks_exact(64)' \
  'for (idx, word) in schedule.iter_mut().take(16).enumerate()' \
  'for idx in 16..64' \
  'for idx in 0..64' \
  'path2_native_zeroize_bytes_v1(workspace);' \
  'path2_native_zeroize_u32s_v1(&mut schedule);' \
  'path2_native_zeroize_u32s_v1(&mut state);'
do
  if ! grep -Fq "$marker" <<<"$private_source"; then
    echo "private SHA-256 source marker missing: $marker" >&2
    exit 1
  fi
done

if grep -Eq 'if[[:space:]]|match[[:space:]]|while[[:space:]]' <<<"$private_source"; then
  echo "unexpected source-level branch form in private SHA-256 body" >&2
  exit 1
fi

cargo rustc --release --lib -- --emit=asm >/dev/null

asm_file="$(find target/release/deps -maxdepth 1 -name 'descender_parameter_search-*.s' | sort | tail -n 1)"
if [[ -z "$asm_file" ]]; then
  echo "missing release assembly output" >&2
  exit 1
fi

private_body="$(
  awk -v symbol='path2_native_private_sha256_digest_v1' '
    $0 ~ "^_ZN.*" symbol ".*:$" { in_body = 1 }
    in_body { print }
    in_body && /^\.Lfunc_end/ { exit }
  ' "$asm_file"
)"

if [[ -z "$private_body" ]]; then
  echo "path2_native_private_sha256_digest_v1 symbol missing from release assembly" >&2
  exit 1
fi

conditional_branches="$(
  grep -E '^[[:space:]]*j[a-z]+[[:space:]]' <<<"$private_body" \
    | grep -Evc '^[[:space:]]*jmp[[:space:]]' || true
)"
unconditional_jumps="$(grep -Ec '^[[:space:]]*jmp[[:space:]]' <<<"$private_body" || true)"
calls="$(grep -Ec '^[[:space:]]*callq?[[:space:]]' <<<"$private_body" || true)"
cmovs="$(grep -Ec '^[[:space:]]*cmov' <<<"$private_body" || true)"
sets="$(grep -Ec '^[[:space:]]*set[a-z]+' <<<"$private_body" || true)"
variable_shifts="$(grep -Ec '(rol|ror|shl|shr|sal|sar)[[:space:]]+%cl' <<<"$private_body" || true)"
memroutine_calls="$(grep -Ec 'callq?[[:space:]].*(memset|memcpy)' <<<"$private_body" || true)"
zeroize_calls="$(grep -Ec 'callq?[[:space:]].*path2_native_zeroize_(bytes|u32s)_v1' <<<"$private_body" || true)"

if [[ "$conditional_branches" -ne 9 ]]; then
  echo "unexpected private SHA-256 conditional branch count: $conditional_branches" >&2
  exit 1
fi

if [[ "$unconditional_jumps" -ne 1 ]]; then
  echo "unexpected private SHA-256 unconditional jump count: $unconditional_jumps" >&2
  exit 1
fi

if [[ "$calls" -ne 3 || "$memroutine_calls" -ne 0 || "$zeroize_calls" -ne 3 ]]; then
  echo "unexpected private SHA-256 call schedule: calls=$calls mem=$memroutine_calls zeroize=$zeroize_calls" >&2
  exit 1
fi

if [[ "$cmovs" -ne 0 || "$sets" -ne 0 || "$variable_shifts" -ne 0 ]]; then
  echo "unexpected cmov/setcc/variable shift in private SHA-256 assembly" >&2
  exit 1
fi

if grep -Eq '__rust_alloc|__rust_dealloc|__rust_realloc|exchange_malloc|RawVec|alloc::raw_vec|sha256rnds2|sha256msg1|sha256msg2|cpuid|shani_cpuid|x86_feature_detected|sha2_asm' <<<"$private_body"; then
  echo "unexpected allocator/SHA-NI/CPU-dispatch marker in private SHA-256 assembly" >&2
  exit 1
fi

private_source_sha="$(printf '%s' "$private_source" | sha256sum | awk '{print $1}')"
private_asm_sha="$(printf '%s' "$private_body" | sha256sum | awk '{print $1}')"

printf 'schema=Descender.Path2.NativeSignerPrivateSha256CtShapeAudit.v1\n'
printf 'active_private_sha256_path=1\n'
printf 'source_public_length_assert_present=1\n'
printf 'source_fixed_block_loop_present=1\n'
printf 'source_fixed_schedule_loop_present=1\n'
printf 'source_fixed_round_loop_present=1\n'
printf 'source_unexpected_conditionals=0\n'
printf 'assembly_public_loop_conditional_branches=9\n'
printf 'assembly_unconditional_jumps=1\n'
printf 'assembly_calls=3\n'
printf 'assembly_public_length_memroutine_calls=0\n'
printf 'assembly_zeroize_calls=3\n'
printf 'assembly_cmovs=0\n'
printf 'assembly_setcc=0\n'
printf 'assembly_variable_shifts=0\n'
printf 'assembly_allocator_sha_ni_dispatch_markers=0\n'
printf 'private_sha256_source_sha256=%s\n' "$private_source_sha"
printf 'private_sha256_assembly_sha256=%s\n' "$private_asm_sha"
printf 'constant_time_claim_complete=0\n'
