#!/usr/bin/env python3
"""Generate the byte frame for state-assisted hidden-source openings."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.md"
)
RELATION_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_carrier_relation_n32_q12289.json"
)
NEGATIVE_CONTROLS_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_negative_controls_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    relation = load_json(RELATION_JSON)
    negative_controls = load_json(NEGATIVE_CONTROLS_JSON)
    opening_commitment_frame = [
        {
            "field": "domain",
            "encoding": "ascii bytes",
            "length_bytes": 34,
            "value": "Descender.hidden_source.opening.v1",
            "public": True,
            "binding_role": "separates the opening commitment from account commitments, transcript digests, and carrier handles",
        },
        {
            "field": "profile_id",
            "encoding": "ascii bytes",
            "length_bytes": 24,
            "value": "n32.q12289.fq12.state.v1",
            "public": True,
            "binding_role": "prevents cross-parameter reuse of opening commitments",
        },
        {
            "field": "slot_id",
            "encoding": "uint64 little-endian",
            "length_bytes": 8,
            "public": True,
            "binding_role": "binds the opening commitment to one spend slot",
        },
        {
            "field": "account_witness_commitment",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": True,
            "binding_role": "binds the hidden source to the account witness commitment fixed at state creation",
        },
        {
            "field": "account_randomizer_commitment",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": True,
            "binding_role": "binds the hidden source to the account randomizer commitment fixed at state creation",
        },
        {
            "field": "restore_anchor_digest",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": True,
            "binding_role": "prevents rollback to an older valid opening commitment outside the trusted restore-anchor policy",
        },
        {
            "field": "witness_opening_digest",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": False,
            "binding_role": "commits to the witness opening material used by the hidden-source accepting relation",
        },
        {
            "field": "pad_seed_digest",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": False,
            "binding_role": "commits to the pad seed source used by the hidden-source accepting relation",
        },
        {
            "field": "carrier_handle_secret_digest",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": False,
            "binding_role": "commits to the secret handle material later checked by the carrier relation",
        },
    ]
    carrier_handle_frame = [
        {
            "field": "domain",
            "encoding": "ascii bytes",
            "length_bytes": 34,
            "value": "Descender.hidden_source.carrier.v1",
            "public": True,
            "binding_role": "separates the 32-byte carrier handle from the hidden opening commitment",
        },
        {
            "field": "hidden_source_opening_commitment",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": True,
            "binding_role": "makes the carrier a handle to one prechallenge opening commitment",
        },
        {
            "field": "proof_transcript_digest",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": True,
            "binding_role": "binds the carrier to the checked-core signature transcript",
        },
        {
            "field": "signature_digest",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": True,
            "binding_role": "prevents substituting the carrier across different byte encodings of the signature envelope",
        },
        {
            "field": "public_key_digest",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": True,
            "binding_role": "prevents public-key replay",
        },
        {
            "field": "message_digest",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": True,
            "binding_role": "prevents message replay",
        },
        {
            "field": "slot_id",
            "encoding": "uint64 little-endian",
            "length_bytes": 8,
            "public": True,
            "binding_role": "prevents slot replay outside the spent-slot ledger",
        },
        {
            "field": "carrier_handle_secret_digest",
            "encoding": "32 raw bytes",
            "length_bytes": 32,
            "public": False,
            "binding_role": "prevents digest-only acceptance by requiring knowledge of the handle opening",
        },
    ]
    rejection_rules = [
        {
            "rule": "noncanonical_length",
            "reject_if": "any fixed-width field has a length other than the specified byte length",
            "covers_controls": ["cached_statement_digest"],
        },
        {
            "rule": "wrong_domain_or_profile",
            "reject_if": "domain or profile bytes differ exactly from the pinned values",
            "covers_controls": ["public_key_replay"],
        },
        {
            "rule": "missing_prechallenge_state",
            "reject_if": "hidden_source_opening_commitment is absent from the account state before challenge derivation",
            "covers_controls": ["postchallenge_opening_commitment"],
        },
        {
            "rule": "carrier_opening_mismatch",
            "reject_if": "carrier handle opens to a different hidden_source_opening_commitment than the slot state",
            "covers_controls": ["wrong_opening_commitment"],
        },
        {
            "rule": "transcript_context_mismatch",
            "reject_if": "carrier handle is reused under a different message, public key, signature digest, proof transcript, or slot",
            "covers_controls": ["message_replay", "public_key_replay", "slot_replay"],
        },
        {
            "rule": "hidden_relation_mismatch",
            "reject_if": "witness opening or pad seed digest does not match the checked hidden-source relation",
            "covers_controls": ["witness_pad_mismatch"],
        },
        {
            "rule": "restore_anchor_mismatch",
            "reject_if": "restore_anchor_digest is not the active trusted anchor for the account state",
            "covers_controls": ["restore_anchor_rollback"],
        },
        {
            "rule": "digest_only_no_opening",
            "reject_if": "verifier sees only a 32-byte digest with no accepted opening relation or named hash-binding assumption",
            "covers_controls": ["digest_only_acceptance"],
        },
    ]
    test_vectors_required = [
        {
            "id": "canonical_opening_frame_round_trip",
            "purpose": "serialize the opening frame once and recompute the same hidden_source_opening_commitment",
        },
        {
            "id": "canonical_carrier_frame_round_trip",
            "purpose": "serialize the carrier frame once and recompute the same 32-byte carrier handle",
        },
        {
            "id": "field_order_mutation_rejects",
            "purpose": "swap two equal-length fields and prove the digest changes or verification rejects",
        },
        {
            "id": "domain_profile_mutation_rejects",
            "purpose": "mutate domain/profile bytes and prove the verifier rejects",
        },
        {
            "id": "public_context_mutation_rejects",
            "purpose": "mutate message, public key, signature digest, proof transcript, or slot id with carrier bytes fixed",
        },
        {
            "id": "private_opening_mutation_rejects",
            "purpose": "mutate witness opening, pad seed, or handle secret while keeping public state fixed",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_opening_frame.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "opening_frame_specified_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "state_assisted_relation": rel(RELATION_JSON),
            "negative_controls": rel(NEGATIVE_CONTROLS_JSON),
            "relation_schema": relation.get("schema"),
            "negative_controls_schema": negative_controls.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_carrier",
            "hash_assumption": "SHA-256 collision and second-preimage resistance over canonical framed inputs",
        },
        "opening_commitment": {
            "digest": "hidden_source_opening_commitment = SHA256(opening_commitment_frame)",
            "frame": opening_commitment_frame,
            "public_frame_bytes": sum(
                row["length_bytes"] for row in opening_commitment_frame if row["public"]
            ),
            "private_frame_bytes": sum(
                row["length_bytes"] for row in opening_commitment_frame if not row["public"]
            ),
        },
        "carrier_handle": {
            "digest": "carrier = SHA256(carrier_handle_frame)",
            "frame": carrier_handle_frame,
            "public_frame_bytes": sum(
                row["length_bytes"] for row in carrier_handle_frame if row["public"]
            ),
            "private_frame_bytes": sum(
                row["length_bytes"] for row in carrier_handle_frame if not row["public"]
            ),
        },
        "rejection_rules": rejection_rules,
        "test_vectors_required": test_vectors_required,
        "decision": {
            "can_accept_now": False,
            "reason": "byte frames are pinned, but no executable verifier or theorem has yet shown that the 32-byte carrier gives knowledge of the hidden source",
            "next_artifact": "state_assisted_opening_frame_test_vectors",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    opening = payload["opening_commitment"]
    carrier = payload["carrier_handle"]
    lines = [
        "# Hidden-Source State-Assisted Opening Frame",
        "",
        "This artifact pins the canonical byte frames for the state-assisted",
        "hidden-source opening commitment and 32-byte carrier handle. It remains",
        "fail-closed: these bytes are necessary for tests and theorem statements,",
        "but they do not by themselves make the carrier accepting.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- n: `{target['n']}`",
        f"- q: `{target['q']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- hash assumption: {target['hash_assumption']}",
        f"- state-assisted relation: `{payload['source_artifacts']['state_assisted_relation']}`",
        f"- negative controls: `{payload['source_artifacts']['negative_controls']}`",
        "",
        "## Opening Commitment Frame",
        "",
        f"- digest: `{opening['digest']}`",
        f"- public frame bytes: `{opening['public_frame_bytes']}`",
        f"- private frame bytes: `{opening['private_frame_bytes']}`",
        "",
        "| field | encoding | bytes | public | binding role |",
        "|---|---|---:|:---:|---|",
    ]
    for row in opening["frame"]:
        lines.append(
            f"| `{row['field']}` | {row['encoding']} | `{row['length_bytes']}` | `{row['public']}` | {row['binding_role']} |"
        )
    lines.extend(
        [
            "",
            "## Carrier Handle Frame",
            "",
            f"- digest: `{carrier['digest']}`",
            f"- public frame bytes: `{carrier['public_frame_bytes']}`",
            f"- private frame bytes: `{carrier['private_frame_bytes']}`",
            "",
            "| field | encoding | bytes | public | binding role |",
            "|---|---|---:|:---:|---|",
        ]
    )
    for row in carrier["frame"]:
        lines.append(
            f"| `{row['field']}` | {row['encoding']} | `{row['length_bytes']}` | `{row['public']}` | {row['binding_role']} |"
        )
    lines.extend(["", "## Rejection Rules", "", "| rule | reject if | controls |", "|---|---|---|"])
    for row in payload["rejection_rules"]:
        lines.append(
            f"| `{row['rule']}` | {row['reject_if']} | `{row['covers_controls']}` |"
        )
    lines.extend(["", "## Required Test Vectors", "", "| test | purpose |", "|---|---|"])
    for row in payload["test_vectors_required"]:
        lines.append(f"| `{row['id']}` | {row['purpose']} |")
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
