use descender_parameter_search::production_reference::{
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_account_stark_fri_aok_boundary_report_v1,
    path2_account_stark_fri_aok_boundary_report_v1_digest,
    path2_account_stark_fri_qrom_residual_report_v1,
    path2_account_stark_fri_qrom_residual_report_v1_digest,
    path2_account_stark_residual_matrix_report_v1,
    path2_account_stark_residual_matrix_report_v1_digest,
    path2_direct_lifecycle_boundary_report_v1,
    path2_direct_lifecycle_boundary_report_v1_digest,
    path2_direct_qrom_action_boundary_report_v1,
    path2_direct_qrom_action_boundary_report_v1_digest,
    path2_direct_theorem_boundary_report_v1,
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
        return Err("direct QROM/action boundary unexpectedly reports closed obligations".to_string());
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
        return Err("direct lifecycle boundary unexpectedly reports closed obligations".to_string());
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

    println!("Path 2 preflight boundary consistency check passed.");
    Ok(())
}
