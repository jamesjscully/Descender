#!/usr/bin/env python3
"""Export validated Descender rows for external cryptanalysis tools.

This script is deliberately an adapter, not a home-grown security estimator.
It consumes `validate_frontier` JSON, keeps only passing rows by default, and
emits:

* a normalized candidate manifest with proof/size margins;
* exact mod-q public operator matrices regenerated from the recorded parameter
  row;
* q-ary dual-lattice bases suitable for fplll/fpylll experiments;
* estimator job templates for external LWE/code estimators.

The external estimators are not invoked here.  The generated files make the
mapping explicit so that a cryptanalysis pass can review whether an off-the-
shelf estimator is faithful, approximate, or inapplicable for a given row.
"""

from __future__ import annotations

import argparse
import json
import math
from dataclasses import dataclass
from pathlib import Path
from typing import Any


MASK64 = (1 << 64) - 1


@dataclass(frozen=True)
class Candidate:
    profile: str
    proof_strategy: str
    primal_distance_model: str
    q: int
    n: int
    m0: int
    m1: int
    fiber_dim: int
    eta: int
    sample_bits: int
    view_coords: int

    @property
    def m(self) -> int:
        return self.m0 + self.m1

    @property
    def view_len(self) -> int:
        return self.m if self.view_coords == 0 else self.view_coords

    @property
    def stem(self) -> str:
        return (
            f"{self.profile}_q{self.q}_n{self.n}_m0{self.m0}_m1{self.m1}_"
            f"k{self.fiber_dim}_eta{self.eta}_bits{self.sample_bits}_view{self.view_len}"
        )


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

    def nonzero_mod(self, q: int) -> int:
        return 1 + self.modulo(q - 1)


def mix64(x: int) -> int:
    x = (x + 0x9E3779B97F4A7C15) & MASK64
    x = ((x ^ (x >> 30)) * 0xBF58476D1CE4E5B9) & MASK64
    x = ((x ^ (x >> 27)) * 0x94D049BB133111EB) & MASK64
    return (x ^ (x >> 31)) & MASK64


def candidate_seed(c: Candidate, domain: int) -> int:
    h = 0xD35CE5DE_5EED_0001 ^ domain
    for byte in c.profile.encode("utf-8"):
        h = mix64(h ^ byte)
    for value in [
        c.q,
        c.n,
        c.m0,
        c.m1,
        c.fiber_dim,
        c.eta,
        c.sample_bits,
    ]:
        h = mix64(h ^ value)
    return h


def inv_mod(x: int, q: int) -> int:
    return pow(x % q, q - 2, q)


def cauchy_block_matrix(n: int, cols: int, q: int, row_offset: int, col_offset: int) -> list[list[int]]:
    matrix = [[0 for _ in range(cols)] for _ in range(n)]
    for i in range(n):
        for j in range(cols):
            x = row_offset + i
            y = col_offset + n + j
            denom = (x + q - (y % q)) % q
            matrix[i][j] = inv_mod(denom, q)
    return matrix


def generated_tilt_matrix(c: Candidate) -> list[list[int]]:
    rng_i = Rng64(candidate_seed(c, 0x1AC1D3CE))
    rng_c = Rng64(candidate_seed(c, 0xC0EFF1C1E))
    rng_d = Rng64(candidate_seed(c, 0xD1A60A1))
    matrix = [[0 for _ in range(c.m1)] for _ in range(c.n)]
    for col in range(c.m1):
        a = rng_i.modulo(c.n)
        b = rng_i.modulo(c.n)
        while b == a:
            b = rng_i.modulo(c.n)
        cc = rng_i.modulo(c.n)
        while cc == a or cc == b:
            cc = rng_i.modulo(c.n)

        diag = rng_d.nonzero_mod(c.q)
        x = rng_c.nonzero_mod(c.q)
        y = rng_c.nonzero_mod(c.q)
        center = (c.q - ((x + y) % c.q)) % c.q
        while center == 0:
            y_new = rng_c.nonzero_mod(c.q)
            center = (c.q - ((x + y_new) % c.q)) % c.q
            if center != 0:
                matrix[cc][col] = (diag * y_new) % c.q
                break
        if matrix[cc][col] == 0:
            matrix[cc][col] = (diag * y) % c.q
        matrix[a][col] = (diag * center) % c.q
        matrix[b][col] = (diag * x) % c.q
    return matrix


