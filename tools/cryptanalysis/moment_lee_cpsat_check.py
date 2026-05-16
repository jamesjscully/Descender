#!/usr/bin/env python3
"""Exact CP-SAT check for shifted/scaled moment Lee graph-code distance.

Input is a `vandermonde_lee_probe` JSON artifact.  The script asks the exact
finite question:

    exists y != 0 with Lee(P y) + Lee(y) < target_l1?

`INFEASIBLE` is an external finite certificate for the concrete block.  A
feasible solution is a real counterexample.  `UNKNOWN` is not a certificate.
"""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path
from typing import Any

from ortools.sat.python import cp_model


def centered_residue(value: int, q: int) -> int:
    value %= q
    if value > q // 2:
        return value - q
    return value


def checked_matrix(payload: dict[str, Any]) -> list[list[int]]:
    q = int(payload["q"])
    rows = payload["coefficient_matrix_rows"]
    return [[centered_residue(int(x), q) for x in row] for row in rows]


def relation(matrix: list[list[int]], y: list[int], q: int) -> list[int]:
    return [centered_residue(sum(row[j] * y[j] for j in range(len(y))), q) for row in matrix]


def verified_probe_counterexample(
    payload: dict[str, Any], matrix: list[list[int]], min_free_support: int
) -> dict[str, Any] | None:
    q = int(payload["q"])
    target_l1 = int(payload["target_l1"])
    probe = payload.get("result", {})
    if probe.get("found_counterexample") is not True:
        return None
    y = [int(x) for x in probe.get("free_vector_centered", [])]
    if not y or len(y) != len(matrix[0]):
        return None
    if sum(1 for x in y if x != 0) < min_free_support:
        return None
    rows = relation(matrix, y, q)
    total = sum(abs(x) for x in y) + sum(abs(x) for x in rows)
    if 0 < total < target_l1:
        return {
            "found_counterexample": True,
            "witness_l1": total,
            "free_l1": sum(abs(x) for x in y),
            "row_l1": sum(abs(x) for x in rows),
            "free_support": sum(1 for x in y if x != 0),
            "row_support": sum(1 for x in rows if x != 0),
            "free_vector_centered": y,
            "row_vector_centered": rows,
            "certificate": "verified_probe_counterexample",
        }
    return None


