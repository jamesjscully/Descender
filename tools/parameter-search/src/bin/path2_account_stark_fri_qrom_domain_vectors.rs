use descender_parameter_search::production_reference::{
    PROFILE, path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_fri_qrom_residual_report_v1_digest,
    validate_path2_account_stark_fri_qrom_residual_report_v1,
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
    let report = path2_account_stark_fri_qrom_residual_report_v1();
    validate_path2_account_stark_fri_qrom_residual_report_v1(&report)?;
    let recomputed_digest = path2_account_stark_fri_qrom_residual_report_v1_digest(&report);
    if recomputed_digest != report.digest {
        return Err("account-STARK FRI/QROM residual report digest mismatch".to_string());
    }

    let cases = vec![
        json!({
            "case_id": "fri_qrom_selected_proof_family_domain",
            "kind": "audit",
            "surface": "Path2AccountStarkFriQromResidualReportV1.selected_proof_family",
            "selected_proof_family": report.selected_proof_family,
            "selected_payload_type": report.selected_payload_type,
            "account_stark_config_source": report.account_stark_config_source,
            "proof_system": report.proof_system,
            "domain": report.domain
        }),
        json!({
            "case_id": "fri_qrom_poseidon2_width_and_sponge_floor",
            "kind": "audit",
            "surface": "Path2AccountStarkFriQromResidualReportV1.poseidon2_width",
            "poseidon2_width": report.poseidon2_width,
            "poseidon2_sponge_floor_bits": report.poseidon2_sponge_floor_bits,
            "fs_current_rate": report.fs_challenger_rate,
            "fs_current_capacity": report.fs_challenger_capacity,
            "fs_target_rate": report.fs_target_rate,
            "fs_target_capacity": report.fs_target_capacity,
            "fs_target_generic_collision_bits_floor":
                report.fs_target_generic_collision_bits_floor,
            "fs_c9_reference_source": report.fs_c9_reference_source,
            "fs_c9_reference_native_fidelity_test":
                report.fs_c9_reference_native_fidelity_test,
            "fs_c9_reference_native_fidelity_bound":
                report.fs_c9_reference_native_fidelity_bound,
            "fs_c9_production_migration_complete":
                report.fs_c9_production_migration_complete,
            "sponge_floor_supports_current_124_bit_floor":
                report.poseidon2_sponge_floor_bits >= 124
        }),
        json!({
            "case_id": "fri_qrom_challenge_field_and_schedule",
            "kind": "audit",
            "surface": "Path2AccountStarkFriQromResidualReportV1.fri_schedule",
            "fri_log_blowup": report.fri_log_blowup,
            "fri_num_queries": report.fri_num_queries,
            "fri_query_pow_bits": report.fri_query_pow_bits,
            "fri_commit_pow_bits": report.fri_commit_pow_bits,
            "fri_conservative_johnson_bits": report.fri_conservative_johnson_bits,
            "fri_optimistic_capacity_bits": report.fri_optimistic_capacity_bits,
            "challenge_extension_degree": report.challenge_extension_degree,
            "challenge_field_bits_floor": report.challenge_field_bits_floor,
            "fri_schedule_pinned": report.fri_schedule_pinned,
            "challenge_field_pinned": report.challenge_field_pinned
        }),
        json!({
            "case_id": "fri_qrom_checked_core_budget_import",
            "kind": "audit",
            "surface": "Path2AccountStarkFriQromResidualReportV1.checked_core_qrom_budget",
            "checked_core_qrom_artifacts": report.checked_core_qrom_artifacts,
            "checked_core_qrom_algebraic_challenge_bits":
                report.checked_core_qrom_algebraic_challenge_bits,
            "checked_core_qrom_hardening_bits": report.checked_core_qrom_hardening_bits,
            "checked_core_qrom_total_transcript_bits":
                report.checked_core_qrom_total_transcript_bits,
            "checked_core_qrom_quantum_hash_query_bound_log2":
                report.checked_core_qrom_quantum_hash_query_bound_log2,
            "checked_core_qrom_classical_hash_query_bound_log2":
                report.checked_core_qrom_classical_hash_query_bound_log2,
            "checked_core_qrom_selected_multi_target_horizon_log2":
                report.checked_core_qrom_selected_multi_target_horizon_log2,
            "checked_core_qrom_selected_transcript_term_bits":
                report.checked_core_qrom_selected_transcript_term_bits,
            "checked_core_qrom_max_target_count_log2_at_128_bits":
                report.checked_core_qrom_max_target_count_log2_at_128_bits,
            "checked_core_qrom_budget_pinned": report.checked_core_qrom_budget_pinned,
            "scope_note":
                "checked-core spend transcript ledger only; account-STARK FRI transcript theorem remains open"
        }),
        json!({
            "case_id": "fri_qrom_conditional_bcs_budget",
            "kind": "audit",
            "surface": "Path2AccountStarkFriQromResidualReportV1.conditional_bcs_qrom_budget",
            "bcs_rom_error_shape": report.bcs_rom_error_shape,
            "bcs_qrom_error_shape": report.bcs_qrom_error_shape,
            "bcs_quantum_query_bound_log2": report.bcs_quantum_query_bound_log2,
            "bcs_multi_target_horizon_log2": report.bcs_multi_target_horizon_log2,
            "bcs_candidate_output_bits": report.bcs_candidate_output_bits,
            "bcs_rbr_schedule_label_bits": report.bcs_rbr_schedule_label_bits,
            "bcs_qrom_lift_constant_log2_bound":
                report.bcs_qrom_lift_constant_log2_bound,
            "bcs_rbr_term_bound_bits": report.bcs_rbr_term_bound_bits,
            "bcs_oracle_term_bound_bits": report.bcs_oracle_term_bound_bits,
            "bcs_dimensions_lean_theorem": report.bcs_dimensions_lean_theorem,
            "bcs_rbr_term_lean_theorem": report.bcs_rbr_term_lean_theorem,
            "bcs_oracle_term_lean_theorem": report.bcs_oracle_term_lean_theorem,
            "bcs_normalized_budget_lean_theorem": report.bcs_normalized_budget_lean_theorem,
            "bcs_dimensions_bound": report.bcs_dimensions_bound,
            "bcs_rbr_term_bound": report.bcs_rbr_term_bound,
            "bcs_oracle_term_bound": report.bcs_oracle_term_bound,
            "bcs_normalized_budget_q128_bound": report.bcs_normalized_budget_q128_bound,
            "scope_note":
                "conditional arithmetic only; RBR, challenger-output mapping, QROM lift constant, primitive replacement, and recursion remain open"
        }),
        json!({
            "case_id": "fri_qrom_published_rbr_single_leaf_budget",
            "kind": "audit",
            "surface": "Path2AccountStarkFriQromResidualReportV1.published_rbr_single_leaf_budget",
            "published_rbr_source": report.published_rbr_source,
            "published_rbr_error_shape": report.published_rbr_error_shape,
            "single_leaf_degree_bits": report.single_leaf_degree_bits,
            "single_leaf_fri_domain_log2": report.single_leaf_fri_domain_log2,
            "published_rbr_analysis_m": report.published_rbr_analysis_m,
            "published_rbr_delta_numerator": report.published_rbr_delta_numerator,
            "published_rbr_delta_denominator": report.published_rbr_delta_denominator,
            "published_rbr_field_term_bound_bits":
                report.published_rbr_field_term_bound_bits,
            "published_rbr_query_term_bound_bits":
                report.published_rbr_query_term_bound_bits,
            "published_rbr_dimensions_lean_theorem":
                report.published_rbr_dimensions_lean_theorem,
            "published_rbr_bound_lean_theorem": report.published_rbr_bound_lean_theorem,
            "adaptive_rbr_term_bound_bits": report.adaptive_rbr_term_bound_bits,
            "adaptive_oracle_term_bound_bits": report.adaptive_oracle_term_bound_bits,
            "adaptive_budget_lean_theorem": report.adaptive_budget_lean_theorem,
            "adaptive_budget_q128_bound": report.adaptive_budget_q128_bound,
            "adaptive_qrom_policy_adr": report.adaptive_qrom_policy_adr,
            "adaptive_qrom_policy_lean_theorem":
                report.adaptive_qrom_policy_lean_theorem,
            "adaptive_global_query_bound": report.adaptive_global_query_bound,
            "bcs_account_target_multiplier": report.bcs_account_target_multiplier,
            "lifetime_union_profile_selected": report.lifetime_union_profile_selected,
            "fs_c9_reference_native_fidelity_bound":
                report.fs_c9_reference_native_fidelity_bound,
            "fs_c9_production_migration_complete":
                report.fs_c9_production_migration_complete,
            "lifetime_field_budget_not_le_q128_lean_theorem":
                report.lifetime_field_budget_not_le_q128_lean_theorem,
            "lifetime_union_bound_q128_established":
                report.lifetime_union_bound_q128_established,
            "scope_note":
                "published-bound arithmetic only; theorem mapping and lifetime-target policy are not closed"
        }),
        json!({
            "case_id": "fri_qrom_open_theorem_flags_fail_closed",
            "kind": "audit",
            "surface": "Path2AccountStarkFriQromResidualReportV1.open_obligations",
            "account_stark_fri_qrom_transcript_reduction_lean_theorem":
                report.account_stark_fri_qrom_transcript_reduction_lean_theorem,
            "account_stark_fri_qrom_transcript_reduction_bound":
                report.account_stark_fri_qrom_transcript_reduction_bound,
            "bcs_challenger_output_mapping_instantiated":
                report.bcs_challenger_output_mapping_instantiated,
            "bcs_rbr_error_theorem_instantiated":
                report.bcs_rbr_error_theorem_instantiated,
            "bcs_qrom_lift_constant_reviewed": report.bcs_qrom_lift_constant_reviewed,
            "account_stark_transcript_qrom_theorem_instantiated":
                report.account_stark_transcript_qrom_theorem_instantiated,
            "recursion_pricing_complete": report.recursion_pricing_complete,
            "external_fri_review_complete": report.external_fri_review_complete,
            "external_qrom_review_complete": report.external_qrom_review_complete,
            "deployment_profile_accepted": report.deployment_profile_accepted,
            "residual_pricing_complete": report.residual_pricing_complete,
            "deployment_ready": report.deployment_ready,
            "blocker": report.blocker
        }),
    ];

    if cases.len() != report.qrom_domain_vector_case_ids.len() {
        return Err("FRI/QROM domain vector generator case count drifted".to_string());
    }
    for (case, expected) in cases.iter().zip(report.qrom_domain_vector_case_ids.iter()) {
        if case["case_id"] != *expected {
            return Err(format!(
                "FRI/QROM domain vector generator case id drift: expected {expected}"
            ));
        }
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountStarkFriQromDomainVectors.v1",
        "corpus_id": "Descender.Path2.AccountStarkFriQromDomainBoundaryVectors.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "status": "digest_pinned_fri_qrom_domain_audit_vectors",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_fri_qrom_domain_vectors",
            "source": "tools/parameter-search/src/bin/path2_account_stark_fri_qrom_domain_vectors.rs"
        },
        "boundary_report": {
            "domain": report.domain,
            "digest": hex_encode(&report.digest),
            "recomputed_digest": hex_encode(&recomputed_digest),
            "selected_proof_family": report.selected_proof_family,
            "selected_payload_type": report.selected_payload_type,
            "proof_system": report.proof_system,
            "deployment_ready": report.deployment_ready,
            "blocker": report.blocker
        },
        "audit_summary": {
            "fri_schedule_pinned": report.fri_schedule_pinned,
            "challenge_field_pinned": report.challenge_field_pinned,
            "checked_core_qrom_budget_pinned": report.checked_core_qrom_budget_pinned,
            "bcs_dimensions_bound": report.bcs_dimensions_bound,
            "bcs_rbr_term_bound": report.bcs_rbr_term_bound,
            "bcs_oracle_term_bound": report.bcs_oracle_term_bound,
            "bcs_normalized_budget_q128_bound": report.bcs_normalized_budget_q128_bound,
            "adaptive_budget_q128_bound": report.adaptive_budget_q128_bound,
            "adaptive_global_query_bound": report.adaptive_global_query_bound,
            "bcs_account_target_multiplier": report.bcs_account_target_multiplier,
            "lifetime_union_profile_selected": report.lifetime_union_profile_selected,
            "fs_c9_reference_native_fidelity_bound":
                report.fs_c9_reference_native_fidelity_bound,
            "fs_c9_production_migration_complete":
                report.fs_c9_production_migration_complete,
            "lifetime_union_bound_q128_established":
                report.lifetime_union_bound_q128_established,
            "bcs_challenger_output_mapping_instantiated":
                report.bcs_challenger_output_mapping_instantiated,
            "bcs_rbr_error_theorem_instantiated":
                report.bcs_rbr_error_theorem_instantiated,
            "bcs_qrom_lift_constant_reviewed": report.bcs_qrom_lift_constant_reviewed,
            "account_stark_fri_qrom_transcript_reduction_bound":
                report.account_stark_fri_qrom_transcript_reduction_bound,
            "account_stark_transcript_qrom_theorem_instantiated":
                report.account_stark_transcript_qrom_theorem_instantiated,
            "recursion_pricing_complete": report.recursion_pricing_complete,
            "external_fri_review_complete": report.external_fri_review_complete,
            "external_qrom_review_complete": report.external_qrom_review_complete,
            "deployment_profile_accepted": report.deployment_profile_accepted,
            "residual_pricing_complete": report.residual_pricing_complete,
            "deployment_ready": report.deployment_ready
        },
        "case_count": cases.len(),
        "cases": cases
    }))
}

fn main() -> Result<(), String> {
    let corpus = manifest_body()?;
    let corpus_json = serde_json::to_string_pretty(&corpus)
        .map_err(|err| format!("failed to serialize FRI/QROM domain corpus body: {err}"))?;
    let out = json!({
        "corpus_digest": hex_encode(&Sha256::digest(corpus_json.as_bytes())),
        "corpus": corpus
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&out)
            .map_err(|err| format!("failed to serialize FRI/QROM domain corpus: {err}"))?
    );
    Ok(())
}
