use descender_parameter_search::production_reference::{
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_account_creation_sampler_timing_report_v1,
    path2_account_creation_sampler_timing_report_v1_digest,
    path2_direct_theorem_boundary_report_v1, path2_direct_theorem_boundary_report_v1_digest,
    PROFILE,
};
use serde_json::{json, Value};
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
    let proof_boundary_report = path2_account_creation_proof_boundary_report_v1();
    let sampler_timing_report = path2_account_creation_sampler_timing_report_v1();
    let recomputed_direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    if recomputed_direct_digest != direct_report.digest {
        return Err("direct theorem boundary report digest mismatch".to_string());
    }
    let recomputed_proof_boundary_digest =
        path2_account_creation_proof_boundary_report_v1_digest(&proof_boundary_report);
    if recomputed_proof_boundary_digest != proof_boundary_report.digest {
        return Err("account-creation proof boundary report digest mismatch".to_string());
    }
    let recomputed_sampler_timing_digest =
        path2_account_creation_sampler_timing_report_v1_digest(&sampler_timing_report);
    if recomputed_sampler_timing_digest != sampler_timing_report.digest {
        return Err("account-creation sampler/timing report digest mismatch".to_string());
    }
    if direct_report.account_creation_proof_boundary_report_digest != proof_boundary_report.digest {
        return Err(
            "direct theorem report is not bound to the current proof-boundary report".to_string(),
        );
    }
    if proof_boundary_report.sampler_timing_report_digest != sampler_timing_report.digest {
        return Err(
            "proof boundary report is not bound to the current sampler/timing report".to_string(),
        );
    }

    Ok(json!({
        "schema": "Descender.Path2.DirectTheoremBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.DirectTheoremBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_theorem_boundary_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_theorem_boundary_manifest",
            "source": "tools/parameter-search/src/bin/path2_direct_theorem_boundary_manifest.rs"
        },
        "authority_snapshot": {
            "direct_report_domain": direct_report.domain,
            "account_creation_proof_boundary_report_domain": proof_boundary_report.domain,
            "account_creation_sampler_timing_report_domain": sampler_timing_report.domain,
            "direct_theorem_blocker": direct_report.blocker,
            "account_creation_proof_boundary_blocker": proof_boundary_report.blocker,
            "sampler_timing_blocker": sampler_timing_report.blocker,
            "selected_payload_type": proof_boundary_report.selected_payload_type,
            "verifier_boundary_endpoint": proof_boundary_report.verifier_boundary_endpoint,
            "production_boundary_endpoint": proof_boundary_report.production_boundary_endpoint,
            "replay_payload_digest_function": proof_boundary_report.replay_payload_digest_function
        },
        "digests": {
            "direct_theorem_boundary_report_digest": hex_encode(&direct_report.digest),
            "direct_theorem_boundary_report_recomputed_digest": hex_encode(
                &recomputed_direct_digest
            ),
            "account_creation_proof_boundary_report_digest": hex_encode(
                &proof_boundary_report.digest
            ),
            "account_creation_proof_boundary_report_recomputed_digest": hex_encode(
                &recomputed_proof_boundary_digest
            ),
            "account_creation_sampler_timing_report_digest": hex_encode(
                &sampler_timing_report.digest
            ),
            "account_creation_sampler_timing_report_recomputed_digest": hex_encode(
                &recomputed_sampler_timing_digest
            ),
            "proof_boundary_report_sampler_timing_digest": hex_encode(
                &proof_boundary_report.sampler_timing_report_digest
            )
        },
        "direct_theorem_boundary": {
            "coverage": {
                "accepted_tuple_field_count": direct_report.accepted_tuple_fields.len(),
                "acceptance_hypothesis_count": direct_report.acceptance_hypotheses.len(),
                "named_break_class_count": direct_report.named_break_classes.len(),
                "obligation_count": direct_report.obligation_count,
                "every_hypothesis_has_obligation":
                    direct_report.every_hypothesis_has_obligation,
                "every_obligation_has_named_break":
                    direct_report.every_obligation_has_named_break,
                "every_open_obligation_blocks_deployment":
                    direct_report.every_open_obligation_blocks_deployment,
                "theorem_statement_pinned": direct_report.theorem_statement_pinned,
                "accepted_tuple_pinned": direct_report.accepted_tuple_pinned,
                "acceptance_hypotheses_pinned":
                    direct_report.acceptance_hypotheses_pinned,
                "named_break_classes_pinned": direct_report.named_break_classes_pinned,
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
            "accepted_tuple_fields": direct_report.accepted_tuple_fields,
            "acceptance_hypotheses": direct_report.acceptance_hypotheses,
            "named_break_classes": direct_report.named_break_classes,
            "obligation_rows": direct_report.obligation_rows
        },
        "account_creation_proof_boundary": {
            "coverage": {
                "sampler_timing_report_digest": hex_encode(
                    &proof_boundary_report.sampler_timing_report_digest
                ),
                "accepting_requirement_count":
                    proof_boundary_report.accepting_requirements.len(),
                "leaf_evidence_binding_field_count":
                    proof_boundary_report.leaf_evidence_binding_fields.len(),
                "certificate_evidence_binding_field_count":
                    proof_boundary_report.certificate_evidence_binding_fields.len(),
                "binding_leaf_context_field_count":
                    proof_boundary_report.binding_leaf_context_fields.len(),
                "payload_binding_field_count":
                    proof_boundary_report.payload_binding_fields.len(),
                "deployed_anchor_field_count":
                    proof_boundary_report.deployed_anchor_fields.len(),
                "no_export_private_material_field_count":
                    proof_boundary_report.no_export_private_material_fields.len(),
                "boundary_statement_pinned":
                    proof_boundary_report.boundary_statement_pinned,
                "selected_payload_pinned": proof_boundary_report.selected_payload_pinned,
                "replay_payload_digest_bound":
                    proof_boundary_report.replay_payload_digest_bound,
                "active_payload_mutation_coverage_present":
                    proof_boundary_report.active_payload_mutation_coverage_present,
                "deployed_anchor_rebind_tests_present":
                    proof_boundary_report.deployed_anchor_rebind_tests_present,
                "production_gate_fail_closed":
                    proof_boundary_report.production_gate_fail_closed,
                "simulator_theorem_instantiated":
                    proof_boundary_report.simulator_theorem_instantiated,
                "fri_aok_extractor_theorem_instantiated":
                    proof_boundary_report.fri_aok_extractor_theorem_instantiated,
                "fri_qrom_residual_priced":
                    proof_boundary_report.fri_qrom_residual_priced,
                "frozen_kat_corpus_complete":
                    proof_boundary_report.frozen_kat_corpus_complete,
                "external_review_complete": proof_boundary_report.external_review_complete,
                "production_gate_open": proof_boundary_report.production_gate_open,
                "deployment_ready": proof_boundary_report.deployment_ready,
                "blocker": proof_boundary_report.blocker
            },
            "accepting_requirements": proof_boundary_report.accepting_requirements,
            "leaf_evidence_binding_fields":
                proof_boundary_report.leaf_evidence_binding_fields,
            "certificate_evidence_binding_fields":
                proof_boundary_report.certificate_evidence_binding_fields,
            "binding_leaf_context_fields":
                proof_boundary_report.binding_leaf_context_fields,
            "payload_binding_fields": proof_boundary_report.payload_binding_fields,
            "deployed_anchor_fields": proof_boundary_report.deployed_anchor_fields,
            "no_export_private_material_fields":
                proof_boundary_report.no_export_private_material_fields
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
