use descender_parameter_search::production_reference::{
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_account_stark_residual_matrix_report_v1,
    path2_account_stark_residual_matrix_report_v1_digest, path2_direct_theorem_boundary_report_v1,
    path2_direct_theorem_boundary_report_v1_digest,
    production_deployment_account_creation_proof_boundary_blockers,
    production_deployment_account_stark_residual_blockers,
    production_deployment_direct_theorem_blockers, production_deployment_preflight,
    production_deployment_preflight_report, production_deployment_preflight_report_digest,
    validate_path2_account_creation_proof_boundary_report_v1,
    validate_path2_account_stark_residual_matrix_report_v1,
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
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    let residual_report = path2_account_stark_residual_matrix_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    validate_path2_account_stark_residual_matrix_report_v1(&residual_report)?;
    if path2_direct_theorem_boundary_report_v1_digest(&direct_report) != direct_report.digest {
        return Err("direct theorem boundary digest mismatch".to_string());
    }
    if path2_account_creation_proof_boundary_report_v1_digest(&proof_report) != proof_report.digest
    {
        return Err("account-creation proof boundary digest mismatch".to_string());
    }
    if path2_account_stark_residual_matrix_report_v1_digest(&residual_report)
        != residual_report.digest
    {
        return Err("account-STARK residual matrix digest mismatch".to_string());
    }
    if direct_report.account_creation_proof_boundary_report_digest != proof_report.digest {
        return Err("direct theorem report is not bound to the proof-boundary report".to_string());
    }
    if proof_report.account_stark_residual_matrix_report_digest != residual_report.digest {
        return Err("proof-boundary report is not bound to the residual matrix".to_string());
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
