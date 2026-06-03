#!/usr/bin/env python3
"""Record byte-level derivation internalization progress for the projection route."""

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
    / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.md"
)
PAD_BOUNDARY_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_pad_prf_projection_boundary_n32_q12289.json"
)
SEMANTIC_PLAN_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"
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
    h.update(b"Descender.hidden_source.byte_level_derivation_internalization.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    pad_boundary = load_json(PAD_BOUNDARY_JSON)
    semantic_plan = load_json(SEMANTIC_PLAN_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    rows = [
        {
            "id": "pad_prf_projection_from_seed",
            "status": "internalized"
            if source_contains(
                "verify_hidden_source_fallback_pad_prf_seed_or_export", sources
            )
            and source_contains(
                "hidden_source_fallback_pad_prf_projection_values_from_seed", sources
            )
            else "open",
            "implemented_symbols": [
                "HiddenSourceFallbackProjectionPadPrfSeedReplayInput",
                "hidden_source_fallback_pad_prf_projection_values_from_seed",
                "verify_hidden_source_fallback_pad_prf_seed_or_export",
            ],
            "remaining_gap": "none for the selected pad projection lanes; this still relies on SHA-256 pad PRF assumption already tracked elsewhere",
        },
        {
            "id": "hidden_opening_account_witness_projection_from_bytes",
            "status": "internalized_frame_decoder"
            if source_contains(
                "verify_hidden_source_fallback_hidden_material_projection_or_export",
                sources,
            )
            and source_contains(
                "verify_hidden_source_fallback_hidden_material_frame_projection_or_export",
                sources,
            )
            and source_contains(
                "decode_hidden_source_fallback_account_witness_opening_frame",
                sources,
            )
            and source_contains(
                "hidden_source_fallback_account_projection_values_from_opening",
                sources,
            )
            else "open",
            "implemented_symbols": [
                "HiddenSourceFallbackProjectionAccountWitnessOpening",
                "decode_hidden_source_fallback_account_witness_opening_frame",
                "encode_hidden_source_fallback_account_witness_opening_frame",
                "hidden_source_fallback_account_projection_values_from_opening",
                "hidden_source_fallback_projection_values_from_hidden_material",
                "verify_hidden_source_fallback_hidden_material_frame_projection_or_export",
                "verify_hidden_source_fallback_hidden_material_projection_or_export",
            ],
            "remaining_gap": "none for the selected account projection lanes; the six q12289 source values are decoded from the canonical account opening frame before projection derivation",
        },
        {
            "id": "checked_core_response_difference_from_two_transcripts",
            "status": "internalized"
            if source_contains(
                "HiddenSourceFallbackProjectionCheckedCoreTranscriptDifferenceInput",
                sources,
            )
            and source_contains(
                "decode_hidden_source_fallback_checked_core_transcript_frame",
                sources,
            )
            and source_contains(
                "hidden_source_fallback_checked_core_response_difference_from_transcripts",
                sources,
            )
            and source_contains(
                "classify_hidden_source_fallback_checked_core_transcript_difference_break",
                sources,
            )
            else "open",
            "implemented_symbols": [
                "HiddenSourceFallbackProjectionCheckedCoreTranscriptDifferenceInput",
                "encode_hidden_source_fallback_checked_core_transcript_frame",
                "decode_hidden_source_fallback_checked_core_transcript_frame",
                "hidden_source_fallback_checked_core_response_difference_from_transcripts",
                "classify_hidden_source_fallback_checked_core_transcript_difference_break",
            ],
            "remaining_gap": "none for byte-level response-difference derivation; two canonical checked-core transcript frames now decode to the 23-coordinate response difference before break export",
        },
        {
            "id": "account_commitment_split_semantic_values_from_opening_frames",
            "status": "internalized"
            if source_contains(
                "verify_hidden_source_fallback_account_commitment_split_frames_or_export",
                sources,
            )
            and source_contains(
                "hidden_source_fallback_account_witness_opening_semantic_value",
                sources,
            )
            else "open",
            "implemented_symbols": [
                "HiddenSourceFallbackProjectionAccountCommitmentSplitFrameReplayInput",
                "decode_hidden_source_fallback_account_witness_opening_frame",
                "hidden_source_fallback_account_witness_opening_semantic_value",
                "verify_hidden_source_fallback_account_commitment_split_frames_or_export",
            ],
            "remaining_gap": "none for the selected account split replay; semantic values are decoded from canonical account opening frames before comparison",
        },
    ]
    internalized = sum(1 for row in rows if row["status"].startswith("internalized"))
    digest_payload = {
        "semantic_plan_digest": semantic_plan.get("target", {}).get(
            "implementation_plan_digest"
        ),
        "pad_boundary_digest": pad_boundary.get("target", {}).get("boundary_digest"),
        "rows": rows,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_byte_level_derivation_internalization.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "byte_level_derivation_internalization_partially_open_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "semantic_predicate_implementation_plan": rel(SEMANTIC_PLAN_JSON),
            "pad_prf_projection_boundary": rel(PAD_BOUNDARY_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "semantic_predicate_implementation_plan_schema": semantic_plan.get("schema"),
            "pad_prf_projection_boundary_schema": pad_boundary.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "internalization_digest": digest(digest_payload),
        },
        "derivation_rows": rows,
        "coverage": {
            "derivation_target_count": len(rows),
            "internalized_derivation_count": internalized,
            "open_derivation_count": len(rows) - internalized,
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "reason": "pad PRF projection values, account-witness projection values, expanded-opening state checks, account split semantic values, and checked-core response differences are now derived from canonical byte frames inside the boundary; the remaining blocker is the exact bridge certificate/leakage proof that relates those decoded values to accepted 308-byte projection-carrier transcripts",
            "next_artifact": "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    coverage = payload["coverage"]
    lines = [
        "# Hidden-Source Projection Byte-Level Derivation Internalization",
        "",
        "This artifact tracks which projection-route semantic values are computed",
        "inside the production-reference boundary rather than supplied by the",
        "caller.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- internalization digest: `{target['internalization_digest']}`",
        f"- internalized derivations: `{coverage['internalized_derivation_count']}` / `{coverage['derivation_target_count']}`",
        f"- open derivations: `{coverage['open_derivation_count']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Derivation Rows",
        "",
        "| row | status | remaining gap |",
        "|---|---|---|",
    ]
    for row in payload["derivation_rows"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['remaining_gap']} |")
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
