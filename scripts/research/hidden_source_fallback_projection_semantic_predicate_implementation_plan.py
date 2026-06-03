#!/usr/bin/env python3
"""Track executable semantic predicate replay progress for the projection route."""

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
    / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.md"
)
INVOCATION_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.json"
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
    h.update(b"Descender.hidden_source.semantic_predicate_implementation_plan.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    invocation = load_json(INVOCATION_CONTRACT_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    predicate_rows = [
        {
            "id": "checked_core_response_relation_mismatch",
            "implementation_symbol": "verify_hidden_source_fallback_checked_core_or_export",
            "status": "implemented_export_invocation_boundary"
            if source_contains("verify_hidden_source_fallback_checked_core_or_export", sources)
            else "open",
            "scope": "replays the existing reviewed Fq12 checked-core verifier and invokes checked-core export packaging on failure",
            "remaining_gap": "closed by checked-core transcript-difference frame decoding and export classification",
        },
        {
            "id": "response_link_kernel_certificate_mismatch",
            "implementation_symbol": "verify_hidden_source_fallback_response_link_kernel_or_export",
            "status": "implemented_certificate_summary_boundary"
            if source_contains("verify_hidden_source_fallback_response_link_kernel_or_export", sources)
            else "open",
            "scope": "replay the bounded response-link kernel certificate against the projection carrier",
            "remaining_gap": "closed for verifier/exporter semantics by validating the pinned exact-certificate summary and bounded survivor condition",
        },
        {
            "id": "hidden_source_opening_projection_mismatch",
            "implementation_symbol": "verify_hidden_source_fallback_opening_projection_or_export",
            "status": "implemented_projection_equality_boundary"
            if source_contains(
                "verify_hidden_source_fallback_opening_projection_or_export", sources
            )
            else "open",
            "scope": "compare decoded carrier lanes, claimed projection symbols, and recomputed opening projections lane-by-lane",
            "remaining_gap": "closed by hidden-material frame replay and projection derivation from decoded account opening plus pad seed",
        },
        {
            "id": "account_commitment_split_opening",
            "implementation_symbol": "verify_hidden_source_fallback_account_commitment_split_or_export",
            "status": "implemented_split_opening_export_boundary"
            if source_contains(
                "verify_hidden_source_fallback_account_commitment_split_or_export",
                sources,
            )
            else "open",
            "scope": "compare two supplied openings for the same account commitment and export when semantic values differ",
            "remaining_gap": "closed by canonical opening-frame decoding before semantic-value comparison",
        },
        {
            "id": "pad_prf_projection_derivation_mismatch",
            "implementation_symbol": "verify_hidden_source_fallback_pad_prf_projection_or_export",
            "status": "implemented_pad_projection_export_boundary"
            if source_contains(
                "verify_hidden_source_fallback_pad_prf_projection_or_export", sources
            )
            else "open",
            "scope": "map selected pad projection symbols to carrier lanes and export when recomputed pad projections differ",
            "remaining_gap": "closed by pad-seed replay through the reviewed Fq12 pad PRF before carrier comparison",
        },
    ]
    implemented_count = sum(1 for row in predicate_rows if row["status"] != "open")
    digest_payload = {
        "invocation_contract_digest": invocation.get("target", {}).get(
            "invocation_contract_digest"
        ),
        "predicate_rows": predicate_rows,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_semantic_predicate_implementation_plan.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": (
            "semantic_predicate_implementation_closed_fail_closed"
            if implemented_count == len(predicate_rows)
            else "semantic_predicate_implementation_partially_open_fail_closed"
        ),
        "deployment_ready": False,
        "source_artifacts": {
            "semantic_classifier_invocation_contract": rel(INVOCATION_CONTRACT_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "semantic_classifier_invocation_contract_schema": invocation.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "implementation_plan_digest": digest(digest_payload),
        },
        "semantic_predicates": predicate_rows,
        "coverage": {
            "semantic_predicate_count": len(predicate_rows),
            "implemented_semantic_predicate_count": implemented_count,
            "open_semantic_predicate_count": len(predicate_rows) - implemented_count,
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "reason": (
                "all five semantic predicate export boundaries are executable "
                "and their byte-level derivation hooks are present; acceptance "
                "remains fail-closed until the exact bridge certificate/map "
                "survivor is resolved"
                if implemented_count == len(predicate_rows)
                else "all five semantic predicate export boundaries are executable, but response-difference and hidden-material derivations are still partly external to the production-reference boundary"
            ),
            "next_artifact": "hidden_source_fallback_projection_byte_level_derivation_internalization_plan",
        },
        "structural_controls": [
            {
                "id": "checked_core_invocation_boundary_present",
                "status": "pass"
                if source_contains("verify_hidden_source_fallback_checked_core_or_export", sources)
                else "fail",
                "observed": source_contains(
                    "verify_hidden_source_fallback_checked_core_or_export", sources
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
        "# Hidden-Source Projection Semantic Predicate Implementation Plan",
        "",
        "This artifact tracks executable semantic predicate replay progress for the",
        "selected projection-carrier route.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- implementation plan digest: `{target['implementation_plan_digest']}`",
        f"- semantic predicates: `{coverage['semantic_predicate_count']}`",
        f"- implemented semantic predicates: `{coverage['implemented_semantic_predicate_count']}`",
        f"- open semantic predicates: `{coverage['open_semantic_predicate_count']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Predicate Status",
        "",
        "| predicate | symbol | status | remaining gap |",
        "|---|---|---|---|",
    ]
    for row in payload["semantic_predicates"]:
        lines.append(
            f"| `{row['id']}` | `{row['implementation_symbol']}` | `{row['status']}` | {row['remaining_gap']} |"
        )
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
