#!/usr/bin/env python3
"""Document controls for packed-witness rewrite across lifecycle operations."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_packed_witness_rewrite_controls_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_packed_witness_rewrite_controls_n32_q12289.md"
)
LIFECYCLE_DISCHARGE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json"
)
PRODUCTION_REFERENCE_RS = ROOT / "tools/parameter-search/src/production_reference.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def contains(path: Path, needle: str) -> bool:
    return needle in path.read_text()


def code_check(id_: str, needle: str, rationale: str) -> dict[str, Any]:
    return {
        "id": id_,
        "path": rel(PRODUCTION_REFERENCE_RS),
        "needle": needle,
        "present": contains(PRODUCTION_REFERENCE_RS, needle),
        "rationale": rationale,
    }


def presence(checks: list[dict[str, Any]]) -> dict[str, bool]:
    return {item["id"]: bool(item["present"]) for item in checks}


def all_present(present: dict[str, bool], ids: list[str]) -> bool:
    return all(present[item] for item in ids)


def mutation_cases() -> list[dict[str, Any]]:
    return [
        {
            "id": "read_transcript_same_slot_witness_commitment_swap",
            "phase": "read",
            "adversary_action": (
                "After an account-state read fixes slot_id, restore_anchor_digest, "
                "pad_seed_digest, and packed witness commitment, replace the "
                "packed witness bytes or commitment while preserving the slot key."
            ),
            "existing_controls": [
                "slot opening compares opening.account_witness_commitment against the stored account slot commitment",
                "slot opening also binds restore_anchor_digest and pad_seed_digest",
                "packed witness opening recomputes the selected projection from the stored commitment",
            ],
            "current_evidence": [
                "rust_slot_opening_witness_commitment_mismatch",
                "rust_slot_opening_restore_anchor_binding",
                "rust_slot_opening_pad_seed_binding",
                "rust_packed_opening_repack_check",
            ],
            "current_status": "slot_bound_subset_discharged",
            "missing_same_game_test": (
                "Capture a read transcript, sample the challenge bit, mutate the "
                "packed witness commitment for the same slot tuple, and assert the "
                "read-derived transcript cannot be reused by sign, verify, restore, "
                "or spend paths."
            ),
        },
        {
            "id": "sign_callback_mutates_witness_under_reserved_slot",
            "phase": "sign",
            "adversary_action": (
                "Reserve a slot, then make the signer callback build a signature "
                "against mutated packed_witness_bytes or a stale witness commitment."
            ),
            "existing_controls": [
                "sign_reserved_and_consume_sanitized passes the reserved slot's stored witness commitment to the callback",
                "transcript-bound account-carried signing rejects signatures whose challenge was not derived from the stored witness commitment",
                "callback failures are public and leave the reservation/account state unchanged",
            ],
            "current_evidence": [
                "rust_sign_reserved_and_consume_sanitized",
                "rust_fq12_transcript_binding_sign_rejects_stale_signature",
                "rust_sanitized_lifecycle_sign_failure_no_state_mutation",
            ],
            "current_status": "control_present_without_packed_same_game_negative",
            "missing_same_game_test": (
                "Use the packed state18 selected path, mutate the packed witness "
                "after reservation and after challenge sampling, then assert the "
                "sign operation returns a public reject and does not consume or "
                "rewrite the reserved slot."
            ),
        },
        {
            "id": "verify_accepts_carrier_with_repacked_witness_swap",
            "phase": "verify",
            "adversary_action": (
                "Submit a projection carrier whose public lanes came from one "
                "packed witness but whose account slot and hidden-source statement "
                "name a different witness commitment."
            ),
            "existing_controls": [
                "masked state18 verifier checks statement.witness_commitment against opening.account_witness_commitment",
                "slot-bound verifier checks opening.account_witness_commitment against the account slot",
                "packed opening must repack to the named witness commitment",
                "projection-carrier binding rejects exported hidden-material breaks",
            ],
            "current_evidence": [
                "rust_masked_projection_statement_commitment_check",
                "rust_slot_bound_packed_witness_replay_reject",
                "rust_packed_opening_repack_check",
                "rust_projection_carrier_hidden_material_break_reject",
            ],
            "current_status": "slot_bound_subset_discharged",
            "missing_same_game_test": (
                "Replay the full read/sign transcript through verify with the same "
                "slot_id and pad_seed_digest but mutated packed witness bytes; "
                "assert the verifier rejects before any accepting certificate path "
                "exports projection material."
            ),
        },
        {
            "id": "restore_snapshot_reintroduces_mutated_witness",
            "phase": "restore",
            "adversary_action": (
                "Restore a snapshot for the same public slot tuple, but with the "
                "account slot's packed witness commitment rewritten to match a "
                "postchallenge mutated witness."
            ),
            "existing_controls": [
                "restore-anchor policy rejects snapshots older than the trusted anchor",
                "account-state reconstruction validates commitment canonicality and pad_seed_digest uniqueness",
                "slot opening compares restored account slot fields against the opening transcript",
            ],
            "current_evidence": [
                "rust_restore_anchor_rejects_old_snapshot",
                "rust_state_assisted_account_state_from_slot_records",
                "rust_slot_opening_witness_commitment_mismatch",
            ],
            "current_status": "restore_policy_subset_discharged",
            "missing_same_game_test": (
                "After challenge sampling, restore or reorg-select a snapshot with "
                "the same slot_id and pad_seed_digest but a different packed witness "
                "commitment; assert public reject or burn and no challenge eligibility."
            ),
        },
        {
            "id": "spend_consumes_slot_after_witness_mutation_attempt",
            "phase": "spend",
            "adversary_action": (
                "Attempt to consume the selected slot using a signature or carrier "
                "derived from mutated packed witness bytes after the prechallenge "
                "slot read fixed the original commitment."
            ),
            "existing_controls": [
                "verify_and_consume_sanitized invokes account binding before accepted-path consumption",
                "selected Fq12 lifecycle spend records pad_seed_digest and spent slot state",
                "slot-bound verifier rejects witness commitment mismatch before accepted consumption",
            ],
            "current_evidence": [
                "rust_verify_with_account_binding_sanitized",
                "rust_reviewed_fq12_sign_reserved_and_consume_records_pad_digest",
                "rust_slot_bound_packed_witness_replay_reject",
            ],
            "current_status": "consume_path_controls_present_without_same_game_negative",
            "missing_same_game_test": (
                "Mutate packed witness bytes after read and challenge sampling, then "
                "run verify-and-consume; assert rejection leaves the slot unconsumed "
                "and does not append a spent-ledger record for the mutated witness."
            ),
        },
    ]


def required_state_binding() -> list[dict[str, Any]]:
    return [
        {
            "field": "slot_id",
            "required_binding": "public slot key selected before challenge",
            "current_control": "slot opening rejects opening.slot_id mismatch",
        },
        {
            "field": "account_witness_commitment",
            "required_binding": "packed witness commitment fixed in account state before challenge",
            "current_control": "slot opening and statement checks reject witness commitment mismatch",
        },
        {
            "field": "pad_seed_digest",
            "required_binding": "public pad seed digest fixed with the slot before challenge",
            "current_control": "slot opening rejects pad_seed_digest mismatch and spent ledger rejects duplicate digest",
        },
        {
            "field": "restore_anchor_digest",
            "required_binding": "trusted anchor fixes the restore epoch for the read transcript",
            "current_control": "slot opening rejects restore anchor mismatch and restore load rejects old snapshots",
        },
        {
            "field": "challenge_epoch_or_id",
            "required_binding": "explicit same-game marker proving the tuple was fixed before challenge sampling",
            "current_control": "missing; lifecycle artifact carries this as a residual blocker",
        },
        {
            "field": "read_transcript_digest",
            "required_binding": "digest of read slot tuple consumed by sign, verify, restore, and spend controls",
            "current_control": "missing as an executable same-game invariant",
        },
    ]


def build_payload() -> dict[str, Any]:
    lifecycle = load_json(LIFECYCLE_DISCHARGE_JSON)
    checks = [
        code_check(
            "rust_slot_opening_witness_commitment_mismatch",
            '"state-assisted witness commitment mismatch"',
            "slot-bound opening rejects alternate packed witness commitments under the original slot",
        ),
        code_check(
            "rust_slot_opening_restore_anchor_binding",
            '"state-assisted restore anchor mismatch"',
            "slot-bound opening binds the restore anchor digest",
        ),
        code_check(
            "rust_slot_opening_pad_seed_binding",
            '"state-assisted pad seed digest mismatch"',
            "slot-bound opening binds the pad seed digest",
        ),
        code_check(
            "rust_packed_opening_repack_check",
            "if &repacked_witness_commitment != witness_commitment",
            "masked state18 packed opening recomputes and checks the precommitted witness commitment",
        ),
        code_check(
            "rust_packed_opening_fields_mismatch_reject",
            '"reviewed Fq12 masked state18 packed witness opening fields mismatch"',
            "tampering with extracted packed-witness opening fields is rejected",
        ),
        code_check(
            "rust_masked_projection_statement_commitment_check",
            "statement.witness_commitment != opening.account_witness_commitment",
            "statement/opening mismatch rejects before projection carrier acceptance",
        ),
        code_check(
            "rust_projection_carrier_hidden_material_break_reject",
            '"reviewed Fq12 masked witness state18 projection carrier tuple exported a hidden-material break"',
            "carrier lanes from the wrong packed witness are rejected",
        ),
        code_check(
            "rust_sign_reserved_and_consume_sanitized",
            "pub fn sign_reserved_and_consume_sanitized<F>",
            "deployment-facing lifecycle signing uses the stored reserved slot witness commitment",
        ),
        code_check(
            "rust_verify_with_account_binding_sanitized",
            "pub fn verify_with_account_binding_sanitized<R, F>",
            "deployment-facing verification account-binds before callback acceptance",
        ),
        code_check(
            "rust_fq12_transcript_binding_sign_rejects_stale_signature",
            "reviewed Fq12 account-carried challenge mismatch",
            "Fq12 account-carried transcript binding rejects stale or mismatched challenges",
        ),
        code_check(
            "rust_sanitized_lifecycle_sign_failure_no_state_mutation",
            "assert!(manager.account().get_slot(0).unwrap().is_unused())",
            "sanitized lifecycle signer failure leaves the reserved slot unused",
        ),
        code_check(
            "rust_restore_anchor_rejects_old_snapshot",
            '"snapshot sequence is older than restore anchor"',
            "restore paths reject rollback below the trusted anchor",
        ),
        code_check(
            "rust_state_assisted_account_state_from_slot_records",
            "reviewed_fq12_state_assisted_account_state_from_slot_records",
            "restore reconstructs account state through the state-assisted slot schema",
        ),
        code_check(
            "rust_reviewed_fq12_sign_reserved_and_consume_records_pad_digest",
            "transition.spend_record.spent_record.pad_seed_digest",
            "selected Fq12 spend records pad_seed_digest in the spent ledger",
        ),
    ]
    present = presence(checks)

    slot_bound_subset = all_present(
        present,
        [
            "rust_slot_opening_witness_commitment_mismatch",
            "rust_slot_opening_restore_anchor_binding",
            "rust_slot_opening_pad_seed_binding",
            "rust_packed_opening_repack_check",
            "rust_packed_opening_fields_mismatch_reject",
            "rust_masked_projection_statement_commitment_check",
            "rust_projection_carrier_hidden_material_break_reject",
        ],
    )
    lifecycle_controls_present = all_present(
        present,
        [
            "rust_sign_reserved_and_consume_sanitized",
            "rust_verify_with_account_binding_sanitized",
            "rust_fq12_transcript_binding_sign_rejects_stale_signature",
            "rust_sanitized_lifecycle_sign_failure_no_state_mutation",
            "rust_restore_anchor_rejects_old_snapshot",
            "rust_state_assisted_account_state_from_slot_records",
            "rust_reviewed_fq12_sign_reserved_and_consume_records_pad_digest",
        ],
    )

    return {
        "schema": "Descender.pre_signature_projection_packed_witness_rewrite_controls.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "packed_witness_rewrite_controls_fail_closed_residual_same_game_tests_required",
        "deployment_ready": False,
        "target": lifecycle["target"],
        "advantage_term": "Adv_witness_rewrite(q_open)",
        "source_artifacts": {
            "lifecycle_same_game_discharge": rel(LIFECYCLE_DISCHARGE_JSON),
            "production_reference_rust": rel(PRODUCTION_REFERENCE_RS),
        },
        "upstream_lifecycle_status": {
            "status": lifecycle["status"],
            "can_claim_A_lifecycle_same_game_zero": lifecycle["fail_closed_decision"][
                "can_claim_A_lifecycle_same_game_zero"
            ],
            "residual_entry": next(
                item
                for item in lifecycle["residual_terms"]
                if item["id"] == "B5_packed_witness_rewrite_full_same_game"
            ),
        },
        "automated_code_presence_checks": checks,
        "existing_commitment_and_repacked_checks": {
            "slot_bound_subset_discharged": slot_bound_subset,
            "lifecycle_controls_present": lifecycle_controls_present,
            "covered": [
                "slot-bound witness commitment mismatch",
                "slot-bound restore_anchor_digest mismatch",
                "slot-bound pad_seed_digest mismatch",
                "masked packed-witness opening repack to stored commitment",
                "packed opening field tamper rejection",
                "statement/opening witness commitment equality",
                "projection carrier wrong-witness hidden-material break rejection",
            ],
            "not_covered": [
                "single executable read transcript digest carried across sign, verify, restore, and spend",
                "postchallenge mutation of packed_witness_bytes under the same slot tuple",
                "restore/reorg same-game rejection keyed by challenge epoch or challenge id",
                "spent-ledger no-mutation assertion for a rejected mutated-witness consume attempt",
            ],
        },
        "mutation_cases": mutation_cases(),
        "required_state_binding": required_state_binding(),
        "discharged_subset": [
            "Adv_witness_rewrite(q_open) is discharged only for the slot-bound verifier/opening subset where the account slot already fixes account_witness_commitment, pad_seed_digest, restore_anchor_digest, and slot_id.",
            "Packed opening tampering is discharged for the repack/field-validation subset.",
            "Lifecycle sign/verify/restore/spend boundaries have relevant fail-closed controls, but they are not yet composed into one same-game mutated-witness test.",
        ],
        "residual_blockers": [
            {
                "id": "missing_read_sign_verify_restore_same_game_negative",
                "term": "Adv_witness_rewrite_full_same_game(q_open)",
                "blocker": (
                    "No executable control captures a prechallenge read transcript, "
                    "samples the challenge, mutates packed_witness_bytes under the "
                    "same slot tuple, and proves sign, verify, restore, and spend "
                    "all reject without state mutation."
                ),
            },
            {
                "id": "missing_challenge_epoch_or_transcript_state_binding",
                "term": "Adv_witness_rewrite_full_same_game(q_open)",
                "blocker": (
                    "The reference path binds slot/opening fields, but lacks an "
                    "explicit challenge_epoch, challenge_id, or read_transcript_digest "
                    "that is consumed uniformly by lifecycle operations."
                ),
            },
            {
                "id": "deployment_database_profile_binding",
                "term": "Adv_lifecycle_db_profile_mismatch(q_life)",
                "blocker": (
                    "The same witness-immutability controls still need final binding "
                    "to the selected production target-chain database profile."
                ),
            },
        ],
        "required_tests": [
            "read_then_challenge_then_mutated_packed_witness_sign_rejects_without_consuming_reserved_slot",
            "read_then_challenge_then_mutated_packed_witness_verify_rejects_before_projection_export",
            "read_then_challenge_then_mutated_packed_witness_restore_rejects_or_burns_without_eligibility",
            "read_then_challenge_then_mutated_packed_witness_verify_and_consume_leaves_spent_ledger_unchanged",
        ],
        "closure_decision": {
            "can_discharge_full_Adv_witness_rewrite": False,
            "can_discharge_slot_bound_subset": slot_bound_subset,
            "can_use_as_control_artifact": slot_bound_subset and lifecycle_controls_present,
            "reason": (
                "Existing code binds packed witness commitments at slot/opening and "
                "repacked-opening boundaries. That closes the local slot-bound "
                "rewrite subset. Full same-game discharge remains blocked until a "
                "challenge-transcript invariant is tested across read, sign, verify, "
                "restore, and spend with explicit no-state-mutation outcomes."
            ),
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Packed-Witness Rewrite Controls",
        "",
        "Control artifact for `Adv_witness_rewrite(q_open)` on the selected",
        "308-byte/Fq12 state-assisted masked projection-carrier path.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- full discharge: `{payload['closure_decision']['can_discharge_full_Adv_witness_rewrite']}`",
        f"- slot-bound subset discharged: `{payload['closure_decision']['can_discharge_slot_bound_subset']}`",
        f"- usable as control artifact: `{payload['closure_decision']['can_use_as_control_artifact']}`",
        "",
        payload["closure_decision"]["reason"],
        "",
        "## Existing Commitment And Repacked Checks",
        "",
        f"- slot-bound subset discharged: `{payload['existing_commitment_and_repacked_checks']['slot_bound_subset_discharged']}`",
        f"- lifecycle controls present: `{payload['existing_commitment_and_repacked_checks']['lifecycle_controls_present']}`",
        "",
        "Covered:",
    ]
    lines.extend(
        f"- {item}" for item in payload["existing_commitment_and_repacked_checks"]["covered"]
    )
    lines.extend(["", "Not covered:"])
    lines.extend(
        f"- {item}" for item in payload["existing_commitment_and_repacked_checks"]["not_covered"]
    )

    lines.extend(["", "## Mutation Cases", ""])
    lines.append("| case | phase | status | missing same-game test |")
    lines.append("|---|---|---|---|")
    for item in payload["mutation_cases"]:
        lines.append(
            f"| `{item['id']}` | `{item['phase']}` | `{item['current_status']}` | {item['missing_same_game_test']} |"
        )

    lines.extend(["", "## Required State Binding", ""])
    lines.append("| field | required binding | current control |")
    lines.append("|---|---|---|")
    for item in payload["required_state_binding"]:
        lines.append(
            f"| `{item['field']}` | {item['required_binding']} | {item['current_control']} |"
        )

    lines.extend(["", "## Discharged Subset", ""])
    lines.extend(f"- {item}" for item in payload["discharged_subset"])

    lines.extend(["", "## Residual Blockers", ""])
    lines.extend(
        f"- `{item['id']}`: `{item['term']}`. {item['blocker']}"
        for item in payload["residual_blockers"]
    )

    lines.extend(["", "## Required Same-Game Tests", ""])
    lines.extend(f"- `{item}`" for item in payload["required_tests"])

    lines.extend(["", "## Evidence Checks", ""])
    lines.extend(
        f"- `{item['id']}`: `{item['present']}` in `{item['path']}`"
        for item in payload["automated_code_presence_checks"]
    )
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
                "full_discharge": payload["closure_decision"][
                    "can_discharge_full_Adv_witness_rewrite"
                ],
                "slot_bound_subset": payload["closure_decision"][
                    "can_discharge_slot_bound_subset"
                ],
                "residual_blockers": len(payload["residual_blockers"]),
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
