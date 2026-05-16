#!/usr/bin/env python3
"""Run small fpylll/fplll sanity reductions on exported q-ary bases."""

import json
from pathlib import Path

try:
    from fpylll import BKZ, GSO, IntegerMatrix, LLL
except Exception as exc:
    raise SystemExit(
        "Missing fpylll/fplll. Install fpylll or use the emitted .txt bases "
        "directly with the fplll CLI. Original error: " + repr(exc)
    )

jobs = json.loads(Path(__file__).with_name("estimator_jobs.json").read_text())

for job in jobs:
    basis_path = Path(__file__).parent / job["dual_qary_lattice"]["basis_file"]
    rows = json.loads(basis_path.read_text())
    A = IntegerMatrix.from_matrix(rows)
    print("\n==", job["id"], "==")
    print("dimension", A.nrows, "x", A.ncols)
    LLL.reduction(A)
    M = GSO.Mat(A)
    M.update_gso()
    print("lll_first_norm2", M.get_r(0, 0))
    # Keep this intentionally modest; serious runs should sweep beta and record
    # machine/runtime metadata.
    BKZ.reduction(A, BKZ.Param(block_size=min(20, A.nrows)))
    M = GSO.Mat(A)
    M.update_gso()
    print("bkz20_first_norm2", M.get_r(0, 0))
