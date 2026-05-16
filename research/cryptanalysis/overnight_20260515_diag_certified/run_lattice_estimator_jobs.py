#!/usr/bin/env python3
"""Experimental lattice-estimator driver for Descender exported jobs.

This file intentionally refuses to make a security claim automatically.  Review
the `mapping_requires_review` fields in `estimator_jobs.json` before trusting outputs.
"""

import json
from pathlib import Path

try:
    from estimator import LWE, ND
except Exception as exc:
    raise SystemExit(
        "Missing malb/lattice-estimator. Install it in a Sage/Python environment "
        "and ensure `from estimator import LWE, ND` works. Original error: "
        + repr(exc)
    )

jobs = json.loads(Path(__file__).with_name("estimator_jobs.json").read_text())

for job in jobs:
    p = job["lwe_like"]
    print("\n==", job["id"], "==")
    print("CAVEAT:", p["caveat"])
    # This is a proxy mapping.  Adjust Xs/Xe once the Descender-to-LWE reduction
    # has been reviewed for the selected row.
    params = LWE.Parameters(
        n=p["n"],
        q=p["q"],
        Xs=ND.UniformMod(p["q"]),
        Xe=ND.DiscreteGaussian(p["error_distribution_proxy"]["sigma_proxy"]),
        m=p["m"],
    )
    print(LWE.estimate(params))
