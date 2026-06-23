use descender_parameter_search::production_reference::{
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_direct_lifecycle_boundary_report_v1, path2_direct_lifecycle_boundary_report_v1_digest,
    path2_direct_qrom_action_boundary_report_v1,
    path2_direct_qrom_action_boundary_report_v1_digest, path2_direct_theorem_boundary_report_v1,
    path2_direct_theorem_boundary_report_v1_digest,
    validate_path2_account_creation_proof_boundary_report_v1,
    validate_path2_direct_lifecycle_boundary_report_v1,
    validate_path2_direct_qrom_action_boundary_report_v1,
    validate_path2_direct_theorem_boundary_report_v1,
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

fn main() -> Result<(), String> {
    let root = repo_root();
    let corpus_path = "formal/kat/path2_direct_theorem_obligation_vectors_v1.json";
    let corpus_file = load_json(&root, corpus_path)?;
    let corpus = at(&corpus_file, "/corpus")?;
    let corpus_json = serde_json::to_string_pretty(corpus)
        .map_err(|err| format!("serialize direct theorem obligation corpus body: {err}"))?;
    let recomputed_digest = hex_encode(&Sha256::digest(corpus_json.as_bytes()));
    if str_at(&corpus_file, "/corpus_digest")? != recomputed_digest {
        return Err("direct theorem obligation vector corpus digest mismatch".to_string());
    }

    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    let direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    let qrom_action_report = path2_direct_qrom_action_boundary_report_v1();
    validate_path2_direct_qrom_action_boundary_report_v1(&qrom_action_report)?;
    let qrom_action_digest =
        path2_direct_qrom_action_boundary_report_v1_digest(&qrom_action_report);
    let lifecycle_report = path2_direct_lifecycle_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&lifecycle_report)?;
    let lifecycle_digest = path2_direct_lifecycle_boundary_report_v1_digest(&lifecycle_report);
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    let proof_digest = path2_account_creation_proof_boundary_report_v1_digest(&proof_report);

    if str_at(corpus, "/schema")? != "Descender.Path2.DirectTheoremObligationVectors.v1" {
        return Err("direct theorem obligation vector corpus schema mismatch".to_string());
    }
    if str_at(corpus, "/status")? != "digest_pinned_direct_theorem_obligation_audit_vectors" {
        return Err("direct theorem obligation vector corpus status mismatch".to_string());
    }
    if str_at(corpus, "/direct_theorem_report/digest")? != hex_encode(&direct_report.digest)
        || str_at(corpus, "/direct_theorem_report/recomputed_digest")? != hex_encode(&direct_digest)
    {
        return Err(
            "direct theorem obligation vector corpus is not bound to direct report".to_string(),
        );
    }
    if str_at(
        corpus,
        "/direct_theorem_report/accepted_tuple_evidence_schema",
    )? != direct_report.accepted_tuple_evidence_schema
        || str_at(
            corpus,
            "/direct_theorem_report/accepted_tuple_evidence_endpoint",
        )? != direct_report.accepted_tuple_evidence_endpoint
        || bool_at(
            corpus,
            "/direct_theorem_report/accepted_tuple_evidence_present",
        )? != direct_report.accepted_tuple_evidence_present
    {
        return Err(
            "direct theorem obligation vector corpus lost accepted-tuple evidence hook".to_string(),
        );
    }
    if str_at(corpus, "/qrom_action_boundary_report/digest")?
        != hex_encode(&qrom_action_report.digest)
        || str_at(corpus, "/qrom_action_boundary_report/recomputed_digest")?
            != hex_encode(&qrom_action_digest)
    {
        return Err("direct theorem obligation vector corpus lost QROM/action binding".to_string());
    }
    if str_at(corpus, "/lifecycle_boundary_report/digest")? != hex_encode(&lifecycle_report.digest)
        || str_at(corpus, "/lifecycle_boundary_report/recomputed_digest")?
            != hex_encode(&lifecycle_digest)
    {
        return Err("direct theorem obligation vector corpus lost lifecycle binding".to_string());
    }
    if str_at(corpus, "/account_creation_proof_boundary_report/digest")?
        != hex_encode(&proof_report.digest)
        || str_at(
            corpus,
            "/account_creation_proof_boundary_report/recomputed_digest",
        )? != hex_encode(&proof_digest)
    {
        return Err(
            "direct theorem obligation vector corpus lost proof-boundary binding".to_string(),
        );
    }
    if str_at(
        corpus,
        "/account_creation_proof_boundary_report/accepted_boundary_evidence_schema",
    )? != proof_report.accepted_boundary_evidence_schema
        || str_at(
            corpus,
            "/account_creation_proof_boundary_report/accepted_boundary_evidence_endpoint",
        )? != proof_report.accepted_boundary_evidence_endpoint
        || bool_at(
            corpus,
            "/account_creation_proof_boundary_report/accepted_boundary_evidence_present",
        )? != proof_report.accepted_boundary_evidence_present
        || bool_at(
            corpus,
            "/account_creation_proof_boundary_report/accepted_boundary_binding_evidence_present",
        )? != proof_report.accepted_boundary_binding_evidence_present
        || bool_at(
            corpus,
            "/account_creation_proof_boundary_report/replay_payload_digest_binding_evidence_present",
        )? != proof_report.replay_payload_digest_binding_evidence_present
        || bool_at(
            corpus,
            "/account_creation_proof_boundary_report/deployed_anchor_binding_evidence_present",
        )? != proof_report.deployed_anchor_binding_evidence_present
    {
        return Err(
            "direct theorem obligation vector corpus lost accepted proof-boundary evidence hook"
                .to_string(),
        );
    }
    if direct_report.direct_qrom_action_boundary_report_digest != qrom_action_report.digest
        || direct_report.direct_lifecycle_boundary_report_digest != lifecycle_report.digest
        || direct_report.account_creation_proof_boundary_report_digest != proof_report.digest
    {
        return Err("direct theorem report lost imported boundary digest binding".to_string());
    }

    if u64_at(corpus, "/case_count")? != direct_report.obligation_count as u64
        || u64_at(corpus, "/audit_summary/direct_obligation_count")?
            != direct_report.obligation_count as u64
        || u64_at(
            corpus,
            "/audit_summary/direct_obligation_audit_vector_case_count",
        )? != direct_report.obligation_audit_vector_case_count as u64
        || u64_at(
            corpus,
            "/direct_theorem_report/obligation_audit_vector_case_count",
        )? != direct_report.obligation_audit_vector_case_count as u64
    {
        return Err("direct theorem obligation vector corpus case count drifted".to_string());
    }
    if !bool_at(
        corpus,
        "/direct_theorem_report/every_hypothesis_has_obligation",
    )? || !bool_at(
        corpus,
        "/direct_theorem_report/every_obligation_has_named_break",
    )? || !bool_at(
        corpus,
        "/direct_theorem_report/every_open_obligation_blocks_deployment",
    )? || !bool_at(corpus, "/audit_summary/all_cases_open")?
        || !bool_at(corpus, "/audit_summary/all_cases_have_named_break")?
        || !bool_at(corpus, "/audit_summary/all_open_cases_block_deployment")?
    {
        return Err(
            "direct theorem obligation vector corpus lost fail-closed row invariants".to_string(),
        );
    }
    for pointer in [
        "/direct_theorem_report/composition_theorem_instantiated",
        "/direct_theorem_report/account_creation_proof_boundary_verified",
        "/direct_theorem_report/lifecycle_transition_profile_verified",
        "/direct_theorem_report/qrom_and_action_bounds_linked_to_theorem",
        "/direct_theorem_report/deployment_ready",
        "/qrom_action_boundary_report/theorem_imports_budget",
        "/qrom_action_boundary_report/external_qrom_review_complete",
        "/qrom_action_boundary_report/qrom_and_action_bounds_linked_to_theorem",
        "/qrom_action_boundary_report/deployment_ready",
        "/lifecycle_boundary_report/target_chain_finality_profile_instantiated",
        "/lifecycle_boundary_report/lifecycle_transition_profile_verified",
        "/lifecycle_boundary_report/deployment_ready",
        "/account_creation_proof_boundary_report/deployment_ready",
        "/audit_summary/composition_theorem_instantiated",
        "/audit_summary/account_creation_proof_boundary_verified",
        "/audit_summary/lifecycle_transition_profile_verified",
        "/audit_summary/qrom_and_action_bounds_linked_to_theorem",
        "/audit_summary/proof_boundary_deployment_ready",
        "/audit_summary/qrom_theorem_imports_budget",
        "/audit_summary/qrom_external_review_complete",
        "/audit_summary/lifecycle_target_chain_finality_profile_instantiated",
        "/audit_summary/deployment_ready",
    ] {
        if bool_at(corpus, pointer)? {
            return Err(format!(
                "direct theorem obligation vector corpus unexpectedly closes {pointer}"
            ));
        }
    }

    if str_at(corpus, "/qrom_action_boundary_report/direct_obligation_id")?
        != qrom_action_report.direct_obligation_id
        || str_at(corpus, "/lifecycle_boundary_report/direct_obligation_id")?
            != lifecycle_report.direct_obligation_id
    {
        return Err("direct imported boundary obligation ids drifted".to_string());
    }

    let cases = at(corpus, "/cases")?
        .as_array()
        .ok_or_else(|| "direct theorem obligation corpus cases are not an array".to_string())?;
    if cases.len() != direct_report.obligation_rows.len() {
        return Err("direct theorem obligation corpus cases array drifted".to_string());
    }
    for (index, ((case, row), expected_case_id)) in cases
        .iter()
        .zip(direct_report.obligation_rows.iter())
        .zip(direct_report.obligation_audit_vector_case_ids.iter())
        .enumerate()
    {
        let case_id = case
            .get("case_id")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("direct theorem obligation case {index} has no case_id"))?;
        if case_id != row.obligation_id || case_id != *expected_case_id {
            return Err(format!(
                "direct theorem obligation case {index} id {case_id:?} drifted"
            ));
        }
        if case.get("kind").and_then(Value::as_str) != Some("audit") {
            return Err(format!(
                "direct theorem obligation case {case_id} is not an audit case"
            ));
        }
        for (field, expected) in [
            ("acceptance_hypothesis", row.acceptance_hypothesis),
            ("supporting_surface", row.supporting_surface),
            ("current_evidence", row.current_evidence),
            ("required_closure", row.required_closure),
            ("named_break", row.named_break),
            ("blocker", row.blocker),
        ] {
            if case.get(field).and_then(Value::as_str) != Some(expected) {
                return Err(format!(
                    "direct theorem obligation case {case_id} field {field} drifted"
                ));
            }
        }
        if case.get("open").and_then(Value::as_bool) != Some(true) || !row.open {
            return Err(format!(
                "direct theorem obligation case {case_id} unexpectedly closed"
            ));
        }
    }

    println!("Path 2 direct theorem obligation vector consistency check passed.");
    Ok(())
}
