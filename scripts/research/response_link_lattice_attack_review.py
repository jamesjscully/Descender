#!/usr/bin/env python3
"""Run reproducible lattice sanity checks for the response-link certificate.

This is an attack-review harness, not the proof object.  The exact certificate
verifier remains authoritative for the bounded-box exclusion.  This script
rebuilds the same integer lattice from the certificate, checks the pinned hashes
and determinant, then runs local LLL/BKZ reductions when fpylll is available.
It reports whether any reduced-basis vector found by these attacks is a
nonzero vector inside the accepted response-difference box.
"""

from __future__ import annotations

import argparse
import json
import math
import time
from pathlib import Path
from typing import Any

import verify_response_link_bounded_kernel_certificate as certver


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_CERTIFICATE = (
    ROOT
    / "research/cryptanalysis/signature_readiness_20260516/"
    "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)


def parse_blocks(text: str) -> list[int]:
    if not text.strip():
        return []
    return [int(item) for item in text.split(",") if item.strip()]


def vector_stats(vector: list[int], matrix: list[list[int]], coords: list[int], bound: int) -> dict[str, object]:
    relation = certver.row_satisfies_relation(matrix, coords, vector)
    stats = certver.vector_stats(vector)
    nonzero = any(value != 0 for value in vector)
    inside_box = nonzero and relation and stats["coeff_bound"] <= bound
    return {
        **stats,
        "euclidean_norm": math.sqrt(stats["euclidean_squared"]),
        "relation_satisfied": relation,
        "inside_response_difference_box": inside_box,
        "prefix": vector[:8],
        "vector": vector,
    }


def integer_matrix_from_rows(rows: list[list[int]]) -> Any:
    from fpylll import IntegerMatrix

    matrix = IntegerMatrix(len(rows), len(rows[0]))
    for row_idx, row in enumerate(rows):
        for col_idx, value in enumerate(row):
            matrix[row_idx, col_idx] = value
    return matrix


def rows_from_integer_matrix(matrix: Any) -> list[list[int]]:
    return [
        [int(matrix[row_idx, col_idx]) for col_idx in range(matrix.ncols)]
        for row_idx in range(matrix.nrows)
    ]


def shortest_row(rows: list[list[int]], matrix: list[list[int]], coords: list[int], bound: int) -> dict[str, object]:
    return min(
        (vector_stats(row, matrix, coords, bound) for row in rows if any(value != 0 for value in row)),
        key=lambda item: (item["euclidean_squared"], item["coeff_bound"], item["l1_weight"]),
    )


def run_reduction(
    name: str,
    basis: list[list[int]],
    matrix: list[list[int]],
    coords: list[int],
    bound: int,
    block_size: int | None = None,
) -> dict[str, object]:
    from fpylll import BKZ, LLL

    lattice = integer_matrix_from_rows(basis)
    started = time.monotonic()
    LLL.reduction(lattice)
    if block_size is not None:
        BKZ.reduction(lattice, BKZ.Param(block_size=block_size))
    rows = rows_from_integer_matrix(lattice)
    candidate = shortest_row(rows, matrix, coords, bound)
    return {
        "name": name,
        "block_size": block_size,
        "wall_time_seconds": time.monotonic() - started,
        "shortest_reduced_basis_row": candidate,
        "found_box_vector": bool(candidate["inside_response_difference_box"]),
    }


def gaussian_heuristic(det_abs: int, dimension: int) -> float:
    return math.sqrt(dimension / (2.0 * math.pi * math.e)) * math.exp(math.log(det_abs) / dimension)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--certificate", type=Path, default=DEFAULT_CERTIFICATE)
    parser.add_argument(
        "--bkz-blocks",
        default="10,15,20,23",
        help="comma-separated BKZ block sizes; empty string disables BKZ",
    )
    parser.add_argument("--no-reduction", action="store_true")
    parser.add_argument("--write-json", type=Path)
    args = parser.parse_args()

    certificate = json.loads(args.certificate.read_text())
    coords = [int(value) for value in certificate["coords"]]
    bound = int(certificate["bound"])
    reduced_basis = [[int(value) for value in row] for row in certificate["reduced_basis"]]
    matrix = certver.full_response_matrix()
    canonical_basis, rank, kernel_dimension = certver.canonical_intersection_basis(matrix, coords)
    validation = certver.verify_embedded_basis(matrix, coords, canonical_basis, reduced_basis)
    shortest_cover = certificate["shortest_vector_in_cover_ball"]
    det_abs = int(validation["canonical_det_abs"])
    cover_radius_squared = int(certificate["euclidean_cover_radius_squared"])
    cover_radius = math.sqrt(cover_radius_squared)

    reductions: list[dict[str, object]] = []
    reduction_error = None
    if not args.no_reduction:
        try:
            reductions.append(run_reduction("LLL", reduced_basis, matrix, coords, bound))
            for block in parse_blocks(args.bkz_blocks):
                reductions.append(
                    run_reduction(
                        f"BKZ-{block}",
                        reduced_basis,
                        matrix,
                        coords,
                        bound,
                        block_size=block,
                    )
                )
        except Exception as exc:  # pragma: no cover - depends on local fpylll install
            reduction_error = {
                "type": type(exc).__name__,
                "message": str(exc),
            }

    found_box_vector = any(item.get("found_box_vector") for item in reductions)
    payload = {
        "schema": "Descender.response_link.lattice_attack_review.v1",
        "certificate_path": str(args.certificate),
        "statement": {
            "q": certver.Q,
            "matrix_rows": certver.ROWS,
            "matrix_cols": certver.COLS,
            "coords_fixed_mod_q_to_zero": coords,
            "response_difference_bound": bound,
            "matrix_transcript_sha256": certver.matrix_transcript_sha256(matrix),
            "combined_constraint_rank": rank,
            "combined_kernel_dimension": kernel_dimension,
        },
        "basis_validation": validation,
        "geometry": {
            "dimension": certver.COLS,
            "determinant_abs": det_abs,
            "determinant_root": math.exp(math.log(det_abs) / certver.COLS),
            "gaussian_heuristic_norm": gaussian_heuristic(det_abs, certver.COLS),
            "box_cover_radius": cover_radius,
            "box_cover_radius_squared": cover_radius_squared,
            "accepted_box_coeff_bound": bound,
            "shortest_cover_ball_vector_coeff_bound": shortest_cover["coeff_bound"],
            "shortest_cover_ball_vector_euclidean_norm": math.sqrt(shortest_cover["euclidean_squared"]),
            "shortest_cover_ball_vector_margin_over_box_bound": shortest_cover["coeff_bound"] - bound,
        },
        "reductions": reductions,
        "reduction_error": reduction_error,
        "status": "box_vector_found" if found_box_vector else "no_box_vector_found_by_reductions",
        "interpretation": (
            "LLL/BKZ rows are independent attack sanity checks only. The exact "
            "bounded enumeration certificate remains the proof object for box exclusion."
        ),
    }
    if args.write_json is not None:
        args.write_json.parent.mkdir(parents=True, exist_ok=True)
        args.write_json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(json.dumps(payload, indent=2, sort_keys=True))
    if found_box_vector:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
