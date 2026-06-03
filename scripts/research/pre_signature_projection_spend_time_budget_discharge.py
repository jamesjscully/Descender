#!/usr/bin/env python3
"""Discharge audit for the masked projection spend-time budget carried term."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_spend_time_budget_discharge_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_spend_time_budget_discharge_n32_q12289.md"
)
SPEND_BUDGET_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_leakage_budget_n32_q12289.json"
)
THEOREM_JSON = (
    READINESS_DIR
    / "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
)
AUDIT_MD = ROOT / "research/pre_signature_projection_leakage_audit.md"
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def classify_disclosure(item: dict[str, str]) -> dict[str, Any]:
    name = item["name"]
    charge = item["charge"]
    phase = item["phase"]

    if name in {
        "packed_witness_opening.projection_masks",
        "packed_witness_opening.account_projection",
        "account_witness_projection_opening_frame",
    }:
        return {
            **item,
            "discharge_class": "charged",
            "discharge_status": "charged_after_valid_spend_only",
            "reason": (
                "selected projection material is intentionally public only in "
                "the accepting certificate; the term is charged to "
                "Adv_spend_opening_leakage(q_open)"
            ),
        }
    if name in {"carrier_lanes", "pad_opening_statement", "checked_core_response_syndrome"}:
        return {
            **item,
            "discharge_class": "charged",
            "discharge_status": "charged_after_valid_spend",
            "reason": "accepted-signature/opening data is public only in the spend transcript",
        }
    if name in {"RejectBeforeAcceptance error string", "hidden-material Export result"}:
        return {
            **item,
            "discharge_class": "residual",
            "discharge_status": "residual_reject_equivalence_and_no_export_tests",
            "reason": (
                "reject paths are fail-closed to errors in the reviewed code, "
                "but still need constant-shape, no-field-export negative tests"
            ),
        }
    if "pre_spend" in phase and name == "packed_witness_opening.masked_projection":
        return {
            **item,
            "discharge_class": "discharged",
            "discharge_status": "discharged_as_masked_live_slot_leakage",
            "reason": (
                "pre-spend account state carries only masked projection rows, "
                "not unmasked selected projection residues or masks"
            ),
        }
    if name == "pad_seed_digest":
        return {
            **item,
            "discharge_class": "discharged",
            "discharge_status": "discharged_as_digest_only",
            "reason": "the digest is public binding material; pad_seed bytes remain forbidden",
        }
    if charge == "public" or charge.startswith("already charged"):
        return {
            **item,
            "discharge_class": "discharged",
            "discharge_status": "discharged_as_public_or_precharged_binding",
            "reason": "the value is public binding material or already in the checked-core budget",
        }
    return {
        **item,
        "discharge_class": "residual",
        "discharge_status": "residual_unclassified_budget_item",
        "reason": "the budget item was not matched by a local discharge rule",
    }


def build_payload() -> dict[str, Any]:
    spend_budget = load_json(SPEND_BUDGET_JSON)
    theorem = load_json(THEOREM_JSON)
    disclosures = [
        classify_disclosure(item)
        for item in spend_budget["accepting_certificate_disclosures"]
    ]

    static_boundary_evidence = [
        {
            "id": "account_state_record_surface",
            "status": "discharged",
            "evidence": "ProductionReviewedFq12StateAssistedAccountSlotRecord fields are slot_id, witness_commitment, randomizer_commitment, hidden_source_opening_commitment, restore_anchor_digest, pad_seed_digest, unused",
            "conclusion": (
                "static account-state records do not export account_projection, "
                "projection_masks, pad_seed bytes, or account witness opening frames"
            ),
        },
        {
            "id": "accepting_certificate_surface",
            "status": "charged",
            "evidence": (
                "ProductionReviewedFq12MaskedProjectionSoundnessCertificate contains "
                "packed_witness_opening, account_witness_projection_opening_frame, "
                "pad_seed_digest, carrier_lanes, check bytes, and transcript binding"
            ),
            "conclusion": (
                "selected projection residues, projection masks, and the derived "
                "opening frame are charged as intentional accepting-certificate disclosure"
            ),
        },
        {
            "id": "candidate_verifier_surface",
            "status": "discharged_with_test_gap",
            "evidence": (
                "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier "
                "returns ProductionReviewedFq12AccountCarriedSignatureParts, not a "
                "packed opening or projection frame"
            ),
            "conclusion": (
                "candidate acceptance does not itself return selected projection material, "
                "but regressions must pin malformed and rejection cases"
            ),
        },
        {
            "id": "helper_export_surface",
            "status": "residual",
            "evidence": (
                "public helpers reviewed_fq12_masked_state18_packed_witness_opening, "
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_opening, "
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening, "
                "and verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple "
                "can return account_projection, projection_masks, or an account witness opening frame "
                "when supplied pad_seed and matching opening material"
            ),
            "conclusion": (
                "these helpers must be internal/proof-only or explicitly documented as "
                "post-opening APIs; they are not sufficient evidence that no pre-spend export API exists"
            ),
        },
        {
            "id": "pad_seed_surface",
            "status": "residual",
            "evidence": (
                "pad_seed is accepted as verifier witness input by the masked projection "
                "candidate/certificate helpers; account-state records carry only pad_seed_digest"
            ),
            "conclusion": (
                "the spend-time term remains fail-closed until public APIs and snapshots "
                "are tested to exclude pad_seed byte export before valid spend"
            ),
        },
        {
            "id": "reject_export_surface",
            "status": "residual",
            "evidence": (
                "hidden-material Export is converted to Err in the masked carrier tuple path, "
                "and reject-before-acceptance returns error strings"
            ),
            "conclusion": (
                "constant-shape reject behavior and no structured secret-bearing error payloads "
                "remain negative-test obligations"
            ),
        },
    ]

    api_restrictions = [
        {
            "id": "certificate_only_projection_exports",
            "restriction": (
                "Only verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate "
                "may be exposed as the external API that returns account_projection, projection_masks, "
                "or account_witness_projection_opening_frame."
            ),
        },
        {
            "id": "internalize_opening_helpers",
            "restriction": (
                "Keep reviewed_fq12_masked_state18_packed_witness_opening and the masked "
                "projection-opening helpers crate-private, test-only, or behind a proof/audit "
                "feature; do not expose them to account-state read, snapshot, signing, or "
                "verification APIs."
            ),
        },
        {
            "id": "digest_only_pad_seed_public_api",
            "restriction": (
                "Public account-state, snapshot, restore, ledger, and error APIs may expose "
                "pad_seed_digest only; pad_seed bytes are verifier witness input and must not "
                "be serialized or returned before a valid spend-opening path."
            ),
        },
        {
            "id": "opaque_reject_errors",
            "restriction": (
                "Reject-before-acceptance APIs must return only slot-scoped public errors, "
                "never partial packed_witness_opening, projection masks, account projection "
                "residues, pad_seed bytes, carrier decoded lanes beyond public signature bytes, "
                "or opening frames."
            ),
        },
        {
            "id": "no_debug_or_serde_leak",
            "restriction": (
                "Do not derive Serialize/Deserialize or expose debug snapshot endpoints for "
                "ProductionReviewedFq12MaskedState18PackedWitnessOpening or accepting "
                "certificate internals unless the endpoint is explicitly post-acceptance."
            ),
        },
    ]

    negative_tests = [
        {
            "id": "account_state_reads_digest_only",
            "required_assertion": (
                "checked slot records and state-assisted slot records contain commitments, "
                "restore anchor, pad_seed_digest, and unused status, but no account_projection, "
                "projection_masks, pad_seed bytes, or account_witness_projection_opening_frame"
            ),
        },
        {
            "id": "snapshot_restore_no_secret_exports",
            "required_assertion": (
                "snapshot, restore, reorg, target-chain validation, and spent-ledger records "
                "round-trip pad_seed_digest only and never serialize pad_seed bytes or selected "
                "projection openings for unused/live slots"
            ),
        },
        {
            "id": "malformed_opening_returns_opaque_error",
            "required_assertion": (
                "slot id mismatch, statement digest mismatch, witness commitment mismatch, "
                "opening digest mismatch, and repack mismatch return errors with no certificate "
                "and no partial packed_witness_opening"
            ),
        },
        {
            "id": "wrong_pad_seed_no_partial_opening",
            "required_assertion": (
                "wrong pad_seed rejects before acceptance and the returned value contains no "
                "projection_masks, account_projection, pad_seed bytes, or opening frame"
            ),
        },
        {
            "id": "carrier_rebinding_no_export",
            "required_assertion": (
                "rebinding carrier lanes, check bytes, transcript binding, message, public key, "
                "signature body, opening commitment, or slot id rejects without returning "
                "selected projection material"
            ),
        },
        {
            "id": "hidden_material_export_is_error_only",
            "required_assertion": (
                "the hidden-material Export branch in the masked tuple path is converted to an "
                "opaque error and cannot be observed as an exported frame or witness payload"
            ),
        },
        {
            "id": "spent_slot_replay_no_opening",
            "required_assertion": (
                "after accepted spend and ledger record, replay under the same slot or "
                "pad_seed_digest rejects and returns no fresh opening frame or unmasked residues"
            ),
        },
        {
            "id": "helper_visibility_guard",
            "required_assertion": (
                "external API tests cannot call helper-level opening materializers from "
                "pre-spend account-state, signing, verification, snapshot, or diagnostics surfaces"
            ),
        },
    ]

    residual_blockers = [
        {
            "id": "helper_export_surface",
            "blocks": "full proof of A_spend_time_projection_budget",
            "required_to_close": (
                "restrict or gate helper APIs that materialize account_projection, "
                "projection_masks, and opening frames from pad_seed plus commitment material"
            ),
        },
        {
            "id": "negative_test_suite_missing",
            "blocks": "full proof of no pre-spend projection export",
            "required_to_close": "implement the exact negative tests listed in this artifact",
        },
        {
            "id": "reject_equivalence_unproved",
            "blocks": "reject-path discharge",
            "required_to_close": (
                "prove or test one public reject shape with no secret-dependent timing or "
                "structured leakage on every malformed path"
            ),
        },
        {
            "id": "composition_not_promoted",
            "blocks": "global theorem deployment readiness",
            "required_to_close": (
                "compose the local fail-closed discharge with mask-PRF simulator, lifecycle "
                "same-game, pad-seed uniqueness, and reject-equivalence artifacts"
            ),
        },
    ]

    counts = {
        "discharged": sum(1 for item in disclosures if item["discharge_class"] == "discharged"),
        "charged": sum(1 for item in disclosures if item["discharge_class"] == "charged"),
        "residual": sum(1 for item in disclosures if item["discharge_class"] == "residual"),
    }

    return {
        "schema": "Descender.pre_signature_projection_spend_time_budget_discharge.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "A_spend_time_projection_budget_fail_closed_not_proved",
        "deployment_ready": False,
        "target": spend_budget["target"],
        "carried_term": {
            "id": "A_spend_time_projection_budget",
            "source": rel(SPEND_BUDGET_JSON),
            "term": "Adv_spend_opening_leakage(q_open)",
            "composition_source_status": theorem.get("status"),
            "local_discharge": "fail_closed",
            "proved": False,
            "reason": (
                "accepted-certificate disclosures are enumerated and charged, and "
                "account-state records are digest/commitment-only, but public helper "
                "surfaces and missing negative tests prevent a full no-pre-spend-export proof"
            ),
        },
        "pre_spend_forbidden_values": spend_budget["pre_spend_live_slot_budget"][
            "forbidden_public_values"
        ],
        "disclosure_counts": counts,
        "disclosure_classification": disclosures,
        "static_boundary_evidence": static_boundary_evidence,
        "api_restrictions_required": api_restrictions,
        "negative_tests_required": negative_tests,
        "residual_blockers": residual_blockers,
        "decision": {
            "can_discharge_as_fail_closed_carried_term": True,
            "can_mark_term_proved": False,
            "can_mark_global_deployment_ready": False,
            "summary": (
                "The carried term can be consumed only as a fail-closed budget: "
                "selected projection residues, masks, and opening frames are charged "
                "after valid spend, while helper/API visibility, pad_seed byte export, "
                "and rejection behavior remain residual blockers."
            ),
        },
        "evidence": [
            {
                "artifact": rel(SPEND_BUDGET_JSON),
                "status": spend_budget.get("status"),
                "role": "source spend-time disclosure budget and remaining obligations",
            },
            {
                "artifact": rel(THEOREM_JSON),
                "status": theorem.get("status"),
                "role": "composition artifact carrying A_spend_time_projection_budget as term 4",
            },
            {
                "artifact": rel(AUDIT_MD),
                "status": "inspected",
                "role": "leakage timing audit for raw vs masked 18-residue state",
            },
            {
                "artifact": rel(PRODUCTION_REFERENCE),
                "status": "inspected_only",
                "role": "static API and field-name evidence; not edited",
            },
        ],
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    carried = payload["carried_term"]
    decision = payload["decision"]
    lines = [
        "# Pre-Signature Projection Spend-Time Budget Discharge",
        "",
        "This artifact audits carried term 4, `A_spend_time_projection_budget`,",
        "for the selected 308-byte/Fq12 state-assisted masked projection-carrier",
        "path. It classifies every spend-time disclosure as discharged, charged,",
        "or residual and fails closed on the API/test gaps.",
        "",
        "## Status",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- carried term: `{carried['id']}`",
        f"- term: `{carried['term']}`",
        f"- local discharge: `{carried['local_discharge']}`",
        f"- proved: `{carried['proved']}`",
        f"- can discharge as fail-closed carried term: `{decision['can_discharge_as_fail_closed_carried_term']}`",
        f"- can mark term proved: `{decision['can_mark_term_proved']}`",
        f"- summary: {decision['summary']}",
        "",
        "## Disclosure Classification",
        "",
        "| value | phase | charge | class | status | reason |",
        "|---|---|---|---|---|---|",
    ]
    for item in payload["disclosure_classification"]:
        lines.append(
            f"| `{item['name']}` | `{item['phase']}` | `{item['charge']}` | "
            f"`{item['discharge_class']}` | `{item['discharge_status']}` | {item['reason']} |"
        )

    lines.extend(
        [
            "",
            "Counts:",
            "",
            f"- discharged: `{payload['disclosure_counts']['discharged']}`",
            f"- charged: `{payload['disclosure_counts']['charged']}`",
            f"- residual: `{payload['disclosure_counts']['residual']}`",
            "",
            "## Static Boundary Evidence",
            "",
            "| boundary | status | conclusion | evidence |",
            "|---|---|---|---|",
        ]
    )
    for item in payload["static_boundary_evidence"]:
        lines.append(
            f"| `{item['id']}` | `{item['status']}` | {item['conclusion']} | {item['evidence']} |"
        )

    lines.extend(
        [
            "",
            "## Required API Restrictions",
            "",
        ]
    )
    for item in payload["api_restrictions_required"]:
        lines.append(f"- `{item['id']}`: {item['restriction']}")

    lines.extend(
        [
            "",
            "## Required Negative Tests",
            "",
        ]
    )
    for item in payload["negative_tests_required"]:
        lines.append(f"- `{item['id']}`: {item['required_assertion']}")

    lines.extend(
        [
            "",
            "## Residual Blockers",
            "",
            "| blocker | blocks | required to close |",
            "|---|---|---|",
        ]
    )
    for item in payload["residual_blockers"]:
        lines.append(
            f"| `{item['id']}` | {item['blocks']} | {item['required_to_close']} |"
        )

    lines.extend(
        [
            "",
            "## Evidence",
            "",
            "| artifact | status | role |",
            "|---|---|---|",
        ]
    )
    for item in payload["evidence"]:
        lines.append(f"| `{item['artifact']}` | `{item['status']}` | {item['role']} |")
    lines.append("")
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
                "proved": payload["carried_term"]["proved"],
                "fail_closed": payload["decision"][
                    "can_discharge_as_fail_closed_carried_term"
                ],
                "disclosure_counts": payload["disclosure_counts"],
                "residual_blockers": len(payload["residual_blockers"]),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
