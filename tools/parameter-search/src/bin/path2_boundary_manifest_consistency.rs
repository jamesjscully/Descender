use std::{
    fs,
    path::{Path, PathBuf},
};

use descender_parameter_search::manifest_vector_evidence::{
    path2_account_creation_proof_obligation_vector_corpus_evidence_v1,
    path2_account_creation_requirement_vector_corpus_evidence_v1,
    path2_direct_theorem_obligation_vector_corpus_evidence_v1,
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

fn field_values(value: &Value, path: &[&str], field: &str) -> Result<Value, String> {
    let rows = get(value, path)?
        .as_array()
        .ok_or_else(|| format!("{} is not an array", path.join(".")))?;
    let mut values = Vec::with_capacity(rows.len());
    for row in rows {
        values.push(get(row, &[field])?.clone());
    }
    Ok(Value::Array(values))
}

fn get_array<'a>(value: &'a Value, path: &[&str]) -> Result<&'a Vec<Value>, String> {
    get(value, path)?
        .as_array()
        .ok_or_else(|| format!("{} is not an array", path.join(".")))
}

fn get_str_field<'a>(value: &'a Value, field: &str, label: &str) -> Result<&'a str, String> {
    get(value, &[field])?
        .as_str()
        .ok_or_else(|| format!("{label}.{field} is not a string"))
}

fn get_bool_field(value: &Value, field: &str, label: &str) -> Result<bool, String> {
    get(value, &[field])?
        .as_bool()
        .ok_or_else(|| format!("{label}.{field} is not a boolean"))
}

fn find_row_by_str_field<'a>(
    rows: &'a [Value],
    field: &str,
    expected: &str,
    label: &str,
) -> Result<&'a Value, String> {
    for row in rows {
        if get_str_field(row, field, label)? == expected {
            return Ok(row);
        }
    }
    Err(format!("{label} missing row with {field}={expected}"))
}

fn expect_field_values_match(
    value: &Value,
    row_path: &[&str],
    row_field: &str,
    evidence_path: &[&str],
    evidence_field: &str,
    label: &str,
) -> Result<(), String> {
    let row_values = field_values(value, row_path, row_field)?;
    let evidence_values = field_values(value, evidence_path, evidence_field)?;
    expect_eq(
        &evidence_values,
        &format!("{label}.{}.{}", evidence_path.join("."), evidence_field),
        &row_values,
        &format!("{label}.{}.{}", row_path.join("."), row_field),
    )
}

fn expect_proof_closure_dependency_evidence_identity(
    value: &Value,
    label: &str,
) -> Result<(), String> {
    for (row_field, evidence_field) in [
        ("dependency_id", "dependency_id"),
        ("source_report", "source_report"),
        ("source_flag_group", "source_flag_group"),
        ("proof_obligation_id", "proof_obligation_id"),
        ("residual_item", "residual_item"),
        ("named_break", "named_break"),
        ("blocker", "blocker"),
        ("open", "closure_dependency_open"),
    ] {
        expect_field_values_match(
            value,
            &["closure_dependency_rows"],
            row_field,
            &["closure_dependency_evidence_rows"],
            evidence_field,
            label,
        )?;
    }
    Ok(())
}

fn expect_direct_closure_dependency_evidence_identity(
    value: &Value,
    label: &str,
) -> Result<(), String> {
    for (row_field, evidence_field) in [
        ("dependency_id", "dependency_id"),
        ("source_report", "source_report"),
        ("source_flag_group", "source_flag_group"),
        ("direct_obligation_id", "direct_obligation_id"),
        ("acceptance_hypothesis", "acceptance_hypothesis"),
        ("support_surface", "support_surface"),
        ("named_break", "named_break"),
        ("blocker", "blocker"),
        ("open", "closure_dependency_open"),
    ] {
        expect_field_values_match(
            value,
            &["closure_dependency_rows"],
            row_field,
            &["closure_dependency_evidence_rows"],
            evidence_field,
            label,
        )?;
    }
    Ok(())
}

