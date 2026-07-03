use std::{
    env, fs,
    path::{Path, PathBuf},
};

use descender_parameter_search::production_reference::{
    path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_fri_qrom_residual_report_v1_digest,
    validate_path2_account_stark_fri_qrom_residual_report_v1,
};
use serde_json::Value;

fn require_contains(source: &str, needle: &str, label: &str) -> Result<(), String> {
    if source.contains(needle) {
        Ok(())
    } else {
        Err(format!("account-STARK config is missing {label}: {needle}"))
    }
}

fn repo_root() -> Result<PathBuf, String> {
    let cwd = env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let mut candidates = Vec::new();
    candidates.push(cwd.clone());
    if let Some(parent) = cwd.parent() {
        candidates.push(parent.to_path_buf());
        if let Some(grandparent) = parent.parent() {
            candidates.push(grandparent.to_path_buf());
        }
    }

    for candidate in candidates {
        if candidate.join("spec/README.md").exists()
            && candidate
                .join("tools/descender-account-stark/src/config.rs")
                .exists()
        {
            return Ok(candidate);
        }
    }
    Err("failed to locate Descender repository root".to_string())
}

fn load_json(root: &Path, path: &str) -> Result<Value, String> {
    let bytes = fs::read(root.join(path)).map_err(|err| format!("failed to read {path}: {err}"))?;
    serde_json::from_slice(&bytes).map_err(|err| format!("failed to parse {path}: {err}"))
}

fn json_usize<'a>(value: &'a Value, path: &[&str]) -> Result<usize, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .ok_or_else(|| format!("missing JSON key {}", path.join(".")))?;
    }
    let value = cursor
        .as_u64()
        .ok_or_else(|| format!("JSON key {} is not an unsigned integer", path.join(".")))?;
    usize::try_from(value).map_err(|_| format!("JSON key {} does not fit usize", path.join(".")))
}

fn json_str<'a>(value: &'a Value, path: &[&str]) -> Result<&'a str, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .ok_or_else(|| format!("missing JSON key {}", path.join(".")))?;
    }
    cursor
        .as_str()
        .ok_or_else(|| format!("JSON key {} is not a string", path.join(".")))
}

fn json_bool(value: &Value, path: &[&str]) -> Result<bool, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .ok_or_else(|| format!("missing JSON key {}", path.join(".")))?;
    }
    cursor
        .as_bool()
        .ok_or_else(|| format!("JSON key {} is not a boolean", path.join(".")))
}

fn json_string_vec<'a>(value: &'a Value, path: &[&str]) -> Result<Vec<&'a str>, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .ok_or_else(|| format!("missing JSON key {}", path.join(".")))?;
    }
    let array = cursor
        .as_array()
        .ok_or_else(|| format!("JSON key {} is not an array", path.join(".")))?;
    array
        .iter()
        .enumerate()
        .map(|(index, item)| {
            item.as_str()
                .ok_or_else(|| format!("JSON key {}[{index}] is not a string", path.join(".")))
        })
        .collect()
}

fn check_equal<T: Eq + std::fmt::Debug>(label: &str, actual: T, expected: T) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{label}: got {actual:?}, expected {expected:?}"))
    }
}

