#!/usr/bin/env python3
"""Reduce q-ary dual lattices for the selected public view only."""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path
from typing import Any

try:
    from fpylll import BKZ, GSO, IntegerMatrix, LLL
except Exception as exc:  # pragma: no cover
    raise SystemExit("Missing fpylll/fplll: " + repr(exc))

from descender_cryptanalysis import dual_qary_basis_from_adjoint


def load_json(path: Path) -> Any:
    return json.loads(path.read_text())


def vector_stats(vector: list[int], m0: int) -> dict[str, Any]:
    guard = vector[: min(m0, len(vector))]
    rest = vector[min(m0, len(vector)) :]
    return {
        "support": sum(1 for x in vector if x),
        "guard_support": sum(1 for x in guard if x),
        "non_guard_support": sum(1 for x in rest if x),
        "l1": sum(abs(x) for x in vector),
        "guard_l1": sum(abs(x) for x in guard),
        "non_guard_l1": sum(abs(x) for x in rest),
        "l2_norm2": sum(x * x for x in vector),
        "l2": math.sqrt(sum(x * x for x in vector)),
        "linf": max((abs(x) for x in vector), default=0),
    }


def dot_residuals(adjoint: list[list[int]], vector: list[int], q: int) -> list[int]:
    return [sum(a * x for a, x in zip(row, vector)) % q for row in adjoint]


def reduce_first_vector(basis: list[list[int]], beta: int) -> tuple[list[int], dict[str, float]]:
    matrix = IntegerMatrix.from_matrix(basis)
    LLL.reduction(matrix)
    gso = GSO.Mat(matrix)
    gso.update_gso()
    lll_norm2 = float(gso.get_r(0, 0))
    BKZ.reduction(matrix, BKZ.Param(block_size=min(beta, matrix.nrows)))
    gso = GSO.Mat(matrix)
    gso.update_gso()
    bkz_norm2 = float(gso.get_r(0, 0))
    return [int(matrix[0, j]) for j in range(matrix.ncols)], {
        "lll_first_norm2": lll_norm2,
        "bkz_first_norm2": bkz_norm2,
    }


def analyze(root: Path, row: dict[str, Any], beta: int) -> dict[str, Any]:
    c = row["candidate"]
    q = int(c["q"])
    m0 = int(c["m0"])
    m1 = int(c["m1"])
    view_len = int(c.get("view_coords") or (m0 + m1))
    full = load_json(root / "matrices" / f"{row['id']}.adjoint_modq.json")
    view = [r[:view_len] for r in full]
    basis = dual_qary_basis_from_adjoint(view, q)
    vector, reduction = reduce_first_vector(basis, beta)
    residual = dot_residuals(view, vector, q)
    effective_eta = int(row["effective_eta"])
    required_l1 = math.ceil((q - 1) / (2 * effective_eta))
    stats = vector_stats(vector, m0)
    return {
        "id": row["id"],
        "candidate": c,
        "q": q,
        "view_columns": view_len,
        "basis_dimension": len(basis),
        "bkz_block_size": min(beta, len(basis)),
        "effective_eta": effective_eta,
        "required_centered_l1": required_l1,
        "reduction": reduction,
        "stats": stats,
        "residual_all_zero_mod_q": all(x == 0 for x in residual),
        "direct_view_counterexample": stats["l1"] < required_l1,
        "vector": vector,
    }


def markdown(results: list[dict[str, Any]]) -> str:
    lines = [
        "# View-only Reduced Dual Vectors",
        "",
        "These reductions use only the selected public coordinate view, not the full",
        "`m0 + m1` regenerated matrix.",
        "",
        "| id | beta | view cols | req l1 | l1 | support | l2^2 | linf | residual ok | view counterexample |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---|---|",
    ]
    for r in results:
        stats = r["stats"]
        lines.append(
            f"| `{r['id']}` | {r['bkz_block_size']} | {r['view_columns']} | "
            f"{r['required_centered_l1']} | {stats['l1']} | {stats['support']} | "
            f"{stats['l2_norm2']} | {stats['linf']} | "
            f"{r['residual_all_zero_mod_q']} | {r['direct_view_counterexample']} |"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dossier-dir", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    parser.add_argument("--limit", type=int, default=4)
    parser.add_argument("--beta", type=int, default=20)
    args = parser.parse_args()

    candidates = load_json(args.dossier_dir / "candidates.json")
    selected = [
        row
        for row in candidates
        if int(row["candidate"]["n"]) == 32 and int(row["candidate"]["m1"]) == 32
    ][: args.limit]
    results = [analyze(args.dossier_dir, row, args.beta) for row in selected]
    args.output_json.parent.mkdir(parents=True, exist_ok=True)
    args.output_json.write_text(json.dumps(results, indent=2) + "\n")
    args.output_md.write_text(markdown(results))
    print(f"wrote {len(results)} view reductions to {args.output_json}")


if __name__ == "__main__":
    main()
