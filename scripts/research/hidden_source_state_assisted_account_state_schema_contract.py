#!/usr/bin/env python3
"""Generate the state-assisted account-state schema contract."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.md"
)
VERIFIER_CONTRACT_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    verifier_contract = load_json(VERIFIER_CONTRACT_JSON)
    slot_fields = [
        {
            "field": "slot_id",
            "encoding": "u32 state index; widened to u64 in opening/carrier frames",
            "invariant": "slots are sorted and slot_id equals array index",
        },
        {
            "field": "commitment",
            "encoding": "existing 68-byte account commitment",
            "invariant": "witness/randomizer commitments remain q-syndrome-canonical with reviewed response-link code",
        },
        {
            "field": "hidden_source_opening_commitment",
            "encoding": "32 raw bytes",
            "invariant": "nonzero prechallenge commitment fixed before challenge derivation",
        },
        {
            "field": "restore_anchor_digest",
            "encoding": "32 raw bytes",
            "invariant": "nonzero active restore anchor or monotonic state-root digest",
        },
        {
            "field": "pad_seed_digest",
            "encoding": "32 raw bytes",
            "invariant": "nonzero per-slot pad seed digest unique within the account and recorded in spent-slot state",
        },
        {
            "field": "unused",
            "encoding": "boolean",
            "invariant": "must be true before state-assisted carrier acceptance",
        },
    ]
    rust_contract = [
        {
            "symbol": "ProductionReviewedFq12StateAssistedAccountSlot",
            "role": "typed per-slot sidecar carrying account commitment, hidden-source opening commitment, restore anchor, pad seed digest, and lifecycle bit",
        },
        {
            "symbol": "ProductionReviewedFq12StateAssistedAccountState",
            "role": "sorted sidecar slot vector with commitment/opening-anchor validation",
        },
        {
            "symbol": "verify_reviewed_fq12_state_assisted_slot_opening",
            "role": "checks unused slot, slot id, witness/randomizer commitments, restore anchor, and opening commitment digest",
        },
    ]
    negative_controls = [
        {
            "control": "postchallenge_opening_commitment",
            "coverage": "schema requires a stored prechallenge opening commitment; deployment must still enforce creation time",
            "status": "partially_covered",
        },
        {
            "control": "wrong_opening_commitment",
            "coverage": "Rust helper rejects if recomputed opening commitment differs from slot state",
            "status": "covered_at_boundary",
        },
        {
            "control": "restore_anchor_rollback",
            "coverage": "Rust helper rejects restore-anchor mismatch; trusted anchor placement remains deployment work",
            "status": "partially_covered",
        },
        {
            "control": "slot_replay",
            "coverage": "Rust helper rejects consumed sidecar slots",
            "status": "covered_at_boundary",
        },
        {
            "control": "witness_pad_mismatch",
            "coverage": "opening digest and stored pad_seed_digest change if private witness/pad fields change; hidden-relation proof remains open",
            "status": "partially_covered",
        },
    ]
    blockers = [
        {
            "blocker": "persistent_storage_format",
            "reason": "the typed sidecar exists in Rust, but no production persistence format or migration is implemented",
        },
        {
            "blocker": "creation_time_enforcement",
            "reason": "the schema can carry a prechallenge commitment, but keygen/account creation must still enforce when it is fixed",
        },
        {
            "blocker": "hidden_relation_verifier",
            "reason": "slot-opening equality does not yet prove the witness/pad relation",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_account_state_schema_contract.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "account_state_schema_contract_partially_implemented_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "carrier_verifier_contract": rel(VERIFIER_CONTRACT_JSON),
            "carrier_verifier_contract_schema": verifier_contract.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_carrier",
        },
        "slot_fields": slot_fields,
        "rust_contract": rust_contract,
        "negative_controls": negative_controls,
        "blockers": blockers,
        "decision": {
            "can_accept_now": False,
            "reason": "the Rust sidecar schema and opening check exist, but persistent storage, creation-time enforcement, and hidden-relation verification remain open",
            "next_artifact": "state_assisted_hidden_relation_verifier_contract",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Hidden-Source State-Assisted Account-State Schema Contract",
        "",
        "This artifact records the sidecar account-state schema used to carry the",
        "prechallenge hidden-source opening commitment. It remains fail-closed:",
        "the Rust type and opening equality check exist, but storage, creation-time",
        "enforcement, and hidden-relation verification remain open.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- carrier verifier contract: `{payload['source_artifacts']['carrier_verifier_contract']}`",
        f"- selected candidate: `{payload['target']['selected_candidate']}`",
        f"- carrier bytes: `{payload['target']['carrier_bytes']}`",
        f"- envelope bytes: `{payload['target']['envelope_bytes']}`",
        "",
        "## Slot Fields",
        "",
        "| field | encoding | invariant |",
        "|---|---|---|",
    ]
    for row in payload["slot_fields"]:
        lines.append(f"| `{row['field']}` | {row['encoding']} | {row['invariant']} |")
    lines.extend(["", "## Rust Contract", "", "| symbol | role |", "|---|---|"])
    for row in payload["rust_contract"]:
        lines.append(f"| `{row['symbol']}` | {row['role']} |")
    lines.extend(["", "## Negative Controls", "", "| control | coverage | status |", "|---|---|---|"])
    for row in payload["negative_controls"]:
        lines.append(f"| `{row['control']}` | {row['coverage']} | `{row['status']}` |")
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
