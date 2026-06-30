use descender_parameter_search::production_reference::{
    path2_direct_lifecycle_boundary_report_v1, path2_direct_lifecycle_boundary_report_v1_digest,
    path2_direct_theorem_boundary_report_v1, validate_path2_direct_lifecycle_boundary_report_v1,
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

fn require_contains(haystack: &str, needle: &str, label: &str) -> Result<(), String> {
    if haystack.contains(needle) {
        Ok(())
    } else {
        Err(format!("{label} is missing {needle:?}"))
    }
}

fn require_order(haystack: &str, first: &str, second: &str, label: &str) -> Result<(), String> {
    let first_index = haystack
        .find(first)
        .ok_or_else(|| format!("{label} is missing {first:?}"))?;
    let second_index = haystack
        .find(second)
        .ok_or_else(|| format!("{label} is missing {second:?}"))?;
    if first_index < second_index {
        Ok(())
    } else {
        Err(format!(
            "{label} has {first:?} after {second:?}, expected public checks before mutation"
        ))
    }
}

fn function_window<'a>(source: &'a str, name: &str) -> Result<&'a str, String> {
    let pub_marker = format!("\npub fn {name}");
    let indented_pub_marker = format!("pub fn {name}");
    let fn_marker = format!("\nfn {name}");
    let start = source
        .find(&pub_marker)
        .or_else(|| source.find(&indented_pub_marker))
        .or_else(|| source.find(&fn_marker))
        .ok_or_else(|| format!("missing function {name}"))?;
    let tail = &source[start..];
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
    let manifest = load_json(&root, "formal/kat/path2_direct_lifecycle_boundary_v1.json")?;
    let source = load_text(&root, "tools/parameter-search/src/production_reference.rs")?;
    let support_doc = load_text(&root, "research/path2_one_use_successor_state_theorem.md")?;
    let adr0010_doc = load_text(
        &root,
        "decisions/ADR-0010-registry-lifecycle-consensus-wiring-and-finality.md",
    )?;
    let deployment_profiles_doc = load_text(&root, "spec/deployment_profiles.md")?;

    let report = path2_direct_lifecycle_boundary_report_v1();
    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&report)?;
    let report_digest = hex_encode(&report.digest);
    let recomputed_digest = hex_encode(&path2_direct_lifecycle_boundary_report_v1_digest(&report));

    if str_at(&manifest, "/manifest/schema")?
        != "Descender.Path2.DirectLifecycleBoundaryManifest.v1"
    {
        return Err("direct lifecycle boundary manifest schema mismatch".to_string());
    }
    if str_at(&manifest, "/manifest/frozen_status")? != "draft_lifecycle_boundary_manifest" {
        return Err("direct lifecycle boundary manifest frozen_status drifted".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/direct_lifecycle_boundary_report_digest",
    )? != report_digest
    {
        return Err("lifecycle manifest digest is not bound to report".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/direct_lifecycle_boundary_report_recomputed_digest",
    )? != recomputed_digest
    {
        return Err("lifecycle manifest recomputed digest mismatch".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/direct_theorem_report_lifecycle_digest",
    )? != hex_encode(&direct_report.direct_lifecycle_boundary_report_digest)
    {
        return Err("direct theorem lifecycle digest is not reflected in manifest".to_string());
    }
    if direct_report.direct_lifecycle_boundary_report_digest != report.digest {
        return Err("direct theorem report is not bound to lifecycle report".to_string());
    }
    if report.spent_slot_ledger_type != "ProductionSpentSlotLedger" {
        return Err(
            "direct lifecycle report drifted from production spent-slot ledger".to_string(),
        );
    }
    if report.chain_file_store_type != "ProductionChainStateFileStores" {
        return Err(
            "direct lifecycle report drifted from production chain file stores".to_string(),
        );
    }

    for (pointer, expected) in [
        (
            "/manifest/authority_snapshot/support_file",
            report.support_file,
        ),
        (
            "/manifest/authority_snapshot/theorem_target",
            report.theorem_target,
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
            "/manifest/authority_snapshot/consume_once_validator",
            report.consume_once_validator,
        ),
        (
            "/manifest/authority_snapshot/state_namespace_validator",
            report.state_namespace_validator,
        ),
        (
            "/manifest/authority_snapshot/direct_spend_endpoint",
            report.direct_spend_endpoint,
        ),
        (
            "/manifest/authority_snapshot/lifecycle_manager_endpoint",
            report.lifecycle_manager_endpoint,
        ),
        (
            "/manifest/authority_snapshot/spent_slot_ledger_type",
            report.spent_slot_ledger_type,
        ),
        (
            "/manifest/authority_snapshot/target_chain_adapter_type",
            report.target_chain_adapter_type,
        ),
        (
            "/manifest/authority_snapshot/chain_file_store_type",
            report.chain_file_store_type,
        ),
        (
            "/manifest/authority_snapshot/target_chain_finality_profile_id",
            report.target_chain_finality_profile_id,
        ),
        (
            "/manifest/authority_snapshot/target_chain_finality_profile_authority",
            report.target_chain_finality_profile_authority,
        ),
        (
            "/manifest/authority_snapshot/target_chain_finality_profile_rule",
            report.target_chain_finality_profile_rule,
        ),
    ] {
        if str_at(&manifest, pointer)? != expected {
            return Err(format!("direct lifecycle manifest drifted at {pointer}"));
        }
    }

    for pointer in [
        "/manifest/local_successor_state_boundary/coverage/local_successor_state_model_pinned",
        "/manifest/local_successor_state_boundary/coverage/consume_once_insert_after_public_checks",
        "/manifest/local_successor_state_boundary/coverage/replay_rejected_by_consume_set",
        "/manifest/local_successor_state_boundary/coverage/spent_slot_ledger_rejects_stale_wallet_replay",
        "/manifest/local_successor_state_boundary/coverage/chain_state_persists_lifecycle_and_spent_record",
        "/manifest/local_successor_state_boundary/coverage/reorg_burn_support_present",
        "/manifest/local_successor_state_boundary/coverage/restore_anchor_support_present",
        "/manifest/local_successor_state_boundary/coverage/target_chain_finality_profile_selected",
        "/manifest/local_successor_state_boundary/coverage/target_chain_finality_profile_instantiated",
        "/manifest/selected_finality_profile/target_chain_finality_profile_selected",
        "/manifest/selected_finality_profile/target_chain_finality_profile_instantiated",
    ] {
        if !bool_at(&manifest, pointer)? {
            return Err(format!(
                "direct lifecycle manifest lost required local flag {pointer}"
            ));
        }
    }
    for (pointer, expected) in [
        (
            "/manifest/selected_finality_profile/target_chain_finality_profile_id",
            report.target_chain_finality_profile_id,
        ),
        (
            "/manifest/selected_finality_profile/target_chain_finality_profile_authority",
            report.target_chain_finality_profile_authority,
        ),
        (
            "/manifest/selected_finality_profile/target_chain_finality_profile_rule",
            report.target_chain_finality_profile_rule,
        ),
        (
            "/manifest/selected_finality_profile/status",
            "selected_and_instantiated_by_executable_d2_reorg_model_not_verified_deployment_profile",
        ),
    ] {
        if str_at(&manifest, pointer)? != expected {
            return Err(format!(
                "direct lifecycle selected finality profile drifted at {pointer}"
            ));
        }
    }
    for pointer in [
        "/manifest/open_obligations/lifecycle_transition_profile_verified",
        "/manifest/open_obligations/deployment_ready",
    ] {
        if bool_at(&manifest, pointer)? {
            return Err(format!(
                "direct lifecycle manifest unexpectedly closes {pointer}"
            ));
        }
    }

    for (needle, label) in [
        ("slot_replay_or_rollback_break", "lifecycle break class"),
        ("Valid Predecessor State", "predecessor-state section"),
        ("Valid Successor State", "successor-state section"),
        (
            "verify_path2_direct_single_slot_spend_authorization",
            "direct spend hook",
        ),
        (
            "verify_path2_prevalidated_slot_consume_once",
            "consume-once hook",
        ),
        (
            "Path 2 slot authorization already consumed",
            "consume-once rejection",
        ),
        ("global consensus finality", "open finality non-claim"),
        (
            "reorg/finality policy",
            "deployment lifecycle policy boundary",
        ),
    ] {
        require_contains(&support_doc, needle, label)?;
    }
    for (needle, label) in [
        ("FINALIZED head", "D2 finality boundary"),
        ("[finalized, head]", "D2 reorg-able window"),
        ("reorg_to(new_head)", "D2 reorg-freeing rule"),
        (
            "permanence boundary equals finalized head",
            "D2 permanence boundary",
        ),
    ] {
        require_contains(&adr0010_doc, needle, label)?;
    }
    for (needle, label) in [
        (
            "It does not change the generic direct-mode theorem",
            "deployment-profile non-claim",
        ),
        (
            "A profile becomes adoption evidence only when the corresponding",
            "deployment-profile readiness rule",
        ),
        (
            "finality depth or finality source",
            "deployment finality requirement",
        ),
    ] {
        require_contains(&deployment_profiles_doc, needle, label)?;
    }

    let consume_once = function_window(&source, "verify_path2_prevalidated_slot_consume_once")?;
    require_order(
        consume_once,
        "verify_path2_prevalidated_slot_creation_payload_bindings",
        "consumed_authorizations.insert(authorization)",
        "consume-once validator",
    )?;
    require_contains(
        consume_once,
        "Path 2 slot authorization already consumed",
        "consume-once validator",
    )?;
    let direct_spend = function_window(
        &source,
        "verify_path2_direct_single_slot_spend_authorization",
    )?;
    for needle in [
        "validate_path2_direct_slot_authorization_state_namespace",
        "verify_path2_checked_core_authorization",
        "Path 2 checked-core slot id does not match prevalidated leaf",
        "verify_path2_prevalidated_slot_consume_once",
    ] {
        require_contains(direct_spend, needle, "direct spend lifecycle composition")?;
    }
    require_order(
        direct_spend,
        "verify_path2_checked_core_authorization",
        "verify_path2_prevalidated_slot_consume_once",
        "direct spend lifecycle composition",
    )?;
    let chain_store = function_window(&source, "verify_consume_record_and_save").unwrap_or("");
    require_contains(
        chain_store,
        "spent_ledger_store.save_ledger(&ledger)",
        "chain state file-store transition",
    )?;
    require_contains(
        chain_store,
        "lifecycle_store.save_manager(&manager)",
        "chain state file-store transition",
    )?;

    for test_name in report.executable_tests {
        require_contains(
            &source,
            &format!("fn {test_name}"),
            "lifecycle executable test",
        )?;
    }
    for needle in [
        "slot is already in spent ledger",
        "revealed valid signature burns slot across reorg",
        "snapshot sequence is older than restore anchor",
        "chain_state_file_stores_verify_record_and_persist_together",
        "Path2D2FinalityProfileState",
        "D2 finality profile rejects reorg below finalized head",
        "path2_d2_finality_profile_reorg_frees_window_but_preserves_finalized",
    ] {
        require_contains(&source, needle, "lifecycle regression evidence")?;
    }

    println!("Path 2 direct lifecycle boundary consistency check passed.");
    Ok(())
}
