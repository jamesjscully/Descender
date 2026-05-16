#!/usr/bin/env python3
"""Exact bounded centered-L1 q-ary dual relation check with OR-Tools CP-SAT."""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path
from typing import Any

from ortools.sat.python import cp_model

from descender_cryptanalysis import parse_candidate, public_adjoint_matrix


def exact_check(row: dict[str, Any], timeout_s: float, workers: int) -> dict[str, Any]:
    c = parse_candidate(row)
    q = c.q
    required = int(row["exact"]["qary_amplitude_l1_required"])
    target = required - 1
    adjoint = public_adjoint_matrix(c)
    n = len(adjoint)
    m = len(adjoint[0])

    model = cp_model.CpModel()
    z = [model.new_int_var(-target, target, f"z_{j}") for j in range(m)]
    abs_z = [model.new_int_var(0, target, f"abs_z_{j}") for j in range(m)]
    for zj, aj in zip(z, abs_z):
        model.add_abs_equality(aj, zj)

    model.add(sum(abs_z) >= 1)
    model.add(sum(abs_z) <= target)

    for i, row_i in enumerate(adjoint):
        coeff_sum_bound = target * sum(int(a) for a in row_i)
        k_abs_bound = math.ceil(coeff_sum_bound / q)
        k = model.new_int_var(-k_abs_bound, k_abs_bound, f"k_{i}")
        model.add(sum(int(row_i[j]) * z[j] for j in range(m)) == q * k)

    solver = cp_model.CpSolver()
    solver.parameters.max_time_in_seconds = timeout_s
    solver.parameters.num_search_workers = workers
    solver.parameters.random_seed = 1
    status = solver.solve(model)
    status_name = solver.status_name(status)

    out: dict[str, Any] = {
        "profile": row["profile"],
        "source": row["source"],
        "verdict": row["verdict"],
        "public_key_bytes": row["reported"]["compact_public_key_bytes"],
        "ciphertext_bytes": row["reported"]["ciphertext_bytes"],
        "q": q,
        "n": c.n,
        "m0": c.m0,
        "m1": c.m1,
        "view_coords": c.view_coords,
        "eta": c.eta,
        "required_centered_l1": required,
        "checked_counterexample_l1_at_most": target,
        "solver": "ortools-cp-sat",
        "status": status_name,
        "wall_time_s": solver.wall_time,
        "timeout_s": timeout_s,
        "workers": workers,
    }
    if status in (cp_model.OPTIMAL, cp_model.FEASIBLE):
        witness = [solver.value(v) for v in z]
        out.update(
            {
                "found_counterexample": True,
                "witness_l1": sum(abs(x) for x in witness),
                "witness_support": sum(1 for x in witness if x != 0),
                "witness": witness,
            }
        )
    elif status == cp_model.INFEASIBLE:
        out.update(
            {
                "found_counterexample": False,
                "certified_centered_l1_lower_bound": required,
                "certificate": "infeasible_no_qary_dual_relation_below_required_centered_l1",
            }
        )
    else:
        out.update(
            {
                "found_counterexample": None,
                "certificate": "unknown_not_a_certificate",
            }
        )
    return out


def markdown(results: list[dict[str, Any]]) -> str:
    lines = [
        "# Exact Q-ary Centered-L1 Dual Check",
        "",
        "This report uses OR-Tools CP-SAT to search for a nonzero q-ary dual",
        "relation with centered `l1` below the amplitude threshold.  `INFEASIBLE`",
        "is the exact certificate; `FEASIBLE`/`OPTIMAL` is a counterexample;",
        "`UNKNOWN` is not a certificate.",
        "",
        "| profile | pk | ct | q | n | m | view | req l1 | status | result |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---|---|",
    ]
    for r in results:
        m = int(r["m0"]) + int(r["m1"])
        if r["status"] == "INFEASIBLE":
            result = f"certified l1 >= {r['required_centered_l1']}"
        elif r["found_counterexample"] is True:
            result = f"counterexample l1 {r['witness_l1']}"
        else:
            result = "unknown"
        lines.append(
            f"| {r['profile']} | {r['public_key_bytes']} | {r['ciphertext_bytes']} | "
            f"{r['q']} | {r['n']} | {m} | {r['view_coords']} | "
            f"{r['required_centered_l1']} | {r['status']} | {result} |"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--validation-json", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    parser.add_argument("--top", type=int, default=1)
    parser.add_argument("--timeout-s", type=float, default=300.0)
    parser.add_argument("--workers", type=int, default=8)
    args = parser.parse_args()

    payload = json.loads(args.validation_json.read_text())
    rows = payload["rows"]
    rows.sort(
        key=lambda row: (
            row["profile"],
            row["reported"]["compact_public_key_bytes"],
            row["reported"]["ciphertext_bytes"],
        )
    )
    selected = rows[: args.top]
    results = [exact_check(row, args.timeout_s, args.workers) for row in selected]
    args.output_json.parent.mkdir(parents=True, exist_ok=True)
    args.output_json.write_text(json.dumps(results, indent=2) + "\n")
    args.output_md.write_text(markdown(results))
    print(f"wrote {len(results)} exact checks")


if __name__ == "__main__":
    main()
