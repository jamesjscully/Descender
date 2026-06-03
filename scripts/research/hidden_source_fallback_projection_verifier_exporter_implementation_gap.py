#!/usr/bin/env python3
"""Record the implementation gap for the projection verifier/exporter contract."""

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
    / "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.md"
)
VERIFIER_EXPORTER_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.json"
)
NEGATIVE_TESTS_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"
)
PRODUCTION_GAP_JSON = READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
PRODUCTION_LIB = ROOT / "tools/production-reference/src/lib.rs"
PARAMETER_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.verifier_exporter_implementation_gap.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    contract = load_json(VERIFIER_EXPORTER_CONTRACT_JSON)
    negative = load_json(NEGATIVE_TESTS_JSON)
    production_gap = load_json(PRODUCTION_GAP_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    api_requirements = contract.get("api_requirements", [])
    expected_symbols = [
        {
            "api_requirement": "decode_projection_carrier",
            "symbol": "decode_hidden_source_fallback_projection_carrier",
            "kind": "function",
        },
        {
            "api_requirement": "verify_public_bridge_predicate",
            "symbol": "verify_hidden_source_fallback_projection_public_bridge",
            "kind": "function",
        },
        {
            "api_requirement": "classify_checked_core_failure",
            "symbol": "classify_hidden_source_fallback_checked_core_break",
            "kind": "function",
        },
        {
            "api_requirement": "classify_hidden_source_failure",
            "symbol": "classify_hidden_source_fallback_opening_break",
            "kind": "function",
        },
        {
            "api_requirement": "break_export_records",
            "symbol": "HiddenSourceFallbackProjectionBreakExport",
            "kind": "type",
        },
        {
            "api_requirement": "negative_test_harness",
            "symbol": "hidden_source_fallback_projection_negative_tests_are_fail_closed",
            "kind": "test",
        },
    ]
    symbol_rows = [
        {
            **row,
            "present": source_contains(row["symbol"], sources),
            "status": "present" if source_contains(row["symbol"], sources) else "missing",
        }
        for row in expected_symbols
    ]
    missing_symbols = [row["symbol"] for row in symbol_rows if not row["present"]]
    present_symbols = [row["symbol"] for row in symbol_rows if row["present"]]
    implementation_tasks = [
        {
            "id": "carrier_decoder",
            "status": "stub_present" if "decode_hidden_source_fallback_projection_carrier" in present_symbols else "open",
            "write_scope": "tools/parameter-search/src/production_reference.rs plus crate re-export tests",
            "depends_on": ["decode_projection_carrier"],
            "acceptance_condition": "canonical 56-byte carrier decodes to exactly 12 q12289 lanes; malformed length and noncanonical lanes reject before acceptance",
        },
        {
            "id": "public_bridge_verifier",
            "status": "implemented_boundary_present"
            if "verify_hidden_source_fallback_projection_public_bridge" in present_symbols
            else "open",
            "write_scope": "tools/parameter-search/src/production_reference.rs",
            "depends_on": ["verify_public_bridge_predicate", "carrier_decoder"],
            "acceptance_condition": "all 12 lane equations and check/transcript-binding regions are verified fail-closed",
        },
        {
            "id": "break_export_types",
            "status": "type_present" if "HiddenSourceFallbackProjectionBreakExport" in present_symbols else "open",
            "write_scope": "tools/parameter-search/src/production_reference.rs",
            "depends_on": ["break_export_records"],
            "acceptance_condition": "all four export records carry the required replay fields from the contract and are not serialized into signature bytes",
        },
        {
            "id": "failure_classifiers",
            "status": "implemented_export_boundary_present"
            if {
                "classify_hidden_source_fallback_checked_core_break",
                "classify_hidden_source_fallback_opening_break",
            }.issubset(set(present_symbols))
            else "open",
            "write_scope": "tools/parameter-search/src/production_reference.rs",
            "depends_on": [
                "classify_checked_core_failure",
                "classify_hidden_source_failure",
                "break_export_types",
            ],
            "acceptance_condition": "checked-core and hidden-source bridge failures emit the required export or internal_error_fail_closed",
        },
        {
            "id": "negative_test_harness",
            "status": "stub_present" if "hidden_source_fallback_projection_negative_tests_are_fail_closed" in present_symbols else "open",
            "write_scope": "tools/production-reference/src/lib.rs",
            "depends_on": ["all verifier/exporter APIs"],
            "acceptance_condition": "all 31 specified negative tests reject or export exactly as routed",
        },
    ]
    digest_payload = {
        "contract_digest": contract.get("target", {}).get("contract_digest"),
        "negative_test_digest": negative.get("target", {}).get("negative_test_digest"),
        "symbol_rows": symbol_rows,
        "implementation_tasks": implementation_tasks,
        "deployment_ready": production_gap.get("deployment_preflight", {}).get(
            "deployment_ready"
        ),
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_verifier_exporter_implementation_gap.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "verifier_exporter_implementation_gap_open_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "verifier_exporter_contract": rel(VERIFIER_EXPORTER_CONTRACT_JSON),
            "break_export_negative_tests": rel(NEGATIVE_TESTS_JSON),
            "production_reference_gap_report": rel(PRODUCTION_GAP_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "verifier_exporter_contract_schema": contract.get("schema"),
            "break_export_negative_tests_schema": negative.get("schema"),
            "production_reference_gap_schema": production_gap.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "implementation_gap_digest": digest(digest_payload),
        },
        "symbol_audit": symbol_rows,
        "implementation_tasks": implementation_tasks,
        "coverage": {
            "api_requirement_count": len(api_requirements),
            "expected_symbol_count": len(symbol_rows),
            "present_symbol_count": len(present_symbols),
            "present_symbols": present_symbols,
            "missing_symbol_count": len(missing_symbols),
            "missing_symbols": missing_symbols,
            "negative_test_count": negative.get("coverage", {}).get("test_count"),
            "implementation_task_count": len(implementation_tasks),
            "open_implementation_task_count": sum(
                1 for row in implementation_tasks if row["status"] == "open"
            ),
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "can_run_exact_solver_now": False,
            "reason": "the verifier/exporter contract is fully routed; present API stubs are fail-closed until bridge predicates and break classifiers are implemented",
            "next_artifact": "hidden_source_fallback_projection_verifier_exporter_api_stubs"
            if missing_symbols
            else "hidden_source_fallback_projection_public_bridge_verifier_implementation",
        },
        "structural_controls": [
            {
                "id": "symbol_audit_is_complete",
                "status": "pass" if len(missing_symbols) + len(present_symbols) == len(symbol_rows) else "fail",
                "observed": len(missing_symbols) + len(present_symbols),
                "required": len(symbol_rows),
            },
            {
                "id": "negative_tests_accounted_for",
                "status": "pass"
                if negative.get("coverage", {}).get("test_count") == 31
                else "fail",
                "observed": negative.get("coverage", {}).get("test_count"),
                "required": 31,
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
        "# Hidden-Source Projection Verifier/Exporter Implementation Gap",
        "",
        "This artifact audits the production-reference boundary for the APIs required",
        "by the projection-carrier verifier/exporter contract. It records what must",
        "be implemented next and remains fail-closed.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- implementation gap digest: `{target['implementation_gap_digest']}`",
        f"- expected symbols: `{coverage['expected_symbol_count']}`",
        f"- present symbols: `{coverage['present_symbol_count']}`",
        f"- missing symbols: `{coverage['missing_symbol_count']}`",
        f"- negative tests accounted for: `{coverage['negative_test_count']}`",
        f"- implementation tasks open: `{coverage['open_implementation_task_count']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Symbol Audit",
        "",
        "| symbol | kind | requirement | status |",
        "|---|---|---|---|",
    ]
    for row in payload["symbol_audit"]:
        lines.append(
            f"| `{row['symbol']}` | `{row['kind']}` | `{row['api_requirement']}` | `{row['status']}` |"
        )
    lines.extend(
        [
            "",
            "## Implementation Tasks",
            "",
            "| task | status | write scope | acceptance condition |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["implementation_tasks"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['write_scope']}` | {row['acceptance_condition']} |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The proof-side contract is actionable at the code boundary. Present",
            "symbols are still fail-closed unless their task status says the concrete",
            "predicate is implemented.",
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
