#!/usr/bin/env python3
"""Extract reduced q-ary dual vectors in Descender coordinates.

Input is a cryptanalysis dossier produced by `descender_cryptanalysis.py`.
For each exported q-ary dual basis, this runs LLL and modest BKZ, then writes
the first reduced lattice vector split into Descender guard and tilt
coordinates.

The output is diagnostic only.  A short Euclidean vector is not automatically a
Descender attack; the key question is whether it has the forbidden support/norm
shape relevant to the proof certificate or a reviewed cryptanalytic attack
model.
"""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path
from typing import Any

try:
    from fpylll import BKZ, GSO, IntegerMatrix, LLL
except Exception as exc:  # pragma: no cover - environment dependent
    raise SystemExit(
        "Missing fpylll/fplll. Run this inside the Sage/fpylll Docker image or "
        "install fpylll locally. Original error: " + repr(exc)
    )


def load_json(path: Path) -> Any:
    return json.loads(path.read_text())


def dot_residuals(adjoint: list[list[int]], vector: list[int]) -> list[int]:
    return [sum(a * x for a, x in zip(row, vector)) for row in adjoint]


def support(indices: list[int], offset: int = 0) -> list[dict[str, int]]:
    return [{"index": offset + i, "value": value} for i, value in enumerate(indices) if value != 0]


def vector_stats(vector: list[int]) -> dict[str, Any]:
    nonzero = [x for x in vector if x != 0]
    return {
        "support": len(nonzero),
        "l1": sum(abs(x) for x in vector),
        "l2_norm2": sum(x * x for x in vector),
        "l2": math.sqrt(sum(x * x for x in vector)),
        "linf": max((abs(x) for x in vector), default=0),
    }


def reduce_first_vector(rows: list[list[int]], beta: int) -> tuple[list[int], dict[str, float]]:
    basis = IntegerMatrix.from_matrix(rows)
    LLL.reduction(basis)
    lll_gso = GSO.Mat(basis)
    lll_gso.update_gso()
    lll_norm2 = float(lll_gso.get_r(0, 0))

    BKZ.reduction(basis, BKZ.Param(block_size=min(beta, basis.nrows)))
    bkz_gso = GSO.Mat(basis)
    bkz_gso.update_gso()
    bkz_norm2 = float(bkz_gso.get_r(0, 0))
    vector = [int(basis[0, j]) for j in range(basis.ncols)]
    return vector, {"lll_first_norm2": lll_norm2, "bkz_first_norm2": bkz_norm2}


def analyze_job(root: Path, job: dict[str, Any], candidate: dict[str, Any], beta: int) -> dict[str, Any]:
    q = int(candidate["q"])
    n = int(candidate["n"])
    m0 = int(candidate["m0"])
    m1 = int(candidate["m1"])
    basis_path = root / job["dual_qary_lattice"]["basis_file"]
    matrix_path = root / "matrices" / f"{job['id']}.adjoint_modq.json"
    basis = load_json(basis_path)
    adjoint = load_json(matrix_path)

    vector, reduction = reduce_first_vector(basis, beta)
    guard = vector[:m0]
    tilt = vector[m0:]
    residuals = dot_residuals(adjoint, vector)
    residual_mod_q = [r % q for r in residuals]
    total_stats = vector_stats(vector)
    guard_stats = vector_stats(guard)
    tilt_stats = vector_stats(tilt)
    certificate_limit = int(job["code_like"]["weight_target"]) - 1

    return {
        "id": job["id"],
        "profile": candidate.get("profile"),
        "q": q,
        "n": n,
        "m0": m0,
        "m1": m1,
        "dimension": len(vector),
        "bkz_block_size": min(beta, len(vector)),
        "reduction": reduction,
        "descender_coordinate_order": {
            "guard": f"coordinates [0, {m0})",
            "tilt": f"coordinates [{m0}, {m0 + m1})",
        },
        "vector": vector,
        "guard_vector": guard,
        "tilt_vector": tilt,
        "nonzero_coordinates": support(vector),
        "guard_nonzero_coordinates": support(guard),
        "tilt_nonzero_coordinates": support(tilt, offset=m0),
        "stats": {
            "total": total_stats,
            "guard": guard_stats,
            "tilt": tilt_stats,
        },
        "adjoint_residual": {
            "all_zero_mod_q": all(x == 0 for x in residual_mod_q),
            "max_abs_integer_residual": max((abs(x) for x in residuals), default=0),
            "residual_mod_q": residual_mod_q,
        },
        "proof_certificate_comparison": {
            "no_dual_witness_up_to_hamming_weight": certificate_limit,
            "hamming_support": total_stats["support"],
            "violates_recorded_hamming_certificate": total_stats["support"] <= certificate_limit,
        },
    }


def markdown(results: list[dict[str, Any]]) -> str:
    lines = [
        "# Reduced Dual Vectors In Descender Coordinates",
        "",
        "Each row is the first BKZ-reduced vector from the exported q-ary dual lattice.",
        "Coordinates are split as guard coordinates first and tilt coordinates second.",
        "",
        "| id | beta | support | guard support | tilt support | l2^2 | linf | mod-q residual ok | cert violation |",
        "|---|---:|---:|---:|---:|---:|---:|---|---|",
    ]
    for r in results:
        stats = r["stats"]
        lines.append(
            f"| `{r['id']}` | {r['bkz_block_size']} | {stats['total']['support']} | "
            f"{stats['guard']['support']} | {stats['tilt']['support']} | "
            f"{stats['total']['l2_norm2']} | {stats['total']['linf']} | "
            f"{r['adjoint_residual']['all_zero_mod_q']} | "
            f"{r['proof_certificate_comparison']['violates_recorded_hamming_certificate']} |"
        )
    lines.extend(
        [
            "",
            "Interpretation:",
            "",
            "- `mod-q residual ok` confirms the vector is a valid q-ary dual vector for the exported public adjoint matrix.",
            "- `cert violation` only checks the recorded Hamming-weight certificate boundary.",
            "- High-support short Euclidean vectors are cryptanalytic warning signs, but not direct proof-certificate counterexamples.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dossier-dir", type=Path, required=True)
    parser.add_argument("--beta", type=int, default=20)
    parser.add_argument("--output-json", type=Path)
    parser.add_argument("--output-md", type=Path)
    args = parser.parse_args()

    root = args.dossier_dir
    jobs = load_json(root / "estimator_jobs.json")
    candidates = {row["id"]: row["candidate"] | {"profile": row["profile"]} for row in load_json(root / "candidates.json")}
    results = [analyze_job(root, job, candidates[job["id"]], args.beta) for job in jobs]

    output_json = args.output_json or root / f"reduced_dual_vectors_bkz{args.beta}.json"
    output_md = args.output_md or root / f"reduced_dual_vectors_bkz{args.beta}.md"
    output_json.write_text(json.dumps(results, indent=2) + "\n")
    output_md.write_text(markdown(results))
    print(f"wrote {len(results)} reduced vector diagnostics to {output_json} and {output_md}")


if __name__ == "__main__":
    main()