def exact_check(
    payload: dict[str, Any], timeout_s: float, workers: int, min_free_support: int
) -> dict[str, Any]:
    q = int(payload["q"])
    target_l1 = int(payload["target_l1"])
    counterexample_l1 = target_l1 - 1
    matrix = checked_matrix(payload)
    rows = len(matrix)
    cols = len(matrix[0]) if matrix else 0
    verified_probe = verified_probe_counterexample(payload, matrix, min_free_support)

    model = cp_model.CpModel()

    y = [model.new_int_var(-counterexample_l1, counterexample_l1, f"y_{j}") for j in range(cols)]
    abs_y = [model.new_int_var(0, counterexample_l1, f"abs_y_{j}") for j in range(cols)]
    nonzero_y = [model.new_bool_var(f"nonzero_y_{j}") for j in range(cols)]
    for yj, ayj in zip(y, abs_y):
        model.add_abs_equality(ayj, yj)
    for yj, nz in zip(y, nonzero_y):
        model.add(yj != 0).only_enforce_if(nz)
        model.add(yj == 0).only_enforce_if(nz.Not())
    if min_free_support > 0:
        model.add(sum(nonzero_y) >= min_free_support)

    row_values = [
        model.new_int_var(-counterexample_l1, counterexample_l1, f"row_{i}")
        for i in range(rows)
    ]
    abs_rows = [
        model.new_int_var(0, counterexample_l1, f"abs_row_{i}")
        for i in range(rows)
    ]
    for ri, ari in zip(row_values, abs_rows):
        model.add_abs_equality(ari, ri)

    total_l1 = sum(abs_y) + sum(abs_rows)
    model.add(total_l1 >= 1)
    model.add(total_l1 <= counterexample_l1)

    for i, row in enumerate(matrix):
        coeff_abs_sum = sum(abs(int(a)) for a in row)
        lhs_bound = coeff_abs_sum * counterexample_l1
        k_bound = math.ceil((lhs_bound + counterexample_l1) / q)
        k = model.new_int_var(-k_bound, k_bound, f"k_{i}")
        model.add(sum(int(row[j]) * y[j] for j in range(cols)) == row_values[i] + q * k)

    if verified_probe is not None:
        for var, value in zip(y, verified_probe["free_vector_centered"]):
            model.add_hint(var, int(value))
        for var, value in zip(row_values, verified_probe["row_vector_centered"]):
            model.add_hint(var, int(value))

    solver = cp_model.CpSolver()
    solver.parameters.max_time_in_seconds = timeout_s
    solver.parameters.num_search_workers = workers
    solver.parameters.random_seed = 1
    status = solver.solve(model)
    status_name = solver.status_name(status)

    out: dict[str, Any] = {
        "source_family": payload.get("family"),
        "source_q": q,
        "group_size": int(payload["group_size"]),
        "rows": rows,
        "target_l1": target_l1,
        "checked_counterexample_l1_at_most": counterexample_l1,
        "alpha_start": int(payload.get("alpha_start", 0)),
        "exponent_start": int(payload.get("exponent_start", 0)),
        "column_scale": payload.get("column_scale"),
        "seed": payload.get("seed"),
        "solver": "ortools-cp-sat",
        "status": status_name,
        "wall_time_s": solver.wall_time,
        "timeout_s": timeout_s,
        "workers": workers,
        "min_free_support": min_free_support,
    }

    if status in (cp_model.OPTIMAL, cp_model.FEASIBLE):
        witness = [solver.value(v) for v in y]
        row_witness = [solver.value(v) for v in row_values]
        out.update(
            {
                "found_counterexample": True,
                "witness_l1": sum(abs(x) for x in witness) + sum(abs(x) for x in row_witness),
                "free_l1": sum(abs(x) for x in witness),
                "row_l1": sum(abs(x) for x in row_witness),
                "free_support": sum(1 for x in witness if x != 0),
                "row_support": sum(1 for x in row_witness if x != 0),
                "free_vector_centered": witness,
                "row_vector_centered": row_witness,
            }
        )
    elif status == cp_model.INFEASIBLE:
        out.update(
            {
                "found_counterexample": False,
                "certified_centered_l1_lower_bound": target_l1,
                "certificate": "infeasible_no_moment_graph_codeword_below_target_l1",
            }
        )
    else:
        out.update(
            {
                "found_counterexample": None,
                "certificate": "unknown_not_a_certificate",
            }
        )
        if verified_probe is not None:
            out.update(verified_probe)
            out["solver_status_before_probe_fallback"] = status_name
    return out


def markdown(result: dict[str, Any]) -> str:
    if result["found_counterexample"] is False:
        verdict = f"certified Lee distance >= {result['target_l1']}"
    elif result["found_counterexample"] is True:
        verdict = f"counterexample l1 {result['witness_l1']}"
    else:
        verdict = "unknown"
    return "\n".join(
        [
            "# Moment Lee CP-SAT Check",
            "",
            "| field | value |",
            "|---|---:|",
            f"| q | {result['source_q']} |",
            f"| group size | {result['group_size']} |",
            f"| rows | {result['rows']} |",
            f"| target l1 | {result['target_l1']} |",
            f"| checked l1 <= | {result['checked_counterexample_l1_at_most']} |",
            f"| exponent start | {result['exponent_start']} |",
            f"| column scale | {result['column_scale']} |",
            f"| seed | {result['seed']} |",
            f"| status | {result['status']} |",
            f"| wall time s | {result['wall_time_s']:.3f} |",
            f"| verdict | {verdict} |",
            "",
            "`INFEASIBLE` is an exact external certificate for the concrete finite block.",
            "`UNKNOWN` is not a certificate.",
            "",
        ]
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--probe-json", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    parser.add_argument("--timeout-s", type=float, default=300.0)
    parser.add_argument("--workers", type=int, default=8)
    parser.add_argument("--min-free-support", type=int, default=1)
    args = parser.parse_args()

    payload = json.loads(args.probe_json.read_text())
    result = exact_check(payload, args.timeout_s, args.workers, args.min_free_support)
    args.output_json.parent.mkdir(parents=True, exist_ok=True)
    args.output_json.write_text(json.dumps(result, indent=2) + "\n")
    args.output_md.write_text(markdown(result))
    print(json.dumps({k: result[k] for k in ("status", "found_counterexample", "wall_time_s")}))


if __name__ == "__main__":
    main()
