use descender_parameter_search::production_reference::{
    path2_account_creation_no_export_boundary_report_v1,
    path2_account_stark_payload_byte_model_report_v1,
    path2_account_stark_payload_kat_boundary_report_v1,
    validate_path2_account_creation_no_export_boundary_report_v1,
    validate_path2_account_stark_payload_byte_model_report_v1,
    validate_path2_account_stark_payload_kat_boundary_report_v1,
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

fn denylist_absent(public_fields: &[String], denylist: &[String]) -> bool {
    public_fields
        .iter()
        .all(|field| !denylist.iter().any(|private| field == private))
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let corpus_path = "formal/kat/path2_account_creation_no_export_vectors_v1.json";
    let corpus_file = load_json(&root, corpus_path)?;
    let corpus = at(&corpus_file, "/corpus")?;
    let corpus_json = serde_json::to_string_pretty(corpus)
        .map_err(|err| format!("serialize no-export corpus body: {err}"))?;
    let recomputed_digest = hex_encode(&Sha256::digest(corpus_json.as_bytes()));
    if str_at(&corpus_file, "/corpus_digest")? != recomputed_digest {
        return Err("no-export vector corpus digest mismatch".to_string());
    }

    let report = path2_account_creation_no_export_boundary_report_v1();
    validate_path2_account_creation_no_export_boundary_report_v1(&report)?;
    let byte_model = path2_account_stark_payload_byte_model_report_v1();
    validate_path2_account_stark_payload_byte_model_report_v1(&byte_model)?;
    let payload_kat = path2_account_stark_payload_kat_boundary_report_v1();
    validate_path2_account_stark_payload_kat_boundary_report_v1(&payload_kat)?;

    if str_at(corpus, "/schema")? != "Descender.Path2.AccountCreationNoExportVectors.v1" {
        return Err("no-export vector corpus schema mismatch".to_string());
    }
    if str_at(corpus, "/status")? != "digest_pinned_no_export_audit_vectors" {
        return Err("no-export vector corpus status mismatch".to_string());
    }
    if str_at(corpus, "/boundary_report/digest")? != hex_encode(&report.digest) {
        return Err("no-export vector corpus is not bound to boundary report".to_string());
    }
    if str_at(corpus, "/digests/payload_byte_model_report_digest")?
        != hex_encode(&byte_model.digest)
    {
        return Err("no-export vector corpus byte-model digest mismatch".to_string());
    }
    if str_at(corpus, "/digests/payload_kat_boundary_report_digest")?
        != hex_encode(&payload_kat.digest)
    {
        return Err("no-export vector corpus payload-KAT digest mismatch".to_string());
    }
    if bool_at(corpus, "/boundary_report/simulator_theorem_instantiated")?
        || bool_at(corpus, "/boundary_report/no_export_theorem_instantiated")?
        || bool_at(corpus, "/boundary_report/deployment_ready")?
        || bool_at(corpus, "/audit_summary/simulator_theorem_instantiated")?
        || bool_at(corpus, "/audit_summary/no_export_theorem_instantiated")?
        || bool_at(corpus, "/audit_summary/deployment_ready")?
    {
        return Err("no-export vector corpus unexpectedly closes theorem status".to_string());
    }
    if !bool_at(corpus, "/boundary_report/frozen_no_export_vectors_complete")?
        || !bool_at(corpus, "/audit_summary/frozen_no_export_vectors_complete")?
    {
        return Err("no-export vector corpus lost frozen-vector closure".to_string());
    }
    if !bool_at(
        corpus,
        "/boundary_report/public_bundle_no_export_predicate_instantiated",
    )? || !bool_at(
        corpus,
        "/audit_summary/public_bundle_no_export_predicate_instantiated",
    )? {
        return Err("no-export vector corpus lost public-bundle Lean hook".to_string());
    }
    if !bool_at(
        corpus,
        "/boundary_report/simulator_input_allowlist_predicate_instantiated",
    )? || !bool_at(
        corpus,
        "/audit_summary/simulator_input_allowlist_predicate_instantiated",
    )? {
        return Err("no-export vector corpus lost simulator-input Lean hook".to_string());
    }
    if !bool_at(
        corpus,
        "/boundary_report/accepted_public_anchor_package_bound",
    )? || !bool_at(
        corpus,
        "/audit_summary/accepted_public_anchor_package_bound",
    )? {
        return Err("no-export vector corpus lost accepted public-anchor package hook".to_string());
    }
    if !bool_at(
        corpus,
        "/boundary_report/accepted_bundle_public_anchor_simulation_boundary_bound",
    )? || !bool_at(
        corpus,
        "/audit_summary/accepted_bundle_public_anchor_simulation_boundary_bound",
    )? {
        return Err(
            "no-export vector corpus lost accepted-bundle public-anchor simulation-boundary hook"
                .to_string(),
        );
    }
    if !bool_at(
        corpus,
        "/boundary_report/accepted_bundle_public_bundle_from_public_anchors_bound",
    )? || !bool_at(
        corpus,
        "/audit_summary/accepted_bundle_public_bundle_from_public_anchors_bound",
    )? {
        return Err(
            "no-export vector corpus lost accepted-bundle public-bundle projection hook"
                .to_string(),
        );
    }
    if !bool_at(
        corpus,
        "/boundary_report/accepted_bundle_simulator_input_from_public_anchors_bound",
    )? || !bool_at(
        corpus,
        "/audit_summary/accepted_bundle_simulator_input_from_public_anchors_bound",
    )? {
        return Err(
            "no-export vector corpus lost accepted-bundle simulator-input projection hook"
                .to_string(),
        );
    }
    if !bool_at(
        corpus,
        "/boundary_report/accepted_bundle_simulator_no_export_reduction_bound",
    )? || !bool_at(
        corpus,
        "/audit_summary/accepted_bundle_simulator_no_export_reduction_bound",
    )? {
        return Err(
            "no-export vector corpus lost accepted-bundle simulator/no-export reduction hook"
                .to_string(),
        );
    }
    if bool_at(
        corpus,
        "/audit_summary/accepted_bundle_simulatable_from_public_anchors",
    )? {
        return Err(
            "no-export vector corpus unexpectedly closes accepted-bundle simulability".to_string(),
        );
    }
    if !bool_at(corpus, "/audit_summary/all_private_material_absent")?
        || !bool_at(corpus, "/audit_summary/payload_denylist_absent")?
        || !bool_at(corpus, "/audit_summary/simulator_denylist_absent")?
        || bool_at(
            corpus,
            "/audit_summary/public_payload_exports_private_material",
        )?
        || bool_at(
            corpus,
            "/audit_summary/full_proof_bytes_inlined_in_manifest",
        )?
        || !bool_at(
            corpus,
            "/audit_summary/deterministic_kat_digest_pins_bundle_bytes",
        )?
    {
        return Err("no-export vector corpus lost audit invariants".to_string());
    }

    if u64_at(corpus, "/case_count")? != report.no_export_vector_case_ids.len() as u64 {
        return Err("no-export vector corpus case count drifted".to_string());
    }
    let cases = at(corpus, "/cases")?
        .as_array()
        .ok_or_else(|| "no-export corpus cases are not an array".to_string())?;
    if cases.len() != report.no_export_vector_case_ids.len() {
        return Err("no-export vector corpus cases array drifted".to_string());
    }
    for (index, (case, expected)) in cases
        .iter()
        .zip(report.no_export_vector_case_ids.iter())
        .enumerate()
    {
        let case_id = case
            .get("case_id")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("no-export case {index} has no case_id"))?;
        if case_id != *expected {
            return Err(format!(
                "no-export vector case {index} id {case_id:?} != {expected:?}"
            ));
        }
        if case.get("kind").and_then(Value::as_str) != Some("audit") {
            return Err(format!(
                "no-export vector case {case_id} is not an audit case"
            ));
        }
        if case
            .get("all_private_material_absent")
            .and_then(Value::as_bool)
            != Some(true)
        {
            return Err(format!(
                "no-export vector case {case_id} lost private-material absence"
            ));
        }
    }

    let payload_fields = string_vec_at(&cases[0], "/public_payload_fields")?;
    let payload_denylist = string_vec_at(&cases[0], "/private_material_denylist")?;
    let simulator_fields = string_vec_at(&cases[1], "/simulator_public_input_fields")?;
    let simulator_denylist = string_vec_at(&cases[1], "/private_material_denylist")?;
    if !denylist_absent(&payload_fields, &payload_denylist)
        || !denylist_absent(&simulator_fields, &simulator_denylist)
    {
        return Err("no-export vector corpus exposes a denylisted field".to_string());
    }
    let public_bundle_case = cases
        .iter()
        .find(|case| {
            case.get("case_id").and_then(Value::as_str)
                == Some("no_export_public_bundle_lean_predicate")
        })
        .ok_or_else(|| "no-export vector corpus lost public-bundle Lean case".to_string())?;
    if public_bundle_case
        .get("public_bundle_no_export_lean_theorem")
        .and_then(Value::as_str)
        != Some(report.public_bundle_no_export_lean_theorem)
        || public_bundle_case
            .get("public_bundle_no_export_predicate_instantiated")
            .and_then(Value::as_bool)
            != Some(true)
    {
        return Err("no-export public-bundle Lean case drifted".to_string());
    }
    let simulator_input_case = cases
        .iter()
        .find(|case| {
            case.get("case_id").and_then(Value::as_str)
                == Some("no_export_simulator_input_lean_predicate")
        })
        .ok_or_else(|| "no-export vector corpus lost simulator-input Lean case".to_string())?;
    if simulator_input_case
        .get("simulator_input_allowlist_lean_theorem")
        .and_then(Value::as_str)
        != Some(report.simulator_input_allowlist_lean_theorem)
        || simulator_input_case
            .get("simulator_input_allowlist_predicate_instantiated")
            .and_then(Value::as_bool)
            != Some(true)
    {
        return Err("no-export simulator-input Lean case drifted".to_string());
    }
    let accepted_anchor_case = cases
        .iter()
        .find(|case| {
            case.get("case_id").and_then(Value::as_str)
                == Some("no_export_accepted_public_anchor_package_lean_predicate")
        })
        .ok_or_else(|| {
            "no-export vector corpus lost accepted public-anchor package Lean case".to_string()
        })?;
    if accepted_anchor_case
        .get("accepted_public_anchor_package_lean_theorem")
        .and_then(Value::as_str)
        != Some(report.accepted_public_anchor_package_lean_theorem)
        || accepted_anchor_case
            .get("accepted_public_anchor_package_bound")
            .and_then(Value::as_bool)
            != Some(true)
        || accepted_anchor_case
            .get("accepted_bundle_simulatable_from_public_anchors")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return Err("no-export accepted public-anchor package Lean case drifted".to_string());
    }
    let accepted_simulation_boundary_case = cases
        .iter()
        .find(|case| {
            case.get("case_id").and_then(Value::as_str)
                == Some("no_export_accepted_bundle_public_anchor_simulation_boundary")
        })
        .ok_or_else(|| {
            "no-export vector corpus lost accepted-bundle simulation-boundary Lean case".to_string()
        })?;
    if accepted_simulation_boundary_case
        .get("accepted_bundle_public_anchor_simulation_boundary_lean_theorem")
        .and_then(Value::as_str)
        != Some(report.accepted_bundle_public_anchor_simulation_boundary_lean_theorem)
        || accepted_simulation_boundary_case
            .get("accepted_bundle_public_anchor_simulation_boundary_bound")
            .and_then(Value::as_bool)
            != Some(true)
        || accepted_simulation_boundary_case
            .get("accepted_bundle_simulatable_from_public_anchors")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return Err("no-export accepted-bundle simulation-boundary Lean case drifted".to_string());
    }
    let accepted_public_bundle_case = cases
        .iter()
        .find(|case| {
            case.get("case_id").and_then(Value::as_str)
                == Some("no_export_accepted_bundle_public_bundle_from_public_anchors")
        })
        .ok_or_else(|| {
            "no-export vector corpus lost accepted-bundle public-bundle projection Lean case"
                .to_string()
        })?;
    if accepted_public_bundle_case
        .get("accepted_bundle_public_bundle_from_public_anchors_lean_theorem")
        .and_then(Value::as_str)
        != Some(report.accepted_bundle_public_bundle_from_public_anchors_lean_theorem)
        || accepted_public_bundle_case
            .get("accepted_bundle_public_bundle_from_public_anchors_bound")
            .and_then(Value::as_bool)
            != Some(true)
        || accepted_public_bundle_case
            .get("accepted_bundle_simulatable_from_public_anchors")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return Err(
            "no-export accepted-bundle public-bundle projection Lean case drifted".to_string(),
        );
    }
    let accepted_simulator_input_case = cases
        .iter()
        .find(|case| {
            case.get("case_id").and_then(Value::as_str)
                == Some("no_export_accepted_bundle_simulator_input_from_public_anchors")
        })
        .ok_or_else(|| {
            "no-export vector corpus lost accepted-bundle simulator-input projection Lean case"
                .to_string()
        })?;
    if accepted_simulator_input_case
        .get("accepted_bundle_simulator_input_from_public_anchors_lean_theorem")
        .and_then(Value::as_str)
        != Some(report.accepted_bundle_simulator_input_from_public_anchors_lean_theorem)
        || accepted_simulator_input_case
            .get("accepted_bundle_simulator_input_from_public_anchors_bound")
            .and_then(Value::as_bool)
            != Some(true)
        || accepted_simulator_input_case
            .get("accepted_bundle_simulatable_from_public_anchors")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return Err(
            "no-export accepted-bundle simulator-input projection Lean case drifted".to_string(),
        );
    }
    let accepted_reduction_case = cases
        .iter()
        .find(|case| {
            case.get("case_id").and_then(Value::as_str)
                == Some("no_export_accepted_bundle_simulator_no_export_reduction")
        })
        .ok_or_else(|| {
            "no-export vector corpus lost accepted-bundle simulator/no-export reduction Lean case"
                .to_string()
        })?;
    if accepted_reduction_case
        .get("accepted_bundle_simulator_no_export_reduction_lean_theorem")
        .and_then(Value::as_str)
        != Some(report.accepted_bundle_simulator_no_export_reduction_lean_theorem)
        || accepted_reduction_case
            .get("accepted_bundle_simulator_no_export_reduction_bound")
            .and_then(Value::as_bool)
            != Some(true)
        || accepted_reduction_case
            .get("accepted_bundle_simulatable_from_public_anchors")
            .and_then(Value::as_bool)
            != Some(false)
        || accepted_reduction_case
            .get("simulator_theorem_instantiated")
            .and_then(Value::as_bool)
            != Some(false)
        || accepted_reduction_case
            .get("no_export_theorem_instantiated")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return Err(
            "no-export accepted-bundle simulator/no-export reduction Lean case drifted"
                .to_string(),
        );
    }

    println!("Path 2 account-creation no-export vector consistency check passed.");
    Ok(())
}
