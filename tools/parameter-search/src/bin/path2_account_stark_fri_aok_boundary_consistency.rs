use std::{
    env, fs,
    path::{Path, PathBuf},
};

use descender_parameter_search::production_reference::{
    path2_account_stark_fri_aok_boundary_report_v1,
    path2_account_stark_fri_aok_boundary_report_v1_digest,
    path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_residual_matrix_report_v1,
    validate_path2_account_stark_fri_aok_boundary_report_v1,
    validate_path2_account_stark_residual_matrix_report_v1,
};
use serde_json::Value;

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn require_contains(source: &str, needle: &str, label: &str) -> Result<(), String> {
    if source.contains(needle) {
        Ok(())
    } else {
        Err(format!("FRI/AoK boundary source is missing {label}: {needle}"))
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

fn json_usize(value: &Value, path: &[&str]) -> Result<usize, String> {
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

fn check_equal<T: Eq + std::fmt::Debug>(label: &str, actual: T, expected: T) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{label}: got {actual:?}, expected {expected:?}"))
    }
}

fn main() -> Result<(), String> {
    let root = repo_root()?;
    let report = path2_account_stark_fri_aok_boundary_report_v1();
    let fri_qrom_report = path2_account_stark_fri_qrom_residual_report_v1();
    let residual_report = path2_account_stark_residual_matrix_report_v1();
    validate_path2_account_stark_fri_aok_boundary_report_v1(&report)?;
    validate_path2_account_stark_residual_matrix_report_v1(&residual_report)?;
    if path2_account_stark_fri_aok_boundary_report_v1_digest(&report) != report.digest {
        return Err("account-STARK FRI/AoK boundary report digest mismatch".to_string());
    }
    if report.fri_qrom_residual_report_digest != fri_qrom_report.digest {
        return Err("FRI/AoK boundary report is not bound to FRI/QROM report".to_string());
    }
    if residual_report.fri_aok_boundary_report_digest != report.digest {
        return Err("residual matrix report is not bound to FRI/AoK report".to_string());
    }
    if report.fri_aok_extractor_theorem_instantiated
        || report.accepting_proof_implies_extractable_witness
        || report.external_extractor_review_complete
        || report.deployment_ready
    {
        return Err("FRI/AoK boundary unexpectedly reports closed obligations".to_string());
    }

    let manifest = load_json(
        &root,
        "formal/kat/path2_account_stark_fri_aok_boundary_v1.json",
    )?;
    check_equal(
        "manifest schema",
        json_str(&manifest, &["manifest", "schema"])?,
        "Descender.Path2.AccountStarkFriAokBoundaryManifest.v1",
    )?;
    check_equal(
        "manifest frozen status",
        json_str(&manifest, &["manifest", "frozen_status"])?,
        "draft_fri_aok_boundary_manifest",
    )?;
    check_equal(
        "manifest report digest",
        json_str(
            &manifest,
            &[
                "manifest",
                "digests",
                "account_stark_fri_aok_boundary_report_digest",
            ],
        )?,
        hex_encode(&report.digest).as_str(),
    )?;
    check_equal(
        "manifest recomputed report digest",
        json_str(
            &manifest,
            &[
                "manifest",
                "digests",
                "account_stark_fri_aok_boundary_report_recomputed_digest",
            ],
        )?,
        hex_encode(&report.digest).as_str(),
    )?;
    check_equal(
        "manifest FRI/QROM digest",
        json_str(
            &manifest,
            &[
                "manifest",
                "digests",
                "fri_aok_boundary_report_fri_qrom_digest",
            ],
        )?,
        hex_encode(&fri_qrom_report.digest).as_str(),
    )?;
    check_equal(
        "manifest residual matrix FRI/AoK digest",
        json_str(
            &manifest,
            &[
                "manifest",
                "digests",
                "residual_matrix_report_fri_aok_digest",
            ],
        )?,
        hex_encode(&report.digest).as_str(),
    )?;
    check_equal(
        "manifest support file",
        json_str(
            &manifest,
            &["manifest", "authority_snapshot", "support_file"],
        )?,
        report.support_file,
    )?;
    check_equal(
        "manifest named break",
        json_str(
            &manifest,
            &["manifest", "authority_snapshot", "named_break"],
        )?,
        report.named_break,
    )?;
    check_equal(
        "manifest theorem target",
        json_str(
            &manifest,
            &["manifest", "authority_snapshot", "extractor_theorem_target"],
        )?,
        report.extractor_theorem_target,
    )?;
    check_equal(
        "manifest relation class count",
        json_usize(
            &manifest,
            &[
                "manifest",
                "air_audit_evidence",
                "air_relation_class_count",
            ],
        )?,
        report.air_relation_class_count,
    )?;
    check_equal(
        "manifest MED remediation count",
        json_usize(
            &manifest,
            &["manifest", "air_audit_evidence", "med_findings_remediated"],
        )?,
        report.med_findings_remediated,
    )?;
    check_equal(
        "manifest open theorem flag",
        json_bool(
            &manifest,
            &[
                "manifest",
                "open_obligations",
                "fri_aok_extractor_theorem_instantiated",
            ],
        )?,
        false,
    )?;
    check_equal(
        "manifest deployment readiness",
        json_bool(&manifest, &["manifest", "open_obligations", "deployment_ready"])?,
        false,
    )?;

    let support = fs::read_to_string(root.join(report.support_file))
        .map_err(|err| format!("failed to read {}: {err}", report.support_file))?;
    require_contains(
        &support,
        "No CRITICAL \"false statement accepts\" hole in the AIR.",
        "no-critical-false-accept audit conclusion",
    )?;
    require_contains(&support, "all 8 relation classes", "AIR relation class coverage")?;
    require_contains(&support, "MED-1", "MED-1 remediation")?;
    require_contains(&support, "MED-2", "MED-2 remediation")?;
    require_contains(&support, "MED-3", "MED-3 remediation")?;
    require_contains(
        &support,
        "ALL THREE MED findings remediated",
        "MED remediation summary",
    )?;
    require_contains(
        &support,
        "FRI argument-of-knowledge",
        "FRI argument-of-knowledge note",
    )?;
    require_contains(&support, "p2_sis_extractor", "p2_sis_extractor obligation")?;
    require_contains(&support, "FRI-IOP extractor", "FRI-IOP extractor obligation")?;

    let config = fs::read_to_string(root.join(report.account_stark_config_source)).map_err(
        |err| {
            format!(
                "failed to read account-STARK config {}: {err}",
                report.account_stark_config_source
            )
        },
    )?;
    require_contains(
        &config,
        "pub const PROD_LOG_BLOWUP: usize = 4;",
        "FRI log_blowup",
    )?;
    require_contains(
        &config,
        "pub const PROD_NUM_QUERIES: usize = 56;",
        "FRI num_queries",
    )?;
    require_contains(
        &config,
        "pub type Challenge = BinomialExtensionField<Val, 8>;",
        "challenge extension field",
    )?;

    let source = fs::read_to_string(root.join("tools/parameter-search/src/production_reference.rs"))
        .map_err(|err| format!("failed to read production reference source: {err}"))?;
    for marker in [
        "PATH2_ACCOUNT_STARK_FRI_AOK_BOUNDARY_REPORT_DOMAIN",
        "PATH2_ACCOUNT_STARK_FRI_AOK_SUPPORT_FILE",
        "Path2AccountStarkFriAokBoundaryReportV1",
        "path2_account_stark_fri_aok_boundary_report_v1",
        "validate_path2_account_stark_fri_aok_boundary_report_v1",
        "fri_aok_boundary_report_digest",
        "account_stark_fri_aok_extractor_break",
    ] {
        require_contains(&source, marker, marker)?;
    }

    println!("Path 2 account-STARK FRI/AoK boundary consistency check passed.");
    Ok(())
}
