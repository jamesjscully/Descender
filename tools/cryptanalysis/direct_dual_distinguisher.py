#!/usr/bin/env python3
"""Direct q-ary dual distinguisher analysis for Descender samples.

For a public sample t = A s + e mod q and an integer vector z satisfying
A^T z = 0 mod q, the statistic <z, t> equals <z, e> mod q.  For a uniform
null sample, <z, t> is uniform mod q whenever z is nonzero modulo q.

This is not an estimator proxy: it is the direct distinguishing statistic for
the exported dual vector under the stated centered independent noise envelope.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


def centered_window_size(radius: int, q: int) -> int:
    """Number of residues with centered absolute value <= radius."""
    if radius >= q // 2:
        return q
    return 2 * radius + 1


def analyze(row: dict[str, Any]) -> dict[str, Any]:
    q = int(row["q"])
    eta = int(row.get("eta", 1))
    vector = [int(x) for x in row["vector"]]
    stats = row["stats"].get("total", row["stats"])
    l1 = sum(abs(x) for x in vector)
    radius = eta * l1
    window = centered_window_size(radius, q)
    uniform_hit_probability = window / q
    real_hit_probability_bound = 1.0
    distinguishing_gap = real_hit_probability_bound - uniform_hit_probability
    dual_residual_ok = row.get("adjoint_residual", {}).get(
        "all_zero_mod_q", row.get("residual_all_zero_mod_q", False)
    )
    return {
        "id": row["id"],
        "profile": row.get("profile", ""),
        "q": q,
        "eta": eta,
        "support": stats["support"],
        "l1": l1,
        "l2_norm2": stats["l2_norm2"],
        "linf": stats["linf"],
        "dual_residual_ok": dual_residual_ok,
        "noise_inner_product_radius": radius,
        "radius_below_half_modulus": radius < q // 2,
        "uniform_window_probability": uniform_hit_probability,
        "real_window_probability_bound": real_hit_probability_bound,
        "distinguishing_gap_lower_bound": distinguishing_gap,
        "interpretation": (
            "direct_break_under_centered_independent_noise"
            if distinguishing_gap > 0.5 and dual_residual_ok
            else "needs_review"
        ),
    }


def markdown(results: list[dict[str, Any]]) -> str:
    lines = [
        "# Direct Dual Distinguisher Analysis",
        "",
        "For any integer vector `z` with `A^T z = 0 mod q`, real samples",
        "`t = A s + e` satisfy `<z,t> = <z,e> mod q`.  Uniform samples make",
        "`<z,t>` uniform modulo `q`.  The table uses the deterministic test",
        "`centeredAbs(<z,t>) <= eta * ||z||_1`, which accepts real samples with",
        "probability 1 under the centered coordinate noise envelope.",
        "",
        "| id | q | eta | support | l1 | radius | uniform hit | gap lower bound | interpretation |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---|",
    ]
    for r in results:
        lines.append(
            f"| `{r['id']}` | {r['q']} | {r['eta']} | {r['support']} | "
            f"{r['l1']} | {r['noise_inner_product_radius']} | "
            f"{r['uniform_window_probability']:.6f} | "
            f"{r['distinguishing_gap_lower_bound']:.6f} | {r['interpretation']} |"
        )
    lines.extend(
        [
            "",
            "Conclusion:",
            "",
            "If the KEM public sample/null game is literally `A s + e` versus",
            "uniform over the same coordinate view, then these q-ary dual vectors",
            "give a direct distinguisher.  This is not a proxy-estimator result.",
            "The scheme must either remove these dual vectors from the public view,",
            "make the noise large enough to wrap modulo `q`, or change the security",
            "game/reduction so this statistic is not available.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--reduced-vectors", type=Path, required=True)
    parser.add_argument("--candidates", type=Path, required=True)
    parser.add_argument("--output-json", type=Path, required=True)
    parser.add_argument("--output-md", type=Path, required=True)
    args = parser.parse_args()

    reduced = json.loads(args.reduced_vectors.read_text())
    candidates = {row["id"]: row for row in json.loads(args.candidates.read_text())}
    enriched = []
    for row in reduced:
        summary = candidates[row["id"]]
        c = summary["candidate"]
        enriched_row = dict(row)
        enriched_row["eta"] = int(summary.get("effective_eta", c["eta"]))
        enriched.append(analyze(enriched_row))

    args.output_json.write_text(json.dumps(enriched, indent=2) + "\n")
    args.output_md.write_text(markdown(enriched))
    print(f"wrote {len(enriched)} direct distinguisher diagnostics")


if __name__ == "__main__":
    main()
