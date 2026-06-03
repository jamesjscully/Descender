#!/usr/bin/env python3
"""Reproducible lattice sanity checks for the full-response quotient/SIS surface.

This is a review harness, not a security proof.  It rebuilds the 12x23
full-response matrix used by the quotient representative relation, constructs
the integer q-kernel lattice, runs local LLL/BKZ reductions when fpylll is
available, and performs the small signed sparse column-relation probe used by
the Rust reference report.
"""

from __future__ import annotations

import argparse
import itertools
import json
import math
import time
from pathlib import Path
from typing import Any, Iterable

import verify_response_link_bounded_kernel_certificate as certver


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OUT = (
    ROOT
    / "research/cryptanalysis/signature_readiness_20260516/"
    "quotient_sis_lattice_attack_review_n32_q12289.json"
)
DEFAULT_RESPONSE_LINK_COORDS = [0, 3]
DEFAULT_RESPONSE_DIFFERENCE_BOUND = 96


def centered_mod_q(value: int) -> int:
    return value % certver.Q


def centered_lift_mod_q(value: int) -> int:
    value %= certver.Q
    return value - certver.Q if value > certver.Q // 2 else value


def lee_weight(values: Iterable[int]) -> int:
    return sum(abs(centered_lift_mod_q(value)) for value in values)


def support(values: Iterable[int]) -> int:
    return sum(1 for value in values if value % certver.Q != 0)


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


def vector_to_mod_q(vector: list[int]) -> list[int]:
    return [centered_mod_q(value) for value in vector]


def syndrome(matrix: list[list[int]], vector: list[int]) -> list[int]:
    vector_mod = vector_to_mod_q(vector)
    return [
        sum(row[col] * vector_mod[col] for col in range(certver.COLS)) % certver.Q
        for row in matrix
    ]


def vector_stats(vector: list[int], matrix: list[list[int]]) -> dict[str, object]:
    stats = certver.vector_stats(vector)
    vector_mod = vector_to_mod_q(vector)
    syn = syndrome(matrix, vector)
    response_link_coords_zero = all(
        vector[coord] % certver.Q == 0 for coord in DEFAULT_RESPONSE_LINK_COORDS
    )
    inside_response_difference_bound = (
        any(value != 0 for value in vector)
        and stats["coeff_bound"] <= DEFAULT_RESPONSE_DIFFERENCE_BOUND
    )
    return {
        **stats,
        "euclidean_norm": math.sqrt(stats["euclidean_squared"]),
        "lee_weight_mod_q": lee_weight(vector_mod),
        "support_mod_q": support(vector_mod),
        "syndrome_zero": all(value == 0 for value in syn),
        "inside_response_difference_bound_96": inside_response_difference_bound,
        "response_link_coords_0_3_zero_mod_q": response_link_coords_zero,
        "would_survive_response_link_code_0_3": inside_response_difference_bound
        and response_link_coords_zero,
        "caught_by_response_link_code_0_3": inside_response_difference_bound
        and not response_link_coords_zero,
        "prefix": vector[:8],
        "vector": vector,
    }


def shortest_row(rows: list[list[int]], matrix: list[list[int]]) -> dict[str, object]:
    return min(
        (vector_stats(row, matrix) for row in rows if any(value != 0 for value in row)),
        key=lambda item: (item["euclidean_squared"], item["coeff_bound"], item["l1_weight"]),
    )


def run_reduction(
    name: str,
    basis: list[list[int]],
    matrix: list[list[int]],
    block_size: int | None = None,
) -> dict[str, object]:
    from fpylll import BKZ, LLL

    lattice = integer_matrix_from_rows(basis)
    started = time.monotonic()
    LLL.reduction(lattice)
    if block_size is not None:
        BKZ.reduction(lattice, BKZ.Param(block_size=block_size))
    rows = rows_from_integer_matrix(lattice)
    candidate = shortest_row(rows, matrix)
    return {
        "name": name,
        "block_size": block_size,
        "wall_time_seconds": time.monotonic() - started,
        "shortest_reduced_basis_row": candidate,
    }


def parse_blocks(text: str) -> list[int]:
    if not text.strip():
        return []
    return [int(item) for item in text.split(",") if item.strip()]


