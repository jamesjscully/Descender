#!/usr/bin/env python3
"""Structural diagnostics for frozen Descender candidate matrices.

This consumes a cryptanalysis dossier emitted by `descender_cryptanalysis.py`
and compares the full regenerated adjoint matrix with the actually selected
public view prefix.  The distinction matters for reduced-view candidates: a
short q-ary dual relation using hidden coordinates is not directly available as
a public-sample distinguisher.
"""

from __future__ import annotations

import argparse
import json
import math
from dataclasses import dataclass
from pathlib import Path
from typing import Any


MASK64 = (1 << 64) - 1


class Rng64:
    def __init__(self, seed: int):
        self.state = seed | 1

    def next_u64(self) -> int:
        x = self.state
        x ^= x >> 12
        x ^= (x << 25) & MASK64
        x ^= x >> 27
        self.state = x & MASK64
        return (self.state * 0x2545F4914F6CDD1D) & MASK64

    def modulo(self, modulus: int) -> int:
        return self.next_u64() % modulus


def load_json(path: Path) -> Any:
    return json.loads(path.read_text())


def centered_abs(x: int, q: int) -> int:
    r = x % q
    return min(r, q - r)


def centered_signed(x: int, q: int) -> int:
    r = x % q
    return r - q if r > q // 2 else r


def signed_to_mod(x: int, q: int) -> int:
    return x % q


def inv_mod(x: int, q: int) -> int:
    return pow(x % q, q - 2, q)


def rank_mod(matrix: list[list[int]], q: int) -> int:
    if not matrix or not matrix[0]:
        return 0
    rows = [row[:] for row in matrix]
    n_rows = len(rows)
    n_cols = len(rows[0])
    rank = 0
    for col in range(n_cols):
        pivot = next((r for r in range(rank, n_rows) if rows[r][col] % q), None)
        if pivot is None:
            continue
        rows[rank], rows[pivot] = rows[pivot], rows[rank]
        inv = inv_mod(rows[rank][col], q)
        rows[rank] = [(v * inv) % q for v in rows[rank]]
        for r in range(n_rows):
            if r == rank:
                continue
            factor = rows[r][col] % q
            if factor:
                rows[r] = [(v - factor * rows[rank][j]) % q for j, v in enumerate(rows[r])]
        rank += 1
        if rank == n_rows:
            break
    return rank


def prefix_rank_profile(matrix: list[list[int]], q: int) -> dict[str, Any]:
    n = len(matrix)
    m = len(matrix[0]) if matrix else 0
    samples = sorted(set([0, 1, 2, 4, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 96, 112, m]))
    samples = [x for x in samples if 0 <= x <= m]
    ranks = []
    first_full = None
    for width in range(1, m + 1):
        rank = rank_mod([row[:width] for row in matrix], q)
        if rank == n:
            first_full = width
            break
    for width in samples:
        ranks.append({"prefix": width, "rank": rank_mod([row[:width] for row in matrix], q)})
    return {"first_full_rank_prefix": first_full, "sampled_prefix_ranks": ranks}


def invert_square_mod(matrix: list[list[int]], q: int) -> list[list[int]]:
    n = len(matrix)
    aug = [row[:] + [1 if i == j else 0 for j in range(n)] for i, row in enumerate(matrix)]
    for col in range(n):
        pivot = next((r for r in range(col, n) if aug[r][col] % q), None)
        if pivot is None:
            raise ValueError("singular pivot block")
        aug[col], aug[pivot] = aug[pivot], aug[col]
        inv = inv_mod(aug[col][col], q)
        aug[col] = [(v * inv) % q for v in aug[col]]
        for r in range(n):
            if r == col:
                continue
            factor = aug[r][col] % q
            if factor:
                aug[r] = [(v - factor * aug[col][j]) % q for j, v in enumerate(aug[r])]
    return [row[n:] for row in aug]


def matmul_mod(a: list[list[int]], b: list[list[int]], q: int) -> list[list[int]]:
    rows = len(a)
    inner = len(b)
    cols = len(b[0]) if b else 0
    out = [[0 for _ in range(cols)] for _ in range(rows)]
    for i in range(rows):
        for k in range(inner):
            aik = a[i][k]
            if not aik:
                continue
            for j in range(cols):
                out[i][j] = (out[i][j] + aik * b[k][j]) % q
    return out


def kernel_free_map(adjoint: list[list[int]], q: int) -> list[list[int]]:
    n = len(adjoint)
    m = len(adjoint[0])
    pivot = [[adjoint[i][j] for j in range(n)] for i in range(n)]
    rest = [[adjoint[i][j] for j in range(n, m)] for i in range(n)]
    pivot_inv = invert_square_mod(pivot, q)
    solved = matmul_mod(pivot_inv, rest, q)
    return [[0 if x == 0 else q - x for x in row] for row in solved]


