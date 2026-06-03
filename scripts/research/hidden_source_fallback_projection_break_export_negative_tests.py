#!/usr/bin/env python3
"""Specify negative tests for projection-carrier break export behavior."""

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
    / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.md"
)
EXPORT_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_contract_n32_q12289.json"
)
PUBLIC_BRIDGE_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json"
)

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.break_export_negative_tests.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def lane_mutation_tests(lane_checks: list[dict[str, Any]]) -> list[dict[str, Any]]:
    tests: list[dict[str, Any]] = []
    for row in lane_checks:
        lane = row["lane"]
        tests.extend(
            [
                {
                    "id": f"lane_{lane}_noncanonical_u16",
                    "class": "carrier_to_projection",
                    "mutation": f"set bytes[{row['byte_offset']}:{row['byte_offset'] + row['byte_length']}] to a little-endian value >= {Q}",
                    "expected_result": "reject_before_acceptance",
                    "export_record": None,
                    "status": "specified_not_implemented",
                },
                {
                    "id": f"lane_{lane}_canonical_value_mismatch",
                    "class": "carrier_to_projection",
                    "mutation": f"set decoded lane {lane} to a different canonical q12289 value",
                    "expected_result": "reject_or_export_checked_core_or_response_link_break",
                    "export_record": "checked_core_or_response_link_break_export",
                    "status": "specified_not_implemented",
                },
            ]
        )
    return tests


def build_payload() -> dict[str, Any]:
    contract = load_json(EXPORT_CONTRACT_JSON)
    bridge = load_json(PUBLIC_BRIDGE_JSON)
    export_ids = {row["id"] for row in contract.get("export_records", [])}
    lane_checks = bridge.get("predicate", {}).get("lane_checks", [])
    tests = lane_mutation_tests(lane_checks)
    tests.extend(
        [
            {
                "id": "carrier_check_region_mutation",
                "class": "carrier_layout_binding",
                "mutation": "mutate bytes[24:27]",
                "expected_result": "reject_before_acceptance",
                "export_record": None,
                "status": "specified_not_implemented",
            },
            {
                "id": "carrier_transcript_binding_region_mutation",
                "class": "carrier_layout_binding",
                "mutation": "mutate bytes[27:56]",
                "expected_result": "reject_before_acceptance",
                "export_record": None,
                "status": "specified_not_implemented",
            },
            {
                "id": "checked_core_response_relation_mismatch",
                "class": "projection_to_checked_core",
                "mutation": "keep public bridge predicate satisfied but provide a checked-core response vector that violates a replayed checked-core equation",
                "expected_result": "export_checked_core_or_response_link_break",
                "export_record": "checked_core_or_response_link_break_export",
                "status": "specified_not_implemented",
            },
            {
                "id": "response_link_kernel_certificate_mismatch",
                "class": "projection_to_checked_core",
                "mutation": "keep carrier lanes fixed but supply a response difference outside the bounded-kernel certificate preconditions",
                "expected_result": "export_checked_core_or_response_link_break",
                "export_record": "checked_core_or_response_link_break_export",
                "status": "specified_not_implemented",
            },
            {
                "id": "hidden_source_opening_projection_mismatch",
                "class": "projection_to_hidden_source_opening",
                "mutation": "keep carrier lanes fixed but alter account or pad opening so recomputed projection symbols differ",
                "expected_result": "export_hidden_relation_proof_soundness_break",
                "export_record": "hidden_source_opening_binding_break_export",
                "status": "specified_not_implemented",
            },
            {
                "id": "account_commitment_split_opening",
                "class": "projection_to_hidden_source_opening",
                "mutation": "produce two accepted openings for the same account commitment with different semantic witness or pad values",
                "expected_result": "export_account_commitment_binding_break",
                "export_record": "account_commitment_binding_break_export",
                "status": "specified_not_implemented",
            },
            {
                "id": "pad_prf_projection_derivation_mismatch",
                "class": "projection_to_hidden_source_opening",
                "mutation": "keep pad opening accepted while recomputed pad PRF projection values differ from carrier lanes",
                "expected_result": "export_pad_prf_or_pad_opening_break",
                "export_record": "pad_prf_or_pad_opening_break_export",
                "status": "specified_not_implemented",
            },
        ]
    )
    export_referenced = {
        row["export_record"] for row in tests if row.get("export_record") is not None
    }
    missing_exports = sorted(export_referenced - export_ids)
    implemented_count = sum(1 for row in tests if row["status"] == "implemented")
    reject_only_count = sum(1 for row in tests if row["export_record"] is None)
    export_test_count = len(tests) - reject_only_count
    digest_payload = {
        "contract_digest": contract.get("target", {}).get("contract_digest"),
        "predicate_digest": bridge.get("target", {}).get("predicate_digest"),
        "tests": tests,
        "missing_exports": missing_exports,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_break_export_negative_tests.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "break_export_negative_tests_specified_not_implemented_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "break_export_contract": rel(EXPORT_CONTRACT_JSON),
            "public_bridge_predicate": rel(PUBLIC_BRIDGE_JSON),
            "break_export_contract_schema": contract.get("schema"),
            "public_bridge_predicate_schema": bridge.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "negative_test_digest": digest(digest_payload),
        },
        "negative_tests": tests,
        "coverage": {
            "test_count": len(tests),
            "reject_only_test_count": reject_only_count,
            "export_test_count": export_test_count,
            "implemented_test_count": implemented_count,
            "referenced_export_records": sorted(export_referenced),
            "missing_export_records": missing_exports,
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "can_run_exact_solver_now": False,
            "reason": "negative tests are specified and reference concrete export records, but verifier/exporter implementation is still open",
            "next_artifact": "hidden_source_fallback_projection_verifier_exporter_contract",
        },
        "structural_controls": [
            {
                "id": "all_referenced_exports_exist",
                "status": "pass" if not missing_exports else "fail",
                "observed": missing_exports,
                "required": [],
            },
            {
                "id": "lane_mutations_cover_all_lanes",
                "status": "pass" if len(lane_checks) == 12 else "fail",
                "observed": len(lane_checks),
                "required": 12,
            },
            {
                "id": "no_signature_byte_growth",
                "status": "pass",
                "observed": 0,
                "required": 0,
            },
            {
                "id": "acceptance_still_fail_closed",
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
    lines = [
        "# Hidden-Source Projection Break Export Negative Tests",
        "",
        "This artifact specifies the negative tests needed to enforce the",
        "projection-carrier break-export contract. The tests are specified but not",
        "yet implemented in the verifier/exporter.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- negative test digest: `{target['negative_test_digest']}`",
        f"- tests: `{coverage['test_count']}`",
        f"- reject-only tests: `{coverage['reject_only_test_count']}`",
        f"- export tests: `{coverage['export_test_count']}`",
        f"- implemented tests: `{coverage['implemented_test_count']}`",
        f"- signature byte increase: `{coverage['signature_byte_increase']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Test Matrix",
        "",
        "| test | class | expected result | export | status |",
        "|---|---|---|---|---|",
    ]
    for row in payload["negative_tests"]:
        lines.append(
            f"| `{row['id']}` | `{row['class']}` | `{row['expected_result']}` | "
            f"`{row.get('export_record')}` | `{row['status']}` |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "This closes the test plan shape, not the implementation. The next proof",
            "artifact needs to bind these cases to an actual verifier/exporter contract",
            "so bad bridge behavior cannot be accepted silently.",
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
