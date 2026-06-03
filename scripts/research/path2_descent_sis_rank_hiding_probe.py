#!/usr/bin/env python3
"""Probe rank and linear hiding risk for Path 2 descent/SIS matrices."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_descent_sis_rank_hiding_probe.json"
OUT_MD = ROOT / "research/path2_descent_sis_rank_hiding_probe.md"

Q = 12_289
REJECTION_LIMIT = 65_535 - (65_535 % Q)
ROW_BANDS = [48, 64]
ROLES = [
    {
        "role": "pad_source",
        "role_id": 1,
        "domain": "DESCENDER_PATH2_DESCENT_SIS_PAD_SOURCE_MATRIX_V1",
        "columns": 30,
        "secret_lanes": 6,
    },
    {
        "role": "projection_relation",
        "role_id": 2,
        "domain": "DESCENDER_PATH2_DESCENT_SIS_PROJECTION_RELATION_MATRIX_V1",
        "columns": 30,
        "secret_lanes": 6,
    },
    {
        "role": "opening_tuple",
        "role_id": 3,
        "domain": "DESCENDER_PATH2_DESCENT_SIS_OPENING_TUPLE_MATRIX_V1",
        "columns": 48,
        "secret_lanes": 8,
    },
    {
        "role": "transcript_commitment",
        "role_id": 4,
        "domain": "DESCENDER_PATH2_DESCENT_SIS_TRANSCRIPT_COMMITMENT_MATRIX_V1",
        "columns": 108,
        "secret_lanes": 20,
    },
]


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


def build_matrix(domain: str, role_id: int, rows: int, cols: int) -> list[list[int]]:
    return [
        [matrix_entry(domain, role_id, rows, row, col) for col in range(cols)]
        for row in range(rows)
    ]


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


def build_model() -> dict[str, object]:
    rows = []
    for role in ROLES:
        for row_band in ROW_BANDS:
            rank = rank_mod_q(
                build_matrix(
                    role["domain"], role["role_id"], row_band, role["columns"]
                )
            )
            columns = role["columns"]
            kernel_dimension = columns - rank
            full_column_rank = rank == columns
            rows.append(
                {
                    "role": role["role"],
                    "role_id": role["role_id"],
                    "row_band": row_band,
                    "columns": columns,
                    "rank_mod_q": rank,
                    "kernel_dimension": kernel_dimension,
                    "secret_lanes": role["secret_lanes"],
                    "full_column_rank": full_column_rank,
                    "linear_hiding_result": (
                        "opening_recoverable_from_commitment"
                        if full_column_rank
                        else "has_linear_kernel"
                    ),
                }
            )
    main_recoverable = [
        row
        for row in rows
        if row["role"] in {"pad_source", "projection_relation", "opening_tuple"}
        and row["full_column_rank"]
    ]
    return {
        "schema": "Descender.path2_descent_sis_rank_hiding_probe.v1",
        "status": "critical_hiding_blocker_identified"
        if main_recoverable
        else "no_main_role_linear_recovery_detected",
        "q": Q,
        "row_results": rows,
        "finding": (
            "The current main role matrices are full column rank for the probed "
            "48/64-row bands, so a public commitment C = A*v mod q is not hiding: "
            "the opening vector is linearly recoverable over F_q. This is not a "
            "binding failure; it is a hiding/design failure."
            if main_recoverable
            else "No full-column-rank main role was detected in the probed bands."
        ),
        "design_consequence": (
            "Do not promote the current three-public-commitment shape. The next "
            "Path 2 descent/SIS design must either make commitments statistically "
            "hiding with more opening columns than rows, keep non-hiding "
            "commitments private inside a ZK proof transcript, or replace the "
            "linear commitment with a proper SIS/LWE-style statistically hiding "
            "commitment shape."
        ),
        "recommended_next_step": (
            "Redesign the feasibility model before hardness estimation: use a "
            "hiding commitment dimension where blinding columns create a large "
            "kernel for each public commitment, then re-run binding, norm/range, "
            "and byte estimates."
        ),
    }


def write_markdown(model: dict[str, object]) -> str:
    lines = [
        "# Path 2 Descent/SIS Rank/Hiding Probe",
        "",
        f"Status: {model['status']}.",
        "",
        "## Finding",
        "",
        str(model["finding"]),
        "",
        "## Rank Results",
        "",
        "| role | rows | columns | rank mod q | kernel dim | result |",
        "|---|---:|---:|---:|---:|---|",
    ]
    for row in model["row_results"]:  # type: ignore[index]
        lines.append(
            "| {role} | {row_band} | {columns} | {rank_mod_q} | "
            "{kernel_dimension} | {linear_hiding_result} |".format(**row)
        )
    lines.extend(
        [
            "",
            "## Design Consequence",
            "",
            str(model["design_consequence"]),
            "",
            "## Recommended Next Step",
            "",
            str(model["recommended_next_step"]),
            "",
            "## Interpretation",
            "",
            "This changes the priority order. Before spending effort on a numeric SIS",
            "hardness estimate, the commitment shape must be made hiding. A full-column",
            "rank `A` can be excellent for binding, but if `C = A*v` is public it also",
            "reveals `v` over the field. The proof payload must not publish that object",
            "for pad/projection/opening roles in the current dimensions.",
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
