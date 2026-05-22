#!/usr/bin/env python3
"""Sweep two-coordinate quotient-level link candidates.

This is a fast Milestone 1 sidecar sweep.  It uses the generalized LLL probe to
rank coordinate-pair candidates, with the current reviewed `[0,3]` target kept
as the control.  LLL margins are not certificates; top rows must be promoted to
exact enumeration before any bounded-kernel pass claim.
"""

from __future__ import annotations

import argparse
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import quotient_level_link_kernel_search as qll
import response_link_bounded_kernel_check as kernel_check


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "quotient_level_link_coordinate_pair_sweep.json"
DEFAULT_MD = READINESS_DIR / "quotient_level_link_coordinate_pair_sweep.md"
DEFAULT_BOUND = 96


def exact_artifact_for(candidate: str, bound: int) -> dict[str, Any] | None:
    path = READINESS_DIR / f"quotient_level_link_exact_enum_{candidate}_b{bound}.json"
    if not path.exists():
        return None
    data = json.loads(path.read_text())
    return {
        "artifact": str(path.relative_to(ROOT)),
        "status": data.get("status"),
        "shortest_coeff_bound": data.get("shortest_coeff_bound"),
        "margin_over_response_difference_bound": data.get(
            "margin_over_response_difference_bound"
        ),
        "wall_time_seconds": data.get("wall_time_seconds"),
    }


def evaluated_pair(matrix: list[list[int]], left: int, right: int, bound: int) -> dict[str, Any]:
    link_rows = [qll.unit_row(left), qll.unit_row(right)]
    semantic_report = qll.semantic_factorization_report(matrix, link_rows)
    probe = qll.lll_probe(matrix, link_rows, bound)
    shortest = probe.get("shortest_vector") or {}
    margin = probe.get("margin_over_response_difference_bound")
    found_bad = probe.get("found_bad_vector")
    if found_bad is True:
        status = "lll_rejected"
        bounded_kernel_status = "fail"
    elif found_bad is False:
        status = "needs_exact_enum"
        bounded_kernel_status = "unknown"
    else:
        status = "needs_probe"
        bounded_kernel_status = "unknown"
    candidate = f"coordinate-link-{left}-{right}"
    exact_artifact = exact_artifact_for(candidate, bound)
    if exact_artifact is not None:
        status = f"exact_{exact_artifact['status']}"
        bounded_kernel_status = "pass" if exact_artifact["status"] == "infeasible" else "fail"
    return {
        "candidate": candidate,
        "family": "coordinate_projection",
        "coordinate_pair": [left, right],
        "link_rows": 2,
        "packed_bits": 2 * qll.PACKED_Q_BITS,
        "packed_bytes": qll.packed_q_residue_bytes(2),
        "rank": probe["rank"],
        "fiber_dimension": probe["fiber_dimension"],
        "semantic_factors": semantic_report["standalone_semantic_factors"],
        "semantic_factorization": semantic_report,
        "bounded_kernel_status": bounded_kernel_status,
        "lll_shortest_bound": shortest.get("coeff_bound"),
        "lll_margin_over_response_difference_bound": margin,
        "lll_shortest_vector_prefix": shortest.get("prefix"),
        "exact_enum": exact_artifact,
        "status": status,
    }


