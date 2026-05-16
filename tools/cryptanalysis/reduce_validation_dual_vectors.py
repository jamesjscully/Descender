#!/usr/bin/env python3
"""Reduce q-ary dual lattices for selected validation rows."""

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

from descender_cryptanalysis import Candidate, dual_qary_basis_from_adjoint, public_adjoint_matrix


def load_json(path: Path) -> Any:
    return json.loads(path.read_text())


def stats(vector: list[int]) -> dict[str, Any]:
    return {
        "support": sum(1 for x in vector if x),
        "l1": sum(abs(x) for x in vector),
        "l2_norm2": sum(x * x for x in vector),
        "linf": max((abs(x) for x in vector), default=0),
    }


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


def row_effective_eta(row: dict[str, Any], c_eta: int) -> int:
    return int(row.get("reported", {}).get("effective_eta", c_eta))


def analyze(row: dict[str, Any], index: int, beta: int) -> dict[str, Any]:
    raw = row["candidate"]
    c = Candidate(
        profile=row["profile"],
        proof_strategy=raw.get("proof_strategy", "unknown"),
        primal_distance_model=raw.get("primal_distance_model", "unknown"),
        q=int(raw["q"]),
        n=int(raw["n"]),
        m0=int(raw["m0"]),
        m1=int(raw["m1"]),
        fiber_dim=int(raw["fiber_dim"]),
        eta=int(raw["eta"]),
        sample_bits=int(raw.get("sample_bits", 0)),
        view_coords=int(raw.get("view_coords", 0)),
    )
    full = public_adjoint_matrix(c)
    view_len = c.view_len
    view = [r[:view_len] for r in full]
    basis = dual_qary_basis_from_adjoint(view, c.q)
    vector, reduction = reduce_first_vector(basis, beta)
    effective_eta = row_effective_eta(row, c.eta)
    exact_required = row.get("exact", {}).get("qary_amplitude_l1_required")
    required_l1 = int(exact_required) if exact_required is not None else math.ceil((c.q - 1) / (2 * effective_eta))
    residual = [sum(a * x for a, x in zip(r, vector)) % c.q for r in view]
    s = stats(vector)
    return {
        "index": index,
        "profile": row["profile"],
        "source": row.get("source"),
        "verdict": row.get("verdict"),
        "candidate": row["candidate"],
        "q": c.q,
        "n": c.n,
        "m0": c.m0,
        "m1": c.m1,
        "view_columns": view_len,
        "effective_eta": effective_eta,
        "required_centered_l1": required_l1,
        "bkz_block_size": min(beta, len(basis)),
        "reduction": reduction,
        "stats": s,
        "residual_all_zero_mod_q": all(x == 0 for x in residual),
        "direct_counterexample": s["l1"] < required_l1,
        "vector": vector,
    }


def markdown(results: list[dict[str, Any]]) -> str:
    lines = [
        "# Validation-row Reduced Dual Vectors",
        "",
        "| idx | q | n | m0 | m1 | view | eta_eff | req l1 | l1 | support | linf | counterexample |",
        "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|",
    ]
    for r in results:
        s = r["stats"]
        lines.append(
            f"| {r['index']} | {r['q']} | {r['n']} | {r['m0']} | {r['m1']} | "
            f"{r['view_columns']} | {r['effective_eta']} | {r['required_centered_l1']} | "
            f"{s['l1']} | {s['support']} | {s['linf']} | {r['direct_counterexample']} |"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--validation-json", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    parser.add_argument("--profile", default="descender-128")
    parser.add_argument("--n", type=int, default=32)
    parser.add_argument("--m1", type=int, default=32)
    parser.add_argument("--limit", type=int, default=8)
    parser.add_argument("--beta", type=int, default=20)
    args = parser.parse_args()

    payload = load_json(args.validation_json)
    selected = []
    for index, row in enumerate(payload["rows"]):
        c = row["candidate"]
        if row["profile"] == args.profile and int(c["n"]) == args.n and int(c["m1"]) == args.m1:
            selected.append((index, row))
        if len(selected) >= args.limit:
            break
    results = [analyze(row, index, args.beta) for index, row in selected]
    args.output_json.parent.mkdir(parents=True, exist_ok=True)
    args.output_json.write_text(json.dumps(results, indent=2) + "\n")
    args.output_md.write_text(markdown(results))
    print(f"wrote {len(results)} validation-row reductions to {args.output_json}")


if __name__ == "__main__":
    main()
