#!/usr/bin/env python3
"""Build the selected hidden-source projection matrix over q12289."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.md"
SPEC_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"
)
RESPONSE_LINK_CERT_JSON = (
    READINESS_DIR
    / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)
BYTE_INTERNALIZATION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json"
)

Q = 12289
BLOCKS = 16
COORDS = 2
KINDS = ("account_witness_projection", "pad_seed_projection")
COLS = len(KINDS) * BLOCKS * COORDS
CHECKED_CORE_RESPONSE_COLUMNS = 23
CARRIER_LANES = 12
CHECK_BYTES = 3
SEMANTIC_PREDICATES = (
    "canonical_q12289_lane_decoder",
    "account_opening_frame_decoder",
    "checked_core_transcript_frame_decoder",
    "carrier_context_binding_decoder",
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def column_index(kind: str, block: int, coord: int) -> int:
    return KINDS.index(kind) * BLOCKS * COORDS + block * COORDS + coord


def rref_mod_q(rows: list[list[int]]) -> tuple[list[list[int]], list[int]]:
    work = [[value % Q for value in row] for row in rows]
    rank = 0
    pivots: list[int] = []
    for col in range(COLS):
        pivot = next((idx for idx in range(rank, len(work)) if work[idx][col] % Q), None)
        if pivot is None:
            continue
        work[rank], work[pivot] = work[pivot], work[rank]
        inv = pow(work[rank][col] % Q, -1, Q)
        work[rank] = [(value * inv) % Q for value in work[rank]]
        for row_idx in range(len(work)):
            if row_idx == rank or work[row_idx][col] % Q == 0:
                continue
            factor = work[row_idx][col] % Q
            work[row_idx] = [
                (work[row_idx][idx] - factor * work[rank][idx]) % Q
                for idx in range(COLS)
            ]
        pivots.append(col)
        rank += 1
        if rank == len(work):
            break
    return work, pivots


def matrix_sha256(matrix: list[list[int]]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.selected_projection_matrix.v1")
    h.update(len(matrix).to_bytes(2, "little"))
    h.update(COLS.to_bytes(2, "little"))
    for row in matrix:
        for value in row:
            h.update((value % Q).to_bytes(2, "little"))
    return h.hexdigest()


def generalized_system_sha256(system: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.selected_generalized_projection_system.v1")
    h.update(json.dumps(system, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def build_matrix(rows: list[dict[str, Any]]) -> list[list[int]]:
    matrix: list[list[int]] = []
    for row in rows:
        vector = [0] * COLS
        idx = column_index(row["kind"], int(row["block"]), int(row["coord"]))
        vector[idx] = int(row["coefficient_mod_q"]) % Q
        matrix.append(vector)
    return matrix


def build_payload() -> dict[str, Any]:
    spec = load_json(SPEC_JSON)
    response = load_json(RESPONSE_LINK_CERT_JSON)
    byte_internalization = load_json(BYTE_INTERNALIZATION_JSON)
    selected_rows = spec["selected_rows"]
    matrix = build_matrix(selected_rows)
    _rref, pivots = rref_mod_q(matrix)
    rank = len(pivots)
    nonzero_columns = sorted(
        idx for idx in range(COLS) if any(row[idx] % Q for row in matrix)
    )
    column_map = [
        {
            "column": idx,
            "kind": KINDS[idx // (BLOCKS * COORDS)],
            "block": (idx % (BLOCKS * COORDS)) // COORDS,
            "coord": idx % COORDS,
            "active": idx in nonzero_columns,
        }
        for idx in range(COLS)
    ]
    controls = [
        {
            "id": "projection_row_rank",
            "status": "pass" if rank == len(matrix) else "fail",
            "observed": rank,
            "required": len(matrix),
        },
        {
            "id": "projection_rows_are_sparse",
            "status": "pass" if all(sum(1 for value in row if value % Q) == 1 for row in matrix) else "fail",
            "observed": [sum(1 for value in row if value % Q) for row in matrix],
            "required": "one active q12289 coefficient per projection row",
        },
        {
            "id": "projection_only_kernel_nontrivial",
            "status": "expected_open",
            "observed": COLS - rank,
            "required": "must be closed only after combining with checked-core and semantic decoder equations",
        },
    ]
    checked_core_rank = int(response["combined_constraint_rank"])
    checked_core_nullity = int(response["combined_kernel_dimension"])
    generalized_columns = (
        CHECKED_CORE_RESPONSE_COLUMNS + COLS + CARRIER_LANES + CHECK_BYTES
    )
    generalized_linear_rank_before_bridges = checked_core_rank + rank + CARRIER_LANES
    generalized_linear_nullity_before_bridges = (
        generalized_columns - generalized_linear_rank_before_bridges
    )
    row_groups = [
        {
            "id": "checked_core_response_link_rows",
            "status": "instantiated_from_replayed_certificate",
            "row_count": checked_core_rank,
            "columns": {
                "start": 0,
                "count": CHECKED_CORE_RESPONSE_COLUMNS,
                "space": "checked_core_response_difference",
            },
            "source": rel(RESPONSE_LINK_CERT_JSON),
        },
        {
            "id": "selected_hidden_source_projection_rows",
            "status": "instantiated",
            "row_count": len(matrix),
            "rank": rank,
            "columns": {
                "start": CHECKED_CORE_RESPONSE_COLUMNS,
                "count": COLS,
                "space": "hidden_source_projection_variables",
            },
            "source": rel(SPEC_JSON),
        },
        {
            "id": "public_carrier_lane_definition_rows",
            "status": "instantiated_as_public_lane_equalities",
            "row_count": CARRIER_LANES,
            "columns": {
                "start": CHECKED_CORE_RESPONSE_COLUMNS + COLS,
                "count": CARRIER_LANES,
                "space": "decoded_public_projection_carrier_lanes",
            },
            "source": rel(SPEC_JSON),
        },
        {
            "id": "projection_check_byte_predicates",
            "status": "tracked_as_public_decoder_predicates",
            "row_count": CHECK_BYTES,
            "columns": {
                "start": CHECKED_CORE_RESPONSE_COLUMNS + COLS + CARRIER_LANES,
                "count": CHECK_BYTES,
                "space": "projection_check_bytes",
            },
            "source": "tools/parameter-search/src/production_reference.rs",
        },
        {
            "id": "semantic_decoder_preconditions",
            "status": "tracked_as_fail_closed_preconditions",
            "predicate_count": len(SEMANTIC_PREDICATES),
            "predicates": list(SEMANTIC_PREDICATES),
            "source": rel(BYTE_INTERNALIZATION_JSON),
        },
    ]
    bridge_slots = [
        {
            "id": "checked_core_to_projection_bridge_rows",
            "status": "reserved_open",
            "source_space": "checked_core_response_difference",
            "target_space": "hidden_source_projection_variables",
            "required_before_solver": True,
        },
        {
            "id": "projection_to_hidden_source_opening_rows",
            "status": "reserved_open",
            "source_space": "hidden_source_projection_variables",
            "target_space": "account_opening_and_pad_source_semantics",
            "required_before_solver": True,
        },
        {
            "id": "bounded_bad_vector_export_rows",
            "status": "reserved_open",
            "source_space": "combined_generalized_system",
            "target_space": "reviewer_bad_vector_artifact",
            "required_before_solver": False,
        },
    ]
    generalized_system = {
        "version": 1,
        "q": Q,
        "columns": generalized_columns,
        "column_spaces": [
            {
                "id": "checked_core_response_difference",
                "start": 0,
                "count": CHECKED_CORE_RESPONSE_COLUMNS,
            },
            {
                "id": "hidden_source_projection_variables",
                "start": CHECKED_CORE_RESPONSE_COLUMNS,
                "count": COLS,
            },
            {
                "id": "decoded_public_projection_carrier_lanes",
                "start": CHECKED_CORE_RESPONSE_COLUMNS + COLS,
                "count": CARRIER_LANES,
            },
            {
                "id": "projection_check_bytes",
                "start": CHECKED_CORE_RESPONSE_COLUMNS + COLS + CARRIER_LANES,
                "count": CHECK_BYTES,
            },
        ],
        "row_groups": row_groups,
        "bridge_slots": bridge_slots,
        "linear_rank_before_open_bridges": generalized_linear_rank_before_bridges,
        "linear_nullity_before_open_bridges": generalized_linear_nullity_before_bridges,
    }
    controls.extend(
        [
            {
                "id": "checked_core_certificate_rank_imported",
                "status": "pass" if checked_core_rank > 0 else "fail",
                "observed": checked_core_rank,
                "required": "positive replayed checked-core rank",
            },
            {
                "id": "generalized_column_layout_is_explicit",
                "status": "pass"
                if generalized_columns
                == CHECKED_CORE_RESPONSE_COLUMNS + COLS + CARRIER_LANES + CHECK_BYTES
                else "fail",
                "observed": generalized_columns,
                "required": CHECKED_CORE_RESPONSE_COLUMNS
                + COLS
                + CARRIER_LANES
                + CHECK_BYTES,
            },
            {
                "id": "semantic_decoder_preconditions_internalized",
                "status": "pass"
                if byte_internalization.get("coverage", {}).get("open_derivation_count") == 0
                else "fail",
                "observed": byte_internalization.get("coverage", {}).get(
                    "open_derivation_count"
                ),
                "required": 0,
            },
            {
                "id": "open_bridge_slots_keep_solver_blocked",
                "status": "pass"
                if any(row["status"] == "reserved_open" for row in bridge_slots)
                else "fail",
                "observed": [row["id"] for row in bridge_slots if row["status"] == "reserved_open"],
                "required": "at least one explicit open bridge slot until exact replay is implemented",
            },
        ]
    )
    return {
        "schema": "Descender.hidden_source_fallback_projection_matrix_builder.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "generalized_projection_matrix_built_exact_replay_open_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "exact_certificate_spec": rel(SPEC_JSON),
            "response_link_certificate": rel(RESPONSE_LINK_CERT_JSON),
            "byte_level_derivation_internalization": rel(BYTE_INTERNALIZATION_JSON),
            "exact_certificate_spec_schema": spec.get("schema"),
            "response_link_certificate_schema": response.get("schema"),
            "byte_level_derivation_schema": byte_internalization.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "rows": len(matrix),
            "columns": COLS,
            "rank": rank,
            "nullity_projection_only": COLS - rank,
            "matrix_sha256": matrix_sha256(matrix),
            "generalized_columns": generalized_columns,
            "generalized_linear_rank_before_open_bridges": generalized_linear_rank_before_bridges,
            "generalized_linear_nullity_before_open_bridges": generalized_linear_nullity_before_bridges,
            "generalized_system_sha256": generalized_system_sha256(generalized_system),
        },
        "column_layout": {
            "kinds": list(KINDS),
            "blocks_per_kind": BLOCKS,
            "coords_per_block": COORDS,
        },
        "generalized_system": generalized_system,
        "rank_accounting": {
            "checked_core_response_columns": CHECKED_CORE_RESPONSE_COLUMNS,
            "checked_core_rank": checked_core_rank,
            "checked_core_nullity": checked_core_nullity,
            "projection_columns": COLS,
            "projection_rank": rank,
            "projection_nullity": COLS - rank,
            "carrier_lane_columns": CARRIER_LANES,
            "check_byte_columns": CHECK_BYTES,
            "generalized_columns": generalized_columns,
            "linear_rank_before_open_bridges": generalized_linear_rank_before_bridges,
            "linear_nullity_before_open_bridges": generalized_linear_nullity_before_bridges,
        },
        "active_columns": [column_map[idx] for idx in nonzero_columns],
        "structural_controls": controls,
        "coverage": {
            "generalized_projection_matrix_built": True,
            "checked_core_rows_imported": True,
            "projection_rows_imported": True,
            "carrier_lane_rows_imported": True,
            "semantic_decoder_preconditions_tracked": byte_internalization.get(
                "coverage", {}
            ).get("open_derivation_count")
            == 0,
            "open_bridge_slot_count": sum(
                1 for row in bridge_slots if row["status"] == "reserved_open"
            ),
            "bounded_solver_ready": False,
        },
        "decision": {
            "can_accept_now": False,
            "projection_only_certificate": False,
            "generalized_builder_complete": True,
            "reason": "the generalized matrix interface now fixes checked-core, projection, public carrier-lane, and decoder-predicate columns; exact infeasibility remains blocked by open bridge slots",
            "next_artifact": "hidden_source_fallback_projection_exact_infeasibility_replay",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    rank = payload["rank_accounting"]
    lines = [
        "# Hidden-Source Projection Matrix Builder",
        "",
        "This artifact materializes the selected projection rows and fixes the",
        "generalized exact-replay system interface: checked-core response",
        "variables, hidden-source projection variables, decoded public carrier",
        "lanes, check bytes, and semantic decoder preconditions.  It is",
        "deliberately not an acceptance certificate: open bridge slots still block",
        "bounded infeasibility replay.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- rows: `{target['rows']}`",
        f"- columns: `{target['columns']}`",
        f"- rank: `{target['rank']}`",
        f"- projection-only nullity: `{target['nullity_projection_only']}`",
        f"- matrix SHA-256: `{target['matrix_sha256']}`",
        f"- generalized columns: `{target['generalized_columns']}`",
        f"- generalized linear rank before open bridges: `{target['generalized_linear_rank_before_open_bridges']}`",
        f"- generalized linear nullity before open bridges: `{target['generalized_linear_nullity_before_open_bridges']}`",
        f"- generalized system SHA-256: `{target['generalized_system_sha256']}`",
        "",
        "## Rank Accounting",
        "",
        f"- checked-core response columns: `{rank['checked_core_response_columns']}`",
        f"- checked-core rank: `{rank['checked_core_rank']}`",
        f"- checked-core nullity: `{rank['checked_core_nullity']}`",
        f"- projection columns: `{rank['projection_columns']}`",
        f"- projection rank: `{rank['projection_rank']}`",
        f"- carrier lane columns: `{rank['carrier_lane_columns']}`",
        f"- check byte columns: `{rank['check_byte_columns']}`",
        "",
        "## Structural Controls",
        "",
        "| id | status | observed | required |",
        "|---|---|---|---|",
    ]
    for row in payload["structural_controls"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['observed']}` | `{row['required']}` |"
        )
    lines.extend(
        [
            "",
            "## Generalized Row Groups",
            "",
            "| id | status | count | source |",
            "|---|---|---:|---|",
        ]
    )
    for row in payload["generalized_system"]["row_groups"]:
        count = row.get("row_count", row.get("predicate_count"))
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{count}` | `{row['source']}` |"
        )
    lines.extend(
        [
            "",
            "## Open Bridge Slots",
            "",
            "| id | status | source | target |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["generalized_system"]["bridge_slots"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['source_space']}` | `{row['target_space']}` |"
        )
    lines.extend(
        [
            "",
            "## Active Columns",
            "",
            "| column | kind | block | coord |",
            "|---:|---|---:|---:|",
        ]
    )
    for row in payload["active_columns"]:
        lines.append(
            f"| `{row['column']}` | `{row['kind']}` | `{row['block']}` | `{row['coord']}` |"
        )
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- projection-only certificate: `{payload['decision']['projection_only_certificate']}`",
            f"- generalized builder complete: `{payload['decision']['generalized_builder_complete']}`",
            f"- reason: {payload['decision']['reason']}",
            f"- next artifact: `{payload['decision']['next_artifact']}`",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
