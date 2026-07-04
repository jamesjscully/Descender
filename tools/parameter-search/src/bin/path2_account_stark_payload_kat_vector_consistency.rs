use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("parameter-search crate lives under tools/")
        .to_path_buf()
}

fn load_json(relative_path: &str) -> Result<Value, String> {
    let path = repo_root().join(relative_path);
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

fn array_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a [Value], String> {
    at(value, pointer)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| format!("JSON pointer {pointer} is not an array"))
}

fn row_string<'a>(row: &'a Value, field: &str) -> Result<&'a str, String> {
    row.get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("boundary row is missing string field {field}"))
}

fn main() -> Result<(), String> {
    let boundary = load_json("formal/kat/path2_account_stark_payload_kat_boundary_v1.json")?;
    let vectors = load_json("formal/kat/path2_account_stark_payload_kat_vectors_v1.json")?;

    if str_at(&boundary, "/manifest/schema")?
        != "Descender.Path2.AccountStarkPayloadKatBoundaryManifest.v1"
    {
        return Err("payload KAT boundary manifest schema mismatch".to_string());
    }
    if str_at(&boundary, "/manifest/frozen_status")? != "frozen_boundary_manifest" {
        return Err("payload KAT boundary manifest frozen_status drifted".to_string());
    }
    if str_at(&vectors, "/corpus/schema")? != "Descender.Path2.AccountStarkPayloadKatVectors.v1" {
        return Err("payload KAT vector corpus schema mismatch".to_string());
    }
    if str_at(&vectors, "/corpus/status")? != "digest_pinned_boundary_vectors" {
        return Err("payload KAT vector corpus status drifted".to_string());
    }

    let boundary_digest = str_at(
        &boundary,
        "/manifest/digests/payload_kat_boundary_report_digest",
    )?;
    let vector_boundary_digest = str_at(&vectors, "/corpus/boundary_report/digest")?;
    if boundary_digest != vector_boundary_digest {
        return Err(format!(
            "payload KAT vector corpus is bound to {vector_boundary_digest}, but boundary manifest pins {boundary_digest}"
        ));
    }

    let boundary_row_count = u64_at(&boundary, "/manifest/coverage/row_count")?;
    let vector_row_count = u64_at(&vectors, "/corpus/boundary_report/row_count")?;
    if boundary_row_count != vector_row_count {
        return Err(format!(
            "payload KAT vector row_count mismatch: boundary={boundary_row_count}, vectors={vector_row_count}"
        ));
    }
    if !bool_at(
        &vectors,
        "/corpus/boundary_report/every_current_row_has_generated_vector",
    )? {
        return Err("payload KAT vectors do not cover every current boundary row".to_string());
    }
    if !bool_at(&boundary, "/manifest/coverage/frozen_corpus_complete")? {
        return Err("payload KAT boundary manifest is not frozen".to_string());
    }
    if str_at(
        &boundary,
        "/manifest/authority_snapshot/payload_kat_corpus_reduction_lean_theorem",
    )? != "selectedRustAcceptedBoundaryPayloadKatCorpusReduction_v1"
    {
        return Err("payload KAT boundary manifest lost Lean reduction theorem".to_string());
    }
    if !bool_at(
        &boundary,
        "/manifest/coverage/payload_kat_corpus_reduction_bound",
    )? {
        return Err("payload KAT boundary manifest lost Lean reduction bound".to_string());
    }
    if bool_at(&boundary, "/manifest/coverage/deployment_ready")? {
        return Err(
            "payload KAT boundary manifest unexpectedly claims deployment readiness".to_string(),
        );
    }

    let rows = array_at(&boundary, "/manifest/rows")?;
    if rows.len() as u64 != boundary_row_count {
        return Err(format!(
            "payload KAT boundary row array length {} does not match row_count {boundary_row_count}",
            rows.len()
        ));
    }

    let cases = array_at(&vectors, "/corpus/cases")?;
    let case_count = u64_at(&vectors, "/corpus/case_count")?;
    let rejection_case_count = u64_at(&vectors, "/corpus/rejection_case_count")?;
    if case_count != cases.len() as u64 {
        return Err(format!(
            "payload KAT vector case_count {case_count} does not match cases array length {}",
            cases.len()
        ));
    }
    if rejection_case_count != boundary_row_count {
        return Err(format!(
            "payload KAT vector rejection_case_count {rejection_case_count} does not match boundary row_count {boundary_row_count}"
        ));
    }
    if case_count != rejection_case_count + 1 {
        return Err(format!(
            "payload KAT vector case_count {case_count} must be exactly one honest case plus {rejection_case_count} rejection cases"
        ));
    }

    let honest_count = cases
        .iter()
        .filter(|case| {
            case.get("case_id").and_then(Value::as_str) == Some("honest_payload_accepts")
        })
        .count();
    if honest_count != 1 {
        return Err(format!(
            "payload KAT vectors must contain exactly one honest acceptance case, found {honest_count}"
        ));
    }

    let mut rejection_cases: BTreeMap<&str, &Value> = BTreeMap::new();
    for case in cases {
        let case_id = case
            .get("case_id")
            .and_then(Value::as_str)
            .ok_or_else(|| "payload KAT vector case missing case_id".to_string())?;
        let kind = case
            .get("kind")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("payload KAT vector case {case_id} missing kind"))?;
        if case_id == "honest_payload_accepts" {
            if kind != "accept" {
                return Err("honest payload KAT case is not marked accept".to_string());
            }
            continue;
        }
        if kind != "reject" {
            return Err(format!(
                "payload KAT vector case {case_id} is not marked reject"
            ));
        }
        if rejection_cases.insert(case_id, case).is_some() {
            return Err(format!(
                "duplicate payload KAT vector rejection case {case_id}"
            ));
        }
    }

    let mut expected_ids = BTreeSet::new();
    for row in rows {
        let case_id = row_string(row, "case_id")?;
        let public_class = row_string(row, "public_rejection_class")?;
        let internal_error = row_string(row, "internal_error_match")?;
        let mutated_surface = row_string(row, "mutated_surface")?;
        expected_ids.insert(case_id);

        let case = rejection_cases
            .get(case_id)
            .ok_or_else(|| format!("missing payload KAT vector rejection case {case_id}"))?;
        let observed_class = row_string(case, "public_rejection_class")?;
        if observed_class != public_class {
            return Err(format!(
                "payload KAT vector public class mismatch for {case_id}: {observed_class} != {public_class}"
            ));
        }
        let observed_error = row_string(case, "observed_error")?;
        if !observed_error.contains(internal_error) {
            return Err(format!(
                "payload KAT vector observed error for {case_id} does not contain boundary marker {internal_error:?}: {observed_error:?}"
            ));
        }
        let observed_surface = row_string(case, "mutated_surface")?;
        if observed_surface != mutated_surface {
            return Err(format!(
                "payload KAT vector mutated surface mismatch for {case_id}: {observed_surface} != {mutated_surface}"
            ));
        }
    }

    let observed_ids: BTreeSet<&str> = rejection_cases.keys().copied().collect();
    if observed_ids != expected_ids {
        return Err(format!(
            "payload KAT vector rejection ids differ from boundary rows: observed={observed_ids:?}, expected={expected_ids:?}"
        ));
    }

    println!("Path 2 account-STARK payload KAT vector consistency check passed.");
    Ok(())
}
