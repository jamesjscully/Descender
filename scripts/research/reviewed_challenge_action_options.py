#!/usr/bin/env python3
"""Analyze challenge-action options for the account-carried quotient target."""

from __future__ import annotations

import json
import math
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "reviewed_challenge_action_options_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "reviewed_challenge_action_options_n32_q12289.md"

Q = 12_289
TARGET_BITS = 128
DEPLOYMENT_QUANTUM_QUERY_LOG2 = 16
DEPLOYMENT_CLASSICAL_QUERY_LOG2 = 33
BASE_SPEND_BYTES = 181
PROOF_COMMITMENT_BYTES = 32
LOCAL_FIBER_STATE_BITS = 16
FIELD_BITS = 14


def floor_log2_q_power(residues: int) -> int:
    return math.floor(residues * math.log2(Q))


def ceil_div(lhs: int, rhs: int) -> int:
    return (lhs + rhs - 1) // rhs


def option(
    name: str,
    response_residues: int,
    public_rows: int,
    action_residues: int,
    notes: str,
) -> dict[str, Any]:
    response_bits = response_residues * FIELD_BITS
    response_bytes = ceil_div(response_bits, 8)
    response_slack_bits = response_bytes * 8 - response_bits
    proof_bytes = PROOF_COMMITMENT_BYTES + response_bytes
    tx_bytes = BASE_SPEND_BYTES + proof_bytes
    action_floor_bits = floor_log2_q_power(action_residues)
    action_quantum_bits = action_floor_bits - 2 * DEPLOYMENT_QUANTUM_QUERY_LOG2
    action_classical_bits = action_floor_bits - DEPLOYMENT_CLASSICAL_QUERY_LOG2
    action_single_term_quantum_query_log2 = max(
        0, (action_floor_bits - TARGET_BITS) // 2
    )
    public_quotient_bits = public_rows * FIELD_BITS
    hidden_raw_bits = (response_residues - public_rows) * FIELD_BITS + response_slack_bits
    hidden_after_local_state_bits = hidden_raw_bits - LOCAL_FIBER_STATE_BITS
    passes_action_quantum = action_quantum_bits >= TARGET_BITS
    passes_action_classical = action_classical_bits >= TARGET_BITS
    passes_public = public_quotient_bits >= TARGET_BITS
    passes_hiding = hidden_after_local_state_bits >= TARGET_BITS
    return {
        "name": name,
        "transaction_bytes": tx_bytes,
        "proof_bytes": proof_bytes,
        "response_bytes": response_bytes,
        "response_residues": response_residues,
        "response_slack_bits": response_slack_bits,
        "public_rows": public_rows,
        "action_residues": action_residues,
        "public_quotient_bits": public_quotient_bits,
        "hidden_raw_bits": hidden_raw_bits,
        "hidden_after_local_state_bits": hidden_after_local_state_bits,
        "action_collision_floor_bits": action_floor_bits,
        "action_collision_bits_after_quantum_queries": action_quantum_bits,
        "action_collision_bits_after_classical_queries": action_classical_bits,
        "action_single_term_quantum_query_log2_for_128": action_single_term_quantum_query_log2,
        "passes": {
            "public_quotient_bits": passes_public,
            "hiding_after_local_state": passes_hiding,
            "action_collision_quantum_2pow16": passes_action_quantum,
            "action_collision_classical_2pow33": passes_action_classical,
            "all_core": passes_public
            and passes_hiding
            and passes_action_quantum
            and passes_action_classical,
        },
        "notes": notes,
    }


