#![recursion_limit = "256"]

use descender_parameter_search::{
    manifest_vector_evidence::{
        path2_account_creation_accepted_proof_boundary_evidence_artifact_v1,
        path2_account_creation_proof_obligation_vector_corpus_evidence_v1,
        path2_account_creation_requirement_vector_corpus_evidence_v1,
    },
    production_reference::{
        PROFILE, path2_account_creation_frame_kat_boundary_report_v1,
        path2_account_creation_frame_kat_boundary_report_v1_digest,
        path2_account_creation_no_export_boundary_report_v1,
        path2_account_creation_no_export_boundary_report_v1_digest,
        path2_account_creation_proof_boundary_report_v1,
        path2_account_creation_proof_boundary_report_v1_digest,
        path2_account_creation_sampler_timing_report_v1,
        path2_account_creation_sampler_timing_report_v1_digest,
        path2_account_stark_fri_aok_boundary_report_v1,
        path2_account_stark_fri_aok_boundary_report_v1_digest,
        path2_account_stark_fri_qrom_residual_report_v1,
        path2_account_stark_fri_qrom_residual_report_v1_digest,
        path2_account_stark_payload_kat_boundary_report_v1,
        path2_account_stark_payload_kat_boundary_report_v1_digest,
        path2_account_stark_residual_matrix_report_v1,
        path2_account_stark_residual_matrix_report_v1_digest,
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
    let proof_boundary_report = path2_account_creation_proof_boundary_report_v1();
    let sampler_timing_report = path2_account_creation_sampler_timing_report_v1();
    let no_export_report = path2_account_creation_no_export_boundary_report_v1();
    let frame_kat_report = path2_account_creation_frame_kat_boundary_report_v1();
    let residual_matrix_report = path2_account_stark_residual_matrix_report_v1();
    let fri_qrom_residual_report = path2_account_stark_fri_qrom_residual_report_v1();
    let fri_aok_boundary_report = path2_account_stark_fri_aok_boundary_report_v1();
    let payload_kat_boundary_report = path2_account_stark_payload_kat_boundary_report_v1();
    let requirement_vector_corpus_evidence =
        path2_account_creation_requirement_vector_corpus_evidence_v1()?;
    let proof_obligation_vector_corpus_evidence =
        path2_account_creation_proof_obligation_vector_corpus_evidence_v1()?;
    let accepted_proof_boundary_evidence_artifact =
        path2_account_creation_accepted_proof_boundary_evidence_artifact_v1()?;

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
    let recomputed_no_export_digest =
        path2_account_creation_no_export_boundary_report_v1_digest(&no_export_report);
    if recomputed_no_export_digest != no_export_report.digest {
        return Err("account-creation no-export boundary report digest mismatch".to_string());
    }
    let recomputed_frame_kat_digest =
        path2_account_creation_frame_kat_boundary_report_v1_digest(&frame_kat_report);
    if recomputed_frame_kat_digest != frame_kat_report.digest {
        return Err("account-creation frame KAT boundary report digest mismatch".to_string());
    }
    let recomputed_residual_matrix_digest =
        path2_account_stark_residual_matrix_report_v1_digest(&residual_matrix_report);
    if recomputed_residual_matrix_digest != residual_matrix_report.digest {
        return Err("account-STARK residual matrix report digest mismatch".to_string());
    }
    let recomputed_fri_qrom_residual_digest =
        path2_account_stark_fri_qrom_residual_report_v1_digest(&fri_qrom_residual_report);
    if recomputed_fri_qrom_residual_digest != fri_qrom_residual_report.digest {
        return Err("account-STARK FRI/QROM residual report digest mismatch".to_string());
    }
    let recomputed_fri_aok_boundary_digest =
        path2_account_stark_fri_aok_boundary_report_v1_digest(&fri_aok_boundary_report);
    if recomputed_fri_aok_boundary_digest != fri_aok_boundary_report.digest {
        return Err("account-STARK FRI/AoK boundary report digest mismatch".to_string());
    }
    let recomputed_payload_kat_boundary_digest =
        path2_account_stark_payload_kat_boundary_report_v1_digest(&payload_kat_boundary_report);
    if recomputed_payload_kat_boundary_digest != payload_kat_boundary_report.digest {
        return Err("payload KAT boundary report digest mismatch".to_string());
    }
    if proof_boundary_report.account_stark_residual_matrix_report_digest
        != residual_matrix_report.digest
    {
        return Err(
            "proof boundary report is not bound to the current account-STARK residual matrix"
                .to_string(),
        );
    }
    if proof_boundary_report.sampler_timing_report_digest != sampler_timing_report.digest {
        return Err(
            "proof boundary report is not bound to the current sampler/timing report".to_string(),
        );
    }
    if proof_boundary_report.no_export_boundary_report_digest != no_export_report.digest {
        return Err(
            "proof boundary report is not bound to the current no-export report".to_string(),
        );
    }
    if proof_boundary_report.frame_kat_boundary_report_digest != frame_kat_report.digest {
        return Err(
            "proof boundary report is not bound to the current frame KAT boundary".to_string(),
        );
    }
    if residual_matrix_report.fri_qrom_residual_report_digest != fri_qrom_residual_report.digest {
        return Err(
            "residual matrix report is not bound to the current FRI/QROM residual report"
                .to_string(),
        );
    }
    if fri_aok_boundary_report.fri_qrom_residual_report_digest != fri_qrom_residual_report.digest {
        return Err(
            "FRI/AoK boundary report is not bound to the current FRI/QROM residual report"
                .to_string(),
        );
    }
    if residual_matrix_report.fri_aok_boundary_report_digest != fri_aok_boundary_report.digest {
        return Err(
            "residual matrix report is not bound to the current FRI/AoK boundary report"
                .to_string(),
        );
    }
    if residual_matrix_report.payload_kat_boundary_report_digest
        != payload_kat_boundary_report.digest
    {
        return Err(
            "residual matrix report is not bound to the current payload KAT boundary".to_string(),
        );
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationProofBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.AccountCreationProofBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_proof_boundary_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_creation_proof_boundary_manifest",
            "source": "tools/parameter-search/src/bin/path2_account_creation_proof_boundary_manifest.rs"
        },
        "authority_snapshot": {
            "proof_boundary_report_domain": proof_boundary_report.domain,
            "sampler_timing_report_domain": sampler_timing_report.domain,
            "no_export_boundary_report_domain": no_export_report.domain,
            "frame_kat_boundary_report_domain": frame_kat_report.domain,
            "residual_matrix_report_domain": residual_matrix_report.domain,
            "fri_qrom_residual_report_domain": fri_qrom_residual_report.domain,
            "fri_aok_boundary_report_domain": fri_aok_boundary_report.domain,
            "payload_kat_boundary_report_domain": payload_kat_boundary_report.domain,
            "sampler_support_file": sampler_timing_report.sampler_support_file,
            "timing_support_file": sampler_timing_report.timing_support_file,
            "no_export_support_file": no_export_report.support_file,
            "no_export_named_break": no_export_report.named_break,
            "fri_aok_support_file": fri_aok_boundary_report.support_file,
            "fri_aok_named_break": fri_aok_boundary_report.named_break,
            "fri_aok_extractor_theorem_target":
                fri_aok_boundary_report.extractor_theorem_target,
            "sampler_theorem_target": sampler_timing_report.sampler_theorem_target,
            "timing_break_class": sampler_timing_report.timing_break_class,
            "selected_proof_family": residual_matrix_report.selected_proof_family,
            "proof_boundary_target": proof_boundary_report.proof_boundary_target,
            "proof_boundary_conclusion":
                proof_boundary_report.proof_boundary_conclusion,
            "selected_payload_type": proof_boundary_report.selected_payload_type,
            "verifier_boundary_endpoint": proof_boundary_report.verifier_boundary_endpoint,
            "production_boundary_endpoint": proof_boundary_report.production_boundary_endpoint,
            "replay_payload_digest_function": proof_boundary_report.replay_payload_digest_function,
            "accepted_boundary_evidence_schema":
                proof_boundary_report.accepted_boundary_evidence_schema,
            "accepted_boundary_evidence_endpoint":
                proof_boundary_report.accepted_boundary_evidence_endpoint,
            "proof_boundary_blocker": proof_boundary_report.blocker,
            "payload_kat_boundary_blocker": payload_kat_boundary_report.blocker
        },
        "digests": {
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
            ),
            "account_creation_no_export_boundary_report_digest": hex_encode(
                &no_export_report.digest
            ),
            "account_creation_no_export_boundary_report_recomputed_digest": hex_encode(
                &recomputed_no_export_digest
            ),
            "proof_boundary_report_no_export_digest": hex_encode(
                &proof_boundary_report.no_export_boundary_report_digest
            ),
            "account_creation_frame_kat_boundary_report_digest": hex_encode(
                &frame_kat_report.digest
            ),
            "account_creation_frame_kat_boundary_report_recomputed_digest": hex_encode(
                &recomputed_frame_kat_digest
            ),
            "proof_boundary_report_frame_kat_digest": hex_encode(
                &proof_boundary_report.frame_kat_boundary_report_digest
            ),
            "account_stark_residual_matrix_report_digest": hex_encode(
                &residual_matrix_report.digest
            ),
            "account_stark_residual_matrix_report_recomputed_digest": hex_encode(
                &recomputed_residual_matrix_digest
            ),
            "account_stark_fri_qrom_residual_report_digest": hex_encode(
                &fri_qrom_residual_report.digest
            ),
            "account_stark_fri_qrom_residual_report_recomputed_digest": hex_encode(
                &recomputed_fri_qrom_residual_digest
            ),
            "account_stark_fri_aok_boundary_report_digest": hex_encode(
                &fri_aok_boundary_report.digest
            ),
            "account_stark_fri_aok_boundary_report_recomputed_digest": hex_encode(
                &recomputed_fri_aok_boundary_digest
            ),
            "fri_aok_boundary_report_fri_qrom_digest": hex_encode(
                &fri_aok_boundary_report.fri_qrom_residual_report_digest
            ),
            "residual_matrix_report_fri_aok_digest": hex_encode(
                &residual_matrix_report.fri_aok_boundary_report_digest
            ),
            "payload_kat_boundary_report_digest": hex_encode(
                &payload_kat_boundary_report.digest
            ),
            "payload_kat_boundary_report_recomputed_digest": hex_encode(
                &recomputed_payload_kat_boundary_digest
            )
        },
        "account_creation_proof_boundary": {
            "coverage": {
                "sampler_timing_report_digest": hex_encode(
                    &proof_boundary_report.sampler_timing_report_digest
                ),
                "no_export_boundary_report_digest": hex_encode(
                    &proof_boundary_report.no_export_boundary_report_digest
                ),
                "account_stark_residual_matrix_report_digest": hex_encode(
                    &proof_boundary_report.account_stark_residual_matrix_report_digest
                ),
                "frame_kat_boundary_report_digest": hex_encode(
                    &proof_boundary_report.frame_kat_boundary_report_digest
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
                "proof_obligation_named_break_class_count":
                    proof_boundary_report.proof_obligation_named_break_classes.len(),
                "requirement_coverage_count":
                    proof_boundary_report.requirement_coverage_count,
                "proof_obligation_count":
                    proof_boundary_report.proof_obligation_count,
                "closure_dependency_count":
                    proof_boundary_report.closure_dependency_count,
                "closure_dependency_evidence_count":
                    proof_boundary_report.closure_dependency_evidence_count,
                "closure_dependency_open_obligation_evidence_count":
                    proof_boundary_report
                        .closure_dependency_open_obligation_evidence_count,
                "closure_dependency_open_support_evidence_count":
                    proof_boundary_report.closure_dependency_open_support_evidence_count,
                "proof_obligation_audit_vector_case_count":
                    proof_boundary_report.proof_obligation_audit_vector_case_count,
                "current_vector_covered_requirement_count":
                    proof_boundary_report.current_vector_covered_requirement_count,
                "current_vector_evidence_kinds": proof_boundary_report
                    .requirement_coverage_rows
                    .iter()
                    .map(|row| row.vector_evidence_kind)
                    .collect::<Vec<_>>(),
                "every_accepting_requirement_has_coverage_row":
                    proof_boundary_report.every_accepting_requirement_has_coverage_row,
                "every_vector_covered_requirement_has_public_rejection_class":
                    proof_boundary_report
                        .every_vector_covered_requirement_has_public_rejection_class,
                "every_current_vector_case_is_known_kat_row":
                    proof_boundary_report.every_current_vector_case_is_known_kat_row,
                "every_requirement_public_rejection_class_is_known":
                    proof_boundary_report.every_requirement_public_rejection_class_is_known,
                "every_proof_obligation_has_named_break":
                    proof_boundary_report.every_proof_obligation_has_named_break,
                "every_open_proof_obligation_blocks_deployment":
                    proof_boundary_report.every_open_proof_obligation_blocks_deployment,
                "every_closure_dependency_has_open_obligation":
                    proof_boundary_report.every_closure_dependency_has_open_obligation,
                "every_closure_dependency_has_open_support_surface":
                    proof_boundary_report.every_closure_dependency_has_open_support_surface,
                "every_open_closure_dependency_blocks_deployment":
                    proof_boundary_report.every_open_closure_dependency_blocks_deployment,
                "every_closure_dependency_has_evidence":
                    proof_boundary_report.every_closure_dependency_has_evidence,
                "boundary_statement_pinned":
                    proof_boundary_report.boundary_statement_pinned,
                "selected_payload_pinned": proof_boundary_report.selected_payload_pinned,
                "accepted_boundary_evidence_present":
                    proof_boundary_report.accepted_boundary_evidence_present,
                "accepted_boundary_binding_evidence_present":
                    proof_boundary_report.accepted_boundary_binding_evidence_present,
                "replay_payload_digest_bound":
                    proof_boundary_report.replay_payload_digest_bound,
                "replay_payload_digest_binding_evidence_present":
                    proof_boundary_report.replay_payload_digest_binding_evidence_present,
                "active_payload_mutation_coverage_present":
                    proof_boundary_report.active_payload_mutation_coverage_present,
                "deployed_anchor_rebind_tests_present":
                    proof_boundary_report.deployed_anchor_rebind_tests_present,
                "deployed_anchor_binding_evidence_present":
                    proof_boundary_report.deployed_anchor_binding_evidence_present,
                "requirement_coverage_matrix_pinned":
                    proof_boundary_report.requirement_coverage_matrix_pinned,
                "proof_obligation_rows_pinned":
                    proof_boundary_report.proof_obligation_rows_pinned,
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
            "proof_boundary_target": proof_boundary_report.proof_boundary_target,
            "proof_boundary_conclusion":
                proof_boundary_report.proof_boundary_conclusion,
            "accepted_boundary_evidence_schema":
                proof_boundary_report.accepted_boundary_evidence_schema,
            "accepted_boundary_evidence_endpoint":
                proof_boundary_report.accepted_boundary_evidence_endpoint,
            "accepting_requirements": proof_boundary_report.accepting_requirements,
            "requirement_coverage_rows": proof_boundary_report.requirement_coverage_rows,
            "leaf_evidence_binding_fields":
                proof_boundary_report.leaf_evidence_binding_fields,
            "certificate_evidence_binding_fields":
                proof_boundary_report.certificate_evidence_binding_fields,
            "binding_leaf_context_fields":
                proof_boundary_report.binding_leaf_context_fields,
            "payload_binding_fields": proof_boundary_report.payload_binding_fields,
            "deployed_anchor_fields": proof_boundary_report.deployed_anchor_fields,
            "no_export_private_material_fields":
                proof_boundary_report.no_export_private_material_fields,
            "proof_obligation_named_break_classes":
                proof_boundary_report.proof_obligation_named_break_classes,
            "proof_obligation_audit_vector_case_ids":
                proof_boundary_report.proof_obligation_audit_vector_case_ids,
            "proof_obligation_rows": proof_boundary_report.proof_obligation_rows,
            "closure_dependency_rows": proof_boundary_report.closure_dependency_rows,
            "closure_dependency_evidence_rows":
                proof_boundary_report.closure_dependency_evidence_rows,
            "proof_obligation_vector_corpus_evidence":
                proof_obligation_vector_corpus_evidence,
            "accepted_proof_boundary_evidence_artifact":
                accepted_proof_boundary_evidence_artifact,
            "requirement_vector_corpus_evidence": requirement_vector_corpus_evidence
        },
        "no_export_boundary_snapshot": {
            "coverage": {
                "payload_byte_model_report_digest": hex_encode(
                    &no_export_report.payload_byte_model_report_digest
                ),
                "payload_kat_boundary_report_digest": hex_encode(
                    &no_export_report.payload_kat_boundary_report_digest
                ),
                "payload_public_field_count": no_export_report.payload_public_field_count,
                "public_payload_field_mutation_count":
                    no_export_report.public_payload_field_mutation_count,
                "no_export_vector_case_count":
                    no_export_report.no_export_vector_case_count,
                "public_payload_exports_private_material":
                    no_export_report.public_payload_exports_private_material,
                "no_export_field_denylist_pinned":
                    no_export_report.no_export_field_denylist_pinned,
                "simulator_input_allowlist_pinned":
                    no_export_report.simulator_input_allowlist_pinned,
                "byte_model_public_payload_only":
                    no_export_report.byte_model_public_payload_only,
                "boundary_mapping_complete": no_export_report.boundary_mapping_complete,
                "roundtrip_regression_present": no_export_report.roundtrip_regression_present,
                "production_gate_fail_closed": no_export_report.production_gate_fail_closed,
                "no_export_vectors_present": no_export_report.no_export_vectors_present,
                "public_bundle_no_export_lean_theorem":
                    no_export_report.public_bundle_no_export_lean_theorem,
                "public_bundle_no_export_predicate_instantiated":
                    no_export_report.public_bundle_no_export_predicate_instantiated,
                "simulator_input_allowlist_lean_theorem":
                    no_export_report.simulator_input_allowlist_lean_theorem,
                "simulator_input_allowlist_predicate_instantiated":
                    no_export_report.simulator_input_allowlist_predicate_instantiated,
                "accepted_public_anchor_package_lean_theorem":
                    no_export_report.accepted_public_anchor_package_lean_theorem,
                "accepted_public_anchor_package_bound":
                    no_export_report.accepted_public_anchor_package_bound,
                "accepted_bundle_public_anchor_simulation_boundary_lean_theorem":
                    no_export_report
                        .accepted_bundle_public_anchor_simulation_boundary_lean_theorem,
                "accepted_bundle_public_anchor_simulation_boundary_bound":
                    no_export_report.accepted_bundle_public_anchor_simulation_boundary_bound,
                "accepted_bundle_simulator_input_from_public_anchors_lean_theorem":
                    no_export_report
                        .accepted_bundle_simulator_input_from_public_anchors_lean_theorem,
                "accepted_bundle_simulator_input_from_public_anchors_bound":
                    no_export_report.accepted_bundle_simulator_input_from_public_anchors_bound,
                "simulator_theorem_instantiated":
                    no_export_report.simulator_theorem_instantiated,
                "no_export_theorem_instantiated":
                    no_export_report.no_export_theorem_instantiated,
                "accepted_bundle_simulatable_from_public_anchors":
                    no_export_report.accepted_bundle_simulatable_from_public_anchors,
                "frozen_no_export_vectors_complete":
                    no_export_report.frozen_no_export_vectors_complete,
                "external_review_complete": no_export_report.external_review_complete,
                "deployment_ready": no_export_report.deployment_ready,
                "blocker": no_export_report.blocker
            },
            "selected_payload_type": no_export_report.selected_payload_type,
            "verifier_boundary_endpoint": no_export_report.verifier_boundary_endpoint,
            "production_boundary_endpoint": no_export_report.production_boundary_endpoint,
            "no_export_requirement": no_export_report.no_export_requirement,
            "simulator_theorem_target": no_export_report.simulator_theorem_target,
            "no_export_theorem_target": no_export_report.no_export_theorem_target,
            "named_break": no_export_report.named_break,
            "simulator_public_input_fields":
                no_export_report.simulator_public_input_fields,
            "no_export_vector_case_ids": no_export_report.no_export_vector_case_ids,
            "no_export_private_material_fields":
                no_export_report.no_export_private_material_fields
        },
        "sampler_timing_snapshot": {
            "coverage": {
                "sampler_boundary_pinned": sampler_timing_report.sampler_boundary_pinned,
                "timing_boundary_pinned": sampler_timing_report.timing_boundary_pinned,
                "creation_statement_excludes_spend_message":
                    sampler_timing_report.creation_statement_excludes_spend_message,
                "creation_statement_excludes_signature_bytes":
                    sampler_timing_report.creation_statement_excludes_signature_bytes,
                "checked_core_challenge_binds_message":
                    sampler_timing_report.checked_core_challenge_binds_message,
                "checked_core_challenge_binds_state_root":
                    sampler_timing_report.checked_core_challenge_binds_state_root,
                "checked_core_challenge_binds_slot_authorization_digest":
                    sampler_timing_report.checked_core_challenge_binds_slot_authorization_digest,
                "checked_core_challenge_binds_proof_commitment":
                    sampler_timing_report.checked_core_challenge_binds_proof_commitment,
                "state_namespace_membership_required":
                    sampler_timing_report.state_namespace_membership_required,
                "consume_once_required": sampler_timing_report.consume_once_required,
                "direct_spend_composition_hook_present":
                    sampler_timing_report.direct_spend_composition_hook_present,
                "honest_sampler_theorem_instantiated":
                    sampler_timing_report.honest_sampler_theorem_instantiated,
                "authorization_class_binding_theorem_instantiated":
                    sampler_timing_report.authorization_class_binding_theorem_instantiated,
                "no_postchallenge_timing_theorem_instantiated":
                    sampler_timing_report.no_postchallenge_timing_theorem_instantiated,
                "deployment_ready": sampler_timing_report.deployment_ready,
                "blocker": sampler_timing_report.blocker
            },
            "sampler_construction_steps": sampler_timing_report.sampler_construction_steps,
            "timing_order_steps": sampler_timing_report.timing_order_steps,
            "prechallenge_fixed_through_step":
                sampler_timing_report.prechallenge_fixed_through_step,
            "spend_challenge_derived_at_step":
                sampler_timing_report.spend_challenge_derived_at_step,
            "creation_statement_builder": sampler_timing_report.creation_statement_builder,
            "checked_core_challenge_frame": sampler_timing_report.checked_core_challenge_frame,
            "direct_state_namespace_validator":
                sampler_timing_report.direct_state_namespace_validator,
            "consume_once_validator": sampler_timing_report.consume_once_validator,
            "direct_spend_composition_endpoint":
                sampler_timing_report.direct_spend_composition_endpoint
        },
        "selected_account_stark_residual_matrix": {
            "coverage": {
                "blocker_count": residual_matrix_report.blocker_count,
                "fri_qrom_residual_report_digest": hex_encode(
                    &residual_matrix_report.fri_qrom_residual_report_digest
                ),
                "fri_aok_boundary_report_digest": hex_encode(
                    &residual_matrix_report.fri_aok_boundary_report_digest
                ),
                "every_row_has_named_break":
                    residual_matrix_report.every_row_has_named_break,
                "every_row_has_blocker": residual_matrix_report.every_row_has_blocker,
                "production_gate_fail_closed":
                    residual_matrix_report.production_gate_fail_closed,
                "legacy_routes_non_authoritative":
                    residual_matrix_report.legacy_routes_non_authoritative,
                "deployment_ready": residual_matrix_report.deployment_ready
            },
            "rows": residual_matrix_report.rows
        },
        "fri_aok_boundary_snapshot": {
            "coverage": {
                "fri_qrom_residual_report_digest": hex_encode(
                    &fri_aok_boundary_report.fri_qrom_residual_report_digest
                ),
                "air_relation_class_count":
                    fri_aok_boundary_report.air_relation_class_count,
                "med_findings_total": fri_aok_boundary_report.med_findings_total,
                "med_findings_remediated":
                    fri_aok_boundary_report.med_findings_remediated,
                "context_binding_hardened_in_air":
                    fri_aok_boundary_report.context_binding_hardened_in_air,
                "packed_witness_binding_negative_tested":
                    fri_aok_boundary_report.packed_witness_binding_negative_tested,
                "c4_inverse_corner_regression_present":
                    fri_aok_boundary_report.c4_inverse_corner_regression_present,
                "air_false_accept_hole_found":
                    fri_aok_boundary_report.air_false_accept_hole_found,
                "fri_schedule_pinned": fri_aok_boundary_report.fri_schedule_pinned,
                "challenge_field_pinned": fri_aok_boundary_report.challenge_field_pinned,
                "fri_aok_extractor_theorem_instantiated":
                    fri_aok_boundary_report.fri_aok_extractor_theorem_instantiated,
                "accepting_proof_implies_extractable_witness":
                    fri_aok_boundary_report.accepting_proof_implies_extractable_witness,
                "external_extractor_review_complete":
                    fri_aok_boundary_report.external_extractor_review_complete,
                "deployment_ready": fri_aok_boundary_report.deployment_ready,
                "blocker": fri_aok_boundary_report.blocker
            },
            "support_file": fri_aok_boundary_report.support_file,
            "selected_proof_family": fri_aok_boundary_report.selected_proof_family,
            "selected_payload_type": fri_aok_boundary_report.selected_payload_type,
            "account_stark_config_source":
                fri_aok_boundary_report.account_stark_config_source,
            "proof_system": fri_aok_boundary_report.proof_system,
            "residual_item": fri_aok_boundary_report.residual_item,
            "extractor_theorem_target": fri_aok_boundary_report.extractor_theorem_target,
            "named_break": fri_aok_boundary_report.named_break
        },
        "fri_qrom_residual_snapshot": {
            "coverage": {
                "fri_schedule_pinned": fri_qrom_residual_report.fri_schedule_pinned,
                "challenge_field_pinned": fri_qrom_residual_report.challenge_field_pinned,
                "checked_core_qrom_budget_pinned":
                    fri_qrom_residual_report.checked_core_qrom_budget_pinned,
                "account_stark_transcript_qrom_theorem_instantiated":
                    fri_qrom_residual_report
                        .account_stark_transcript_qrom_theorem_instantiated,
                "recursion_pricing_complete":
                    fri_qrom_residual_report.recursion_pricing_complete,
                "external_fri_review_complete":
                    fri_qrom_residual_report.external_fri_review_complete,
                "external_qrom_review_complete":
                    fri_qrom_residual_report.external_qrom_review_complete,
                "deployment_profile_accepted":
                    fri_qrom_residual_report.deployment_profile_accepted,
                "residual_pricing_complete":
                    fri_qrom_residual_report.residual_pricing_complete,
                "deployment_ready": fri_qrom_residual_report.deployment_ready,
                "blocker": fri_qrom_residual_report.blocker
            },
            "fri_schedule": {
                "config_source": fri_qrom_residual_report.account_stark_config_source,
                "proof_system": fri_qrom_residual_report.proof_system,
                "poseidon2_width": fri_qrom_residual_report.poseidon2_width,
                "log_blowup": fri_qrom_residual_report.fri_log_blowup,
                "num_queries": fri_qrom_residual_report.fri_num_queries,
                "query_pow_bits": fri_qrom_residual_report.fri_query_pow_bits,
                "commit_pow_bits": fri_qrom_residual_report.fri_commit_pow_bits,
                "conservative_johnson_bits":
                    fri_qrom_residual_report.fri_conservative_johnson_bits,
                "optimistic_capacity_bits":
                    fri_qrom_residual_report.fri_optimistic_capacity_bits,
                "challenge_extension_degree":
                    fri_qrom_residual_report.challenge_extension_degree,
                "challenge_field_bits_floor":
                    fri_qrom_residual_report.challenge_field_bits_floor,
                "poseidon2_sponge_floor_bits":
                    fri_qrom_residual_report.poseidon2_sponge_floor_bits
            },
            "checked_core_qrom_budget": {
                "source_artifacts": fri_qrom_residual_report.checked_core_qrom_artifacts,
                "algebraic_challenge_bits":
                    fri_qrom_residual_report.checked_core_qrom_algebraic_challenge_bits,
                "hardening_bits":
                    fri_qrom_residual_report.checked_core_qrom_hardening_bits,
                "total_transcript_bits":
                    fri_qrom_residual_report.checked_core_qrom_total_transcript_bits,
                "quantum_hash_query_bound_log2":
                    fri_qrom_residual_report
                        .checked_core_qrom_quantum_hash_query_bound_log2,
                "classical_hash_query_bound_log2":
                    fri_qrom_residual_report
                        .checked_core_qrom_classical_hash_query_bound_log2,
                "selected_multi_target_horizon_log2":
                    fri_qrom_residual_report
                        .checked_core_qrom_selected_multi_target_horizon_log2,
                "selected_transcript_term_bits":
                    fri_qrom_residual_report
                        .checked_core_qrom_selected_transcript_term_bits,
                "max_target_count_log2_at_128_bits":
                    fri_qrom_residual_report
                        .checked_core_qrom_max_target_count_log2_at_128_bits,
                "scope_note":
                    "checked-core spend transcript ledger; not a substitute for the open account-STARK FRI transcript QROM theorem"
            }
        },
        "payload_kat_boundary_snapshot": {
            "coverage": {
                "row_count": payload_kat_boundary_report.rows.len(),
                "public_payload_field_mutation_count":
                    payload_kat_boundary_report.public_payload_field_mutation_count,
                "current_boundary_mapping_complete":
                    payload_kat_boundary_report.current_boundary_mapping_complete,
                "every_row_has_public_rejection_class":
                    payload_kat_boundary_report.every_row_has_public_rejection_class,
                "roundtrip_regression_present":
                    payload_kat_boundary_report.roundtrip_regression_present,
                "frozen_corpus_complete":
                    payload_kat_boundary_report.frozen_corpus_complete,
                "deployment_ready": payload_kat_boundary_report.deployment_ready,
                "blocker": payload_kat_boundary_report.blocker
            }
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
