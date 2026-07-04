use descender_parameter_search::production_reference::{
    Path2AccountCreationProofBoundaryReportV1, Path2AccountStarkResidualMatrixReportV1,
    Path2DirectTheoremBoundaryReportV1, path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_account_stark_fri_aok_boundary_report_v1,
    path2_account_stark_fri_aok_boundary_report_v1_digest,
    path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_fri_qrom_residual_report_v1_digest,
    path2_account_stark_residual_matrix_report_v1,
    path2_account_stark_residual_matrix_report_v1_digest,
    path2_direct_lifecycle_boundary_report_v1, path2_direct_lifecycle_boundary_report_v1_digest,
    path2_direct_qrom_action_boundary_report_v1,
    path2_direct_qrom_action_boundary_report_v1_digest, path2_direct_theorem_boundary_report_v1,
    path2_direct_theorem_boundary_report_v1_digest,
    production_deployment_account_creation_proof_boundary_blockers,
    production_deployment_account_stark_residual_blockers,
    production_deployment_direct_theorem_blockers, production_deployment_preflight,
    production_deployment_preflight_blocker_digest, production_deployment_preflight_blocker_report,
    production_deployment_preflight_blocker_report_digest,
    production_deployment_preflight_blockers, production_deployment_preflight_certificate,
    production_deployment_preflight_certificate_digest,
    production_deployment_preflight_ready_assertion,
    production_deployment_preflight_ready_assertion_digest, production_deployment_preflight_report,
    production_deployment_preflight_report_digest, production_deployment_preflight_verdict,
    production_deployment_preflight_verdict_digest, require_production_deployment_preflight_ready,
    validate_path2_account_creation_proof_boundary_report_v1,
    validate_path2_account_stark_fri_aok_boundary_report_v1,
    validate_path2_account_stark_fri_qrom_residual_report_v1,
    validate_path2_account_stark_residual_matrix_report_v1,
    validate_path2_direct_lifecycle_boundary_report_v1,
    validate_path2_direct_qrom_action_boundary_report_v1,
    validate_path2_direct_theorem_boundary_report_v1,
    validate_production_deployment_preflight_blocker,
    validate_production_deployment_preflight_blocker_report,
    validate_production_deployment_preflight_certificate,
    validate_production_deployment_preflight_ready_assertion,
    validate_production_deployment_preflight_report,
    validate_production_deployment_preflight_verdict,
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

fn load_json(root: &Path, relative_path: &str) -> Result<Value, String> {
    let path = root.join(relative_path);
    let text =
        fs::read_to_string(&path).map_err(|err| format!("read {}: {err}", path.display()))?;
    serde_json::from_str(&text).map_err(|err| format!("parse {}: {err}", path.display()))
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

fn array_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a Vec<Value>, String> {
    at(value, pointer)?
        .as_array()
        .ok_or_else(|| format!("JSON pointer {pointer} is not an array"))
}

fn array_field_values_at(value: &Value, pointer: &str, field: &str) -> Result<Value, String> {
    let rows = array_at(value, pointer)?;
    let mut values = Vec::with_capacity(rows.len());
    for row in rows {
        values.push(at(row, &format!("/{field}"))?.clone());
    }
    Ok(Value::Array(values))
}

fn require_array_field_values_match(
    value: &Value,
    row_pointer: &str,
    row_field: &str,
    evidence_pointer: &str,
    evidence_field: &str,
    label: &str,
) -> Result<(), String> {
    require_array_field_values_match_between(
        value,
        row_pointer,
        row_field,
        value,
        evidence_pointer,
        evidence_field,
        label,
    )
}

fn require_array_field_values_match_between(
    row_value: &Value,
    row_pointer: &str,
    row_field: &str,
    evidence_value: &Value,
    evidence_pointer: &str,
    evidence_field: &str,
    label: &str,
) -> Result<(), String> {
    let row_values = array_field_values_at(row_value, row_pointer, row_field)?;
    let evidence_values = array_field_values_at(evidence_value, evidence_pointer, evidence_field)?;
    if evidence_values != row_values {
        return Err(format!(
            "{label} evidence field {evidence_pointer}/{evidence_field} drifted from {row_pointer}/{row_field}",
        ));
    }
    Ok(())
}

fn find_array_row_by_str_field<'a>(
    rows: &'a [Value],
    field: &str,
    expected: &str,
    label: &str,
) -> Result<&'a Value, String> {
    for row in rows {
        if str_at(row, &format!("/{field}"))? == expected {
            return Ok(row);
        }
    }
    Err(format!("{label} missing row with {field}={expected}"))
}

fn require_proof_closure_dependency_evidence_identity(
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
        require_array_field_values_match(
            value,
            "/closure_dependency_rows",
            row_field,
            "/closure_dependency_evidence_rows",
            evidence_field,
            label,
        )?;
    }
    Ok(())
}

fn require_direct_closure_dependency_evidence_identity(
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
        require_array_field_values_match(
            value,
            "/closure_dependency_rows",
            row_field,
            "/closure_dependency_evidence_rows",
            evidence_field,
            label,
        )?;
    }
    Ok(())
}

fn require_requirement_coverage_accepted_evidence_identity(
    boundary: &Value,
    accepted_evidence: &Value,
    label: &str,
) -> Result<(), String> {
    if at(accepted_evidence, "/accepting_requirements")? != at(boundary, "/accepting_requirements")?
    {
        return Err(format!(
            "{label} accepted proof-boundary evidence lost accepting-requirement identity"
        ));
    }
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
        require_array_field_values_match_between(
            boundary,
            "/requirement_coverage_rows",
            row_field,
            accepted_evidence,
            "/accepted_requirement_evidence_rows",
            evidence_field,
            label,
        )?;
    }
    Ok(())
}

fn require_direct_accepted_tuple_field_evidence_identity(
    boundary: &Value,
    accepted_evidence: &Value,
    label: &str,
) -> Result<(), String> {
    if at(accepted_evidence, "/accepted_tuple_fields")? != at(boundary, "/accepted_tuple_fields")? {
        return Err(format!(
            "{label} accepted spend tuple evidence lost accepted-tuple-field identity"
        ));
    }
    let accepted_field_values = array_field_values_at(
        accepted_evidence,
        "/accepted_tuple_field_evidence_rows",
        "accepted_tuple_field",
    )?;
    if accepted_field_values != *at(boundary, "/accepted_tuple_fields")? {
        return Err(format!(
            "{label} accepted spend tuple field evidence rows drifted from boundary accepted tuple fields"
        ));
    }
    let obligation_rows = array_at(boundary, "/obligation_rows")?;
    for (index, row) in array_at(accepted_evidence, "/accepted_tuple_field_evidence_rows")?
        .iter()
        .enumerate()
    {
        let row_label =
            format!("{label}.accepted_evidence.accepted_tuple_field_evidence_rows[{index}]");
        let row_blocker = str_at(row, "/blocker")?;
        let direct_obligation_ids = array_at(row, "/direct_obligation_ids")?;
        if direct_obligation_ids.is_empty() {
            return Err(format!("{row_label}.direct_obligation_ids is empty"));
        }
        let mut all_referenced_open = true;
        let mut all_referenced_open_or_imported = true;
        for obligation_id_value in direct_obligation_ids {
            let obligation_id = obligation_id_value.as_str().ok_or_else(|| {
                format!("{row_label}.direct_obligation_ids contains a non-string")
            })?;
            let source_row = find_array_row_by_str_field(
                obligation_rows,
                "obligation_id",
                obligation_id,
                &format!("{label}.boundary.obligation_rows"),
            )?;
            if str_at(source_row, "/blocker")? != row_blocker {
                return Err(format!(
                    "{row_label} blocker does not match boundary obligation {obligation_id}"
                ));
            }
            let source_open = bool_at(source_row, "/open")?;
            let acceptance_hypothesis = str_at(source_row, "/acceptance_hypothesis")?;
            let imported_closed = !source_open
                && ((obligation_id == "direct_obligation_consume_once"
                    && acceptance_hypothesis
                        == "lifecycle_transition_consumes_authorization_once")
                    || (obligation_id == "direct_obligation_checked_core_frame_binding"
                        && acceptance_hypothesis
                            == "checked_core_challenge_binds_public_spend_context")
                    || (obligation_id == "direct_obligation_slot_authorization_digest"
                        && acceptance_hypothesis
                            == "slot_authorization_digest_matches_canonical_leaf"));
            all_referenced_open &= source_open;
            all_referenced_open_or_imported &= source_open || imported_closed;
        }
        if bool_at(row, "/all_referenced_direct_obligations_still_open")? != all_referenced_open {
            return Err(format!(
                "{row_label}.all_referenced_direct_obligations_still_open drifted from boundary obligation rows"
            ));
        }
        if bool_at(row, "/all_referenced_direct_obligations_open_or_imported")?
            != all_referenced_open_or_imported
        {
            return Err(format!(
                "{row_label}.all_referenced_direct_obligations_open_or_imported drifted from boundary obligation rows"
            ));
        }
    }
    Ok(())
}

