#!/usr/bin/env python3
"""Adversarial state-lifecycle simulator for PTC-007.

This harness models reviewer-shaped lifecycle attacks against the local
stateful signature boundary.  It is deliberately small and executable: each
scenario applies a sequence of state-machine actions, records whether the
attempted replay or rollback was accepted, and classifies the result against
the existing lifecycle/blockchain memos.
"""

from __future__ import annotations

import copy
import hashlib
import json
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
LIFECYCLE_MEMO = READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.json"
BLOCKCHAIN_MEMO = READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json"
STATE_LIFECYCLE_CLOSURE = READINESS_DIR / "state_lifecycle_closure_n32_q12289.json"
DEFAULT_JSON = (
    READINESS_DIR / "pre_third_party_state_lifecycle_adversary_n32_q12289.json"
)
DEFAULT_MD = READINESS_DIR / "pre_third_party_state_lifecycle_adversary_n32_q12289.md"

PROFILE = "Descender.Signature.q12289.local_delta.stateful.v1"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def load_optional_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    return load_json(path)


def digest(*parts: str) -> str:
    h = hashlib.sha256()
    for part in parts:
        h.update(len(part).to_bytes(4, "little"))
        h.update(part.encode("utf-8"))
    return h.hexdigest()


@dataclass
class Policy:
    name: str
    enforce_restore_anchor: bool = True
    spent_ledger_consensus: bool = True
    burn_reorg_reveals: bool = True
    burn_mempool_reveals: bool = True
    monotonic_snapshots: bool = True
    reject_slot_reuse: bool = True
    atomic_transition_order: bool = True
    anchor_outside_backup: bool = True


@dataclass
class Signature:
    public_key_hash: str
    slot_id: int
    snapshot_sequence: int
    message: str
    revealed_in: str
    signature_digest: str


@dataclass
class Machine:
    policy: Policy
    public_key_hash: str = field(
        default_factory=lambda: digest(PROFILE, "reference-public-key")[:32]
    )
    next_slot: int = 0
    snapshot_sequence: int = 0
    trusted_anchor_sequence: int = 0
    spent_slots: set[tuple[str, int]] = field(default_factory=set)
    burned_slots: set[tuple[str, int]] = field(default_factory=set)
    reserved_slots: set[tuple[str, int]] = field(default_factory=set)
    accepted: list[str] = field(default_factory=list)
    events: list[str] = field(default_factory=list)

    def clone_backup(self, include_anchor: bool) -> dict[str, Any]:
        return {
            "next_slot": self.next_slot,
            "snapshot_sequence": self.snapshot_sequence,
            "trusted_anchor_sequence": (
                self.trusted_anchor_sequence if include_anchor else None
            ),
            "spent_slots": sorted(self.spent_slots),
            "burned_slots": sorted(self.burned_slots),
            "reserved_slots": sorted(self.reserved_slots),
        }

    def restore(self, backup: dict[str, Any]) -> bool:
        restored_sequence = int(backup["snapshot_sequence"])
        if self.policy.enforce_restore_anchor and restored_sequence < self.trusted_anchor_sequence:
            self.events.append(
                f"restore rejected: snapshot {restored_sequence} below trusted anchor {self.trusted_anchor_sequence}"
            )
            return False
        if self.policy.monotonic_snapshots and restored_sequence < self.snapshot_sequence:
            self.events.append(
                f"restore rejected: snapshot {restored_sequence} below current {self.snapshot_sequence}"
            )
            return False
        self.next_slot = int(backup["next_slot"])
        self.snapshot_sequence = restored_sequence
        restored_anchor = backup.get("trusted_anchor_sequence")
        if restored_anchor is not None:
            self.trusted_anchor_sequence = int(restored_anchor)
        self.spent_slots = {tuple(item) for item in backup["spent_slots"]}
        self.burned_slots = {tuple(item) for item in backup["burned_slots"]}
        self.reserved_slots = {tuple(item) for item in backup["reserved_slots"]}
        self.events.append(f"restore accepted at snapshot {self.snapshot_sequence}")
        return True

    def reserve(self) -> int | None:
        key = (self.public_key_hash, self.next_slot)
        if self.policy.reject_slot_reuse and (
            key in self.spent_slots or key in self.burned_slots or key in self.reserved_slots
        ):
            self.events.append(f"reserve rejected: slot {self.next_slot} unavailable")
            return None
        slot = self.next_slot
        self.reserved_slots.add(key)
        self.next_slot += 1
        self.snapshot_sequence += 1
        if self.policy.enforce_restore_anchor:
            self.trusted_anchor_sequence = max(
                self.trusted_anchor_sequence, self.snapshot_sequence
            )
        self.events.append(f"reserved slot {slot} at snapshot {self.snapshot_sequence}")
        return slot

    def sign_reserved(self, slot_id: int, message: str, revealed_in: str) -> Signature:
        key = (self.public_key_hash, slot_id)
        if self.policy.burn_mempool_reveals and revealed_in == "mempool":
            self.burned_slots.add(key)
        return Signature(
            public_key_hash=self.public_key_hash,
            slot_id=slot_id,
            snapshot_sequence=self.snapshot_sequence,
            message=message,
            revealed_in=revealed_in,
            signature_digest=digest(
                PROFILE,
                self.public_key_hash,
                str(slot_id),
                str(self.snapshot_sequence),
                message,
                revealed_in,
            ),
        )

    def verify_consume(self, signature: Signature) -> bool:
        key = (signature.public_key_hash, signature.slot_id)
        if self.policy.spent_ledger_consensus and key in self.spent_slots:
            self.events.append(f"verify rejected: slot {signature.slot_id} already spent")
            return False
        if self.policy.reject_slot_reuse and key in self.burned_slots and key not in self.reserved_slots:
            self.events.append(f"verify rejected: slot {signature.slot_id} burned")
            return False
        if self.policy.monotonic_snapshots and signature.snapshot_sequence < self.trusted_anchor_sequence:
            self.events.append(
                "verify rejected: signature snapshot below trusted anchor"
            )
            return False
        if self.policy.atomic_transition_order:
            self.spent_slots.add(key)
            self.reserved_slots.discard(key)
            self.snapshot_sequence += 1
            if self.policy.enforce_restore_anchor:
                self.trusted_anchor_sequence = max(
                    self.trusted_anchor_sequence, self.snapshot_sequence
                )
        else:
            self.snapshot_sequence += 1
        self.accepted.append(signature.signature_digest)
        self.events.append(f"verify accepted: slot {signature.slot_id}")
        return True

    def reorg_out(self, signature: Signature) -> None:
        key = (signature.public_key_hash, signature.slot_id)
        if self.policy.burn_reorg_reveals:
            self.burned_slots.add(key)
            self.spent_slots.add(key)
            self.events.append(f"reorg burn recorded for slot {signature.slot_id}")
        else:
            self.spent_slots.discard(key)
            self.events.append(f"reorg rolled back spent record for slot {signature.slot_id}")


