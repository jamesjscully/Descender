use descender_parameter_search::production_reference::{
    PROFILE, path2_account_creation_no_export_boundary_report_v1,
    path2_account_creation_no_export_boundary_report_v1_digest,
    path2_account_creation_proof_boundary_report_v1,
    validate_path2_account_creation_no_export_boundary_report_v1,
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
    let report = path2_account_creation_no_export_boundary_report_v1();
    validate_path2_account_creation_no_export_boundary_report_v1(&report)?;
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    let recomputed_digest = path2_account_creation_no_export_boundary_report_v1_digest(&report);
    if recomputed_digest != report.digest {
        return Err("account-creation no-export boundary report digest mismatch".to_string());
    }
    if proof_report.no_export_boundary_report_digest != report.digest {
        return Err("proof-boundary report is not bound to no-export report".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationNoExportBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.AccountCreationNoExportBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_no_export_boundary_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_creation_no_export_boundary_manifest",
            "source": "tools/parameter-search/src/bin/path2_account_creation_no_export_boundary_manifest.rs"
        },
        "authority_snapshot": {
            "no_export_report_domain": report.domain,
            "proof_boundary_report_domain": proof_report.domain,
            "support_file": report.support_file,
            "selected_payload_type": report.selected_payload_type,
            "verifier_boundary_endpoint": report.verifier_boundary_endpoint,
            "production_boundary_endpoint": report.production_boundary_endpoint,
            "no_export_requirement": report.no_export_requirement,
            "simulator_theorem_target": report.simulator_theorem_target,
            "no_export_theorem_target": report.no_export_theorem_target,
            "public_bundle_no_export_lean_theorem":
                report.public_bundle_no_export_lean_theorem,
            "simulator_input_allowlist_lean_theorem":
                report.simulator_input_allowlist_lean_theorem,
            "accepted_public_anchor_package_lean_theorem":
                report.accepted_public_anchor_package_lean_theorem,
            "accepted_bundle_public_anchor_simulation_boundary_lean_theorem":
                report.accepted_bundle_public_anchor_simulation_boundary_lean_theorem,
            "accepted_bundle_simulator_input_from_public_anchors_lean_theorem":
                report.accepted_bundle_simulator_input_from_public_anchors_lean_theorem,
            "named_break": report.named_break,
            "blocker": report.blocker
        },
        "digests": {
            "account_creation_no_export_boundary_report_digest": hex_encode(&report.digest),
            "account_creation_no_export_boundary_report_recomputed_digest":
                hex_encode(&recomputed_digest),
            "account_creation_proof_boundary_report_digest": hex_encode(&proof_report.digest),
            "proof_boundary_report_no_export_digest":
                hex_encode(&proof_report.no_export_boundary_report_digest),
            "payload_byte_model_report_digest":
                hex_encode(&report.payload_byte_model_report_digest),
            "payload_kat_boundary_report_digest":
                hex_encode(&report.payload_kat_boundary_report_digest)
        },
        "no_export_boundary": {
            "coverage": {
                "payload_public_field_count": report.payload_public_field_count,
                "public_payload_field_mutation_count":
                    report.public_payload_field_mutation_count,
                "no_export_vector_case_count": report.no_export_vector_case_count,
                "public_payload_exports_private_material":
                    report.public_payload_exports_private_material,
                "no_export_field_denylist_pinned": report.no_export_field_denylist_pinned,
                "simulator_input_allowlist_pinned":
                    report.simulator_input_allowlist_pinned,
                "byte_model_public_payload_only": report.byte_model_public_payload_only,
                "boundary_mapping_complete": report.boundary_mapping_complete,
                "roundtrip_regression_present": report.roundtrip_regression_present,
                "production_gate_fail_closed": report.production_gate_fail_closed,
                "no_export_vectors_present": report.no_export_vectors_present,
                "public_bundle_no_export_predicate_instantiated":
                    report.public_bundle_no_export_predicate_instantiated,
                "simulator_input_allowlist_predicate_instantiated":
                    report.simulator_input_allowlist_predicate_instantiated,
                "accepted_public_anchor_package_bound":
                    report.accepted_public_anchor_package_bound,
                "accepted_bundle_public_anchor_simulation_boundary_bound":
                    report.accepted_bundle_public_anchor_simulation_boundary_bound,
                "accepted_bundle_simulator_input_from_public_anchors_bound":
                    report.accepted_bundle_simulator_input_from_public_anchors_bound,
                "simulator_theorem_instantiated": report.simulator_theorem_instantiated,
                "no_export_theorem_instantiated": report.no_export_theorem_instantiated,
                "accepted_bundle_simulatable_from_public_anchors":
                    report.accepted_bundle_simulatable_from_public_anchors,
                "frozen_no_export_vectors_complete": report.frozen_no_export_vectors_complete,
                "external_review_complete": report.external_review_complete,
                "deployment_ready": report.deployment_ready,
                "blocker": report.blocker
            },
            "simulator_theorem_target": report.simulator_theorem_target,
            "no_export_theorem_target": report.no_export_theorem_target,
            "public_bundle_no_export_lean_theorem":
                report.public_bundle_no_export_lean_theorem,
            "simulator_input_allowlist_lean_theorem":
                report.simulator_input_allowlist_lean_theorem,
            "accepted_public_anchor_package_lean_theorem":
                report.accepted_public_anchor_package_lean_theorem,
            "accepted_bundle_public_anchor_simulation_boundary_lean_theorem":
                report.accepted_bundle_public_anchor_simulation_boundary_lean_theorem,
            "accepted_bundle_simulator_input_from_public_anchors_lean_theorem":
                report.accepted_bundle_simulator_input_from_public_anchors_lean_theorem,
            "no_export_vector_case_ids": report.no_export_vector_case_ids,
            "simulator_public_input_fields": report.simulator_public_input_fields,
            "no_export_private_material_fields": report.no_export_private_material_fields
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
