use descender_parameter_search::production_reference::{
    path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_fri_qrom_residual_report_v1_digest,
    validate_path2_account_stark_fri_qrom_residual_report_v1,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("parameter-search crate lives under tools/")
        .to_path_buf()
}

fn load_text(root: &Path, relative_path: &str) -> Result<String, String> {
    let path = root.join(relative_path);
    fs::read_to_string(&path).map_err(|err| format!("read {}: {err}", path.display()))
}

fn load_json(root: &Path, relative_path: &str) -> Result<Value, String> {
    let text = load_text(root, relative_path)?;
    serde_json::from_str(&text).map_err(|err| format!("parse {relative_path}: {err}"))
}

fn at<'a>(value: &'a Value, pointer: &str) -> Result<&'a Value, String> {
    value
        .pointer(pointer)
        .ok_or_else(|| format!("missing JSON pointer {pointer}"))
}

fn str_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, String> {
    at(value, pointer)?
        .as_str()
        .ok_or_else(|| format!("JSON pointer {pointer} is not a string"))
}

fn bool_at(value: &Value, pointer: &str) -> Result<bool, String> {
    at(value, pointer)?
        .as_bool()
        .ok_or_else(|| format!("JSON pointer {pointer} is not a bool"))
}

fn u64_at(value: &Value, pointer: &str) -> Result<u64, String> {
    at(value, pointer)?
        .as_u64()
        .ok_or_else(|| format!("JSON pointer {pointer} is not an unsigned integer"))
}

fn string_vec_at(value: &Value, pointer: &str) -> Result<Vec<String>, String> {
    let array = at(value, pointer)?
        .as_array()
        .ok_or_else(|| format!("JSON pointer {pointer} is not an array"))?;
    array
        .iter()
        .enumerate()
        .map(|(index, item)| {
            item.as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| format!("{pointer}[{index}] is not a string"))
        })
        .collect()
}