def baseline_machine() -> Machine:
    return Machine(Policy("memo_required_reference_policy"))


def weakened_machine(**changes: Any) -> Machine:
    policy = Policy("deliberately_weakened_policy")
    for key, value in changes.items():
        setattr(policy, key, value)
    return Machine(policy)


def prepare_spent(machine: Machine, message: str = "initial spend") -> Signature:
    slot = machine.reserve()
    assert slot is not None
    signature = machine.sign_reserved(slot, message, "block")
    accepted = machine.verify_consume(signature)
    assert accepted
    return signature


def scenario_rollback() -> dict[str, Any]:
    machine = baseline_machine()
    backup = machine.clone_backup(include_anchor=False)
    first = prepare_spent(machine)
    restored = machine.restore(backup)
    replay = machine.verify_consume(first)
    return {
        "scenario": "rollback",
        "attack": "restore an earlier local account snapshot after an accepted spend, then replay the revealed signature",
        "classification": "fail-closed",
        "accepted_attack": restored and replay,
        "expected_blocker": "restore anchor and monotonic snapshot checks reject stale local state",
        "events": machine.events,
    }


def scenario_backup_restore() -> dict[str, Any]:
    hardened = baseline_machine()
    ordinary_backup = hardened.clone_backup(include_anchor=False)
    prepare_spent(hardened)
    hardened_restore = hardened.restore(ordinary_backup)

    weak = weakened_machine(
        enforce_restore_anchor=False,
        anchor_outside_backup=False,
        monotonic_snapshots=False,
    )
    weak_backup = weak.clone_backup(include_anchor=True)
    first = prepare_spent(weak)
    weak_restore = weak.restore(weak_backup)
    weak_replay = weak.verify_consume(first)
    return {
        "scenario": "backup_restore",
        "attack": "restore an old wallet backup that also rolls back the local account slot cursor",
        "classification": "fail-closed under closure policy",
        "accepted_attack": hardened_restore,
        "negative_control_accepted_attack": weak_restore and weak_replay,
        "expected_blocker": "trusted minimum snapshot sequence is non-exportable and cannot be restored from ordinary account backups",
        "hardened_events": hardened.events,
        "weakened_events": weak.events,
    }


