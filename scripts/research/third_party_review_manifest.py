#!/usr/bin/env python3
"""Generate a hashed table of contents for the third-party review packet."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
COST_MODEL_DIR = (
    ROOT / "research/parameter_search/committed_sigma_account_carried_20260521"
)
DEFAULT_JSON = READINESS_DIR / "third_party_review_manifest_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "third_party_review_manifest_n32_q12289.md"
REPLAY_REPORT = READINESS_DIR / "review_gate_replay_report_n32_q12289.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    return json.loads(path.read_text())


def packet_files() -> list[dict[str, Any]]:
    entries = [
        ("entrypoint", READINESS_DIR / "third_party_review_packet.md"),
        ("entrypoint", READINESS_DIR / "third_party_review_status_index_n32_q12289.md"),
        ("entrypoint", READINESS_DIR / "third_party_review_status_index_n32_q12289.json"),
        ("entrypoint", READINESS_DIR / "compact_signature_todos.md"),
        ("entrypoint", READINESS_DIR / "bucket1_deterministic_proof_tracker_n32_q12289.md"),
        ("entrypoint", READINESS_DIR / "bucket1_deterministic_proof_tracker_n32_q12289.json"),
        ("entrypoint", READINESS_DIR / "bucket2_universal_pad_prf_tracker_n32_q12289.md"),
        ("entrypoint", READINESS_DIR / "bucket2_universal_pad_prf_tracker_n32_q12289.json"),
        ("entrypoint", READINESS_DIR / "proof_boundary_ledger_n32_q12289.md"),
        ("entrypoint", READINESS_DIR / "proof_boundary_ledger_n32_q12289.json"),
        ("entrypoint", READINESS_DIR / "proof_boundary_prose_audit_n32_q12289.md"),
        ("entrypoint", READINESS_DIR / "proof_boundary_prose_audit_n32_q12289.json"),
        ("entrypoint", READINESS_DIR / "review_gate_replay_report_n32_q12289.md"),
        ("entrypoint", READINESS_DIR / "review_gate_replay_report_n32_q12289.json"),
        ("entrypoint", READINESS_DIR / "selected_deployment_terminology_verification_n32_q12289.md"),
        ("entrypoint", READINESS_DIR / "selected_deployment_terminology_verification_n32_q12289.json"),
        ("entrypoint", READINESS_DIR / "compact_signature_todos_consistency_n32_q12289.md"),
        ("entrypoint", READINESS_DIR / "compact_signature_todos_consistency_n32_q12289.json"),
        ("closed_sidecar", READINESS_DIR / "quotient_level_link_promotion_gate.md"),
        ("closed_sidecar", READINESS_DIR / "quotient_level_link_promotion_gate.json"),
        ("closed_sidecar", READINESS_DIR / "quotient_level_link_implementation_handoff.md"),
        ("closed_sidecar", READINESS_DIR / "quotient_level_link_firebreak_audit.md"),
        ("closed_sidecar", READINESS_DIR / "quotient_level_link_firebreak_audit.json"),
        ("closed_sidecar", READINESS_DIR / "quotient_level_link_research_program.md"),
        ("byte_dashboard", COST_MODEL_DIR / "committed_sigma_account_carried_cost_model.md"),
        ("byte_dashboard", COST_MODEL_DIR / "committed_sigma_account_carried_cost_model.json"),
        ("response_link_certificate", READINESS_DIR / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"),
        ("response_link_certificate", READINESS_DIR / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"),
        ("response_link_certificate", READINESS_DIR / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json"),
        ("response_link_certificate", READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json"),
        ("response_link_certificate", ROOT / "Descender/ResponseLinkExactCertificate.lean"),
        ("response_link_certificate", ROOT / "Descender/Generated/ResponseLinkExactCertificateN32Q12289Coords03B96.lean"),
        ("red_team", READINESS_DIR / "red_team_cryptoanalysis_n32_q12289.md"),
        ("red_team", READINESS_DIR / "red_team_cryptoanalysis_n32_q12289.json"),
        ("assumption_review", READINESS_DIR / "keygen_security_assumption_surface_n32_q12289.md"),
        ("assumption_review", READINESS_DIR / "keygen_security_assumption_surface_n32_q12289.json"),
        ("assumption_review", READINESS_DIR / "assumption_attack_review_matrix_n32_q12289.md"),
        ("assumption_review", READINESS_DIR / "assumption_attack_review_matrix_n32_q12289.json"),
        ("assumption_review", READINESS_DIR / "assumption_red_team_closure_n32_q12289.md"),
        ("assumption_review", READINESS_DIR / "assumption_red_team_closure_n32_q12289.json"),
        ("assumption_review", READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.md"),
        ("assumption_review", READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.json"),
        ("qrom_and_sampler", READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.md"),
        ("qrom_and_sampler", READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.json"),
        ("qrom_and_sampler", READINESS_DIR / "qrom_security_loss_review_n32_q12289.md"),
        ("qrom_and_sampler", READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"),
        ("qrom_and_sampler", READINESS_DIR / "pad_sampler_boundary_memo_n32_q12289.md"),
        ("qrom_and_sampler", READINESS_DIR / "pad_sampler_boundary_memo_n32_q12289.json"),
        ("qrom_and_sampler", READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.md"),
        ("qrom_and_sampler", READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json"),
        ("production_reference", READINESS_DIR / "production_reference_gap_report_n32_q12289.md"),
        ("production_reference", READINESS_DIR / "production_reference_gap_report_n32_q12289.json"),
        ("production_reference", READINESS_DIR / "reviewed_equation_spec_gap_n32_q12289.md"),
        ("production_reference", READINESS_DIR / "reviewed_equation_spec_gap_n32_q12289.json"),
        ("production_reference", READINESS_DIR / "reviewed_challenge_action_options_n32_q12289.md"),
        ("production_reference", READINESS_DIR / "reviewed_challenge_action_options_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_accepting_relation_spec_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_accepting_relation_spec_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_carrier_construction_decision_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_carrier_construction_decision_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_carrier_relation_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_carrier_relation_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_extractor_obligations_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_extractor_obligations_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_negative_controls_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_negative_controls_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_carrier_projection_relation_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_carrier_projection_relation_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_map_search_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_map_search_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_map_enumerator_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_map_enumerator_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_lane_map_consistency_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_lane_map_consistency_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_checked_core_lift_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_checked_core_lift_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_break_export_contract_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_break_export_contract_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.json"),
        ("proof_target", ROOT / "Descender/HiddenSourceActiveSubspace.lean"),
        ("proof_target", ROOT / "Descender/HiddenSourceBucket1.lean"),
        ("proof_target", ROOT / "Descender/Generated/HiddenSourceActiveSubspaceN32Q12289.lean"),
        ("proof_target", READINESS_DIR / "hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_ro_pok_external_review_packet_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_ro_pok_external_review_packet_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.json"),
        ("proof_target", READINESS_DIR / "hidden_source_ro_pok_leakage_simulator_n32_q12289.md"),
        ("proof_target", READINESS_DIR / "hidden_source_ro_pok_leakage_simulator_n32_q12289.json"),
        ("production_reference", READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.md"),
        ("production_reference", READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.json"),
        ("production_reference", READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.md"),
        ("production_reference", READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.json"),
        ("production_reference", READINESS_DIR / "constant_time_side_channel_audit_memo_n32_q12289.md"),
        ("production_reference", READINESS_DIR / "constant_time_side_channel_audit_memo_n32_q12289.json"),
        ("production_reference", READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.md"),
        ("production_reference", READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json"),
        ("production_reference", READINESS_DIR / "external_storage_review_packet_n32_q12289.md"),
        ("production_reference", READINESS_DIR / "external_storage_review_packet_n32_q12289.json"),
        ("production_reference", READINESS_DIR / "production_preflight_certificate_verification_n32_q12289.md"),
        ("production_reference", READINESS_DIR / "production_preflight_certificate_verification_n32_q12289.json"),
        ("timing", READINESS_DIR / "reference_timing_report_n32_q12289.md"),
        ("timing", READINESS_DIR / "reference_timing_report_n32_q12289.json"),
        ("script", ROOT / "scripts/research/finalize_third_party_review_packet.py"),
        ("script", ROOT / "scripts/research/third_party_review_status_index.py"),
        ("script", ROOT / "scripts/research/verify_third_party_review_status_index.py"),
        ("script", ROOT / "scripts/research/red_team_cryptoanalysis.py"),
        ("script", ROOT / "scripts/research/assumption_red_team_closure.py"),
        ("script", ROOT / "scripts/research/review_gate_replay.py"),
        ("script", ROOT / "scripts/research/verify_quotient_level_link_closure.py"),
        ("script", ROOT / "scripts/research/quotient_level_link_promotion_gate.py"),
        ("script", ROOT / "scripts/research/quotient_level_link_firebreak_audit.py"),
        ("script", ROOT / "scripts/research/third_party_review_manifest.py"),
        ("script", ROOT / "scripts/research/verify_third_party_review_manifest.py"),
        ("script", ROOT / "scripts/research/verify_third_party_review_finalization.py"),
        ("script", ROOT / "scripts/research/verify_production_preflight_certificate.py"),
        ("script", ROOT / "scripts/research/verify_selected_deployment_terminology.py"),
        ("script", ROOT / "scripts/research/verify_compact_signature_todos_consistency.py"),
        ("script", ROOT / "scripts/research/proof_boundary_prose_audit.py"),
        ("script", ROOT / "scripts/research/verify_response_link_bounded_kernel_certificate.py"),
        ("script", ROOT / "scripts/research/audit_response_link_bounded_kernel_manifest.py"),
        ("script", ROOT / "scripts/research/generate_response_link_exact_certificate_lean.py"),
        ("script", ROOT / "scripts/parameter_search/committed_sigma_cost_model.py"),
        ("script", ROOT / "scripts/research/production_reference_gap_report.py"),
        ("script", ROOT / "scripts/research/reviewed_equation_spec_gap.py"),
        ("script", ROOT / "scripts/research/reviewed_challenge_action_options.py"),
        ("script", ROOT / "scripts/research/hidden_source_proof_semantics_target.py"),
        ("script", ROOT / "scripts/research/hidden_source_accepting_relation_spec.py"),
        ("script", ROOT / "scripts/research/hidden_source_carrier_construction_decision.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_carrier_relation.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_extractor_obligations.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_negative_controls.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_opening_frame.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_opening_frame_test_vectors.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_carrier_verifier_contract.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_account_state_schema_contract.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_hidden_relation_verifier_contract.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_hidden_relation_negative_controls.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_hidden_relation_extractor_theorem.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_compact_proof_instantiation.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_ro_pok_assumption_api.py"),
        ("script", ROOT / "scripts/research/hidden_source_state_assisted_ro_pok_byte_negative_tests.py"),
        ("script", ROOT / "scripts/research/hidden_source_algebraic_carrier_fallback_byte_model.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_carrier_projection_relation.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_map_search.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_map_enumerator.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_map_certificate_attempt.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_map_exact_certificate_spec.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_lane_map_consistency.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_matrix_builder.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_checked_core_lift.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_semantic_bridge_spec.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_bridge_exact_replay.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_exact_infeasibility_replay.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_bridge_equation_derivation.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_checked_core_coupling_options.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_direct_linear_lift_attempt.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_checked_core_break_reduction.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_public_bridge_predicate.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_break_export_contract.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_break_export_negative_tests.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_map_redesign_or_bridge_rows.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_verifier_exporter_contract.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_verifier_exporter_implementation_gap.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_verifier_exporter_api_stubs.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_semantic_classifier_invocation_contract.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_semantic_predicate_implementation_plan.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_response_link_kernel_replay_boundary.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_opening_projection_boundary.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_account_commitment_split_boundary.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_pad_prf_projection_boundary.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_byte_level_derivation_internalization.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_expanded_opening_commitment_binding.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_account_state_opening_commitment_wiring.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure.py"),
        ("script", ROOT / "scripts/research/hidden_source_fallback_projection_active_subspace_exact_replay.py"),
        ("script", ROOT / "scripts/research/generate_hidden_source_active_subspace_lean.py"),
        ("script", ROOT / "scripts/research/hidden_source_ro_pok_vs_fallback_decision_gate.py"),
        ("script", ROOT / "scripts/research/hidden_source_ro_pok_external_review_packet.py"),
        ("script", ROOT / "scripts/research/hidden_source_ro_pok_leakage_global_composition_theorem.py"),
        ("script", ROOT / "scripts/research/hidden_source_ro_pok_leakage_simulator.py"),
        ("script", ROOT / "scripts/research/production_reference_facade_timing_report.py"),
        ("script", ROOT / "scripts/research/qrom_security_loss_review.py"),
        ("script", ROOT / "scripts/research/pad_sampler_boundary_review.py"),
        ("crate_boundary", ROOT / "tools/production-reference/Cargo.toml"),
        ("crate_boundary", ROOT / "tools/production-reference/Cargo.lock"),
        ("crate_boundary", ROOT / "tools/production-reference/README.md"),
        ("crate_boundary", ROOT / "tools/production-reference/src/lib.rs"),
        ("crate_boundary", ROOT / "tools/production-reference/src/bin/production_reference_facade_timing.rs"),
        ("crate_boundary", ROOT / "tools/production-reference/src/bin/production_reference_preflight_certificate.rs"),
    ]
    rows = []
    for role, path in entries:
        row: dict[str, Any] = {
            "role": role,
            "path": rel(path),
            "present": path.exists(),
        }
        if path.exists():
            row.update(
                {
                    "bytes": path.stat().st_size,
                    "sha256": file_sha256(path),
                }
            )
            if path.suffix == ".json":
                row["schema"] = (load_json(path) or {}).get("schema")
        rows.append(row)
    return rows


def build_payload() -> dict[str, Any]:
    replay = load_json(REPLAY_REPORT) or {}
    files = packet_files()
    missing = [item for item in files if not item["present"]]
    return {
        "schema": "Descender.third_party_review_manifest.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "target": {
            "n": 32,
            "q": 12289,
            "transaction_bytes": 250,
            "account_link_bytes": 68,
            "public_key_bytes": 172,
        },
        "recommended_finalize_command": "python3 scripts/research/finalize_third_party_review_packet.py",
        "recommended_replay_command": "python3 scripts/research/review_gate_replay.py --skip-timing --replay-exact",
        "replay_report": {
            "artifact": rel(REPLAY_REPORT),
            "status": replay.get("status"),
            "replay_exact": replay.get("replay_exact"),
            "failed_step_count": replay.get("failed_step_count"),
            "invalid_json_artifact_count": replay.get("invalid_json_artifact_count"),
            "hashed_json_artifact_count": replay.get("hashed_json_artifact_count"),
        },
        "summary": {
            "files_indexed": len(files),
            "files_present": len(files) - len(missing),
            "files_missing": len(missing),
        },
        "files": files,
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Third-Party Review Manifest",
        "",
        "This manifest is a hashed table of contents for the local review packet.",
        "It is not a security proof; it is intended to make replay and handoff",
        "auditable.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- target: `n={payload['target']['n']}`, `q={payload['target']['q']}`",
        f"- transaction bytes: `{payload['target']['transaction_bytes']}`",
        f"- account-link bytes: `{payload['target']['account_link_bytes']}`",
        f"- public-key bytes: `{payload['target']['public_key_bytes']}`",
        f"- files indexed: `{payload['summary']['files_indexed']}`",
        f"- files missing: `{payload['summary']['files_missing']}`",
        f"- replay status: `{payload['replay_report']['status']}`",
        f"- replay exact: `{payload['replay_report']['replay_exact']}`",
        f"- replay failed steps: `{payload['replay_report']['failed_step_count']}`",
        f"- replay invalid JSON artifacts: `{payload['replay_report']['invalid_json_artifact_count']}`",
        "",
        "Recommended finalization command:",
        "",
        "```bash",
        payload["recommended_finalize_command"],
        "```",
        "",
        "Lower-level replay command:",
        "",
        "```bash",
        payload["recommended_replay_command"],
        "```",
        "",
        "## Files",
        "",
        "| role | path | present | bytes | sha256 | schema |",
        "|---|---|:---:|---:|---|---|",
    ]
    for row in payload["files"]:
        lines.append(
            f"| {row['role']} | {row['path']} | {'yes' if row['present'] else 'no'} | "
            f"{row.get('bytes') or '-'} | {row.get('sha256') or '-'} | "
            f"{row.get('schema') or '-'} |"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))
    if payload["summary"]["files_missing"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
