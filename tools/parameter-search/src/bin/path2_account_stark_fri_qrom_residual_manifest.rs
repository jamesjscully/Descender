use descender_parameter_search::production_reference::{
    PROFILE, path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_fri_qrom_residual_report_v1_digest,
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
    let recomputed_digest = path2_account_stark_fri_qrom_residual_report_v1_digest(&report);
    if recomputed_digest != report.digest {
        return Err("account-STARK FRI/QROM residual report digest mismatch".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountStarkFriQromResidualManifest.v1",
        "corpus_id": "Descender.Path2.AccountStarkFriQromResidual.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_residual_pricing_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_fri_qrom_residual_manifest",
            "source": "tools/parameter-search/src/bin/path2_account_stark_fri_qrom_residual_manifest.rs"
        },
        "authority_snapshot": {
            "fri_qrom_residual_report_domain": report.domain,
            "selected_proof_family": report.selected_proof_family,
            "selected_payload_type": report.selected_payload_type,
            "account_stark_fri_qrom_transcript_reduction_lean_theorem":
                report.account_stark_fri_qrom_transcript_reduction_lean_theorem,
            "blocker": report.blocker
        },
        "digests": {
            "account_stark_fri_qrom_residual_report_digest": hex_encode(&report.digest),
            "account_stark_fri_qrom_residual_report_recomputed_digest":
                hex_encode(&recomputed_digest)
        },
        "fri_schedule": {
            "config_source": report.account_stark_config_source,
            "proof_system": report.proof_system,
            "poseidon2_width": report.poseidon2_width,
            "log_blowup": report.fri_log_blowup,
            "num_queries": report.fri_num_queries,
            "query_pow_bits": report.fri_query_pow_bits,
            "commit_pow_bits": report.fri_commit_pow_bits,
            "conservative_johnson_bits": report.fri_conservative_johnson_bits,
            "optimistic_capacity_bits": report.fri_optimistic_capacity_bits,
            "challenge_extension_degree": report.challenge_extension_degree,
            "challenge_field_bits_floor": report.challenge_field_bits_floor,
            "poseidon2_sponge_floor_bits": report.poseidon2_sponge_floor_bits
        },
        "fiat_shamir_challenger": {
            "rate": report.fs_challenger_rate,
            "capacity": report.fs_challenger_capacity,
            "capacity_bits_floor": report.fs_challenger_capacity_bits_floor,
            "generic_collision_bits_floor":
                report.fs_challenger_generic_collision_bits_floor,
            "target_rate": report.fs_target_rate,
            "target_capacity": report.fs_target_capacity,
            "target_generic_collision_bits_floor":
                report.fs_target_generic_collision_bits_floor,
            "c9_reference_source": report.fs_c9_reference_source,
            "c9_reference_native_fidelity_test":
                report.fs_c9_reference_native_fidelity_test,
            "c9_reference_native_fidelity_bound":
                report.fs_c9_reference_native_fidelity_bound,
            "c9_production_migration_complete":
                report.fs_c9_production_migration_complete,
            "scope_note":
                "the capacity floor is not the BCS random-oracle output mapping; that mapping remains an explicit open theorem input"
        },
        "conditional_bcs_qrom_budget": {
            "rom_error_shape": report.bcs_rom_error_shape,
            "qrom_error_shape": report.bcs_qrom_error_shape,
            "quantum_query_bound_log2": report.bcs_quantum_query_bound_log2,
            "multi_target_horizon_log2": report.bcs_multi_target_horizon_log2,
            "candidate_output_bits": report.bcs_candidate_output_bits,
            "rbr_schedule_label_bits": report.bcs_rbr_schedule_label_bits,
            "qrom_lift_constant_log2_bound":
                report.bcs_qrom_lift_constant_log2_bound,
            "rbr_term_bound_bits": report.bcs_rbr_term_bound_bits,
            "oracle_term_bound_bits": report.bcs_oracle_term_bound_bits,
            "dimensions_lean_theorem": report.bcs_dimensions_lean_theorem,
            "rbr_term_lean_theorem": report.bcs_rbr_term_lean_theorem,
            "oracle_term_lean_theorem": report.bcs_oracle_term_lean_theorem,
            "normalized_budget_lean_theorem": report.bcs_normalized_budget_lean_theorem,
            "dimensions_bound": report.bcs_dimensions_bound,
            "rbr_term_bound": report.bcs_rbr_term_bound,
            "oracle_term_bound": report.bcs_oracle_term_bound,
            "normalized_budget_q128_bound": report.bcs_normalized_budget_q128_bound,
            "scope_note":
                "local arithmetic only; requires the selected RBR theorem, an effective kappa mapping for every relevant challenger output, and a reviewed bound on the QROM lift constant"
        },
        "published_rbr_single_leaf_budget": {
            "source": report.published_rbr_source,
            "error_shape": report.published_rbr_error_shape,
            "single_leaf_degree_bits": report.single_leaf_degree_bits,
            "single_leaf_fri_domain_log2": report.single_leaf_fri_domain_log2,
            "analysis_m": report.published_rbr_analysis_m,
            "delta_numerator": report.published_rbr_delta_numerator,
            "delta_denominator": report.published_rbr_delta_denominator,
            "field_term_bound_bits": report.published_rbr_field_term_bound_bits,
            "query_term_bound_bits": report.published_rbr_query_term_bound_bits,
            "dimensions_lean_theorem": report.published_rbr_dimensions_lean_theorem,
            "bound_lean_theorem": report.published_rbr_bound_lean_theorem,
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
                "exact arithmetic for the selected single-leaf domain; Plonky3-to-RBR mapping, challenger/QRO replacement, QROM constant review, and the lifetime-target policy remain open"
        },
        "checked_core_qrom_budget": {
            "source_artifacts": report.checked_core_qrom_artifacts,
            "qrom_domain_vector_case_ids": report.qrom_domain_vector_case_ids,
            "qrom_domain_vector_case_count": report.qrom_domain_vector_case_count,
            "algebraic_challenge_bits": report.checked_core_qrom_algebraic_challenge_bits,
            "hardening_bits": report.checked_core_qrom_hardening_bits,
            "total_transcript_bits": report.checked_core_qrom_total_transcript_bits,
            "quantum_hash_query_bound_log2":
                report.checked_core_qrom_quantum_hash_query_bound_log2,
            "classical_hash_query_bound_log2":
                report.checked_core_qrom_classical_hash_query_bound_log2,
            "selected_multi_target_horizon_log2":
                report.checked_core_qrom_selected_multi_target_horizon_log2,
            "selected_transcript_term_bits":
                report.checked_core_qrom_selected_transcript_term_bits,
            "max_target_count_log2_at_128_bits":
                report.checked_core_qrom_max_target_count_log2_at_128_bits,
            "scope_note":
                "checked-core spend transcript ledger; not a substitute for the open account-STARK FRI transcript QROM theorem"
        },
        "open_obligations": {
            "fri_schedule_pinned": report.fri_schedule_pinned,
            "challenge_field_pinned": report.challenge_field_pinned,
            "checked_core_qrom_budget_pinned": report.checked_core_qrom_budget_pinned,
            "account_stark_fri_qrom_transcript_reduction_bound":
                report.account_stark_fri_qrom_transcript_reduction_bound,
            "bcs_challenger_output_mapping_instantiated":
                report.bcs_challenger_output_mapping_instantiated,
            "bcs_rbr_error_theorem_instantiated":
                report.bcs_rbr_error_theorem_instantiated,
            "bcs_qrom_lift_constant_reviewed":
                report.bcs_qrom_lift_constant_reviewed,
            "adaptive_budget_q128_bound": report.adaptive_budget_q128_bound,
            "adaptive_global_query_bound": report.adaptive_global_query_bound,
            "bcs_account_target_multiplier": report.bcs_account_target_multiplier,
            "lifetime_union_profile_selected": report.lifetime_union_profile_selected,
            "lifetime_union_bound_q128_established":
                report.lifetime_union_bound_q128_established,
            "account_stark_transcript_qrom_theorem_instantiated":
                report.account_stark_transcript_qrom_theorem_instantiated,
            "recursion_pricing_complete": report.recursion_pricing_complete,
            "external_fri_review_complete": report.external_fri_review_complete,
            "external_qrom_review_complete": report.external_qrom_review_complete,
            "deployment_profile_accepted": report.deployment_profile_accepted,
            "residual_pricing_complete": report.residual_pricing_complete,
            "deployment_ready": report.deployment_ready
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
