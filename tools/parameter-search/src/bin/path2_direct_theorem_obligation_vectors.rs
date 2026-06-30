use descender_parameter_search::{
    manifest_vector_evidence::{
        path2_direct_accepted_composition_evidence_artifact_v1,
        path2_direct_accepted_spend_tuple_evidence_artifact_v1,
    },
    production_reference::{
        PROFILE, path2_account_creation_proof_boundary_report_v1,
        path2_account_creation_proof_boundary_report_v1_digest,
        path2_direct_lifecycle_boundary_report_v1,
        path2_direct_lifecycle_boundary_report_v1_digest,
        path2_direct_qrom_action_boundary_report_v1,
        path2_direct_qrom_action_boundary_report_v1_digest,
        path2_direct_theorem_boundary_report_v1, path2_direct_theorem_boundary_report_v1_digest,
        validate_path2_account_creation_proof_boundary_report_v1,
        validate_path2_direct_lifecycle_boundary_report_v1,
        validate_path2_direct_qrom_action_boundary_report_v1,
        validate_path2_direct_theorem_boundary_report_v1,
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

fn manifest_body() -> Result<Value, String> {
    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    let direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    if direct_digest != direct_report.digest {
        return Err("direct theorem boundary report digest mismatch".to_string());
    }

    let qrom_action_report = path2_direct_qrom_action_boundary_report_v1();
    validate_path2_direct_qrom_action_boundary_report_v1(&qrom_action_report)?;
    let qrom_action_digest =
        path2_direct_qrom_action_boundary_report_v1_digest(&qrom_action_report);
    if qrom_action_digest != qrom_action_report.digest {
        return Err("direct QROM/action boundary report digest mismatch".to_string());
    }
    if direct_report.direct_qrom_action_boundary_report_digest != qrom_action_report.digest {
        return Err("direct theorem report lost QROM/action digest binding".to_string());
    }

    let lifecycle_report = path2_direct_lifecycle_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&lifecycle_report)?;
    let lifecycle_digest = path2_direct_lifecycle_boundary_report_v1_digest(&lifecycle_report);
    if lifecycle_digest != lifecycle_report.digest {
        return Err("direct lifecycle boundary report digest mismatch".to_string());
    }
    if direct_report.direct_lifecycle_boundary_report_digest != lifecycle_report.digest {
        return Err("direct theorem report lost lifecycle digest binding".to_string());
    }

    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    let proof_digest = path2_account_creation_proof_boundary_report_v1_digest(&proof_report);
    if proof_digest != proof_report.digest {
        return Err("account-creation proof-boundary report digest mismatch".to_string());
    }
    if direct_report.account_creation_proof_boundary_report_digest != proof_report.digest {
        return Err("direct theorem report lost proof-boundary digest binding".to_string());
    }
    let accepted_spend_tuple_artifact = path2_direct_accepted_spend_tuple_evidence_artifact_v1()?;
    let accepted_composition_artifact = path2_direct_accepted_composition_evidence_artifact_v1()?;

    let cases: Vec<Value> = direct_report
        .obligation_rows
        .iter()
        .map(|row| {
            json!({
                "case_id": row.obligation_id,
                "kind": "audit",
                "acceptance_hypothesis": row.acceptance_hypothesis,
                "supporting_surface": row.supporting_surface,
                "current_evidence": row.current_evidence,
                "required_closure": row.required_closure,
                "named_break": row.named_break,
                "blocker": row.blocker,
                "open": row.open
            })
        })
        .collect();

    if cases.len() != direct_report.obligation_audit_vector_case_ids.len() {
        return Err("direct theorem obligation vector generator case count drifted".to_string());
    }
    for (case, expected) in cases
        .iter()
        .zip(direct_report.obligation_audit_vector_case_ids.iter())
    {
        if case["case_id"] != *expected {
            return Err(format!(
                "direct theorem obligation vector case id drift: expected {expected}"
            ));
        }
    }

    Ok(json!({
        "schema": "Descender.Path2.DirectTheoremObligationVectors.v1",
        "corpus_id": "Descender.Path2.DirectTheoremObligationAuditVectors.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "status": "digest_pinned_direct_theorem_obligation_audit_vectors",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_theorem_obligation_vectors",
            "source": "tools/parameter-search/src/bin/path2_direct_theorem_obligation_vectors.rs"
        },
        "direct_theorem_report": {
            "domain": direct_report.domain,
            "digest": hex_encode(&direct_report.digest),
            "recomputed_digest": hex_encode(&direct_digest),
            "theorem_target": direct_report.theorem_target,
            "theorem_conclusion": direct_report.theorem_conclusion,
            "accepted_tuple_pinned": direct_report.accepted_tuple_pinned,
            "accepted_tuple_evidence_schema":
                direct_report.accepted_tuple_evidence_schema,
            "accepted_tuple_evidence_endpoint":
                direct_report.accepted_tuple_evidence_endpoint,
            "accepted_tuple_evidence_present":
                direct_report.accepted_tuple_evidence_present,
            "accepted_composition_evidence_schema":
                direct_report.accepted_composition_evidence_schema,
            "accepted_composition_evidence_endpoint":
                direct_report.accepted_composition_evidence_endpoint,
            "accepted_composition_evidence_present":
                direct_report.accepted_composition_evidence_present,
            "acceptance_hypotheses_pinned": direct_report.acceptance_hypotheses_pinned,
            "named_break_classes_pinned": direct_report.named_break_classes_pinned,
            "obligation_count": direct_report.obligation_count,
            "closure_dependency_count": direct_report.closure_dependency_count,
            "closure_dependency_rows": direct_report.closure_dependency_rows,
            "obligation_audit_vector_case_count":
                direct_report.obligation_audit_vector_case_count,
            "every_hypothesis_has_obligation":
                direct_report.every_hypothesis_has_obligation,
            "every_obligation_has_named_break":
                direct_report.every_obligation_has_named_break,
            "every_open_obligation_blocks_deployment":
                direct_report.every_open_obligation_blocks_deployment,
            "every_closure_dependency_has_open_obligation":
                direct_report.every_closure_dependency_has_open_obligation,
            "every_closure_dependency_has_open_support_surface":
                direct_report.every_closure_dependency_has_open_support_surface,
            "every_open_closure_dependency_blocks_deployment":
                direct_report.every_open_closure_dependency_blocks_deployment,
            "composition_theorem_instantiated":
                direct_report.composition_theorem_instantiated,
            "account_creation_proof_boundary_verified":
                direct_report.account_creation_proof_boundary_verified,
            "lifecycle_transition_profile_verified":
                direct_report.lifecycle_transition_profile_verified,
            "qrom_and_action_bounds_linked_to_theorem":
                direct_report.qrom_and_action_bounds_linked_to_theorem,
            "deployment_ready": direct_report.deployment_ready,
            "blocker": direct_report.blocker
        },
        "qrom_action_boundary_report": {
            "domain": qrom_action_report.domain,
            "digest": hex_encode(&qrom_action_report.digest),
            "recomputed_digest": hex_encode(&qrom_action_digest),
            "direct_obligation_id": qrom_action_report.direct_obligation_id,
            "acceptance_hypothesis": qrom_action_report.acceptance_hypothesis,
            "named_break": qrom_action_report.named_break,
            "theorem_imports_budget": qrom_action_report.theorem_imports_budget,
            "external_qrom_review_complete":
                qrom_action_report.external_qrom_review_complete,
            "qrom_and_action_bounds_linked_to_theorem":
                qrom_action_report.qrom_and_action_bounds_linked_to_theorem,
            "deployment_ready": qrom_action_report.deployment_ready,
            "blocker": qrom_action_report.blocker
        },
        "lifecycle_boundary_report": {
            "domain": lifecycle_report.domain,
            "digest": hex_encode(&lifecycle_report.digest),
            "recomputed_digest": hex_encode(&lifecycle_digest),
            "direct_obligation_id": lifecycle_report.direct_obligation_id,
            "acceptance_hypothesis": lifecycle_report.acceptance_hypothesis,
            "named_break": lifecycle_report.named_break,
            "target_chain_finality_profile_instantiated":
                lifecycle_report.target_chain_finality_profile_instantiated,
            "target_chain_lifecycle_profile_review_pinned":
                lifecycle_report.target_chain_lifecycle_profile_review_pinned,
            "target_chain_lifecycle_profile_verification_pinned":
                lifecycle_report.target_chain_lifecycle_profile_verification_pinned,
            "lifecycle_transition_profile_verified":
                lifecycle_report.lifecycle_transition_profile_verified,
            "deployment_ready": lifecycle_report.deployment_ready,
            "blocker": lifecycle_report.blocker
        },
        "account_creation_proof_boundary_report": {
            "domain": proof_report.domain,
            "digest": hex_encode(&proof_report.digest),
            "recomputed_digest": hex_encode(&proof_digest),
            "proof_boundary_target": proof_report.proof_boundary_target,
            "proof_boundary_conclusion": proof_report.proof_boundary_conclusion,
            "accepted_boundary_evidence_schema":
                proof_report.accepted_boundary_evidence_schema,
            "accepted_boundary_evidence_endpoint":
                proof_report.accepted_boundary_evidence_endpoint,
            "accepted_boundary_evidence_present":
                proof_report.accepted_boundary_evidence_present,
            "accepted_boundary_binding_evidence_present":
                proof_report.accepted_boundary_binding_evidence_present,
            "replay_payload_digest_binding_evidence_present":
                proof_report.replay_payload_digest_binding_evidence_present,
            "deployed_anchor_binding_evidence_present":
                proof_report.deployed_anchor_binding_evidence_present,
            "proof_obligation_rows_pinned": proof_report.proof_obligation_rows_pinned,
            "every_proof_obligation_has_named_break":
                proof_report.every_proof_obligation_has_named_break,
            "every_open_proof_obligation_blocks_deployment":
                proof_report.every_open_proof_obligation_blocks_deployment,
            "closure_dependency_count": proof_report.closure_dependency_count,
            "closure_dependency_rows": proof_report.closure_dependency_rows,
            "every_closure_dependency_has_open_obligation":
                proof_report.every_closure_dependency_has_open_obligation,
            "every_closure_dependency_has_open_support_surface":
                proof_report.every_closure_dependency_has_open_support_surface,
            "every_open_closure_dependency_blocks_deployment":
                proof_report.every_open_closure_dependency_blocks_deployment,
            "deployment_ready": proof_report.deployment_ready,
            "blocker": proof_report.blocker
        },
        "direct_accepted_spend_tuple_evidence_artifact": accepted_spend_tuple_artifact,
        "direct_accepted_composition_evidence_artifact": accepted_composition_artifact,
        "audit_summary": {
            "direct_obligation_count": direct_report.obligation_count,
            "direct_obligation_audit_vector_case_count":
                direct_report.obligation_audit_vector_case_count,
            "direct_named_break_classes": direct_report.named_break_classes,
            "open_direct_obligation_count":
                direct_report.obligation_rows.iter().filter(|row| row.open).count(),
            "closed_direct_obligation_count":
                direct_report.obligation_rows.iter().filter(|row| !row.open).count(),
            "all_cases_open": direct_report.obligation_rows.iter().all(|row| row.open),
            "all_cases_have_named_break": direct_report.every_obligation_has_named_break,
            "all_open_cases_block_deployment":
                direct_report.every_open_obligation_blocks_deployment,
            "composition_theorem_instantiated":
                direct_report.composition_theorem_instantiated,
            "account_creation_proof_boundary_verified":
                direct_report.account_creation_proof_boundary_verified,
            "lifecycle_transition_profile_verified":
                direct_report.lifecycle_transition_profile_verified,
            "qrom_and_action_bounds_linked_to_theorem":
                direct_report.qrom_and_action_bounds_linked_to_theorem,
            "proof_boundary_deployment_ready": proof_report.deployment_ready,
            "qrom_theorem_imports_budget": qrom_action_report.theorem_imports_budget,
            "qrom_external_review_complete":
                qrom_action_report.external_qrom_review_complete,
            "lifecycle_target_chain_finality_profile_instantiated":
                lifecycle_report.target_chain_finality_profile_instantiated,
            "lifecycle_target_chain_profile_review_pinned":
                lifecycle_report.target_chain_lifecycle_profile_review_pinned,
            "lifecycle_target_chain_profile_verification_pinned":
                lifecycle_report.target_chain_lifecycle_profile_verification_pinned,
            "deployment_ready": direct_report.deployment_ready
        },
        "case_count": cases.len(),
        "cases": cases
    }))
}

fn main() -> Result<(), String> {
    let corpus = manifest_body()?;
    let corpus_json = serde_json::to_string_pretty(&corpus).map_err(|err| {
        format!("failed to serialize direct theorem obligation corpus body: {err}")
    })?;
    let out = json!({
        "corpus_digest": hex_encode(&Sha256::digest(corpus_json.as_bytes())),
        "corpus": corpus
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&out).map_err(|err| {
            format!("failed to serialize direct theorem obligation corpus: {err}")
        })?
    );
    Ok(())
}
