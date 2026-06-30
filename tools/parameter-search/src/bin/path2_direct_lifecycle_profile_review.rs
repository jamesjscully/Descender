use descender_parameter_search::production_reference::{
    PATH2_DIRECT_LIFECYCLE_PROFILE_REVIEW_ARTIFACT, PATH2_DIRECT_LIFECYCLE_PROFILE_REVIEW_ID,
    PROFILE, path2_direct_lifecycle_boundary_report_v1,
    path2_direct_lifecycle_boundary_report_v1_digest,
    validate_path2_direct_lifecycle_boundary_report_v1,
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
                .ok_or_else(|| "lifecycle profile review row missing requirement_id".to_string())
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

    let requirements = vec![
        json!({
            "requirement_id": "profile_finality_source",
            "normative_source": "spec/lifecycle_model.md#reorg-and-finality-policy",
            "profile_decision": "D2 permanence boundary is the chain FINALIZED head",
            "evidence_surface": "Path2D2FinalityProfileState::finalize_to",
            "support_tests": [
                "path2_d2_finality_profile_reorg_frees_window_but_preserves_finalized"
            ],
            "reviewed": true,
            "adoption_evidence_ready": false,
            "residual": "concrete target-chain finality source and client mapping remain deployment-profile work"
        }),
        json!({
            "requirement_id": "profile_pending_release_policy",
            "normative_source": "spec/lifecycle_model.md#local-custody-states",
            "profile_decision": "safe default: a released spend keeps the slot locally unavailable until finality evidence or burn/replacement",
            "evidence_surface": "Path2LifecycleFinalityTheoremV1 safe default",
            "support_tests": [
                "spent_slot_ledger_blocks_stale_wallet_replay",
                "lifecycle_manager_verify_and_consume_blocks_replay"
            ],
            "reviewed": true,
            "adoption_evidence_ready": false,
            "residual": "wallet/HSM transport and persistence profile remains deployment-profile work"
        }),
        json!({
            "requirement_id": "profile_dropped_before_finality_policy",
            "normative_source": "spec/lifecycle_model.md#reorg-and-finality-policy",
            "profile_decision": "records in [finalized, head] may be freed by reorg_to(new_head), but signer-local reuse remains forbidden absent finality or burn/replacement evidence",
            "evidence_surface": "Path2D2FinalityProfileState::reorg_to",
            "support_tests": [
                "path2_d2_finality_profile_reorg_frees_window_but_preserves_finalized",
                "spent_slot_ledger_records_reorg_burns"
            ],
            "reviewed": true,
            "adoption_evidence_ready": false,
            "residual": "target-chain mempool/drop policy and signer retry UX remain deployment-profile work"
        }),
        json!({
            "requirement_id": "profile_finalized_reorg_policy",
            "normative_source": "spec/lifecycle_model.md#invariants",
            "profile_decision": "reorg below finalized head is rejected and exports the lifecycle break rather than resurrecting consumed digests",
            "evidence_surface": "Path2D2FinalityProfileState::reorg_to finalized-head guard",
            "support_tests": [
                "path2_d2_finality_profile_reorg_frees_window_but_preserves_finalized"
            ],
            "reviewed": true,
            "adoption_evidence_ready": false,
            "residual": "concrete chain finality-client review remains deployment-profile work"
        }),
        json!({
            "requirement_id": "profile_recovery_burn_policy",
            "normative_source": "spec/lifecycle_model.md#direct-mode-state-transitions",
            "profile_decision": "recovery may admit replacement slots or burn/recover old slots, but never resurrects a consumed digest",
            "evidence_surface": "ProductionAccountLifecycleManager recovery burn path",
            "support_tests": [
                "lifecycle_manager_recovery_burns_reserved_slots",
                "lifecycle_snapshot_roundtrips_and_recovers_by_burning_reservations"
            ],
            "reviewed": true,
            "adoption_evidence_ready": false,
            "residual": "account-policy recovery authorization remains policy/deployment work"
        }),
        json!({
            "requirement_id": "profile_restore_anchor_policy",
            "normative_source": "spec/lifecycle_model.md#local-custody-states",
            "profile_decision": "restore-uncertain or stale snapshots cannot re-enable released slots",
            "evidence_surface": "ProductionAccountLifecycleManager restore-anchor checks",
            "support_tests": [
                "lifecycle_restore_anchor_rejects_old_backup_snapshots",
                "lifecycle_file_store_roundtrips_and_recovers_reserved_slots"
            ],
            "reviewed": true,
            "adoption_evidence_ready": false,
            "residual": "device backup, restore, and custody operational profile remains deployment work"
        }),
        json!({
            "requirement_id": "profile_persist_before_release_policy",
            "normative_source": "spec/lifecycle_model.md#direct-mode-state-transitions",
            "profile_decision": "accepted spend transition is persisted with the consume-once marker before the accepted result is exposed",
            "evidence_surface": "ProductionChainStateFileStores::verify_consume_record_and_save",
            "support_tests": [
                "chain_state_file_stores_verify_record_and_persist_together"
            ],
            "reviewed": true,
            "adoption_evidence_ready": false,
            "residual": "target-chain client write-ahead/finality integration remains deployment-profile work"
        }),
    ];

    let generated_requirement_ids = requirement_ids(&requirements)?;
    if generated_requirement_ids != report.target_chain_lifecycle_profile_review_requirement_ids {
        return Err("direct lifecycle profile review requirement ids drifted".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.DirectLifecycleProfileReview.v1",
        "corpus_id": "Descender.Path2.DirectLifecycleProfileReview.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "status": "digest_pinned_target_chain_lifecycle_profile_review_not_deployment_ready",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_lifecycle_profile_review",
            "source": "tools/parameter-search/src/bin/path2_direct_lifecycle_profile_review.rs"
        },
        "review": {
            "review_id": PATH2_DIRECT_LIFECYCLE_PROFILE_REVIEW_ID,
            "artifact_path": PATH2_DIRECT_LIFECYCLE_PROFILE_REVIEW_ARTIFACT,
            "review_scope": "Mode A direct lifecycle profile review for ADR-0010 D2 finalized-head safe default",
            "normative_sources": [
                "spec/lifecycle_model.md",
                "spec/deployment_profiles.md",
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
            "lifecycle_transition_profile_verified":
                report.lifecycle_transition_profile_verified,
            "deployment_ready": report.deployment_ready,
            "blocker": report.blocker
        },
        "audit_summary": {
            "requirement_count": requirements.len(),
            "requirement_ids": report.target_chain_lifecycle_profile_review_requirement_ids,
            "all_requirements_have_authority": true,
            "all_requirements_reviewed": true,
            "all_reviewed_requirements_have_evidence_surface": true,
            "all_relaxations_require_new_profile": true,
            "d2_safe_default_profile_reviewed": true,
            "target_chain_finality_profile_instantiated":
                report.target_chain_finality_profile_instantiated,
            "target_chain_lifecycle_profile_review_pinned":
                report.target_chain_lifecycle_profile_review_pinned,
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
        .map_err(|err| format!("failed to serialize lifecycle profile review body: {err}"))?;
    let out = json!({
        "review_digest": hex_encode(&Sha256::digest(body_json.as_bytes())),
        "review": body
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&out)
            .map_err(|err| format!("failed to serialize lifecycle profile review: {err}"))?
    );
    Ok(())
}
