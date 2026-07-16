use descender_parameter_search::production_reference::{
    path2_direct_qrom_action_boundary_report_v1,
    path2_direct_qrom_action_boundary_report_v1_digest, path2_direct_theorem_boundary_report_v1,
    validate_path2_direct_qrom_action_boundary_report_v1,
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
        .ok_or_else(|| format!("JSON pointer {pointer} is not an unsigned integer"))
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
        Err(format!("{label} still contains stale text {needle:?}"))
    } else {
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let manifest = load_json(
        &root,
        "formal/kat/path2_direct_qrom_action_boundary_v1.json",
    )?;
    let source = load_text(&root, "tools/parameter-search/src/production_reference.rs")?;
    let full_spend = load_text(&root, "Descender/Path2FullSpendConcrete.lean")?;
    let trace_model = load_text(&root, "Descender/Path2TraceModel.lean")?;

    let report = path2_direct_qrom_action_boundary_report_v1();
    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_qrom_action_boundary_report_v1(&report)?;
    let report_digest = hex_encode(&report.digest);
    let recomputed_digest =
        hex_encode(&path2_direct_qrom_action_boundary_report_v1_digest(&report));

    if str_at(&manifest, "/manifest/schema")?
        != "Descender.Path2.DirectQromActionBoundaryManifest.v1"
    {
        return Err("direct QROM/action boundary manifest schema mismatch".to_string());
    }
    if str_at(&manifest, "/manifest/frozen_status")? != "draft_qrom_action_boundary_manifest" {
        return Err("direct QROM/action boundary manifest frozen_status drifted".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/direct_qrom_action_boundary_report_digest",
    )? != report_digest
    {
        return Err("QROM/action manifest digest is not bound to report".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/direct_qrom_action_boundary_report_recomputed_digest",
    )? != recomputed_digest
    {
        return Err("QROM/action manifest recomputed digest mismatch".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/direct_theorem_report_qrom_action_digest",
    )? != hex_encode(&direct_report.direct_qrom_action_boundary_report_digest)
    {
        return Err("direct theorem QROM/action digest is not reflected in manifest".to_string());
    }
    if direct_report.direct_qrom_action_boundary_report_digest != report.digest {
        return Err("direct theorem report is not bound to QROM/action report".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/coverage/direct_qrom_action_theorem_linkage_reduction_lean_theorem",
    )? != report.direct_qrom_action_theorem_linkage_reduction_lean_theorem
    {
        return Err("QROM/action linkage reduction theorem drifted".to_string());
    }

    for (pointer, expected) in [
        (
            "/manifest/authority_snapshot/support_file",
            report.support_file,
        ),
        (
            "/manifest/authority_snapshot/direct_obligation_id",
            report.direct_obligation_id,
        ),
        (
            "/manifest/authority_snapshot/acceptance_hypothesis",
            report.acceptance_hypothesis,
        ),
        (
            "/manifest/authority_snapshot/named_break",
            report.named_break,
        ),
        (
            "/manifest/authority_snapshot/query_budget_schema",
            report.query_budget_schema,
        ),
        (
            "/manifest/authority_snapshot/query_budget_mode",
            report.query_budget_mode,
        ),
        (
            "/manifest/authority_snapshot/checked_core_challenge_frame",
            report.checked_core_challenge_frame,
        ),
        (
            "/manifest/authority_snapshot/fq12_action_domain",
            report.fq12_action_domain,
        ),
    ] {
        if str_at(&manifest, pointer)? != expected {
            return Err(format!("direct QROM/action manifest drifted at {pointer}"));
        }
    }

    for (pointer, expected) in [
        (
            "/manifest/numeric_budget/target_security_bits",
            report.target_security_bits,
        ),
        (
            "/manifest/numeric_budget/transcript_term_bits_after_qrom_loss",
            report.transcript_term_bits_after_qrom_loss,
        ),
        (
            "/manifest/numeric_budget/total_challenge_bits_after_mask",
            report.total_challenge_bits_after_mask,
        ),
        (
            "/manifest/numeric_budget/action_collision_floor_bits",
            report.action_collision_floor_bits,
        ),
        (
            "/manifest/numeric_budget/minimum_deployment_margin_bits",
            report.minimum_deployment_margin_bits,
        ),
    ] {
        if u64_at(&manifest, pointer)? != expected as u64 {
            return Err(format!("direct QROM/action manifest drifted at {pointer}"));
        }
    }

    for pointer in [
        "/manifest/coverage/checked_core_qrom_budget_pinned",
        "/manifest/coverage/action_map_budget_pinned",
        "/manifest/coverage/direct_mode_q_chal_active",
        "/manifest/coverage/direct_mode_q_batch_inactive",
        "/manifest/coverage/optional_aggregation_terms_q_batch_only",
        "/manifest/coverage/selected_horizon_within_margin",
        "/manifest/coverage/negative_control_exceeds_margin",
        "/manifest/coverage/direct_qrom_action_budget_bridge_bound",
        "/manifest/coverage/direct_qrom_action_local_linkage_reduction_imported",
        "/manifest/coverage/production_checked_core_acceptance_semantics_bound",
        "/manifest/coverage/qrom_action_valid_or_named_break_theorem_instantiated",
        "/manifest/coverage/qrom_domain_break_coupled_to_direct_game",
    ] {
        if !bool_at(&manifest, pointer)? {
            return Err(format!(
                "direct QROM/action manifest lost required flag {pointer}"
            ));
        }
    }
    for pointer in [
        "/manifest/open_obligations/qrom_action_assumption_instantiated",
        "/manifest/open_obligations/theorem_imports_budget",
        "/manifest/open_obligations/external_qrom_review_complete",
        "/manifest/open_obligations/qrom_and_action_bounds_linked_to_theorem",
        "/manifest/open_obligations/deployment_ready",
    ] {
        if bool_at(&manifest, pointer)? {
            return Err(format!(
                "direct QROM/action manifest unexpectedly closes {pointer}"
            ));
        }
    }

    let support_doc = load_text(&root, report.support_file)?;
    for needle in [
        "path2_direct_query_budget_v1",
        "DESCENDER_PATH2_DIRECT_QUERY_BUDGET_V1",
        "transcript term after QROM loss | `182` bits",
        "minimum deployment margin | `53` bits",
        "q_batch",
        "Path2DirectQromActionBoundaryReportV1",
        "qrom_or_domain_separation_break",
    ] {
        require_contains(&support_doc, needle, report.support_file)?;
    }
    for stale in [
        "transcript term after QROM loss | 130 bits",
        "minimum deployment margin | 1 bit",
    ] {
        require_absent(&support_doc, stale, report.support_file)?;
    }

    for artifact in report.checked_core_qrom_artifacts {
        let artifact_text = load_text(&root, artifact)?;
        require_contains(&artifact_text, "182", artifact)?;
    }

    for needle in [
        "Path2DirectQromActionBoundaryReportV1",
        "path2_direct_qrom_action_boundary_report_v1",
        "path2_direct_query_budget_v1",
        "PATH2_FQ12_ACTION_COLLISION_FLOOR_BITS",
        "path2_checked_core_authorization_challenge_frame",
        "path2_direct_query_budget_keeps_batch_inactive",
        "selectedRustDirectQromActionBudgetBridge_v1",
        "selectedRustDirectQromActionTheoremLinkageReduction_v1",
        "path2_direct_production_checked_core_acceptance_semantics_bound_v1",
        "path2_direct_checked_core_valid_or_named_break_instantiated_v1",
        "path2_direct_qrom_domain_break_coupled_to_direct_game_v1",
    ] {
        require_contains(&source, needle, "production reference source")?;
    }

    for needle in [
        "spend.signatureBody.qromOrDomainSeparationBreak",
        "spend.signatureBody.liveSlotLeakageOrKeygenBreak",
        "selectedRustDirectQromActionTheoremSupport",
        "selectedRustDirectQromActionTheoremLinkageReduction_v1",
        "ConcreteCheckedCoreAuthorizationAcceptance",
        "ConcreteCheckedCoreAuthorizationValid",
        "selectedRustProductionCheckedCoreAccepted_valid_or_named_break_v1",
        "concreteCheckedCoreDomainQromBreak_couples_to_directGame_v1",
    ] {
        require_contains(&full_spend, needle, "Path2FullSpendConcrete.lean")?;
    }
    for stale in [
        "selectedRustConcreteAcceptedSpendPricingWitness_qromBreakProb_eq_zero",
        "selectedRustConcreteAcceptedSpendPricingWitness_liveSlotLeakageBreakProb_eq_zero",
        "SelectedRustAcceptedSpendRemainingBreakPricingPackage.qromBreakProb_eq_zero",
        "SelectedRustAcceptedSpendRemainingBreakPricingPackage.liveSlotLeakageBreakProb_eq_zero",
    ] {
        require_absent(&trace_model, stale, "Path2TraceModel.lean")?;
    }
    for needle in [
        "remainingCryptographicBreakEvents.card = 9",
        "Fintype.card RemainingCryptographicBreak = 9",
        "ws7RemainingBreakBoundObligations.card = 9",
        "ws7RemainingClosureObligations.card = 10",
    ] {
        require_contains(&trace_model, needle, "Path2TraceModel.lean")?;
    }

    println!("Path 2 direct QROM/action boundary consistency check passed.");
    Ok(())
}
