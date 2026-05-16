#!/usr/bin/env python3
"""Exact bounded centered-L1 q-ary dual relation check with Z3.

Given a `validate_frontier` report, this script regenerates the public adjoint
matrix `A` for selected rows and asks Z3 the direct certificate question:

    exists z != 0 with A z = 0 mod q and ||z||_1 < required?

If Z3 returns UNSAT for a row, then the row has the exact amplitude certificate
needed by the deterministic proof boundary for that required centered-L1
threshold.  SAT gives a real dual-relation counterexample.  UNKNOWN is not a
certificate.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from z3 import If, Int, Or, Solver, Sum, sat, unsat

from descender_cryptanalysis import parse_candidate, public_adjoint_matrix


def exact_check(row: dict[str, Any], timeout_ms: int) -> dict[str, Any]:
    c = parse_candidate(row)
    q = c.q
    required = int(row["exact"]["qary_amplitude_l1_required"])
    target = required - 1
    adjoint = public_adjoint_matrix(c)
    n = len(adjoint)
    m = len(adjoint[0])

    z = [Int(f"z_{j}") for j in range(m)]
    abs_z = [Int(f"abs_z_{j}") for j in range(m)]
    quot = [Int(f"k_{i}") for i in range(n)]
    solver = Solver()
    solver.set(timeout=timeout_ms)

    for zj, aj in zip(z, abs_z):
        solver.add(zj >= -target, zj <= target)
        solver.add(aj == If(zj >= 0, zj, -zj))

    l1 = Sum(abs_z)
    solver.add(l1 >= 1)
    solver.add(l1 <= target)

    for i, row_i in enumerate(adjoint):
        solver.add(Sum([int(row_i[j]) * z[j] for j in range(m)]) == q * quot[i])

    result = solver.check()
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
        "z3_result": str(result),
        "timeout_ms": timeout_ms,
    }

    if result == sat:
        model = solver.model()
        witness = [model.evaluate(zj).as_long() for zj in z]
        witness_l1 = sum(abs(x) for x in witness)
        out.update(
            {
                "found_counterexample": True,
                "witness_l1": witness_l1,
                "witness_support": sum(1 for x in witness if x != 0),
                "witness": witness,
            }
        )
    elif result == unsat:
        out.update(
            {
                "found_counterexample": False,
                "certified_centered_l1_lower_bound": required,
                "certificate": "unsat_no_qary_dual_relation_below_required_centered_l1",
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


def markdown(results: list[dict[str, Any]]) -> str:
    lines = [
        "# Exact Q-ary Centered-L1 Dual Check",
        "",
        "This report asks Z3 whether there is a nonzero q-ary dual relation",
        "`z` with centered `l1` below the amplitude threshold required by the",
        "deterministic proof boundary.  `unsat` is a proof-side certificate for",
        "the checked row; `sat` is a real counterexample; `unknown` is not a",
        "certificate.",
        "",
        "| profile | pk | ct | q | n | m | view | req l1 | Z3 | result |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---|---|",
    ]
    for r in results:
        m = int(r["m0"]) + int(r["m1"])
        if r["z3_result"] == "unsat":
            result = f"certified l1 >= {r['required_centered_l1']}"
        elif r["z3_result"] == "sat":
            result = f"counterexample l1 {r['witness_l1']}"
        else:
            result = r.get("reason", "unknown")
        lines.append(
            f"| {r['profile']} | {r['public_key_bytes']} | {r['ciphertext_bytes']} | "
            f"{r['q']} | {r['n']} | {m} | {r['view_coords']} | "
            f"{r['required_centered_l1']} | {r['z3_result']} | {result} |"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--validation-json", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    parser.add_argument("--top", type=int, default=1)
    parser.add_argument("--timeout-ms", type=int, default=300_000)
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
    results = [exact_check(row, args.timeout_ms) for row in selected]

    args.output_json.parent.mkdir(parents=True, exist_ok=True)
    args.output_json.write_text(json.dumps(results, indent=2) + "\n")
    args.output_md.write_text(markdown(results))
    print(f"wrote {len(results)} exact checks")


if __name__ == "__main__":
    main()
