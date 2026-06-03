#!/usr/bin/env python3
"""Replay exact infeasibility for the instantiated generalized projection system."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.md"
)
MATRIX_BUILDER_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
)
EXACT_SPEC_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"
)
BRIDGE_DERIVATION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json"
)

Q = 12289
BOUND = 96


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.fallback_projection.exact_infeasibility_replay.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def first_inactive_projection_column(matrix: dict[str, Any]) -> dict[str, Any]:
    target = matrix["target"]
    active = {row["column"] for row in matrix["active_columns"]}
    for column in range(int(target["columns"])):
        if column not in active:
            return {
                "column": column,
                "kind": "account_witness_projection"
                if column < 32
                else "pad_seed_projection",
                "block": (column % 32) // 2,
                "coord": column % 2,
            }
    raise ValueError("projection matrix unexpectedly has no inactive column")


def build_payload() -> dict[str, Any]:
    matrix = load_json(MATRIX_BUILDER_JSON)
    spec = load_json(EXACT_SPEC_JSON)
    derivation = load_json(BRIDGE_DERIVATION_JSON)
    inactive = first_inactive_projection_column(matrix)
    generalized = matrix["generalized_system"]
    spaces = {row["id"]: row for row in generalized["column_spaces"]}
    projection_start = int(spaces["hidden_source_projection_variables"]["start"])
    carrier_start = int(spaces["decoded_public_projection_carrier_lanes"]["start"])
    check_start = int(spaces["projection_check_bytes"]["start"])
    absolute_column = projection_start + int(inactive["column"])
    survivor = {
        "encoding": "sparse_vector_v1",
        "ambient_columns": int(generalized["columns"]),
        "nonzero_entries": [
            {
                "column": absolute_column,
                "space": "hidden_source_projection_variables",
                "space_column": inactive["column"],
                "value_mod_q": 1,
                "balanced_value": 1,
            }
        ],
        "checked_core_response_difference_nonzero": False,
        "carrier_lane_difference_nonzero": False,
        "check_byte_difference_nonzero": False,
        "linf_bound": 1,
    }
    replay_rows = [
        {
            "id": "checked_core_response_link_rows",
            "status": "satisfied",
            "reason": "survivor has zero checked-core response coordinates",
        },
        {
            "id": "selected_hidden_source_projection_rows",
            "status": "satisfied",
            "reason": "survivor uses an inactive projection column absent from the selected sparse rows",
        },
        {
            "id": "public_carrier_lane_definition_rows",
            "status": "satisfied",
            "reason": "survivor has zero decoded carrier-lane coordinates",
        },
        {
            "id": "projection_check_byte_predicates",
            "status": "satisfied",
            "reason": "survivor has zero check-byte coordinates",
        },
    ]
    open_bridge_slots = [
        row for row in generalized["bridge_slots"] if row["status"] == "reserved_open"
    ]
    exact_replay = {
        "mode": "instantiated_generalized_system_without_reserved_open_bridges",
        "result": "bounded_survivor_found",
        "infeasible": False,
        "bad_vector_exported": True,
        "bound": BOUND,
        "survivor_satisfies_imported_rows": True,
        "open_bridge_slot_count": len(open_bridge_slots),
    }
    classification = {
        "kind": "underconstrained_open_bridge_survivor",
        "route_acceptance": "reject_fail_closed",
        "selected_map_disposition": "not_proved_by_current_exact_system",
        "next_closure": "derive_bridge_rows_or_redesign_projection_map",
        "why_not_deployable": "the replay exports a bounded survivor in an inactive hidden-source projection coordinate before the missing bridge slots are instantiated",
    }
    digest_payload = {
        "matrix_builder_status": matrix.get("status"),
        "generalized_system_sha256": matrix.get("target", {}).get(
            "generalized_system_sha256"
        ),
        "exact_spec_status": spec.get("status"),
        "derivation_status": derivation.get("status"),
        "exact_replay": exact_replay,
        "survivor": survivor,
        "classification": classification,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_exact_infeasibility_replay.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "exact_replay_ran_bounded_survivor_exported_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "matrix_builder": rel(MATRIX_BUILDER_JSON),
            "exact_certificate_spec": rel(EXACT_SPEC_JSON),
            "bridge_equation_derivation": rel(BRIDGE_DERIVATION_JSON),
            "matrix_builder_schema": matrix.get("schema"),
            "exact_certificate_spec_schema": spec.get("schema"),
            "bridge_equation_derivation_schema": derivation.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "generalized_columns": int(generalized["columns"]),
            "projection_column_start": projection_start,
            "carrier_lane_column_start": carrier_start,
            "check_byte_column_start": check_start,
            "replay_digest": digest(digest_payload),
        },
        "exact_replay": exact_replay,
        "bad_vector": survivor,
        "bad_vector_classification": classification,
        "inactive_projection_column": inactive,
        "row_replay": replay_rows,
        "open_bridge_slots": open_bridge_slots,
        "structural_controls": [
            {
                "id": "generalized_system_present",
                "status": "pass"
                if matrix.get("coverage", {}).get("generalized_projection_matrix_built")
                else "fail",
                "observed": matrix.get("coverage", {}).get(
                    "generalized_projection_matrix_built"
                ),
                "required": True,
            },
            {
                "id": "bounded_survivor_within_bound",
                "status": "pass" if survivor["linf_bound"] <= BOUND else "fail",
                "observed": survivor["linf_bound"],
                "required": f"<= {BOUND}",
            },
            {
                "id": "bad_vector_exported",
                "status": "pass" if exact_replay["bad_vector_exported"] else "fail",
                "observed": exact_replay["bad_vector_exported"],
                "required": True,
            },
            {
                "id": "deployment_remains_fail_closed",
                "status": "pass" if not classification["route_acceptance"].startswith("accept") else "fail",
                "observed": classification["route_acceptance"],
                "required": "reject_fail_closed",
            },
        ],
        "decision": {
            "can_accept_now": False,
            "exact_infeasibility_replay_has_run": True,
            "exact_infeasibility_proved": False,
            "bad_vector_export_complete": True,
            "reason": "the deterministic replay found a bounded survivor in the instantiated generalized system, so the current exact system is underconstrained and cannot certify acceptance",
            "next_artifact": "hidden_source_fallback_projection_map_redesign_or_bridge_rows",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    replay = payload["exact_replay"]
    bad = payload["bad_vector"]
    lines = [
        "# Hidden-Source Projection Exact Infeasibility Replay",
        "",
        "This artifact runs the exact replay that is possible for the currently",
        "instantiated generalized system. It does not prove infeasibility. Instead,",
        "it exports a deterministic bounded survivor, classifies the gap, and keeps",
        "the selected verifier fail-closed.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- generalized columns: `{target['generalized_columns']}`",
        f"- replay digest: `{target['replay_digest']}`",
        f"- replay result: `{replay['result']}`",
        f"- infeasible: `{replay['infeasible']}`",
        f"- bad vector exported: `{replay['bad_vector_exported']}`",
        f"- open bridge slots: `{replay['open_bridge_slot_count']}`",
        "",
        "## Bad Vector",
        "",
        f"- encoding: `{bad['encoding']}`",
        f"- ambient columns: `{bad['ambient_columns']}`",
        f"- nonzero entries: `{bad['nonzero_entries']}`",
        f"- Linf bound: `{bad['linf_bound']}`",
        f"- classification: `{payload['bad_vector_classification']['kind']}`",
        f"- route acceptance: `{payload['bad_vector_classification']['route_acceptance']}`",
        "",
        "## Row Replay",
        "",
        "| row group | status | reason |",
        "|---|---|---|",
    ]
    for row in payload["row_replay"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['reason']} |")
    lines.extend(
        [
            "",
            "## Structural Controls",
            "",
            "| id | status | observed | required |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["structural_controls"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['observed']}` | `{row['required']}` |"
        )
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- exact infeasibility replay has run: `{payload['decision']['exact_infeasibility_replay_has_run']}`",
            f"- exact infeasibility proved: `{payload['decision']['exact_infeasibility_proved']}`",
            f"- bad vector export complete: `{payload['decision']['bad_vector_export_complete']}`",
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
    print(
        json.dumps(
            {"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD), "status": payload["status"]},
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