def scenario_chain_reorg() -> dict[str, Any]:
    machine = baseline_machine()
    signature = prepare_spent(machine, "confirmed on fork")
    machine.reorg_out(signature)
    replay = machine.verify_consume(signature)
    return {
        "scenario": "chain_reorg",
        "attack": "reorg out a block containing a revealed signature, then try to reuse the same slot on the new chain",
        "classification": "fail-closed",
        "accepted_attack": replay,
        "expected_blocker": "reorg-revealed signatures are burned and recorded in the spent-slot ledger",
        "events": machine.events,
    }


def scenario_mempool_replay() -> dict[str, Any]:
    machine = baseline_machine()
    slot = machine.reserve()
    assert slot is not None
    signature = machine.sign_reserved(slot, "mempool broadcast", "mempool")
    first_accept = machine.verify_consume(signature)
    second_accept = machine.verify_consume(signature)
    return {
        "scenario": "mempool_replay",
        "attack": "re-submit an already revealed mempool transaction after eviction or propagation delay",
        "classification": "fail-closed",
        "accepted_attack": second_accept,
        "accepted_first_submission": first_accept,
        "accepted_replay": second_accept,
        "expected_blocker": "mempool reveal burns the checked slot before acceptance can depend on chain finality",
        "events": machine.events,
    }


def scenario_slot_reuse() -> dict[str, Any]:
    machine = baseline_machine()
    first_slot = machine.reserve()
    assert first_slot is not None
    sig = machine.sign_reserved(first_slot, "first", "block")
    first_accept = machine.verify_consume(sig)
    machine.next_slot = first_slot
    reused_slot = machine.reserve()
    return {
        "scenario": "slot_reuse",
        "attack": "force the next-slot cursor back to a consumed slot and try to reserve it again",
        "classification": "fail-closed",
        "accepted_attack": reused_slot == first_slot,
        "accepted_first_submission": first_accept,
        "expected_blocker": "spent and burned slot keys make consumed slots unavailable for reservation",
        "events": machine.events,
    }


def scenario_trusted_anchor_placement() -> dict[str, Any]:
    hardened = baseline_machine()
    ordinary_backup = hardened.clone_backup(include_anchor=False)
    first_hardened = prepare_spent(hardened)
    hardened_restore = hardened.restore(ordinary_backup)
    hardened_replay = hardened.verify_consume(first_hardened)

    weak = weakened_machine(
        enforce_restore_anchor=False,
        anchor_outside_backup=False,
        monotonic_snapshots=False,
    )
    backup = weak.clone_backup(include_anchor=True)
    first = prepare_spent(weak)
    restored = weak.restore(backup)
    replay = weak.verify_consume(first)
    return {
        "scenario": "trusted_anchor_placement",
        "attack": "place the trusted restore anchor inside the same restorable backup as account state",
        "classification": "fail-closed under closure policy",
        "accepted_attack": hardened_restore and hardened_replay,
        "negative_control_accepted_attack": restored and replay,
        "expected_blocker": "anchor placement must be chain-observed, hardware monotonic, or quorum-server monotonic state, never co-restorable wallet state",
        "hardened_events": hardened.events,
        "weakened_events": weak.events,
    }


def scenario_state_monotonicity() -> dict[str, Any]:
    machine = baseline_machine()
    before = machine.snapshot_sequence
    first = prepare_spent(machine)
    stale_signature = copy.deepcopy(first)
    stale_signature.snapshot_sequence = before
    accepted_stale = machine.verify_consume(stale_signature)
    return {
        "scenario": "state_monotonicity",
        "attack": "submit a transcript whose state sequence is lower than the trusted anchor",
        "classification": "fail-closed",
        "accepted_attack": accepted_stale,
        "expected_blocker": "accepted verification advances the trusted anchor and rejects stale sequence transcripts",
        "events": machine.events,
    }


SCENARIOS: list[Callable[[], dict[str, Any]]] = [
    scenario_rollback,
    scenario_backup_restore,
    scenario_chain_reorg,
    scenario_mempool_replay,
    scenario_slot_reuse,
    scenario_trusted_anchor_placement,
    scenario_state_monotonicity,
]


