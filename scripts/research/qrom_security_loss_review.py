#!/usr/bin/env python3
"""Generate a checkable Fiat-Shamir/QROM security-loss review object.

This script consumes the QROM challenge-budget memo and turns its arithmetic
claims into explicit inequalities.  It remains conditional on the reviewed QROM
forking theorem and Poseidon2-as-QRO assumption; the purpose here is to remove
wiggle room from the concrete query ledger and byte-frame accounting.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
QROM_MEMO = READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "qrom_security_loss_review_n32_q12289.md"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def checked(condition: bool, description: str, evidence: dict[str, Any]) -> dict[str, Any]:
    return {
        "description": description,
        "holds": condition,
        "evidence": evidence,
    }


def build_payload() -> dict[str, Any]:
    memo = load_json(QROM_MEMO)
    frame = memo["challenge_frame"]
    derivation = memo["challenge_derivation"]
    budget = memo["query_budget"]

    target_bits = int(budget["target_security_bits"])
    total_bits = int(derivation["total_transcript_challenge_bits"])
    algebraic_bits = int(derivation["algebraic_challenge_bits"])
    hardening_bits = int(derivation["hardening_bits"])
    q_quantum_log2 = int(budget["deployment_quantum_hash_query_bound_log2"])
    q_classical_log2 = int(budget["deployment_classical_hash_query_bound_log2"])
    selected_multi_target_log2 = int(budget["selected_multi_target_horizon_log2"])
    selected_transcript_term_bits = int(budget["transcript_term_bits_after_qrom_loss"])
    minimum_deployment_margin_bits = int(budget["minimum_deployment_margin_bits"])

    quantum_transcript_term_bits = total_bits - 2 * q_quantum_log2
    classical_transcript_term_bits = total_bits - q_classical_log2
    single_term_quantum_log2 = (total_bits - target_bits) // 2
    single_term_classical_log2 = total_bits - target_bits
    deployment_min_term_bits = min(
        quantum_transcript_term_bits,
        classical_transcript_term_bits,
        algebraic_bits,
        selected_transcript_term_bits,
    )

    frame_fields = frame["fields"]
    labels = [field["label"] for field in frame_fields]
    secret_fields = [field["label"] for field in frame_fields if field["secret"]]
    required_labels = [
        "domain",
        "profile",
        "q",
        "groups",
        "local_pivot",
        "local_free",
        "repetitions",
        "slot_id",
        "public_key",
        "message",
        "slot_digest",
        "proof_commitment",
    ]

    inequalities = [
        checked(
            total_bits == algebraic_bits + hardening_bits,
            "total transcript challenge bits equal algebraic bits plus hardening bits",
            {
                "total_bits": total_bits,
                "algebraic_bits": algebraic_bits,
                "hardening_bits": hardening_bits,
            },
        ),
        checked(
            q_quantum_log2 <= single_term_quantum_log2,
            "deployment quantum query exponent fits the single-term 128-bit ledger",
            {
                "deployment_quantum_hash_query_bound_log2": q_quantum_log2,
                "single_term_quantum_query_bound_log2_for_128": single_term_quantum_log2,
            },
        ),
        checked(
            q_classical_log2 <= single_term_classical_log2,
            "deployment classical query exponent fits the single-term 128-bit ledger",
            {
                "deployment_classical_hash_query_bound_log2": q_classical_log2,
                "single_term_classical_query_bound_log2_for_128": single_term_classical_log2,
            },
        ),
        checked(
            quantum_transcript_term_bits >= target_bits,
            "QROM quadratic-loss transcript term remains above target",
            {
                "quantum_transcript_term_bits": quantum_transcript_term_bits,
                "target_bits": target_bits,
                "margin_bits": quantum_transcript_term_bits - target_bits,
            },
        ),
        checked(
            classical_transcript_term_bits >= target_bits,
            "classical transcript term remains above target",
            {
                "classical_transcript_term_bits": classical_transcript_term_bits,
                "target_bits": target_bits,
                "margin_bits": classical_transcript_term_bits - target_bits,
            },
        ),
        checked(
            algebraic_bits >= target_bits,
            "algebraic challenge-action space remains above target",
            {
                "algebraic_challenge_bits": algebraic_bits,
                "target_bits": target_bits,
                "margin_bits": algebraic_bits - target_bits,
            },
        ),
        checked(
            deployment_min_term_bits >= target_bits,
            "minimum deployment term remains above target",
            {
                "deployment_min_term_bits": deployment_min_term_bits,
                "target_bits": target_bits,
                "margin_bits": deployment_min_term_bits - target_bits,
            },
        ),
        checked(
            selected_transcript_term_bits == total_bits - 2 * q_quantum_log2 - selected_multi_target_log2,
            "selected deployment transcript term matches QROM plus multi-target ledger",
            {
                "selected_transcript_term_bits": selected_transcript_term_bits,
                "total_bits": total_bits,
                "qrom_quadratic_loss_bits": 2 * q_quantum_log2,
                "selected_multi_target_horizon_log2": selected_multi_target_log2,
            },
        ),
        checked(
            selected_transcript_term_bits >= target_bits + minimum_deployment_margin_bits,
            "selected deployment transcript term carries the recorded minimum margin",
            {
                "selected_transcript_term_bits": selected_transcript_term_bits,
                "target_bits": target_bits,
                "minimum_deployment_margin_bits": minimum_deployment_margin_bits,
            },
        ),
    ]
    frame_checks = [
        checked(
            labels == required_labels,
            "challenge frame labels match the reviewed order",
            {"labels": labels, "required_labels": required_labels},
        ),
        checked(
            not secret_fields,
            "challenge frame contains no secret-labeled fields",
            {"secret_fields": secret_fields},
        ),
        checked(
            frame["first_message_field"] == "proof_commitment"
            and int(frame["first_message_bytes"]) == 32,
            "proof commitment is the 32-byte prechallenge first-message field",
            {
                "first_message_field": frame["first_message_field"],
                "first_message_bytes": frame["first_message_bytes"],
            },
        ),
        checked(
            frame["verifier_recomputes_frame"] is True,
            "verifier is required to recompute the Fiat-Shamir frame",
            {"verifier_recomputes_frame": frame["verifier_recomputes_frame"]},
        ),
    ]
    all_checks = inequalities + frame_checks
    failed_checks = [item for item in all_checks if not item["holds"]]
    return {
        "schema": "Descender.qrom_security_loss_review.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source_memo": rel(QROM_MEMO),
        "review_scope": {
            "claim": (
                "For the reviewed challenge-frame oracle and the deployment query "
                "ledger, the concrete transcript/challenge terms keep at least "
                "128 bits of margin under the stated QROM/classical loss model."
            ),
            "conditional_on": [
                "Poseidon2 modeled as the transaction challenge quantum random oracle",
                "reviewed QROM fork/reprogramming theorem for the committed-sigma game",
                "no collapse of the separate pad PRF, matrix sampler, or ordinary public-object digest assumptions into this oracle",
            ],
            "not_claimed": [
                "proof of Poseidon2",
                "constant-time implementation security",
                "external acceptance of q <= 2^16 quantum challenge-frame queries",
            ],
        },
        "adversary_ledger": {
            "target_security_bits": target_bits,
            "deployment_quantum_hash_query_bound_log2": q_quantum_log2,
            "deployment_classical_hash_query_bound_log2": q_classical_log2,
            "selected_multi_target_horizon_log2": selected_multi_target_log2,
            "qrom_quadratic_loss_model": "term_bits = total_transcript_challenge_bits - 2*log2(Q_H)",
            "classical_loss_model": "term_bits = total_transcript_challenge_bits - log2(Q_H)",
            "algebraic_collision_model": "term_bits = algebraic_challenge_bits",
            "selected_deployment_model": "term_bits = total_transcript_challenge_bits - 2*log2(Q_H) - selected_multi_target_horizon_log2",
        },
        "derived_terms": {
            "total_transcript_challenge_bits": total_bits,
            "algebraic_challenge_bits": algebraic_bits,
            "hardening_bits": hardening_bits,
            "quantum_transcript_term_bits": quantum_transcript_term_bits,
            "classical_transcript_term_bits": classical_transcript_term_bits,
            "selected_transcript_term_bits": selected_transcript_term_bits,
            "deployment_min_term_bits": deployment_min_term_bits,
            "deployment_min_margin_bits": deployment_min_term_bits - target_bits,
            "selected_transcript_margin_bits": selected_transcript_term_bits - target_bits,
            "single_term_quantum_query_bound_log2_for_128": single_term_quantum_log2,
            "single_term_classical_query_bound_log2_for_128": single_term_classical_log2,
        },
        "checks": all_checks,
        "failed_checks": failed_checks,
        "status": "checks_passed" if not failed_checks else "checks_failed",
        "external_review_needed": [
            "Review whether the QROM loss model matches the final game hop.",
            "Review whether q <= 2^16 quantum challenge-sponge queries and the selected multi-target horizon are adequate for the target blockchain deployment.",
            "Review that production verification rejects before acceptance if challenge-frame recomputation mismatches.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    ledger = payload["adversary_ledger"]
    terms = payload["derived_terms"]
    lines = [
        "# QROM Security-Loss Review",
        "",
        "This generated review object turns the QROM challenge-budget memo into",
        "explicit arithmetic checks.  It remains conditional on the QROM model and",
        "Poseidon2 assumption; it does not prove either one.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- source memo: `{payload['source_memo']}`",
        f"- status: `{payload['status']}`",
        f"- target security bits: `{ledger['target_security_bits']}`",
        f"- deployment quantum query bound: `2^{ledger['deployment_quantum_hash_query_bound_log2']}`",
        f"- deployment classical query bound: `2^{ledger['deployment_classical_hash_query_bound_log2']}`",
        f"- selected multi-target horizon: `2^{ledger['selected_multi_target_horizon_log2']}`",
        f"- selected transcript term: `{terms['selected_transcript_term_bits']}` bits",
        f"- minimum deployment term: `{terms['deployment_min_term_bits']}` bits",
        f"- minimum deployment margin: `{terms['deployment_min_margin_bits']}` bits",
        "",
        "## Checks",
        "",
        "| check | holds | evidence |",
        "|---|:---:|---|",
    ]
    for item in payload["checks"]:
        evidence = ", ".join(f"{key}={value}" for key, value in item["evidence"].items())
        lines.append(
            f"| {item['description']} | {'yes' if item['holds'] else 'no'} | `{evidence}` |"
        )
    lines.extend(["", "## External Review Still Needed", ""])
    lines.extend(f"- {item}" for item in payload["external_review_needed"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))
    if payload["failed_checks"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