def relation_from_free(map_rows: list[list[int]], free: list[int], q: int) -> list[int]:
    n = len(map_rows)
    d = len(free)
    out = [0 for _ in range(n + d)]
    for i in range(n):
        acc = 0
        for j, coeff in enumerate(free):
            if coeff:
                acc += map_rows[i][j] * signed_to_mod(coeff, q)
        out[i] = acc % q
    for j, coeff in enumerate(free):
        out[n + j] = signed_to_mod(coeff, q)
    return out


def relation_stats(vec: list[int], q: int, m0: int) -> dict[str, Any]:
    signed = [centered_signed(x, q) for x in vec]
    guard = signed[: min(m0, len(signed))]
    rest = signed[min(m0, len(signed)) :]
    return {
        "centered_l1": sum(abs(x) for x in signed),
        "support": sum(1 for x in signed if x),
        "linf": max((abs(x) for x in signed), default=0),
        "guard_support": sum(1 for x in guard if x),
        "non_guard_support": sum(1 for x in rest if x),
        "guard_l1": sum(abs(x) for x in guard),
        "non_guard_l1": sum(abs(x) for x in rest),
        "witness": signed,
    }


@dataclass
class Best:
    method: str
    free_sparsity: int
    stats: dict[str, Any]


def update_best(best: Best | None, method: str, free_sparsity: int, vec: list[int], q: int, m0: int) -> Best:
    stats = relation_stats(vec, q, m0)
    if stats["centered_l1"] == 0:
        return best if best is not None else Best(method, free_sparsity, stats)
    if best is None or stats["centered_l1"] < best.stats["centered_l1"]:
        return Best(method, free_sparsity, stats)
    return best


