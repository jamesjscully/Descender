#!/usr/bin/env python3
"""Audit helper/API export boundaries for selected projection material."""

from __future__ import annotations

import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_helper_api_export_boundary_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_helper_api_export_boundary_n32_q12289.md"
)
SPEND_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_budget_discharge_n32_q12289.json"
)
OVERREAD_JSON = (
    READINESS_DIR / "pre_signature_projection_projection_overread_controls_n32_q12289.json"
)
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def source_text() -> str:
    return PRODUCTION_REFERENCE.read_text()


def line_for(text: str, needle: str) -> int | None:
    offset = text.find(needle)
    if offset < 0:
        return None
    return text.count("\n", 0, offset) + 1


def code_presence(text: str, needle: str) -> dict[str, Any]:
    line = line_for(text, needle)
    return {
        "path": rel(PRODUCTION_REFERENCE),
        "needle": needle,
        "present": line is not None,
        "line": line,
    }


def rust_visibility(text: str, symbol: str, kind: str) -> dict[str, Any]:
    pattern = re.compile(rf"(?m)^(?P<visibility>pub\s+)?{kind}\s+{re.escape(symbol)}\b")
    match = pattern.search(text)
    return {
        "kind": kind,
        "declared": match is not None,
        "visibility": "pub" if match and match.group("visibility") else "private",
        "line": text.count("\n", 0, match.start()) + 1 if match else None,
    }


def surface(
    text: str,
    id_: str,
    symbol: str,
    kind: str,
    materializes: list[str],
    classification: str,
    exposure: str,
    risk: str,
    required_boundary: str,
    wrapper_tests: list[str],
    needle: str | None = None,
) -> dict[str, Any]:
    return {
        "id": id_,
        "symbol": symbol,
        "declaration": rust_visibility(text, symbol, kind),
        "materializes": materializes,
        "classification": classification,
        "exposure": exposure,
        "risk": risk,
        "required_boundary": required_boundary,
        "wrapper_tests": wrapper_tests,
        "evidence": code_presence(text, needle or f"pub {kind} {symbol}"),
    }


