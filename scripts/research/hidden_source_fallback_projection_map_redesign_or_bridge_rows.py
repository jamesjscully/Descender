#!/usr/bin/env python3
"""Decide whether the exact survivor requires map redesign or bridge rows."""

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
    / "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.md"
)
EXACT_REPLAY_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json"
)
MATRIX_BUILDER_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
)
DIRECT_LINEAR_LIFT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.json"
)
CHECKED_CORE_BREAK_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.json"
)
PUBLIC_BRIDGE_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json"
)
EXPORT_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_contract_n32_q12289.json"
)
NEGATIVE_TESTS_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"
)

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.map_redesign_or_bridge_rows.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def build_payload() -> dict[str, Any]:
    exact = load_json(EXACT_REPLAY_JSON)
    matrix = load_json(MATRIX_BUILDER_JSON)
    direct_lift = load_json(DIRECT_LINEAR_LIFT_JSON)
    checked_break = load_json(CHECKED_CORE_BREAK_JSON)
    public_bridge = load_json(PUBLIC_BRIDGE_JSON)
    export_contract = load_json(EXPORT_CONTRACT_JSON)
    negative_tests = load_json(NEGATIVE_TESTS_JSON)

    rank_accounting = matrix.get("rank_accounting", {})
    projection_columns = int(rank_accounting.get("projection_columns", 0))
    active_projection_columns = len(matrix.get("active_columns", []))
    inactive_projection_columns = projection_columns - active_projection_columns
    survivor_entry = exact.get("bad_vector", {}).get("nonzero_entries", [{}])[0]
    survivor_space_column = survivor_entry.get("space_column")
    survivor_is_active = survivor_space_column in {
        row.get("column") for row in matrix.get("active_columns", [])
    }
    open_bridge_slots = matrix.get("generalized_system", {}).get("bridge_slots", [])
    implemented_exports = int(
        export_contract.get("coverage", {}).get("implemented_export_count", 0)
    )
    implemented_tests = int(
        negative_tests.get("coverage", {}).get("implemented_test_count", 0)
    )
    lane_checks = int(public_bridge.get("coverage", {}).get("lane_checks_specified", 0))
    missing_projection_capacity = max(0, projection_columns - lane_checks)
    existing_bridge_specs = [
        public_bridge.get("status"),
        export_contract.get("status"),
        negative_tests.get("status"),
        checked_break.get("status"),
    ]
    options = [
        {
            "id": "accept_current_map",
            "decision": "rejected_fail_closed",
            "reason": "exact replay exported a bounded survivor in the current generalized system",
        },
        {
            "id": "map_only_add_more_public_lanes",
            "decision": "rejected_under_fixed_56_byte_carrier",
            "reason": (
                "the current carrier exposes 12 public q12289 lanes while the "
                "hidden-source projection space has 64 columns"
            ),
            "additional_public_lanes_required_to_cover_projection_space": missing_projection_capacity,
        },
        {
            "id": "redefine_projection_space_to_active_lanes",
            "decision": "deferred_research_map_redesign",
            "reason": (
                "collapsing the relation to only the active lane symbols would "
                "change the hidden-source relation and require fresh replay and "
                "cryptanalysis rather than incremental closure"
            ),
        },
        {
            "id": "derive_bridge_rows_or_break_exports",
            "decision": "selected",
            "reason": (
                "the existing proof track already names checked-core and "
                "hidden-source bridge failures and can remain byte-preserving "
                "by exporting review witnesses instead of accepting underconstrained rows"
            ),
        },
    ]
    digest_payload = {
        "exact_replay_status": exact.get("status"),
        "survivor_space_column": survivor_space_column,
        "projection_columns": projection_columns,
        "active_projection_columns": active_projection_columns,
        "inactive_projection_columns": inactive_projection_columns,
        "lane_checks": lane_checks,
        "open_bridge_slot_ids": [row.get("id") for row in open_bridge_slots],
        "existing_bridge_specs": existing_bridge_specs,
        "options": options,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_map_redesign_or_bridge_rows.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "map_redesign_or_bridge_rows_gate_bridge_export_selected_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "exact_infeasibility_replay": rel(EXACT_REPLAY_JSON),
            "matrix_builder": rel(MATRIX_BUILDER_JSON),
            "direct_linear_lift_attempt": rel(DIRECT_LINEAR_LIFT_JSON),
            "checked_core_break_reduction": rel(CHECKED_CORE_BREAK_JSON),
            "public_bridge_predicate": rel(PUBLIC_BRIDGE_JSON),
            "break_export_contract": rel(EXPORT_CONTRACT_JSON),
            "break_export_negative_tests": rel(NEGATIVE_TESTS_JSON),
            "exact_infeasibility_schema": exact.get("schema"),
            "matrix_builder_schema": matrix.get("schema"),
            "direct_linear_lift_schema": direct_lift.get("schema"),
            "checked_core_break_schema": checked_break.get("schema"),
            "public_bridge_schema": public_bridge.get("schema"),
            "break_export_contract_schema": export_contract.get("schema"),
            "break_export_negative_tests_schema": negative_tests.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "projection_columns": projection_columns,
            "active_projection_columns": active_projection_columns,
            "inactive_projection_columns": inactive_projection_columns,
            "lane_checks": lane_checks,
            "decision_digest": digest(digest_payload),
        },
        "survivor": {
            "ambient_columns": exact.get("bad_vector", {}).get("ambient_columns"),
            "space": survivor_entry.get("space"),
            "space_column": survivor_space_column,
            "is_selected_active_projection_column": survivor_is_active,
            "linf_bound": exact.get("bad_vector", {}).get("linf_bound"),
            "route_acceptance": exact.get("bad_vector_classification", {}).get(
                "route_acceptance"
            ),
        },
        "bridge_slots": open_bridge_slots,
        "options": options,
        "coverage": {
            "exact_replay_has_run": exact.get("decision", {}).get(
                "exact_infeasibility_replay_has_run"
            ),
            "bad_vector_exported": exact.get("decision", {}).get(
                "bad_vector_export_complete"
            ),
            "projection_columns": projection_columns,
            "active_projection_columns": active_projection_columns,
            "inactive_projection_columns": inactive_projection_columns,
            "lane_checks_specified": lane_checks,
            "additional_public_lanes_required_to_cover_projection_space": missing_projection_capacity,
            "open_bridge_slot_count": len(open_bridge_slots),
            "export_record_count": export_contract.get("coverage", {}).get(
                "export_record_count"
            ),
            "implemented_export_count": implemented_exports,
            "negative_test_count": negative_tests.get("coverage", {}).get(
                "test_count"
            ),
            "implemented_test_count": implemented_tests,
            "signature_byte_increase": 0,
        },
        "decision": {
            "selected_path": "bridge_rows_or_break_export_implementation",
            "map_redesign_required_now": False,
            "map_redesign_disposition": "deferred_to_new_relation_research_if_bridge_export_path_fails",
            "can_accept_now": False,
            "can_run_exact_solver_now": False,
            "reason": (
                "the current exact survivor is a real underconstraint, but a "
                "map-only change cannot close 64 projection coordinates through "
                "the fixed 12-lane carrier; the byte-preserving path is to "
                "implement verifier/exporter bridge semantics and keep acceptance fail-closed"
            ),
            "next_artifact": "hidden_source_fallback_projection_verifier_exporter_contract",
        },
        "structural_controls": [
            {
                "id": "bounded_survivor_accounted_for",
                "status": "pass"
                if exact.get("decision", {}).get("bad_vector_export_complete")
                else "fail",
                "observed": exact.get("bad_vector", {}).get("nonzero_entries"),
                "required": "exported bounded survivor",
            },
            {
                "id": "survivor_not_covered_by_active_lane_set",
                "status": "pass" if not survivor_is_active else "fail",
                "observed": survivor_space_column,
                "required": "inactive projection column",
            },
            {
                "id": "map_only_lane_capacity_insufficient",
                "status": "pass" if missing_projection_capacity > 0 else "fail",
                "observed": lane_checks,
                "required": projection_columns,
            },
            {
                "id": "bridge_export_track_is_specified",
                "status": "pass"
                if all(status is not None for status in existing_bridge_specs)
                else "fail",
                "observed": existing_bridge_specs,
                "required": "public predicate, export contract, negative tests, checked-core break reduction",
            },
            {
                "id": "deployment_remains_fail_closed",
                "status": "pass",
                "observed": False,
                "required": False,
            },
        ],
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    coverage = payload["coverage"]
    survivor = payload["survivor"]
    decision = payload["decision"]
    lines = [
        "# Hidden-Source Projection Map Redesign Or Bridge Rows",
        "",
        "This artifact closes the fork exposed by the exact survivor replay:",
        "either redesign the projection map/relation, or continue with concrete",
        "bridge rows and break-export verifier semantics.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- projection columns: `{target['projection_columns']}`",
        f"- active projection columns: `{target['active_projection_columns']}`",
        f"- inactive projection columns: `{target['inactive_projection_columns']}`",
        f"- lane checks specified: `{target['lane_checks']}`",
        f"- decision digest: `{target['decision_digest']}`",
        f"- selected path: `{decision['selected_path']}`",
        f"- next artifact: `{decision['next_artifact']}`",
        "",
        "## Survivor",
        "",
        f"- space: `{survivor['space']}`",
        f"- space column: `{survivor['space_column']}`",
        f"- selected active projection column: `{survivor['is_selected_active_projection_column']}`",
        f"- linf bound: `{survivor['linf_bound']}`",
        f"- route acceptance: `{survivor['route_acceptance']}`",
        "",
        "## Options",
        "",
        "| option | decision | reason |",
        "|---|---|---|",
    ]
    for row in payload["options"]:
        lines.append(f"| `{row['id']}` | `{row['decision']}` | {row['reason']} |")
    lines.extend(
        [
            "",
            "## Coverage",
            "",
            f"- exact replay has run: `{coverage['exact_replay_has_run']}`",
            f"- bad vector exported: `{coverage['bad_vector_exported']}`",
            f"- public lanes missing to cover full projection space: `{coverage['additional_public_lanes_required_to_cover_projection_space']}`",
            f"- open bridge slots: `{coverage['open_bridge_slot_count']}`",
            f"- export records specified: `{coverage['export_record_count']}`",
            f"- exports implemented: `{coverage['implemented_export_count']}`",
            f"- negative tests specified: `{coverage['negative_test_count']}`",
            f"- negative tests implemented: `{coverage['implemented_test_count']}`",
            "",
            "## Interpretation",
            "",
            decision["reason"],
            "",
            "## Source Artifacts",
            "",
        ]
    )
    for key, value in payload["source_artifacts"].items():
        lines.append(f"- `{key}`: `{value}`")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(f"wrote {rel(DEFAULT_JSON)}")
    print(f"wrote {rel(DEFAULT_MD)}")


if __name__ == "__main__":
    main()
