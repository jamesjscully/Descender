use descender_parameter_search::production_reference::{
    PROFILE, path2_direct_lifecycle_boundary_report_v1,
    path2_direct_lifecycle_boundary_report_v1_digest,
    validate_path2_direct_lifecycle_boundary_report_v1,
};
use serde_json::{Value, json};
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

fn support_tests_present(report_tests: &[&str], support_tests: &[&str]) -> bool {
    support_tests
        .iter()
        .all(|support| report_tests.iter().any(|test| test == support))
}

fn manifest_body() -> Result<Value, String> {
    let report = path2_direct_lifecycle_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&report)?;
    let recomputed_digest = path2_direct_lifecycle_boundary_report_v1_digest(&report);
    if recomputed_digest != report.digest {
        return Err("direct lifecycle boundary report digest mismatch".to_string());
    }

    let cases = vec![
        json!({
            "case_id": "lifecycle_transition_admit_spend_consume_once",
            "kind": "lifecycle_transition_kat",
            "transition_phases": ["admit", "spend", "consume"],
            "support_tests": [
                "path2_prevalidated_slot_consume_once_rejects_reauthorization",
                "path2_direct_single_slot_spend_composes_admission_signature_and_consume_once"
            ],
            "evidence_present": true,
            "named_break_if_missing": report.named_break,
            "profile_status": "local_successor_state_evidence_not_deployment_profile"
        }),
        json!({
            "case_id": "lifecycle_transition_replay_rejected",
            "kind": "lifecycle_transition_kat",
            "transition_phases": ["spend", "replay"],
            "support_tests": [
                "lifecycle_manager_verify_and_consume_blocks_replay",
                "spent_slot_ledger_blocks_stale_wallet_replay"
            ],
            "evidence_present": true,
            "named_break_if_missing": report.named_break,
            "profile_status": "local_replay_evidence_not_deployment_profile"
        }),
        json!({
            "case_id": "lifecycle_transition_chain_persistence",
            "kind": "lifecycle_transition_kat",
            "transition_phases": ["spend", "persist", "restore"],
            "support_tests": ["chain_state_file_stores_verify_record_and_persist_together"],
            "evidence_present": true,
            "named_break_if_missing": report.named_break,
            "profile_status": "local_file_store_evidence_not_deployment_profile"
        }),
        json!({
            "case_id": "lifecycle_transition_reorg_burn",
            "kind": "lifecycle_transition_kat",
            "transition_phases": ["reorg", "burn"],
            "support_tests": ["spent_slot_ledger_records_reorg_burns"],
            "evidence_present": true,
            "named_break_if_missing": report.named_break,
            "profile_status": "local_reorg_burn_evidence_not_deployment_profile"
        }),
        json!({
            "case_id": "lifecycle_transition_recovery_burn",
            "kind": "lifecycle_transition_kat",
            "transition_phases": ["recovery", "burn"],
            "support_tests": ["lifecycle_manager_recovery_burns_reserved_slots"],
            "evidence_present": true,
            "named_break_if_missing": report.named_break,
            "profile_status": "local_recovery_burn_evidence_not_deployment_profile"
        }),
        json!({
            "case_id": "lifecycle_transition_restore_anchor",
            "kind": "lifecycle_transition_kat",
            "transition_phases": ["restore", "reject_stale_snapshot"],
            "support_tests": ["lifecycle_restore_anchor_rejects_old_backup_snapshots"],
            "evidence_present": true,
            "named_break_if_missing": report.named_break,
            "profile_status": "local_restore_anchor_evidence_not_deployment_profile"
        }),
        json!({
            "case_id": "lifecycle_transition_file_store_recovery",
            "kind": "lifecycle_transition_kat",
            "transition_phases": ["restore", "recovery", "burn", "persist"],
            "support_tests": [
                "lifecycle_snapshot_roundtrips_and_recovers_by_burning_reservations",
                "lifecycle_file_store_roundtrips_and_recovers_reserved_slots"
            ],
            "evidence_present": true,
            "named_break_if_missing": report.named_break,
            "profile_status": "local_recovery_persistence_evidence_not_deployment_profile"
        }),
        json!({
            "case_id": "lifecycle_transition_d2_finality_window",
            "kind": "lifecycle_transition_kat",
            "transition_phases": ["finalize", "reorg", "free_pending", "preserve_finalized"],
            "support_tests": [
                "path2_d2_finality_profile_reorg_frees_window_but_preserves_finalized"
            ],
            "evidence_present": true,
            "named_break_if_missing": report.named_break,
            "profile_status": "selected_d2_finality_evidence_not_deployment_profile"
        }),
    ];

    if cases.len() != report.transition_vector_case_ids.len() {
        return Err("lifecycle transition vector generator case count drifted".to_string());
    }
    let mut all_support_tests_present = true;
    for (case, expected) in cases
        .iter()
        .zip(report.transition_vector_case_ids.iter())
    {
        if case["case_id"] != *expected {
            return Err(format!(
                "lifecycle transition vector case id drift: expected {expected}"
            ));
        }
        let support_tests = case
            .get("support_tests")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("case {expected} has no support_tests array"))?
            .iter()
            .map(|test| {
                test.as_str()
                    .ok_or_else(|| format!("case {expected} has non-string support test"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        all_support_tests_present &=
            support_tests_present(report.executable_tests, support_tests.as_slice());
    }
    if !all_support_tests_present {
        return Err("lifecycle transition vector generator lost support test binding".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.DirectLifecycleTransitionVectors.v1",
        "corpus_id": "Descender.Path2.DirectLifecycleTransitionVectors.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "status": "digest_pinned_direct_lifecycle_transition_vectors",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_lifecycle_transition_vectors",
            "source": "tools/parameter-search/src/bin/path2_direct_lifecycle_transition_vectors.rs"
        },
        "lifecycle_boundary_report": {
            "domain": report.domain,
            "digest": hex_encode(&report.digest),
            "recomputed_digest": hex_encode(&recomputed_digest),
            "theorem_target": report.theorem_target,
            "direct_obligation_id": report.direct_obligation_id,
            "acceptance_hypothesis": report.acceptance_hypothesis,
            "named_break": report.named_break,
            "target_chain_finality_profile_id": report.target_chain_finality_profile_id,
            "target_chain_finality_profile_instantiated":
                report.target_chain_finality_profile_instantiated,
            "lifecycle_transition_profile_verified":
                report.lifecycle_transition_profile_verified,
            "deployment_ready": report.deployment_ready,
            "blocker": report.blocker
        },
        "audit_summary": {
            "case_count": cases.len(),
            "case_ids": report.transition_vector_case_ids,
            "executable_test_count": report.executable_tests.len(),
            "all_cases_have_support_tests": true,
            "all_support_tests_present": all_support_tests_present,
            "all_cases_have_evidence": true,
            "covers_admit": true,
            "covers_spend": true,
            "covers_replay": true,
            "covers_burn": true,
            "covers_recovery": true,
            "covers_restore": true,
            "covers_reorg": true,
            "covers_d2_finality_window": true,
            "local_successor_state_model_pinned":
                report.local_successor_state_model_pinned,
            "transition_vector_corpus_pinned": report.transition_vector_corpus_pinned,
            "target_chain_finality_profile_instantiated":
                report.target_chain_finality_profile_instantiated,
            "lifecycle_transition_profile_verified":
                report.lifecycle_transition_profile_verified,
            "deployment_ready": report.deployment_ready
        },
        "case_count": cases.len(),
        "cases": cases
    }))
}

fn main() -> Result<(), String> {
    let corpus = manifest_body()?;
    let corpus_json = serde_json::to_string_pretty(&corpus)
        .map_err(|err| format!("failed to serialize lifecycle transition corpus body: {err}"))?;
    let out = json!({
        "corpus_digest": hex_encode(&Sha256::digest(corpus_json.as_bytes())),
        "corpus": corpus
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&out)
            .map_err(|err| format!("failed to serialize lifecycle transition corpus: {err}"))?
    );
    Ok(())
}
