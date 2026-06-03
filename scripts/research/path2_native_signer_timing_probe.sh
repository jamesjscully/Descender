#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
crate_dir="$repo_root/tools/parameter-search"
iterations="${PATH2_NATIVE_SIGNER_TIMING_PROBE_ITERS:-256}"

cd "$crate_dir"
output="$(
  PATH2_NATIVE_SIGNER_TIMING_PROBE_ITERS="$iterations" \
    cargo test --release path2_native_signer_timing_probe_emits_sanitized_release_evidence \
    -- --nocapture
)"

probe_line="$(grep '^path2_native_signer_timing_probe_v1=' <<<"$output" | tail -n 1 || true)"
if [[ -z "$probe_line" ]]; then
  echo "$output" >&2
  echo "timing probe JSON line missing" >&2
  exit 1
fi

json="${probe_line#path2_native_signer_timing_probe_v1=}"
if ! grep -q '"schema":"Descender.Path2.NativeSignerTimingProbe.v1"' <<<"$json"; then
  echo "timing probe schema mismatch" >&2
  exit 1
fi

if ! grep -q '"public_reports_sanitized":true' <<<"$json"; then
  echo "timing probe reports were not sanitized" >&2
  exit 1
fi

if ! grep -q '"timing_claim_complete":false' <<<"$json"; then
  echo "timing probe must not claim complete timing audit" >&2
  exit 1
fi

printf 'schema=Descender.Path2.NativeSignerTimingProbeHarness.v1\n'
printf 'iterations=%s\n' "$iterations"
printf 'probe_json=%s\n' "$json"
