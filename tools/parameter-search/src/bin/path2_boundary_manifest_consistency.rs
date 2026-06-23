use std::{
    fs,
    path::{Path, PathBuf},
};

use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json(root: &Path, relative: &str) -> Result<Value, String> {
    let path = root.join(relative);
    let bytes = fs::read(&path).map_err(|err| format!("failed to read {relative}: {err}"))?;
    serde_json::from_slice(&bytes).map_err(|err| format!("failed to parse {relative}: {err}"))
}

fn get<'a>(value: &'a Value, path: &[&str]) -> Result<&'a Value, String> {
    let mut current = value;
    for segment in path {
        current = current
            .get(*segment)
            .ok_or_else(|| format!("missing JSON field {}", path.join(".")))?;
    }
    Ok(current)
}

fn expect_eq(left: &Value, left_path: &str, right: &Value, right_path: &str) -> Result<(), String> {
    if left == right {
        Ok(())
    } else {
        Err(format!("{left_path} does not match {right_path}"))
    }
}

fn expect_bool(value: &Value, path: &[&str], expected: bool) -> Result<(), String> {
    let actual = get(value, path)?;
    match actual.as_bool() {
        Some(actual_bool) if actual_bool == expected => Ok(()),
        Some(actual_bool) => Err(format!(
            "{} was {actual_bool}, expected {expected}",
            path.join(".")
        )),
        None => Err(format!("{} is not a boolean", path.join("."))),
    }
}

fn expect_u64(value: &Value, path: &[&str], expected: u64) -> Result<(), String> {
    let actual = get(value, path)?;
    match actual.as_u64() {
        Some(actual_u64) if actual_u64 == expected => Ok(()),
        Some(actual_u64) => Err(format!(
            "{} was {actual_u64}, expected {expected}",
            path.join(".")
        )),
        None => Err(format!("{} is not an unsigned integer", path.join("."))),
    }
}

fn check_consistency() -> Result<(), String> {
    let root = repo_root();
    let proof = read_json(
        &root,
        "formal/kat/path2_account_creation_proof_boundary_v1.json",
    )?;
    let direct = read_json(&root, "formal/kat/path2_direct_theorem_boundary_v1.json")?;
    let proof_manifest = get(&proof, &["manifest"])?;
    let direct_manifest = get(&direct, &["manifest"])?;

    expect_eq(
        get(proof_manifest, &["account_creation_proof_boundary"])?,
        "proof.manifest.account_creation_proof_boundary",
        get(direct_manifest, &["account_creation_proof_boundary"])?,
        "direct.manifest.account_creation_proof_boundary",
    )?;

    expect_eq(
        get(
            proof_manifest,
            &["digests", "account_creation_proof_boundary_report_digest"],
        )?,
        "proof.manifest.digests.account_creation_proof_boundary_report_digest",
        get(
            direct_manifest,
            &["digests", "account_creation_proof_boundary_report_digest"],
        )?,
        "direct.manifest.digests.account_creation_proof_boundary_report_digest",
    )?;
    expect_eq(
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "coverage",
                "sampler_timing_report_digest",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.coverage.sampler_timing_report_digest",
        get(
            direct_manifest,
            &[
                "account_creation_proof_boundary",
                "coverage",
                "sampler_timing_report_digest",
            ],
        )?,
        "direct.manifest.account_creation_proof_boundary.coverage.sampler_timing_report_digest",
    )?;
    expect_eq(
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "coverage",
                "frame_kat_boundary_report_digest",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.coverage.frame_kat_boundary_report_digest",
        get(
            direct_manifest,
            &[
                "account_creation_proof_boundary",
                "coverage",
                "frame_kat_boundary_report_digest",
            ],
        )?,
        "direct.manifest.account_creation_proof_boundary.coverage.frame_kat_boundary_report_digest",
    )?;
    expect_eq(
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "coverage",
                "account_stark_residual_matrix_report_digest",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.coverage.account_stark_residual_matrix_report_digest",
        get(
            direct_manifest,
            &[
                "account_creation_proof_boundary",
                "coverage",
                "account_stark_residual_matrix_report_digest",
            ],
        )?,
        "direct.manifest.account_creation_proof_boundary.coverage.account_stark_residual_matrix_report_digest",
    )?;
    expect_eq(
        get(
            proof_manifest,
            &["authority_snapshot", "proof_boundary_report_domain"],
        )?,
        "proof.manifest.authority_snapshot.proof_boundary_report_domain",
        get(
            direct_manifest,
            &[
                "authority_snapshot",
                "account_creation_proof_boundary_report_domain",
            ],
        )?,
        "direct.manifest.authority_snapshot.account_creation_proof_boundary_report_domain",
    )?;

    for field in [
        "selected_payload_type",
        "verifier_boundary_endpoint",
        "production_boundary_endpoint",
        "replay_payload_digest_function",
        "accepted_boundary_evidence_schema",
        "accepted_boundary_evidence_endpoint",
    ] {
        expect_eq(
            get(proof_manifest, &["authority_snapshot", field])?,
            &format!("proof.manifest.authority_snapshot.{field}"),
            get(direct_manifest, &["authority_snapshot", field])?,
            &format!("direct.manifest.authority_snapshot.{field}"),
        )?;
    }

    expect_eq(
        get(
            proof_manifest,
            &["authority_snapshot", "proof_boundary_blocker"],
        )?,
        "proof.manifest.authority_snapshot.proof_boundary_blocker",
        get(
            direct_manifest,
            &[
                "authority_snapshot",
                "account_creation_proof_boundary_blocker",
            ],
        )?,
        "direct.manifest.authority_snapshot.account_creation_proof_boundary_blocker",
    )?;

    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "coverage",
            "account_creation_proof_boundary_verified",
        ],
        false,
    )?;
    expect_u64(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "coverage",
            "obligation_audit_vector_case_count",
        ],
        9,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "deployment_ready",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "accepted_boundary_evidence_present",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "every_accepting_requirement_has_coverage_row",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "every_vector_covered_requirement_has_public_rejection_class",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "every_current_vector_case_is_known_kat_row",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "every_requirement_public_rejection_class_is_known",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "requirement_coverage_matrix_pinned",
        ],
        true,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "requirement_coverage_count",
        ],
        10,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "current_vector_covered_requirement_count",
        ],
        10,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "every_proof_obligation_has_named_break",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "every_open_proof_obligation_blocks_deployment",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "proof_obligation_rows_pinned",
        ],
        true,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "proof_obligation_audit_vector_case_count",
        ],
        8,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "deployment_ready",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "selected_account_stark_residual_matrix",
            "coverage",
            "deployment_ready",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "fri_aok_boundary_snapshot",
            "coverage",
            "fri_aok_extractor_theorem_instantiated",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &["fri_aok_boundary_snapshot", "coverage", "deployment_ready"],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "payload_kat_boundary_snapshot",
            "coverage",
            "deployment_ready",
        ],
        false,
    )?;

    Ok(())
}

fn main() -> Result<(), String> {
    check_consistency()?;
    println!("Path 2 boundary manifest consistency check passed.");
    Ok(())
}