def public_adjoint_matrix(c: Candidate) -> list[list[int]]:
    b0 = cauchy_block_matrix(c.n, c.m0, c.q, 0, 0)
    b1 = cauchy_block_matrix(c.n, c.m1, c.q, c.m0, c.m0)
    tilt = generated_tilt_matrix(c)
    for i in range(c.n):
        for j in range(c.m1):
            b1[i][j] = (b1[i][j] + tilt[i][j]) % c.q
    return [left + right for left, right in zip(b0, b1)]


def invert_square_mod(matrix: list[list[int]], q: int) -> list[list[int]]:
    n = len(matrix)
    aug = [row[:] + [1 if i == j else 0 for j in range(n)] for i, row in enumerate(matrix)]
    for col in range(n):
        pivot = next((r for r in range(col, n) if aug[r][col] % q != 0), None)
        if pivot is None:
            raise ValueError("singular leading block")
        aug[col], aug[pivot] = aug[pivot], aug[col]
        inv = inv_mod(aug[col][col], q)
        aug[col] = [(v * inv) % q for v in aug[col]]
        for r in range(n):
            if r == col:
                continue
            factor = aug[r][col] % q
            if factor == 0:
                continue
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
            if aik == 0:
                continue
            for j in range(cols):
                out[i][j] = (out[i][j] + aik * b[k][j]) % q
    return out


def centered_mod(x: int, q: int) -> int:
    x %= q
    if x > q // 2:
        return x - q
    return x


def dual_qary_basis_from_adjoint(adjoint: list[list[int]], q: int) -> list[list[int]]:
    """Basis for {x in Z^m : adjoint * x = 0 mod q}.

    Uses the first n columns as an invertible pivot block.  This matches the
    Cauchy guard-prefix rows used by the current validator.
    """
    n = len(adjoint)
    m = len(adjoint[0])
    pivot = [[adjoint[i][j] for j in range(n)] for i in range(n)]
    rest = [[adjoint[i][j] for j in range(n, m)] for i in range(n)]
    pivot_inv = invert_square_mod(pivot, q)
    solved = matmul_mod(pivot_inv, rest, q)
    basis: list[list[int]] = []
    for free_col in range(m - n):
        vec = [centered_mod(-solved[i][free_col], q) for i in range(n)]
        vec.extend(1 if j == free_col else 0 for j in range(m - n))
        basis.append(vec)
    for i in range(n):
        vec = [0 for _ in range(m)]
        vec[i] = q
        basis.append(vec)
    return basis


def write_matrix(path: Path, matrix: list[list[int]]) -> None:
    path.write_text(json.dumps(matrix, separators=(",", ":")) + "\n")


def parse_candidate(row: dict[str, Any]) -> Candidate:
    raw = row["candidate"]
    return Candidate(
        profile=row["profile"],
        proof_strategy=raw["proof_strategy"],
        primal_distance_model=raw["primal_distance_model"],
        q=int(raw["q"]),
        n=int(raw["n"]),
        m0=int(raw["m0"]),
        m1=int(raw["m1"]),
        fiber_dim=int(raw["fiber_dim"]),
        eta=int(raw["eta"]),
        sample_bits=int(raw.get("sample_bits", 0)),
        view_coords=int(raw.get("view_coords", 0)),
    )


