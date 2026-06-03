#!/usr/bin/env python3
"""Generate the QROM multi-target deployment-margin decision artifact.

PTC-003 found that the current 128-bit margin survives only through
log2(targets)=1.  This script makes that target-count term explicit and records
the fail-closed deployment cap plus parameter requirements for wider ledgers.
"""

from __future__ import annotations

import json
import math
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


def hardening_requirement(
    target_log2: int,
    *,
    bits_per_rep: int,
    target_bits: int,
    quantum_query_log2: int,
    classical_query_log2: int,
) -> dict[str, Any]:
    required_algebraic_bits = target_bits + target_log2
    required_repetitions = ceil_div(required_algebraic_bits, bits_per_rep)
    resulting_algebraic_bits = required_repetitions * bits_per_rep
    required_total_bits = max(
        target_bits + target_log2 + 2 * quantum_query_log2,
        target_bits + target_log2 + classical_query_log2,
    )
    required_hardening_bits = max(0, required_total_bits - resulting_algebraic_bits)
    required_hardening_bytes = ceil_div(required_hardening_bits, 8)
    return {
        "target_count_log2": target_log2,
        "required_algebraic_bits": required_algebraic_bits,
        "minimum_repetitions_at_current_bits_per_rep": required_repetitions,
        "resulting_algebraic_bits": resulting_algebraic_bits,
        "required_total_transcript_bits_at_current_query_bounds": required_total_bits,
        "minimum_hardening_bits": required_hardening_bits,
        "minimum_hardening_bytes": required_hardening_bytes,
        "resulting_total_bits_with_byte_hardening": resulting_algebraic_bits
        + 8 * required_hardening_bytes,
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
    bits_per_rep = int(derivation["bits_per_repetition"])

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
        for target_log2 in [0, 1, 2, 4, 8, 16, 20, 24, 32]
    ]
    first_failure = next(row for row in rows if not row["meets_128"])
    requirements = [
        hardening_requirement(
            target_log2,
            bits_per_rep=bits_per_rep,
            target_bits=target_bits,
            quantum_query_log2=quantum_query_log2,
            classical_query_log2=classical_query_log2,
        )
        for target_log2 in [2, 16, 20, 24, 32]
    ]
    current_target_2_query_tightening = {
        "target_count_log2": 2,
        "deployment_quantum_hash_query_bound_log2_may_remain": quantum_query_log2,
        "maximum_classical_hash_query_bound_log2": total_bits - target_bits - 2,
        "reason": (
            "At log2(targets)=2, the current quantum and algebraic terms land "
            "exactly at 128 bits, but the classical term needs one fewer query bit."
        ),
    }

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
            "summary": "Multi-target 128-bit margin only holds through log2(targets)=1.",
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
                "bits_per_repetition": bits_per_rep,
                "deployment_quantum_hash_query_bound_log2": quantum_query_log2,
                "deployment_classical_hash_query_bound_log2": classical_query_log2,
            },
        },
        "current_single_target_margins": current_single_margins,
        "multi_target_rows": rows,
        "fail_closed_limit": {
            "maximum_covered_target_count_log2": max_target_log2,
            "maximum_covered_target_count": 2**max_target_log2,
            "first_failing_target_count_log2_in_table": first_failure["target_count_log2"],
            "decision": (
                "Current n=32, q=12289 deployment statements must cap the aggregate "
                "public-key/message/account target ledger at log2(T) <= 1 unless a "
                "stricter query cap or wider challenge-action parameter set is adopted."
            ),
        },
        "mitigations": {
            "permit_log2_targets_2_without_widening": current_target_2_query_tightening,
            "widening_requirements_at_current_query_bounds": requirements,
            "hardening_note": (
                "Hardening bytes alone only increase transcript terms. For log2(T)>2, "
                "the algebraic action term also falls below 128 bits, so repetitions or "
                "challenge-action width must increase."
            ),
        },
        "status": "closed_by_fail_closed_deployment_cap",
        "deployment_ready_for_realistic_multitarget": False,
        "next_steps": [
            "Wire this fail-closed cap into any deployment profile that claims the current QROM ledger.",
            "For log2(T)=32, require at least 32 repetitions and 5 hardening bytes at the current 2^16/2^33 query bounds.",
            "If only log2(T)=2 is needed, tighten the classical challenge-frame query ledger from 2^33 to 2^32.",
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
            "- For `log2(targets)=2`, keep the quantum query cap but tighten the classical challenge-frame query cap to `2^32`.",
            "- For `log2(targets)>2`, increase repetitions or challenge-action width; hardening bytes alone do not repair the algebraic action term.",
            "",
            "| log2(targets) | min reps | resulting algebraic bits | required total bits | min hardening bytes | total bits with byte hardening |",
            "|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for req in payload["mitigations"]["widening_requirements_at_current_query_bounds"]:
        lines.append(
            f"| {req['target_count_log2']} | "
            f"{req['minimum_repetitions_at_current_bits_per_rep']} | "
            f"{req['resulting_algebraic_bits']} | "
            f"{req['required_total_transcript_bits_at_current_query_bounds']} | "
            f"{req['minimum_hardening_bytes']} | "
            f"{req['resulting_total_bits_with_byte_hardening']} |"
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
