use descender_parameter_search::production_reference::{
    PROFILE, path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_account_creation_sampler_timing_report_v1,
    path2_account_creation_sampler_timing_report_v1_digest,
    path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_fri_qrom_residual_report_v1_digest,
    path2_account_stark_payload_kat_boundary_report_v1,
    path2_account_stark_payload_kat_boundary_report_v1_digest,
    path2_account_stark_residual_matrix_report_v1,
    path2_account_stark_residual_matrix_report_v1_digest,
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
    let residual_matrix_report = path2_account_stark_residual_matrix_report_v1();
    let fri_qrom_residual_report = path2_account_stark_fri_qrom_residual_report_v1();
    let payload_kat_boundary_report = path2_account_stark_payload_kat_boundary_report_v1();

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
    if residual_matrix_report.fri_qrom_residual_report_digest != fri_qrom_residual_report.digest {
        return Err(
            "residual matrix report is not bound to the current FRI/QROM residual report"
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
            "residual_matrix_report_domain": residual_matrix_report.domain,
            "fri_qrom_residual_report_domain": fri_qrom_residual_report.domain,
            "payload_kat_boundary_report_domain": payload_kat_boundary_report.domain,
            "sampler_support_file": sampler_timing_report.sampler_support_file,
            "timing_support_file": sampler_timing_report.timing_support_file,
            "sampler_theorem_target": sampler_timing_report.sampler_theorem_target,
            "timing_break_class": sampler_timing_report.timing_break_class,
            "selected_proof_family": residual_matrix_report.selected_proof_family,
            "selected_payload_type": proof_boundary_report.selected_payload_type,
            "verifier_boundary_endpoint": proof_boundary_report.verifier_boundary_endpoint,
            "production_boundary_endpoint": proof_boundary_report.production_boundary_endpoint,
            "replay_payload_digest_function": proof_boundary_report.replay_payload_digest_function,
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
