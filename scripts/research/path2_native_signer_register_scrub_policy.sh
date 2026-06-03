#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

residual_output="$("$repo_root/scripts/research/path2_native_signer_register_residual_audit.sh")"

for marker in \
  'schema=Descender.Path2.NativeSignerRegisterResidualAudit.v1' \
  'private_sha_post_cleanup_mov_count=0' \
  'entropy_wrapper_post_cleanup_mov_count=0' \
  'secret_wrapper_post_cleanup_mov_count=0' \
  'general_register_scrub_complete=0' \
  'simd_register_scrub_complete=0' \
  'register_residual_lifetime_claim_complete=0'
do
  if ! grep -Fq "$marker" <<<"$residual_output"; then
    echo "register residual audit marker missing: $marker" >&2
    exit 1
  fi
done

residual_sha="$(printf '%s' "$residual_output" | sha256sum | awk '{print $1}')"

printf 'schema=Descender.Path2.NativeSignerRegisterScrubPolicy.v1\n'
printf 'residual_audit_schema=Descender.Path2.NativeSignerRegisterResidualAudit.v1\n'
printf 'residual_audit_output_sha256=%s\n' "$residual_sha"
printf 'internal_stack_lifetime_closed=1\n'
printf 'digest_wrapper_stack_lifetime_closed=1\n'
printf 'general_register_scrub_complete=0\n'
printf 'simd_register_scrub_complete=0\n'
printf 'current_release_uses_abi_epilogue_restore=1\n'
printf 'target_toolchain_policy_required=1\n'
printf 'accepted_closure_option_1=audited_target_assembly_epilogue_scrubs_gpr_and_simd_before_return\n'
printf 'accepted_closure_option_2=audited_compiler_register_zeroing_flag_with_release_assembly_evidence\n'
printf 'accepted_closure_option_3=deployment_process_isolation_policy_with_no_cross_secret_reuse_claim\n'
printf 'rejected_closure=source_level_zeroize_alone_for_registers\n'
printf 'register_scrub_policy_complete=0\n'
printf 'constant_time_claim_complete=0\n'