def row_summary(row: dict[str, Any], c: Candidate, index: int) -> dict[str, Any]:
    reported = row["reported"]
    exact = row["exact"]
    return {
        "id": f"{index:04d}_{c.stem}",
        "profile": c.profile,
        "candidate": row["candidate"],
        "public_key_bytes": reported["compact_public_key_bytes"],
        "ciphertext_bytes": reported["ciphertext_bytes"],
        "security_proxy_bits": reported["security_proxy_bits"],
        "average_to_worst_slack": reported["average_to_worst_slack"],
        "guard_slack": reported["guard_slack"],
        "effective_eta": reported.get("effective_eta", c.eta),
        "correctness_margin": exact["correctness_distance_exact"] - exact["correctness_required_distance"],
        "diagonal_slack_bits": exact.get("lossy_diagonal_slack_bits", 0.0),
        "view_route": exact["view_certificate_route"],
        "dual_search_weight": row["generated_setup"]["dual_search_weight"],
        "no_dual_witness_up_to": row["generated_setup"]["no_dual_witness_up_to"],
        "risk_flags": risk_flags(row, c),
    }


def risk_flags(row: dict[str, Any], c: Candidate) -> list[str]:
    flags: list[str] = []
    exact = row["exact"]
    reported = row["reported"]
    if c.n < 64:
        flags.append("small_n_extrapolation")
    if not exact["full_view"]:
        flags.append("reduced_view")
    if c.sample_bits:
        flags.append("quantized_public_sample")
    if reported.get("lossy_diagonal_slack_bits", 0.0) < 16:
        flags.append("thin_diagonal_slack")
    if row["generated_setup"]["no_dual_witness_up_to"] <= row["generated_setup"]["dual_search_weight"]:
        flags.append("bounded_dual_search_only")
    return flags


def estimator_job(summary: dict[str, Any]) -> dict[str, Any]:
    c = summary["candidate"]
    q = int(c["q"])
    n = int(c["n"])
    m = int(c["m0"]) + int(c["m1"])
    eta = int(summary["effective_eta"])
    sigma_proxy = max(eta / math.sqrt(3.0), 1.0 / math.sqrt(12.0))
    return {
        "id": summary["id"],
        "status": "mapping_requires_review",
        "lwe_like": {
            "n": n,
            "m": m,
            "q": q,
            "secret_distribution": "uniform_mod_q_or_projected_secret",
            "error_distribution_proxy": {
                "kind": "centered_uniform_radius",
                "eta": eta,
                "sigma_proxy": sigma_proxy,
            },
            "caveat": "Descender's projected coboundary problem is not vanilla LWE; use this only as a first-pass attack-cost proxy.",
        },
        "dual_qary_lattice": {
            "dimension": m,
            "modulus": q,
            "constraint_rank": n,
            "basis_file": f"fplll/{summary['id']}.dual_qary_basis.txt",
            "caveat": "Basis represents the integer lattice {x : A^T x = 0 mod q} for the regenerated public adjoint matrix.",
        },
        "code_like": {
            "length": m,
            "dimension_proxy": max(m - n, 0),
            "weight_target": summary["no_dual_witness_up_to"] + 1,
            "status": "candidate_mapping_only",
            "caveat": "Use syndrome-decoding estimators only after confirming the relevant attack is Hamming/Lee decoding for this row.",
        },
    }


def write_lattice_estimator_driver(path: Path, jobs_file: str) -> None:
    path.write_text(
        f'''#!/usr/bin/env python3
"""Experimental lattice-estimator driver for Descender exported jobs.

This file intentionally refuses to make a security claim automatically.  Review
the `mapping_requires_review` fields in `{jobs_file}` before trusting outputs.
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

jobs = json.loads(Path(__file__).with_name("{jobs_file}").read_text())

for job in jobs:
    p = job["lwe_like"]
    print("\\n==", job["id"], "==")
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
''')
    path.chmod(0o755)


def write_fpylll_driver(path: Path, jobs_file: str) -> None:
    path.write_text(
        f'''#!/usr/bin/env python3
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

jobs = json.loads(Path(__file__).with_name("{jobs_file}").read_text())

for job in jobs:
    basis_path = Path(__file__).parent / job["dual_qary_lattice"]["basis_file"]
    rows = json.loads(basis_path.read_text())
    A = IntegerMatrix.from_matrix(rows)
    print("\\n==", job["id"], "==")
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
''')
    path.chmod(0o755)


