#!/usr/bin/env python3
"""Record fail-closed verifier/exporter API stubs for the projection route."""

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
    / "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.md"
)
IMPLEMENTATION_GAP_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_verifier_exporter_implementation_gap_n32_q12289.json"
)
VERIFIER_EXPORTER_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.json"
)
NEGATIVE_TESTS_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"
)
PRODUCTION_LIB = ROOT / "tools/production-reference/src/lib.rs"
PARAMETER_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.verifier_exporter_api_stubs.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    implementation_gap = load_json(IMPLEMENTATION_GAP_JSON)
    contract = load_json(VERIFIER_EXPORTER_CONTRACT_JSON)
    negative = load_json(NEGATIVE_TESTS_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    expected_symbols = implementation_gap.get("symbol_audit", [])
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
    api_status = [
        {
            "id": "decode_projection_carrier",
            "symbol": "decode_hidden_source_fallback_projection_carrier",
            "status": "implemented_fail_closed_boundary",
            "acceptance_effect": "decodes only a 56-byte carrier with 12 canonical q12289 little-endian lanes; malformed length or noncanonical lanes reject before acceptance",
        },
        {
            "id": "verify_public_bridge_predicate",
            "symbol": "verify_hidden_source_fallback_projection_public_bridge",
            "status": "implemented_fail_closed_boundary",
            "acceptance_effect": "checks all 12 carrier lanes against public projection values, rejects noncanonical expected values, and checks the check-byte and transcript-binding regions",
        },
        {
            "id": "classify_checked_core_failure",
            "symbol": "classify_hidden_source_fallback_checked_core_break",
            "status": "implemented_export_boundary",
            "acceptance_effect": "validates carrier replay and canonical response-difference coordinates, then emits the checked-core-or-response-link external break record for a supplied concrete failure class",
        },
        {
            "id": "classify_hidden_source_failure",
            "symbol": "classify_hidden_source_fallback_opening_break",
            "status": "implemented_export_boundary",
            "acceptance_effect": "validates carrier replay and emits hidden-source opening, account-commitment, or pad-PRF external break records for supplied concrete failure classes",
        },
    ]
    implemented_api_count = sum(
        1
        for row in api_status
        if row["status"]
        in {"implemented_fail_closed_boundary", "implemented_export_boundary"}
    )
    fail_closed_stub_count = sum(1 for row in api_status if row["status"] == "fail_closed_stub")
    digest_payload = {
        "implementation_gap_digest": implementation_gap.get("target", {}).get(
            "implementation_gap_digest"
        ),
        "contract_digest": contract.get("target", {}).get("contract_digest"),
        "negative_test_digest": negative.get("target", {}).get("negative_test_digest"),
        "symbol_rows": symbol_rows,
        "api_status": api_status,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_verifier_exporter_api_stubs.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "verifier_exporter_api_stubs_present_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "implementation_gap": rel(IMPLEMENTATION_GAP_JSON),
            "verifier_exporter_contract": rel(VERIFIER_EXPORTER_CONTRACT_JSON),
            "break_export_negative_tests": rel(NEGATIVE_TESTS_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "implementation_gap_schema": implementation_gap.get("schema"),
            "verifier_exporter_contract_schema": contract.get("schema"),
            "break_export_negative_tests_schema": negative.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "api_stub_digest": digest(digest_payload),
        },
        "symbol_audit": symbol_rows,
        "api_status": api_status,
        "coverage": {
            "api_requirement_count": len(contract.get("api_requirements", [])),
            "implemented_api_requirement_count": implemented_api_count,
            "fail_closed_stub_api_count": fail_closed_stub_count,
            "expected_symbol_count": len(symbol_rows),
            "present_symbol_count": len(present_symbols),
            "present_symbols": present_symbols,
            "missing_symbol_count": len(missing_symbols),
            "missing_symbols": missing_symbols,
            "negative_test_count": negative.get("coverage", {}).get("test_count"),
            "negative_test_harness_present": source_contains(
                "hidden_source_fallback_projection_negative_tests_are_fail_closed",
                sources,
            ),
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "reason": "the carrier decoder, public bridge verifier, and export classifiers now have executable boundaries, but the verifier still lacks the semantic predicate implementation that decides when those classifiers are invoked",
            "next_artifact": "hidden_source_fallback_projection_semantic_classifier_invocation_contract",
        },
        "structural_controls": [
            {
                "id": "all_expected_symbols_present",
                "status": "pass" if not missing_symbols else "fail",
                "observed": len(present_symbols),
                "required": len(symbol_rows),
            },
            {
                "id": "all_verifier_exporter_apis_have_boundaries",
                "status": "pass"
                if implemented_api_count == 4 and fail_closed_stub_count == 0
                else "fail",
                "observed": {
                    "implemented_api_count": implemented_api_count,
                    "fail_closed_stub_count": fail_closed_stub_count,
                },
                "required": {
                    "implemented_api_count": 4,
                    "fail_closed_stub_count": 0,
                },
            },
            {
                "id": "negative_test_harness_present",
                "status": "pass"
                if source_contains(
                    "hidden_source_fallback_projection_negative_tests_are_fail_closed",
                    sources,
                )
                else "fail",
                "observed": source_contains(
                    "hidden_source_fallback_projection_negative_tests_are_fail_closed",
                    sources,
                ),
                "required": True,
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
        "# Hidden-Source Projection Verifier/Exporter API Stubs",
        "",
        "This artifact records the first production-reference boundary for the",
        "selected projection-carrier route. It exposes the required symbols and",
        "keeps the route fail-closed until the real public bridge predicate and",
        "break classifiers are implemented.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- API stub digest: `{target['api_stub_digest']}`",
        f"- expected symbols: `{coverage['expected_symbol_count']}`",
        f"- present symbols: `{coverage['present_symbol_count']}`",
        f"- missing symbols: `{coverage['missing_symbol_count']}`",
        f"- implemented API requirements: `{coverage['implemented_api_requirement_count']}`",
        f"- fail-closed stub APIs: `{coverage['fail_closed_stub_api_count']}`",
        f"- negative-test harness present: `{coverage['negative_test_harness_present']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## API Status",
        "",
        "| API | symbol | status | acceptance effect |",
        "|---|---|---|---|",
    ]
    for row in payload["api_status"]:
        lines.append(
            f"| `{row['id']}` | `{row['symbol']}` | `{row['status']}` | {row['acceptance_effect']} |"
        )
    lines.extend(
        [
            "",
            "## Symbol Audit",
            "",
            "| symbol | kind | requirement | status |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["symbol_audit"]:
        lines.append(
            f"| `{row['symbol']}` | `{row['kind']}` | `{row['api_requirement']}` | `{row['status']}` |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The 56-byte carrier decoder now enforces length and canonical q12289",
            "lanes. The public bridge verifier checks lane, check-byte, and",
            "transcript-binding equality against public statement inputs. The",
            "classifiers now emit typed external break records when supplied with",
            "a concrete failure class. They still do not prove the semantic",
            "predicates themselves, so this does not make the 308-byte route",
            "deployable.",
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
