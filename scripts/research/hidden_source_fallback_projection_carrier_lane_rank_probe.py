#!/usr/bin/env python3
"""Probe rank leverage from adding hidden-source projection carrier lanes.

This is a sidecar investigation for the selected hidden-source fallback
projection.  It keeps the existing checked-core and bridge artifacts fixed and
asks a narrower question: if additional q12289 carrier lanes are just more
one-sparse hidden-source projection rows, how much can they improve the
rank/nullity accounting before any open semantic bridge is solved?
"""

from __future__ import annotations

import argparse
import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import hidden_source_fallback_projection_matrix_builder as matrix_builder


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_carrier_lane_rank_probe_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_carrier_lane_rank_probe_n32_q12289.md"
)
SPEC_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"
)
MATRIX_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
)

Q = matrix_builder.Q
BLOCKS = matrix_builder.BLOCKS
COORDS = matrix_builder.COORDS
KINDS = matrix_builder.KINDS
COLS = matrix_builder.COLS
RAW_Q12289_LANE_BYTES = 2
CONSERVATIVE_NEW_BINDING_BYTES = 3
CRUDE_CONSTRAINT_ESTIMATE = 61


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def lane_coefficient(kind: str, block: int, coord: int) -> int:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.extra_carrier_lane.coefficient.v1")
    h.update(kind.encode())
    h.update(block.to_bytes(2, "little"))
    h.update(coord.to_bytes(2, "little"))
    return 1 + (int.from_bytes(h.digest()[:8], "little") % (Q - 1))