def build_payload() -> dict[str, Any]:
    text = source_text()
    spend_discharge = load_json(SPEND_DISCHARGE_JSON)
    overread = load_json(OVERREAD_JSON)

    wrapper_tests = [
        {
            "id": "public_account_state_digest_only",
            "scope": "account-state read and checked records",
            "assertion": (
                "Public state records expose slot id, commitments, restore anchor, "
                "pad_seed_digest, and unused status only; account_projection, "
                "projection_masks, pad_seed bytes, and opening frames are absent."
            ),
        },
        {
            "id": "snapshot_restore_digest_only",
            "scope": "snapshot, restore, target-chain DB, and spent-ledger records",
            "assertion": (
                "Serialization round trips preserve pad_seed_digest and opening "
                "commitments only, never selected projection residues, masks, "
                "pad_seed bytes, or freshly generated opening frames."
            ),
        },
        {
            "id": "helper_visibility_compile_guard",
            "scope": "external/public API surface",
            "assertion": (
                "External consumers cannot call pad_seed-taking materializers except "
                "through an explicit proof/audit feature or accepting-certificate wrapper."
            ),
        },
        {
            "id": "wrong_pad_seed_no_partial_export",
            "scope": "pad_seed-supplied helper wrappers",
            "assertion": (
                "Wrong pad_seed or pad_seed_digest mismatch returns Err only, with no "
                "packed opening, account_projection, projection_masks, pad_seed bytes, "
                "or account_witness_projection_opening_frame in the returned value."
            ),
        },
        {
            "id": "malformed_opening_no_partial_export",
            "scope": "statement, slot id, witness commitment, opening digest, and repack mismatch",
            "assertion": (
                "Every malformed path rejects before acceptance with an opaque public "
                "error and no structured selected-material payload."
            ),
        },
        {
            "id": "carrier_export_branch_is_opaque_error",
            "scope": "hidden-material Export and carrier rebinding paths",
            "assertion": (
                "Carrier rebinding and hidden-material Export branches return Err only "
                "and never expose the internally built frame replay input."
            ),
        },
        {
            "id": "accepting_certificate_single_export_wrapper",
            "scope": "selected-material public wrapper",
            "assertion": (
                "The accepting certificate wrapper is the only public path returning "
                "account_projection, projection_masks, or account_witness_projection_opening_frame, "
                "and only after all slot, statement, pad, carrier, and checked-core checks pass."
            ),
        },
        {
            "id": "post_spend_lifecycle_result_digest_only",
            "scope": "consensus opening-frame application",
            "assertion": (
                "Post-spend lifecycle APIs may consume an opening frame but return only "
                "transition/snapshot/spent records with pad_seed_digest and commitments."
            ),
        },
    ]

    surfaces = [
        surface(
            text,
            "state_slot_record",
            "ProductionReviewedFq12StateAssistedAccountSlotRecord",
            "struct",
            [
                "slot_id",
                "witness_commitment",
                "randomizer_commitment",
                "hidden_source_opening_commitment",
                "restore_anchor_digest",
                "pad_seed_digest",
            ],
            "public_api_allowed",
            "pub serialized record",
            "low: digest/commitment-only state record has no selected opening fields",
            "keep as the public pre-spend account-state shape",
            ["public_account_state_digest_only", "snapshot_restore_digest_only"],
            "pub struct ProductionReviewedFq12StateAssistedAccountSlotRecord",
        ),
        surface(
            text,
            "hidden_source_witness",
            "ProductionReviewedFq12HiddenSourceWitness",
            "struct",
            ["witness_syndrome", "pad_seed"],
            "internal_witness_only",
            "pub struct carrying pad_seed",
            "high if exported: contains raw pad_seed bytes",
            "do not serialize or return from account-state, snapshot, signing, verification, diagnostics, or reject APIs",
            ["helper_visibility_compile_guard", "wrong_pad_seed_no_partial_export"],
            "pub struct ProductionReviewedFq12HiddenSourceWitness",
        ),
        surface(
            text,
            "projection_opening_from_commitment",
            "reviewed_fq12_packed_witness_state18_projection_opening",
            "fn",
            ["account_projection"],
            "risky_public_helper",
            "pub helper",
            "high: extracts selected account projection residues from packed witness commitment",
            "crate-private/proof-only; not callable by pre-spend public APIs",
            ["helper_visibility_compile_guard"],
        ),
        surface(
            text,
            "projection_masks_from_pad_seed",
            "reviewed_fq12_packed_witness_state18_projection_masks",
            "fn",
            ["projection_masks"],
            "risky_public_helper",
            "pub helper accepting pad_seed",
            "high: derives forbidden pre-spend mask residues when supplied pad_seed",
            "crate-private/proof-only or accepting-certificate-only wrapper",
            ["helper_visibility_compile_guard", "wrong_pad_seed_no_partial_export"],
        ),
        surface(
            text,
            "projection_mask_prf_input_frame",
            "reviewed_fq12_packed_witness_state18_projection_mask_prf_input_frame",
            "fn",
            ["pad_seed-containing mask PRF input frame"],
            "risky_public_helper",
            "pub helper accepting pad_seed",
            "high: returns a frame embedding pad_seed-derived PRF input material",
            "proof/audit-only; no public diagnostics or error payloads",
            ["helper_visibility_compile_guard"],
        ),
        surface(
            text,
            "pack_masked_witness_from_parts",
            "reviewed_fq12_pack_masked_witness_state18_from_parts",
            "fn",
            ["projection_masks", "masked_projection"],
            "risky_public_helper",
            "pub helper accepting account_projection and pad_seed",
            "medium: builds masked commitment from selected residues and pad_seed",
            "test/proof fixture only; not an account-state or verifier export API",
            ["helper_visibility_compile_guard"],
        ),
        surface(
            text,
            "masked_projection_opening",
            "reviewed_fq12_masked_packed_witness_state18_projection_opening",
            "fn",
            ["account_projection"],
            "risky_public_helper",
            "pub helper accepting pad_seed",
            "high: unmasks selected residues from witness_commitment using pad_seed",
            "crate-private/proof-only or accepting-certificate-only wrapper",
            ["helper_visibility_compile_guard", "wrong_pad_seed_no_partial_export"],
        ),
        surface(
            text,
            "packed_witness_opening_struct",
            "ProductionReviewedFq12MaskedState18PackedWitnessOpening",
            "struct",
            ["masked_projection", "projection_masks", "account_projection", "witness_syndrome"],
            "post_acceptance_certificate_material",
            "pub struct",
            "high if returned before acceptance: directly contains masks and selected residues",
            "only returned inside accepting certificate after all checks pass",
            ["accepting_certificate_single_export_wrapper", "malformed_opening_no_partial_export"],
            "pub struct ProductionReviewedFq12MaskedState18PackedWitnessOpening",
        ),
        surface(
            text,
            "packed_witness_opening_frame",
            "reviewed_fq12_masked_state18_packed_witness_opening_frame",
            "fn",
            ["projection_masks", "account_projection", "masked_projection"],
            "post_acceptance_certificate_material",
            "pub serializer",
            "high if exposed in diagnostics: serializes selected opening internals",
            "certificate-internal serialization only; no pre-spend debug/snapshot endpoint",
            ["helper_visibility_compile_guard", "malformed_opening_no_partial_export"],
        ),
        surface(
            text,
            "packed_witness_opening_materializer",
            "reviewed_fq12_masked_state18_packed_witness_opening",
            "fn",
            ["projection_masks", "account_projection", "masked_projection", "repacked_witness_commitment"],
            "risky_public_helper",
            "pub helper accepting pad_seed",
            "high: materializes full selected opening from witness_commitment plus pad_seed",
            "internal/proof-only unless invoked by accepting-certificate construction",
            ["helper_visibility_compile_guard", "wrong_pad_seed_no_partial_export"],
        ),
        surface(
            text,
            "masked_projection_opening_verifier",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_opening",
            "fn",
            ["account_witness_projection_opening_frame"],
            "risky_public_helper",
            "pub verifier accepting pad_seed",
            "high: returns selected account witness opening frame from pad_seed and opening material",
            "not an external pre-spend verifier; route selected-frame exports through accepting certificate only",
            ["helper_visibility_compile_guard", "wrong_pad_seed_no_partial_export", "malformed_opening_no_partial_export"],
        ),
        surface(
            text,
            "masked_packed_opening_verifier",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening",
            "fn",
            ["ProductionReviewedFq12MaskedState18PackedWitnessOpening"],
            "risky_public_helper",
            "pub verifier accepting pad_seed",
            "high: returns the full selected opening object before certificate wrapping",
            "crate-private/proof-only or called solely by accepting-certificate wrapper",
            ["helper_visibility_compile_guard", "wrong_pad_seed_no_partial_export", "malformed_opening_no_partial_export"],
        ),
        surface(
            text,
            "projection_carrier_tuple",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
            "fn",
            ["account_witness_projection_opening_frame", "pad_seed_opening_frame internally", "pad_seed in replay input internally"],
            "risky_public_helper",
            "pub verifier accepting pad_seed",
            "high: builds selected frames internally and returns account_witness_projection_opening_frame on success",
            "internal wrapper under accepting certificate; reject paths must convert Export to opaque Err",
            ["carrier_export_branch_is_opaque_error", "wrong_pad_seed_no_partial_export"],
        ),
        surface(
            text,
            "slot_projection_carrier_tuple",
            "verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple",
            "fn",
            ["account_witness_projection_opening_frame"],
            "risky_public_helper",
            "pub verifier accepting pad_seed and slot",
            "high: slot-checking wrapper still returns selected opening frame",
            "internal wrapper under accepting certificate only",
            ["helper_visibility_compile_guard", "carrier_export_branch_is_opaque_error"],
        ),
        surface(
            text,
            "candidate_verifier",
            "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            "fn",
            ["checked-core account-carried signature parts"],
            "public_api_allowed_if_return_type_stays_parts_only",
            "pub verifier accepting pad_seed",
            "medium: verifies selected tuple with pad_seed but currently returns no selected opening material",
            "pin return type to checked-core parts only; wrong/rebound inputs return opaque Err",
            ["wrong_pad_seed_no_partial_export", "carrier_export_branch_is_opaque_error"],
        ),
        surface(
            text,
            "accepting_certificate",
            "verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate",
            "fn",
            [
                "packed_witness_opening",
                "account_witness_projection_opening_frame",
                "pad_seed_digest",
                "carrier_lanes",
            ],
            "allowed_public_export_after_acceptance",
            "pub verifier accepting pad_seed",
            "charged: intentionally returns selected material after valid spend checks",
            "the only external selected-material export path",
            ["accepting_certificate_single_export_wrapper", "malformed_opening_no_partial_export"],
        ),
        surface(
            text,
            "hidden_material_replay_input",
            "HiddenSourceFallbackProjectionHiddenMaterialFrameReplayInput",
            "struct",
            ["account_witness_opening_frame", "pad_seed_opening_frame", "pad_seed"],
            "internal_witness_only",
            "pub replay-input struct",
            "high if exported: aggregates both selected frame and raw pad_seed",
            "remain private and never returned from RejectBeforeAcceptance or Export paths",
            ["carrier_export_branch_is_opaque_error"],
            "struct HiddenSourceFallbackProjectionHiddenMaterialFrameReplayInput",
        ),
        surface(
            text,
            "consensus_opening_application",
            "apply_selected_projection_consensus_opening_frame",
            "fn",
            ["account_witness_projection_opening_frame argument", "digest-only transition result"],
            "post_spend_public_consumer",
            "pub lifecycle function",
            "low if result stays digest-only: consumes an accepted frame and records spend state",
            "may accept post-acceptance opening frame; must not return fresh selected opening material",
            ["post_spend_lifecycle_result_digest_only", "snapshot_restore_digest_only"],
        ),
    ]

    missing_evidence = [
        surface_item["id"]
        for surface_item in surfaces
        if not surface_item["evidence"]["present"] or not surface_item["declaration"]["declared"]
    ]
    risky = [
        item
        for item in surfaces
        if item["classification"]
        in {
            "risky_public_helper",
            "internal_witness_only",
            "post_acceptance_certificate_material",
        }
        and item["declaration"]["visibility"] == "pub"
    ]
    counts: dict[str, int] = {}
    for item in surfaces:
        counts[item["classification"]] = counts.get(item["classification"], 0) + 1

    return {
        "schema": "Descender.pre_signature_projection_helper_api_export_boundary.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "helper_api_export_boundary_audited_restrictions_required",
        "deployment_ready": False,
        "target": spend_discharge["target"],
        "boundary_decision": {
            "can_discharge_helper_api_residual": False,
            "can_consume_as_fail_closed_budget": True,
            "reason": (
                "The digest/commitment-only account-state and lifecycle records are safe, "
                "but multiple pub helper/verifier surfaces can materialize account_projection, "
                "projection_masks, pad_seed-derived frames, or account witness opening frames "
                "when supplied pad_seed. The residual closes only after visibility restrictions "
                "or wrapper tests prove those helpers are not pre-spend/reject export APIs."
            ),
        },
        "classification_counts": counts,
        "risky_public_or_exportable_surface_count": len(risky),
        "risky_public_or_exportable_surfaces": [
            {
                "id": item["id"],
                "symbol": item["symbol"],
                "materializes": item["materializes"],
                "risk": item["risk"],
                "required_boundary": item["required_boundary"],
                "line": item["declaration"]["line"],
            }
            for item in risky
        ],
        "surfaces": surfaces,
        "api_restrictions_required": [
            {
                "id": "selected_material_single_public_export",
                "restriction": (
                    "Only verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate "
                    "may be exposed as a public API returning account_projection, projection_masks, "
                    "or account_witness_projection_opening_frame, and only after valid spend checks."
                ),
            },
            {
                "id": "internalize_pad_seed_materializers",
                "restriction": (
                    "Pad_seed-taking helpers that derive masks, unmask selected residues, "
                    "construct packed openings, or construct account witness opening frames "
                    "must be crate-private, proof/test-only, or feature-gated."
                ),
            },
            {
                "id": "digest_only_public_state",
                "restriction": (
                    "Account-state, snapshot, restore, target-chain DB, spent-ledger, "
                    "diagnostic, and reject APIs may expose pad_seed_digest but never pad_seed bytes."
                ),
            },
            {
                "id": "opaque_reject_boundary",
                "restriction": (
                    "Reject-before-acceptance returns Err only; no structured error payload, "
                    "Export result, partial packed opening, selected frame, projection mask, "
                    "account projection, or pad_seed bytes may cross the API boundary."
                ),
            },
            {
                "id": "post_spend_consumer_digest_only_result",
                "restriction": (
                    "Post-spend lifecycle APIs may consume an accepted opening frame, but "
                    "their return records stay digest/commitment-only and do not re-export "
                    "fresh selected opening material."
                ),
            },
        ],
        "wrapper_tests_required": wrapper_tests,
        "residual_blockers": [
            {
                "id": "pub_helper_visibility",
                "required_to_close": "make risky helpers non-public or prove external API cannot reach them before accepted spend",
            },
            {
                "id": "wrapper_tests_missing",
                "required_to_close": "implement wrapper tests listed in this artifact",
            },
            {
                "id": "reject_payload_shape",
                "required_to_close": "pin all malformed, wrong-pad, and carrier Export paths to opaque Err-only results",
            },
            {
                "id": "debug_or_serde_selected_material",
                "required_to_close": "prevent diagnostics or serialization from exposing selected opening structs outside the post-acceptance certificate path",
            },
        ],
        "evidence_summary": {
            "surface_count": len(surfaces),
            "missing_evidence": missing_evidence,
            "all_needles_present": not missing_evidence,
        },
        "source_artifacts": {
            "spend_time_budget_discharge": rel(SPEND_DISCHARGE_JSON),
            "projection_overread_controls": rel(OVERREAD_JSON),
            "production_reference": rel(PRODUCTION_REFERENCE),
        },
        "source_status": {
            "spend_time_budget_discharge": spend_discharge.get("status"),
            "projection_overread_controls": overread.get("status"),
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    decision = payload["boundary_decision"]
    lines = [
        "# Pre-Signature Projection Helper/API Export Boundary",
        "",
        "This artifact audits Rust helper and API surfaces that can compute or",
        "return selected projection openings, masks, or pad-seed-derived opening",
        "frames for the selected n32/q12289 Fq12 projection path.",
        "",
        "## Status",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- can discharge helper/API residual: `{decision['can_discharge_helper_api_residual']}`",
        f"- can consume as fail-closed budget: `{decision['can_consume_as_fail_closed_budget']}`",
        f"- reason: {decision['reason']}",
        "",
        "## Risky Public or Exportable Surfaces",
        "",
        "| symbol | line | materializes | required boundary |",
        "|---|---:|---|---|",
    ]
    for item in payload["risky_public_or_exportable_surfaces"]:
        lines.append(
            f"| `{item['symbol']}` | {item['line']} | "
            f"{', '.join(f'`{value}`' for value in item['materializes'])} | "
            f"{item['required_boundary']} |"
        )

    lines.extend(
        [
            "",
            "## Full Surface Inventory",
            "",
            "| id | symbol | visibility | classification | materializes | risk |",
            "|---|---|---|---|---|---|",
        ]
    )
    for item in payload["surfaces"]:
        declaration = item["declaration"]
        lines.append(
            f"| `{item['id']}` | `{item['symbol']}` | `{declaration['visibility']}` | "
            f"`{item['classification']}` | "
            f"{', '.join(f'`{value}`' for value in item['materializes'])} | "
            f"{item['risk']} |"
        )

    lines.extend(["", "## Required API Restrictions", ""])
    for item in payload["api_restrictions_required"]:
        lines.append(f"- `{item['id']}`: {item['restriction']}")

    lines.extend(["", "## Required Wrapper Tests", ""])
    for item in payload["wrapper_tests_required"]:
        lines.append(f"- `{item['id']}`: {item['assertion']}")

    lines.extend(
        [
            "",
            "## Residual Blockers",
            "",
            "| blocker | required to close |",
            "|---|---|",
        ]
    )
    for item in payload["residual_blockers"]:
        lines.append(f"| `{item['id']}` | {item['required_to_close']} |")

    lines.extend(
        [
            "",
            "## Evidence",
            "",
            f"- production reference: `{payload['source_artifacts']['production_reference']}`",
            f"- spend-time budget discharge: `{payload['source_artifacts']['spend_time_budget_discharge']}`",
            f"- projection overread controls: `{payload['source_artifacts']['projection_overread_controls']}`",
            f"- all needles present: `{payload['evidence_summary']['all_needles_present']}`",
            f"- surface count: `{payload['evidence_summary']['surface_count']}`",
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
                "risky_surfaces": payload["risky_public_or_exportable_surface_count"],
                "all_needles_present": payload["evidence_summary"]["all_needles_present"],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
