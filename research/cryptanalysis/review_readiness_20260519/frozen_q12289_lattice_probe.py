#!/usr/bin/env python3

import json
import math
import sys
from pathlib import Path

from fpylll import BKZ, IntegerMatrix, LLL


def centered_value(x: int, q: int) -> int:
    r = x % q
    if r > q // 2:
        return r - q
    return r


def build_systematic_matrix(coeffs: list[list[int]], group_count: int) -> list[list[int]]:
    rows_per_group = len(coeffs)
    out = [[0 for _ in range(group_count * 2)] for _ in range(group_count * rows_per_group)]
    for g in range(group_count):
        for r, (a, b) in enumerate(coeffs):
            out[g * rows_per_group + r][g * 2] = a
            out[g * rows_per_group + r][g * 2 + 1] = b
    return out


def build_qary_basis(systematic: list[list[int]], q: int) -> list[list[int]]:
    pivot_dim = len(systematic)
    free_dim = len(systematic[0]) if systematic else 0
    total = pivot_dim + free_dim
    basis = []
    for j in range(free_dim):
        vec = [0] * total
        for i in range(pivot_dim):
            vec[i] = -systematic[i][j]
        vec[pivot_dim + j] = 1
        basis.append(vec)
    for i in range(pivot_dim):
        vec = [0] * total
        vec[i] = q
        basis.append(vec)
    return basis


def relation_holds(systematic: list[list[int]], vec: list[int], q: int) -> bool:
    pivot_dim = len(systematic)
    free_dim = len(systematic[0]) if systematic else 0
    pivot = vec[:pivot_dim]
    free = vec[pivot_dim:pivot_dim + free_dim]
    for i in range(pivot_dim):
        acc = pivot[i]
        for j in range(free_dim):
            acc += systematic[i][j] * free[j]
        if acc % q != 0:
            return False
    return True


def centered_l1(vec: list[int], q: int) -> int:
    return sum(abs(centered_value(v, q)) for v in vec)


def centered_support(vec: list[int], q: int) -> int:
    return sum(1 for v in vec if centered_value(v, q) != 0)


def main() -> int:
    if len(sys.argv) != 5:
        print(
            "usage: frozen_q12289_lattice_probe.py sidecar.json bkz_block_size output.json output.md",
            file=sys.stderr,
        )
        return 2
    sidecar = json.loads(Path(sys.argv[1]).read_text())
    bkz_block = int(sys.argv[2])
    output_json = Path(sys.argv[3])
    output_md = Path(sys.argv[4])

    q = sidecar["q"]
    group_count = sidecar["group_count"]
    coeffs = sidecar["pair_block"]["coefficients"]
    systematic = build_systematic_matrix(coeffs, group_count)
    basis_rows = build_qary_basis(systematic, q)
    mat = IntegerMatrix.from_matrix(basis_rows)
    LLL.reduction(mat)
    BKZ.reduction(mat, BKZ.Param(block_size=bkz_block))
    first = [int(mat[0, j]) for j in range(mat.ncols)]

    report = {
        "artifact": "frozen-q12289-lattice-probe",
        "input_sidecar": sys.argv[1],
        "q": q,
        "group_count": group_count,
        "rows_per_group": len(coeffs),
        "basis_dimension": mat.nrows,
        "ambient_dimension": mat.ncols,
        "bkz_block_size": bkz_block,
        "first_vector": first,
        "first_vector_euclidean_norm": math.sqrt(sum(v * v for v in first)),
        "first_vector_squared_norm": sum(v * v for v in first),
        "first_vector_centered_l1": centered_l1(first, q),
        "first_vector_centered_support": centered_support(first, q),
        "first_vector_relation_holds_mod_q": relation_holds(systematic, first, q),
    }

    lines = [
        "# Frozen q12289 Lattice Probe",
        "",
        f"- q: `{q}`",
        f"- group count: `{group_count}`",
        f"- rows per group: `{len(coeffs)}`",
        f"- basis dimension: `{mat.nrows}`",
        f"- ambient dimension: `{mat.ncols}`",
        f"- BKZ block size: `{bkz_block}`",
        f"- first vector squared norm: `{report['first_vector_squared_norm']}`",
        f"- first vector centered l1: `{report['first_vector_centered_l1']}`",
        f"- first vector centered support: `{report['first_vector_centered_support']}`",
        f"- first vector satisfies relation mod q: `{report['first_vector_relation_holds_mod_q']}`",
        "",
        "```text",
        str(first),
        "```",
        "",
    ]
    output_json.write_text(json.dumps(report, indent=2) + "\n")
    output_md.write_text("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
