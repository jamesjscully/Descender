#!/usr/bin/env python3
"""Generate the small algebraic carrier fallback byte model."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_algebraic_carrier_fallback_byte_model_n32_q12289.md"
)
COMPACT_PROOF_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_compact_proof_instantiation_n32_q12289.json"
)
RO_POK_NEGATIVE_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"
)

CHECKED_CORE_SIGNATURE_BYTES = 252
OPAQUE_HANDLE_BYTES = 32
FIELD_BITS = 14


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def packed_q_elements(byte_budget: int) -> int:
    return (byte_budget * 8) // FIELD_BITS


def build_payload() -> dict[str, Any]:
    compact = load_json(COMPACT_PROOF_JSON)
    negative_tests = load_json(RO_POK_NEGATIVE_JSON)
    candidates = []
    for carrier_bytes in (40, 48, 56, 64):
        algebraic_payload_bytes = carrier_bytes - OPAQUE_HANDLE_BYTES
        q_elements = packed_q_elements(algebraic_payload_bytes)
        if carrier_bytes == 40:
            classification = "minimal_control"
            proof_use = (
                "context digest plus four packed q-elements and one byte of flags; "
                "useful for falsifying digest-only acceptance, not enough for a "
                "robust standalone opening relation"
            )
            soundness_posture = "likely still needs the RO-PoK assumption for extraction"
        elif carrier_bytes == 48:
            classification = "first_plausible_algebraic_hook"
            proof_use = (
                "context digest plus eight packed q-elements and two flag/check bytes; "
                "enough to bind a small response projection or checksum subview"
            )
            soundness_posture = (
                "may close replay/context binding, but probably not full witness/pad extraction"
            )
        elif carrier_bytes == 56:
            classification = "balanced_fallback_candidate"
            proof_use = (
                "context digest plus twelve packed q-elements and three flag/check bytes; "
                "enough for two small projected openings plus a consistency check"
            )
            soundness_posture = (
                "best initial fallback target if 32-byte RO-PoK review is rejected"
            )
        else:
            classification = "strong_fallback_candidate"
            proof_use = (
                "context digest plus sixteen packed q-elements and four flag/check bytes; "
                "largest fallback still near the compact target and most likely to make "
                "negative controls executable without hidden verifier state"
            )
            soundness_posture = (
                "preferred fallback if extraction needs public algebraic material"
            )
        candidates.append(
            {
                "carrier_bytes": carrier_bytes,
                "envelope_bytes": CHECKED_CORE_SIGNATURE_BYTES + carrier_bytes,
                "extra_bytes_over_284b": carrier_bytes - OPAQUE_HANDLE_BYTES,
                "algebraic_payload_bytes_after_context_digest": algebraic_payload_bytes,
                "max_q12289_elements_in_algebraic_payload": q_elements,
                "classification": classification,
                "proof_use": proof_use,
                "soundness_posture": soundness_posture,
                "deployment_ready": False,
            }
        )
    selection = {
        "keep_32b_track_if": [
            "the RO proof-handle assumption is accepted as a named assumption",
            "byte-level negative controls pass against the accepting verifier",
            "zero-knowledge/leakage theorem charges only the opaque 32-byte handle",
            "global theorem composes the assumption with checked-core, QROM, pad, keygen, and lifecycle rules",
        ],
        "activate_fallback_if": [
            "third-party review rejects the 32-byte proof-handle assumption as too strong",
            "semantic negative controls require public algebraic material to be executable",
            "the extractor theorem cannot obtain witness/pad openings from an accepted opaque handle",
            "leakage analysis needs explicit public projection accounting rather than an opaque handle",
        ],
        "recommended_first_fallback": "56_byte_balanced_fallback_candidate",
        "reason": "56 bytes adds 24 bytes over the checked core's 32-byte handle budget, carries twelve q12289 elements plus check flags, and keeps the envelope at 308 bytes while giving the theorem public algebraic material to bind.",
    }
    return {
        "schema": "Descender.hidden_source_algebraic_carrier_fallback_byte_model.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "fallback_byte_model_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "compact_proof_instantiation": rel(COMPACT_PROOF_JSON),
            "compact_proof_instantiation_schema": compact.get("schema"),
            "ro_pok_byte_negative_tests": rel(RO_POK_NEGATIVE_JSON),
            "ro_pok_byte_negative_tests_schema": negative_tests.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "field_bits": FIELD_BITS,
            "checked_core_signature_bytes": CHECKED_CORE_SIGNATURE_BYTES,
            "opaque_handle_bytes": OPAQUE_HANDLE_BYTES,
            "current_envelope_bytes": CHECKED_CORE_SIGNATURE_BYTES
            + OPAQUE_HANDLE_BYTES,
        },
        "candidate_carriers": candidates,
        "selection_policy": selection,
        "proof_obligations": [
            {
                "id": "projection_soundness",
                "requirement": "public algebraic payload must force the hidden-source relation or name a checked-core/commitment/hash break",
            },
            {
                "id": "projection_leakage_budget",
                "requirement": "each q12289 element revealed by the carrier must be charged in the leakage theorem",
            },
            {
                "id": "domain_separated_frame",
                "requirement": "fallback carrier bytes must be hashed under a domain distinct from the 32-byte RO-PoK frame",
            },
            {
                "id": "negative_controls",
                "requirement": "false statement, pad substitution, carrier replay, stale slot, and rollback tests must reject without secret verifier inputs",
            },
            {
                "id": "byte_budget_update",
                "requirement": "if fallback is selected, update all envelope-byte claims, status indexes, and blockchain comparison tables",
            },
        ],
        "decision": {
            "can_accept_now": False,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "fallback_is_selected": True,
            "next_artifact": "fallback_carrier_projection_relation_selected",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    selection = payload["selection_policy"]
    lines = [
        "# Hidden-Source Algebraic Carrier Fallback Byte Model",
        "",
        "This artifact puts concrete byte costs on the fallback path if the",
        "32-byte state-assisted RO-PoK handle is rejected.  It is not a",
        "deployment claim and does not select a larger carrier by itself.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- checked-core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- current envelope bytes: `{target['current_envelope_bytes']}`",
        f"- q field bits: `{target['field_bits']}`",
        f"- compact proof source: `{payload['source_artifacts']['compact_proof_instantiation']}`",
        "",
        "## Candidate Carriers",
        "",
        "| carrier bytes | envelope bytes | extra over 284 B | q12289 elements | classification | proof use |",
        "|---:|---:|---:|---:|---|---|",
    ]
    for row in payload["candidate_carriers"]:
        lines.append(
            f"| `{row['carrier_bytes']}` | `{row['envelope_bytes']}` | "
            f"`{row['extra_bytes_over_284b']}` | "
            f"`{row['max_q12289_elements_in_algebraic_payload']}` | "
            f"`{row['classification']}` | {row['proof_use']} |"
        )
    lines.extend(["", "## Selection Policy", "", "Keep the 32-byte track if:"])
    lines.extend(f"- {item}" for item in selection["keep_32b_track_if"])
    lines.extend(["", "Activate the fallback if:"])
    lines.extend(f"- {item}" for item in selection["activate_fallback_if"])
    lines.extend(
        [
            "",
            f"- recommended first fallback: `{selection['recommended_first_fallback']}`",
            f"- reason: {selection['reason']}",
            "",
            "## Proof Obligations",
            "",
            "| id | requirement |",
            "|---|---|",
        ]
    )
    for obligation in payload["proof_obligations"]:
        lines.append(f"| `{obligation['id']}` | {obligation['requirement']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- selected branch: {payload['decision']['selected_branch']}",
            f"- fallback is selected: `{payload['decision']['fallback_is_selected']}`",
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
