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

function_body="$(
  awk '
    /^_ZN.*path2_native_signer_entropy_q_v1.*:$/ { in_body = 1 }
    in_body { print }
    in_body && /^\.Lfunc_end/ { exit }
  ' "$asm_file"
)"

if [[ -z "$function_body" ]]; then
  echo "path2_native_signer_entropy_q_v1 symbol missing from release assembly" >&2
  exit 1
fi

digest_call_count="$(grep -c 'callq.*path2_native_signer_entropy_digest_v1' <<<"$function_body" || true)"
if [[ "$digest_call_count" -ne 4 ]]; then
  echo "expected 4 fixed digest calls, found $digest_call_count" >&2
  exit 1
fi

if grep -Eq '^[[:space:]]*(idiv|div)[bwlq]?[[:space:]]' <<<"$function_body"; then
  echo "division instruction found in signer q sampler assembly" >&2
  exit 1
fi

if grep -Eq '^[[:space:]]*j[a-z]+[[:space:]]' <<<"$function_body"; then
  echo "conditional branch found in signer q sampler assembly" >&2
  exit 1
fi

if ! grep -Eq '^[[:space:]]*cmov' <<<"$function_body"; then
  echo "conditional move lowering not found in signer q sampler assembly" >&2
  exit 1
fi

if ! grep -q 'movzbl' <<<"$function_body"; then
  echo "fixed digest byte loads not found in signer q sampler assembly" >&2
  exit 1
fi

sha="$(printf '%s' "$function_body" | sha256sum | awk '{print $1}')"
printf 'schema=Descender.Path2.NativeSignerQSamplerBackendAudit.v1\n'
printf 'asm_file=%s\n' "$asm_file"
printf 'function=path2_native_signer_entropy_q_v1\n'
printf 'digest_call_count=%s\n' "$digest_call_count"
printf 'division_instructions=0\n'
printf 'conditional_branches=0\n'
printf 'conditional_move_present=1\n'
printf 'fixed_digest_byte_loads_present=1\n'
printf 'function_assembly_sha256=%s\n' "$sha"
