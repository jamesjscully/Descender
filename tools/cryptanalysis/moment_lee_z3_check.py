#!/usr/bin/env python3
"""Exact Z3 check for shifted/scaled moment Lee graph-code distance."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from z3 import If, Int, Or, Solver, Sum, sat, unsat


def centered_residue(value: int, q: int) -> int:
    value %= q
    if value > q // 2:
        return value - q
    return value


def exact_check(payload: dict[str, Any], timeout_ms: int) -> dict[str, Any]:
    q = int(payload["q"])
    target_l1 = int(payload["target_l1"])
    counterexample_l1 = target_l1 - 1
    matrix = [
        [centered_residue(int(x), q) for x in row]
        for row in payload["coefficient_matrix_rows"]
    ]
    rows = len(matrix)
    cols = len(matrix[0]) if matrix else 0

    solver = Solver()
    solver.set(timeout=timeout_ms)

    y = [Int(f"y_{j}") for j in range(cols)]
    abs_y = [Int(f"abs_y_{j}") for j in range(cols)]
    row_values = [Int(f"row_{i}") for i in range(rows)]
    abs_rows = [Int(f"abs_row_{i}") for i in range(rows)]
    quotients = [Int(f"k_{i}") for i in range(rows)]

    for yj, ayj in zip(y, abs_y):
        solver.add(yj >= -counterexample_l1, yj <= counterexample_l1)
        solver.add(ayj == If(yj >= 0, yj, -yj))
    for ri, ari in zip(row_values, abs_rows):
        solver.add(ri >= -counterexample_l1, ri <= counterexample_l1)
        solver.add(ari == If(ri >= 0, ri, -ri))

    solver.add(Sum(abs_y + abs_rows) >= 1)
    solver.add(Sum(abs_y + abs_rows) <= counterexample_l1)

    for i, row in enumerate(matrix):
        solver.add(Sum([int(row[j]) * y[j] for j in range(cols)]) == row_values[i] + q * quotients[i])

    # If the probe found a below-target vector, help Z3 test the encoding by
    # pinning at least one nonzero coordinate from that witness as a branch.
    probe = payload.get("result", {}).get("free_vector_centered")
    if isinstance(probe, list) and any(int(x) != 0 for x in probe):
        solver.add(Or([y[j] == int(x) for j, x in enumerate(probe) if int(x) != 0]))

    status = solver.check()
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
        "solver": "z3",
        "status": str(status),
        "timeout_ms": timeout_ms,
    }
    if status == sat:
        model = solver.model()
        witness = [model.evaluate(v).as_long() for v in y]
        row_witness = [model.evaluate(v).as_long() for v in row_values]
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
    elif status == unsat:
        out.update(
            {
                "found_counterexample": False,
                "certified_centered_l1_lower_bound": target_l1,
                "certificate": "unsat_no_moment_graph_codeword_below_target_l1",
            }
        )
    else:
        out.update(
            {
                "found_counterexample": None,
                "certificate": "unknown_not_a_certificate",
                "reason": solver.reason_unknown(),
            }
        )
    return out


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--probe-json", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--timeout-ms", type=int, default=300_000)
    args = parser.parse_args()

    payload = json.loads(args.probe_json.read_text())
    result = exact_check(payload, args.timeout_ms)
    args.output_json.parent.mkdir(parents=True, exist_ok=True)
    args.output_json.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({k: result[k] for k in ("status", "found_counterexample")}))


if __name__ == "__main__":
    main()
