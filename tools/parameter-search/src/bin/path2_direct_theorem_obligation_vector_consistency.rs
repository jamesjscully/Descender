use descender_parameter_search::production_reference::{
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_direct_lifecycle_boundary_report_v1, path2_direct_lifecycle_boundary_report_v1_digest,
    path2_direct_qrom_action_boundary_report_v1,
    path2_direct_qrom_action_boundary_report_v1_digest, path2_direct_theorem_boundary_report_v1,
    path2_direct_theorem_boundary_report_v1_digest,
    validate_path2_account_creation_proof_boundary_report_v1,
    validate_path2_direct_lifecycle_boundary_report_v1,
    validate_path2_direct_qrom_action_boundary_report_v1,
    validate_path2_direct_theorem_boundary_report_v1,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
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

fn require_str(value: &Value, pointer: &str, expected: &str) -> Result<(), String> {
    let observed = str_at(value, pointer)?;
    if observed != expected {
        return Err(format!(
            "direct theorem obligation consistency {pointer} mismatch: {observed:?} != {expected:?}"
        ));
    }
    Ok(())
}

fn require_hex(value: &Value, pointer: &str, expected: &[u8]) -> Result<(), String> {
    let observed = str_at(value, pointer)?;
    let expected_hex = hex_encode(expected);
    if observed != expected_hex {
        return Err(format!(
            "direct theorem obligation consistency {pointer} mismatch: {observed} != {expected_hex}"
        ));
    }
    Ok(())
}

fn require_bool(value: &Value, pointer: &str, expected: bool) -> Result<(), String> {
    let observed = bool_at(value, pointer)?;
    if observed != expected {
        return Err(format!(
            "direct theorem obligation consistency {pointer} mismatch: {observed} != {expected}"
        ));
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let corpus_path = "formal/kat/path2_direct_theorem_obligation_vectors_v1.json";
    let corpus_file = load_json(&root, corpus_path)?;
    let corpus = at(&corpus_file, "/corpus")?;
    let corpus_json = serde_json::to_string_pretty(corpus)
        .map_err(|err| format!("serialize direct theorem obligation corpus body: {err}"))?;
    let recomputed_digest = hex_encode(&Sha256::digest(corpus_json.as_bytes()));
    if str_at(&corpus_file, "/corpus_digest")? != recomputed_digest {
        return Err("direct theorem obligation vector corpus digest mismatch".to_string());
    }
    let accepted_tuple_path = "formal/kat/path2_direct_accepted_spend_tuple_evidence_v1.json";
    let accepted_tuple_file = load_json(&root, accepted_tuple_path)?;
    let accepted_tuple = at(&accepted_tuple_file, "/manifest")?;
    let accepted_tuple_json = serde_json::to_string_pretty(accepted_tuple)
        .map_err(|err| format!("serialize direct accepted spend tuple manifest body: {err}"))?;
    let accepted_tuple_recomputed_digest =
        hex_encode(&Sha256::digest(accepted_tuple_json.as_bytes()));
    if str_at(&accepted_tuple_file, "/manifest_digest")? != accepted_tuple_recomputed_digest {
        return Err("direct accepted spend tuple manifest digest mismatch".to_string());
    }
    require_str(
        accepted_tuple,
        "/schema",
        "Descender.Path2.DirectAcceptedSpendTupleDeterministicKatEvidenceManifest.v1",
    )?;
    require_str(
        accepted_tuple,
        "/status",
        "digest_pinned_deterministic_kat_accepted_direct_spend_tuple_evidence",
    )?;
    require_bool(
        accepted_tuple,
        "/deterministic_tuple_mode/evidence_digest_recomputed",
        true,
    )?;
    require_bool(
        accepted_tuple,
        "/deterministic_tuple_mode/replay_rejected_by_consume_once",
        true,
    )?;
    if u64_at(accepted_tuple, "/deterministic_tuple_mode/rayon_threads")? != 1 {
        return Err(
            "direct accepted spend tuple manifest is not in one-thread KAT mode".to_string(),
        );
    }
    let accepted_composition_path = "formal/kat/path2_direct_accepted_composition_evidence_v1.json";
    let accepted_composition_file = load_json(&root, accepted_composition_path)?;
    let accepted_composition = at(&accepted_composition_file, "/manifest")?;
    let accepted_composition_json = serde_json::to_string_pretty(accepted_composition)
        .map_err(|err| format!("serialize direct accepted composition manifest body: {err}"))?;
    let accepted_composition_recomputed_digest =
        hex_encode(&Sha256::digest(accepted_composition_json.as_bytes()));
    if str_at(&accepted_composition_file, "/manifest_digest")?
        != accepted_composition_recomputed_digest
    {
        return Err("direct accepted composition manifest digest mismatch".to_string());
    }
    require_str(
        accepted_composition,
        "/schema",
        "Descender.Path2.DirectAcceptedCompositionDeterministicKatEvidenceManifest.v1",
    )?;
    require_str(
        accepted_composition,
        "/status",
        "digest_pinned_deterministic_kat_accepted_direct_composition_evidence",
    )?;
    require_bool(
        accepted_composition,
        "/deterministic_proof_byte_mode/evidence_digest_recomputed",
        true,
    )?;
    if u64_at(
        accepted_composition,
        "/deterministic_proof_byte_mode/rayon_threads",
    )? != 1
    {
        return Err(
            "direct accepted composition manifest is not in one-thread KAT mode".to_string(),
        );
    }

    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    let direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    let qrom_action_report = path2_direct_qrom_action_boundary_report_v1();
    validate_path2_direct_qrom_action_boundary_report_v1(&qrom_action_report)?;
    let qrom_action_digest =
        path2_direct_qrom_action_boundary_report_v1_digest(&qrom_action_report);
    let lifecycle_report = path2_direct_lifecycle_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&lifecycle_report)?;
    let lifecycle_digest = path2_direct_lifecycle_boundary_report_v1_digest(&lifecycle_report);
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    let proof_digest = path2_account_creation_proof_boundary_report_v1_digest(&proof_report);

    if str_at(corpus, "/schema")? != "Descender.Path2.DirectTheoremObligationVectors.v1" {
        return Err("direct theorem obligation vector corpus schema mismatch".to_string());
    }
    if str_at(corpus, "/status")? != "digest_pinned_direct_theorem_obligation_audit_vectors" {
        return Err("direct theorem obligation vector corpus status mismatch".to_string());
    }
    if str_at(corpus, "/direct_theorem_report/digest")? != hex_encode(&direct_report.digest)
        || str_at(corpus, "/direct_theorem_report/recomputed_digest")? != hex_encode(&direct_digest)
    {
        return Err(
            "direct theorem obligation vector corpus is not bound to direct report".to_string(),
        );
    }
    if str_at(
        corpus,
        "/direct_theorem_report/accepted_tuple_evidence_schema",
    )? != direct_report.accepted_tuple_evidence_schema
        || str_at(
            corpus,
            "/direct_theorem_report/accepted_tuple_evidence_endpoint",
        )? != direct_report.accepted_tuple_evidence_endpoint
        || bool_at(
            corpus,
            "/direct_theorem_report/accepted_tuple_evidence_present",
        )? != direct_report.accepted_tuple_evidence_present
        || str_at(
            corpus,
            "/direct_theorem_report/accepted_composition_evidence_schema",
        )? != direct_report.accepted_composition_evidence_schema
        || str_at(
            corpus,
            "/direct_theorem_report/accepted_composition_evidence_endpoint",
        )? != direct_report.accepted_composition_evidence_endpoint
        || bool_at(
            corpus,
            "/direct_theorem_report/accepted_composition_evidence_present",
        )? != direct_report.accepted_composition_evidence_present
    {
        return Err(
            "direct theorem obligation vector corpus lost accepted evidence hooks".to_string(),
        );
    }
    require_str(
        accepted_tuple,
        "/deterministic_tuple_mode/accepted_tuple_endpoint",
        direct_report.accepted_tuple_evidence_endpoint,
    )?;
    require_str(
        accepted_tuple,
        "/evidence/schema",
        direct_report.accepted_tuple_evidence_schema,
    )?;
    require_str(
        accepted_tuple,
        "/evidence/theorem_target",
        direct_report.theorem_target,
    )?;
    require_str(
        accepted_tuple,
        "/evidence/theorem_conclusion",
        direct_report.theorem_conclusion,
    )?;
    require_hex(
        accepted_tuple,
        "/evidence/direct_qrom_action_boundary_report_digest",
        &direct_report.direct_qrom_action_boundary_report_digest,
    )?;
    require_hex(
        accepted_tuple,
        "/evidence/direct_lifecycle_boundary_report_digest",
        &direct_report.direct_lifecycle_boundary_report_digest,
    )?;
    require_hex(
        accepted_tuple,
        "/evidence/account_creation_proof_boundary_report_digest",
        &direct_report.account_creation_proof_boundary_report_digest,
    )?;
    require_str(
        accepted_composition,
        "/deterministic_proof_byte_mode/accepted_composition_endpoint",
        direct_report.accepted_composition_evidence_endpoint,
    )?;
    require_str(
        accepted_composition,
        "/evidence/schema",
        direct_report.accepted_composition_evidence_schema,
    )?;
    require_str(
        accepted_composition,
        "/evidence/theorem_target",
        direct_report.theorem_target,
    )?;
    require_str(
        accepted_composition,
        "/evidence/theorem_conclusion",
        direct_report.theorem_conclusion,
    )?;
    require_hex(
        accepted_composition,
        "/evidence/direct_theorem_boundary_report_digest",
        &direct_report.digest,
    )?;
    require_hex(
        accepted_composition,
        "/evidence/direct_qrom_action_boundary_report_digest",
        &direct_report.direct_qrom_action_boundary_report_digest,
    )?;
    require_hex(
        accepted_composition,
        "/evidence/direct_lifecycle_boundary_report_digest",
        &direct_report.direct_lifecycle_boundary_report_digest,
    )?;
    require_hex(
        accepted_composition,
        "/evidence/account_creation_proof_boundary_report_digest",
        &direct_report.account_creation_proof_boundary_report_digest,
    )?;
    if str_at(corpus, "/qrom_action_boundary_report/digest")?
        != hex_encode(&qrom_action_report.digest)
        || str_at(corpus, "/qrom_action_boundary_report/recomputed_digest")?
            != hex_encode(&qrom_action_digest)
    {
        return Err("direct theorem obligation vector corpus lost QROM/action binding".to_string());
    }
    if str_at(corpus, "/lifecycle_boundary_report/digest")? != hex_encode(&lifecycle_report.digest)
        || str_at(corpus, "/lifecycle_boundary_report/recomputed_digest")?
            != hex_encode(&lifecycle_digest)
    {
        return Err("direct theorem obligation vector corpus lost lifecycle binding".to_string());
    }
    if str_at(corpus, "/account_creation_proof_boundary_report/digest")?
        != hex_encode(&proof_report.digest)
        || str_at(
            corpus,
            "/account_creation_proof_boundary_report/recomputed_digest",
        )? != hex_encode(&proof_digest)
    {
        return Err(
            "direct theorem obligation vector corpus lost proof-boundary binding".to_string(),
        );
    }
    if str_at(
        corpus,
        "/account_creation_proof_boundary_report/accepted_boundary_evidence_schema",
    )? != proof_report.accepted_boundary_evidence_schema
        || str_at(
            corpus,
            "/account_creation_proof_boundary_report/accepted_boundary_evidence_endpoint",
        )? != proof_report.accepted_boundary_evidence_endpoint
        || bool_at(
            corpus,
            "/account_creation_proof_boundary_report/accepted_boundary_evidence_present",
        )? != proof_report.accepted_boundary_evidence_present
        || bool_at(
            corpus,
            "/account_creation_proof_boundary_report/accepted_boundary_binding_evidence_present",
        )? != proof_report.accepted_boundary_binding_evidence_present
        || bool_at(
            corpus,
            "/account_creation_proof_boundary_report/replay_payload_digest_binding_evidence_present",
        )? != proof_report.replay_payload_digest_binding_evidence_present
        || bool_at(
            corpus,
            "/account_creation_proof_boundary_report/deployed_anchor_binding_evidence_present",
        )? != proof_report.deployed_anchor_binding_evidence_present
    {
        return Err(
            "direct theorem obligation vector corpus lost accepted proof-boundary evidence hook"
                .to_string(),
        );
    }
    if direct_report.direct_qrom_action_boundary_report_digest != qrom_action_report.digest
        || direct_report.direct_lifecycle_boundary_report_digest != lifecycle_report.digest
        || direct_report.account_creation_proof_boundary_report_digest != proof_report.digest
    {
        return Err("direct theorem report lost imported boundary digest binding".to_string());
    }
    require_hex(
        accepted_composition,
        "/evidence/account_creation_proof_boundary_report_digest",
        &proof_report.digest,
    )?;
    let tuple_artifact = at(corpus, "/direct_accepted_spend_tuple_evidence_artifact")?;
    if str_at(tuple_artifact, "/artifact_manifest_digest")?
        != str_at(&accepted_tuple_file, "/manifest_digest")?
        || str_at(tuple_artifact, "/artifact_manifest_recomputed_digest")?
            != accepted_tuple_recomputed_digest
        || str_at(tuple_artifact, "/evidence_digest")?
            != str_at(accepted_tuple, "/evidence/digest")?
        || str_at(tuple_artifact, "/evidence_recomputed_digest")?
            != str_at(accepted_tuple, "/evidence/recomputed_digest")?
        || str_at(tuple_artifact, "/direct_theorem_boundary_report_digest")?
            != hex_encode(&direct_report.digest)
        || str_at(tuple_artifact, "/direct_qrom_action_boundary_report_digest")?
            != hex_encode(&qrom_action_report.digest)
        || str_at(tuple_artifact, "/direct_lifecycle_boundary_report_digest")?
            != hex_encode(&lifecycle_report.digest)
        || str_at(
            tuple_artifact,
            "/account_creation_proof_boundary_report_digest",
        )? != hex_encode(&proof_report.digest)
    {
        return Err(
            "direct theorem obligation vector corpus lost accepted tuple artifact binding"
                .to_string(),
        );
    }

    if u64_at(corpus, "/case_count")? != direct_report.obligation_count as u64
        || u64_at(corpus, "/audit_summary/direct_obligation_count")?
            != direct_report.obligation_count as u64
        || u64_at(
            corpus,
            "/audit_summary/direct_obligation_audit_vector_case_count",
        )? != direct_report.obligation_audit_vector_case_count as u64
        || u64_at(
            corpus,
            "/direct_theorem_report/obligation_audit_vector_case_count",
        )? != direct_report.obligation_audit_vector_case_count as u64
    {
        return Err("direct theorem obligation vector corpus case count drifted".to_string());
    }
    if !bool_at(
        corpus,
        "/direct_theorem_report/every_hypothesis_has_obligation",
    )? || !bool_at(
        corpus,
        "/direct_theorem_report/every_obligation_has_named_break",
    )? || !bool_at(
        corpus,
        "/direct_theorem_report/every_open_obligation_blocks_deployment",
    )? || !bool_at(corpus, "/audit_summary/all_cases_open")?
        || !bool_at(corpus, "/audit_summary/all_cases_have_named_break")?
        || !bool_at(corpus, "/audit_summary/all_open_cases_block_deployment")?
    {
        return Err(
            "direct theorem obligation vector corpus lost fail-closed row invariants".to_string(),
        );
    }
    for pointer in [
        "/direct_theorem_report/composition_theorem_instantiated",
        "/direct_theorem_report/account_creation_proof_boundary_verified",
        "/direct_theorem_report/lifecycle_transition_profile_verified",
        "/direct_theorem_report/qrom_and_action_bounds_linked_to_theorem",
        "/direct_theorem_report/deployment_ready",
        "/qrom_action_boundary_report/theorem_imports_budget",
        "/qrom_action_boundary_report/external_qrom_review_complete",
        "/qrom_action_boundary_report/qrom_and_action_bounds_linked_to_theorem",
        "/qrom_action_boundary_report/deployment_ready",
        "/lifecycle_boundary_report/target_chain_finality_profile_instantiated",
        "/lifecycle_boundary_report/lifecycle_transition_profile_verified",
        "/lifecycle_boundary_report/deployment_ready",
        "/account_creation_proof_boundary_report/deployment_ready",
        "/audit_summary/composition_theorem_instantiated",
        "/audit_summary/account_creation_proof_boundary_verified",
        "/audit_summary/lifecycle_transition_profile_verified",
        "/audit_summary/qrom_and_action_bounds_linked_to_theorem",
        "/audit_summary/proof_boundary_deployment_ready",
        "/audit_summary/qrom_theorem_imports_budget",
        "/audit_summary/qrom_external_review_complete",
        "/audit_summary/lifecycle_target_chain_finality_profile_instantiated",
        "/audit_summary/deployment_ready",
    ] {
        if bool_at(corpus, pointer)? {
            return Err(format!(
                "direct theorem obligation vector corpus unexpectedly closes {pointer}"
            ));
        }
    }
    for pointer in [
        "/evidence/state_namespace_record_present",
        "/evidence/consumed_authorization_absent_before",
        "/evidence/consumed_authorization_present_after",
        "/evidence/consume_once_inserted_exactly_one_authorization",
        "/evidence/public_result_accepts_same_slot",
        "/evidence/tuple_fields_match_theorem_boundary",
        "/evidence/replay_rejected_by_consume_once",
    ] {
        require_bool(accepted_tuple, pointer, true)?;
    }
    for pointer in [
        "/evidence/creation_evidence_matches_direct_tuple",
        "/evidence/proof_boundary_digest_matches_direct_theorem",
        "/evidence/qrom_action_digest_matches_direct_theorem",
        "/evidence/lifecycle_digest_matches_direct_theorem",
        "/evidence/direct_admission_namespace_matches_authorization",
        "/evidence/consume_once_transition_matches_tuple",
        "/evidence/verifier_boundary_accepts_creation",
        "/evidence/direct_spend_tuple_accepts",
        "/evidence/public_result_accepts_same_slot",
        "/evidence/direct_theorem_obligations_still_open",
        "/evidence/proof_family_obligations_still_open",
    ] {
        require_bool(accepted_composition, pointer, true)?;
    }
    for pointer in [
        "/evidence/composition_theorem_instantiated",
        "/evidence/account_creation_proof_boundary_verified",
        "/evidence/deployment_ready",
    ] {
        require_bool(accepted_composition, pointer, false)?;
    }
    if str_at(
        accepted_composition,
        "/evidence/direct_accepted_spend_tuple_evidence_digest",
    )? != str_at(accepted_tuple, "/evidence/digest")?
        || str_at(accepted_composition, "/evidence/public_key_identity")?
            != str_at(accepted_tuple, "/evidence/public_key_identity")?
        || str_at(accepted_composition, "/evidence/message_digest")?
            != str_at(accepted_tuple, "/evidence/message_digest")?
        || str_at(accepted_composition, "/evidence/account_id")?
            != str_at(accepted_tuple, "/evidence/account_id")?
        || str_at(accepted_composition, "/evidence/account_state_root_before")?
            != str_at(accepted_tuple, "/evidence/account_state_root_before")?
        || u64_at(accepted_composition, "/evidence/slot_id")?
            != u64_at(accepted_tuple, "/evidence/slot_id")?
        || str_at(
            accepted_composition,
            "/evidence/prevalidated_slot_leaf_digest",
        )? != str_at(accepted_tuple, "/evidence/prevalidated_slot_leaf_digest")?
        || str_at(
            accepted_composition,
            "/evidence/account_creation_evidence_transcript_digest",
        )? != str_at(
            accepted_tuple,
            "/evidence/account_creation_evidence_transcript_digest",
        )?
        || str_at(
            accepted_composition,
            "/evidence/selected_account_creation_payload_digest",
        )? != str_at(
            accepted_tuple,
            "/evidence/selected_account_creation_payload_digest",
        )?
        || str_at(accepted_composition, "/evidence/slot_authorization_digest")?
            != str_at(accepted_tuple, "/evidence/slot_authorization_digest")?
        || str_at(
            accepted_composition,
            "/evidence/direct_admission_namespace_record_digest",
        )? != str_at(
            accepted_tuple,
            "/evidence/direct_admission_namespace_record_digest",
        )?
        || str_at(
            accepted_composition,
            "/evidence/consume_once_state_before_digest",
        )? != str_at(accepted_tuple, "/evidence/consume_once_state_before_digest")?
        || str_at(
            accepted_composition,
            "/evidence/consume_once_state_after_digest",
        )? != str_at(accepted_tuple, "/evidence/consume_once_state_after_digest")?
    {
        return Err(
            "direct theorem obligation vector corpus accepted composition tuple fields drifted"
                .to_string(),
        );
    }
    let payload_vectors = load_json(
        &root,
        "formal/kat/path2_account_stark_payload_kat_vectors_v1.json",
    )?;
    require_str(
        accepted_tuple,
        "/evidence/selected_account_creation_payload_digest",
        str_at(&payload_vectors, "/corpus/honest_payload/payload_digest")?,
    )?;
    require_str(
        accepted_composition,
        "/evidence/selected_account_creation_payload_digest",
        str_at(&payload_vectors, "/corpus/honest_payload/payload_digest")?,
    )?;

    if str_at(corpus, "/qrom_action_boundary_report/direct_obligation_id")?
        != qrom_action_report.direct_obligation_id
        || str_at(corpus, "/lifecycle_boundary_report/direct_obligation_id")?
            != lifecycle_report.direct_obligation_id
    {
        return Err("direct imported boundary obligation ids drifted".to_string());
    }

    let cases = at(corpus, "/cases")?
        .as_array()
        .ok_or_else(|| "direct theorem obligation corpus cases are not an array".to_string())?;
    if cases.len() != direct_report.obligation_rows.len() {
        return Err("direct theorem obligation corpus cases array drifted".to_string());
    }
    for (index, ((case, row), expected_case_id)) in cases
        .iter()
        .zip(direct_report.obligation_rows.iter())
        .zip(direct_report.obligation_audit_vector_case_ids.iter())
        .enumerate()
    {
        let case_id = case
            .get("case_id")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("direct theorem obligation case {index} has no case_id"))?;
        if case_id != row.obligation_id || case_id != *expected_case_id {
            return Err(format!(
                "direct theorem obligation case {index} id {case_id:?} drifted"
            ));
        }
        if case.get("kind").and_then(Value::as_str) != Some("audit") {
            return Err(format!(
                "direct theorem obligation case {case_id} is not an audit case"
            ));
        }
        for (field, expected) in [
            ("acceptance_hypothesis", row.acceptance_hypothesis),
            ("supporting_surface", row.supporting_surface),
            ("current_evidence", row.current_evidence),
            ("required_closure", row.required_closure),
            ("named_break", row.named_break),
            ("blocker", row.blocker),
        ] {
            if case.get(field).and_then(Value::as_str) != Some(expected) {
                return Err(format!(
                    "direct theorem obligation case {case_id} field {field} drifted"
                ));
            }
        }
        if case.get("open").and_then(Value::as_bool) != Some(true) || !row.open {
            return Err(format!(
                "direct theorem obligation case {case_id} unexpectedly closed"
            ));
        }
    }

    println!("Path 2 direct theorem obligation vector consistency check passed.");
    Ok(())
}