fn require_direct_accepted_hypothesis_evidence_identity(
    boundary: &Value,
    accepted_evidence: &Value,
    label: &str,
) -> Result<(), String> {
    let accepted_hypothesis_values = array_field_values_at(
        accepted_evidence,
        "/accepted_hypothesis_evidence_rows",
        "acceptance_hypothesis",
    )?;
    if accepted_hypothesis_values != *at(boundary, "/acceptance_hypotheses")? {
        return Err(format!(
            "{label} accepted composition hypothesis evidence rows drifted from boundary acceptance hypotheses"
        ));
    }
    require_array_field_values_match_between(
        boundary,
        "/obligation_rows",
        "obligation_id",
        accepted_evidence,
        "/accepted_hypothesis_evidence_rows",
        "direct_obligation_id",
        label,
    )?;
    let obligation_rows = array_at(boundary, "/obligation_rows")?;
    for (index, row) in array_at(accepted_evidence, "/accepted_hypothesis_evidence_rows")?
        .iter()
        .enumerate()
    {
        let row_label =
            format!("{label}.accepted_evidence.accepted_hypothesis_evidence_rows[{index}]");
        let obligation_id = str_at(row, "/direct_obligation_id")?;
        let source_row = find_array_row_by_str_field(
            obligation_rows,
            "obligation_id",
            obligation_id,
            &format!("{label}.boundary.obligation_rows"),
        )?;
        if str_at(row, "/acceptance_hypothesis")? != str_at(source_row, "/acceptance_hypothesis")? {
            return Err(format!(
                "{row_label}.acceptance_hypothesis drifted from boundary obligation {obligation_id}"
            ));
        }
        if bool_at(row, "/theorem_obligation_still_open")? != bool_at(source_row, "/open")? {
            return Err(format!(
                "{row_label}.theorem_obligation_still_open drifted from boundary obligation {obligation_id}"
            ));
        }
    }
    Ok(())
}

fn body_digest(value: &Value, label: &str) -> Result<String, String> {
    let json =
        serde_json::to_string_pretty(value).map_err(|err| format!("serialize {label}: {err}"))?;
    Ok(hex_encode(&Sha256::digest(json.as_bytes())))
}

fn require_manifest_digest<'a>(file: &'a Value, label: &str) -> Result<&'a Value, String> {
    let manifest = at(file, "/manifest")?;
    let recomputed = body_digest(manifest, label)?;
    if str_at(file, "/manifest_digest")? != recomputed {
        return Err(format!("{label} manifest digest mismatch"));
    }
    Ok(manifest)
}

fn contains(haystack: &[&str], needle: &str) -> bool {
    haystack.iter().any(|value| *value == needle)
}

fn require_open_direct_obligation(
    report: &Path2DirectTheoremBoundaryReportV1,
    blockers: &[&str],
    flag_name: &str,
    obligation_id: &str,
    acceptance_hypothesis: &str,
    supporting_surface_fragment: &str,
    required_closure_fragment: &str,
) -> Result<(), String> {
    let row = report
        .obligation_rows
        .iter()
        .find(|row| row.obligation_id == obligation_id)
        .ok_or_else(|| {
            format!("direct closure flag {flag_name} is missing obligation {obligation_id}")
        })?;
    if !row.open
        || row.acceptance_hypothesis != acceptance_hypothesis
        || row.blocker != report.blocker
        || !contains(blockers, row.blocker)
        || !row.supporting_surface.contains(supporting_surface_fragment)
        || !row.required_closure.contains(required_closure_fragment)
    {
        return Err(format!(
            "direct closure flag {flag_name} is not backed by the expected open obligation {obligation_id}"
        ));
    }
    Ok(())
}

fn require_open_proof_obligation(
    report: &Path2AccountCreationProofBoundaryReportV1,
    blockers: &[&str],
    flag_name: &str,
    obligation_id: &str,
    closure_target_fragment: &str,
    supporting_surface_fragment: &str,
    required_closure_fragment: &str,
) -> Result<(), String> {
    let row = report
        .proof_obligation_rows
        .iter()
        .find(|row| row.obligation_id == obligation_id)
        .ok_or_else(|| {
            format!("proof-boundary flag {flag_name} is missing obligation {obligation_id}")
        })?;
    if !row.open
        || row.blocker != report.blocker
        || !contains(blockers, row.blocker)
        || !row.closure_target.contains(closure_target_fragment)
        || !row.supporting_surface.contains(supporting_surface_fragment)
        || !row.required_closure.contains(required_closure_fragment)
    {
        return Err(format!(
            "proof-boundary flag {flag_name} is not backed by the expected open obligation {obligation_id}"
        ));
    }
    Ok(())
}

fn require_open_residual_item(
    report: &Path2AccountStarkResidualMatrixReportV1,
    blockers: &[&str],
    residual_item: &str,
    required_closure_fragment: &str,
) -> Result<(), String> {
    let row = report
        .rows
        .iter()
        .find(|row| row.residual_item == residual_item)
        .ok_or_else(|| format!("residual matrix is missing item {residual_item}"))?;
    if !row.open
        || !contains(blockers, row.blocker)
        || !row.required_closure.contains(required_closure_fragment)
    {
        return Err(format!(
            "residual matrix item {residual_item} is not backed by the expected open blocker"
        ));
    }
    Ok(())
}

