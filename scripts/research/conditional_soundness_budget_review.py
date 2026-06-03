#!/usr/bin/env python3
"""Generate the conditional deployed-soundness budget review.

This aggregates the local check objects into one reviewable game ledger.  It is
not an unconditional security proof: each named break remains an external
cryptographic/model assumption unless a local exact certificate discharges it.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
QROM_REVIEW = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
PAD_REVIEW = READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json"
RESPONSE_MANIFEST = (
    READINESS_DIR
    / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
)
RESPONSE_ATTACK = (
    READINESS_DIR / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json"
)
PRODUCTION_GAP = READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
ASSUMPTION_MATRIX = READINESS_DIR / "assumption_attack_review_matrix_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.md"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def term(
    break_name: str,
    adversary_class: str,
    no_break_claim: str,
    evidence: dict[str, Any],
    query_limits: dict[str, Any],
    local_status: str,
    charged_bound: str,
    residual_assumption: str,
) -> dict[str, Any]:
    return {
        "break_name": break_name,
        "adversary_class": adversary_class,
        "no_break_claim": no_break_claim,
        "evidence": evidence,
        "query_limits": query_limits,
        "local_status": local_status,
        "charged_bound": charged_bound,
        "residual_assumption": residual_assumption,
    }


def build_payload() -> dict[str, Any]:
    qrom = load_json(QROM_REVIEW)
    pad = load_json(PAD_REVIEW)
    response_manifest = load_json(RESPONSE_MANIFEST)
    response_attack = load_json(RESPONSE_ATTACK)
    production_gap = load_json(PRODUCTION_GAP)
    matrix = load_json(ASSUMPTION_MATRIX)

    qrom_terms = qrom["derived_terms"]
    qrom_ledger = qrom["adversary_ledger"]
    deployment_preflight = production_gap["deployment_preflight"]
    response_replay = response_manifest.get("replay", {})
    response_geometry = response_attack.get("geometry", {})

    pad_seed_bits = 256
    pad_quantum_query_log2 = 64
    pad_quantum_term_bits = pad_seed_bits - 2 * pad_quantum_query_log2
    qrom_min_term_bits = int(qrom_terms["deployment_min_term_bits"])
    response_margin = int(response_geometry.get("shortest_cover_ball_vector_margin_over_box_bound", 0))
    frame_min_term_bits = min(
        int(qrom_terms["quantum_transcript_term_bits"]),
        int(qrom_terms["classical_transcript_term_bits"]),
    )

    budget_terms = [
        term(
            "noExtractionBreak",
            "committed-sigma extractor adversary producing an accepting account-carried transcript without an extractable hidden witness",
            "noExtractionBreak => no_extractionFailure",
            {
                "qrom_security_loss_status": qrom["status"],
                "response_link_replay_status": response_replay.get("status"),
                "response_link_attack_status": response_attack.get("status"),
                "response_link_margin_over_bound": response_margin,
            },
            {
                "challenge_frame_quantum_queries_log2": qrom_ledger[
                    "deployment_quantum_hash_query_bound_log2"
                ],
                "challenge_frame_classical_queries_log2": qrom_ledger[
                    "deployment_classical_hash_query_bound_log2"
                ],
            },
            "local_qrom_arithmetic_and_response_link_certificate_checks_passed",
            f"Adv_extraction_break + transcript/action term at >= {qrom_min_term_bits} bits",
            "external review of the QROM fork/reprogramming game and independent response-link replay",
        ),
        term(
            "noWitnessHidingBreak",
            "public-view distinguisher against the hidden representative/account-carried witness distribution",
            "noWitnessHidingBreak => witnessHiding",
            {
                "pad_sampler_boundary_status": pad["status"],
                "pad_failed_checks": len(pad["failed_checks"]),
                "sampler_tv_distance": "0 under the reviewed conditioned-fiber sampler certificate",
            },
            {
                "samples_per_stateful_slot": 1,
                "representative_retry_loop": "none",
            },
            "local_boundary_checks_passed_zero_tv_claim_remains_certificate_scoped",
            "Adv_witness_hiding = 0 under the local sampler certificate and final API-boundary preservation",
            "external review that the final production sampler preserves the checked forbidden-condition boundary",
        ),
        term(
            "noPadPRFBreak",
            "oracle distinguisher for SHA-256-derived local pad residues",
            "noPadPRFBreak => pseudorandomLocalPad",
            {
                "pad_sampler_boundary_status": pad["status"],
                "pad_frame_check": "domain || block-index, counter u32_le, cutoff floor(2^16/q)*q",
                "pad_seed_bits": pad_seed_bits,
            },
            {
                "pad_quantum_queries_log2": pad_quantum_query_log2,
            },
            "local_domain_and_rejection_arithmetic_checks_passed",
            f"Adv_pad_prf, modeled with {pad_quantum_term_bits} quantum bits at q <= 2^{pad_quantum_query_log2}",
            "SHA-256 pad expansion pseudorandomness under the named PadPRFBinding assumption",
        ),
        term(
            "noFrameMismatchBreak",
            "byte-level adversary causing verifier/proof-oracle challenge-frame disagreement",
            "noFrameMismatchBreak => fiatShamirFrameAgreement",
            {
                "qrom_frame_checks_status": qrom["status"],
                "frame_failed_checks": len(qrom["failed_checks"]),
                "proof_commitment_is_prechallenge": True,
                "verifier_recomputes_frame": True,
            },
            {
                "challenge_frame_quantum_queries_log2": qrom_ledger[
                    "deployment_quantum_hash_query_bound_log2"
                ],
                "challenge_frame_classical_queries_log2": qrom_ledger[
                    "deployment_classical_hash_query_bound_log2"
                ],
            },
            "local_byte_frame_checks_passed",
            f"Adv_frame_mismatch + frame collision term at >= {frame_min_term_bits} bits",
            "external review that final production verifier uses the same byte frame before acceptance",
        ),
        term(
            "matrixKeygenBreak",
            "estimator adversary exploiting the SHA-256 rejection-sampled q-ary matrix/public-view distribution",
            "noMatrixKeygenBreak => publicMatrixDistributionAccepted",
            {
                "assumption_matrix_row_status": next(
                    row["local_review_status"]
                    for row in matrix["rows"]
                    if row["assumption_id"] == "matrix_sampler_and_public_key_distribution"
                ),
                "matrix_transcript_sha256": next(
                    row["evidence"]["matrix_transcript_sha256"]
                    for row in matrix["rows"]
                    if row["assumption_id"] == "matrix_sampler_and_public_key_distribution"
                ),
            },
            {},
            "local_lattice_sanity_harness_present",
            "Adv_matrix_keygen_estimator",
            "independent SIS/lattice-estimator review for the exact public key distribution",
        ),
    ]
    failed_local_checks = [
        source
        for source, status in [
            ("qrom_security_loss_review", qrom["status"]),
            ("pad_sampler_boundary_review", pad["status"]),
            ("response_link_manifest_replay", response_replay.get("status")),
        ]
        if status not in {"checks_passed", "verified"}
    ]
    external_obligations = [
        term["residual_assumption"] for term in budget_terms
    ] + deployment_preflight["blockers"]
    unique_external_obligations = list(dict.fromkeys(external_obligations))
    return {
        "schema": "Descender.conditional_soundness_budget_review.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "sources": [
            rel(QROM_REVIEW),
            rel(PAD_REVIEW),
            rel(RESPONSE_MANIFEST),
            rel(RESPONSE_ATTACK),
            rel(PRODUCTION_GAP),
            rel(ASSUMPTION_MATRIX),
        ],
        "target": {
            "profile": "Descender.Signature.q12289.local_delta.stateful.v1",
            "transaction_target_bytes": 250,
            "account_link_bytes": 68,
            "target_security_bits": 128,
        },
        "top_level_formula": (
            "Adv_forge <= Adv_extraction_break + Adv_witness_hiding + "
            "Adv_pad_prf + Adv_frame_mismatch + Adv_matrix_keygen + "
            "transcript/action collision terms"
        ),
        "budget_terms": budget_terms,
        "local_check_summary": {
            "qrom_security_loss_status": qrom["status"],
            "pad_sampler_boundary_status": pad["status"],
            "response_link_replay_status": response_replay.get("status"),
            "response_link_attack_status": response_attack.get("status"),
            "failed_local_checks": failed_local_checks,
            "all_local_checks_passed": not failed_local_checks,
            "minimum_checked_term_bits": min(qrom_min_term_bits, pad_quantum_term_bits),
        },
        "deployment_preflight": deployment_preflight,
        "external_obligations": unique_external_obligations,
        "status": (
            "local_budget_checks_passed_external_review_open"
            if not failed_local_checks
            else "local_budget_checks_failed"
        ),
        "deployment_ready": False,
    }


def markdown(payload: dict[str, Any]) -> str:
    summary = payload["local_check_summary"]
    lines = [
        "# Conditional Soundness Budget Review",
        "",
        "This generated ledger aggregates the local check objects into the named",
        "break terms consumed by the conditional deployed-verifier theorem.  It is",
        "not an unconditional deployment claim.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- all local checks passed: `{summary['all_local_checks_passed']}`",
        f"- minimum checked term bits: `{summary['minimum_checked_term_bits']}`",
        f"- top-level formula: `{payload['top_level_formula']}`",
        "",
        "## Budget Terms",
        "",
        "| break | local status | charged bound | residual assumption |",
        "|---|---|---|---|",
    ]
    for item in payload["budget_terms"]:
        lines.append(
            f"| {item['break_name']} | {item['local_status']} | "
            f"{item['charged_bound']} | {item['residual_assumption']} |"
        )
    lines.extend(["", "## Deployment Preflight Blockers", ""])
    lines.extend(f"- {item}" for item in payload["deployment_preflight"]["blockers"])
    lines.extend(["", "## External Obligations", ""])
    lines.extend(f"- {item}" for item in payload["external_obligations"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))
    if payload["status"] == "local_budget_checks_failed":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