def build_payload(top_k: int, exact_margin_floor: int, bound: int) -> dict[str, Any]:
    matrix = kernel_check.full_response_matrix()
    rows = [
        evaluated_pair(matrix, left, right, bound)
        for left in range(qll.REPRESENTATIVE_RESIDUES)
        for right in range(left + 1, qll.REPRESENTATIVE_RESIDUES)
    ]
    rows_sorted = sorted(
        rows,
        key=lambda row: (
            row["lll_margin_over_response_difference_bound"]
            if row["lll_margin_over_response_difference_bound"] is not None
            else -10**9,
            row["lll_shortest_bound"] or -1,
            -row["coordinate_pair"][0],
            -row["coordinate_pair"][1],
        ),
        reverse=True,
    )
    control = next(row for row in rows if row["coordinate_pair"] == [0, 3])
    exact_candidates = [
        row
        for row in rows_sorted
        if row["lll_margin_over_response_difference_bound"] is not None
        and row["lll_margin_over_response_difference_bound"] >= exact_margin_floor
        and row["exact_enum"] is None
    ]
    exact_completed = [
        row for row in rows_sorted if row["exact_enum"] is not None
    ]
    return {
        "schema": "Descender.quotient_level_link.coordinate_pair_sweep.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "lll_sweep_complete_partial_exact",
        "firebreak": {
            "touches_deployable_path": False,
            "touches_current_verifier": False,
            "touches_current_proof_path": False,
        },
        "base_surface": {
            "q": qll.Q,
            "representative_residues": qll.REPRESENTATIVE_RESIDUES,
            "semantic_quotient_rows": qll.SEMANTIC_QUOTIENT_ROWS,
            "matrix_domain": qll.MATRIX_DOMAIN,
            "matrix_transcript_sha256": kernel_check.matrix_transcript_sha256(matrix),
            "response_difference_bound": bound,
        },
        "summary": {
            "pairs_tested": len(rows),
            "top_k": top_k,
            "exact_margin_floor": exact_margin_floor,
            "control_candidate": control["candidate"],
            "control_lll_margin": control["lll_margin_over_response_difference_bound"],
            "control_exact_margin": 43,
            "max_lll_margin": rows_sorted[0]["lll_margin_over_response_difference_bound"],
            "max_lll_candidate": rows_sorted[0]["candidate"],
            "pairs_at_or_above_exact_floor": len(exact_candidates),
            "exact_enum_completed": len(exact_completed),
            "max_exact_margin": max(
                row["exact_enum"]["margin_over_response_difference_bound"]
                for row in exact_completed
            )
            if exact_completed
            else None,
            "standalone_semantic_factor_candidates": sum(
                1 for row in rows if row["semantic_factors"]
            ),
            "exact_completed_standalone_semantic_factor_candidates": sum(
                1 for row in exact_completed if row["semantic_factors"]
            ),
        },
        "top_candidates": rows_sorted[:top_k],
        "exact_enum_completed": exact_completed,
        "exact_enum_candidates": exact_candidates,
        "all_candidates": rows,
        "interpretation": (
            "LLL sweep ranks coordinate pairs only. Rows with strong LLL margins "
            "must be promoted to exact enumeration before being considered "
            "bounded-kernel passes. Coordinate candidates with exact-enum "
            "artifacts have passed only this local sidecar enumeration; they "
            "still require hiding review and independent cryptanalytic replay "
            "before any deployment claim."
        ),
    }