fn expect_requirement_coverage_accepted_evidence_identity(
    boundary: &Value,
    accepted_evidence: &Value,
    label: &str,
) -> Result<(), String> {
    expect_eq(
        get(accepted_evidence, &["accepting_requirements"])?,
        &format!("{label}.accepted_evidence.accepting_requirements"),
        get(boundary, &["accepting_requirements"])?,
        &format!("{label}.boundary.accepting_requirements"),
    )?;
    for (row_field, evidence_field) in [
        ("requirement", "requirement"),
        ("coverage_surface", "coverage_surface"),
        ("vector_evidence_kind", "vector_evidence_kind"),
        ("current_vector_case_ids", "current_vector_case_ids"),
        (
            "covered_by_current_vectors",
            "boundary_or_audit_evidence_present",
        ),
        ("blocker", "blocker"),
    ] {
        let row_values = field_values(boundary, &["requirement_coverage_rows"], row_field)?;
        let evidence_values = field_values(
            accepted_evidence,
            &["accepted_requirement_evidence_rows"],
            evidence_field,
        )?;
        expect_eq(
            &evidence_values,
            &format!(
                "{label}.accepted_evidence.accepted_requirement_evidence_rows.{evidence_field}"
            ),
            &row_values,
            &format!("{label}.boundary.requirement_coverage_rows.{row_field}"),
        )?;
    }
    Ok(())
}

fn expect_direct_accepted_tuple_field_evidence_identity(
    boundary: &Value,
    accepted_evidence: &Value,
    label: &str,
) -> Result<(), String> {
    expect_eq(
        get(accepted_evidence, &["accepted_tuple_fields"])?,
        &format!("{label}.accepted_evidence.accepted_tuple_fields"),
        get(boundary, &["accepted_tuple_fields"])?,
        &format!("{label}.boundary.accepted_tuple_fields"),
    )?;
    let accepted_field_values = field_values(
        accepted_evidence,
        &["accepted_tuple_field_evidence_rows"],
        "accepted_tuple_field",
    )?;
    expect_eq(
        &accepted_field_values,
        &format!(
            "{label}.accepted_evidence.accepted_tuple_field_evidence_rows.accepted_tuple_field"
        ),
        get(boundary, &["accepted_tuple_fields"])?,
        &format!("{label}.boundary.accepted_tuple_fields"),
    )?;

    let obligation_rows = get_array(boundary, &["obligation_rows"])?;
    for (index, row) in get_array(accepted_evidence, &["accepted_tuple_field_evidence_rows"])?
        .iter()
        .enumerate()
    {
        let row_label =
            format!("{label}.accepted_evidence.accepted_tuple_field_evidence_rows[{index}]");
        let row_blocker = get_str_field(row, "blocker", &row_label)?;
        let direct_obligation_ids = get_array(row, &["direct_obligation_ids"])?;
        if direct_obligation_ids.is_empty() {
            return Err(format!("{row_label}.direct_obligation_ids is empty"));
        }
        let mut all_referenced_open = true;
        for obligation_id_value in direct_obligation_ids {
            let obligation_id = obligation_id_value
                .as_str()
                .ok_or_else(|| format!("{row_label}.direct_obligation_ids is not a string"))?;
            let source_row = find_row_by_str_field(
                obligation_rows,
                "obligation_id",
                obligation_id,
                &format!("{label}.boundary.obligation_rows"),
            )?;
            if get_str_field(
                source_row,
                "blocker",
                &format!("{label}.boundary.obligation_rows"),
            )? != row_blocker
            {
                return Err(format!(
                    "{row_label} blocker does not match boundary obligation {obligation_id}"
                ));
            }
            all_referenced_open &= get_bool_field(
                source_row,
                "open",
                &format!("{label}.boundary.obligation_rows"),
            )?;
        }
        if get_bool_field(
            row,
            "all_referenced_direct_obligations_still_open",
            &row_label,
        )? != all_referenced_open
        {
            return Err(format!(
                "{row_label}.all_referenced_direct_obligations_still_open drifted from boundary obligation rows"
            ));
        }
    }
    Ok(())
}

