#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
out_dir="${PATH2_NATIVE_SIGNER_CT_EVIDENCE_OUT_DIR:-}"
if [[ -n "$out_dir" ]]; then
  work_dir="$out_dir"
  mkdir -p "$work_dir"
else
  work_dir="$(mktemp -d)"
  trap 'rm -rf "$work_dir"' EXIT
fi

audits=(
  "path2_native_signer_q_sampler_backend_audit.sh"
  "path2_native_signer_zeroize_backend_audit.sh"
  "path2_native_signer_digest_frame_backend_audit.sh"
  "path2_native_signer_sha256_backend_dependency_audit.sh"
  "path2_native_signer_sha256_source_backend_audit.sh"
  "path2_native_signer_sha256_release_backend_audit.sh"
  "path2_native_signer_sha256_soft_ct_shape_audit.sh"
  "path2_native_signer_private_sha256_ct_shape_audit.sh"
  "path2_native_signer_sha256_internal_state_lifetime_audit.sh"
  "path2_native_signer_register_residual_audit.sh"
  "path2_native_signer_production_routing_backend_audit.sh"
  "path2_native_signer_prevalidated_timing_stability_probe.sh"
)

manifest="$work_dir/ct_evidence_bundle.manifest"
bundle_payload="$work_dir/ct_evidence_bundle.payload"
environment_manifest="$work_dir/ct_evidence_bundle.environment"
: >"$manifest"
: >"$bundle_payload"
: >"$environment_manifest"

emit() {
  printf '%s\n' "$1" | tee -a "$manifest"
}

emit 'schema=Descender.Path2.NativeSignerCtEvidenceBundle.v1'
emit "audit_count=${#audits[@]}"
if [[ -n "$out_dir" ]]; then
  emit "evidence_output_dir=$work_dir"
fi

{
  printf 'schema=Descender.Path2.NativeSignerCtEvidenceEnvironment.v1\n'
  printf 'repo_root=%s\n' "$repo_root"
  printf 'uname_s=%s\n' "$(uname -s)"
  printf 'uname_m=%s\n' "$(uname -m)"
  printf 'uname_r=%s\n' "$(uname -r)"
  printf 'rustc_version=%s\n' "$(rustc --version)"
  printf 'cargo_version=%s\n' "$(cargo --version)"
  printf 'rustc_host=%s\n' "$(rustc -vV | awk -F': ' '/^host:/ { print $2; exit }')"
  printf 'rustc_commit_hash=%s\n' "$(rustc -vV | awk -F': ' '/^commit-hash:/ { print $2; exit }')"
  if [[ -r /proc/cpuinfo ]]; then
    printf 'cpu_model_name=%s\n' "$(awk -F': ' '/model name/ { print $2; exit }' /proc/cpuinfo)"
    printf 'cpu_flags_sha256=%s\n' "$(awk -F': ' '/flags/ { print $2; exit }' /proc/cpuinfo | sha256sum | awk '{print $1}')"
  else
    printf 'cpu_model_name=unavailable\n'
    printf 'cpu_flags_sha256=unavailable\n'
  fi
  printf 'audit_count=%s\n' "${#audits[@]}"
  printf 'timing_probe_iters=%s\n' "${PATH2_NATIVE_SIGNER_TIMING_PROBE_ITERS:-256}"
  printf 'timing_stability_trials=%s\n' "${PATH2_NATIVE_SIGNER_TIMING_STABILITY_TRIALS:-5}"
  printf 'target_host_execution_claim_complete=0\n'
} >"$environment_manifest"
environment_sha="$(sha256sum "$environment_manifest" | awk '{print $1}')"
{
  printf 'environment_manifest_sha256=%s\n' "$environment_sha"
} >>"$bundle_payload"
emit "environment_manifest_sha256=$environment_sha"
if [[ -n "$out_dir" ]]; then
  emit "environment_manifest_path=$environment_manifest"
fi

for audit in "${audits[@]}"; do
  script="$repo_root/scripts/research/$audit"
  output="$work_dir/$audit.out"
  if [[ ! -x "$script" ]]; then
    echo "audit script is missing or not executable: $audit" >&2
    exit 1
  fi
  if ! "$script" >"$output" 2>&1; then
    cat "$output" >&2
    echo "audit script failed: $audit" >&2
    exit 1
  fi
  output_sha="$(sha256sum "$output" | awk '{print $1}')"
  {
    printf 'audit_script=%s\n' "$audit"
    printf 'audit_output_sha256=%s\n' "$output_sha"
  } >>"$bundle_payload"
  emit "audit_script=$audit"
  if [[ -n "$out_dir" ]]; then
    emit "audit_output_path=$output"
  fi
  emit "audit_output_sha256=$output_sha"
done

bundle_sha="$(sha256sum "$bundle_payload" | awk '{print $1}')"
emit 'constant_time_claim_complete=0'
emit "bundle_payload_sha256=$bundle_sha"
if [[ -n "$out_dir" ]]; then
  emit "manifest_path=$manifest"
fi
