#!/usr/bin/env bash
set -euo pipefail

campaign_root="${PATH2_NATIVE_SIGNER_TARGET_HOST_CAMPAIGN_DIR:-}"
min_iterations="${PATH2_NATIVE_SIGNER_PROMOTION_MIN_TIMING_ITERS:-256}"
min_trials="${PATH2_NATIVE_SIGNER_PROMOTION_MIN_TIMING_TRIALS:-5}"

if [[ -z "$campaign_root" ]]; then
  echo "PATH2_NATIVE_SIGNER_TARGET_HOST_CAMPAIGN_DIR must name an archived campaign" >&2
  exit 1
fi

campaign_manifest="$campaign_root/target_host_timing_campaign.manifest"
bundle_dir="$campaign_root/ct_bundle"
bundle_manifest="$bundle_dir/ct_evidence_bundle.manifest"
bundle_payload="$bundle_dir/ct_evidence_bundle.payload"
environment_manifest="$bundle_dir/ct_evidence_bundle.environment"
timing_output="$bundle_dir/path2_native_signer_prevalidated_timing_stability_probe.sh.out"
register_policy_output="$campaign_root/register_scrub_policy.out"

for required in \
  "$campaign_manifest" \
  "$bundle_manifest" \
  "$bundle_payload" \
  "$environment_manifest" \
  "$timing_output" \
  "$register_policy_output"
do
  if [[ ! -s "$required" ]]; then
    echo "archived campaign artifact missing: $required" >&2
    exit 1
  fi
done

manifest_value() {
  local key="$1"
  grep -E "^${key}=" "$campaign_manifest" | tail -n 1 | cut -d= -f2-
}

iterations="$(manifest_value iterations_per_trial)"
trials="$(manifest_value timing_stability_trials)"

if [[ -z "$iterations" || -z "$trials" ]]; then
  echo "campaign manifest missing timing dimensions" >&2
  exit 1
fi

if (( iterations < min_iterations )); then
  echo "campaign timing iterations below promotion floor: $iterations < $min_iterations" >&2
  exit 1
fi

if (( trials < min_trials )); then
  echo "campaign timing trials below promotion floor: $trials < $min_trials" >&2
  exit 1
fi

check_manifest_sha() {
  local key="$1"
  local path="$2"
  local expected actual
  expected="$(manifest_value "$key")"
  actual="$(sha256sum "$path" | awk '{print $1}')"
  if [[ -z "$expected" || "$expected" != "$actual" ]]; then
    echo "campaign manifest hash mismatch for $key" >&2
    exit 1
  fi
  printf '%s' "$actual"
}

bundle_manifest_sha="$(check_manifest_sha bundle_manifest_sha256 "$bundle_manifest")"
bundle_payload_sha="$(check_manifest_sha bundle_payload_sha256 "$bundle_payload")"
environment_sha="$(check_manifest_sha environment_manifest_sha256 "$environment_manifest")"
timing_sha="$(check_manifest_sha timing_stability_output_sha256 "$timing_output")"
register_policy_sha="$(check_manifest_sha register_scrub_policy_output_sha256 "$register_policy_output")"

for marker in \
  'target_host_bundle_archived=1' \
  'register_scrub_policy_complete=0' \
  'microarchitectural_timing_claim_complete=0' \
  'constant_time_claim_complete=0'
do
  if ! grep -Fq "$marker" "$campaign_manifest"; then
    echo "campaign manifest marker missing: $marker" >&2
    exit 1
  fi
done

if ! grep -Fq 'register_scrub_policy_complete=0' "$register_policy_output"; then
  echo "register policy output must keep register scrub open" >&2
  exit 1
fi

if ! grep -Fq 'timing_claim_complete":false' "$timing_output"; then
  echo "timing output must keep timing claim open" >&2
  exit 1
fi

printf 'schema=Descender.Path2.NativeSignerTargetHostCampaignPromotionGate.v1\n'
printf 'campaign_dir=%s\n' "$campaign_root"
printf 'min_iterations_per_trial=%s\n' "$min_iterations"
printf 'min_timing_stability_trials=%s\n' "$min_trials"
printf 'observed_iterations_per_trial=%s\n' "$iterations"
printf 'observed_timing_stability_trials=%s\n' "$trials"
printf 'bundle_manifest_sha256=%s\n' "$bundle_manifest_sha"
printf 'bundle_payload_sha256=%s\n' "$bundle_payload_sha"
printf 'environment_manifest_sha256=%s\n' "$environment_sha"
printf 'timing_stability_output_sha256=%s\n' "$timing_sha"
printf 'register_scrub_policy_output_sha256=%s\n' "$register_policy_sha"
printf 'target_host_campaign_thresholds_met=1\n'
printf 'target_host_artifact_hashes_verified=1\n'
printf 'register_scrub_policy_complete=0\n'
printf 'microarchitectural_timing_claim_complete=0\n'
printf 'release_promotion_claim_complete=0\n'
printf 'constant_time_claim_complete=0\n'
