#!/usr/bin/env python3
"""Discharge the lifecycle same-game carried term where evidence is available."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.md"
)
COMPOSITION_JSON = (
    READINESS_DIR
    / "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
)
LIFT_JSON = (
    READINESS_DIR / "pre_signature_projection_lifecycle_same_game_lift_n32_q12289.json"
)
LIFECYCLE_JSON = READINESS_DIR / "state_lifecycle_closure_n32_q12289.json"
PAD_REUSE_OPTIONS_JSON = (
    READINESS_DIR
    / "pre_signature_projection_pad_seed_reuse_closure_options_n32_q12289.json"
)
SPEND_BUDGET_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_leakage_budget_n32_q12289.json"
)
GAP_REPORT_JSON = READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
LEAKAGE_AUDIT_MD = ROOT / "research/pre_signature_projection_leakage_audit.md"
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


def checks_by_id(checks: list[dict[str, Any]]) -> dict[str, bool]:
    return {item["id"]: bool(item["present"]) for item in checks}


def all_present(presence: dict[str, bool], ids: list[str]) -> bool:
    return all(presence[item] for item in ids)


def build_payload() -> dict[str, Any]:
    composition = load_json(COMPOSITION_JSON)
    lift = load_json(LIFT_JSON)
    lifecycle = load_json(LIFECYCLE_JSON)
    pad_options = load_json(PAD_REUSE_OPTIONS_JSON)
    spend_budget = load_json(SPEND_BUDGET_JSON)
    gap_report = load_json(GAP_REPORT_JSON)

    checks = [
        code_check(
            "rust_account_state_unique_pad_seed_digest_test",
            PRODUCTION_REFERENCE_RS,
            "fn state_assisted_account_schema_enforces_unique_pad_seed_digest()",
            "live account-state construction and persisted-record restore reject duplicate pad_seed_digest values",
        ),
        code_check(
            "rust_slot_opening_pad_seed_digest_mismatch_test",
            PRODUCTION_REFERENCE_RS,
            '"state-assisted pad seed digest mismatch"',
            "slot opening binds the stored public pad_seed_digest before accepting an opening",
        ),
        code_check(
            "rust_spent_record_pad_seed_digest_persistence",
            PRODUCTION_REFERENCE_RS,
            "transition.spend_record.spent_record.pad_seed_digest",
            "selected-projection spends persist pad_seed_digest in the spent-slot record",
        ),
        code_check(
            "rust_target_chain_duplicate_pad_digest_reject",
            PRODUCTION_REFERENCE_RS,
            '"pad seed digest is already in spent ledger"',
            "target-chain commit validation rejects duplicate selected-projection pad_seed_digest records",
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
            "rust_slot_bound_packed_witness_replay_reject",
            PRODUCTION_REFERENCE_RS,
            '"state-assisted witness commitment mismatch"',
            "alternate packed witness bytes under the original slot are rejected by the slot-bound verifier",
        ),
        code_check(
            "rust_repacked_witness_commitment_check",
            PRODUCTION_REFERENCE_RS,
            "packed_opening_a.repacked_witness_commitment",
            "masked state18 packed-witness opening repacks to the precommitted witness commitment",
        ),
        code_check(
            "gap_report_account_pad_digest_invariant",
            GAP_REPORT_JSON,
            '"state_assisted_pad_seed_digest_account_invariant_present": true',
            "preflight gap report records account-state pad_seed_digest invariant presence",
        ),
        code_check(
            "gap_report_spent_pad_digest_invariant",
            GAP_REPORT_JSON,
            '"selected_projection_spent_pad_seed_digest_invariant_present": true',
            "preflight gap report records selected-projection spent-ledger pad_seed_digest invariant presence",
        ),
        code_check(
            "gap_report_target_chain_pad_digest_validation",
            GAP_REPORT_JSON,
            '"target_chain_pad_seed_digest_commit_validation_present": true',
            "preflight gap report records target-chain pad_seed_digest commit validation presence",
        ),
        code_check(
            "audit_notes_remaining_lifecycle_work",
            LEAKAGE_AUDIT_MD,
            "remaining lifecycle work is final\ndeployment database review",
            "human audit explicitly keeps deployment database review and theorem integration open",
        ),
    ]
    presence = checks_by_id(checks)

    pad_seed_closed = all_present(
        presence,
        [
            "rust_account_state_unique_pad_seed_digest_test",
            "rust_slot_opening_pad_seed_digest_mismatch_test",
            "rust_spent_record_pad_seed_digest_persistence",
            "rust_target_chain_duplicate_pad_digest_reject",
            "gap_report_account_pad_digest_invariant",
            "gap_report_spent_pad_digest_invariant",
            "gap_report_target_chain_pad_digest_validation",
        ],
    )
    restore_reorg_closed = all_present(
        presence,
        [
            "rust_restore_anchor_replay_reject",
            "rust_reorg_or_recovery_burn_reserved_slot",
        ],
    )
    packed_witness_partial = all_present(
        presence,
        [
            "rust_slot_bound_packed_witness_replay_reject",
            "rust_repacked_witness_commitment_check",
        ],
    )

    discharge_items = [
        {
            "id": "pad_seed_digest_uniqueness",
            "breaks_addressed": ["B2_pad_seed_duplicate"],
            "advantage_terms": ["Adv_pad_seed_duplicate(q_slots)"],
            "status": (
                "discharged_for_reference_account_state_and_spent_ledger"
                if pad_seed_closed
                else "residual_blocker"
            ),
            "evidence": [
                "rust_account_state_unique_pad_seed_digest_test",
                "rust_slot_opening_pad_seed_digest_mismatch_test",
                "rust_spent_record_pad_seed_digest_persistence",
                "rust_target_chain_duplicate_pad_digest_reject",
                "gap_report_account_pad_digest_invariant",
                "gap_report_spent_pad_digest_invariant",
                "gap_report_target_chain_pad_digest_validation",
            ],
            "fail_closed_rule": (
                "A selected-path account state with duplicate or zero public "
                "pad_seed_digest values is rejected before slot eligibility; an "
                "accepted selected-projection spend persists the digest and target-chain "
                "commit validation rejects a duplicate digest in the spent ledger."
            ),
            "residual_if_any": [
                "Bind the same invariant to the final deployment database profile and external storage review.",
                "Compose the executable invariant into the final same-game theorem rather than relying on prose linkage.",
            ],
        },
        {
            "id": "postchallenge_slot_creation_rejection",
            "breaks_addressed": ["B1_postchallenge_slot_creation"],
            "advantage_terms": ["Adv_postchallenge_slot_creation(q_create)"],
            "status": "residual_blocker",
            "evidence": [],
            "fail_closed_rule": (
                "The lift states that postchallenge attempts to create or make "
                "eligible the challenge-bearing tuple reject, but the current "
                "evidence does not cite an executable same-game challenge epoch gate."
            ),
            "required_implementation": [
                "Add an explicit challenge_eligibility_epoch or challenge_id to the selected-path lifecycle harness.",
                "Make CreateSlot reject after challenge sampling when the request creates, rewrites, restores, or reorg-selects the challenge-bearing (pk_hash, slot_id, pad_seed_digest, packed_witness_commitment) tuple.",
                "Persist the rejection as a public reject/no-state-mutation outcome, not as a partially reserved slot or burn that changes challenge eligibility.",
            ],
            "required_tests": [
                "Pre-read an eligible slot, sample the challenge bit, then attempt CreateSlot with the same slot_id and a different packed witness; expect reject_postchallenge_challenge_eligible and unchanged account state.",
                "After challenge sampling, attempt CreateSlot with a fresh slot_id but duplicate pad_seed_digest selected for the challenge; expect public reject and no digest-ledger mutation.",
                "Restore or reorg to a snapshot that introduces a challenge-bearing tuple not present in the prechallenge transcript; expect reject/burn and no eligibility.",
            ],
        },
        {
            "id": "packed_witness_immutability",
            "breaks_addressed": ["B5_packed_witness_rewrite"],
            "advantage_terms": ["Adv_witness_rewrite(q_open)"],
            "status": (
                "partially_discharged_for_slot_bound_opening"
                if packed_witness_partial
                else "residual_blocker"
            ),
            "evidence": [
                "rust_slot_bound_packed_witness_replay_reject",
                "rust_repacked_witness_commitment_check",
            ],
            "fail_closed_rule": (
                "The slot-bound masked verifier derives the opening commitment "
                "from the precommitted account slot. A different 18-row packed "
                "witness under the same slot key rejects as a witness commitment "
                "mismatch, and accepted packed-witness openings must repack to "
                "the stored witness commitment."
            ),
            "residual_if_any": [
                "Add a same-game negative control that captures an account-state read transcript, samples the challenge, and then tries to sign, verify, restore, and spend with mutated packed_witness_bytes under the same slot key.",
                "Record the mutated-witness rejection in the lifecycle discharge artifact as a challenge-transcript invariant, not only as a local verifier mismatch.",
            ],
        },
        {
            "id": "restore_reorg_replay",
            "breaks_addressed": [
                "B3_slot_id_reuse_or_restore_reanimation",
                "B4_reorg_reveal_unburn",
            ],
            "advantage_terms": [
                "Adv_slot_reanimation(q_restore, q_slots)",
                "Adv_reorg_unburn(q_reorg)",
            ],
            "status": (
                "discharged_for_policy_and_reference_controls"
                if restore_reorg_closed
                else "residual_blocker"
            ),
            "evidence": [
                "state_lifecycle_closure",
                "rust_restore_anchor_replay_reject",
                "rust_reorg_or_recovery_burn_reserved_slot",
            ],
            "fail_closed_rule": (
                "The trusted restore anchor is monotonic and identity-bound; "
                "reserved, spent, and burned ledgers are authoritative over slot "
                "cursors; reorg or recovery of an ambiguous revealed/reserved "
                "slot records a burn instead of reviving eligibility."
            ),
            "residual_if_any": [
                "Keep final deployment database review open until the production target chain uses the same anchor and burn ledgers.",
            ],
        },
        {
            "id": "spend_burn_ledger_behavior",
            "breaks_addressed": [
                "B3_slot_id_reuse_or_restore_reanimation",
                "B4_reorg_reveal_unburn",
                "B2_pad_seed_duplicate",
            ],
            "advantage_terms": [
                "Adv_slot_reanimation(q_restore, q_slots)",
                "Adv_reorg_unburn(q_reorg)",
                "Adv_pad_seed_duplicate(q_slots)",
            ],
            "status": (
                "discharged_for_reference_selected_projection_ledger"
                if pad_seed_closed and restore_reorg_closed
                else "residual_blocker"
            ),
            "evidence": [
                "rust_spent_record_pad_seed_digest_persistence",
                "rust_target_chain_duplicate_pad_digest_reject",
                "rust_restore_anchor_replay_reject",
                "rust_reorg_or_recovery_burn_reserved_slot",
            ],
            "fail_closed_rule": (
                "Accepted selected-projection spends atomically consume the slot, "
                "persist a spent-slot key plus pad_seed_digest, advance the trusted "
                "anchor, and reject later duplicate slot or duplicate digest commits."
            ),
            "residual_if_any": [
                "Run the same conformance checks against the selected production chain database profile after external storage review.",
            ],
        },
        {
            "id": "spend_time_projection_overread",
            "breaks_addressed": ["B6_spend_time_projection_overread"],
            "advantage_terms": ["Adv_projection_overread(q_sign, q_verify)"],
            "status": "residual_blocker",
            "evidence": ["spend_time_leakage_budget"],
            "fail_closed_rule": (
                "The spend-time budget enumerates selected projection disclosures "
                "and marks account_projection/projection_masks as allowed only in "
                "the accepting certificate path; the no-pre-spend export audit and "
                "negative tests remain open."
            ),
            "required_implementation": [
                "Audit public APIs for account_projection, projection_masks, account_witness_projection_opening_frame, and pad_seed byte export outside accepted certificates.",
                "Centralize selected projection opening material behind the accepting certificate constructor or a private verifier-witness-only type.",
            ],
            "required_tests": [
                "AccountStateRead, snapshot export, malformed opening, wrong pad seed, carrier rebinding, hidden-material export, and spent-slot replay must never return the six unmasked projection residues.",
            ],
        },
    ]

    residual_terms = [
        {
            "id": "B1_postchallenge_slot_creation",
            "term": "Adv_postchallenge_slot_creation(q_create)",
            "blocker": "no executable challenge epoch gate or postchallenge CreateSlot negative control is cited",
        },
        {
            "id": "B5_packed_witness_rewrite_full_same_game",
            "term": "Adv_witness_rewrite(q_open)",
            "blocker": "slot-bound verifier rejects alternate witnesses, but a full postchallenge read/sign/verify/restore same-game negative control is still needed",
        },
        {
            "id": "B6_spend_time_projection_overread",
            "term": "Adv_projection_overread(q_sign, q_verify)",
            "blocker": "no-pre-spend projection export static audit and negative tests remain open",
        },
        {
            "id": "deployment_database_binding",
            "term": "Adv_lifecycle_db_profile_mismatch(q_life)",
            "blocker": "reference lifecycle and ledger controls must be bound to the selected production target-chain database profile",
        },
    ]

    return {
        "schema": "Descender.pre_signature_projection_lifecycle_same_game_discharge.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "lifecycle_same_game_discharge_fail_closed",
        "deployment_ready": False,
        "carried_assumption": {
            "id": "A_lifecycle_same_game",
            "source": rel(COMPOSITION_JSON),
            "original_term": next(
                item["term"]
                for item in composition["carried_assumptions"]
                if item["id"] == "A_lifecycle_same_game"
            ),
        },
        "target": lift["target"],
        "source_artifacts": {
            "composition": rel(COMPOSITION_JSON),
            "lifecycle_same_game_lift": rel(LIFT_JSON),
            "state_lifecycle_closure": rel(LIFECYCLE_JSON),
            "pad_seed_reuse_closure_options": rel(PAD_REUSE_OPTIONS_JSON),
            "spend_time_leakage_budget": rel(SPEND_BUDGET_JSON),
            "production_reference_gap_report": rel(GAP_REPORT_JSON),
            "production_reference_leakage_audit": rel(LEAKAGE_AUDIT_MD),
            "production_reference_rust": rel(PRODUCTION_REFERENCE_RS),
        },
        "input_status": {
            "composition_status": composition["status"],
            "lift_status": lift["status"],
            "lifecycle_closure_status": lifecycle["closure_status"],
            "pad_seed_reuse_closure_status": pad_options["status"],
            "spend_time_budget_status": spend_budget["status"],
            "gap_report_lifecycle_invariants": gap_report["deployment_preflight"][
                "certificate_export"
            ]["lifecycle_invariants"],
        },
        "automated_code_presence_checks": checks,
        "discharge_items": discharge_items,
        "residual_terms": residual_terms,
        "discharged_terms": [
            "Adv_pad_seed_duplicate(q_slots) for the reference account-state and selected-projection spent-ledger path",
            "Adv_slot_reanimation(q_restore, q_slots) for the policy/reference restore-anchor and slot-ledger path",
            "Adv_reorg_unburn(q_reorg) for the policy/reference revealed-slot burn path",
            "the slot-bound verifier subset of Adv_witness_rewrite(q_open)",
        ],
        "fail_closed_decision": {
            "can_claim_A_lifecycle_same_game_zero": False,
            "can_reduce_carried_term": True,
            "reason": (
                "Existing evidence discharges pad_seed_digest uniqueness, "
                "restore/reorg replay, and selected-projection spend/burn ledger "
                "behavior for the reference path. The term is not zero because "
                "postchallenge slot creation rejection, full packed-witness "
                "same-game immutability controls, no-pre-spend projection export "
                "evidence, and final deployment database binding remain open."
            ),
        },
        "residual_advantage_bound": [
            "A_lifecycle_same_game <=",
            "  Adv_postchallenge_slot_creation(q_create)",
            "  + Adv_witness_rewrite_full_same_game(q_open)",
            "  + Adv_projection_overread(q_sign, q_verify)",
            "  + Adv_lifecycle_db_profile_mismatch(q_life)",
        ],
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Lifecycle Same-Game Discharge",
        "",
        "Fail-closed discharge artifact for carried term `A_lifecycle_same_game`",
        "on the selected 308-byte/Fq12 state-assisted masked projection-carrier path.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- can claim term zero: `{payload['fail_closed_decision']['can_claim_A_lifecycle_same_game_zero']}`",
        f"- can reduce carried term: `{payload['fail_closed_decision']['can_reduce_carried_term']}`",
        f"- original term: `{payload['carried_assumption']['original_term']}`",
        "",
        payload["fail_closed_decision"]["reason"],
        "",
        "## Discharge Matrix",
        "",
        "| item | status | terms | fail-closed rule |",
        "|---|---|---|---|",
    ]
    for item in payload["discharge_items"]:
        terms = ", ".join(f"`{term}`" for term in item["advantage_terms"])
        lines.append(
            f"| `{item['id']}` | `{item['status']}` | {terms} | {item['fail_closed_rule']} |"
        )

    lines.extend(["", "## Discharged Terms", ""])
    lines.extend(f"- {term}" for term in payload["discharged_terms"])

    lines.extend(["", "## Residual Terms", ""])
    lines.extend(
        f"- `{item['id']}`: `{item['term']}`. {item['blocker']}"
        for item in payload["residual_terms"]
    )

    lines.extend(["", "## Exact Remaining Implementation And Tests", ""])
    for item in payload["discharge_items"]:
        required_impl = item.get("required_implementation", [])
        required_tests = item.get("required_tests", [])
        residual = item.get("residual_if_any", [])
        if not required_impl and not required_tests and not residual:
            continue
        lines.extend([f"### {item['id']}", ""])
        if required_impl:
            lines.append("Implementation:")
            lines.extend(f"- {entry}" for entry in required_impl)
            lines.append("")
        if required_tests:
            lines.append("Tests:")
            lines.extend(f"- {entry}" for entry in required_tests)
            lines.append("")
        if residual:
            lines.append("Residual:")
            lines.extend(f"- {entry}" for entry in residual)
            lines.append("")

    lines.extend(["## Evidence Checks", ""])
    lines.extend(
        f"- `{item['id']}`: `{item['present']}` in `{item['path']}`"
        for item in payload["automated_code_presence_checks"]
    )

    lines.extend(["", "## Residual Bound", ""])
    lines.extend(f"- `{line}`" for line in payload["residual_advantage_bound"])
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
                "can_claim_term_zero": payload["fail_closed_decision"][
                    "can_claim_A_lifecycle_same_game_zero"
                ],
                "residual_terms": len(payload["residual_terms"]),
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