def markdown_report(summaries: list[dict[str, Any]]) -> str:
    lines = [
        "# Descender External Cryptanalysis Export",
        "",
        "This report packages validator-passing Descender rows for external tools.",
        "It is not a cryptanalytic security estimate by itself.",
        "",
        "| id | profile | q | n | m0 | m1 | view | eta | pk | ct | route | margins | flags |",
        "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|",
    ]
    for s in summaries:
        c = s["candidate"]
        margins = (
            f"corr {s['correctness_margin']}; "
            f"diag {s['diagonal_slack_bits']:.1f}; "
            f"avg2wc {s['average_to_worst_slack']:.1f}"
        )
        flags = ", ".join(s["risk_flags"]) or "-"
        lines.append(
            f"| `{s['id']}` | {s['profile']} | {c['q']} | {c['n']} | {c['m0']} | "
            f"{c['m1']} | {c.get('view_coords') or int(c['m0']) + int(c['m1'])} | "
            f"{c['eta']} | {s['public_key_bytes']} | {s['ciphertext_bytes']} | "
            f"{s['view_route']} | {margins} | {flags} |"
        )
    lines.extend(
        [
            "",
            "## External Tool Plan",
            "",
            "1. Run `run_lattice_estimator_jobs.py` only after reviewing the LWE-like mapping caveats.",
            "2. Run `run_fpylll_jobs.py` or the `fplll` CLI on `fplll/*.dual_qary_basis.txt` for concrete reduction sanity checks.",
            "3. Use `estimator_jobs.json` as the handoff point for code/syndrome-decoding estimator mappings.",
            "4. Treat any unexpectedly low cost or short vector as a blocker for the candidate row.",
            "",
        ]
    )
    return "\n".join(lines)


def load_rows(paths: list[Path], include_failures: bool) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in paths:
        payload = json.loads(path.read_text())
        for row in payload["rows"]:
            if include_failures or row["verdict"] == "pass":
                rows.append(row)
    return rows


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--validation-json", nargs="+", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--limit-per-profile", type=int, default=8)
    parser.add_argument("--include-failures", action="store_true")
    args = parser.parse_args()

    rows = load_rows(args.validation_json, args.include_failures)
    rows.sort(
        key=lambda r: (
            r["profile"],
            r["reported"]["compact_public_key_bytes"],
            r["reported"]["ciphertext_bytes"],
            -r["reported"]["security_proxy_bits"],
        )
    )

    selected: list[dict[str, Any]] = []
    per_profile: dict[str, int] = {}
    for row in rows:
        profile = row["profile"]
        if per_profile.get(profile, 0) >= args.limit_per_profile:
            continue
        per_profile[profile] = per_profile.get(profile, 0) + 1
        selected.append(row)

    args.output_dir.mkdir(parents=True, exist_ok=True)
    fplll_dir = args.output_dir / "fplll"
    matrices_dir = args.output_dir / "matrices"
    fplll_dir.mkdir(exist_ok=True)
    matrices_dir.mkdir(exist_ok=True)

    summaries: list[dict[str, Any]] = []
    jobs: list[dict[str, Any]] = []
    for index, row in enumerate(selected):
        c = parse_candidate(row)
        summary = row_summary(row, c, index)
        summaries.append(summary)
        adjoint = public_adjoint_matrix(c)
        basis = dual_qary_basis_from_adjoint(adjoint, c.q)
        write_matrix(matrices_dir / f"{summary['id']}.adjoint_modq.json", adjoint)
        write_matrix(fplll_dir / f"{summary['id']}.dual_qary_basis.txt", basis)
        jobs.append(estimator_job(summary))

    (args.output_dir / "candidates.json").write_text(json.dumps(summaries, indent=2) + "\n")
    (args.output_dir / "estimator_jobs.json").write_text(json.dumps(jobs, indent=2) + "\n")
    (args.output_dir / "README.md").write_text(markdown_report(summaries))
    write_lattice_estimator_driver(args.output_dir / "run_lattice_estimator_jobs.py", "estimator_jobs.json")
    write_fpylll_driver(args.output_dir / "run_fpylll_jobs.py", "estimator_jobs.json")

    print(f"wrote {len(summaries)} candidate dossiers to {args.output_dir}")


if __name__ == "__main__":
    main()
