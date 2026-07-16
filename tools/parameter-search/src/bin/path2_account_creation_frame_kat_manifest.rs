use descender_parameter_search::production_reference::{
    PROFILE, path2_account_creation_frame_kat_boundary_report_v1,
    path2_account_creation_frame_kat_boundary_report_v1_digest,
    validate_path2_account_creation_frame_kat_boundary_report_v1,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn manifest_body() -> Result<Value, String> {
    let report = path2_account_creation_frame_kat_boundary_report_v1();
    validate_path2_account_creation_frame_kat_boundary_report_v1(&report)?;
    let recomputed_digest = path2_account_creation_frame_kat_boundary_report_v1_digest(&report);
    if recomputed_digest != report.digest {
        return Err("account-creation frame KAT report digest mismatch".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationFrameKatBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.AccountCreationFrameKatBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "frozen_boundary_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_creation_frame_kat_manifest",
            "source": "tools/parameter-search/src/bin/path2_account_creation_frame_kat_manifest.rs"
        },
        "authority_snapshot": {
            "frame_kat_boundary_report_domain": report.domain,
            "selected_leaf_type": report.selected_leaf_type,
            "selected_evidence_type": report.selected_evidence_type,
            "selected_certificate_type": report.selected_certificate_type,
            "leaf_frame_function": report.leaf_frame_function,
            "leaf_digest_function": report.leaf_digest_function,
            "evidence_transcript_digest_function":
                report.evidence_transcript_digest_function,
            "frame_binding_endpoint": report.frame_binding_endpoint,
            "payload_binding_endpoint": report.payload_binding_endpoint
        },
        "digests": {
            "frame_kat_boundary_report_digest": hex_encode(&report.digest),
            "frame_kat_boundary_report_recomputed_digest": hex_encode(&recomputed_digest)
        },
        "coverage": {
            "row_count": report.row_count,
            "canonical_frame_regression_present": report.canonical_frame_regression_present,
            "leaf_evidence_binding_field_count": report.leaf_evidence_binding_field_count,
            "frame_requirement_case_count": report.frame_requirement_case_count,
            "binding_requirement_case_count": report.binding_requirement_case_count,
            "current_boundary_mapping_complete": report.current_boundary_mapping_complete,
            "every_row_has_public_rejection_class": report.every_row_has_public_rejection_class,
            "unit_test_coverage_present": report.unit_test_coverage_present,
            "frozen_corpus_complete": report.frozen_corpus_complete,
            "deployment_ready": report.deployment_ready,
            "blocker": report.blocker
        },
        "rows": report.rows
    }))
}

fn main() -> Result<(), String> {
    let body = manifest_body()?;
    let body_json = serde_json::to_string_pretty(&body)
        .map_err(|err| format!("failed to serialize manifest body: {err}"))?;
    let manifest = json!({
        "manifest_digest": hex_encode(&Sha256::digest(body_json.as_bytes())),
        "manifest": body
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&manifest)
            .map_err(|err| format!("failed to serialize manifest: {err}"))?
    );
    Ok(())
}
