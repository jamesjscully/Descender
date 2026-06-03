#!/usr/bin/env python3
"""Generate the state-assisted hidden-source carrier verifier contract."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.md"
)
OPENING_FRAME_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.json"
)
TEST_VECTORS_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.json"
)
NEGATIVE_CONTROLS_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_negative_controls_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    opening_frame = load_json(OPENING_FRAME_JSON)
    test_vectors = load_json(TEST_VECTORS_JSON)
    negative_controls = load_json(NEGATIVE_CONTROLS_JSON)
    public_inputs = [
        {
            "input": "public_key_digest",
            "source": "SHA-256 of canonical public-key bytes",
            "required": True,
        },
        {
            "input": "message_digest",
            "source": "SHA-256 of transaction message bytes",
            "required": True,
        },
        {
            "input": "signature_digest",
            "source": "SHA-256 of the strict 284-byte signature-carried envelope",
            "required": True,
        },
        {
            "input": "proof_transcript_digest",
            "source": "existing reviewed Fq12 hidden-source proof transcript digest",
            "required": True,
        },
        {
            "input": "slot_id",
            "source": "account-state slot index, widened to uint64 little-endian in the carrier frame",
            "required": True,
        },
    ]
    state_inputs = [
        {
            "input": "account_witness_commitment",
            "source": "prechallenge account-state witness commitment",
            "required": True,
        },
        {
            "input": "account_randomizer_commitment",
            "source": "prechallenge account-state randomizer commitment",
            "required": True,
        },
        {
            "input": "hidden_source_opening_commitment",
            "source": "prechallenge account-state SHA-256 opening commitment",
            "required": True,
        },
        {
            "input": "restore_anchor_digest",
            "source": "active trusted restore anchor or monotonic state-root digest",
            "required": True,
        },
        {
            "input": "spent_slot_state",
            "source": "deployment spent-slot ledger",
            "required": True,
        },
    ]
    witness_inputs = [
        {
            "input": "witness_opening_digest",
            "checked_by": "opening commitment frame and later hidden-source accepting relation",
        },
        {
            "input": "pad_seed_digest",
            "checked_by": "opening commitment frame and later pad-opening relation",
        },
        {
            "input": "carrier_handle_secret_digest",
            "checked_by": "opening commitment frame and carrier handle frame",
        },
    ]
    verifier_phases = [
        {
            "phase": "parse_strict_envelope",
            "acceptance": "strictly parse the 284-byte envelope into 252-byte Fq12 checked core plus 32-byte carrier",
            "rejects": ["noncanonical_length"],
        },
        {
            "phase": "recompute_public_context",
            "acceptance": "recompute public-key, message, signature, proof-transcript, and slot inputs from bytes rather than caches",
            "rejects": ["cached_statement_digest", "message_replay", "public_key_replay"],
        },
        {
            "phase": "load_prechallenge_state",
            "acceptance": "load hidden_source_opening_commitment and related commitments from account state fixed before challenge derivation",
            "rejects": ["postchallenge_opening_commitment", "wrong_opening_commitment"],
        },
        {
            "phase": "check_lifecycle",
            "acceptance": "check spent-slot and restore-anchor policy before any accepting carrier result",
            "rejects": ["slot_replay", "restore_anchor_rollback"],
        },
        {
            "phase": "check_carrier_handle",
            "acceptance": "recompute carrier = SHA256(carrier_handle_frame) and compare to the 32-byte hidden-source proof carrier",
            "rejects": ["digest_only_acceptance", "wrong_opening_commitment"],
        },
        {
            "phase": "check_hidden_relation",
            "acceptance": "prove or verify that the opening commitment binds witness and pad sources satisfying the hidden-source relation",
            "rejects": ["witness_pad_mismatch"],
        },
    ]
    theorem_hooks = [
        {
            "hook": "carrier_binding",
            "statement": "if check_carrier_handle accepts, then the carrier is bound to the recomputed public context and prechallenge opening commitment unless SHA-256 binding fails",
        },
        {
            "hook": "state_precommitment",
            "statement": "if load_prechallenge_state accepts, then the opening commitment was fixed before the Fiat-Shamir challenge unless the account-state lifecycle assumption fails",
        },
        {
            "hook": "hidden_source_extraction",
            "statement": "if check_hidden_relation accepts, extract witness-opening and pad-seed openings or return a named commitment/hash/proof break",
        },
    ]
    blockers = [
        {
            "blocker": "state_schema_not_deployed",
            "reason": "production account state does not yet store or derive hidden_source_opening_commitment",
        },
        {
            "blocker": "hidden_relation_verifier_not_implemented",
            "reason": "carrier-handle equality is implemented, but witness/pad knowledge semantics remain open",
        },
        {
            "blocker": "extractor_theorem_not_proved",
            "reason": "acceptance has not yet been proved to imply a hidden-source witness or named break",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_carrier_verifier_contract.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "carrier_verifier_contract_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "opening_frame": rel(OPENING_FRAME_JSON),
            "test_vectors": rel(TEST_VECTORS_JSON),
            "negative_controls": rel(NEGATIVE_CONTROLS_JSON),
            "opening_frame_schema": opening_frame.get("schema"),
            "test_vectors_schema": test_vectors.get("schema"),
            "negative_controls_schema": negative_controls.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_carrier",
        },
        "rust_boundary": {
            "frame_helpers": "tools/parameter-search/src/production_reference.rs",
            "opening_frame_bytes": opening_frame.get("opening_commitment", {}).get(
                "public_frame_bytes"
            )
            + opening_frame.get("opening_commitment", {}).get("private_frame_bytes"),
            "carrier_frame_bytes": opening_frame.get("carrier_handle", {}).get(
                "public_frame_bytes"
            )
            + opening_frame.get("carrier_handle", {}).get("private_frame_bytes"),
            "test_vector_count": len(test_vectors.get("mutation_vectors", [])),
            "carrier_handle_check": "verify_reviewed_fq12_state_assisted_carrier_handle",
        },
        "public_inputs": public_inputs,
        "state_inputs": state_inputs,
        "witness_inputs": witness_inputs,
        "verifier_phases": verifier_phases,
        "theorem_hooks": theorem_hooks,
        "blockers": blockers,
        "decision": {
            "can_accept_now": False,
            "reason": "the carrier byte contract and Rust handle check exist, but state schema, hidden-relation verification, and extractor theorem remain open",
            "next_artifact": "state_assisted_account_state_schema_contract",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    rust = payload["rust_boundary"]
    lines = [
        "# Hidden-Source State-Assisted Carrier Verifier Contract",
        "",
        "This artifact pins the verifier contract that composes the state-assisted",
        "opening/carrier byte frames with the selected 308-byte/Fq12 projection-carrier envelope. It",
        "remains fail-closed because carrier-handle equality is not yet a proof of",
        "the hidden-source witness relation.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- checked-core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- Rust boundary: `{rust['frame_helpers']}`",
        f"- opening frame bytes: `{rust['opening_frame_bytes']}`",
        f"- carrier frame bytes: `{rust['carrier_frame_bytes']}`",
        f"- mutation test vectors: `{rust['test_vector_count']}`",
        f"- carrier handle check: `{rust['carrier_handle_check']}`",
        "",
        "## Public Inputs",
        "",
        "| input | source | required |",
        "|---|---|:---:|",
    ]
    for row in payload["public_inputs"]:
        lines.append(f"| `{row['input']}` | {row['source']} | `{row['required']}` |")
    lines.extend(["", "## State Inputs", "", "| input | source | required |", "|---|---|:---:|"])
    for row in payload["state_inputs"]:
        lines.append(f"| `{row['input']}` | {row['source']} | `{row['required']}` |")
    lines.extend(["", "## Witness Inputs", "", "| input | checked by |", "|---|---|"])
    for row in payload["witness_inputs"]:
        lines.append(f"| `{row['input']}` | {row['checked_by']} |")
    lines.extend(["", "## Verifier Phases", "", "| phase | acceptance | rejects |", "|---|---|---|"])
    for row in payload["verifier_phases"]:
        lines.append(f"| `{row['phase']}` | {row['acceptance']} | `{row['rejects']}` |")
    lines.extend(["", "## Theorem Hooks", "", "| hook | statement |", "|---|---|"])
    for row in payload["theorem_hooks"]:
        lines.append(f"| `{row['hook']}` | {row['statement']} |")
    lines.extend(["", "## Blockers", "", "| blocker | reason |", "|---|---|"])
    for row in payload["blockers"]:
        lines.append(f"| `{row['blocker']}` | {row['reason']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
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
