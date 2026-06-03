#!/usr/bin/env python3
"""Investigate reframing spend-time hidden-source proof as slot authorization."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_slot_opening_authorization_reframe_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_slot_opening_authorization_reframe_n32_q12289.md"
)

SOURCE_ARTIFACTS = [
    READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.json",
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json",
    READINESS_DIR
    / "hidden_source_state_assisted_masked_projection_global_soundness_n32_q12289.json",
    READINESS_DIR / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json",
    READINESS_DIR / "pre_signature_projection_lifecycle_db_profile_binding_n32_q12289.json",
    READINESS_DIR / "pre_signature_projection_postchallenge_slot_creation_gate_n32_q12289.json",
    READINESS_DIR / "state_lifecycle_closure_n32_q12289.json",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"missing": True, "path": rel(path)}
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    sources = {rel(path): load_json(path) for path in SOURCE_ARTIFACTS}

    moved_to_account_creation = [
        {
            "item": "hidden_source_relation_validation",
            "account_creation_obligation": (
                "prove or verify that the slot's packed masked-state18 witness "
                "opening, pad source, public pad-opening statement, and account "
                "commitments satisfy the canonical hidden-source relation before "
                "the slot becomes live"
            ),
            "spend_time_residual": (
                "verify an authorization handle for that prevalidated relation, "
                "not re-extract the private witness/pad opening"
            ),
        },
        {
            "item": "slot_prevalidation_certificate",
            "account_creation_obligation": (
                "store a digest or state-root inclusion commitment to the reviewed "
                "creation-time validation transcript, including verifier version, "
                "public-key identity, slot id, opening commitment, pad-seed digest, "
                "and relation statement digest"
            ),
            "spend_time_residual": (
                "check inclusion/freshness of the prevalidation record and consume "
                "the exact slot named by the signature"
            ),
        },
        {
            "item": "proof_of_knowledge_or_named_break",
            "account_creation_obligation": (
                "move compact proof soundness, witness extraction, and hidden-source "
                "proof failure events into the account creation game"
            ),
            "spend_time_residual": (
                "prove authorization-to-consume or export lifecycle, registry, "
                "binding, checked-core, or QROM break events"
            ),
        },
        {
            "item": "leakage_budget_for_live_slots",
            "account_creation_obligation": (
                "charge all public slot metadata and prevalidation transcript "
                "leakage when the account publishes live slots, including slots "
                "that are never spent"
            ),
            "spend_time_residual": (
                "charge only message/signature-core leakage and public lifecycle "
                "state observed at spend time"
            ),
        },
        {
            "item": "lifecycle_and_storage_binding",
            "account_creation_obligation": (
                "make slot creation atomic with canonical account state, duplicate "
                "pad-seed rejection, restore-anchor update, verifier-version binding, "
                "and durable data availability"
            ),
            "spend_time_residual": (
                "enforce unused-to-consumed transition, spent/burn ledger insertion, "
                "rollback rejection, and prevalidation record availability"
            ),
        },
    ]

    assumptions_introduced = [
        {
            "id": "creation_time_hidden_source_soundness",
            "meaning": (
                "the account-creation verifier has the full hidden-source soundness "
                "property currently demanded from spend-time acceptance"
            ),
            "new_or_moved": "moved_from_spend_time",
        },
        {
            "id": "prevalidation_registry_binding",
            "meaning": (
                "a live slot cannot point to a forged, stale, wrong-version, "
                "wrong-key, or wrong-slot prevalidation certificate"
            ),
            "new_or_moved": "new_state_assumption",
        },
        {
            "id": "slot_authorization_unforgeability",
            "meaning": (
                "an adversary cannot obtain spend-time acceptance for a slot unless "
                "that slot was previously accepted by account creation and remains "
                "unused under the canonical state root"
            ),
            "new_or_moved": "new_theorem_target",
        },
        {
            "id": "message_independent_capability_suffices",
            "meaning": (
                "the creation-time validated slot is a one-use signing capability "
                "for future messages; spend-time checked-core verification must "
                "still bind the actual message, public key, slot id, challenge, and "
                "response equation"
            ),
            "new_or_moved": "new_model_assumption",
        },
        {
            "id": "creation_time_no_postchallenge_programming",
            "meaning": (
                "slot creation and prevalidation happen before any spend challenge "
                "for that slot and cannot be adaptively rewritten after seeing the "
                "message or checked-core challenge"
            ),
            "new_or_moved": "strengthened_lifecycle_assumption",
        },
        {
            "id": "prevalidation_data_availability",
            "meaning": (
                "verifiers can check the prevalidation record or a reviewed succinct "
                "commitment to it at spend time; missing records fail closed"
            ),
            "new_or_moved": "new_deployment_assumption",
        },
    ]

    theorem = {
        "name": "prevalidated_slot_opening_authorization_or_break",
        "informal_statement": (
            "For n=32, q=12289, if the spend-time verifier accepts a checked-core "
            "signature and consumes a state slot, then either the slot was created "
            "by the account-creation verifier with a valid hidden-source relation "
            "certificate bound to the same public-key identity, slot id, opening "
            "commitment, pad-seed digest, verifier version, and account state root, "
            "or the adversary exports a named break. Spend-time verification proves "
            "authorization to consume that prevalidated slot plus checked-core "
            "message binding; it no longer claims to extract the hidden-source "
            "witness from the spend transcript itself."
        ),
        "spend_time_statement": [
            "slot is live, unused, and bound to a valid prevalidation record",
            "signature names the slot and public-key identity in the checked-core challenge frame",
            "checked-core active product-block/response equation accepts for the message",
            "state transition consumes or burns the slot atomically",
        ],
        "creation_time_statement": [
            "hidden-source relation is proved or verified with full witness/pad extraction semantics",
            "slot metadata and leakage are committed into account state before spend challenge derivation",
            "slot authorization certificate is bound to verifier version and public-key identity",
        ],
        "non_claim": (
            "This is not a way to skip hidden-source soundness. It moves that "
            "proof to account creation and changes the signature model to a "
            "stateful prevalidated-slot capability."
        ),
    }

    break_events = [
        {
            "id": "creation_time_hidden_source_soundness_break",
            "evidence": "account creation accepts a slot whose hidden-source relation lacks a valid witness/pad opening",
        },
        {
            "id": "prevalidation_registry_binding_break",
            "evidence": "spend acceptance uses a prevalidation record not bound to the consumed slot, public key, verifier version, or state root",
        },
        {
            "id": "postchallenge_slot_creation_or_rewrite_break",
            "evidence": "slot metadata or prevalidation certificate is created or mutated after seeing the spend challenge/message",
        },
        {
            "id": "slot_replay_or_rollback_break",
            "evidence": "the same prevalidated slot authorizes more than one accepted spend or is restored after burn/consume",
        },
        {
            "id": "checked_core_message_binding_break",
            "evidence": "accepted spend fails the checked-core equation or rebinding to message/public key/slot challenge frame",
        },
        {
            "id": "prevalidation_data_availability_break",
            "evidence": "spend verifier accepts without access to a valid prevalidation record or reviewed succinct accumulator proof",
        },
        {
            "id": "live_slot_leakage_or_keygen_break",
            "evidence": "published prevalidated slot metadata leaks beyond the creation-time leakage budget or invalidates keygen assumptions",
        },
        {
            "id": "qrom_domain_separation_break",
            "evidence": "creation-time and spend-time random-oracle domains collide, double count, or allow adaptive programming across phases",
        },
    ]

    byte_state_impact = {
        "spend_bytes": {
            "current_selected_envelope_bytes": 308,
            "checked_core_signature_bytes": 252,
            "projection_carrier_bytes_removed_if_authorization_handle_is_in_state": 56,
            "estimated_spend_envelope_after_reframe": 252,
            "note": (
                "If the verifier still needs a Merkle/accumulator authorization "
                "proof in the transaction, add that proof size back. A plain "
                "state lookup can save the 56-byte projection carrier; avoiding "
                "a roughly 100-byte hidden-source proof is only credible when the "
                "chain state already carries the authorization certificate."
            ),
        },
        "account_creation_or_state_bytes": [
            "hidden-source proof/certificate transcript moves to account creation or account-state sidecar",
            "prevalidation record must bind slot id, public-key identity, verifier version, opening commitment, pad-seed digest, statement digest, and lifecycle anchor",
            "live slot state may grow by a 32-byte prevalidation digest plus any accumulator path/state-root data not already present",
            "public leakage is paid for every live prevalidated slot, including never-spent slots",
        ],
        "security_impact": (
            "Unforgeability is not weakened if account creation has the same hidden-source "
            "soundness target, slot authorization is immutable and identity-bound, and "
            "checked-core message binding remains at spend time. It is weakened if the "
            "prevalidation record is treated as a bare hash with no creation-time proof, "
            "if slots can be created after challenge/message observation, or if one "
            "prevalidated capability can authorize multiple spends."
        ),
    }

    feasibility = {
        "status": "conditionally_feasible_reframe",
        "can_remove_spend_time_hidden_witness_extraction": True,
        "can_remove_hidden_source_soundness_from_protocol": False,
        "main_blocker": (
            "the current artifacts do not yet define a creation-time verifier and "
            "registry theorem that is strong enough to replace spend-time extraction"
        ),
        "recommended_next_artifact": (
            "account_creation_prevalidated_slot_authorization_theorem"
        ),
    }

    return {
        "schema": "Descender.hidden_source_slot_opening_authorization_reframe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "preliminary_investigation",
        "deployment_ready": False,
        "source_artifacts": {
            path: {
                "schema": data.get("schema"),
                "status": data.get("status") or data.get("decision", {}).get("status"),
                "missing": data.get("missing", False),
            }
            for path, data in sources.items()
        },
        "theorem": theorem,
        "moved_to_account_creation": moved_to_account_creation,
        "assumptions_introduced": assumptions_introduced,
        "break_events": break_events,
        "byte_state_impact": byte_state_impact,
        "feasibility": feasibility,
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    theorem = payload["theorem"]
    impact = payload["byte_state_impact"]
    spend = impact["spend_bytes"]
    feasibility = payload["feasibility"]
    lines = [
        "# Hidden-Source Slot-Opening Authorization Reframe",
        "",
        "This preliminary artifact investigates replacing spend-time hidden-source",
        "witness extraction with authorization to consume a prevalidated account slot.",
        "It does not mark the deployment verifier ready.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- feasibility: `{feasibility['status']}`",
        f"- can remove spend-time hidden witness extraction: `{feasibility['can_remove_spend_time_hidden_witness_extraction']}`",
        f"- can remove hidden-source soundness from protocol: `{feasibility['can_remove_hidden_source_soundness_from_protocol']}`",
        f"- main blocker: {feasibility['main_blocker']}",
        f"- recommended next artifact: `{feasibility['recommended_next_artifact']}`",
        "",
        "## Proposed Theorem",
        "",
        f"- name: `{theorem['name']}`",
        f"- statement: {theorem['informal_statement']}",
        f"- non-claim: {theorem['non_claim']}",
        "",
        "### Spend-Time Statement",
        "",
    ]
    lines.extend(f"- {item}" for item in theorem["spend_time_statement"])
    lines.extend(["", "### Creation-Time Statement", ""])
    lines.extend(f"- {item}" for item in theorem["creation_time_statement"])

    lines.extend(
        [
            "",
            "## What Moves To Account Creation",
            "",
            "| item | creation-time obligation | spend-time residual |",
            "|---|---|---|",
        ]
    )
    for row in payload["moved_to_account_creation"]:
        lines.append(
            f"| `{row['item']}` | {row['account_creation_obligation']} | "
            f"{row['spend_time_residual']} |"
        )

    lines.extend(["", "## New Or Moved Assumptions", "", "| assumption | class | meaning |", "|---|---|---|"])
    for row in payload["assumptions_introduced"]:
        lines.append(f"| `{row['id']}` | `{row['new_or_moved']}` | {row['meaning']} |")

    lines.extend(["", "## Break Events", "", "| break | evidence |", "|---|---|"])
    for row in payload["break_events"]:
        lines.append(f"| `{row['id']}` | {row['evidence']} |")

    lines.extend(
        [
            "",
            "## Byte And State Impact",
            "",
            f"- current selected envelope bytes: `{spend['current_selected_envelope_bytes']}`",
            f"- checked-core signature bytes: `{spend['checked_core_signature_bytes']}`",
            f"- projection carrier bytes removable from spend path: `{spend['projection_carrier_bytes_removed_if_authorization_handle_is_in_state']}`",
            f"- estimated spend envelope after reframe: `{spend['estimated_spend_envelope_after_reframe']}`",
            f"- note: {spend['note']}",
            "",
            "### Account Creation / State Cost",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in impact["account_creation_or_state_bytes"])
    lines.extend(["", "### Security Impact", "", impact["security_impact"], ""])

    lines.extend(["## Source Artifacts", "", "| artifact | schema | status | missing |", "|---|---|---|:---:|"])
    for path, row in payload["source_artifacts"].items():
        lines.append(
            f"| `{path}` | `{row.get('schema')}` | `{row.get('status')}` | `{row.get('missing')}` |"
        )
    lines.extend(
        [
            "",
            "## Recommendation",
            "",
            "Proceed only as a theorem reframe, not as a proof deletion. The next step is",
            "to specify account creation as the place where full hidden-source relation",
            "soundness is checked, then make spend-time acceptance prove only",
            "prevalidated-slot authorization plus checked-core message binding.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": rel(DEFAULT_JSON),
                "markdown": rel(DEFAULT_MD),
                "status": payload["feasibility"]["status"],
                "estimated_spend_envelope_after_reframe": payload["byte_state_impact"][
                    "spend_bytes"
                ]["estimated_spend_envelope_after_reframe"],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
