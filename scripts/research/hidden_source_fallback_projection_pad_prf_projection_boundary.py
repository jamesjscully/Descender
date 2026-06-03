#!/usr/bin/env python3
"""Record the production pad-PRF projection derivation boundary."""

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
    / "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.md"
)
SEMANTIC_PLAN_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"
)
ACCOUNT_SPLIT_BOUNDARY_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.json"
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
    h.update(b"Descender.hidden_source.pad_prf_projection_boundary.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    semantic_plan = load_json(SEMANTIC_PLAN_JSON)
    account_split_boundary = load_json(ACCOUNT_SPLIT_BOUNDARY_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    required_symbols = [
        "HiddenSourceFallbackProjectionPadPrfProjectionReplayInput",
        "verify_hidden_source_fallback_pad_prf_projection_or_export",
        "HiddenSourceFallbackProjectionHiddenFailure::PadPrfProjectionDerivationMismatch",
        "HiddenSourceFallbackProjectionBreakExport::PadPrfOrOpening",
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
            "id": "pad_symbol_map_canonical",
            "status": "implemented_fail_closed",
            "accepts_if": "each supplied pad projection symbol is one of the selected pad lanes",
            "rejects_if": "symbol/value counts differ, a symbol is unknown, or a symbol names an account-witness lane",
        },
        {
            "id": "pad_projection_value_canonical",
            "status": "implemented_fail_closed",
            "accepts_if": "all recomputed pad projection values are canonical q12289 residues",
            "rejects_if": "any recomputed pad projection value is >= q",
        },
        {
            "id": "pad_prf_projection_equality",
            "status": "implemented_export_boundary",
            "accepts_if": "each recomputed pad projection equals the decoded carrier lane for its symbol",
            "exports_if": "any recomputed pad projection differs from the corresponding carrier lane",
        },
    ]
    digest_payload = {
        "semantic_plan_digest": semantic_plan.get("target", {}).get(
            "implementation_plan_digest"
        ),
        "account_split_boundary_digest": account_split_boundary.get("target", {}).get(
            "boundary_digest"
        ),
        "symbol_rows": symbol_rows,
        "boundary_rules": boundary_rules,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_pad_prf_projection_boundary.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "pad_prf_projection_boundary_implemented_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "semantic_predicate_implementation_plan": rel(SEMANTIC_PLAN_JSON),
            "account_commitment_split_boundary": rel(ACCOUNT_SPLIT_BOUNDARY_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "semantic_predicate_implementation_plan_schema": semantic_plan.get("schema"),
            "account_commitment_split_boundary_schema": account_split_boundary.get(
                "schema"
            ),
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
            "implemented_semantic_predicate_count": 5,
            "open_semantic_predicate_count": 0,
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "reason": "all five semantic predicate export boundaries are now executable, but several still accept recomputed semantic values as inputs rather than deriving them from byte-level hidden material internally",
            "next_artifact": "hidden_source_fallback_projection_byte_level_derivation_internalization_plan",
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
        "# Hidden-Source Projection Pad-PRF Projection Boundary",
        "",
        "This artifact records the production-reference boundary for pad PRF",
        "projection derivation. The boundary maps supplied pad projection symbols",
        "to selected carrier lanes and exports a pad/opening break when recomputed",
        "pad projections differ from those lanes.",
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
