#!/usr/bin/env python3
"""Generate the quotient-level link hiding ledger.

This is a Milestone 2 sidecar artifact.  It does not promote any candidate into
the deployable path.  The review separates raw product-fiber dimension from the
stronger sampler/support claims already reviewed for the current control.
"""

from __future__ import annotations

import json
import math
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
KERNEL_SEARCH = READINESS_DIR / "quotient_level_link_kernel_search.json"
PAIR_SWEEP = READINESS_DIR / "quotient_level_link_coordinate_pair_sweep.json"
PAD_REVIEW = READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json"
PAD_MEMO = READINESS_DIR / "pad_sampler_boundary_memo_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "quotient_level_link_hiding_review.json"
DEFAULT_MD = READINESS_DIR / "quotient_level_link_hiding_review.md"

Q = 12289
TARGET_BITS = 128
REPRESENTATIVE_RESIDUES = 23


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def log2_support_for_formula(q: int, free_residues: int, low_bit_class_count: int) -> float:
    return free_residues * math.log2(q) + math.log2(low_bit_class_count)


def base_capacity(rank: int) -> dict[str, Any]:
    free_q_residues = REPRESENTATIVE_RESIDUES - rank
    raw_bits = free_q_residues * math.log2(Q)
    return {
        "free_q_residues_after_product_constraints": free_q_residues,
        "raw_q_residue_bits": raw_bits,
        "raw_q_residue_bits_floor": math.floor(raw_bits),
        "raw_q_residue_margin_vs_target_bits": raw_bits - TARGET_BITS,
    }


def sampler_replay_for(candidate: str) -> dict[str, Any] | None:
    path = READINESS_DIR / f"quotient_level_link_sampler_replay_{candidate}_b96.json"
    if not path.exists():
        return None
    data = load_json(path)
    public_model = data["support_models"]["semantic_plus_public_link_rows"]
    return {
        "artifact": rel(path),
        "status": data["status"],
        "public_link_support_formula": public_model["support_formula"],
        "public_link_support_floor_bits": public_model["support_floor_bits"],
        "public_link_support_floor_margin_bits": public_model[
            "support_floor_margin_bits"
        ],
        "public_link_meets_target_bits": public_model["meets_target_bits"],
        "low_bit_condition": public_model["low_bit_condition"],
        "support_zero_risk_if_account_value_not_in_low_bit_class": public_model[
            "support_zero_risk_if_account_value_not_in_low_bit_class"
        ],
    }


def current_control(kernel: dict[str, Any]) -> dict[str, Any]:
    row = next(
        item
        for item in kernel["control_candidates"]
        if item["candidate"] == "current-coordinate-link-0-3"
    )
    return {
        "candidate": row["candidate"],
        "family": row["family"],
        "priority": "control",
        "packed_bytes": row["packed_bytes"],
        "rank": row["rank"],
        "fiber_dimension": row["fiber_dimension"],
        "semantic_class": "auxiliary_product_quotient_only",
        "bounded_kernel_margin": row["margin_over_response_difference_bound"],
        "bounded_kernel_status": row["bounded_kernel_status"],
        "exact_artifact": row.get("exact_artifact"),
        "source": "kernel_search.control_candidates",
    }


def exact_coordinate_rows(pair_sweep: dict[str, Any]) -> list[dict[str, Any]]:
    rows = []
    for row in pair_sweep["exact_enum_completed"]:
        if row["candidate"] == "coordinate-link-1-2":
            priority = "deprioritized_lower_kernel_margin"
        elif row["exact_enum"]["margin_over_response_difference_bound"] == pair_sweep["summary"]["max_exact_margin"]:
            priority = "top_exact_coordinate_shortlist"
        else:
            priority = "exact_coordinate_comparison"
        rows.append(
            {
                "candidate": row["candidate"],
                "family": row["family"],
                "priority": priority,
                "packed_bytes": row["packed_bytes"],
                "rank": row["rank"],
                "fiber_dimension": row["fiber_dimension"],
                "semantic_class": row["semantic_factorization"]["classification"],
                "bounded_kernel_margin": row["exact_enum"][
                    "margin_over_response_difference_bound"
                ],
                "bounded_kernel_status": row["bounded_kernel_status"],
                "exact_artifact": row["exact_enum"]["artifact"],
                "source": "coordinate_pair_sweep.exact_enum_completed",
            }
        )
    return rows


def exact_generalized_rows(kernel: dict[str, Any]) -> list[dict[str, Any]]:
    rows = []
    for row in kernel["evaluated_candidates"]:
        if row.get("bounded_kernel_status") != "pass":
            continue
        if not row.get("exact_artifact"):
            continue
        rows.append(
            {
                "candidate": row["candidate"],
                "family": row["family"],
                "priority": "deprioritized_lower_kernel_margin",
                "packed_bytes": row["packed_bytes"],
                "rank": row["rank"],
                "fiber_dimension": row["fiber_dimension"],
                "semantic_class": row["semantic_factorization"]["classification"],
                "bounded_kernel_margin": row["margin_over_response_difference_bound"],
                "bounded_kernel_status": row["bounded_kernel_status"],
                "exact_artifact": row["exact_artifact"],
                "source": "kernel_search.evaluated_candidates",
            }
        )
    return rows


