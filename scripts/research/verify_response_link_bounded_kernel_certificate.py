#!/usr/bin/env python3
"""Standalone verifier for the response-link bounded-kernel certificate.

This verifier intentionally uses only Python's standard library.  It reads a
JSON certificate containing an embedded reduced basis, reconstructs the
canonical q=12289 response-link matrix and intersection lattice, proves the
embedded basis spans the same integer lattice, then replays the exact bounded
Euclidean-cover enumeration.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import platform
import sys
import time
from fractions import Fraction
from pathlib import Path

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


def update_signed_vector_hash(h: "hashlib._Hash", vector: list[int]) -> None:
    h.update(len(vector).to_bytes(2, "little"))
    for value in vector:
        h.update(value.to_bytes(8, "little", signed=True))


def vector_list_sha256(domain: bytes, vectors: list[list[int]]) -> str:
    h = hashlib.sha256()
    h.update(domain)
    h.update(len(vectors).to_bytes(4, "little"))
    for vector in vectors:
        update_signed_vector_hash(h, vector)
    return h.hexdigest()


def matrix_transcript_sha256(matrix: list[list[int]]) -> str:
    h = hashlib.sha256()
    for row in matrix:
        for value in row:
            h.update(value.to_bytes(2, "little"))
    return h.hexdigest()


def file_sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def canonical_json_sha256(payload: object) -> str:
    encoded = json.dumps(payload, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(encoded).hexdigest()


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


def canonical_intersection_basis(matrix: list[list[int]], coords: list[int]) -> tuple[list[list[int]], int, int]:
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


def row_lattice_membership(basis_rows: list[list[int]], vector: list[int]) -> bool:
    transposed = [[basis_rows[row][col] for row in range(len(basis_rows))] for col in range(COLS)]
    solution = solve_rational_square(transposed, vector)
    return solution is not None and all(value.denominator == 1 for value in solution)


def row_satisfies_relation(matrix: list[list[int]], coords: list[int], vector: list[int]) -> bool:
    return all(sum(row[col] * vector[col] for col in range(COLS)) % Q == 0 for row in matrix) and all(
        vector[coord] % Q == 0 for coord in coords
    )


def vector_stats(vector: list[int]) -> dict[str, int]:
    return {
        "coeff_bound": max(abs(value) for value in vector) if vector else 0,
        "euclidean_squared": sum(value * value for value in vector),
        "l1_weight": sum(abs(value) for value in vector),
        "support": sum(1 for value in vector if value),
    }


def ceil_fraction(value: Fraction) -> int:
    return -((-value.numerator) // value.denominator)


def floor_fraction(value: Fraction) -> int:
    return value.numerator // value.denominator


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


def verify_embedded_basis(matrix: list[list[int]], coords: list[int], canonical_basis: list[list[int]], reduced_basis: list[list[int]]) -> dict[str, object]:
    if len(reduced_basis) != COLS or any(len(row) != COLS for row in reduced_basis):
        return {"valid": False, "reason": "shape"}
    if not all(row_satisfies_relation(matrix, coords, row) for row in reduced_basis):
        return {"valid": False, "reason": "relation"}
    canonical_det_abs = abs(bareiss_det(canonical_basis))
    reduced_det_abs = abs(bareiss_det(reduced_basis))
    if canonical_det_abs == 0 or canonical_det_abs != reduced_det_abs:
        return {
            "valid": False,
            "reason": "determinant",
            "canonical_det_abs": canonical_det_abs,
            "reduced_det_abs": reduced_det_abs,
        }
    for row_idx, row in enumerate(reduced_basis):
        if not row_lattice_membership(canonical_basis, row):
            return {"valid": False, "reason": "membership", "row": row_idx}
    return {
        "valid": True,
        "canonical_det_abs": canonical_det_abs,
        "reduced_det_abs": reduced_det_abs,
        "canonical_basis_sha256": vector_list_sha256(b"Descender.response_link.canonical_basis.v1", canonical_basis),
        "reduced_basis_sha256": vector_list_sha256(b"Descender.response_link.reduced_basis.v1", reduced_basis),
    }


def exact_enumeration(reduced_basis: list[list[int]], coords: list[int], bound: int, timeout: float) -> dict[str, object]:
    dimension = len(reduced_basis)

    def dot(left: list[Fraction], right: list[Fraction]) -> Fraction:
        return sum((a * b for a, b in zip(left, right)), start=Fraction(0))

    basis_star: list[list[Fraction]] = []
    diagonal: list[Fraction] = []
    mu = [[Fraction(0) for _ in range(dimension)] for __ in range(dimension)]
    for row_idx, basis_row in enumerate(reduced_basis):
        rational_row = [Fraction(value) for value in basis_row]
        star_row = rational_row[:]
        for prev in range(row_idx):
            mu[row_idx][prev] = dot(rational_row, basis_star[prev]) / diagonal[prev]
            for col in range(COLS):
                star_row[col] -= mu[row_idx][prev] * basis_star[prev][col]
        basis_star.append(star_row)
        diagonal.append(dot(star_row, star_row))

    coeffs = [0] * dimension
    radius_squared = COLS * bound * bound
    cover_hash = hashlib.sha256()
    cover_hash.update(b"Descender.response_link.cover_ball_vectors.v1")
    cover_hash.update(Q.to_bytes(4, "little"))
    cover_hash.update(bound.to_bytes(4, "little"))
    cover_hash.update(len(coords).to_bytes(2, "little"))
    for coord in coords:
        cover_hash.update(coord.to_bytes(2, "little"))

    start = time.monotonic()
    visited_nodes = 0
    visited_leaves = 0
    nonzero_vectors = 0
    best_vector: list[int] | None = None
    best_norm: int | None = None

    def materialize() -> list[int]:
        vector = [0] * COLS
        for row, coeff in enumerate(coeffs):
            if coeff == 0:
                continue
            for col in range(COLS):
                vector[col] += coeff * reduced_basis[row][col]
        return vector

    def recurse(row: int, partial_norm: Fraction) -> None:
        nonlocal visited_nodes, visited_leaves, nonzero_vectors, best_vector, best_norm
        if timeout > 0 and time.monotonic() - start > timeout:
            raise TimeoutError
        visited_nodes += 1
        if row < 0:
            visited_leaves += 1
            if all(coeff == 0 for coeff in coeffs):
                return
            vector = materialize()
            exact_norm = sum(value * value for value in vector)
            if exact_norm > radius_squared:
                return
            nonzero_vectors += 1
            update_signed_vector_hash(cover_hash, vector)
            if best_norm is None or exact_norm < best_norm:
                best_norm = exact_norm
                best_vector = vector
            if max(abs(value) for value in vector) <= bound:
                raise ValueError("bounded nonzero kernel vector found")
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

    recurse(dimension - 1, Fraction(0))
    return {
        "status": "infeasible",
        "euclidean_cover_radius_squared": radius_squared,
        "visited_nodes": visited_nodes,
        "visited_leaves": visited_leaves,
        "nonzero_vectors_in_cover_ball": nonzero_vectors,
        "cover_ball_vector_sha256": cover_hash.hexdigest(),
        "shortest_vector_in_cover_ball": {
            **vector_stats(best_vector or []),
            "vector": best_vector,
            "prefix": (best_vector or [])[:8],
        },
        "wall_time_seconds": time.monotonic() - start,
    }


def build_audit_manifest(
    certificate_path: Path,
    certificate: dict[str, object],
    validation: dict[str, object],
    enumeration: dict[str, object],
    mismatches: list[dict[str, object]],
) -> dict[str, object]:
    """Build a compact audit manifest for a successful exact replay.

    The manifest is deliberately a summary, not a replacement for replaying the
    certificate.  It binds the input certificate, this verifier script, the
    canonical matrix transcript, the lattice-basis hashes, and the exact
    enumeration transcript hash into one small JSON object reviewers can cite.
    """

    verifier_path = Path(__file__).resolve()
    statement = {
        "claim": "No nonzero response-link kernel vector exists in the accepted response-difference box.",
        "q": Q,
        "matrix_rows": ROWS,
        "matrix_cols": COLS,
        "matrix_domain": DOMAIN.decode(),
        "coords_fixed_mod_q_to_zero": certificate.get("coords"),
        "box": {
            "type": "l_infinity",
            "lower": -int(certificate["bound"]),
            "upper": int(certificate["bound"]),
            "dimension": COLS,
        },
        "euclidean_cover_radius_squared": enumeration["euclidean_cover_radius_squared"],
    }
    replay = {
        "status": "verified" if validation.get("valid") and not mismatches else "mismatch",
        "basis_validation": validation,
        "visited_nodes": enumeration["visited_nodes"],
        "visited_leaves": enumeration["visited_leaves"],
        "nonzero_vectors_in_cover_ball": enumeration["nonzero_vectors_in_cover_ball"],
        "cover_ball_vector_sha256": enumeration["cover_ball_vector_sha256"],
        "shortest_vector_in_cover_ball": enumeration["shortest_vector_in_cover_ball"],
        "mismatches": mismatches,
        "replay_wall_time_seconds": enumeration["wall_time_seconds"],
    }
    manifest = {
        "schema": "Descender.response_link.bounded_kernel.audit_manifest.v1",
        "statement": statement,
        "artifacts": {
            "certificate_path": str(certificate_path),
            "certificate_file_sha256": file_sha256(certificate_path),
            "certificate_canonical_json_sha256": canonical_json_sha256(certificate),
            "verifier_path": str(verifier_path),
            "verifier_file_sha256": file_sha256(verifier_path),
        },
        "replay": replay,
        "replay_command": [
            sys.executable,
            str(verifier_path),
            str(certificate_path),
            "--timeout",
            "240",
        ],
        "environment": {
            "python_version": sys.version,
            "python_implementation": platform.python_implementation(),
            "platform": platform.platform(),
        },
    }
    manifest["manifest_payload_sha256"] = canonical_json_sha256(
        {key: value for key, value in manifest.items() if key != "manifest_payload_sha256"}
    )
    return manifest


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("certificate", type=Path)
    parser.add_argument("--timeout", type=float, default=240.0)
    parser.add_argument(
        "--write-manifest",
        type=Path,
        help="write an audit manifest after replaying and validating the certificate",
    )
    args = parser.parse_args()

    cert = json.loads(args.certificate.read_text())
    coords = [int(value) for value in cert["coords"]]
    bound = int(cert["bound"])
    reduced_basis = [[int(value) for value in row] for row in cert["reduced_basis"]]
    matrix = full_response_matrix()
    canonical_basis, rank, kernel_dimension = canonical_intersection_basis(matrix, coords)
    validation = verify_embedded_basis(matrix, coords, canonical_basis, reduced_basis)
    enumeration = exact_enumeration(reduced_basis, coords, bound, args.timeout)

    expected = {
        "matrix_transcript_sha256": matrix_transcript_sha256(matrix),
        "combined_constraint_rank": rank,
        "combined_kernel_dimension": kernel_dimension,
        "canonical_basis_sha256": validation.get("canonical_basis_sha256"),
        "reduced_basis_sha256": validation.get("reduced_basis_sha256"),
        "reduced_basis_lattice_validation": validation,
        "status": enumeration["status"],
        "euclidean_cover_radius_squared": enumeration["euclidean_cover_radius_squared"],
        "visited_nodes": enumeration["visited_nodes"],
        "visited_leaves": enumeration["visited_leaves"],
        "nonzero_vectors_in_cover_ball": enumeration["nonzero_vectors_in_cover_ball"],
        "cover_ball_vector_sha256": enumeration["cover_ball_vector_sha256"],
        "shortest_vector_in_cover_ball": enumeration["shortest_vector_in_cover_ball"],
    }
    mismatches = [
        {"field": key, "expected": cert.get(key), "actual": actual}
        for key, actual in expected.items()
        if cert.get(key) != actual
    ]
    result = {
        "solver": "standalone-verify",
        "status": "verified" if validation.get("valid") and not mismatches else "mismatch",
        "certificate_path": str(args.certificate),
        "mismatches": mismatches,
        "replay_wall_time_seconds": enumeration["wall_time_seconds"],
    }
    if args.write_manifest is not None:
        manifest = build_audit_manifest(
            args.certificate, cert, validation, enumeration, mismatches
        )
        args.write_manifest.parent.mkdir(parents=True, exist_ok=True)
        args.write_manifest.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
        result["audit_manifest_path"] = str(args.write_manifest)
        result["audit_manifest_payload_sha256"] = manifest["manifest_payload_sha256"]
    print(json.dumps(result, indent=2, sort_keys=True))
    if result["status"] != "verified":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
