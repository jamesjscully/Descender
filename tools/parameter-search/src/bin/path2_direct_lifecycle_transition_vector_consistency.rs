use descender_parameter_search::production_reference::{
    path2_direct_lifecycle_boundary_report_v1, path2_direct_lifecycle_boundary_report_v1_digest,
    validate_path2_direct_lifecycle_boundary_report_v1,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
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
    at(value, pointer)?
        .as_array()
        .ok_or_else(|| format!("JSON pointer {pointer} is not an array"))?
        .iter()
        .enumerate()
        .map(|(index, item)| {
            item.as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| format!("{pointer}[{index}] is not a string"))
        })
        .collect()
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let corpus_path = "formal/kat/path2_direct_lifecycle_transition_vectors_v1.json";
    let corpus_file = load_json(&root, corpus_path)?;
    let corpus = at(&corpus_file, "/corpus")?;
    let corpus_json = serde_json::to_string_pretty(corpus)
        .map_err(|err| format!("serialize lifecycle transition corpus body: {err}"))?;
    let recomputed_digest = hex_encode(&Sha256::digest(corpus_json.as_bytes()));
    if str_at(&corpus_file, "/corpus_digest")? != recomputed_digest {
        return Err("lifecycle transition vector corpus digest mismatch".to_string());
    }

    let report = path2_direct_lifecycle_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&report)?;
    let lifecycle_digest = path2_direct_lifecycle_boundary_report_v1_digest(&report);
    if lifecycle_digest != report.digest {
        return Err("direct lifecycle boundary report digest mismatch".to_string());
    }

    if str_at(corpus, "/schema")? != "Descender.Path2.DirectLifecycleTransitionVectors.v1" {
        return Err("lifecycle transition vector corpus schema mismatch".to_string());
    }
    if str_at(corpus, "/status")? != "digest_pinned_direct_lifecycle_transition_vectors" {
        return Err("lifecycle transition vector corpus status mismatch".to_string());
    }
    if str_at(corpus, "/lifecycle_boundary_report/digest")? != hex_encode(&report.digest)
        || str_at(corpus, "/lifecycle_boundary_report/recomputed_digest")?
            != hex_encode(&lifecycle_digest)
        || str_at(corpus, "/lifecycle_boundary_report/theorem_target")? != report.theorem_target
        || str_at(corpus, "/lifecycle_boundary_report/direct_obligation_id")?
            != report.direct_obligation_id
        || str_at(corpus, "/lifecycle_boundary_report/acceptance_hypothesis")?
            != report.acceptance_hypothesis
        || str_at(corpus, "/lifecycle_boundary_report/named_break")? != report.named_break
        || str_at(corpus, "/lifecycle_boundary_report/target_chain_finality_profile_id")?
            != report.target_chain_finality_profile_id
        || str_at(corpus, "/lifecycle_boundary_report/blocker")? != report.blocker
    {
        return Err(
            "lifecycle transition vector corpus is not bound to lifecycle report".to_string(),
        );
    }
    if !bool_at(
        corpus,
        "/lifecycle_boundary_report/target_chain_finality_profile_instantiated",
    )? || bool_at(
        corpus,
        "/lifecycle_boundary_report/lifecycle_transition_profile_verified",
    )? || bool_at(corpus, "/lifecycle_boundary_report/deployment_ready")?
    {
        return Err(
            "lifecycle transition vector corpus has unexpected lifecycle readiness flags"
                .to_string(),
        );
    }

    let expected_case_ids = report
        .transition_vector_case_ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>();
    let summary_case_ids = string_vec_at(corpus, "/audit_summary/case_ids")?;
    if summary_case_ids != expected_case_ids {
        return Err("lifecycle transition vector corpus case ids drifted".to_string());
    }
    if u64_at(corpus, "/case_count")? != expected_case_ids.len() as u64
        || u64_at(corpus, "/audit_summary/case_count")? != expected_case_ids.len() as u64
        || u64_at(corpus, "/audit_summary/executable_test_count")?
            != report.executable_tests.len() as u64
    {
        return Err("lifecycle transition vector corpus case counts drifted".to_string());
    }
    for pointer in [
        "/audit_summary/all_cases_have_support_tests",
        "/audit_summary/all_support_tests_present",
        "/audit_summary/all_cases_have_evidence",
        "/audit_summary/covers_admit",
        "/audit_summary/covers_spend",
        "/audit_summary/covers_replay",
        "/audit_summary/covers_burn",
        "/audit_summary/covers_recovery",
        "/audit_summary/covers_restore",
        "/audit_summary/covers_reorg",
        "/audit_summary/covers_d2_finality_window",
        "/audit_summary/local_successor_state_model_pinned",
        "/audit_summary/transition_vector_corpus_pinned",
        "/audit_summary/target_chain_finality_profile_instantiated",
    ] {
        if !bool_at(corpus, pointer)? {
            return Err(format!(
                "lifecycle transition vector corpus lost true flag {pointer}"
            ));
        }
    }
    for pointer in [
        "/audit_summary/lifecycle_transition_profile_verified",
        "/audit_summary/deployment_ready",
    ] {
        if bool_at(corpus, pointer)? {
            return Err(format!(
                "lifecycle transition vector corpus unexpectedly closes {pointer}"
            ));
        }
    }

    let report_tests = report.executable_tests.iter().copied().collect::<BTreeSet<_>>();
    let cases = at(corpus, "/cases")?
        .as_array()
        .ok_or_else(|| "lifecycle transition corpus cases are not an array".to_string())?;
    if cases.len() != expected_case_ids.len() {
        return Err("lifecycle transition vector corpus cases array drifted".to_string());
    }
    let mut covered_phases = BTreeSet::new();
    for (index, (case, expected)) in cases.iter().zip(expected_case_ids.iter()).enumerate() {
        let case_id = str_at(case, "/case_id")?;
        if case_id != expected {
            return Err(format!(
                "lifecycle transition vector case {index} id {case_id:?} != {expected:?}"
            ));
        }
        if str_at(case, "/kind")? != "lifecycle_transition_kat" {
            return Err(format!(
                "lifecycle transition vector case {case_id} is not a transition KAT"
            ));
        }
        if !bool_at(case, "/evidence_present")? {
            return Err(format!(
                "lifecycle transition vector case {case_id} lost evidence"
            ));
        }
        if str_at(case, "/named_break_if_missing")? != report.named_break {
            return Err(format!(
                "lifecycle transition vector case {case_id} lost named break binding"
            ));
        }
        for support in string_vec_at(case, "/support_tests")? {
            if !report_tests.contains(support.as_str()) {
                return Err(format!(
                    "lifecycle transition vector case {case_id} references unpinned support test {support}"
                ));
            }
        }
        for phase in string_vec_at(case, "/transition_phases")? {
            covered_phases.insert(phase);
        }
    }
    for phase in ["admit", "spend", "replay", "burn", "recovery", "restore", "reorg"] {
        if !covered_phases.contains(phase) {
            return Err(format!(
                "lifecycle transition vector corpus does not cover phase {phase}"
            ));
        }
    }

    println!("Path 2 direct lifecycle transition vector consistency check passed.");
    Ok(())
}
