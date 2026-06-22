use descender_parameter_search::production_reference::{
    PROFILE, path2_account_stark_payload_kat_boundary_report_v1,
    path2_account_stark_payload_kat_boundary_report_v1_digest,
    path2_account_stark_residual_matrix_report_v1,
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
    let payload_kat_report = path2_account_stark_payload_kat_boundary_report_v1();
    let residual_matrix_report = path2_account_stark_residual_matrix_report_v1();
    let recomputed_payload_kat_digest =
        path2_account_stark_payload_kat_boundary_report_v1_digest(&payload_kat_report);
    if recomputed_payload_kat_digest != payload_kat_report.digest {
        return Err("payload KAT boundary report digest mismatch".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountStarkPayloadKatBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.AccountStarkPayloadKatBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_boundary_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_payload_kat_manifest",
            "source": "tools/parameter-search/src/bin/path2_account_stark_payload_kat_manifest.rs"
        },
        "authority_snapshot": {
            "payload_report_domain": payload_kat_report.domain,
            "residual_matrix_report_domain": residual_matrix_report.domain,
            "selected_payload_type": payload_kat_report.selected_payload_type,
            "payload_digest_function": payload_kat_report.payload_digest_function,
            "verifier_boundary_endpoint": payload_kat_report.verifier_boundary_endpoint,
            "production_boundary_endpoint": payload_kat_report.production_boundary_endpoint
        },
        "digests": {
            "payload_kat_boundary_report_digest": hex_encode(&payload_kat_report.digest),
            "payload_kat_boundary_report_recomputed_digest": hex_encode(
                &recomputed_payload_kat_digest
            ),
            "residual_matrix_report_digest": hex_encode(&residual_matrix_report.digest)
        },
        "coverage": {
            "row_count": payload_kat_report.rows.len(),
            "public_payload_field_mutation_count":
                payload_kat_report.public_payload_field_mutation_count,
            "current_boundary_mapping_complete":
                payload_kat_report.current_boundary_mapping_complete,
            "every_row_has_public_rejection_class":
                payload_kat_report.every_row_has_public_rejection_class,
            "roundtrip_regression_present": payload_kat_report.roundtrip_regression_present,
            "frozen_corpus_complete": payload_kat_report.frozen_corpus_complete,
            "deployment_ready": payload_kat_report.deployment_ready,
            "blocker": payload_kat_report.blocker
        },
        "rows": payload_kat_report.rows
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