def dedupe_rows(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    seen: set[str] = set()
    out = []
    for row in rows:
        if row["candidate"] in seen:
            continue
        seen.add(row["candidate"])
        out.append(row)
    return out


def enrich_row(
    row: dict[str, Any],
    control_margin: int,
    reviewed_support_bits: float,
    reviewed_support_floor: int,
) -> dict[str, Any]:
    capacity = base_capacity(int(row["rank"]))
    is_control = row["priority"] == "control"
    margin_delta = (
        None
        if row["bounded_kernel_margin"] is None
        else row["bounded_kernel_margin"] - control_margin
    )
    if is_control:
        sampler_status = "reviewed_current_control"
        usable_floor = reviewed_support_floor
        support_claim = "reviewed_exact_support_formula"
        status = "control_pass"
    elif row["priority"] == "top_exact_coordinate_shortlist":
        sampler_status = "needs_parameterized_sampler_certificate"
        usable_floor = None
        support_claim = "not_claimed"
        status = "needs_sampler_certificate"
    else:
        sampler_status = "not_prioritized_until_kernel_margin_or_semantics_improve"
        usable_floor = None
        support_claim = "not_claimed"
        status = "deprioritized"
    replay = sampler_replay_for(row["candidate"])
    if replay is not None and not replay["public_link_meets_target_bits"]:
        if is_control:
            status = "control_pass_but_public_link_replacement_fails_hiding"
        else:
            sampler_status = "public_link_conditioned_sampler_replay_fails"
            status = "rejected_public_link_hiding"
    return {
        **row,
        **capacity,
        "bounded_kernel_margin_delta_vs_control": margin_delta,
        "reviewed_support_bits_if_current_sampler_applies": (
            reviewed_support_bits if is_control else None
        ),
        "reviewed_support_floor_if_current_sampler_applies": (
            reviewed_support_floor if is_control else None
        ),
        "modeled_support_bits_if_same_packed_sampler_can_be_replayed": (
            reviewed_support_bits if not is_control and row["rank"] == 14 else None
        ),
        "modeled_support_floor_if_same_packed_sampler_can_be_replayed": (
            reviewed_support_floor if not is_control and row["rank"] == 14 else None
        ),
        "sampler_compatibility": sampler_status,
        "usable_hiding_bits_floor": usable_floor,
        "support_or_tv_claim": support_claim,
        "timing_risk": (
            "inherits reviewed no-retry boundary"
            if is_control
            else "unknown until candidate-specific sampler replay"
        ),
        "sampler_replay": replay,
        "status": status,
    }


def build_payload() -> dict[str, Any]:
    kernel = load_json(KERNEL_SEARCH)
    pair_sweep = load_json(PAIR_SWEEP)
    pad_review = load_json(PAD_REVIEW)
    pad_memo = load_json(PAD_MEMO)
    sampler = pad_memo["conditioned_quotient_fiber_sampler"]
    low_bit_class_count = (Q + 3) // 4
    reviewed_support_bits = log2_support_for_formula(
        Q, int(sampler["sampled_free_residues"]), low_bit_class_count
    )
    reviewed_support_floor = math.floor(reviewed_support_bits)
    control = current_control(kernel)
    control_margin = int(control["bounded_kernel_margin"])
    rows = dedupe_rows([control] + exact_coordinate_rows(pair_sweep) + exact_generalized_rows(kernel))
    enriched = [
        enrich_row(row, control_margin, reviewed_support_bits, reviewed_support_floor)
        for row in rows
    ]
    top_shortlist = [
        row
        for row in enriched
        if row["priority"] == "top_exact_coordinate_shortlist"
    ]
    return {
        "schema": "Descender.quotient_level_link.hiding_review.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "firebreak": {
            "touches_deployable_path": False,
            "touches_current_verifier": False,
            "touches_current_proof_path": False,
        },
        "inputs": {
            "kernel_search": rel(KERNEL_SEARCH),
            "coordinate_pair_sweep": rel(PAIR_SWEEP),
            "pad_sampler_boundary_review": rel(PAD_REVIEW),
            "pad_sampler_boundary_memo": rel(PAD_MEMO),
            "sampler_replay_pattern": rel(
                READINESS_DIR / "quotient_level_link_sampler_replay_<candidate>_b96.json"
            ),
        },
        "reviewed_sampler_boundary": {
            "status": pad_review["status"],
            "sampler_name": sampler["sampler_name"],
            "exact_support_formula": sampler["exact_support_formula"],
            "sampled_free_residues": sampler["sampled_free_residues"],
            "low_bit_class_count": low_bit_class_count,
            "support_bits": reviewed_support_bits,
            "support_bits_floor": reviewed_support_floor,
            "target_bits": TARGET_BITS,
            "support_floor_margin_bits": reviewed_support_floor - TARGET_BITS,
            "retry_loop_required": sampler["retry_loop_required"],
            "max_retries": sampler["max_retries"],
            "conditions_allowed": sampler["conditions_allowed"],
            "conditions_forbidden": sampler["conditions_forbidden"],
        },
        "summary": {
            "control_candidate": control["candidate"],
            "control_margin": control_margin,
            "review_rows": len(enriched),
            "top_exact_coordinate_shortlist_rows": len(top_shortlist),
            "top_shortlist_raw_q_residue_bits_floor": sorted(
                {row["raw_q_residue_bits_floor"] for row in top_shortlist}
            ),
            "top_shortlist_sampler_certificates_complete": 0,
            "top_shortlist_public_link_hiding_failures": sum(
                1
                for row in top_shortlist
                if row.get("sampler_replay") is not None
                and not row["sampler_replay"]["public_link_meets_target_bits"]
            ),
            "top_shortlist_status": "rejected_as_public_quotient_links_under_current_sampler_surface",
        },
        "candidates": enriched,
        "interpretation": [
            (
                "The exact-passed top coordinate rows match the current control "
                "on rank, packed link bytes, and raw product-fiber dimension, "
                "while improving the bounded-kernel margin from 43 to 94."
            ),
            (
                "Raw q-residue fiber capacity at rank 14 is only about 122.27 "
                "bits, below the 128-bit target.  The current control relies on "
                "the reviewed packed low-bit support formula to reach a 133-bit "
                "floor."
            ),
            (
                "The first candidate-specific sampler replays reject the top "
                "coordinate rows as public quotient links: once their link rows "
                "condition the representative, support falls below the 128-bit "
                "hiding target."
            ),
        ],
        "next_required_work": [
            "Search for link rows that improve bounded-kernel margin without being public conditioning data, or increase product-fiber dimension before public-link promotion.",
            "Add a candidate family for hidden commitment links where account binding does not reveal link row values.",
            "Only promote a public quotient-level link if its sampler replay keeps at least 128 support-floor bits under public-link conditioning.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    sampler = payload["reviewed_sampler_boundary"]
    summary = payload["summary"]
    lines = [
        "# Quotient-Level Link Hiding Review",
        "",
        "Status: candidate-specific sampler replays reject the current",
        "public-link-conditioned shortlist.",
        "",
        "This sidecar artifact does not change the current q12289 verifier, proof",
        "path, byte target, or implementation.",
        "",
        "## Summary",
        "",
        "| item | value |",
        "|---|---:|",
        f"| control candidate | `{summary['control_candidate']}` |",
        f"| control bounded-kernel margin | `{summary['control_margin']}` |",
        f"| review rows | `{summary['review_rows']}` |",
        f"| top exact coordinate shortlist rows | `{summary['top_exact_coordinate_shortlist_rows']}` |",
        f"| top shortlist sampler certificates complete | `{summary['top_shortlist_sampler_certificates_complete']}` |",
        f"| top shortlist public-link hiding failures | `{summary['top_shortlist_public_link_hiding_failures']}` |",
        f"| top shortlist status | `{summary['top_shortlist_status']}` |",
        "",
        "## Reviewed Sampler Boundary",
        "",
        "| item | value |",
        "|---|---:|",
        f"| sampler | `{sampler['sampler_name']}` |",
        f"| boundary status | `{sampler['status']}` |",
        f"| exact support formula | `{sampler['exact_support_formula']}` |",
        f"| support bits | `{sampler['support_bits']:.6f}` |",
        f"| support floor | `{sampler['support_bits_floor']}` |",
        f"| target bits | `{sampler['target_bits']}` |",
        f"| support floor margin | `{sampler['support_floor_margin_bits']}` |",
        f"| retry loop required | `{sampler['retry_loop_required']}` |",
        "",
        "## Candidate Ledger",
        "",
        "| candidate | priority | bytes | rank | raw bits floor | public-link support floor | kernel margin | delta vs control | sampler | usable floor | status |",
        "|---|---|---:|---:|---:|---:|---:|---:|---|---:|---|",
    ]
    for row in payload["candidates"]:
        usable = "-" if row["usable_hiding_bits_floor"] is None else str(row["usable_hiding_bits_floor"])
        replay_floor = "-"
        if row["sampler_replay"] is not None:
            replay_floor = str(row["sampler_replay"]["public_link_support_floor_bits"])
        delta = (
            "-"
            if row["bounded_kernel_margin_delta_vs_control"] is None
            else str(row["bounded_kernel_margin_delta_vs_control"])
        )
        lines.append(
            f"| {row['candidate']} | {row['priority']} | {row['packed_bytes']} | "
            f"{row['rank']} | {row['raw_q_residue_bits_floor']} | "
            f"{replay_floor} | "
            f"{row['bounded_kernel_margin']} | {delta} | "
            f"{row['sampler_compatibility']} | {usable} | {row['status']} |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
        ]
    )
    for item in payload["interpretation"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Next Required Work",
            "",
        ]
    )
    for item in payload["next_required_work"]:
        lines.append(f"- {item}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "review_rows": payload["summary"]["review_rows"],
                "top_shortlist_status": payload["summary"]["top_shortlist_status"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
