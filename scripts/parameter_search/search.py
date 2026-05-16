#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import math
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_PROFILES = Path(__file__).resolve().with_name("default_profiles.json")


@dataclass(frozen=True)
class Candidate:
    q: int
    n: int
    m0: int
    m1: int
    fiber_dim: int
    eta: int

    @property
    def m(self) -> int:
        return self.m0 + self.m1


@dataclass(frozen=True)
class Metrics:
    logq_bits: float
    elem_bytes: int
    dense_public_key_bytes: int
    compact_public_key_bytes: int
    ciphertext_bytes: int
    secret_key_bytes: int
    guard_distance: int
    guard_slack: int
    cauchy_field_slack: int
    noise_headroom: int
    primal_proxy_bits: float
    dual_proxy_bits: float
    self_randomization_proxy_bits: float
    security_proxy_bits: float
    average_to_worst_slack: float


@dataclass(frozen=True)
class Profile:
    id: str
    target_security_bits: int
    max_public_key_bytes: int
    max_ciphertext_bytes: int
    max_secret_key_bytes: int
    min_guard_slack: int
    min_average_to_worst_slack: int
    description: str


def ceil_div(a: int, b: int) -> int:
    return (a + b - 1) // b


def elem_bytes(q: int) -> int:
    return max(1, ceil_div(q.bit_length(), 8))


def log2_choose_upper(n: int, k: int) -> float:
    if k < 0 or k > n:
        return float("-inf")
    if k == 0 or k == n:
        return 0.0
    k = min(k, n - k)
    return sum(math.log2(n - i) - math.log2(i + 1) for i in range(k))