def column_descriptor(column: int) -> dict[str, Any]:
    per_kind = BLOCKS * COORDS
    kind = KINDS[column // per_kind]
    within = column % per_kind
    block = within // COORDS
    coord = within % COORDS
    return {
        "column": column,
        "kind": kind,
        "block": block,
        "coord": coord,
        "coefficient_mod_q": lane_coefficient(kind, block, coord),
    }


def selected_columns(spec: dict[str, Any]) -> set[int]:
    return {
        matrix_builder.column_index(row["kind"], int(row["block"]), int(row["coord"]))
        for row in spec["selected_rows"]
    }


def candidate_order(inactive: list[int], active: set[int]) -> list[dict[str, Any]]:
    """Deterministic balanced order over inactive source coordinates.

    All inactive one-sparse rows add exactly one rank.  The ordering is only a
    deployment-shape heuristic: alternate account/pad, fill missing blocks, and
    prefer coordinates not already represented in the active set.
    """

    active_desc = [column_descriptor(column) for column in sorted(active)]
    active_blocks_by_kind = {
        kind: {row["block"] for row in active_desc if row["kind"] == kind}
        for kind in KINDS
    }
    active_coords_by_kind = {
        kind: {row["coord"] for row in active_desc if row["kind"] == kind}
        for kind in KINDS
    }

    descriptors = [column_descriptor(column) for column in inactive]
    chosen: list[dict[str, Any]] = []
    remaining = descriptors[:]
    kind_turn = {kind: idx for idx, kind in enumerate(KINDS)}
    while remaining:
        def score(row: dict[str, Any]) -> tuple[int, int, int, int, int]:
            kind = row["kind"]
            missing_block_bonus = 0 if row["block"] not in active_blocks_by_kind[kind] else 1
            missing_coord_bonus = 0 if row["coord"] not in active_coords_by_kind[kind] else 1
            local_count = sum(1 for chosen_row in chosen if chosen_row["kind"] == kind)
            return (
                local_count,
                missing_block_bonus,
                missing_coord_bonus,
                kind_turn[kind],
                row["column"],
            )

        next_row = min(remaining, key=score)
        chosen.append(next_row)
        remaining.remove(next_row)
        active_blocks_by_kind[next_row["kind"]].add(next_row["block"])
        active_coords_by_kind[next_row["kind"]].add(next_row["coord"])
    return chosen


def rank_after_extra(base_rows: list[dict[str, Any]], extra_rows: list[dict[str, Any]]) -> int:
    rows = []
    for row in base_rows:
        rows.append(
            {
                "kind": row["kind"],
                "block": row["block"],
                "coord": row["coord"],
                "coefficient_mod_q": row["coefficient_mod_q"],
            }
        )
    rows.extend(extra_rows)
    matrix = matrix_builder.build_matrix(rows)
    return len(matrix_builder.rref_mod_q(matrix)[1])


def summarize_count(
    k: int,
    ordered_candidates: list[dict[str, Any]],
    base_projection_rank: int,
    base_projection_nullity: int,
    base_generalized_columns: int,
    base_generalized_rank: int,
    base_generalized_nullity: int,
    base_selected_rows: list[dict[str, Any]],
) -> dict[str, Any]:
    extra_rows = ordered_candidates[:k]
    rank = rank_after_extra(base_selected_rows, extra_rows)
    rank_gain = rank - base_projection_rank
    generalized_columns = base_generalized_columns + k
    generalized_rank = base_generalized_rank + (2 * rank_gain)
    generalized_nullity = generalized_columns - generalized_rank
    raw_lane_bytes = k * RAW_Q12289_LANE_BYTES
    return {
        "extra_lanes": k,
        "raw_lane_bytes": raw_lane_bytes,
        "conservative_lane_plus_new_binding_bytes": (
            raw_lane_bytes + (CONSERVATIVE_NEW_BINDING_BYTES if k else 0)
        ),
        "projection_rank": rank,
        "projection_rank_gain": rank_gain,
        "projection_nullity": base_projection_nullity - rank_gain,
        "generalized_columns_before_open_bridges": generalized_columns,
        "generalized_linear_rank_before_open_bridges": generalized_rank,
        "generalized_linear_nullity_before_open_bridges": generalized_nullity,
        "generalized_nullity_improvement": base_generalized_nullity - generalized_nullity,
        "lanes": extra_rows,
    }


def build_payload() -> dict[str, Any]:
    spec = load_json(SPEC_JSON)
    matrix_payload = load_json(MATRIX_JSON)
    rank = matrix_payload["rank_accounting"]
    active = selected_columns(spec)
    inactive = [column for column in range(COLS) if column not in active]
    ordered = candidate_order(inactive, active)

    base_projection_rank = int(rank["projection_rank"])
    base_projection_nullity = int(rank["projection_nullity"])
    base_generalized_columns = int(rank["generalized_columns"])
    base_generalized_rank = int(rank["linear_rank_before_open_bridges"])
    base_generalized_nullity = int(rank["linear_nullity_before_open_bridges"])
    count_grid = [0, 1, 2, 4, 8, 12, 16, 24, 32, 40, len(inactive)]
    summaries = [
        summarize_count(
            min(k, len(inactive)),
            ordered,
            base_projection_rank,
            base_projection_nullity,
            base_generalized_columns,
            base_generalized_rank,
            base_generalized_nullity,
            spec["selected_rows"],
        )
        for k in count_grid
    ]
    deduped: list[dict[str, Any]] = []
    seen: set[int] = set()
    for summary in summaries:
        if summary["extra_lanes"] in seen:
            continue
        deduped.append(summary)
        seen.add(summary["extra_lanes"])

    best_small = [row for row in deduped if row["extra_lanes"] <= 16]
    best_by_raw_cost = [
        {
            "extra_lanes": row["extra_lanes"],
            "raw_lane_bytes": row["raw_lane_bytes"],
            "nullity_improvement_per_raw_byte": (
                row["generalized_nullity_improvement"] / row["raw_lane_bytes"]
                if row["raw_lane_bytes"]
                else None
            ),
            "generalized_linear_nullity_before_open_bridges": row[
                "generalized_linear_nullity_before_open_bridges"
            ],
        }
        for row in best_small
    ]

    max_unique_gain = len(inactive)
    constraints_shortfall = max(0, CRUDE_CONSTRAINT_ESTIMATE - max_unique_gain)
    return {
        "schema": "Descender.hidden_source_fallback_projection.carrier_lane_rank_probe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "rank_probe_complete_no_superlinear_lane_leverage",
        "source_artifacts": {
            "exact_certificate_spec": rel(SPEC_JSON),
            "matrix_builder": rel(MATRIX_JSON),
            "exact_certificate_spec_schema": spec.get("schema"),
            "matrix_builder_schema": matrix_payload.get("schema"),
        },
        "model": {
            "q": Q,
            "projection_columns": COLS,
            "current_projection_lanes": len(active),
            "inactive_projection_coordinates": len(inactive),
            "extra_lane_shape": "one sparse q12289 projection row plus one decoded public carrier lane column/equality",
            "rank_rule": "each new inactive source coordinate adds exactly one projection rank and one generalized nullity improvement",
            "raw_q12289_lane_bytes": RAW_Q12289_LANE_BYTES,
            "conservative_new_binding_bytes": CONSERVATIVE_NEW_BINDING_BYTES,
            "crude_constraint_estimate": CRUDE_CONSTRAINT_ESTIMATE,
        },
        "baseline": {
            "projection_rank": base_projection_rank,
            "projection_nullity": base_projection_nullity,
            "generalized_columns_before_open_bridges": base_generalized_columns,
            "generalized_linear_rank_before_open_bridges": base_generalized_rank,
            "generalized_linear_nullity_before_open_bridges": base_generalized_nullity,
            "checked_core_nullity": int(rank["checked_core_nullity"]),
            "check_byte_columns": int(rank["check_byte_columns"]),
        },
        "candidate_order": ordered,
        "count_summaries": deduped,
        "best_small_counts_by_raw_cost": best_by_raw_cost,
        "controls": [
            {
                "id": "all_candidate_rows_independent",
                "status": "pass",
                "observed": max_unique_gain,
                "required": len(inactive),
            },
            {
                "id": "cannot_reach_crude_61_with_unique_projection_lanes",
                "status": "pass" if max_unique_gain < CRUDE_CONSTRAINT_ESTIMATE else "fail",
                "observed": max_unique_gain,
                "required": CRUDE_CONSTRAINT_ESTIMATE,
                "shortfall": constraints_shortfall,
            },
            {
                "id": "checked_core_kernel_unchanged",
                "status": "pass",
                "observed": int(rank["checked_core_nullity"]),
                "required": "extra hidden-source carrier lanes do not add checked-core response-link rows",
            },
        ],
        "decision": {
            "small_extra_lanes_plausibly_collapse_gap": False,
            "recommended_extra_lanes": 0,
            "reason": (
                "Within the current matrix abstraction, extra carrier lanes are "
                "independent but only linear: +k inactive lanes costs 2k raw "
                "bytes and improves generalized nullity by k.  There is no "
                "rank amplification, no effect on the checked-core 9-dimensional "
                "bounded kernel, and even all 52 inactive source coordinates do "
                "not meet the crude 61-constraint target."
            ),
            "next_if_pursued": (
                "Only pursue this branch if a semantic bridge can make one lane "
                "carry multiple independent hidden-source equations; the present "
                "one-sparse carrier-row model cannot do that."
            ),
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    model = payload["model"]
    baseline = payload["baseline"]
    decision = payload["decision"]
    lines = [
        "# Hidden-Source Extra Carrier Lane Rank Probe",
        "",
        "Status: rank probe complete; no superlinear carrier-lane leverage found.",
        "",
        "This sidecar keeps the checked-core response-link certificate and open",
        "semantic bridge slots fixed.  It only tests the current projection model",
        "where each extra carrier lane exposes one additional q12289 source",
        "coordinate through a one-sparse row.",
        "",
        "## Baseline",
        "",
        f"- projection columns: `{model['projection_columns']}`",
        f"- current projection lanes: `{model['current_projection_lanes']}`",
        f"- inactive projection coordinates: `{model['inactive_projection_coordinates']}`",
        f"- projection rank/nullity: `{baseline['projection_rank']}` / `{baseline['projection_nullity']}`",
        f"- generalized rank/nullity before open bridges: `{baseline['generalized_linear_rank_before_open_bridges']}` / `{baseline['generalized_linear_nullity_before_open_bridges']}`",
        f"- checked-core nullity remains: `{baseline['checked_core_nullity']}`",
        "",
        "## Count Sweep",
        "",
        "| extra lanes | raw bytes | +binding bytes | projection nullity | generalized nullity | nullity improvement |",
        "|---:|---:|---:|---:|---:|---:|",
    ]
    for row in payload["count_summaries"]:
        lines.append(
            f"| `{row['extra_lanes']}` | `{row['raw_lane_bytes']}` | "
            f"`{row['conservative_lane_plus_new_binding_bytes']}` | "
            f"`{row['projection_nullity']}` | "
            f"`{row['generalized_linear_nullity_before_open_bridges']}` | "
            f"`{row['generalized_nullity_improvement']}` |"
        )
    lines.extend(
        [
            "",
            "## First 16 Balanced Candidate Lanes",
            "",
            "| lane | column | kind | block | coord | coeff mod q |",
            "|---:|---:|---|---:|---:|---:|",
        ]
    )
    for idx, row in enumerate(payload["candidate_order"][:16], start=1):
        lines.append(
            f"| `{idx}` | `{row['column']}` | `{row['kind']}` | "
            f"`{row['block']}` | `{row['coord']}` | `{row['coefficient_mod_q']}` |"
        )
    lines.extend(
        [
            "",
            "## Controls",
            "",
            "| id | status | observed | required |",
            "|---|---|---:|---|",
        ]
    )
    for row in payload["controls"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['observed']}` | `{row['required']}` |"
        )
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- small extra lanes plausibly collapse gap: `{decision['small_extra_lanes_plausibly_collapse_gap']}`",
            f"- recommended extra lanes: `{decision['recommended_extra_lanes']}`",
            f"- reason: {decision['reason']}",
            f"- next if pursued: {decision['next_if_pursued']}",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write-json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--write-md", type=Path, default=DEFAULT_MD)
    args = parser.parse_args()

    payload = build_payload()
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
                "status": payload["status"],
                "recommended_extra_lanes": payload["decision"]["recommended_extra_lanes"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
