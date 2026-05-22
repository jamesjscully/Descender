#!/usr/bin/env python3
"""Replay sampler support counts for candidate quotient-level links.

The current reviewed sampler certificate counts the public semantic quotient
plus byte-packing/fiber-state constraints.  A quotient-level account link that
is itself public/account-visible has a stricter support surface: the candidate
link rows also condition the representative.  This script computes that support
model for concrete link rows.
"""

from __future__ import annotations

import argparse
import json
import math
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import quotient_level_link_kernel_search as qll
import response_link_bounded_kernel_check as kernel_check


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_BOUND = 96
TARGET_BITS = 128
LOW_BIT_COORD = 1
FULL_FIXED_COORD = 0
LOW_BIT_MODULUS = 4
LOW_BIT_CLASS = 0


def candidate_specs() -> dict[str, dict[str, Any]]:
    return {
        "current-coordinate-link-0-3": {
            "candidate": "current-coordinate-link-0-3",
            "link_rows": [qll.unit_row(0), qll.unit_row(3)],
            "description": "Current reviewed response-link coordinate control [0,3].",
        },
        "coordinate-link-1-2": {
            "candidate": "coordinate-link-1-2",
            "link_rows": [qll.unit_row(1), qll.unit_row(2)],
            "description": "Earlier exact-passed coordinate comparison row [1,2].",
        },
        "sparse-difference-link-0m3-1m2": {
            "candidate": "sparse-difference-link-0m3-1m2",
            "link_rows": [
                qll.sparse_row([(0, 1), (3, -1)]),
                qll.sparse_row([(1, 1), (2, -1)]),
            ],
            "description": "Earlier exact-passed sparse difference row.",
        },
    }


def candidate_spec(candidate: str) -> dict[str, Any]:
    specs = candidate_specs()
    if candidate in specs:
        return specs[candidate]
    match = re.fullmatch(r"coordinate-link-(\d+)-(\d+)", candidate)
    if match:
        left = int(match.group(1))
        right = int(match.group(2))
        if not (0 <= left < qll.REPRESENTATIVE_RESIDUES and 0 <= right < qll.REPRESENTATIVE_RESIDUES):
            raise ValueError(f"coordinate pair out of range: {candidate}")
        if left == right:
            raise ValueError(f"coordinate pair must be distinct: {candidate}")
        return {
            "candidate": candidate,
            "link_rows": [qll.unit_row(left), qll.unit_row(right)],
            "description": f"Two-coordinate projection candidate [{left},{right}].",
        }
    raise ValueError(f"unknown candidate: {candidate}")


def default_output(candidate: str, bound: int) -> Path:
    return READINESS_DIR / f"quotient_level_link_sampler_replay_{candidate}_b{bound}.json"


def rank(rows: list[list[int]]) -> int:
    return qll.row_rank(rows)


def low_bit_allowed_values() -> int:
    return sum(1 for value in range(qll.Q) if value % LOW_BIT_MODULUS == LOW_BIT_CLASS)


def decimal_support(exponent: int, multiplier: int) -> str:
    return str((qll.Q**exponent) * multiplier)


def support_case(rows: list[list[int]], mode: str) -> dict[str, Any]:
    base_rank = rank(rows)
    with_full_rank = rank(rows + [qll.unit_row(FULL_FIXED_COORD)])
    with_low_rank = rank(rows + [qll.unit_row(FULL_FIXED_COORD), qll.unit_row(LOW_BIT_COORD)])
    full_fixed_independent = with_full_rank > base_rank
    low_bit_coord_independent = with_low_rank > with_full_rank
    allowed_values = low_bit_allowed_values()
    if low_bit_coord_independent:
        exponent = qll.REPRESENTATIVE_RESIDUES - with_low_rank
        multiplier = allowed_values
        low_bit_condition = "surjective_low_bit_coordinate"
        support_zero_risk = False
    else:
        exponent = qll.REPRESENTATIVE_RESIDUES - with_full_rank
        multiplier = 1
        low_bit_condition = "low_bit_coordinate_already_fixed_by_prior_constraints"
        support_zero_risk = True
    support_log2 = exponent * math.log2(qll.Q) + math.log2(multiplier)
    support_floor = math.floor(support_log2)
    return {
        "mode": mode,
        "base_rank": base_rank,
        "rank_after_full_fixed_coord": with_full_rank,
        "rank_after_low_bit_coord": with_low_rank,
        "full_fixed_coord_independent": full_fixed_independent,
        "low_bit_coord_independent": low_bit_coord_independent,
        "q_power_exponent": exponent,
        "low_bit_multiplier": multiplier,
        "support_formula": f"{qll.Q}^{exponent} * {multiplier}",
        "support_decimal": decimal_support(exponent, multiplier),
        "support_log2": support_log2,
        "support_floor_bits": support_floor,
        "target_bits": TARGET_BITS,
        "support_floor_margin_bits": support_floor - TARGET_BITS,
        "meets_target_bits": support_floor >= TARGET_BITS,
        "low_bit_condition": low_bit_condition,
        "support_zero_risk_if_account_value_not_in_low_bit_class": support_zero_risk,
    }


