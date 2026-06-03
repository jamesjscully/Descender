#!/usr/bin/env python3
"""Plan no-export negative tests for selected pre-signature projection paths."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "pre_signature_projection_no_export_negative_tests_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "pre_signature_projection_no_export_negative_tests_n32_q12289.md"
)
OVERREAD_CONTROLS_JSON = (
    READINESS_DIR
    / "pre_signature_projection_projection_overread_controls_n32_q12289.json"
)
SPEND_DISCHARGE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_spend_time_budget_discharge_n32_q12289.json"
)
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"


FORBIDDEN_RETURN_FIELDS = [
    "ProductionReviewedFq12MaskedState18PackedWitnessOpening",
    "packed_witness_opening",
    "account_witness_projection_opening_frame",
    "account_projection",
    "projection_masks",
    "masked_projection",
    "pad_seed",
    "pad_seed_opening_frame",
    "HiddenSourceFallbackProjectionBreakExport",
    "HiddenSourceFallbackProjectionHiddenMaterialFrameReplayInput",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rust_text() -> str:
    return PRODUCTION_REFERENCE.read_text()


def present(needle: str) -> bool:
    return needle in rust_text()


def evidence(id_: str, needle: str, role: str) -> dict[str, Any]:
    return {
        "id": id_,
        "path": rel(PRODUCTION_REFERENCE),
        "needle": needle,
        "present": present(needle),
        "role": role,
    }


def test_ref(name: str, role: str, coverage: str = "partial") -> dict[str, Any]:
    return {
        "rust_test": name,
        "present": present(f"fn {name}"),
        "coverage": coverage,
        "role": role,
    }


def missing_test(
    id_: str,
    rust_test: str,
    surface: list[str],
    current_coverage: list[dict[str, Any]],
    setup: str,
    mutations: list[str],
    expected_reject_shape: str,
    forbidden_fields: list[str] | None,
    acceptance_criteria: list[str],
) -> dict[str, Any]:
    implemented = present(f"fn {rust_test}")
    return {
        "id": id_,
        "rust_test": rust_test,
        "status": "implemented" if implemented else "missing",
        "required_surface": surface,
        "current_coverage": current_coverage,
        "setup": setup,
        "mutations": mutations,
        "expected_reject_shape": expected_reject_shape,
        "forbidden_returned_fields": forbidden_fields or FORBIDDEN_RETURN_FIELDS,
        "acceptance_criteria": acceptance_criteria,
    }


def build_payload() -> dict[str, Any]:
    overread = load_json(OVERREAD_CONTROLS_JSON)
    spend = load_json(SPEND_DISCHARGE_JSON)

    negative_tests = [
        missing_test(
            "account_state_read_no_selected_material",
            "state_assisted_account_state_checked_records_no_selected_material",
            [
                "ProductionReviewedFq12StateAssistedAccountState::checked_slot_records",
                "ProductionReviewedFq12StateAssistedAccountSlotRecord",
                "reviewed_fq12_state_assisted_account_state_from_slot_records",
            ],
            [
                test_ref(
                    "account_state_requires_sorted_valid_slots_and_consumes",
                    "legacy account records expose commitments and unused status only",
                ),
                test_ref(
                    "deployment_preflight_fails_closed_until_selected_verifier_and_reviews_land",
                    "preflight asserts the state-assisted pad_seed_digest invariant is present",
                ),
            ],
            (
                "Construct a state-assisted account with a nonzero hidden-source opening "
                "commitment, restore anchor digest, pad_seed_digest, and an unused slot."
            ),
            [
                "call checked_slot_records",
                "round-trip through reviewed_fq12_state_assisted_account_state_from_slot_records",
                "serialize ProductionReviewedFq12StateAssistedLifecycleSnapshot containing the records",
            ],
            "Ok digest/commitment-only records; no reject path expected for valid state reads",
            [
                "account_projection",
                "projection_masks",
                "masked_projection",
                "pad_seed",
                "account_witness_projection_opening_frame",
                "packed_witness_opening",
            ],
            [
                "record fields are exactly slot_id, witness_commitment, randomizer_commitment, hidden_source_opening_commitment, restore_anchor_digest, pad_seed_digest, unused",
                "JSON contains pad_seed_digest and never contains a pad_seed byte field, projection_masks, account_projection, masked_projection, or opening frame field names",
                "round-trip reconstruction does not require or materialize selected projection material",
            ],
        ),
        missing_test(
            "snapshot_restore_no_selected_material",
            "selected_projection_lifecycle_snapshot_roundtrips_digest_only",
            [
                "ProductionReviewedFq12StateAssistedLifecycleSnapshot",
                "ProductionReviewedFq12StateAssistedAccountLifecycleManager::from_snapshot",
                "ProductionSelectedProjectionConsensusSpendRecord",
                "ProductionSpentSlotRecord",
            ],
            [
                test_ref(
                    "lifecycle_file_store_serializes_only_public_account_state",
                    "legacy lifecycle store excludes secret sentinels",
                ),
                test_ref(
                    "spent_slot_ledger_file_store_serializes_only_public_spent_records",
                    "spent ledger persistence excludes full public key and secret sentinels",
                ),
                test_ref(
                    "lifecycle_snapshot_roundtrips_and_recovers_by_burning_reservations",
                    "legacy lifecycle snapshots round-trip and recover reservations",
                ),
            ],
            (
                "Build a state-assisted lifecycle snapshot with a live slot and a spent "
                "record carrying Some(pad_seed_digest)."
            ),
            [
                "serialize and deserialize lifecycle snapshot",
                "restore manager from snapshot and restore anchor",
                "serialize and deserialize spent records and selected projection transition records",
            ],
            "Ok for valid snapshot/ledger; malformed restore errors must be ProductionLifecycleError with public reason only",
            None,
            [
                "serialized lifecycle, spent ledger, and transition JSON expose pad_seed_digest only",
                "restored live slots still expose no selected projection residue, masks, pad_seed bytes, or opening frame",
                "restore-anchor rejection returns ProductionLifecycleError { slot_id, reason } with no structured payload",
            ],
        ),
        missing_test(
            "wrong_pad_seed_reject_no_partial_opening",
            "masked_projection_wrong_pad_seed_rejects_without_partial_opening",
            [
                "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
                "verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate",
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening",
            ],
            [
                test_ref(
                    "reviewed_fq12_account_carried_sigma_equation_checks_action_relation",
                    "wrong pad_seed already rejects pad-opening witness checks on the older hidden-source path",
                )
            ],
            "Create a valid selected masked projection fixture, then flip one byte of pad_seed.",
            [
                "candidate verifier with wrong pad_seed",
                "soundness certificate verifier with wrong pad_seed",
                "packed witness opening verifier with wrong pad_seed",
            ],
            (
                "Err(ProductionAccountBindingError) for external verifiers, or Err(String) "
                "for helper-level verifier; slot_id must be Some(slot) where available"
            ),
            None,
            [
                "all wrong-pad paths return Err and never Ok(certificate), Ok(parts with selected material), Ok(Vec<u8> frame), or Ok(packed opening)",
                "error reason is one public string such as pad seed digest mismatch, pad-opening statement witness mismatch, or projection digest mismatch",
                "serialized/debug error text contains none of the forbidden field names or test pad_seed sentinel bytes",
            ],
        ),
        missing_test(
            "malformed_opening_reject_no_material",
            "masked_projection_malformed_openings_reject_without_material",
            [
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_opening",
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening",
                "verify_reviewed_fq12_state_assisted_slot_opening",
            ],
            [],
            "Start from a valid state-assisted opening, statement, and packed witness commitment.",
            [
                "statement digest mismatch",
                "slot id mismatch",
                "witness commitment mismatch",
                "randomizer commitment mismatch at slot boundary",
                "opening digest mismatch",
                "repacked witness commitment mismatch",
                "noncanonical selected residue in an input frame where decoding is exercised",
            ],
            "Err(String) or Err(ProductionAccountBindingError) with public reason and optional slot_id only",
            None,
            [
                "each malformed variant rejects before accepting certificate construction",
                "no partial ProductionReviewedFq12MaskedState18PackedWitnessOpening is returned",
                "no account_witness_projection_opening_frame Vec is returned on rejecting variants",
                "for every variant, the observable value is an error reason only",
            ],
        ),
        missing_test(
            "carrier_rebinding_reject_no_material",
            "masked_projection_carrier_rebinding_rejects_without_export",
            [
                "decode_reviewed_fq12_account_carried_with_fallback_projection",
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
                "reviewed_fq12_state_assisted_projection_carrier_binding",
            ],
            [],
            "Use a valid 308-byte/Fq12 state-assisted masked projection carrier fixture.",
            [
                "mutate one carrier lane",
                "mutate carrier check bytes",
                "mutate carrier transcript binding",
                "rebind message",
                "rebind public key",
                "rebind signature body",
                "rebind opening commitment",
                "rebind slot id",
            ],
            "Err(ProductionAccountBindingError) at external boundary or Err(String) at tuple helper boundary",
            None,
            [
                "every rebinding mutation rejects",
                "reject reason is public and slot-scoped where the slot has been decoded",
                "no HiddenSourceFallbackProjectionBreakExport, selected opening frame, projection mask, account projection, or pad_seed bytes are returned",
            ],
        ),
        missing_test(
            "hidden_material_export_converted_to_error",
            "masked_projection_hidden_material_export_branch_is_opaque_error",
            [
                "verify_hidden_source_fallback_hidden_material_frame_projection_or_export",
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
                "verify_reviewed_fq12_state_assisted_projection_carrier_tuple",
            ],
            [],
            (
                "Construct or mutate hidden-material replay input so the lower semantic "
                "predicate takes Export, then call the selected tuple verifier boundary."
            ),
            [
                "exercise the masked witness tuple boundary",
                "exercise the unmasked state-assisted tuple boundary if it remains externally reachable",
            ],
            "Err(String) whose reason is the tuple-level hidden-material break export message",
            [
                "HiddenSourceFallbackProjectionBreakExport",
                "HiddenSourceFallbackProjectionHiddenOpeningBreakExport",
                "HiddenSourceFallbackProjectionAccountCommitmentBreakExport",
                "HiddenSourceFallbackProjectionPadBreakExport",
                "account_witness_opening_frame",
                "pad_seed_opening_frame",
                "pad_seed",
            ],
            [
                "the lower helper may return Export in an internal unit fixture, but the selected external tuple verifier never returns Export",
                "the external observable is one opaque Err string",
                "the Err does not serialize the exported frame, carrier bytes, pad_seed, or selected projection values",
            ],
        ),
        missing_test(
            "spent_slot_replay_no_new_opening",
            "selected_projection_spent_replay_rejects_digest_only_no_opening",
            [
                "apply_selected_projection_consensus_opening_frame",
                "ProductionSpentSlotLedger::require_pad_seed_digest_unspent",
                "ProductionSelectedProjectionConsensusSpendRecord",
            ],
            [
                test_ref(
                    "spent_slot_ledger_blocks_stale_wallet_replay",
                    "legacy spent ledger blocks stale wallet replay",
                ),
                test_ref(
                    "spent_slot_ledger_records_reorg_burns",
                    "legacy reorg burn records spent slots",
                ),
                test_ref(
                    "spent_slot_ledger_file_store_serializes_only_public_spent_records",
                    "spent records serialize public ledger state only",
                ),
            ],
            "Apply one valid selected projection opening frame and keep the resulting spent ledger.",
            [
                "replay same slot with same opening frame",
                "replay under stale lifecycle snapshot with existing spent record",
                "attempt a second slot carrying duplicate pad_seed_digest",
            ],
            "Err(ProductionLifecycleError) with public reason: slot already consumed, slot already in spent ledger, or pad seed digest already in spent ledger",
            None,
            [
                "accepted spend record stores slot_id, snapshot_sequence, hidden_source_opening_commitment, pad_seed_digest, and public spent_record only",
                "replay errors return no fresh opening frame and no certificate/opening struct",
                "spent ledger duplicate-digest path is covered, not only duplicate slot id",
            ],
        ),
        missing_test(
            "helper_visibility_guard_no_pre_spend_callers",
            "selected_projection_materializers_are_not_public_pre_spend_api",
            [
                "reviewed_fq12_packed_witness_state18_projection_opening",
                "reviewed_fq12_packed_witness_state18_projection_masks",
                "reviewed_fq12_masked_state18_packed_witness_opening",
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_opening",
                "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
            ],
            [
                {
                    "rust_test": "not_applicable_static_visibility",
                    "present": False,
                    "coverage": "gap",
                    "role": "current helper materializers are pub and no static API boundary test exists",
                }
            ],
            "Static check over production_reference.rs public function signatures and external API callers.",
            [
                "detect pub fn materializers that return selected residues, masks, packed openings, or opening frames",
                "verify account-state, snapshot, signing, verification, diagnostics, and reject APIs do not call them except through accepted certificate construction",
            ],
            "Static test failure listing any public pre-spend materializer or external rejecting caller",
            None,
            [
                "selected-material helper functions are crate-private, cfg(test), feature-gated, or documented and tested as post-acceptance only",
                "the only external selected-material export path is verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate after all checks pass",
                "public state/read/reject APIs have no call path returning selected material",
            ],
        ),
    ]

    evidence_items = [
        evidence(
            "state_assisted_slot_record_fields",
            "pub struct ProductionReviewedFq12StateAssistedAccountSlotRecord",
            "digest/commitment-only state-assisted account record exists",
        ),
        evidence(
            "selected_certificate_return_type",
            "pub struct ProductionReviewedFq12MaskedProjectionSoundnessCertificate",
            "accepting certificate intentionally contains selected material",
        ),
        evidence(
            "candidate_verifier_return_type",
            "-> Result<ProductionReviewedFq12AccountCarriedSignatureParts, ProductionAccountBindingError>",
            "candidate verifier returns checked parts, not selected opening material",
        ),
        evidence(
            "export_branch_converted_to_error",
            "masked witness state18 projection carrier tuple exported a hidden-material break",
            "selected tuple verifier converts hidden-material Export to Err",
        ),
        evidence(
            "duplicate_pad_seed_digest_guard",
            "pad seed digest is already in spent ledger",
            "spent ledger has duplicate pad_seed_digest reject path",
        ),
    ]

    missing = [test["id"] for test in negative_tests if test["status"] == "missing"]
    present_tests = [test["id"] for test in negative_tests if test["status"] == "implemented"]

    return {
        "schema": "Descender.pre_signature_projection_no_export_negative_tests.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "negative_test_plan_complete_implementation_missing",
        "deployment_ready": False,
        "target": overread["target"],
        "scope": {
            "purpose": (
                "Map required no-export negative tests to current Rust coverage and "
                "pin the exact missing tests needed to prove no pre-spend or reject "
                "path exports selected projection material."
            ),
            "production_reference": rel(PRODUCTION_REFERENCE),
            "rust_edits_made": False,
        },
        "forbidden_returned_fields_default": FORBIDDEN_RETURN_FIELDS,
        "required_tests": negative_tests,
        "coverage_summary": {
            "required_test_count": len(negative_tests),
            "implemented_required_tests": present_tests,
            "missing_required_tests": missing,
            "missing_required_test_count": len(missing),
            "current_rust_coverage_assessment": (
                "Current tests cover adjacent public metadata, legacy lifecycle/ledger "
                "serialization, and replay behavior, but do not yet instantiate the "
                "selected 308-byte/Fq12 state-assisted no-export negative paths."
            ),
        },
        "acceptance_gate": {
            "can_close_projection_overread_residual": False,
            "can_mark_spend_time_term_proved": False,
            "required_to_close": [
                "implement every missing Rust test named in required_tests",
                "ensure every reject-before-acceptance path returns Err-only public shape",
                "ensure account-state/snapshot/ledger serialization contains digest/commitment fields only",
                "restrict or gate public selected-material helper surfaces or prove they are post-acceptance only",
            ],
        },
        "evidence": evidence_items,
        "source_artifacts": {
            "projection_overread_controls": {
                "path": rel(OVERREAD_CONTROLS_JSON),
                "status": overread.get("status"),
                "residual_status": overread.get("residual_term", {}).get("status"),
            },
            "spend_time_budget_discharge": {
                "path": rel(SPEND_DISCHARGE_JSON),
                "status": spend.get("status"),
                "proved": spend.get("carried_term", {}).get("proved"),
            },
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection No-Export Negative Tests",
        "",
        "This artifact is the implementation plan for the selected 308-byte/Fq12",
        "state-assisted masked projection-carrier no-export tests. It maps each",
        "required negative test to current Rust coverage and names the missing",
        "test that must be added before the projection-overread and spend-time",
        "budget residuals can be closed.",
        "",
        "## Status",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- production reference: `{payload['scope']['production_reference']}`",
        f"- Rust edits made: `{payload['scope']['rust_edits_made']}`",
        f"- required tests: `{payload['coverage_summary']['required_test_count']}`",
        f"- missing required tests: `{payload['coverage_summary']['missing_required_test_count']}`",
        "",
        "## Required Tests",
        "",
    ]
    for test in payload["required_tests"]:
        lines.extend(
            [
                f"### `{test['id']}`",
                "",
                f"- Rust test: `{test['rust_test']}`",
                f"- status: `{test['status']}`",
                f"- expected shape: {test['expected_reject_shape']}",
                f"- setup: {test['setup']}",
                "",
                "Surface:",
            ]
        )
        for item in test["required_surface"]:
            lines.append(f"- `{item}`")
        lines.extend(["", "Mutations/checks:"])
        for item in test["mutations"]:
            lines.append(f"- {item}")
        lines.extend(["", "Current coverage:"])
        if test["current_coverage"]:
            for item in test["current_coverage"]:
                lines.append(
                    f"- `{item['rust_test']}`: present=`{item['present']}`, "
                    f"coverage=`{item['coverage']}`; {item['role']}"
                )
        else:
            lines.append("- none found")
        lines.extend(["", "Forbidden returned fields:"])
        for item in test["forbidden_returned_fields"]:
            lines.append(f"- `{item}`")
        lines.extend(["", "Acceptance criteria:"])
        for item in test["acceptance_criteria"]:
            lines.append(f"- {item}")
        lines.append("")

    lines.extend(
        [
            "## Acceptance Gate",
            "",
            f"- can close projection-overread residual: `{payload['acceptance_gate']['can_close_projection_overread_residual']}`",
            f"- can mark spend-time term proved: `{payload['acceptance_gate']['can_mark_spend_time_term_proved']}`",
            "",
            "Required to close:",
        ]
    )
    for item in payload["acceptance_gate"]["required_to_close"]:
        lines.append(f"- {item}")

    lines.extend(["", "## Evidence", "", "| id | present | role | needle |", "|---|---|---|---|"])
    for item in payload["evidence"]:
        lines.append(
            f"| `{item['id']}` | `{item['present']}` | {item['role']} | `{item['needle']}` |"
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
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "status": payload["status"],
                "missing_required_test_count": payload["coverage_summary"][
                    "missing_required_test_count"
                ],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
