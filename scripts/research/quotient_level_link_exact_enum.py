#!/usr/bin/env python3
"""Exact bounded-box enumeration for quotient-level link candidates.

This is a sidecar research checker for Milestone 1.  It generalizes the
coordinate-only response-link exact enumeration to arbitrary linear link rows.
It writes new quotient-level-link artifacts and does not mutate the reviewed
`[0,3]` certificate.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
import time
from fractions import Fraction
from pathlib import Path
from typing import Any

from fpylll import LLL, IntegerMatrix

import quotient_level_link_kernel_search as qll
import response_link_bounded_kernel_check as kernel_check


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_BOUND = 96


def candidate_specs() -> dict[str, dict[str, Any]]:
    return {
        "coordinate-link-1-2": {
            "family": "coordinate_projection",
            "link_rows": [qll.unit_row(1), qll.unit_row(2)],
            "description": (
                "Two-coordinate comparison row with the strongest LLL margin "
                "among the first coordinate-projection probes."
            ),
        },
        "sparse-difference-link-0m3-1m2": {
            "family": "sparse_linear_rows",
            "link_rows": [
                qll.sparse_row([(0, 1), (3, -1)]),
                qll.sparse_row([(1, 1), (2, -1)]),
            ],
            "description": (
                "Sparse two-row link with the strongest LLL margin among the "
                "first hand-picked sparse probes."
            ),
        },
    }


def candidate_spec(candidate: str) -> dict[str, Any]:
    specs = candidate_specs()
    if candidate in specs:
        return specs[candidate]
    match = re.fullmatch(r"coordinate-link-(\d+)-(\d+)", candidate)
    if match:
        left = int(match.group(1))
        right = int(match.group(2))
        if not (0 <= left < right < qll.REPRESENTATIVE_RESIDUES):
            raise ValueError(f"coordinate pair out of range: {candidate}")
        return {
            "family": "coordinate_projection",
            "link_rows": [qll.unit_row(left), qll.unit_row(right)],
            "description": f"Two-coordinate projection candidate [{left},{right}].",
        }
    raise ValueError(f"unknown candidate: {candidate}")


def default_output(candidate: str, bound: int) -> Path:
    return (
        READINESS_DIR
        / f"quotient_level_link_exact_enum_{candidate}_b{bound}.json"
    )


def dot(left: list[Fraction], right: list[Fraction]) -> Fraction:
    return sum((a * b for a, b in zip(left, right)), start=Fraction(0))


def relation_row_holds(
    matrix: list[list[int]], link_rows: list[list[int]], vector: list[int]
) -> bool:
    for row in matrix:
        if sum(row[col] * vector[col] for col in range(qll.REPRESENTATIVE_RESIDUES)) % qll.Q != 0:
            return False
    for row in link_rows:
        if sum(row[col] * vector[col] for col in range(qll.REPRESENTATIVE_RESIDUES)) % qll.Q != 0:
            return False
    return True


def validate_reduced_basis(
    matrix: list[list[int]],
    link_rows: list[list[int]],
    canonical_basis: list[list[int]],
    reduced_basis: list[list[int]],
) -> dict[str, Any]:
    if len(reduced_basis) != qll.REPRESENTATIVE_RESIDUES or any(
        len(row) != qll.REPRESENTATIVE_RESIDUES for row in reduced_basis
    ):
        return {"valid": False, "reason": "basis_shape"}
    if not all(isinstance(value, int) for row in reduced_basis for value in row):
        return {"valid": False, "reason": "basis_entries_not_int"}
    if not all(relation_row_holds(matrix, link_rows, row) for row in reduced_basis):
        return {"valid": False, "reason": "row_not_in_constraint_kernel"}

    canonical_det_abs = abs(kernel_check.bareiss_det(canonical_basis))
    reduced_det_abs = abs(kernel_check.bareiss_det(reduced_basis))
    if canonical_det_abs == 0 or reduced_det_abs == 0:
        return {
            "valid": False,
            "reason": "singular_basis",
            "canonical_det_abs": canonical_det_abs,
            "reduced_det_abs": reduced_det_abs,
        }
    if canonical_det_abs != reduced_det_abs:
        return {
            "valid": False,
            "reason": "determinant_mismatch",
            "canonical_det_abs": canonical_det_abs,
            "reduced_det_abs": reduced_det_abs,
        }
    for row_idx, row in enumerate(reduced_basis):
        if kernel_check.row_lattice_membership_coordinates(canonical_basis, row) is None:
            return {
                "valid": False,
                "reason": "row_not_in_canonical_lattice",
                "row": row_idx,
                "canonical_det_abs": canonical_det_abs,
                "reduced_det_abs": reduced_det_abs,
            }
    return {
        "valid": True,
        "canonical_det_abs": canonical_det_abs,
        "reduced_det_abs": reduced_det_abs,
        "canonical_basis_sha256": kernel_check.vector_list_sha256(
            b"Descender.quotient_level_link.canonical_basis.v1", canonical_basis
        ),
        "reduced_basis_sha256": kernel_check.vector_list_sha256(
            b"Descender.quotient_level_link.reduced_basis.v1", reduced_basis
        ),
    }


def ceil_fraction(value: Fraction) -> int:
    return -((-value.numerator) // value.denominator)


def floor_fraction(value: Fraction) -> int:
    return value.numerator // value.denominator


def exact_interval(offset: Fraction, diag: Fraction, remaining: Fraction) -> tuple[int, int]:
    cover = remaining / diag
    sqrt_upper = (
        math.isqrt((cover.numerator + cover.denominator - 1) // cover.denominator)
        + 2
    )
    center = -offset
    lo = ceil_fraction(center - sqrt_upper)
    hi = floor_fraction(center + sqrt_upper)
    while lo <= hi and (Fraction(lo) + offset) * (Fraction(lo) + offset) * diag > remaining:
        lo += 1
    while lo <= hi and (Fraction(hi) + offset) * (Fraction(hi) + offset) * diag > remaining:
        hi -= 1
    return lo, hi


def run_exact_enum(candidate: str, spec: dict[str, Any], bound: int, timeout: float) -> dict[str, Any]:
    matrix = kernel_check.full_response_matrix()
    link_rows = spec["link_rows"]
    canonical_basis, rank, fiber_dimension = qll.generic_intersection_basis(matrix, link_rows)
    mat = IntegerMatrix.from_matrix(canonical_basis)
    LLL.reduction(mat, delta=0.99)
    reduced_basis = [
        [int(mat[row, col]) for col in range(qll.REPRESENTATIVE_RESIDUES)]
        for row in range(mat.nrows)
    ]
    validation = validate_reduced_basis(matrix, link_rows, canonical_basis, reduced_basis)
    if not validation.get("valid", False):
        return {
            "schema": "Descender.quotient_level_link.exact_enum.v1",
            "candidate": candidate,
            "status": "invalid_reduced_basis",
            "reduced_basis_lattice_validation": validation,
        }

    dimension = len(reduced_basis)
    basis_star: list[list[Fraction]] = []
    diagonal: list[Fraction] = []
    mu = [[Fraction(0) for _ in range(dimension)] for __ in range(dimension)]
    gs_start = time.monotonic()
    for row_idx, basis_row in enumerate(reduced_basis):
        rational_row = [Fraction(value) for value in basis_row]
        star_row = rational_row[:]
        for prev in range(row_idx):
            mu[row_idx][prev] = dot(rational_row, basis_star[prev]) / diagonal[prev]
            for col in range(qll.REPRESENTATIVE_RESIDUES):
                star_row[col] -= mu[row_idx][prev] * basis_star[prev][col]
        basis_star.append(star_row)
        diagonal.append(dot(star_row, star_row))
    gs_seconds = time.monotonic() - gs_start

    coeffs = [0] * dimension
    radius_squared = qll.REPRESENTATIVE_RESIDUES * bound * bound
    start = time.monotonic()
    cover_hash = hashlib.sha256()
    cover_hash.update(b"Descender.quotient_level_link.cover_ball_vectors.v1")
    cover_hash.update(candidate.encode())
    cover_hash.update(qll.Q.to_bytes(4, "little"))
    cover_hash.update(bound.to_bytes(4, "little"))
    for row in link_rows:
        kernel_check.update_signed_vector_hash(cover_hash, row)

    stats: dict[str, Any] = {
        "schema": "Descender.quotient_level_link.exact_enum.v1",
        "candidate": candidate,
        "family": spec["family"],
        "description": spec["description"],
        "q": qll.Q,
        "matrix_rows": qll.SEMANTIC_QUOTIENT_ROWS,
        "matrix_cols": qll.REPRESENTATIVE_RESIDUES,
        "matrix_domain": qll.MATRIX_DOMAIN,
        "matrix_transcript_sha256": kernel_check.matrix_transcript_sha256(matrix),
        "link_rows": link_rows,
        "link_row_count": len(link_rows),
        "link_packed_bits": len(link_rows) * qll.PACKED_Q_BITS,
        "link_packed_bytes": qll.packed_q_residue_bytes(len(link_rows)),
        "combined_constraint_rank": rank,
        "combined_kernel_dimension": fiber_dimension,
        "response_difference_bound": bound,
        "euclidean_cover_radius_squared": radius_squared,
        "canonical_basis_sha256": validation.get("canonical_basis_sha256"),
        "reduced_basis_sha256": validation.get("reduced_basis_sha256"),
        "reduced_basis_lattice_validation": validation,
        "reduced_basis": reduced_basis,
        "gram_schmidt_seconds": gs_seconds,
        "lll_first_vector_stats": qll.vector_stats(reduced_basis[0]),
        "visited_leaves": 0,
        "visited_nodes": 0,
        "nonzero_vectors_in_cover_ball": 0,
        "shortest_vector_in_cover_ball": None,
        "cover_ball_vector_sha256": None,
    }

    best_norm: int | None = None
    best_vector: list[int] | None = None
    witness: list[int] | None = None

    def materialize_vector() -> list[int]:
        vector = [0] * qll.REPRESENTATIVE_RESIDUES
        for row, coeff in enumerate(coeffs):
            if coeff == 0:
                continue
            basis_row = reduced_basis[row]
            for col in range(qll.REPRESENTATIVE_RESIDUES):
                vector[col] += coeff * basis_row[col]
        return vector

    def recurse(row: int, partial_norm: Fraction) -> None:
        nonlocal best_norm, best_vector, witness
        if timeout > 0 and time.monotonic() - start > timeout:
            raise TimeoutError
        stats["visited_nodes"] = int(stats["visited_nodes"]) + 1
        if row < 0:
            stats["visited_leaves"] = int(stats["visited_leaves"]) + 1
            if all(coeff == 0 for coeff in coeffs):
                return
            vector = materialize_vector()
            exact_norm = sum(value * value for value in vector)
            if exact_norm > radius_squared:
                return
            stats["nonzero_vectors_in_cover_ball"] = (
                int(stats["nonzero_vectors_in_cover_ball"]) + 1
            )
            kernel_check.update_signed_vector_hash(cover_hash, vector)
            if best_norm is None or exact_norm < best_norm:
                best_norm = exact_norm
                best_vector = vector
            if max(abs(value) for value in vector) <= bound:
                witness = vector
                raise StopIteration
            return

        remaining = Fraction(radius_squared) - partial_norm
        if remaining < 0:
            return
        offset = sum(
            (Fraction(coeffs[idx]) * mu[idx][row] for idx in range(row + 1, dimension)),
            start=Fraction(0),
        )
        lo, hi = exact_interval(offset, diagonal[row], remaining)
        for coeff in range(lo, hi + 1):
            coeffs[row] = coeff
            term = (Fraction(coeff) + offset) * (Fraction(coeff) + offset) * diagonal[row]
            recurse(row - 1, partial_norm + term)
        coeffs[row] = 0

    try:
        recurse(dimension - 1, Fraction(0))
    except StopIteration:
        stats["status"] = "feasible"
        stats["witness"] = witness
        if witness is not None:
            stats.update(qll.vector_stats(witness))
    except TimeoutError:
        stats["status"] = "timeout"
    else:
        stats["status"] = "infeasible"

    stats["wall_time_seconds"] = time.monotonic() - start
    stats["cover_ball_vector_sha256"] = cover_hash.hexdigest()
    if best_vector is not None:
        stats["shortest_vector_in_cover_ball"] = {
            **qll.vector_stats(best_vector),
            "vector": best_vector,
            "prefix": best_vector[:8],
        }
        stats["shortest_coeff_bound"] = qll.vector_coeff_bound(best_vector)
        stats["margin_over_response_difference_bound"] = (
            qll.vector_coeff_bound(best_vector) - bound
        )
    return stats


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("candidate")
    parser.add_argument("--bound", type=int, default=DEFAULT_BOUND)
    parser.add_argument("--timeout", type=float, default=240.0)
    parser.add_argument("--write-json", type=Path)
    args = parser.parse_args()

    try:
        spec = candidate_spec(args.candidate)
    except ValueError as exc:
        parser.error(str(exc))
    payload = run_exact_enum(args.candidate, spec, args.bound, args.timeout)
    out = args.write_json or default_output(args.candidate, args.bound)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(
        json.dumps(
            {
                "json": str(out),
                "candidate": args.candidate,
                "status": payload.get("status"),
                "wall_time_seconds": payload.get("wall_time_seconds"),
                "shortest_coeff_bound": payload.get("shortest_coeff_bound"),
                "margin_over_response_difference_bound": payload.get(
                    "margin_over_response_difference_bound"
                ),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