def load_exact_artifact(candidate: str, bound: int) -> dict[str, Any] | None:
    paths = [
        READINESS_DIR / f"quotient_level_link_exact_enum_{candidate}_b{bound}.json",
    ]
    if candidate == "current-coordinate-link-0-3":
        paths.append(
            READINESS_DIR
            / f"response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b{bound}.json"
        )
    for path in paths:
        if path.exists():
            data = json.loads(path.read_text())
            return {
                "artifact": str(path.relative_to(ROOT)),
                "status": data.get("status"),
                "shortest_coeff_bound": data.get("shortest_coeff_bound"),
                "margin_over_response_difference_bound": data.get("margin_over_response_difference_bound"),
                "combined_constraint_rank": data.get("combined_constraint_rank"),
            }
    return None


def build_payload(candidate: str, bound: int) -> dict[str, Any]:
    spec = candidate_spec(candidate)
    matrix = kernel_check.full_response_matrix()
    semantic_rows = [row[:] for row in matrix]
    link_rows = spec["link_rows"]
    semantic_only = support_case(semantic_rows, "semantic_only_current_sampler_surface")
    public_link_conditioned = support_case(
        semantic_rows + link_rows,
        "semantic_plus_public_link_rows",
    )
    semantic_report = qll.semantic_factorization_report(semantic_rows, link_rows)
    exact = load_exact_artifact(candidate, bound)
    status = (
        "hiding_target_met_under_public_link_conditioning"
        if public_link_conditioned["meets_target_bits"]
        else "hiding_target_fails_under_public_link_conditioning"
    )
    return {
        "schema": "Descender.quotient_level_link.sampler_replay.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "candidate": candidate,
        "description": spec["description"],
        "firebreak": {
            "touches_deployable_path": False,
            "touches_current_verifier": False,
            "touches_current_proof_path": False,
        },
        "base_surface": {
            "q": qll.Q,
            "representative_residues": qll.REPRESENTATIVE_RESIDUES,
            "semantic_rows": qll.SEMANTIC_QUOTIENT_ROWS,
            "full_fixed_coord": FULL_FIXED_COORD,
            "low_bit_coord": LOW_BIT_COORD,
            "low_bit_modulus": LOW_BIT_MODULUS,
            "low_bit_class": LOW_BIT_CLASS,
            "low_bit_allowed_values": low_bit_allowed_values(),
            "matrix_transcript_sha256": kernel_check.matrix_transcript_sha256(matrix),
            "response_difference_bound": bound,
        },
        "semantic_factorization": semantic_report,
        "exact_kernel_artifact": exact,
        "support_models": {
            "semantic_only_current_sampler_surface": semantic_only,
            "semantic_plus_public_link_rows": public_link_conditioned,
        },
        "interpretation": [
            (
                "The semantic-only model reproduces the existing sampler support "
                "surface and should not be used for a public quotient-level link "
                "unless the link is hidden by a separate commitment argument."
            ),
            (
                "The public-link-conditioned model treats the candidate link rows "
                "as account-visible quotient data.  This is the relevant first "
                "screen for replacing separate account binding with a quotient "
                "level link."
            ),
        ],
        "status": status,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("candidate")
    parser.add_argument("--bound", type=int, default=DEFAULT_BOUND)
    parser.add_argument("--write-json", type=Path)
    args = parser.parse_args()

    payload = build_payload(args.candidate, args.bound)
    out = args.write_json or default_output(args.candidate, args.bound)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    public_model = payload["support_models"]["semantic_plus_public_link_rows"]
    print(
        json.dumps(
            {
                "json": str(out),
                "candidate": args.candidate,
                "status": payload["status"],
                "public_link_support_formula": public_model["support_formula"],
                "public_link_support_floor_bits": public_model["support_floor_bits"],
                "public_link_support_floor_margin_bits": public_model[
                    "support_floor_margin_bits"
                ],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
