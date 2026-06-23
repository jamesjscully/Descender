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
        || bool_at(corpus, "/audit_summary/account_stark_transcript_qrom_theorem_instantiated")?
        || bool_at(corpus, "/audit_summary/recursion_pricing_complete")?
        || bool_at(corpus, "/audit_summary/external_fri_review_complete")?
        || bool_at(corpus, "/audit_summary/external_qrom_review_complete")?
        || bool_at(corpus, "/audit_summary/deployment_profile_accepted")?
        || bool_at(corpus, "/audit_summary/residual_pricing_complete")?
        || bool_at(corpus, "/audit_summary/deployment_ready")?
    {
        return Err("FRI/QROM domain vector corpus unexpectedly closes theorem status".to_string());
    }
    if !bool_at(corpus, "/audit_summary/fri_schedule_pinned")?
        || !bool_at(corpus, "/audit_summary/challenge_field_pinned")?
        || !bool_at(corpus, "/audit_summary/checked_core_qrom_budget_pinned")?
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
            return Err(format!("FRI/QROM domain vector case {case_id} is not an audit case"));
        }
    }

    require_u64(&cases[1], "/poseidon2_width", report.poseidon2_width)?;
    require_u64(
        &cases[1],
        "/poseidon2_sponge_floor_bits",
        report.poseidon2_sponge_floor_bits,
    )?;
    if !bool_at(&cases[1], "/sponge_floor_supports_current_124_bit_floor")? {
        return Err("FRI/QROM domain vector corpus lost Poseidon2 sponge floor".to_string());
    }
    require_u64(&cases[2], "/fri_log_blowup", report.fri_log_blowup)?;
    require_u64(&cases[2], "/fri_num_queries", report.fri_num_queries)?;
    require_u64(&cases[2], "/fri_query_pow_bits", report.fri_query_pow_bits)?;
    require_u64(&cases[2], "/fri_commit_pow_bits", report.fri_commit_pow_bits)?;
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

    if bool_at(&cases[4], "/account_stark_transcript_qrom_theorem_instantiated")?
        || bool_at(&cases[4], "/recursion_pricing_complete")?
        || bool_at(&cases[4], "/external_fri_review_complete")?
        || bool_at(&cases[4], "/external_qrom_review_complete")?
        || bool_at(&cases[4], "/deployment_profile_accepted")?
        || bool_at(&cases[4], "/residual_pricing_complete")?
        || bool_at(&cases[4], "/deployment_ready")?
    {
        return Err("FRI/QROM open-theorem case unexpectedly closes obligations".to_string());
    }

    println!("Path 2 account-STARK FRI/QROM domain vector consistency check passed.");
    Ok(())
}
