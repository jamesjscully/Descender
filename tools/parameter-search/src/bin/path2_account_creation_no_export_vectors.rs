use descender_parameter_search::production_reference::{
    PROFILE, path2_account_creation_no_export_boundary_report_v1,
    path2_account_stark_payload_byte_model_report_v1,
    path2_account_stark_payload_kat_boundary_report_v1,
    validate_path2_account_creation_no_export_boundary_report_v1,
    validate_path2_account_stark_payload_byte_model_report_v1,
    validate_path2_account_stark_payload_kat_boundary_report_v1,
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

fn denylist_absent<'a>(
    public_fields: impl IntoIterator<Item = &'a str>,
    denylist: &[&str],
) -> bool {
    public_fields
        .into_iter()
        .all(|field| !denylist.iter().any(|private| field == *private))
}

fn manifest_body() -> Result<Value, String> {
    let report = path2_account_creation_no_export_boundary_report_v1();
    validate_path2_account_creation_no_export_boundary_report_v1(&report)?;
    let byte_model = path2_account_stark_payload_byte_model_report_v1();
    validate_path2_account_stark_payload_byte_model_report_v1(&byte_model)?;
    let payload_kat = path2_account_stark_payload_kat_boundary_report_v1();
    validate_path2_account_stark_payload_kat_boundary_report_v1(&payload_kat)?;

    let public_payload_fields: Vec<&str> = byte_model
        .field_order
        .iter()
        .filter(|field| field.public_payload_field)
        .map(|field| field.field_name)
        .collect();
    let simulator_public_input_fields: Vec<&str> = report.simulator_public_input_fields.to_vec();
    let private_material_fields: Vec<&str> = report.no_export_private_material_fields.to_vec();

    let payload_denylist_absent = denylist_absent(
        public_payload_fields.iter().copied(),
        &private_material_fields,
    );
    let simulator_denylist_absent = denylist_absent(
        simulator_public_input_fields.iter().copied(),
        &private_material_fields,
    );
    let all_private_material_absent = payload_denylist_absent && simulator_denylist_absent;
    if !all_private_material_absent {
        return Err(
            "no-export vector generator found denylisted private material in public fields"
                .to_string(),
        );
    }
    if byte_model.full_proof_bytes_inlined_in_manifest {
        return Err("no-export vector generator found inlined proof bytes".to_string());
    }
    if byte_model.public_payload_exports_private_material {
        return Err("no-export vector generator found private payload export".to_string());
    }

    let cases = vec![
        json!({
            "case_id": "no_export_public_payload_field_allowlist",
            "kind": "audit",
            "surface": "Path2AccountStarkPayloadByteModelReportV1.field_order",
            "requirement": report.no_export_requirement,
            "public_payload_fields": public_payload_fields.clone(),
            "private_material_denylist": private_material_fields.clone(),
            "all_private_material_absent": payload_denylist_absent
        }),
        json!({
            "case_id": "no_export_simulator_public_input_allowlist",
            "kind": "audit",
            "surface": "Path2AccountCreationNoExportBoundaryReportV1.simulator_public_input_fields",
            "requirement": report.no_export_requirement,
            "simulator_public_input_fields": simulator_public_input_fields.clone(),
            "private_material_denylist": private_material_fields.clone(),
            "all_private_material_absent": simulator_denylist_absent
        }),
        json!({
            "case_id": "no_export_private_material_denylist_absence",
            "kind": "audit",
            "surface": "combined public payload and simulator-input fields",
            "requirement": report.no_export_requirement,
            "public_payload_exports_private_material":
                byte_model.public_payload_exports_private_material,
            "private_material_denylist": private_material_fields.clone(),
            "all_private_material_absent": all_private_material_absent
        }),
        json!({
            "case_id": "no_export_opaque_bundle_not_manifest_inlined",
            "kind": "audit",
            "surface": "Path2AccountStarkCreationPayloadV1.account_stark_bundle",
            "requirement": report.no_export_requirement,
            "account_stark_bundle_field_is_public_payload": public_payload_fields
                .contains(&"account_stark_bundle"),
            "full_proof_bytes_inlined_in_manifest": byte_model.full_proof_bytes_inlined_in_manifest,
            "deterministic_kat_digest_pins_bundle_bytes":
                byte_model.deterministic_kat_digest_pins_bundle_bytes,
            "all_private_material_absent": true
        }),
        json!({
            "case_id": "no_export_public_bundle_lean_predicate",
            "kind": "audit",
            "surface": "Descender.Path2NoExportConcrete",
            "requirement": report.no_export_requirement,
            "public_bundle_no_export_lean_theorem":
                report.public_bundle_no_export_lean_theorem,
            "public_bundle_no_export_predicate_instantiated":
                report.public_bundle_no_export_predicate_instantiated,
            "all_private_material_absent": true
        }),
    ];

    if cases.len() != report.no_export_vector_case_ids.len() {
        return Err("no-export vector generator case count drifted".to_string());
    }
    for (case, expected) in cases.iter().zip(report.no_export_vector_case_ids.iter()) {
        if case["case_id"] != *expected {
            return Err(format!(
                "no-export vector generator case id drift: expected {expected}"
            ));
        }
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationNoExportVectors.v1",
        "corpus_id": "Descender.Path2.AccountCreationNoExportBoundaryVectors.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "status": "digest_pinned_no_export_audit_vectors",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_creation_no_export_vectors",
            "source": "tools/parameter-search/src/bin/path2_account_creation_no_export_vectors.rs"
        },
        "boundary_report": {
            "domain": report.domain,
            "digest": hex_encode(&report.digest),
            "selected_payload_type": report.selected_payload_type,
            "no_export_requirement": report.no_export_requirement,
            "simulator_theorem_target": report.simulator_theorem_target,
            "no_export_theorem_target": report.no_export_theorem_target,
            "simulator_theorem_instantiated": report.simulator_theorem_instantiated,
            "no_export_theorem_instantiated": report.no_export_theorem_instantiated,
            "public_bundle_no_export_lean_theorem":
                report.public_bundle_no_export_lean_theorem,
            "public_bundle_no_export_predicate_instantiated":
                report.public_bundle_no_export_predicate_instantiated,
            "frozen_no_export_vectors_complete": report.frozen_no_export_vectors_complete,
            "deployment_ready": report.deployment_ready,
            "blocker": report.blocker
        },
        "digests": {
            "payload_byte_model_report_digest": hex_encode(&byte_model.digest),
            "payload_kat_boundary_report_digest": hex_encode(&payload_kat.digest)
        },
        "audit_summary": {
            "payload_public_field_count": public_payload_fields.len(),
            "simulator_public_input_field_count": simulator_public_input_fields.len(),
            "private_material_denylist_count": private_material_fields.len(),
            "payload_denylist_absent": payload_denylist_absent,
            "simulator_denylist_absent": simulator_denylist_absent,
            "public_payload_exports_private_material":
                byte_model.public_payload_exports_private_material,
            "full_proof_bytes_inlined_in_manifest":
                byte_model.full_proof_bytes_inlined_in_manifest,
            "deterministic_kat_digest_pins_bundle_bytes":
                byte_model.deterministic_kat_digest_pins_bundle_bytes,
            "all_private_material_absent": all_private_material_absent,
            "simulator_theorem_instantiated": report.simulator_theorem_instantiated,
            "no_export_theorem_instantiated": report.no_export_theorem_instantiated,
            "public_bundle_no_export_predicate_instantiated":
                report.public_bundle_no_export_predicate_instantiated,
            "frozen_no_export_vectors_complete": report.frozen_no_export_vectors_complete,
            "deployment_ready": report.deployment_ready
        },
        "case_count": cases.len(),
        "cases": cases
    }))
}

fn main() -> Result<(), String> {
    let corpus = manifest_body()?;
    let corpus_json = serde_json::to_string_pretty(&corpus)
        .map_err(|err| format!("failed to serialize no-export corpus body: {err}"))?;
    let out = json!({
        "corpus_digest": hex_encode(&Sha256::digest(corpus_json.as_bytes())),
        "corpus": corpus
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&out)
            .map_err(|err| format!("failed to serialize no-export corpus: {err}"))?
    );
    Ok(())
}
