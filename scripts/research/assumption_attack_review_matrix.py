#!/usr/bin/env python3
"""Generate an assumption-level attack-review matrix.

This is a conservative review index.  It does not turn memo-only assumptions
into proofs; it records which keygen/security assumptions currently have
executable attack/replay evidence and which remain external-review or
implementation-hardening obligations.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
KEYGEN_SURFACE = READINESS_DIR / "keygen_security_assumption_surface_n32_q12289.json"
QUOTIENT_ATTACK_REVIEW = (
    READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json"
)
RESPONSE_LINK_ATTACK_REVIEW = (
    READINESS_DIR / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json"
)
RESPONSE_LINK_MANIFEST = (
    READINESS_DIR
    / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
)
QROM_SECURITY_LOSS_REVIEW = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
CONDITIONAL_SOUNDNESS_BUDGET_REVIEW = (
    READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.json"
)
PAD_SAMPLER_BOUNDARY_REVIEW = (
    READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json"
)
DEFAULT_JSON = READINESS_DIR / "assumption_attack_review_matrix_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "assumption_attack_review_matrix_n32_q12289.md"


def load_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def artifact_status(path: Path) -> str:
    return "present" if path.exists() else "missing"


def row(
    assumption: dict[str, Any],
    review_class: str,
    local_review_status: str,
    executable_evidence: list[Path],
    evidence: dict[str, Any],
    residual_risk: str,
    next_review_step: str,
) -> dict[str, Any]:
    return {
        "assumption_id": assumption["assumption_id"],
        "external_status": assumption["external_status"],
        "review_class": review_class,
        "local_review_status": local_review_status,
        "executable_evidence": [rel(path) for path in executable_evidence],
        "executable_evidence_status": {
            rel(path): artifact_status(path) for path in executable_evidence
        },
        "evidence": evidence,
        "residual_risk": residual_risk,
        "next_review_step": next_review_step,
    }


def build_rows(keygen_surface: dict[str, Any]) -> list[dict[str, Any]]:
    quotient = load_json(QUOTIENT_ATTACK_REVIEW) or {}
    response_attack = load_json(RESPONSE_LINK_ATTACK_REVIEW) or {}
    response_manifest = load_json(RESPONSE_LINK_MANIFEST) or {}
    qrom_review = load_json(QROM_SECURITY_LOSS_REVIEW) or {}
    soundness_budget = load_json(CONDITIONAL_SOUNDNESS_BUDGET_REVIEW) or {}
    pad_review = load_json(PAD_SAMPLER_BOUNDARY_REVIEW) or {}
    assumptions = {
        item["assumption_id"]: item for item in keygen_surface["assumptions"]
    }
    rows: list[dict[str, Any]] = []

    matrix = assumptions["matrix_sampler_and_public_key_distribution"]
    rows.append(
        row(
            matrix,
            "executable_lattice_sanity_plus_estimator_memo",
            "local_attack_harness_present_external_estimator_open",
            [QUOTIENT_ATTACK_REVIEW],
            {
                "quotient_attack_status": quotient.get("status"),
                "matrix_transcript_sha256": quotient.get("statement", {}).get(
                    "matrix_transcript_sha256"
                ),
                "rank": quotient.get("statement", {}).get("rank"),
                "kernel_dimension": quotient.get("statement", {}).get(
                    "kernel_dimension"
                ),
                "sparse_zero_relations_found": quotient.get(
                    "sparse_signed_relation_probe", {}
                ).get("zero_relations_found"),
                "shortest_reduction_coeff_bound": (
                    quotient.get("shortest_reduction_row") or {}
                ).get("coeff_bound"),
            },
            (
                "The local quotient/SIS harness checks obvious short relations "
                "for the exact matrix but is not an independent lattice-estimator "
                "security claim for keygen."
            ),
            (
                "Replay the matrix derivation independently, then run a standard "
                "SIS/lattice-estimator analysis for the exact public distribution."
            ),
        )
    )

    bare = assumptions["bare_quotient_kernel_control"]
    rows.append(
        row(
            bare,
            "negative_control",
            "local_negative_control_rejected",
            [QUOTIENT_ATTACK_REVIEW],
            {
                "shortest_coeff_bound": (quotient.get("shortest_reduction_row") or {}).get(
                    "coeff_bound"
                ),
                "inside_response_difference_bound_96": (
                    quotient.get("shortest_reduction_row") or {}
                ).get("inside_response_difference_bound_96"),
                "caught_by_response_link_code_0_3": (
                    quotient.get("shortest_reduction_row") or {}
                ).get("caught_by_response_link_code_0_3"),
            },
            (
                "This is intentionally not a deployable path; any verifier using "
                "the bare quotient relation is a regression."
            ),
            "Keep this row as a CI/review guard against accidentally re-enabling the bare quotient verifier.",
        )
    )

    response = assumptions["bounded_response_link_kernel"]
    rows.append(
        row(
            response,
            "exact_certificate_plus_executable_attack_review",
            "local_certificate_verified_external_replay_open",
            [RESPONSE_LINK_MANIFEST, RESPONSE_LINK_ATTACK_REVIEW],
            {
                "manifest_payload_sha256": response_manifest.get(
                    "manifest_payload_sha256"
                ),
                "replay_status": response_manifest.get("replay", {}).get("status"),
                "attack_review_status": response_attack.get("status"),
                "accepted_bound": response_attack.get("geometry", {}).get(
                    "accepted_box_coeff_bound"
                ),
                "shortest_cover_ball_vector_coeff_bound": response_attack.get(
                    "geometry", {}
                ).get("shortest_cover_ball_vector_coeff_bound"),
                "margin_over_bound": response_attack.get("geometry", {}).get(
                    "shortest_cover_ball_vector_margin_over_box_bound"
                ),
            },
            (
                "This is the strongest local cryptanalytic artifact, but it is "
                "still local; deployment needs independent replay of the exact "
                "certificate and preferably a separate SVP/BKZ sanity run."
            ),
            "Send the manifest and verifier to an independent reviewer for replay and one independent geometry check.",
        )
    )

    qrom = assumptions["fiat_shamir_qrom_and_challenge_budget"]
    rows.append(
        row(
            qrom,
            "checkable_qrom_budget_review",
            "security_loss_checks_passed_external_qrom_model_review_open"
            if qrom_review.get("status") == "checks_passed"
            else "security_loss_checks_missing_or_failed",
            [QROM_SECURITY_LOSS_REVIEW, CONDITIONAL_SOUNDNESS_BUDGET_REVIEW],
            {
                "security_loss_review_status": qrom_review.get("status"),
                "conditional_soundness_budget_status": soundness_budget.get("status"),
                "conditional_soundness_budget_terms": len(
                    soundness_budget.get("budget_terms", [])
                ),
                "conditional_soundness_min_checked_term_bits": soundness_budget.get(
                    "local_check_summary", {}
                ).get("minimum_checked_term_bits"),
                "deployment_min_term_bits": qrom_review.get("derived_terms", {}).get(
                    "deployment_min_term_bits"
                ),
                "deployment_min_margin_bits": qrom_review.get("derived_terms", {}).get(
                    "deployment_min_margin_bits"
                ),
                "failed_checks": len(qrom_review.get("failed_checks", [])),
                "challenge_bytes": qrom.get("evidence", {}).get("challenge_bytes"),
                "total_challenge_bits": qrom.get("evidence", {}).get(
                    "total_challenge_bits"
                ),
                "deployment_quantum_hash_query_bound_log2": qrom.get("evidence", {}).get(
                    "deployment_quantum_hash_query_bound_log2"
                ),
                "transcript_term_bits_after_qrom_loss": qrom.get("evidence", {}).get(
                    "transcript_term_bits_after_qrom_loss"
                ),
            },
            (
                "The byte frame and concrete query-loss arithmetic now have a "
                "checkable review object, but the QROM fork/reprogramming model "
                "and acceptable blockchain query ledger still need external review."
            ),
            "Externally review the QROM game hop and whether the q <= 2^16 quantum challenge-frame ledger is adequate for deployment.",
        )
    )

    pad = assumptions["pad_prf_and_sampler_independence"]
    rows.append(
        row(
            pad,
            "checkable_pad_sampler_boundary_review",
            "boundary_checks_passed_external_prf_sampler_review_open"
            if pad_review.get("status") == "checks_passed"
            else "boundary_checks_missing_or_failed",
            [PAD_SAMPLER_BOUNDARY_REVIEW],
            {
                "boundary_review_status": pad_review.get("status"),
                "boundary_review_failed_checks": len(
                    pad_review.get("failed_checks", [])
                ),
                "pad_domain": pad.get("evidence", {}).get("pad_domain"),
                "pad_frame_bytes": pad.get("evidence", {}).get("pad_frame_bytes"),
                "sampler_name": pad.get("evidence", {}).get("sampler_name"),
                "retry_loop_required": pad.get("evidence", {}).get(
                    "retry_loop_required"
                ),
                "conditions_forbidden": pad.get("evidence", {}).get(
                    "conditions_forbidden", []
                ),
            },
            (
                "The concrete pad/sampler arithmetic and forbidden-condition "
                "boundary now have a checkable review object, but SHA-256 PRF "
                "behavior and final production sampler-code review remain external."
            ),
            "Externally review the SHA-256 pad PRF assumption and replay the same boundary checks against the final production crate.",
        )
    )

    lifecycle = assumptions["production_keygen_and_state_lifecycle"]
    rows.append(
        row(
            lifecycle,
            "deployment_hardening_gate",
            "fail_closed_preflight_present_not_deployment_ready",
            [],
            {
                "deployment_preflight_present": lifecycle.get("evidence", {}).get(
                    "deployment_preflight_present"
                ),
                "deployment_preflight_ready": lifecycle.get("evidence", {}).get(
                    "deployment_preflight_ready"
                ),
                "deployment_preflight_blockers": lifecycle.get("evidence", {}).get(
                    "deployment_preflight_blockers", []
                ),
                "executable_signature_bytes": lifecycle.get("evidence", {}).get(
                    "executable_signature_bytes"
                ),
                "reviewed_target_transaction_bytes": lifecycle.get("evidence", {}).get(
                    "reviewed_target_transaction_bytes"
                ),
                "constant_time_claim": lifecycle.get("evidence", {}).get(
                    "constant_time_claim"
                ),
            },
            (
                "This is not primarily a lattice attack surface; the remaining "
                "risk is implementation soundness, constant-time behavior, state "
                "lifecycle correctness, and replacing the 254-byte callback path."
            ),
            "Close the fail-closed deployment blockers before claiming deployable cryptography.",
        )
    )

    return rows


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Assumption Attack Review Matrix",
        "",
        "This generated matrix widens the attack-review view from the response-link",
        "certificate to every keygen/security assumption tracked by the readiness",
        "surface.  It is an index of evidence and residual risk, not a proof.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- assumptions mapped: `{payload['summary']['assumptions_mapped']}`",
        f"- executable-backed rows: `{payload['summary']['executable_backed_rows']}`",
        f"- memo-only rows: `{payload['summary']['memo_only_rows']}`",
        f"- deployment-ready rows: `{payload['summary']['deployment_ready_rows']}`",
        "",
        "## Matrix",
        "",
        "| assumption | class | local status | next review step |",
        "|---|---|---|---|",
    ]
    for item in payload["rows"]:
        lines.append(
            f"| {item['assumption_id']} | {item['review_class']} | "
            f"{item['local_review_status']} | {item['next_review_step']} |"
        )
    lines.extend(["", "## Residual Risks", ""])
    for item in payload["rows"]:
        lines.append(f"- `{item['assumption_id']}`: {item['residual_risk']}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    keygen_surface = load_json(KEYGEN_SURFACE)
    if keygen_surface is None:
        raise SystemExit(f"missing keygen surface: {KEYGEN_SURFACE}")
    rows = build_rows(keygen_surface)
    executable_backed = sum(
        1 for item in rows if item["executable_evidence"] and "memo_only" not in item["review_class"]
    )
    memo_only = sum(1 for item in rows if "memo_only" in item["review_class"])
    deployment_ready = sum(
        1
        for item in rows
        if item["evidence"].get("deployment_preflight_ready") is True
    )
    payload = {
        "schema": "Descender.assumption_attack_review_matrix.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source_surface": rel(KEYGEN_SURFACE),
        "summary": {
            "assumptions_mapped": len(rows),
            "executable_backed_rows": executable_backed,
            "memo_only_rows": memo_only,
            "deployment_ready_rows": deployment_ready,
        },
        "rows": rows,
    }
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
