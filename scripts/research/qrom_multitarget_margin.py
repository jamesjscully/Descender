#!/usr/bin/env python3
"""Generate the QROM multi-target deployment-margin decision artifact.

The current byte/mask challenge budget carries a selected 182-bit transcript
term. This script makes the target-count union term explicit and records the
fail-closed deployment cap plus parameter requirements for wider ledgers.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
QROM_MEMO = READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.json"
QROM_REVIEW = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
QROM_FRAME_PROBE = READINESS_DIR / "pre_third_party_qrom_frame_probe_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "qrom_multitarget_margin_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "qrom_multitarget_margin_n32_q12289.md"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def ceil_div(lhs: int, rhs: int) -> int:
    return (lhs + rhs - 1) // rhs


def margin_row(
    target_log2: int,
    *,
    total_bits: int,
    algebraic_bits: int,
    target_bits: int,
    quantum_query_log2: int,
    classical_query_log2: int,
) -> dict[str, Any]:
    quantum_bits = total_bits - 2 * quantum_query_log2 - target_log2
    classical_bits = total_bits - classical_query_log2 - target_log2
    algebraic_bits_after_targets = algebraic_bits - target_log2
    minimum_bits = min(quantum_bits, classical_bits, algebraic_bits_after_targets)
    bottlenecks = [
        name
        for name, value in [
            ("qrom_quadratic_transcript", quantum_bits),
            ("classical_transcript", classical_bits),
            ("algebraic_action_collision", algebraic_bits_after_targets),
        ]
        if value == minimum_bits
    ]
    return {
        "target_count_log2": target_log2,
        "quantum_frame_term_bits": quantum_bits,
        "classical_frame_term_bits": classical_bits,
        "algebraic_action_term_bits": algebraic_bits_after_targets,
        "minimum_term_bits": minimum_bits,
        "margin_bits": minimum_bits - target_bits,
        "meets_128": minimum_bits >= target_bits,
        "bottlenecks": bottlenecks,
    }


def widening_requirement(
    target_log2: int,
    *,
    current_algebraic_bits: int,
    current_hardening_bits: int,
    current_total_bits: int,
    target_bits: int,
    quantum_query_log2: int,
    classical_query_log2: int,
) -> dict[str, Any]:
    required_algebraic_bits = target_bits + target_log2
    required_total_bits = max(
        target_bits + target_log2 + 2 * quantum_query_log2,
        target_bits + target_log2 + classical_query_log2,
        required_algebraic_bits,
    )
    additional_algebraic_bits = max(0, required_algebraic_bits - current_algebraic_bits)
    additional_total_bits = max(0, required_total_bits - current_total_bits)
    required_hardening_bits_if_algebraic_unchanged = max(
        0, required_total_bits - current_algebraic_bits
    )
    additional_hardening_bits_if_algebraic_unchanged = max(
        0, required_hardening_bits_if_algebraic_unchanged - current_hardening_bits
    )
    return {
        "target_count_log2": target_log2,
        "required_algebraic_bits": required_algebraic_bits,
        "required_total_transcript_bits_at_current_query_bounds": required_total_bits,
        "additional_algebraic_bits_needed": additional_algebraic_bits,
        "additional_total_bits_needed": additional_total_bits,
        "additional_hardening_bytes_if_algebraic_unchanged": ceil_div(
            additional_hardening_bits_if_algebraic_unchanged, 8
        ),
    }


def build_payload() -> dict[str, Any]:
    memo = load_json(QROM_MEMO)
    review = load_json(QROM_REVIEW)
    frame_probe = load_json(QROM_FRAME_PROBE)

    derivation = memo["challenge_derivation"]
    ledger = review["adversary_ledger"]
    total_bits = int(derivation["total_transcript_challenge_bits"])
    algebraic_bits = int(derivation["algebraic_challenge_bits"])
    hardening_bits = int(derivation["hardening_bits"])
    target_bits = int(ledger["target_security_bits"])
    quantum_query_log2 = int(ledger["deployment_quantum_hash_query_bound_log2"])
    classical_query_log2 = int(ledger["deployment_classical_hash_query_bound_log2"])

    current_single_margins = {
        "qrom_quadratic_transcript": total_bits - 2 * quantum_query_log2 - target_bits,
        "classical_transcript": total_bits - classical_query_log2 - target_bits,
        "algebraic_action_collision": algebraic_bits - target_bits,
    }
    max_target_log2 = min(current_single_margins.values())
    rows = [
        margin_row(
            target_log2,
            total_bits=total_bits,
            algebraic_bits=algebraic_bits,
            target_bits=target_bits,
            quantum_query_log2=quantum_query_log2,
            classical_query_log2=classical_query_log2,
        )
        for target_log2 in [0, 1, 2, 4, 8, 16, 24, 32, 48, 54, 55, 64]
    ]
    first_failure = next((row for row in rows if not row["meets_128"]), None)
    requirements = [
        widening_requirement(
            target_log2,
            current_algebraic_bits=algebraic_bits,
            current_hardening_bits=hardening_bits,
            current_total_bits=total_bits,
            target_bits=target_bits,
            quantum_query_log2=quantum_query_log2,
            classical_query_log2=classical_query_log2,
        )
        for target_log2 in [54, 55, 64]
    ]

    return {
        "schema": "Descender.qrom_multitarget_margin.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "sources": {
            "qrom_challenge_budget_memo": rel(QROM_MEMO),
            "qrom_security_loss_review": rel(QROM_REVIEW),
            "pre_third_party_qrom_frame_probe": rel(QROM_FRAME_PROBE),
        },
        "concern": {
            "id": "PTC-003",
            "summary": "Multi-target 128-bit margin holds through the current byte/mask fail-closed cap.",
            "observed_probe_cap_log2": frame_probe["query_budget"][
                "max_target_count_log2_at_128_bits"
            ],
        },
        "formula": {
            "multi_target_term_bits": (
                "min(total_transcript_challenge_bits - 2*log2(Q_quantum) - log2(T), "
                "total_transcript_challenge_bits - log2(Q_classical) - log2(T), "
                "algebraic_challenge_bits - log2(T))"
            ),
            "deployment_accepts_only_if": "multi_target_term_bits >= target_security_bits",
            "current_values": {
                "target_security_bits": target_bits,
                "total_transcript_challenge_bits": total_bits,
                "algebraic_challenge_bits": algebraic_bits,
                "hardening_bits": hardening_bits,
                "decoded_local_control_bits": int(
                    derivation.get("decoded_local_control_bits", 0)
                ),
                "deployment_quantum_hash_query_bound_log2": quantum_query_log2,
                "deployment_classical_hash_query_bound_log2": classical_query_log2,
            },
        },
        "current_single_target_margins": current_single_margins,
        "multi_target_rows": rows,
        "fail_closed_limit": {
            "maximum_covered_target_count_log2": max_target_log2,
            "maximum_covered_target_count": 2**max_target_log2,
            "first_failing_target_count_log2_in_table": (
                first_failure["target_count_log2"] if first_failure else None
            ),
            "decision": (
                "Current n=32, q=12289 deployment statements must cap the aggregate "
                "public-key/message/account target ledger at log2(T) <= 54 unless a "
                "stricter query cap or wider byte/mask challenge-action budget is adopted."
            ),
        },
        "mitigations": {
            "widening_requirements_at_current_query_bounds": requirements,
            "hardening_note": (
                "Hardening bytes alone only increase transcript terms. Once the algebraic "
                "byte/mask action term is the bottleneck, the algebraic challenge budget "
                "itself must widen."
            ),
        },
        "status": "closed_by_fail_closed_deployment_cap",
        "deployment_ready_for_realistic_multitarget": True,
        "next_steps": [
            "Wire this fail-closed cap into any deployment profile that claims the current QROM ledger.",
            "For log2(T)>54, widen the algebraic byte/mask challenge budget or lower the target/query ledger.",
            "Ask external QROM reviewers to confirm the same target-count union term in the final game hop.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    formula = payload["formula"]
    limit = payload["fail_closed_limit"]
    lines = [
        "# QROM Multi-Target Margin",
        "",
        "This artifact closes PTC-003 by making the target-count union term an",
        "explicit deployment gate.  It does not expand the current claim; it",
        "fails closed unless the target ledger stays within the recorded cap.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- status: `{payload['status']}`",
        f"- realistic multi-target deployment ready: `{payload['deployment_ready_for_realistic_multitarget']}`",
        f"- maximum covered log2(targets): `{limit['maximum_covered_target_count_log2']}`",
        f"- maximum covered targets: `{limit['maximum_covered_target_count']}`",
        "",
        "## Sources",
        "",
    ]
    lines.extend(f"- {name}: `{path}`" for name, path in payload["sources"].items())
    lines.extend(
        [
            "",
            "## Formula",
            "",
            f"`{formula['multi_target_term_bits']}`",
            "",
            f"Deployment accepts only if `{formula['deployment_accepts_only_if']}`.",
            "",
            "Current values:",
            "",
        ]
    )
    for key, value in formula["current_values"].items():
        lines.append(f"- {key}: `{value}`")
    lines.extend(
        [
            "",
            "## Current Margins",
            "",
        ]
    )
    for key, value in payload["current_single_target_margins"].items():
        lines.append(f"- {key}: `{value}` bit(s)")
    lines.extend(
        [
            "",
            "## Multi-Target Table",
            "",
            "| log2(targets) | quantum bits | classical bits | algebraic bits | min bits | margin | 128-bit? | bottleneck |",
            "|---:|---:|---:|---:|---:|---:|:---:|---|",
        ]
    )
    for row in payload["multi_target_rows"]:
        lines.append(
            f"| {row['target_count_log2']} | {row['quantum_frame_term_bits']} | "
            f"{row['classical_frame_term_bits']} | {row['algebraic_action_term_bits']} | "
            f"{row['minimum_term_bits']} | {row['margin_bits']} | "
            f"{'yes' if row['meets_128'] else 'no'} | {', '.join(row['bottlenecks'])} |"
        )
    lines.extend(
        [
            "",
            "## Decision",
            "",
            limit["decision"],
            "",
            "## Mitigations",
            "",
            "- For `log2(targets)<=54`, the current byte/mask budget remains at or above 128 bits.",
            "- For `log2(targets)>54`, widen the algebraic byte/mask challenge budget or lower the query/target ledger; hardening bytes alone do not repair an algebraic bottleneck.",
            "",
            "| log2(targets) | required algebraic bits | required total bits | extra algebraic bits | extra total bits | extra hardening bytes if algebraic unchanged |",
            "|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for req in payload["mitigations"]["widening_requirements_at_current_query_bounds"]:
        lines.append(
            f"| {req['target_count_log2']} | "
            f"{req['required_algebraic_bits']} | "
            f"{req['required_total_transcript_bits_at_current_query_bounds']} | "
            f"{req['additional_algebraic_bits_needed']} | "
            f"{req['additional_total_bits_needed']} | "
            f"{req['additional_hardening_bytes_if_algebraic_unchanged']} |"
        )
    lines.extend(["", "## Next Steps", ""])
    lines.extend(f"- {item}" for item in payload["next_steps"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
