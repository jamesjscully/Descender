#!/usr/bin/env python3
"""Probe linear leakage in the Path 2 two-view masked leaf equations."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_two_view_linear_leakage_probe.json"
OUT_MD = ROOT / "research/path2_two_view_linear_leakage_probe.md"
Q = 12_289


def rank_mod_q(rows: list[list[int]], q: int = Q) -> int:
    if not rows:
        return 0
    matrix = [[value % q for value in row] for row in rows]
    height = len(matrix)
    width = len(matrix[0])
    rank = 0
    for col in range(width):
        pivot = None
        for row in range(rank, height):
            if matrix[row][col] % q:
                pivot = row
                break
        if pivot is None:
            continue
        matrix[rank], matrix[pivot] = matrix[pivot], matrix[rank]
        inv = pow(matrix[rank][col], -1, q)
        matrix[rank] = [(value * inv) % q for value in matrix[rank]]
        for row in range(height):
            if row == rank:
                continue
            factor = matrix[row][col] % q
            if factor:
                matrix[row] = [
                    (matrix[row][i] - factor * matrix[rank][i]) % q
                    for i in range(width)
                ]
        rank += 1
        if rank == height:
            break
    return rank


def projection_rank(rows: list[list[int]], secret_cols: int, q: int = Q) -> int:
    """Return rank over secret columns after eliminating mask-only columns.

    A secret leaks linearly iff a nonzero linear combination of public equations
    cancels every mask coefficient while leaving a nonzero secret coefficient.
    This is the dimension of the projection of the left nullspace of mask columns
    onto secret columns.
    """

    if not rows:
        return 0
    total_cols = len(rows[0])
    mask_cols = total_cols - secret_cols
    # Brute-force the small scenarios through row-reduced mask equations:
    # leakage dimension = rank([secret|mask]) - rank(mask), for equation spans.
    full_rank = rank_mod_q(rows, q)
    mask_rows = [row[secret_cols:] for row in rows]
    mask_rank = rank_mod_q(mask_rows, q)
    return full_rank - mask_rank


def build_rows(secret_cols: int, openings: list[tuple[int, int]]) -> list[list[int]]:
    """Rows for public openings y = secret_i + mask_j.

    Constants y do not affect leakage rank. The matrix tracks coefficients of
    hidden secrets and masks in opened linear equations.
    """

    mask_count = 1 + max(mask for _, mask in openings)
    rows = []
    for secret, mask in openings:
        row = [0] * (secret_cols + mask_count)
        row[secret] = 1
        row[secret_cols + mask] = 1
        rows.append(row)
    return rows


def scenario(
    scenario_id: str,
    secret_cols: int,
    openings: list[tuple[int, int]],
    expectation: str,
) -> dict[str, Any]:
    rows = build_rows(secret_cols, openings)
    leak_rank = projection_rank(rows, secret_cols)
    return {
        "id": scenario_id,
        "secret_cols": secret_cols,
        "opened_equations": len(openings),
        "distinct_masks": len({mask for _, mask in openings}),
        "equation_rank": rank_mod_q(rows),
        "secret_leakage_rank": leak_rank,
        "expectation": expectation,
        "passes": (leak_rank == 0 if expectation == "no_leak" else leak_rank > 0),
    }


def build_model() -> dict[str, Any]:
    scenarios = [
        scenario(
            "fresh_mask_per_opening",
            8,
            [(secret, secret) for secret in range(8)],
            "no_leak",
        ),
        scenario(
            "same_secret_two_fresh_masks",
            1,
            [(0, 0), (0, 1)],
            "no_leak",
        ),
        scenario(
            "two_secrets_reuse_one_mask",
            2,
            [(0, 0), (1, 0)],
            "leak",
        ),
        scenario(
            "all_digits_reuse_one_role_mask",
            8,
            [(secret, 0) for secret in range(8)],
            "leak",
        ),
        scenario(
            "domain_separated_role_masks",
            8,
            [(secret, secret + 8 * role) for role in range(2) for secret in range(8)],
            "no_leak",
        ),
    ]
    failures = [item for item in scenarios if not item["passes"]]
    return {
        "schema": "Descender.path2_two_view_linear_leakage_probe.v1",
        "status": "pass" if not failures else "fail",
        "q": Q,
        "scenarios": scenarios,
        "finding": (
            "Fresh per-opening masks leave zero linear leakage in this model. "
            "Reusing one mask across two hidden values leaks one or more linear "
            "relations. Therefore the two-view theorem must require row/column/"
            "role/query domain separation for every opened masked value."
        ),
        "required_rule": (
            "No mask variable may appear in public equations for two different "
            "hidden values unless the proof system proves an independent "
            "simulator theorem for that reuse pattern."
        ),
        "next_blocker": (
            "Lift this linear rule into the column-profile specification: every "
            "opened_masked column must have a unique mask domain or an explicitly "
            "named simulator lemma."
        ),
        "failures": failures,
        "non_claims": [
            "this is a linear leakage sanity probe, not a full ZK proof",
            "this does not model nonlinear range gadgets",
            "this does not prove mask pseudorandomness",
            "this does not make Path 2 deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Two-View Linear Leakage Probe",
        "",
        f"Status: `{model['status']}`.",
        "",
        model["finding"],
        "",
        f"- q: `{model['q']}`",
        "",
        "## Scenarios",
        "",
        "| id | secrets | openings | masks | equation rank | secret leakage rank | expectation | passes |",
        "|---|---:|---:|---:|---:|---:|---|---|",
    ]
    for item in model["scenarios"]:
        lines.append(
            "| `{id}` | {secret_cols} | {opened_equations} | {distinct_masks} | "
            "{equation_rank} | {secret_leakage_rank} | `{expectation}` | `{passes}` |".format(
                **item
            )
        )
    lines.extend(
        [
            "",
            "## Required Rule",
            "",
            model["required_rule"],
            "",
            "## Next Blocker",
            "",
            model["next_blocker"],
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
