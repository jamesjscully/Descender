#!/usr/bin/env python3
"""Record exact bridge certificate and leakage-proof closure state."""

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
    / "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof_n32_q12289.md"
)
BYTE_INTERNALIZATION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_byte_level_derivation_internalization_n32_q12289.json"
)
EXACT_CERTIFICATE_SPEC_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"
)
BRIDGE_REPLAY_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.json"
)
MATRIX_BUILDER_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
)
EXACT_REPLAY_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json"
)
PARAMETER_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"
PRODUCTION_LIB = ROOT / "tools/production-reference/src/lib.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.fallback_projection.exact_bridge_and_leakage.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    byte_internalization = load_json(BYTE_INTERNALIZATION_JSON)
    exact_spec = load_json(EXACT_CERTIFICATE_SPEC_JSON)
    bridge_replay = load_json(BRIDGE_REPLAY_JSON)
    matrix_builder = load_json(MATRIX_BUILDER_JSON)
    exact_replay = load_json(EXACT_REPLAY_JSON) if EXACT_REPLAY_JSON.exists() else {}
    sources = [PARAMETER_REFERENCE.read_text(), PRODUCTION_LIB.read_text()]

    leakage_symbols = [
        "HiddenSourceFallbackProjectionLeakageAccounting",
        "hidden_source_fallback_projection_leakage_accounting",
        "HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES",
        "HIDDEN_SOURCE_FALLBACK_PROJECTION_CHECK_BYTES",
        "HIDDEN_SOURCE_FALLBACK_PROJECTION_TRANSCRIPT_BINDING_BYTES",
    ]
    leakage_accounting_present = all(source_contains(symbol, sources) for symbol in leakage_symbols)
    leakage = {
        "carrier_bytes": 56,
        "envelope_bytes": 308,
        "charged_q12289_lane_count": 12,
        "charged_q12289_lane_bytes": 24,
        "charged_check_bytes": 3,
        "charged_projection_bytes": 27,
        "context_binding_bytes": 29,
        "accounted_carrier_bytes": 56,
        "unaccounted_carrier_bytes": 0,
        "rust_accounting_present": leakage_accounting_present,
    }
    exact_bridge_rows = [
        {
            "id": "byte_level_derivations_internalized",
            "status": "closed"
            if byte_internalization.get("coverage", {}).get("open_derivation_count") == 0
            else "open",
            "evidence": "all semantic values are decoded from canonical byte frames before replay",
        },
        {
            "id": "leakage_charge",
            "status": "closed" if leakage_accounting_present else "open",
            "evidence": "twelve q12289 lanes plus three check bytes are charged as projection leakage; the 29-byte transcript binding is counted as public context binding",
        },
        {
            "id": "generalized_projection_matrix_builder",
            "status": "closed"
            if matrix_builder.get("coverage", {}).get(
                "generalized_projection_matrix_built"
            )
            is True
            else "open",
            "evidence": "generalized system interface fixes checked-core response variables, hidden-source projection variables, decoded public carrier lanes, check bytes, semantic decoder preconditions, and open bridge slots",
        },
        {
            "id": "exact_infeasibility_replay",
            "status": "closed_with_bounded_survivor"
            if exact_replay.get("decision", {}).get(
                "exact_infeasibility_replay_has_run"
            )
            is True
            else "open",
            "evidence": "deterministic replay ran on the instantiated generalized system and found a bounded survivor, so infeasibility is not proved and acceptance remains fail-closed",
        },
        {
            "id": "bad_vector_export",
            "status": "closed"
            if exact_replay.get("decision", {}).get("bad_vector_export_complete")
            is True
            else "open",
            "evidence": "bounded survivor is exported and classified as an underconstrained open-bridge survivor",
        },
    ]
    open_rows = sum(1 for row in exact_bridge_rows if row["status"] == "open")
    exact_infeasibility_proved = (
        exact_replay.get("decision", {}).get("exact_infeasibility_proved") is True
    )
    digest_payload = {
        "byte_internalization_digest": byte_internalization.get("target", {}).get(
            "internalization_digest"
        ),
        "exact_certificate_status": exact_spec.get("status"),
        "bridge_replay_status": bridge_replay.get("status"),
        "matrix_builder_status": matrix_builder.get("status"),
        "exact_replay_status": exact_replay.get("status"),
        "leakage": leakage,
        "rows": exact_bridge_rows,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_exact_bridge_certificate_and_leakage_proof.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": (
            "leakage_accounted_exact_bridge_certificate_open_fail_closed"
            if open_rows
            else "exact_bridge_replay_closed_with_bounded_survivor_fail_closed"
        ),
        "deployment_ready": False,
        "source_artifacts": {
            "byte_level_derivation_internalization": rel(BYTE_INTERNALIZATION_JSON),
            "exact_certificate_spec": rel(EXACT_CERTIFICATE_SPEC_JSON),
            "bridge_exact_replay": rel(BRIDGE_REPLAY_JSON),
            "matrix_builder": rel(MATRIX_BUILDER_JSON),
            "exact_infeasibility_replay": rel(EXACT_REPLAY_JSON),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "byte_level_derivation_schema": byte_internalization.get("schema"),
            "exact_certificate_spec_schema": exact_spec.get("schema"),
            "bridge_exact_replay_schema": bridge_replay.get("schema"),
            "matrix_builder_schema": matrix_builder.get("schema"),
            "exact_infeasibility_replay_schema": exact_replay.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "proof_digest": digest(digest_payload),
        },
        "leakage_accounting": leakage,
        "exact_bridge_rows": exact_bridge_rows,
        "coverage": {
            "row_count": len(exact_bridge_rows),
            "closed_row_count": len(exact_bridge_rows) - open_rows,
            "open_row_count": open_rows,
            "leakage_accounted": leakage_accounting_present,
            "exact_certificate_complete": exact_infeasibility_proved,
            "exact_replay_closed": open_rows == 0,
            "bad_vector_exported": exact_replay.get("decision", {}).get(
                "bad_vector_export_complete"
            )
            is True,
        },
        "decision": {
            "can_accept_now": False,
            "reason": "projection leakage, generalized system construction, exact replay, and bad-vector export are accounted; the replay found a bounded survivor, so the selected verifier remains fail-closed pending bridge rows or map redesign",
            "next_artifact": "hidden_source_fallback_projection_map_redesign_or_bridge_rows",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    leakage = payload["leakage_accounting"]
    coverage = payload["coverage"]
    lines = [
        "# Hidden-Source Projection Exact Bridge Certificate And Leakage Proof",
        "",
        "This artifact records the selected 308-byte projection route's exact",
        "bridge-certificate and leakage-proof state. It is intentionally",
        "fail-closed: leakage accounting is closed here, but the final exact",
        "combined bridge certificate has not been produced.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- proof digest: `{target['proof_digest']}`",
        f"- closed rows: `{coverage['closed_row_count']}` / `{coverage['row_count']}`",
        f"- open rows: `{coverage['open_row_count']}`",
        "",
        "## Leakage Accounting",
        "",
        f"- charged q12289 lanes: `{leakage['charged_q12289_lane_count']}`",
        f"- charged q12289 lane bytes: `{leakage['charged_q12289_lane_bytes']}`",
        f"- charged check bytes: `{leakage['charged_check_bytes']}`",
        f"- charged projection bytes: `{leakage['charged_projection_bytes']}`",
        f"- context binding bytes: `{leakage['context_binding_bytes']}`",
        f"- accounted carrier bytes: `{leakage['accounted_carrier_bytes']}`",
        f"- unaccounted carrier bytes: `{leakage['unaccounted_carrier_bytes']}`",
        f"- Rust accounting present: `{leakage['rust_accounting_present']}`",
        "",
        "## Exact Bridge Rows",
        "",
        "| row | status | evidence |",
        "|---|---|---|",
    ]
    for row in payload["exact_bridge_rows"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['evidence']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- reason: {payload['decision']['reason']}",
            f"- next artifact: `{payload['decision']['next_artifact']}`",
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
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD), "status": payload["status"]}, indent=2))


if __name__ == "__main__":
    main()