fn require_u64(corpus: &Value, pointer: &str, expected: usize) -> Result<(), String> {
    let actual = u64_at(corpus, pointer)?;
    if actual == expected as u64 {
        Ok(())
    } else {
        Err(format!("{pointer}: got {actual}, expected {expected}"))
    }
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let corpus_path = "formal/kat/path2_account_stark_fri_qrom_domain_vectors_v1.json";
    let corpus_file = load_json(&root, corpus_path)?;
    let corpus = at(&corpus_file, "/corpus")?;
    let corpus_json = serde_json::to_string_pretty(corpus)
        .map_err(|err| format!("serialize FRI/QROM domain corpus body: {err}"))?;
    let recomputed_digest = hex_encode(&Sha256::digest(corpus_json.as_bytes()));
    if str_at(&corpus_file, "/corpus_digest")? != recomputed_digest {
        return Err("FRI/QROM domain vector corpus digest mismatch".to_string());
    }

    let report = path2_account_stark_fri_qrom_residual_report_v1();
    validate_path2_account_stark_fri_qrom_residual_report_v1(&report)?;
    let report_digest = path2_account_stark_fri_qrom_residual_report_v1_digest(&report);
    if report_digest != report.digest {
        return Err("account-STARK FRI/QROM residual report digest mismatch".to_string());
    }

    if str_at(corpus, "/schema")? != "Descender.Path2.AccountStarkFriQromDomainVectors.v1" {
        return Err("FRI/QROM domain vector corpus schema mismatch".to_string());
    }
    if str_at(corpus, "/status")? != "digest_pinned_fri_qrom_domain_audit_vectors" {
        return Err("FRI/QROM domain vector corpus status mismatch".to_string());
    }
    if str_at(corpus, "/boundary_report/digest")? != hex_encode(&report.digest)
        || str_at(corpus, "/boundary_report/recomputed_digest")? != hex_encode(&report_digest)
    {
        return Err("FRI/QROM domain vector corpus is not bound to residual report".to_string());
    }
    if bool_at(corpus, "/boundary_report/deployment_ready")?
        || !bool_at(
            corpus,
            "/audit_summary/account_stark_fri_qrom_transcript_reduction_bound",
        )?
        || bool_at(
            corpus,
            "/audit_summary/account_stark_transcript_qrom_theorem_instantiated",
        )?
        || bool_at(corpus, "/audit_summary/recursion_pricing_complete")?
        || bool_at(corpus, "/audit_summary/external_fri_review_complete")?
        || bool_at(corpus, "/audit_summary/external_qrom_review_complete")?
        || bool_at(corpus, "/audit_summary/deployment_profile_accepted")?
        || bool_at(corpus, "/audit_summary/residual_pricing_complete")?
        || bool_at(corpus, "/audit_summary/deployment_ready")?
        || bool_at(
            corpus,
            "/audit_summary/bcs_challenger_output_mapping_instantiated",
        )?
        || bool_at(corpus, "/audit_summary/bcs_rbr_error_theorem_instantiated")?
        || bool_at(corpus, "/audit_summary/bcs_qrom_lift_constant_reviewed")?
        || bool_at(
            corpus,
            "/audit_summary/lifetime_union_bound_q128_established",
        )?
        || bool_at(corpus, "/audit_summary/lifetime_union_profile_selected")?
        || bool_at(corpus, "/audit_summary/fs_c9_production_migration_complete")?
    {
        return Err("FRI/QROM domain vector corpus unexpectedly closes theorem status".to_string());
    }
    if !bool_at(corpus, "/audit_summary/fri_schedule_pinned")?
        || !bool_at(corpus, "/audit_summary/challenge_field_pinned")?
        || !bool_at(corpus, "/audit_summary/checked_core_qrom_budget_pinned")?
        || !bool_at(corpus, "/audit_summary/bcs_dimensions_bound")?
        || !bool_at(corpus, "/audit_summary/bcs_rbr_term_bound")?
        || !bool_at(corpus, "/audit_summary/bcs_oracle_term_bound")?
        || !bool_at(corpus, "/audit_summary/bcs_normalized_budget_q128_bound")?
        || !bool_at(corpus, "/audit_summary/adaptive_budget_q128_bound")?
        || !bool_at(corpus, "/audit_summary/adaptive_global_query_bound")?
        || !bool_at(
            corpus,
            "/audit_summary/fs_c9_reference_native_fidelity_bound",
        )?
    {
        return Err("FRI/QROM domain vector corpus lost pinned-domain flags".to_string());
    }

    if u64_at(corpus, "/case_count")? != report.qrom_domain_vector_case_count as u64 {
        return Err("FRI/QROM domain vector corpus case count drifted".to_string());
    }
    let cases = at(corpus, "/cases")?
        .as_array()
        .ok_or_else(|| "FRI/QROM domain corpus cases are not an array".to_string())?;
    if cases.len() != report.qrom_domain_vector_case_ids.len() {
        return Err("FRI/QROM domain vector corpus cases array drifted".to_string());
    }
    for (index, (case, expected)) in cases
        .iter()
        .zip(report.qrom_domain_vector_case_ids.iter())
        .enumerate()
    {
        let case_id = case
            .get("case_id")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("FRI/QROM domain case {index} has no case_id"))?;
        if case_id != *expected {
            return Err(format!(
                "FRI/QROM domain vector case {index} id {case_id:?} != {expected:?}"
            ));
        }
        if case.get("kind").and_then(Value::as_str) != Some("audit") {
            return Err(format!(
                "FRI/QROM domain vector case {case_id} is not an audit case"
            ));
        }
    }

    require_u64(&cases[1], "/poseidon2_width", report.poseidon2_width)?;
    require_u64(
        &cases[1],
        "/poseidon2_sponge_floor_bits",
        report.poseidon2_sponge_floor_bits,
    )?;
    require_u64(&cases[1], "/fs_current_rate", report.fs_challenger_rate)?;
    require_u64(
        &cases[1],
        "/fs_current_capacity",
        report.fs_challenger_capacity,
    )?;
    require_u64(&cases[1], "/fs_target_rate", report.fs_target_rate)?;
    require_u64(&cases[1], "/fs_target_capacity", report.fs_target_capacity)?;
    require_u64(
        &cases[1],
        "/fs_target_generic_collision_bits_floor",
        report.fs_target_generic_collision_bits_floor,
    )?;
    if str_at(&cases[1], "/fs_c9_reference_source")? != report.fs_c9_reference_source
        || str_at(&cases[1], "/fs_c9_reference_native_fidelity_test")?
            != report.fs_c9_reference_native_fidelity_test
        || !bool_at(&cases[1], "/fs_c9_reference_native_fidelity_bound")?
        || bool_at(&cases[1], "/fs_c9_production_migration_complete")?
    {
        return Err("FRI/QROM capacity-9 migration evidence drifted".to_string());
    }
    if !bool_at(&cases[1], "/sponge_floor_supports_current_124_bit_floor")? {
        return Err("FRI/QROM domain vector corpus lost Poseidon2 sponge floor".to_string());
    }
    require_u64(&cases[2], "/fri_log_blowup", report.fri_log_blowup)?;
    require_u64(&cases[2], "/fri_num_queries", report.fri_num_queries)?;
    require_u64(&cases[2], "/fri_query_pow_bits", report.fri_query_pow_bits)?;
    require_u64(
        &cases[2],
        "/fri_commit_pow_bits",
        report.fri_commit_pow_bits,
    )?;
    require_u64(
        &cases[2],
        "/fri_conservative_johnson_bits",
        report.fri_conservative_johnson_bits,
    )?;
    require_u64(
        &cases[2],
        "/fri_optimistic_capacity_bits",
        report.fri_optimistic_capacity_bits,
    )?;
    require_u64(
        &cases[2],
        "/challenge_extension_degree",
        report.challenge_extension_degree,
    )?;
    require_u64(
        &cases[2],
        "/challenge_field_bits_floor",
        report.challenge_field_bits_floor,
    )?;
    if !bool_at(&cases[2], "/fri_schedule_pinned")?
        || !bool_at(&cases[2], "/challenge_field_pinned")?
    {
        return Err("FRI/QROM challenge schedule case lost pinned flags".to_string());
    }

    let artifacts = string_vec_at(&cases[3], "/checked_core_qrom_artifacts")?;
    let expected_artifacts: Vec<String> = report
        .checked_core_qrom_artifacts
        .iter()
        .map(|item| (*item).to_string())
        .collect();
    if artifacts != expected_artifacts {
        return Err("FRI/QROM domain vector corpus QROM artifact list drifted".to_string());
    }
    require_u64(
        &cases[3],
        "/checked_core_qrom_algebraic_challenge_bits",
        report.checked_core_qrom_algebraic_challenge_bits,
    )?;
    require_u64(
        &cases[3],
        "/checked_core_qrom_hardening_bits",
        report.checked_core_qrom_hardening_bits,
    )?;
    require_u64(
        &cases[3],
        "/checked_core_qrom_total_transcript_bits",
        report.checked_core_qrom_total_transcript_bits,
    )?;
    require_u64(
        &cases[3],
        "/checked_core_qrom_quantum_hash_query_bound_log2",
        report.checked_core_qrom_quantum_hash_query_bound_log2,
    )?;
    require_u64(
        &cases[3],
        "/checked_core_qrom_classical_hash_query_bound_log2",
        report.checked_core_qrom_classical_hash_query_bound_log2,
    )?;
    require_u64(
        &cases[3],
        "/checked_core_qrom_selected_multi_target_horizon_log2",
        report.checked_core_qrom_selected_multi_target_horizon_log2,
    )?;
    require_u64(
        &cases[3],
        "/checked_core_qrom_selected_transcript_term_bits",
        report.checked_core_qrom_selected_transcript_term_bits,
    )?;
    require_u64(
        &cases[3],
        "/checked_core_qrom_max_target_count_log2_at_128_bits",
        report.checked_core_qrom_max_target_count_log2_at_128_bits,
    )?;
    if !bool_at(&cases[3], "/checked_core_qrom_budget_pinned")? {
        return Err("FRI/QROM checked-core budget case lost pinned flag".to_string());
    }

    require_u64(
        &cases[4],
        "/bcs_quantum_query_bound_log2",
        report.bcs_quantum_query_bound_log2,
    )?;
    require_u64(
        &cases[4],
        "/bcs_multi_target_horizon_log2",
        report.bcs_multi_target_horizon_log2,
    )?;
    require_u64(
        &cases[4],
        "/bcs_candidate_output_bits",
        report.bcs_candidate_output_bits,
    )?;
    require_u64(
        &cases[4],
        "/bcs_rbr_schedule_label_bits",
        report.bcs_rbr_schedule_label_bits,
    )?;
    require_u64(
        &cases[4],
        "/bcs_qrom_lift_constant_log2_bound",
        report.bcs_qrom_lift_constant_log2_bound,
    )?;
    require_u64(
        &cases[4],
        "/bcs_rbr_term_bound_bits",
        report.bcs_rbr_term_bound_bits,
    )?;
    require_u64(
        &cases[4],
        "/bcs_oracle_term_bound_bits",
        report.bcs_oracle_term_bound_bits,
    )?;
    for (pointer, expected) in [
        (
            "/bcs_dimensions_lean_theorem",
            report.bcs_dimensions_lean_theorem,
        ),
        (
            "/bcs_rbr_term_lean_theorem",
            report.bcs_rbr_term_lean_theorem,
        ),
        (
            "/bcs_oracle_term_lean_theorem",
            report.bcs_oracle_term_lean_theorem,
        ),
        (
            "/bcs_normalized_budget_lean_theorem",
            report.bcs_normalized_budget_lean_theorem,
        ),
    ] {
        if str_at(&cases[4], pointer)? != expected {
            return Err(format!(
                "FRI/QROM conditional BCS theorem drifted at {pointer}"
            ));
        }
    }
    for pointer in [
        "/bcs_dimensions_bound",
        "/bcs_rbr_term_bound",
        "/bcs_oracle_term_bound",
        "/bcs_normalized_budget_q128_bound",
    ] {
        if !bool_at(&cases[4], pointer)? {
            return Err(format!("FRI/QROM conditional BCS bound lost at {pointer}"));
        }
    }

    require_u64(
        &cases[5],
        "/single_leaf_degree_bits",
        report.single_leaf_degree_bits,
    )?;
    require_u64(
        &cases[5],
        "/single_leaf_fri_domain_log2",
        report.single_leaf_fri_domain_log2,
    )?;
    require_u64(
        &cases[5],
        "/published_rbr_analysis_m",
        report.published_rbr_analysis_m,
    )?;
    require_u64(
        &cases[5],
        "/published_rbr_delta_numerator",
        report.published_rbr_delta_numerator,
    )?;
    require_u64(
        &cases[5],
        "/published_rbr_delta_denominator",
        report.published_rbr_delta_denominator,
    )?;
    require_u64(
        &cases[5],
        "/published_rbr_field_term_bound_bits",
        report.published_rbr_field_term_bound_bits,
    )?;
    require_u64(
        &cases[5],
        "/published_rbr_query_term_bound_bits",
        report.published_rbr_query_term_bound_bits,
    )?;
    require_u64(
        &cases[5],
        "/adaptive_rbr_term_bound_bits",
        report.adaptive_rbr_term_bound_bits,
    )?;
    require_u64(
        &cases[5],
        "/adaptive_oracle_term_bound_bits",
        report.adaptive_oracle_term_bound_bits,
    )?;
    if str_at(&cases[5], "/published_rbr_source")? != report.published_rbr_source
        || str_at(&cases[5], "/published_rbr_error_shape")? != report.published_rbr_error_shape
        || str_at(&cases[5], "/published_rbr_dimensions_lean_theorem")?
            != report.published_rbr_dimensions_lean_theorem
        || str_at(&cases[5], "/published_rbr_bound_lean_theorem")?
            != report.published_rbr_bound_lean_theorem
        || str_at(&cases[5], "/adaptive_budget_lean_theorem")?
            != report.adaptive_budget_lean_theorem
        || str_at(&cases[5], "/adaptive_qrom_policy_adr")? != report.adaptive_qrom_policy_adr
        || str_at(&cases[5], "/adaptive_qrom_policy_lean_theorem")?
            != report.adaptive_qrom_policy_lean_theorem
        || str_at(&cases[5], "/lifetime_field_budget_not_le_q128_lean_theorem")?
            != report.lifetime_field_budget_not_le_q128_lean_theorem
        || !bool_at(&cases[5], "/adaptive_budget_q128_bound")?
        || !bool_at(&cases[5], "/adaptive_global_query_bound")?
        || u64_at(&cases[5], "/bcs_account_target_multiplier")?
            != report.bcs_account_target_multiplier as u64
        || bool_at(&cases[5], "/lifetime_union_profile_selected")?
        || bool_at(&cases[5], "/lifetime_union_bound_q128_established")?
    {
        return Err("FRI/QROM published-RBR candidate case drifted".to_string());
    }

    if bool_at(
        &cases[6],
        "/account_stark_transcript_qrom_theorem_instantiated",
    )? || bool_at(&cases[6], "/recursion_pricing_complete")?
        || bool_at(&cases[6], "/bcs_challenger_output_mapping_instantiated")?
        || bool_at(&cases[6], "/bcs_rbr_error_theorem_instantiated")?
        || bool_at(&cases[6], "/bcs_qrom_lift_constant_reviewed")?
        || bool_at(&cases[6], "/external_fri_review_complete")?
        || bool_at(&cases[6], "/external_qrom_review_complete")?
        || bool_at(&cases[6], "/deployment_profile_accepted")?
        || bool_at(&cases[6], "/residual_pricing_complete")?
        || bool_at(&cases[6], "/deployment_ready")?
    {
        return Err("FRI/QROM open-theorem case unexpectedly closes obligations".to_string());
    }
    if str_at(
        &cases[6],
        "/account_stark_fri_qrom_transcript_reduction_lean_theorem",
    )? != report.account_stark_fri_qrom_transcript_reduction_lean_theorem
        || !bool_at(
            &cases[6],
            "/account_stark_fri_qrom_transcript_reduction_bound",
        )?
    {
        return Err("FRI/QROM open-theorem case lost transcript reduction hook".to_string());
    }

    println!("Path 2 account-STARK FRI/QROM domain vector consistency check passed.");
    Ok(())
}
