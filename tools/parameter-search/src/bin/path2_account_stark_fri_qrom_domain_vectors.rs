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
            "case_id": "fri_qrom_open_theorem_flags_fail_closed",
            "kind": "audit",
            "surface": "Path2AccountStarkFriQromResidualReportV1.open_obligations",
            "account_stark_fri_qrom_transcript_reduction_lean_theorem":
                report.account_stark_fri_qrom_transcript_reduction_lean_theorem,
            "account_stark_fri_qrom_transcript_reduction_bound":
                report.account_stark_fri_qrom_transcript_reduction_bound,
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
