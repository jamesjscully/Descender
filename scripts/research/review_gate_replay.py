#!/usr/bin/env python3
"""Replay Descender review-gate artifact generation and validation."""

from __future__ import annotations

import argparse
import hashlib
import json
import platform
import subprocess
import sys
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
COST_MODEL_DIR = (
    ROOT / "research/parameter_search/committed_sigma_account_carried_20260521"
)
DEFAULT_JSON = READINESS_DIR / "review_gate_replay_report_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "review_gate_replay_report_n32_q12289.md"


@dataclass(frozen=True)
class Step:
    name: str
    command: list[str]
    produces: tuple[Path, ...] = ()
    timing: bool = False
    optional: bool = False


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def python(script: str, *args: str) -> list[str]:
    return [sys.executable, str(ROOT / script), *args]


def file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def build_steps(iterations: int, skip_timing: bool, replay_exact: bool) -> list[Step]:
    steps: list[Step] = []
    if not skip_timing:
        steps.extend(
            [
                Step(
                    "build_reference_timing_binary",
                    [
                        "cargo",
                        "build",
                        "--release",
                        "--manifest-path",
                        "tools/parameter-search/Cargo.toml",
                        "--bin",
                        "signature_q12289_local_delta_stateful",
                    ],
                    timing=True,
                ),
                Step(
                    "reference_timing_report",
                    python(
                        "scripts/research/reference_timing_report.py",
                        "--iterations",
                        str(iterations),
                    ),
                    (
                        READINESS_DIR / "reference_timing_report_n32_q12289.json",
                        READINESS_DIR / "reference_timing_report_n32_q12289.md",
                    ),
                    timing=True,
                ),
                Step(
                    "production_reference_facade_timing_report",
                    python(
                        "scripts/research/production_reference_facade_timing_report.py",
                        "--iterations",
                        str(iterations),
                    ),
                    (
                        READINESS_DIR
                        / "production_reference_facade_timing_report_n32_q12289.json",
                        READINESS_DIR
                        / "production_reference_facade_timing_report_n32_q12289.md",
                    ),
                    timing=True,
                ),
            ]
        )

    audit_args = ["--replay"] if replay_exact else []
    steps.extend(
        [
            Step("qrom_challenge_budget_memo", python("scripts/research/qrom_challenge_budget_memo.py")),
            Step("qrom_security_loss_review", python("scripts/research/qrom_security_loss_review.py")),
            Step("pad_sampler_boundary_memo", python("scripts/research/pad_sampler_boundary_memo.py")),
            Step("pad_sampler_boundary_review", python("scripts/research/pad_sampler_boundary_review.py")),
            Step("matrix_keygen_estimator_memo", python("scripts/research/matrix_keygen_estimator_memo.py")),
            Step(
                "quotient_sis_lattice_attack_review",
                python("scripts/research/quotient_sis_lattice_attack_review.py"),
                (READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json",),
            ),
            Step(
                "response_link_lattice_attack_review",
                python(
                    "scripts/research/response_link_lattice_attack_review.py",
                    "--write-json",
                    str(
                        READINESS_DIR
                        / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json"
                    ),
                ),
                (
                    READINESS_DIR
                    / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json",
                ),
            ),
            Step(
                "response_link_manifest_static_audit"
                if not replay_exact
                else "response_link_manifest_exact_replay",
                python("scripts/research/audit_response_link_bounded_kernel_manifest.py", *audit_args),
            ),
            Step(
                "generate_response_link_exact_certificate_lean",
                python("scripts/research/generate_response_link_exact_certificate_lean.py"),
                (
                    ROOT
                    / "Descender/Generated/ResponseLinkExactCertificateN32Q12289Coords03B96.lean",
                ),
            ),
            Step(
                "build_response_link_exact_certificate_lean",
                [
                    "lake",
                    "build",
                    "Descender.Generated.ResponseLinkExactCertificateN32Q12289Coords03B96",
                ],
            ),
            Step("production_reference_gap_report", python("scripts/research/production_reference_gap_report.py")),
            Step(
                "reviewed_equation_spec_gap",
                python("scripts/research/reviewed_equation_spec_gap.py"),
                (
                    READINESS_DIR / "reviewed_equation_spec_gap_n32_q12289.json",
                    READINESS_DIR / "reviewed_equation_spec_gap_n32_q12289.md",
                ),
            ),
            Step(
                "reviewed_challenge_action_options",
                python("scripts/research/reviewed_challenge_action_options.py"),
                (
                    READINESS_DIR / "reviewed_challenge_action_options_n32_q12289.json",
                    READINESS_DIR / "reviewed_challenge_action_options_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_proof_semantics_target",
                python("scripts/research/hidden_source_proof_semantics_target.py"),
                (
                    READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.json",
                    READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_accepting_relation_spec",
                python("scripts/research/hidden_source_accepting_relation_spec.py"),
                (
                    READINESS_DIR / "hidden_source_accepting_relation_spec_n32_q12289.json",
                    READINESS_DIR / "hidden_source_accepting_relation_spec_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_carrier_construction_decision",
                python("scripts/research/hidden_source_carrier_construction_decision.py"),
                (
                    READINESS_DIR / "hidden_source_carrier_construction_decision_n32_q12289.json",
                    READINESS_DIR / "hidden_source_carrier_construction_decision_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_carrier_relation",
                python("scripts/research/hidden_source_state_assisted_carrier_relation.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_carrier_relation_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_carrier_relation_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_extractor_obligations",
                python("scripts/research/hidden_source_state_assisted_extractor_obligations.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_extractor_obligations_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_extractor_obligations_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_negative_controls",
                python("scripts/research/hidden_source_state_assisted_negative_controls.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_negative_controls_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_negative_controls_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_opening_frame",
                python("scripts/research/hidden_source_state_assisted_opening_frame.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_opening_frame_test_vectors",
                python("scripts/research/hidden_source_state_assisted_opening_frame_test_vectors.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_carrier_verifier_contract",
                python("scripts/research/hidden_source_state_assisted_carrier_verifier_contract.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_account_state_schema_contract",
                python("scripts/research/hidden_source_state_assisted_account_state_schema_contract.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_hidden_relation_verifier_contract",
                python("scripts/research/hidden_source_state_assisted_hidden_relation_verifier_contract.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_hidden_relation_negative_controls",
                python("scripts/research/hidden_source_state_assisted_hidden_relation_negative_controls.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_hidden_relation_extractor_theorem",
                python("scripts/research/hidden_source_state_assisted_hidden_relation_extractor_theorem.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_compact_proof_instantiation",
                python("scripts/research/hidden_source_state_assisted_compact_proof_instantiation.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_ro_pok_assumption_api",
                python("scripts/research/hidden_source_state_assisted_ro_pok_assumption_api.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_state_assisted_ro_pok_byte_negative_tests",
                python("scripts/research/hidden_source_state_assisted_ro_pok_byte_negative_tests.py"),
                (
                    READINESS_DIR / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json",
                    READINESS_DIR / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_algebraic_carrier_fallback_byte_model",
                python("scripts/research/hidden_source_algebraic_carrier_fallback_byte_model.py"),
                (
                    READINESS_DIR / "hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.json",
                    READINESS_DIR / "hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_carrier_projection_relation",
                python("scripts/research/hidden_source_fallback_carrier_projection_relation.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_carrier_projection_relation_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_carrier_projection_relation_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_map_search",
                python("scripts/research/hidden_source_fallback_projection_map_search.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_map_search_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_map_search_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_map_enumerator",
                python("scripts/research/hidden_source_fallback_projection_map_enumerator.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_map_enumerator_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_map_enumerator_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_map_certificate_attempt",
                python("scripts/research/hidden_source_fallback_projection_map_certificate_attempt.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_map_exact_certificate_spec",
                python("scripts/research/hidden_source_fallback_projection_map_exact_certificate_spec.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_lane_map_consistency",
                python("scripts/research/hidden_source_fallback_projection_lane_map_consistency.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_lane_map_consistency_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_lane_map_consistency_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_matrix_builder",
                python("scripts/research/hidden_source_fallback_projection_matrix_builder.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_checked_core_lift",
                python("scripts/research/hidden_source_fallback_projection_checked_core_lift.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_checked_core_lift_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_checked_core_lift_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_semantic_bridge_spec",
                python("scripts/research/hidden_source_fallback_projection_semantic_bridge_spec.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_bridge_exact_replay",
                python("scripts/research/hidden_source_fallback_projection_bridge_exact_replay.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_bridge_equation_derivation",
                python("scripts/research/hidden_source_fallback_projection_bridge_equation_derivation.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_exact_infeasibility_replay",
                python("scripts/research/hidden_source_fallback_projection_exact_infeasibility_replay.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_map_exact_certificate_spec_refresh",
                python("scripts/research/hidden_source_fallback_projection_map_exact_certificate_spec.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_checked_core_coupling_options",
                python("scripts/research/hidden_source_fallback_projection_checked_core_coupling_options.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_direct_linear_lift_attempt",
                python("scripts/research/hidden_source_fallback_projection_direct_linear_lift_attempt.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_checked_core_break_reduction",
                python("scripts/research/hidden_source_fallback_projection_checked_core_break_reduction.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_public_bridge_predicate",
                python("scripts/research/hidden_source_fallback_projection_public_bridge_predicate.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_break_export_contract",
                python("scripts/research/hidden_source_fallback_projection_break_export_contract.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_break_export_contract_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_break_export_contract_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_break_export_negative_tests",
                python("scripts/research/hidden_source_fallback_projection_break_export_negative_tests.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_map_redesign_or_bridge_rows",
                python("scripts/research/hidden_source_fallback_projection_map_redesign_or_bridge_rows.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_verifier_exporter_contract",
                python("scripts/research/hidden_source_fallback_projection_verifier_exporter_contract.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_verifier_exporter_implementation_gap",
                python("scripts/research/hidden_source_fallback_projection_verifier_exporter_implementation_gap.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_verifier_exporter_api_stubs",
                python("scripts/research/hidden_source_fallback_projection_verifier_exporter_api_stubs.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_semantic_classifier_invocation_contract",
                python("scripts/research/hidden_source_fallback_projection_semantic_classifier_invocation_contract.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_semantic_predicate_implementation_plan",
                python("scripts/research/hidden_source_fallback_projection_semantic_predicate_implementation_plan.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_response_link_kernel_replay_boundary",
                python("scripts/research/hidden_source_fallback_projection_response_link_kernel_replay_boundary.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_opening_projection_boundary",
                python("scripts/research/hidden_source_fallback_projection_opening_projection_boundary.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_account_commitment_split_boundary",
                python("scripts/research/hidden_source_fallback_projection_account_commitment_split_boundary.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_pad_prf_projection_boundary",
                python("scripts/research/hidden_source_fallback_projection_pad_prf_projection_boundary.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_byte_level_derivation_internalization",
                python("scripts/research/hidden_source_fallback_projection_byte_level_derivation_internalization.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_expanded_opening_commitment_binding",
                python("scripts/research/hidden_source_fallback_projection_expanded_opening_commitment_binding.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_account_state_opening_commitment_wiring",
                python("scripts/research/hidden_source_fallback_projection_account_state_opening_commitment_wiring.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof",
                python("scripts/research/hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure",
                python("scripts/research/hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_fallback_projection_active_subspace_exact_replay",
                python("scripts/research/hidden_source_fallback_projection_active_subspace_exact_replay.py"),
                (
                    READINESS_DIR / "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.json",
                    READINESS_DIR / "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.md",
                ),
            ),
            Step(
                "generate_hidden_source_active_subspace_lean",
                python("scripts/research/generate_hidden_source_active_subspace_lean.py"),
                (
                    ROOT
                    / "Descender/Generated/HiddenSourceActiveSubspaceN32Q12289.lean",
                ),
            ),
            Step(
                "build_hidden_source_active_subspace_lean",
                [
                    "lake",
                    "build",
                    "Descender.Generated.HiddenSourceActiveSubspaceN32Q12289",
                ],
            ),
            Step(
                "hidden_source_ro_pok_vs_fallback_decision_gate",
                python("scripts/research/hidden_source_ro_pok_vs_fallback_decision_gate.py"),
                (
                    READINESS_DIR / "hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.json",
                    READINESS_DIR / "hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_ro_pok_external_review_packet",
                python("scripts/research/hidden_source_ro_pok_external_review_packet.py"),
                (
                    READINESS_DIR / "hidden_source_ro_pok_external_review_packet_n32_q12289.json",
                    READINESS_DIR / "hidden_source_ro_pok_external_review_packet_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_ro_pok_leakage_global_composition_theorem",
                python("scripts/research/hidden_source_ro_pok_leakage_global_composition_theorem.py"),
                (
                    READINESS_DIR / "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.json",
                    READINESS_DIR / "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.md",
                ),
            ),
            Step(
                "hidden_source_ro_pok_leakage_simulator",
                python("scripts/research/hidden_source_ro_pok_leakage_simulator.py"),
                (
                    READINESS_DIR / "hidden_source_ro_pok_leakage_simulator_n32_q12289.json",
                    READINESS_DIR / "hidden_source_ro_pok_leakage_simulator_n32_q12289.md",
                ),
            ),
            Step(
                "production_lifecycle_hardening_memo",
                python("scripts/research/production_lifecycle_hardening_memo.py"),
            ),
            Step(
                "constant_time_side_channel_audit_memo",
                python("scripts/research/constant_time_side_channel_audit_memo.py"),
            ),
            Step(
                "blockchain_deployment_integration_memo",
                python("scripts/research/blockchain_deployment_integration_memo.py"),
            ),
            Step(
                "external_storage_review_packet",
                python("scripts/research/external_storage_review_packet.py"),
            ),
            Step(
                "keygen_security_assumption_surface",
                python("scripts/research/keygen_security_assumption_surface.py"),
            ),
            Step(
                "assumption_attack_review_matrix",
                python("scripts/research/assumption_attack_review_matrix.py"),
            ),
            Step(
                "red_team_cryptoanalysis",
                python("scripts/research/red_team_cryptoanalysis.py"),
                (
                    READINESS_DIR / "red_team_cryptoanalysis_n32_q12289.json",
                    READINESS_DIR / "red_team_cryptoanalysis_n32_q12289.md",
                ),
            ),
            Step(
                "conditional_soundness_budget_review",
                python("scripts/research/conditional_soundness_budget_review.py"),
            ),
            Step(
                "assumption_red_team_closure",
                python("scripts/research/assumption_red_team_closure.py"),
                (
                    READINESS_DIR / "assumption_red_team_closure_n32_q12289.json",
                    READINESS_DIR / "assumption_red_team_closure_n32_q12289.md",
                ),
            ),
            Step(
                "committed_sigma_cost_model",
                python("scripts/parameter_search/committed_sigma_cost_model.py"),
                (
                    COST_MODEL_DIR / "committed_sigma_account_carried_cost_model.json",
                    COST_MODEL_DIR / "committed_sigma_account_carried_cost_model.md",
                ),
            ),
            Step(
                "quotient_level_link_closure_verify",
                python("scripts/research/verify_quotient_level_link_closure.py"),
            ),
        ]
    )
    return steps


def expected_json_artifacts() -> list[Path]:
    return [
        READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.json",
        READINESS_DIR / "qrom_security_loss_review_n32_q12289.json",
        READINESS_DIR / "pad_sampler_boundary_memo_n32_q12289.json",
        READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json",
        READINESS_DIR / "matrix_keygen_estimator_memo_n32_q12289.json",
        READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json",
        READINESS_DIR / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json",
        READINESS_DIR / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json",
        READINESS_DIR / "reference_timing_report_n32_q12289.json",
        READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.json",
        READINESS_DIR / "production_reference_gap_report_n32_q12289.json",
        READINESS_DIR / "reviewed_equation_spec_gap_n32_q12289.json",
        READINESS_DIR / "reviewed_challenge_action_options_n32_q12289.json",
        READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.json",
        READINESS_DIR / "hidden_source_accepting_relation_spec_n32_q12289.json",
        READINESS_DIR / "hidden_source_carrier_construction_decision_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_carrier_relation_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_extractor_obligations_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_negative_controls_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json",
        READINESS_DIR / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json",
        READINESS_DIR / "hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_carrier_projection_relation_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_map_search_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_map_enumerator_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_lane_map_consistency_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_checked_core_lift_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_break_export_contract_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.json",
        READINESS_DIR / "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.json",
        READINESS_DIR / "hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.json",
        READINESS_DIR / "hidden_source_ro_pok_external_review_packet_n32_q12289.json",
        READINESS_DIR / "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.json",
        READINESS_DIR / "hidden_source_ro_pok_leakage_simulator_n32_q12289.json",
        READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.json",
        READINESS_DIR / "constant_time_side_channel_audit_memo_n32_q12289.json",
        READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json",
        READINESS_DIR / "keygen_security_assumption_surface_n32_q12289.json",
        READINESS_DIR / "assumption_attack_review_matrix_n32_q12289.json",
        READINESS_DIR / "red_team_cryptoanalysis_n32_q12289.json",
        READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.json",
        READINESS_DIR / "assumption_red_team_closure_n32_q12289.json",
        COST_MODEL_DIR / "committed_sigma_account_carried_cost_model.json",
        READINESS_DIR / "quotient_level_link_promotion_gate.json",
        READINESS_DIR / "quotient_level_link_firebreak_audit.json",
    ]


def run_step(step: Step, dry_run: bool) -> dict[str, Any]:
    started = time.perf_counter()
    command_display = " ".join(step.command)
    if dry_run:
        return {
            "name": step.name,
            "command": command_display,
            "status": "dry_run",
            "elapsed_seconds": 0.0,
            "produces": [rel(path) for path in step.produces],
        }
    completed = subprocess.run(
        step.command,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    elapsed = time.perf_counter() - started
    produced = []
    missing = []
    for path in step.produces:
        if path.exists():
            produced.append(rel(path))
        else:
            missing.append(rel(path))
    ok = completed.returncode == 0 and not missing
    if step.optional and completed.returncode != 0:
        status = "optional_failed"
    else:
        status = "pass" if ok else "fail"
    return {
        "name": step.name,
        "command": command_display,
        "status": status,
        "returncode": completed.returncode,
        "elapsed_seconds": round(elapsed, 3),
        "produced": produced,
        "missing_outputs": missing,
        "stdout_tail": completed.stdout[-4000:],
        "stderr_tail": completed.stderr[-4000:],
    }


def validate_json_artifacts(paths: list[Path]) -> list[dict[str, Any]]:
    rows = []
    for path in paths:
        if not path.exists():
            rows.append({"artifact": rel(path), "status": "missing"})
            continue
        try:
            payload = json.loads(path.read_text())
        except json.JSONDecodeError as exc:
            rows.append(
                {
                    "artifact": rel(path),
                    "status": "invalid_json",
                    "error": str(exc),
                }
            )
            continue
        rows.append(
            {
                "artifact": rel(path),
                "status": "valid",
                "schema": payload.get("schema"),
                "bytes": path.stat().st_size,
                "sha256": file_sha256(path),
            }
        )
    return rows


def report_markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Review Gate Replay Report",
        "",
        "This report records one local replay of the Descender review-gate artifact",
        "pipeline. It is a reproducibility/checklist artifact, not a security proof.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- timing iterations: `{payload['timing_iterations']}`",
        f"- timing steps skipped: `{payload['skip_timing']}`",
        f"- exact response-link replay requested: `{payload['replay_exact']}`",
        f"- dry run: `{payload['dry_run']}`",
        f"- overall status: `{payload['status']}`",
        f"- failed steps: `{payload['failed_step_count']}`",
        f"- invalid or missing JSON artifacts: `{payload['invalid_json_artifact_count']}`",
        f"- hashed JSON artifacts: `{payload['hashed_json_artifact_count']}`",
        "",
        "## Steps",
        "",
        "| step | status | seconds |",
        "|---|---|---:|",
    ]
    for step in payload["steps"]:
        lines.append(
            f"| {step['name']} | {step['status']} | {step['elapsed_seconds']} |"
        )
    lines.extend(
        [
            "",
            "## JSON Artifacts",
            "",
            "| artifact | status | schema | bytes | sha256 |",
            "|---|---|---|---:|---|",
        ]
    )
    for row in payload["json_artifacts"]:
        lines.append(
            f"| {row['artifact']} | {row['status']} | {row.get('schema') or '-'} | "
            f"{row.get('bytes') or '-'} | {row.get('sha256') or '-'} |"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--timing-iterations", type=int, default=10_000)
    parser.add_argument("--skip-timing", action="store_true")
    parser.add_argument("--replay-exact", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    args = parser.parse_args()

    if args.timing_iterations <= 0:
        raise SystemExit("--timing-iterations must be positive")

    steps = [
        run_step(step, args.dry_run)
        for step in build_steps(args.timing_iterations, args.skip_timing, args.replay_exact)
    ]
    json_artifacts = validate_json_artifacts(expected_json_artifacts())
    failed_step_count = sum(1 for step in steps if step["status"] == "fail")
    invalid_json_count = sum(
        1 for row in json_artifacts if row["status"] != "valid"
    )
    hashed_json_count = sum(1 for row in json_artifacts if row.get("sha256"))
    status = (
        "dry_run"
        if args.dry_run
        else "pass"
        if failed_step_count == 0 and invalid_json_count == 0
        else "fail"
    )
    payload = {
        "schema": "Descender.review_gate_replay_report.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "environment": {
            "platform": platform.platform(),
            "python_version": platform.python_version(),
        },
        "timing_iterations": args.timing_iterations,
        "skip_timing": args.skip_timing,
        "replay_exact": args.replay_exact,
        "dry_run": args.dry_run,
        "status": status,
        "failed_step_count": failed_step_count,
        "invalid_json_artifact_count": invalid_json_count,
        "hashed_json_artifact_count": hashed_json_count,
        "steps": steps,
        "json_artifacts": json_artifacts,
    }
    args.json.parent.mkdir(parents=True, exist_ok=True)
    args.json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    args.md.write_text(report_markdown(payload))
    print(json.dumps({"json": str(args.json), "markdown": str(args.md), "status": status}, indent=2))
    if status == "fail":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
