use descender_parameter_search::production_reference::{
    path2_account_creation_no_export_boundary_report_v1,
    path2_account_creation_no_export_boundary_report_v1_digest,
    path2_account_creation_proof_boundary_report_v1,
    validate_path2_account_creation_no_export_boundary_report_v1,
};
use serde_json::Value;
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

fn require_contains(haystack: &str, needle: &str, label: &str) -> Result<(), String> {
    if haystack.contains(needle) {
        Ok(())
    } else {
        Err(format!("{label} is missing {needle:?}"))
    }
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let manifest = load_json(
        &root,
        "formal/kat/path2_account_creation_no_export_boundary_v1.json",
    )?;
    let source = load_text(&root, "tools/parameter-search/src/production_reference.rs")?;

    let report = path2_account_creation_no_export_boundary_report_v1();
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_no_export_boundary_report_v1(&report)?;
    let report_digest = hex_encode(&report.digest);
    let recomputed_digest = hex_encode(
        &path2_account_creation_no_export_boundary_report_v1_digest(&report),
    );

    if str_at(&manifest, "/manifest/schema")?
        != "Descender.Path2.AccountCreationNoExportBoundaryManifest.v1"
    {
        return Err("account-creation no-export manifest schema mismatch".to_string());
    }
    if str_at(&manifest, "/manifest/frozen_status")? != "draft_no_export_boundary_manifest" {
        return Err("account-creation no-export manifest frozen_status drifted".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/account_creation_no_export_boundary_report_digest",
    )? != report_digest
    {
        return Err("no-export manifest digest is not bound to report".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/account_creation_no_export_boundary_report_recomputed_digest",
    )? != recomputed_digest
    {
        return Err("no-export manifest recomputed digest mismatch".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/proof_boundary_report_no_export_digest",
    )? != hex_encode(&proof_report.no_export_boundary_report_digest)
    {
        return Err("proof-boundary no-export digest is not reflected in manifest".to_string());
    }
    if proof_report.no_export_boundary_report_digest != report.digest {
        return Err("proof-boundary report is not bound to no-export report".to_string());
    }

    for (pointer, expected) in [
        (
            "/manifest/authority_snapshot/support_file",
            report.support_file,
        ),
        (
            "/manifest/authority_snapshot/selected_payload_type",
            report.selected_payload_type,
        ),
        (
            "/manifest/authority_snapshot/verifier_boundary_endpoint",
            report.verifier_boundary_endpoint,
        ),
        (
            "/manifest/authority_snapshot/production_boundary_endpoint",
            report.production_boundary_endpoint,
        ),
        (
            "/manifest/authority_snapshot/no_export_requirement",
            report.no_export_requirement,
        ),
        (
            "/manifest/authority_snapshot/simulator_theorem_target",
            report.simulator_theorem_target,
        ),
        (
            "/manifest/authority_snapshot/no_export_theorem_target",
            report.no_export_theorem_target,
        ),
        (
            "/manifest/authority_snapshot/named_break",
            report.named_break,
        ),
    ] {
        if str_at(&manifest, pointer)? != expected {
            return Err(format!(
                "account-creation no-export manifest drifted at {pointer}"
            ));
        }
    }
    for (pointer, expected) in [
        (
            "/manifest/no_export_boundary/coverage/payload_public_field_count",
            report.payload_public_field_count,
        ),
        (
            "/manifest/no_export_boundary/coverage/public_payload_field_mutation_count",
            report.public_payload_field_mutation_count,
        ),
    ] {
        if u64_at(&manifest, pointer)? != expected as u64 {
            return Err(format!(
                "account-creation no-export manifest drifted at {pointer}"
            ));
        }
    }
    for pointer in [
        "/manifest/no_export_boundary/coverage/no_export_field_denylist_pinned",
        "/manifest/no_export_boundary/coverage/simulator_input_allowlist_pinned",
        "/manifest/no_export_boundary/coverage/byte_model_public_payload_only",
        "/manifest/no_export_boundary/coverage/boundary_mapping_complete",
        "/manifest/no_export_boundary/coverage/roundtrip_regression_present",
        "/manifest/no_export_boundary/coverage/production_gate_fail_closed",
    ] {
        if !bool_at(&manifest, pointer)? {
            return Err(format!(
                "account-creation no-export manifest lost required flag {pointer}"
            ));
        }
    }
    for pointer in [
        "/manifest/no_export_boundary/coverage/public_payload_exports_private_material",
        "/manifest/no_export_boundary/coverage/simulator_theorem_instantiated",
        "/manifest/no_export_boundary/coverage/no_export_theorem_instantiated",
        "/manifest/no_export_boundary/coverage/accepted_bundle_simulatable_from_public_anchors",
        "/manifest/no_export_boundary/coverage/frozen_no_export_vectors_complete",
        "/manifest/no_export_boundary/coverage/external_review_complete",
        "/manifest/no_export_boundary/coverage/deployment_ready",
    ] {
        if bool_at(&manifest, pointer)? {
            return Err(format!(
                "account-creation no-export manifest unexpectedly closes {pointer}"
            ));
        }
    }

    let support_doc = load_text(&root, report.support_file)?;
    for needle in [
        "Path2AccountCreationNoExportBoundaryReportV1",
        "zero_knowledge_no_export_semantics",
        "selected_account_stark_public_bundle_simulator_v1",
        "selected_account_stark_no_export_v1",
        "account_stark_simulator_or_no_export_break",
        "Forbidden Exports",
        "Error Path Rule",
        "raw pad seed",
        "private_trace_material",
        "private trace material",
    ] {
        require_contains(&support_doc, needle, report.support_file)?;
    }
    for field in report.no_export_private_material_fields {
        require_contains(&support_doc, field, report.support_file)?;
    }
    for field in report.simulator_public_input_fields {
        require_contains(&support_doc, field, report.support_file)?;
    }

    for needle in [
        "Path2AccountCreationNoExportBoundaryReportV1",
        "path2_account_creation_no_export_boundary_report_v1",
        "PATH2_ACCOUNT_CREATION_SIMULATOR_THEOREM_TARGET",
        "PATH2_ACCOUNT_CREATION_NO_EXPORT_THEOREM_TARGET",
        "PATH2_ACCOUNT_CREATION_SIMULATOR_PUBLIC_INPUT_FIELDS",
        "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_NO_EXPORT_PRIVATE_MATERIAL_FIELDS",
        "verify_path2_direct_creation_account_stark_payload_boundary",
        "path2_account_stark_payload_byte_model_report_v1",
        "path2_account_stark_payload_kat_boundary_report_v1",
    ] {
        require_contains(&source, needle, "production reference source")?;
    }

    println!("Path 2 account-creation no-export boundary consistency check passed.");
    Ok(())
}
