#!/usr/bin/env python3
"""Generate a toy lattice-sigma transcript model for Path 2 private-ZK."""

from __future__ import annotations

import json
from math import ceil
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_descent_sis_lattice_sigma_toy_model.json"
OUT_MD = ROOT / "research/path2_descent_sis_lattice_sigma_toy_model.md"

Q_BITS = 14
ACTIVE_LANES = 6


def packed_q_bytes(lanes: int) -> int:
    return ceil(lanes * Q_BITS / 8)


ROLE_DIGIT_MODEL = [
    {
        "role": "pad_source",
        "semantic_residues": 6,
        "base128_digits": 12,
        "range_witnesses": 12,
    },
    {
        "role": "projection_relation",
        "semantic_residues": 6,
        "base128_digits": 12,
        "range_witnesses": 12,
    },
    {
        "role": "opening_tuple",
        "semantic_residues": 8,
        "base128_digits": 16,
        "range_witnesses": 16,
    },
]


def build_rows() -> list[dict[str, Any]]:
    rows = []
    private_witness_lanes = sum(
        item["base128_digits"] + item["range_witnesses"] for item in ROLE_DIGIT_MODEL
    )
    relation_q_rows = (
        ACTIVE_LANES  # masked projection equations
        + sum(item["semantic_residues"] for item in ROLE_DIGIT_MODEL)  # digit reconstruction
        + sum(item["base128_digits"] for item in ROLE_DIGIT_MODEL)  # digit range checks
    )
    response_bytes = packed_q_bytes(private_witness_lanes)
    first_message_bytes = packed_q_bytes(relation_q_rows)
    for repetitions in (16, 32, 64, 96, 128):
        proof_bytes_floor = 32 + repetitions * (first_message_bytes + response_bytes)
        rows.append(
            {
                "repetitions": repetitions,
                "private_witness_lanes": private_witness_lanes,
                "relation_q_rows": relation_q_rows,
                "first_message_bytes_per_round_floor": first_message_bytes,
                "response_bytes_per_round_floor": response_bytes,
                "proof_bytes_floor": proof_bytes_floor,
                "verifier_q_ops_floor": repetitions
                * (private_witness_lanes + relation_q_rows),
                "status": (
                    "too_weak_without_large_challenge"
                    if repetitions < 64
                    else "byte_heavy_candidate_floor"
                ),
            }
        )
    return rows


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_descent_sis_lattice_sigma_toy_model.v1",
        "status": "toy_transcript_model_pinned_not_deployable",
        "proof_family": "lattice_sigma_rejection_zk",
        "relation_shape": {
            "private_witness": ROLE_DIGIT_MODEL,
            "public_relation": "masked_projection_i = pad_i + account_projection_i mod q plus base-128 digit reconstruction/range checks",
            "private_linear_messages": (
                "proof-internal commitments/masks are not serialized through the "
                "Path 2 public payload serializer except as proof-system public messages"
            ),
        },
        "sigma_skeleton": [
            {
                "step": "commit",
                "public_message": "t = LinearRelation(mask) or its proof-system encoding",
                "private_state": "mask and rejection-sampling coins",
            },
            {
                "step": "challenge",
                "public_message": "c = FS(private_zk_qrom_frame || public_commitment_digest || round_index)",
                "private_state": "none",
            },
            {
                "step": "response",
                "public_message": "z = mask + c * witness, accepted only if rejection/norm test passes",
                "private_state": "witness remains hidden under simulator/rejection assumption",
            },
            {
                "step": "verify",
                "public_message": "check public linear equations and range/digit proof equations from (t,c,z)",
                "private_state": "none",
            },
        ],
        "simulator_surface": {
            "claim": (
                "A simulator samples challenges and responses first, derives public "
                "messages consistent with the verifier equations, and produces a "
                "transcript indistinguishable from honest transcripts under the "
                "selected rejection-sampling distribution."
            ),
            "named_break": "samplerHidingBreak",
            "blocked_on": [
                "exact mask distribution",
                "exact rejection threshold",
                "range/digit proof encoding",
                "proof-family-specific statistical distance bound",
            ],
        },
        "extractor_surface": {
            "claim": (
                "Two accepting transcripts with the same first message and distinct "
                "challenges extract a witness satisfying the masked-projection and "
                "digit/range equations, or export proof-system soundness/QROM break."
            ),
            "named_breaks": [
                "creation_time_hidden_source_soundness_break",
                "path2_sis_binding_break",
                "path2_sis_norm_or_range_break",
                "qrom_or_domain_separation_break",
            ],
            "blocked_on": [
                "challenge space and rewind strategy",
                "response norm bounds",
                "base-128 digit range constraints",
                "mapping extracted digits back to q12289 semantic lanes",
            ],
        },
        "byte_floor_rows": build_rows(),
        "go_no_go_read": (
            "Even this optimistic floor is kilobyte-scale for plausible repetition "
            "counts. That may still be acceptable at account creation, but it means "
            "the private lattice-sigma branch should be compared immediately with "
            "the transparent hash proof fallback rather than optimized in isolation."
        ),
        "non_claims": [
            "this is not a proof-size estimate for a concrete lattice ZK protocol",
            "this does not choose repetition count or challenge distribution",
            "this does not prove simulator indistinguishability",
            "this does not prove extractor soundness",
            "this does not implement the verifier",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Descent/SIS Lattice-Sigma Toy Model",
        "",
        "Status: toy transcript model pinned; not deployable.",
        "",
        "## Relation Shape",
        "",
        f"- proof family: `{model['proof_family']}`",
        f"- public relation: {model['relation_shape']['public_relation']}",
        f"- private linear messages: {model['relation_shape']['private_linear_messages']}",
        "",
        "Private witness lanes:",
        "",
        "| role | residues | base-128 digits | range witnesses |",
        "|---|---:|---:|---:|",
    ]
    for item in model["relation_shape"]["private_witness"]:
        lines.append(
            "| {role} | {semantic_residues} | {base128_digits} | "
            "{range_witnesses} |".format(**item)
        )
    lines.extend(["", "## Sigma Skeleton", ""])
    for item in model["sigma_skeleton"]:
        lines.append(
            f"- `{item['step']}`: public `{item['public_message']}`; private `{item['private_state']}`"
        )
    lines.extend(
        [
            "",
            "## Simulator Surface",
            "",
            model["simulator_surface"]["claim"],
            "",
            f"Named break: `{model['simulator_surface']['named_break']}`.",
            "",
            "Blocked on:",
            "",
        ]
    )
    for item in model["simulator_surface"]["blocked_on"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Extractor Surface",
            "",
            model["extractor_surface"]["claim"],
            "",
            "Named breaks:",
            "",
        ]
    )
    for item in model["extractor_surface"]["named_breaks"]:
        lines.append(f"- `{item}`")
    lines.extend(["", "Blocked on:", ""])
    for item in model["extractor_surface"]["blocked_on"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Byte Floor",
            "",
            "| repetitions | witness lanes | relation rows | first msg/round | response/round | proof floor | verifier q-ops floor | status |",
            "|---:|---:|---:|---:|---:|---:|---:|---|",
        ]
    )
    for item in model["byte_floor_rows"]:
        lines.append(
            "| {repetitions} | {private_witness_lanes} | {relation_q_rows} | "
            "{first_message_bytes_per_round_floor} | {response_bytes_per_round_floor} | "
            "{proof_bytes_floor} | {verifier_q_ops_floor} | {status} |".format(**item)
        )
    lines.extend(
        [
            "",
            "## Go/No-Go Read",
            "",
            model["go_no_go_read"],
            "",
            "## Non-Claims",
            "",
        ]
    )
    for item in model["non_claims"]:
        lines.append(f"- {item}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    model = build_model()
    OUT_JSON.write_text(json.dumps(model, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(write_markdown(model))
    print(json.dumps({"json": str(OUT_JSON), "markdown": str(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
