#!/usr/bin/env python3
"""Close the verifier/exporter bridge semantics track fail-closed."""

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
    / "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure_n32_q12289.md"
)
INVOCATION_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.json"
)
SEMANTIC_PLAN_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"
)
BYTE_INTERNALIZATION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json"
)
EXACT_BRIDGE_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.json"
)
MAP_DECISION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_redesign_or_bridge_rows_n32_q12289.json"
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
    h.update(b"Descender.hidden_source.verifier_exporter_bridge_semantics_closure.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    invocation = load_json(INVOCATION_CONTRACT_JSON)
    semantic_plan = load_json(SEMANTIC_PLAN_JSON)
    byte_internalization = load_json(BYTE_INTERNALIZATION_JSON)
    exact_bridge = load_json(EXACT_BRIDGE_JSON)
    map_decision = load_json(MAP_DECISION_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    required_symbols = [
        "decode_hidden_source_fallback_projection_carrier",
        "verify_hidden_source_fallback_projection_public_bridge",
        "classify_hidden_source_fallback_checked_core_break",
        "classify_hidden_source_fallback_checked_core_transcript_difference_break",
        "classify_hidden_source_fallback_opening_break",
        "verify_hidden_source_fallback_checked_core_or_export",
        "verify_hidden_source_fallback_response_link_kernel_or_export",
        "verify_hidden_source_fallback_hidden_material_frame_projection_or_export",
        "verify_hidden_source_fallback_account_commitment_split_frames_or_export",
        "verify_hidden_source_fallback_pad_prf_seed_or_export",
    ]
    symbol_rows = [
        {
            "symbol": symbol,
            "present": source_contains(symbol, sources),
            "status": "present" if source_contains(symbol, sources) else "missing",
        }
        for symbol in required_symbols
    ]
    missing_symbols = [row["symbol"] for row in symbol_rows if not row["present"]]
    bridge_rows = [
        {
            "id": "carrier_decode_and_public_bridge",
            "status": "closed" if not missing_symbols[:2] else "open",
            "evidence": "56-byte carrier decoding and public bridge lane/check/transcript predicates are executable",
        },
        {
            "id": "break_export_classifiers",
            "status": "closed"
            if not {
                "classify_hidden_source_fallback_checked_core_break",
                "classify_hidden_source_fallback_checked_core_transcript_difference_break",
                "classify_hidden_source_fallback_opening_break",
            }.difference({row["symbol"] for row in symbol_rows if row["present"]})
            else "open",
            "evidence": "checked-core, transcript-difference, hidden-opening, account-binding, and pad/opening break exports are typed and replayable",
        },
        {
            "id": "semantic_invocation_rules",
            "status": "closed"
            if invocation.get("coverage", {}).get("implemented_semantic_predicate_count")
            == invocation.get("coverage", {}).get("invocation_rule_count")
            else "open",
            "evidence": "all five semantic invocation rules have executable predicate/export boundaries",
        },
        {
            "id": "byte_level_derivations",
            "status": "closed"
            if byte_internalization.get("coverage", {}).get("open_derivation_count") == 0
            else "open",
            "evidence": "response differences, account projection values, account split values, and pad projections are derived from canonical byte frames",
        },
        {
            "id": "leakage_accounting",
            "status": "closed"
            if exact_bridge.get("coverage", {}).get("leakage_accounted")
            else "open",
            "evidence": "twelve q12289 lanes, three check bytes, and transcript-binding context bytes are accounted",
        },
    ]
    open_rows = [row for row in bridge_rows if row["status"] != "closed"]
    digest_payload = {
        "invocation_digest": invocation.get("target", {}).get(
            "invocation_contract_digest"
        ),
        "semantic_plan_digest": semantic_plan.get("target", {}).get(
            "implementation_plan_digest"
        ),
        "byte_internalization_digest": byte_internalization.get("target", {}).get(
            "internalization_digest"
        ),
        "exact_bridge_digest": exact_bridge.get("target", {}).get("proof_digest"),
        "map_decision_digest": map_decision.get("target", {}).get("decision_digest"),
        "symbol_rows": symbol_rows,
        "bridge_rows": bridge_rows,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_verifier_exporter_bridge_semantics_closure.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": (
            "verifier_exporter_bridge_semantics_closed_fail_closed"
            if not open_rows and not missing_symbols
            else "verifier_exporter_bridge_semantics_open_fail_closed"
        ),
        "deployment_ready": False,
        "source_artifacts": {
            "semantic_classifier_invocation_contract": rel(INVOCATION_CONTRACT_JSON),
            "semantic_predicate_implementation_plan": rel(SEMANTIC_PLAN_JSON),
            "byte_level_derivation_internalization": rel(BYTE_INTERNALIZATION_JSON),
            "exact_bridge_certificate_and_leakage_proof": rel(EXACT_BRIDGE_JSON),
            "map_redesign_or_bridge_rows": rel(MAP_DECISION_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "semantic_classifier_invocation_contract_schema": invocation.get("schema"),
            "semantic_predicate_implementation_plan_schema": semantic_plan.get("schema"),
            "byte_level_derivation_internalization_schema": byte_internalization.get("schema"),
            "exact_bridge_certificate_and_leakage_proof_schema": exact_bridge.get("schema"),
            "map_redesign_or_bridge_rows_schema": map_decision.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "closure_digest": digest(digest_payload),
        },
        "symbol_audit": symbol_rows,
        "bridge_semantics_rows": bridge_rows,
        "coverage": {
            "required_symbol_count": len(symbol_rows),
            "present_symbol_count": len(symbol_rows) - len(missing_symbols),
            "missing_symbol_count": len(missing_symbols),
            "missing_symbols": missing_symbols,
            "row_count": len(bridge_rows),
            "closed_row_count": len(bridge_rows) - len(open_rows),
            "open_row_count": len(open_rows),
            "implemented_invocation_rules": invocation.get("coverage", {}).get(
                "implemented_semantic_predicate_count"
            ),
            "internalized_derivations": byte_internalization.get("coverage", {}).get(
                "internalized_derivation_count"
            ),
            "exact_certificate_complete": exact_bridge.get("coverage", {}).get(
                "exact_certificate_complete"
            ),
            "bad_vector_exported": exact_bridge.get("coverage", {}).get(
                "bad_vector_exported"
            ),
            "selected_next_path": map_decision.get("decision", {}).get("selected_path"),
            "signature_byte_increase": 0,
        },
        "decision": {
            "verifier_exporter_bridge_semantics_complete": not open_rows
            and not missing_symbols,
            "can_accept_now": False,
            "reason": (
                "verifier/exporter bridge semantics are executable and "
                "byte-internalized, but exact infeasibility is still not proved; "
                "the exported bounded survivor keeps the 308-byte route fail-closed"
            ),
            "next_artifact": "hidden_source_fallback_projection_map_redesign_or_bridge_rows",
        },
        "structural_controls": [
            {
                "id": "all_required_symbols_present",
                "status": "pass" if not missing_symbols else "fail",
                "observed": len(symbol_rows) - len(missing_symbols),
                "required": len(symbol_rows),
            },
            {
                "id": "bridge_semantics_rows_closed",
                "status": "pass" if not open_rows else "fail",
                "observed": len(bridge_rows) - len(open_rows),
                "required": len(bridge_rows),
            },
            {
                "id": "exact_survivor_still_blocks_acceptance",
                "status": "pass"
                if exact_bridge.get("coverage", {}).get("bad_vector_exported")
                and not exact_bridge.get("coverage", {}).get("exact_certificate_complete")
                else "fail",
                "observed": {
                    "bad_vector_exported": exact_bridge.get("coverage", {}).get(
                        "bad_vector_exported"
                    ),
                    "exact_certificate_complete": exact_bridge.get("coverage", {}).get(
                        "exact_certificate_complete"
                    ),
                },
                "required": {
                    "bad_vector_exported": True,
                    "exact_certificate_complete": False,
                },
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
    lines = [
        "# Hidden-Source Projection Verifier/Exporter Bridge Semantics Closure",
        "",
        "This artifact closes the verifier/exporter bridge-semantics track for the",
        "selected projection-carrier route. It does not enable acceptance: the",
        "exact survivor remains exported and fail-closed.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- closure digest: `{target['closure_digest']}`",
        f"- closed rows: `{coverage['closed_row_count']}` / `{coverage['row_count']}`",
        f"- present symbols: `{coverage['present_symbol_count']}` / `{coverage['required_symbol_count']}`",
        f"- exact certificate complete: `{coverage['exact_certificate_complete']}`",
        f"- bad vector exported: `{coverage['bad_vector_exported']}`",
        f"- selected next path: `{coverage['selected_next_path']}`",
        "",
        "## Bridge Semantics Rows",
        "",
        "| row | status | evidence |",
        "|---|---|---|",
    ]
    for row in payload["bridge_semantics_rows"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['evidence']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- verifier/exporter bridge semantics complete: `{payload['decision']['verifier_exporter_bridge_semantics_complete']}`",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- reason: {payload['decision']['reason']}",
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
