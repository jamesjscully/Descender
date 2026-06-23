use descender_parameter_search::production_reference::{
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_direct_theorem_boundary_report_v1, path2_direct_theorem_boundary_report_v1_digest,
    validate_path2_account_creation_proof_boundary_report_v1,
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
    let corpus_path = "formal/kat/path2_account_creation_proof_obligation_vectors_v1.json";
    let corpus_file = load_json(&root, corpus_path)?;
    let corpus = at(&corpus_file, "/corpus")?;
    let corpus_json = serde_json::to_string_pretty(corpus)
        .map_err(|err| format!("serialize proof-obligation corpus body: {err}"))?;
    let recomputed_digest = hex_encode(&Sha256::digest(corpus_json.as_bytes()));
    if str_at(&corpus_file, "/corpus_digest")? != recomputed_digest {
        return Err("proof-obligation vector corpus digest mismatch".to_string());
    }

    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    let proof_digest = path2_account_creation_proof_boundary_report_v1_digest(&proof_report);
    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    let direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);

    if str_at(corpus, "/schema")?
        != "Descender.Path2.AccountCreationProofObligationVectors.v1"
    {
        return Err("proof-obligation vector corpus schema mismatch".to_string());
    }
    if str_at(corpus, "/status")? != "digest_pinned_proof_obligation_audit_vectors" {
        return Err("proof-obligation vector corpus status mismatch".to_string());
    }
    if str_at(corpus, "/proof_boundary_report/digest")? != hex_encode(&proof_report.digest)
        || str_at(corpus, "/proof_boundary_report/recomputed_digest")?
            != hex_encode(&proof_digest)
    {
        return Err("proof-obligation vector corpus is not bound to proof report".to_string());
    }
    if str_at(corpus, "/direct_theorem_report/digest")? != hex_encode(&direct_report.digest)
        || str_at(corpus, "/direct_theorem_report/recomputed_digest")?
            != hex_encode(&direct_digest)
        || str_at(
            corpus,
            "/direct_theorem_report/account_creation_proof_boundary_report_digest",
        )? != hex_encode(&proof_report.digest)
    {
        return Err("proof-obligation vector corpus is not bound to direct report".to_string());
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
            "proof-obligation vector corpus lost direct accepted-tuple evidence hook"
                .to_string(),
        );
    }
    if str_at(
        corpus,
        "/proof_boundary_report/accepted_boundary_evidence_schema",
    )? != proof_report.accepted_boundary_evidence_schema
        || str_at(
            corpus,
            "/proof_boundary_report/accepted_boundary_evidence_endpoint",
        )? != proof_report.accepted_boundary_evidence_endpoint
        || bool_at(
            corpus,
            "/proof_boundary_report/accepted_boundary_evidence_present",
        )? != proof_report.accepted_boundary_evidence_present
    {
        return Err(
            "proof-obligation vector corpus lost accepted proof-boundary evidence hook"
                .to_string(),
        );
    }
    if direct_report.account_creation_proof_boundary_report_digest != proof_report.digest {
        return Err("direct theorem report lost proof-boundary binding".to_string());
    }

    if u64_at(corpus, "/case_count")? != proof_report.proof_obligation_count as u64
        || u64_at(corpus, "/audit_summary/proof_obligation_count")?
            != proof_report.proof_obligation_count as u64
        || u64_at(
            corpus,
            "/audit_summary/proof_obligation_audit_vector_case_count",
        )? != proof_report.proof_obligation_audit_vector_case_count as u64
    {
        return Err("proof-obligation vector corpus case count drifted".to_string());
    }
    if !bool_at(corpus, "/proof_boundary_report/proof_obligation_rows_pinned")?
        || !bool_at(
            corpus,
            "/proof_boundary_report/every_proof_obligation_has_named_break",
        )?
        || !bool_at(
            corpus,
            "/proof_boundary_report/every_open_proof_obligation_blocks_deployment",
        )?
        || !bool_at(corpus, "/audit_summary/all_cases_open")?
        || !bool_at(corpus, "/audit_summary/all_cases_have_named_break")?
        || !bool_at(corpus, "/audit_summary/all_open_cases_block_deployment")?
    {
        return Err("proof-obligation vector corpus lost fail-closed row invariants".to_string());
    }
    for pointer in [
        "/proof_boundary_report/deployment_ready",
        "/direct_theorem_report/composition_theorem_instantiated",
        "/direct_theorem_report/account_creation_proof_boundary_verified",
        "/direct_theorem_report/deployment_ready",
        "/audit_summary/simulator_theorem_instantiated",
        "/audit_summary/fri_aok_extractor_theorem_instantiated",
        "/audit_summary/fri_qrom_residual_priced",
        "/audit_summary/frozen_kat_corpus_complete",
        "/audit_summary/external_review_complete",
        "/audit_summary/production_gate_open",
        "/audit_summary/deployment_ready",
    ] {
        if bool_at(corpus, pointer)? {
            return Err(format!(
                "proof-obligation vector corpus unexpectedly closes {pointer}"
            ));
        }
    }

    let cases = at(corpus, "/cases")?
        .as_array()
        .ok_or_else(|| "proof-obligation corpus cases are not an array".to_string())?;
    if cases.len() != proof_report.proof_obligation_rows.len() {
        return Err("proof-obligation corpus cases array drifted".to_string());
    }
    for (index, ((case, row), expected_case_id)) in cases
        .iter()
        .zip(proof_report.proof_obligation_rows.iter())
        .zip(proof_report.proof_obligation_audit_vector_case_ids.iter())
        .enumerate()
    {
        let case_id = case
            .get("case_id")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("proof-obligation case {index} has no case_id"))?;
        if case_id != row.obligation_id || case_id != *expected_case_id {
            return Err(format!(
                "proof-obligation case {index} id {case_id:?} drifted"
            ));
        }
        if case.get("kind").and_then(Value::as_str) != Some("audit") {
            return Err(format!("proof-obligation case {case_id} is not an audit case"));
        }
        for (field, expected) in [
            ("closure_target", row.closure_target),
            ("supporting_surface", row.supporting_surface),
            ("current_evidence", row.current_evidence),
            ("required_closure", row.required_closure),
            ("named_break", row.named_break),
            ("blocker", row.blocker),
        ] {
            if case.get(field).and_then(Value::as_str) != Some(expected) {
                return Err(format!(
                    "proof-obligation case {case_id} field {field} drifted"
                ));
            }
        }
        if case.get("open").and_then(Value::as_bool) != Some(true) || !row.open {
            return Err(format!(
                "proof-obligation case {case_id} unexpectedly closed"
            ));
        }
    }

    println!("Path 2 account-creation proof-obligation vector consistency check passed.");
    Ok(())
}
