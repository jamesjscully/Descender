#!/usr/bin/env python3
"""Rank checked-core coupling routes for the selected projection carrier."""

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
    / "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_checked_core_coupling_options_n32_q12289.md"
)
DERIVATION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json"
)
MAP_SEARCH_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_map_search_n32_q12289.json"
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
    h.update(b"Descender.hidden_source.checked_core_coupling_options.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def score(option: dict[str, Any]) -> int:
    return (
        int(option["proof_directness"])
        + int(option["byte_stability"])
        + int(option["replayability"])
        + int(option["assumption_discipline"])
        - int(option["speculation_penalty"])
    )


def build_payload() -> dict[str, Any]:
    derivation = load_json(DERIVATION_JSON)
    search = load_json(MAP_SEARCH_JSON)
    response_link = load_json(RESPONSE_LINK_CERT_JSON)
    route_options = [
        {
            "id": "direct_linear_lift",
            "status": "recommended_first",
            "summary": "derive explicit public rows mapping checked-core response-difference coordinates into the selected witness/pad projection coordinates",
            "proof_directness": 5,
            "byte_stability": 5,
            "replayability": 5,
            "assumption_discipline": 5,
            "speculation_penalty": 2,
            "required_next_artifact": "hidden_source_fallback_projection_direct_linear_lift_attempt",
            "success_condition": "at least one nonzero checked-core coupling row is derived and replayed without changing the 56-byte carrier",
            "failure_condition": "no public checked-core response expression can be tied to selected witness/pad projections without adding secret verifier input",
        },
        {
            "id": "checked_core_break_reduction",
            "status": "fallback_if_direct_lift_fails",
            "summary": "show any accepted mismatch between checked-core response relation and selected projections yields checked_core_or_response_link_break",
            "proof_directness": 4,
            "byte_stability": 5,
            "replayability": 3,
            "assumption_discipline": 4,
            "speculation_penalty": 3,
            "required_next_artifact": "hidden_source_fallback_projection_checked_core_break_reduction",
            "success_condition": "formal reduction names an externally reviewable checked_core_or_response_link_break and does not rely on opaque RO-PoK extraction",
            "failure_condition": "reduction silently assumes the projection coordinates are already checked-core-linked",
        },
        {
            "id": "response_link_anchored_map_redesign",
            "status": "fallback_if_selected_map_cannot_couple",
            "summary": "replace or augment the balanced_witness_pad_subview map with response-link anchored coordinates whose checked-core coupling is already certificate-adjacent",
            "proof_directness": 3,
            "byte_stability": 2,
            "replayability": 4,
            "assumption_discipline": 5,
            "speculation_penalty": 2,
            "required_next_artifact": "hidden_source_fallback_projection_map_redesign_gate",
            "success_condition": "new selected map preserves 56-byte carrier and obtains explicit checked-core coupling rows",
            "failure_condition": "requires more projection bytes or loses hidden-source witness/pad semantic coverage",
        },
        {
            "id": "hash_seeded_statement_local_projection",
            "status": "research_only",
            "summary": "derive projection coordinates from transcript hash so the bridge is statement-local rather than fixed-map",
            "proof_directness": 2,
            "byte_stability": 4,
            "replayability": 2,
            "assumption_discipline": 3,
            "speculation_penalty": 4,
            "required_next_artifact": "hidden_source_fallback_projection_statement_local_model",
            "success_condition": "statement-local certificates become efficiently replayable or reducible under a clean random-projection theorem",
            "failure_condition": "requires per-statement exact certificates or adds hard-to-review random-projection assumptions",
        },
    ]
    for option in route_options:
        option["score"] = score(option)
    route_options = sorted(route_options, key=lambda row: (-row["score"], row["id"]))
    selected = route_options[0]
    decision = {
        "selected_route": selected["id"],
        "can_accept_now": False,
        "can_run_exact_solver_now": False,
        "reason": "direct linear lift is the least speculative route that preserves the 308-byte target; if it fails, fall back to a named checked-core break reduction before redesigning the map",
        "next_artifact": selected["required_next_artifact"],
    }
    digest_payload = {
        "derivation_digest": derivation["target"]["derivation_digest"],
        "response_link_status": response_link.get("status"),
        "map_search_status": search.get("status"),
        "route_options": route_options,
        "decision": decision,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_checked_core_coupling_options.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "selected_projection_checked_core_coupling_options_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "bridge_equation_derivation": rel(DERIVATION_JSON),
            "projection_map_search": rel(MAP_SEARCH_JSON),
            "response_link_certificate": rel(RESPONSE_LINK_CERT_JSON),
            "bridge_equation_derivation_schema": derivation.get("schema"),
            "projection_map_search_schema": search.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "options_digest": digest(digest_payload),
        },
        "route_options": route_options,
        "decision": decision,
        "structural_controls": [
            {
                "id": "selected_route_preserves_bytes",
                "status": "pass" if selected["byte_stability"] >= 5 else "fail",
                "observed": selected["id"],
                "required": "first route should preserve the 56-byte carrier",
            },
            {
                "id": "selected_route_not_assumption_only",
                "status": "pass" if selected["replayability"] >= 5 else "fail",
                "observed": selected["id"],
                "required": "first route must produce replayable rows before a pure reduction",
            },
            {
                "id": "acceptance_still_fail_closed",
                "status": "pass",
                "observed": decision["can_accept_now"],
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
    lines = [
        "# Hidden-Source Projection Checked-Core Coupling Options",
        "",
        "This artifact ranks the available routes for closing the checked-core",
        "coupling gap.  It selects a route to attempt next; it does not claim the",
        "coupling has been proved.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- options digest: `{target['options_digest']}`",
        f"- selected route: `{payload['decision']['selected_route']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Route Options",
        "",
        "| id | status | score | proof | bytes | replay | discipline | penalty | next artifact |",
        "|---|---|---:|---:|---:|---:|---:|---:|---|",
    ]
    for row in payload["route_options"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['score']}` | "
            f"`{row['proof_directness']}` | `{row['byte_stability']}` | "
            f"`{row['replayability']}` | `{row['assumption_discipline']}` | "
            f"`{row['speculation_penalty']}` | `{row['required_next_artifact']}` |"
        )
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
            f"- can run exact solver now: `{payload['decision']['can_run_exact_solver_now']}`",
            f"- reason: {payload['decision']['reason']}",
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
