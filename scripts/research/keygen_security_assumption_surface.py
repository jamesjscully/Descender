#!/usr/bin/env python3
"""Generate the keygen/security-estimator assumption surface.

This report is deliberately conservative.  It does not estimate concrete
security by itself; it names the assumptions that a third-party estimator or
reviewer must close and links them to the local evidence already present in the
repository.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
RESPONSE_LINK_MANIFEST = (
    READINESS_DIR
    / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
)
RESPONSE_LINK_ATTACK_REVIEW = (
    READINESS_DIR
    / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json"
)
QUOTIENT_SIS_ATTACK_REVIEW = (
    READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json"
)
MATRIX_KEYGEN_ESTIMATOR_MEMO = (
    READINESS_DIR / "matrix_keygen_estimator_memo_n32_q12289.json"
)
REFERENCE_TIMING_REPORT = READINESS_DIR / "reference_timing_report_n32_q12289.json"
PRODUCTION_REFERENCE_GAP_REPORT = (
    READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
)
QROM_CHALLENGE_BUDGET_MEMO = READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.json"
QROM_SECURITY_LOSS_REVIEW = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
CONDITIONAL_SOUNDNESS_BUDGET_REVIEW = (
    READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.json"
)
PAD_SAMPLER_BOUNDARY_MEMO = READINESS_DIR / "pad_sampler_boundary_memo_n32_q12289.json"
PAD_SAMPLER_BOUNDARY_REVIEW = (
    READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json"
)
PRODUCTION_LIFECYCLE_HARDENING_MEMO = (
    READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.json"
)
BLOCKCHAIN_DEPLOYMENT_INTEGRATION_MEMO = (
    READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json"
)
CONSTANT_TIME_SIDE_CHANNEL_AUDIT_MEMO = (
    READINESS_DIR / "constant_time_side_channel_audit_memo_n32_q12289.json"
)
DEFAULT_JSON = READINESS_DIR / "keygen_security_assumption_surface_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "keygen_security_assumption_surface_n32_q12289.md"


def load_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def artifact_status(path: Path) -> str:
    return "present" if path.exists() else "missing"


def row(
    assumption_id: str,
    statement: str,
    local_status: str,
    external_status: str,
    artifacts: list[Path],
    evidence: dict[str, Any],
    required_review: str,
) -> dict[str, Any]:
    return {
        "assumption_id": assumption_id,
        "statement": statement,
        "local_status": local_status,
        "external_status": external_status,
        "artifacts": [rel(path) for path in artifacts],
        "artifact_status": {rel(path): artifact_status(path) for path in artifacts},
        "evidence": evidence,
        "required_review": required_review,
    }


def assumption_rows() -> list[dict[str, Any]]:
    response_manifest = load_json(RESPONSE_LINK_MANIFEST)
    response_attack = load_json(RESPONSE_LINK_ATTACK_REVIEW)
    quotient_review = load_json(QUOTIENT_SIS_ATTACK_REVIEW)
    matrix_memo = load_json(MATRIX_KEYGEN_ESTIMATOR_MEMO)
    timing_report = load_json(REFERENCE_TIMING_REPORT)
    production_gap = load_json(PRODUCTION_REFERENCE_GAP_REPORT)
    qrom_memo = load_json(QROM_CHALLENGE_BUDGET_MEMO)
    qrom_review = load_json(QROM_SECURITY_LOSS_REVIEW)
    soundness_budget = load_json(CONDITIONAL_SOUNDNESS_BUDGET_REVIEW)
    pad_memo = load_json(PAD_SAMPLER_BOUNDARY_MEMO)
    pad_review = load_json(PAD_SAMPLER_BOUNDARY_REVIEW)
    lifecycle_memo = load_json(PRODUCTION_LIFECYCLE_HARDENING_MEMO)
    deployment_memo = load_json(BLOCKCHAIN_DEPLOYMENT_INTEGRATION_MEMO)
    side_channel_memo = load_json(CONSTANT_TIME_SIDE_CHANNEL_AUDIT_MEMO)

    rows: list[dict[str, Any]] = []

    quotient_shortest = (quotient_review or {}).get("shortest_reduction_row", {})
    rows.append(
        row(
            "matrix_sampler_and_public_key_distribution",
            (
                "The SHA-256 rejection-sampled q-ary matrix/public-view "
                "distribution used by keygen matches the reviewed n=32, q=12289 "
                "relation and has no exploitable sampler bias."
            ),
            "matrix_keygen_estimator_memo_generated"
            if matrix_memo
            else ("local_artifacts_present" if quotient_review else "missing_local_artifact"),
            "open_external_estimator",
            [MATRIX_KEYGEN_ESTIMATOR_MEMO, QUOTIENT_SIS_ATTACK_REVIEW],
            {
                "matrix_domain": (matrix_memo or {})
                .get("matrix_sampler", {})
                .get(
                    "domain",
                    (quotient_review or {}).get("statement", {}).get("matrix_domain"),
                ),
                "matrix_transcript_sha256": (matrix_memo or {})
                .get("matrix_sampler", {})
                .get(
                    "matrix_transcript_sha256",
                    (quotient_review or {})
                    .get("statement", {})
                    .get("matrix_transcript_sha256"),
                ),
                "rank": (matrix_memo or {})
                .get("matrix_sampler", {})
                .get("rank", (quotient_review or {}).get("statement", {}).get("rank")),
                "kernel_dimension": (matrix_memo or {})
                .get("matrix_sampler", {})
                .get(
                    "kernel_dimension",
                    (quotient_review or {}).get("statement", {}).get("kernel_dimension"),
                ),
                "short_bare_quotient_vector_coeff_bound": (matrix_memo or {})
                .get("negative_control_vector", {})
                .get("coeff_bound"),
                "bare_vector_caught_by_response_link_code_0_3": (matrix_memo or {})
                .get("negative_control_vector", {})
                .get("caught_by_response_link_code_0_3"),
                "sparse_zero_relations_found": (quotient_review or {})
                .get("sparse_signed_relation_probe", {})
                .get("zero_relations_found"),
            },
            (
                "Independently reconstruct the matrix sampler, check rejection "
                "sampling bias bounds, and run a standard lattice-estimator/SIS "
                "cost analysis for the exact public key distribution."
            ),
        )
    )

    rows.append(
        row(
            "bare_quotient_kernel_control",
            (
                "The full public quotient kernel has short bounded vectors, so "
                "bare quotient verification is intentionally rejected and must "
                "not be used without the account response-link code."
            ),
            "negative_control_confirmed"
            if quotient_shortest.get("inside_response_difference_bound_96") is True
            else "review_needed",
            "closed_as_negative_control",
            [QUOTIENT_SIS_ATTACK_REVIEW],
            {
                "shortest_coeff_bound": quotient_shortest.get("coeff_bound"),
                "inside_response_difference_bound_96": quotient_shortest.get(
                    "inside_response_difference_bound_96"
                ),
                "caught_by_response_link_code_0_3": quotient_shortest.get(
                    "caught_by_response_link_code_0_3"
                ),
                "response_link_coords_0_3_zero_mod_q": quotient_shortest.get(
                    "response_link_coords_0_3_zero_mod_q"
                ),
            },
            (
                "Keep this as a regression guard: any production verifier path "
                "that accepts the bare quotient relation should be considered a "
                "security failure."
            ),
        )
    )

    response_replay = (response_manifest or {}).get("replay", {})
    response_geometry = (response_attack or {}).get("geometry", {})
    rows.append(
        row(
            "bounded_response_link_kernel",
            (
                "No nonzero response-difference vector inside [-96,96]^23 lies "
                "in the quotient kernel while also vanishing on coordinates [0,3]."
            ),
            "exact_certificate_verified"
            if response_replay.get("status") == "verified"
            else "review_needed",
            "open_independent_replay",
            [RESPONSE_LINK_MANIFEST, RESPONSE_LINK_ATTACK_REVIEW],
            {
                "manifest_payload_sha256": (response_manifest or {}).get(
                    "manifest_payload_sha256"
                ),
                "replay_status": response_replay.get("status"),
                "replay_mismatches": response_replay.get("mismatches"),
                "visited_leaves": response_replay.get("visited_leaves"),
                "attack_review_status": (response_attack or {}).get("status"),
                "shortest_reduced_row_coeff_bound": response_geometry.get(
                    "shortest_cover_ball_vector_coeff_bound"
                ),
                "accepted_bound": response_geometry.get("accepted_box_coeff_bound"),
                "margin_over_bound": response_geometry.get(
                    "shortest_cover_ball_vector_margin_over_box_bound"
                ),
            },
            (
                "Have an independent reviewer replay the manifest and reproduce "
                "at least one of the matrix derivation, canonical basis, exact "
                "enumeration, or SVP/BKZ sanity checks."
            ),
        )
    )

    rows.append(
        row(
            "fiat_shamir_qrom_and_challenge_budget",
            (
                "The proof-bound Fiat-Shamir frame is modeled as a QROM random "
                "oracle with the recorded challenge entropy/query budget."
            ),
            "qrom_memo_generated" if qrom_memo else "named_assumption_only",
            "open_external_qrom_review",
            [
                QROM_CHALLENGE_BUDGET_MEMO,
                QROM_SECURITY_LOSS_REVIEW,
                CONDITIONAL_SOUNDNESS_BUDGET_REVIEW,
                PRODUCTION_REFERENCE_GAP_REPORT,
            ],
            {
                "security_loss_review_status": (qrom_review or {}).get("status"),
                "security_loss_failed_checks": len(
                    (qrom_review or {}).get("failed_checks", [])
                ),
                "deployment_min_term_bits": (qrom_review or {})
                .get("derived_terms", {})
                .get("deployment_min_term_bits"),
                "deployment_min_margin_bits": (qrom_review or {})
                .get("derived_terms", {})
                .get("deployment_min_margin_bits"),
                "conditional_soundness_budget_status": (soundness_budget or {}).get(
                    "status"
                ),
                "conditional_soundness_budget_terms": len(
                    (soundness_budget or {}).get("budget_terms", [])
                ),
                "conditional_soundness_min_checked_term_bits": (soundness_budget or {})
                .get("local_check_summary", {})
                .get("minimum_checked_term_bits"),
                "profile": (qrom_memo or {})
                .get("challenge_frame", {})
                .get("profile", "Descender.Signature.q12289.local_delta.stateful.v1"),
                "domain": (qrom_memo or {}).get("challenge_frame", {}).get("domain"),
                "challenge_bytes": (qrom_memo or {})
                .get("challenge_derivation", {})
                .get("output_bytes_taken", 21),
                "algebraic_challenge_bits": (qrom_memo or {})
                .get("challenge_derivation", {})
                .get("algebraic_challenge_bits", 130),
                "hardening_bits": (qrom_memo or {})
                .get("challenge_derivation", {})
                .get("hardening_bits", 32),
                "total_challenge_bits": (qrom_memo or {})
                .get("challenge_derivation", {})
                .get("total_transcript_challenge_bits", 162),
                "deployment_quantum_hash_query_bound_log2": (qrom_memo or {})
                .get("query_budget", {})
                .get("deployment_quantum_hash_query_bound_log2"),
                "transcript_term_bits_after_qrom_loss": (qrom_memo or {})
                .get("query_budget", {})
                .get("transcript_term_bits_after_qrom_loss"),
            },
            (
                "Externally review the QROM memo's byte frame, domain labels, "
                "adversary query interface, and allowed signing query budget."
            ),
        )
    )

    rows.append(
        row(
            "pad_prf_and_sampler_independence",
            (
                "SHA-256 pad expansion and the hidden-response sampler are "
                "treated as secret-independent except for the reviewed "
                "conditioned quotient-fiber predicate."
            ),
            "pad_sampler_boundary_memo_generated"
            if pad_memo
            else "named_assumption_only",
            "open_external_sampler_code_review",
            [
                PAD_SAMPLER_BOUNDARY_MEMO,
                PAD_SAMPLER_BOUNDARY_REVIEW,
                PRODUCTION_REFERENCE_GAP_REPORT,
            ],
            {
                "boundary_review_status": (pad_review or {}).get("status"),
                "boundary_review_failed_checks": len(
                    (pad_review or {}).get("failed_checks", [])
                ),
                "pad_domain": (pad_memo or {})
                .get("pad_prf_assumption", {})
                .get("domain", "Descender.Signature.q12289.local_delta.pad.v1"),
                "pad_frame_bytes": (pad_memo or {})
                .get("pad_prf_assumption", {})
                .get("pad_frame_bytes"),
                "pad_seed_bytes": (pad_memo or {})
                .get("pad_prf_assumption", {})
                .get("pad_seed_bytes"),
                "counter_encoding": (pad_memo or {})
                .get("pad_prf_assumption", {})
                .get("counter_encoding"),
                "sampler_name": (pad_memo or {})
                .get("conditioned_quotient_fiber_sampler", {})
                .get("sampler_name"),
                "sampled_free_residues": (pad_memo or {})
                .get("conditioned_quotient_fiber_sampler", {})
                .get("sampled_free_residues"),
                "retry_loop_required": (pad_memo or {})
                .get("conditioned_quotient_fiber_sampler", {})
                .get("retry_loop_required"),
                "conditions_forbidden": (pad_memo or {})
                .get("conditioned_quotient_fiber_sampler", {})
                .get("conditions_forbidden", []),
            },
            (
                "Externally review the pad/sampler boundary memo and audit that "
                "active pad/local opening equations are composed outside the "
                "hidden representative sampler."
            ),
        )
    )

    timing = (timing_report or {}).get("timing_report", {})
    production_facade = (production_gap or {}).get("production_facade", {})
    deployment_preflight = (production_gap or {}).get("deployment_preflight", {})
    rows.append(
        row(
            "production_keygen_and_state_lifecycle",
            (
                "Production keygen, account-state reservation, and slot "
                "consumption must preserve the reviewed account link while "
                "avoiding secret-dependent timing and state-reuse failures."
            ),
            "production_lifecycle_hardening_memo_generated"
            if lifecycle_memo
            else (
                "facade_smoke_tested"
                if timing_report and production_gap
                else "missing_local_artifact"
            ),
            "open_production_hardening",
            [
                PRODUCTION_LIFECYCLE_HARDENING_MEMO,
                BLOCKCHAIN_DEPLOYMENT_INTEGRATION_MEMO,
                CONSTANT_TIME_SIDE_CHANNEL_AUDIT_MEMO,
                REFERENCE_TIMING_REPORT,
                PRODUCTION_REFERENCE_GAP_REPORT,
            ],
            {
                "timing_signature_bytes": timing.get("signature_bytes"),
                "production_facade_keygen_avg_us": next(
                    (
                        row.get("avg_us")
                        for row in timing.get("timings", [])
                        if row.get("operation") == "production_facade_keygen"
                    ),
                    None,
                ),
                "production_facade_sign_avg_us": next(
                    (
                        row.get("avg_us")
                        for row in timing.get("timings", [])
                        if row.get("operation") == "production_facade_sign"
                    ),
                    None,
                ),
                "production_facade_verify_avg_us": next(
                    (
                        row.get("avg_us")
                        for row in timing.get("timings", [])
                        if row.get("operation") == "production_facade_verify"
                    ),
                    None,
                ),
                "account_link_bytes": production_facade.get("account_commitment_bytes"),
                "executable_signature_bytes": production_facade.get(
                    "executable_signature_bytes"
                ),
                "reviewed_target_transaction_bytes": production_facade.get(
                    "target_transaction_signature_bytes"
                ),
                "executable_matches_target": production_facade.get(
                    "executable_matches_target"
                ),
                "lifecycle_requirements_count": len(
                    (lifecycle_memo or {}).get("lifecycle_requirements", [])
                ),
                "constant_time_requirements_count": len(
                    (lifecycle_memo or {}).get("constant_time_requirements", [])
                ),
                "open_blockers": (lifecycle_memo or {}).get("open_blockers", []),
                "deployment_bindings_count": len(
                    (deployment_memo or {}).get("required_deployment_bindings", [])
                ),
                "deployment_not_claims": (deployment_memo or {}).get(
                    "still_not_deployment_claims", []
                ),
                "constant_time_claim": (side_channel_memo or {}).get("current_claim"),
                "side_channel_audited_surfaces_count": len(
                    (side_channel_memo or {}).get("audited_surfaces", [])
                ),
                "side_channel_open_blockers": (side_channel_memo or {}).get(
                    "open_blockers", []
                ),
                "deployment_preflight_present": deployment_preflight.get(
                    "present_in_library"
                ),
                "deployment_preflight_ready": deployment_preflight.get(
                    "deployment_ready"
                ),
                "deployment_preflight_blockers": deployment_preflight.get(
                    "blockers", []
                ),
                "secret_material_forbidden_count": len(
                    (side_channel_memo or {})
                    .get("secret_material_classification", {})
                    .get("forbidden_in_logs_stores_errors", [])
                ),
            },
            (
                "Externally review the lifecycle hardening, blockchain "
                "deployment integration, and constant-time/side-channel audit "
                "memos; split the hardened reference crate, implement accepting "
                "308-byte/Fq12 projection-carrier fallback semantics, and "
                "perform constant-time/state-lifecycle implementation review."
            ),
        )
    )

    return rows


def markdown(payload: dict[str, Any]) -> str:
    production_lifecycle = next(
        (
            item
            for item in payload["assumptions"]
            if item["assumption_id"] == "production_keygen_and_state_lifecycle"
        ),
        {},
    )
    deployment_evidence = production_lifecycle.get("evidence", {})
    deployment_preflight_blockers = deployment_evidence.get(
        "deployment_preflight_blockers", []
    )
    lines = [
        "# Keygen Security Assumption Surface",
        "",
        "This report names the remaining keygen/security-estimator assumptions for",
        "the account-carried committed-sigma target.  It is not a security proof;",
        "it is a review map from local evidence to the external work still needed.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- assumptions tracked: `{len(payload['assumptions'])}`",
        f"- local artifacts missing: `{payload['summary']['missing_local_artifacts']}`",
        f"- external review items open: `{payload['summary']['open_external_items']}`",
        f"- deployment preflight present: `{deployment_evidence.get('deployment_preflight_present', False)}`",
        f"- deployment preflight ready: `{deployment_evidence.get('deployment_preflight_ready', False)}`",
        f"- deployment preflight blockers: `{len(deployment_preflight_blockers)}`",
        "",
        "## Assumptions",
        "",
        "| assumption | local status | external status | required review |",
        "|---|---|---|---|",
    ]
    for item in payload["assumptions"]:
        lines.append(
            f"| {item['assumption_id']} | {item['local_status']} | "
            f"{item['external_status']} | {item['required_review']} |"
        )
    if deployment_preflight_blockers:
        lines.extend(["", "## Deployment Preflight Blockers", ""])
        lines.extend(f"- {blocker}" for blocker in deployment_preflight_blockers)
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The local response-link certificate is the strongest current evidence.",
            "The keygen/security-estimator surface is still not externally closed:",
            "matrix-sampler bias, SIS/keygen cost estimates, QROM query budgets,",
            "sampler-code boundaries, and production state lifecycle all need",
            "review before deployment claims.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    assumptions = assumption_rows()
    missing_local = sum(
        1
        for item in assumptions
        if any(status == "missing" for status in item["artifact_status"].values())
    )
    open_external = sum(
        1 for item in assumptions if item["external_status"].startswith("open")
    )
    payload = {
        "schema": "Descender.keygen_security_assumption_surface.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "target": {
            "profile": "Descender.Signature.q12289.local_delta.stateful.v1",
            "n": 32,
            "q": 12289,
            "transaction_target_bytes": 250,
            "account_link_bytes": 68,
        },
        "summary": {
            "missing_local_artifacts": missing_local,
            "open_external_items": open_external,
        },
        "assumptions": assumptions,
    }
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