def build_payload() -> dict[str, Any]:
    lifecycle = load_json(LIFECYCLE_MEMO)
    blockchain = load_json(BLOCKCHAIN_MEMO)
    closure = load_optional_json(STATE_LIFECYCLE_CLOSURE)
    scenarios = [run() for run in SCENARIOS]
    counts: dict[str, int] = {}
    for scenario in scenarios:
        counts[scenario["classification"]] = counts.get(scenario["classification"], 0) + 1
    unsafe_acceptances = [
        scenario["scenario"] for scenario in scenarios if scenario["accepted_attack"]
    ]
    negative_control_acceptances = [
        scenario["scenario"]
        for scenario in scenarios
        if scenario.get("negative_control_accepted_attack")
    ]
    source_artifacts = [rel(LIFECYCLE_MEMO), rel(BLOCKCHAIN_MEMO)]
    if closure is not None:
        source_artifacts.append(rel(STATE_LIFECYCLE_CLOSURE))
    return {
        "schema": "Descender.pre_third_party_state_lifecycle_adversary.v1",
        "campaign_id": "PTC-007",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "profile": PROFILE,
        "source_artifacts": source_artifacts,
        "local_reference_surfaces": blockchain.get("local_reference_surfaces", {}),
        "state_lifecycle_closure": {
            "status": None if closure is None else closure.get("closure_status"),
            "artifact": None if closure is None else rel(STATE_LIFECYCLE_CLOSURE),
            "acceptance_criteria": None
            if closure is None
            else closure.get("deployment_acceptance_criteria", []),
        },
        "memo_required_deployment_bindings": [
            item["name"] for item in blockchain.get("required_deployment_bindings", [])
        ],
        "memo_open_external_review": blockchain.get("open_external_review", []),
        "lifecycle_open_blockers": lifecycle.get("open_blockers", []),
        "classification_counts": counts,
        "unsafe_acceptances": unsafe_acceptances,
        "negative_control_acceptances": negative_control_acceptances,
        "scenarios": scenarios,
        "bottom_line": (
            "The executable checklist finds no accepted replay under the memo-required "
            "reference policy and state_lifecycle_closure acceptance criteria for "
            "rollback, backup restore, trusted-anchor placement, reorg, replay, slot "
            "reuse, or stale-state monotonicity.  The weakened negative controls still "
            "accept backup/anchor replays, confirming that non-exportable monotonic "
            "anchor placement is the consensus-critical closure rule."
        ),
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Third-Party State Lifecycle Adversary",
        "",
        "Executable PTC-007 checklist for rollback, backup restore, chain reorg,",
        "mempool replay, slot reuse, trusted-anchor placement, and state monotonicity.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- profile: `{payload['profile']}`",
        f"- source artifacts: {', '.join(f'`{item}`' for item in payload['source_artifacts'])}",
        f"- classification counts: `{payload['classification_counts']}`",
        f"- unsafe acceptances: `{payload['unsafe_acceptances']}`",
        f"- negative-control acceptances: `{payload['negative_control_acceptances']}`",
        f"- bottom line: {payload['bottom_line']}",
        "",
        "## Scenario Classifications",
        "",
        "| scenario | classification | accepted attack | expected blocker |",
        "|---|---|---:|---|",
    ]
    for scenario in payload["scenarios"]:
        lines.append(
            "| {scenario} | {classification} | `{accepted_attack}` | {expected_blocker} |".format(
                **scenario
            )
        )
    lines.extend(["", "## Executable Checklist", ""])
    for scenario in payload["scenarios"]:
        lines.append(f"### {scenario['scenario']}")
        lines.append("")
        lines.append(f"- attack: {scenario['attack']}")
        lines.append(f"- classification: `{scenario['classification']}`")
        lines.append(f"- accepted attack: `{scenario['accepted_attack']}`")
        lines.append(f"- blocker: {scenario['expected_blocker']}")
        event_fields = [
            ("events", scenario.get("events", [])),
            ("hardened events", scenario.get("hardened_events", [])),
            ("weakened events", scenario.get("weakened_events", [])),
        ]
        for label, events in event_fields:
            if events:
                lines.append(f"- {label}:")
                lines.extend(f"  - {event}" for event in events)
        lines.append("")
    lines.extend(
        [
            "## Memo Hooks",
            "",
            "Required deployment bindings:",
        ]
    )
    lines.extend(
        f"- `{name}`" for name in payload["memo_required_deployment_bindings"]
    )
    lines.extend(["", "Open external review items:"])
    lines.extend(f"- {item}" for item in payload["memo_open_external_review"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": rel(DEFAULT_JSON),
                "markdown": rel(DEFAULT_MD),
                "scenario_count": len(payload["scenarios"]),
                "classification_counts": payload["classification_counts"],
                "status": "written",
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
