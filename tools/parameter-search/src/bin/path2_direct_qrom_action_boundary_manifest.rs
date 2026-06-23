use descender_parameter_search::production_reference::{
    path2_direct_qrom_action_boundary_report_v1,
    path2_direct_qrom_action_boundary_report_v1_digest,
    path2_direct_query_budget_v1,
    path2_direct_theorem_boundary_report_v1,
    validate_path2_direct_qrom_action_boundary_report_v1,
    PROFILE,
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
    let report = path2_direct_qrom_action_boundary_report_v1();
    validate_path2_direct_qrom_action_boundary_report_v1(&report)?;
    let direct_report = path2_direct_theorem_boundary_report_v1();
    let query_budget = path2_direct_query_budget_v1();
    let recomputed_digest = path2_direct_qrom_action_boundary_report_v1_digest(&report);
    if recomputed_digest != report.digest {
        return Err("direct QROM/action boundary report digest mismatch".to_string());
    }
    if direct_report.direct_qrom_action_boundary_report_digest != report.digest {
        return Err("direct theorem report is not bound to QROM/action report".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.DirectQromActionBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.DirectQromActionBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_qrom_action_boundary_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_qrom_action_boundary_manifest",
            "source": "tools/parameter-search/src/bin/path2_direct_qrom_action_boundary_manifest.rs"
        },
        "authority_snapshot": {
            "qrom_action_report_domain": report.domain,
            "direct_report_domain": direct_report.domain,
            "support_file": report.support_file,
            "direct_obligation_id": report.direct_obligation_id,
            "acceptance_hypothesis": report.acceptance_hypothesis,
            "named_break": report.named_break,
            "query_budget_schema": report.query_budget_schema,
            "query_budget_mode": report.query_budget_mode,
            "checked_core_challenge_frame": report.checked_core_challenge_frame,
            "fq12_action_domain": report.fq12_action_domain,
            "blocker": report.blocker
        },
        "digests": {
            "direct_qrom_action_boundary_report_digest": hex_encode(&report.digest),
            "direct_qrom_action_boundary_report_recomputed_digest":
                hex_encode(&recomputed_digest),
            "direct_theorem_boundary_report_digest": hex_encode(&direct_report.digest),
            "direct_theorem_report_qrom_action_digest": hex_encode(
                &direct_report.direct_qrom_action_boundary_report_digest
            )
        },
        "numeric_budget": {
            "target_security_bits": report.target_security_bits,
            "deployment_quantum_hash_query_bound_log2":
                report.deployment_quantum_hash_query_bound_log2,
            "deployment_classical_hash_query_bound_log2":
                report.deployment_classical_hash_query_bound_log2,
            "qrom_quadratic_loss_bits_at_deployment_bound":
                report.qrom_quadratic_loss_bits_at_deployment_bound,
            "transcript_term_bits_after_qrom_loss":
                report.transcript_term_bits_after_qrom_loss,
            "algebraic_challenge_bytes": report.algebraic_challenge_bytes,
            "challenge_hardening_bytes": report.challenge_hardening_bytes,
            "algebraic_challenge_last_byte_mask": report.algebraic_challenge_last_byte_mask,
            "algebraic_challenge_bits_after_mask": report.algebraic_challenge_bits_after_mask,
            "total_challenge_bits_after_mask": report.total_challenge_bits_after_mask,
            "action_collision_floor_bits": report.action_collision_floor_bits,
            "action_collision_quantum_bits_at_union_bound":
                report.action_collision_quantum_bits_at_union_bound,
            "action_collision_classical_bits_at_union_bound":
                report.action_collision_classical_bits_at_union_bound,
            "selected_multi_target_horizon_log2":
                report.selected_multi_target_horizon_log2,
            "negative_control_horizon_log2": report.negative_control_horizon_log2,
            "minimum_deployment_margin_bits": report.minimum_deployment_margin_bits
        },
        "query_shape": {
            "active_query_symbol_count": report.active_query_symbol_count,
            "inactive_query_symbol_count": report.inactive_query_symbol_count,
            "direct_advantage_term_count": report.direct_advantage_term_count,
            "optional_aggregation_term_count": report.optional_aggregation_term_count,
            "active_query_symbols": query_budget.active_query_symbols,
            "inactive_query_symbols": query_budget.inactive_query_symbols,
            "direct_advantage_terms": query_budget.direct_advantage_terms,
            "optional_aggregation_terms": query_budget.optional_aggregation_terms
        },
        "coverage": {
            "checked_core_qrom_budget_pinned": report.checked_core_qrom_budget_pinned,
            "action_map_budget_pinned": report.action_map_budget_pinned,
            "direct_mode_q_chal_active": report.direct_mode_q_chal_active,
            "direct_mode_q_batch_inactive": report.direct_mode_q_batch_inactive,
            "optional_aggregation_terms_q_batch_only":
                report.optional_aggregation_terms_q_batch_only,
            "selected_horizon_within_margin": report.selected_horizon_within_margin,
            "negative_control_exceeds_margin": report.negative_control_exceeds_margin,
            "checked_core_qrom_artifacts": report.checked_core_qrom_artifacts
        },
        "open_obligations": {
            "theorem_imports_budget": report.theorem_imports_budget,
            "external_qrom_review_complete": report.external_qrom_review_complete,
            "qrom_and_action_bounds_linked_to_theorem":
                report.qrom_and_action_bounds_linked_to_theorem,
            "deployment_ready": report.deployment_ready,
            "blocker": report.blocker
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
