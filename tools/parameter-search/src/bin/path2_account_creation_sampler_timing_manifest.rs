use descender_parameter_search::production_reference::{
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_account_creation_sampler_timing_report_v1,
    path2_account_creation_sampler_timing_report_v1_digest,
    path2_authorization_class_binding_audit_report_v1,
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
    let sampler_timing_report = path2_account_creation_sampler_timing_report_v1();
    let proof_boundary_report = path2_account_creation_proof_boundary_report_v1();
    let authorization_class_binding_audit_report =
        path2_authorization_class_binding_audit_report_v1();
    validate_path2_account_creation_sampler_timing_report_v1(&sampler_timing_report)?;
    validate_path2_authorization_class_binding_audit_report_v1(
        &authorization_class_binding_audit_report,
    )?;

    let recomputed_sampler_timing_digest =
        path2_account_creation_sampler_timing_report_v1_digest(&sampler_timing_report);
    if recomputed_sampler_timing_digest != sampler_timing_report.digest {
        return Err("account-creation sampler/timing report digest mismatch".to_string());
    }
    let recomputed_proof_boundary_digest =
        path2_account_creation_proof_boundary_report_v1_digest(&proof_boundary_report);
    if recomputed_proof_boundary_digest != proof_boundary_report.digest {
        return Err("account-creation proof boundary report digest mismatch".to_string());
    }
    if proof_boundary_report.sampler_timing_report_digest != sampler_timing_report.digest {
        return Err(
            "proof boundary report is not bound to the current sampler/timing report".to_string(),
        );
    }
    if sampler_timing_report.authorization_class_binding_audit_report_digest
        != authorization_class_binding_audit_report.digest
    {
        return Err("sampler/timing report is not bound to authorization-class audit".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationSamplerTimingManifest.v1",
        "corpus_id": "Descender.Path2.AccountCreationSamplerTiming.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_sampler_timing_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_creation_sampler_timing_manifest",
            "source": "tools/parameter-search/src/bin/path2_account_creation_sampler_timing_manifest.rs"
        },
        "authority_snapshot": {
            "sampler_timing_report_domain": sampler_timing_report.domain,
            "proof_boundary_report_domain": proof_boundary_report.domain,
            "sampler_support_file": sampler_timing_report.sampler_support_file,
            "timing_support_file": sampler_timing_report.timing_support_file,
            "sampler_theorem_target": sampler_timing_report.sampler_theorem_target,
            "timing_break_class": sampler_timing_report.timing_break_class,
            "creation_statement_builder": sampler_timing_report.creation_statement_builder,
            "checked_core_challenge_frame": sampler_timing_report.checked_core_challenge_frame,
            "direct_state_namespace_validator":
                sampler_timing_report.direct_state_namespace_validator,
            "consume_once_validator": sampler_timing_report.consume_once_validator,
            "direct_spend_composition_endpoint":
                sampler_timing_report.direct_spend_composition_endpoint,
            "proof_boundary_blocker": sampler_timing_report.blocker
        },
        "digests": {
            "account_creation_sampler_timing_report_digest": hex_encode(
                &sampler_timing_report.digest
            ),
            "account_creation_sampler_timing_report_recomputed_digest": hex_encode(
                &recomputed_sampler_timing_digest
            ),
            "authorization_class_binding_audit_report_digest": hex_encode(
                &sampler_timing_report.authorization_class_binding_audit_report_digest
            ),
            "account_creation_proof_boundary_report_digest": hex_encode(
                &proof_boundary_report.digest
            ),
            "account_creation_proof_boundary_report_recomputed_digest": hex_encode(
                &recomputed_proof_boundary_digest
            ),
            "proof_boundary_report_sampler_timing_digest": hex_encode(
                &proof_boundary_report.sampler_timing_report_digest
            )
        },
        "sampler_boundary": {
            "support_file": sampler_timing_report.sampler_support_file,
            "theorem_target": sampler_timing_report.sampler_theorem_target,
            "construction_steps": sampler_timing_report.sampler_construction_steps,
            "authorization_class_binding_audit_report_digest": hex_encode(
                &sampler_timing_report.authorization_class_binding_audit_report_digest
            ),
            "authorization_class_binding_theorem_instantiated":
                sampler_timing_report.authorization_class_binding_theorem_instantiated,
            "honest_sampler_theorem_instantiated":
                sampler_timing_report.honest_sampler_theorem_instantiated,
            "sampler_boundary_pinned": sampler_timing_report.sampler_boundary_pinned
        },
        "timing_boundary": {
            "support_file": sampler_timing_report.timing_support_file,
            "timing_order_steps": sampler_timing_report.timing_order_steps,
            "prechallenge_fixed_through_step":
                sampler_timing_report.prechallenge_fixed_through_step,
            "spend_challenge_derived_at_step":
                sampler_timing_report.spend_challenge_derived_at_step,
            "timing_break_class": sampler_timing_report.timing_break_class,
            "no_postchallenge_timing_theorem_instantiated":
                sampler_timing_report.no_postchallenge_timing_theorem_instantiated,
            "timing_boundary_pinned": sampler_timing_report.timing_boundary_pinned
        },
        "executable_hooks": {
            "creation_statement_builder": sampler_timing_report.creation_statement_builder,
            "creation_statement_excludes_spend_message":
                sampler_timing_report.creation_statement_excludes_spend_message,
            "creation_statement_excludes_signature_bytes":
                sampler_timing_report.creation_statement_excludes_signature_bytes,
            "checked_core_challenge_frame": sampler_timing_report.checked_core_challenge_frame,
            "checked_core_challenge_binds_message":
                sampler_timing_report.checked_core_challenge_binds_message,
            "checked_core_challenge_binds_state_root":
                sampler_timing_report.checked_core_challenge_binds_state_root,
            "checked_core_challenge_binds_slot_authorization_digest":
                sampler_timing_report.checked_core_challenge_binds_slot_authorization_digest,
            "checked_core_challenge_binds_proof_commitment":
                sampler_timing_report.checked_core_challenge_binds_proof_commitment,
            "direct_state_namespace_validator":
                sampler_timing_report.direct_state_namespace_validator,
            "state_namespace_membership_required":
                sampler_timing_report.state_namespace_membership_required,
            "consume_once_validator": sampler_timing_report.consume_once_validator,
            "consume_once_required": sampler_timing_report.consume_once_required,
            "direct_spend_composition_endpoint":
                sampler_timing_report.direct_spend_composition_endpoint,
            "direct_spend_composition_hook_present":
                sampler_timing_report.direct_spend_composition_hook_present
        },
        "open_obligations": {
            "honest_sampler_theorem_instantiated":
                sampler_timing_report.honest_sampler_theorem_instantiated,
            "authorization_class_binding_theorem_instantiated":
                sampler_timing_report.authorization_class_binding_theorem_instantiated,
            "no_postchallenge_timing_theorem_instantiated":
                sampler_timing_report.no_postchallenge_timing_theorem_instantiated,
            "deployment_ready": sampler_timing_report.deployment_ready,
            "blocker": sampler_timing_report.blocker
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
