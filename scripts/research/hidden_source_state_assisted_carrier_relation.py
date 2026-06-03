#!/usr/bin/env python3
"""Generate the state-assisted 32-byte hidden-source carrier relation."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "hidden_source_state_assisted_carrier_relation_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "hidden_source_state_assisted_carrier_relation_n32_q12289.md"
CARRIER_DECISION_JSON = (
    READINESS_DIR / "hidden_source_carrier_construction_decision_n32_q12289.json"
)
ACCEPTING_RELATION_JSON = (
    READINESS_DIR / "hidden_source_accepting_relation_spec_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    decision = load_json(CARRIER_DECISION_JSON)
    accepting_relation = load_json(ACCEPTING_RELATION_JSON)
    state_creation_frame = [
        {
            "field": "domain",
            "source": "fixed domain separator for state-assisted hidden-source carrier commitments",
            "binding": "prevents reuse with ordinary account commitments or transcript digests",
        },
        {
            "field": "profile",
            "source": "n=32, q=12289, Fq12 checked-core profile",
            "binding": "prevents cross-profile carrier reuse",
        },
        {
            "field": "slot_id",
            "source": "account-state slot index",
            "binding": "couples the hidden-source opening to exactly one spend slot",
        },
        {
            "field": "account_witness_commitment",
            "source": "existing 32-byte witness commitment in account state",
            "binding": "commits the hidden witness syndrome before challenge derivation",
        },
        {
            "field": "account_randomizer_commitment",
            "source": "existing 32-byte randomizer commitment in account state",
            "binding": "commits the proof/randomizer material charged by the checked core",
        },
        {
            "field": "hidden_source_opening_commitment",
            "source": "new logical precommitment digest, placed in account state or derived from existing reserve metadata if a theorem justifies it",
            "binding": "commits the pad seed, witness opening selector, and carrier handle secret before the message challenge",
        },
        {
            "field": "restore_anchor_digest",
            "source": "deployment-specific trusted restore anchor or monotonic state root",
            "binding": "prevents old backups from resurrecting hidden-source openings",
        },
    ]
    carrier_handle_predicate = [
        {
            "predicate": "carrier_is_transcript_bound",
            "rule": "carrier = H(handle_domain, hidden_source_opening_commitment, proof_transcript_digest, slot_id, public_key_digest, message_digest) or a stricter equivalent",
            "break_if_false": "carrier can be replayed across messages, keys, slots, or signature cores",
        },
        {
            "predicate": "opening_commitment_is_prechallenge",
            "rule": "hidden_source_opening_commitment must be fixed in account state before the Fiat-Shamir challenge and before carrier publication",
            "break_if_false": "signer can choose the hidden-source opening after seeing the challenge",
        },
        {
            "predicate": "opening_commitment_binds_witness_and_pad",
            "rule": "opening commitment must bind the account witness opening and pad seed used by the exact accepting relation",
            "break_if_false": "carrier proves at most state membership, not the hidden-source witness relation",
        },
        {
            "predicate": "carrier_is_single_use",
            "rule": "the accepted carrier consumes the slot and nullifies the opening commitment under the spent-slot ledger",
            "break_if_false": "one hidden-source opening can support multiple accepted signatures",
        },
    ]
    theorem_shape = [
        {
            "case": "extract_hidden_source",
            "conclusion": "extract account witness syndrome opening and pad seed opening satisfying the accepting relation",
        },
        {
            "case": "break_opening_commitment",
            "conclusion": "find two distinct hidden-source openings for the same prechallenge opening commitment",
        },
        {
            "case": "break_carrier_binding",
            "conclusion": "move a carrier across transcript context without changing its 32 bytes",
        },
        {
            "case": "break_state_lifecycle",
            "conclusion": "accept a consumed, rolled-back, or unanchored slot against the spent-ledger/restore-anchor rules",
        },
        {
            "case": "break_underlying_checked_core",
            "conclusion": "violate the checked Fq12 core, response-link certificate, pad PRF, QROM budget, or matrix/keygen assumptions",
        },
    ]
    negative_controls = [
        {
            "id": "digest_only_acceptance",
            "must_reject": "carrier bytes are a transcript digest but no hidden_source_opening_commitment is present",
        },
        {
            "id": "postchallenge_opening_commitment",
            "must_reject": "opening commitment is not fixed in account state before the challenge frame",
        },
        {
            "id": "wrong_opening_commitment",
            "must_reject": "carrier is bound to an opening commitment that is not the slot's committed hidden-source opening",
        },
        {
            "id": "message_replay",
            "must_reject": "same carrier and opening commitment are attached to a different message digest",
        },
        {
            "id": "slot_replay",
            "must_reject": "same carrier and opening commitment are reused for a consumed slot",
        },
        {
            "id": "restore_anchor_rollback",
            "must_reject": "old account snapshot resurrects an opening commitment after a later restore anchor",
        },
        {
            "id": "witness_pad_mismatch",
            "must_reject": "opening commitment binds a witness opening or pad seed different from the checked accepting relation",
        },
    ]
    implementation_hooks = [
        {
            "hook": "account_state_schema",
            "required_change": "add or derive a hidden_source_opening_commitment row per spend slot",
            "current_status": "not implemented",
        },
        {
            "hook": "carrier_verifier_profile",
            "required_change": "replace unimplemented proof-semantics profile with state-assisted carrier verification profile",
            "current_status": "not implemented",
        },
        {
            "hook": "spent_slot_ledger",
            "required_change": "bind carrier acceptance to the existing public-key-hash/slot-id spent-ledger policy",
            "current_status": "reference lifecycle exists, deployment hardening open",
        },
        {
            "hook": "restore_anchor_policy",
            "required_change": "bind carrier acceptance to a trusted restore-anchor or monotonic state-root rule",
            "current_status": "reference checks exist, trusted deployment placement open",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_carrier_relation.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "state_assisted_relation_spec_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "carrier_decision": rel(CARRIER_DECISION_JSON),
            "accepting_relation": rel(ACCEPTING_RELATION_JSON),
            "carrier_decision_schema": decision.get("schema"),
            "accepting_relation_schema": accepting_relation.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_carrier",
        },
        "state_creation_frame": state_creation_frame,
        "carrier_handle_predicate": carrier_handle_predicate,
        "extractor_or_break_theorem_shape": theorem_shape,
        "negative_controls_required": negative_controls,
        "implementation_hooks": implementation_hooks,
        "decision": {
            "can_accept_now": False,
            "reason": "the account-state opening commitment and extractor theorem are specified here but not implemented or proved",
            "next_artifact": "state_assisted_carrier_extractor_obligations",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    lines = [
        "# Hidden-Source State-Assisted Carrier Relation",
        "",
        "This artifact specifies the state-assisted 32-byte carrier relation chosen",
        "by the carrier decision artifact. It remains fail-closed: the relation is",
        "now explicit, but the account-state commitment and extractor theorem are",
        "not implemented or proved.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- n: `{target['n']}`",
        f"- q: `{target['q']}`",
        f"- checked core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier decision: `{payload['source_artifacts']['carrier_decision']}`",
        f"- accepting relation: `{payload['source_artifacts']['accepting_relation']}`",
        "",
        "## State Creation Frame",
        "",
        "| field | source | binding |",
        "|---|---|---|",
    ]
    for row in payload["state_creation_frame"]:
        lines.append(f"| `{row['field']}` | {row['source']} | {row['binding']} |")
    lines.extend(
        [
            "",
            "## Carrier-Handle Predicate",
            "",
            "| predicate | rule | break if false |",
            "|---|---|---|",
        ]
    )
    for row in payload["carrier_handle_predicate"]:
        lines.append(f"| `{row['predicate']}` | {row['rule']} | {row['break_if_false']} |")
    lines.extend(
        [
            "",
            "## Extractor-Or-Break Theorem Shape",
            "",
            "| case | conclusion |",
            "|---|---|",
        ]
    )
    for row in payload["extractor_or_break_theorem_shape"]:
        lines.append(f"| `{row['case']}` | {row['conclusion']} |")
    lines.extend(["", "## Negative Controls Required", "", "| control | must reject |", "|---|---|"])
    for row in payload["negative_controls_required"]:
        lines.append(f"| `{row['id']}` | {row['must_reject']} |")
    lines.extend(["", "## Implementation Hooks", "", "| hook | required change | current status |", "|---|---|---|"])
    for row in payload["implementation_hooks"]:
        lines.append(
            f"| `{row['hook']}` | {row['required_change']} | `{row['current_status']}` |"
        )
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
