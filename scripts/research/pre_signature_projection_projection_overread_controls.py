#!/usr/bin/env python3
"""Audit controls for selected-projection overread before accepted spend."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_projection_overread_controls_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_projection_overread_controls_n32_q12289.md"
)
SPEND_BUDGET_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_leakage_budget_n32_q12289.json"
)
SPEND_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_budget_discharge_n32_q12289.json"
)
LIFECYCLE_DISCHARGE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json"
)
LEAKAGE_AUDIT_MD = ROOT / "research/pre_signature_projection_leakage_audit.md"
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def contains(path: Path, needle: str) -> bool:
    return needle in path.read_text()


def code_presence(id_: str, needle: str, role: str) -> dict[str, Any]:
    return {
        "id": id_,
        "path": rel(PRODUCTION_REFERENCE),
        "needle": needle,
        "present": contains(PRODUCTION_REFERENCE, needle),
        "role": role,
    }


def export_surface(
    id_: str,
    symbol: str,
    materializes: list[str],
    path_class: str,
    status: str,
    control: str,
    needle: str,
    negative_test_refs: list[str],
) -> dict[str, Any]:
    return {
        "id": id_,
        "symbol": symbol,
        "materializes": materializes,
        "path_class": path_class,
        "status": status,
        "control": control,
        "negative_tests": negative_test_refs,
        "evidence": code_presence(id_, needle, control),
    }


def build_payload() -> dict[str, Any]:
    spend_budget = load_json(SPEND_BUDGET_JSON)
    spend_discharge = load_json(SPEND_DISCHARGE_JSON)
    lifecycle_discharge = load_json(LIFECYCLE_DISCHARGE_JSON)

    forbidden = spend_budget["pre_spend_live_slot_budget"]["forbidden_public_values"]
    allowed = spend_budget["pre_spend_live_slot_budget"]["allowed_public_values"]

    negative_tests = [
        {
            "id": "account_state_read_no_selected_material",
            "phase": "pre_spend_account_state_read",
            "surface": [
                "checked_slot_records",
                "reviewed_fq12_state_assisted_account_state_from_slot_records",
                "ProductionReviewedFq12StateAssistedAccountSlotRecord",
            ],
            "assertion": (
                "account-state reads and reconstructed slot records expose only slot id, "
                "commitments, restore anchor, pad_seed_digest, and unused status; they "
                "must not expose account_projection, projection_masks, pad_seed bytes, "
                "or account_witness_projection_opening_frame"
            ),
            "expected_result": "no forbidden field is serializable or returned before spend",
        },
        {
            "id": "snapshot_restore_no_selected_material",
            "phase": "pre_spend_snapshot_restore_reorg",
            "surface": [
                "ProductionReviewedFq12StateAssistedLifecycleSnapshot",
                "ProductionSpentSlotRecord",
                "ProductionSelectedProjectionConsensusSpendRecord",
            ],
            "assertion": (
                "snapshot, restore, reorg, target-chain validation, and spent-ledger "
                "records serialize pad_seed_digest only and never serialize live-slot "
                "pad_seed bytes, projection masks, unmasked selected residues, or "
                "opening frames"
            ),
            "expected_result": "round trips preserve digest-only public state",
        },
        {
            "id": "wrong_pad_seed_reject_no_partial_opening",
            "phase": "reject_before_acceptance",
            "surface": [
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening",
                "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            ],
            "assertion": (
                "wrong pad_seed or pad_seed_digest mismatch rejects with a public error "
                "and returns no ProductionReviewedFq12MaskedState18PackedWitnessOpening, "
                "account_witness_projection_opening_frame, projection_masks, "
                "account_projection, or pad_seed bytes"
            ),
            "expected_result": "Err only; no partially materialized opening is observable",
        },
        {
            "id": "malformed_opening_reject_no_material",
            "phase": "reject_before_acceptance",
            "surface": [
                "slot id mismatch",
                "statement digest mismatch",
                "witness commitment mismatch",
                "opening digest mismatch",
                "repack mismatch",
            ],
            "assertion": (
                "all malformed opening paths reject before acceptance with one opaque "
                "public error shape and no selected projection residues, masks, pad_seed "
                "bytes, or opening frames in the returned value"
            ),
            "expected_result": "Err only across every mismatch variant",
        },
        {
            "id": "carrier_rebinding_reject_no_material",
            "phase": "reject_before_acceptance",
            "surface": [
                "carrier lane mutation",
                "carrier check byte mutation",
                "transcript binding mutation",
                "message/public-key/signature/opening/slot rebinding",
            ],
            "assertion": (
                "rebinding any carrier or transcript input rejects without returning "
                "account_witness_projection_opening_frame, hidden-material Export data, "
                "projection_masks, account_projection, or pad_seed bytes"
            ),
            "expected_result": "Err only; no Export payload is externally visible",
        },
        {
            "id": "hidden_material_export_converted_to_error",
            "phase": "reject_before_acceptance",
            "surface": [
                "verify_hidden_source_fallback_hidden_material_frame_projection_or_export",
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
            ],
            "assertion": (
                "the hidden-material Export branch is converted to an opaque error at "
                "the selected masked carrier tuple boundary and cannot be observed as "
                "a structured break export carrying opening frames or pad material"
            ),
            "expected_result": "selected external verifier returns Err, never Export",
        },
        {
            "id": "spent_slot_replay_no_new_opening",
            "phase": "post_spend_replay_reject",
            "surface": [
                "spent slot ledger",
                "pad_seed_digest duplicate check",
                "apply_selected_projection_consensus_opening_frame",
            ],
            "assertion": (
                "after accepted spend consumes the slot and records pad_seed_digest, "
                "same-slot replay or duplicate digest replay rejects and returns no "
                "fresh packed opening, projection masks, account projection, pad_seed, "
                "or opening frame"
            ),
            "expected_result": "ledger rejection is public and digest-only",
        },
        {
            "id": "helper_visibility_guard_no_pre_spend_callers",
            "phase": "api_boundary",
            "surface": [
                "reviewed_fq12_masked_state18_packed_witness_opening",
                "reviewed_fq12_packed_witness_state18_projection_masks",
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_opening",
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
            ],
            "assertion": (
                "helper-level materializers remain crate-private, proof-only, test-only, "
                "or reachable externally only through the accepting certificate path; "
                "account-state, snapshot, signing, verification, and diagnostics APIs "
                "cannot call them to export selected material before acceptance"
            ),
            "expected_result": "public API surface has one selected-material export path",
        },
    ]

    surfaces = [
        export_surface(
            "account_state_slot_record",
            "ProductionReviewedFq12StateAssistedAccountSlotRecord",
            ["witness_commitment", "randomizer_commitment", "hidden_source_opening_commitment", "restore_anchor_digest", "pad_seed_digest"],
            "allowed_pre_spend_public_state",
            "allowed_digest_and_commitment_only",
            "pre-spend state record has no selected residues, masks, pad_seed bytes, or opening frame fields",
            "pub struct ProductionReviewedFq12StateAssistedAccountSlotRecord",
            ["account_state_read_no_selected_material"],
        ),
        export_surface(
            "lifecycle_snapshot",
            "ProductionReviewedFq12StateAssistedLifecycleSnapshot",
            ["account_slots", "active_reservations", "snapshot_sequence"],
            "allowed_pre_spend_public_state",
            "allowed_digest_and_commitment_only",
            "snapshot serializes state-assisted slot records, not selected opening internals",
            "pub struct ProductionReviewedFq12StateAssistedLifecycleSnapshot",
            ["snapshot_restore_no_selected_material"],
        ),
        export_surface(
            "spent_ledger_records",
            "ProductionSpentSlotRecord and ProductionSelectedProjectionConsensusSpendRecord",
            ["slot_id", "snapshot_sequence", "hidden_source_opening_commitment", "pad_seed_digest"],
            "allowed_post_spend_lifecycle_public_state",
            "allowed_digest_only",
            "spent records persist pad_seed_digest for replay prevention but not pad_seed or selected opening internals",
            "pub struct ProductionSelectedProjectionConsensusSpendRecord",
            ["snapshot_restore_no_selected_material", "spent_slot_replay_no_new_opening"],
        ),
        export_surface(
            "packed_projection_from_commitment_helper",
            "reviewed_fq12_packed_witness_state18_projection_opening",
            ["selected account projection residues"],
            "forbidden_pre_spend_helper_surface",
            "must_not_be_public_pre_spend_api",
            "raw selected residues from a packed witness commitment are not a public pre-spend interface",
            "pub fn reviewed_fq12_packed_witness_state18_projection_opening",
            ["helper_visibility_guard_no_pre_spend_callers"],
        ),
        export_surface(
            "projection_masks_helper",
            "reviewed_fq12_packed_witness_state18_projection_masks",
            ["projection_masks"],
            "forbidden_pre_spend_helper_surface",
            "must_not_be_public_pre_spend_api",
            "mask residues are derived from secret pad_seed and slot_id and are forbidden before accepted spend",
            "pub fn reviewed_fq12_packed_witness_state18_projection_masks",
            ["helper_visibility_guard_no_pre_spend_callers", "wrong_pad_seed_reject_no_partial_opening"],
        ),
        export_surface(
            "mask_prf_input_frame_helper",
            "reviewed_fq12_packed_witness_state18_projection_mask_prf_input_frame",
            ["pad_seed-containing mask PRF input frame"],
            "forbidden_pre_spend_helper_surface",
            "must_not_be_public_pre_spend_api",
            "the generated frame embeds pad_seed bytes and is proof/audit material only",
            "pub fn reviewed_fq12_packed_witness_state18_projection_mask_prf_input_frame",
            ["helper_visibility_guard_no_pre_spend_callers"],
        ),
        export_surface(
            "masked_projection_opening_helper",
            "reviewed_fq12_masked_packed_witness_state18_projection_opening",
            ["selected account projection residues"],
            "forbidden_pre_spend_helper_surface",
            "must_not_be_public_pre_spend_api",
            "unmasks the six selected residues from witness_commitment using pad_seed and slot_id",
            "pub fn reviewed_fq12_masked_packed_witness_state18_projection_opening",
            ["helper_visibility_guard_no_pre_spend_callers", "wrong_pad_seed_reject_no_partial_opening"],
        ),
        export_surface(
            "packed_witness_opening_struct",
            "ProductionReviewedFq12MaskedState18PackedWitnessOpening",
            ["masked_projection", "projection_masks", "account_projection", "witness_syndrome"],
            "allowed_accepting_certificate_only",
            "charged_after_valid_spend_only",
            "the struct contains forbidden pre-spend fields and may be returned only inside an accepting certificate",
            "pub struct ProductionReviewedFq12MaskedState18PackedWitnessOpening",
            ["wrong_pad_seed_reject_no_partial_opening", "malformed_opening_reject_no_material"],
        ),
        export_surface(
            "packed_witness_opening_frame",
            "reviewed_fq12_masked_state18_packed_witness_opening_frame",
            ["projection_masks", "account_projection", "masked_projection"],
            "allowed_accepting_certificate_only",
            "charged_after_valid_spend_only",
            "opening frame serialization includes selected residues and masks, so reject paths must not expose it",
            "pub fn reviewed_fq12_masked_state18_packed_witness_opening_frame",
            ["malformed_opening_reject_no_material", "wrong_pad_seed_reject_no_partial_opening"],
        ),
        export_surface(
            "masked_packed_witness_opening_materializer",
            "reviewed_fq12_masked_state18_packed_witness_opening",
            ["projection_masks", "account_projection", "masked_projection", "repacked_witness_commitment"],
            "forbidden_pre_spend_helper_surface",
            "must_not_be_public_pre_spend_api",
            "materializes the full selected opening from witness_commitment plus pad_seed",
            "pub fn reviewed_fq12_masked_state18_packed_witness_opening",
            ["helper_visibility_guard_no_pre_spend_callers", "wrong_pad_seed_reject_no_partial_opening"],
        ),
        export_surface(
            "masked_projection_opening_verifier",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_opening",
            ["account_witness_projection_opening_frame"],
            "forbidden_pre_spend_helper_surface",
            "must_not_be_public_pre_spend_api",
            "returns the selected account witness opening frame and must not be exposed before accepted spend",
            "pub fn verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_opening",
            ["helper_visibility_guard_no_pre_spend_callers", "malformed_opening_reject_no_material"],
        ),
        export_surface(
            "masked_packed_witness_opening_verifier",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening",
            ["ProductionReviewedFq12MaskedState18PackedWitnessOpening"],
            "forbidden_pre_spend_helper_surface",
            "must_not_be_public_pre_spend_api",
            "returns full selected opening material and must be reachable externally only through accepted certificate construction",
            "pub fn verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening",
            ["wrong_pad_seed_reject_no_partial_opening", "malformed_opening_reject_no_material"],
        ),
        export_surface(
            "masked_projection_carrier_tuple_verifier",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
            ["account_witness_projection_opening_frame", "pad_seed_opening_frame internally"],
            "forbidden_reject_export_surface",
            "must_convert_export_to_opaque_error",
            "tuple verifier builds selected frames internally and converts hidden-material Export to Err",
            "pub fn verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
            ["carrier_rebinding_reject_no_material", "hidden_material_export_converted_to_error"],
        ),
        export_surface(
            "candidate_verifier",
            "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            ["checked-core account-carried signature parts"],
            "forbidden_reject_export_surface",
            "allowed_only_if_return_type_stays_parts_only",
            "candidate acceptance verifies the tuple but returns parts, not selected opening material",
            "pub fn verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            ["wrong_pad_seed_reject_no_partial_opening", "carrier_rebinding_reject_no_material"],
        ),
        export_surface(
            "accepting_certificate",
            "verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate",
            ["packed_witness_opening", "account_witness_projection_opening_frame", "pad_seed_digest", "carrier_lanes"],
            "allowed_accepting_certificate_path",
            "charged_after_valid_spend_only",
            "the only allowed external selected-material export path after all slot, statement, pad, carrier, and checked-core checks pass",
            "pub fn verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate",
            ["malformed_opening_reject_no_material", "spent_slot_replay_no_new_opening"],
        ),
        export_surface(
            "hidden_material_replay_inputs",
            "HiddenSourceFallbackProjectionHiddenMaterialReplayInput and FrameReplayInput",
            ["account_witness_opening_frame", "pad_seed_opening_frame", "pad_seed"],
            "forbidden_reject_export_surface",
            "internal_witness_only",
            "replay inputs carry pad_seed and selected frames and must never be returned from rejecting selected APIs",
            "pub struct HiddenSourceFallbackProjectionHiddenMaterialFrameReplayInput",
            ["hidden_material_export_converted_to_error", "carrier_rebinding_reject_no_material"],
        ),
        export_surface(
            "consensus_opening_frame_application",
            "apply_selected_projection_consensus_opening_frame",
            ["account_witness_opening_frame argument", "digest-only spend record result"],
            "post_spend_lifecycle_path",
            "argument_may_be_post_acceptance_result_must_be_digest_only",
            "consensus application consumes an opening frame and returns transition records without fresh selected material",
            "pub fn apply_selected_projection_consensus_opening_frame",
            ["spent_slot_replay_no_new_opening", "snapshot_restore_no_selected_material"],
        ),
    ]

    presence_checks = [surface["evidence"] for surface in surfaces]
    missing_evidence = [item["id"] for item in presence_checks if not item["present"]]

    classification_counts: dict[str, int] = {}
    for surface in surfaces:
        classification_counts[surface["path_class"]] = (
            classification_counts.get(surface["path_class"], 0) + 1
        )

    overread_risks = [
        {
            "id": "helper_materializers_are_pub",
            "risk": (
                "Rust helper functions can materialize projection masks, unmasked selected "
                "residues, account opening frames, or pad-seed-derived frames when called "
                "with pad_seed and matching opening data."
            ),
            "control": (
                "Treat them as crate-internal/proof-only/test-only unless called by the "
                "accepting certificate constructor after all verification checks pass."
            ),
        },
        {
            "id": "reject_path_partial_exports",
            "risk": (
                "Wrong pad seed, malformed opening, carrier rebinding, or hidden-material "
                "Export branches could leak partially built selected material if future "
                "return types become structured."
            ),
            "control": "Pin every rejecting selected path to an opaque Err with no certificate payload.",
        },
        {
            "id": "snapshot_or_debug_serde",
            "risk": (
                "Serialize/Deserialize or diagnostics on selected opening structs could "
                "turn post-spend certificate internals into a pre-spend export surface."
            ),
            "control": (
                "Snapshot and account-state schemas stay digest/commitment-only; selected "
                "opening structs are certificate-internal unless explicitly post-acceptance."
            ),
        },
        {
            "id": "pad_seed_frame_exposure",
            "risk": (
                "Mask PRF frames and hidden-material replay inputs embed pad_seed bytes or "
                "pad_seed-derived opening frames."
            ),
            "control": "Expose pad_seed_digest only on account-state, snapshot, ledger, and reject APIs.",
        },
    ]

    required_api_policy = [
        "Pre-spend account-state, snapshot, restore, reorg, signing, verification, diagnostics, and reject APIs may expose masked_projection and pad_seed_digest only as already-declared public binding material.",
        "The only external API that may return account_projection, projection_masks, or account_witness_projection_opening_frame is the accepting certificate path after successful selected spend verification.",
        "Helper materializers that accept pad_seed or return selected frames/openings are proof-only/internal unless wrapped by the accepting certificate constructor.",
        "Reject-before-acceptance returns one public error shape and no structured payload containing selected material or pad_seed bytes.",
        "Post-spend lifecycle records may persist slot keys, opening commitments, and pad_seed_digest for replay prevention, but not fresh selected opening material.",
    ]

    return {
        "schema": "Descender.pre_signature_projection_projection_overread_controls.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "projection_overread_controls_specified_negative_tests_required",
        "deployment_ready": False,
        "target": spend_budget["target"],
        "residual_term": {
            "id": "Adv_projection_overread(q_sign, q_verify)",
            "source": rel(LIFECYCLE_DISCHARGE_JSON),
            "status": "residual_blocker",
            "can_mark_zero": False,
            "local_control_status": "concrete_audit_complete_tests_not_implemented",
            "reason": (
                "Allowed accepting-certificate disclosure is enumerated, but helper "
                "visibility and reject/no-pre-spend negative tests still have to be "
                "implemented before the residual can be discharged."
            ),
        },
        "pre_spend_allowed_public_values": allowed,
        "pre_spend_forbidden_values": forbidden,
        "accepting_certificate_allowed_values": [
            "packed_witness_opening.projection_masks",
            "packed_witness_opening.account_projection",
            "account_witness_projection_opening_frame",
            "pad_seed_digest",
            "carrier_lanes",
            "checked-core and transcript binding fields",
        ],
        "path_policy": required_api_policy,
        "classification_counts": classification_counts,
        "export_surfaces": surfaces,
        "negative_tests_required": negative_tests,
        "overread_risks": overread_risks,
        "residual_blockers": [
            {
                "id": "negative_tests_missing",
                "required_to_close": "implement the negative tests listed in this artifact",
            },
            {
                "id": "helper_visibility_not_enforced",
                "required_to_close": "make selected materializers non-public, feature-gated, or prove no pre-spend/reject external callers can reach them",
            },
            {
                "id": "reject_equivalence_not_proved",
                "required_to_close": "prove or test opaque constant-shape rejects for every malformed selected path",
            },
            {
                "id": "composition_not_promoted",
                "required_to_close": "feed this artifact back into lifecycle same-game and spend-time budget discharges",
            },
        ],
        "evidence_summary": {
            "presence_checks": len(presence_checks),
            "missing_evidence": missing_evidence,
            "all_needles_present": not missing_evidence,
        },
        "source_artifacts": {
            "spend_time_budget": rel(SPEND_BUDGET_JSON),
            "spend_time_budget_discharge": rel(SPEND_DISCHARGE_JSON),
            "lifecycle_same_game_discharge": rel(LIFECYCLE_DISCHARGE_JSON),
            "leakage_audit": rel(LEAKAGE_AUDIT_MD),
            "production_reference": rel(PRODUCTION_REFERENCE),
        },
        "source_status": {
            "spend_time_budget": spend_budget.get("status"),
            "spend_time_budget_discharge": spend_discharge.get("status"),
            "lifecycle_same_game_discharge": lifecycle_discharge.get("status"),
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    residual = payload["residual_term"]
    lines = [
        "# Pre-Signature Projection Overread Controls",
        "",
        "Concrete export-control audit for residual term",
        "`Adv_projection_overread(q_sign, q_verify)` on the selected",
        "308-byte/Fq12 state-assisted masked projection-carrier path.",
        "",
        "## Status",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- residual term: `{residual['id']}`",
        f"- can mark zero: `{residual['can_mark_zero']}`",
        f"- local control status: `{residual['local_control_status']}`",
        f"- reason: {residual['reason']}",
        "",
        "## Path Policy",
        "",
    ]
    lines.extend(f"- {item}" for item in payload["path_policy"])

    lines.extend(
        [
            "",
            "## Pre-Spend Boundary",
            "",
            "Allowed public values before spend:",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["pre_spend_allowed_public_values"])
    lines.extend(["", "Forbidden before accepted spend:", ""])
    lines.extend(f"- {item}" for item in payload["pre_spend_forbidden_values"])

    lines.extend(
        [
            "",
            "Allowed only after accepted spend certificate:",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["accepting_certificate_allowed_values"])

    lines.extend(
        [
            "",
            "## Export Surface Audit",
            "",
            "| surface | class | status | materializes | control |",
            "|---|---|---|---|---|",
        ]
    )
    for item in payload["export_surfaces"]:
        materializes = ", ".join(f"`{value}`" for value in item["materializes"])
        lines.append(
            f"| `{item['symbol']}` | `{item['path_class']}` | `{item['status']}` | "
            f"{materializes} | {item['control']} |"
        )

    lines.extend(["", "Classification counts:", ""])
    for key, value in sorted(payload["classification_counts"].items()):
        lines.append(f"- `{key}`: `{value}`")

    lines.extend(["", "## Overread Risks", ""])
    for item in payload["overread_risks"]:
        lines.append(f"- `{item['id']}`: {item['risk']} Control: {item['control']}")

    lines.extend(["", "## Required Negative Tests", ""])
    for item in payload["negative_tests_required"]:
        surfaces = ", ".join(f"`{surface}`" for surface in item["surface"])
        lines.append(
            f"- `{item['id']}` (`{item['phase']}`): {item['assertion']} "
            f"Surfaces: {surfaces}. Expected: {item['expected_result']}."
        )

    lines.extend(
        [
            "",
            "## Residual Blockers",
            "",
        ]
    )
    for item in payload["residual_blockers"]:
        lines.append(f"- `{item['id']}`: {item['required_to_close']}")

    lines.extend(
        [
            "",
            "## Evidence Checks",
            "",
            f"- all needles present: `{payload['evidence_summary']['all_needles_present']}`",
            f"- missing evidence ids: `{payload['evidence_summary']['missing_evidence']}`",
            "",
            "| check | present | path |",
            "|---|---|---|",
        ]
    )
    for item in payload["export_surfaces"]:
        evidence = item["evidence"]
        lines.append(
            f"| `{evidence['id']}` | `{evidence['present']}` | `{evidence['path']}` |"
        )

    lines.extend(["", "## Sources", ""])
    for key, value in payload["source_artifacts"].items():
        lines.append(f"- `{key}`: `{value}`")
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
                "all_needles_present": payload["evidence_summary"]["all_needles_present"],
                "negative_tests_required": len(payload["negative_tests_required"]),
                "export_surfaces": len(payload["export_surfaces"]),
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