fn main() -> Result<(), String> {
    let root = repo_root()?;
    let report = path2_account_stark_fri_qrom_residual_report_v1();
    validate_path2_account_stark_fri_qrom_residual_report_v1(&report)?;
    if path2_account_stark_fri_qrom_residual_report_v1_digest(&report) != report.digest {
        return Err("account-STARK FRI/QROM residual report digest mismatch".to_string());
    }
    if report.residual_pricing_complete
        || report.deployment_ready
        || !report.account_stark_fri_qrom_transcript_reduction_bound
    {
        return Err("FRI/QROM residual lost fail-closed reduction/open-boundary state".to_string());
    }

    let config_source =
        fs::read_to_string(root.join(report.account_stark_config_source)).map_err(|err| {
            format!(
                "failed to read account-STARK config {}: {err}",
                report.account_stark_config_source
            )
        })?;
    require_contains(
        &config_source,
        &format!("pub const WIDTH: usize = {};", report.poseidon2_width),
        "Poseidon2 width",
    )?;
    require_contains(
        &config_source,
        &format!(
            "pub const PROD_LOG_BLOWUP: usize = {};",
            report.fri_log_blowup
        ),
        "FRI log_blowup",
    )?;
    require_contains(
        &config_source,
        &format!(
            "pub const PROD_NUM_QUERIES: usize = {};",
            report.fri_num_queries
        ),
        "FRI num_queries",
    )?;
    require_contains(
        &config_source,
        &format!(
            "pub const PROD_QUERY_POW_BITS: usize = {};",
            report.fri_query_pow_bits
        ),
        "FRI query proof-of-work bits",
    )?;
    require_contains(
        &config_source,
        &format!(
            "pub const PROD_COMMIT_POW_BITS: usize = {};",
            report.fri_commit_pow_bits
        ),
        "FRI commit proof-of-work bits",
    )?;
    require_contains(
        &config_source,
        "pub type Challenge = BinomialExtensionField<Val, 8>;",
        "degree-8 challenge field",
    )?;

    check_equal(
        "conservative Johnson FRI bits",
        report.fri_num_queries * (report.fri_log_blowup / 2) + report.fri_query_pow_bits,
        report.fri_conservative_johnson_bits,
    )?;
    check_equal(
        "optimistic capacity FRI bits",
        report.fri_num_queries * report.fri_log_blowup + report.fri_query_pow_bits,
        report.fri_optimistic_capacity_bits,
    )?;

    let manifest = load_json(&root, "formal/kat/path2_account_stark_fri_qrom_residual_v1.json")?;
    check_equal(
        "manifest transcript reduction theorem",
        json_str(
            &manifest,
            &[
                "manifest",
                "authority_snapshot",
                "account_stark_fri_qrom_transcript_reduction_lean_theorem",
            ],
        )?,
        report.account_stark_fri_qrom_transcript_reduction_lean_theorem,
    )?;
    check_equal(
        "manifest open transcript reduction bound",
        json_bool(
            &manifest,
            &[
                "manifest",
                "open_obligations",
                "account_stark_fri_qrom_transcript_reduction_bound",
            ],
        )?,
        report.account_stark_fri_qrom_transcript_reduction_bound,
    )?;
    check_equal(
        "FRI/QROM domain vector case count",
        json_usize(
            &manifest,
            &[
                "manifest",
                "checked_core_qrom_budget",
                "qrom_domain_vector_case_count",
            ],
        )?,
        report.qrom_domain_vector_case_count,
    )?;
    check_equal(
        "FRI/QROM domain vector case ids",
        json_string_vec(
            &manifest,
            &[
                "manifest",
                "checked_core_qrom_budget",
                "qrom_domain_vector_case_ids",
            ],
        )?,
        report.qrom_domain_vector_case_ids.to_vec(),
    )?;

    let memo = load_json(&root, report.checked_core_qrom_artifacts[0])?;
    let review = load_json(&root, report.checked_core_qrom_artifacts[1])?;
    let frame_probe = load_json(&root, report.checked_core_qrom_artifacts[2])?;
    let multitarget = load_json(&root, report.checked_core_qrom_artifacts[3])?;

    check_equal(
        "QROM memo algebraic challenge bits",
        json_usize(&memo, &["challenge_derivation", "algebraic_challenge_bits"])?,
        report.checked_core_qrom_algebraic_challenge_bits,
    )?;
    check_equal(
        "QROM memo hardening bits",
        json_usize(&memo, &["challenge_derivation", "hardening_bits"])?,
        report.checked_core_qrom_hardening_bits,
    )?;
    check_equal(
        "QROM memo total transcript bits",
        json_usize(
            &memo,
            &["challenge_derivation", "total_transcript_challenge_bits"],
        )?,
        report.checked_core_qrom_total_transcript_bits,
    )?;
    check_equal(
        "QROM memo quantum query bound",
        json_usize(
            &memo,
            &["query_budget", "deployment_quantum_hash_query_bound_log2"],
        )?,
        report.checked_core_qrom_quantum_hash_query_bound_log2,
    )?;
    check_equal(
        "QROM memo classical query bound",
        json_usize(
            &memo,
            &["query_budget", "deployment_classical_hash_query_bound_log2"],
        )?,
        report.checked_core_qrom_classical_hash_query_bound_log2,
    )?;
    check_equal(
        "QROM memo selected target horizon",
        json_usize(
            &memo,
            &["query_budget", "selected_multi_target_horizon_log2"],
        )?,
        report.checked_core_qrom_selected_multi_target_horizon_log2,
    )?;
    check_equal(
        "QROM memo selected transcript term",
        json_usize(
            &memo,
            &["query_budget", "transcript_term_bits_after_qrom_loss"],
        )?,
        report.checked_core_qrom_selected_transcript_term_bits,
    )?;
    check_equal(
        "QROM review status",
        json_str(&review, &["status"])?,
        "checks_passed",
    )?;
    check_equal(
        "QROM review selected transcript term",
        json_usize(&review, &["derived_terms", "selected_transcript_term_bits"])?,
        report.checked_core_qrom_selected_transcript_term_bits,
    )?;
    check_equal(
        "QROM frame probe status",
        json_str(&frame_probe, &["status"])?,
        "checks_passed",
    )?;
    check_equal(
        "QROM frame probe max target cap",
        json_usize(
            &frame_probe,
            &["query_budget", "max_target_count_log2_at_128_bits"],
        )?,
        report.checked_core_qrom_max_target_count_log2_at_128_bits,
    )?;
    check_equal(
        "QROM multi-target status",
        json_str(&multitarget, &["status"])?,
        "closed_by_fail_closed_deployment_cap",
    )?;
    check_equal(
        "QROM multi-target fail-closed cap",
        json_usize(
            &multitarget,
            &["fail_closed_limit", "maximum_covered_target_count_log2"],
        )?,
        report.checked_core_qrom_max_target_count_log2_at_128_bits,
    )?;

    let source = fs::read_to_string(root.join("tools/parameter-search/src/production_reference.rs"))
        .map_err(|err| format!("failed to read production reference source: {err}"))?;
    for marker in [
        "PATH2_ACCOUNT_STARK_FRI_QROM_TRANSCRIPT_REDUCTION_LEAN_THEOREM",
        "account_stark_fri_qrom_transcript_reduction_lean_theorem",
        "account_stark_fri_qrom_transcript_reduction_bound",
    ] {
        require_contains(&source, marker, marker)?;
    }

    let lean = fs::read_to_string(root.join("Descender/Path2FullSpendConcrete.lean"))
        .map_err(|err| format!("failed to read Path2FullSpendConcrete.lean: {err}"))?;
    for marker in [
        "SelectedRustAcceptedBoundaryFriQromTranscriptWith",
        "selectedRustAcceptedBoundaryFriQromTranscriptReduction_v1",
    ] {
        require_contains(&lean, marker, marker)?;
    }

    println!("Path 2 account-STARK FRI/QROM residual consistency check passed.");
    Ok(())
}
