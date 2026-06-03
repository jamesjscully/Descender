#!/usr/bin/env python3
"""Record the production opening-projection equality boundary."""

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
    / "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.md"
)
SEMANTIC_PLAN_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"
)
SEMANTIC_BRIDGE_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json"
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
    h.update(b"Descender.hidden_source.opening_projection_boundary.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    semantic_plan = load_json(SEMANTIC_PLAN_JSON)
    semantic_bridge = load_json(SEMANTIC_BRIDGE_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    required_symbols = [
        "HiddenSourceFallbackProjectionOpeningProjectionReplayInput",
        "verify_hidden_source_fallback_opening_projection_or_export",
        "HiddenSourceFallbackProjectionHiddenFailure::OpeningProjectionMismatch",
        "HiddenSourceFallbackProjectionBreakExport::HiddenSourceOpeningBinding",
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
    boundary_rules = [
        {
            "id": "carrier_replay_exact",
            "status": "implemented_fail_closed",
            "accepts_if": "projection carrier bytes decode to exactly the caller-supplied decoded carrier",
            "rejects_if": "carrier length, q-canonical encoding, or decoded replay mismatch fails",
        },
        {
            "id": "claimed_projection_canonical",
            "status": "implemented_fail_closed",
            "accepts_if": "all claimed projection symbols are canonical q12289 residues",
            "rejects_if": "any claimed projection symbol is >= q",
        },
        {
            "id": "recomputed_projection_canonical",
            "status": "implemented_fail_closed",
            "accepts_if": "all recomputed projection values are canonical q12289 residues",
            "rejects_if": "any recomputed projection value is >= q",
        },
        {
            "id": "lane_claim_recompute_equality",
            "status": "implemented_export_boundary",
            "accepts_if": "carrier_lane[i] == claimed_projection[i] == recomputed_projection[i] for all 12 lanes",
            "exports_if": "any lane differs; the export names the first mismatching lane and carries statement/opening replay material",
        },
    ]
    digest_payload = {
        "semantic_plan_digest": semantic_plan.get("target", {}).get(
            "implementation_plan_digest"
        ),
        "semantic_bridge_digest": semantic_bridge.get("target", {}).get(
            "bridge_spec_digest"
        ),
        "symbol_rows": symbol_rows,
        "boundary_rules": boundary_rules,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_opening_projection_boundary.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "opening_projection_equality_boundary_implemented_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "semantic_predicate_implementation_plan": rel(SEMANTIC_PLAN_JSON),
            "semantic_bridge_spec": rel(SEMANTIC_BRIDGE_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "semantic_predicate_implementation_plan_schema": semantic_plan.get("schema"),
            "semantic_bridge_spec_schema": semantic_bridge.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "boundary_digest": digest(digest_payload),
        },
        "symbol_audit": symbol_rows,
        "boundary_rules": boundary_rules,
        "coverage": {
            "required_symbol_count": len(symbol_rows),
            "present_symbol_count": len(symbol_rows) - len(missing_symbols),
            "missing_symbol_count": len(missing_symbols),
            "missing_symbols": missing_symbols,
            "implemented_semantic_predicate_count": 3,
            "open_semantic_predicate_count": 2,
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "reason": "opening projection equality is now executable and fail-closed, but deriving recomputed values from byte-level hidden material, account split binding, and pad PRF derivation are not all internalized yet",
            "next_artifact": "hidden_source_fallback_projection_account_commitment_split_boundary",
        },
        "structural_controls": [
            {
                "id": "all_required_symbols_present",
                "status": "pass" if not missing_symbols else "fail",
                "observed": len(symbol_rows) - len(missing_symbols),
                "required": len(symbol_rows),
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
        "# Hidden-Source Projection Opening-Projection Boundary",
        "",
        "This artifact records the production-reference equality boundary for hidden",
        "opening projections. The boundary accepts only when the decoded carrier,",
        "claimed projection symbols, and recomputed projection values agree for all",
        "twelve selected lanes; otherwise it exports the hidden-opening break record.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- boundary digest: `{target['boundary_digest']}`",
        f"- present symbols: `{coverage['present_symbol_count']}` / `{coverage['required_symbol_count']}`",
        f"- implemented semantic predicates: `{coverage['implemented_semantic_predicate_count']}`",
        f"- open semantic predicates: `{coverage['open_semantic_predicate_count']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Boundary Rules",
        "",
        "| rule | status | fail-closed behavior |",
        "|---|---|---|",
    ]
    for row in payload["boundary_rules"]:
        behavior = row.get("rejects_if", row.get("exports_if", ""))
        lines.append(f"| `{row['id']}` | `{row['status']}` | {behavior} |")
    lines.extend(["", "## Source Artifacts", ""])
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
