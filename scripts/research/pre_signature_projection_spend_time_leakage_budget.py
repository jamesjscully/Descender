#!/usr/bin/env python3
"""Generate the spend-time leakage budget for the masked projection path."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_leakage_budget_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "pre_signature_projection_spend_time_leakage_budget_n32_q12289.md"
)
SIMULATOR_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.json"
)
REUSE_JSON = READINESS_DIR / "pre_signature_projection_pad_seed_reuse_surface_n32_q12289.json"
LIFECYCLE_JSON = READINESS_DIR / "state_lifecycle_closure_n32_q12289.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def disclosed_value(
    name: str,
    source: str,
    category: str,
    phase: str,
    cardinality: str,
    charge: str,
    rule: str,
) -> dict[str, str]:
    return {
        "name": name,
        "source": source,
        "category": category,
        "phase": phase,
        "cardinality": cardinality,
        "charge": charge,
        "rule": rule,
    }


def build_payload() -> dict[str, Any]:
    simulator = load_json(SIMULATOR_JSON)
    reuse = load_json(REUSE_JSON)
    lifecycle = load_json(LIFECYCLE_JSON)

    pre_spend_live_slot_budget = {
        "phase": "pre_spend_live_slot",
        "allowed_public_values": [
            "slot metadata and account-state record metadata",
            "32-byte packed witness field containing 12 checked-core residues and six masked projection residues",
            "opening commitment digest",
            "restore anchor digest",
            "public key/account binding material",
            "public pad_seed_digest if present in the account-state schema",
        ],
        "forbidden_public_values": [
            "six unmasked selected account projection residues",
            "six projection-mask residues",
            "pad seed bytes",
            "account_witness_projection_opening_frame",
            "accepted soundness certificate fields derived from a valid opening",
        ],
        "simulator_rule": (
            "before an accepted spend opening, the six packed projection slots are "
            "charged only as masked_projection = account_projection + mask mod q; "
            "the live-slot simulator may replace those six masked residues with "
            "independent uniform q12289 residues under the mask-PRF assumption"
        ),
        "budget_status": "no_selected_projection_opening_budget_spent_pre_spend",
    }

    spend_opening_budget = {
        "phase": "accepted_spend_certificate",
        "trigger": (
            "verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate "
            "returns an accepting certificate after the slot-bound masked tuple, "
            "checked core, hidden-source statement, pad opening, carrier binding, "
            "and slot opening all verify"
        ),
        "selected_projection_residues_intentionally_public_after_spend": True,
        "selected_projection_residue_count": 6,
        "selected_projection_residue_modulus": 12289,
        "selected_projection_disclosure_rule": (
            "the six account_projection residues are intentionally disclosed only "
            "inside the accepting certificate path, either directly in "
            "packed_witness_opening.account_projection or equivalently inside "
            "account_witness_projection_opening_frame"
        ),
        "not_a_claim": (
            "the spend-time simulator does not hide the selected projection "
            "residues after a valid opening; it charges them as spend-opening "
            "disclosure and relies on transcript/lifecycle binding for replay"
        ),
    }

    accepting_certificate_disclosures = [
        disclosed_value(
            "slot_id",
            "ProductionReviewedFq12MaskedProjectionSoundnessCertificate.slot_id",
            "public binding",
            "accepted_spend_certificate",
            "one u32 slot index",
            "public",
            "binds the certificate to the account slot selected by checked-core verification",
        ),
        disclosed_value(
            "statement_digest",
            "certificate.statement_digest",
            "public binding",
            "accepted_spend_certificate",
            "32 bytes",
            "public",
            "binds the public key, message, account state, signature body, and pad-opening statement",
        ),
        disclosed_value(
            "opening_commitment",
            "certificate.opening_commitment",
            "public binding",
            "accepted_spend_certificate",
            "32 bytes",
            "public",
            "must equal the commitment derived from the prechallenge account slot opening",
        ),
        disclosed_value(
            "checked_core_challenge",
            "certificate.checked_core_challenge",
            "public binding",
            "accepted_spend_certificate",
            "challenge bytes from the checked-core signature",
            "public",
            "Fiat-Shamir challenge already carried by the accepted signature envelope",
        ),
        disclosed_value(
            "checked_core_witness_syndrome",
            "certificate.checked_core_witness_syndrome and packed_witness_opening.witness_syndrome",
            "checked-core leakage",
            "pre_spend_live_slot and accepted_spend_certificate",
            "12 q12289 residues",
            "already charged as checked-core account-state rows",
            "the opening must match the first 12 rows packed in the account witness field",
        ),
        disclosed_value(
            "checked_core_response_syndrome",
            "certificate.checked_core_response_syndrome",
            "checked-core leakage",
            "accepted_spend_certificate",
            "12 q12289 residues",
            "checked-core signature leakage",
            "public response syndrome derived from the accepted checked-core proof response",
        ),
        disclosed_value(
            "packed_witness_opening.digest",
            "certificate.packed_witness_opening.digest",
            "public binding",
            "accepted_spend_certificate",
            "32 bytes",
            "public",
            "commits to the structured masked state18 packed-witness opening frame",
        ),
        disclosed_value(
            "packed_witness_opening.slot_id",
            "certificate.packed_witness_opening.slot_id",
            "public binding",
            "accepted_spend_certificate",
            "one u64 slot index",
            "public",
            "must match the statement slot and projection-mask PRF slot input",
        ),
        disclosed_value(
            "packed_witness_opening.witness_commitment",
            "certificate.packed_witness_opening.witness_commitment",
            "public binding",
            "pre_spend_live_slot and accepted_spend_certificate",
            "32 bytes",
            "public",
            "the 18-residue account witness field committed in the live account slot",
        ),
        disclosed_value(
            "packed_witness_opening.masked_projection",
            "certificate.packed_witness_opening.masked_projection",
            "mask-derived value",
            "pre_spend_live_slot and accepted_spend_certificate",
            "six q12289 residues",
            "masked live-slot leakage",
            "equals account_projection_i + projection_mask_i mod q and is public before spend only in masked form",
        ),
        disclosed_value(
            "packed_witness_opening.projection_masks",
            "certificate.packed_witness_opening.projection_masks",
            "mask-derived value",
            "accepted_spend_certificate",
            "six q12289 residues",
            "spend-opening disclosure",
            "derived from pad_seed and slot_id; forbidden before accepted spend opening",
        ),
        disclosed_value(
            "packed_witness_opening.account_projection",
            "certificate.packed_witness_opening.account_projection",
            "selected account projection disclosure",
            "accepted_spend_certificate",
            "six q12289 residues",
            "spend-opening disclosure",
            "the selected account projection residues are intentionally public after a valid spend",
        ),
        disclosed_value(
            "packed_witness_opening.repacked_witness_commitment",
            "certificate.packed_witness_opening.repacked_witness_commitment",
            "public binding",
            "accepted_spend_certificate",
            "32 bytes",
            "public",
            "must equal witness_commitment after recomputing masked_projection from account_projection and projection_masks",
        ),
        disclosed_value(
            "account_witness_projection_opening_frame",
            "certificate.account_witness_projection_opening_frame",
            "selected account projection disclosure",
            "accepted_spend_certificate",
            "encoded six selected_source_values",
            "spend-opening disclosure",
            "canonical hidden-source account witness opening frame for the same six account_projection residues",
        ),
        disclosed_value(
            "pad_seed_digest",
            "certificate.pad_seed_digest and opening.pad_seed_digest",
            "pad opening",
            "pre_spend_live_slot_digest_only and accepted_spend_certificate",
            "32 bytes",
            "public digest only",
            "digest binds the pad seed; the pad_seed bytes remain forbidden except as verifier witness input",
        ),
        disclosed_value(
            "pad_opening_statement",
            "statement.pad_opening_statement",
            "pad opening",
            "accepted_spend_certificate",
            "observed block bitmap plus public statement digest",
            "spend-opening disclosure",
            "the pad opening statement is verified against pad_seed, but certificate exports only the statement digest",
        ),
        disclosed_value(
            "carrier_lanes",
            "certificate.carrier_lanes",
            "pad opening",
            "accepted_spend_certificate",
            "12 q12289 projection-carrier lanes",
            "spend-opening disclosure",
            "contains six account-side projection lanes and six pad-source projection lanes consumed by the hidden-source predicate",
        ),
        disclosed_value(
            "carrier_check_bytes",
            "certificate.carrier_check_bytes",
            "public binding",
            "accepted_spend_certificate",
            "projection check bytes",
            "public",
            "must match reviewed_fq12_state_assisted_projection_carrier_binding",
        ),
        disclosed_value(
            "carrier_transcript_binding",
            "certificate.carrier_transcript_binding",
            "public binding",
            "accepted_spend_certificate",
            "projection transcript-binding bytes",
            "public",
            "binds carrier lanes to opening commitment, statement, slot, message, public key, and checked-core body",
        ),
        disclosed_value(
            "RejectBeforeAcceptance error string",
            "masked tuple and candidate verifier error path",
            "reject behavior",
            "reject_before_acceptance",
            "public reject code/message",
            "negligible_rejection_mismatch until constant-time reject equivalence is proved",
            "must not export projection_masks, account_projection, pad_seed bytes, or opening frames on rejection",
        ),
        disclosed_value(
            "hidden-material Export result",
            "verify_hidden_source_fallback_hidden_material_frame_projection_or_export",
            "reject behavior",
            "reject_before_acceptance",
            "public rejection of break-export path",
            "named break, not accepted leakage",
            "the masked tuple converts Export into rejection and does not return an accepting certificate",
        ),
    ]

    replay_interaction = {
        "summary": (
            "The six selected projection residues become public after a valid spend, "
            "but they are not replay authorizers by themselves."
        ),
        "bindings": [
            "slot id is carried by checked-core verification, statement, packed opening, and mask PRF input",
            "opening_commitment is derived from the prechallenge account slot before tuple verification",
            "statement_digest binds public key, message, account state, signature body, and pad-opening statement",
            "carrier_check_bytes and carrier_transcript_binding bind the decoded carrier lanes to the same opening and statement",
            "accepted verification consumes the slot under spent-slot and restore-anchor lifecycle rules",
            "selected-projection spent records carry pad_seed_digest and target-chain validation rejects duplicate or mismatching digests",
        ],
        "residual_risk": (
            "public observers can remember the six residues after spend; replay safety "
            "therefore relies on transcript binding and lifecycle consumption, not on "
            "post-spend secrecy of those residues"
        ),
        "budget_conclusion": "post_spend_public_residues_do_not_expand_pre_spend_live_slot_view",
    }

    remaining_proof_and_test_obligations = [
        {
            "id": "no_pre_spend_projection_export_static_boundary",
            "status": "open",
            "required_to_close": (
                "audit public APIs so only the accepting certificate path returns "
                "account_projection, projection_masks, or account_witness_projection_opening_frame"
            ),
        },
        {
            "id": "no_pre_spend_projection_export_negative_tests",
            "status": "open",
            "required_to_close": (
                "add regressions that account-state reads, snapshots, malformed openings, "
                "wrong pad seed, carrier rebinding, hidden-material export, and spent-slot "
                "replay never return the six unmasked projection residues"
            ),
        },
        {
            "id": "same_game_spend_opening_composition",
            "status": "open",
            "required_to_close": (
                "compose this spend-time budget with the mask-PRF simulator game, "
                "lifecycle same-game lift, and pad-seed single-use invariant"
            ),
        },
        {
            "id": "constant_time_reject_equivalence",
            "status": "open",
            "required_to_close": (
                "show all rejecting paths expose one public reject behavior and no "
                "secret-dependent timing or structured leakage"
            ),
        },
    ]

    evidence = [
        {
            "artifact": rel(SIMULATOR_JSON),
            "status": simulator.get("status"),
            "role": "declares spend_time_leakage_budget as gap 3 and separates live-slot from valid-spend worlds",
        },
        {
            "artifact": rel(REUSE_JSON),
            "status": reuse.get("status"),
            "role": "documents pad-seed digest and reuse surface relevant to replay and lifecycle binding",
        },
        {
            "artifact": rel(LIFECYCLE_JSON),
            "status": lifecycle.get("closure_status"),
            "role": "policy/executable lifecycle input for slot consumption and restore-anchor binding",
        },
        {
            "artifact": "tools/parameter-search/src/production_reference.rs",
            "status": "inspected_only",
            "role": "source of accepting certificate field names and verifier path; not edited",
        },
    ]

    return {
        "schema": "Descender.pre_signature_projection_spend_time_leakage_budget.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "spend_time_leakage_budget_specified_fail_closed",
        "deployment_ready": False,
        "target": {
            "n": 32,
            "q": 12289,
            "selected_signature_bytes": 308,
            "path": "selected_308_byte_fq12_state_assisted_masked_projection_carrier",
            "packed_witness_rows": 18,
            "checked_core_rows": 12,
            "masked_projection_rows": 6,
        },
        "pre_spend_live_slot_budget": pre_spend_live_slot_budget,
        "spend_opening_budget": spend_opening_budget,
        "accepting_certificate_disclosures": accepting_certificate_disclosures,
        "replay_interaction": replay_interaction,
        "remaining_proof_and_test_obligations": remaining_proof_and_test_obligations,
        "evidence": evidence,
        "decision": {
            "gap_3_local_status": "specified_not_fully_closed",
            "can_consume_as_fail_closed_budget": True,
            "can_mark_global_deployment_ready": False,
            "reason": (
                "the artifact enumerates and charges every accepting-certificate "
                "disclosure, and it makes the six selected residues intentionally "
                "public after spend; proof and negative-test work remains to ensure "
                "no pre-spend or reject path exports them"
            ),
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Spend-Time Leakage Budget",
        "",
        "This artifact addresses gap 3 for the selected 308-byte/Fq12",
        "state-assisted masked projection-carrier path. It keeps the live-slot",
        "view separate from the accepting spend certificate and fails closed on",
        "the proof/test work still needed to rule out pre-spend exports.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- path: `{payload['target']['path']}`",
        f"- selected signature bytes: `{payload['target']['selected_signature_bytes']}`",
        f"- checked-core rows: `{payload['target']['checked_core_rows']}`",
        f"- masked projection rows: `{payload['target']['masked_projection_rows']}`",
        f"- gap 3 local status: `{payload['decision']['gap_3_local_status']}`",
        f"- can consume as fail-closed budget: `{payload['decision']['can_consume_as_fail_closed_budget']}`",
        "",
        "## Pre-Spend Live-Slot Budget",
        "",
        f"- budget status: `{payload['pre_spend_live_slot_budget']['budget_status']}`",
        f"- simulator rule: {payload['pre_spend_live_slot_budget']['simulator_rule']}",
        "",
        "Allowed public values before spend:",
        "",
    ]
    lines.extend(f"- {item}" for item in payload["pre_spend_live_slot_budget"]["allowed_public_values"])
    lines.extend(["", "Forbidden public values before spend:", ""])
    lines.extend(f"- {item}" for item in payload["pre_spend_live_slot_budget"]["forbidden_public_values"])
    spend = payload["spend_opening_budget"]
    lines.extend(
        [
            "",
            "## Spend Opening Budget",
            "",
            f"- trigger: {spend['trigger']}",
            f"- six selected projection residues intentionally public after spend: `{spend['selected_projection_residues_intentionally_public_after_spend']}`",
            f"- selected projection residue count: `{spend['selected_projection_residue_count']}`",
            f"- selected projection disclosure rule: {spend['selected_projection_disclosure_rule']}",
            f"- non-claim: {spend['not_a_claim']}",
            "",
            "## Accepting Certificate Disclosure Table",
            "",
            "| value | category | phase | charge | rule |",
            "|---|---|---|---|---|",
        ]
    )
    for item in payload["accepting_certificate_disclosures"]:
        lines.append(
            f"| `{item['name']}` | `{item['category']}` | `{item['phase']}` | `{item['charge']}` | {item['rule']} |"
        )
    replay = payload["replay_interaction"]
    lines.extend(
        [
            "",
            "## Replay Interaction",
            "",
            replay["summary"],
            "",
            "Replay bindings:",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in replay["bindings"])
    lines.extend(
        [
            "",
            f"Residual risk: {replay['residual_risk']}",
            "",
            f"Budget conclusion: `{replay['budget_conclusion']}`",
            "",
            "## Remaining Proof And Test Obligations",
            "",
            "| obligation | status | required to close |",
            "|---|---|---|",
        ]
    )
    for item in payload["remaining_proof_and_test_obligations"]:
        lines.append(f"| `{item['id']}` | `{item['status']}` | {item['required_to_close']} |")
    lines.extend(["", "## Evidence", "", "| artifact | status | role |", "|---|---|---|"])
    for item in payload["evidence"]:
        lines.append(f"| `{item['artifact']}` | `{item['status']}` | {item['role']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can mark global deployment ready: `{payload['decision']['can_mark_global_deployment_ready']}`",
            f"- reason: {payload['decision']['reason']}",
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
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "status": payload["status"],
                "deployment_ready": payload["deployment_ready"],
                "disclosed_values": len(payload["accepting_certificate_disclosures"]),
                "open_obligations": len(payload["remaining_proof_and_test_obligations"]),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
