use descender_parameter_search::production_reference::{
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_account_stark_fri_aok_boundary_report_v1,
    path2_account_stark_fri_aok_boundary_report_v1_digest,
    path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_fri_qrom_residual_report_v1_digest,
    path2_account_stark_residual_matrix_report_v1,
    path2_account_stark_residual_matrix_report_v1_digest,
    path2_direct_lifecycle_boundary_report_v1, path2_direct_lifecycle_boundary_report_v1_digest,
    path2_direct_qrom_action_boundary_report_v1,
    path2_direct_qrom_action_boundary_report_v1_digest, path2_direct_theorem_boundary_report_v1,
    path2_direct_theorem_boundary_report_v1_digest,
    production_deployment_account_creation_proof_boundary_blockers,
    production_deployment_account_stark_residual_blockers,
    production_deployment_direct_theorem_blockers, production_deployment_preflight,
    production_deployment_preflight_report, production_deployment_preflight_report_digest,
    validate_path2_account_creation_proof_boundary_report_v1,
    validate_path2_account_stark_fri_aok_boundary_report_v1,
    validate_path2_account_stark_fri_qrom_residual_report_v1,
    validate_path2_account_stark_residual_matrix_report_v1,
    validate_path2_direct_lifecycle_boundary_report_v1,
    validate_path2_direct_qrom_action_boundary_report_v1,
    validate_path2_direct_theorem_boundary_report_v1,
    validate_production_deployment_preflight_report,
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

fn load_json(root: &Path, relative_path: &str) -> Result<Value, String> {
    let path = root.join(relative_path);
    let text =
        fs::read_to_string(&path).map_err(|err| format!("read {}: {err}", path.display()))?;
    serde_json::from_str(&text).map_err(|err| format!("parse {}: {err}", path.display()))
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

fn array_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a Vec<Value>, String> {
    at(value, pointer)?
        .as_array()
        .ok_or_else(|| format!("JSON pointer {pointer} is not an array"))
}

fn body_digest(value: &Value, label: &str) -> Result<String, String> {
    let json =
        serde_json::to_string_pretty(value).map_err(|err| format!("serialize {label}: {err}"))?;
    Ok(hex_encode(&Sha256::digest(json.as_bytes())))
}

fn require_manifest_digest<'a>(file: &'a Value, label: &str) -> Result<&'a Value, String> {
    let manifest = at(file, "/manifest")?;
    let recomputed = body_digest(manifest, label)?;
    if str_at(file, "/manifest_digest")? != recomputed {
        return Err(format!("{label} manifest digest mismatch"));
    }
    Ok(manifest)
}

fn contains(haystack: &[&str], needle: &str) -> bool {
    haystack.iter().any(|value| *value == needle)
}