def evaluate(c: Candidate) -> Metrics:
    eb = elem_bytes(c.q)
    logq = math.log2(c.q)

    # Full dense storage is not the intended implementation, but it is useful as
    # a sanity ceiling for compact-evaluator claims.
    dense_pk = c.n * c.m * eb

    # Compact public description proxy:
    # Cauchy x/alpha elements, sampler seeds, wedge coefficients, and diagonal.
    # This is deliberately conservative until concrete serialization lands.
    setup_seed_bytes = 96
    cauchy_description = (c.n + c.m) * eb
    wedge_coefficients = c.m1 * (2 * c.fiber_dim * eb)
    wedge_endpoint_ids = c.m1 * 6
    diagonal = c.m1 * eb
    compact_pk = setup_seed_bytes + cauchy_description + wedge_coefficients + wedge_endpoint_ids + diagonal

    ct = c.m * eb + 32
    sk = c.n * eb + 64

    guard_distance = c.m0 - c.n + 1
    guard_slack = guard_distance - (2 * c.eta + 1)
    cauchy_slack = c.q - (c.n + c.m)
    noise_headroom = (c.q // 2) - c.eta

    # Screening proxies, not security estimates.  The intent is to rank and
    # reject candidates before running heavier exact searches and external
    # lattice/coding estimators.
    noise_width = 2 * c.eta + 1
    primal_proxy = 0.35 * c.n * max(0.0, math.log2(c.q / noise_width))

    dual_window = max(1, min(guard_distance, c.fiber_dim * c.m1 // max(1, c.n)))
    dual_support_cost = log2_choose_upper(c.m, dual_window)
    dual_coeff_cost = dual_window * math.log2(max(2, c.q // noise_width))
    dual_proxy = 0.5 * (dual_support_cost + dual_coeff_cost)

    entropy_source = c.n * math.log2(c.q)
    public_measurements = c.m * math.log2(noise_width)
    randomization_proxy = max(0.0, entropy_source - public_measurements)

    security_proxy = min(primal_proxy, dual_proxy, randomization_proxy)
    average_to_worst_slack = randomization_proxy - 0.125 * c.n

    return Metrics(
        logq_bits=logq,
        elem_bytes=eb,
        dense_public_key_bytes=dense_pk,
        compact_public_key_bytes=compact_pk,
        ciphertext_bytes=ct,
        secret_key_bytes=sk,
        guard_distance=guard_distance,
        guard_slack=guard_slack,
        cauchy_field_slack=cauchy_slack,
        noise_headroom=noise_headroom,
        primal_proxy_bits=primal_proxy,
        dual_proxy_bits=dual_proxy,
        self_randomization_proxy_bits=randomization_proxy,
        security_proxy_bits=security_proxy,
        average_to_worst_slack=average_to_worst_slack,
    )


def hard_failures(c: Candidate, m: Metrics, p: Profile) -> list[str]:
    failures: list[str] = []
    if c.q <= c.n + c.m:
        failures.append("cauchy_field_size")
    if c.m0 < c.n:
        failures.append("guard_budget_m0_ge_n")
    if c.eta <= 0:
        failures.append("positive_noise")
    if c.eta >= c.q // 2:
        failures.append("centered_noise_headroom")
    if m.guard_slack < p.min_guard_slack:
        failures.append("guard_correctness_slack")
    if m.average_to_worst_slack < p.min_average_to_worst_slack:
        failures.append("average_to_worst_slack_proxy")
    if m.security_proxy_bits < p.target_security_bits:
        failures.append("security_proxy_bits")
    if m.compact_public_key_bytes > p.max_public_key_bytes:
        failures.append("public_key_budget")
    if m.ciphertext_bytes > p.max_ciphertext_bytes:
        failures.append("ciphertext_budget")
    if m.secret_key_bytes > p.max_secret_key_bytes:
        failures.append("secret_key_budget")
    return failures


def load_profiles(path: Path) -> list[Profile]:
    raw = json.loads(path.read_text(encoding="utf-8"))
    return [Profile(**item) for item in raw["profiles"]]


def display_path(path: Path) -> str:
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)


def parse_ints(value: str) -> list[int]:
    return [int(part) for part in value.split(",") if part.strip()]


def candidates(args: argparse.Namespace) -> Iterable[Candidate]:
    q_values = parse_ints(args.q_values)
    n_values = parse_ints(args.n_values)
    m0_overheads = parse_ints(args.m0_overheads)
    m1_multipliers = parse_ints(args.m1_multipliers)
    fiber_dims = parse_ints(args.fiber_dims)
    eta_values = parse_ints(args.eta_values)
    for q in q_values:
        for n in n_values:
            for overhead in m0_overheads:
                m0 = n + overhead
                for mult in m1_multipliers:
                    m1 = mult * n
                    for fiber_dim in fiber_dims:
                        for eta in eta_values:
                            yield Candidate(q=q, n=n, m0=m0, m1=m1, fiber_dim=fiber_dim, eta=eta)


def pareto_frontier(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    frontier: list[dict[str, Any]] = []
    for row in rows:
        dominated = False
        for other in rows:
            if other is row:
                continue
            if (
                other["metrics"]["security_proxy_bits"] >= row["metrics"]["security_proxy_bits"]
                and other["metrics"]["compact_public_key_bytes"] <= row["metrics"]["compact_public_key_bytes"]
                and other["metrics"]["ciphertext_bytes"] <= row["metrics"]["ciphertext_bytes"]
                and (
                    other["metrics"]["security_proxy_bits"] > row["metrics"]["security_proxy_bits"]
                    or other["metrics"]["compact_public_key_bytes"] < row["metrics"]["compact_public_key_bytes"]
                    or other["metrics"]["ciphertext_bytes"] < row["metrics"]["ciphertext_bytes"]
                )
            ):
                dominated = True
                break
        if not dominated:
            frontier.append(row)
    return sorted(
        frontier,
        key=lambda r: (
            -r["metrics"]["security_proxy_bits"],
            r["metrics"]["compact_public_key_bytes"],
            r["metrics"]["ciphertext_bytes"],
        ),
    )


def markdown_report(
    profile: Profile,
    accepted_top: list[dict[str, Any]],
    accepted_total: int,
    rejected_count: int,
) -> str:
    lines = [
        f"# Parameter Search Report: {profile.id}",
        "",
        "This is a coarse screening report. The `*_proxy_bits` columns are not security claims;",
        "they are filters for deciding which candidates deserve exact projection, dual-witness,",
        "and lattice/coding-estimator follow-up.",
        "",
        f"- target security bits: `{profile.target_security_bits}`",
        f"- accepted candidates before truncation: `{accepted_total}`",
        f"- candidates shown: `{len(accepted_top)}`",
        f"- rejected candidates: `{rejected_count}`",
        "",
        "| q | n | m0 | m1 | k | eta | security proxy | primal | dual | self-rand | pk bytes | ct bytes | guard slack |",
        "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for row in accepted_top[:20]:
        c = row["candidate"]
        m = row["metrics"]
        lines.append(
            "| {q} | {n} | {m0} | {m1} | {fiber_dim} | {eta} | {sec:.1f} | {prim:.1f} | "
            "{dual:.1f} | {rand:.1f} | {pk} | {ct} | {guard} |".format(
                q=c["q"],
                n=c["n"],
                m0=c["m0"],
                m1=c["m1"],
                fiber_dim=c["fiber_dim"],
                eta=c["eta"],
                sec=m["security_proxy_bits"],
                prim=m["primal_proxy_bits"],
                dual=m["dual_proxy_bits"],
                rand=m["self_randomization_proxy_bits"],
                pk=m["compact_public_key_bytes"],
                ct=m["ciphertext_bytes"],
                guard=m["guard_slack"],
            )
        )
    lines.extend([
        "",
        "## Next Checks",
        "",
        "1. Run exact finite-field rank and projection-distance checks for the top frontier rows.",
        "2. Run dual-witness searches on scaled toy instances preserving the same ratios.",
        "3. Replace screening proxies with a real lattice/coding attack estimator.",
        "4. Keep only candidates whose proof-side constraints can be instantiated in Lean.",
        "",
    ])
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Coarse Descender concrete-parameter search")
    parser.add_argument("--profiles", type=Path, default=DEFAULT_PROFILES)
    parser.add_argument("--profile", default="descender-128")
    parser.add_argument("--output-dir", type=Path, default=ROOT / "research" / "parameter_search")
    parser.add_argument("--q-values", default="3329,7681,12289,40961,65537")
    parser.add_argument("--n-values", default="64,96,128,192,256,384,512")
    parser.add_argument("--m0-overheads", default="8,16,32,64,128,256")
    parser.add_argument("--m1-multipliers", default="1,2,3,4")
    parser.add_argument("--fiber-dims", default="4,8,16")
    parser.add_argument("--eta-values", default="1,2,3,4,6,8")
    parser.add_argument("--top", type=int, default=50)
    args = parser.parse_args()

    profiles = {profile.id: profile for profile in load_profiles(args.profiles)}
    if args.profile not in profiles:
        raise SystemExit(f"unknown profile {args.profile!r}; choices: {', '.join(sorted(profiles))}")
    profile = profiles[args.profile]

    accepted: list[dict[str, Any]] = []
    rejected: dict[str, int] = {}
    total = 0
    for candidate in candidates(args):
        total += 1
        metrics = evaluate(candidate)
        failures = hard_failures(candidate, metrics, profile)
        if failures:
            for failure in failures:
                rejected[failure] = rejected.get(failure, 0) + 1
            continue
        accepted.append({
            "candidate": asdict(candidate),
            "metrics": asdict(metrics),
        })

    accepted.sort(
        key=lambda row: (
            -row["metrics"]["security_proxy_bits"],
            row["metrics"]["compact_public_key_bytes"],
            row["metrics"]["ciphertext_bytes"],
        )
    )
    accepted_total = len(accepted)
    accepted_top = accepted[: args.top]
    frontier = pareto_frontier(accepted_top)

    args.output_dir.mkdir(parents=True, exist_ok=True)
    payload = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "profile": asdict(profile),
        "search_space": {
            "q_values": parse_ints(args.q_values),
            "n_values": parse_ints(args.n_values),
            "m0_overheads": parse_ints(args.m0_overheads),
            "m1_multipliers": parse_ints(args.m1_multipliers),
            "fiber_dims": parse_ints(args.fiber_dims),
            "eta_values": parse_ints(args.eta_values),
            "total_candidates": total,
        },
        "rejected_by_reason": rejected,
        "accepted_total": accepted_total,
        "accepted_top": accepted_top,
        "pareto_frontier": frontier,
        "notes": [
            "security_proxy_bits is a screening score, not a cryptanalytic estimate",
            "hard filters encode current proof-side constraints such as q > n + m0 + m1 and m0 >= n",
            "top candidates require exact finite-field and lattice/coding follow-up before any security claim",
        ],
    }
    json_path = args.output_dir / f"{profile.id}.json"
    md_path = args.output_dir / f"{profile.id}.md"
    json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    md_path.write_text(
        markdown_report(profile, accepted_top, accepted_total, total - accepted_total),
        encoding="utf-8",
    )

    print(f"wrote {display_path(json_path)}")
    print(f"wrote {display_path(md_path)}")
    print(f"accepted_total={accepted_total} accepted_top={len(accepted_top)} pareto={len(frontier)} total={total}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