def sparse_signed_relations(
    matrix: list[list[int]],
    max_support: int,
    coefficient_bound: int,
) -> dict[str, object]:
    candidates_tested = 0
    zero_relations_found = 0
    best_nonzero: dict[str, object] | None = None
    best_coefficients: list[int] | None = None

    coeff_values = [
        value for value in range(-coefficient_bound, coefficient_bound + 1) if value != 0
    ]
    for weight in range(1, max_support + 1):
        for cols in itertools.combinations(range(certver.COLS), weight):
            for coeffs in itertools.product(coeff_values, repeat=weight):
                candidates_tested += 1
                relation = [0] * certver.COLS
                for col, coeff in zip(cols, coeffs, strict=True):
                    relation[col] = coeff
                syn = syndrome(matrix, relation)
                if all(value == 0 for value in syn):
                    zero_relations_found += 1
                    continue
                syn_stats = {
                    "syndrome_lee_weight": lee_weight(syn),
                    "syndrome_support": support(syn),
                    "coefficients": relation,
                    "syndrome_prefix": syn[:8],
                }
                if best_nonzero is None or (
                    syn_stats["syndrome_lee_weight"],
                    syn_stats["syndrome_support"],
                ) < (
                    best_nonzero["syndrome_lee_weight"],
                    best_nonzero["syndrome_support"],
                ):
                    best_nonzero = syn_stats
                    best_coefficients = relation

    return {
        "relation_space": "signed sparse public-syndrome relations over columns of the 12x23 full-response matrix",
        "coefficient_bound": coefficient_bound,
        "max_support": max_support,
        "candidates_tested": candidates_tested,
        "zero_relations_found": zero_relations_found,
        "shortest_nonzero_syndrome": best_nonzero,
        "shortest_nonzero_relation": best_coefficients,
    }


def gaussian_heuristic(det_abs: int, dimension: int) -> float:
    return math.sqrt(dimension / (2.0 * math.pi * math.e)) * math.exp(math.log(det_abs) / dimension)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--bkz-blocks", default="10,15,20,23")
    parser.add_argument("--sparse-max-support", type=int, default=3)
    parser.add_argument("--sparse-coeff-bound", type=int, default=2)
    parser.add_argument("--no-reduction", action="store_true")
    parser.add_argument("--write-json", type=Path, default=DEFAULT_OUT)
    args = parser.parse_args()

    matrix = certver.full_response_matrix()
    kernel_basis, rank, kernel_dimension = certver.canonical_intersection_basis(matrix, [])
    det_abs = abs(certver.bareiss_det(kernel_basis))
    basis_hash = certver.vector_list_sha256(
        b"Descender.quotient_sis.full_kernel_basis.v1", kernel_basis
    )

    reductions: list[dict[str, object]] = []
    reduction_error = None
    if not args.no_reduction:
        try:
            reductions.append(run_reduction("LLL", kernel_basis, matrix))
            for block in parse_blocks(args.bkz_blocks):
                reductions.append(
                    run_reduction(f"BKZ-{block}", kernel_basis, matrix, block_size=block)
                )
        except Exception as exc:  # pragma: no cover - depends on local fpylll install
            reduction_error = {"type": type(exc).__name__, "message": str(exc)}

    sparse_probe = sparse_signed_relations(
        matrix, args.sparse_max_support, args.sparse_coeff_bound
    )
    shortest_reduction = None
    if reductions:
        shortest_reduction = min(
            (item["shortest_reduced_basis_row"] for item in reductions),
            key=lambda item: (item["euclidean_squared"], item["coeff_bound"], item["l1_weight"]),
        )

    payload = {
        "schema": "Descender.quotient_sis.lattice_attack_review.v1",
        "statement": {
            "q": certver.Q,
            "matrix_rows": certver.ROWS,
            "matrix_cols": certver.COLS,
            "matrix_domain": certver.DOMAIN.decode(),
            "matrix_transcript_sha256": certver.matrix_transcript_sha256(matrix),
            "rank": rank,
            "kernel_dimension": kernel_dimension,
        },
        "geometry": {
            "integer_lattice_dimension": certver.COLS,
            "determinant_abs": det_abs,
            "determinant_root": math.exp(math.log(det_abs) / certver.COLS),
            "gaussian_heuristic_norm": gaussian_heuristic(det_abs, certver.COLS),
            "kernel_basis_sha256": basis_hash,
        },
        "reductions": reductions,
        "reduction_error": reduction_error,
        "shortest_reduction_row": shortest_reduction,
        "sparse_signed_relation_probe": sparse_probe,
        "status": "review_complete",
        "interpretation": (
            "These checks look for obvious short quotient/SIS relations. A short "
            "full-kernel vector is expected here and explains why the account "
            "response-link code is required; vectors inside the response bound "
            "must not also vanish on coordinates [0,3]. This harness does not "
            "prove keygen security or knowledge soundness."
        ),
    }
    if args.write_json is not None:
        args.write_json.parent.mkdir(parents=True, exist_ok=True)
        args.write_json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(json.dumps(payload, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
