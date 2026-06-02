#!/usr/bin/env python3
"""Probe wider challenge-action options for the active Path 2 QROM horizon."""

from __future__ import annotations

import json
import math
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "qrom_wide_challenge_action_probe_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "qrom_wide_challenge_action_probe_n32_q12289.md"

TARGET_BITS = 128
REPS = 26
CURRENT_BITS_PER_REP = 5
CURRENT_HARDENING_BYTES = 4
SLOT_ID_BYTES = 4
ACTIVE_RESPONSE_BYTES = 52
LOCAL_OPENING_BYTES_PER_REP = 4
PROOF_COMMITMENT_BYTES = 32
FQ12_MASKED_RESPONSE_BYTES = 39
QUANTUM_QUERY_LOG2 = 16
CLASSICAL_QUERY_LOG2 = 33


def ceil_div(lhs: int, rhs: int) -> int:
    return (lhs + rhs - 1) // rhs


def security_row(bits_per_rep: int, target_log2: int, hardening_bytes: int) -> dict[str, Any]:
    algebraic_bits = REPS * bits_per_rep
    total_bits = algebraic_bits + 8 * hardening_bytes
    quantum_bits = total_bits - 2 * QUANTUM_QUERY_LOG2 - target_log2
    classical_bits = total_bits - CLASSICAL_QUERY_LOG2 - target_log2
    algebraic_after_targets = algebraic_bits - target_log2
    minimum_bits = min(quantum_bits, classical_bits, algebraic_after_targets)
    return {
        "target_count_log2": target_log2,
        "total_transcript_bits": total_bits,
        "algebraic_bits": algebraic_bits,
        "quantum_bits": quantum_bits,
        "classical_bits": classical_bits,
        "algebraic_after_targets": algebraic_after_targets,
        "minimum_bits": minimum_bits,
        "margin_bits": minimum_bits - TARGET_BITS,
        "meets_128": minimum_bits >= TARGET_BITS,
        "bottleneck": [
            name
            for name, value in [
                ("qrom_quadratic_transcript", quantum_bits),
                ("classical_transcript", classical_bits),
                ("algebraic_action_collision", algebraic_after_targets),
            ]
            if value == minimum_bits
        ],
    }


def option(
    name: str,
    bits_per_rep: int,
    hardening_bytes: int,
    structural_model: str,
    proof_risk: str,
) -> dict[str, Any]:
    challenge_bytes = ceil_div(REPS * bits_per_rep, 8) + hardening_bytes
    local_opening_bytes = REPS * LOCAL_OPENING_BYTES_PER_REP
    proof_bytes = PROOF_COMMITMENT_BYTES + FQ12_MASKED_RESPONSE_BYTES
    transaction_bytes = (
        SLOT_ID_BYTES
        + challenge_bytes
        + ACTIVE_RESPONSE_BYTES
        + local_opening_bytes
        + proof_bytes
    )
    rows = [
        security_row(bits_per_rep, target_log2, hardening_bytes)
        for target_log2 in [1, 2, 16, 24, 32]
    ]
    max_covered = max(row["target_count_log2"] for row in rows if row["meets_128"])
    return {
        "name": name,
        "bits_per_rep": bits_per_rep,
        "hardening_bytes": hardening_bytes,
        "challenge_bytes": challenge_bytes,
        "transaction_bytes": transaction_bytes,
        "with_hidden_source_proof_bytes": transaction_bytes + 32,
        "delta_transaction_bytes_from_current": transaction_bytes - 252,
        "security_rows": rows,
        "max_covered_log2_in_probe_table": max_covered,
        "meets_log2_targets_32": next(
            row for row in rows if row["target_count_log2"] == 32
        )["meets_128"],
        "structural_model": structural_model,
        "proof_risk": proof_risk,
    }