fn find_subsequence(haystack: &[&str], needle: &[&str]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn require_subsequence_after(
    haystack: &[&str],
    prior_start: usize,
    prior_len: usize,
    needle: &[&str],
    label: &str,
) -> Result<usize, String> {
    let required_start = prior_start + prior_len;
    let observed_start = find_subsequence(haystack, needle)
        .ok_or_else(|| format!("production preflight is missing {label} blocker group"))?;
    if observed_start != required_start {
        return Err(format!(
            "production preflight {label} blocker group starts at {observed_start}, expected {required_start}"
        ));
    }
    Ok(observed_start)
}

fn main() -> Result<(), String> {
    let direct_report = path2_direct_theorem_boundary_report_v1();
    let qrom_action_report = path2_direct_qrom_action_boundary_report_v1();
    let lifecycle_report = path2_direct_lifecycle_boundary_report_v1();
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    let fri_qrom_report = path2_account_stark_fri_qrom_residual_report_v1();
    let fri_aok_report = path2_account_stark_fri_aok_boundary_report_v1();
    let residual_report = path2_account_stark_residual_matrix_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    validate_path2_direct_qrom_action_boundary_report_v1(&qrom_action_report)?;
    validate_path2_direct_lifecycle_boundary_report_v1(&lifecycle_report)?;
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    validate_path2_account_stark_fri_qrom_residual_report_v1(&fri_qrom_report)?;
    validate_path2_account_stark_fri_aok_boundary_report_v1(&fri_aok_report)?;
    validate_path2_account_stark_residual_matrix_report_v1(&residual_report)?;
    if path2_direct_theorem_boundary_report_v1_digest(&direct_report) != direct_report.digest {
        return Err("direct theorem boundary digest mismatch".to_string());
    }
    if path2_direct_qrom_action_boundary_report_v1_digest(&qrom_action_report)
        != qrom_action_report.digest
    {
        return Err("direct QROM/action boundary digest mismatch".to_string());
    }
    if path2_direct_lifecycle_boundary_report_v1_digest(&lifecycle_report)
        != lifecycle_report.digest
    {
        return Err("direct lifecycle boundary digest mismatch".to_string());
    }
    if path2_account_creation_proof_boundary_report_v1_digest(&proof_report) != proof_report.digest
    {
        return Err("account-creation proof boundary digest mismatch".to_string());
    }
    if path2_account_stark_fri_qrom_residual_report_v1_digest(&fri_qrom_report)
        != fri_qrom_report.digest
    {
        return Err("account-STARK FRI/QROM residual digest mismatch".to_string());
    }
    if path2_account_stark_fri_aok_boundary_report_v1_digest(&fri_aok_report)
        != fri_aok_report.digest
    {
        return Err("account-STARK FRI/AoK boundary digest mismatch".to_string());
    }
    if path2_account_stark_residual_matrix_report_v1_digest(&residual_report)
        != residual_report.digest
    {
        return Err("account-STARK residual matrix digest mismatch".to_string());
    }
    if direct_report.account_creation_proof_boundary_report_digest != proof_report.digest {
        return Err("direct theorem report is not bound to the proof-boundary report".to_string());
    }
    if direct_report.direct_qrom_action_boundary_report_digest != qrom_action_report.digest {
        return Err("direct theorem report is not bound to the QROM/action report".to_string());
    }
    if direct_report.direct_lifecycle_boundary_report_digest != lifecycle_report.digest {
        return Err("direct theorem report is not bound to the lifecycle report".to_string());
    }
    if proof_report.account_stark_residual_matrix_report_digest != residual_report.digest {
        return Err("proof-boundary report is not bound to the residual matrix".to_string());
    }
    if residual_report.fri_qrom_residual_report_digest != fri_qrom_report.digest {
        return Err("residual matrix is not bound to the FRI/QROM residual report".to_string());
    }
    if fri_aok_report.fri_qrom_residual_report_digest != fri_qrom_report.digest {
        return Err("FRI/AoK boundary is not bound to the FRI/QROM residual report".to_string());
    }
    if residual_report.fri_aok_boundary_report_digest != fri_aok_report.digest {
        return Err("residual matrix is not bound to the FRI/AoK boundary report".to_string());
    }

    let preflight = production_deployment_preflight();
    let report = production_deployment_preflight_report();
    validate_production_deployment_preflight_report(&report)?;
    if production_deployment_preflight_report_digest(&report) != report.digest {
        return Err("production preflight report digest mismatch".to_string());
    }
    if report.preflight != preflight {
        return Err("production preflight report does not embed the current preflight".to_string());
    }
    if preflight.deployment_ready {
        return Err("production preflight unexpectedly reports deployment_ready=true".to_string());
    }

    let root = repo_root();
    let proof_manifest_file = load_json(
        &root,
        "formal/kat/path2_account_creation_proof_boundary_v1.json",
    )?;
    let direct_manifest_file =
        load_json(&root, "formal/kat/path2_direct_theorem_boundary_v1.json")?;
    let proof_manifest =
        require_manifest_digest(&proof_manifest_file, "account-creation proof boundary")?;
    let direct_manifest =
        require_manifest_digest(&direct_manifest_file, "direct theorem boundary")?;
    let proof_boundary = at(proof_manifest, "/account_creation_proof_boundary")?;
    let direct_boundary = at(direct_manifest, "/direct_theorem_boundary")?;
    if proof_boundary != at(direct_manifest, "/account_creation_proof_boundary")? {
        return Err(
            "production preflight manifest bind lost shared proof-boundary object".to_string(),
        );
    }

    let direct_report_digest = hex_encode(&direct_report.digest);
    let qrom_action_report_digest = hex_encode(&qrom_action_report.digest);
    let lifecycle_report_digest = hex_encode(&lifecycle_report.digest);
    let proof_report_digest = hex_encode(&proof_report.digest);
    let fri_qrom_report_digest = hex_encode(&fri_qrom_report.digest);
    let fri_aok_report_digest = hex_encode(&fri_aok_report.digest);
    let residual_report_digest = hex_encode(&residual_report.digest);
    if str_at(
        direct_manifest,
        "/digests/direct_theorem_boundary_report_digest",
    )? != direct_report_digest
        || str_at(
            direct_manifest,
            "/digests/direct_qrom_action_boundary_report_digest",
        )? != qrom_action_report_digest
        || str_at(
            direct_manifest,
            "/digests/direct_lifecycle_boundary_report_digest",
        )? != lifecycle_report_digest
        || str_at(
            direct_manifest,
            "/digests/account_creation_proof_boundary_report_digest",
        )? != proof_report_digest
        || str_at(
            proof_manifest,
            "/digests/account_creation_proof_boundary_report_digest",
        )? != proof_report_digest
        || str_at(
            proof_manifest,
            "/digests/account_stark_fri_qrom_residual_report_digest",
        )? != fri_qrom_report_digest
        || str_at(
            proof_manifest,
            "/digests/account_stark_fri_aok_boundary_report_digest",
        )? != fri_aok_report_digest
        || str_at(
            proof_manifest,
            "/digests/account_stark_residual_matrix_report_digest",
        )? != residual_report_digest
    {
        return Err("production preflight manifest digests drifted from reports".to_string());
    }

    if at(direct_boundary, "/direct_accepted_composition_evidence_artifact")?
        != at(
            direct_boundary,
            "/direct_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact",
        )?
        || at(direct_boundary, "/direct_accepted_composition_evidence_artifact")?
            != at(
                proof_boundary,
                "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact",
            )?
        || at(proof_boundary, "/accepted_proof_boundary_evidence_artifact")?
            != at(
                proof_boundary,
                "/proof_obligation_vector_corpus_evidence/accepted_proof_boundary_evidence_artifact",
            )?
        || at(proof_boundary, "/accepted_proof_boundary_evidence_artifact")?
            != at(
                direct_boundary,
                "/direct_accepted_composition_evidence_artifact/account_creation_accepted_proof_boundary_evidence_artifact",
            )?
    {
        return Err(
            "production preflight manifest bind lost accepted evidence artifact equality"
                .to_string(),
        );
    }

    for pointer in [
        "/direct_accepted_composition_evidence_artifact/audit_summary/direct_theorem_obligations_still_open",
        "/direct_accepted_composition_evidence_artifact/audit_summary/proof_family_obligations_still_open",
        "/direct_accepted_composition_evidence_artifact/audit_summary/verifier_boundary_accepts_creation",
        "/direct_accepted_composition_evidence_artifact/audit_summary/direct_spend_tuple_accepts",
    ] {
        if !bool_at(direct_boundary, pointer)? {
            return Err(format!(
                "production preflight manifest bind lost expected true flag {pointer}"
            ));
        }
    }
    for pointer in [
        "/direct_accepted_composition_evidence_artifact/audit_summary/composition_theorem_instantiated",
        "/direct_accepted_composition_evidence_artifact/audit_summary/account_creation_proof_boundary_verified",
        "/direct_accepted_composition_evidence_artifact/audit_summary/deployment_ready",
        "/direct_obligation_vector_corpus_evidence/audit_summary/composition_theorem_instantiated",
        "/direct_obligation_vector_corpus_evidence/audit_summary/account_creation_proof_boundary_verified",
        "/direct_obligation_vector_corpus_evidence/audit_summary/lifecycle_transition_profile_verified",
        "/direct_obligation_vector_corpus_evidence/audit_summary/qrom_and_action_bounds_linked_to_theorem",
        "/direct_obligation_vector_corpus_evidence/audit_summary/deployment_ready",
    ] {
        if bool_at(direct_boundary, pointer)? {
            return Err(format!(
                "production preflight manifest bind unexpectedly closed {pointer}"
            ));
        }
    }
    for pointer in [
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/direct_theorem_obligations_still_open",
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/proof_family_obligations_still_open",
        "/accepted_proof_boundary_evidence_artifact/audit_summary/production_gate_fail_closed",
        "/accepted_proof_boundary_evidence_artifact/audit_summary/proof_family_obligations_still_open",
    ] {
        if !bool_at(proof_boundary, pointer)? {
            return Err(format!(
                "production preflight proof-boundary manifest lost expected true flag {pointer}"
            ));
        }
    }
    for pointer in [
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/composition_theorem_instantiated",
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/account_creation_proof_boundary_verified",
        "/proof_obligation_vector_corpus_evidence/direct_accepted_composition_evidence_artifact/audit_summary/deployment_ready",
        "/proof_obligation_vector_corpus_evidence/audit_summary/simulator_theorem_instantiated",
        "/proof_obligation_vector_corpus_evidence/audit_summary/fri_aok_extractor_theorem_instantiated",
        "/proof_obligation_vector_corpus_evidence/audit_summary/fri_qrom_residual_priced",
        "/proof_obligation_vector_corpus_evidence/audit_summary/frozen_kat_corpus_complete",
        "/proof_obligation_vector_corpus_evidence/audit_summary/production_gate_open",
        "/proof_obligation_vector_corpus_evidence/audit_summary/deployment_ready",
    ] {
        if bool_at(proof_boundary, pointer)? {
            return Err(format!(
                "production preflight proof-boundary manifest unexpectedly closed {pointer}"
            ));
        }
    }

    if direct_report.deployment_ready
        || direct_report.composition_theorem_instantiated
        || direct_report.account_creation_proof_boundary_verified
        || direct_report.lifecycle_transition_profile_verified
        || direct_report.qrom_and_action_bounds_linked_to_theorem
    {
        return Err("direct theorem boundary unexpectedly reports closed obligations".to_string());
    }
    if qrom_action_report.deployment_ready
        || qrom_action_report.theorem_imports_budget
        || qrom_action_report.external_qrom_review_complete
        || qrom_action_report.qrom_and_action_bounds_linked_to_theorem
    {
        return Err(
            "direct QROM/action boundary unexpectedly reports closed obligations".to_string(),
        );
    }
    if !qrom_action_report.checked_core_qrom_budget_pinned
        || !qrom_action_report.action_map_budget_pinned
        || !qrom_action_report.direct_mode_q_chal_active
        || !qrom_action_report.direct_mode_q_batch_inactive
        || !qrom_action_report.optional_aggregation_terms_q_batch_only
        || !qrom_action_report.selected_horizon_within_margin
        || !qrom_action_report.negative_control_exceeds_margin
    {
        return Err("direct QROM/action boundary is missing budget evidence".to_string());
    }
    if lifecycle_report.deployment_ready
        || lifecycle_report.target_chain_finality_profile_instantiated
        || lifecycle_report.lifecycle_transition_profile_verified
    {
        return Err(
            "direct lifecycle boundary unexpectedly reports closed obligations".to_string(),
        );
    }
    if !lifecycle_report.local_successor_state_model_pinned
        || !lifecycle_report.consume_once_insert_after_public_checks
        || !lifecycle_report.replay_rejected_by_consume_set
        || !lifecycle_report.spent_slot_ledger_rejects_stale_wallet_replay
        || !lifecycle_report.chain_state_persists_lifecycle_and_spent_record
        || !lifecycle_report.reorg_burn_support_present
        || !lifecycle_report.restore_anchor_support_present
    {
        return Err(
            "direct lifecycle boundary is missing local successor-state evidence".to_string(),
        );
    }
    if proof_report.deployment_ready
        || proof_report.simulator_theorem_instantiated
        || proof_report.fri_aok_extractor_theorem_instantiated
        || proof_report.fri_qrom_residual_priced
        || proof_report.frozen_kat_corpus_complete
        || proof_report.external_review_complete
        || proof_report.production_gate_open
    {
        return Err(
            "account-creation proof boundary unexpectedly reports closed obligations".to_string(),
        );
    }
    if !proof_report.proof_obligation_rows_pinned
        || !proof_report.every_proof_obligation_has_named_break
        || !proof_report.every_open_proof_obligation_blocks_deployment
        || proof_report.proof_obligation_count != proof_report.proof_obligation_rows.len()
    {
        return Err(
            "account-creation proof boundary lost proof-obligation blocker coverage".to_string(),
        );
    }
    if fri_qrom_report.deployment_ready
        || fri_qrom_report.residual_pricing_complete
        || fri_qrom_report.account_stark_transcript_qrom_theorem_instantiated
        || fri_qrom_report.recursion_pricing_complete
        || fri_qrom_report.external_fri_review_complete
        || fri_qrom_report.external_qrom_review_complete
        || fri_qrom_report.deployment_profile_accepted
    {
        return Err(
            "account-STARK FRI/QROM residual unexpectedly reports closed obligations".to_string(),
        );
    }
    if fri_aok_report.deployment_ready
        || fri_aok_report.fri_aok_extractor_theorem_instantiated
        || fri_aok_report.accepting_proof_implies_extractable_witness
        || fri_aok_report.external_extractor_review_complete
    {
        return Err(
            "account-STARK FRI/AoK boundary unexpectedly reports closed obligations".to_string(),
        );
    }
    if residual_report.deployment_ready {
        return Err(
            "account-STARK residual matrix unexpectedly reports deployment readiness".to_string(),
        );
    }
    if !residual_report.every_row_has_blocker || !residual_report.every_row_has_named_break {
        return Err(
            "account-STARK residual matrix rows are missing blockers or named breaks".to_string(),
        );
    }

    let blockers = preflight.blockers.as_slice();
    let direct_blockers = production_deployment_direct_theorem_blockers();
    let proof_blockers = production_deployment_account_creation_proof_boundary_blockers();
    let residual_blockers = production_deployment_account_stark_residual_blockers();
    if u64_at(direct_boundary, "/coverage/obligation_count")?
        != direct_report.obligation_count as u64
        || u64_at(
            direct_boundary,
            "/direct_obligation_vector_corpus_evidence/case_count",
        )? != direct_report.obligation_audit_vector_case_count as u64
        || u64_at(proof_boundary, "/coverage/proof_obligation_count")?
            != proof_report.proof_obligation_count as u64
        || u64_at(
            proof_boundary,
            "/proof_obligation_vector_corpus_evidence/case_count",
        )? != proof_report.proof_obligation_audit_vector_case_count as u64
    {
        return Err("production preflight manifest bind lost obligation case counts".to_string());
    }
    let direct_start = find_subsequence(blockers, direct_blockers)
        .ok_or_else(|| "production preflight is missing direct theorem blockers".to_string())?;
    let proof_start = require_subsequence_after(
        blockers,
        direct_start,
        direct_blockers.len(),
        proof_blockers,
        "account-creation proof-boundary",
    )?;
    require_subsequence_after(
        blockers,
        proof_start,
        proof_blockers.len(),
        residual_blockers,
        "account-STARK residual",
    )?;

    if !contains(blockers, direct_report.blocker) {
        return Err("production preflight does not include direct theorem blocker".to_string());
    }
    if !contains(blockers, proof_report.blocker) {
        return Err("production preflight does not include proof-boundary blocker".to_string());
    }
    if !contains(blockers, fri_aok_report.blocker) {
        return Err("production preflight does not include FRI/AoK boundary blocker".to_string());
    }
    for row in direct_report.obligation_rows {
        if row.open && !contains(blockers, row.blocker) {
            return Err(format!(
                "production preflight is missing blocker for direct obligation {}",
                row.obligation_id
            ));
        }
    }
    for row in residual_report.rows {
        if row.open && !contains(blockers, row.blocker) {
            return Err(format!(
                "production preflight is missing blocker for residual item {}",
                row.residual_item
            ));
        }
    }
    let manifest_direct_rows = array_at(direct_boundary, "/obligation_rows")?;
    if manifest_direct_rows.len() != direct_report.obligation_rows.len() {
        return Err("direct manifest obligation row count drifted from preflight".to_string());
    }
    for row in manifest_direct_rows {
        if bool_at(row, "/open")? {
            let blocker = str_at(row, "/blocker")?;
            if !contains(blockers, blocker) {
                return Err(format!(
                    "production preflight is missing manifest blocker for direct obligation {}",
                    str_at(row, "/obligation_id")?
                ));
            }
        }
    }
    let manifest_proof_rows = array_at(proof_boundary, "/proof_obligation_rows")?;
    if manifest_proof_rows.len() != proof_report.proof_obligation_rows.len() {
        return Err(
            "proof-boundary manifest obligation row count drifted from preflight".to_string(),
        );
    }
    for row in manifest_proof_rows {
        if bool_at(row, "/open")? {
            let blocker = str_at(row, "/blocker")?;
            if !contains(blockers, blocker) {
                return Err(format!(
                    "production preflight is missing manifest blocker for proof obligation {}",
                    str_at(row, "/obligation_id")?
                ));
            }
        }
    }

    println!("Path 2 preflight boundary consistency check passed.");
    Ok(())
}
