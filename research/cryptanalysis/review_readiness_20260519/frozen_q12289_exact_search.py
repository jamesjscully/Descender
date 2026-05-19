#!/usr/bin/env python3

import json
import sys
from pathlib import Path


def centered_value(x: int, q: int) -> int:
    r = x % q
    if r > q // 2:
        return -(q - r)
    return r


def signed_to_mod(x: int, q: int) -> int:
    if x >= 0:
        return x % q
    y = (-x) % q
    return 0 if y == 0 else q - y


def relation_for_pair(q: int, coeffs: list[list[int]], x: int, y: int) -> list[int]:
    xm = signed_to_mod(x, q)
    ym = signed_to_mod(y, q)
    relation = []
    for a, b in coeffs:
        relation.append(centered_value((a * xm + b * ym) % q, q))
    relation.extend([x, y])
    return relation


def relation_l1(relation: list[int]) -> int:
    return sum(abs(v) for v in relation)


def top_local_pairs(q: int, coeffs: list[list[int]], target_l1: int, keep: int) -> tuple[int, list[dict], int]:
    rows = []
    checked = 0
    radius = target_l1 - 1
    for x in range(-radius, radius + 1):
        for y in range(-radius, radius + 1):
            if x == 0 and y == 0:
                continue
            if abs(x) + abs(y) >= target_l1:
                continue
            checked += 1
            relation = relation_for_pair(q, coeffs, x, y)
            rows.append(
                {
                    "pair": [x, y],
                    "free_l1": abs(x) + abs(y),
                    "relation": relation,
                    "relation_support": sum(1 for v in relation if v != 0),
                    "centered_l1": relation_l1(relation),
                }
            )
    rows.sort(key=lambda row: (row["centered_l1"], row["free_l1"], abs(row["pair"][0]), abs(row["pair"][1]), row["pair"]))
    return checked, rows[:keep], rows[0]["centered_l1"]


def build_report(sidecar: dict, keep: int) -> dict:
    q = sidecar["q"]
    target_l1 = sidecar["target_l1"]
    group_count = sidecar["group_count"]
    coeffs = sidecar["pair_block"]["coefficients"]
    checked, top_rows, min_local_l1 = top_local_pairs(q, coeffs, target_l1, keep)
    per_active_block = []
    for active_blocks in range(1, min(group_count, 8) + 1):
        per_active_block.append(
            {
                "active_blocks": active_blocks,
                "derived_global_min_centered_l1": active_blocks * min_local_l1,
                "beats_target_l1": active_blocks * min_local_l1 < target_l1,
            }
        )
    return {
        "artifact": "frozen-q12289-exact-search",
        "input_sidecar": sidecar,
        "checked_local_pairs": checked,
        "top_local_pairs": top_rows,
        "min_local_centered_l1": min_local_l1,
        "target_l1": target_l1,
        "group_count": group_count,
        "mitm_direct_sum_argument": (
            "The frozen public view is a direct sum of identical 3x2 local blocks. "
            "Global centered-l1 is therefore additive across active blocks, so the "
            "best h-block witness has cost at least h * min_local_centered_l1."
        ),
        "derived_multi_block_bounds": per_active_block,
        "found_counterexample_below_target": min_local_l1 < target_l1,
    }


def markdown(report: dict) -> str:
    lines = [
        "# Frozen q12289 Exact Search",
        "",
        f"- target l1: `{report['target_l1']}`",
        f"- group count: `{report['group_count']}`",
        f"- checked local pairs: `{report['checked_local_pairs']}`",
        f"- min local centered l1: `{report['min_local_centered_l1']}`",
        f"- counterexample below target: `{report['found_counterexample_below_target']}`",
        "",
        "## Top Local Pairs",
        "",
        "| pair | free l1 | centered l1 | support | relation |",
        "|---|---:|---:|---:|---|",
    ]
    for row in report["top_local_pairs"]:
        lines.append(
            f"| `{tuple(row['pair'])}` | {row['free_l1']} | {row['centered_l1']} | {row['relation_support']} | `{row['relation']}` |"
        )
    lines.extend(
        [
            "",
            "## Derived Multi-Block Bounds",
            "",
            report["mitm_direct_sum_argument"],
            "",
            "| active blocks | derived global min centered l1 | below target? |",
            "|---:|---:|---|",
        ]
    )
    for row in report["derived_multi_block_bounds"]:
        lines.append(
            f"| {row['active_blocks']} | {row['derived_global_min_centered_l1']} | {row['beats_target_l1']} |"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    if len(sys.argv) != 4:
        print("usage: frozen_q12289_exact_search.py SIDEcar.json output.json output.md", file=sys.stderr)
        return 2
    input_path = Path(sys.argv[1])
    output_json = Path(sys.argv[2])
    output_md = Path(sys.argv[3])
    sidecar = json.loads(input_path.read_text())
    report = build_report(sidecar, keep=12)
    output_json.write_text(json.dumps(report, indent=2) + "\n")
    output_md.write_text(markdown(report))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
