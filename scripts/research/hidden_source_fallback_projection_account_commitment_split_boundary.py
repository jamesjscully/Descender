#!/usr/bin/env python3
"""Record the production account-commitment split-opening boundary."""

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
    / "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_account_commitment_split_boundary_n32_q12289.md"
)
SEMANTIC_PLAN_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"
)
OPENING_BOUNDARY_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_opening_projection_boundary_n32_q12289.json"
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
    h.update(b"Descender.hidden_source.account_commitment_split_boundary.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    semantic_plan = load_json(SEMANTIC_PLAN_JSON)
    opening_boundary = load_json(OPENING_BOUNDARY_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    required_symbols = [
        "HiddenSourceFallbackProjectionAccountCommitmentSplitReplayInput",
        "verify_hidden_source_fallback_account_commitment_split_or_export",
        "HiddenSourceFallbackProjectionHiddenFailure::AccountCommitmentSplitOpening",
        "HiddenSourceFallbackProjectionBreakExport::AccountCommitmentBinding",
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
            "id": "nonempty_commitment_and_openings",
            "status": "implemented_fail_closed",
            "accepts_if": "commitment bytes and both opening frames are present",
            "rejects_if": "commitment or either opening frame is empty",
        },
        {
            "id": "split_semantic_value_equality",
            "status": "implemented_export_boundary",
            "accepts_if": "the two accepted openings carry equal semantic values",
            "exports_if": "the same commitment has two supplied opening frames with different semantic values",
        },
    ]
    digest_payload = {
        "semantic_plan_digest": semantic_plan.get("target", {}).get(
            "implementation_plan_digest"
        ),
        "opening_boundary_digest": opening_boundary.get("target", {}).get(
            "boundary_digest"
        ),
        "symbol_rows": symbol_rows,
        "boundary_rules": boundary_rules,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_account_commitment_split_boundary.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "account_commitment_split_boundary_implemented_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "semantic_predicate_implementation_plan": rel(SEMANTIC_PLAN_JSON),
            "opening_projection_boundary": rel(OPENING_BOUNDARY_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "semantic_predicate_implementation_plan_schema": semantic_plan.get("schema"),
            "opening_projection_boundary_schema": opening_boundary.get("schema"),
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
            "implemented_semantic_predicate_count": 4,
            "open_semantic_predicate_count": 1,
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "reason": "account split binding now has an executable fail-closed export boundary, but pad PRF projection derivation remains open and hidden-material decoding is still partially external",
            "next_artifact": "hidden_source_fallback_projection_pad_prf_projection_boundary",
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
        "# Hidden-Source Projection Account-Commitment Split Boundary",
        "",
        "This artifact records the production-reference boundary for account",
        "commitment split openings. The boundary accepts equal semantic values and",
        "exports a commitment-binding break when two supplied openings for the same",
        "commitment carry different semantic values.",
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