fn expect_direct_accepted_hypothesis_evidence_identity(
    boundary: &Value,
    accepted_evidence: &Value,
    label: &str,
) -> Result<(), String> {
    let accepted_hypotheses = field_values(
        accepted_evidence,
        &["accepted_hypothesis_evidence_rows"],
        "acceptance_hypothesis",
    )?;
    expect_eq(
        &accepted_hypotheses,
        &format!(
            "{label}.accepted_evidence.accepted_hypothesis_evidence_rows.acceptance_hypothesis"
        ),
        get(boundary, &["acceptance_hypotheses"])?,
        &format!("{label}.boundary.acceptance_hypotheses"),
    )?;
    let direct_obligation_ids = field_values(
        accepted_evidence,
        &["accepted_hypothesis_evidence_rows"],
        "direct_obligation_id",
    )?;
    let boundary_obligation_ids = field_values(boundary, &["obligation_rows"], "obligation_id")?;
    expect_eq(
        &direct_obligation_ids,
        &format!(
            "{label}.accepted_evidence.accepted_hypothesis_evidence_rows.direct_obligation_id"
        ),
        &boundary_obligation_ids,
        &format!("{label}.boundary.obligation_rows.obligation_id"),
    )?;

    let obligation_rows = get_array(boundary, &["obligation_rows"])?;
    for (index, row) in get_array(accepted_evidence, &["accepted_hypothesis_evidence_rows"])?
        .iter()
        .enumerate()
    {
        let row_label =
            format!("{label}.accepted_evidence.accepted_hypothesis_evidence_rows[{index}]");
        let obligation_id = get_str_field(row, "direct_obligation_id", &row_label)?;
        let source_row = find_row_by_str_field(
            obligation_rows,
            "obligation_id",
            obligation_id,
            &format!("{label}.boundary.obligation_rows"),
        )?;
        if get_str_field(row, "acceptance_hypothesis", &row_label)?
            != get_str_field(
                source_row,
                "acceptance_hypothesis",
                &format!("{label}.boundary.obligation_rows"),
            )?
        {
            return Err(format!(
                "{row_label}.acceptance_hypothesis drifted from boundary obligation {obligation_id}"
            ));
        }
        if get_bool_field(row, "theorem_obligation_still_open", &row_label)?
            != get_bool_field(
                source_row,
                "open",
                &format!("{label}.boundary.obligation_rows"),
            )?
        {
            return Err(format!(
                "{row_label}.theorem_obligation_still_open drifted from boundary obligation {obligation_id}"
            ));
        }
    }
    Ok(())
}

