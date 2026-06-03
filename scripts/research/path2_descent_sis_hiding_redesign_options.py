#!/usr/bin/env python3
"""Generate hiding redesign options for Path 2 descent/SIS commitments."""

from __future__ import annotations

import hashlib
import json
from math import ceil
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_descent_sis_hiding_redesign_options.json"
OUT_MD = ROOT / "research/path2_descent_sis_hiding_redesign_options.md"

Q = 12_289
Q_BITS = 14
REJECTION_LIMIT = 65_535 - (65_535 % Q)
ROW_BANDS = [48, 64]
ROLES = [
    {
        "role": "pad_source",
        "role_id": 1,
        "domain": "DESCENDER_PATH2_DESCENT_SIS_PAD_SOURCE_MATRIX_V1",
        "semantic_residues": 6,
    },
    {
        "role": "projection_relation",
        "role_id": 2,
        "domain": "DESCENDER_PATH2_DESCENT_SIS_PROJECTION_RELATION_MATRIX_V1",
        "semantic_residues": 6,
    },
    {
        "role": "opening_tuple",
        "role_id": 3,
        "domain": "DESCENDER_PATH2_DESCENT_SIS_OPENING_TUPLE_MATRIX_V1",
        "semantic_residues": 8,
    },
]


def packed_q_bytes(lanes: int) -> int:
    return ceil(lanes * Q_BITS / 8)


def matrix_entry(domain: str, role_id: int, rows: int, row: int, col: int) -> int:
    counter = 0
    while True:
        frame = (
            domain.encode("ascii")
            + bytes([role_id])
            + rows.to_bytes(2, "little")
            + row.to_bytes(2, "little")
            + col.to_bytes(2, "little")
            + counter.to_bytes(4, "little")
        )
        digest = hashlib.sha256(frame).digest()
        for offset in range(0, len(digest), 2):
            candidate = int.from_bytes(digest[offset : offset + 2], "little")
            if candidate < REJECTION_LIMIT:
                return candidate % Q
        counter += 1


def rank_mod_q(matrix: list[list[int]]) -> int:
    mat = [row[:] for row in matrix]
    row_count = len(mat)
    col_count = len(mat[0]) if mat else 0
    rank = 0
    for col in range(col_count):
        pivot = None
        for candidate in range(rank, row_count):
            if mat[candidate][col] % Q != 0:
                pivot = candidate
                break
        if pivot is None:
            continue
        mat[rank], mat[pivot] = mat[pivot], mat[rank]
        inv = pow(mat[rank][col] % Q, -1, Q)
        mat[rank] = [(value * inv) % Q for value in mat[rank]]
        for target in range(row_count):
            if target == rank:
                continue
            factor = mat[target][col] % Q
            if factor:
                mat[target] = [
                    (left - factor * right) % Q
                    for left, right in zip(mat[target], mat[rank])
                ]
        rank += 1
        if rank == row_count:
            break
    return rank


def submatrix_rank(
    domain: str, role_id: int, rows: int, start_col: int, column_count: int
) -> int:
    matrix = [
        [
            matrix_entry(domain, role_id, rows, row, start_col + col)
            for col in range(column_count)
        ]
        for row in range(rows)
    ]
    return rank_mod_q(matrix)


def build_candidate_rows() -> list[dict[str, Any]]:
    candidates = []
    for rows in ROW_BANDS:
        role_rows = []
        total_base128_columns = 0
        total_raw_columns = 0
        for role in ROLES:
            raw_semantic = role["semantic_residues"]
            base128_semantic = role["semantic_residues"] * 2
            base128_range_witness = role["semantic_residues"] * 2
            blinding_columns = rows
            rank = submatrix_rank(
                role["domain"],
                role["role_id"],
                rows,
                base128_semantic + base128_range_witness,
                blinding_columns,
            )
            base128_total = base128_semantic + base128_range_witness + blinding_columns
            raw_total = raw_semantic + blinding_columns
            total_base128_columns += base128_total
            total_raw_columns += raw_total
            role_rows.append(
                {
                    "role": role["role"],
                    "rows": rows,
                    "raw_semantic_columns": raw_semantic,
                    "base128_semantic_columns": base128_semantic,
                    "base128_range_witness_columns": base128_range_witness,
                    "blinding_columns": blinding_columns,
                    "base128_total_columns": base128_total,
                    "raw_total_columns": raw_total,
                    "blinding_submatrix_rank": rank,
                    "blinding_submatrix_full_row_rank": rank == rows,
                }
            )
        commitments = 3 * packed_q_bytes(rows)
        candidates.append(
            {
                "rows": rows,
                "role_rows": role_rows,
                "public_commitment_bytes": commitments,
                "base128_total_opening_columns": total_base128_columns,
                "base128_response_bytes_lower_bound": packed_q_bytes(
                    total_base128_columns
                ),
                "base128_payload_lower_bound": (
                    commitments
                    + commitments
                    + packed_q_bytes(total_base128_columns)
                    + 32
                ),
                "raw_total_opening_columns": total_raw_columns,
                "raw_response_bytes_lower_bound": packed_q_bytes(total_raw_columns),
                "raw_payload_lower_bound": (
                    commitments + commitments + packed_q_bytes(total_raw_columns) + 32
                ),
            }
        )
    return candidates


