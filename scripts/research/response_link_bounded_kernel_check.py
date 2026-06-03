#!/usr/bin/env python3
"""Check bounded response-link kernel candidates.

This script reconstructs the same q=12289 12x23 full-response matrix used by
`signature_q12289_local_delta_stateful.rs` and checks whether a nonzero signed
integer delta exists in the response-difference box with

    L(delta) = 0 mod q
    C(delta) = 0

where `C` is a coordinate projection such as `[0, 3]`.

The `lll` mode is fast evidence.  The `lattice-enum` mode enumerates the
Euclidean cover ball quickly with fpylll GSO data.  The `exact-enum` mode
replays that enumeration with rational pruning and exact integer leaf checks.
The `cp-sat` and `scip` modes are exact integer feasibility checks, but may
return UNKNOWN under the timeout.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import shutil
import subprocess
import tempfile
import time
from fractions import Fraction
from pathlib import Path
from typing import Literal

Q = 12289
ROWS = 12
COLS = 23
DOMAIN = b"Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1"


def matrix_entry(row: int, col: int) -> int:
    rejection_limit = 65535 - (65535 % Q)
    counter = 0
    while True:
        h = hashlib.sha256()
        h.update(DOMAIN)
        h.update(row.to_bytes(2, "little"))
        h.update(col.to_bytes(2, "little"))
        h.update(counter.to_bytes(4, "little"))
        digest = h.digest()
        for idx in range(0, len(digest), 2):
            value = int.from_bytes(digest[idx : idx + 2], "little")
            if value < rejection_limit:
                return value % Q
        counter += 1


def full_response_matrix() -> list[list[int]]:
    return [[matrix_entry(row, col) for col in range(COLS)] for row in range(ROWS)]


def matrix_transcript_sha256(matrix: list[list[int]]) -> str:
    h = hashlib.sha256()
    for row in matrix:
        for value in row:
            h.update(value.to_bytes(2, "little"))
    return h.hexdigest()


def update_signed_vector_hash(h: "hashlib._Hash", vector: list[int]) -> None:
    h.update(len(vector).to_bytes(2, "little"))
    for value in vector:
        h.update(int(value).to_bytes(8, "little", signed=True))


def vector_list_sha256(domain: bytes, vectors: list[list[int]]) -> str:
    h = hashlib.sha256()
    h.update(domain)
    h.update(len(vectors).to_bytes(4, "little"))
    for vector in vectors:
        update_signed_vector_hash(h, vector)
    return h.hexdigest()


def centered_lift(value: int) -> int:
    return value - Q if value > Q // 2 else value


def rref_mod_q(rows: list[list[int]]) -> tuple[list[list[int]], list[int]]:
    work = [row[:] for row in rows]
    rank = 0
    pivots: list[int] = []
    for col in range(COLS):
        pivot = next((row for row in range(rank, len(work)) if work[row][col] % Q), None)
        if pivot is None:
            continue
        work[rank], work[pivot] = work[pivot], work[rank]
        inv = pow(work[rank][col] % Q, -1, Q)
        work[rank] = [(value * inv) % Q for value in work[rank]]
        for row in range(len(work)):
            if row == rank or work[row][col] % Q == 0:
                continue
            factor = work[row][col] % Q
            work[row] = [(work[row][idx] - factor * work[rank][idx]) % Q for idx in range(COLS)]
        pivots.append(col)
        rank += 1
        if rank == len(work):
            break
    return work, pivots


def integer_intersection_basis(matrix: list[list[int]], coords: list[int]) -> tuple[list[list[int]], int, int]:
    rows = [row[:] for row in matrix]
    for coord in coords:
        row = [0] * COLS
        row[coord] = 1
        rows.append(row)
    rref, pivots = rref_mod_q(rows)
    free_cols = [col for col in range(COLS) if col not in pivots]
    basis: list[list[int]] = []
    for free_col in free_cols:
        vector = [0] * COLS
        vector[free_col] = 1
        for row, pivot_col in enumerate(pivots):
            vector[pivot_col] = (-rref[row][free_col]) % Q
        basis.append([centered_lift(value) for value in vector])
    for pivot_col in pivots:
        vector = [0] * COLS
        vector[pivot_col] = Q
        basis.append(vector)
    return basis, len(pivots), len(free_cols)


def vector_stats(vector: list[int]) -> dict[str, int]:
    return {
        "coeff_bound": max(abs(value) for value in vector) if vector else 0,
        "l1_weight": sum(abs(value) for value in vector),
        "support": sum(1 for value in vector if value),
        "euclidean_squared": sum(value * value for value in vector),
    }


def ceil_fraction(value: Fraction) -> int:
    return -((-value.numerator) // value.denominator)


def floor_fraction(value: Fraction) -> int:
    return value.numerator // value.denominator


def bareiss_det(matrix: list[list[int]]) -> int:
    n = len(matrix)
    work = [row[:] for row in matrix]
    sign = 1
    previous = 1
    for col in range(n - 1):
        pivot = next((row for row in range(col, n) if work[row][col] != 0), None)
        if pivot is None:
            return 0
        if pivot != col:
            work[col], work[pivot] = work[pivot], work[col]
            sign = -sign
        pivot_value = work[col][col]
        for row in range(col + 1, n):
            for inner_col in range(col + 1, n):
                work[row][inner_col] = (
                    work[row][inner_col] * pivot_value - work[row][col] * work[col][inner_col]
                ) // previous
        previous = pivot_value
        for row in range(col + 1, n):
            work[row][col] = 0
    return sign * work[n - 1][n - 1]


def solve_rational_square(matrix: list[list[int]], rhs: list[int]) -> list[Fraction] | None:
    n = len(matrix)
    work = [[Fraction(value) for value in row] + [Fraction(rhs[row_idx])] for row_idx, row in enumerate(matrix)]
    for col in range(n):
        pivot = next((row for row in range(col, n) if work[row][col] != 0), None)
        if pivot is None:
            return None
        if pivot != col:
            work[col], work[pivot] = work[pivot], work[col]
        pivot_value = work[col][col]
        for idx in range(col, n + 1):
            work[col][idx] /= pivot_value
        for row in range(n):
            if row == col or work[row][col] == 0:
                continue
            factor = work[row][col]
            for idx in range(col, n + 1):
                work[row][idx] -= factor * work[col][idx]
    return [work[row][n] for row in range(n)]


def row_lattice_membership_coordinates(basis_rows: list[list[int]], vector: list[int]) -> list[int] | None:
    transposed = [[basis_rows[row][col] for row in range(len(basis_rows))] for col in range(COLS)]
    solution = solve_rational_square(transposed, vector)
    if solution is None or any(value.denominator != 1 for value in solution):
        return None
    return [int(value) for value in solution]


def relation_row_holds(matrix: list[list[int]], coords: list[int], vector: list[int]) -> bool:
    for row in matrix:
        if sum(row[col] * vector[col] for col in range(COLS)) % Q != 0:
            return False
    return all(vector[coord] % Q == 0 for coord in coords)


def validate_reduced_basis(
    matrix: list[list[int]],
    coords: list[int],
    canonical_basis: list[list[int]],
    reduced_basis: list[list[int]],
) -> dict[str, object]:
    if len(reduced_basis) != COLS or any(len(row) != COLS for row in reduced_basis):
        return {"valid": False, "reason": "basis_shape"}
    if not all(isinstance(value, int) for row in reduced_basis for value in row):
        return {"valid": False, "reason": "basis_entries_not_int"}
    if not all(relation_row_holds(matrix, coords, row) for row in reduced_basis):
        return {"valid": False, "reason": "row_not_in_constraint_kernel"}

    canonical_det_abs = abs(bareiss_det(canonical_basis))
    reduced_det_abs = abs(bareiss_det(reduced_basis))
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
        if row_lattice_membership_coordinates(canonical_basis, row) is None:
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
        "canonical_basis_sha256": vector_list_sha256(b"Descender.response_link.canonical_basis.v1", canonical_basis),
        "reduced_basis_sha256": vector_list_sha256(b"Descender.response_link.reduced_basis.v1", reduced_basis),
    }


def run_lll(matrix: list[list[int]], coords: list[int], bound: int) -> dict[str, object]:
    from fpylll import GSO, LLL, IntegerMatrix

    basis, rank, kernel_dimension = integer_intersection_basis(matrix, coords)
    mat = IntegerMatrix.from_matrix(basis)
    LLL.reduction(mat, delta=0.99)
    candidates = [[int(mat[row, col]) for col in range(COLS)] for row in range(mat.nrows)]
    shortest = min(
        (vector for vector in candidates if any(vector)),
        key=lambda vector: sum(value * value for value in vector),
    )
    stats = vector_stats(shortest)
    stats.update(
        {
            "solver": "lll",
            "status": "evidence",
            "combined_constraint_rank": rank,
            "combined_kernel_dimension": kernel_dimension,
            "response_difference_bound": bound,
            "excludes_box_by_coeff_bound": stats["coeff_bound"] > bound,
            "shortest_vector_prefix": shortest[:8],
        }
    )
    return stats


def run_lattice_enum(
    matrix: list[list[int]],
    coords: list[int],
    bound: int,
    timeout: float,
) -> dict[str, object]:
    """Enumerate all short intersection-lattice vectors covering the box.

    Every vector in [-bound, bound]^COLS has Euclidean norm at most
    COLS * bound^2.  Therefore, enumerating the full intersection lattice up
    to that Euclidean radius and finding no vector with coefficient bound
    <= bound proves the bounded-box kernel check for this concrete matrix.
    """

    from fpylll import GSO, LLL, IntegerMatrix

    basis, rank, kernel_dimension = integer_intersection_basis(matrix, coords)
    mat = IntegerMatrix.from_matrix(basis)
    LLL.reduction(mat, delta=0.99)
    reduced_basis = [[int(mat[row, col]) for col in range(COLS)] for row in range(mat.nrows)]
    validation = validate_reduced_basis(matrix, coords, basis, reduced_basis)

    gso = GSO.Mat(mat)
    gso.update_gso()
    dimension = mat.nrows
    mu = [[0.0] * dimension for _ in range(dimension)]
    diagonal = [0.0] * dimension
    for row in range(dimension):
        diagonal[row] = gso.get_r(row, row)
        for col in range(row):
            mu[row][col] = gso.get_mu(row, col)

    coeffs = [0] * dimension
    radius_squared = COLS * bound * bound
    start = time.monotonic()
    stats: dict[str, object] = {
        "solver": "lattice-enum",
        "combined_constraint_rank": rank,
        "combined_kernel_dimension": kernel_dimension,
        "response_difference_bound": bound,
        "euclidean_cover_radius_squared": radius_squared,
        "canonical_basis_sha256": validation.get("canonical_basis_sha256"),
        "reduced_basis_sha256": vector_list_sha256(b"Descender.response_link.reduced_basis.v1", reduced_basis),
        "reduced_basis_lattice_validation": validation,
        "reduced_basis": reduced_basis,
        "cover_ball_vector_sha256": None,
        "lll_first_vector_stats": vector_stats(reduced_basis[0]),
        "visited_leaves": 0,
        "visited_nodes": 0,
        "nonzero_vectors_in_cover_ball": 0,
        "shortest_vector_in_cover_ball": None,
    }
    best_vector: list[int] | None = None
    best_norm: int | None = None
    witness: list[int] | None = None
    cover_hash = hashlib.sha256()
    cover_hash.update(b"Descender.response_link.cover_ball_vectors.v1")
    cover_hash.update(Q.to_bytes(4, "little"))
    cover_hash.update(bound.to_bytes(4, "little"))
    cover_hash.update(len(coords).to_bytes(2, "little"))
    for coord in coords:
        cover_hash.update(coord.to_bytes(2, "little"))

    def materialize_vector() -> list[int]:
        vector = [0] * COLS
        for row, coeff in enumerate(coeffs):
            if coeff == 0:
                continue
            basis_row = reduced_basis[row]
            for col in range(COLS):
                vector[col] += coeff * basis_row[col]
        return vector

    def recurse(row: int, partial_norm: float) -> None:
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
            stats["nonzero_vectors_in_cover_ball"] = int(stats["nonzero_vectors_in_cover_ball"]) + 1
            update_signed_vector_hash(cover_hash, vector)
            if best_norm is None or exact_norm < best_norm:
                best_norm = exact_norm
                best_vector = vector
            if max(abs(value) for value in vector) <= bound:
                witness = vector
                raise StopIteration
            return

        remaining = radius_squared - partial_norm
        # The small slack is intentional: it prevents floating GSO roundoff from
        # pruning a boundary vector.  Exact norm filtering happens at leaves.
        if remaining < -1e-7:
            return
        center_offset = sum(coeffs[idx] * mu[idx][row] for idx in range(row + 1, dimension))
        radius = math.sqrt(max(0.0, (remaining + 1e-7) / diagonal[row]))
        center = -center_offset
        lo = math.ceil(center - radius - 1e-9)
        hi = math.floor(center + radius + 1e-9)
        for coeff in range(lo, hi + 1):
            coeffs[row] = coeff
            term = (coeff + center_offset) * (coeff + center_offset) * diagonal[row]
            recurse(row - 1, partial_norm + term)
        coeffs[row] = 0

    try:
        recurse(dimension - 1, 0.0)
    except StopIteration:
        stats["status"] = "feasible"
        stats["witness"] = witness
        if witness is not None:
            stats.update(vector_stats(witness))
    except TimeoutError:
        stats["status"] = "timeout"
    else:
        stats["status"] = "infeasible"

    stats["wall_time_seconds"] = time.monotonic() - start
    stats["cover_ball_vector_sha256"] = cover_hash.hexdigest()
    if best_vector is not None:
        stats["shortest_vector_in_cover_ball"] = {
            **vector_stats(best_vector),
            "vector": best_vector,
            "prefix": best_vector[:8],
        }
    return stats


def run_exact_lattice_enum(
    matrix: list[list[int]],
    coords: list[int],
    bound: int,
    timeout: float,
) -> dict[str, object]:
    """Exact-rational replay of the bounded-box lattice enumeration.

    This uses fpylll only to obtain a reduced integer basis.  The enumeration
    bounds and pruning checks are then evaluated with Python `Fraction`
    arithmetic, and exact integer norms are checked at every leaf.
    """

    from fpylll import LLL, IntegerMatrix

    basis, rank, kernel_dimension = integer_intersection_basis(matrix, coords)
    mat = IntegerMatrix.from_matrix(basis)
    LLL.reduction(mat, delta=0.99)
    reduced_basis = [[int(mat[row, col]) for col in range(COLS)] for row in range(mat.nrows)]
    return run_exact_lattice_enum_from_basis(matrix, coords, bound, timeout, basis, reduced_basis, rank, kernel_dimension)


def run_exact_lattice_enum_from_basis(
    matrix: list[list[int]],
    coords: list[int],
    bound: int,
    timeout: float,
    canonical_basis: list[list[int]],
    reduced_basis: list[list[int]],
    rank: int,
    kernel_dimension: int,
) -> dict[str, object]:
    """Exact-rational bounded-box enumeration from an embedded reduced basis."""

    validation = validate_reduced_basis(matrix, coords, canonical_basis, reduced_basis)
    if not validation.get("valid", False):
        return {
            "solver": "exact-enum",
            "status": "invalid_reduced_basis",
            "reduced_basis_lattice_validation": validation,
        }

    dimension = len(reduced_basis)

    def dot(left: list[Fraction], right: list[Fraction]) -> Fraction:
        return sum((a * b for a, b in zip(left, right)), start=Fraction(0))

    basis_star: list[list[Fraction]] = []
    diagonal: list[Fraction] = []
    mu = [[Fraction(0) for _ in range(dimension)] for __ in range(dimension)]
    gs_start = time.monotonic()
    for row_idx, basis_row in enumerate(reduced_basis):
        rational_row = [Fraction(value) for value in basis_row]
        star_row = rational_row[:]
        for prev in range(row_idx):
            mu[row_idx][prev] = dot(rational_row, basis_star[prev]) / diagonal[prev]
            for col in range(COLS):
                star_row[col] -= mu[row_idx][prev] * basis_star[prev][col]
        basis_star.append(star_row)
        diagonal.append(dot(star_row, star_row))
    gs_seconds = time.monotonic() - gs_start

    coeffs = [0] * dimension
    radius_squared = COLS * bound * bound
    start = time.monotonic()
    stats: dict[str, object] = {
        "solver": "exact-enum",
        "combined_constraint_rank": rank,
        "combined_kernel_dimension": kernel_dimension,
        "response_difference_bound": bound,
        "euclidean_cover_radius_squared": radius_squared,
        "canonical_basis_sha256": validation.get("canonical_basis_sha256"),
        "reduced_basis_sha256": vector_list_sha256(b"Descender.response_link.reduced_basis.v1", reduced_basis),
        "reduced_basis_lattice_validation": validation,
        "reduced_basis": reduced_basis,
        "cover_ball_vector_sha256": None,
        "gram_schmidt_seconds": gs_seconds,
        "lll_first_vector_stats": vector_stats(reduced_basis[0]),
        "visited_leaves": 0,
        "visited_nodes": 0,
        "nonzero_vectors_in_cover_ball": 0,
        "shortest_vector_in_cover_ball": None,
    }
    best_norm: int | None = None
    best_vector: list[int] | None = None
    witness: list[int] | None = None
    cover_hash = hashlib.sha256()
    cover_hash.update(b"Descender.response_link.cover_ball_vectors.v1")
    cover_hash.update(Q.to_bytes(4, "little"))
    cover_hash.update(bound.to_bytes(4, "little"))
    cover_hash.update(len(coords).to_bytes(2, "little"))
    for coord in coords:
        cover_hash.update(coord.to_bytes(2, "little"))

    def materialize_vector() -> list[int]:
        vector = [0] * COLS
        for row, coeff in enumerate(coeffs):
            if coeff == 0:
                continue
            basis_row = reduced_basis[row]
            for col in range(COLS):
                vector[col] += coeff * basis_row[col]
        return vector

    def exact_interval(offset: Fraction, diag: Fraction, remaining: Fraction) -> tuple[int, int]:
        cover = remaining / diag
        sqrt_upper = math.isqrt((cover.numerator + cover.denominator - 1) // cover.denominator) + 2
        center = -offset
        lo = ceil_fraction(center - sqrt_upper)
        hi = floor_fraction(center + sqrt_upper)
        while lo <= hi and (Fraction(lo) + offset) * (Fraction(lo) + offset) * diag > remaining:
            lo += 1
        while lo <= hi and (Fraction(hi) + offset) * (Fraction(hi) + offset) * diag > remaining:
            hi -= 1
        return lo, hi

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
            stats["nonzero_vectors_in_cover_ball"] = int(stats["nonzero_vectors_in_cover_ball"]) + 1
            update_signed_vector_hash(cover_hash, vector)
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
        offset = sum((Fraction(coeffs[idx]) * mu[idx][row] for idx in range(row + 1, dimension)), start=Fraction(0))
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
            stats.update(vector_stats(witness))
    except TimeoutError:
        stats["status"] = "timeout"
    else:
        stats["status"] = "infeasible"

    stats["wall_time_seconds"] = time.monotonic() - start
    stats["cover_ball_vector_sha256"] = cover_hash.hexdigest()
    if best_vector is not None:
        stats["shortest_vector_in_cover_ball"] = {
            **vector_stats(best_vector),
            "vector": best_vector,
            "prefix": best_vector[:8],
        }
    return stats


SplitSign = Literal[-1, 1]
SplitCase = tuple[int, SplitSign]


def split_candidate_indices(coords: list[int]) -> list[int]:
    coord_set = set(coords)
    return [idx for idx in range(COLS) if idx not in coord_set]


def add_cp_sat_nonzero_constraint(
    model: object,
    x: list[object],
    coords: list[int],
    bound: int,
    split_case: SplitCase | None,
) -> None:
    from ortools.sat.python import cp_model

    if split_case is not None:
        coord, sign = split_case
        for earlier in split_candidate_indices(coords):
            if earlier == coord:
                break
            model.Add(x[earlier] == 0)
        model.Add(x[coord] >= 1 if sign > 0 else x[coord] <= -1)
        return

    selected = []
    big_m = bound + 1
    for idx in range(COLS):
        pos = model.NewBoolVar(f"pos{idx}")
        neg = model.NewBoolVar(f"neg{idx}")
        model.Add(pos + neg <= 1)
        model.Add(x[idx] >= 1 - big_m * (1 - pos))
        model.Add(x[idx] <= -1 + big_m * (1 - neg))
        selected.extend([pos, neg])
    model.Add(sum(selected) >= 1)


def run_cp_sat(
    matrix: list[list[int]],
    coords: list[int],
    bound: int,
    timeout: float,
    split_case: SplitCase | None = None,
) -> dict[str, object]:
    from ortools.sat.python import cp_model

    model = cp_model.CpModel()
    x = [model.NewIntVar(-bound, bound, f"x{idx}") for idx in range(COLS)]
    for coord in coords:
        model.Add(x[coord] == 0)

    add_cp_sat_nonzero_constraint(model, x, coords, bound, split_case)

    for row_idx, row in enumerate(matrix):
        k_bound = math.ceil(sum(abs(value) * bound for value in row) / Q)
        k = model.NewIntVar(-k_bound, k_bound, f"k{row_idx}")
        model.Add(sum(row[col] * x[col] for col in range(COLS)) == Q * k)

    solver = cp_model.CpSolver()
    solver.parameters.max_time_in_seconds = timeout
    solver.parameters.num_search_workers = 8
    status = solver.Solve(model)
    name = solver.StatusName(status)
    result: dict[str, object] = {
        "solver": "cp-sat",
        "status": name.lower(),
        "wall_time_seconds": solver.WallTime(),
        "conflicts": solver.NumConflicts(),
        "branches": solver.NumBranches(),
    }
    if split_case is not None:
        result["split_case"] = {"first_nonzero_coord": split_case[0], "sign": split_case[1]}
    if status in (cp_model.FEASIBLE, cp_model.OPTIMAL):
        vector = [solver.Value(value) for value in x]
        result["witness"] = vector
        result.update(vector_stats(vector))
    return result


def add_scip_nonzero_constraint(
    solver: object,
    x: list[object],
    coords: list[int],
    bound: int,
    split_case: SplitCase | None,
) -> None:
    if split_case is not None:
        coord, sign = split_case
        for earlier in split_candidate_indices(coords):
            if earlier == coord:
                break
            solver.Add(x[earlier] == 0)
        solver.Add(x[coord] >= 1 if sign > 0 else x[coord] <= -1)
        return

    selected = []
    big_m = bound + 1
    for idx in range(COLS):
        pos = solver.IntVar(0, 1, f"pos{idx}")
        neg = solver.IntVar(0, 1, f"neg{idx}")
        solver.Add(pos + neg <= 1)
        solver.Add(x[idx] >= 1 - big_m * (1 - pos))
        solver.Add(x[idx] <= -1 + big_m * (1 - neg))
        selected.extend([pos, neg])
    solver.Add(sum(selected) >= 1)


def run_scip(
    matrix: list[list[int]],
    coords: list[int],
    bound: int,
    timeout: float,
    split_case: SplitCase | None = None,
) -> dict[str, object]:
    from ortools.linear_solver import pywraplp

    solver = pywraplp.Solver.CreateSolver("SCIP")
    if solver is None:
        return {"solver": "scip", "status": "unavailable"}
    solver.SetTimeLimit(int(timeout * 1000))
    x = [solver.IntVar(-bound, bound, f"x{idx}") for idx in range(COLS)]
    for coord in coords:
        solver.Add(x[coord] == 0)

    add_scip_nonzero_constraint(solver, x, coords, bound, split_case)

    for row_idx, row in enumerate(matrix):
        k_bound = math.ceil(sum(abs(value) * bound for value in row) / Q)
        k = solver.IntVar(-k_bound, k_bound, f"k{row_idx}")
        solver.Add(sum(row[col] * x[col] for col in range(COLS)) == Q * k)

    status = solver.Solve()
    status_names = {
        pywraplp.Solver.OPTIMAL: "optimal",
        pywraplp.Solver.FEASIBLE: "feasible",
        pywraplp.Solver.INFEASIBLE: "infeasible",
        pywraplp.Solver.UNBOUNDED: "unbounded",
        pywraplp.Solver.ABNORMAL: "abnormal",
        pywraplp.Solver.NOT_SOLVED: "not_solved",
    }
    result: dict[str, object] = {
        "solver": "scip",
        "status": status_names.get(status, f"unknown_{status}"),
        "nodes": solver.nodes(),
    }
    if split_case is not None:
        result["split_case"] = {"first_nonzero_coord": split_case[0], "sign": split_case[1]}
    if status in (pywraplp.Solver.FEASIBLE, pywraplp.Solver.OPTIMAL):
        vector = [int(round(value.solution_value())) for value in x]
        result["witness"] = vector
        result.update(vector_stats(vector))
    return result


def split_cases(coords: list[int], start_case: int, max_cases: int) -> list[SplitCase]:
    cases = [(idx, sign) for idx in split_candidate_indices(coords) for sign in (1, -1)]
    if start_case:
        cases = cases[start_case:]
    if max_cases:
        cases = cases[:max_cases]
    return cases


def run_split_search(
    matrix: list[list[int]],
    coords: list[int],
    bound: int,
    timeout: float,
    backend: Literal["cp-sat", "scip"],
    start_case: int,
    max_cases: int,
) -> dict[str, object]:
    cases = split_cases(coords, start_case, max_cases)
    case_results = []
    status_counts: dict[str, int] = {}
    for case_offset, split_case in enumerate(cases, start=start_case):
        if backend == "cp-sat":
            result = run_cp_sat(matrix, coords, bound, timeout, split_case=split_case)
        else:
            result = run_scip(matrix, coords, bound, timeout, split_case=split_case)
        result["case_index"] = case_offset
        case_results.append(result)
        status = str(result["status"])
        status_counts[status] = status_counts.get(status, 0) + 1
        if "witness" in result:
            return {
                "solver": f"{backend}-split",
                "status": "feasible",
                "case_count": len(cases),
                "checked_case_count": len(case_results),
                "status_counts": status_counts,
                "witness_case": result,
                "cases": case_results,
            }

    exact_unsat_statuses = {"infeasible"}
    if backend == "scip":
        exact_unsat_statuses = {"infeasible"}
    if case_results and all(str(result["status"]) in exact_unsat_statuses for result in case_results):
        status = "infeasible"
    else:
        status = "unknown"
    return {
        "solver": f"{backend}-split",
        "status": status,
        "case_count": len(cases),
        "checked_case_count": len(case_results),
        "status_counts": status_counts,
        "cases": case_results,
    }


def emit_smt2(matrix: list[list[int]], coords: list[int], bound: int, path: Path) -> None:
    lines = ["(set-logic QF_LIA)"]
    for idx in range(COLS):
        lines.append(f"(declare-const x{idx} Int)")
    for row_idx in range(ROWS):
        lines.append(f"(declare-const k{row_idx} Int)")
    for idx in range(COLS):
        lines.append(f"(assert (and (>= x{idx} {-bound}) (<= x{idx} {bound})))")
    for coord in coords:
        lines.append(f"(assert (= x{coord} 0))")
    lines.append("(assert (or " + " ".join(f"(not (= x{idx} 0))" for idx in range(COLS)) + "))")
    for row_idx, row in enumerate(matrix):
        terms = " ".join(f"(* {value} x{col})" for col, value in enumerate(row) if value)
        lines.append(f"(assert (= (+ {terms}) (* {Q} k{row_idx})))")
    lines.append("(check-sat)")
    path.write_text("\n".join(lines) + "\n")


CERTIFICATE_FIELDS = [
    "q",
    "matrix_domain",
    "matrix_transcript_sha256",
    "coords",
    "bound",
    "solver",
    "status",
    "combined_constraint_rank",
    "combined_kernel_dimension",
    "response_difference_bound",
    "euclidean_cover_radius_squared",
    "canonical_basis_sha256",
    "reduced_basis_sha256",
    "reduced_basis_lattice_validation",
    "reduced_basis",
    "cover_ball_vector_sha256",
    "visited_leaves",
    "visited_nodes",
    "nonzero_vectors_in_cover_ball",
    "shortest_vector_in_cover_ball",
]


def verify_certificate(path: Path, timeout: float) -> dict[str, object]:
    expected = json.loads(path.read_text())
    solver = expected.get("solver")
    if solver not in {"lattice-enum", "exact-enum"}:
        return {
            "solver": "verify-json",
            "status": "unsupported_solver",
            "certificate_path": str(path),
            "certificate_solver": solver,
        }

    coords = [int(value) for value in expected["coords"]]
    bound = int(expected["bound"])
    matrix = full_response_matrix()
    canonical_basis, rank, kernel_dimension = integer_intersection_basis(matrix, coords)
    actual: dict[str, object] = {
        "q": Q,
        "matrix_domain": DOMAIN.decode(),
        "matrix_transcript_sha256": matrix_transcript_sha256(matrix),
        "coords": coords,
        "bound": bound,
    }
    if solver == "exact-enum" and "reduced_basis" in expected:
        reduced_basis = [[int(value) for value in row] for row in expected["reduced_basis"]]
        actual.update(
            run_exact_lattice_enum_from_basis(
                matrix,
                coords,
                bound,
                timeout,
                canonical_basis,
                reduced_basis,
                rank,
                kernel_dimension,
            )
        )
    elif solver == "exact-enum":
        actual.update(run_exact_lattice_enum(matrix, coords, bound, timeout))
    else:
        actual.update(run_lattice_enum(matrix, coords, bound, timeout))

    mismatches = []
    for field in CERTIFICATE_FIELDS:
        if expected.get(field) != actual.get(field):
            mismatches.append(
                {
                    "field": field,
                    "expected": expected.get(field),
                    "actual": actual.get(field),
                }
            )

    return {
        "solver": "verify-json",
        "status": "verified" if not mismatches else "mismatch",
        "certificate_path": str(path),
        "certificate_solver": solver,
        "checked_fields": CERTIFICATE_FIELDS,
        "mismatches": mismatches,
        "replay_wall_time_seconds": actual.get("wall_time_seconds"),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--coords", default="0,3")
    parser.add_argument("--bound", type=int, default=96)
    parser.add_argument(
        "--solver",
        choices=[
            "lll",
            "lattice-enum",
            "exact-enum",
            "cp-sat",
            "scip",
            "z3",
            "cp-sat-split",
            "scip-split",
        ],
        default="lll",
    )
    parser.add_argument("--timeout", type=float, default=30.0)
    parser.add_argument(
        "--split-start-case",
        type=int,
        default=0,
        help="first split case to run for split solvers; cases are ordered by first nonzero coordinate, then sign",
    )
    parser.add_argument(
        "--split-max-cases",
        type=int,
        default=0,
        help="maximum split cases to run; 0 means all remaining cases",
    )
    parser.add_argument("--emit-smt2", type=Path)
    parser.add_argument("--write-json", type=Path)
    parser.add_argument("--verify-json", type=Path)
    args = parser.parse_args()

    if args.verify_json is not None:
        result = verify_certificate(args.verify_json, args.timeout)
        print(json.dumps(result, indent=2, sort_keys=True))
        return

    coords = [int(part) for part in args.coords.split(",") if part]
    matrix = full_response_matrix()
    result: dict[str, object] = {
        "q": Q,
        "matrix_domain": DOMAIN.decode(),
        "matrix_transcript_sha256": matrix_transcript_sha256(matrix),
        "coords": coords,
        "bound": args.bound,
    }

    if args.emit_smt2 is not None:
        emit_smt2(matrix, coords, args.bound, args.emit_smt2)
        result["smt2_path"] = str(args.emit_smt2)

    if args.solver == "lll":
        result.update(run_lll(matrix, coords, args.bound))
    elif args.solver == "lattice-enum":
        result.update(run_lattice_enum(matrix, coords, args.bound, args.timeout))
    elif args.solver == "exact-enum":
        result.update(run_exact_lattice_enum(matrix, coords, args.bound, args.timeout))
    elif args.solver == "cp-sat":
        result.update(run_cp_sat(matrix, coords, args.bound, args.timeout))
    elif args.solver == "scip":
        result.update(run_scip(matrix, coords, args.bound, args.timeout))
    elif args.solver == "cp-sat-split":
        result.update(
            run_split_search(
                matrix,
                coords,
                args.bound,
                args.timeout,
                "cp-sat",
                args.split_start_case,
                args.split_max_cases,
            )
        )
    elif args.solver == "scip-split":
        result.update(
            run_split_search(
                matrix,
                coords,
                args.bound,
                args.timeout,
                "scip",
                args.split_start_case,
                args.split_max_cases,
            )
        )
    else:
        if shutil.which("z3") is None:
            result.update({"solver": "z3", "status": "unavailable"})
        else:
            with tempfile.NamedTemporaryFile("w", suffix=".smt2", delete=False) as tmp:
                smt_path = Path(tmp.name)
            emit_smt2(matrix, coords, args.bound, smt_path)
            proc = subprocess.run(
                ["z3", f"-T:{int(args.timeout)}", str(smt_path)],
                check=False,
                text=True,
                capture_output=True,
            )
            smt_path.unlink(missing_ok=True)
            result.update(
                {
                    "solver": "z3",
                    "status": proc.stdout.strip().splitlines()[0] if proc.stdout.strip() else "no_output",
                    "returncode": proc.returncode,
                }
            )

    if args.write_json is not None:
        args.write_json.parent.mkdir(parents=True, exist_ok=True)
        args.write_json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
        result["json_path"] = str(args.write_json)

    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
