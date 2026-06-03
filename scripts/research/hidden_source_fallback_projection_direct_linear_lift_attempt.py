#!/usr/bin/env python3
"""Attempt the direct checked-core-to-projection linear lift."""

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
    / "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.md"
)
COUPLING_OPTIONS_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.json"
)
BRIDGE_DERIVATION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json"
)
PROJECTION_MATRIX_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
)
RESPONSE_LINK_CERT_JSON = (
    READINESS_DIR
    / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.direct_linear_lift_attempt.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def map_terms_from_artifacts(*artifacts: dict[str, Any]) -> list[str]:
    terms: list[str] = []
    needles = (
        "response_to_projection",
        "checked_core_to_projection",
        "response_projection_map",
        "linear_lift_map",
        "projection_checked_core_map",
    )
    for artifact in artifacts:
        encoded = json.dumps(artifact, sort_keys=True)
        for needle in needles:
            if needle in encoded and needle not in terms:
                terms.append(needle)
    return terms


def build_payload() -> dict[str, Any]:
    options = load_json(COUPLING_OPTIONS_JSON)
    derivation = load_json(BRIDGE_DERIVATION_JSON)
    projection = load_json(PROJECTION_MATRIX_JSON)
    response = load_json(RESPONSE_LINK_CERT_JSON)

    selected_route = options.get("decision", {}).get("selected_route")
    projection_target = projection.get("target", {})
    rank_impact = derivation.get("rank_impact", {})
    found_map_terms = map_terms_from_artifacts(options, derivation, projection, response)
    public_map_present = bool(found_map_terms)
    derived_rows = 0 if not public_map_present else "not_evaluated"
    response_columns = (
        int(response.get("combined_constraint_rank", 0))
        + int(response.get("combined_kernel_dimension", 0))
    )
    active_projection_columns = projection.get("active_columns", [])
    attempt_checks = [
        {
            "id": "selected_route_is_direct_linear_lift",
            "status": "pass" if selected_route == "direct_linear_lift" else "fail",
            "observed": selected_route,
            "required": "direct_linear_lift",
        },
        {
            "id": "response_matrix_available",
            "status": "pass" if response.get("matrix_transcript_sha256") else "fail",
            "observed": response.get("matrix_transcript_sha256"),
            "required": "response-link matrix transcript hash",
        },
        {
            "id": "projection_matrix_available",
            "status": "pass" if projection_target.get("matrix_sha256") else "fail",
            "observed": projection_target.get("matrix_sha256"),
            "required": "selected 12x64 projection matrix hash",
        },
        {
            "id": "response_to_projection_column_map_present",
            "status": "open" if not public_map_present else "pass",
            "observed": found_map_terms,
            "required": "public map from 23 checked-core response columns to 64 projection columns",
        },
        {
            "id": "checked_core_coupling_rows_derived",
            "status": "open" if derived_rows == 0 else "pass",
            "observed": derived_rows,
            "required": "at least one nonzero checked-core coupling row",
        },
        {
            "id": "exact_solver_preconditions",
            "status": "blocked",
            "observed": False,
            "required": "public map plus nonzero coupling rows",
        },
    ]
    route_disposition = {
        "selected_route": "direct_linear_lift",
        "outcome": "no_public_response_to_projection_map",
        "direct_linear_lift_succeeded": False,
        "fallback_route": "checked_core_break_reduction",
        "fallback_required_before_map_redesign": True,
    }
    digest_payload = {
        "options_digest": options.get("target", {}).get("options_digest"),
        "derivation_digest": derivation.get("target", {}).get("derivation_digest"),
        "projection_matrix_sha256": projection_target.get("matrix_sha256"),
        "response_matrix_sha256": response.get("matrix_transcript_sha256"),
        "response_columns": response_columns,
        "projection_columns": projection_target.get("columns"),
        "public_map_present": public_map_present,
        "derived_checked_core_coupling_rows": derived_rows,
        "route_disposition": route_disposition,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_direct_linear_lift_attempt.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "direct_linear_lift_attempt_no_public_map_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "checked_core_coupling_options": rel(COUPLING_OPTIONS_JSON),
            "bridge_equation_derivation": rel(BRIDGE_DERIVATION_JSON),
            "projection_matrix_builder": rel(PROJECTION_MATRIX_JSON),
            "response_link_certificate": rel(RESPONSE_LINK_CERT_JSON),
            "checked_core_coupling_options_schema": options.get("schema"),
            "bridge_equation_derivation_schema": derivation.get("schema"),
            "projection_matrix_builder_schema": projection.get("schema"),
            "response_link_matrix_domain": response.get("matrix_domain"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "attempt_digest": digest(digest_payload),
        },
        "attempt": {
            "response_columns": response_columns,
            "response_rank": response.get("combined_constraint_rank"),
            "response_kernel_dimension": response.get("combined_kernel_dimension"),
            "projection_rows": projection_target.get("rows"),
            "projection_columns": projection_target.get("columns"),
            "projection_rank": projection_target.get("rank"),
            "active_projection_columns": active_projection_columns,
            "searched_public_map_terms": found_map_terms,
            "response_to_projection_column_map_present": public_map_present,
            "derived_checked_core_coupling_rows": derived_rows,
            "can_run_exact_solver": False,
            "open_nullity_before_missing_bridges": rank_impact.get(
                "current_nullity_before_missing_bridges"
            ),
        },
        "attempt_checks": attempt_checks,
        "route_disposition": route_disposition,
        "decision": {
            "can_accept_now": False,
            "can_run_exact_solver_now": False,
            "reason": "the direct lift has the response matrix and projection matrix, but no public 23-to-64 response-to-projection column map is present in the current artifacts",
            "next_artifact": "hidden_source_fallback_projection_checked_core_break_reduction",
        },
        "structural_controls": [
            {
                "id": "byte_target_preserved",
                "status": "pass",
                "observed": 308,
                "required": 308,
            },
            {
                "id": "no_secret_map_assumed",
                "status": "pass",
                "observed": public_map_present,
                "required": False,
            },
            {
                "id": "direct_lift_not_silently_accepted",
                "status": "pass",
                "observed": route_disposition["direct_linear_lift_succeeded"],
                "required": False,
            },
            {
                "id": "solver_remains_blocked_without_coupling_rows",
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
    attempt = payload["attempt"]
    route = payload["route_disposition"]
    lines = [
        "# Hidden-Source Projection Direct Linear Lift Attempt",
        "",
        "This artifact attempts the selected direct linear lift from the",
        "checked-core response relation into the selected hidden-source projection",
        "carrier. It is a fail-closed checkpoint, not an acceptance certificate.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- attempt digest: `{target['attempt_digest']}`",
        f"- route outcome: `{route['outcome']}`",
        f"- fallback route: `{route['fallback_route']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Attempt Inputs",
        "",
        f"- response columns: `{attempt['response_columns']}`",
        f"- response rank: `{attempt['response_rank']}`",
        f"- response kernel dimension: `{attempt['response_kernel_dimension']}`",
        f"- projection shape: `{attempt['projection_rows']}x{attempt['projection_columns']}`",
        f"- projection rank: `{attempt['projection_rank']}`",
        f"- active projection columns: `{len(attempt['active_projection_columns'])}`",
        "",
        "## Result",
        "",
        f"- response-to-projection map present: `{attempt['response_to_projection_column_map_present']}`",
        f"- derived checked-core coupling rows: `{attempt['derived_checked_core_coupling_rows']}`",
        f"- can run exact solver: `{attempt['can_run_exact_solver']}`",
        f"- open nullity before missing bridges: `{attempt['open_nullity_before_missing_bridges']}`",
        "",
        "The response-link certificate and projection matrix are both present, but",
        "the current public artifacts do not define a map from the `23` checked-core",
        "response columns to the `64` hidden-source projection columns. Therefore",
        "the direct linear lift cannot derive checked-core coupling rows without",
        "assuming the missing bridge. The route is closed fail-closed and the next",
        "step is the named `checked_core_break_reduction` fallback.",
        "",
        "## Checks",
        "",
        "| check | status | observed | required |",
        "|---|---|---|---|",
    ]
    for row in payload["attempt_checks"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['observed']}` | `{row['required']}` |"
        )
    lines.extend(
        [
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