def build_model() -> dict[str, Any]:
    candidate_rows = build_candidate_rows()
    return {
        "schema": "Descender.path2_descent_sis_hiding_redesign_options.v1",
        "status": "redesign_options_pinned_not_selected",
        "active_blocker": (
            "The initial public linear commitments are full-column-rank for the "
            "main roles and reveal openings. A viable Path 2 descent/SIS payload "
            "must change the public commitment/proof transcript shape."
        ),
        "options": [
            {
                "id": "statistically_hiding_public_kernel",
                "status": "promising_for_hiding_but_changes_binding_assumption",
                "public_object": "publish C = A_m*m + A_r*r mod q with A_r full row rank",
                "hiding_mode": (
                    "perfect/statistical over F_q if r is sampled broadly enough "
                    "from a distribution whose image under A_r covers F_q^rows"
                ),
                "binding_mode": (
                    "not a clean short-SIS binding claim if r is full-field; "
                    "double openings export large randomness deltas unless the "
                    "opening acceptance bounds are changed"
                ),
                "proof_consequence": (
                    "needs an explicit dual-mode theorem: hiding from broad "
                    "randomness, binding from a named bounded-kernel assumption "
                    "or a separate short binding commitment"
                ),
            },
            {
                "id": "small_noise_computational_hiding",
                "status": "coherent_but_new_assumption",
                "public_object": "publish C = A_m*m + A_r*r mod q with bounded r",
                "hiding_mode": (
                    "computational hiding, closer to LWE/module-LWE or leftover "
                    "hash style assumptions depending on the exact sampler"
                ),
                "binding_mode": "short openings remain meaningful for SIS-style binding",
                "proof_consequence": (
                    "probably the cleanest one-object theorem, but it adds a "
                    "named hiding assumption instead of pure SIS binding"
                ),
            },
            {
                "id": "private_linear_commitments_inside_zk",
                "status": "promising_if_payload_proof_system_supports_it",
                "public_object": (
                    "do not publish the non-hiding role commitments; publish only "
                    "a zero-knowledge proof commitment/transcript that proves the "
                    "linear relation and binds to the leaf"
                ),
                "hiding_mode": "delegated to the ZK proof's simulator",
                "binding_mode": (
                    "binding break is exported through proof knowledge soundness "
                    "plus the private witness commitment relation"
                ),
                "proof_consequence": (
                    "matches the theorem shape well, but requires a real ZK "
                    "payload proof rather than treating C=A*v as the public proof object"
                ),
            },
            {
                "id": "dual_commitment_hiding_plus_binding",
                "status": "fallback_due_to_bytes",
                "public_object": (
                    "use one statistically hiding commitment for privacy and a "
                    "separate short-binding commitment/proof for extraction"
                ),
                "hiding_mode": "straightforward",
                "binding_mode": "straightforward",
                "proof_consequence": "largest account-creation payload and verifier surface",
            },
        ],
        "dimension_probe": candidate_rows,
        "recommended_path": (
            "First try `private_linear_commitments_inside_zk` if the proof payload "
            "can avoid exposing C=A*v for private role openings. In parallel, keep "
            "`small_noise_computational_hiding` as the most coherent public-object "
            "fallback. Treat `statistically_hiding_public_kernel` as a hiding fix "
            "that still needs a redesigned binding theorem."
        ),
        "non_claims": [
            "no option is selected by this artifact",
            "rank of A_r only checks linear coverage over F_q, not distributional hiding",
            "payload lower bounds exclude rejection sampling, challenge repetition, and proof-system overhead",
            "the theorem remains blocked until one public-object shape is selected",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Descent/SIS Hiding Redesign Options",
        "",
        "Status: redesign options pinned; not selected.",
        "",
        "## Active Blocker",
        "",
        model["active_blocker"],
        "",
        "## Options",
        "",
        "| id | status | hiding mode | binding mode | proof consequence |",
        "|---|---|---|---|---|",
    ]
    for option in model["options"]:
        lines.append(
            "| `{id}` | {status} | {hiding_mode} | {binding_mode} | "
            "{proof_consequence} |".format(**option)
        )
    lines.extend(
        [
            "",
            "## Dimension Probe",
            "",
            "This probe asks whether a redesigned public commitment with one blinding",
            "column per row has full row-rank blinding submatrices. Full row rank is",
            "necessary for the simple broad-randomness hiding story, but not sufficient",
            "for the final distributional proof.",
            "",
            "| rows | public commitments | base-128 columns | base-128 response bytes | base-128 payload floor | raw columns | raw payload floor |",
            "|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for item in model["dimension_probe"]:
        lines.append(
            "| {rows} | {public_commitment_bytes} | {base128_total_opening_columns} | "
            "{base128_response_bytes_lower_bound} | {base128_payload_lower_bound} | "
            "{raw_total_opening_columns} | {raw_payload_lower_bound} |".format(**item)
        )
    lines.extend(
        [
            "",
            "### Blinding Submatrix Ranks",
            "",
            "| rows | role | blinding columns | rank | full row rank |",
            "|---:|---|---:|---:|---|",
        ]
    )
    for item in model["dimension_probe"]:
        for role_row in item["role_rows"]:
            lines.append(
                "| {rows} | {role} | {blinding_columns} | "
                "{blinding_submatrix_rank} | {blinding_submatrix_full_row_rank} |".format(
                    **role_row
                )
            )
    lines.extend(
        [
            "",
            "## Recommended Path",
            "",
            model["recommended_path"],
            "",
            "## Non-Claims",
            "",
        ]
    )
    for item in model["non_claims"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The cleanest theorem is no longer the current public `C=A*v` object. If",
            "the linear commitments are public, hiding and short-SIS binding pull in",
            "different directions: broad randomness hides but weakens short binding,",
            "while bounded randomness helps binding but requires a computational hiding",
            "assumption. If the proof system can keep the linear commitments private",
            "inside a ZK transcript, that best preserves the selected Path 2 goal.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    model = build_model()
    OUT_JSON.write_text(json.dumps(model, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(write_markdown(model))
    print(json.dumps({"json": str(OUT_JSON), "markdown": str(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
