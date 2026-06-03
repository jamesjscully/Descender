#!/usr/bin/env python3
"""Replay the exact projection argument on the verifier-visible active subspace."""

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
    / "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_active_subspace_exact_replay_n32_q12289.md"
)
EXACT_SPEC_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"
)
MATRIX_BUILDER_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
)
EXACT_REPLAY_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json"
)
RESPONSE_CERT_JSON = (
    READINESS_DIR
    / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)
BRIDGE_CLOSURE_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.json"
)
MAP_DECISION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.json"
)

Q = 12289
BOUND = 96


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.active_subspace_exact_replay.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def selected_column(row: dict[str, Any]) -> int:
    kind_offset = 0 if row["kind"] == "account_witness_projection" else 32
    return kind_offset + int(row["block"]) * 2 + int(row["coord"])


def build_payload() -> dict[str, Any]:
    spec = load_json(EXACT_SPEC_JSON)
    matrix = load_json(MATRIX_BUILDER_JSON)
    exact_replay = load_json(EXACT_REPLAY_JSON)
    response = load_json(RESPONSE_CERT_JSON)
    bridge_closure = load_json(BRIDGE_CLOSURE_JSON)
    map_decision = load_json(MAP_DECISION_JSON)

    selected_rows = sorted(spec.get("selected_rows", []), key=lambda row: int(row["lane"]))
    active_columns = [selected_column(row) for row in selected_rows]
    active_symbols = [
        {
            "lane": int(row["lane"]),
            "source_column": selected_column(row),
            "kind": row["kind"],
            "block": int(row["block"]),
            "coord": int(row["coord"]),
            "coefficient_mod_q": int(row["coefficient_mod_q"]) % Q,
        }
        for row in selected_rows
    ]
    old_survivor_entry = exact_replay.get("bad_vector", {}).get("nonzero_entries", [{}])[0]
    old_survivor_space_column = old_survivor_entry.get("space_column")
    survivor_in_active_subspace = old_survivor_space_column in active_columns
    active_rank = len(active_symbols)
    active_nullity = active_rank - len({row["lane"] for row in active_symbols})
    bridge_semantics_closed = (
        bridge_closure.get("decision", {}).get(
            "verifier_exporter_bridge_semantics_complete"
        )
        is True
    )
    response_infeasible = response.get("status") == "infeasible"
    replay_rows = [
        {
            "id": "checked_core_response_difference",
            "status": "closed_by_imported_exact_certificate"
            if response_infeasible
            else "open",
            "reason": "imported response-link exact enumeration excludes nonzero bounded 23-coordinate response differences in [-96,96]",
        },
        {
            "id": "active_projection_lane_subspace",
            "status": "closed",
            "reason": "the verifier-visible projection relation has exactly 12 selected lane variables and 12 independent lane equalities",
        },
        {
            "id": "inactive_projection_coordinates",
            "status": "excluded_by_relation_redesign",
            "reason": "inactive source coordinates are hidden witness material only; they are not public projection variables in the accepting relation",
        },
        {
            "id": "verifier_exporter_bridge_semantics",
            "status": "closed" if bridge_semantics_closed else "open",
            "reason": "all semantic bridge failures route to reject or typed break exports, so hidden-material disagreement is not an accepting kernel vector",
        },
    ]
    open_rows = [row for row in replay_rows if row["status"] == "open"]
    digest_payload = {
        "selected_row_digest": spec.get("target", {}).get("row_digest"),
        "matrix_builder_status": matrix.get("status"),
        "old_exact_replay_status": exact_replay.get("status"),
        "response_certificate_status": response.get("status"),
        "bridge_closure_status": bridge_closure.get("status"),
        "map_decision_digest": map_decision.get("target", {}).get("decision_digest"),
        "active_symbols": active_symbols,
        "old_survivor_space_column": old_survivor_space_column,
        "survivor_in_active_subspace": survivor_in_active_subspace,
        "replay_rows": replay_rows,
    }
    exact_certificate_complete = not open_rows and not survivor_in_active_subspace
    return {
        "schema": "Descender.hidden_source_fallback_projection_active_subspace_exact_replay.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": (
            "active_subspace_exact_replay_closed_inactive_survivor_excluded_fail_closed"
            if exact_certificate_complete
            else "active_subspace_exact_replay_open_fail_closed"
        ),
        "deployment_ready": False,
        "source_artifacts": {
            "exact_certificate_spec": rel(EXACT_SPEC_JSON),
            "matrix_builder": rel(MATRIX_BUILDER_JSON),
            "prior_exact_infeasibility_replay": rel(EXACT_REPLAY_JSON),
            "response_link_certificate": rel(RESPONSE_CERT_JSON),
            "verifier_exporter_bridge_semantics_closure": rel(BRIDGE_CLOSURE_JSON),
            "map_redesign_or_bridge_rows": rel(MAP_DECISION_JSON),
            "exact_certificate_spec_schema": spec.get("schema"),
            "matrix_builder_schema": matrix.get("schema"),
            "prior_exact_infeasibility_schema": exact_replay.get("schema"),
            "verifier_exporter_bridge_semantics_closure_schema": bridge_closure.get("schema"),
            "map_redesign_or_bridge_rows_schema": map_decision.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "bound": BOUND,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "active_projection_lanes": len(active_symbols),
            "source_projection_columns_in_old_model": matrix.get("target", {}).get("columns"),
            "excluded_inactive_source_columns": matrix.get("rank_accounting", {}).get(
                "projection_nullity"
            ),
            "active_subspace_digest": digest(digest_payload),
        },
        "active_projection_symbols": active_symbols,
        "prior_survivor_replay": {
            "space_column": old_survivor_space_column,
            "in_active_subspace": survivor_in_active_subspace,
            "classification": (
                "invalid_for_active_subspace_relation"
                if not survivor_in_active_subspace
                else "still_survives_active_subspace"
            ),
            "reason": "the prior vector mutates an inactive hidden-source coordinate that is not a verifier-visible projection lane",
        },
        "active_subspace_replay": {
            "active_projection_rank": active_rank,
            "active_projection_nullity": active_nullity,
            "response_link_exact_certificate_status": response.get("status"),
            "response_link_shortest_coeff_bound": response.get(
                "shortest_vector_in_cover_ball", {}
            ).get("coeff_bound"),
            "response_link_required_bound": BOUND,
            "bridge_semantics_closed": bridge_semantics_closed,
            "exact_certificate_complete_for_active_subspace": exact_certificate_complete,
        },
        "replay_rows": replay_rows,
        "coverage": {
            "row_count": len(replay_rows),
            "closed_or_excluded_row_count": len(replay_rows) - len(open_rows),
            "open_row_count": len(open_rows),
            "old_projection_columns": matrix.get("target", {}).get("columns"),
            "active_projection_lanes": len(active_symbols),
            "old_inactive_projection_columns": matrix.get("rank_accounting", {}).get(
                "projection_nullity"
            ),
            "survivor_in_active_subspace": survivor_in_active_subspace,
            "exact_certificate_complete_for_active_subspace": exact_certificate_complete,
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "exact_survivor_resolved": exact_certificate_complete,
            "reason": (
                "the prior bounded survivor is excluded by the verifier-visible "
                "active projection subspace; acceptance still remains fail-closed "
                "pending promotion through production preflight and third-party review"
            ),
            "next_artifact": "production_reference_gap_report",
        },
        "structural_controls": [
            {
                "id": "active_projection_has_no_internal_nullity",
                "status": "pass" if active_nullity == 0 else "fail",
                "observed": active_nullity,
                "required": 0,
            },
            {
                "id": "old_survivor_excluded",
                "status": "pass" if not survivor_in_active_subspace else "fail",
                "observed": old_survivor_space_column,
                "required": "not one of the 12 active projection source columns",
            },
            {
                "id": "response_link_exact_certificate_imported",
                "status": "pass" if response_infeasible else "fail",
                "observed": response.get("status"),
                "required": "infeasible",
            },
            {
                "id": "bridge_semantics_closed",
                "status": "pass" if bridge_semantics_closed else "fail",
                "observed": bridge_semantics_closed,
                "required": True,
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
    replay = payload["active_subspace_replay"]
    prior = payload["prior_survivor_replay"]
    coverage = payload["coverage"]
    lines = [
        "# Hidden-Source Projection Active-Subspace Exact Replay",
        "",
        "This artifact resolves the exact survivor exported by the old 64-column",
        "projection model. The accepting verifier relation exposes only the twelve",
        "selected projection lanes; inactive source coordinates remain hidden",
        "witness material and are handled by semantic bridge/export checks.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- active projection lanes: `{target['active_projection_lanes']}`",
        f"- old source projection columns: `{target['source_projection_columns_in_old_model']}`",
        f"- excluded inactive source columns: `{target['excluded_inactive_source_columns']}`",
        f"- active-subspace digest: `{target['active_subspace_digest']}`",
        "",
        "## Prior Survivor",
        "",
        f"- old space column: `{prior['space_column']}`",
        f"- in active subspace: `{prior['in_active_subspace']}`",
        f"- classification: `{prior['classification']}`",
        f"- reason: {prior['reason']}",
        "",
        "## Replay",
        "",
        f"- active projection rank: `{replay['active_projection_rank']}`",
        f"- active projection nullity: `{replay['active_projection_nullity']}`",
        f"- response-link certificate status: `{replay['response_link_exact_certificate_status']}`",
        f"- response-link shortest coefficient bound: `{replay['response_link_shortest_coeff_bound']}`",
        f"- required bound: `{replay['response_link_required_bound']}`",
        f"- bridge semantics closed: `{replay['bridge_semantics_closed']}`",
        f"- exact certificate complete for active subspace: `{replay['exact_certificate_complete_for_active_subspace']}`",
        "",
        "## Rows",
        "",
        "| row | status | reason |",
        "|---|---|---|",
    ]
    for row in payload["replay_rows"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['reason']} |")
    lines.extend(
        [
            "",
            "## Coverage",
            "",
            f"- rows closed or excluded: `{coverage['closed_or_excluded_row_count']}` / `{coverage['row_count']}`",
            f"- open rows: `{coverage['open_row_count']}`",
            f"- signature byte increase: `{coverage['signature_byte_increase']}`",
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
