use descender_parameter_search::production_reference::{
    path2_account_creation_sampler_timing_report_v1,
    path2_authorization_class_binding_audit_report_v1,
    path2_authorization_class_binding_audit_report_v1_digest,
    validate_path2_account_creation_sampler_timing_report_v1,
    validate_path2_authorization_class_binding_audit_report_v1, PROFILE,
};
use serde_json::{json, Value};
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
    let audit_report = path2_authorization_class_binding_audit_report_v1();
    validate_path2_authorization_class_binding_audit_report_v1(&audit_report)?;
    let sampler_timing_report = path2_account_creation_sampler_timing_report_v1();
    validate_path2_account_creation_sampler_timing_report_v1(&sampler_timing_report)?;

    let recomputed_audit_digest =
        path2_authorization_class_binding_audit_report_v1_digest(&audit_report);
    if recomputed_audit_digest != audit_report.digest {
        return Err("authorization-class binding audit digest mismatch".to_string());
    }
    if sampler_timing_report.authorization_class_binding_audit_report_digest != audit_report.digest
    {
        return Err("sampler/timing report is not bound to authorization-class audit".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.AuthorizationClassBindingAuditManifest.v1",
        "corpus_id": "Descender.Path2.AuthorizationClassBindingAudit.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_authorization_class_binding_audit_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_authorization_class_binding_audit_manifest",
            "source": "tools/parameter-search/src/bin/path2_authorization_class_binding_audit_manifest.rs"
        },
        "authority_snapshot": {
            "audit_report_domain": audit_report.domain,
            "sampler_timing_report_domain": sampler_timing_report.domain,
            "support_file": audit_report.support_file,
            "projection_audit_file": audit_report.projection_audit_file,
            "authorization_local_classification_file":
                audit_report.authorization_local_classification_file,
            "theorem_target": audit_report.theorem_target,
            "spend_authorization_class_digest_function":
                audit_report.spend_authorization_class_digest_function,
            "spend_projection_encoder": audit_report.spend_projection_encoder,
            "direct_spend_verifier_endpoint": audit_report.direct_spend_verifier_endpoint,
            "checked_core_verifier_endpoint": audit_report.checked_core_verifier_endpoint,
            "state_namespace_validator": audit_report.state_namespace_validator,
            "consume_once_validator": audit_report.consume_once_validator,
            "lean_boundary_predicate": audit_report.lean_boundary_predicate,
            "proof_boundary_blocker": audit_report.blocker
        },
        "digests": {
            "authorization_class_binding_audit_report_digest": hex_encode(
                &audit_report.digest
            ),
            "authorization_class_binding_audit_report_recomputed_digest": hex_encode(
                &recomputed_audit_digest
            ),
            "sampler_timing_report_digest": hex_encode(&sampler_timing_report.digest),
            "sampler_timing_report_authorization_class_audit_digest": hex_encode(
                &sampler_timing_report.authorization_class_binding_audit_report_digest
            )
        },
        "field_completeness": {
            "p_spend_field_ids": audit_report.p_spend_field_ids,
            "p_spend_field_count": audit_report.p_spend_field_count,
            "rows": audit_report.rows,
            "row_count": audit_report.row_count,
            "p_spend_digest_covers_revised_fields":
                audit_report.p_spend_digest_covers_revised_fields,
            "field_completeness_negative_audit_pinned":
                audit_report.field_completeness_negative_audit_pinned,
            "field_completeness_negative_audit_complete":
                audit_report.field_completeness_negative_audit_complete
        },
        "source_shape_audit": {
            "source_shape_hook_ids": audit_report.source_shape_hook_ids,
            "source_shape_hook_count": audit_report.source_shape_hook_count,
            "checked_core_frame_binding_imported":
                audit_report.checked_core_frame_binding_imported,
            "slot_authorization_digest_imported":
                audit_report.slot_authorization_digest_imported,
            "consume_once_lifecycle_imported": audit_report.consume_once_lifecycle_imported,
            "no_private_authorization_material_in_direct_verifier":
                audit_report.no_private_authorization_material_in_direct_verifier,
            "lean_boundary_hooks_present": audit_report.lean_boundary_hooks_present,
            "existing_negative_tests_present": audit_report.existing_negative_tests_present
        },
        "open_obligations": {
            "theorem_instantiated": audit_report.theorem_instantiated,
            "external_review_complete": audit_report.external_review_complete,
            "deployment_ready": audit_report.deployment_ready
        }
    }))
}

fn main() -> Result<(), String> {
    let body = manifest_body()?;
    let canonical =
        serde_json::to_vec(&body).map_err(|err| format!("serialize manifest body: {err}"))?;
    let manifest_digest: [u8; 32] = Sha256::digest(&canonical).into();
    let output = json!({
        "manifest": body,
        "manifest_digest": hex_encode(&manifest_digest)
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&output)
            .map_err(|err| format!("serialize manifest: {err}"))?
    );
    Ok(())
}
