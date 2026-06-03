#!/usr/bin/env python3
"""Generate the 56-byte algebraic fallback carrier projection relation target."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_carrier_projection_relation_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "hidden_source_fallback_carrier_projection_relation_n32_q12289.md"
)
FALLBACK_MODEL_JSON = (
    READINESS_DIR
    / "hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    fallback_model = load_json(FALLBACK_MODEL_JSON)
    layout = {
        "carrier_bytes": 56,
        "envelope_bytes": 308,
        "context_digest_bytes": 32,
        "projection_payload_bytes": 21,
        "check_flag_bytes": 3,
        "q12289_projection_elements": 12,
        "field_bits": 14,
    }
    frame = [
        {
            "field": "domain",
            "encoding": "ASCII",
            "value": "Descender.hidden_source.algebraic_carrier.v1",
        },
        {
            "field": "profile",
            "encoding": "ASCII",
            "value": "n32.q12289.fq12.fallback56.v1",
        },
        {
            "field": "statement_digest",
            "encoding": "32 bytes",
            "value": "SHA-256 of the canonical verifier-owned hidden-source statement frame",
        },
        {
            "field": "opening_commitment",
            "encoding": "32 bytes",
            "value": "prechallenge account-state hidden-source opening commitment",
        },
        {
            "field": "proof_transcript_digest",
            "encoding": "32 bytes",
            "value": "reviewed hidden-source transcript digest for the checked-core signature",
        },
        {
            "field": "projection_payload",
            "encoding": "21 bytes",
            "value": "twelve packed q12289 projection elements, 14 bits each",
        },
        {
            "field": "check_flags",
            "encoding": "3 bytes",
            "value": "version, projection selector, and public checksum class",
        },
    ]
    acceptance_predicate = [
        {
            "id": "context_digest_binding",
            "rule": "carrier[0..32] must equal SHA-256 of the domain-separated fallback frame without carrier[0..32]",
            "status": "specified_not_accepting",
        },
        {
            "id": "projection_range",
            "rule": "all twelve packed projection elements must decode to integers in [0, 12289)",
            "status": "specified_not_accepting",
        },
        {
            "id": "projection_equation",
            "rule": "decoded elements must satisfy the selected public projection of the hidden-source relation",
            "status": "open",
        },
        {
            "id": "opening_commitment_link",
            "rule": "projection must be linked to the prechallenge opening commitment and cannot be replayed under another statement/transcript",
            "status": "open",
        },
        {
            "id": "fixed_rejection",
            "rule": "all rejection paths must map to the fixed public hidden-source proof failure reason",
            "status": "specified_not_accepting",
        },
    ]
    negative_controls = [
        "mutate_context_digest",
        "mutate_projection_payload_byte",
        "projection_element_out_of_range",
        "mutate_check_flags",
        "swap_statement_digest",
        "swap_opening_commitment",
        "swap_proof_transcript_digest",
        "replay_projection_under_new_slot",
        "digest_only_context_without_projection_equation",
    ]
    return {
        "schema": "Descender.hidden_source_fallback_carrier_projection_relation.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "fallback_projection_relation_target_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "fallback_byte_model": rel(FALLBACK_MODEL_JSON),
            "fallback_byte_model_schema": fallback_model.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "selected_only_if_32b_ro_pok_rejected": False,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
        },
        "layout": layout,
        "frame": frame,
        "acceptance_predicate": acceptance_predicate,
        "negative_controls": negative_controls,
        "proof_obligations": [
            {
                "id": "choose_projection_map",
                "requirement": "select the twelve q12289 coordinates/projections and prove they force the needed hidden-source relation under named breaks",
            },
            {
                "id": "prove_projection_hiding",
                "requirement": "bound leakage from the twelve revealed q12289 projection elements and three check bytes",
            },
            {
                "id": "implement_fallback_decoder",
                "requirement": "write a byte decoder that rejects noncanonical q12289 packing and malformed flags",
            },
            {
                "id": "rerun_byte_budget",
                "requirement": "if this fallback is selected, update envelope bytes to 308 in all selected-branch claims",
            },
        ],
        "decision": {
            "can_accept_now": False,
            "fallback_selected_now": True,
            "reason": "the 56-byte projection relation is the selected fail-closed route, but the projection map certificate and leakage theorem are still open",
            "next_artifact": "fallback_projection_map_search_selected",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    layout = payload["layout"]
    lines = [
        "# Hidden-Source Fallback Carrier Projection Relation",
        "",
        "This artifact instantiates the selected 56-byte projection carrier as a",
        "byte-level projection-relation target.  It remains fail-closed until",
        "the exact projection certificate and leakage theorem close.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- carrier bytes: `{layout['carrier_bytes']}`",
        f"- envelope bytes: `{layout['envelope_bytes']}`",
        f"- context digest bytes: `{layout['context_digest_bytes']}`",
        f"- projection payload bytes: `{layout['projection_payload_bytes']}`",
        f"- check flag bytes: `{layout['check_flag_bytes']}`",
        f"- q12289 projection elements: `{layout['q12289_projection_elements']}`",
        f"- source byte model: `{payload['source_artifacts']['fallback_byte_model']}`",
        f"- selected branch: {payload['target']['selected_branch']}",
        "",
        "## Frame",
        "",
        "| field | encoding | value |",
        "|---|---|---|",
    ]
    for row in payload["frame"]:
        lines.append(f"| `{row['field']}` | {row['encoding']} | {row['value']} |")
    lines.extend(
        [
            "",
            "## Acceptance Predicate",
            "",
            "| id | status | rule |",
            "|---|---|---|",
        ]
    )
    for row in payload["acceptance_predicate"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['rule']} |")
    lines.extend(["", "## Negative Controls", ""])
    lines.extend(f"- `{control}`" for control in payload["negative_controls"])
    lines.extend(
        [
            "",
            "## Proof Obligations",
            "",
            "| id | requirement |",
            "|---|---|",
        ]
    )
    for row in payload["proof_obligations"]:
        lines.append(f"| `{row['id']}` | {row['requirement']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- fallback selected now: `{payload['decision']['fallback_selected_now']}`",
            f"- reason: {payload['decision']['reason']}",
            f"- next artifact: `{payload['decision']['next_artifact']}`",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
