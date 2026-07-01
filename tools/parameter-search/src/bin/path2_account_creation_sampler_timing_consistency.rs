use descender_parameter_search::production_reference::{
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_sampler_timing_report_v1,
    path2_account_creation_sampler_timing_report_v1_digest,
    path2_authorization_class_binding_audit_report_v1,
    validate_path2_account_creation_sampler_timing_report_v1,
    validate_path2_authorization_class_binding_audit_report_v1,
    PATH2_ACCOUNT_CREATION_SAMPLER_AUTHORIZATION_BRIDGE_HOOKS,
    PATH2_ACCOUNT_CREATION_SAMPLER_INTEGRATION_ROWS,
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
        "formal/kat/path2_account_creation_sampler_timing_v1.json",
    )?;
    let source = load_text(&root, "tools/parameter-search/src/production_reference.rs")?;
    let sampler_doc = load_text(&root, "research/path2_honest_create_sampler_v1.md")?;
    let timing_doc = load_text(
        &root,
        "research/path2_no_postchallenge_creation_timing_game.md",
    )?;
    let full_spend_lean = load_text(&root, "Descender/Path2FullSpendConcrete.lean")?;

    let report = path2_account_creation_sampler_timing_report_v1();
    let authorization_class_audit = path2_authorization_class_binding_audit_report_v1();
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_sampler_timing_report_v1(&report)?;
    validate_path2_authorization_class_binding_audit_report_v1(&authorization_class_audit)?;
    let report_digest = hex_encode(&report.digest);
    let recomputed_digest = hex_encode(&path2_account_creation_sampler_timing_report_v1_digest(
        &report,
    ));

    if str_at(&manifest, "/manifest/schema")?
        != "Descender.Path2.AccountCreationSamplerTimingManifest.v1"
    {
        return Err("sampler/timing manifest schema mismatch".to_string());
    }
    if str_at(&manifest, "/manifest/frozen_status")? != "draft_sampler_timing_manifest" {
        return Err("sampler/timing manifest frozen_status drifted".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/account_creation_sampler_timing_report_digest",
    )? != report_digest
    {
        return Err("sampler/timing manifest digest is not bound to report".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/account_creation_sampler_timing_report_recomputed_digest",
    )? != recomputed_digest
    {
        return Err("sampler/timing manifest recomputed digest mismatch".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/authorization_class_binding_audit_report_digest",
    )? != hex_encode(&authorization_class_audit.digest)
        || report.authorization_class_binding_audit_report_digest
            != authorization_class_audit.digest
    {
        return Err(
            "sampler/timing manifest is not bound to authorization-class audit".to_string(),
        );
    }
    if str_at(
        &manifest,
        "/manifest/digests/proof_boundary_report_sampler_timing_digest",
    )? != hex_encode(&proof_report.sampler_timing_report_digest)
    {
        return Err(
            "proof-boundary sampler/timing digest is not reflected in manifest".to_string(),
        );
    }
    if proof_report.sampler_timing_report_digest != report.digest {
        return Err("proof-boundary report is not bound to sampler/timing report".to_string());
    }

    for (pointer, expected) in [
        (
            "/manifest/authority_snapshot/sampler_support_file",
            report.sampler_support_file,
        ),
        (
            "/manifest/authority_snapshot/timing_support_file",
            report.timing_support_file,
        ),
        (
            "/manifest/authority_snapshot/sampler_theorem_target",
            report.sampler_theorem_target,
        ),
        (
            "/manifest/authority_snapshot/timing_break_class",
            report.timing_break_class,
        ),
        (
            "/manifest/authority_snapshot/creation_statement_builder",
            report.creation_statement_builder,
        ),
        (
            "/manifest/authority_snapshot/checked_core_challenge_frame",
            report.checked_core_challenge_frame,
        ),
        (
            "/manifest/authority_snapshot/direct_state_namespace_validator",
            report.direct_state_namespace_validator,
        ),
        (
            "/manifest/authority_snapshot/consume_once_validator",
            report.consume_once_validator,
        ),
        (
            "/manifest/authority_snapshot/direct_spend_composition_endpoint",
            report.direct_spend_composition_endpoint,
        ),
    ] {
        if str_at(&manifest, pointer)? != expected {
            return Err(format!("sampler/timing manifest drifted at {pointer}"));
        }
    }
    if u64_at(
        &manifest,
        "/manifest/sampler_boundary/authorization_bridge_hook_count",
    )? != PATH2_ACCOUNT_CREATION_SAMPLER_AUTHORIZATION_BRIDGE_HOOKS.len() as u64
        || u64_at(
            &manifest,
            "/manifest/sampler_boundary/authorization_bridge_hook_count",
        )? != report.sampler_authorization_bridge_hook_count as u64
    {
        return Err("sampler/timing authorization bridge hook count drift".to_string());
    }
    if u64_at(
        &manifest,
        "/manifest/sampler_boundary/sampler_integration_row_count",
    )? != PATH2_ACCOUNT_CREATION_SAMPLER_INTEGRATION_ROWS.len() as u64
        || u64_at(
            &manifest,
            "/manifest/sampler_boundary/sampler_integration_row_count",
        )? != report.sampler_integration_row_count as u64
    {
        return Err("sampler/timing integration row count drift".to_string());
    }
    if u64_at(
        &manifest,
        "/manifest/sampler_boundary/sampler_integration_open_count",
    )? != report.sampler_integration_open_count as u64
        || report.sampler_integration_open_count
            != PATH2_ACCOUNT_CREATION_SAMPLER_INTEGRATION_ROWS
                .iter()
                .filter(|row| row.open)
                .count()
    {
        return Err("sampler/timing integration open count drift".to_string());
    }
    for pointer in [
        "/manifest/sampler_boundary/every_sampler_integration_row_has_evidence",
        "/manifest/sampler_boundary/selected_rust_bridge_hooks_imported_by_sampler_integration",
        "/manifest/sampler_boundary/accepted_boundary_imports_sampler_integration",
    ] {
        if !bool_at(&manifest, pointer)? {
            return Err(format!(
                "sampler/timing manifest lost integration evidence flag {pointer}"
            ));
        }
    }
    if bool_at(
        &manifest,
        "/manifest/sampler_boundary/full_honest_sampler_integration_complete",
    )? {
        return Err(
            "sampler/timing manifest unexpectedly closes full honest sampler integration"
                .to_string(),
        );
    }
    let integration_rows_json = serde_json::to_string(at(
        &manifest,
        "/manifest/sampler_boundary/sampler_integration_rows",
    )?)
    .map_err(|err| format!("serialize sampler integration rows: {err}"))?;
    let integration_support = format!("{source}\n{sampler_doc}\n{timing_doc}\n{full_spend_lean}");
    for row in PATH2_ACCOUNT_CREATION_SAMPLER_INTEGRATION_ROWS {
        require_contains(
            &integration_rows_json,
            row.integration_id,
            "sampler/timing integration manifest rows",
        )?;
        require_contains(
            &integration_rows_json,
            row.current_evidence,
            "sampler/timing integration manifest evidence",
        )?;
        require_contains(
            &integration_support,
            row.current_evidence,
            "sampler/timing integration support sources",
        )?;
    }
    for integration_id in ["sampler_integration_authorization_bridge_hooks"] {
        require_contains(
            &integration_rows_json,
            integration_id,
            "required sampler/timing integration row",
        )?;
    }
    for hook in PATH2_ACCOUNT_CREATION_SAMPLER_AUTHORIZATION_BRIDGE_HOOKS {
        require_contains(
            &serde_json::to_string(at(
                &manifest,
                "/manifest/sampler_boundary/authorization_bridge_hook_ids",
            )?)
            .map_err(|err| format!("serialize sampler bridge hook ids: {err}"))?,
            hook,
            "sampler/timing authorization bridge manifest hooks",
        )?;
        require_contains(
            &full_spend_lean,
            hook,
            "Lean sampler authorization bridge hooks",
        )?;
    }
    for hook in [
        "selectedRustHonestCreate_authorization_over_class",
        "SelectedRustHonestCreateSamplerSoundTheoremClosed",
        "selectedRustHonestCreateSamplerSoundTheoremClosed",
        "selectedRustHonestSamplerIntegrationChecklist_consumed_of_boundaryAccepted",
        "selectedRustAcceptedBoundaryHonestSamplerSound_of_boundaryAccepted_honestCreate",
        "selectedRustAcceptedBoundaryHonestCreate_of_boundaryAccepted",
        "SelectedRustAcceptedBoundaryPrivateToPublicOrderRealized",
        "selectedRustAcceptedBoundaryPrivateToPublicOrder_of_boundaryAccepted",
        "SelectedRustAcceptedBoundaryAuthorizationClassBindingSupport",
        "selectedRustAcceptedBoundaryAuthorizationClassBindingSupport_of_boundaryAccepted",
        "SelectedRustAuthorizationClassBindingTheoremClosed",
        "selectedRustAuthorizationClassBindingTheoremClosed_of_boundaryAccepted",
        "selectedRustAcceptedBoundaryHonestSamplerSound_of_boundaryAccepted",
        "SelectedRustAcceptedBoundarySamplerIntegrationRowsSupported",
        "selectedRustAcceptedBoundarySamplerIntegrationRowsSupported_of_boundaryAccepted",
        "selectedRustMeaningProjection_spendAuthorizationClass_eq_pSpend",
    ] {
        require_contains(
            &full_spend_lean,
            hook,
            "required selected-Rust sampler bridge hook",
        )?;
    }

    for pointer in [
        "/manifest/executable_hooks/creation_statement_excludes_spend_message",
        "/manifest/executable_hooks/creation_statement_excludes_signature_bytes",
        "/manifest/executable_hooks/checked_core_challenge_binds_message",
        "/manifest/executable_hooks/checked_core_challenge_binds_state_root",
        "/manifest/executable_hooks/checked_core_challenge_binds_slot_authorization_digest",
        "/manifest/executable_hooks/checked_core_challenge_binds_proof_commitment",
        "/manifest/executable_hooks/state_namespace_membership_required",
        "/manifest/executable_hooks/consume_once_required",
        "/manifest/executable_hooks/direct_spend_composition_hook_present",
        "/manifest/sampler_boundary/selected_rust_honest_create_bridge_present",
        "/manifest/sampler_boundary/selected_rust_projection_bridge_present",
    ] {
        if !bool_at(&manifest, pointer)? {
            return Err(format!(
                "sampler/timing manifest lost required hook flag {pointer}"
            ));
        }
    }
    for pointer in ["/manifest/open_obligations/deployment_ready"] {
        if bool_at(&manifest, pointer)? {
            return Err(format!(
                "sampler/timing manifest unexpectedly closes {pointer}"
            ));
        }
    }
    if !bool_at(
        &manifest,
        "/manifest/open_obligations/honest_sampler_theorem_instantiated",
    )? {
        return Err(
            "sampler/timing manifest lost imported deterministic honest-sampler theorem"
                .to_string(),
        );
    }
    if !bool_at(
        &manifest,
        "/manifest/open_obligations/authorization_class_binding_theorem_instantiated",
    )? {
        return Err(
            "sampler/timing manifest lost imported authorization-class binding theorem"
                .to_string(),
        );
    }
    if !bool_at(
        &manifest,
        "/manifest/open_obligations/no_postchallenge_timing_theorem_instantiated",
    )? {
        return Err(
            "sampler/timing manifest lost imported no-postchallenge timing theorem".to_string(),
        );
    }

    for (needle, label) in [
        ("honest_create_sampler_sound_v1", "sampler theorem target"),
        (
            "authorizationLocal",
            "authorization-local construction step",
        ),
        (
            "p_spend(q)=AuthClass_c(authorizationLocal)",
            "authorization class binding law",
        ),
        ("No Consensus Leakage", "no consensus leakage section"),
        (
            "authorization_class_binding_break",
            "authorization binding break class",
        ),
    ] {
        require_contains(&sampler_doc, needle, label)?;
    }
    for (needle, label) in [
        (
            "postchallenge_slot_creation_or_rewrite_break",
            "timing break class",
        ),
        (
            "Steps 1 through 6 must precede step 9",
            "prechallenge timing order",
        ),
        (
            "Path2CreationHiddenSourceStatementV1` excludes spend message",
            "creation statement exclusion hook",
        ),
        (
            "path2_checked_core_authorization_challenge_frame",
            "checked-core challenge hook",
        ),
        (
            "validate_path2_direct_slot_authorization_state_namespace",
            "state namespace hook",
        ),
        (
            "verify_path2_prevalidated_slot_consume_once",
            "consume-once hook",
        ),
    ] {
        require_contains(&timing_doc, needle, label)?;
    }
    for (needle, label) in [
        (
            "AccountCreationAuthorizationClassObligationTracked",
            "tracked authorization-class predicate",
        ),
        (
            "authorizationClass_obligationDigest_nonzero_of_obligationTracked",
            "authorization-class nonzero digest Lean theorem",
        ),
        (
            "authorizationClass_boundary_eq_leafBoundary_of_obligationTracked",
            "authorization-class boundary equality Lean theorem",
        ),
        (
            "authorizationClass_matches_pSpend_of_obligationTracked",
            "authorization-class pSpend Lean theorem",
        ),
        (
            "spend.leaf.authorizationClass =",
            "authorization-class leaf equality conclusion",
        ),
        (
            "C.nativeProjections.pSpend spend.creationEvidence.boundary",
            "authorization-class pSpend conclusion",
        ),
        (
            "AccountCreationNoPostchallengeTimingObligationTracked",
            "tracked no-postchallenge timing predicate",
        ),
        (
            "noPostchallengeTiming_createdBeforeChallenge",
            "created-before-challenge Lean theorem",
        ),
        (
            "noPostchallengeTiming_lateAdmission_excluded",
            "late-admission exclusion Lean theorem",
        ),
        (
            "spend.creationEvidence.acceptedAt ≤ (spend.spendChallengeTime : Nat)",
            "accepted evidence timing bound",
        ),
        (
            "e'.acceptedAt ≠ spend.creationEvidence.acceptedAt",
            "late admission exclusion conclusion",
        ),
    ] {
        require_contains(&full_spend_lean, needle, label)?;
    }

    let creation_frame =
        function_window(&source, "path2_creation_hidden_source_statement_v1_frame")?;
    require_absent(
        creation_frame,
        "b\"message\"",
        "creation hidden-source statement frame",
    )?;
    require_absent(
        creation_frame,
        "b\"signature\"",
        "creation hidden-source statement frame",
    )?;
    let checked_core_frame =
        function_window(&source, "path2_checked_core_authorization_challenge_frame")?;
    for needle in [
        "b\"message\"",
        "b\"account_state_root\"",
        "b\"slot_authorization_digest\"",
        "b\"proof_commitment\"",
    ] {
        require_contains(checked_core_frame, needle, "checked-core challenge frame")?;
    }
    let state_namespace = function_window(
        &source,
        "validate_path2_direct_slot_authorization_state_namespace",
    )?;
    for needle in [
        "leaf.account_id",
        "leaf.account_state_root_before",
        "authorization",
        "admitted_state_authorizations.contains(&namespace)",
    ] {
        require_contains(state_namespace, needle, "direct state namespace validator")?;
    }
    let consume_once = function_window(&source, "verify_path2_prevalidated_slot_consume_once")?;
    require_contains(
        consume_once,
        "consumed_authorizations.insert(authorization)",
        "consume-once validator",
    )?;
    let direct_spend = function_window(
        &source,
        "verify_path2_direct_single_slot_spend_authorization",
    )?;
    for needle in [
        "validate_path2_direct_slot_authorization_state_namespace",
        "verify_path2_checked_core_authorization",
        "verify_path2_prevalidated_slot_consume_once",
    ] {
        require_contains(direct_spend, needle, "direct spend composition endpoint")?;
    }

    println!("Path 2 account-creation sampler/timing consistency check passed.");
    Ok(())
}
