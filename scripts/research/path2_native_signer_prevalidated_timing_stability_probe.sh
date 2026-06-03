#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
iterations="${PATH2_NATIVE_SIGNER_TIMING_PROBE_ITERS:-256}"
trials="${PATH2_NATIVE_SIGNER_TIMING_STABILITY_TRIALS:-5}"

if ! [[ "$iterations" =~ ^[0-9]+$ ]] || [[ "$iterations" -lt 1 ]]; then
  echo "PATH2_NATIVE_SIGNER_TIMING_PROBE_ITERS must be a positive integer" >&2
  exit 1
fi

if ! [[ "$trials" =~ ^[0-9]+$ ]] || [[ "$trials" -lt 1 ]]; then
  echo "PATH2_NATIVE_SIGNER_TIMING_STABILITY_TRIALS must be a positive integer" >&2
  exit 1
fi

tmp_json="$(mktemp)"
trap 'rm -f "$tmp_json"' EXIT

for trial in $(seq 1 "$trials"); do
  output="$(
    PATH2_NATIVE_SIGNER_TIMING_PROBE_ITERS="$iterations" \
      "$repo_root/scripts/research/path2_native_signer_prevalidated_timing_probe.sh"
  )"
  json_line="$(grep '^probe_json=' <<<"$output" | tail -n 1 || true)"
  if [[ -z "$json_line" ]]; then
    echo "$output" >&2
    echo "prevalidated timing probe JSON missing for trial $trial" >&2
    exit 1
  fi
  printf '%s\n' "${json_line#probe_json=}" >>"$tmp_json"
done

python3 - "$tmp_json" "$iterations" "$trials" <<'PY'
import hashlib
import json
import sys

path, iterations, trials = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
rows = []
with open(path, "r", encoding="utf-8") as handle:
    for line in handle:
        rows.append(json.loads(line))

if len(rows) != trials:
    raise SystemExit(f"expected {trials} probe rows, got {len(rows)}")

schema = "Descender.Path2.NativeSignerPrevalidatedTimingProbe.v1"
ratios = []
first_avgs = []
second_avgs = []
for index, row in enumerate(rows, start=1):
    if row.get("schema") != schema:
        raise SystemExit(f"trial {index} schema mismatch")
    if row.get("iterations") != iterations:
        raise SystemExit(f"trial {index} iteration mismatch")
    if row.get("public_reports_sanitized") is not True:
        raise SystemExit(f"trial {index} report sanitization failed")
    if row.get("timing_claim_complete") is not False:
        raise SystemExit(f"trial {index} must not claim complete timing audit")
    ratios.append(int(row["max_to_min_avg_ratio_micros"]))
    first_avgs.append(int(row["first_valid_avg_ns"]))
    second_avgs.append(int(row["second_valid_avg_ns"]))

payload = {
    "schema": "Descender.Path2.NativeSignerPrevalidatedTimingStabilityProbe.v1",
    "trials": trials,
    "iterations_per_trial": iterations,
    "max_ratio_micros": max(ratios),
    "min_ratio_micros": min(ratios),
    "mean_ratio_micros": sum(ratios) // len(ratios),
    "first_valid_avg_ns_min": min(first_avgs),
    "first_valid_avg_ns_max": max(first_avgs),
    "second_valid_avg_ns_min": min(second_avgs),
    "second_valid_avg_ns_max": max(second_avgs),
    "public_reports_sanitized": True,
    "timing_claim_complete": False,
    "explicit_non_claims": [
        "stability probe is wall-clock smoke evidence only",
        "stability probe is not microarchitectural leakage proof",
        "stability probe does not complete signer constant-time audit",
    ],
}
encoded = json.dumps(payload, sort_keys=True, separators=(",", ":")).encode()
payload["payload_sha256"] = hashlib.sha256(encoded).hexdigest()

print("schema=Descender.Path2.NativeSignerPrevalidatedTimingStabilityHarness.v1")
print(f"trials={trials}")
print(f"iterations_per_trial={iterations}")
print(f"max_ratio_micros={payload['max_ratio_micros']}")
print(f"mean_ratio_micros={payload['mean_ratio_micros']}")
print(f"first_valid_avg_ns_min={payload['first_valid_avg_ns_min']}")
print(f"first_valid_avg_ns_max={payload['first_valid_avg_ns_max']}")
print(f"second_valid_avg_ns_min={payload['second_valid_avg_ns_min']}")
print(f"second_valid_avg_ns_max={payload['second_valid_avg_ns_max']}")
print(f"payload_sha256={payload['payload_sha256']}")
print("stability_json=" + json.dumps(payload, sort_keys=True, separators=(",", ":")))
PY
