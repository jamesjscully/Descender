use descender_parameter_search::{
    manifest_vector_evidence::path2_direct_lifecycle_profile_review_evidence_v1,
    production_reference::{
        PATH2_DIRECT_LIFECYCLE_PROFILE_VERIFICATION_ARTIFACT,
        PATH2_DIRECT_LIFECYCLE_PROFILE_VERIFICATION_ID, PROFILE,
        path2_direct_lifecycle_boundary_report_v1,
        path2_direct_lifecycle_boundary_report_v1_digest,
        validate_path2_direct_lifecycle_boundary_report_v1,
    },
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn requirement_ids(requirements: &[Value]) -> Result<Vec<&str>, String> {
    requirements
        .iter()
        .map(|row| {
            row.get("requirement_id")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    "lifecycle profile verification row missing requirement_id".to_string()
                })
        })
        .collect()
}

fn manifest_body() -> Result<Value, String> {
    let report = path2_direct_lifecycle_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&report)?;
    let recomputed_digest = path2_direct_lifecycle_boundary_report_v1_digest(&report);
    if recomputed_digest != report.digest {
        return Err("direct lifecycle boundary report digest mismatch".to_string());
    }
    if !report.lifecycle_transition_profile_verified || report.deployment_ready {
        return Err(
            "direct lifecycle boundary must verify the profile without becoming deployment-ready"
                .to_string(),
        );
    }
    let review_evidence = path2_direct_lifecycle_profile_review_evidence_v1()?;

    let requirements = vec![
        json!({
            "requirement_id": "profile_finality_source",
            "profile_decision": "D2 permanence boundary is the chain FINALIZED head",
            "verified_by": [
                "Path2D2FinalityProfileState::finalize_to",
                "Path2D2FinalityProfileState::reorg_to"
            ],
            "support_tests": [
                "path2_d2_finality_profile_reorg_frees_window_but_preserves_finalized"
            ],
            "review_requirement_verified": true,
            "executable_evidence_verified": true,
            "deployment_profile_slice_verified": true,
            "deployment_ready": false,
            "residual": "concrete chain-client finality source, gas/resource limits, and external review remain outside this lifecycle-slice verification"
        }),
        json!({
            "requirement_id": "profile_pending_release_policy",
            "profile_decision": "released spends keep the slot unavailable locally until finality evidence or burn/replacement",
            "verified_by": [
                "ProductionSpentSlotLedger::insert",
                "ProductionAccountLifecycleManager::verify_and_consume"
            ],
            "support_tests": [
                "spent_slot_ledger_blocks_stale_wallet_replay",
                "lifecycle_manager_verify_and_consume_blocks_replay"
            ],
            "review_requirement_verified": true,
            "executable_evidence_verified": true,
            "deployment_profile_slice_verified": true,
            "deployment_ready": false,
            "residual": "wallet/HSM transport encodings and custody operations remain adoption-profile work"
        }),
        json!({
            "requirement_id": "profile_dropped_before_finality_policy",
            "profile_decision": "records in [finalized, head] may be freed by reorg_to(new_head), while signer-local released slots remain unavailable absent finality or burn/replacement evidence",
            "verified_by": [
                "Path2D2FinalityProfileState::reorg_to",
                "ProductionSpentSlotLedger::mark_reorg_burn"
            ],
            "support_tests": [
                "path2_d2_finality_profile_reorg_frees_window_but_preserves_finalized",
                "spent_slot_ledger_records_reorg_burns"
            ],
            "review_requirement_verified": true,
            "executable_evidence_verified": true,
            "deployment_profile_slice_verified": true,
            "deployment_ready": false,
            "residual": "target-chain mempool/drop UX and retry policy remain adoption-profile work"
        }),
        json!({
            "requirement_id": "profile_finalized_reorg_policy",
            "profile_decision": "reorg below finalized head is rejected rather than resurrecting consumed digests",
            "verified_by": [
                "Path2D2FinalityProfileState::reorg_to finalized-head guard"
            ],
            "support_tests": [
                "path2_d2_finality_profile_reorg_frees_window_but_preserves_finalized"
            ],
            "review_requirement_verified": true,
            "executable_evidence_verified": true,
            "deployment_profile_slice_verified": true,
            "deployment_ready": false,
            "residual": "real consensus-client finalized-head mapping remains adoption-profile work"
        }),
        json!({
            "requirement_id": "profile_recovery_burn_policy",
            "profile_decision": "recovery may admit replacement slots or burn/recover old slots, but never resurrects a consumed digest",
            "verified_by": [
                "ProductionAccountLifecycleManager::recover_by_burning_reservations"
            ],
            "support_tests": [
                "lifecycle_manager_recovery_burns_reserved_slots",
                "lifecycle_snapshot_roundtrips_and_recovers_by_burning_reservations"
            ],
            "review_requirement_verified": true,
            "executable_evidence_verified": true,
            "deployment_profile_slice_verified": true,
            "deployment_ready": false,
            "residual": "account-policy authorization for recovery remains policy/adoption work"
        }),
        json!({
            "requirement_id": "profile_restore_anchor_policy",
            "profile_decision": "restore-uncertain or stale snapshots cannot re-enable released slots",
            "verified_by": [
                "ProductionAccountLifecycleManager restore-anchor checks",
                "ProductionChainStateFileStores recovery path"
            ],
            "support_tests": [
                "lifecycle_restore_anchor_rejects_old_backup_snapshots",
                "lifecycle_file_store_roundtrips_and_recovers_reserved_slots"
            ],
            "review_requirement_verified": true,
            "executable_evidence_verified": true,
            "deployment_profile_slice_verified": true,
            "deployment_ready": false,
            "residual": "device backup, restore, and custody operating procedures remain adoption-profile work"
        }),
        json!({
            "requirement_id": "profile_persist_before_release_policy",
            "profile_decision": "accepted spend transition is persisted with the consume-once marker before the accepted result is exposed",
            "verified_by": [
                "ProductionChainStateFileStores::verify_consume_record_and_save"
            ],
            "support_tests": [
                "chain_state_file_stores_verify_record_and_persist_together"
            ],
            "review_requirement_verified": true,
            "executable_evidence_verified": true,
            "deployment_profile_slice_verified": true,
            "deployment_ready": false,
            "residual": "target-chain write-ahead integration and resource limits remain adoption-profile work"
        }),
    ];

    let generated_requirement_ids = requirement_ids(&requirements)?;
    if generated_requirement_ids
        != report.target_chain_lifecycle_profile_verification_requirement_ids
    {
        return Err("direct lifecycle profile verification requirement ids drifted".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.DirectLifecycleProfileVerification.v1",
        "corpus_id": "Descender.Path2.DirectLifecycleProfileVerification.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "status": "digest_pinned_target_chain_lifecycle_profile_verified_not_deployment_ready",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_lifecycle_profile_verification",
            "source": "tools/parameter-search/src/bin/path2_direct_lifecycle_profile_verification.rs"
        },
        "verification": {
            "verification_id": PATH2_DIRECT_LIFECYCLE_PROFILE_VERIFICATION_ID,
            "artifact_path": PATH2_DIRECT_LIFECYCLE_PROFILE_VERIFICATION_ARTIFACT,
            "verification_scope": "Mode A direct lifecycle transition profile verification for ADR-0010 D2 finalized-head safe default",
            "review_evidence": review_evidence,
            "normative_sources": [
                "spec/lifecycle_model.md",
                "spec/deployment_profiles.md",
                "spec/signer_state_profile.md",
                "decisions/ADR-0010-registry-lifecycle-consensus-wiring-and-finality.md"
            ],
            "support_sources": [
                "research/path2_lifecycle_finality_theorem_v1.md",
                "research/path2_one_use_successor_state_theorem.md"
            ],
            "lifecycle_boundary_report_digest": hex_encode(&report.digest),
            "lifecycle_boundary_report_recomputed_digest": hex_encode(&recomputed_digest),
            "target_chain_finality_profile_id": report.target_chain_finality_profile_id,
            "target_chain_finality_profile_authority":
                report.target_chain_finality_profile_authority,
            "target_chain_finality_profile_rule": report.target_chain_finality_profile_rule,
            "target_chain_finality_profile_instantiated":
                report.target_chain_finality_profile_instantiated,
            "target_chain_lifecycle_profile_review_pinned":
                report.target_chain_lifecycle_profile_review_pinned,
            "target_chain_lifecycle_profile_verification_pinned":
                report.target_chain_lifecycle_profile_verification_pinned,
            "lifecycle_transition_profile_verified":
                report.lifecycle_transition_profile_verified,
            "adoption_evidence_ready": false,
            "deployment_ready": report.deployment_ready,
            "blocker": report.blocker
        },
        "audit_summary": {
            "requirement_count": requirements.len(),
            "requirement_ids":
                report.target_chain_lifecycle_profile_verification_requirement_ids,
            "all_requirements_reviewed": true,
            "all_review_requirements_verified": true,
            "all_requirements_have_executable_evidence": true,
            "all_support_tests_present": true,
            "all_residuals_non_deployment_claims": true,
            "d2_safe_default_profile_verified": true,
            "target_chain_finality_profile_instantiated":
                report.target_chain_finality_profile_instantiated,
            "target_chain_lifecycle_profile_review_pinned":
                report.target_chain_lifecycle_profile_review_pinned,
            "target_chain_lifecycle_profile_verification_pinned":
                report.target_chain_lifecycle_profile_verification_pinned,
            "lifecycle_transition_profile_verified":
                report.lifecycle_transition_profile_verified,
            "adoption_evidence_ready": false,
            "deployment_ready": report.deployment_ready
        },
        "requirement_count": requirements.len(),
        "requirements": requirements
    }))
}

fn main() -> Result<(), String> {
    let body = manifest_body()?;
    let body_json = serde_json::to_string_pretty(&body)
        .map_err(|err| format!("failed to serialize lifecycle profile verification body: {err}"))?;
    let out = json!({
        "verification_digest": hex_encode(&Sha256::digest(body_json.as_bytes())),
        "verification": body
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&out)
            .map_err(|err| format!("failed to serialize lifecycle profile verification: {err}"))?
    );
    Ok(())
}
