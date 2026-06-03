#!/usr/bin/env python3
"""Bind projection-carrier negative tests to verifier/exporter behavior."""

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
    / "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.md"
)
NEGATIVE_TESTS_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"
)
EXPORT_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_contract_n32_q12289.json"
)
PUBLIC_BRIDGE_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json"
)
PRODUCTION_GAP_JSON = READINESS_DIR / "production_reference_gap_report_n32_q12289.json"

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.verifier_exporter_contract.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def build_payload() -> dict[str, Any]:
    negative = load_json(NEGATIVE_TESTS_JSON)
    export_contract = load_json(EXPORT_CONTRACT_JSON)
    public_bridge = load_json(PUBLIC_BRIDGE_JSON)
    production_gap = load_json(PRODUCTION_GAP_JSON)
    tests = negative.get("negative_tests", [])
    export_ids = {row["id"] for row in export_contract.get("export_records", [])}
    referenced_exports = {
        row["export_record"] for row in tests if row.get("export_record") is not None
    }
    verifier_states = [
        {
            "id": "reject_before_acceptance",
            "terminal": True,
            "deployment_accepts": False,
            "export_required": False,
            "description": "canonical decoding, carrier length, check bytes, transcript binding, or digest binding fails",
        },
        {
            "id": "accept",
            "terminal": True,
            "deployment_accepts": True,
            "export_required": False,
            "description": "all byte predicates, checked-core predicates, and hidden-source opening predicates pass",
        },
        {
            "id": "export_break_record",
            "terminal": True,
            "deployment_accepts": False,
            "export_required": True,
            "description": "public carrier predicate passes but a checked-core or hidden-source semantic bridge predicate fails",
        },
        {
            "id": "internal_error_fail_closed",
            "terminal": True,
            "deployment_accepts": False,
            "export_required": False,
            "description": "implementation lacks enough material to classify the failure; it must fail closed, never accept",
        },
    ]
    routing_rules = [
        {
            "id": "noncanonical_or_layout_failure",
            "applies_to": sorted(
                row["id"] for row in tests if row.get("export_record") is None
            ),
            "terminal_state": "reject_before_acceptance",
            "required_export": None,
        },
        {
            "id": "checked_core_or_response_link_failure",
            "applies_to": sorted(
                row["id"]
                for row in tests
                if row.get("export_record") == "checked_core_or_response_link_break_export"
            ),
            "terminal_state": "export_break_record",
            "required_export": "checked_core_or_response_link_break_export",
        },
        {
            "id": "hidden_source_opening_failure",
            "applies_to": sorted(
                row["id"]
                for row in tests
                if row.get("export_record")
                == "hidden_source_opening_binding_break_export"
            ),
            "terminal_state": "export_break_record",
            "required_export": "hidden_source_opening_binding_break_export",
        },
        {
            "id": "account_commitment_binding_failure",
            "applies_to": sorted(
                row["id"]
                for row in tests
                if row.get("export_record") == "account_commitment_binding_break_export"
            ),
            "terminal_state": "export_break_record",
            "required_export": "account_commitment_binding_break_export",
        },
        {
            "id": "pad_prf_or_opening_failure",
            "applies_to": sorted(
                row["id"]
                for row in tests
                if row.get("export_record") == "pad_prf_or_pad_opening_break_export"
            ),
            "terminal_state": "export_break_record",
            "required_export": "pad_prf_or_pad_opening_break_export",
        },
    ]
    api_requirements = [
        {
            "id": "decode_projection_carrier",
            "status": "specified_not_implemented",
            "inputs": ["projection_carrier_56_bytes"],
            "outputs": ["decoded_12_q12289_lanes", "layout_or_canonical_rejection"],
        },
        {
            "id": "verify_public_bridge_predicate",
            "status": "specified_not_implemented",
            "inputs": ["decoded_12_q12289_lanes", "projection_symbols", "carrier_check_bytes", "transcript_binding_bytes"],
            "outputs": ["bridge_predicate_accept", "reject_before_acceptance"],
        },
        {
            "id": "classify_checked_core_failure",
            "status": "specified_not_implemented",
            "inputs": ["checked_core_transcript", "response_difference_23_field_vector", "projection_carrier_56_bytes"],
            "outputs": ["checked_core_or_response_link_break_export", "internal_error_fail_closed"],
        },
        {
            "id": "classify_hidden_source_failure",
            "status": "specified_not_implemented",
            "inputs": ["hidden_relation_statement", "opening_frames", "projection_carrier_56_bytes"],
            "outputs": [
                "hidden_source_opening_binding_break_export",
                "account_commitment_binding_break_export",
                "pad_prf_or_pad_opening_break_export",
                "internal_error_fail_closed",
            ],
        },
    ]
    missing_referenced_exports = sorted(referenced_exports - export_ids)
    total_routed = sum(len(row["applies_to"]) for row in routing_rules)
    implemented_api_count = sum(
        1 for row in api_requirements if row["status"] == "implemented"
    )
    digest_payload = {
        "negative_test_digest": negative.get("target", {}).get("negative_test_digest"),
        "contract_digest": export_contract.get("target", {}).get("contract_digest"),
        "predicate_digest": public_bridge.get("target", {}).get("predicate_digest"),
        "verifier_states": verifier_states,
        "routing_rules": routing_rules,
        "api_requirements": api_requirements,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_verifier_exporter_contract.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "verifier_exporter_contract_specified_not_implemented_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "break_export_negative_tests": rel(NEGATIVE_TESTS_JSON),
            "break_export_contract": rel(EXPORT_CONTRACT_JSON),
            "public_bridge_predicate": rel(PUBLIC_BRIDGE_JSON),
            "production_reference_gap_report": rel(PRODUCTION_GAP_JSON),
            "break_export_negative_tests_schema": negative.get("schema"),
            "break_export_contract_schema": export_contract.get("schema"),
            "public_bridge_predicate_schema": public_bridge.get("schema"),
            "production_reference_gap_schema": production_gap.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "contract_digest": digest(digest_payload),
        },
        "verifier_states": verifier_states,
        "routing_rules": routing_rules,
        "api_requirements": api_requirements,
        "coverage": {
            "negative_test_count": len(tests),
            "routed_negative_test_count": total_routed,
            "referenced_export_records": sorted(referenced_exports),
            "missing_referenced_export_records": missing_referenced_exports,
            "api_requirement_count": len(api_requirements),
            "implemented_api_requirement_count": implemented_api_count,
            "signature_byte_increase": 0,
            "production_reference_deployment_ready": production_gap.get(
                "deployment_preflight", {}
            ).get("deployment_ready"),
        },
        "decision": {
            "can_accept_now": False,
            "can_run_exact_solver_now": False,
            "reason": "all negative tests are routed to reject or export behavior, but the verifier/exporter API remains specified rather than implemented",
            "next_artifact": "hidden_source_fallback_projection_verifier_exporter_implementation_gap",
        },
        "structural_controls": [
            {
                "id": "all_negative_tests_routed",
                "status": "pass" if total_routed == len(tests) else "fail",
                "observed": total_routed,
                "required": len(tests),
            },
            {
                "id": "all_referenced_exports_exist",
                "status": "pass" if not missing_referenced_exports else "fail",
                "observed": missing_referenced_exports,
                "required": [],
            },
            {
                "id": "unclassified_failure_cannot_accept",
                "status": "pass",
                "observed": "internal_error_fail_closed",
                "required": "deployment_accepts=false",
            },
            {
                "id": "no_signature_byte_growth",
                "status": "pass",
                "observed": 0,
                "required": 0,
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
    lines = [
        "# Hidden-Source Projection Verifier/Exporter Contract",
        "",
        "This artifact binds the projection-carrier negative-test matrix to",
        "verifier/exporter behavior. It is a contract, not the implementation.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- contract digest: `{target['contract_digest']}`",
        f"- negative tests routed: `{coverage['routed_negative_test_count']}/{coverage['negative_test_count']}`",
        f"- API requirements implemented: `{coverage['implemented_api_requirement_count']}/{coverage['api_requirement_count']}`",
        f"- signature byte increase: `{coverage['signature_byte_increase']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Terminal States",
        "",
        "| state | accepts | export required | description |",
        "|---|---:|---:|---|",
    ]
    for row in payload["verifier_states"]:
        lines.append(
            f"| `{row['id']}` | `{row['deployment_accepts']}` | `{row['export_required']}` | {row['description']} |"
        )
    lines.extend(
        [
            "",
            "## Routing Rules",
            "",
            "| rule | cases | terminal state | export |",
            "|---|---:|---|---|",
        ]
    )
    for row in payload["routing_rules"]:
        lines.append(
            f"| `{row['id']}` | `{len(row['applies_to'])}` | `{row['terminal_state']}` | `{row['required_export']}` |"
        )
    lines.extend(
        [
            "",
            "## API Requirements",
            "",
            "| requirement | status | outputs |",
            "|---|---|---|",
        ]
    )
    for row in payload["api_requirements"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{', '.join(row['outputs'])}` |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The reduction boundary is now explicit: every specified bridge-negative",
            "case routes either to rejection or to one concrete export record, and",
            "unclassified implementation failures route to fail-closed rejection. The",
            "remaining work is implementation and executable tests.",
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
