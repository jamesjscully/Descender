#!/usr/bin/env python3
"""Generate the Path 2 descent/SIS norm/range surface."""

from __future__ import annotations

import json
from math import ceil, sqrt
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
BINDING_JSON = ROOT / "research/path2_descent_sis_binding_surface.json"
OUT_JSON = ROOT / "research/path2_descent_sis_norm_range_surface.json"
OUT_MD = ROOT / "research/path2_descent_sis_norm_range_surface.md"

Q = 12_289
CENTERED_MAX = (Q - 1) // 2


def load_binding() -> dict[str, Any]:
    return json.loads(BINDING_JSON.read_text())


def l2_bound(entries: int, bound_each: int) -> float:
    return sqrt(entries) * bound_each


def build_model() -> dict[str, Any]:
    binding = load_binding()
    role_map = {role["role"]: role for role in binding["role_surface"]}
    raw_centered = []
    digit_encoded = []
    for role_name in ("pad_source", "projection_relation", "opening_tuple"):
        role = role_map[role_name]
        semantic_lanes = role["secret_lanes"]
        raw_centered.append(
            {
                "role": role_name,
                "semantic_lanes": semantic_lanes,
                "canonical_interval": f"[-{CENTERED_MAX}, {CENTERED_MAX}]",
                "delta_linf_bound_per_semantic_lane": Q - 1,
                "semantic_delta_l2_bound": round(
                    l2_bound(semantic_lanes, Q - 1), 3
                ),
                "binding_consequence": (
                    "valid canonical residues are unique, but the exported "
                    "kernel vector is not small in the sense needed for a "
                    "clean SIS binding claim"
                ),
            }
        )
        two_limb_columns = semantic_lanes * 2
        four_limb_columns = semantic_lanes * 4
        digit_encoded.append(
            {
                "role": role_name,
                "semantic_lanes": semantic_lanes,
                "base_128_columns": two_limb_columns,
                "base_128_linf_per_limb": 127,
                "base_128_delta_l2_bound": round(l2_bound(two_limb_columns, 127), 3),
                "base_16_columns": four_limb_columns,
                "base_16_linf_per_limb": 15,
                "base_16_delta_l2_bound": round(l2_bound(four_limb_columns, 15), 3),
            }
        )

    return {
        "schema": "Descender.path2_descent_sis_norm_range_surface.v1",
        "status": "norm_range_blocker_pinned",
        "q": Q,
        "binding_dependency": "research/path2_descent_sis_binding_surface.md",
        "core_issue": (
            "SIS binding needs two accepting openings to export a nonzero short "
            "kernel vector. The current feasibility vectors name q12289 semantic "
            "lanes directly; canonical residue uniqueness is not the same as "
            "shortness."
        ),
        "raw_centered_residue_surface": raw_centered,
        "candidate_representation_options": [
            {
                "id": "raw_centered_residue",
                "status": "not_sufficient_as_binding_hardness_surface",
                "description": (
                    "Keep one q12289 column per semantic lane and prove only "
                    "canonical centered residue bounds."
                ),
                "advantage": "minimal columns and keeps the current 725/893 byte lower-bound shape",
                "problem": (
                    "semantic deltas can be as large as q-1, so the exported "
                    "kernel vector is not a compelling short-SIS witness without "
                    "a separate, likely weak, bounded-SIS assumption"
                ),
            },
            {
                "id": "base_128_digit_opening",
                "status": "primary_next_probe",
                "description": (
                    "Represent each q12289 semantic residue by two base-128 "
                    "digits plus a reconstruction/range check."
                ),
                "advantage": (
                    "keeps semantic deltas within 127 per digit while adding only "
                    "one extra semantic column per residue"
                ),
                "problem": (
                    "requires proof constraints that the digits reconstruct the "
                    "q12289 residue and that each digit is in [0,127]"
                ),
            },
            {
                "id": "base_16_digit_opening",
                "status": "fallback_if_estimator_dislikes_base_128",
                "description": (
                    "Represent each q12289 semantic residue by four nibbles plus "
                    "a reconstruction/range check."
                ),
                "advantage": "much smaller per-coordinate delta bound",
                "problem": "doubles digit columns again and likely raises account-creation proof bytes",
            },
            {
                "id": "masked_short_lift",
                "status": "research_only_until_hiding_sampler_is_fixed",
                "description": (
                    "Use the descent/sampler construction to commit only short "
                    "lifts while proving they map to the public q12289 residue."
                ),
                "advantage": "most aligned with the descent thesis if a natural short lift exists",
                "problem": (
                    "risks reintroducing sampler-conditioning or projection-link "
                    "leakage unless the hiding sampler row closes at the same time"
                ),
            },
        ],
        "digit_encoding_cost_probe": digit_encoded,
        "recommended_next_step": (
            "Run the binding hardness sanity probe for base_128_digit_opening "
            "and raw_centered_residue side by side. If raw-centered looks weak "
            "or assumption-heavy, promote base-128 digit openings and update "
            "the feasibility byte model."
        ),
        "non_claims": [
            "this artifact does not choose final norm bounds",
            "this artifact does not prove range-check soundness",
            "this artifact does not update the deployable byte estimate",
            "this artifact deliberately keeps p2_sis_norm_range open",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Descent/SIS Norm/Range Surface",
        "",
        "Status: norm/range blocker pinned.",
        "",
        "## Core Issue",
        "",
        model["core_issue"],
        "",
        "The binding surface already gives the algebraic reduction:",
        "`delta = v0 - v1` and `A_role,rowband * delta = 0 mod q`. The missing",
        "piece is whether `delta` is short enough to support the intended SIS",
        "binding claim.",
        "",
        "## Raw Centered Residue Surface",
        "",
        "| role | semantic lanes | canonical interval | delta Linf/lane | semantic delta L2 | consequence |",
        "|---|---:|---|---:|---:|---|",
    ]
    for item in model["raw_centered_residue_surface"]:
        lines.append(
            "| {role} | {semantic_lanes} | `{canonical_interval}` | "
            "{delta_linf_bound_per_semantic_lane} | {semantic_delta_l2_bound} | "
            "{binding_consequence} |".format(**item)
        )
    lines.extend(
        [
            "",
            "## Candidate Representation Options",
            "",
            "| id | status | advantage | problem |",
            "|---|---|---|---|",
        ]
    )
    for item in model["candidate_representation_options"]:
        lines.append(
            "| `{id}` | {status} | {advantage} | {problem} |".format(**item)
        )
    lines.extend(
        [
            "",
            "## Digit Encoding Cost Probe",
            "",
            "| role | residues | base-128 columns | base-128 delta L2 | base-16 columns | base-16 delta L2 |",
            "|---|---:|---:|---:|---:|---:|",
        ]
    )
    for item in model["digit_encoding_cost_probe"]:
        lines.append(
            "| {role} | {semantic_lanes} | {base_128_columns} | "
            "{base_128_delta_l2_bound} | {base_16_columns} | "
            "{base_16_delta_l2_bound} |".format(**item)
        )
    lines.extend(
        [
            "",
            "## Recommended Next Step",
            "",
            model["recommended_next_step"],
            "",
            "## Non-Claims",
            "",
        ]
    )
    for item in model["non_claims"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "This is the first place where the descent/SIS path could genuinely change",
            "shape. The existing feasibility model is byte-attractive because semantic",
            "q12289 residues occupy one column each, but that is exactly what makes the",
            "binding reduction awkward. A digit opening is likely the cleanest way to",
            "make the binding assumption conventional, at the cost of more columns and",
            "range-check constraints in the account-creation payload.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    model = build_model()
    OUT_JSON.write_text(json.dumps(model, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(write_markdown(model))
    print(json.dumps({"json": str(OUT_JSON), "markdown": str(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
