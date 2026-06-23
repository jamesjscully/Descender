use descender_parameter_search::production_reference::{
    PROFILE, path2_account_stark_fri_aok_boundary_report_v1,
    path2_account_stark_fri_aok_boundary_report_v1_digest,
    path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_residual_matrix_report_v1,
    validate_path2_account_stark_fri_aok_boundary_report_v1,
    validate_path2_account_stark_residual_matrix_report_v1,
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
    let report = path2_account_stark_fri_aok_boundary_report_v1();
    let fri_qrom_residual_report = path2_account_stark_fri_qrom_residual_report_v1();
    let residual_matrix_report = path2_account_stark_residual_matrix_report_v1();
    validate_path2_account_stark_fri_aok_boundary_report_v1(&report)?;
    validate_path2_account_stark_residual_matrix_report_v1(&residual_matrix_report)?;

    let recomputed_digest = path2_account_stark_fri_aok_boundary_report_v1_digest(&report);
    if recomputed_digest != report.digest {
        return Err("account-STARK FRI/AoK boundary report digest mismatch".to_string());
    }
    if report.fri_qrom_residual_report_digest != fri_qrom_residual_report.digest {
        return Err(
            "FRI/AoK boundary report is not bound to the current FRI/QROM residual report"
                .to_string(),
        );
    }
    if residual_matrix_report.fri_aok_boundary_report_digest != report.digest {
        return Err("residual matrix is not bound to the current FRI/AoK boundary".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountStarkFriAokBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.AccountStarkFriAokBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_fri_aok_boundary_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_stark_fri_aok_boundary_manifest",
            "source": "tools/parameter-search/src/bin/path2_account_stark_fri_aok_boundary_manifest.rs"
        },
        "authority_snapshot": {
            "fri_aok_boundary_report_domain": report.domain,
            "support_file": report.support_file,
            "selected_proof_family": report.selected_proof_family,
            "selected_payload_type": report.selected_payload_type,
            "account_stark_config_source": report.account_stark_config_source,
            "proof_system": report.proof_system,
            "residual_item": report.residual_item,
            "extractor_theorem_target": report.extractor_theorem_target,
            "named_break": report.named_break,
            "blocker": report.blocker
        },
        "digests": {
            "account_stark_fri_aok_boundary_report_digest": hex_encode(&report.digest),
            "account_stark_fri_aok_boundary_report_recomputed_digest": hex_encode(
                &recomputed_digest
            ),
            "account_stark_fri_qrom_residual_report_digest": hex_encode(
                &fri_qrom_residual_report.digest
            ),
            "fri_aok_boundary_report_fri_qrom_digest": hex_encode(
                &report.fri_qrom_residual_report_digest
            ),
            "account_stark_residual_matrix_report_digest": hex_encode(
                &residual_matrix_report.digest
            ),
            "residual_matrix_report_fri_aok_digest": hex_encode(
                &residual_matrix_report.fri_aok_boundary_report_digest
            )
        },
        "air_audit_evidence": {
            "support_file": report.support_file,
            "air_relation_class_count": report.air_relation_class_count,
            "med_findings_total": report.med_findings_total,
            "med_findings_remediated": report.med_findings_remediated,
            "context_binding_hardened_in_air": report.context_binding_hardened_in_air,
            "packed_witness_binding_negative_tested":
                report.packed_witness_binding_negative_tested,
            "c4_inverse_corner_regression_present":
                report.c4_inverse_corner_regression_present,
            "air_false_accept_hole_found": report.air_false_accept_hole_found
        },
        "fri_and_challenge_inputs": {
            "fri_schedule_pinned": report.fri_schedule_pinned,
            "challenge_field_pinned": report.challenge_field_pinned,
            "fri_qrom_residual_report_digest": hex_encode(
                &report.fri_qrom_residual_report_digest
            ),
            "account_stark_config_source": report.account_stark_config_source,
            "proof_system": report.proof_system
        },
        "open_obligations": {
            "fri_aok_extractor_theorem_instantiated":
                report.fri_aok_extractor_theorem_instantiated,
            "accepting_proof_implies_extractable_witness":
                report.accepting_proof_implies_extractable_witness,
            "external_extractor_review_complete":
                report.external_extractor_review_complete,
            "deployment_ready": report.deployment_ready,
            "blocker": report.blocker
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
