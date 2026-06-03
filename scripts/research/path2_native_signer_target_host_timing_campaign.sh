#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
campaign_root="${PATH2_NATIVE_SIGNER_TARGET_HOST_CAMPAIGN_DIR:-}"
iterations="${PATH2_NATIVE_SIGNER_TIMING_PROBE_ITERS:-256}"
trials="${PATH2_NATIVE_SIGNER_TIMING_STABILITY_TRIALS:-5}"

if [[ -z "$campaign_root" ]]; then
  campaign_root="$(mktemp -d)"
fi
mkdir -p "$campaign_root"

bundle_dir="$campaign_root/ct_bundle"
campaign_manifest="$campaign_root/target_host_timing_campaign.manifest"
rm -rf "$bundle_dir"
mkdir -p "$bundle_dir"

PATH2_NATIVE_SIGNER_CT_EVIDENCE_OUT_DIR="$bundle_dir" \
PATH2_NATIVE_SIGNER_TIMING_PROBE_ITERS="$iterations" \
PATH2_NATIVE_SIGNER_TIMING_STABILITY_TRIALS="$trials" \
  "$repo_root/scripts/research/path2_native_signer_ct_evidence_bundle.sh" \
  >"$campaign_root/ct_bundle.stdout"

bundle_manifest="$bundle_dir/ct_evidence_bundle.manifest"
bundle_payload="$bundle_dir/ct_evidence_bundle.payload"
environment_manifest="$bundle_dir/ct_evidence_bundle.environment"
timing_output="$bundle_dir/path2_native_signer_prevalidated_timing_stability_probe.sh.out"
register_policy_output="$campaign_root/register_scrub_policy.out"

if [[ ! -s "$bundle_manifest" || ! -s "$bundle_payload" || ! -s "$environment_manifest" ]]; then
  echo "target-host bundle did not preserve expected manifest files" >&2
  exit 1
fi

"$repo_root/scripts/research/path2_native_signer_register_scrub_policy.sh" >"$register_policy_output"

for marker in \
  'constant_time_claim_complete=0' \
  'environment_manifest_sha256=' \
  'audit_count=12'
do
  if ! grep -Fq "$marker" "$bundle_manifest"; then
    echo "target-host bundle manifest marker missing: $marker" >&2
    exit 1
  fi
done

if [[ ! -s "$timing_output" ]]; then
  echo "prevalidated timing stability output missing from target-host campaign" >&2
  exit 1
fi

if ! grep -Fq 'timing_claim_complete":false' "$timing_output"; then
  echo "timing stability output must not claim complete timing audit" >&2
  exit 1
fi

if ! grep -Fq 'register_scrub_policy_complete=0' "$register_policy_output"; then
  echo "register scrub policy must keep register scrub incomplete" >&2
  exit 1
fi

bundle_manifest_sha="$(sha256sum "$bundle_manifest" | awk '{print $1}')"
bundle_payload_sha="$(sha256sum "$bundle_payload" | awk '{print $1}')"
environment_sha="$(sha256sum "$environment_manifest" | awk '{print $1}')"
timing_sha="$(sha256sum "$timing_output" | awk '{print $1}')"
register_policy_sha="$(sha256sum "$register_policy_output" | awk '{print $1}')"

{
  printf 'schema=Descender.Path2.NativeSignerTargetHostTimingCampaign.v1\n'
  printf 'campaign_dir=%s\n' "$campaign_root"
  printf 'bundle_dir=%s\n' "$bundle_dir"
  printf 'iterations_per_trial=%s\n' "$iterations"
  printf 'timing_stability_trials=%s\n' "$trials"
  printf 'bundle_manifest_sha256=%s\n' "$bundle_manifest_sha"
  printf 'bundle_payload_sha256=%s\n' "$bundle_payload_sha"
  printf 'environment_manifest_sha256=%s\n' "$environment_sha"
  printf 'timing_stability_output_sha256=%s\n' "$timing_sha"
  printf 'register_scrub_policy_output_sha256=%s\n' "$register_policy_sha"
  printf 'target_host_bundle_archived=1\n'
  printf 'register_scrub_policy_complete=0\n'
  printf 'microarchitectural_timing_claim_complete=0\n'
  printf 'constant_time_claim_complete=0\n'
} >"$campaign_manifest"

cat "$campaign_manifest"
