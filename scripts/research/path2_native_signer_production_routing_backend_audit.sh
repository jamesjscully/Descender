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

public_body="$(extract_function 'path2_native_signer_execution_report_from_public_input_os_entropy_v1')"
prevalidated_os_body="$(extract_function 'path2_native_signer_execution_report_from_prevalidated_os_entropy_v1')"

if [[ -z "$public_body" ]]; then
  echo "public-input production signer entrypoint symbol missing from release assembly" >&2
  exit 1
fi

if [[ -z "$prevalidated_os_body" ]]; then
  echo "prevalidated OS signer entrypoint symbol missing from release assembly" >&2
  exit 1
fi

prevalidation_line="$(grep -n 'callq.*path2_native_signer_prevalidated_public_input_v1' <<<"$public_body" | head -n 1 | cut -d: -f1 || true)"
prevalidated_os_line="$(grep -n 'callq.*path2_native_signer_execution_report_from_prevalidated_os_entropy_v1' <<<"$public_body" | head -n 1 | cut -d: -f1 || true)"
public_entropy_calls="$(grep -c 'callq.*path2_native_signer_entropy_root_from_os_v1' <<<"$public_body" || true)"
prevalidated_os_entropy_calls="$(grep -c 'callq.*path2_native_signer_entropy_root_from_os_v1' <<<"$prevalidated_os_body" || true)"

if [[ -z "$prevalidation_line" ]]; then
  echo "public-input entrypoint does not call public prevalidation in release assembly" >&2
  exit 1
fi

if [[ -z "$prevalidated_os_line" ]]; then
  echo "public-input entrypoint does not route to prevalidated OS entrypoint in release assembly" >&2
  exit 1
fi

if (( prevalidation_line >= prevalidated_os_line )); then
  echo "public-input entrypoint routes to OS/private path before public prevalidation" >&2
  exit 1
fi

if [[ "$public_entropy_calls" -ne 0 ]]; then
  echo "public-input entrypoint calls OS entropy directly before/around prevalidation" >&2
  exit 1
fi

if [[ "$prevalidated_os_entropy_calls" -ne 1 ]]; then
  echo "expected exactly one OS entropy call in prevalidated OS entrypoint, found $prevalidated_os_entropy_calls" >&2
  exit 1
fi

public_sha="$(printf '%s' "$public_body" | sha256sum | awk '{print $1}')"
prevalidated_os_sha="$(printf '%s' "$prevalidated_os_body" | sha256sum | awk '{print $1}')"

printf 'schema=Descender.Path2.NativeSignerProductionRoutingBackendAudit.v1\n'
printf 'asm_file=%s\n' "$asm_file"
printf 'public_entrypoint=path2_native_signer_execution_report_from_public_input_os_entropy_v1\n'
printf 'public_prevalidation_call_line=%s\n' "$prevalidation_line"
printf 'public_prevalidated_os_call_line=%s\n' "$prevalidated_os_line"
printf 'public_direct_entropy_calls=%s\n' "$public_entropy_calls"
printf 'prevalidated_os_entrypoint=path2_native_signer_execution_report_from_prevalidated_os_entropy_v1\n'
printf 'prevalidated_os_entropy_calls=%s\n' "$prevalidated_os_entropy_calls"
printf 'public_entrypoint_assembly_sha256=%s\n' "$public_sha"
printf 'prevalidated_os_entrypoint_assembly_sha256=%s\n' "$prevalidated_os_sha"
