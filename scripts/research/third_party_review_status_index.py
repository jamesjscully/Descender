#!/usr/bin/env python3
"""Generate the reviewer-facing status index for the q12289 packet."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
COST_MODEL_DIR = (
    ROOT / "research/parameter_search/committed_sigma_account_carried_20260521"
)
DEFAULT_JSON = READINESS_DIR / "third_party_review_status_index_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "third_party_review_status_index_n32_q12289.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def artifact(name: str) -> Path:
    return READINESS_DIR / name


def build_payload() -> dict[str, Any]:
    replay = load_json(artifact("review_gate_replay_report_n32_q12289.json"))
    production_gap = load_json(
        artifact("production_reference_gap_report_n32_q12289.json")
    )
    equation_gap = load_json(artifact("reviewed_equation_spec_gap_n32_q12289.json"))
    action_options = load_json(
        artifact("reviewed_challenge_action_options_n32_q12289.json")
    )
    hidden_source_target = load_json(
        artifact("hidden_source_proof_semantics_target_n32_q12289.json")
    )
    hidden_source_relation = load_json(
        artifact("hidden_source_accepting_relation_spec_n32_q12289.json")
    )
    hidden_source_carrier = load_json(
        artifact("hidden_source_carrier_construction_decision_n32_q12289.json")
    )
    state_assisted_carrier = load_json(
        artifact("hidden_source_state_assisted_carrier_relation_n32_q12289.json")
    )
    state_assisted_extractor = load_json(
        artifact("hidden_source_state_assisted_extractor_obligations_n32_q12289.json")
    )
    state_assisted_negative_controls = load_json(
        artifact("hidden_source_state_assisted_negative_controls_n32_q12289.json")
    )
    state_assisted_opening_frame = load_json(
        artifact("hidden_source_state_assisted_opening_frame_n32_q12289.json")
    )
    state_assisted_opening_frame_test_vectors = load_json(
        artifact(
            "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.json"
        )
    )
    state_assisted_carrier_verifier_contract = load_json(
        artifact(
            "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.json"
        )
    )
    state_assisted_account_state_schema = load_json(
        artifact(
            "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json"
        )
    )
    state_assisted_hidden_relation_verifier = load_json(
        artifact(
            "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.json"
        )
    )
    state_assisted_hidden_relation_negative_controls = load_json(
        artifact(
            "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.json"
        )
    )
    state_assisted_hidden_relation_extractor_theorem = load_json(
        artifact(
            "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json"
        )
    )
    state_assisted_masked_projection_global_soundness = load_json(
        artifact(
            "hidden_source_state_assisted_masked_projection_global_soundness_n32_q12289.json"
        )
    )
    pre_signature_projection_mask_prf_leakage_simulator = load_json(
        artifact("pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.json")
    )
    pre_signature_projection_mask_prf_ro_reduction = load_json(
        artifact("pre_signature_projection_mask_prf_ro_reduction_n32_q12289.json")
    )
    pre_signature_projection_lifecycle_same_game_lift = load_json(
        artifact("pre_signature_projection_lifecycle_same_game_lift_n32_q12289.json")
    )
    pre_signature_projection_spend_time_leakage_budget = load_json(
        artifact("pre_signature_projection_spend_time_leakage_budget_n32_q12289.json")
    )
    pre_signature_projection_reject_equivalence = load_json(
        artifact("pre_signature_projection_reject_equivalence_n32_q12289.json")
    )
    pre_signature_projection_mask_prf_leakage_theorem_composition = load_json(
        artifact(
            "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
        )
    )
    pre_signature_projection_sha256_mask_prf_assumption_resolution = load_json(
        artifact(
            "pre_signature_projection_sha256_mask_prf_assumption_resolution_n32_q12289.json"
        )
    )
    pre_signature_projection_global_domain_registry = load_json(
        artifact("pre_signature_projection_global_domain_registry_n32_q12289.json")
    )
    pre_signature_projection_lifecycle_same_game_discharge = load_json(
        artifact("pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json")
    )
    pre_signature_projection_spend_time_budget_discharge = load_json(
        artifact("pre_signature_projection_spend_time_budget_discharge_n32_q12289.json")
    )
    pre_signature_projection_reject_equivalence_discharge = load_json(
        artifact("pre_signature_projection_reject_equivalence_discharge_n32_q12289.json")
    )
    pre_signature_projection_helper_api_export_boundary = load_json(
        artifact("pre_signature_projection_helper_api_export_boundary_n32_q12289.json")
    )
    pre_signature_projection_helper_api_export_boundary_closure = load_json(
        artifact("pre_signature_projection_helper_api_export_boundary_closure_n32_q12289.json")
    )
    pre_signature_projection_no_export_negative_tests = load_json(
        artifact("pre_signature_projection_no_export_negative_tests_n32_q12289.json")
    )
    pre_signature_projection_no_export_negative_tests_closure = load_json(
        artifact("pre_signature_projection_no_export_negative_tests_closure_n32_q12289.json")
    )
    pre_signature_projection_req_pad_reject_discharge = load_json(
        artifact("pre_signature_projection_req_pad_reject_discharge_n32_q12289.json")
    )
    pre_signature_projection_req_pad_reject_closure = load_json(
        artifact("pre_signature_projection_req_pad_reject_closure_n32_q12289.json")
    )
    pre_signature_projection_req_witness_reject_discharge = load_json(
        artifact("pre_signature_projection_req_witness_reject_discharge_n32_q12289.json")
    )
    pre_signature_projection_req_witness_reject_closure = load_json(
        artifact("pre_signature_projection_req_witness_reject_closure_n32_q12289.json")
    )
    pre_signature_projection_req_carrier_reject_discharge = load_json(
        artifact("pre_signature_projection_req_carrier_reject_discharge_n32_q12289.json")
    )
    pre_signature_projection_req_carrier_reject_closure = load_json(
        artifact("pre_signature_projection_req_carrier_reject_closure_n32_q12289.json")
    )
    state_assisted_compact_proof_instantiation = load_json(
        artifact(
            "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.json"
        )
    )
    state_assisted_ro_pok_assumption_api = load_json(
        artifact("hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json")
    )
    state_assisted_ro_pok_byte_negative_tests = load_json(
        artifact(
            "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"
        )
    )
    algebraic_carrier_fallback_byte_model = load_json(
        artifact("hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.json")
    )
    fallback_carrier_projection_relation = load_json(
        artifact("hidden_source_fallback_carrier_projection_relation_n32_q12289.json")
    )
    fallback_projection_map_search = load_json(
        artifact("hidden_source_fallback_projection_map_search_n32_q12289.json")
    )
    fallback_projection_map_enumerator = load_json(
        artifact("hidden_source_fallback_projection_map_enumerator_n32_q12289.json")
    )
    fallback_projection_map_certificate_attempt = load_json(
        artifact(
            "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.json"
        )
    )
    fallback_projection_map_exact_certificate_spec = load_json(
        artifact(
            "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"
        )
    )
    fallback_projection_lane_map_consistency = load_json(
        artifact("hidden_source_fallback_projection_lane_map_consistency_n32_q12289.json")
    )
    fallback_projection_matrix_builder = load_json(
        artifact("hidden_source_fallback_projection_matrix_builder_n32_q12289.json")
    )
    fallback_projection_checked_core_lift = load_json(
        artifact(
            "hidden_source_fallback_projection_checked_core_lift_n32_q12289.json"
        )
    )
    fallback_projection_semantic_bridge_spec = load_json(
        artifact(
            "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json"
        )
    )
    fallback_projection_bridge_exact_replay = load_json(
        artifact(
            "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.json"
        )
    )
    fallback_projection_exact_infeasibility_replay = load_json(
        artifact(
            "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json"
        )
    )
    fallback_projection_bridge_equation_derivation = load_json(
        artifact(
            "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json"
        )
    )
    fallback_projection_checked_core_coupling_options = load_json(
        artifact(
            "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.json"
        )
    )
    fallback_projection_direct_linear_lift_attempt = load_json(
        artifact(
            "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.json"
        )
    )
    fallback_projection_checked_core_break_reduction = load_json(
        artifact(
            "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.json"
        )
    )
    fallback_projection_public_bridge_predicate = load_json(
        artifact(
            "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json"
        )
    )
    fallback_projection_break_export_contract = load_json(
        artifact(
            "hidden_source_fallback_projection_break_export_contract_n32_q12289.json"
        )
    )
    fallback_projection_break_export_negative_tests = load_json(
        artifact(
            "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"
        )
    )
    fallback_projection_map_redesign_or_bridge_rows = load_json(
        artifact(
            "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.json"
        )
    )
    fallback_projection_verifier_exporter_contract = load_json(
        artifact(
            "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.json"
        )
    )
    fallback_projection_verifier_exporter_implementation_gap = load_json(
        artifact(
            "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.json"
        )
    )
    fallback_projection_verifier_exporter_api_stubs = load_json(
        artifact(
            "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.json"
        )
    )
    fallback_projection_semantic_classifier_invocation_contract = load_json(
        artifact(
            "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.json"
        )
    )
    fallback_projection_semantic_predicate_implementation_plan = load_json(
        artifact(
            "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"
        )
    )
    fallback_projection_response_link_kernel_replay_boundary = load_json(
        artifact(
            "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.json"
        )
    )
    fallback_projection_opening_projection_boundary = load_json(
        artifact(
            "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.json"
        )
    )
    fallback_projection_account_commitment_split_boundary = load_json(
        artifact(
            "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.json"
        )
    )
    fallback_projection_pad_prf_projection_boundary = load_json(
        artifact(
            "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.json"
        )
    )
    fallback_projection_byte_level_derivation_internalization = load_json(
        artifact(
            "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json"
        )
    )
    fallback_projection_expanded_opening_commitment_binding = load_json(
        artifact(
            "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.json"
        )
    )
    fallback_projection_account_state_opening_commitment_wiring = load_json(
        artifact(
            "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.json"
        )
    )
    fallback_projection_exact_bridge_certificate_and_leakage_proof = load_json(
        artifact(
            "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.json"
        )
    )
    fallback_projection_verifier_exporter_bridge_semantics_closure = load_json(
        artifact(
            "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.json"
        )
    )
    fallback_projection_active_subspace_exact_replay = load_json(
        artifact(
            "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.json"
        )
    )
    ro_pok_vs_fallback_decision_gate = load_json(
        artifact("hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.json")
    )
    ro_pok_external_review_packet = load_json(
        artifact("hidden_source_ro_pok_external_review_packet_n32_q12289.json")
    )
    ro_pok_leakage_global_composition = load_json(
        artifact(
            "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.json"
        )
    )
    ro_pok_leakage_simulator = load_json(
        artifact("hidden_source_ro_pok_leakage_simulator_n32_q12289.json")
    )
    assumption_surface = load_json(
        artifact("keygen_security_assumption_surface_n32_q12289.json")
    )
    attack_matrix = load_json(
        artifact("assumption_attack_review_matrix_n32_q12289.json")
    )
    conditional_budget = load_json(
        artifact("conditional_soundness_budget_review_n32_q12289.json")
    )
    external_storage_review = load_json(
        artifact("external_storage_review_packet_n32_q12289.json")
    )
    red_team = load_json(artifact("red_team_cryptoanalysis_n32_q12289.json"))
    assumption_closure = load_json(
        artifact("assumption_red_team_closure_n32_q12289.json")
    )
    cost_model = load_json(
        COST_MODEL_DIR / "committed_sigma_account_carried_cost_model.json"
    )
    quotient_link_gate = load_json(artifact("quotient_level_link_promotion_gate.json"))
    quotient_link_firebreak = load_json(
        artifact("quotient_level_link_firebreak_audit.json")
    )
    terminology = load_json(
        artifact("selected_deployment_terminology_verification_n32_q12289.json")
    )
    todos_consistency = load_json(
        artifact("compact_signature_todos_consistency_n32_q12289.json")
    )

    preflight = production_gap["deployment_preflight"]
    certificate_export = preflight.get("certificate_export", {})
    certificate_digests = certificate_export.get("digests", {})
    preflight_certificate = {
        "schema": certificate_export.get("schema"),
        "export_status": certificate_export.get("export_status"),
        "binary_source": certificate_export.get("binary_source"),
        "deployment_ready": certificate_export.get("deployment_ready"),
        "blocker_count": certificate_export.get("blocker_count"),
        "primary_blocker": certificate_export.get("primary_blocker"),
        "certificate_digest": certificate_digests.get("certificate"),
        "preflight_report_digest": certificate_digests.get("preflight_report"),
        "blocker_report_digest": certificate_digests.get("blocker_report"),
        "verdict_digest": certificate_digests.get("verdict"),
        "ready_assertion_digest": certificate_digests.get("ready_assertion"),
        "cross_checks": certificate_export.get("cross_checks", {}),
        "verification_artifact_json": rel(
            artifact("production_preflight_certificate_verification_n32_q12289.json")
        ),
        "verification_artifact_md": rel(
            artifact("production_preflight_certificate_verification_n32_q12289.md")
        ),
    }
    review_gates = cost_model.get("review_gates", [])
    review_gate_rows = [
        {
            "gate": gate.get("gate"),
            "status": gate.get("status"),
            "artifact": gate.get("artifact"),
            "summary": gate.get("summary"),
            "key_metrics": gate.get("key_metrics", {}),
        }
        for gate in review_gates
    ]
    gate_status_counts: dict[str, int] = {}
    for gate in review_gates:
        status = str(gate.get("status"))
        gate_status_counts[status] = gate_status_counts.get(status, 0) + 1

    external_review_items = [
        {
            "id": "response_link_certificate",
            "status": "internal_replay_passed_external_review_open",
            "evidence": rel(
                artifact(
                    "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
                )
            ),
            "next_step": "Independent replay plus third-party SVP/BKZ review.",
        },
        {
            "id": "security_assumption_surface",
            "status": "fail_closed_closure_ledger_generated",
            "evidence": rel(
                artifact("assumption_red_team_closure_n32_q12289.json")
            ),
            "next_step": "Promote or reject every closure row using independent replay, estimator, QROM, sampler, and implementation review.",
        },
        {
            "id": "qrom_and_sampler_budgets",
            "status": conditional_budget.get("status"),
            "evidence": rel(
                artifact("conditional_soundness_budget_review_n32_q12289.json")
            ),
            "next_step": "Confirm QROM query budget and sampler assumptions against deployment limits.",
        },
        {
            "id": "production_implementation",
            "status": "fail_closed_preflight",
            "evidence": rel(
                artifact("production_reference_gap_report_n32_q12289.json")
            ),
            "next_step": "Implement accepting projection-carrier proof semantics for the selected 308-byte/Fq12 envelope, promote that verifier through the deployment facade, then audit constant-time and storage behavior.",
        },
        {
            "id": "external_storage_review",
            "status": external_storage_review.get("status"),
            "evidence": rel(artifact("external_storage_review_packet_n32_q12289.json")),
            "next_step": "Map the reference lifecycle/spent-ledger/trusted-anchor stores onto the target chain database and have an external reviewer confirm atomicity, rollback, and anchor-placement semantics.",
        },
        {
            "id": "reviewed_equation_spec",
            "status": equation_gap.get("decision", {}).get("status"),
            "evidence": rel(artifact("reviewed_equation_spec_gap_n32_q12289.json")),
            "next_step": "Pin the reviewed 10-row challenge-action algebra/domain or update the byte target before implementing the final 37-byte masked-response equation.",
        },
        {
            "id": "reviewed_challenge_action_options",
            "status": "252b_fq12_recommended",
            "evidence": rel(
                artifact("reviewed_challenge_action_options_n32_q12289.json")
            ),
            "next_step": "Decide whether to promote the 252-byte/12-row fallback or pursue a new exact-250-byte action argument.",
        },
        {
            "id": "hidden_source_proof_semantics_target",
            "status": hidden_source_target.get("status"),
            "evidence": rel(
                artifact("hidden_source_proof_semantics_target_n32_q12289.json")
            ),
            "next_step": "Choose carrier semantics, prove the accepting hidden-source relation, and keep the deployment verifier fail-closed until all obligations are discharged.",
        },
        {
            "id": "hidden_source_accepting_relation_spec",
            "status": hidden_source_relation.get("status"),
            "evidence": rel(
                artifact("hidden_source_accepting_relation_spec_n32_q12289.json")
            ),
            "next_step": "Implement carrier semantics against the exact byte-level relation and add the listed negative controls.",
        },
        {
            "id": "hidden_source_carrier_construction_decision",
            "status": hidden_source_carrier.get("status"),
            "evidence": rel(
                artifact("hidden_source_carrier_construction_decision_n32_q12289.json")
            ),
            "next_step": "Formalize the state-assisted 32-byte carrier relation, or fall back to a larger algebraic carrier if extraction fails.",
        },
        {
            "id": "hidden_source_state_assisted_carrier_relation",
            "status": state_assisted_carrier.get("status"),
            "evidence": rel(
                artifact("hidden_source_state_assisted_carrier_relation_n32_q12289.json")
            ),
            "next_step": "Prove the extractor-or-break theorem and implement the hidden-source opening commitment/account-state hook.",
        },
        {
            "id": "hidden_source_state_assisted_extractor_obligations",
            "status": state_assisted_extractor.get("status"),
            "evidence": rel(
                artifact("hidden_source_state_assisted_extractor_obligations_n32_q12289.json")
            ),
            "next_step": "Discharge the extractor/named-break obligations or activate the larger-carrier fallback.",
        },
        {
            "id": "hidden_source_state_assisted_negative_controls",
            "status": state_assisted_negative_controls.get("status"),
            "evidence": rel(
                artifact("hidden_source_state_assisted_negative_controls_n32_q12289.json")
            ),
            "next_step": "Implement carrier-specific negative controls before enabling acceptance.",
        },
        {
            "id": "hidden_source_state_assisted_opening_frame",
            "status": state_assisted_opening_frame.get("status"),
            "evidence": rel(
                artifact("hidden_source_state_assisted_opening_frame_n32_q12289.json")
            ),
            "next_step": "Generate canonical test vectors and wire the frame into the executable carrier verifier.",
        },
        {
            "id": "hidden_source_state_assisted_opening_frame_test_vectors",
            "status": state_assisted_opening_frame_test_vectors.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.json"
                )
            ),
            "next_step": "Promote the deterministic vectors into Rust verifier tests and implement the carrier verifier contract.",
        },
        {
            "id": "hidden_source_state_assisted_carrier_verifier_contract",
            "status": state_assisted_carrier_verifier_contract.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.json"
                )
            ),
            "next_step": "Implement account-state schema support and the hidden-relation verifier behind this contract.",
        },
        {
            "id": "hidden_source_state_assisted_account_state_schema_contract",
            "status": state_assisted_account_state_schema.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json"
                )
            ),
            "next_step": "Add persistent storage and creation-time enforcement, then implement the hidden-relation verifier.",
        },
        {
            "id": "hidden_source_state_assisted_hidden_relation_verifier_contract",
            "status": state_assisted_hidden_relation_verifier.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.json"
                )
            ),
            "next_step": "Instantiate the compact proof verifier or theorem that turns this relation contract into accepting hidden-source semantics.",
        },
        {
            "id": "hidden_source_state_assisted_hidden_relation_negative_controls",
            "status": state_assisted_hidden_relation_negative_controls.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.json"
                )
            ),
            "next_step": "Promote these semantic controls into byte-level Rust tests once the compact hidden-relation verifier exists.",
        },
        {
            "id": "hidden_source_state_assisted_hidden_relation_extractor_theorem",
            "status": state_assisted_hidden_relation_extractor_theorem.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json"
                )
            ),
            "next_step": "Use the executable local certificate as input to the selected masked-projection global soundness composition theorem.",
        },
        {
            "id": "hidden_source_state_assisted_masked_projection_global_soundness",
            "status": state_assisted_masked_projection_global_soundness.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_state_assisted_masked_projection_global_soundness_n32_q12289.json"
                )
            ),
            "next_step": "Discharge the open global hops: checked-core/QROM composition, mask-PRF leakage simulation, lifecycle single-use game, and constant-time boundary.",
        },
        {
            "id": "pre_signature_projection_mask_prf_leakage_simulator",
            "status": pre_signature_projection_mask_prf_leakage_simulator.get("status"),
            "evidence": rel(
                artifact(
                    "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.json"
                )
            ),
            "next_step": "Prove the mask PRF/RO reduction, lift lifecycle uniqueness into the same game, and charge valid spend-opening projection disclosure explicitly.",
        },
        {
            "id": "pre_signature_projection_mask_prf_ro_reduction",
            "status": pre_signature_projection_mask_prf_ro_reduction.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_mask_prf_ro_reduction_n32_q12289.json")
            ),
            "next_step": "Choose and prove the ROM-vs-SHA-256 assumption treatment, then close the reduction contract dependencies in the parent leakage theorem.",
        },
        {
            "id": "pre_signature_projection_lifecycle_same_game_lift",
            "status": pre_signature_projection_lifecycle_same_game_lift.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_lifecycle_same_game_lift_n32_q12289.json")
            ),
            "next_step": "Implement the same-game lifecycle negative controls and enforce pad-seed digest uniqueness plus packed-witness immutability.",
        },
        {
            "id": "pre_signature_projection_spend_time_leakage_budget",
            "status": pre_signature_projection_spend_time_leakage_budget.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_spend_time_leakage_budget_n32_q12289.json")
            ),
            "next_step": "Audit public APIs and add negative tests proving no pre-spend or reject path exports selected projection residues.",
        },
        {
            "id": "pre_signature_projection_reject_equivalence",
            "status": pre_signature_projection_reject_equivalence.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_reject_equivalence_n32_q12289.json")
            ),
            "next_step": "Add accepting-verifier timing evidence, constant-shape carrier review, parser/account timing equalization, and selected-route diagnostics tests.",
        },
        {
            "id": "pre_signature_projection_mask_prf_leakage_theorem_composition",
            "status": pre_signature_projection_mask_prf_leakage_theorem_composition.get("status"),
            "evidence": rel(
                artifact(
                    "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
                )
            ),
            "next_step": "Discharge or explicitly accept the composed SHA-256/RO, lifecycle, spend-opening, and reject-equivalence terms in the final bound.",
        },
        {
            "id": "pre_signature_projection_sha256_mask_prf_assumption_resolution",
            "status": pre_signature_projection_sha256_mask_prf_assumption_resolution.get("status"),
            "evidence": rel(
                artifact(
                    "pre_signature_projection_sha256_mask_prf_assumption_resolution_n32_q12289.json"
                )
            ),
            "next_step": "Carry the explicit SHA-256 frame-PRF term, prove it, or change the construction.",
        },
        {
            "id": "pre_signature_projection_global_domain_registry",
            "status": pre_signature_projection_global_domain_registry.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_global_domain_registry_n32_q12289.json")
            ),
            "next_step": "Prove byte-level disjointness across the 21 registered selected-route frame families and cover ambiguity risks with negative tests.",
        },
        {
            "id": "pre_signature_projection_lifecycle_same_game_discharge",
            "status": pre_signature_projection_lifecycle_same_game_discharge.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json")
            ),
            "next_step": "Close postchallenge slot creation, full witness rewrite, projection overread, and deployment DB profile residuals.",
        },
        {
            "id": "pre_signature_projection_spend_time_budget_discharge",
            "status": pre_signature_projection_spend_time_budget_discharge.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_spend_time_budget_discharge_n32_q12289.json")
            ),
            "next_step": "Restrict helper/API export surfaces and add no-export negative tests for pre-spend and reject paths.",
        },
        {
            "id": "pre_signature_projection_reject_equivalence_discharge",
            "status": pre_signature_projection_reject_equivalence_discharge.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_reject_equivalence_discharge_n32_q12289.json")
            ),
            "next_step": "Build the final selected 308-byte/Fq12 projection-carrier timing and diagnostic harness.",
        },
        {
            "id": "pre_signature_projection_helper_api_export_boundary",
            "status": pre_signature_projection_helper_api_export_boundary.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_helper_api_export_boundary_n32_q12289.json")
            ),
            "next_step": "Restrict or gate projection materializer helpers and add wrapper tests for public/pre-spend callers.",
        },
        {
            "id": "pre_signature_projection_no_export_negative_tests",
            "status": pre_signature_projection_no_export_negative_tests.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_no_export_negative_tests_n32_q12289.json")
            ),
            "next_step": "Implement the eight selected-material no-export Rust negative tests named by the artifact.",
        },
        {
            "id": "pre_signature_projection_req_pad_reject_discharge",
            "status": pre_signature_projection_req_pad_reject_discharge.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_req_pad_reject_discharge_n32_q12289.json")
            ),
            "next_step": "Add selected-route pad-opening reject KATs, diagnostic checks, no-state mutation checks, and timing rows.",
        },
        {
            "id": "pre_signature_projection_req_witness_reject_discharge",
            "status": pre_signature_projection_req_witness_reject_discharge.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_req_witness_reject_discharge_n32_q12289.json")
            ),
            "next_step": "Add selected-route packed-witness opening reject KATs, diagnostic checks, and timing rows.",
        },
        {
            "id": "pre_signature_projection_req_carrier_reject_discharge",
            "status": pre_signature_projection_req_carrier_reject_discharge.get("status"),
            "evidence": rel(
                artifact("pre_signature_projection_req_carrier_reject_discharge_n32_q12289.json")
            ),
            "next_step": "Add selected-route projection-carrier reject KATs, diagnostic checks, state-preservation checks, and timing rows.",
        },
        {
            "id": "hidden_source_state_assisted_compact_proof_instantiation",
            "status": state_assisted_compact_proof_instantiation.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.json"
                )
            ),
            "next_step": "Decide whether the 32-byte proof-handle assumption is acceptable after executable controls, or activate the larger algebraic carrier fallback.",
        },
        {
            "id": "hidden_source_state_assisted_ro_pok_assumption_api",
            "status": state_assisted_ro_pok_assumption_api.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json"
                )
            ),
            "next_step": "Implement byte-level semantic negative controls for this API, then decide whether the assumption is reviewable or too strong.",
        },
        {
            "id": "hidden_source_state_assisted_ro_pok_byte_negative_tests",
            "status": state_assisted_ro_pok_byte_negative_tests.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"
                )
            ),
            "next_step": "Run these byte controls against any accepting verifier candidate while keeping the default RO-PoK verifier fail-closed.",
        },
        {
            "id": "hidden_source_algebraic_carrier_fallback_byte_model",
            "status": algebraic_carrier_fallback_byte_model.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.json"
                )
            ),
            "next_step": "Continue with the selected 56-byte projection carrier and run the exact projection certificate plus leakage theorem.",
        },
        {
            "id": "hidden_source_fallback_carrier_projection_relation",
            "status": fallback_carrier_projection_relation.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_carrier_projection_relation_n32_q12289.json"
                )
            ),
            "next_step": "For the selected 308-byte projection route, choose the twelve q12289 projection coordinates and prove projection soundness/leakage.",
        },
        {
            "id": "hidden_source_fallback_projection_map_search",
            "status": fallback_projection_map_search.get("status"),
            "evidence": rel(
                artifact("hidden_source_fallback_projection_map_search_n32_q12289.json")
            ),
            "next_step": "Build and run the deterministic projection-map exact certificate replay for the selected 308-byte route.",
        },
        {
            "id": "hidden_source_fallback_projection_map_enumerator",
            "status": fallback_projection_map_enumerator.get("status"),
            "evidence": rel(
                artifact("hidden_source_fallback_projection_map_enumerator_n32_q12289.json")
            ),
            "next_step": "Attempt exact bounded-kernel and leakage certificates for the selected top-ranked projection map.",
        },
        {
            "id": "hidden_source_fallback_projection_map_certificate_attempt",
            "status": fallback_projection_map_certificate_attempt.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.json"
                )
            ),
            "next_step": "Implement the exact projection-matrix replay named here; this is now on the selected 308-byte route.",
        },
        {
            "id": "hidden_source_fallback_projection_map_exact_certificate_spec",
            "status": fallback_projection_map_exact_certificate_spec.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"
                )
            ),
            "next_step": "Implement the generalized hidden-source projection exact replay named by this spec.",
        },
        {
            "id": "hidden_source_fallback_projection_lane_map_consistency",
            "status": fallback_projection_lane_map_consistency.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_lane_map_consistency_n32_q12289.json"
                )
            ),
            "next_step": "Keep Rust lane symbols and coefficients pinned to this selected map while account-witness derivation is internalized.",
        },
        {
            "id": "hidden_source_fallback_projection_matrix_builder",
            "status": fallback_projection_matrix_builder.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
                )
            ),
            "next_step": "Run exact infeasibility replay on the instantiated generalized system; open bridge slots remain fail-closed.",
        },
        {
            "id": "hidden_source_fallback_projection_checked_core_lift",
            "status": fallback_projection_checked_core_lift.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_checked_core_lift_n32_q12289.json"
                )
            ),
            "next_step": "Instantiate the bridge equations and semantic decoder constraints named here; direct-sum rank accounting remains fail-closed.",
        },
        {
            "id": "hidden_source_fallback_projection_semantic_bridge_spec",
            "status": fallback_projection_semantic_bridge_spec.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json"
                )
            ),
            "next_step": "Implement the exact bridge replay: carrier-to-projection is specified, but projection-to-checked-core and projection-to-hidden-source equations remain open.",
        },
        {
            "id": "hidden_source_fallback_projection_bridge_exact_replay",
            "status": fallback_projection_bridge_exact_replay.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.json"
                )
            ),
            "next_step": "Derive the projection-to-checked-core and projection-to-hidden-source equations; carrier-lane replay alone is underconstrained.",
        },
        {
            "id": "hidden_source_fallback_projection_exact_infeasibility_replay",
            "status": fallback_projection_exact_infeasibility_replay.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json"
                )
            ),
            "next_step": "Use the exported bounded survivor to drive bridge-row derivation or projection-map redesign; verifier remains fail-closed.",
        },
        {
            "id": "hidden_source_fallback_projection_bridge_equation_derivation",
            "status": fallback_projection_bridge_equation_derivation.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json"
                )
            ),
            "next_step": "Choose and prove a checked-core coupling route; public lane equations are derived but no checked-core coupling equation is derived yet.",
        },
        {
            "id": "hidden_source_fallback_projection_checked_core_coupling_options",
            "status": fallback_projection_checked_core_coupling_options.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.json"
                )
            ),
            "next_step": "Attempt the selected direct linear lift before falling back to a checked-core break reduction or map redesign.",
        },
        {
            "id": "hidden_source_fallback_projection_direct_linear_lift_attempt",
            "status": fallback_projection_direct_linear_lift_attempt.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.json"
                )
            ),
            "next_step": "Close the checked-core break reduction fallback before redesigning the projection map.",
        },
        {
            "id": "hidden_source_fallback_projection_checked_core_break_reduction",
            "status": fallback_projection_checked_core_break_reduction.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.json"
                )
            ),
            "next_step": "Instantiate the public bridge predicate and concrete break-export contract.",
        },
        {
            "id": "hidden_source_fallback_projection_public_bridge_predicate",
            "status": fallback_projection_public_bridge_predicate.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json"
                )
            ),
            "next_step": "Instantiate concrete break-export witnesses for checked-core and hidden-source bridge failures.",
        },
        {
            "id": "hidden_source_fallback_projection_break_export_contract",
            "status": fallback_projection_break_export_contract.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_break_export_contract_n32_q12289.json"
                )
            ),
            "next_step": "Implement negative tests that reject or export one of the specified bridge-break records.",
        },
        {
            "id": "hidden_source_fallback_projection_break_export_negative_tests",
            "status": fallback_projection_break_export_negative_tests.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"
                )
            ),
            "next_step": "Bind the negative-test matrix to the verifier/exporter contract.",
        },
        {
            "id": "hidden_source_fallback_projection_map_redesign_or_bridge_rows",
            "status": fallback_projection_map_redesign_or_bridge_rows.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.json"
                )
            ),
            "next_step": "Continue on the bridge/export implementation path; map redesign is deferred unless the bridge-export path fails.",
        },
        {
            "id": "hidden_source_fallback_projection_verifier_exporter_contract",
            "status": fallback_projection_verifier_exporter_contract.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.json"
                )
            ),
            "next_step": "Implement the verifier/exporter API and executable negative tests.",
        },
        {
            "id": "hidden_source_fallback_projection_verifier_exporter_implementation_gap",
            "status": fallback_projection_verifier_exporter_implementation_gap.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.json"
                )
            ),
            "next_step": "Add fail-closed verifier/exporter API stubs at the production-reference boundary.",
        },
        {
            "id": "hidden_source_fallback_projection_verifier_exporter_api_stubs",
            "status": fallback_projection_verifier_exporter_api_stubs.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.json"
                )
            ),
            "next_step": "Implement semantic predicates that decide when the checked-core and hidden-source classifiers are invoked.",
        },
        {
            "id": "hidden_source_fallback_projection_semantic_classifier_invocation_contract",
            "status": fallback_projection_semantic_classifier_invocation_contract.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.json"
                )
            ),
            "next_step": "Implement the five semantic predicate replays named by the invocation contract.",
        },
        {
            "id": "hidden_source_fallback_projection_semantic_predicate_implementation_plan",
            "status": fallback_projection_semantic_predicate_implementation_plan.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"
                )
            ),
            "next_step": "Implement the hidden-source opening projection boundary next.",
        },
        {
            "id": "hidden_source_fallback_projection_response_link_kernel_replay_boundary",
            "status": fallback_projection_response_link_kernel_replay_boundary.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.json"
                )
            ),
            "next_step": "Implement the hidden-source opening projection boundary next.",
        },
        {
            "id": "hidden_source_fallback_projection_opening_projection_boundary",
            "status": fallback_projection_opening_projection_boundary.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.json"
                )
            ),
            "next_step": "Implement the account commitment split-opening boundary next.",
        },
        {
            "id": "hidden_source_fallback_projection_account_commitment_split_boundary",
            "status": fallback_projection_account_commitment_split_boundary.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.json"
                )
            ),
            "next_step": "Implement the pad PRF projection derivation boundary next.",
        },
        {
            "id": "hidden_source_fallback_projection_pad_prf_projection_boundary",
            "status": fallback_projection_pad_prf_projection_boundary.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.json"
                )
            ),
            "next_step": "Internalize byte-level derivation of response differences and hidden-material projections.",
        },
        {
            "id": "hidden_source_fallback_projection_byte_level_derivation_internalization",
            "status": fallback_projection_byte_level_derivation_internalization.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json"
                )
            ),
            "next_step": "Wire the expanded-opening commitment into account state, derive response differences internally, and decode account split semantic values from opening frames.",
        },
        {
            "id": "hidden_source_fallback_projection_expanded_opening_commitment_binding",
            "status": fallback_projection_expanded_opening_commitment_binding.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.json"
                )
            ),
            "next_step": "Wire this commitment into account-state creation and decode the six selected source values from the full opening frame.",
        },
        {
            "id": "hidden_source_fallback_projection_account_state_opening_commitment_wiring",
            "status": fallback_projection_account_state_opening_commitment_wiring.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.json"
                )
            ),
            "next_step": "Plug the selected projection transactional-KV binding into a concrete node database engine, choose the trusted restore-anchor root of trust, and complete external storage review.",
        },
        {
            "id": "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure",
            "status": fallback_projection_verifier_exporter_bridge_semantics_closure.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.json"
                )
            ),
            "next_step": "Verifier/exporter bridge semantics are closed fail-closed; use active-subspace exact replay to exclude the old inactive-coordinate survivor before enabling acceptance.",
        },
        {
            "id": "hidden_source_fallback_projection_active_subspace_exact_replay",
            "status": fallback_projection_active_subspace_exact_replay.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.json"
                )
            ),
            "next_step": "Exact survivor is resolved for the verifier-visible active projection relation; keep the route fail-closed until production preflight and third-party review complete.",
        },
        {
            "id": "hidden_source_ro_pok_vs_fallback_decision_gate",
            "status": ro_pok_vs_fallback_decision_gate.get("status"),
            "evidence": rel(
                artifact("hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.json")
            ),
            "next_step": "Use this gate as the route-pivot record: the 308-byte projection certificate path is now selected.",
        },
        {
            "id": "hidden_source_ro_pok_external_review_packet",
            "status": ro_pok_external_review_packet.get("status"),
            "evidence": rel(
                artifact("hidden_source_ro_pok_external_review_packet_n32_q12289.json")
            ),
            "next_step": "Send the 32-byte RO-PoK assumption boundary to external review; if rejected, activate the 308-byte projection fallback certificate path.",
        },
        {
            "id": "hidden_source_ro_pok_leakage_global_composition_theorem",
            "status": ro_pok_leakage_global_composition.get("status"),
            "evidence": rel(
                artifact(
                    "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.json"
                )
            ),
            "next_step": "If external review accepts the 32-byte RO-PoK assumption, instantiate the leakage simulator and discharge the global composition edges named here.",
        },
        {
            "id": "hidden_source_ro_pok_leakage_simulator",
            "status": ro_pok_leakage_simulator.get("status"),
            "evidence": rel(
                artifact("hidden_source_ro_pok_leakage_simulator_n32_q12289.json")
            ),
            "next_step": "Implement the accepting-model controls and simulator oracle hook, or switch to the 308-byte projection fallback if the RO-PoK simulator is rejected.",
        },
        {
            "id": "red_team_followup",
            "status": "selected_target_not_broken_bad_variants_found",
            "evidence": rel(artifact("red_team_cryptoanalysis_n32_q12289.json")),
            "next_step": "Independently reproduce the coordinate sweep and run exact checks for any candidate bad variants before changing response-link coordinates.",
        },
        {
            "id": "quotient_level_link_sidecar",
            "status": quotient_link_gate["final_decision"]["classification"],
            "evidence": rel(artifact("quotient_level_link_promotion_gate.json")),
            "next_step": "No migration; keep the reviewed split account-link design unless a later promotion gate supersedes this closure.",
        },
    ]

    deployment_ready = (
        preflight.get("deployment_ready") is True
        and conditional_budget.get("deployment_ready") is True
        and assumption_closure.get("summary", {}).get("deployment_ready") is True
        and replay.get("status") == "pass"
        and replay.get("replay_exact") is True
    )
    return {
        "schema": "Descender.third_party_review_status_index.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "verdict": {
            "internal_packet_reproducible": replay.get("status") == "pass"
            and replay.get("replay_exact") is True,
            "deployment_ready": deployment_ready,
            "external_review_required": True,
            "headline": "Internal packet is reproducible; deployable cryptography is still fail-closed pending Path 2 account-creation prevalidation proof semantics, constant-time/storage review, and external review.",
        },
        "target": {
            "n": 32,
            "q": 12289,
            "transaction_bytes": 250,
            "account_link_bytes": 68,
            "online_plus_account_bytes": 318,
            "public_key_bytes": 172,
            "executable_signature_bytes": preflight.get("executable_signature_bytes"),
        },
        "reproducibility": {
            "recommended_command": "python3 scripts/research/finalize_third_party_review_packet.py",
            "replay_status": replay.get("status"),
            "replay_exact": replay.get("replay_exact"),
            "failed_steps": replay.get("failed_step_count"),
            "invalid_json_artifacts": replay.get("invalid_json_artifact_count"),
            "hashed_json_artifacts": replay.get("hashed_json_artifact_count"),
        },
        "review_gates": {
            "total": len(review_gates),
            "status_counts": gate_status_counts,
            "open_external_items": assumption_surface.get("summary", {}).get(
                "open_external_items"
            ),
            "assumptions_mapped": attack_matrix.get("summary", {}).get(
                "assumptions_mapped"
            ),
            "deployment_ready_rows": attack_matrix.get("summary", {}).get(
                "deployment_ready_rows"
            ),
            "rows": review_gate_rows,
        },
        "deployment_preflight": preflight,
        "preflight_certificate": preflight_certificate,
        "selected_deployment_terminology": {
            "artifact_json": rel(
                artifact(
                    "selected_deployment_terminology_verification_n32_q12289.json"
                )
            ),
            "artifact_md": rel(
                artifact(
                    "selected_deployment_terminology_verification_n32_q12289.md"
                )
            ),
            "status": terminology.get("status"),
            "files_checked": terminology.get("files_checked"),
            "failure_count": len(terminology.get("failures", [])),
            "missing_required_phrase_count": len(
                terminology.get("missing_required_phrases", [])
            ),
            "forbidden_patterns": terminology.get("forbidden_patterns", []),
        },
        "todo_consistency": {
            "artifact_json": rel(
                artifact("compact_signature_todos_consistency_n32_q12289.json")
            ),
            "artifact_md": rel(
                artifact("compact_signature_todos_consistency_n32_q12289.md")
            ),
            "status": todos_consistency.get("status"),
            "replay_step_count": todos_consistency.get("observed", {}).get(
                "replay_step_count"
            ),
            "hashed_json_artifact_count": todos_consistency.get("observed", {}).get(
                "hashed_json_artifact_count"
            ),
            "manifest_file_count": todos_consistency.get("observed", {}).get(
                "manifest_file_count"
            ),
            "forbidden_pattern_failure_count": len(
                todos_consistency.get("forbidden_pattern_failures", [])
            ),
            "missing_required_snippet_count": len(
                todos_consistency.get("missing_required_snippets", [])
            ),
        },
        "reviewed_equation_spec": {
            "artifact": rel(artifact("reviewed_equation_spec_gap_n32_q12289.json")),
            "status": equation_gap.get("decision", {}).get("status"),
            "deployment_ready": equation_gap.get("decision", {}).get(
                "deployment_ready"
            ),
            "findings": equation_gap.get("findings", []),
        },
        "reviewed_challenge_action_options": {
            "artifact": rel(
                artifact("reviewed_challenge_action_options_n32_q12289.json")
            ),
            "recommended": action_options.get("recommendation", {}).get("selected"),
            "recommended_transaction_bytes": action_options.get(
                "recommendation", {}
            ).get("transaction_bytes"),
            "exact_250b_requires_new_argument": action_options.get("decision", {}).get(
                "exact_250b_requires_new_argument"
            ),
        },
        "hidden_source_proof_semantics_target": {
            "artifact": rel(
                artifact("hidden_source_proof_semantics_target_n32_q12289.json")
            ),
            "report": rel(
                artifact("hidden_source_proof_semantics_target_n32_q12289.md")
            ),
            "status": hidden_source_target.get("status"),
            "deployment_ready": hidden_source_target.get("deployment_ready"),
            "current_envelope_bytes": hidden_source_target.get("target", {}).get(
                "current_envelope_bytes"
            ),
            "carrier_options": [
                option.get("id")
                for option in hidden_source_target.get("carrier_options", [])
            ],
            "open_obligations": [
                item.get("id")
                for item in hidden_source_target.get(
                    "accepting_verifier_obligations", []
                )
                if item.get("status") != "closed"
            ],
            "decision_gates": hidden_source_target.get("decision_gates", {}),
        },
        "hidden_source_accepting_relation_spec": {
            "artifact": rel(
                artifact("hidden_source_accepting_relation_spec_n32_q12289.json")
            ),
            "report": rel(
                artifact("hidden_source_accepting_relation_spec_n32_q12289.md")
            ),
            "status": hidden_source_relation.get("status"),
            "deployment_ready": hidden_source_relation.get("deployment_ready"),
            "envelope_bytes": hidden_source_relation.get("target", {}).get(
                "envelope_bytes"
            ),
            "verifier_phases": [
                phase.get("phase")
                for phase in hidden_source_relation.get("verifier_phases", [])
            ],
            "witness_predicates": [
                predicate.get("predicate")
                for predicate in hidden_source_relation.get("witness_relation", [])
            ],
            "theorem_hooks": [
                hook.get("hook")
                for hook in hidden_source_relation.get("theorem_hooks", [])
            ],
            "negative_control_count": len(
                hidden_source_relation.get("negative_controls_required", [])
            ),
        },
        "hidden_source_carrier_construction_decision": {
            "artifact": rel(
                artifact("hidden_source_carrier_construction_decision_n32_q12289.json")
            ),
            "report": rel(
                artifact("hidden_source_carrier_construction_decision_n32_q12289.md")
            ),
            "status": hidden_source_carrier.get("status"),
            "deployment_ready": hidden_source_carrier.get("deployment_ready"),
            "selected_candidate": hidden_source_carrier.get("selected_candidate"),
            "selected_envelope_bytes": hidden_source_carrier.get("target", {}).get(
                "selected_envelope_bytes"
            ),
            "candidate_count": len(
                hidden_source_carrier.get("candidate_evaluations", [])
            ),
            "open_gate_count": sum(
                1
                for gate in hidden_source_carrier.get("decision_gates", [])
                if gate.get("status") == "open"
            ),
            "required_modeling_work": [
                item.get("id")
                for item in hidden_source_carrier.get("required_modeling_work", [])
            ],
        },
        "hidden_source_state_assisted_carrier_relation": {
            "artifact": rel(
                artifact("hidden_source_state_assisted_carrier_relation_n32_q12289.json")
            ),
            "report": rel(
                artifact("hidden_source_state_assisted_carrier_relation_n32_q12289.md")
            ),
            "status": state_assisted_carrier.get("status"),
            "deployment_ready": state_assisted_carrier.get("deployment_ready"),
            "carrier_bytes": state_assisted_carrier.get("target", {}).get(
                "carrier_bytes"
            ),
            "envelope_bytes": state_assisted_carrier.get("target", {}).get(
                "envelope_bytes"
            ),
            "state_creation_fields": [
                item.get("field")
                for item in state_assisted_carrier.get("state_creation_frame", [])
            ],
            "carrier_predicates": [
                item.get("predicate")
                for item in state_assisted_carrier.get(
                    "carrier_handle_predicate", []
                )
            ],
            "theorem_cases": [
                item.get("case")
                for item in state_assisted_carrier.get(
                    "extractor_or_break_theorem_shape", []
                )
            ],
            "negative_control_count": len(
                state_assisted_carrier.get("negative_controls_required", [])
            ),
            "can_accept_now": state_assisted_carrier.get("decision", {}).get(
                "can_accept_now"
            ),
        },
        "hidden_source_state_assisted_extractor_obligations": {
            "artifact": rel(
                artifact("hidden_source_state_assisted_extractor_obligations_n32_q12289.json")
            ),
            "report": rel(
                artifact("hidden_source_state_assisted_extractor_obligations_n32_q12289.md")
            ),
            "status": state_assisted_extractor.get("status"),
            "deployment_ready": state_assisted_extractor.get("deployment_ready"),
            "carrier_bytes": state_assisted_extractor.get("target", {}).get(
                "carrier_bytes"
            ),
            "envelope_bytes": state_assisted_extractor.get("target", {}).get(
                "envelope_bytes"
            ),
            "obligation_count": len(
                state_assisted_extractor.get("obligation_rows", [])
            ),
            "named_assumption_count": len(
                state_assisted_extractor.get("named_assumptions", [])
            ),
            "fallback_trigger_count": len(
                state_assisted_extractor.get("fallback_triggers", [])
            ),
            "can_accept_now": state_assisted_extractor.get("decision", {}).get(
                "can_accept_now"
            ),
        },
        "hidden_source_state_assisted_negative_controls": {
            "artifact": rel(
                artifact("hidden_source_state_assisted_negative_controls_n32_q12289.json")
            ),
            "report": rel(
                artifact("hidden_source_state_assisted_negative_controls_n32_q12289.md")
            ),
            "status": state_assisted_negative_controls.get("status"),
            "deployment_ready": state_assisted_negative_controls.get(
                "deployment_ready"
            ),
            "control_count": len(
                state_assisted_negative_controls.get("negative_controls", [])
            ),
            "coverage_summary": state_assisted_negative_controls.get(
                "coverage_summary"
            ),
            "open_control_count": sum(
                1
                for control in state_assisted_negative_controls.get(
                    "negative_controls", []
                )
                if control.get("status") == "needs_executable_test"
            ),
            "partial_control_count": sum(
                1
                for control in state_assisted_negative_controls.get(
                    "negative_controls", []
                )
                if control.get("status") == "partially_covered"
            ),
            "can_accept_now": state_assisted_negative_controls.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_state_assisted_opening_frame": {
            "artifact": rel(
                artifact("hidden_source_state_assisted_opening_frame_n32_q12289.json")
            ),
            "report": rel(
                artifact("hidden_source_state_assisted_opening_frame_n32_q12289.md")
            ),
            "status": state_assisted_opening_frame.get("status"),
            "deployment_ready": state_assisted_opening_frame.get(
                "deployment_ready"
            ),
            "carrier_bytes": state_assisted_opening_frame.get("target", {}).get(
                "carrier_bytes"
            ),
            "envelope_bytes": state_assisted_opening_frame.get("target", {}).get(
                "envelope_bytes"
            ),
            "hash_assumption": state_assisted_opening_frame.get("target", {}).get(
                "hash_assumption"
            ),
            "opening_public_frame_bytes": state_assisted_opening_frame.get(
                "opening_commitment", {}
            ).get("public_frame_bytes"),
            "opening_private_frame_bytes": state_assisted_opening_frame.get(
                "opening_commitment", {}
            ).get("private_frame_bytes"),
            "carrier_public_frame_bytes": state_assisted_opening_frame.get(
                "carrier_handle", {}
            ).get("public_frame_bytes"),
            "carrier_private_frame_bytes": state_assisted_opening_frame.get(
                "carrier_handle", {}
            ).get("private_frame_bytes"),
            "rejection_rule_count": len(
                state_assisted_opening_frame.get("rejection_rules", [])
            ),
            "test_vector_count": len(
                state_assisted_opening_frame.get("test_vectors_required", [])
            ),
            "can_accept_now": state_assisted_opening_frame.get("decision", {}).get(
                "can_accept_now"
            ),
        },
        "hidden_source_state_assisted_opening_frame_test_vectors": {
            "artifact": rel(
                artifact(
                    "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.md"
                )
            ),
            "status": state_assisted_opening_frame_test_vectors.get("status"),
            "deployment_ready": state_assisted_opening_frame_test_vectors.get(
                "deployment_ready"
            ),
            "opening_frame_bytes": state_assisted_opening_frame_test_vectors.get(
                "canonical_vectors", {}
            ).get("opening_frame_bytes"),
            "carrier_frame_bytes": state_assisted_opening_frame_test_vectors.get(
                "canonical_vectors", {}
            ).get("carrier_frame_bytes"),
            "mutation_vector_count": len(
                state_assisted_opening_frame_test_vectors.get(
                    "mutation_vectors", []
                )
            ),
            "all_mutations_change_digest": state_assisted_opening_frame_test_vectors.get(
                "all_mutations_change_digest"
            ),
            "can_accept_now": state_assisted_opening_frame_test_vectors.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_state_assisted_carrier_verifier_contract": {
            "artifact": rel(
                artifact(
                    "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.md"
                )
            ),
            "status": state_assisted_carrier_verifier_contract.get("status"),
            "deployment_ready": state_assisted_carrier_verifier_contract.get(
                "deployment_ready"
            ),
            "opening_frame_bytes": state_assisted_carrier_verifier_contract.get(
                "rust_boundary", {}
            ).get("opening_frame_bytes"),
            "carrier_frame_bytes": state_assisted_carrier_verifier_contract.get(
                "rust_boundary", {}
            ).get("carrier_frame_bytes"),
            "public_input_count": len(
                state_assisted_carrier_verifier_contract.get("public_inputs", [])
            ),
            "state_input_count": len(
                state_assisted_carrier_verifier_contract.get("state_inputs", [])
            ),
            "witness_input_count": len(
                state_assisted_carrier_verifier_contract.get("witness_inputs", [])
            ),
            "phase_count": len(
                state_assisted_carrier_verifier_contract.get("verifier_phases", [])
            ),
            "blocker_count": len(
                state_assisted_carrier_verifier_contract.get("blockers", [])
            ),
            "can_accept_now": state_assisted_carrier_verifier_contract.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_state_assisted_account_state_schema_contract": {
            "artifact": rel(
                artifact(
                    "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.md"
                )
            ),
            "status": state_assisted_account_state_schema.get("status"),
            "deployment_ready": state_assisted_account_state_schema.get(
                "deployment_ready"
            ),
            "slot_field_count": len(
                state_assisted_account_state_schema.get("slot_fields", [])
            ),
            "rust_symbol_count": len(
                state_assisted_account_state_schema.get("rust_contract", [])
            ),
            "negative_control_count": len(
                state_assisted_account_state_schema.get("negative_controls", [])
            ),
            "blocker_count": len(
                state_assisted_account_state_schema.get("blockers", [])
            ),
            "can_accept_now": state_assisted_account_state_schema.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_state_assisted_hidden_relation_verifier_contract": {
            "artifact": rel(
                artifact(
                    "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.md"
                )
            ),
            "status": state_assisted_hidden_relation_verifier.get("status"),
            "deployment_ready": state_assisted_hidden_relation_verifier.get(
                "deployment_ready"
            ),
            "public_statement_field_count": len(
                state_assisted_hidden_relation_verifier.get(
                    "public_statement_fields", []
                )
            ),
            "private_opening_field_count": len(
                state_assisted_hidden_relation_verifier.get(
                    "private_opening_fields", []
                )
            ),
            "verifier_check_count": len(
                state_assisted_hidden_relation_verifier.get("verifier_checks", [])
            ),
            "extraction_case_count": len(
                state_assisted_hidden_relation_verifier.get("extraction_cases", [])
            ),
            "implementation_target_count": len(
                state_assisted_hidden_relation_verifier.get(
                    "implementation_targets", []
                )
            ),
            "blocker_count": len(
                state_assisted_hidden_relation_verifier.get("blockers", [])
            ),
            "can_accept_now": state_assisted_hidden_relation_verifier.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_state_assisted_hidden_relation_negative_controls": {
            "artifact": rel(
                artifact(
                    "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.md"
                )
            ),
            "status": state_assisted_hidden_relation_negative_controls.get("status"),
            "deployment_ready": state_assisted_hidden_relation_negative_controls.get(
                "deployment_ready"
            ),
            "control_count": len(
                state_assisted_hidden_relation_negative_controls.get(
                    "negative_controls", []
                )
            ),
            "coverage_summary": state_assisted_hidden_relation_negative_controls.get(
                "coverage_summary"
            ),
            "theorem_requirement_count": len(
                state_assisted_hidden_relation_negative_controls.get(
                    "theorem_requirements", []
                )
            ),
            "can_accept_now": state_assisted_hidden_relation_negative_controls.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_state_assisted_hidden_relation_extractor_theorem": {
            "artifact": rel(
                artifact(
                    "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.md"
                )
            ),
            "status": state_assisted_hidden_relation_extractor_theorem.get("status"),
            "deployment_ready": state_assisted_hidden_relation_extractor_theorem.get(
                "deployment_ready"
            ),
            "theorem_name": state_assisted_hidden_relation_extractor_theorem.get(
                "theorem_statement", {}
            ).get("name"),
            "hypothesis_count": len(
                state_assisted_hidden_relation_extractor_theorem.get(
                    "hypotheses", []
                )
            ),
            "extractor_output_count": len(
                state_assisted_hidden_relation_extractor_theorem.get(
                    "extractor_outputs", []
                )
            ),
            "named_break_count": len(
                state_assisted_hidden_relation_extractor_theorem.get(
                    "named_breaks", []
                )
            ),
            "proof_step_count": len(
                state_assisted_hidden_relation_extractor_theorem.get(
                    "proof_skeleton", []
                )
            ),
            "open_obligation_count": len(
                state_assisted_hidden_relation_extractor_theorem.get(
                    "open_obligations", []
                )
            ),
            "can_accept_now": state_assisted_hidden_relation_extractor_theorem.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_state_assisted_masked_projection_global_soundness": {
            "artifact": rel(
                artifact(
                    "hidden_source_state_assisted_masked_projection_global_soundness_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_state_assisted_masked_projection_global_soundness_n32_q12289.md"
                )
            ),
            "status": state_assisted_masked_projection_global_soundness.get("status"),
            "deployment_ready": state_assisted_masked_projection_global_soundness.get(
                "deployment_ready"
            ),
            "theorem_name": state_assisted_masked_projection_global_soundness.get(
                "theorem", {}
            ).get("name"),
            "implemented_local_hops": state_assisted_masked_projection_global_soundness.get(
                "decision", {}
            ).get("implemented_local_hops"),
            "open_global_hops": state_assisted_masked_projection_global_soundness.get(
                "decision", {}
            ).get("open_global_hops"),
            "proof_obligation_count": len(
                state_assisted_masked_projection_global_soundness.get(
                    "proof_obligations", []
                )
            ),
            "named_break_count": len(
                state_assisted_masked_projection_global_soundness.get("named_breaks", [])
            ),
            "can_accept_now": state_assisted_masked_projection_global_soundness.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": state_assisted_masked_projection_global_soundness.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "pre_signature_projection_mask_prf_leakage_simulator": {
            "artifact": rel(
                artifact(
                    "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.md"
                )
            ),
            "status": pre_signature_projection_mask_prf_leakage_simulator.get("status"),
            "deployment_ready": pre_signature_projection_mask_prf_leakage_simulator.get(
                "deployment_ready"
            ),
            "world_count": len(
                pre_signature_projection_mask_prf_leakage_simulator.get("worlds", [])
            ),
            "simulator_step_count": len(
                pre_signature_projection_mask_prf_leakage_simulator.get(
                    "simulator_steps", []
                )
            ),
            "named_break_count": len(
                pre_signature_projection_mask_prf_leakage_simulator.get(
                    "named_breaks", []
                )
            ),
            "advantage_term_count": len(
                pre_signature_projection_mask_prf_leakage_simulator.get(
                    "advantage_bound", []
                )
            ),
            "open_gap_count": len(
                pre_signature_projection_mask_prf_leakage_simulator.get(
                    "open_gaps", []
                )
            ),
            "can_accept_now": pre_signature_projection_mask_prf_leakage_simulator.get(
                "decision", {}
            ).get("can_accept_now"),
            "can_consume_in_global_soundness": pre_signature_projection_mask_prf_leakage_simulator.get(
                "decision", {}
            ).get("can_consume_in_global_soundness"),
            "next_artifact": pre_signature_projection_mask_prf_leakage_simulator.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "pre_signature_projection_mask_prf_ro_reduction": {
            "artifact": rel(
                artifact("pre_signature_projection_mask_prf_ro_reduction_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_mask_prf_ro_reduction_n32_q12289.md")
            ),
            "status": pre_signature_projection_mask_prf_ro_reduction.get("status"),
            "deployment_ready": pre_signature_projection_mask_prf_ro_reduction.get(
                "deployment_ready"
            ),
            "hybrid_step_count": len(
                pre_signature_projection_mask_prf_ro_reduction.get("hybrid_steps", [])
            ),
            "named_break_count": len(
                pre_signature_projection_mask_prf_ro_reduction.get("named_breaks", [])
            ),
            "open_item_count": len(
                pre_signature_projection_mask_prf_ro_reduction.get(
                    "open_before_gap_closes", []
                )
            ),
            "can_consume_in_parent_simulator": pre_signature_projection_mask_prf_ro_reduction.get(
                "decision", {}
            ).get("can_consume_in_parent_simulator"),
            "closes_simulator_gap_1": pre_signature_projection_mask_prf_ro_reduction.get(
                "decision", {}
            ).get("closes_simulator_gap_1"),
        },
        "pre_signature_projection_lifecycle_same_game_lift": {
            "artifact": rel(
                artifact("pre_signature_projection_lifecycle_same_game_lift_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_lifecycle_same_game_lift_n32_q12289.md")
            ),
            "status": pre_signature_projection_lifecycle_same_game_lift.get("status"),
            "deployment_ready": pre_signature_projection_lifecycle_same_game_lift.get(
                "deployment_ready"
            ),
            "named_break_count": len(
                pre_signature_projection_lifecycle_same_game_lift.get(
                    "named_lifecycle_breaks", []
                )
            ),
            "supported_now_count": len(
                pre_signature_projection_lifecycle_same_game_lift.get("supported_now", [])
            ),
            "remains_unproved_count": len(
                pre_signature_projection_lifecycle_same_game_lift.get(
                    "remains_unproved", []
                )
            ),
            "can_consume_in_mask_simulator": pre_signature_projection_lifecycle_same_game_lift.get(
                "decision", {}
            ).get("can_consume_in_mask_simulator"),
            "can_claim_lifecycle_term_zero": pre_signature_projection_lifecycle_same_game_lift.get(
                "decision", {}
            ).get("can_claim_lifecycle_term_zero"),
        },
        "pre_signature_projection_spend_time_leakage_budget": {
            "artifact": rel(
                artifact("pre_signature_projection_spend_time_leakage_budget_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_spend_time_leakage_budget_n32_q12289.md")
            ),
            "status": pre_signature_projection_spend_time_leakage_budget.get("status"),
            "deployment_ready": pre_signature_projection_spend_time_leakage_budget.get(
                "deployment_ready"
            ),
            "accepting_disclosure_count": len(
                pre_signature_projection_spend_time_leakage_budget.get(
                    "accepting_certificate_disclosures", []
                )
            ),
            "remaining_obligation_count": len(
                pre_signature_projection_spend_time_leakage_budget.get(
                    "remaining_proof_and_test_obligations", []
                )
            ),
            "can_consume_as_fail_closed_budget": pre_signature_projection_spend_time_leakage_budget.get(
                "decision", {}
            ).get("can_consume_as_fail_closed_budget"),
            "gap_3_local_status": pre_signature_projection_spend_time_leakage_budget.get(
                "decision", {}
            ).get("gap_3_local_status"),
        },
        "pre_signature_projection_reject_equivalence": {
            "artifact": rel(
                artifact("pre_signature_projection_reject_equivalence_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_reject_equivalence_n32_q12289.md")
            ),
            "status": pre_signature_projection_reject_equivalence.get("status"),
            "deployment_ready": pre_signature_projection_reject_equivalence.get(
                "fail_closed_result", {}
            ).get("deployment_ready"),
            "failure_class_count": len(
                pre_signature_projection_reject_equivalence.get("failure_classes", [])
            ),
            "remaining_gap_count": len(
                pre_signature_projection_reject_equivalence.get("remaining_gaps", [])
            ),
            "constant_time_claim": pre_signature_projection_reject_equivalence.get(
                "fail_closed_result", {}
            ).get("can_claim_constant_time"),
            "simulator_gap_status": pre_signature_projection_reject_equivalence.get(
                "fail_closed_result", {}
            ).get("constant_time_reject_equivalence_gap_status"),
        },
        "pre_signature_projection_mask_prf_leakage_theorem_composition": {
            "artifact": rel(
                artifact(
                    "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.md"
                )
            ),
            "status": pre_signature_projection_mask_prf_leakage_theorem_composition.get(
                "status"
            ),
            "deployment_ready": pre_signature_projection_mask_prf_leakage_theorem_composition.get(
                "deployment_ready"
            ),
            "theorem_name": pre_signature_projection_mask_prf_leakage_theorem_composition.get(
                "theorem", {}
            ).get("name"),
            "composition_hop_count": len(
                pre_signature_projection_mask_prf_leakage_theorem_composition.get(
                    "composition_hops", []
                )
            ),
            "named_break_count": len(
                pre_signature_projection_mask_prf_leakage_theorem_composition.get(
                    "named_breaks", []
                )
            ),
            "carried_assumption_count": len(
                pre_signature_projection_mask_prf_leakage_theorem_composition.get(
                    "carried_assumptions", []
                )
            ),
            "open_item_count": len(
                pre_signature_projection_mask_prf_leakage_theorem_composition.get(
                    "open_to_close", []
                )
            ),
            "can_consume_in_global_soundness": pre_signature_projection_mask_prf_leakage_theorem_composition.get(
                "decision", {}
            ).get("can_consume_in_global_soundness"),
            "next_artifact": pre_signature_projection_mask_prf_leakage_theorem_composition.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "pre_signature_projection_sha256_mask_prf_assumption_resolution": {
            "artifact": rel(
                artifact(
                    "pre_signature_projection_sha256_mask_prf_assumption_resolution_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "pre_signature_projection_sha256_mask_prf_assumption_resolution_n32_q12289.md"
                )
            ),
            "status": pre_signature_projection_sha256_mask_prf_assumption_resolution.get("status"),
            "deployment_ready": pre_signature_projection_sha256_mask_prf_assumption_resolution.get(
                "deployment_ready"
            ),
            "closure_path": pre_signature_projection_sha256_mask_prf_assumption_resolution.get(
                "decision", {}
            ).get("closure_path"),
            "can_discharge": pre_signature_projection_sha256_mask_prf_assumption_resolution.get(
                "decision", {}
            ).get("can_mark_A_sha256_projection_mask_frame_prf_discharged"),
        },
        "pre_signature_projection_global_domain_registry": {
            "artifact": rel(
                artifact("pre_signature_projection_global_domain_registry_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_global_domain_registry_n32_q12289.md")
            ),
            "status": pre_signature_projection_global_domain_registry.get("status"),
            "deployment_ready": pre_signature_projection_global_domain_registry.get(
                "deployment_ready"
            ),
            "registry_entry_count": len(
                pre_signature_projection_global_domain_registry.get("registry_entries", [])
            ),
            "registered_duplicate_count": pre_signature_projection_global_domain_registry.get(
                "domain_string_uniqueness", {}
            ).get("exact_duplicate_registered_domain_count"),
            "byte_frame_ambiguity_risk_count": len(
                pre_signature_projection_global_domain_registry.get(
                    "byte_frame_ambiguity_risks", []
                )
            ),
            "can_discharge": pre_signature_projection_global_domain_registry.get(
                "decision", {}
            ).get("can_claim_global_frame_disjointness_now"),
        },
        "pre_signature_projection_lifecycle_same_game_discharge": {
            "artifact": rel(
                artifact("pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.md")
            ),
            "status": pre_signature_projection_lifecycle_same_game_discharge.get("status"),
            "deployment_ready": pre_signature_projection_lifecycle_same_game_discharge.get(
                "deployment_ready"
            ),
            "discharged_term_count": len(
                pre_signature_projection_lifecycle_same_game_discharge.get(
                    "discharged_terms", []
                )
            ),
            "residual_term_count": len(
                pre_signature_projection_lifecycle_same_game_discharge.get(
                    "residual_terms", []
                )
            ),
        },
        "pre_signature_projection_spend_time_budget_discharge": {
            "artifact": rel(
                artifact("pre_signature_projection_spend_time_budget_discharge_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_spend_time_budget_discharge_n32_q12289.md")
            ),
            "status": pre_signature_projection_spend_time_budget_discharge.get("status"),
            "deployment_ready": pre_signature_projection_spend_time_budget_discharge.get(
                "deployment_ready"
            ),
            "disclosure_counts": pre_signature_projection_spend_time_budget_discharge.get(
                "disclosure_counts", {}
            ),
            "can_mark_term_proved": pre_signature_projection_spend_time_budget_discharge.get(
                "decision", {}
            ).get("can_mark_term_proved"),
        },
        "pre_signature_projection_reject_equivalence_discharge": {
            "artifact": rel(
                artifact("pre_signature_projection_reject_equivalence_discharge_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_reject_equivalence_discharge_n32_q12289.md")
            ),
            "status": pre_signature_projection_reject_equivalence_discharge.get("status"),
            "deployment_ready": pre_signature_projection_reject_equivalence_discharge.get(
                "deployment_ready"
            ),
            "status_counts": pre_signature_projection_reject_equivalence_discharge.get(
                "status_counts", {}
            ),
            "can_remove_term": pre_signature_projection_reject_equivalence_discharge.get(
                "decision", {}
            ).get("can_remove_A_reject_equivalence_term"),
        },
        "pre_signature_projection_helper_api_export_boundary": {
            "artifact": rel(
                artifact("pre_signature_projection_helper_api_export_boundary_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_helper_api_export_boundary_n32_q12289.md")
            ),
            "status": pre_signature_projection_helper_api_export_boundary.get("status"),
            "deployment_ready": pre_signature_projection_helper_api_export_boundary.get(
                "deployment_ready"
            ),
            "risky_surface_count": pre_signature_projection_helper_api_export_boundary.get(
                "risky_public_or_exportable_surface_count"
            ),
            "wrapper_test_count": len(
                pre_signature_projection_helper_api_export_boundary.get(
                    "wrapper_tests_required", []
                )
            ),
            "residual_blocker_count": len(
                pre_signature_projection_helper_api_export_boundary.get(
                    "residual_blockers", []
                )
            ),
        },
        "pre_signature_projection_helper_api_export_boundary_closure": {
            "artifact": rel(
                artifact("pre_signature_projection_helper_api_export_boundary_closure_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_helper_api_export_boundary_closure_n32_q12289.md")
            ),
            "status": pre_signature_projection_helper_api_export_boundary_closure.get("status"),
            "deployment_ready": pre_signature_projection_helper_api_export_boundary_closure.get(
                "deployment_ready"
            ),
            "closed_blocker_count": len(
                pre_signature_projection_helper_api_export_boundary_closure.get(
                    "blockers_closed_by_this_artifact", []
                )
            ),
            "remaining_blocker_count": len(
                pre_signature_projection_helper_api_export_boundary_closure.get(
                    "blockers_remaining", []
                )
            ),
            "wrapper_test_count": len(
                pre_signature_projection_helper_api_export_boundary_closure.get(
                    "wrapper_tests_required", []
                )
            ),
        },
        "pre_signature_projection_no_export_negative_tests": {
            "artifact": rel(
                artifact("pre_signature_projection_no_export_negative_tests_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_no_export_negative_tests_n32_q12289.md")
            ),
            "status": pre_signature_projection_no_export_negative_tests.get("status"),
            "deployment_ready": pre_signature_projection_no_export_negative_tests.get(
                "deployment_ready"
            ),
            "required_test_count": len(
                pre_signature_projection_no_export_negative_tests.get(
                    "required_tests", []
                )
            ),
            "can_close_projection_overread_residual": pre_signature_projection_no_export_negative_tests.get(
                "acceptance_gate", {}
            ).get("can_close_projection_overread_residual"),
            "can_mark_spend_time_term_proved": pre_signature_projection_no_export_negative_tests.get(
                "acceptance_gate", {}
            ).get("can_mark_spend_time_term_proved"),
        },
        "pre_signature_projection_no_export_negative_tests_closure": {
            "artifact": rel(
                artifact("pre_signature_projection_no_export_negative_tests_closure_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_no_export_negative_tests_closure_n32_q12289.md")
            ),
            "status": pre_signature_projection_no_export_negative_tests_closure.get("status"),
            "deployment_ready": pre_signature_projection_no_export_negative_tests_closure.get(
                "deployment_ready"
            ),
            "implemented_count": pre_signature_projection_no_export_negative_tests_closure.get(
                "implemented_count"
            ),
            "planned_count": pre_signature_projection_no_export_negative_tests_closure.get(
                "planned_count"
            ),
            "remaining_required_tests": pre_signature_projection_no_export_negative_tests_closure.get(
                "remaining_required_tests", []
            ),
        },
        "pre_signature_projection_req_pad_reject_discharge": {
            "artifact": rel(
                artifact("pre_signature_projection_req_pad_reject_discharge_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_req_pad_reject_discharge_n32_q12289.md")
            ),
            "status": pre_signature_projection_req_pad_reject_discharge.get("status"),
            "deployment_ready": pre_signature_projection_req_pad_reject_discharge.get(
                "deployment_ready"
            ),
            "failure_case_count": len(
                pre_signature_projection_req_pad_reject_discharge.get("failure_cases", [])
            ),
            "required_kat_count": len(
                pre_signature_projection_req_pad_reject_discharge.get("required_kats", [])
            ),
            "can_remove": pre_signature_projection_req_pad_reject_discharge.get(
                "decision", {}
            ).get("can_remove_req_pad_from_A_reject_equivalence"),
        },
        "pre_signature_projection_req_pad_reject_closure": {
            "artifact": rel(
                artifact("pre_signature_projection_req_pad_reject_closure_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_req_pad_reject_closure_n32_q12289.md")
            ),
            "status": pre_signature_projection_req_pad_reject_closure.get("status"),
            "deployment_ready": pre_signature_projection_req_pad_reject_closure.get(
                "deployment_ready"
            ),
            "kat_count": pre_signature_projection_req_pad_reject_closure.get("kat_count"),
            "five_required_kats_implemented": pre_signature_projection_req_pad_reject_closure.get(
                "decision", {}
            ).get("five_required_kats_implemented"),
            "can_remove_without_timing": pre_signature_projection_req_pad_reject_closure.get(
                "decision", {}
            ).get("can_remove_req_pad_from_A_reject_equivalence_without_timing_review"),
        },
        "pre_signature_projection_req_witness_reject_discharge": {
            "artifact": rel(
                artifact("pre_signature_projection_req_witness_reject_discharge_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_req_witness_reject_discharge_n32_q12289.md")
            ),
            "status": pre_signature_projection_req_witness_reject_discharge.get("status"),
            "deployment_ready": pre_signature_projection_req_witness_reject_discharge.get(
                "deployment_ready"
            ),
            "failure_case_count": len(
                pre_signature_projection_req_witness_reject_discharge.get(
                    "witness_failure_cases", []
                )
            ),
            "required_kat_count": len(
                pre_signature_projection_req_witness_reject_discharge.get("required_kats", [])
            ),
            "can_remove": pre_signature_projection_req_witness_reject_discharge.get(
                "decision", {}
            ).get("can_remove_REQ_WITNESS_from_A_reject_equivalence"),
        },
        "pre_signature_projection_req_witness_reject_closure": {
            "artifact": rel(
                artifact("pre_signature_projection_req_witness_reject_closure_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_req_witness_reject_closure_n32_q12289.md")
            ),
            "status": pre_signature_projection_req_witness_reject_closure.get("status"),
            "deployment_ready": pre_signature_projection_req_witness_reject_closure.get(
                "deployment_ready"
            ),
            "covered_failure_case_count": len(
                pre_signature_projection_req_witness_reject_closure.get(
                    "covered_failure_cases", []
                )
            ),
            "kat_gap_closed": pre_signature_projection_req_witness_reject_closure.get(
                "decision", {}
            ).get("kat_gap_closed"),
            "can_remove": pre_signature_projection_req_witness_reject_closure.get(
                "decision", {}
            ).get("can_remove_REQ_WITNESS_from_A_reject_equivalence"),
        },
        "pre_signature_projection_req_carrier_reject_discharge": {
            "artifact": rel(
                artifact("pre_signature_projection_req_carrier_reject_discharge_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_req_carrier_reject_discharge_n32_q12289.md")
            ),
            "status": pre_signature_projection_req_carrier_reject_discharge.get("status"),
            "deployment_ready": pre_signature_projection_req_carrier_reject_discharge.get(
                "deployment_ready"
            ),
            "failure_case_count": len(
                pre_signature_projection_req_carrier_reject_discharge.get(
                    "carrier_failure_cases", []
                )
            ),
            "required_kat_count": len(
                pre_signature_projection_req_carrier_reject_discharge.get(
                    "exact_kat_harness_needed", []
                )
            ),
            "can_remove": pre_signature_projection_req_carrier_reject_discharge.get(
                "decision", {}
            ).get("can_close_req_carrier"),
        },
        "pre_signature_projection_req_carrier_reject_closure": {
            "artifact": rel(
                artifact("pre_signature_projection_req_carrier_reject_closure_n32_q12289.json")
            ),
            "report": rel(
                artifact("pre_signature_projection_req_carrier_reject_closure_n32_q12289.md")
            ),
            "status": pre_signature_projection_req_carrier_reject_closure.get("status"),
            "deployment_ready": pre_signature_projection_req_carrier_reject_closure.get(
                "deployment_ready"
            ),
            "kat_count": pre_signature_projection_req_carrier_reject_closure.get(
                "decision", {}
            ).get("kat_count"),
            "can_remove": pre_signature_projection_req_carrier_reject_closure.get(
                "decision", {}
            ).get("can_close_req_carrier"),
        },
        "hidden_source_state_assisted_compact_proof_instantiation": {
            "artifact": rel(
                artifact(
                    "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.md"
                )
            ),
            "status": state_assisted_compact_proof_instantiation.get("status"),
            "deployment_ready": state_assisted_compact_proof_instantiation.get(
                "deployment_ready"
            ),
            "candidate_count": len(
                state_assisted_compact_proof_instantiation.get(
                    "candidate_instantiations", []
                )
            ),
            "selected_next_track": state_assisted_compact_proof_instantiation.get(
                "decision", {}
            ).get("selected_next_track"),
            "fallback_track": state_assisted_compact_proof_instantiation.get(
                "decision", {}
            ).get("fallback_track"),
            "decision_gate_count": len(
                state_assisted_compact_proof_instantiation.get("decision_gates", [])
            ),
            "implementation_task_count": len(
                state_assisted_compact_proof_instantiation.get(
                    "implementation_tasks", []
                )
            ),
            "can_accept_now": state_assisted_compact_proof_instantiation.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_state_assisted_ro_pok_assumption_api": {
            "artifact": rel(
                artifact(
                    "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.md"
                )
            ),
            "status": state_assisted_ro_pok_assumption_api.get("status"),
            "deployment_ready": state_assisted_ro_pok_assumption_api.get(
                "deployment_ready"
            ),
            "assumption_id": state_assisted_ro_pok_assumption_api.get(
                "assumption", {}
            ).get("id"),
            "verifier_function": state_assisted_ro_pok_assumption_api.get(
                "verifier_api", {}
            ).get("function"),
            "input_count": len(
                state_assisted_ro_pok_assumption_api.get("verifier_api", {}).get(
                    "inputs", []
                )
            ),
            "forbidden_input_count": len(
                state_assisted_ro_pok_assumption_api.get("verifier_api", {}).get(
                    "forbidden_inputs", []
                )
            ),
            "acceptance_rule_count": len(
                state_assisted_ro_pok_assumption_api.get("acceptance_rules", [])
            ),
            "review_question_count": len(
                state_assisted_ro_pok_assumption_api.get("review_questions", [])
            ),
            "can_accept_now": state_assisted_ro_pok_assumption_api.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_state_assisted_ro_pok_byte_negative_tests": {
            "artifact": rel(
                artifact(
                    "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.md"
                )
            ),
            "status": state_assisted_ro_pok_byte_negative_tests.get("status"),
            "deployment_ready": state_assisted_ro_pok_byte_negative_tests.get(
                "deployment_ready"
            ),
            "test_count": state_assisted_ro_pok_byte_negative_tests.get("test_count"),
            "executable_status": state_assisted_ro_pok_byte_negative_tests.get(
                "executable_status", {}
            ).get("current"),
            "can_accept_now": state_assisted_ro_pok_byte_negative_tests.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_fallback_projection_lane_map_consistency": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_lane_map_consistency_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_lane_map_consistency_n32_q12289.md"
                )
            ),
            "status": fallback_projection_lane_map_consistency.get("status"),
            "deployment_ready": fallback_projection_lane_map_consistency.get(
                "deployment_ready"
            ),
            "row_digest": fallback_projection_lane_map_consistency.get(
                "target", {}
            ).get("row_digest"),
            "passing_lane_count": fallback_projection_lane_map_consistency.get(
                "coverage", {}
            ).get("passing_lane_count"),
            "failing_lane_count": fallback_projection_lane_map_consistency.get(
                "coverage", {}
            ).get("failing_lane_count"),
            "can_accept_now": fallback_projection_lane_map_consistency.get(
                "decision", {}
            ).get("can_accept_now"),
        },
        "hidden_source_fallback_projection_matrix_builder": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_matrix_builder_n32_q12289.md"
                )
            ),
            "status": fallback_projection_matrix_builder.get("status"),
            "deployment_ready": fallback_projection_matrix_builder.get(
                "deployment_ready"
            ),
            "rows": fallback_projection_matrix_builder.get("target", {}).get("rows"),
            "columns": fallback_projection_matrix_builder.get("target", {}).get(
                "columns"
            ),
            "rank": fallback_projection_matrix_builder.get("target", {}).get("rank"),
            "nullity_projection_only": fallback_projection_matrix_builder.get(
                "target", {}
            ).get("nullity_projection_only"),
            "matrix_sha256": fallback_projection_matrix_builder.get("target", {}).get(
                "matrix_sha256"
            ),
            "generalized_columns": fallback_projection_matrix_builder.get(
                "target", {}
            ).get("generalized_columns"),
            "generalized_system_sha256": fallback_projection_matrix_builder.get(
                "target", {}
            ).get("generalized_system_sha256"),
            "generalized_projection_matrix_built": fallback_projection_matrix_builder.get(
                "coverage", {}
            ).get("generalized_projection_matrix_built"),
            "open_bridge_slot_count": fallback_projection_matrix_builder.get(
                "coverage", {}
            ).get("open_bridge_slot_count"),
            "bounded_solver_ready": fallback_projection_matrix_builder.get(
                "coverage", {}
            ).get("bounded_solver_ready"),
            "can_accept_now": fallback_projection_matrix_builder.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_matrix_builder.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_checked_core_lift": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_checked_core_lift_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_checked_core_lift_n32_q12289.md"
                )
            ),
            "status": fallback_projection_checked_core_lift.get("status"),
            "deployment_ready": fallback_projection_checked_core_lift.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_checked_core_lift.get(
                "target", {}
            ).get("envelope_bytes"),
            "lift_digest": fallback_projection_checked_core_lift.get(
                "target", {}
            ).get("lift_digest"),
            "direct_sum_columns": fallback_projection_checked_core_lift.get(
                "rank_accounting", {}
            ).get("direct_sum_columns"),
            "direct_sum_rank_without_bridges": fallback_projection_checked_core_lift.get(
                "rank_accounting", {}
            ).get("direct_sum_rank_without_bridges"),
            "direct_sum_nullity_without_bridges": fallback_projection_checked_core_lift.get(
                "rank_accounting", {}
            ).get("direct_sum_nullity_without_bridges"),
            "bridge_interface_count": len(
                fallback_projection_checked_core_lift.get("bridge_interfaces", [])
            ),
            "can_accept_now": fallback_projection_checked_core_lift.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_checked_core_lift.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_semantic_bridge_spec": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.md"
                )
            ),
            "status": fallback_projection_semantic_bridge_spec.get("status"),
            "deployment_ready": fallback_projection_semantic_bridge_spec.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_semantic_bridge_spec.get(
                "target", {}
            ).get("envelope_bytes"),
            "bridge_spec_digest": fallback_projection_semantic_bridge_spec.get(
                "target", {}
            ).get("bridge_spec_digest"),
            "carrier_bytes": fallback_projection_semantic_bridge_spec.get(
                "byte_layout", {}
            ).get("carrier_bytes"),
            "lane_count": fallback_projection_semantic_bridge_spec.get(
                "byte_layout", {}
            ).get("lane_count"),
            "semantic_rejection_predicate_count": len(
                fallback_projection_semantic_bridge_spec.get(
                    "semantic_rejection_predicates", []
                )
            ),
            "bridge_group_count": len(
                fallback_projection_semantic_bridge_spec.get("bridge_groups", [])
            ),
            "can_accept_now": fallback_projection_semantic_bridge_spec.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_semantic_bridge_spec.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_bridge_exact_replay": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.md"
                )
            ),
            "status": fallback_projection_bridge_exact_replay.get("status"),
            "deployment_ready": fallback_projection_bridge_exact_replay.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_bridge_exact_replay.get(
                "target", {}
            ).get("envelope_bytes"),
            "bridge_replay_digest": fallback_projection_bridge_exact_replay.get(
                "target", {}
            ).get("bridge_replay_digest"),
            "carrier_bridge_rank": fallback_projection_bridge_exact_replay.get(
                "carrier_bridge", {}
            ).get("rank"),
            "rank_model_count": len(
                fallback_projection_bridge_exact_replay.get("rank_models", [])
            ),
            "missing_bridge_obligation_count": len(
                fallback_projection_bridge_exact_replay.get(
                    "missing_bridge_obligations", []
                )
            ),
            "can_accept_now": fallback_projection_bridge_exact_replay.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_bridge_exact_replay.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_exact_infeasibility_replay": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.md"
                )
            ),
            "status": fallback_projection_exact_infeasibility_replay.get("status"),
            "deployment_ready": fallback_projection_exact_infeasibility_replay.get(
                "deployment_ready"
            ),
            "replay_digest": fallback_projection_exact_infeasibility_replay.get(
                "target", {}
            ).get("replay_digest"),
            "replay_result": fallback_projection_exact_infeasibility_replay.get(
                "exact_replay", {}
            ).get("result"),
            "infeasible": fallback_projection_exact_infeasibility_replay.get(
                "exact_replay", {}
            ).get("infeasible"),
            "bad_vector_exported": fallback_projection_exact_infeasibility_replay.get(
                "exact_replay", {}
            ).get("bad_vector_exported"),
            "bad_vector_kind": fallback_projection_exact_infeasibility_replay.get(
                "bad_vector_classification", {}
            ).get("kind"),
            "can_accept_now": fallback_projection_exact_infeasibility_replay.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_exact_infeasibility_replay.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_bridge_equation_derivation": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.md"
                )
            ),
            "status": fallback_projection_bridge_equation_derivation.get("status"),
            "deployment_ready": fallback_projection_bridge_equation_derivation.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_bridge_equation_derivation.get(
                "target", {}
            ).get("envelope_bytes"),
            "derivation_digest": fallback_projection_bridge_equation_derivation.get(
                "target", {}
            ).get("derivation_digest"),
            "lane_equations_derived": fallback_projection_bridge_equation_derivation.get(
                "rank_impact", {}
            ).get("carrier_lane_definition_equations_derived"),
            "new_checked_core_coupling_equations_derived": fallback_projection_bridge_equation_derivation.get(
                "rank_impact", {}
            ).get("new_checked_core_coupling_equations_derived"),
            "can_run_bounded_solver": fallback_projection_bridge_equation_derivation.get(
                "rank_impact", {}
            ).get("can_run_bounded_solver"),
            "can_accept_now": fallback_projection_bridge_equation_derivation.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_bridge_equation_derivation.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_checked_core_coupling_options": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.md"
                )
            ),
            "status": fallback_projection_checked_core_coupling_options.get("status"),
            "deployment_ready": fallback_projection_checked_core_coupling_options.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_checked_core_coupling_options.get(
                "target", {}
            ).get("envelope_bytes"),
            "options_digest": fallback_projection_checked_core_coupling_options.get(
                "target", {}
            ).get("options_digest"),
            "selected_route": fallback_projection_checked_core_coupling_options.get(
                "decision", {}
            ).get("selected_route"),
            "route_option_count": len(
                fallback_projection_checked_core_coupling_options.get(
                    "route_options", []
                )
            ),
            "can_accept_now": fallback_projection_checked_core_coupling_options.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_checked_core_coupling_options.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_direct_linear_lift_attempt": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.md"
                )
            ),
            "status": fallback_projection_direct_linear_lift_attempt.get("status"),
            "deployment_ready": fallback_projection_direct_linear_lift_attempt.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_direct_linear_lift_attempt.get(
                "target", {}
            ).get("envelope_bytes"),
            "attempt_digest": fallback_projection_direct_linear_lift_attempt.get(
                "target", {}
            ).get("attempt_digest"),
            "selected_route": fallback_projection_direct_linear_lift_attempt.get(
                "route_disposition", {}
            ).get("selected_route"),
            "outcome": fallback_projection_direct_linear_lift_attempt.get(
                "route_disposition", {}
            ).get("outcome"),
            "derived_checked_core_coupling_rows": fallback_projection_direct_linear_lift_attempt.get(
                "attempt", {}
            ).get("derived_checked_core_coupling_rows"),
            "can_run_exact_solver": fallback_projection_direct_linear_lift_attempt.get(
                "attempt", {}
            ).get("can_run_exact_solver"),
            "fallback_route": fallback_projection_direct_linear_lift_attempt.get(
                "route_disposition", {}
            ).get("fallback_route"),
            "next_artifact": fallback_projection_direct_linear_lift_attempt.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_checked_core_break_reduction": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.md"
                )
            ),
            "status": fallback_projection_checked_core_break_reduction.get("status"),
            "deployment_ready": fallback_projection_checked_core_break_reduction.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_checked_core_break_reduction.get(
                "target", {}
            ).get("envelope_bytes"),
            "reduction_digest": fallback_projection_checked_core_break_reduction.get(
                "target", {}
            ).get("reduction_digest"),
            "named_break_target": fallback_projection_checked_core_break_reduction.get(
                "named_break_target", {}
            ).get("id"),
            "named_break_available": fallback_projection_checked_core_break_reduction.get(
                "named_break_target", {}
            ).get("available_in_extractor_theorem"),
            "obligation_counts": fallback_projection_checked_core_break_reduction.get(
                "obligation_counts", {}
            ),
            "can_accept_now": fallback_projection_checked_core_break_reduction.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_checked_core_break_reduction.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_public_bridge_predicate": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.md"
                )
            ),
            "status": fallback_projection_public_bridge_predicate.get("status"),
            "deployment_ready": fallback_projection_public_bridge_predicate.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_public_bridge_predicate.get(
                "target", {}
            ).get("envelope_bytes"),
            "predicate_digest": fallback_projection_public_bridge_predicate.get(
                "target", {}
            ).get("predicate_digest"),
            "lane_checks_specified": fallback_projection_public_bridge_predicate.get(
                "coverage", {}
            ).get("lane_checks_specified"),
            "lane_symbols_equal_active_projection_symbols": fallback_projection_public_bridge_predicate.get(
                "coverage", {}
            ).get("lane_symbols_equal_active_projection_symbols"),
            "implemented_predicate_classes": fallback_projection_public_bridge_predicate.get(
                "coverage", {}
            ).get("implemented_predicate_classes"),
            "open_predicate_classes": fallback_projection_public_bridge_predicate.get(
                "coverage", {}
            ).get("open_predicate_classes"),
            "can_accept_now": fallback_projection_public_bridge_predicate.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_public_bridge_predicate.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_break_export_contract": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_break_export_contract_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_break_export_contract_n32_q12289.md"
                )
            ),
            "status": fallback_projection_break_export_contract.get("status"),
            "deployment_ready": fallback_projection_break_export_contract.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_break_export_contract.get(
                "target", {}
            ).get("envelope_bytes"),
            "contract_digest": fallback_projection_break_export_contract.get(
                "target", {}
            ).get("contract_digest"),
            "export_record_count": fallback_projection_break_export_contract.get(
                "coverage", {}
            ).get("export_record_count"),
            "named_break_exports_available": fallback_projection_break_export_contract.get(
                "coverage", {}
            ).get("named_break_exports_available"),
            "implemented_export_count": fallback_projection_break_export_contract.get(
                "coverage", {}
            ).get("implemented_export_count"),
            "signature_byte_increase": fallback_projection_break_export_contract.get(
                "coverage", {}
            ).get("signature_byte_increase"),
            "can_accept_now": fallback_projection_break_export_contract.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_break_export_contract.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_break_export_negative_tests": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.md"
                )
            ),
            "status": fallback_projection_break_export_negative_tests.get("status"),
            "deployment_ready": fallback_projection_break_export_negative_tests.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_break_export_negative_tests.get(
                "target", {}
            ).get("envelope_bytes"),
            "negative_test_digest": fallback_projection_break_export_negative_tests.get(
                "target", {}
            ).get("negative_test_digest"),
            "test_count": fallback_projection_break_export_negative_tests.get(
                "coverage", {}
            ).get("test_count"),
            "export_test_count": fallback_projection_break_export_negative_tests.get(
                "coverage", {}
            ).get("export_test_count"),
            "implemented_test_count": fallback_projection_break_export_negative_tests.get(
                "coverage", {}
            ).get("implemented_test_count"),
            "signature_byte_increase": fallback_projection_break_export_negative_tests.get(
                "coverage", {}
            ).get("signature_byte_increase"),
            "can_accept_now": fallback_projection_break_export_negative_tests.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_break_export_negative_tests.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_map_redesign_or_bridge_rows": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.md"
                )
            ),
            "status": fallback_projection_map_redesign_or_bridge_rows.get("status"),
            "deployment_ready": fallback_projection_map_redesign_or_bridge_rows.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_map_redesign_or_bridge_rows.get(
                "target", {}
            ).get("envelope_bytes"),
            "decision_digest": fallback_projection_map_redesign_or_bridge_rows.get(
                "target", {}
            ).get("decision_digest"),
            "selected_path": fallback_projection_map_redesign_or_bridge_rows.get(
                "decision", {}
            ).get("selected_path"),
            "projection_columns": fallback_projection_map_redesign_or_bridge_rows.get(
                "coverage", {}
            ).get("projection_columns"),
            "lane_checks_specified": fallback_projection_map_redesign_or_bridge_rows.get(
                "coverage", {}
            ).get("lane_checks_specified"),
            "additional_public_lanes_required_to_cover_projection_space": fallback_projection_map_redesign_or_bridge_rows.get(
                "coverage", {}
            ).get("additional_public_lanes_required_to_cover_projection_space"),
            "open_bridge_slot_count": fallback_projection_map_redesign_or_bridge_rows.get(
                "coverage", {}
            ).get("open_bridge_slot_count"),
            "signature_byte_increase": fallback_projection_map_redesign_or_bridge_rows.get(
                "coverage", {}
            ).get("signature_byte_increase"),
            "can_accept_now": fallback_projection_map_redesign_or_bridge_rows.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_map_redesign_or_bridge_rows.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_verifier_exporter_contract": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.md"
                )
            ),
            "status": fallback_projection_verifier_exporter_contract.get("status"),
            "deployment_ready": fallback_projection_verifier_exporter_contract.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_verifier_exporter_contract.get(
                "target", {}
            ).get("envelope_bytes"),
            "contract_digest": fallback_projection_verifier_exporter_contract.get(
                "target", {}
            ).get("contract_digest"),
            "negative_test_count": fallback_projection_verifier_exporter_contract.get(
                "coverage", {}
            ).get("negative_test_count"),
            "routed_negative_test_count": fallback_projection_verifier_exporter_contract.get(
                "coverage", {}
            ).get("routed_negative_test_count"),
            "api_requirement_count": fallback_projection_verifier_exporter_contract.get(
                "coverage", {}
            ).get("api_requirement_count"),
            "implemented_api_requirement_count": fallback_projection_verifier_exporter_contract.get(
                "coverage", {}
            ).get("implemented_api_requirement_count"),
            "signature_byte_increase": fallback_projection_verifier_exporter_contract.get(
                "coverage", {}
            ).get("signature_byte_increase"),
            "can_accept_now": fallback_projection_verifier_exporter_contract.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_verifier_exporter_contract.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_verifier_exporter_implementation_gap": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.md"
                )
            ),
            "status": fallback_projection_verifier_exporter_implementation_gap.get("status"),
            "deployment_ready": fallback_projection_verifier_exporter_implementation_gap.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_verifier_exporter_implementation_gap.get(
                "target", {}
            ).get("envelope_bytes"),
            "implementation_gap_digest": fallback_projection_verifier_exporter_implementation_gap.get(
                "target", {}
            ).get("implementation_gap_digest"),
            "expected_symbol_count": fallback_projection_verifier_exporter_implementation_gap.get(
                "coverage", {}
            ).get("expected_symbol_count"),
            "missing_symbol_count": fallback_projection_verifier_exporter_implementation_gap.get(
                "coverage", {}
            ).get("missing_symbol_count"),
            "negative_test_count": fallback_projection_verifier_exporter_implementation_gap.get(
                "coverage", {}
            ).get("negative_test_count"),
            "open_implementation_task_count": fallback_projection_verifier_exporter_implementation_gap.get(
                "coverage", {}
            ).get("open_implementation_task_count"),
            "can_accept_now": fallback_projection_verifier_exporter_implementation_gap.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_verifier_exporter_implementation_gap.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_verifier_exporter_api_stubs": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.md"
                )
            ),
            "status": fallback_projection_verifier_exporter_api_stubs.get("status"),
            "deployment_ready": fallback_projection_verifier_exporter_api_stubs.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_verifier_exporter_api_stubs.get(
                "target", {}
            ).get("envelope_bytes"),
            "api_stub_digest": fallback_projection_verifier_exporter_api_stubs.get(
                "target", {}
            ).get("api_stub_digest"),
            "present_symbol_count": fallback_projection_verifier_exporter_api_stubs.get(
                "coverage", {}
            ).get("present_symbol_count"),
            "missing_symbol_count": fallback_projection_verifier_exporter_api_stubs.get(
                "coverage", {}
            ).get("missing_symbol_count"),
            "implemented_api_requirement_count": fallback_projection_verifier_exporter_api_stubs.get(
                "coverage", {}
            ).get("implemented_api_requirement_count"),
            "fail_closed_stub_api_count": fallback_projection_verifier_exporter_api_stubs.get(
                "coverage", {}
            ).get("fail_closed_stub_api_count"),
            "can_accept_now": fallback_projection_verifier_exporter_api_stubs.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_verifier_exporter_api_stubs.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_semantic_classifier_invocation_contract": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.md"
                )
            ),
            "status": fallback_projection_semantic_classifier_invocation_contract.get("status"),
            "deployment_ready": fallback_projection_semantic_classifier_invocation_contract.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_semantic_classifier_invocation_contract.get(
                "target", {}
            ).get("envelope_bytes"),
            "invocation_contract_digest": fallback_projection_semantic_classifier_invocation_contract.get(
                "target", {}
            ).get("invocation_contract_digest"),
            "invocation_rule_count": fallback_projection_semantic_classifier_invocation_contract.get(
                "coverage", {}
            ).get("invocation_rule_count"),
            "implemented_classifier_boundary_count": fallback_projection_semantic_classifier_invocation_contract.get(
                "coverage", {}
            ).get("implemented_classifier_boundary_count"),
            "implemented_semantic_predicate_count": fallback_projection_semantic_classifier_invocation_contract.get(
                "coverage", {}
            ).get("implemented_semantic_predicate_count"),
            "can_accept_now": fallback_projection_semantic_classifier_invocation_contract.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_semantic_classifier_invocation_contract.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_semantic_predicate_implementation_plan": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.md"
                )
            ),
            "status": fallback_projection_semantic_predicate_implementation_plan.get("status"),
            "deployment_ready": fallback_projection_semantic_predicate_implementation_plan.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_semantic_predicate_implementation_plan.get(
                "target", {}
            ).get("envelope_bytes"),
            "implementation_plan_digest": fallback_projection_semantic_predicate_implementation_plan.get(
                "target", {}
            ).get("implementation_plan_digest"),
            "semantic_predicate_count": fallback_projection_semantic_predicate_implementation_plan.get(
                "coverage", {}
            ).get("semantic_predicate_count"),
            "implemented_semantic_predicate_count": fallback_projection_semantic_predicate_implementation_plan.get(
                "coverage", {}
            ).get("implemented_semantic_predicate_count"),
            "open_semantic_predicate_count": fallback_projection_semantic_predicate_implementation_plan.get(
                "coverage", {}
            ).get("open_semantic_predicate_count"),
            "can_accept_now": fallback_projection_semantic_predicate_implementation_plan.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_semantic_predicate_implementation_plan.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_response_link_kernel_replay_boundary": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.md"
                )
            ),
            "status": fallback_projection_response_link_kernel_replay_boundary.get("status"),
            "deployment_ready": fallback_projection_response_link_kernel_replay_boundary.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_response_link_kernel_replay_boundary.get(
                "target", {}
            ).get("envelope_bytes"),
            "boundary_digest": fallback_projection_response_link_kernel_replay_boundary.get(
                "target", {}
            ).get("boundary_digest"),
            "implemented_semantic_predicate_count": fallback_projection_response_link_kernel_replay_boundary.get(
                "coverage", {}
            ).get("implemented_semantic_predicate_count"),
            "open_semantic_predicate_count": fallback_projection_response_link_kernel_replay_boundary.get(
                "coverage", {}
            ).get("open_semantic_predicate_count"),
            "can_accept_now": fallback_projection_response_link_kernel_replay_boundary.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_response_link_kernel_replay_boundary.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_opening_projection_boundary": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.md"
                )
            ),
            "status": fallback_projection_opening_projection_boundary.get("status"),
            "deployment_ready": fallback_projection_opening_projection_boundary.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_opening_projection_boundary.get(
                "target", {}
            ).get("envelope_bytes"),
            "boundary_digest": fallback_projection_opening_projection_boundary.get(
                "target", {}
            ).get("boundary_digest"),
            "implemented_semantic_predicate_count": fallback_projection_opening_projection_boundary.get(
                "coverage", {}
            ).get("implemented_semantic_predicate_count"),
            "open_semantic_predicate_count": fallback_projection_opening_projection_boundary.get(
                "coverage", {}
            ).get("open_semantic_predicate_count"),
            "can_accept_now": fallback_projection_opening_projection_boundary.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_opening_projection_boundary.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_account_commitment_split_boundary": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.md"
                )
            ),
            "status": fallback_projection_account_commitment_split_boundary.get("status"),
            "deployment_ready": fallback_projection_account_commitment_split_boundary.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_account_commitment_split_boundary.get(
                "target", {}
            ).get("envelope_bytes"),
            "boundary_digest": fallback_projection_account_commitment_split_boundary.get(
                "target", {}
            ).get("boundary_digest"),
            "implemented_semantic_predicate_count": fallback_projection_account_commitment_split_boundary.get(
                "coverage", {}
            ).get("implemented_semantic_predicate_count"),
            "open_semantic_predicate_count": fallback_projection_account_commitment_split_boundary.get(
                "coverage", {}
            ).get("open_semantic_predicate_count"),
            "can_accept_now": fallback_projection_account_commitment_split_boundary.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_account_commitment_split_boundary.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_pad_prf_projection_boundary": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.md"
                )
            ),
            "status": fallback_projection_pad_prf_projection_boundary.get("status"),
            "deployment_ready": fallback_projection_pad_prf_projection_boundary.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_pad_prf_projection_boundary.get(
                "target", {}
            ).get("envelope_bytes"),
            "boundary_digest": fallback_projection_pad_prf_projection_boundary.get(
                "target", {}
            ).get("boundary_digest"),
            "implemented_semantic_predicate_count": fallback_projection_pad_prf_projection_boundary.get(
                "coverage", {}
            ).get("implemented_semantic_predicate_count"),
            "open_semantic_predicate_count": fallback_projection_pad_prf_projection_boundary.get(
                "coverage", {}
            ).get("open_semantic_predicate_count"),
            "can_accept_now": fallback_projection_pad_prf_projection_boundary.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_pad_prf_projection_boundary.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_byte_level_derivation_internalization": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.md"
                )
            ),
            "status": fallback_projection_byte_level_derivation_internalization.get(
                "status"
            ),
            "deployment_ready": fallback_projection_byte_level_derivation_internalization.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_byte_level_derivation_internalization.get(
                "target", {}
            ).get("envelope_bytes"),
            "internalization_digest": fallback_projection_byte_level_derivation_internalization.get(
                "target", {}
            ).get("internalization_digest"),
            "internalized_derivation_count": fallback_projection_byte_level_derivation_internalization.get(
                "coverage", {}
            ).get("internalized_derivation_count"),
            "open_derivation_count": fallback_projection_byte_level_derivation_internalization.get(
                "coverage", {}
            ).get("open_derivation_count"),
            "can_accept_now": fallback_projection_byte_level_derivation_internalization.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_byte_level_derivation_internalization.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.md"
                )
            ),
            "status": fallback_projection_exact_bridge_certificate_and_leakage_proof.get(
                "status"
            ),
            "deployment_ready": fallback_projection_exact_bridge_certificate_and_leakage_proof.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_exact_bridge_certificate_and_leakage_proof.get(
                "target", {}
            ).get("envelope_bytes"),
            "proof_digest": fallback_projection_exact_bridge_certificate_and_leakage_proof.get(
                "target", {}
            ).get("proof_digest"),
            "closed_row_count": fallback_projection_exact_bridge_certificate_and_leakage_proof.get(
                "coverage", {}
            ).get("closed_row_count"),
            "open_row_count": fallback_projection_exact_bridge_certificate_and_leakage_proof.get(
                "coverage", {}
            ).get("open_row_count"),
            "leakage_accounted": fallback_projection_exact_bridge_certificate_and_leakage_proof.get(
                "coverage", {}
            ).get("leakage_accounted"),
            "exact_certificate_complete": fallback_projection_exact_bridge_certificate_and_leakage_proof.get(
                "coverage", {}
            ).get("exact_certificate_complete"),
            "can_accept_now": fallback_projection_exact_bridge_certificate_and_leakage_proof.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_exact_bridge_certificate_and_leakage_proof.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.md"
                )
            ),
            "status": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "status"
            ),
            "deployment_ready": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "target", {}
            ).get("envelope_bytes"),
            "closure_digest": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "target", {}
            ).get("closure_digest"),
            "closed_row_count": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "coverage", {}
            ).get("closed_row_count"),
            "open_row_count": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "coverage", {}
            ).get("open_row_count"),
            "present_symbol_count": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "coverage", {}
            ).get("present_symbol_count"),
            "required_symbol_count": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "coverage", {}
            ).get("required_symbol_count"),
            "exact_certificate_complete": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "coverage", {}
            ).get("exact_certificate_complete"),
            "bad_vector_exported": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "coverage", {}
            ).get("bad_vector_exported"),
            "can_accept_now": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_verifier_exporter_bridge_semantics_closure.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_fallback_projection_active_subspace_exact_replay": {
            "artifact": rel(
                artifact(
                    "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.md"
                )
            ),
            "status": fallback_projection_active_subspace_exact_replay.get("status"),
            "deployment_ready": fallback_projection_active_subspace_exact_replay.get(
                "deployment_ready"
            ),
            "envelope_bytes": fallback_projection_active_subspace_exact_replay.get(
                "target", {}
            ).get("envelope_bytes"),
            "active_subspace_digest": fallback_projection_active_subspace_exact_replay.get(
                "target", {}
            ).get("active_subspace_digest"),
            "active_projection_lanes": fallback_projection_active_subspace_exact_replay.get(
                "coverage", {}
            ).get("active_projection_lanes"),
            "old_projection_columns": fallback_projection_active_subspace_exact_replay.get(
                "coverage", {}
            ).get("old_projection_columns"),
            "old_inactive_projection_columns": fallback_projection_active_subspace_exact_replay.get(
                "coverage", {}
            ).get("old_inactive_projection_columns"),
            "survivor_in_active_subspace": fallback_projection_active_subspace_exact_replay.get(
                "coverage", {}
            ).get("survivor_in_active_subspace"),
            "exact_certificate_complete_for_active_subspace": fallback_projection_active_subspace_exact_replay.get(
                "coverage", {}
            ).get("exact_certificate_complete_for_active_subspace"),
            "can_accept_now": fallback_projection_active_subspace_exact_replay.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": fallback_projection_active_subspace_exact_replay.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_ro_pok_external_review_packet": {
            "artifact": rel(
                artifact("hidden_source_ro_pok_external_review_packet_n32_q12289.json")
            ),
            "report": rel(
                artifact("hidden_source_ro_pok_external_review_packet_n32_q12289.md")
            ),
            "status": ro_pok_external_review_packet.get("status"),
            "deployment_ready": ro_pok_external_review_packet.get("deployment_ready"),
            "review_question_count": len(
                ro_pok_external_review_packet.get("review_questions", [])
            ),
            "red_flag_count": len(ro_pok_external_review_packet.get("red_flags", [])),
            "external_review_required": ro_pok_external_review_packet.get(
                "decision", {}
            ).get("external_review_required"),
            "can_accept_now": ro_pok_external_review_packet.get("decision", {}).get(
                "can_accept_now"
            ),
            "next_artifact_if_rejected": ro_pok_external_review_packet.get(
                "decision", {}
            ).get("next_artifact_if_rejected"),
        },
        "hidden_source_ro_pok_leakage_global_composition_theorem": {
            "artifact": rel(
                artifact(
                    "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.json"
                )
            ),
            "report": rel(
                artifact(
                    "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.md"
                )
            ),
            "status": ro_pok_leakage_global_composition.get("status"),
            "deployment_ready": ro_pok_leakage_global_composition.get(
                "deployment_ready"
            ),
            "leakage_channel_count": len(
                ro_pok_leakage_global_composition.get("leakage_channels", [])
            ),
            "composition_edge_count": len(
                ro_pok_leakage_global_composition.get("composition_edges", [])
            ),
            "proof_obligation_count": len(
                ro_pok_leakage_global_composition.get("proof_obligations", [])
            ),
            "named_break_count": len(
                ro_pok_leakage_global_composition.get("named_breaks", [])
            ),
            "can_accept_now": ro_pok_leakage_global_composition.get(
                "decision", {}
            ).get("can_accept_now"),
            "next_artifact": ro_pok_leakage_global_composition.get(
                "decision", {}
            ).get("next_artifact"),
        },
        "hidden_source_ro_pok_leakage_simulator": {
            "artifact": rel(
                artifact("hidden_source_ro_pok_leakage_simulator_n32_q12289.json")
            ),
            "report": rel(
                artifact("hidden_source_ro_pok_leakage_simulator_n32_q12289.md")
            ),
            "status": ro_pok_leakage_simulator.get("status"),
            "deployment_ready": ro_pok_leakage_simulator.get("deployment_ready"),
            "public_input_count": len(ro_pok_leakage_simulator.get("public_inputs", [])),
            "simulator_step_count": len(
                ro_pok_leakage_simulator.get("simulator_steps", [])
            ),
            "indistinguishability_claim_count": len(
                ro_pok_leakage_simulator.get("indistinguishability_claims", [])
            ),
            "required_test_count": len(
                ro_pok_leakage_simulator.get("required_tests", [])
            ),
            "open_gap_count": len(ro_pok_leakage_simulator.get("open_gaps", [])),
            "can_accept_now": ro_pok_leakage_simulator.get("decision", {}).get(
                "can_accept_now"
            ),
            "next_artifact": ro_pok_leakage_simulator.get("decision", {}).get(
                "next_artifact"
            ),
        },
        "red_team": {
            "artifact": rel(artifact("red_team_cryptoanalysis_n32_q12289.json")),
            "verdict": red_team.get("verdict"),
            "finding_classes": [
                item.get("class") for item in red_team.get("findings", [])
            ],
        },
        "assumption_closure": {
            "artifact": rel(artifact("assumption_red_team_closure_n32_q12289.json")),
            "summary": assumption_closure.get("summary"),
            "closure_states": {
                row["assumption_id"]: row["closure_state"]
                for row in assumption_closure.get("rows", [])
            },
        },
        "closed_sidecars": {
            "quotient_level_link": {
                "classification": quotient_link_gate["final_decision"][
                    "classification"
                ],
                "decision": quotient_link_gate["final_decision"]["decision"],
                "migration_plan_required": quotient_link_gate["final_decision"][
                    "migration_plan_required"
                ],
                "firebreak_passed": quotient_link_firebreak["passed"],
                "firebreak_violations": quotient_link_firebreak["violations"],
                "promotion_gate": rel(
                    artifact("quotient_level_link_promotion_gate.json")
                ),
                "handoff": rel(
                    artifact("quotient_level_link_implementation_handoff.md")
                ),
                "closure_verifier": "scripts/research/verify_quotient_level_link_closure.py",
            }
        },
        "external_review_items": external_review_items,
        "terminal_blockers": preflight.get("blockers", []),
        "primary_artifacts": [
            rel(artifact("third_party_review_packet.md")),
            rel(artifact("third_party_review_manifest_n32_q12289.md")),
            rel(artifact("third_party_review_packet_finalization_n32_q12289.md")),
            rel(artifact("red_team_cryptoanalysis_n32_q12289.md")),
            rel(artifact("assumption_red_team_closure_n32_q12289.md")),
            rel(COST_MODEL_DIR / "committed_sigma_account_carried_cost_model.md"),
            rel(artifact("conditional_soundness_budget_review_n32_q12289.md")),
            rel(artifact("production_reference_gap_report_n32_q12289.md")),
            rel(artifact("production_preflight_certificate_verification_n32_q12289.md")),
            rel(
                artifact(
                    "selected_deployment_terminology_verification_n32_q12289.md"
                )
            ),
            rel(artifact("compact_signature_todos_consistency_n32_q12289.md")),
            rel(artifact("reviewed_equation_spec_gap_n32_q12289.md")),
            rel(artifact("reviewed_challenge_action_options_n32_q12289.md")),
            rel(artifact("hidden_source_proof_semantics_target_n32_q12289.md")),
            rel(artifact("hidden_source_accepting_relation_spec_n32_q12289.md")),
            rel(artifact("hidden_source_carrier_construction_decision_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_carrier_relation_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_extractor_obligations_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_negative_controls_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_opening_frame_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_account_state_schema_contract_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_masked_projection_global_soundness_n32_q12289.md")),
            rel(artifact("pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.md")),
            rel(artifact("pre_signature_projection_mask_prf_ro_reduction_n32_q12289.md")),
            rel(artifact("pre_signature_projection_lifecycle_same_game_lift_n32_q12289.md")),
            rel(artifact("pre_signature_projection_spend_time_leakage_budget_n32_q12289.md")),
            rel(artifact("pre_signature_projection_reject_equivalence_n32_q12289.md")),
            rel(artifact("pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.md")),
            rel(artifact("pre_signature_projection_sha256_mask_prf_assumption_resolution_n32_q12289.md")),
            rel(artifact("pre_signature_projection_global_domain_registry_n32_q12289.md")),
            rel(artifact("pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.md")),
            rel(artifact("pre_signature_projection_spend_time_budget_discharge_n32_q12289.md")),
            rel(artifact("pre_signature_projection_reject_equivalence_discharge_n32_q12289.md")),
            rel(artifact("pre_signature_projection_helper_api_export_boundary_n32_q12289.md")),
            rel(artifact("pre_signature_projection_helper_api_export_boundary_closure_n32_q12289.md")),
            rel(artifact("pre_signature_projection_no_export_negative_tests_n32_q12289.md")),
            rel(artifact("pre_signature_projection_no_export_negative_tests_closure_n32_q12289.md")),
            rel(artifact("pre_signature_projection_req_pad_reject_discharge_n32_q12289.md")),
            rel(artifact("pre_signature_projection_req_pad_reject_closure_n32_q12289.md")),
            rel(artifact("pre_signature_projection_req_witness_reject_discharge_n32_q12289.md")),
            rel(artifact("pre_signature_projection_req_witness_reject_closure_n32_q12289.md")),
            rel(artifact("pre_signature_projection_req_carrier_reject_discharge_n32_q12289.md")),
            rel(artifact("pre_signature_projection_req_carrier_reject_closure_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.md")),
            rel(artifact("hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.md")),
            rel(artifact("quotient_level_link_promotion_gate.md")),
            rel(artifact("quotient_level_link_implementation_handoff.md")),
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    verdict = payload["verdict"]
    target = payload["target"]
    replay = payload["reproducibility"]
    gates = payload["review_gates"]
    preflight = payload["deployment_preflight"]
    preflight_certificate = payload["preflight_certificate"]
    terminology = payload["selected_deployment_terminology"]
    todo_consistency = payload["todo_consistency"]
    red_team = payload["red_team"]
    closure = payload["assumption_closure"]
    closed_sidecars = payload["closed_sidecars"]
    equation_gap = payload["reviewed_equation_spec"]
    action_options = payload["reviewed_challenge_action_options"]
    hidden_source_target = payload["hidden_source_proof_semantics_target"]
    hidden_source_relation = payload["hidden_source_accepting_relation_spec"]
    hidden_source_carrier = payload["hidden_source_carrier_construction_decision"]
    state_assisted_carrier = payload["hidden_source_state_assisted_carrier_relation"]
    state_assisted_extractor = payload[
        "hidden_source_state_assisted_extractor_obligations"
    ]
    state_assisted_negative_controls = payload[
        "hidden_source_state_assisted_negative_controls"
    ]
    state_assisted_opening_frame = payload[
        "hidden_source_state_assisted_opening_frame"
    ]
    state_assisted_opening_frame_test_vectors = payload[
        "hidden_source_state_assisted_opening_frame_test_vectors"
    ]
    state_assisted_carrier_verifier_contract = payload[
        "hidden_source_state_assisted_carrier_verifier_contract"
    ]
    state_assisted_account_state_schema = payload[
        "hidden_source_state_assisted_account_state_schema_contract"
    ]
    state_assisted_hidden_relation_verifier = payload[
        "hidden_source_state_assisted_hidden_relation_verifier_contract"
    ]
    state_assisted_hidden_relation_negative_controls = payload[
        "hidden_source_state_assisted_hidden_relation_negative_controls"
    ]
    state_assisted_hidden_relation_extractor_theorem = payload[
        "hidden_source_state_assisted_hidden_relation_extractor_theorem"
    ]
    state_assisted_masked_projection_global_soundness = payload[
        "hidden_source_state_assisted_masked_projection_global_soundness"
    ]
    pre_signature_projection_mask_prf_leakage_simulator = payload[
        "pre_signature_projection_mask_prf_leakage_simulator"
    ]
    pre_signature_projection_mask_prf_ro_reduction = payload[
        "pre_signature_projection_mask_prf_ro_reduction"
    ]
    pre_signature_projection_lifecycle_same_game_lift = payload[
        "pre_signature_projection_lifecycle_same_game_lift"
    ]
    pre_signature_projection_spend_time_leakage_budget = payload[
        "pre_signature_projection_spend_time_leakage_budget"
    ]
    pre_signature_projection_reject_equivalence = payload[
        "pre_signature_projection_reject_equivalence"
    ]
    pre_signature_projection_mask_prf_leakage_theorem_composition = payload[
        "pre_signature_projection_mask_prf_leakage_theorem_composition"
    ]
    pre_signature_projection_sha256_mask_prf_assumption_resolution = payload[
        "pre_signature_projection_sha256_mask_prf_assumption_resolution"
    ]
    pre_signature_projection_global_domain_registry = payload[
        "pre_signature_projection_global_domain_registry"
    ]
    pre_signature_projection_lifecycle_same_game_discharge = payload[
        "pre_signature_projection_lifecycle_same_game_discharge"
    ]
    pre_signature_projection_spend_time_budget_discharge = payload[
        "pre_signature_projection_spend_time_budget_discharge"
    ]
    pre_signature_projection_reject_equivalence_discharge = payload[
        "pre_signature_projection_reject_equivalence_discharge"
    ]
    pre_signature_projection_helper_api_export_boundary = payload[
        "pre_signature_projection_helper_api_export_boundary"
    ]
    pre_signature_projection_helper_api_export_boundary_closure = payload[
        "pre_signature_projection_helper_api_export_boundary_closure"
    ]
    pre_signature_projection_no_export_negative_tests = payload[
        "pre_signature_projection_no_export_negative_tests"
    ]
    pre_signature_projection_no_export_negative_tests_closure = payload[
        "pre_signature_projection_no_export_negative_tests_closure"
    ]
    pre_signature_projection_req_pad_reject_discharge = payload[
        "pre_signature_projection_req_pad_reject_discharge"
    ]
    pre_signature_projection_req_pad_reject_closure = payload[
        "pre_signature_projection_req_pad_reject_closure"
    ]
    pre_signature_projection_req_witness_reject_discharge = payload[
        "pre_signature_projection_req_witness_reject_discharge"
    ]
    pre_signature_projection_req_witness_reject_closure = payload[
        "pre_signature_projection_req_witness_reject_closure"
    ]
    pre_signature_projection_req_carrier_reject_discharge = payload[
        "pre_signature_projection_req_carrier_reject_discharge"
    ]
    pre_signature_projection_req_carrier_reject_closure = payload[
        "pre_signature_projection_req_carrier_reject_closure"
    ]
    state_assisted_compact_proof_instantiation = payload[
        "hidden_source_state_assisted_compact_proof_instantiation"
    ]
    state_assisted_ro_pok_assumption_api = payload[
        "hidden_source_state_assisted_ro_pok_assumption_api"
    ]
    state_assisted_ro_pok_byte_negative_tests = payload[
        "hidden_source_state_assisted_ro_pok_byte_negative_tests"
    ]
    lines = [
        "# Third-Party Review Status Index",
        "",
        "This is the reviewer landing page for the local q12289 account-carried",
        "signature packet.  It summarizes what the local artifacts establish and",
        "what remains outside the current proof/implementation envelope.",
        "",
        "## Verdict",
        "",
        f"- internal packet reproducible: `{verdict['internal_packet_reproducible']}`",
        f"- deployment ready: `{verdict['deployment_ready']}`",
        f"- external review required: `{verdict['external_review_required']}`",
        f"- headline: {verdict['headline']}",
        "",
        "## Target",
        "",
        f"- n: `{target['n']}`",
        f"- q: `{target['q']}`",
        f"- transaction bytes: `{target['transaction_bytes']}`",
        f"- account-link bytes: `{target['account_link_bytes']}`",
        f"- online plus account bytes: `{target['online_plus_account_bytes']}`",
        f"- public-key bytes: `{target['public_key_bytes']}`",
        f"- current executable signature bytes: `{target['executable_signature_bytes']}`",
        "",
        "## Reproducibility",
        "",
        "```bash",
        replay["recommended_command"],
        "```",
        "",
        f"- replay status: `{replay['replay_status']}`",
        f"- exact response-link replay: `{replay['replay_exact']}`",
        f"- failed replay steps: `{replay['failed_steps']}`",
        f"- invalid JSON artifacts: `{replay['invalid_json_artifacts']}`",
        f"- hashed JSON artifacts: `{replay['hashed_json_artifacts']}`",
        "",
        "## Deployment Preflight Certificate",
        "",
        f"- export status: `{preflight_certificate['export_status']}`",
        f"- certificate digest: `{preflight_certificate['certificate_digest']}`",
        f"- deployment ready: `{preflight_certificate['deployment_ready']}`",
        f"- blocker count: `{preflight_certificate['blocker_count']}`",
        f"- primary blocker: {preflight_certificate['primary_blocker']}",
        f"- preflight report digest: `{preflight_certificate['preflight_report_digest']}`",
        f"- blocker report digest: `{preflight_certificate['blocker_report_digest']}`",
        f"- verdict digest: `{preflight_certificate['verdict_digest']}`",
        f"- ready assertion digest: `{preflight_certificate['ready_assertion_digest']}`",
        f"- cross checks: `{preflight_certificate['cross_checks']}`",
        f"- verification JSON: `{preflight_certificate['verification_artifact_json']}`",
        f"- verification report: `{preflight_certificate['verification_artifact_md']}`",
        "",
        "## Selected Deployment Terminology Guard",
        "",
        f"- status: `{terminology['status']}`",
        f"- files checked: `{terminology['files_checked']}`",
        f"- failures: `{terminology['failure_count']}`",
        f"- missing required phrases: `{terminology['missing_required_phrase_count']}`",
        f"- verification JSON: `{terminology['artifact_json']}`",
        f"- verification report: `{terminology['artifact_md']}`",
        f"- forbidden patterns: `{terminology['forbidden_patterns']}`",
        "",
        "## TODO Consistency Guard",
        "",
        f"- status: `{todo_consistency['status']}`",
        f"- replay steps: `{todo_consistency['replay_step_count']}`",
        f"- hashed JSON artifacts: `{todo_consistency['hashed_json_artifact_count']}`",
        f"- manifest files: `{todo_consistency['manifest_file_count']}`",
        f"- forbidden pattern failures: `{todo_consistency['forbidden_pattern_failure_count']}`",
        f"- missing required snippets: `{todo_consistency['missing_required_snippet_count']}`",
        f"- verification JSON: `{todo_consistency['artifact_json']}`",
        f"- verification report: `{todo_consistency['artifact_md']}`",
        "",
        "## Review Gates",
        "",
        f"- total cost-model review gates: `{gates['total']}`",
        f"- status counts: `{gates['status_counts']}`",
        f"- assumptions mapped: `{gates['assumptions_mapped']}`",
        f"- open external assumption items: `{gates['open_external_items']}`",
        f"- deployment-ready assumption rows: `{gates['deployment_ready_rows']}`",
        "",
        "| gate | status | artifact | summary |",
        "|---|---|---|---|",
    ]
    for row in gates["rows"]:
        lines.append(
            f"| {row['gate']} | {row['status']} | `{row.get('artifact') or '-'}` | "
            f"{row.get('summary') or '-'} |"
        )
    lines.extend(
        [
            "",
            "## Gate Metrics",
            "",
        ]
    )
    for row in gates["rows"]:
        metrics = row.get("key_metrics") or {}
        metrics_text = json.dumps(metrics, sort_keys=True)
        lines.append(f"- `{row['gate']}`: `{metrics_text}`")
    lines.extend(
        [
            "",
            "## Red-Team Findings",
            "",
            f"- artifact: `{red_team['artifact']}`",
            f"- selected target break found: `{red_team['verdict']['new_break_against_selected_250b_target_found']}`",
            f"- bare quotient confirmed broken: `{red_team['verdict']['bare_public_quotient_confirmed_broken']}`",
            f"- bad alternate two-coordinate codes found: `{red_team['verdict']['bad_alternate_two_coordinate_codes_found']}`",
            f"- finding classes: `{red_team['finding_classes']}`",
            "",
            "## Assumption Closure",
            "",
            f"- artifact: `{closure['artifact']}`",
            f"- deployment ready: `{closure['summary']['deployment_ready']}`",
            f"- deployment-blocking open rows: `{closure['summary']['deployment_blocking_open_rows']}`",
            f"- closed negative controls: `{closure['summary']['closed_negative_control_rows']}`",
            f"- closure states: `{closure['closure_states']}`",
            "",
            "## Closed Sidecars",
            "",
            f"- quotient-level link classification: `{closed_sidecars['quotient_level_link']['classification']}`",
            f"- quotient-level link decision: `{closed_sidecars['quotient_level_link']['decision']}`",
            f"- migration plan required: `{closed_sidecars['quotient_level_link']['migration_plan_required']}`",
            f"- import firebreak passed: `{closed_sidecars['quotient_level_link']['firebreak_passed']}`",
            f"- promotion gate: `{closed_sidecars['quotient_level_link']['promotion_gate']}`",
            f"- implementation handoff: `{closed_sidecars['quotient_level_link']['handoff']}`",
            f"- closure verifier: `{closed_sidecars['quotient_level_link']['closure_verifier']}`",
            "",
            "## Reviewed Equation Spec",
            "",
            f"- artifact: `{equation_gap['artifact']}`",
            f"- status: `{equation_gap['status']}`",
            f"- deployment ready: `{equation_gap['deployment_ready']}`",
            f"- findings: `{[finding.get('finding') for finding in equation_gap['findings']]}`",
            f"- challenge-action recommendation: `{action_options['recommended']}`",
            f"- recommended transaction bytes: `{action_options['recommended_transaction_bytes']}`",
            f"- exact 250-byte path requires: {action_options['exact_250b_requires_new_argument']}",
            "",
            "## Hidden-Source Proof Semantics Target",
            "",
            f"- artifact: `{hidden_source_target['artifact']}`",
            f"- report: `{hidden_source_target['report']}`",
            f"- status: `{hidden_source_target['status']}`",
            f"- deployment ready: `{hidden_source_target['deployment_ready']}`",
            f"- current envelope bytes: `{hidden_source_target['current_envelope_bytes']}`",
            f"- carrier options: `{hidden_source_target['carrier_options']}`",
            f"- open obligations: `{hidden_source_target['open_obligations']}`",
            f"- decision gates: `{hidden_source_target['decision_gates']}`",
            "",
            "## Hidden-Source Accepting Relation Spec",
            "",
            f"- artifact: `{hidden_source_relation['artifact']}`",
            f"- report: `{hidden_source_relation['report']}`",
            f"- status: `{hidden_source_relation['status']}`",
            f"- deployment ready: `{hidden_source_relation['deployment_ready']}`",
            f"- envelope bytes: `{hidden_source_relation['envelope_bytes']}`",
            f"- verifier phases: `{hidden_source_relation['verifier_phases']}`",
            f"- witness predicates: `{hidden_source_relation['witness_predicates']}`",
            f"- theorem hooks: `{hidden_source_relation['theorem_hooks']}`",
            f"- negative controls required: `{hidden_source_relation['negative_control_count']}`",
            "",
            "## Hidden-Source Carrier Construction Decision",
            "",
            f"- artifact: `{hidden_source_carrier['artifact']}`",
            f"- report: `{hidden_source_carrier['report']}`",
            f"- status: `{hidden_source_carrier['status']}`",
            f"- deployment ready: `{hidden_source_carrier['deployment_ready']}`",
            f"- selected candidate: `{hidden_source_carrier['selected_candidate']}`",
            f"- selected envelope bytes: `{hidden_source_carrier['selected_envelope_bytes']}`",
            f"- candidate count: `{hidden_source_carrier['candidate_count']}`",
            f"- open decision gates: `{hidden_source_carrier['open_gate_count']}`",
            f"- required modeling work: `{hidden_source_carrier['required_modeling_work']}`",
            "",
            "## Hidden-Source State-Assisted Carrier Relation",
            "",
            f"- artifact: `{state_assisted_carrier['artifact']}`",
            f"- report: `{state_assisted_carrier['report']}`",
            f"- status: `{state_assisted_carrier['status']}`",
            f"- deployment ready: `{state_assisted_carrier['deployment_ready']}`",
            f"- carrier bytes: `{state_assisted_carrier['carrier_bytes']}`",
            f"- envelope bytes: `{state_assisted_carrier['envelope_bytes']}`",
            f"- state creation fields: `{state_assisted_carrier['state_creation_fields']}`",
            f"- carrier predicates: `{state_assisted_carrier['carrier_predicates']}`",
            f"- theorem cases: `{state_assisted_carrier['theorem_cases']}`",
            f"- negative controls required: `{state_assisted_carrier['negative_control_count']}`",
            f"- can accept now: `{state_assisted_carrier['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted Extractor Obligations",
            "",
            f"- artifact: `{state_assisted_extractor['artifact']}`",
            f"- report: `{state_assisted_extractor['report']}`",
            f"- status: `{state_assisted_extractor['status']}`",
            f"- deployment ready: `{state_assisted_extractor['deployment_ready']}`",
            f"- carrier bytes: `{state_assisted_extractor['carrier_bytes']}`",
            f"- envelope bytes: `{state_assisted_extractor['envelope_bytes']}`",
            f"- obligation count: `{state_assisted_extractor['obligation_count']}`",
            f"- named assumption count: `{state_assisted_extractor['named_assumption_count']}`",
            f"- fallback trigger count: `{state_assisted_extractor['fallback_trigger_count']}`",
            f"- can accept now: `{state_assisted_extractor['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted Negative Controls",
            "",
            f"- artifact: `{state_assisted_negative_controls['artifact']}`",
            f"- report: `{state_assisted_negative_controls['report']}`",
            f"- status: `{state_assisted_negative_controls['status']}`",
            f"- deployment ready: `{state_assisted_negative_controls['deployment_ready']}`",
            f"- control count: `{state_assisted_negative_controls['control_count']}`",
            f"- coverage summary: `{state_assisted_negative_controls['coverage_summary']}`",
            f"- open control count: `{state_assisted_negative_controls['open_control_count']}`",
            f"- partial control count: `{state_assisted_negative_controls['partial_control_count']}`",
            f"- can accept now: `{state_assisted_negative_controls['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted Opening Frame",
            "",
            f"- artifact: `{state_assisted_opening_frame['artifact']}`",
            f"- report: `{state_assisted_opening_frame['report']}`",
            f"- status: `{state_assisted_opening_frame['status']}`",
            f"- deployment ready: `{state_assisted_opening_frame['deployment_ready']}`",
            f"- carrier bytes: `{state_assisted_opening_frame['carrier_bytes']}`",
            f"- envelope bytes: `{state_assisted_opening_frame['envelope_bytes']}`",
            f"- hash assumption: {state_assisted_opening_frame['hash_assumption']}",
            f"- opening public/private frame bytes: `{state_assisted_opening_frame['opening_public_frame_bytes']}` / `{state_assisted_opening_frame['opening_private_frame_bytes']}`",
            f"- carrier public/private frame bytes: `{state_assisted_opening_frame['carrier_public_frame_bytes']}` / `{state_assisted_opening_frame['carrier_private_frame_bytes']}`",
            f"- rejection rule count: `{state_assisted_opening_frame['rejection_rule_count']}`",
            f"- test vector count: `{state_assisted_opening_frame['test_vector_count']}`",
            f"- can accept now: `{state_assisted_opening_frame['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted Opening-Frame Test Vectors",
            "",
            f"- artifact: `{state_assisted_opening_frame_test_vectors['artifact']}`",
            f"- report: `{state_assisted_opening_frame_test_vectors['report']}`",
            f"- status: `{state_assisted_opening_frame_test_vectors['status']}`",
            f"- deployment ready: `{state_assisted_opening_frame_test_vectors['deployment_ready']}`",
            f"- opening frame bytes: `{state_assisted_opening_frame_test_vectors['opening_frame_bytes']}`",
            f"- carrier frame bytes: `{state_assisted_opening_frame_test_vectors['carrier_frame_bytes']}`",
            f"- mutation vector count: `{state_assisted_opening_frame_test_vectors['mutation_vector_count']}`",
            f"- all mutations change digest: `{state_assisted_opening_frame_test_vectors['all_mutations_change_digest']}`",
            f"- can accept now: `{state_assisted_opening_frame_test_vectors['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted Carrier Verifier Contract",
            "",
            f"- artifact: `{state_assisted_carrier_verifier_contract['artifact']}`",
            f"- report: `{state_assisted_carrier_verifier_contract['report']}`",
            f"- status: `{state_assisted_carrier_verifier_contract['status']}`",
            f"- deployment ready: `{state_assisted_carrier_verifier_contract['deployment_ready']}`",
            f"- opening frame bytes: `{state_assisted_carrier_verifier_contract['opening_frame_bytes']}`",
            f"- carrier frame bytes: `{state_assisted_carrier_verifier_contract['carrier_frame_bytes']}`",
            f"- public/state/witness inputs: `{state_assisted_carrier_verifier_contract['public_input_count']}` / `{state_assisted_carrier_verifier_contract['state_input_count']}` / `{state_assisted_carrier_verifier_contract['witness_input_count']}`",
            f"- verifier phases: `{state_assisted_carrier_verifier_contract['phase_count']}`",
            f"- blocker count: `{state_assisted_carrier_verifier_contract['blocker_count']}`",
            f"- can accept now: `{state_assisted_carrier_verifier_contract['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted Account-State Schema Contract",
            "",
            f"- artifact: `{state_assisted_account_state_schema['artifact']}`",
            f"- report: `{state_assisted_account_state_schema['report']}`",
            f"- status: `{state_assisted_account_state_schema['status']}`",
            f"- deployment ready: `{state_assisted_account_state_schema['deployment_ready']}`",
            f"- slot field count: `{state_assisted_account_state_schema['slot_field_count']}`",
            f"- Rust symbol count: `{state_assisted_account_state_schema['rust_symbol_count']}`",
            f"- negative control count: `{state_assisted_account_state_schema['negative_control_count']}`",
            f"- blocker count: `{state_assisted_account_state_schema['blocker_count']}`",
            f"- can accept now: `{state_assisted_account_state_schema['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted Hidden-Relation Verifier Contract",
            "",
            f"- artifact: `{state_assisted_hidden_relation_verifier['artifact']}`",
            f"- report: `{state_assisted_hidden_relation_verifier['report']}`",
            f"- status: `{state_assisted_hidden_relation_verifier['status']}`",
            f"- deployment ready: `{state_assisted_hidden_relation_verifier['deployment_ready']}`",
            f"- public statement fields: `{state_assisted_hidden_relation_verifier['public_statement_field_count']}`",
            f"- private opening fields: `{state_assisted_hidden_relation_verifier['private_opening_field_count']}`",
            f"- verifier checks: `{state_assisted_hidden_relation_verifier['verifier_check_count']}`",
            f"- extraction cases: `{state_assisted_hidden_relation_verifier['extraction_case_count']}`",
            f"- implementation targets: `{state_assisted_hidden_relation_verifier['implementation_target_count']}`",
            f"- blocker count: `{state_assisted_hidden_relation_verifier['blocker_count']}`",
            f"- can accept now: `{state_assisted_hidden_relation_verifier['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted Hidden-Relation Negative Controls",
            "",
            f"- artifact: `{state_assisted_hidden_relation_negative_controls['artifact']}`",
            f"- report: `{state_assisted_hidden_relation_negative_controls['report']}`",
            f"- status: `{state_assisted_hidden_relation_negative_controls['status']}`",
            f"- deployment ready: `{state_assisted_hidden_relation_negative_controls['deployment_ready']}`",
            f"- control count: `{state_assisted_hidden_relation_negative_controls['control_count']}`",
            f"- coverage summary: `{state_assisted_hidden_relation_negative_controls['coverage_summary']}`",
            f"- theorem requirements: `{state_assisted_hidden_relation_negative_controls['theorem_requirement_count']}`",
            f"- can accept now: `{state_assisted_hidden_relation_negative_controls['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted Hidden-Relation Extractor Theorem",
            "",
            f"- artifact: `{state_assisted_hidden_relation_extractor_theorem['artifact']}`",
            f"- report: `{state_assisted_hidden_relation_extractor_theorem['report']}`",
            f"- status: `{state_assisted_hidden_relation_extractor_theorem['status']}`",
            f"- deployment ready: `{state_assisted_hidden_relation_extractor_theorem['deployment_ready']}`",
            f"- theorem name: `{state_assisted_hidden_relation_extractor_theorem['theorem_name']}`",
            f"- hypotheses: `{state_assisted_hidden_relation_extractor_theorem['hypothesis_count']}`",
            f"- extractor outputs: `{state_assisted_hidden_relation_extractor_theorem['extractor_output_count']}`",
            f"- named breaks: `{state_assisted_hidden_relation_extractor_theorem['named_break_count']}`",
            f"- proof steps: `{state_assisted_hidden_relation_extractor_theorem['proof_step_count']}`",
            f"- open obligations: `{state_assisted_hidden_relation_extractor_theorem['open_obligation_count']}`",
            f"- can accept now: `{state_assisted_hidden_relation_extractor_theorem['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted Masked Projection Global Soundness",
            "",
            f"- artifact: `{state_assisted_masked_projection_global_soundness['artifact']}`",
            f"- report: `{state_assisted_masked_projection_global_soundness['report']}`",
            f"- status: `{state_assisted_masked_projection_global_soundness['status']}`",
            f"- deployment ready: `{state_assisted_masked_projection_global_soundness['deployment_ready']}`",
            f"- theorem name: `{state_assisted_masked_projection_global_soundness['theorem_name']}`",
            f"- implemented local hops: `{state_assisted_masked_projection_global_soundness['implemented_local_hops']}`",
            f"- open global hops: `{state_assisted_masked_projection_global_soundness['open_global_hops']}`",
            f"- proof obligations: `{state_assisted_masked_projection_global_soundness['proof_obligation_count']}`",
            f"- named breaks: `{state_assisted_masked_projection_global_soundness['named_break_count']}`",
            f"- can accept now: `{state_assisted_masked_projection_global_soundness['can_accept_now']}`",
            f"- next artifact: `{state_assisted_masked_projection_global_soundness['next_artifact']}`",
            "",
            "## Pre-Signature Projection Mask PRF Leakage Simulator",
            "",
            f"- artifact: `{pre_signature_projection_mask_prf_leakage_simulator['artifact']}`",
            f"- report: `{pre_signature_projection_mask_prf_leakage_simulator['report']}`",
            f"- status: `{pre_signature_projection_mask_prf_leakage_simulator['status']}`",
            f"- deployment ready: `{pre_signature_projection_mask_prf_leakage_simulator['deployment_ready']}`",
            f"- worlds: `{pre_signature_projection_mask_prf_leakage_simulator['world_count']}`",
            f"- simulator steps: `{pre_signature_projection_mask_prf_leakage_simulator['simulator_step_count']}`",
            f"- named breaks: `{pre_signature_projection_mask_prf_leakage_simulator['named_break_count']}`",
            f"- advantage terms: `{pre_signature_projection_mask_prf_leakage_simulator['advantage_term_count']}`",
            f"- open gaps: `{pre_signature_projection_mask_prf_leakage_simulator['open_gap_count']}`",
            f"- can consume in global soundness: `{pre_signature_projection_mask_prf_leakage_simulator['can_consume_in_global_soundness']}`",
            f"- can accept now: `{pre_signature_projection_mask_prf_leakage_simulator['can_accept_now']}`",
            f"- next artifact: `{pre_signature_projection_mask_prf_leakage_simulator['next_artifact']}`",
            "",
            "## Mask PRF Child Gap Contracts",
            "",
            "| contract | status | key counts | can consume |",
            "|---|---|---|---|",
            (
                "| mask PRF RO reduction | "
                f"`{pre_signature_projection_mask_prf_ro_reduction['status']}` | "
                f"hybrids `{pre_signature_projection_mask_prf_ro_reduction['hybrid_step_count']}`, "
                f"breaks `{pre_signature_projection_mask_prf_ro_reduction['named_break_count']}`, "
                f"open `{pre_signature_projection_mask_prf_ro_reduction['open_item_count']}` | "
                f"`{pre_signature_projection_mask_prf_ro_reduction['can_consume_in_parent_simulator']}` |"
            ),
            (
                "| lifecycle same-game lift | "
                f"`{pre_signature_projection_lifecycle_same_game_lift['status']}` | "
                f"breaks `{pre_signature_projection_lifecycle_same_game_lift['named_break_count']}`, "
                f"supported `{pre_signature_projection_lifecycle_same_game_lift['supported_now_count']}`, "
                f"unproved `{pre_signature_projection_lifecycle_same_game_lift['remains_unproved_count']}` | "
                f"`{pre_signature_projection_lifecycle_same_game_lift['can_consume_in_mask_simulator']}` |"
            ),
            (
                "| spend-time leakage budget | "
                f"`{pre_signature_projection_spend_time_leakage_budget['status']}` | "
                f"disclosures `{pre_signature_projection_spend_time_leakage_budget['accepting_disclosure_count']}`, "
                f"remaining `{pre_signature_projection_spend_time_leakage_budget['remaining_obligation_count']}` | "
                f"`{pre_signature_projection_spend_time_leakage_budget['can_consume_as_fail_closed_budget']}` |"
            ),
            (
                "| reject equivalence | "
                f"`{pre_signature_projection_reject_equivalence['status']}` | "
                f"failure classes `{pre_signature_projection_reject_equivalence['failure_class_count']}`, "
                f"remaining `{pre_signature_projection_reject_equivalence['remaining_gap_count']}` | "
                f"`{pre_signature_projection_reject_equivalence['simulator_gap_status']}` |"
            ),
            "",
            "## Mask PRF Leakage Theorem Composition",
            "",
            f"- artifact: `{pre_signature_projection_mask_prf_leakage_theorem_composition['artifact']}`",
            f"- report: `{pre_signature_projection_mask_prf_leakage_theorem_composition['report']}`",
            f"- status: `{pre_signature_projection_mask_prf_leakage_theorem_composition['status']}`",
            f"- deployment ready: `{pre_signature_projection_mask_prf_leakage_theorem_composition['deployment_ready']}`",
            f"- theorem: `{pre_signature_projection_mask_prf_leakage_theorem_composition['theorem_name']}`",
            f"- composition hops: `{pre_signature_projection_mask_prf_leakage_theorem_composition['composition_hop_count']}`",
            f"- named breaks: `{pre_signature_projection_mask_prf_leakage_theorem_composition['named_break_count']}`",
            f"- carried assumptions: `{pre_signature_projection_mask_prf_leakage_theorem_composition['carried_assumption_count']}`",
            f"- open items: `{pre_signature_projection_mask_prf_leakage_theorem_composition['open_item_count']}`",
            f"- can consume in global soundness: `{pre_signature_projection_mask_prf_leakage_theorem_composition['can_consume_in_global_soundness']}`",
            f"- next artifact: `{pre_signature_projection_mask_prf_leakage_theorem_composition['next_artifact']}`",
            "",
            "## Mask PRF Carried-Term Discharge",
            "",
            "| term | status | key result |",
            "|---|---|---|",
            (
                "| SHA-256 mask frame PRF | "
                f"`{pre_signature_projection_sha256_mask_prf_assumption_resolution['status']}` | "
                f"closure path `{pre_signature_projection_sha256_mask_prf_assumption_resolution['closure_path']}`, "
                f"discharged `{pre_signature_projection_sha256_mask_prf_assumption_resolution['can_discharge']}` |"
            ),
            (
                "| global domain registry | "
                f"`{pre_signature_projection_global_domain_registry['status']}` | "
                f"entries `{pre_signature_projection_global_domain_registry['registry_entry_count']}`, "
                f"duplicates `{pre_signature_projection_global_domain_registry['registered_duplicate_count']}`, "
                f"ambiguity risks `{pre_signature_projection_global_domain_registry['byte_frame_ambiguity_risk_count']}` |"
            ),
            (
                "| lifecycle same-game | "
                f"`{pre_signature_projection_lifecycle_same_game_discharge['status']}` | "
                f"discharged `{pre_signature_projection_lifecycle_same_game_discharge['discharged_term_count']}`, "
                f"residual `{pre_signature_projection_lifecycle_same_game_discharge['residual_term_count']}` |"
            ),
            (
                "| spend-time budget | "
                f"`{pre_signature_projection_spend_time_budget_discharge['status']}` | "
                f"counts `{pre_signature_projection_spend_time_budget_discharge['disclosure_counts']}` |"
            ),
            (
                "| reject equivalence | "
                f"`{pre_signature_projection_reject_equivalence_discharge['status']}` | "
                f"counts `{pre_signature_projection_reject_equivalence_discharge['status_counts']}` |"
            ),
            "",
            "## Mask PRF Residual Blockers",
            "",
            "| blocker | status | key counts |",
            "|---|---|---|",
            (
                "| helper/API export boundary | "
                f"`{pre_signature_projection_helper_api_export_boundary['status']}` | "
                f"risky surfaces `{pre_signature_projection_helper_api_export_boundary['risky_surface_count']}`, "
                f"wrapper tests `{pre_signature_projection_helper_api_export_boundary['wrapper_test_count']}`, "
                f"residual blockers `{pre_signature_projection_helper_api_export_boundary['residual_blocker_count']}` |"
            ),
            (
                "| no-export negative tests | "
                f"`{pre_signature_projection_no_export_negative_tests['status']}` | "
                f"required tests `{pre_signature_projection_no_export_negative_tests['required_test_count']}`, "
                f"closes overread `{pre_signature_projection_no_export_negative_tests['can_close_projection_overread_residual']}` |"
            ),
            (
                "| REQ-PAD reject | "
                f"`{pre_signature_projection_req_pad_reject_discharge['status']}` | "
                f"failures `{pre_signature_projection_req_pad_reject_discharge['failure_case_count']}`, "
                f"KATs `{pre_signature_projection_req_pad_reject_discharge['required_kat_count']}`, "
                f"remove `{pre_signature_projection_req_pad_reject_discharge['can_remove']}` |"
            ),
            (
                "| REQ-WITNESS reject | "
                f"`{pre_signature_projection_req_witness_reject_discharge['status']}` | "
                f"failures `{pre_signature_projection_req_witness_reject_discharge['failure_case_count']}`, "
                f"KATs `{pre_signature_projection_req_witness_reject_discharge['required_kat_count']}`, "
                f"remove `{pre_signature_projection_req_witness_reject_discharge['can_remove']}` |"
            ),
            (
                "| REQ-CARRIER reject | "
                f"`{pre_signature_projection_req_carrier_reject_discharge['status']}` | "
                f"failures `{pre_signature_projection_req_carrier_reject_discharge['failure_case_count']}`, "
                f"KATs `{pre_signature_projection_req_carrier_reject_discharge['required_kat_count']}`, "
                f"remove `{pre_signature_projection_req_carrier_reject_discharge['can_remove']}` |"
            ),
            "",
            "## Mask PRF Residual Closure Progress",
            "",
            "| blocker | status | closure counts |",
            "|---|---|---|",
            (
                "| helper/API export boundary closure | "
                f"`{pre_signature_projection_helper_api_export_boundary_closure['status']}` | "
                f"closed `{pre_signature_projection_helper_api_export_boundary_closure['closed_blocker_count']}`, "
                f"remaining `{pre_signature_projection_helper_api_export_boundary_closure['remaining_blocker_count']}`, "
                f"wrapper tests `{pre_signature_projection_helper_api_export_boundary_closure['wrapper_test_count']}` |"
            ),
            (
                "| no-export negative tests closure | "
                f"`{pre_signature_projection_no_export_negative_tests_closure['status']}` | "
                f"implemented `{pre_signature_projection_no_export_negative_tests_closure['implemented_count']}`, "
                f"planned `{pre_signature_projection_no_export_negative_tests_closure['planned_count']}` |"
            ),
            (
                "| REQ-PAD closure | "
                f"`{pre_signature_projection_req_pad_reject_closure['status']}` | "
                f"KATs `{pre_signature_projection_req_pad_reject_closure['kat_count']}`, "
                f"five required KATs implemented `{pre_signature_projection_req_pad_reject_closure['five_required_kats_implemented']}`, "
                f"remove without timing `{pre_signature_projection_req_pad_reject_closure['can_remove_without_timing']}` |"
            ),
            (
                "| REQ-WITNESS closure | "
                f"`{pre_signature_projection_req_witness_reject_closure['status']}` | "
                f"covered failures `{pre_signature_projection_req_witness_reject_closure['covered_failure_case_count']}`, "
                f"KAT gap closed `{pre_signature_projection_req_witness_reject_closure['kat_gap_closed']}`, "
                f"remove `{pre_signature_projection_req_witness_reject_closure['can_remove']}` |"
            ),
            (
                "| REQ-CARRIER closure | "
                f"`{pre_signature_projection_req_carrier_reject_closure['status']}` | "
                f"KATs `{pre_signature_projection_req_carrier_reject_closure['kat_count']}`, "
                f"remove `{pre_signature_projection_req_carrier_reject_closure['can_remove']}` |"
            ),
            "",
            "## Hidden-Source State-Assisted Compact Proof Instantiation",
            "",
            f"- artifact: `{state_assisted_compact_proof_instantiation['artifact']}`",
            f"- report: `{state_assisted_compact_proof_instantiation['report']}`",
            f"- status: `{state_assisted_compact_proof_instantiation['status']}`",
            f"- deployment ready: `{state_assisted_compact_proof_instantiation['deployment_ready']}`",
            f"- candidates: `{state_assisted_compact_proof_instantiation['candidate_count']}`",
            f"- selected next track: `{state_assisted_compact_proof_instantiation['selected_next_track']}`",
            f"- fallback track: `{state_assisted_compact_proof_instantiation['fallback_track']}`",
            f"- decision gates: `{state_assisted_compact_proof_instantiation['decision_gate_count']}`",
            f"- implementation tasks: `{state_assisted_compact_proof_instantiation['implementation_task_count']}`",
            f"- can accept now: `{state_assisted_compact_proof_instantiation['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted RO-PoK Assumption API",
            "",
            f"- artifact: `{state_assisted_ro_pok_assumption_api['artifact']}`",
            f"- report: `{state_assisted_ro_pok_assumption_api['report']}`",
            f"- status: `{state_assisted_ro_pok_assumption_api['status']}`",
            f"- deployment ready: `{state_assisted_ro_pok_assumption_api['deployment_ready']}`",
            f"- assumption id: `{state_assisted_ro_pok_assumption_api['assumption_id']}`",
            f"- verifier function: `{state_assisted_ro_pok_assumption_api['verifier_function']}`",
            f"- input count: `{state_assisted_ro_pok_assumption_api['input_count']}`",
            f"- forbidden input count: `{state_assisted_ro_pok_assumption_api['forbidden_input_count']}`",
            f"- acceptance rules: `{state_assisted_ro_pok_assumption_api['acceptance_rule_count']}`",
            f"- review questions: `{state_assisted_ro_pok_assumption_api['review_question_count']}`",
            f"- can accept now: `{state_assisted_ro_pok_assumption_api['can_accept_now']}`",
            "",
            "## Hidden-Source State-Assisted RO-PoK Byte Negative Tests",
            "",
            f"- artifact: `{state_assisted_ro_pok_byte_negative_tests['artifact']}`",
            f"- report: `{state_assisted_ro_pok_byte_negative_tests['report']}`",
            f"- status: `{state_assisted_ro_pok_byte_negative_tests['status']}`",
            f"- deployment ready: `{state_assisted_ro_pok_byte_negative_tests['deployment_ready']}`",
            f"- test count: `{state_assisted_ro_pok_byte_negative_tests['test_count']}`",
            f"- executable status: `{state_assisted_ro_pok_byte_negative_tests['executable_status']}`",
            f"- can accept now: `{state_assisted_ro_pok_byte_negative_tests['can_accept_now']}`",
            "",
            "## Terminal Blockers",
            "",
        ]
    )
    lines.extend(f"- {blocker}" for blocker in preflight.get("blockers", []))
    lines.extend(
        [
            "",
            "## External Review Items",
            "",
            "| item | status | next step |",
            "|---|---|---|",
        ]
    )
    for item in payload["external_review_items"]:
        lines.append(
            f"| {item['id']} | {item['status']} | {item['next_step']} |"
        )
    lines.extend(
        [
            "",
            "## Primary Artifacts",
            "",
        ]
    )
    lines.extend(f"- `{path}`" for path in payload["primary_artifacts"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
