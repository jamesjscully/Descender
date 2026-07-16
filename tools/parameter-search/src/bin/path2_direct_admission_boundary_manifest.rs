use descender_parameter_search::production_reference::{
    PROFILE, path2_direct_admission_boundary_report_v1,
    path2_direct_admission_boundary_report_v1_digest, path2_direct_theorem_boundary_report_v1,
    path2_direct_theorem_boundary_report_v1_digest,
    validate_path2_direct_admission_boundary_report_v1,
    validate_path2_direct_theorem_boundary_report_v1,
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
    let admission_report = path2_direct_admission_boundary_report_v1();
    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_admission_boundary_report_v1(&admission_report)?;
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    let recomputed_admission_digest =
        path2_direct_admission_boundary_report_v1_digest(&admission_report);
    let recomputed_direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    if admission_report.digest != recomputed_admission_digest {
        return Err("direct admission boundary report digest mismatch".to_string());
    }
    if direct_report.digest != recomputed_direct_digest {
        return Err("direct theorem boundary report digest mismatch".to_string());
    }
    if direct_report.direct_admission_boundary_report_digest != admission_report.digest {
        return Err("direct theorem report is not bound to admission boundary".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.DirectAdmissionBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.DirectAdmissionBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "closed_theorem_scoped_admission_boundary_not_deployment_ready",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_admission_boundary_manifest",
            "source": "tools/parameter-search/src/bin/path2_direct_admission_boundary_manifest.rs"
        },
        "authority_snapshot": {
            "selected_target_profile_id": admission_report.profile_id,
            "architecture_authority": admission_report.architecture_authority,
            "lifecycle_authority": admission_report.lifecycle_authority,
            "deployment_profile_authority": admission_report.deployment_profile_authority,
            "direct_obligation_id": admission_report.direct_obligation_id,
            "acceptance_hypothesis": admission_report.acceptance_hypothesis,
            "registry_named_break": admission_report.registry_named_break,
            "data_availability_named_break": admission_report.data_availability_named_break,
            "storage_class": admission_report.storage_class,
            "admission_state_type": admission_report.admission_state_type,
            "admission_record_type": admission_report.admission_record_type,
            "admission_endpoint": admission_report.admission_endpoint,
            "verified_public_anchors_type": admission_report.verified_public_anchors_type,
            "verified_nullifier_decode_endpoint":
                admission_report.verified_nullifier_decode_endpoint,
            "verified_admission_candidate_type":
                admission_report.verified_admission_candidate_type,
            "verified_admission_candidate_endpoint":
                admission_report.verified_admission_candidate_endpoint,
            "nullifier_registry_key_type": admission_report.nullifier_registry_key_type,
            "nullifier_encoding": admission_report.nullifier_encoding,
            "nullifier_block_admission_endpoint":
                admission_report.nullifier_block_admission_endpoint,
            "restore_anchor_type": admission_report.restore_anchor_type,
            "restore_anchor_endpoint": admission_report.restore_anchor_endpoint,
            "restore_endpoint": admission_report.restore_endpoint,
            "available_record_endpoint": admission_report.available_record_endpoint,
            "spend_endpoint": admission_report.spend_endpoint,
            "admission_closure_lean_theorem": admission_report.admission_closure_lean_theorem,
            "pad_seed_reorg_safe_uniqueness_lean_theorem":
                admission_report.pad_seed_reorg_safe_uniqueness_lean_theorem
        },
        "digests": {
            "direct_admission_boundary_report_digest": hex_encode(&admission_report.digest),
            "direct_admission_boundary_report_recomputed_digest":
                hex_encode(&recomputed_admission_digest),
            "direct_theorem_boundary_report_digest": hex_encode(&direct_report.digest),
            "direct_theorem_boundary_report_recomputed_digest":
                hex_encode(&recomputed_direct_digest),
            "direct_theorem_report_admission_digest":
                hex_encode(&direct_report.direct_admission_boundary_report_digest)
        },
        "consensus_state_contract": {
            "namespace_key_fields": admission_report.namespace_key_fields,
            "record_value_fields": admission_report.record_value_fields,
            "executable_tests": admission_report.executable_tests,
            "exact_namespace_tuple_key_pinned": admission_report.exact_namespace_tuple_key_pinned,
            "canonical_leaf_bytes_stored_directly":
                admission_report.canonical_leaf_bytes_stored_directly,
            "leaf_and_evidence_digests_rechecked":
                admission_report.leaf_and_evidence_digests_rechecked,
            "spend_reads_canonical_consensus_record":
                admission_report.spend_reads_canonical_consensus_record,
            "missing_record_rejected": admission_report.missing_record_rejected,
            "corrupt_record_fails_closed": admission_report.corrupt_record_fails_closed,
            "duplicate_admission_rejected": admission_report.duplicate_admission_rejected,
            "d2_finalized_head_reorg_semantics":
                admission_report.d2_finalized_head_reorg_semantics,
            "verified_creation_nullifier_anchor_bound":
                admission_report.verified_creation_nullifier_anchor_bound,
            "sovereign_nullifier_registry_consensus_wired":
                admission_report.sovereign_nullifier_registry_consensus_wired,
            "sovereign_nullifier_block_atomic":
                admission_report.sovereign_nullifier_block_atomic,
            "sovereign_nullifier_d2_reorg_index_rebuild_verified":
                admission_report.sovereign_nullifier_d2_reorg_index_rebuild_verified,
            "pad_seed_reorg_safe_uniqueness_selected_state_reduction_imported":
                admission_report.pad_seed_reorg_safe_uniqueness_selected_state_reduction_imported,
            "snapshot_restores_nullifier_registry":
                admission_report.snapshot_restores_nullifier_registry,
            "snapshot_restore_anchor_bound":
                admission_report.snapshot_restore_anchor_bound,
            "target_chain_data_availability_instantiated":
                admission_report.target_chain_data_availability_instantiated,
            "admission_closure_lean_imported": admission_report.admission_closure_lean_imported,
            "direct_admission_namespace_closed_by_sovereign_state":
                direct_report.direct_admission_namespace_closed_by_sovereign_state,
            "external_storage_review_complete":
                admission_report.external_storage_review_complete,
            "deployment_ready": admission_report.deployment_ready,
            "blocker": admission_report.blocker
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