fn check_consistency() -> Result<(), String> {
    let root = repo_root();
    let proof = read_json(
        &root,
        "formal/kat/path2_account_creation_proof_boundary_v1.json",
    )?;
    let direct = read_json(&root, "formal/kat/path2_direct_theorem_boundary_v1.json")?;
    let accepted_proof_boundary = read_json(
        &root,
        "formal/kat/path2_account_creation_accepted_proof_boundary_evidence_v1.json",
    )?;
    let accepted_direct_tuple = read_json(
        &root,
        "formal/kat/path2_direct_accepted_spend_tuple_evidence_v1.json",
    )?;
    let accepted_direct_composition = read_json(
        &root,
        "formal/kat/path2_direct_accepted_composition_evidence_v1.json",
    )?;
    let proof_manifest = get(&proof, &["manifest"])?;
    let direct_manifest = get(&direct, &["manifest"])?;
    let accepted_proof_boundary_evidence =
        get(&accepted_proof_boundary, &["manifest", "evidence"])?;
    let accepted_direct_tuple_evidence = get(&accepted_direct_tuple, &["manifest", "evidence"])?;
    let accepted_direct_composition_evidence =
        get(&accepted_direct_composition, &["manifest", "evidence"])?;

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
    expect_u64(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "coverage",
            "closure_dependency_count",
        ],
        9,
    )?;
    expect_u64(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "coverage",
            "closure_dependency_evidence_count",
        ],
        9,
    )?;
    expect_u64(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "coverage",
            "closure_dependency_open_obligation_evidence_count",
        ],
        9,
    )?;
    expect_u64(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "coverage",
            "closure_dependency_open_support_evidence_count",
        ],
        9,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "coverage",
            "every_closure_dependency_has_open_obligation",
        ],
        true,
    )?;
    expect_direct_closure_dependency_evidence_identity(
        get(direct_manifest, &["direct_theorem_boundary"])?,
        "direct.manifest.direct_theorem_boundary",
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "coverage",
            "every_closure_dependency_has_open_support_surface",
        ],
        true,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "coverage",
            "every_open_closure_dependency_blocks_deployment",
        ],
        true,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "coverage",
            "every_closure_dependency_has_evidence",
        ],
        true,
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
            "accepted_boundary_binding_evidence_present",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "replay_payload_digest_binding_evidence_present",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "deployed_anchor_binding_evidence_present",
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
    expect_proof_closure_dependency_evidence_identity(
        get(proof_manifest, &["account_creation_proof_boundary"])?,
        "proof.manifest.account_creation_proof_boundary",
    )?;
    expect_proof_closure_dependency_evidence_identity(
        get(direct_manifest, &["account_creation_proof_boundary"])?,
        "direct.manifest.account_creation_proof_boundary",
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
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "every_closure_dependency_has_open_obligation",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "every_closure_dependency_has_open_support_surface",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "every_open_closure_dependency_blocks_deployment",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "every_closure_dependency_has_evidence",
        ],
        true,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "closure_dependency_count",
        ],
        8,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "closure_dependency_evidence_count",
        ],
        8,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "closure_dependency_open_obligation_evidence_count",
        ],
        8,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "coverage",
            "closure_dependency_open_support_evidence_count",
        ],
        8,
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
    let requirement_vector_corpus_evidence =
        path2_account_creation_requirement_vector_corpus_evidence_v1()?;
    expect_eq(
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "requirement_vector_corpus_evidence",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.requirement_vector_corpus_evidence",
        &requirement_vector_corpus_evidence,
        "standalone account-creation requirement vector corpus evidence",
    )?;
    expect_eq(
        get(
            direct_manifest,
            &[
                "account_creation_proof_boundary",
                "requirement_vector_corpus_evidence",
            ],
        )?,
        "direct.manifest.account_creation_proof_boundary.requirement_vector_corpus_evidence",
        &requirement_vector_corpus_evidence,
        "standalone account-creation requirement vector corpus evidence",
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "all_requirement_vector_corpora_bound",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "frame_kat_vectors_cover_current_rows",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "no_export_vectors_cover_boundary_cases",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "fri_qrom_domain_vectors_cover_boundary_cases",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "payload_kat_vectors_cover_current_rows",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "any_requirement_vector_corpus_claims_theorem_ready",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "any_requirement_vector_corpus_claims_deployment_ready",
        ],
        false,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "frame_kat_case_count",
        ],
        18,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "no_export_case_count",
        ],
        4,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "fri_qrom_domain_case_count",
        ],
        5,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "requirement_vector_corpus_evidence",
            "audit_summary",
            "payload_kat_case_count",
        ],
        14,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "all_cases_open",
        ],
        true,
    )?;
    expect_eq(
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "proof_obligation_vector_corpus_evidence",
                "audit_summary",
                "proof_obligation_named_break_classes",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.proof_obligation_vector_corpus_evidence.audit_summary.proof_obligation_named_break_classes",
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "proof_obligation_named_break_classes",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.proof_obligation_named_break_classes",
    )?;
    let proof_obligation_row_ids = field_values(
        proof_manifest,
        &["account_creation_proof_boundary", "proof_obligation_rows"],
        "obligation_id",
    )?;
    expect_eq(
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "proof_obligation_vector_corpus_evidence",
                "proof_obligation_case_ids",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.proof_obligation_vector_corpus_evidence.proof_obligation_case_ids",
        &proof_obligation_row_ids,
        "proof.manifest.account_creation_proof_boundary.proof_obligation_rows obligation ids",
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "all_cases_have_named_break",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "all_open_cases_block_deployment",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "current_no_export_vectors_cover_boundary_cases",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "current_fri_qrom_domain_vectors_cover_boundary_cases",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "current_payload_kat_vectors_cover_every_boundary_row",
        ],
        true,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "proof_obligation_count",
        ],
        8,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "proof_obligation_audit_vector_case_count",
        ],
        8,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "simulator_theorem_instantiated",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "external_review_complete",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "fri_aok_extractor_theorem_instantiated",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "fri_qrom_residual_priced",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "frozen_kat_corpus_complete",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "production_gate_open",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "proof_obligation_vector_corpus_evidence",
            "audit_summary",
            "deployment_ready",
        ],
        false,
    )?;
    let proof_obligation_vector_corpus_evidence =
        path2_account_creation_proof_obligation_vector_corpus_evidence_v1()?;
    expect_eq(
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "proof_obligation_vector_corpus_evidence",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.proof_obligation_vector_corpus_evidence",
        &proof_obligation_vector_corpus_evidence,
        "standalone account-creation proof-obligation vector corpus evidence",
    )?;
    expect_eq(
        get(
            direct_manifest,
            &[
                "account_creation_proof_boundary",
                "proof_obligation_vector_corpus_evidence",
            ],
        )?,
        "direct.manifest.account_creation_proof_boundary.proof_obligation_vector_corpus_evidence",
        &proof_obligation_vector_corpus_evidence,
        "standalone account-creation proof-obligation vector corpus evidence",
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "verifier_boundary_accepts",
        ],
        true,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "accepted_requirement_evidence_count",
        ],
        10,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "accepted_path_applicable_requirement_count",
        ],
        8,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "accepted_path_requirement_evidence_count",
        ],
        8,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "boundary_or_audit_requirement_evidence_count",
        ],
        10,
    )?;
    expect_u64(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "separate_closure_required_count",
        ],
        10,
    )?;
    expect_eq(
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "accepted_proof_boundary_evidence_artifact",
                "audit_summary",
                "accepted_requirement_ids",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.accepted_proof_boundary_evidence_artifact.audit_summary.accepted_requirement_ids",
        get(
            proof_manifest,
            &["account_creation_proof_boundary", "accepting_requirements"],
        )?,
        "proof.manifest.account_creation_proof_boundary.accepting_requirements",
    )?;
    expect_requirement_coverage_accepted_evidence_identity(
        get(proof_manifest, &["account_creation_proof_boundary"])?,
        accepted_proof_boundary_evidence,
        "proof.manifest.account_creation_proof_boundary",
    )?;
    expect_requirement_coverage_accepted_evidence_identity(
        get(direct_manifest, &["account_creation_proof_boundary"])?,
        accepted_proof_boundary_evidence,
        "direct.manifest.account_creation_proof_boundary",
    )?;
    for flag in [
        "every_applicable_accepting_requirement_has_accepted_path_evidence",
        "every_accepting_requirement_has_boundary_or_audit_evidence",
        "every_separate_closure_requirement_has_open_obligation",
        "deterministic_rejection_requirement_deferred_to_rejection_kats",
        "qrom_requirement_deferred_to_residual_theorem",
    ] {
        expect_bool(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "accepted_proof_boundary_evidence_artifact",
                "audit_summary",
                flag,
            ],
            true,
        )?;
    }
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "production_gate_fail_closed",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "proof_boundary_report_validated",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "proof_family_obligations_still_open",
        ],
        true,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "public_payload_exports_private_material",
        ],
        false,
    )?;
    expect_bool(
        proof_manifest,
        &[
            "account_creation_proof_boundary",
            "accepted_proof_boundary_evidence_artifact",
            "audit_summary",
            "deployment_ready",
        ],
        false,
    )?;
    expect_eq(
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "accepted_proof_boundary_evidence_artifact",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.accepted_proof_boundary_evidence_artifact",
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "proof_obligation_vector_corpus_evidence",
                "accepted_proof_boundary_evidence_artifact",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.proof_obligation_vector_corpus_evidence.accepted_proof_boundary_evidence_artifact",
    )?;
    expect_eq(
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "accepted_proof_boundary_evidence_artifact",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.accepted_proof_boundary_evidence_artifact",
        get(
            proof_manifest,
            &[
                "account_creation_proof_boundary",
                "proof_obligation_vector_corpus_evidence",
                "direct_accepted_composition_evidence_artifact",
                "account_creation_accepted_proof_boundary_evidence_artifact",
            ],
        )?,
        "proof.manifest.account_creation_proof_boundary.proof_obligation_vector_corpus_evidence.direct_accepted_composition_evidence_artifact.account_creation_accepted_proof_boundary_evidence_artifact",
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "all_cases_open",
        ],
        true,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "all_cases_have_named_break",
        ],
        true,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "all_open_cases_block_deployment",
        ],
        true,
    )?;
    expect_eq(
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_obligation_vector_corpus_evidence",
                "audit_summary",
                "direct_named_break_classes",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_obligation_vector_corpus_evidence.audit_summary.direct_named_break_classes",
        get(
            direct_manifest,
            &["direct_theorem_boundary", "named_break_classes"],
        )?,
        "direct.manifest.direct_theorem_boundary.named_break_classes",
    )?;
    let direct_obligation_row_ids = field_values(
        direct_manifest,
        &["direct_theorem_boundary", "obligation_rows"],
        "obligation_id",
    )?;
    expect_eq(
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_obligation_vector_corpus_evidence",
                "direct_obligation_case_ids",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_obligation_vector_corpus_evidence.direct_obligation_case_ids",
        &direct_obligation_row_ids,
        "direct.manifest.direct_theorem_boundary.obligation_rows obligation ids",
    )?;
    expect_u64(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "direct_obligation_count",
        ],
        9,
    )?;
    expect_u64(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "direct_obligation_audit_vector_case_count",
        ],
        9,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "composition_theorem_instantiated",
        ],
        false,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "proof_boundary_deployment_ready",
        ],
        false,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "qrom_theorem_imports_budget",
        ],
        false,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "qrom_external_review_complete",
        ],
        false,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "lifecycle_target_chain_finality_profile_instantiated",
        ],
        false,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "account_creation_proof_boundary_verified",
        ],
        false,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "lifecycle_transition_profile_verified",
        ],
        false,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "qrom_and_action_bounds_linked_to_theorem",
        ],
        false,
    )?;
    expect_bool(
        direct_manifest,
        &[
            "direct_theorem_boundary",
            "direct_obligation_vector_corpus_evidence",
            "audit_summary",
            "deployment_ready",
        ],
        false,
    )?;
    let direct_obligation_vector_corpus_evidence =
        path2_direct_theorem_obligation_vector_corpus_evidence_v1()?;
    expect_eq(
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_obligation_vector_corpus_evidence",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_obligation_vector_corpus_evidence",
        &direct_obligation_vector_corpus_evidence,
        "standalone direct theorem obligation vector corpus evidence",
    )?;
    expect_eq(
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_spend_tuple_evidence_artifact",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_accepted_spend_tuple_evidence_artifact",
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_composition_evidence_artifact",
                "direct_accepted_spend_tuple_artifact",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_accepted_composition_evidence_artifact.direct_accepted_spend_tuple_artifact",
    )?;
    expect_eq(
        get(
            direct_manifest,
            &[
                "account_creation_proof_boundary",
                "accepted_proof_boundary_evidence_artifact",
            ],
        )?,
        "direct.manifest.account_creation_proof_boundary.accepted_proof_boundary_evidence_artifact",
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_composition_evidence_artifact",
                "account_creation_accepted_proof_boundary_evidence_artifact",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_accepted_composition_evidence_artifact.account_creation_accepted_proof_boundary_evidence_artifact",
    )?;
    expect_eq(
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_composition_evidence_artifact",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_accepted_composition_evidence_artifact",
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_obligation_vector_corpus_evidence",
                "direct_accepted_composition_evidence_artifact",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_obligation_vector_corpus_evidence.direct_accepted_composition_evidence_artifact",
    )?;
    expect_eq(
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_composition_evidence_artifact",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_accepted_composition_evidence_artifact",
        get(
            direct_manifest,
            &[
                "account_creation_proof_boundary",
                "proof_obligation_vector_corpus_evidence",
                "direct_accepted_composition_evidence_artifact",
            ],
        )?,
        "direct.manifest.account_creation_proof_boundary.proof_obligation_vector_corpus_evidence.direct_accepted_composition_evidence_artifact",
    )?;
    for flag in [
        "state_namespace_record_present",
        "consumed_authorization_absent_before",
        "consumed_authorization_present_after",
        "consume_once_inserted_exactly_one_authorization",
        "public_result_accepts_same_slot",
        "tuple_fields_match_theorem_boundary",
        "replay_rejected_by_consume_once",
    ] {
        expect_bool(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_spend_tuple_evidence_artifact",
                "audit_summary",
                flag,
            ],
            true,
        )?;
    }
    for (field, expected) in [
        ("accepted_tuple_field_evidence_count", 11),
        ("accepted_tuple_field_evidence_present_count", 11),
        ("accepted_tuple_field_open_obligation_count", 11),
    ] {
        expect_u64(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_spend_tuple_evidence_artifact",
                "audit_summary",
                field,
            ],
            expected,
        )?;
    }
    expect_eq(
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_spend_tuple_evidence_artifact",
                "audit_summary",
                "accepted_tuple_field_ids",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_accepted_spend_tuple_evidence_artifact.audit_summary.accepted_tuple_field_ids",
        get(
            direct_manifest,
            &["direct_theorem_boundary", "accepted_tuple_fields"],
        )?,
        "direct.manifest.direct_theorem_boundary.accepted_tuple_fields",
    )?;
    expect_direct_accepted_tuple_field_evidence_identity(
        get(direct_manifest, &["direct_theorem_boundary"])?,
        accepted_direct_tuple_evidence,
        "direct.manifest.direct_theorem_boundary",
    )?;
    for flag in [
        "every_accepted_tuple_field_has_evidence",
        "every_accepted_tuple_field_has_open_direct_obligation",
    ] {
        expect_bool(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_spend_tuple_evidence_artifact",
                "audit_summary",
                flag,
            ],
            true,
        )?;
    }
    for flag in [
        "creation_evidence_matches_direct_tuple",
        "proof_boundary_digest_matches_direct_theorem",
        "qrom_action_digest_matches_direct_theorem",
        "lifecycle_digest_matches_direct_theorem",
        "direct_admission_namespace_matches_authorization",
        "consume_once_transition_matches_tuple",
        "verifier_boundary_accepts_creation",
        "direct_spend_tuple_accepts",
        "public_result_accepts_same_slot",
        "direct_theorem_obligations_still_open",
        "proof_family_obligations_still_open",
    ] {
        expect_bool(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_composition_evidence_artifact",
                "audit_summary",
                flag,
            ],
            true,
        )?;
    }
    for (field, expected) in [
        ("accepted_hypothesis_evidence_count", 9),
        ("positive_acceptance_hypothesis_evidence_count", 8),
    ] {
        expect_u64(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_composition_evidence_artifact",
                "audit_summary",
                field,
            ],
            expected,
        )?;
    }
    expect_eq(
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_composition_evidence_artifact",
                "audit_summary",
                "accepted_hypothesis_ids",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_accepted_composition_evidence_artifact.audit_summary.accepted_hypothesis_ids",
        get(
            direct_manifest,
            &["direct_theorem_boundary", "acceptance_hypotheses"],
        )?,
        "direct.manifest.direct_theorem_boundary.acceptance_hypotheses",
    )?;
    expect_eq(
        get(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_composition_evidence_artifact",
                "audit_summary",
                "accepted_hypothesis_direct_obligation_ids",
            ],
        )?,
        "direct.manifest.direct_theorem_boundary.direct_accepted_composition_evidence_artifact.audit_summary.accepted_hypothesis_direct_obligation_ids",
        &direct_obligation_row_ids,
        "direct.manifest.direct_theorem_boundary.obligation_rows obligation ids",
    )?;
    expect_direct_accepted_hypothesis_evidence_identity(
        get(direct_manifest, &["direct_theorem_boundary"])?,
        accepted_direct_composition_evidence,
        "direct.manifest.direct_theorem_boundary",
    )?;
    for flag in [
        "every_positive_acceptance_hypothesis_has_accepted_path_evidence",
        "public_rejection_hypothesis_deferred_to_rejection_kats",
    ] {
        expect_bool(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_composition_evidence_artifact",
                "audit_summary",
                flag,
            ],
            true,
        )?;
    }
    for flag in [
        "composition_theorem_instantiated",
        "account_creation_proof_boundary_verified",
        "deployment_ready",
    ] {
        expect_bool(
            direct_manifest,
            &[
                "direct_theorem_boundary",
                "direct_accepted_composition_evidence_artifact",
                "audit_summary",
                flag,
            ],
            false,
        )?;
    }

    Ok(())
}

fn main() -> Result<(), String> {
    check_consistency()?;
    println!("Path 2 boundary manifest consistency check passed.");
    Ok(())
}