def build_payload() -> dict[str, Any]:
    options = [
        option(
            "exact-250b-10row-fq10",
            response_residues=21,
            public_rows=10,
            action_residues=10,
            notes=(
                "Keeps the current 250-byte target and hiding margin, but the "
                "10-residue action collision space is too small for the 2^16 "
                "quantum-query ledger."
            ),
        ),
        option(
            "intermediate-250b-11row-fq11",
            response_residues=21,
            public_rows=11,
            action_residues=11,
            notes=(
                "Still 250 bytes, but loses both action-collision quantum "
                "margin and the 128-bit post-local-state hiding target."
            ),
        ),
        option(
            "250b-12row-fq12-control",
            response_residues=21,
            public_rows=12,
            action_residues=12,
            notes=(
                "Reuses the robust Fq12 action budget, but the 37-byte response "
                "does not leave enough hidden-fiber capacity after the local "
                "state constraint."
            ),
        ),
        option(
            "252b-12row-fq12",
            response_residues=22,
            public_rows=12,
            action_residues=12,
            notes=(
                "Adds two signature bytes.  This keeps the reviewed account "
                "architecture, reuses the Fq12 action budget, and exactly meets "
                "the 128-bit hidden-fiber target after the local-state constraint."
            ),
        ),
        option(
            "254b-12row-fq12-wide",
            response_residues=23,
            public_rows=12,
            action_residues=12,
            notes=(
                "Four-byte wider control with comfortable hiding margin; close "
                "to the legacy 254-byte size but can be rebuilt in the reviewed "
                "account-carried proof structure."
            ),
        ),
    ]
    recommended = next(row for row in options if row["name"] == "252b-12row-fq12")
    return {
        "schema": "Descender.reviewed_challenge_action_options.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "parameters": {
            "q": Q,
            "target_bits": TARGET_BITS,
            "deployment_quantum_query_log2": DEPLOYMENT_QUANTUM_QUERY_LOG2,
            "deployment_classical_query_log2": DEPLOYMENT_CLASSICAL_QUERY_LOG2,
            "base_spend_bytes": BASE_SPEND_BYTES,
            "proof_commitment_bytes": PROOF_COMMITMENT_BYTES,
            "local_fiber_state_bits": LOCAL_FIBER_STATE_BITS,
            "field_bits": FIELD_BITS,
        },
        "options": options,
        "recommendation": {
            "selected": recommended["name"],
            "transaction_bytes": recommended["transaction_bytes"],
            "reason": (
                "The exact 250-byte/10-row row preserves hiding but fails the "
                "multi-query action-collision budget.  The 252-byte/12-row row "
                "is the smallest listed option that keeps public quotient, "
                "post-local-state hiding, and Fq12 action-collision margins at "
                "or above 128 bits under the current ledger."
            ),
            "next_steps": [
                "Promote the 252-byte/12-row row to the reviewed fallback target.",
                "Pin a 12x22 matrix domain and regenerate matrix/rank/kernel certificates.",
                "Update byte model, Lean target constants, and Rust codec constants from 21 to 22 response residues.",
                "Implement the final equation using the already reviewed Fq12 action algebra after the 12x22 quotient map is pinned.",
            ],
        },
        "decision": {
            "exact_250b_still_possible": True,
            "exact_250b_requires_new_argument": (
                "A non-field or adapter challenge action with a larger bad-event "
                "denominator than q^10, while still acting soundly on a 10-row "
                "quotient without public-projection leakage."
            ),
            "deployment_ready": False,
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    params = payload["parameters"]
    rec = payload["recommendation"]
    lines = [
        "# Reviewed Challenge-Action Options",
        "",
        "This artifact evaluates the concrete byte/security tradeoff exposed by",
        "the reviewed equation-spec gap.  It is a design checkpoint, not a final",
        "security proof.",
        "",
        "## Parameters",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- q: `{params['q']}`",
        f"- target bits: `{params['target_bits']}`",
        f"- deployment quantum query log2: `{params['deployment_quantum_query_log2']}`",
        f"- deployment classical query log2: `{params['deployment_classical_query_log2']}`",
        f"- base spend bytes: `{params['base_spend_bytes']}`",
        f"- proof commitment bytes: `{params['proof_commitment_bytes']}`",
        f"- local fiber-state bits charged: `{params['local_fiber_state_bits']}`",
        "",
        "## Options",
        "",
        "| option | tx B | response B | rows | action residues | public bits | hidden bits after local state | action bits after q=2^16 quantum | all core? |",
        "|---|---:|---:|---:|---:|---:|---:|---:|:---:|",
    ]
    for row in payload["options"]:
        lines.append(
            f"| {row['name']} | {row['transaction_bytes']} | {row['response_bytes']} | "
            f"{row['public_rows']} | {row['action_residues']} | "
            f"{row['public_quotient_bits']} | {row['hidden_after_local_state_bits']} | "
            f"{row['action_collision_bits_after_quantum_queries']} | "
            f"{'yes' if row['passes']['all_core'] else 'no'} |"
        )
    lines.extend(
        [
            "",
            "## Recommendation",
            "",
            f"- selected: `{rec['selected']}`",
            f"- transaction bytes: `{rec['transaction_bytes']}`",
            f"- reason: {rec['reason']}",
            "",
            "Next steps:",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in rec["next_steps"])
    lines.extend(
        [
            "",
            "## Exact 250-Byte Status",
            "",
            f"- still possible: `{payload['decision']['exact_250b_still_possible']}`",
            f"- requires: {payload['decision']['exact_250b_requires_new_argument']}",
            f"- deployment ready: `{payload['decision']['deployment_ready']}`",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "recommended": payload["recommendation"]["selected"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
