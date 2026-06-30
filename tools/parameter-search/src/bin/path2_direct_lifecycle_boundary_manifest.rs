use descender_parameter_search::production_reference::{
    PROFILE, path2_direct_lifecycle_boundary_report_v1,
    path2_direct_lifecycle_boundary_report_v1_digest, path2_direct_theorem_boundary_report_v1,
    path2_direct_theorem_boundary_report_v1_digest,
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

fn manifest_body() -> Result<Value, String> {
    let lifecycle_report = path2_direct_lifecycle_boundary_report_v1();
    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&lifecycle_report)?;

    let recomputed_lifecycle_digest =
        path2_direct_lifecycle_boundary_report_v1_digest(&lifecycle_report);
    if recomputed_lifecycle_digest != lifecycle_report.digest {
        return Err("direct lifecycle boundary report digest mismatch".to_string());
    }
    let recomputed_direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    if recomputed_direct_digest != direct_report.digest {
        return Err("direct theorem boundary report digest mismatch".to_string());
    }
    if direct_report.direct_lifecycle_boundary_report_digest != lifecycle_report.digest {
        return Err(
            "direct theorem report is not bound to the current lifecycle report".to_string(),
        );
    }

    Ok(json!({
        "schema": "Descender.Path2.DirectLifecycleBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.DirectLifecycleBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_lifecycle_boundary_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_lifecycle_boundary_manifest",
            "source": "tools/parameter-search/src/bin/path2_direct_lifecycle_boundary_manifest.rs"
        },
        "authority_snapshot": {
            "lifecycle_report_domain": lifecycle_report.domain,
            "direct_report_domain": direct_report.domain,
            "support_file": lifecycle_report.support_file,
            "theorem_target": lifecycle_report.theorem_target,
            "direct_obligation_id": lifecycle_report.direct_obligation_id,
            "acceptance_hypothesis": lifecycle_report.acceptance_hypothesis,
            "named_break": lifecycle_report.named_break,
            "consume_once_validator": lifecycle_report.consume_once_validator,
            "state_namespace_validator": lifecycle_report.state_namespace_validator,
            "direct_spend_endpoint": lifecycle_report.direct_spend_endpoint,
            "lifecycle_manager_endpoint": lifecycle_report.lifecycle_manager_endpoint,
            "spent_slot_ledger_type": lifecycle_report.spent_slot_ledger_type,
            "target_chain_adapter_type": lifecycle_report.target_chain_adapter_type,
            "chain_file_store_type": lifecycle_report.chain_file_store_type,
            "target_chain_finality_profile_id":
                lifecycle_report.target_chain_finality_profile_id,
            "target_chain_finality_profile_authority":
                lifecycle_report.target_chain_finality_profile_authority,
            "target_chain_finality_profile_rule":
                lifecycle_report.target_chain_finality_profile_rule,
            "direct_theorem_blocker": lifecycle_report.blocker
        },
        "digests": {
            "direct_lifecycle_boundary_report_digest": hex_encode(&lifecycle_report.digest),
            "direct_lifecycle_boundary_report_recomputed_digest": hex_encode(
                &recomputed_lifecycle_digest
            ),
            "direct_theorem_boundary_report_digest": hex_encode(&direct_report.digest),
            "direct_theorem_boundary_report_recomputed_digest": hex_encode(
                &recomputed_direct_digest
            ),
            "direct_theorem_report_lifecycle_digest": hex_encode(
                &direct_report.direct_lifecycle_boundary_report_digest
            )
        },
        "local_successor_state_boundary": {
            "coverage": {
                "local_successor_state_model_pinned":
                    lifecycle_report.local_successor_state_model_pinned,
                "consume_once_insert_after_public_checks":
                    lifecycle_report.consume_once_insert_after_public_checks,
                "replay_rejected_by_consume_set":
                    lifecycle_report.replay_rejected_by_consume_set,
                "spent_slot_ledger_rejects_stale_wallet_replay":
                    lifecycle_report.spent_slot_ledger_rejects_stale_wallet_replay,
                "chain_state_persists_lifecycle_and_spent_record":
                    lifecycle_report.chain_state_persists_lifecycle_and_spent_record,
                "reorg_burn_support_present": lifecycle_report.reorg_burn_support_present,
                "restore_anchor_support_present": lifecycle_report.restore_anchor_support_present,
                "target_chain_finality_profile_selected":
                    lifecycle_report.target_chain_finality_profile_selected,
                "target_chain_finality_profile_instantiated":
                    lifecycle_report.target_chain_finality_profile_instantiated,
                "lifecycle_transition_profile_verified":
                    lifecycle_report.lifecycle_transition_profile_verified,
                "deployment_ready": lifecycle_report.deployment_ready,
                "blocker": lifecycle_report.blocker
            },
            "executable_tests": lifecycle_report.executable_tests
        },
        "selected_finality_profile": {
            "target_chain_finality_profile_selected":
                lifecycle_report.target_chain_finality_profile_selected,
            "target_chain_finality_profile_id":
                lifecycle_report.target_chain_finality_profile_id,
            "target_chain_finality_profile_authority":
                lifecycle_report.target_chain_finality_profile_authority,
            "target_chain_finality_profile_rule":
                lifecycle_report.target_chain_finality_profile_rule,
            "status":
                "selected_by_adr_not_instantiated_as_verified_deployment_profile"
        },
        "open_obligations": {
            "target_chain_finality_profile_instantiated":
                lifecycle_report.target_chain_finality_profile_instantiated,
            "lifecycle_transition_profile_verified":
                lifecycle_report.lifecycle_transition_profile_verified,
            "deployment_ready": lifecycle_report.deployment_ready,
            "blocker": lifecycle_report.blocker
        }
    }))
}

fn main() -> Result<(), String> {
    let body = manifest_body()?;
    let body_json = serde_json::to_string_pretty(&body)
        .map_err(|err| format!("failed to serialize manifest body: {err}"))?;
    let manifest = json!({
        "manifest_digest": hex_encode(&Sha256::digest(body_json.as_bytes())),
        "manifest": body
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&manifest)
            .map_err(|err| format!("failed to serialize manifest: {err}"))?
    );
    Ok(())
}