def markdown(payload: dict[str, Any]) -> str:
    summary = payload["summary"]
    lines = [
        "# Quotient-Level Coordinate Pair Sweep",
        "",
        "Status: LLL sweep complete; partial exact enumeration complete.",
        "",
        "This is a sidecar Milestone 1 artifact. It does not change the current",
        "q12289 account-carried proof path or implementation.",
        "",
        "## Summary",
        "",
        "| item | value |",
        "|---|---:|",
        f"| pairs tested | `{summary['pairs_tested']}` |",
        f"| exact margin floor | `{summary['exact_margin_floor']}` |",
        f"| control candidate | `{summary['control_candidate']}` |",
        f"| control LLL margin | `{summary['control_lll_margin']}` |",
        f"| control reviewed exact margin | `{summary['control_exact_margin']}` |",
        f"| max LLL candidate | `{summary['max_lll_candidate']}` |",
        f"| max LLL margin | `{summary['max_lll_margin']}` |",
        f"| exact-enum artifacts completed | `{summary['exact_enum_completed']}` |",
        f"| max exact margin | `{summary['max_exact_margin']}` |",
        f"| standalone semantic-factor candidates | `{summary['standalone_semantic_factor_candidates']}` |",
        f"| exact completed standalone semantic-factor candidates | `{summary['exact_completed_standalone_semantic_factor_candidates']}` |",
        f"| remaining pairs at or above exact floor | `{summary['pairs_at_or_above_exact_floor']}` |",
        "",
        "Semantic screen: no two-coordinate projection candidate standalone",
        "determines the 12-row semantic quotient.  Exact-passed candidates are",
        "therefore auxiliary account/bounded-kernel rows for the product quotient",
        "`semantic x link`, not standalone replacements for the semantic digest.",
        "",
        "## Top Candidates",
        "",
        "| candidate | pair | LLL margin | exact margin | rank | fiber dim | semantic class | status |",
        "|---|---|---:|---:|---:|---:|---|---|",
    ]
    for row in payload["top_candidates"]:
        exact_margin = "-"
        if row["exact_enum"] is not None:
            exact_margin = str(row["exact_enum"]["margin_over_response_difference_bound"])
        lines.append(
            f"| {row['candidate']} | {row['coordinate_pair']} | "
            f"{row['lll_margin_over_response_difference_bound']} | "
            f"{exact_margin} | "
            f"{row['rank']} | {row['fiber_dimension']} | "
            f"{row['semantic_factorization']['classification']} | {row['status']} |"
        )
    lines.extend(
        [
            "",
            "## Exact-Enum Completed",
            "",
            "| candidate | pair | semantic class | status | shortest bound | exact margin | artifact |",
            "|---|---|---|---|---:|---:|---|",
        ]
    )
    for row in payload["exact_enum_completed"]:
        artifact = row["exact_enum"]
        lines.append(
            f"| {row['candidate']} | {row['coordinate_pair']} | "
            f"{row['semantic_factorization']['classification']} | "
            f"{artifact['status']} | {artifact['shortest_coeff_bound']} | "
            f"{artifact['margin_over_response_difference_bound']} | "
            f"`{artifact['artifact']}` |"
        )
    if not payload["exact_enum_completed"]:
        lines.append("| - | - | - | - | - | - | - |")
    lines.extend(
        [
            "",
            "## Exact-Enum Queue",
            "",
            "Rows below met or exceeded the configured LLL margin floor. They are",
            "not certified until exact enumeration completes.",
            "",
            "| candidate | pair | LLL margin |",
            "|---|---|---:|",
        ]
    )
    for row in payload["exact_enum_candidates"]:
        lines.append(
            f"| {row['candidate']} | {row['coordinate_pair']} | "
            f"{row['lll_margin_over_response_difference_bound']} |"
        )
    if not payload["exact_enum_candidates"]:
        lines.append("| - | - | - |")
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            payload["interpretation"],
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--top-k", type=int, default=20)
    parser.add_argument("--exact-margin-floor", type=int, default=43)
    parser.add_argument("--bound", type=int, default=DEFAULT_BOUND)
    parser.add_argument("--write-json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--write-md", type=Path, default=DEFAULT_MD)
    args = parser.parse_args()

    payload = build_payload(args.top_k, args.exact_margin_floor, args.bound)
    if args.write_json is not None:
        args.write_json.parent.mkdir(parents=True, exist_ok=True)
        args.write_json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    if args.write_md is not None:
        args.write_md.parent.mkdir(parents=True, exist_ok=True)
        args.write_md.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": str(args.write_json) if args.write_json else None,
                "markdown": str(args.write_md) if args.write_md else None,
                "pairs_tested": payload["summary"]["pairs_tested"],
                "max_lll_candidate": payload["summary"]["max_lll_candidate"],
                "max_lll_margin": payload["summary"]["max_lll_margin"],
                "exact_queue": [
                    row["candidate"] for row in payload["exact_enum_candidates"]
                ],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
