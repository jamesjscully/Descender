#![recursion_limit = "256"]

use descender_parameter_search::{
    manifest_vector_evidence::{
        path2_account_creation_accepted_proof_boundary_evidence_artifact_v1,
        path2_account_creation_proof_obligation_vector_corpus_evidence_v1,
        path2_account_creation_requirement_vector_corpus_evidence_v1,
        path2_direct_accepted_composition_evidence_artifact_v1,
        path2_direct_accepted_spend_tuple_evidence_artifact_v1,
        path2_direct_lifecycle_profile_review_evidence_v1,
        path2_direct_lifecycle_profile_verification_evidence_v1,
        path2_direct_lifecycle_transition_vector_corpus_evidence_v1,
        path2_direct_theorem_obligation_vector_corpus_evidence_v1,
    },
    production_reference::{
        PROFILE, path2_account_creation_proof_boundary_report_v1,
        path2_account_creation_proof_boundary_report_v1_digest,
        path2_account_creation_sampler_timing_report_v1,
        path2_account_creation_sampler_timing_report_v1_digest,
        path2_direct_admission_boundary_report_v1,
        path2_direct_admission_boundary_report_v1_digest,
        path2_direct_lifecycle_boundary_report_v1,
        path2_direct_lifecycle_boundary_report_v1_digest,
        path2_direct_qrom_action_boundary_report_v1,
        path2_direct_qrom_action_boundary_report_v1_digest, path2_direct_query_budget_v1,
        path2_direct_theorem_boundary_report_v1, path2_direct_theorem_boundary_report_v1_digest,
    },
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

fn manifest_body() -> Result<Value, String> {
    let direct_report = path2_direct_theorem_boundary_report_v1();
    let qrom_action_report = path2_direct_qrom_action_boundary_report_v1();
    let lifecycle_report = path2_direct_lifecycle_boundary_report_v1();
    let admission_report = path2_direct_admission_boundary_report_v1();
    let query_budget = path2_direct_query_budget_v1();
    let proof_boundary_report = path2_account_creation_proof_boundary_report_v1();
    let sampler_timing_report = path2_account_creation_sampler_timing_report_v1();
    let requirement_vector_corpus_evidence =
        path2_account_creation_requirement_vector_corpus_evidence_v1()?;
    let proof_obligation_vector_corpus_evidence =
        path2_account_creation_proof_obligation_vector_corpus_evidence_v1()?;
    let accepted_proof_boundary_evidence_artifact =
        path2_account_creation_accepted_proof_boundary_evidence_artifact_v1()?;
    let lifecycle_transition_vector_corpus_evidence =
        path2_direct_lifecycle_transition_vector_corpus_evidence_v1()?;
    let lifecycle_profile_review_evidence = path2_direct_lifecycle_profile_review_evidence_v1()?;
    let lifecycle_profile_verification_evidence =
        path2_direct_lifecycle_profile_verification_evidence_v1()?;
    let direct_obligation_vector_corpus_evidence =
        path2_direct_theorem_obligation_vector_corpus_evidence_v1()?;
    let direct_accepted_spend_tuple_evidence_artifact =
        path2_direct_accepted_spend_tuple_evidence_artifact_v1()?;
    let direct_accepted_composition_evidence_artifact =
        path2_direct_accepted_composition_evidence_artifact_v1()?;
    let recomputed_direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    if recomputed_direct_digest != direct_report.digest {
        return Err("direct theorem boundary report digest mismatch".to_string());
    }
    let recomputed_qrom_action_digest =
        path2_direct_qrom_action_boundary_report_v1_digest(&qrom_action_report);
    if recomputed_qrom_action_digest != qrom_action_report.digest {
        return Err("direct QROM/action boundary report digest mismatch".to_string());
    }
    let recomputed_lifecycle_digest =
        path2_direct_lifecycle_boundary_report_v1_digest(&lifecycle_report);
    if recomputed_lifecycle_digest != lifecycle_report.digest {
        return Err("direct lifecycle boundary report digest mismatch".to_string());
    }
    let recomputed_admission_digest =
        path2_direct_admission_boundary_report_v1_digest(&admission_report);
    if recomputed_admission_digest != admission_report.digest {
        return Err("direct admission boundary report digest mismatch".to_string());
    }
    let recomputed_proof_boundary_digest =
        path2_account_creation_proof_boundary_report_v1_digest(&proof_boundary_report);
    if recomputed_proof_boundary_digest != proof_boundary_report.digest {
        return Err("account-creation proof boundary report digest mismatch".to_string());
    }
    let recomputed_sampler_timing_digest =
        path2_account_creation_sampler_timing_report_v1_digest(&sampler_timing_report);
    if recomputed_sampler_timing_digest != sampler_timing_report.digest {
        return Err("account-creation sampler/timing report digest mismatch".to_string());
    }
    if direct_report.account_creation_proof_boundary_report_digest != proof_boundary_report.digest {
        return Err(
            "direct theorem report is not bound to the current proof-boundary report".to_string(),
        );
    }
    if direct_report.direct_qrom_action_boundary_report_digest != qrom_action_report.digest {
        return Err(
            "direct theorem report is not bound to the current QROM/action report".to_string(),
        );
    }
    if direct_report.direct_lifecycle_boundary_report_digest != lifecycle_report.digest {
        return Err(
            "direct theorem report is not bound to the current lifecycle report".to_string(),
        );
    }
    if direct_report.direct_admission_boundary_report_digest != admission_report.digest {
        return Err(
            "direct theorem report is not bound to the current admission report".to_string(),
        );
    }
    if proof_boundary_report.sampler_timing_report_digest != sampler_timing_report.digest {
        return Err(
            "proof boundary report is not bound to the current sampler/timing report".to_string(),
        );
    }

    Ok(json!({
        "schema": "Descender.Path2.DirectTheoremBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.DirectTheoremBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_theorem_boundary_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_theorem_boundary_manifest",
            "source": "tools/parameter-search/src/bin/path2_direct_theorem_boundary_manifest.rs"
        },
        "authority_snapshot": {
            "direct_report_domain": direct_report.domain,
            "direct_qrom_action_boundary_report_domain": qrom_action_report.domain,
            "direct_lifecycle_boundary_report_domain": lifecycle_report.domain,
            "direct_admission_boundary_report_domain": admission_report.domain,
            "account_creation_proof_boundary_report_domain": proof_boundary_report.domain,
            "account_creation_sampler_timing_report_domain": sampler_timing_report.domain,
            "direct_theorem_blocker": direct_report.blocker,
            "direct_qrom_action_blocker": qrom_action_report.blocker,
            "direct_lifecycle_blocker": lifecycle_report.blocker,
            "direct_admission_blocker": admission_report.blocker,
            "account_creation_proof_boundary_blocker": proof_boundary_report.blocker,
            "sampler_timing_blocker": sampler_timing_report.blocker,
            "direct_theorem_target": direct_report.theorem_target,
            "direct_theorem_conclusion": direct_report.theorem_conclusion,
            "direct_qrom_action_support_file": qrom_action_report.support_file,
            "direct_qrom_action_named_break": qrom_action_report.named_break,
            "direct_qrom_action_obligation_id": qrom_action_report.direct_obligation_id,
            "direct_lifecycle_support_file": lifecycle_report.support_file,
            "direct_lifecycle_theorem_target": lifecycle_report.theorem_target,
            "selected_payload_type": proof_boundary_report.selected_payload_type,
            "verifier_boundary_endpoint": proof_boundary_report.verifier_boundary_endpoint,
            "production_boundary_endpoint": proof_boundary_report.production_boundary_endpoint,
            "replay_payload_digest_function": proof_boundary_report.replay_payload_digest_function,
            "accepted_boundary_evidence_schema":
                proof_boundary_report.accepted_boundary_evidence_schema,
            "accepted_boundary_evidence_endpoint":
                proof_boundary_report.accepted_boundary_evidence_endpoint,
            "accepted_composition_evidence_schema":
                direct_report.accepted_composition_evidence_schema,
            "accepted_composition_evidence_endpoint":
                direct_report.accepted_composition_evidence_endpoint,
            "proof_boundary_target": proof_boundary_report.proof_boundary_target,
            "proof_boundary_conclusion":
                proof_boundary_report.proof_boundary_conclusion
        },
        "digests": {
            "direct_theorem_boundary_report_digest": hex_encode(&direct_report.digest),
            "direct_theorem_boundary_report_recomputed_digest": hex_encode(
                &recomputed_direct_digest
            ),
            "direct_qrom_action_boundary_report_digest": hex_encode(
                &qrom_action_report.digest
            ),
            "direct_qrom_action_boundary_report_recomputed_digest": hex_encode(
                &recomputed_qrom_action_digest
            ),
            "direct_theorem_report_qrom_action_digest": hex_encode(
                &direct_report.direct_qrom_action_boundary_report_digest
            ),
            "direct_lifecycle_boundary_report_digest": hex_encode(
                &lifecycle_report.digest
            ),
            "direct_lifecycle_boundary_report_recomputed_digest": hex_encode(
                &recomputed_lifecycle_digest
            ),
            "direct_theorem_report_lifecycle_digest": hex_encode(
                &direct_report.direct_lifecycle_boundary_report_digest
            ),
            "direct_admission_boundary_report_digest": hex_encode(
                &admission_report.digest
            ),
            "direct_admission_boundary_report_recomputed_digest": hex_encode(
                &recomputed_admission_digest
            ),
            "direct_theorem_report_admission_digest": hex_encode(
                &direct_report.direct_admission_boundary_report_digest
            ),
            "account_creation_proof_boundary_report_digest": hex_encode(
                &proof_boundary_report.digest
            ),
            "account_creation_proof_boundary_report_recomputed_digest": hex_encode(
                &recomputed_proof_boundary_digest
            ),
            "account_creation_sampler_timing_report_digest": hex_encode(
                &sampler_timing_report.digest
            ),
            "account_creation_sampler_timing_report_recomputed_digest": hex_encode(
                &recomputed_sampler_timing_digest
            ),
            "proof_boundary_report_sampler_timing_digest": hex_encode(
                &proof_boundary_report.sampler_timing_report_digest
            )
        },
        "direct_theorem_boundary": {
            "coverage": {
                "direct_qrom_action_boundary_report_digest": hex_encode(
                    &direct_report.direct_qrom_action_boundary_report_digest
                ),
                "direct_lifecycle_boundary_report_digest": hex_encode(
                    &direct_report.direct_lifecycle_boundary_report_digest
                ),
                "direct_admission_boundary_report_digest": hex_encode(
                    &direct_report.direct_admission_boundary_report_digest
                ),
                "accepted_tuple_field_count": direct_report.accepted_tuple_fields.len(),
                "acceptance_hypothesis_count": direct_report.acceptance_hypotheses.len(),
                "named_break_class_count": direct_report.named_break_classes.len(),
                "obligation_count": direct_report.obligation_count,
                "closure_dependency_count": direct_report.closure_dependency_count,
                "closure_dependency_evidence_count":
                    direct_report.closure_dependency_evidence_count,
                "closure_dependency_open_obligation_evidence_count":
                    direct_report.closure_dependency_open_obligation_evidence_count,
                "closure_dependency_open_support_evidence_count":
                    direct_report.closure_dependency_open_support_evidence_count,
                "obligation_audit_vector_case_count":
                    direct_report.obligation_audit_vector_case_count,
                "every_hypothesis_has_obligation":
                    direct_report.every_hypothesis_has_obligation,
                "every_obligation_has_named_break":
                    direct_report.every_obligation_has_named_break,
                "every_open_obligation_blocks_deployment":
                    direct_report.every_open_obligation_blocks_deployment,
                "every_closure_dependency_has_open_obligation":
                    direct_report.every_closure_dependency_has_open_obligation,
                "every_closure_dependency_has_open_support_surface":
                    direct_report.every_closure_dependency_has_open_support_surface,
                "every_open_closure_dependency_blocks_deployment":
                    direct_report.every_open_closure_dependency_blocks_deployment,
                "every_closure_dependency_has_evidence":
                    direct_report.every_closure_dependency_has_evidence,
                "theorem_statement_pinned": direct_report.theorem_statement_pinned,
                "accepted_tuple_pinned": direct_report.accepted_tuple_pinned,
                "accepted_tuple_evidence_present":
                    direct_report.accepted_tuple_evidence_present,
                "accepted_composition_evidence_present":
                    direct_report.accepted_composition_evidence_present,
                "acceptance_hypotheses_pinned":
                    direct_report.acceptance_hypotheses_pinned,
                "named_break_classes_pinned": direct_report.named_break_classes_pinned,
                "composition_theorem_reduction_lean_theorem":
                    direct_report.composition_theorem_reduction_lean_theorem,
                "composition_theorem_reduction_imported":
                    direct_report.composition_theorem_reduction_imported,
                "composition_external_hypotheses_explicit":
                    direct_report.composition_external_hypotheses_explicit,
                "composition_capstone_under_explicit_named_assumptions_lean_theorem":
                    direct_report
                        .composition_capstone_under_explicit_named_assumptions_lean_theorem,
                "composition_capstone_under_explicit_named_assumptions_imported":
                    direct_report
                        .composition_capstone_under_explicit_named_assumptions_imported,
                "composition_theorem_instantiated":
                    direct_report.composition_theorem_instantiated,
                "account_creation_proof_boundary_verified":
                    direct_report.account_creation_proof_boundary_verified,
                "direct_leaf_canonical_live_lean_theorem":
                    direct_report.direct_leaf_canonical_live_lean_theorem,
                "direct_leaf_canonical_live_imported":
                    direct_report.direct_leaf_canonical_live_imported,
                "direct_leaf_canonical_live_lifecycle_and_kat_hypothesis_explicit":
                    direct_report
                        .direct_leaf_canonical_live_lifecycle_and_kat_hypothesis_explicit,
                "direct_leaf_canonical_live_closed_by_frozen_kat_and_lifecycle":
                    direct_report
                        .direct_leaf_canonical_live_closed_by_frozen_kat_and_lifecycle,
                "checked_core_frame_binding_imported":
                    direct_report.checked_core_frame_binding_imported,
                "slot_authorization_digest_imported":
                    direct_report.slot_authorization_digest_imported,
                "direct_evidence_leaf_binding_lean_theorem":
                    direct_report.direct_evidence_leaf_binding_lean_theorem,
                "direct_evidence_leaf_binding_imported":
                    direct_report.direct_evidence_leaf_binding_imported,
                "direct_evidence_leaf_binding_proof_boundary_hypothesis_explicit":
                    direct_report
                        .direct_evidence_leaf_binding_proof_boundary_hypothesis_explicit,
                "direct_evidence_leaf_binding_closed_by_frozen_kat":
                    direct_report.direct_evidence_leaf_binding_closed_by_frozen_kat,
                "direct_admission_namespace_membership_lean_theorem":
                    direct_report.direct_admission_namespace_membership_lean_theorem,
                "direct_admission_namespace_membership_imported":
                    direct_report.direct_admission_namespace_membership_imported,
                "direct_admission_namespace_data_availability_hypothesis_explicit":
                    direct_report
                        .direct_admission_namespace_data_availability_hypothesis_explicit,
                "direct_admission_namespace_and_data_availability_lean_theorem":
                    direct_report
                        .direct_admission_namespace_and_data_availability_lean_theorem,
                "direct_admission_namespace_and_data_availability_imported":
                    direct_report
                        .direct_admission_namespace_and_data_availability_imported,
                "direct_admission_namespace_closed_by_sovereign_state":
                    direct_report.direct_admission_namespace_closed_by_sovereign_state,
                "direct_public_rejection_state_preserving_lean_theorem":
                    direct_report.direct_public_rejection_state_preserving_lean_theorem,
                "direct_public_rejection_state_preserving_imported":
                    direct_report.direct_public_rejection_state_preserving_imported,
                "direct_public_rejection_classifier_total":
                    direct_report.direct_public_rejection_classifier_total,
                "direct_public_rejection_class_kat_test":
                    direct_report.direct_public_rejection_class_kat_test,
                "direct_public_rejection_all_classes_kat_covered":
                    direct_report.direct_public_rejection_all_classes_kat_covered,
                "direct_public_rejection_panic_fail_closed_kat_covered":
                    direct_report.direct_public_rejection_panic_fail_closed_kat_covered,
                "direct_public_rejection_all_public_objects_kat_covered":
                    direct_report.direct_public_rejection_all_public_objects_kat_covered,
                "direct_public_rejection_target_client_mapping_complete":
                    direct_report.direct_public_rejection_target_client_mapping_complete,
                "direct_public_rejection_consensus_kat_hypothesis_explicit":
                    direct_report.direct_public_rejection_consensus_kat_hypothesis_explicit,
                "lifecycle_transition_profile_verified":
                    direct_report.lifecycle_transition_profile_verified,
                "qrom_action_budget_bridge_lean_theorem":
                    direct_report.qrom_action_budget_bridge_lean_theorem,
                "qrom_action_budget_bridge_imported":
                    direct_report.qrom_action_budget_bridge_imported,
                "qrom_action_theorem_linkage_reduction_lean_theorem":
                    direct_report.qrom_action_theorem_linkage_reduction_lean_theorem,
                "qrom_action_theorem_linkage_reduction_imported":
                    direct_report.qrom_action_theorem_linkage_reduction_imported,
                "qrom_action_review_hypothesis_explicit":
                    direct_report.qrom_action_review_hypothesis_explicit,
                "qrom_and_action_bounds_linked_to_theorem":
                    direct_report.qrom_and_action_bounds_linked_to_theorem,
                "deployment_ready": direct_report.deployment_ready,
                "blocker": direct_report.blocker
            },
            "theorem_target": direct_report.theorem_target,
            "theorem_conclusion": direct_report.theorem_conclusion,
            "accepted_tuple_fields": direct_report.accepted_tuple_fields,
            "accepted_tuple_evidence_schema":
                direct_report.accepted_tuple_evidence_schema,
            "accepted_tuple_evidence_endpoint":
                direct_report.accepted_tuple_evidence_endpoint,
            "accepted_composition_evidence_schema":
                direct_report.accepted_composition_evidence_schema,
            "accepted_composition_evidence_endpoint":
                direct_report.accepted_composition_evidence_endpoint,
            "acceptance_hypotheses": direct_report.acceptance_hypotheses,
            "named_break_classes": direct_report.named_break_classes,
            "obligation_audit_vector_case_ids":
                direct_report.obligation_audit_vector_case_ids,
            "obligation_rows": direct_report.obligation_rows,
            "closure_dependency_rows": direct_report.closure_dependency_rows,
            "closure_dependency_evidence_rows":
                direct_report.closure_dependency_evidence_rows,
            "direct_obligation_vector_corpus_evidence":
                direct_obligation_vector_corpus_evidence,
            "direct_accepted_spend_tuple_evidence_artifact":
                direct_accepted_spend_tuple_evidence_artifact,
            "direct_accepted_composition_evidence_artifact":
                direct_accepted_composition_evidence_artifact
        },
        "direct_qrom_action_boundary": {
            "coverage": {
                "checked_core_qrom_budget_pinned":
                    qrom_action_report.checked_core_qrom_budget_pinned,
                "action_map_budget_pinned": qrom_action_report.action_map_budget_pinned,
                "direct_mode_q_chal_active": qrom_action_report.direct_mode_q_chal_active,
                "direct_mode_q_batch_inactive":
                    qrom_action_report.direct_mode_q_batch_inactive,
                "optional_aggregation_terms_q_batch_only":
                    qrom_action_report.optional_aggregation_terms_q_batch_only,
                "selected_horizon_within_margin":
                    qrom_action_report.selected_horizon_within_margin,
                "negative_control_exceeds_margin":
                    qrom_action_report.negative_control_exceeds_margin,
                "direct_qrom_action_budget_bridge_lean_theorem":
                    qrom_action_report.direct_qrom_action_budget_bridge_lean_theorem,
                "direct_qrom_action_budget_bridge_bound":
                    qrom_action_report.direct_qrom_action_budget_bridge_bound,
                "theorem_imports_budget": qrom_action_report.theorem_imports_budget,
                "external_qrom_review_complete":
                    qrom_action_report.external_qrom_review_complete,
                "qrom_and_action_bounds_linked_to_theorem":
                    qrom_action_report.qrom_and_action_bounds_linked_to_theorem,
                "deployment_ready": qrom_action_report.deployment_ready,
                "blocker": qrom_action_report.blocker
            },
            "direct_obligation_id": qrom_action_report.direct_obligation_id,
            "acceptance_hypothesis": qrom_action_report.acceptance_hypothesis,
            "named_break": qrom_action_report.named_break,
            "query_budget_schema": qrom_action_report.query_budget_schema,
            "query_budget_mode": qrom_action_report.query_budget_mode,
            "checked_core_challenge_frame": qrom_action_report.checked_core_challenge_frame,
            "fq12_action_domain": qrom_action_report.fq12_action_domain,
            "checked_core_qrom_artifacts": qrom_action_report.checked_core_qrom_artifacts,
            "numeric_budget": {
                "target_security_bits": qrom_action_report.target_security_bits,
                "deployment_quantum_hash_query_bound_log2":
                    qrom_action_report.deployment_quantum_hash_query_bound_log2,
                "deployment_classical_hash_query_bound_log2":
                    qrom_action_report.deployment_classical_hash_query_bound_log2,
                "qrom_quadratic_loss_bits_at_deployment_bound":
                    qrom_action_report.qrom_quadratic_loss_bits_at_deployment_bound,
                "transcript_term_bits_after_qrom_loss":
                    qrom_action_report.transcript_term_bits_after_qrom_loss,
                "algebraic_challenge_bytes": qrom_action_report.algebraic_challenge_bytes,
                "challenge_hardening_bytes": qrom_action_report.challenge_hardening_bytes,
                "algebraic_challenge_last_byte_mask":
                    qrom_action_report.algebraic_challenge_last_byte_mask,
                "algebraic_challenge_bits_after_mask":
                    qrom_action_report.algebraic_challenge_bits_after_mask,
                "total_challenge_bits_after_mask":
                    qrom_action_report.total_challenge_bits_after_mask,
                "action_collision_floor_bits":
                    qrom_action_report.action_collision_floor_bits,
                "action_collision_quantum_bits_at_union_bound":
                    qrom_action_report.action_collision_quantum_bits_at_union_bound,
                "action_collision_classical_bits_at_union_bound":
                    qrom_action_report.action_collision_classical_bits_at_union_bound,
                "selected_multi_target_horizon_log2":
                    qrom_action_report.selected_multi_target_horizon_log2,
                "negative_control_horizon_log2":
                    qrom_action_report.negative_control_horizon_log2,
                "minimum_deployment_margin_bits":
                    qrom_action_report.minimum_deployment_margin_bits
            },
            "query_symbols": {
                "active": query_budget.active_query_symbols,
                "inactive": query_budget.inactive_query_symbols
            },
            "advantage_terms": {
                "direct": query_budget.direct_advantage_terms,
                "optional_aggregation": query_budget.optional_aggregation_terms
            }
        },
        "direct_lifecycle_boundary": {
            "coverage": {
                "local_successor_state_model_pinned":
                    lifecycle_report.local_successor_state_model_pinned,
                "transition_vector_corpus_pinned":
                    lifecycle_report.transition_vector_corpus_pinned,
                "target_chain_lifecycle_profile_review_pinned":
                    lifecycle_report.target_chain_lifecycle_profile_review_pinned,
                "target_chain_lifecycle_profile_verification_pinned":
                    lifecycle_report.target_chain_lifecycle_profile_verification_pinned,
                "consume_once_insert_after_public_checks":
                    lifecycle_report.consume_once_insert_after_public_checks,
                "replay_rejected_by_consume_set":
                    lifecycle_report.replay_rejected_by_consume_set,
                "spent_slot_ledger_rejects_stale_wallet_replay":
                    lifecycle_report.spent_slot_ledger_rejects_stale_wallet_replay,
                "chain_state_persists_lifecycle_and_spent_record":
                    lifecycle_report.chain_state_persists_lifecycle_and_spent_record,
                "reorg_burn_support_present": lifecycle_report.reorg_burn_support_present,
                "restore_anchor_support_present": lifecycle_report.restore_anchor_support_present,
                "recovery_burn_support_present":
                    lifecycle_report.recovery_burn_support_present,
                "snapshot_recovery_support_present":
                    lifecycle_report.snapshot_recovery_support_present,
                "file_store_recovery_support_present":
                    lifecycle_report.file_store_recovery_support_present,
                "target_chain_finality_profile_instantiated":
                    lifecycle_report.target_chain_finality_profile_instantiated,
                "lifecycle_transition_profile_verified":
                    lifecycle_report.lifecycle_transition_profile_verified,
                "deployment_ready": lifecycle_report.deployment_ready,
                "blocker": lifecycle_report.blocker
            },
            "direct_obligation_id": lifecycle_report.direct_obligation_id,
            "acceptance_hypothesis": lifecycle_report.acceptance_hypothesis,
            "named_break": lifecycle_report.named_break,
            "consume_once_validator": lifecycle_report.consume_once_validator,
            "state_namespace_validator": lifecycle_report.state_namespace_validator,
            "direct_spend_endpoint": lifecycle_report.direct_spend_endpoint,
            "lifecycle_manager_endpoint": lifecycle_report.lifecycle_manager_endpoint,
            "spent_slot_ledger_type": lifecycle_report.spent_slot_ledger_type,
            "target_chain_adapter_type": lifecycle_report.target_chain_adapter_type,
            "chain_file_store_type": lifecycle_report.chain_file_store_type,
            "executable_tests": lifecycle_report.executable_tests,
            "transition_vector_corpus_evidence": lifecycle_transition_vector_corpus_evidence,
            "target_chain_lifecycle_profile_review_evidence":
                lifecycle_profile_review_evidence,
            "target_chain_lifecycle_profile_verification_evidence":
                lifecycle_profile_verification_evidence
        },
        "direct_admission_boundary": {
            "coverage": {
                "exact_namespace_tuple_key_pinned":
                    admission_report.exact_namespace_tuple_key_pinned,
                "canonical_leaf_bytes_stored_directly":
                    admission_report.canonical_leaf_bytes_stored_directly,
                "leaf_and_evidence_digests_rechecked":
                    admission_report.leaf_and_evidence_digests_rechecked,
                "spend_reads_canonical_consensus_record":
                    admission_report.spend_reads_canonical_consensus_record,
                "missing_record_rejected": admission_report.missing_record_rejected,
                "corrupt_record_fails_closed": admission_report.corrupt_record_fails_closed,
                "duplicate_admission_rejected": admission_report.duplicate_admission_rejected,
                "d2_finalized_head_reorg_semantics":
                    admission_report.d2_finalized_head_reorg_semantics,
                "verified_creation_nullifier_anchor_bound":
                    admission_report.verified_creation_nullifier_anchor_bound,
                "sovereign_nullifier_registry_consensus_wired":
                    admission_report.sovereign_nullifier_registry_consensus_wired,
                "sovereign_nullifier_block_atomic":
                    admission_report.sovereign_nullifier_block_atomic,
                "sovereign_nullifier_d2_reorg_index_rebuild_verified":
                    admission_report.sovereign_nullifier_d2_reorg_index_rebuild_verified,
                "pad_seed_reorg_safe_uniqueness_selected_state_reduction_imported":
                    admission_report
                        .pad_seed_reorg_safe_uniqueness_selected_state_reduction_imported,
                "snapshot_restores_nullifier_registry":
                    admission_report.snapshot_restores_nullifier_registry,
                "snapshot_restore_anchor_bound":
                    admission_report.snapshot_restore_anchor_bound,
                "target_chain_data_availability_instantiated":
                    admission_report.target_chain_data_availability_instantiated,
                "admission_closure_lean_imported":
                    admission_report.admission_closure_lean_imported,
                "external_storage_review_complete":
                    admission_report.external_storage_review_complete,
                "deployment_ready": admission_report.deployment_ready,
                "blocker": admission_report.blocker
            },
            "profile_id": admission_report.profile_id,
            "architecture_authority": admission_report.architecture_authority,
            "lifecycle_authority": admission_report.lifecycle_authority,
            "storage_class": admission_report.storage_class,
            "namespace_key_fields": admission_report.namespace_key_fields,
            "record_value_fields": admission_report.record_value_fields,
            "admission_state_type": admission_report.admission_state_type,
            "admission_record_type": admission_report.admission_record_type,
            "admission_endpoint": admission_report.admission_endpoint,
            "verified_public_anchors_type": admission_report.verified_public_anchors_type,
            "verified_nullifier_decode_endpoint":
                admission_report.verified_nullifier_decode_endpoint,
            "verified_admission_candidate_type":
                admission_report.verified_admission_candidate_type,
            "verified_admission_candidate_endpoint":
                admission_report.verified_admission_candidate_endpoint,
            "nullifier_registry_key_type": admission_report.nullifier_registry_key_type,
            "nullifier_encoding": admission_report.nullifier_encoding,
            "nullifier_block_admission_endpoint":
                admission_report.nullifier_block_admission_endpoint,
            "restore_anchor_type": admission_report.restore_anchor_type,
            "restore_anchor_endpoint": admission_report.restore_anchor_endpoint,
            "restore_endpoint": admission_report.restore_endpoint,
            "available_record_endpoint": admission_report.available_record_endpoint,
            "spend_endpoint": admission_report.spend_endpoint,
            "executable_tests": admission_report.executable_tests,
            "admission_closure_lean_theorem": admission_report.admission_closure_lean_theorem,
            "pad_seed_reorg_safe_uniqueness_lean_theorem":
                admission_report.pad_seed_reorg_safe_uniqueness_lean_theorem
        },
        "account_creation_proof_boundary": {
            "coverage": {
                "sampler_timing_report_digest": hex_encode(
                    &proof_boundary_report.sampler_timing_report_digest
                ),
                "no_export_boundary_report_digest": hex_encode(
                    &proof_boundary_report.no_export_boundary_report_digest
                ),
                "account_stark_residual_matrix_report_digest": hex_encode(
                    &proof_boundary_report.account_stark_residual_matrix_report_digest
                ),
                "frame_kat_boundary_report_digest": hex_encode(
                    &proof_boundary_report.frame_kat_boundary_report_digest
                ),
                "accepting_requirement_count":
                    proof_boundary_report.accepting_requirements.len(),
                "leaf_evidence_binding_field_count":
                    proof_boundary_report.leaf_evidence_binding_fields.len(),
                "certificate_evidence_binding_field_count":
                    proof_boundary_report.certificate_evidence_binding_fields.len(),
                "binding_leaf_context_field_count":
                    proof_boundary_report.binding_leaf_context_fields.len(),
                "payload_binding_field_count":
                    proof_boundary_report.payload_binding_fields.len(),
                "deployed_anchor_field_count":
                    proof_boundary_report.deployed_anchor_fields.len(),
                "no_export_private_material_field_count":
                    proof_boundary_report.no_export_private_material_fields.len(),
                "proof_obligation_named_break_class_count":
                    proof_boundary_report.proof_obligation_named_break_classes.len(),
                "requirement_coverage_count":
                    proof_boundary_report.requirement_coverage_count,
                "proof_obligation_count":
                    proof_boundary_report.proof_obligation_count,
                "closure_dependency_count":
                    proof_boundary_report.closure_dependency_count,
                "closure_dependency_evidence_count":
                    proof_boundary_report.closure_dependency_evidence_count,
                "closure_dependency_open_obligation_evidence_count":
                    proof_boundary_report
                        .closure_dependency_open_obligation_evidence_count,
                "closure_dependency_open_support_evidence_count":
                    proof_boundary_report.closure_dependency_open_support_evidence_count,
                "proof_obligation_audit_vector_case_count":
                    proof_boundary_report.proof_obligation_audit_vector_case_count,
                "current_vector_covered_requirement_count":
                    proof_boundary_report.current_vector_covered_requirement_count,
                "current_vector_evidence_kinds": proof_boundary_report
                    .requirement_coverage_rows
                    .iter()
                    .map(|row| row.vector_evidence_kind)
                    .collect::<Vec<_>>(),
                "every_accepting_requirement_has_coverage_row":
                    proof_boundary_report.every_accepting_requirement_has_coverage_row,
                "every_vector_covered_requirement_has_public_rejection_class":
                    proof_boundary_report
                        .every_vector_covered_requirement_has_public_rejection_class,
                "every_current_vector_case_is_known_kat_row":
                    proof_boundary_report.every_current_vector_case_is_known_kat_row,
                "every_requirement_public_rejection_class_is_known":
                    proof_boundary_report.every_requirement_public_rejection_class_is_known,
                "every_proof_obligation_has_named_break":
                    proof_boundary_report.every_proof_obligation_has_named_break,
                "every_open_proof_obligation_blocks_deployment":
                    proof_boundary_report.every_open_proof_obligation_blocks_deployment,
                "every_closure_dependency_has_open_obligation":
                    proof_boundary_report.every_closure_dependency_has_open_obligation,
                "every_closure_dependency_has_open_support_surface":
                    proof_boundary_report.every_closure_dependency_has_open_support_surface,
                "every_open_closure_dependency_blocks_deployment":
                    proof_boundary_report.every_open_closure_dependency_blocks_deployment,
                "every_closure_dependency_has_evidence":
                    proof_boundary_report.every_closure_dependency_has_evidence,
                "boundary_statement_pinned":
                    proof_boundary_report.boundary_statement_pinned,
                "selected_payload_pinned": proof_boundary_report.selected_payload_pinned,
                "accepted_boundary_evidence_present":
                    proof_boundary_report.accepted_boundary_evidence_present,
                "accepted_boundary_binding_evidence_present":
                    proof_boundary_report.accepted_boundary_binding_evidence_present,
                "replay_payload_digest_bound":
                    proof_boundary_report.replay_payload_digest_bound,
                "replay_payload_digest_binding_evidence_present":
                    proof_boundary_report.replay_payload_digest_binding_evidence_present,
                "active_payload_mutation_coverage_present":
                    proof_boundary_report.active_payload_mutation_coverage_present,
                "deployed_anchor_rebind_tests_present":
                    proof_boundary_report.deployed_anchor_rebind_tests_present,
                "deployed_anchor_binding_evidence_present":
                    proof_boundary_report.deployed_anchor_binding_evidence_present,
                "requirement_coverage_matrix_pinned":
                    proof_boundary_report.requirement_coverage_matrix_pinned,
                "proof_obligation_rows_pinned":
                    proof_boundary_report.proof_obligation_rows_pinned,
                "production_gate_fail_closed":
                    proof_boundary_report.production_gate_fail_closed,
                "accepted_boundary_production_gate_review_reduction_lean_theorem":
                    proof_boundary_report
                        .accepted_boundary_production_gate_review_reduction_lean_theorem,
                "accepted_boundary_production_gate_review_reduction_bound":
                    proof_boundary_report
                        .accepted_boundary_production_gate_review_reduction_bound,
                "accepted_boundary_proof_family_closure_reductions_lean_theorem":
                    proof_boundary_report
                        .accepted_boundary_proof_family_closure_reductions_lean_theorem,
                "accepted_boundary_proof_family_closure_reductions_bound":
                    proof_boundary_report
                        .accepted_boundary_proof_family_closure_reductions_bound,
                "accepted_boundary_proof_family_closure_witness_reduction_lean_theorem":
                    proof_boundary_report
                        .accepted_boundary_proof_family_closure_witness_reduction_lean_theorem,
                "accepted_boundary_proof_family_closure_witness_reduction_bound":
                    proof_boundary_report
                        .accepted_boundary_proof_family_closure_witness_reduction_bound,
                "simulator_theorem_instantiated":
                    proof_boundary_report.simulator_theorem_instantiated,
                "fri_aok_extractor_theorem_instantiated":
                    proof_boundary_report.fri_aok_extractor_theorem_instantiated,
                "fri_qrom_residual_priced":
                    proof_boundary_report.fri_qrom_residual_priced,
                "frozen_kat_corpus_complete":
                    proof_boundary_report.frozen_kat_corpus_complete,
                "external_review_complete": proof_boundary_report.external_review_complete,
                "production_gate_open": proof_boundary_report.production_gate_open,
                "deployment_ready": proof_boundary_report.deployment_ready,
                "blocker": proof_boundary_report.blocker
            },
            "proof_boundary_target": proof_boundary_report.proof_boundary_target,
            "proof_boundary_conclusion":
                proof_boundary_report.proof_boundary_conclusion,
            "accepted_boundary_evidence_schema":
                proof_boundary_report.accepted_boundary_evidence_schema,
            "accepted_boundary_evidence_endpoint":
                proof_boundary_report.accepted_boundary_evidence_endpoint,
            "accepting_requirements": proof_boundary_report.accepting_requirements,
            "requirement_coverage_rows": proof_boundary_report.requirement_coverage_rows,
            "leaf_evidence_binding_fields":
                proof_boundary_report.leaf_evidence_binding_fields,
            "certificate_evidence_binding_fields":
                proof_boundary_report.certificate_evidence_binding_fields,
            "binding_leaf_context_fields":
                proof_boundary_report.binding_leaf_context_fields,
            "payload_binding_fields": proof_boundary_report.payload_binding_fields,
            "deployed_anchor_fields": proof_boundary_report.deployed_anchor_fields,
            "no_export_private_material_fields":
                proof_boundary_report.no_export_private_material_fields,
            "proof_obligation_named_break_classes":
                proof_boundary_report.proof_obligation_named_break_classes,
            "proof_obligation_audit_vector_case_ids":
                proof_boundary_report.proof_obligation_audit_vector_case_ids,
            "proof_obligation_rows": proof_boundary_report.proof_obligation_rows,
            "closure_dependency_rows": proof_boundary_report.closure_dependency_rows,
            "closure_dependency_evidence_rows":
                proof_boundary_report.closure_dependency_evidence_rows,
            "proof_obligation_vector_corpus_evidence":
                proof_obligation_vector_corpus_evidence,
            "accepted_proof_boundary_evidence_artifact":
                accepted_proof_boundary_evidence_artifact,
            "requirement_vector_corpus_evidence": requirement_vector_corpus_evidence
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
