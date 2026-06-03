#!/usr/bin/env python3
"""Generate a transparent-proof byte model for the Path 2 private relation."""

from __future__ import annotations

import json
from math import ceil, log2
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_transparent_proof_byte_model.json"
OUT_MD = ROOT / "research/path2_transparent_proof_byte_model.md"

HASH_BYTES = 32
FIELD_BITS = 14
WITNESS_LANES = 80
MASKED_WITNESS_LANE_FLOOR = 138
RANGE_ACCOUNT_WITNESS_LANE_FLOOR = 194
RELATION_ROWS = 66


def packed_q_bytes(lanes: int) -> int:
    return ceil(lanes * FIELD_BITS / 8)


def merkle_path_bytes(trace_rows: int, queries: int) -> int:
    return int(log2(trace_rows)) * HASH_BYTES * queries


def build_model() -> dict[str, Any]:
    rows = []
    for trace_rows, queries in ((128, 12), (256, 16), (512, 20), (1024, 24)):
        trace_root_bytes = 2 * HASH_BYTES
        opened_values = queries * packed_q_bytes(8)
        masked_opened_values = queries * packed_q_bytes(12)
        range_account_opened_values = queries * packed_q_bytes(16)
        paths = merkle_path_bytes(trace_rows, queries)
        fri_or_consistency = queries * HASH_BYTES
        masked_consistency = 2 * queries * HASH_BYTES
        range_account_consistency = 3 * queries * HASH_BYTES
        proof_floor = trace_root_bytes + opened_values + paths + fri_or_consistency
        masked_proof_floor = (
            trace_root_bytes + masked_opened_values + paths + masked_consistency
        )
        range_account_proof_floor = (
            trace_root_bytes
            + range_account_opened_values
            + paths
            + range_account_consistency
        )
        rows.append(
            {
                "trace_rows": trace_rows,
                "queries": queries,
                "trace_root_bytes": trace_root_bytes,
                "opened_value_bytes_floor": opened_values,
                "masked_opened_value_bytes_floor": masked_opened_values,
                "range_account_opened_value_bytes_floor": range_account_opened_values,
                "merkle_path_bytes": paths,
                "fri_or_consistency_bytes_floor": fri_or_consistency,
                "masked_consistency_bytes_floor": masked_consistency,
                "range_account_consistency_bytes_floor": range_account_consistency,
                "proof_bytes_floor": proof_floor,
                "masked_proof_bytes_floor": masked_proof_floor,
                "range_account_proof_bytes_floor": range_account_proof_floor,
                "status": (
                    "low_security_planning_row"
                    if queries < 16
                    else "planning_comparator"
                ),
            }
        )
    return {
        "schema": "Descender.path2_transparent_proof_byte_model.v1",
        "status": "transparent_planning_model_pinned_not_deployable",
        "relation": {
            "witness_lanes": WITNESS_LANES,
            "masked_witness_lane_floor": MASKED_WITNESS_LANE_FLOOR,
            "range_account_witness_lane_floor": RANGE_ACCOUNT_WITNESS_LANE_FLOOR,
            "relation_rows": RELATION_ROWS,
            "witness_bytes_if_opened": packed_q_bytes(WITNESS_LANES),
            "masked_witness_bytes_if_opened": packed_q_bytes(
                MASKED_WITNESS_LANE_FLOOR
            ),
            "range_account_witness_bytes_if_opened": packed_q_bytes(
                RANGE_ACCOUNT_WITNESS_LANE_FLOOR
            ),
            "description": (
                "R_core_66 remains the semantic extractor relation, but the "
                "active transparent candidate now requires a masked query view. "
                "The 80-lane model is a lower-bound comparator; 138 lanes is the "
                "masking floor, and 194 lanes is the current conservative floor "
                "after range-ZK and account-binding deltas."
            ),
        },
        "masked_transform_source": "research/path2_masked_trace_transform.md",
        "range_account_delta_source": "research/path2_masked_trace_range_account_delta.md",
        "planning_rows": rows,
        "comparison_read": (
            "The old transparent planning rows remain useful only as lower-bound "
            "comparators. The range/account floor is still below the 16416-byte "
            "lattice-sigma repetition floor in these rows, but the margin is "
            "smaller and the estimate still excludes a concrete range gadget and "
            "real FRI/IOP security analysis."
        ),
        "next_decision": (
            "Use the 194-lane floor as the current transparent candidate for "
            "proof planning. The next proof move is not verifier implementation; "
            "it is to formalize the zkMaskedTraceIOPBindingAndHiding assumption "
            "and set a fallback threshold if concrete range/FRI overhead pushes "
            "the payload near the lattice-sigma floor."
        ),
        "non_claims": [
            "this is not a concrete STARK/IOP implementation estimate",
            "query counts are planning rows, not security parameters",
            "FRI/constraint composition bytes are lower-bound placeholders",
            "masked rows are still not a concrete STARK/IOP estimate",
            "this does not close QROM, no-export, or extractor obligations",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    relation = model["relation"]
    lines = [
        "# Path 2 Transparent Proof Byte Model",
        "",
        "Status: transparent planning model pinned; not deployable.",
        "",
        "## Relation",
        "",
        f"- witness lanes: `{relation['witness_lanes']}`",
        f"- masked witness lane floor: `{relation['masked_witness_lane_floor']}`",
        f"- range/account witness lane floor: `{relation['range_account_witness_lane_floor']}`",
        f"- relation rows: `{relation['relation_rows']}`",
        f"- witness bytes if opened: `{relation['witness_bytes_if_opened']}`",
        f"- masked witness bytes if opened: `{relation['masked_witness_bytes_if_opened']}`",
        f"- range/account witness bytes if opened: `{relation['range_account_witness_bytes_if_opened']}`",
        f"- description: {relation['description']}",
        f"- masked transform source: `{model['masked_transform_source']}`",
        f"- range/account delta source: `{model['range_account_delta_source']}`",
        "",
        "## Planning Rows",
        "",
        "| trace rows | queries | roots | old opened | masked opened | range/account opened | Merkle paths | old consistency | masked consistency | range/account consistency | old floor | masked floor | range/account floor | status |",
        "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|",
    ]
    for item in model["planning_rows"]:
        lines.append(
            "| {trace_rows} | {queries} | {trace_root_bytes} | "
            "{opened_value_bytes_floor} | {masked_opened_value_bytes_floor} | "
            "{range_account_opened_value_bytes_floor} | {merkle_path_bytes} | "
            "{fri_or_consistency_bytes_floor} | "
            "{masked_consistency_bytes_floor} | "
            "{range_account_consistency_bytes_floor} | {proof_bytes_floor} | "
            "{masked_proof_bytes_floor} | {range_account_proof_bytes_floor} | "
            "{status} |".format(**item)
        )
    lines.extend(
        [
            "",
            "## Comparison Read",
            "",
            model["comparison_read"],
            "",
            "## Next Decision",
            "",
            model["next_decision"],
            "",
            "## Non-Claims",
            "",
        ]
    )
    for item in model["non_claims"]:
        lines.append(f"- {item}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    model = build_model()
    OUT_JSON.write_text(json.dumps(model, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(write_markdown(model))
    print(json.dumps({"json": str(OUT_JSON), "markdown": str(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
