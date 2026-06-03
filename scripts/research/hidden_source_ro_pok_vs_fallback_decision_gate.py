#!/usr/bin/env python3
"""Generate the decision gate between 32-byte RO-PoK and 56-byte fallback."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "hidden_source_ro_pok_vs_fallback_decision_gate_n32_q12289.md"
RO_POK_API_JSON = READINESS_DIR / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json"
RO_POK_NEGATIVE_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"
)
FALLBACK_CERT_ATTEMPT_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    ro_pok_api = load_json(RO_POK_API_JSON)
    ro_pok_negative = load_json(RO_POK_NEGATIVE_JSON)
    fallback = load_json(FALLBACK_CERT_ATTEMPT_JSON)
    gate_rows = [
        {
            "id": "ro_pok_assumption_review",
            "route": "32_byte_ro_pok",
            "required_result": "external reviewers accept state_assisted_hidden_relation_ro_pok as an explicit nonstandard assumption",
            "current_status": "open",
            "failure_action": "activate_56_byte_fallback_certificate_attempt",
        },
        {
            "id": "ro_pok_executable_controls",
            "route": "32_byte_ro_pok",
            "required_result": "byte negative controls pass against an audited accepting verifier or oracle model",
            "current_status": ro_pok_negative.get("executable_status", {}).get("current"),
            "failure_action": "activate_56_byte_fallback_certificate_attempt",
        },
        {
            "id": "ro_pok_leakage_theorem",
            "route": "32_byte_ro_pok",
            "required_result": "opaque 32-byte handle leakage is fully charged and no hidden witness/pad material leaks through diagnostics",
            "current_status": "open",
            "failure_action": "activate_56_byte_fallback_certificate_attempt",
        },
        {
            "id": "fallback_exact_certificate",
            "route": "56_byte_fallback",
            "required_result": "top fallback projection map has exact bounded-kernel, semantic-control, decoder, and leakage certificates",
            "current_status": fallback.get("status"),
            "failure_action": "try_larger_fallback_or_reject_compact_target",
        },
        {
            "id": "global_composition",
            "route": "both",
            "required_result": "chosen route composes with checked-core, response-link, QROM, pad, keygen, and lifecycle assumptions",
            "current_status": "open",
            "failure_action": "keep_verifier_fail_closed",
        },
    ]
    route_comparison = [
        {
            "route": "32_byte_ro_pok",
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "advantage": "smallest blockchain transaction footprint",
            "primary_risk": "nonstandard proof-of-knowledge handle assumption may be too strong for third-party review",
            "promotion_state": "archived_aggressive_research_option",
        },
        {
            "route": "56_byte_fallback",
            "carrier_bytes": 56,
            "envelope_bytes": 308,
            "advantage": "adds public algebraic projection material so extraction and negative controls can be made concrete",
            "primary_risk": "larger signatures and still needs exact projection certificate plus leakage theorem",
            "promotion_state": "fail_closed_selected_research_track",
        },
    ]
    return {
        "schema": "Descender.hidden_source_ro_pok_vs_fallback_decision_gate.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "projection_fallback_selected_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "ro_pok_api": rel(RO_POK_API_JSON),
            "ro_pok_api_schema": ro_pok_api.get("schema"),
            "ro_pok_byte_negative_tests": rel(RO_POK_NEGATIVE_JSON),
            "ro_pok_byte_negative_tests_schema": ro_pok_negative.get("schema"),
            "fallback_certificate_attempt": rel(FALLBACK_CERT_ATTEMPT_JSON),
            "fallback_certificate_attempt_schema": fallback.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "archived_branch": "fail-closed `284 B/Fq12` RO-PoK handle route",
        },
        "route_comparison": route_comparison,
        "gate_rows": gate_rows,
        "decision_policy": {
            "archive_284b_reason": [
                "RO-PoK proof handle is a nonstandard deployability risk",
                "32-byte handle semantics are harder to reduce to concrete algebraic checks",
                "blockchain target can tolerate the 24-byte increase",
            ],
            "reconsider_284b_only_if": [
                "RO-PoK assumption is accepted by external review as a named nonstandard assumption",
                "byte-level controls pass against an accepting verifier model",
                "leakage theorem closes for the opaque 32-byte handle",
                "global theorem composes without extra hidden verifier inputs",
            ],
            "select_308b_now_because": [
                "projection bytes expose public algebraic material for exact certificate attempts",
                "semantic controls can be made executable without a digest-sized knowledge oracle",
                "308 bytes is still small enough for the blockchain target",
            ],
            "reject_both_if": [
                "fallback projection certificate fails and no larger acceptable carrier closes",
                "global composition fails for both routes",
                "state lifecycle assumptions cannot be made deployable",
            ],
        },
        "decision": {
            "can_accept_now": False,
            "selected_now": "56_byte_projection_fallback_fail_closed",
            "fallback_selected_now": True,
            "reason": "the project has pivoted to the 308-byte projection route because it is more concrete and still compact enough for the blockchain target",
            "next_artifact": "fallback_projection_map_exact_certificate",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Hidden-Source RO-PoK vs Fallback Decision Gate",
        "",
        "This artifact records the route pivot: the fail-closed `308 B/Fq12`",
        "projection-carrier route is now selected, while the `284 B/Fq12` RO-PoK",
        "handle route is archived as an aggressive research option.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {payload['target']['selected_branch']}",
        f"- archived branch: {payload['target']['archived_branch']}",
        "",
        "## Route Comparison",
        "",
        "| route | carrier bytes | envelope bytes | promotion state | primary risk |",
        "|---|---:|---:|---|---|",
    ]
    for row in payload["route_comparison"]:
        lines.append(
            f"| `{row['route']}` | `{row['carrier_bytes']}` | `{row['envelope_bytes']}` | "
            f"`{row['promotion_state']}` | {row['primary_risk']} |"
        )
    lines.extend(
        [
            "",
            "## Gate Rows",
            "",
            "| id | route | current status | failure action | required result |",
            "|---|---|---|---|---|",
        ]
    )
    for row in payload["gate_rows"]:
        lines.append(
            f"| `{row['id']}` | `{row['route']}` | `{row['current_status']}` | "
            f"`{row['failure_action']}` | {row['required_result']} |"
        )
    policy = payload["decision_policy"]
    lines.extend(["", "## Archive 284 B Reason", ""])
    lines.extend(f"- {item}" for item in policy["archive_284b_reason"])
    lines.extend(["", "## Reconsider 284 B Only If", ""])
    lines.extend(f"- {item}" for item in policy["reconsider_284b_only_if"])
    lines.extend(["", "## Select 308 B Now Because", ""])
    lines.extend(f"- {item}" for item in policy["select_308b_now_because"])
    lines.extend(["", "## Reject Both If", ""])
    lines.extend(f"- {item}" for item in policy["reject_both_if"])
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- selected now: `{payload['decision']['selected_now']}`",
            f"- fallback selected now: `{payload['decision']['fallback_selected_now']}`",
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