def qary_l1_probe(
    adjoint: list[list[int]],
    q: int,
    m0: int,
    coeff_bound: int,
    random_samples: int,
    max_random_sparsity: int,
    anneal_restarts: int,
    anneal_steps: int,
    seed: int,
) -> dict[str, Any]:
    map_rows = kernel_free_map(adjoint, q)
    d = len(map_rows[0]) if map_rows else 0
    best: Best | None = None

    for j in range(d):
        for coeff in range(-coeff_bound, coeff_bound + 1):
            if coeff == 0:
                continue
            free = [0 for _ in range(d)]
            free[j] = coeff
            best = update_best(best, "free-unit-exact", 1, relation_from_free(map_rows, free, q), q, m0)

    for j0 in range(d):
        for j1 in range(j0 + 1, d):
            for c0 in range(-coeff_bound, coeff_bound + 1):
                if c0 == 0:
                    continue
                for c1 in range(-coeff_bound, coeff_bound + 1):
                    if c1 == 0:
                        continue
                    free = [0 for _ in range(d)]
                    free[j0] = c0
                    free[j1] = c1
                    best = update_best(best, "free-pair-exact", 2, relation_from_free(map_rows, free, q), q, m0)

    rng = Rng64(seed)
    for _ in range(random_samples):
        sparsity = 1 + rng.modulo(max(1, max_random_sparsity))
        free = [0 for _ in range(d)]
        filled = 0
        while filled < min(sparsity, d):
            j = rng.modulo(d)
            if free[j]:
                continue
            mag = 1 + rng.modulo(coeff_bound)
            free[j] = mag if rng.next_u64() & 1 else -mag
            filled += 1
        best = update_best(best, "free-random", filled, relation_from_free(map_rows, free, q), q, m0)

    for restart in range(anneal_restarts):
        free = [0 for _ in range(d)]
        init = 1 + rng.modulo(max(1, max_random_sparsity))
        filled = 0
        while filled < min(init, d):
            j = rng.modulo(d)
            if free[j]:
                continue
            mag = 1 + rng.modulo(coeff_bound)
            free[j] = mag if rng.next_u64() & 1 else -mag
            filled += 1
        current = relation_stats(relation_from_free(map_rows, free, q), q, m0)["centered_l1"]
        best = update_best(best, "free-anneal", filled, relation_from_free(map_rows, free, q), q, m0)
        for _ in range(anneal_steps):
            j = rng.modulo(d)
            old = free[j]
            delta = [-4, -2, -1, 1, 2, 4][rng.modulo(6)]
            new = max(-coeff_bound, min(coeff_bound, old + delta))
            if new == old:
                continue
            free[j] = new
            candidate = relation_from_free(map_rows, free, q)
            candidate_l1 = relation_stats(candidate, q, m0)["centered_l1"]
            uphill = 1 + min(current // 128, 10_000)
            if candidate_l1 <= current or (candidate_l1 <= current + uphill and rng.modulo(max(1, anneal_steps)) == 0):
                current = candidate_l1
                best = update_best(
                    best,
                    "free-anneal",
                    sum(1 for x in free if x),
                    candidate,
                    q,
                    m0,
                )
            else:
                free[j] = old

    assert best is not None
    out = {
        "kernel_free_dimension": d,
        "coeff_bound": coeff_bound,
        "random_samples": random_samples,
        "anneal_restarts": anneal_restarts,
        "anneal_steps": anneal_steps,
        "best_method": best.method,
        "best_free_sparsity": best.free_sparsity,
    }
    out.update({k: v for k, v in best.stats.items() if k != "witness"})
    out["witness_prefix"] = best.stats["witness"][:16]
    return out


def analyze_candidate(root: Path, row: dict[str, Any], args: argparse.Namespace) -> dict[str, Any]:
    c = row["candidate"]
    q = int(c["q"])
    n = int(c["n"])
    m0 = int(c["m0"])
    m1 = int(c["m1"])
    view_len = int(c.get("view_coords") or (m0 + m1))
    matrix_path = root / "matrices" / f"{row['id']}.adjoint_modq.json"
    full = load_json(matrix_path)
    view = [r[:view_len] for r in full]
    full_probe = qary_l1_probe(
        full,
        q,
        m0,
        args.coeff_bound,
        args.random_samples,
        args.max_random_sparsity,
        args.anneal_restarts,
        args.anneal_steps,
        seed=0x535452554354 ^ q ^ n ^ m0 ^ m1,
    )
    view_probe = qary_l1_probe(
        view,
        q,
        m0,
        args.coeff_bound,
        args.random_samples,
        args.max_random_sparsity,
        args.anneal_restarts,
        args.anneal_steps,
        seed=0x565745575F4C31 ^ q ^ n ^ m0 ^ m1,
    )
    effective_eta = int(row["effective_eta"])
    return {
        "id": row["id"],
        "candidate": c,
        "effective_eta": effective_eta,
        "required_centered_l1": math.ceil((q - 1) / (2 * effective_eta)),
        "full_matrix": {
            "columns": m0 + m1,
            "rank": rank_mod(full, q),
            "prefix_rank_profile": prefix_rank_profile(full, q),
            "qary_l1_probe": full_probe,
        },
        "view_matrix": {
            "columns": view_len,
            "rank": rank_mod(view, q),
            "prefix_rank_profile": prefix_rank_profile(view, q),
            "qary_l1_probe": view_probe,
        },
        "interpretation": {
            "view_hides_tilt_coordinates": view_len <= m0,
            "full_probe_publicly_available": view_len == m0 + m1,
            "view_probe_counterexample": view_probe["centered_l1"] < math.ceil((q - 1) / (2 * effective_eta)),
            "full_probe_counterexample": full_probe["centered_l1"] < math.ceil((q - 1) / (2 * effective_eta)),
        },
    }


def markdown(results: list[dict[str, Any]]) -> str:
    lines = [
        "# n=32, m1=32 Structural Dossier",
        "",
        "This report compares the full regenerated adjoint matrix with the selected",
        "public view prefix.  For reduced-view rows, only the view-prefix dual",
        "relations are directly available from the public sample.",
        "",
        "| id | q | m0 | view | k | eff eta | req l1 | full best l1 | full non-guard supp | view best l1 | view supp | view counterexample |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|",
    ]
    for r in results:
        c = r["candidate"]
        full = r["full_matrix"]["qary_l1_probe"]
        view = r["view_matrix"]["qary_l1_probe"]
        lines.append(
            f"| `{r['id']}` | {c['q']} | {c['m0']} | {c.get('view_coords')} | "
            f"{c['fiber_dim']} | {r['effective_eta']} | {r['required_centered_l1']} | "
            f"{full['centered_l1']} | {full['non_guard_support']} | "
            f"{view['centered_l1']} | {view['support']} | "
            f"{r['interpretation']['view_probe_counterexample']} |"
        )
    lines.extend(
        [
            "",
            "## Notes",
            "",
            "- `full best l1` may use hidden tilt coordinates when `view <= m0`.",
            "- `view best l1` is the relevant direct-dual statistic for the public view.",
            "- These are heuristic probes, not lower-bound certificates.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dossier-dir", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    parser.add_argument("--limit", type=int, default=4)
    parser.add_argument("--coeff-bound", type=int, default=16)
    parser.add_argument("--random-samples", type=int, default=50_000)
    parser.add_argument("--max-random-sparsity", type=int, default=16)
    parser.add_argument("--anneal-restarts", type=int, default=128)
    parser.add_argument("--anneal-steps", type=int, default=10_000)
    args = parser.parse_args()

    candidates = load_json(args.dossier_dir / "candidates.json")
    selected = [
        row
        for row in candidates
        if int(row["candidate"]["n"]) == 32 and int(row["candidate"]["m1"]) == 32
    ][: args.limit]
    results = [analyze_candidate(args.dossier_dir, row, args) for row in selected]
    args.output_json.parent.mkdir(parents=True, exist_ok=True)
    args.output_json.write_text(json.dumps(results, indent=2) + "\n")
    args.output_md.write_text(markdown(results))
    print(f"wrote {len(results)} structural rows to {args.output_json}")


if __name__ == "__main__":
    main()