fn find_subsequence(haystack: &[&str], needle: &[&str]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn require_subsequence_after(
    haystack: &[&str],
    prior_start: usize,
    prior_len: usize,
    needle: &[&str],
    label: &str,
) -> Result<usize, String> {
    let required_start = prior_start + prior_len;
    let observed_start = find_subsequence(haystack, needle)
        .ok_or_else(|| format!("production preflight is missing {label} blocker group"))?;
    if observed_start != required_start {
        return Err(format!(
            "production preflight {label} blocker group starts at {observed_start}, expected {required_start}"
        ));
    }
    Ok(observed_start)
}

fn main() -> Result<(), String> {
    let direct_report = path2_direct_theorem_boundary_report_v1();
    let qrom_action_report = path2_direct_qrom_action_boundary_report_v1();
    let lifecycle_report = path2_direct_lifecycle_boundary_report_v1();
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    let fri_qrom_report = path2_account_stark_fri_qrom_residual_report_v1();
    let fri_aok_report = path2_account_stark_fri_aok_boundary_report_v1();
    let residual_report = path2_account_stark_residual_matrix_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    validate_path2_direct_qrom_action_boundary_report_v1(&qrom_action_report)?;
    validate_path2_direct_lifecycle_boundary_report_v1(&lifecycle_report)?;
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    validate_path2_account_stark_fri_qrom_residual_report_v1(&fri_qrom_report)?;
    validate_path2_account_stark_fri_aok_boundary_report_v1(&fri_aok_report)?;
    validate_path2_account_stark_residual_matrix_report_v1(&residual_report)?;
    if path2_direct_theorem_boundary_report_v1_digest(&direct_report) != direct_report.digest {
        return Err("direct theorem boundary digest mismatch".to_string());
    }
    if path2_direct_qrom_action_boundary_report_v1_digest(&qrom_action_report)
        != qrom_action_report.digest
    {
        return Err("direct QROM/action boundary digest mismatch".to_string());
    }
    if path2_direct_lifecycle_boundary_report_v1_digest(&lifecycle_report)
        != lifecycle_report.digest
    {
        return Err("direct lifecycle boundary digest mismatch".to_string());
    }
    if path2_account_creation_proof_boundary_report_v1_digest(&proof_report) != proof_report.digest
    {
        return Err("account-creation proof boundary digest mismatch".to_string());
    }
    if path2_account_stark_fri_qrom_residual_report_v1_digest(&fri_qrom_report)
        != fri_qrom_report.digest
    {
        return Err("account-STARK FRI/QROM residual digest mismatch".to_string());
    }
    if path2_account_stark_fri_aok_boundary_report_v1_digest(&fri_aok_report)
        != fri_aok_report.digest
    {
        return Err("account-STARK FRI/AoK boundary digest mismatch".to_string());
    }
    if path2_account_stark_residual_matrix_report_v1_digest(&residual_report)
        != residual_report.digest
    {
        return Err("account-STARK residual matrix digest mismatch".to_string());
    }
    if direct_report.account_creation_proof_boundary_report_digest != proof_report.digest {
        return Err("direct theorem report is not bound to the proof-boundary report".to_string());
    }
    if direct_report.direct_qrom_action_boundary_report_digest != qrom_action_report.digest {
        return Err("direct theorem report is not bound to the QROM/action report".to_string());
    }
    if direct_report.direct_lifecycle_boundary_report_digest != lifecycle_report.digest {
        return Err("direct theorem report is not bound to the lifecycle report".to_string());
    }
    if proof_report.account_stark_residual_matrix_report_digest != residual_report.digest {
        return Err("proof-boundary report is not bound to the residual matrix".to_string());
    }
    if residual_report.fri_qrom_residual_report_digest != fri_qrom_report.digest {
        return Err("residual matrix is not bound to the FRI/QROM residual report".to_string());
    }
    if fri_aok_report.fri_qrom_residual_report_digest != fri_qrom_report.digest {
        return Err("FRI/AoK boundary is not bound to the FRI/QROM residual report".to_string());
    }
    if residual_report.fri_aok_boundary_report_digest != fri_aok_report.digest {
        return Err("residual matrix is not bound to the FRI/AoK boundary report".to_string());
    }

    let preflight = production_deployment_preflight();
    let blockers = preflight.blockers.as_slice();
    let report = production_deployment_preflight_report();
    validate_production_deployment_preflight_report(&report)?;
    if production_deployment_preflight_report_digest(&report) != report.digest {
        return Err("production preflight report digest mismatch".to_string());
    }
    if report.preflight != preflight {
        return Err("production preflight report does not embed the current preflight".to_string());
    }
    if preflight.deployment_ready {
        return Err("production preflight unexpectedly reports deployment_ready=true".to_string());
    }
    let blocker_rows = production_deployment_preflight_blockers(&report)?;
    if blocker_rows.len() != preflight.blockers.len() {
        return Err("production preflight blocker row count mismatch".to_string());
    }
    for (index, blocker_row) in blocker_rows.iter().enumerate() {
        validate_production_deployment_preflight_blocker(&report, blocker_row)?;
        if production_deployment_preflight_blocker_digest(blocker_row) != blocker_row.digest {
            return Err(format!(
                "production preflight blocker row {index} digest mismatch"
            ));
        }
        if blocker_row.digest == [0_u8; 32]
            || blocker_row.preflight_report_digest != report.digest
            || blocker_row.index != index
            || blocker_row.blocker != preflight.blockers[index]
        {
            return Err(format!(
                "production preflight blocker row {index} lost report/index/blocker binding"
            ));
        }
    }
    let blocker_report = production_deployment_preflight_blocker_report(&report)?;
    validate_production_deployment_preflight_blocker_report(&report, &blocker_report)?;
    if production_deployment_preflight_blocker_report_digest(&blocker_report)
        != blocker_report.digest
        || blocker_report.digest == [0_u8; 32]
        || blocker_report.preflight_report_digest != report.digest
        || blocker_report.blocker_count != blocker_rows.len()
        || blocker_report.blocker_digests.len() != blocker_rows.len()
    {
        return Err("production preflight blocker report lost digest/count binding".to_string());
    }
    for (index, blocker_digest) in blocker_report.blocker_digests.iter().enumerate() {
        if *blocker_digest != blocker_rows[index].digest {
            return Err(format!(
                "production preflight blocker report digest {index} drifted from row"
            ));
        }
    }
    let verdict = production_deployment_preflight_verdict(&report)?;
    validate_production_deployment_preflight_verdict(&report, &verdict)?;
    if production_deployment_preflight_verdict_digest(&verdict) != verdict.digest
        || verdict.digest == [0_u8; 32]
        || verdict.preflight_report_digest != report.digest
        || verdict.blocker_report_digest != blocker_report.digest
        || verdict.deployment_ready
        || verdict.blocker_count != preflight.blockers.len()
        || verdict.primary_blocker != preflight.blockers[0]
    {
        return Err("production preflight verdict lost fail-closed binding".to_string());
    }
    let ready_assertion = production_deployment_preflight_ready_assertion(&report, &verdict)?;
    validate_production_deployment_preflight_ready_assertion(&report, &verdict, &ready_assertion)?;
    if production_deployment_preflight_ready_assertion_digest(&ready_assertion)
        != ready_assertion.digest
        || ready_assertion.digest == [0_u8; 32]
        || ready_assertion.preflight_report_digest != report.digest
        || ready_assertion.blocker_report_digest != blocker_report.digest
        || ready_assertion.verdict_digest != verdict.digest
        || ready_assertion.assertion_passed
        || ready_assertion.blocker != verdict.primary_blocker
    {
        return Err("production preflight ready assertion lost fail-closed binding".to_string());
    }
    let certificate = production_deployment_preflight_certificate(&report)?;
    validate_production_deployment_preflight_certificate(&report, &certificate)?;
    if production_deployment_preflight_certificate_digest(&certificate) != certificate.digest
        || certificate.digest == [0_u8; 32]
        || certificate.preflight_report_digest != report.digest
        || certificate.blocker_report_digest != blocker_report.digest
        || certificate.verdict_digest != verdict.digest
        || certificate.ready_assertion_digest != ready_assertion.digest
        || certificate.deployment_ready
        || certificate.blocker_count != preflight.blockers.len()
        || certificate.primary_blocker != ready_assertion.blocker
    {
        return Err("production preflight certificate lost fail-closed binding".to_string());
    }
    if require_production_deployment_preflight_ready(&report, &verdict).unwrap_err()
        != ready_assertion.blocker
    {
        return Err(
            "production preflight ready requirement did not return primary blocker".to_string(),
        );
    }

    let root = repo_root();
    let proof_manifest_file = load_json(
        &root,
        "formal/kat/path2_account_creation_proof_boundary_v1.json",
    )?;
    let direct_manifest_file =
        load_json(&root, "formal/kat/path2_direct_theorem_boundary_v1.json")?;
    let accepted_proof_boundary_file = load_json(
        &root,
        "formal/kat/path2_account_creation_accepted_proof_boundary_evidence_v1.json",
    )?;
    let accepted_direct_tuple_file = load_json(
        &root,
        "formal/kat/path2_direct_accepted_spend_tuple_evidence_v1.json",
    )?;
    let accepted_direct_composition_file = load_json(
        &root,
        "formal/kat/path2_direct_accepted_composition_evidence_v1.json",
    )?;
    let proof_manifest =
        require_manifest_digest(&proof_manifest_file, "account-creation proof boundary")?;
    let direct_manifest =
        require_manifest_digest(&direct_manifest_file, "direct theorem boundary")?;
    let accepted_proof_boundary_manifest = require_manifest_digest(
        &accepted_proof_boundary_file,
        "accepted account-creation proof-boundary evidence",
    )?;
    let accepted_direct_tuple_manifest = require_manifest_digest(
        &accepted_direct_tuple_file,
        "accepted direct spend tuple evidence",
    )?;
    let accepted_direct_composition_manifest = require_manifest_digest(
        &accepted_direct_composition_file,
        "accepted direct composition evidence",
    )?;
    let proof_boundary = at(proof_manifest, "/account_creation_proof_boundary")?;
    let direct_boundary = at(direct_manifest, "/direct_theorem_boundary")?;
    let accepted_proof_boundary_evidence = at(accepted_proof_boundary_manifest, "/evidence")?;
    let accepted_direct_tuple_evidence = at(accepted_direct_tuple_manifest, "/evidence")?;
    let accepted_direct_composition_evidence =
        at(accepted_direct_composition_manifest, "/evidence")?;
    if proof_boundary != at(direct_manifest, "/account_creation_proof_boundary")? {
        return Err(
            "production preflight manifest bind lost shared proof-boundary object".to_string(),
        );
    }

    let direct_report_digest = hex_encode(&direct_report.digest);
    let qrom_action_report_digest = hex_encode(&qrom_action_report.digest);
    let lifecycle_report_digest = hex_encode(&lifecycle_report.digest);
    let proof_report_digest = hex_encode(&proof_report.digest);
    let fri_qrom_report_digest = hex_encode(&fri_qrom_report.digest);
    let fri_aok_report_digest = hex_encode(&fri_aok_report.digest);
    let residual_report_digest = hex_encode(&residual_report.digest);
    if str_at(
        direct_manifest,
        "/digests/direct_theorem_boundary_report_digest",
    )? != direct_report_digest
        || str_at(
            direct_manifest,
            "/digests/direct_qrom_action_boundary_report_digest",
        )? != qrom_action_report_digest
        || str_at(
            direct_manifest,
            "/digests/direct_lifecycle_boundary_report_digest",
        )? != lifecycle_report_digest
        || str_at(
            direct_manifest,
            "/digests/account_creation_proof_boundary_report_digest",
        )? != proof_report_digest
        || str_at(
            proof_manifest,
            "/digests/account_creation_proof_boundary_report_digest",
        )? != proof_report_digest
        || str_at(
            proof_manifest,
            "/digests/account_stark_fri_qrom_residual_report_digest",
        )? != fri_qrom_report_digest
        || str_at(
            proof_manifest,
            "/digests/account_stark_fri_aok_boundary_report_digest",
        )? != fri_aok_report_digest
        || str_at(
            proof_manifest,
            "/digests/account_stark_residual_matrix_report_digest",
        )? != residual_report_digest
    {
        return Err("production preflight manifest digests drifted from reports".to_string());
    }

    if at(
        direct_boundary,
        "/direct_accepted_composition_evidence_artifact",
    )? != at(
        direct_boundary,
        "/direct_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact",
    )? || at(
        direct_boundary,
        "/direct_accepted_composition_evidence_artifact",
    )? != at(
        proof_boundary,
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact",
    )? || at(proof_boundary, "/accepted_proof_boundary_evidence_artifact")?
        != at(
            proof_boundary,
            "/proof_obligation_vector_corpus_evidence/accepted_proof_boundary_evidence_artifact",
        )?
        || at(proof_boundary, "/accepted_proof_boundary_evidence_artifact")?
            != at(
                direct_boundary,
                "/direct_accepted_composition_evidence_artifact/account_creation_accepted_proof_boundary_evidence_artifact",
            )?
    {
        return Err(
            "production preflight manifest bind lost accepted evidence artifact equality"
                .to_string(),
        );
    }

    for pointer in [
        "/direct_accepted_composition_evidence_artifact/audit_summary/direct_theorem_obligations_still_open",
        "/direct_accepted_composition_evidence_artifact/audit_summary/proof_family_obligations_still_open",
        "/direct_accepted_composition_evidence_artifact/audit_summary/verifier_boundary_accepts_creation",
        "/direct_accepted_composition_evidence_artifact/audit_summary/direct_spend_tuple_accepts",
        "/direct_obligation_vector_corpus_evidence/audit_summary/lifecycle_transition_profile_verified",
        "/coverage/every_closure_dependency_has_open_obligation",
        "/coverage/every_closure_dependency_has_open_support_surface",
        "/coverage/every_open_closure_dependency_blocks_deployment",
        "/coverage/every_closure_dependency_has_evidence",
    ] {
        if !bool_at(direct_boundary, pointer)? {
            return Err(format!(
                "production preflight manifest bind lost expected true flag {pointer}"
            ));
        }
    }
    for pointer in [
        "/direct_accepted_composition_evidence_artifact/audit_summary/composition_theorem_instantiated",
        "/direct_accepted_composition_evidence_artifact/audit_summary/account_creation_proof_boundary_verified",
        "/direct_accepted_composition_evidence_artifact/audit_summary/deployment_ready",
        "/direct_obligation_vector_corpus_evidence/audit_summary/composition_theorem_instantiated",
        "/direct_obligation_vector_corpus_evidence/audit_summary/account_creation_proof_boundary_verified",
        "/direct_obligation_vector_corpus_evidence/audit_summary/proof_boundary_deployment_ready",
        "/direct_obligation_vector_corpus_evidence/audit_summary/qrom_theorem_imports_budget",
        "/direct_obligation_vector_corpus_evidence/audit_summary/qrom_external_review_complete",
        "/direct_obligation_vector_corpus_evidence/audit_summary/qrom_and_action_bounds_linked_to_theorem",
        "/direct_obligation_vector_corpus_evidence/audit_summary/deployment_ready",
    ] {
        if bool_at(direct_boundary, pointer)? {
            return Err(format!(
                "production preflight manifest bind unexpectedly closed {pointer}"
            ));
        }
    }
    for pointer in [
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/deployment_ready",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_review_evidence/audit_summary/adoption_evidence_ready",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_review_evidence/audit_summary/deployment_ready",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/adoption_evidence_ready",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/deployment_ready",
    ] {
        if bool_at(direct_manifest, pointer)? {
            return Err(format!(
                "production preflight manifest bind unexpectedly closed {pointer}"
            ));
        }
    }
    if !bool_at(
        direct_boundary,
        "/direct_obligation_vector_corpus_evidence/audit_summary/lifecycle_target_chain_finality_profile_instantiated",
    )? {
        return Err(
            "production preflight manifest bind lost D2 finality instantiation evidence"
                .to_string(),
        );
    }
    for pointer in [
        "/direct_lifecycle_boundary/coverage/transition_vector_corpus_pinned",
        "/direct_lifecycle_boundary/coverage/target_chain_lifecycle_profile_review_pinned",
        "/direct_lifecycle_boundary/coverage/target_chain_lifecycle_profile_verification_pinned",
        "/direct_lifecycle_boundary/coverage/lifecycle_transition_profile_verified",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/all_cases_have_support_tests",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/all_support_tests_present",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/all_cases_have_evidence",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/covers_admit",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/covers_spend",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/covers_replay",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/covers_burn",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/covers_recovery",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/covers_restore",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/covers_reorg",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/covers_d2_finality_window",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/transition_vector_corpus_pinned",
        "/direct_lifecycle_boundary/transition_vector_corpus_evidence/audit_summary/lifecycle_transition_profile_verified",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_review_evidence/audit_summary/all_requirements_have_authority",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_review_evidence/audit_summary/all_requirements_reviewed",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_review_evidence/audit_summary/all_reviewed_requirements_have_evidence_surface",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_review_evidence/audit_summary/all_relaxations_require_new_profile",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_review_evidence/audit_summary/d2_safe_default_profile_reviewed",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_review_evidence/audit_summary/target_chain_finality_profile_instantiated",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_review_evidence/audit_summary/target_chain_lifecycle_profile_review_pinned",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_review_evidence/audit_summary/lifecycle_transition_profile_verified",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/all_requirements_reviewed",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/all_review_requirements_verified",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/all_requirements_have_executable_evidence",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/all_support_tests_present",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/all_residuals_non_deployment_claims",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/d2_safe_default_profile_verified",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/target_chain_finality_profile_instantiated",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/target_chain_lifecycle_profile_review_pinned",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/target_chain_lifecycle_profile_verification_pinned",
        "/direct_lifecycle_boundary/target_chain_lifecycle_profile_verification_evidence/audit_summary/lifecycle_transition_profile_verified",
    ] {
        if !bool_at(direct_manifest, pointer)? {
            return Err(format!(
                "production preflight manifest bind lost lifecycle transition evidence {pointer}"
            ));
        }
    }
    for pointer in [
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/direct_theorem_obligations_still_open",
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/proof_family_obligations_still_open",
        "/accepted_proof_boundary_evidence_artifact/audit_summary/production_gate_fail_closed",
        "/accepted_proof_boundary_evidence_artifact/audit_summary/proof_family_obligations_still_open",
        "/accepted_proof_boundary_evidence_artifact/audit_summary/every_applicable_accepting_requirement_has_accepted_path_evidence",
        "/accepted_proof_boundary_evidence_artifact/audit_summary/every_accepting_requirement_has_boundary_or_audit_evidence",
        "/accepted_proof_boundary_evidence_artifact/audit_summary/every_separate_closure_requirement_has_open_obligation",
        "/accepted_proof_boundary_evidence_artifact/audit_summary/deterministic_rejection_requirement_deferred_to_rejection_kats",
        "/accepted_proof_boundary_evidence_artifact/audit_summary/qrom_requirement_deferred_to_residual_theorem",
        "/proof_obligation_vector_corpus_evidence/audit_summary/current_no_export_vectors_cover_boundary_cases",
        "/proof_obligation_vector_corpus_evidence/audit_summary/current_fri_qrom_domain_vectors_cover_boundary_cases",
        "/proof_obligation_vector_corpus_evidence/audit_summary/current_payload_kat_vectors_cover_every_boundary_row",
        "/coverage/every_closure_dependency_has_open_obligation",
        "/coverage/every_closure_dependency_has_open_support_surface",
        "/coverage/every_open_closure_dependency_blocks_deployment",
        "/coverage/every_closure_dependency_has_evidence",
    ] {
        if !bool_at(proof_boundary, pointer)? {
            return Err(format!(
                "production preflight proof-boundary manifest lost expected true flag {pointer}"
            ));
        }
    }
    for (pointer, expected) in [
        (
            "/accepted_proof_boundary_evidence_artifact/audit_summary/accepted_requirement_evidence_count",
            10,
        ),
        (
            "/accepted_proof_boundary_evidence_artifact/audit_summary/accepted_path_applicable_requirement_count",
            8,
        ),
        (
            "/accepted_proof_boundary_evidence_artifact/audit_summary/accepted_path_requirement_evidence_count",
            8,
        ),
        (
            "/accepted_proof_boundary_evidence_artifact/audit_summary/boundary_or_audit_requirement_evidence_count",
            10,
        ),
        (
            "/accepted_proof_boundary_evidence_artifact/audit_summary/separate_closure_required_count",
            4,
        ),
    ] {
        if u64_at(proof_boundary, pointer)? != expected {
            return Err(format!(
                "production preflight proof-boundary manifest lost accepted requirement evidence count {pointer}"
            ));
        }
    }
    if at(
        proof_boundary,
        "/accepted_proof_boundary_evidence_artifact/audit_summary/accepted_requirement_ids",
    )? != at(proof_boundary, "/accepting_requirements")?
    {
        return Err(
            "production preflight proof-boundary accepted evidence lost requirement identity"
                .to_string(),
        );
    }
    require_requirement_coverage_accepted_evidence_identity(
        proof_boundary,
        accepted_proof_boundary_evidence,
        "production preflight proof-boundary",
    )?;
    require_requirement_coverage_accepted_evidence_identity(
        at(direct_manifest, "/account_creation_proof_boundary")?,
        accepted_proof_boundary_evidence,
        "production preflight direct-imported proof-boundary",
    )?;
    require_direct_accepted_tuple_field_evidence_identity(
        direct_boundary,
        accepted_direct_tuple_evidence,
        "production preflight direct theorem boundary",
    )?;
    require_direct_accepted_hypothesis_evidence_identity(
        direct_boundary,
        accepted_direct_composition_evidence,
        "production preflight direct theorem boundary",
    )?;
    if !bool_at(
        proof_boundary,
        "/proof_obligation_vector_corpus_evidence/audit_summary/frozen_kat_corpus_complete",
    )? {
        return Err(
            "production preflight proof-boundary manifest lost frozen payload KAT evidence"
                .to_string(),
        );
    }
    if str_at(
        proof_boundary,
        "/requirement_vector_corpus_evidence/payload_kat_vector_evidence/payload_kat_corpus_reduction_lean_theorem",
    )? != "selectedRustAcceptedBoundaryPayloadKatCorpusReduction_v1"
        || !bool_at(
            proof_boundary,
            "/requirement_vector_corpus_evidence/payload_kat_vector_evidence/payload_kat_corpus_reduction_bound",
        )?
    {
        return Err(
            "production preflight proof-boundary manifest lost payload KAT reduction evidence"
                .to_string(),
        );
    }
    if str_at(
        proof_boundary,
        "/requirement_vector_corpus_evidence/no_export_vector_evidence/accepted_boundary_simulator_no_export_reduction_lean_theorem",
    )? != "selectedRustAcceptedBoundarySimulatorNoExportReduction_v1"
        || !bool_at(
            proof_boundary,
            "/requirement_vector_corpus_evidence/no_export_vector_evidence/accepted_boundary_simulator_no_export_reduction_bound",
        )?
    {
        return Err(
            "production preflight proof-boundary manifest lost no-export reduction evidence"
                .to_string(),
        );
    }
    for pointer in [
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/composition_theorem_instantiated",
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/account_creation_proof_boundary_verified",
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/deployment_ready",
        "/proof_obligation_vector_corpus_evidence/audit_summary/simulator_theorem_instantiated",
        "/proof_obligation_vector_corpus_evidence/audit_summary/fri_aok_extractor_theorem_instantiated",
        "/proof_obligation_vector_corpus_evidence/audit_summary/fri_qrom_residual_priced",
        "/proof_obligation_vector_corpus_evidence/audit_summary/external_review_complete",
        "/proof_obligation_vector_corpus_evidence/audit_summary/production_gate_open",
        "/proof_obligation_vector_corpus_evidence/audit_summary/deployment_ready",
    ] {
        if bool_at(proof_boundary, pointer)? {
            return Err(format!(
                "production preflight proof-boundary manifest unexpectedly closed {pointer}"
            ));
        }
    }
    if at(
        proof_boundary,
        "/proof_obligation_vector_corpus_evidence/audit_summary/proof_obligation_named_break_classes",
    )? != at(proof_boundary, "/proof_obligation_named_break_classes")?
    {
        return Err(
            "production preflight proof-obligation summary lost named-break taxonomy".to_string(),
        );
    }
    let proof_obligation_row_ids =
        array_field_values_at(proof_boundary, "/proof_obligation_rows", "obligation_id")?;
    if at(
        proof_boundary,
        "/proof_obligation_vector_corpus_evidence/proof_obligation_case_ids",
    )? != &proof_obligation_row_ids
    {
        return Err(
            "production preflight proof-obligation summary lost case-id row binding".to_string(),
        );
    }

    if direct_report.deployment_ready
        || direct_report.composition_theorem_instantiated
        || direct_report.account_creation_proof_boundary_verified
        || direct_report.qrom_and_action_bounds_linked_to_theorem
    {
        return Err("direct theorem boundary unexpectedly reports closed obligations".to_string());
    }
    if !direct_report.lifecycle_transition_profile_verified {
        return Err(
            "direct theorem boundary has not imported verified lifecycle profile".to_string(),
        );
    }
    if !direct_report.checked_core_frame_binding_imported {
        return Err(
            "direct theorem boundary has not imported checked-core frame binding".to_string(),
        );
    }
    if !direct_report.slot_authorization_digest_imported {
        return Err(
            "direct theorem boundary has not imported slot authorization digest".to_string(),
        );
    }
    if qrom_action_report.deployment_ready
        || qrom_action_report.theorem_imports_budget
        || qrom_action_report.external_qrom_review_complete
        || qrom_action_report.qrom_and_action_bounds_linked_to_theorem
    {
        return Err(
            "direct QROM/action boundary unexpectedly reports closed obligations".to_string(),
        );
    }
    if !qrom_action_report.checked_core_qrom_budget_pinned
        || !qrom_action_report.action_map_budget_pinned
        || !qrom_action_report.direct_mode_q_chal_active
        || !qrom_action_report.direct_mode_q_batch_inactive
        || !qrom_action_report.optional_aggregation_terms_q_batch_only
        || !qrom_action_report.selected_horizon_within_margin
        || !qrom_action_report.negative_control_exceeds_margin
    {
        return Err("direct QROM/action boundary is missing budget evidence".to_string());
    }
    if lifecycle_report.deployment_ready || !lifecycle_report.lifecycle_transition_profile_verified
    {
        return Err(
            "direct lifecycle boundary lost verified profile or unexpectedly became deployable"
                .to_string(),
        );
    }
    if !lifecycle_report.target_chain_finality_profile_instantiated {
        return Err("direct lifecycle boundary lost D2 finality instantiation".to_string());
    }
    if !lifecycle_report.local_successor_state_model_pinned
        || !lifecycle_report.consume_once_insert_after_public_checks
        || !lifecycle_report.replay_rejected_by_consume_set
        || !lifecycle_report.spent_slot_ledger_rejects_stale_wallet_replay
        || !lifecycle_report.chain_state_persists_lifecycle_and_spent_record
        || !lifecycle_report.reorg_burn_support_present
        || !lifecycle_report.restore_anchor_support_present
        || !lifecycle_report.recovery_burn_support_present
        || !lifecycle_report.snapshot_recovery_support_present
        || !lifecycle_report.file_store_recovery_support_present
        || !lifecycle_report.target_chain_lifecycle_profile_review_pinned
        || !lifecycle_report.target_chain_lifecycle_profile_verification_pinned
    {
        return Err(
            "direct lifecycle boundary is missing local successor-state evidence".to_string(),
        );
    }
    if proof_report.deployment_ready
        || proof_report.simulator_theorem_instantiated
        || proof_report.fri_aok_extractor_theorem_instantiated
        || proof_report.fri_qrom_residual_priced
        || !proof_report.frozen_kat_corpus_complete
        || proof_report.external_review_complete
        || proof_report.production_gate_open
    {
        return Err("account-creation proof boundary has inconsistent closure status".to_string());
    }
    let open_proof_closure_dependency_count = proof_report
        .closure_dependency_rows
        .iter()
        .filter(|row| row.open)
        .count();
    if !proof_report.proof_obligation_rows_pinned
        || !proof_report.every_proof_obligation_has_named_break
        || !proof_report.every_open_proof_obligation_blocks_deployment
        || !proof_report.every_closure_dependency_has_open_obligation
        || !proof_report.every_closure_dependency_has_open_support_surface
        || !proof_report.every_open_closure_dependency_blocks_deployment
        || !proof_report.every_closure_dependency_has_evidence
        || proof_report.proof_obligation_count != proof_report.proof_obligation_rows.len()
        || proof_report.closure_dependency_count != proof_report.closure_dependency_rows.len()
        || proof_report.closure_dependency_evidence_count
            != proof_report.closure_dependency_rows.len()
        || proof_report.closure_dependency_open_obligation_evidence_count
            != open_proof_closure_dependency_count
        || proof_report.closure_dependency_open_support_evidence_count
            != open_proof_closure_dependency_count
        || proof_report.closure_dependency_count != proof_report.proof_obligation_rows.len()
    {
        return Err(
            "account-creation proof boundary lost proof-obligation or closure-dependency blocker coverage".to_string(),
        );
    }
    require_proof_closure_dependency_evidence_identity(
        proof_boundary,
        "production preflight proof-boundary manifest",
    )?;
    if fri_qrom_report.deployment_ready
        || fri_qrom_report.residual_pricing_complete
        || !fri_qrom_report.account_stark_fri_qrom_transcript_reduction_bound
        || fri_qrom_report.account_stark_transcript_qrom_theorem_instantiated
        || fri_qrom_report.recursion_pricing_complete
        || fri_qrom_report.external_fri_review_complete
        || fri_qrom_report.external_qrom_review_complete
        || fri_qrom_report.deployment_profile_accepted
    {
        return Err(
            "account-STARK FRI/QROM residual unexpectedly reports closed obligations".to_string(),
        );
    }
    if fri_aok_report.deployment_ready
        || !fri_aok_report.fri_aok_extractor_reduction_bound
        || fri_aok_report.fri_aok_extractor_theorem_instantiated
        || fri_aok_report.accepting_proof_implies_extractable_witness
        || fri_aok_report.external_extractor_review_complete
    {
        return Err(
            "account-STARK FRI/AoK boundary unexpectedly reports closed obligations".to_string(),
        );
    }
    if residual_report.deployment_ready {
        return Err(
            "account-STARK residual matrix unexpectedly reports deployment readiness".to_string(),
        );
    }
    if !residual_report.every_row_has_blocker || !residual_report.every_row_has_named_break {
        return Err(
            "account-STARK residual matrix rows are missing blockers or named breaks".to_string(),
        );
    }

    for pointer in [
        "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/every_accepted_tuple_field_has_evidence",
        "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/every_accepted_tuple_field_has_open_or_imported_direct_obligation",
        "/direct_accepted_composition_evidence_artifact/audit_summary/every_positive_acceptance_hypothesis_has_accepted_path_evidence",
        "/direct_accepted_composition_evidence_artifact/audit_summary/public_rejection_hypothesis_deferred_to_rejection_kats",
    ] {
        if !bool_at(direct_boundary, pointer)? {
            return Err(format!(
                "production preflight manifest lost accepted direct evidence flag {pointer}"
            ));
        }
    }
    for (pointer, expected) in [
        (
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/accepted_tuple_field_evidence_count",
            11,
        ),
        (
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/accepted_tuple_field_evidence_present_count",
            11,
        ),
        (
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/accepted_tuple_field_open_obligation_count",
            6,
        ),
        (
            "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/accepted_tuple_field_imported_closure_count",
            2,
        ),
        (
            "/direct_accepted_composition_evidence_artifact/audit_summary/accepted_hypothesis_evidence_count",
            9,
        ),
        (
            "/direct_accepted_composition_evidence_artifact/audit_summary/positive_acceptance_hypothesis_evidence_count",
            8,
        ),
    ] {
        if u64_at(direct_boundary, pointer)? != expected {
            return Err(format!(
                "production preflight manifest lost accepted direct evidence count {pointer}"
            ));
        }
    }
    if bool_at(
        direct_boundary,
        "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/every_accepted_tuple_field_has_open_direct_obligation",
    )? {
        return Err(
            "production preflight manifest unexpectedly kept all-open tuple field flag".to_string(),
        );
    }
    if at(
        direct_boundary,
        "/direct_accepted_spend_tuple_evidence_artifact/audit_summary/accepted_tuple_field_ids",
    )? != at(direct_boundary, "/accepted_tuple_fields")?
    {
        return Err(
            "production preflight direct accepted tuple evidence lost field identity".to_string(),
        );
    }
    if at(
        direct_boundary,
        "/direct_accepted_composition_evidence_artifact/audit_summary/accepted_hypothesis_ids",
    )? != at(direct_boundary, "/acceptance_hypotheses")?
    {
        return Err(
            "production preflight direct accepted composition evidence lost hypothesis identity"
                .to_string(),
        );
    }
    let direct_obligation_row_ids =
        array_field_values_at(direct_boundary, "/obligation_rows", "obligation_id")?;
    if at(
        direct_boundary,
        "/direct_accepted_composition_evidence_artifact/audit_summary/accepted_hypothesis_direct_obligation_ids",
    )? != &direct_obligation_row_ids
    {
        return Err(
            "production preflight direct accepted composition evidence lost hypothesis obligation identity"
                .to_string(),
        );
    }
    if at(
        direct_boundary,
        "/direct_obligation_vector_corpus_evidence/audit_summary/direct_named_break_classes",
    )? != at(direct_boundary, "/named_break_classes")?
    {
        return Err(
            "production preflight direct-obligation summary lost named-break taxonomy".to_string(),
        );
    }
    if at(
        direct_boundary,
        "/direct_obligation_vector_corpus_evidence/direct_obligation_case_ids",
    )? != &direct_obligation_row_ids
    {
        return Err(
            "production preflight direct-obligation summary lost case-id row binding".to_string(),
        );
    }

    if !direct_report.composition_theorem_instantiated
        && (!direct_report.obligation_rows.iter().any(|row| row.open)
            || !direct_report
                .obligation_rows
                .iter()
                .filter(|row| row.open)
                .all(|row| row.blocker == direct_report.blocker && contains(blockers, row.blocker))
            || !direct_report
                .obligation_rows
                .iter()
                .filter(|row| !row.open)
                .all(|row| {
                    (row.obligation_id == "direct_obligation_consume_once"
                        && row.acceptance_hypothesis
                            == "lifecycle_transition_consumes_authorization_once")
                        || (row.obligation_id == "direct_obligation_checked_core_frame_binding"
                            && row.acceptance_hypothesis
                                == "checked_core_challenge_binds_public_spend_context"
                            && direct_report.checked_core_frame_binding_imported)
                        || (row.obligation_id == "direct_obligation_slot_authorization_digest"
                            && row.acceptance_hypothesis
                                == "slot_authorization_digest_matches_canonical_leaf"
                            && direct_report.slot_authorization_digest_imported)
                }))
    {
        return Err(
            "direct composition theorem flag is not backed by all open direct obligations"
                .to_string(),
        );
    }
    if !direct_report.every_closure_dependency_has_open_obligation
        || !direct_report.every_closure_dependency_has_open_support_surface
        || !direct_report.every_open_closure_dependency_blocks_deployment
        || !direct_report.every_closure_dependency_has_evidence
        || direct_report.closure_dependency_count != direct_report.closure_dependency_rows.len()
        || direct_report.closure_dependency_evidence_count
            != direct_report.closure_dependency_rows.len()
        || direct_report.closure_dependency_open_obligation_evidence_count
            != direct_report
                .closure_dependency_rows
                .iter()
                .filter(|row| row.open)
                .count()
        || direct_report.closure_dependency_open_support_evidence_count
            != direct_report
                .closure_dependency_rows
                .iter()
                .filter(|row| row.open)
                .count()
        || direct_report.closure_dependency_count != direct_report.obligation_rows.len()
    {
        return Err("direct theorem boundary lost closure-dependency blocker coverage".to_string());
    }
    require_direct_closure_dependency_evidence_identity(
        direct_boundary,
        "production preflight direct theorem manifest",
    )?;
    if !direct_report.account_creation_proof_boundary_verified {
        require_open_direct_obligation(
            &direct_report,
            blockers,
            "account_creation_proof_boundary_verified",
            "direct_obligation_creation_before_challenge",
            "selected_creation_verifier_accepted_before_spend_challenge",
            "Path2AccountCreationProofBoundaryReportV1",
            "account-creation proof boundary is verified",
        )?;
        if proof_report.deployment_ready || proof_report.production_gate_open {
            return Err(
                "direct proof-boundary flag is open while proof-boundary report appears deployable"
                    .to_string(),
            );
        }
    }
    if direct_report.lifecycle_transition_profile_verified {
        let consume_once_row = direct_report
            .obligation_rows
            .iter()
            .find(|row| row.obligation_id == "direct_obligation_consume_once")
            .ok_or_else(|| "direct theorem boundary lost consume-once row".to_string())?;
        if consume_once_row.open
            || !lifecycle_report.target_chain_finality_profile_instantiated
            || !lifecycle_report.lifecycle_transition_profile_verified
            || lifecycle_report.deployment_ready
        {
            return Err(
                "direct lifecycle import is not backed by verified non-deployable lifecycle boundary"
                    .to_string(),
            );
        }
    }
    if direct_report.checked_core_frame_binding_imported {
        let checked_core_row = direct_report
            .obligation_rows
            .iter()
            .find(|row| row.obligation_id == "direct_obligation_checked_core_frame_binding")
            .ok_or_else(|| {
                "direct theorem boundary lost checked-core frame-binding row".to_string()
            })?;
        if checked_core_row.open
            || checked_core_row.acceptance_hypothesis
                != "checked_core_challenge_binds_public_spend_context"
            || checked_core_row.named_break != "checked_core_message_binding_break"
        {
            return Err(
                "direct checked-core frame-binding import is not backed by a closed direct row"
                    .to_string(),
            );
        }
    }
    if direct_report.slot_authorization_digest_imported {
        let slot_authorization_row = direct_report
            .obligation_rows
            .iter()
            .find(|row| row.obligation_id == "direct_obligation_slot_authorization_digest")
            .ok_or_else(|| {
                "direct theorem boundary lost slot-authorization digest row".to_string()
            })?;
        if slot_authorization_row.open
            || slot_authorization_row.acceptance_hypothesis
                != "slot_authorization_digest_matches_canonical_leaf"
            || slot_authorization_row.named_break != "prevalidation_leaf_binding_break"
        {
            return Err(
                "direct slot-authorization digest import is not backed by a closed direct row"
                    .to_string(),
            );
        }
    }
    if !direct_report.qrom_and_action_bounds_linked_to_theorem {
        require_open_direct_obligation(
            &direct_report,
            blockers,
            "qrom_and_action_bounds_linked_to_theorem",
            "direct_obligation_checked_core_acceptance",
            "checked_core_verification_accepts_poseidon2_and_fq12_action",
            "Fq12 action map",
            "QROM/action-map numeric bounds",
        )?;
        if !contains(blockers, qrom_action_report.blocker) {
            return Err(
                "direct QROM/action closure flag is open but QROM/action blocker is absent"
                    .to_string(),
            );
        }
    }
    if !direct_report.deployment_ready && preflight.deployment_ready {
        return Err("direct theorem is open while production preflight is ready".to_string());
    }

    if !proof_report.simulator_theorem_instantiated {
        require_open_proof_obligation(
            &proof_report,
            blockers,
            "simulator_theorem_instantiated",
            "creation_proof_obligation_simulator_no_export",
            "selected_account_stark_public_bundle_simulator_v1",
            "Path2AccountCreationNoExportBoundaryReportV1",
            "accepted public bundles are simulatable",
        )?;
    }
    if !proof_report.fri_aok_extractor_theorem_instantiated {
        require_open_proof_obligation(
            &proof_report,
            blockers,
            "fri_aok_extractor_theorem_instantiated",
            "creation_proof_obligation_fri_aok_extractor",
            "selected_plonky3_q12289_fri_iop_aok_extractor",
            "Path2AccountStarkFriAokBoundaryReportV1",
            "accepting proof",
        )?;
    }
    if !proof_report.fri_qrom_residual_priced {
        require_open_proof_obligation(
            &proof_report,
            blockers,
            "fri_qrom_residual_priced",
            "creation_proof_obligation_fri_qrom_residual_pricing",
            "selected_account_stark_fri_poseidon2_qrom_residual_ledger_v1",
            "Path2AccountStarkFriQromResidualReportV1",
            "deployment profile",
        )?;
    }
    if !proof_report.frozen_kat_corpus_complete {
        require_open_proof_obligation(
            &proof_report,
            blockers,
            "frozen_kat_corpus_complete",
            "creation_proof_obligation_frozen_payload_kats",
            "selected_account_stark_payload_frozen_kat_corpus_v1",
            "Path2AccountStarkPayloadKatBoundaryReportV1",
            "freeze the full byte corpus",
        )?;
    }
    if !proof_report.production_gate_open || !proof_report.external_review_complete {
        require_open_proof_obligation(
            &proof_report,
            blockers,
            "production_gate_open_or_external_review_complete",
            "creation_proof_obligation_production_gate_and_review",
            "selected_account_stark_production_admission_gate_v1",
            "Path2AccountStarkResidualMatrixReportV1",
            "external review",
        )?;
    }
    if !proof_report.deployment_ready
        && !proof_report.proof_obligation_rows.iter().all(|row| {
            !row.open || (row.blocker == proof_report.blocker && contains(blockers, row.blocker))
        })
    {
        return Err(
            "proof-boundary deployment flag is not backed by all open proof obligations"
                .to_string(),
        );
    }

    if !residual_report.deployment_ready
        && !residual_report
            .rows
            .iter()
            .all(|row| row.open && contains(blockers, row.blocker))
    {
        return Err(
            "residual deployment flag is not backed by all open residual blockers".to_string(),
        );
    }
    require_open_residual_item(
        &residual_report,
        blockers,
        "Path2AccountStarkCreationPayloadV1 byte and replay binding",
        "frozen byte corpus",
    )?;
    require_open_residual_item(
        &residual_report,
        blockers,
        "AIR relation soundness",
        "AIR constraints match",
    )?;
    require_open_residual_item(
        &residual_report,
        blockers,
        "FRI argument-of-knowledge extractor",
        "extractable witness",
    )?;
    require_open_residual_item(
        &residual_report,
        blockers,
        "Zero-knowledge simulator and no-export statement",
        "simulator theorem",
    )?;
    require_open_residual_item(
        &residual_report,
        blockers,
        "FRI soundness margin and recursion ledger",
        "residual-assumption ledger prices",
    )?;
    require_open_residual_item(
        &residual_report,
        blockers,
        "Production admission gate and review disposition",
        "external review",
    )?;
    require_open_residual_item(
        &residual_report,
        blockers,
        "Legacy proof-route disposition",
        "route change requires ADR",
    )?;

    let direct_blockers = production_deployment_direct_theorem_blockers();
    let proof_blockers = production_deployment_account_creation_proof_boundary_blockers();
    let residual_blockers = production_deployment_account_stark_residual_blockers();
    if u64_at(direct_boundary, "/coverage/obligation_count")?
        != direct_report.obligation_count as u64
        || u64_at(direct_boundary, "/coverage/closure_dependency_count")?
            != direct_report.closure_dependency_count as u64
        || u64_at(
            direct_boundary,
            "/coverage/closure_dependency_evidence_count",
        )? != direct_report.closure_dependency_evidence_count as u64
        || u64_at(
            direct_boundary,
            "/coverage/closure_dependency_open_obligation_evidence_count",
        )? != direct_report.closure_dependency_open_obligation_evidence_count as u64
        || u64_at(
            direct_boundary,
            "/coverage/closure_dependency_open_support_evidence_count",
        )? != direct_report.closure_dependency_open_support_evidence_count as u64
        || u64_at(
            direct_boundary,
            "/direct_obligation_vector_corpus_evidence/case_count",
        )? != direct_report.obligation_audit_vector_case_count as u64
        || u64_at(proof_boundary, "/coverage/proof_obligation_count")?
            != proof_report.proof_obligation_count as u64
        || u64_at(proof_boundary, "/coverage/closure_dependency_count")?
            != proof_report.closure_dependency_count as u64
        || u64_at(
            proof_boundary,
            "/coverage/closure_dependency_evidence_count",
        )? != proof_report.closure_dependency_evidence_count as u64
        || u64_at(
            proof_boundary,
            "/coverage/closure_dependency_open_obligation_evidence_count",
        )? != proof_report.closure_dependency_open_obligation_evidence_count as u64
        || u64_at(
            proof_boundary,
            "/coverage/closure_dependency_open_support_evidence_count",
        )? != proof_report.closure_dependency_open_support_evidence_count as u64
        || u64_at(
            proof_boundary,
            "/proof_obligation_vector_corpus_evidence/case_count",
        )? != proof_report.proof_obligation_audit_vector_case_count as u64
    {
        return Err("production preflight manifest bind lost obligation case counts".to_string());
    }
    let direct_start = find_subsequence(blockers, direct_blockers)
        .ok_or_else(|| "production preflight is missing direct theorem blockers".to_string())?;
    let proof_start = require_subsequence_after(
        blockers,
        direct_start,
        direct_blockers.len(),
        proof_blockers,
        "account-creation proof-boundary",
    )?;
    require_subsequence_after(
        blockers,
        proof_start,
        proof_blockers.len(),
        residual_blockers,
        "account-STARK residual",
    )?;

    if !contains(blockers, direct_report.blocker) {
        return Err("production preflight does not include direct theorem blocker".to_string());
    }
    if !contains(blockers, proof_report.blocker) {
        return Err("production preflight does not include proof-boundary blocker".to_string());
    }
    if !contains(blockers, fri_aok_report.blocker) {
        return Err("production preflight does not include FRI/AoK boundary blocker".to_string());
    }
    for row in direct_report.obligation_rows {
        if row.open && !contains(blockers, row.blocker) {
            return Err(format!(
                "production preflight is missing blocker for direct obligation {}",
                row.obligation_id
            ));
        }
    }
    for row in residual_report.rows {
        if row.open && !contains(blockers, row.blocker) {
            return Err(format!(
                "production preflight is missing blocker for residual item {}",
                row.residual_item
            ));
        }
    }
    let manifest_direct_rows = array_at(direct_boundary, "/obligation_rows")?;
    if manifest_direct_rows.len() != direct_report.obligation_rows.len() {
        return Err("direct manifest obligation row count drifted from preflight".to_string());
    }
    for row in manifest_direct_rows {
        if bool_at(row, "/open")? {
            let blocker = str_at(row, "/blocker")?;
            if !contains(blockers, blocker) {
                return Err(format!(
                    "production preflight is missing manifest blocker for direct obligation {}",
                    str_at(row, "/obligation_id")?
                ));
            }
        }
    }
    let manifest_proof_rows = array_at(proof_boundary, "/proof_obligation_rows")?;
    if manifest_proof_rows.len() != proof_report.proof_obligation_rows.len() {
        return Err(
            "proof-boundary manifest obligation row count drifted from preflight".to_string(),
        );
    }
    for row in manifest_proof_rows {
        if bool_at(row, "/open")? {
            let blocker = str_at(row, "/blocker")?;
            if !contains(blockers, blocker) {
                return Err(format!(
                    "production preflight is missing manifest blocker for proof obligation {}",
                    str_at(row, "/obligation_id")?
                ));
            }
        }
    }

    println!("Path 2 preflight boundary consistency check passed.");
    Ok(())
}
