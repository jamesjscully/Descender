use descender_parameter_search::production_reference::{
    path2_account_creation_sampler_timing_report_v1,
    path2_authorization_class_binding_audit_report_v1,
    path2_authorization_class_binding_audit_report_v1_digest,
    validate_path2_account_creation_sampler_timing_report_v1,
    validate_path2_authorization_class_binding_audit_report_v1,
    PATH2_AUTHORIZATION_CLASS_BINDING_AUDIT_FIELDS, PATH2_AUTHORIZATION_CLASS_BINDING_AUDIT_HOOKS,
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
        .ok_or_else(|| format!("JSON pointer {pointer} is not a u64"))
}

fn require_contains(haystack: &str, needle: &str, label: &str) -> Result<(), String> {
    if haystack.contains(needle) {
        Ok(())
    } else {
        Err(format!("{label} is missing {needle:?}"))
    }
}

fn require_absent(haystack: &str, needle: &str, label: &str) -> Result<(), String> {
    if haystack.contains(needle) {
        Err(format!("{label} unexpectedly contains {needle:?}"))
    } else {
        Ok(())
    }
}

fn function_window<'a>(source: &'a str, name: &str) -> Result<&'a str, String> {
    let pub_marker = format!("\npub fn {name}");
    let fn_marker = format!("\nfn {name}");
    let start = source
        .find(&pub_marker)
        .or_else(|| source.find(&fn_marker))
        .ok_or_else(|| format!("missing function {name}"))?;
    let tail = &source[start + 1..];
    let next_pub = tail[1..]
        .find("\npub fn ")
        .map(|offset| offset + 1)
        .unwrap_or(tail.len());
    let next_private = tail[1..]
        .find("\nfn ")
        .map(|offset| offset + 1)
        .unwrap_or(tail.len());
    Ok(&tail[..next_pub.min(next_private)])
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let manifest = load_json(
        &root,
        "formal/kat/path2_authorization_class_binding_audit_v1.json",
    )?;
    let source = load_text(&root, "tools/parameter-search/src/production_reference.rs")?;
    let theorem_doc = load_text(
        &root,
        "research/path2_authorization_class_binding_theorem_v1.md",
    )?;
    let projection_audit = load_text(
        &root,
        "research/path2_spend_projection_completeness_audit_v1.md",
    )?;
    let local_classification = load_text(
        &root,
        "research/path2_authorization_local_classification.md",
    )?;
    let full_spend_lean = load_text(&root, "Descender/Path2FullSpendConcrete.lean")?;

    let report = path2_authorization_class_binding_audit_report_v1();
    validate_path2_authorization_class_binding_audit_report_v1(&report)?;
    let sampler_report = path2_account_creation_sampler_timing_report_v1();
    validate_path2_account_creation_sampler_timing_report_v1(&sampler_report)?;
    let report_digest = hex_encode(&report.digest);
    let recomputed_digest = hex_encode(&path2_authorization_class_binding_audit_report_v1_digest(
        &report,
    ));

    if str_at(&manifest, "/manifest/schema")?
        != "Descender.Path2.AuthorizationClassBindingAuditManifest.v1"
    {
        return Err("authorization-class audit manifest schema mismatch".to_string());
    }
    if str_at(&manifest, "/manifest/frozen_status")?
        != "draft_authorization_class_binding_audit_manifest"
    {
        return Err("authorization-class audit manifest frozen status drifted".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/authorization_class_binding_audit_report_digest",
    )? != report_digest
    {
        return Err("authorization-class audit manifest digest mismatch".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/authorization_class_binding_audit_report_recomputed_digest",
    )? != recomputed_digest
    {
        return Err("authorization-class audit recomputed digest mismatch".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/sampler_timing_report_authorization_class_audit_digest",
    )? != report_digest
        || sampler_report.authorization_class_binding_audit_report_digest != report.digest
    {
        return Err("sampler/timing report is not bound to authorization-class audit".to_string());
    }

    for (pointer, expected) in [
        (
            "/manifest/authority_snapshot/support_file",
            report.support_file,
        ),
        (
            "/manifest/authority_snapshot/projection_audit_file",
            report.projection_audit_file,
        ),
        (
            "/manifest/authority_snapshot/authorization_local_classification_file",
            report.authorization_local_classification_file,
        ),
        (
            "/manifest/authority_snapshot/theorem_target",
            report.theorem_target,
        ),
        (
            "/manifest/authority_snapshot/spend_authorization_class_digest_function",
            report.spend_authorization_class_digest_function,
        ),
        (
            "/manifest/authority_snapshot/spend_projection_encoder",
            report.spend_projection_encoder,
        ),
        (
            "/manifest/authority_snapshot/direct_spend_verifier_endpoint",
            report.direct_spend_verifier_endpoint,
        ),
        (
            "/manifest/authority_snapshot/checked_core_verifier_endpoint",
            report.checked_core_verifier_endpoint,
        ),
        (
            "/manifest/authority_snapshot/state_namespace_validator",
            report.state_namespace_validator,
        ),
        (
            "/manifest/authority_snapshot/consume_once_validator",
            report.consume_once_validator,
        ),
        (
            "/manifest/authority_snapshot/lean_boundary_predicate",
            report.lean_boundary_predicate,
        ),
    ] {
        if str_at(&manifest, pointer)? != expected {
            return Err(format!(
                "authorization-class audit manifest drifted at {pointer}"
            ));
        }
    }

    if u64_at(
        &manifest,
        "/manifest/field_completeness/p_spend_field_count",
    )? != PATH2_AUTHORIZATION_CLASS_BINDING_AUDIT_FIELDS.len() as u64
        || u64_at(&manifest, "/manifest/field_completeness/row_count")? != report.rows.len() as u64
        || u64_at(
            &manifest,
            "/manifest/source_shape_audit/source_shape_hook_count",
        )? != PATH2_AUTHORIZATION_CLASS_BINDING_AUDIT_HOOKS.len() as u64
    {
        return Err("authorization-class audit count drift".to_string());
    }
    for pointer in [
        "/manifest/field_completeness/p_spend_digest_covers_revised_fields",
        "/manifest/field_completeness/field_completeness_negative_audit_pinned",
        "/manifest/field_completeness/field_completeness_negative_audit_complete",
        "/manifest/source_shape_audit/checked_core_frame_binding_imported",
        "/manifest/source_shape_audit/slot_authorization_digest_imported",
        "/manifest/source_shape_audit/consume_once_lifecycle_imported",
        "/manifest/source_shape_audit/no_private_authorization_material_in_direct_verifier",
        "/manifest/source_shape_audit/lean_boundary_hooks_present",
        "/manifest/source_shape_audit/existing_negative_tests_present",
    ] {
        if !bool_at(&manifest, pointer)? {
            return Err(format!(
                "authorization-class audit lost required flag {pointer}"
            ));
        }
    }
    for pointer in [
        "/manifest/open_obligations/theorem_instantiated",
        "/manifest/open_obligations/external_review_complete",
        "/manifest/open_obligations/deployment_ready",
    ] {
        if bool_at(&manifest, pointer)? {
            return Err(format!(
                "authorization-class audit unexpectedly closes {pointer}"
            ));
        }
    }

    for field in PATH2_AUTHORIZATION_CLASS_BINDING_AUDIT_FIELDS {
        require_contains(
            &projection_audit,
            field,
            "spend projection completeness audit",
        )?;
    }
    for marker in [
        "Field-completeness",
        "reads a field *outside* this set",
        "AuthClass_c(auth) = p_spend(q)",
        "typed compatibility",
        "AuthorizationCompatibleOverClass",
    ] {
        require_contains(&theorem_doc, marker, "authorization-class theorem support")?;
    }
    for marker in [
        "AuthorizationLocal_c(q)",
        "AuthClass_c(auth)=p_spend(q)",
        "public_spend_class_law",
        "spend_closure_v1",
    ] {
        require_contains(
            &local_classification,
            marker,
            "authorization-local classification",
        )?;
    }

    let class_digest_window =
        function_window(&source, "path2_native_spend_authorization_class_digest_v1")?;
    for marker in [
        "spend_verifier_family_id",
        "checked_core_verifier_handle_digest",
        "public_key_identity",
        "allowed_authorization_local_mode",
        "allowed_state_transition_class_digest",
        "aggregation_mode",
        "spend_domain_separator_digest",
        "PATH2_NATIVE_AUTHORIZATION_CLASS_DOMAIN",
        "PATH2_NATIVE_AUTHORIZATION_LOCAL_MODE_DIRECT",
    ] {
        require_contains(
            marker_window(class_digest_window),
            marker,
            "authorization-class digest",
        )?;
    }

    let spend_projection_window =
        function_window(&source, "path2_native_spend_projection_v1_bytes")?;
    for marker in [
        "path2_native_spend_authorization_class_digest_v1",
        "spend_authorization_class_digest != expected_authorization_class_digest",
        "allowed_state_transition_class_digest",
        "aggregation_mode",
        "spend_domain_separator_digest",
    ] {
        require_contains(spend_projection_window, marker, "spend projection encoder")?;
    }

    let direct_verifier_window = function_window(
        &source,
        "verify_path2_direct_single_slot_spend_authorization",
    )?;
    for marker in [
        "leaf.public_key_identity",
        "Sha256::digest(public_key.as_bytes())",
        "validate_path2_direct_slot_authorization_state_namespace",
        "verify_path2_checked_core_authorization",
        "&leaf.account_id",
        "&leaf.account_state_root_before",
        "&authorization",
        "u64::from(parts.slot_id) != leaf.slot_id",
        "verify_path2_prevalidated_slot_consume_once",
        "consumed_authorization != authorization",
    ] {
        require_contains(direct_verifier_window, marker, "direct spend verifier")?;
    }
    for marker in [
        "private_fiber",
        "authorization_local_digest",
        "gauge_digest",
        "randomizer_digest",
        "wallet_recovery_digest",
    ] {
        require_absent(direct_verifier_window, marker, "direct spend verifier")?;
    }

    for marker in [
        "AccountCreationAuthorizationClassObligationTracked",
        "authorizationClass_obligationDigest_nonzero_of_obligationTracked",
        "authorizationClass_boundary_eq_leafBoundary_of_obligationTracked",
        "authorizationClass_matches_pSpend_of_obligationTracked",
        "selectedRustProjectionCompleteness",
        "selectedRustMeaningProjection_spendAuthorizationClass_eq_pSpend",
        "selectedRustAuthorizationCompatibleOverClass_of_compatible",
        "selectedRustAuthorizationClassBinding",
        "selectedRustHonestCreate_authorization_over_class",
        "selectedRust_accountCreationAuthorizationCompatibleOverClass_of_obligationTracked",
        "selectedRust_accountCreationAuthorizationClass_matchesMeaning_of_obligationTracked",
        "spend.leaf.authorizationClass =",
        "C.nativeProjections.pSpend spend.creationEvidence.boundary",
    ] {
        require_contains(&full_spend_lean, marker, "Lean authorization-class hooks")?;
    }
    for hook in PATH2_AUTHORIZATION_CLASS_BINDING_AUDIT_HOOKS {
        let haystack = if hook.starts_with("AccountCreation")
            || hook.starts_with("authorizationClass")
            || hook.starts_with("selectedRust")
            || hook == "concreteLeafAuthorizationFiber"
        {
            &full_spend_lean
        } else {
            &source
        };
        require_contains(haystack, hook, "authorization-class source hook")?;
    }

    println!("Path 2 authorization-class binding audit consistency check passed.");
    Ok(())
}

fn marker_window(window: &str) -> &str {
    window
}
