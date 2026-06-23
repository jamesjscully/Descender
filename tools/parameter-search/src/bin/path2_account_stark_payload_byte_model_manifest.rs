use descender_parameter_search::production_reference::{
    PROFILE, path2_account_stark_payload_byte_model_report_v1,
    path2_account_stark_payload_byte_model_report_v1_digest,
    path2_account_stark_payload_kat_boundary_report_v1,
    validate_path2_account_stark_payload_byte_model_report_v1,
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
    let byte_model_report = path2_account_stark_payload_byte_model_report_v1();
    validate_path2_account_stark_payload_byte_model_report_v1(&byte_model_report)?;
    let payload_kat_report = path2_account_stark_payload_kat_boundary_report_v1();
    let recomputed_byte_model_digest =
        path2_account_stark_payload_byte_model_report_v1_digest(&byte_model_report);
    if recomputed_byte_model_digest != byte_model_report.digest {
        return Err("payload byte model report digest mismatch".to_string());
    }
    if payload_kat_report.payload_byte_model_report_digest != byte_model_report.digest {
        return Err("payload KAT boundary report is not bound to byte model report".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountStarkPayloadByteModelManifest.v1",
        "corpus_id": "Descender.Path2.AccountStarkPayloadByteModel.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_byte_model_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_payload_byte_model_manifest",
            "source": "tools/parameter-search/src/bin/path2_account_stark_payload_byte_model_manifest.rs"
        },
        "authority_snapshot": {
            "payload_byte_model_report_domain": byte_model_report.domain,
            "payload_kat_boundary_report_domain": payload_kat_report.domain,
            "selected_payload_type": byte_model_report.selected_payload_type,
            "payload_schema": byte_model_report.payload_schema,
            "payload_version": byte_model_report.payload_version,
            "account_stark_verifier_version":
                byte_model_report.account_stark_verifier_version,
            "payload_frame_function": byte_model_report.payload_frame_function,
            "payload_digest_function": byte_model_report.payload_digest_function,
            "digest_algorithm": byte_model_report.digest_algorithm
        },
        "digests": {
            "payload_byte_model_report_digest": hex_encode(&byte_model_report.digest),
            "payload_byte_model_report_recomputed_digest":
                hex_encode(&recomputed_byte_model_digest),
            "payload_kat_boundary_report_digest": hex_encode(&payload_kat_report.digest),
            "payload_kat_boundary_report_byte_model_digest":
                hex_encode(&payload_kat_report.payload_byte_model_report_digest)
        },
        "byte_model": {
            "field_count": byte_model_report.field_count,
            "field_order": byte_model_report.field_order,
            "masked_projection_lane_count": byte_model_report.masked_projection_lane_count,
            "masked_projection_bytes": byte_model_report.masked_projection_bytes,
            "fixed_data_bytes_excluding_bundle":
                byte_model_report.fixed_data_bytes_excluding_bundle,
            "fixed_frame_overhead_excluding_bundle":
                byte_model_report.fixed_frame_overhead_excluding_bundle,
            "bundle_nonempty_required": byte_model_report.bundle_nonempty_required,
            "bundle_variable_length": byte_model_report.bundle_variable_length,
            "full_proof_bytes_inlined_in_manifest":
                byte_model_report.full_proof_bytes_inlined_in_manifest,
            "deterministic_kat_digest_pins_bundle_bytes":
                byte_model_report.deterministic_kat_digest_pins_bundle_bytes,
            "public_payload_exports_private_material":
                byte_model_report.public_payload_exports_private_material,
            "byte_model_pinned": byte_model_report.byte_model_pinned,
            "deployment_ready": byte_model_report.deployment_ready
        },
        "coverage": {
            "payload_kat_boundary_bound_to_byte_model": true,
            "deterministic_kat_digest_pins_bundle_bytes":
                byte_model_report.deterministic_kat_digest_pins_bundle_bytes,
            "full_proof_bytes_inlined_in_manifest":
                byte_model_report.full_proof_bytes_inlined_in_manifest,
            "public_payload_exports_private_material":
                byte_model_report.public_payload_exports_private_material,
            "deployment_ready": byte_model_report.deployment_ready
        }
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
