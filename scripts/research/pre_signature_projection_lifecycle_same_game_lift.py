#!/usr/bin/env python3
"""Generate the same-game lifecycle lift for masked projection carrier state."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_lifecycle_same_game_lift_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_lifecycle_same_game_lift_n32_q12289.md"
)
MASK_SIMULATOR_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.json"
)
LIFECYCLE_JSON = READINESS_DIR / "state_lifecycle_closure_n32_q12289.json"
PAD_REUSE_JSON = (
    READINESS_DIR / "pre_signature_projection_pad_seed_reuse_surface_n32_q12289.json"
)
PAD_REUSE_OPTIONS_JSON = (
    READINESS_DIR
    / "pre_signature_projection_pad_seed_reuse_closure_options_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    mask_simulator = load_json(MASK_SIMULATOR_JSON)
    lifecycle = load_json(LIFECYCLE_JSON)
    pad_reuse = load_json(PAD_REUSE_JSON)
    pad_reuse_options = load_json(PAD_REUSE_OPTIONS_JSON)

    target = {
        "n": 32,
        "q": 12_289,
        "selected_signature_bytes": 308,
        "carrier": "state_assisted_masked_projection",
        "packed_witness_rows": 18,
        "checked_core_rows": 12,
        "masked_projection_rows": 6,
        "state_field": "Fq12",
    }

    experiment = {
        "id": "Exp_lifecycle_same_game_lift_selected_308B_Fq12",
        "purpose": (
            "Run the projection-mask real/ideal challenge in the same game that "
            "also exposes account-state reads, slot creation, restore/reorg "
            "attempts, signing/spend attempts, and verifier queries."
        ),
        "prechallenge_commit_rule": (
            "Before the challenge bit is sampled, every challenge-eligible slot "
            "has a fixed tuple (public_key_hash, account_identity, snapshot_sequence, "
            "slot_id, pad_seed_digest, packed_witness_commitment, packed_witness_bytes). "
            "The adversary may read the public account-state view before choosing "
            "which eligible slot to challenge, but cannot create, restore, rewrite, "
            "or reorg-select that tuple after the challenge bit."
        ),
        "challenge_worlds": {
            "real": (
                "The six projection-carrier lanes in packed_witness_bytes contain "
                "raw_projection_i + mask_i mod q, with mask_i derived from the "
                "projection-mask domain, fixed pad_seed, fixed slot_id, and lane id."
            ),
            "ideal": (
                "The same six lane positions are replaced by independent uniform "
                "Z_q residues; all checked-core rows, commitments, slot metadata, "
                "restore anchors, and verifier behavior not determined by those "
                "six bytes are preserved."
            ),
        },
        "oracle_queries": [
            {
                "name": "AccountStateRead(pk_hash)",
                "allowed": "prechallenge_and_postchallenge",
                "returns": [
                    "public slot metadata",
                    "snapshot_sequence",
                    "slot_id",
                    "pad_seed_digest",
                    "packed_witness_commitment",
                    "packed_witness_bytes containing masked projection lanes",
                    "trusted restore anchor digest or sequence",
                ],
                "restriction": (
                    "Reads never reveal pad_seed, raw selected projection residues, "
                    "projection-mask PRF lane outputs, or account witness opening "
                    "preimages before an accepted spend opening."
                ),
            },
            {
                "name": "CreateSlot(pk_hash, slot_id, packed_witness_bytes)",
                "allowed": "prechallenge_and_postchallenge",
                "returns": ["accept", "reject", "reject_postchallenge_challenge_eligible"],
                "restriction": (
                    "Accept only if slot_id is fresh for the public-key identity, "
                    "pad_seed_digest is fresh under the selected-path policy, and "
                    "the packed witness commitment is bound before challenge for "
                    "any challenge-eligible slot. Postchallenge attempts to create "
                    "or make eligible the challenge-bearing tuple reject."
                ),
            },
            {
                "name": "Restore(snapshot, claimed_anchor)",
                "allowed": "prechallenge_and_postchallenge",
                "returns": ["restored", "reject_and_burn_if_ambiguous"],
                "restriction": (
                    "Reject if the restored snapshot sequence is below the trusted "
                    "identity-bound anchor; restored challenge slots cannot lower "
                    "or rewrite pad_seed_digest, slot_id, or packed witness bytes."
                ),
            },
            {
                "name": "ReorgAttempt(trace)",
                "allowed": "prechallenge_and_postchallenge",
                "returns": ["state_after_reorg", "burn_record", "reject"],
                "restriction": (
                    "Any revealed signature or ambiguous reserved/opened slot is "
                    "burned under the same public-key-hash/slot-id key; a reorg "
                    "cannot make a revealed challenge slot eligible again."
                ),
            },
            {
                "name": "SignOrSpend(slot_id, message)",
                "allowed": "prechallenge_and_postchallenge_except_challenge_opening_rules",
                "returns": [
                    "signature_or_spend_certificate",
                    "public reject",
                    "burn_record",
                ],
                "restriction": (
                    "The challenge slot may be spent at most once. An accepted "
                    "spend consumes the slot, records the slot key and pad_seed_digest "
                    "according to the selected lifecycle policy, and charges any "
                    "selected projection opening to spend-time leakage."
                ),
            },
            {
                "name": "Verify(certificate, message)",
                "allowed": "prechallenge_and_postchallenge",
                "returns": ["accept", "public reject"],
                "restriction": (
                    "Verifier queries share the same spent/burned slot ledger as "
                    "wallet lifecycle recovery; duplicate slot_id or duplicate "
                    "pad_seed_digest paths fail closed."
                ),
            },
        ],
    }

    fixed_single_use = [
        {
            "field": "pad_seed",
            "prechallenge_fixed_by": [
                "pad_seed_digest in the account-state slot record",
                "packed witness commitment",
                "slot-opening commitment",
            ],
            "single_use_rule": (
                "For the selected path, one pad_seed_digest may appear in at most "
                "one live or accepted account slot for a public-key identity. A "
                "duplicate digest is either rejected at account-state construction "
                "or rejected by a durable spend-time digest ledger; until one path "
                "is implemented, reuse is charged to a named break."
            ),
            "failure_event": "E_pad_seed_duplicate_or_postchallenge_selected",
        },
        {
            "field": "slot_id",
            "prechallenge_fixed_by": [
                "identity-bound slot key",
                "snapshot sequence",
                "spent/reserved/burned ledger membership",
            ],
            "single_use_rule": (
                "For each public-key identity, slot_id is accepted once across "
                "reserved, spent, and burned ledgers; restore and reorg cannot "
                "remove that fact."
            ),
            "failure_event": "E_slot_id_reuse_or_reanimation",
        },
        {
            "field": "packed_witness_bytes",
            "prechallenge_fixed_by": [
                "packed_witness_commitment",
                "account-state read transcript",
                "challenge eligibility snapshot",
            ],
            "single_use_rule": (
                "The 18-row packed witness byte string used for the challenge slot "
                "is immutable after challenge. Any restore, signing, or verifier "
                "path that supplies different bytes for the same slot key rejects "
                "as a commitment mismatch."
            ),
            "failure_event": "E_packed_witness_rewrite",
        },
    ]

    lifecycle_breaks = [
        {
            "id": "B1_postchallenge_slot_creation",
            "event": (
                "challenge-bearing slot is created, selected, or made eligible "
                "after the real/ideal bit is sampled"
            ),
            "advantage_term": "Adv_postchallenge_slot_creation(q_create)",
            "closed_by_policy": False,
            "supported_by_existing_controls": False,
        },
        {
            "id": "B2_pad_seed_duplicate",
            "event": (
                "two live or accepted slots for the same public-key identity share "
                "a pad_seed_digest, or an accepted trace cannot prove freshness"
            ),
            "advantage_term": "Adv_pad_seed_duplicate(q_slots)",
            "closed_by_policy": False,
            "supported_by_existing_controls": False,
        },
        {
            "id": "B3_slot_id_reuse_or_restore_reanimation",
            "event": (
                "restore, backup replay, stale snapshot, or slot cursor rollback "
                "makes a reserved, spent, or burned slot_id usable again"
            ),
            "advantage_term": "Adv_slot_reanimation(q_restore, q_slots)",
            "closed_by_policy": True,
            "supported_by_existing_controls": True,
        },
        {
            "id": "B4_reorg_reveal_unburn",
            "event": (
                "mempool, orphaned-block, or finalized-block reveal is rolled back "
                "without a persistent burn under the public-key-hash/slot-id key"
            ),
            "advantage_term": "Adv_reorg_unburn(q_reorg)",
            "closed_by_policy": True,
            "supported_by_existing_controls": True,
        },
        {
            "id": "B5_packed_witness_rewrite",
            "event": (
                "the adversary obtains a challenge account-state read for one "
                "packed witness but later opens, signs, restores, or verifies with "
                "different packed witness bytes under the same slot key"
            ),
            "advantage_term": "Adv_witness_rewrite(q_open)",
            "closed_by_policy": False,
            "supported_by_existing_controls": False,
        },
        {
            "id": "B6_spend_time_projection_overread",
            "event": (
                "a query learns selected projection residue information before an "
                "accepted spend opening, or outside the declared spend-time leakage "
                "interface"
            ),
            "advantage_term": "Adv_projection_overread(q_sign, q_verify)",
            "closed_by_policy": False,
            "supported_by_existing_controls": False,
        },
    ]

    advantage_bound = [
        "Adv_same_game_lifecycle_lift <=",
        "  Adv_mask_prf_projection_domain(q_mask, t)",
        "  + Adv_ro_domain_collision(q_hash)",
        "  + Adv_postchallenge_slot_creation(q_create)",
        "  + Adv_pad_seed_duplicate(q_slots)",
        "  + Adv_slot_reanimation(q_restore, q_slots)",
        "  + Adv_reorg_unburn(q_reorg)",
        "  + Adv_witness_rewrite(q_open)",
        "  + Adv_projection_overread(q_sign, q_verify)",
        "  + negligible_rejection_mismatch",
    ]

    supported_now = [
        {
            "claim": "identity-bound monotonic restore anchor rejects stale snapshots",
            "source": rel(LIFECYCLE_JSON),
            "support": lifecycle["closure_status"],
        },
        {
            "claim": "slot_id reuse through reserve/spend/burn ledgers is policy-rejected",
            "source": rel(LIFECYCLE_JSON),
            "support": "slot_reuse_prevention plus deployment acceptance criteria",
        },
        {
            "claim": "reorg and mempool reveal do not unspend or resurrect a slot_id",
            "source": rel(LIFECYCLE_JSON),
            "support": "revealed_signature_burn, no_reorg_unspend, rollback_advances_anchor",
        },
        {
            "claim": "pad-seed reuse is an exposed lifecycle surface, not closed by slot-id separation",
            "source": rel(PAD_REUSE_JSON),
            "support": (
                "pad_seed_digest_repeats="
                f"{pad_reuse['pad_seed_digest_repeats']}; "
                "pad_opening_statement_repeats="
                f"{pad_reuse['pad_opening_statement_repeats']}"
            ),
        },
        {
            "claim": "preferred closure is account-state precommit pad_seed_digest uniqueness",
            "source": rel(PAD_REUSE_OPTIONS_JSON),
            "support": pad_reuse_options["recommendation"]["preferred"],
        },
    ]

    remains_unproved = [
        {
            "id": "same_game_executable_negative_control",
            "detail": (
                "No executable test currently runs this full experiment with "
                "account-state reads, restore/reorg attempts, signing/spend attempts, "
                "and verifier queries against one challenge slot."
            ),
        },
        {
            "id": "pad_seed_digest_uniqueness_implementation",
            "detail": (
                "The preferred 32-byte public pad_seed_digest uniqueness rule is "
                "documented but not implemented in the owned scope; without it, "
                "B2 remains an explicit break term."
            ),
        },
        {
            "id": "packed_witness_immutability_negative_control",
            "detail": (
                "The same-game commitment mismatch rule is stated here, but an "
                "executable negative control for postchallenge packed witness "
                "rewrites is not cited by the current lifecycle closure artifact."
            ),
        },
        {
            "id": "spend_time_leakage_budget",
            "detail": (
                "The spend-time selected projection opening is still only a charged "
                "interface in the mask simulator; a proof that no pre-spend path "
                "exports those residues remains open."
            ),
        },
        {
            "id": "sha256_prf_or_ro_reduction",
            "detail": (
                "The lifecycle lift assumes the mask PRF replacement game supplied "
                "by the simulator; it does not prove the SHA-256 PRF/RO reduction."
            ),
        },
        {
            "id": "reject_equivalence",
            "detail": (
                "The artifact requires all lifecycle failures to collapse to public "
                "reject/burn behavior, but does not prove constant-time or oracle "
                "transcript equivalence for all malformed paths."
            ),
        },
    ]

    return {
        "schema": "Descender.pre_signature_projection_lifecycle_same_game_lift.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "lifecycle_same_game_lift_defined_fail_closed",
        "deployment_ready": False,
        "gap": "lifecycle_same_game_lift",
        "gap_disposition": (
            "The same-game experiment and break accounting are now explicit. The "
            "artifact is fail-closed because unsupported lifecycle paths become "
            "named advantage terms rather than assumed-zero conditions."
        ),
        "target": target,
        "source_artifacts": {
            "mask_simulator": {
                "path": rel(MASK_SIMULATOR_JSON),
                "status": mask_simulator.get("status"),
            },
            "state_lifecycle_closure": {
                "path": rel(LIFECYCLE_JSON),
                "status": lifecycle.get("closure_status"),
            },
            "pad_seed_reuse_surface": {
                "path": rel(PAD_REUSE_JSON),
                "status": pad_reuse.get("status"),
            },
            "pad_seed_reuse_closure_options": {
                "path": rel(PAD_REUSE_OPTIONS_JSON),
                "status": pad_reuse_options.get("status"),
            },
        },
        "experiment": experiment,
        "prechallenge_fixed_single_use_fields": fixed_single_use,
        "named_lifecycle_breaks": lifecycle_breaks,
        "advantage_bound": advantage_bound,
        "supported_now": supported_now,
        "remains_unproved": remains_unproved,
        "decision": {
            "can_claim_lifecycle_term_zero": False,
            "can_consume_in_mask_simulator": True,
            "reason": (
                "The artifact supplies the missing same-game lift contract and "
                "exact lifecycle breaks. Existing policy and negative controls "
                "support slot-id restore/reorg closure, but pad_seed_digest "
                "uniqueness, packed-witness immutability controls, spend-time "
                "leakage, PRF/RO reduction, and reject equivalence remain unproved."
            ),
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Lifecycle Same-Game Lift",
        "",
        "Fail-closed lifecycle lift for the selected 308-byte/Fq12 state-assisted",
        "masked projection-carrier path.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- gap: `{payload['gap']}`",
        f"- selected signature bytes: `{payload['target']['selected_signature_bytes']}`",
        f"- carrier: `{payload['target']['carrier']}`",
        f"- field: `{payload['target']['state_field']}`",
        "",
        payload["gap_disposition"],
        "",
        "## Experiment",
        "",
        f"`{payload['experiment']['id']}`",
        "",
        payload["experiment"]["purpose"],
        "",
        f"Prechallenge rule: {payload['experiment']['prechallenge_commit_rule']}",
        "",
        "Challenge worlds:",
        "",
        f"- real: {payload['experiment']['challenge_worlds']['real']}",
        f"- ideal: {payload['experiment']['challenge_worlds']['ideal']}",
        "",
        "Oracle surface:",
        "",
        "| query | allowed | restriction |",
        "|---|---|---|",
    ]
    for query in payload["experiment"]["oracle_queries"]:
        lines.append(
            f"| `{query['name']}` | `{query['allowed']}` | {query['restriction']} |"
        )

    lines.extend(
        [
            "",
            "## Prechallenge Fixed And Single-Use Fields",
            "",
            "| field | fixed by | single-use rule | break |",
            "|---|---|---|---|",
        ]
    )
    for item in payload["prechallenge_fixed_single_use_fields"]:
        fixed_by = ", ".join(f"`{value}`" for value in item["prechallenge_fixed_by"])
        lines.append(
            f"| `{item['field']}` | {fixed_by} | {item['single_use_rule']} | `{item['failure_event']}` |"
        )

    lines.extend(
        [
            "",
            "## Named Lifecycle Breaks",
            "",
            "| break | event | advantage term | supported now |",
            "|---|---|---|---|",
        ]
    )
    for item in payload["named_lifecycle_breaks"]:
        support = (
            "policy and executable controls"
            if item["supported_by_existing_controls"]
            else "open"
        )
        lines.append(
            f"| `{item['id']}` | {item['event']} | `{item['advantage_term']}` | {support} |"
        )

    lines.extend(["", "## Advantage Bound", ""])
    lines.extend(f"- `{term}`" for term in payload["advantage_bound"])

    lines.extend(
        [
            "",
            "## Supported Now",
            "",
            "| claim | source | support |",
            "|---|---|---|",
        ]
    )
    for item in payload["supported_now"]:
        lines.append(f"| {item['claim']} | `{item['source']}` | {item['support']} |")

    lines.extend(
        [
            "",
            "## Remains Unproved",
            "",
            "| item | detail |",
            "|---|---|",
        ]
    )
    for item in payload["remains_unproved"]:
        lines.append(f"| `{item['id']}` | {item['detail']} |")

    decision = payload["decision"]
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can claim lifecycle term zero: `{decision['can_claim_lifecycle_term_zero']}`",
            f"- can consume in mask simulator: `{decision['can_consume_in_mask_simulator']}`",
            f"- reason: {decision['reason']}",
            "",
        ]
    )
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
                "deployment_ready": payload["deployment_ready"],
                "named_lifecycle_breaks": len(payload["named_lifecycle_breaks"]),
                "remains_unproved": len(payload["remains_unproved"]),
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
