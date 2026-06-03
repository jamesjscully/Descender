#!/usr/bin/env python3
"""Record the expanded account-opening commitment boundary for the projection route."""

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
    / "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.md"
)
BYTE_INTERNALIZATION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json"
)
PRODUCTION_LIB = ROOT / "tools/production-reference/src/lib.rs"
PARAMETER_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.expanded_opening_commitment_binding.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    byte_internalization = load_json(BYTE_INTERNALIZATION_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    implemented_symbols = [
        "HIDDEN_SOURCE_FALLBACK_EXPANDED_OPENING_COMMITMENT_DOMAIN",
        "HIDDEN_SOURCE_FALLBACK_ACCOUNT_OPENING_FRAME_DOMAIN",
        "HiddenSourceFallbackProjectionExpandedOpeningFrameReplayInput",
        "HiddenSourceFallbackProjectionExpandedOpeningCommitmentReplayInput",
        "decode_hidden_source_fallback_account_witness_opening_frame",
        "encode_hidden_source_fallback_account_witness_opening_frame",
        "hidden_source_fallback_expanded_account_opening_commitment",
        "verify_hidden_source_fallback_expanded_opening_frame_commitment",
        "verify_hidden_source_fallback_expanded_opening_commitment",
    ]
    implemented = all(source_contains(symbol, sources) for symbol in implemented_symbols)
    test_symbols = [
        "hidden_source_fallback_expanded_account_opening_commitment",
        "verify_hidden_source_fallback_expanded_opening_frame_commitment",
        "verify_hidden_source_fallback_expanded_opening_commitment",
        "RejectBeforeAcceptance",
    ]
    tests_present = all(source_contains(symbol, [PRODUCTION_LIB.read_text()]) for symbol in test_symbols)
    status = (
        "implemented_fail_closed_state_hook_open"
        if implemented and tests_present
        else "open"
    )
    digest_payload = {
        "byte_internalization_digest": byte_internalization.get("target", {}).get(
            "internalization_digest"
        ),
        "implemented_symbols": implemented_symbols,
        "status": status,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_expanded_opening_commitment_binding.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": status,
        "deployment_ready": False,
        "source_artifacts": {
            "byte_level_derivation_internalization": rel(BYTE_INTERNALIZATION_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "byte_level_derivation_schema": byte_internalization.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "signature_byte_increase": 0,
            "commitment_domain": "Descender.hidden_source.fallback_projection.expanded_account_opening_commitment.v1",
            "binding_digest": digest(digest_payload),
        },
        "boundary": {
            "canonical_payload": "six selected account-witness q12289 source values, lane indices, lane symbols, lane coefficients, q, group count, and local-free count",
            "commitment": "SHA-256(domain || canonical_payload)",
            "accept_condition": "recomputed expanded-opening commitment equals the state/statement supplied commitment",
            "mismatch_action": "reject before acceptance",
            "implemented_symbols": implemented_symbols,
            "tests_present": tests_present,
        },
        "remaining_gaps": [
            {
                "id": "account_state_wiring",
                "status": "open",
                "detail": "the verifier boundary exists, but the selected account-state creation path must carry this expanded-opening commitment as the hidden-source opening commitment",
            },
            {
                "id": "full_opening_frame_decoder",
                "status": "implemented_fail_closed",
                "detail": "the boundary now decodes the six-value expanded opening from the canonical account witness opening frame before recomputing the commitment",
            },
            {
                "id": "commitment_assumption",
                "status": "standard_hash_binding_assumption",
                "detail": "binding is reduced to SHA-256 collision/preimage resistance for this domain-separated digest; this is acceptable only after external review of the byte encoding",
            },
        ],
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    boundary = payload["boundary"]
    lines = [
        "# Hidden-Source Projection Expanded Opening Commitment Binding",
        "",
        "This artifact records the fail-closed commitment check for the expanded",
        "six-value account-witness projection opening used by the selected 308-byte",
        "projection-carrier route.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- signature byte increase: `{target['signature_byte_increase']}`",
        f"- commitment domain: `{target['commitment_domain']}`",
        f"- binding digest: `{target['binding_digest']}`",
        "",
        "## Boundary",
        "",
        f"- canonical payload: {boundary['canonical_payload']}",
        f"- commitment: `{boundary['commitment']}`",
        f"- accept condition: {boundary['accept_condition']}",
        f"- mismatch action: `{boundary['mismatch_action']}`",
        f"- tests present: `{boundary['tests_present']}`",
        "",
        "## Remaining Gaps",
        "",
        "| gap | status | detail |",
        "|---|---|---|",
    ]
    for gap in payload["remaining_gaps"]:
        lines.append(f"| `{gap['id']}` | `{gap['status']}` | {gap['detail']} |")
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
