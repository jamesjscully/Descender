#!/usr/bin/env python3
"""Closure artifact for postchallenge selected-slot creation gating."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_postchallenge_slot_creation_gate_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_postchallenge_slot_creation_gate_n32_q12289.md"
)
LIFT_JSON = (
    READINESS_DIR / "pre_signature_projection_lifecycle_same_game_lift_n32_q12289.json"
)
DISCHARGE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json"
)
LIFECYCLE_JSON = READINESS_DIR / "state_lifecycle_closure_n32_q12289.json"
GAP_REPORT_JSON = READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
PRODUCTION_REFERENCE_RS = ROOT / "tools/production-reference/src/lib.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def contains(path: Path, needle: str) -> bool:
    return needle in path.read_text()


def code_check(id_: str, path: Path, needle: str, rationale: str) -> dict[str, Any]:
    return {
        "id": id_,
        "path": rel(path),
        "needle": needle,
        "present": contains(path, needle),
        "rationale": rationale,
    }


def build_payload() -> dict[str, Any]:
    lift = load_json(LIFT_JSON)
    discharge = load_json(DISCHARGE_JSON)
    lifecycle = load_json(LIFECYCLE_JSON)
    gap_report = load_json(GAP_REPORT_JSON)

    checks = [
        code_check(
            "rust_challenge_eligibility_epoch_gate",
            PRODUCTION_REFERENCE_RS,
            "challenge_eligibility_epoch",
            "explicit lifecycle epoch separating prechallenge slot creation from postchallenge mutation",
        ),
        code_check(
            "rust_challenge_id_gate",
            PRODUCTION_REFERENCE_RS,
            "challenge_id",
            "challenge instance identifier bound to the selected challenge-bearing tuple",
        ),
        code_check(
            "rust_postchallenge_create_slot_reject",
            PRODUCTION_REFERENCE_RS,
            "reject_postchallenge_challenge_eligible",
            "public no-mutation reject code for postchallenge CreateSlot attempts against the challenge tuple",
        ),
        code_check(
            "rust_create_slot_api_surface",
            PRODUCTION_REFERENCE_RS,
            "CreateSlot",
            "named CreateSlot lifecycle API or test surface for selected-path slot creation",
        ),
        code_check(
            "rust_account_state_unique_pad_seed_digest_test",
            PRODUCTION_REFERENCE_RS,
            "fn state_assisted_account_schema_enforces_unique_pad_seed_digest()",
            "prechallenge account-state construction rejects duplicate pad_seed_digest values",
        ),
        code_check(
            "rust_slot_bound_witness_commitment_reject",
            PRODUCTION_REFERENCE_RS,
            '"state-assisted witness commitment mismatch"',
            "slot-bound opening rejects a different packed witness commitment under the original slot",
        ),
        code_check(
            "rust_restore_anchor_replay_reject",
            PRODUCTION_REFERENCE_RS,
            '"snapshot sequence is older than restore anchor"',
            "restore/replay paths are gated by the trusted monotonic restore anchor",
        ),
        code_check(
            "rust_reorg_or_recovery_burn_reserved_slot",
            PRODUCTION_REFERENCE_RS,
            "burn_reserved_slot(reservation)",
            "ambiguous reserved selected-projection slots are burned on recovery/reorg policy",
        ),
        code_check(
            "rust_target_chain_duplicate_pad_digest_reject",
            PRODUCTION_REFERENCE_RS,
            '"pad seed digest is already in spent ledger"',
            "target-chain commit validation rejects duplicate selected-projection pad_seed_digest records",
        ),
        code_check(
            "gap_report_lifecycle_pad_digest_invariants",
            GAP_REPORT_JSON,
            '"selected_projection_spent_pad_seed_digest_invariant_present": true',
            "gap report records selected-projection spent-ledger pad_seed_digest invariant presence",
        ),
    ]
    presence = {item["id"]: bool(item["present"]) for item in checks}
    epoch_gate_present = all(
        presence[item]
        for item in [
            "rust_challenge_eligibility_epoch_gate",
            "rust_challenge_id_gate",
            "rust_postchallenge_create_slot_reject",
            "rust_create_slot_api_surface",
        ]
    )
    adjacent_controls_present = all(
        presence[item]
        for item in [
            "rust_account_state_unique_pad_seed_digest_test",
            "rust_slot_bound_witness_commitment_reject",
            "rust_restore_anchor_replay_reject",
            "rust_reorg_or_recovery_burn_reserved_slot",
            "rust_target_chain_duplicate_pad_digest_reject",
            "gap_report_lifecycle_pad_digest_invariants",
        ]
    )

    epoch_gate = {
        "name": "challenge_epoch_create_slot_gate",
        "state_variables": [
            "challenge_epoch in {prechallenge, challenge_sampled, postchallenge_closed}",
            "challenge_id: domain-separated digest of experiment id, account identity, and challenge transcript prefix",
            "challenge_tuple: (pk_hash, slot_id, pad_seed_digest, packed_witness_commitment, snapshot_sequence)",
            "challenge_tuple_digest: public digest of challenge_id || challenge_tuple",
            "prechallenge_eligible_set_digest: digest of all slots eligible before challenge sampling",
            "spent_reserved_burned_ledgers: authoritative public-key-hash/slot-id and pad_seed_digest ledgers",
        ],
        "gate_rule": (
            "After challenge_epoch advances past prechallenge, CreateSlot MUST reject "
            "without state mutation if the request creates, rewrites, restores, or "
            "reorg-selects the challenge-bearing tuple, shares its slot_id, shares "
            "its pad_seed_digest, changes its packed_witness_commitment, or changes "
            "the prechallenge eligible set from which the challenge slot was sampled."
        ),
        "reject_code": "reject_postchallenge_challenge_eligible",
        "reject_semantics": [
            "public reject outcome",
            "no account-state mutation",
            "no reservation record",
            "no spent-ledger mutation",
            "no burn record solely for the rejected CreateSlot request",
            "same reject transcript in real and ideal challenge worlds",
        ],
    }

    transitions = [
        {
            "from": "init",
            "query": "CreateSlot",
            "precondition": "challenge_epoch == prechallenge",
            "decision": "accept only if slot_id, pad_seed_digest, and packed_witness_commitment are fresh and canonical",
            "next_state": "prechallenge eligible set may grow; challenge_tuple is not yet sampled",
        },
        {
            "from": "prechallenge",
            "query": "AccountStateRead",
            "precondition": "slot is live and eligible",
            "decision": "return public slot metadata and masked packed witness bytes",
            "next_state": "transcript records the eligible tuple digest",
        },
        {
            "from": "prechallenge",
            "query": "ChallengeSample",
            "precondition": "challenge slot is chosen from the prechallenge eligible-set transcript",
            "decision": "freeze challenge_tuple, challenge_tuple_digest, and prechallenge_eligible_set_digest",
            "next_state": "challenge_sampled",
        },
        {
            "from": "challenge_sampled",
            "query": "CreateSlot",
            "precondition": "candidate collides with challenge slot_id, challenge pad_seed_digest, challenge packed_witness_commitment, or challenge eligibility membership",
            "decision": "reject_postchallenge_challenge_eligible with no mutation",
            "next_state": "challenge_sampled",
        },
        {
            "from": "challenge_sampled",
            "query": "CreateSlot",
            "precondition": "candidate is disjoint from challenge tuple and cannot alter the frozen eligible set",
            "decision": "may accept under ordinary freshness policy, but is not challenge eligible for this experiment",
            "next_state": "challenge_sampled with separate nonchallenge slot",
        },
        {
            "from": "challenge_sampled",
            "query": "Restore or ReorgAttempt",
            "precondition": "snapshot or trace introduces a challenge-bearing tuple absent from the frozen prechallenge transcript",
            "decision": "reject or burn under restore/reorg policy, and do not add challenge eligibility",
            "next_state": "challenge_sampled with frozen eligible set unchanged",
        },
        {
            "from": "challenge_sampled",
            "query": "SignOrSpend or Verify",
            "precondition": "opening references the frozen challenge tuple",
            "decision": "use existing slot-bound commitment, spent-ledger, and burn rules; no CreateSlot fallback is permitted",
            "next_state": "spent, burned, or unchanged public reject state",
        },
    ]

    negative_controls = [
        {
            "id": "same_slot_different_witness_after_challenge",
            "setup": "Create and read an eligible slot, then sample the challenge on that slot.",
            "postchallenge_action": "CreateSlot(pk_hash, same slot_id, same pad_seed_digest, different packed_witness_commitment)",
            "expected": "reject_postchallenge_challenge_eligible and byte-identical account state, ledger state, and eligible-set digest",
            "disabled_gate_control": "accepts or rewrites the challenge slot when the epoch gate is disabled",
            "covered_by_existing_evidence": False,
        },
        {
            "id": "fresh_slot_duplicate_challenge_pad_digest",
            "setup": "Sample the challenge on a pre-read slot with pad_seed_digest d.",
            "postchallenge_action": "CreateSlot(pk_hash, fresh slot_id, pad_seed_digest d, fresh packed_witness_commitment)",
            "expected": "reject_postchallenge_challenge_eligible or public duplicate-digest reject with no digest-ledger mutation",
            "disabled_gate_control": "creates a fresh slot that can reuse the challenged mask source",
            "covered_by_existing_evidence": False,
        },
        {
            "id": "restore_introduces_unseen_challenge_tuple",
            "setup": "Freeze a challenge eligible-set transcript and trusted restore anchor.",
            "postchallenge_action": "Restore(snapshot) where the snapshot contains a tuple not in the frozen prechallenge transcript but eligible for the challenge identity",
            "expected": "restore reject or burn; frozen eligible-set digest unchanged",
            "disabled_gate_control": "restored tuple becomes challenge eligible after sampling",
            "covered_by_existing_evidence": False,
        },
        {
            "id": "reorg_selects_postchallenge_tuple",
            "setup": "Sample the challenge, then present a reorg trace containing an alternative reservation for the challenge identity.",
            "postchallenge_action": "ReorgAttempt(trace) followed by CreateSlot or AccountStateRead on the alternative tuple",
            "expected": "burn/reject alternative tuple and keep challenge_tuple_digest unchanged",
            "disabled_gate_control": "alternative tuple can be selected into the challenge set after sampling",
            "covered_by_existing_evidence": False,
        },
        {
            "id": "disjoint_postchallenge_create_is_not_challenge_eligible",
            "setup": "Sample the challenge on tuple T.",
            "postchallenge_action": "CreateSlot for a fresh disjoint tuple U.",
            "expected": "ordinary acceptance may occur, but U is marked challenge_epoch > prechallenge and excluded from this experiment's challenge eligible set",
            "disabled_gate_control": "U is incorrectly allowed to replace or enlarge the frozen challenge set",
            "covered_by_existing_evidence": False,
        },
    ]

    subset_discharge = [
        {
            "subset": "prechallenge pad_seed_digest duplicate rejection",
            "status": (
                "discharged_for_reference_account_state"
                if presence["rust_account_state_unique_pad_seed_digest_test"]
                else "not_discharged"
            ),
            "evidence": ["rust_account_state_unique_pad_seed_digest_test"],
            "limits": "Does not prove any postchallenge epoch gate or no-mutation CreateSlot reject.",
        },
        {
            "subset": "post-spend duplicate pad_seed_digest ledger rejection",
            "status": (
                "discharged_for_reference_spent_ledger"
                if presence["rust_target_chain_duplicate_pad_digest_reject"]
                else "not_discharged"
            ),
            "evidence": ["rust_target_chain_duplicate_pad_digest_reject"],
            "limits": "Applies after accepted spend/commit, not to postchallenge slot creation before spend.",
        },
        {
            "subset": "slot-bound alternate witness opening rejection",
            "status": (
                "partially_discharged_for_opening_path"
                if presence["rust_slot_bound_witness_commitment_reject"]
                else "not_discharged"
            ),
            "evidence": ["rust_slot_bound_witness_commitment_reject"],
            "limits": "Rejects an alternate opening under an existing slot, but does not prevent postchallenge CreateSlot from creating eligibility.",
        },
        {
            "subset": "restore and reorg stale-slot replay controls",
            "status": (
                "discharged_for_policy_and_reference_controls"
                if presence["rust_restore_anchor_replay_reject"]
                and presence["rust_reorg_or_recovery_burn_reserved_slot"]
                else "not_discharged"
            ),
            "evidence": [
                "rust_restore_anchor_replay_reject",
                "rust_reorg_or_recovery_burn_reserved_slot",
            ],
            "limits": "Covers slot reanimation and burn policy, not a frozen challenge eligible-set gate.",
        },
        {
            "subset": "Adv_postchallenge_slot_creation(q_create)",
            "status": (
                "discharged"
                if epoch_gate_present
                else "residual_blocker_no_epoch_gate_or_negative_controls"
            ),
            "evidence": [
                "rust_challenge_eligibility_epoch_gate",
                "rust_challenge_id_gate",
                "rust_postchallenge_create_slot_reject",
                "rust_create_slot_api_surface",
            ],
            "limits": "Requires all epoch-gate evidence and postchallenge negative controls before the term can be removed.",
        },
    ]

    return {
        "schema": "Descender.pre_signature_projection_postchallenge_slot_creation_gate.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "postchallenge_slot_creation_gate_residual_blocker",
        "deployment_ready": False,
        "advantage_term": "Adv_postchallenge_slot_creation(q_create)",
        "target": lift["target"],
        "source_artifacts": {
            "lifecycle_same_game_lift": rel(LIFT_JSON),
            "lifecycle_same_game_discharge": rel(DISCHARGE_JSON),
            "state_lifecycle_closure": rel(LIFECYCLE_JSON),
            "production_reference_gap_report": rel(GAP_REPORT_JSON),
            "production_reference_rust": rel(PRODUCTION_REFERENCE_RS),
        },
        "input_status": {
            "lift_status": lift["status"],
            "discharge_status": discharge["status"],
            "lifecycle_closure_status": lifecycle["closure_status"],
            "gap_report_schema": gap_report.get("schema"),
        },
        "required_epoch_gate": epoch_gate,
        "state_transitions": transitions,
        "negative_controls": negative_controls,
        "automated_code_presence_checks": checks,
        "existing_evidence_can_discharge_subset": subset_discharge,
        "discharge_decision": {
            "can_discharge_adv_postchallenge_slot_creation": epoch_gate_present,
            "can_discharge_adjacent_subsets": adjacent_controls_present,
            "reason": (
                "Existing reference evidence supports adjacent pad digest, witness "
                "commitment, restore-anchor, and reorg burn behavior. It does not "
                "exhibit a CreateSlot API with a challenge_eligibility_epoch or "
                "challenge_id gate, and it does not cite the required postchallenge "
                "CreateSlot negative controls. Therefore Adv_postchallenge_slot_creation(q_create) "
                "remains a residual blocker."
            ),
        },
        "required_implementation": [
            "Add a selected-path challenge_eligibility_epoch and challenge_id to the lifecycle harness.",
            "Freeze challenge_tuple_digest and prechallenge_eligible_set_digest when the challenge bit is sampled.",
            "Route every CreateSlot, Restore, and ReorgAttempt that could affect challenge eligibility through the epoch gate.",
            "Return reject_postchallenge_challenge_eligible as a public no-state-mutation outcome for colliding postchallenge slot creation attempts.",
            "Persist enough public transcript material to prove that accepted postchallenge disjoint slots are excluded from the sampled challenge set.",
        ],
        "required_tests": [item["id"] for item in negative_controls],
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Postchallenge Slot Creation Gate",
        "",
        "Closure artifact for the lifecycle residual `Adv_postchallenge_slot_creation(q_create)`.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- advantage term: `{payload['advantage_term']}`",
        f"- can discharge term: `{payload['discharge_decision']['can_discharge_adv_postchallenge_slot_creation']}`",
        "",
        payload["discharge_decision"]["reason"],
        "",
        "## Required Epoch Gate",
        "",
        f"- name: `{payload['required_epoch_gate']['name']}`",
        f"- reject code: `{payload['required_epoch_gate']['reject_code']}`",
        f"- gate rule: {payload['required_epoch_gate']['gate_rule']}",
        "- state variables:",
    ]
    lines.extend(f"  - `{item}`" for item in payload["required_epoch_gate"]["state_variables"])
    lines.append("- reject semantics:")
    lines.extend(f"  - {item}" for item in payload["required_epoch_gate"]["reject_semantics"])

    lines.extend(
        [
            "",
            "## State Transitions",
            "",
            "| from | query | precondition | decision | next state |",
            "|---|---|---|---|---|",
        ]
    )
    for item in payload["state_transitions"]:
        lines.append(
            f"| `{item['from']}` | `{item['query']}` | {item['precondition']} | {item['decision']} | {item['next_state']} |"
        )

    lines.extend(
        [
            "",
            "## Negative Controls",
            "",
            "| id | postchallenge action | expected | disabled-gate control | existing evidence |",
            "|---|---|---|---|---|",
        ]
    )
    for item in payload["negative_controls"]:
        lines.append(
            f"| `{item['id']}` | {item['postchallenge_action']} | {item['expected']} | {item['disabled_gate_control']} | `{item['covered_by_existing_evidence']}` |"
        )

    lines.extend(
        [
            "",
            "## Existing Evidence Subsets",
            "",
            "| subset | status | limits |",
            "|---|---|---|",
        ]
    )
    for item in payload["existing_evidence_can_discharge_subset"]:
        lines.append(f"| {item['subset']} | `{item['status']}` | {item['limits']} |")

    lines.extend(
        [
            "",
            "## Automated Presence Checks",
            "",
            "| id | present | path | rationale |",
            "|---|---|---|---|",
        ]
    )
    for item in payload["automated_code_presence_checks"]:
        lines.append(
            f"| `{item['id']}` | `{item['present']}` | `{item['path']}` | {item['rationale']} |"
        )

    lines.extend(["", "## Required Implementation", ""])
    lines.extend(f"- {item}" for item in payload["required_implementation"])
    lines.extend(["", "## Required Tests", ""])
    lines.extend(f"- `{item}`" for item in payload["required_tests"])
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
                "status": payload["status"],
                "can_discharge_adv_postchallenge_slot_creation": payload[
                    "discharge_decision"
                ]["can_discharge_adv_postchallenge_slot_creation"],
                "negative_controls": len(payload["negative_controls"]),
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