def build_payload() -> dict[str, Any]:
    options = [
        option(
            "current-5bit-action",
            bits_per_rep=5,
            hardening_bytes=4,
            structural_model="16 block choices plus sign; existing active path",
            proof_risk="baseline; fails above one-bit target horizon",
        ),
        option(
            "six-bit-action-control",
            bits_per_rep=6,
            hardening_bytes=4,
            structural_model="hypothetical one extra action bit without changing reps",
            proof_risk=(
                "still fails log2(T)=32 because algebraic action bits are only 156; "
                "useful for mid-sized target ledgers, not full deployment scale"
            ),
        ),
        option(
            "seven-bit-effective-action-aux-fq12",
            bits_per_rep=7,
            hardening_bytes=4,
            structural_model=(
                "effective 7 action bits per repetition by appending auxiliary "
                "canonical Fq12 action bits after the existing 130 compact-local "
                "bits; preserves 26 compact block/sign fields, 52 active response "
                "bytes, 104 opening bytes, and 39 masked-response bytes"
            ),
            proof_risk=(
                "small byte cost under the auxiliary-Fq12 route; remaining proof "
                "work is challenge-byte canonicality, Fq12 action-collision/QROM "
                "budget, and byte-frame binding. Compact coefficient widening and "
                "larger block universes are not the selected route."
            ),
        ),
    ]
    return {
        "schema": "Descender.qrom_wide_challenge_action_probe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "parameters": {
            "target_bits": TARGET_BITS,
            "repetitions": REPS,
            "current_bits_per_rep": CURRENT_BITS_PER_REP,
            "current_hardening_bytes": CURRENT_HARDENING_BYTES,
            "quantum_query_log2": QUANTUM_QUERY_LOG2,
            "classical_query_log2": CLASSICAL_QUERY_LOG2,
            "current_transaction_bytes": 252,
            "current_with_hidden_source_proof_bytes": 284,
            "fixed_cost_assumption": (
                "wider action changes only challenge bytes; response/opening/proof byte "
                "counts are held fixed to isolate the best-case cost"
            ),
        },
        "options": options,
        "result": {
            "seven_bit_best_case_bytes": 258,
            "seven_bit_best_case_with_hidden_source_proof_bytes": 290,
            "seven_bit_best_case_delta_bytes": 6,
            "seven_bit_runtime_delta_if_reps_fixed": "no repetition-scaled runtime increase",
            "recommendation": (
                "Use auxiliary Fq12 action bits as the primary redesign route. It "
                "fixes log2(T)=32 in the budget model with about +6 bytes while "
                "leaving compact local block/sign semantics and response bounds unchanged."
            ),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    params = payload["parameters"]
    lines = [
        "# QROM Wide Challenge-Action Probe",
        "",
        "This is a best-case byte/security probe for widening the effective",
        "challenge action. The follow-up algebraic probe selects auxiliary Fq12",
        "action bits rather than compact coefficient widening.",
        "",
        "## Parameters",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- target bits: `{params['target_bits']}`",
        f"- repetitions: `{params['repetitions']}`",
        f"- current bits per repetition: `{params['current_bits_per_rep']}`",
        f"- quantum query log2: `{params['quantum_query_log2']}`",
        f"- classical query log2: `{params['classical_query_log2']}`",
        f"- current transaction bytes: `{params['current_transaction_bytes']}`",
        f"- current with hidden-source proof bytes: `{params['current_with_hidden_source_proof_bytes']}`",
        f"- fixed-cost assumption: {params['fixed_cost_assumption']}",
        "",
        "## Options",
        "",
        "| option | bits/rep | challenge B | tx B | tx+hidden B | delta B | log2(T)=32? | max covered in table | proof risk |",
        "|---|---:|---:|---:|---:|---:|:---:|---:|---|",
    ]
    for row in payload["options"]:
        lines.append(
            f"| {row['name']} | {row['bits_per_rep']} | {row['challenge_bytes']} | "
            f"{row['transaction_bytes']} | {row['with_hidden_source_proof_bytes']} | "
            f"{row['delta_transaction_bytes_from_current']} | "
            f"{'yes' if row['meets_log2_targets_32'] else 'no'} | "
            f"{row['max_covered_log2_in_probe_table']} | {row['proof_risk']} |"
        )
    lines.extend(["", "## Security Rows", ""])
    for row in payload["options"]:
        lines.extend(
            [
                f"### {row['name']}",
                "",
                "| log2(T) | quantum | classical | algebraic | min | margin | 128-bit? | bottleneck |",
                "|---:|---:|---:|---:|---:|---:|:---:|---|",
            ]
        )
        for sec in row["security_rows"]:
            lines.append(
                f"| {sec['target_count_log2']} | {sec['quantum_bits']} | "
                f"{sec['classical_bits']} | {sec['algebraic_after_targets']} | "
                f"{sec['minimum_bits']} | {sec['margin_bits']} | "
                f"{'yes' if sec['meets_128'] else 'no'} | "
                f"{','.join(sec['bottleneck'])} |"
            )
        lines.append("")
    result = payload["result"]
    lines.extend(
        [
            "## Result",
            "",
            f"- seven-bit best-case tx bytes: `{result['seven_bit_best_case_bytes']}`",
            f"- seven-bit best-case tx+hidden-source-proof bytes: `{result['seven_bit_best_case_with_hidden_source_proof_bytes']}`",
            f"- seven-bit best-case delta bytes: `{result['seven_bit_best_case_delta_bytes']}`",
            f"- runtime delta if reps fixed: {result['seven_bit_runtime_delta_if_reps_fixed']}",
            f"- recommendation: {result['recommendation']}",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload) + "\n")
    print(
        json.dumps(
            {
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "seven_bit_best_case_tx_bytes": payload["result"][
                    "seven_bit_best_case_bytes"
                ],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
