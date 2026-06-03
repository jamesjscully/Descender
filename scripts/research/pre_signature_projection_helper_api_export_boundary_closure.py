#!/usr/bin/env python3
"""Produce the helper/API export-boundary closure matrix."""

from __future__ import annotations

import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
SOURCE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_helper_api_export_boundary_n32_q12289.json"
)
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_helper_api_export_boundary_closure_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_helper_api_export_boundary_closure_n32_q12289.md"
)
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"
PRODUCTION_REFERENCE_FACADE = ROOT / "tools/production-reference/src/lib.rs"


SELECTED_MATERIAL_FACADE_WITHHELD_SYMBOLS = [
    "ProductionReviewedFq12HiddenSourceWitness",
    "ProductionReviewedFq12MaskedState18PackedWitnessOpening",
    "HiddenSourceFallbackProjectionHiddenMaterialReplayInput",
    "reviewed_fq12_packed_witness_state18_projection_opening",
    "reviewed_fq12_packed_witness_state18_projection_masks",
    "reviewed_fq12_packed_witness_state18_projection_mask_prf_input_frame",
    "reviewed_fq12_pack_masked_witness_state18_from_parts",
    "reviewed_fq12_masked_packed_witness_state18_projection_opening",
    "reviewed_fq12_masked_state18_packed_witness_opening_frame",
    "reviewed_fq12_masked_state18_packed_witness_opening_digest",
    "reviewed_fq12_masked_state18_packed_witness_opening",
    "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_opening",
    "verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening",
    "verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
    "verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple",
]


TEST_CONTRACTS: dict[str, dict[str, Any]] = {
    "public_account_state_digest_only": {
        "rust_test": "production_reviewed_fq12_public_account_state_digest_only",
        "acceptance_criteria": [
            "Account-state reads and checked slot records expose slot id, commitments, restore anchor, pad_seed_digest, and unused only.",
            "The public record type has no account_projection, projection_masks, pad_seed bytes, or account_witness_projection_opening_frame field.",
            "Serde round trip of the public record preserves digest/commitment fields and cannot encode selected material.",
        ],
    },
    "snapshot_restore_digest_only": {
        "rust_test": "production_reviewed_fq12_snapshot_restore_digest_only",
        "acceptance_criteria": [
            "Lifecycle snapshot, target-chain DB, restore, and spent-ledger round trips preserve pad_seed_digest and commitments only.",
            "No serialized snapshot/restore/spent record contains raw pad_seed bytes, selected residues, projection masks, or opening frames.",
            "Reconstructed live-slot state after restore remains indistinguishable from digest/commitment-only public state.",
        ],
    },
    "helper_visibility_compile_guard": {
        "rust_test": "production_reviewed_fq12_helper_visibility_compile_guard",
        "acceptance_criteria": [
            "An integration-style compile guard proves external consumers cannot call pad_seed-taking materializers directly.",
            "The only selected-material public export path is the accepting-certificate wrapper.",
            "Proof/test fixtures may use internal helpers only behind crate-private, test-only, or explicit audit/proof feature boundaries.",
        ],
    },
    "wrong_pad_seed_no_partial_export": {
        "rust_test": "production_reviewed_fq12_wrong_pad_seed_no_partial_export",
        "acceptance_criteria": [
            "Wrong pad_seed and pad_seed_digest mismatch paths return Err only.",
            "The returned error exposes no packed opening, account_projection, projection_masks, pad_seed bytes, or account_witness_projection_opening_frame.",
            "Candidate verifier and wrapper paths preserve the same opaque error shape.",
        ],
    },
    "malformed_opening_no_partial_export": {
        "rust_test": "production_reviewed_fq12_malformed_opening_no_partial_export",
        "acceptance_criteria": [
            "Slot id, statement digest, witness commitment, opening digest, and repack mismatches all reject before acceptance.",
            "Every malformed path returns an opaque Err with no structured selected-material payload.",
            "The accepting certificate is not constructed after any malformed opening rejection.",
        ],
    },
    "carrier_export_branch_is_opaque_error": {
        "rust_test": "production_reviewed_fq12_carrier_export_branch_is_opaque_error",
        "acceptance_criteria": [
            "Carrier lane, check-byte, transcript-binding, and rebinding mutations reject as Err.",
            "Hidden-material Export branches are converted to opaque errors at the selected carrier tuple boundary.",
            "No frame replay input, pad_seed, account witness frame, projection mask, or account projection crosses the rejection boundary.",
        ],
    },
    "accepting_certificate_single_export_wrapper": {
        "rust_test": "production_reviewed_fq12_accepting_certificate_single_export_wrapper",
        "acceptance_criteria": [
            "verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate is the only public wrapper returning selected material.",
            "The wrapper returns selected material only after checked-core, slot, statement, pad, carrier, and packed-opening validation pass.",
            "All other public pre-spend verification APIs return checked-core parts, digest/commitment records, or Err only.",
        ],
    },
    "post_spend_lifecycle_result_digest_only": {
        "rust_test": "production_reviewed_fq12_post_spend_lifecycle_result_digest_only",
        "acceptance_criteria": [
            "Post-spend lifecycle APIs may consume an accepted opening frame but return only transition, snapshot, and spent records.",
            "Returned records contain pad_seed_digest and commitments, not fresh opening frames, projection masks, account_projection, or pad_seed bytes.",
            "Same-slot and duplicate-digest replay rejection stays digest-only.",
        ],
    },
}


SURFACE_DECISIONS: dict[str, dict[str, Any]] = {
    "hidden_source_witness": {
        "decision": ["internal-only allowed", "requires Rust API redesign", "needs test"],
        "closure_action": "Keep as signer/proof witness material only; make construction/export opaque or crate-confined before deployment.",
        "blocker": "public witness struct still carries raw pad_seed bytes",
        "acceptance": "No account-state, snapshot, signing, verification, diagnostic, or reject API returns or serializes this type.",
    },
    "projection_opening_from_commitment": {
        "decision": ["needs visibility restriction", "needs test"],
        "closure_action": "Restrict to crate-private/proof-only helper or call solely inside the accepting-certificate implementation.",
        "blocker": "pub helper extracts selected account_projection from witness commitment",
        "acceptance": "External compile guard cannot call the helper; no pre-spend public API returns its output.",
    },
    "projection_masks_from_pad_seed": {
        "decision": ["needs visibility restriction", "needs wrapper", "needs test"],
        "closure_action": "Route mask derivation through accepting-certificate construction or crate-private proof fixtures only.",
        "blocker": "pub pad_seed-taking helper derives forbidden projection_masks",
        "acceptance": "Wrong pad_seed paths produce Err only; no public helper returns masks before accepted spend.",
    },
    "projection_mask_prf_input_frame": {
        "decision": ["needs visibility restriction", "needs test"],
        "closure_action": "Keep PRF-input frame construction proof/audit-only and absent from diagnostics or error payloads.",
        "blocker": "pub helper returns pad_seed-derived PRF input frame",
        "acceptance": "External compile guard cannot call the helper and no reject/debug path includes the frame bytes.",
    },
    "pack_masked_witness_from_parts": {
        "decision": ["needs visibility restriction", "needs test"],
        "closure_action": "Keep as test/proof fixture helper; production wrappers should not accept selected account_projection plus pad_seed as public inputs.",
        "blocker": "pub helper builds masked commitments from selected residues and pad_seed",
        "acceptance": "The helper is unavailable to external consumers outside explicit proof/test scope.",
    },
    "masked_projection_opening": {
        "decision": ["needs visibility restriction", "needs wrapper", "needs test"],
        "closure_action": "Unmask selected residues only inside the accepting-certificate path after pad and opening checks.",
        "blocker": "pub helper unmasks account_projection from witness commitment plus pad_seed",
        "acceptance": "Wrong pad_seed returns Err only through wrappers and no pre-acceptance API returns the opening.",
    },
    "packed_witness_opening_struct": {
        "decision": ["needs wrapper", "needs test", "requires Rust API redesign"],
        "closure_action": "Make this post-acceptance certificate material opaque or field-private unless returned by the accepting certificate.",
        "blocker": "pub struct exposes masks and selected residues if constructed or returned before acceptance",
        "acceptance": "Only the accepting certificate can expose this object; malformed opening paths never construct it for callers.",
    },
    "packed_witness_opening_frame": {
        "decision": ["needs visibility restriction", "needs wrapper", "needs test"],
        "closure_action": "Use serialization as certificate-internal digest material only, not as a public diagnostics/snapshot surface.",
        "blocker": "pub serializer emits selected opening internals",
        "acceptance": "External compile guard cannot call the serializer directly and no reject/debug path returns the frame.",
    },
    "packed_witness_opening_materializer": {
        "decision": ["needs visibility restriction", "needs wrapper", "needs test"],
        "closure_action": "Materialize the full selected opening only behind accepting-certificate validation.",
        "blocker": "pub pad_seed-taking materializer returns full selected opening",
        "acceptance": "External consumers cannot call the materializer; wrong pad_seed and malformed inputs return opaque Err only.",
    },
    "masked_projection_opening_verifier": {
        "decision": ["needs visibility restriction", "needs wrapper", "needs test"],
        "closure_action": "Make the selected-frame verifier internal to accepting-certificate construction.",
        "blocker": "pub verifier returns account_witness_projection_opening_frame",
        "acceptance": "No external pre-spend verifier returns an account witness projection opening frame.",
    },
    "masked_packed_opening_verifier": {
        "decision": ["needs visibility restriction", "needs wrapper", "needs test"],
        "closure_action": "Return packed witness opening only inside the accepting-certificate wrapper.",
        "blocker": "pub verifier returns full selected opening object before certificate wrapping",
        "acceptance": "External callers either receive a soundness certificate after full acceptance or an opaque Err.",
    },
    "projection_carrier_tuple": {
        "decision": ["needs visibility restriction", "needs wrapper", "needs test"],
        "closure_action": "Keep carrier tuple verification under the accepting-certificate/candidate-verifier boundary.",
        "blocker": "pub verifier returns selected account witness frame and internally builds pad_seed replay input",
        "acceptance": "Carrier Export and rebinding failures convert to Err; no hidden-material replay input is observable.",
    },
    "slot_projection_carrier_tuple": {
        "decision": ["needs visibility restriction", "needs wrapper", "needs test"],
        "closure_action": "Keep slot-checking carrier tuple verification internal to candidate/certificate wrappers.",
        "blocker": "pub slot wrapper still returns selected account witness frame",
        "acceptance": "External callers cannot obtain the frame except through the accepting certificate.",
    },
    "hidden_material_replay_input": {
        "decision": ["internal-only allowed", "needs visibility restriction", "needs test"],
        "closure_action": "Keep replay input private/internal and never return it from RejectBeforeAcceptance or Export handling.",
        "blocker": "pub replay-input struct aggregates account witness frame, pad_seed frame, and raw pad_seed",
        "acceptance": "Hidden-material Export is opaque Err at the API boundary; replay input cannot be named or serialized externally.",
    },
}


SAFE_PUBLIC_DECISIONS: dict[str, dict[str, Any]] = {
    "state_slot_record": {
        "decision": ["internal-only not required", "needs test"],
        "closure_action": "Keep public as the digest/commitment-only account-state record.",
        "acceptance": "Wrapper tests prove it serializes no selected material.",
    },
    "candidate_verifier": {
        "decision": ["needs wrapper", "needs test"],
        "closure_action": "Allowed only while its return type remains checked-core signature parts and all selected-material checks are fail-closed.",
        "acceptance": "Wrong pad, malformed opening, and carrier Export failures return ProductionAccountBindingError only.",
    },
    "accepting_certificate": {
        "decision": ["needs wrapper", "needs test"],
        "closure_action": "Designate as the single selected-material export wrapper after full acceptance.",
        "acceptance": "Certificate returns selected material only after checked-core, slot, statement, pad, carrier, and packed-opening checks pass.",
    },
    "consensus_opening_application": {
        "decision": ["internal-only not required", "needs test"],
        "closure_action": "Allowed post-spend consumer if results stay digest/commitment-only.",
        "acceptance": "Transition, snapshot, and spent records do not return fresh selected material.",
    },
}


LOW_RISK_RUST_CHANGES_APPLIED = [
    {
        "id": "make_hidden_material_frame_replay_input_private",
        "symbol": "HiddenSourceFallbackProjectionHiddenMaterialFrameReplayInput",
        "change": "Removed public visibility from the frame replay-input struct and its selected-material fields.",
        "evidence": "tools/parameter-search/src/production_reference.rs keeps construction and verification inside the carrier-checking implementation.",
    },
    {
        "id": "hide_mask_prf_input_frame_helper",
        "symbol": "reviewed_fq12_packed_witness_state18_projection_mask_prf_input_frame",
        "change": "Restricted the pad-seed-derived PRF input-frame helper to crate-local test code.",
        "evidence": "The production-reference facade test no longer calls the helper; parameter-search keeps internal domain-separation coverage.",
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def production_reference_facade_section() -> str:
    source = PRODUCTION_REFERENCE_FACADE.read_text(encoding="utf-8")
    return source.split("#[cfg(test)]", 1)[0]


def facade_exports_symbol(facade: str, symbol: str) -> bool:
    return re.search(rf"(?<![A-Za-z0-9_]){re.escape(symbol)}(?![A-Za-z0-9_])", facade) is not None


def closure_for_surface(surface: dict[str, Any]) -> dict[str, Any]:
    surface_id = surface["id"]
    decision = SURFACE_DECISIONS.get(surface_id) or SAFE_PUBLIC_DECISIONS.get(surface_id)
    if decision is None:
        decision = {
            "decision": ["needs test"],
            "closure_action": surface["required_boundary"],
            "acceptance": "Preserve the required boundary from the source artifact.",
        }
    return {
        "id": surface_id,
        "symbol": surface["symbol"],
        "line": surface["declaration"]["line"],
        "classification": surface["classification"],
        "materializes": surface["materializes"],
        "current_exposure": surface["exposure"],
        "decision": decision["decision"],
        "closure_action": decision["closure_action"],
        "surface_acceptance_criteria": decision["acceptance"],
        "blocker": decision.get("blocker"),
        "required_wrapper_tests": [
            {
                "id": test_id,
                "rust_test": TEST_CONTRACTS[test_id]["rust_test"],
            }
            for test_id in surface["wrapper_tests"]
        ],
    }


def build_payload() -> dict[str, Any]:
    source = load_json(SOURCE_JSON)
    facade = production_reference_facade_section()
    closures = [closure_for_surface(surface) for surface in source["surfaces"]]
    risky_closures = [
        item
        for item in closures
        if item["id"] in SURFACE_DECISIONS
    ]
    source_surface_symbols = [surface["symbol"] for surface in source["surfaces"]]
    withheld_selected_material_symbols = [
        symbol
        for symbol in SELECTED_MATERIAL_FACADE_WITHHELD_SYMBOLS
        if not facade_exports_symbol(facade, symbol)
    ]
    selected_material_symbols_still_exported_by_facade = [
        symbol
        for symbol in SELECTED_MATERIAL_FACADE_WITHHELD_SYMBOLS
        if facade_exports_symbol(facade, symbol)
    ]
    remaining_source_surface_symbols_exported_by_facade = [
        symbol
        for symbol in source_surface_symbols
        if facade_exports_symbol(facade, symbol)
        and symbol not in SELECTED_MATERIAL_FACADE_WITHHELD_SYMBOLS
    ]
    blockers_remaining = [
        {
            "id": "lower_reference_direct_pub_helper_visibility",
            "status": "remaining",
            "closes_when": "remaining selected-material helper structs/functions in tools/parameter-search/src/production_reference.rs are crate-private/proof-only/test-only",
            "covered_by": ["production_reviewed_fq12_helper_visibility_compile_guard"],
        },
        {
            "id": "production_reference_facade_timing_binary_uses_withheld_helper",
            "status": "remaining",
            "closes_when": "tools/production-reference/src/bin/production_reference_facade_timing.rs stops calling reviewed_fq12_pack_masked_witness_state18_from_parts through the production-reference facade",
            "covered_by": ["cargo test --manifest-path tools/production-reference/Cargo.toml"],
        },
    ]
    return {
        "schema": "Descender.pre_signature_projection_helper_api_export_boundary_closure.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "helper_api_export_boundary_facade_closed_lower_reference_residual",
        "deployment_ready": False,
        "source_artifact": rel(SOURCE_JSON),
        "source_status": source["status"],
        "target": source["target"],
        "source_counts": {
            "surface_count": source["evidence_summary"]["surface_count"],
            "risky_public_or_exportable_surface_count": source[
                "risky_public_or_exportable_surface_count"
            ],
            "wrapper_tests_required": len(TEST_CONTRACTS),
            "residual_blockers": len(blockers_remaining),
        },
        "closure_decision": {
            "decision_artifact_complete": True,
            "production_reference_facade_glob_reexport_removed": (
                "pub use descender_parameter_search::production_reference::*" not in facade
            ),
            "production_reference_facade_selected_material_exports_closed": (
                not selected_material_symbols_still_exported_by_facade
            ),
            "helper_api_residual_closed_by_artifact_only": False,
            "why": (
                "This artifact now records the Rust wrapper/reject/digest-only tests and the "
                "production-reference curated facade. The remaining implementation residual is "
                "direct lower-crate public helper visibility plus one diagnostic binary that "
                "still calls a withheld helper through the facade."
            ),
        },
        "production_reference_facade_boundary": {
            "facade": rel(PRODUCTION_REFERENCE_FACADE),
            "glob_reexport_present": (
                "pub use descender_parameter_search::production_reference::*" in facade
            ),
            "withheld_selected_material_symbols": withheld_selected_material_symbols,
            "selected_material_symbols_still_exported_by_facade": selected_material_symbols_still_exported_by_facade,
            "remaining_source_surface_symbols_exported_by_facade": remaining_source_surface_symbols_exported_by_facade,
        },
        "wrapper_tests_required": [
            {"id": test_id, **contract}
            for test_id, contract in TEST_CONTRACTS.items()
        ],
        "surface_closure_matrix": closures,
        "risky_surface_closure_matrix": risky_closures,
        "low_risk_rust_changes_applied": LOW_RISK_RUST_CHANGES_APPLIED,
        "low_risk_rust_changes_proposed_not_applied": [],
        "blockers_closed_by_this_artifact": [
            {
                "id": "production_reference_facade_glob_reexport",
                "status": "closed",
                "evidence": "tools/production-reference/src/lib.rs now uses an explicit curated re-export list instead of `pub use descender_parameter_search::production_reference::*`",
            },
            {
                "id": "production_reference_facade_selected_material_helpers",
                "status": "closed",
                "evidence": "production_reference_facade_excludes_selected_material_helpers asserts the facade withholds all selected-material helper symbols listed in production_reference_facade_boundary.withheld_selected_material_symbols",
            },
            {
                "id": "per_surface_boundary_decision",
                "status": "closed",
                "evidence": "surface_closure_matrix assigns closure_action, decision tags, wrapper tests, and acceptance criteria to all source surfaces",
            },
            {
                "id": "wrapper_test_contract_names",
                "status": "closed",
                "evidence": "wrapper_tests_required lists exact Rust test names and acceptance criteria",
            },
            {
                "id": "wrapper_tests_missing",
                "status": "closed",
                "evidence": "all eight exact Rust wrapper tests pass under `cargo test --manifest-path tools/parameter-search/Cargo.toml production_reviewed_fq12 --lib`",
            },
            {
                "id": "reject_payload_shape",
                "status": "closed",
                "evidence": "wrong-pad, malformed-opening, and carrier Export contract tests observe the opaque `reject_fail_closed_invalid_opening` reason and no selected-material debug markers",
            },
            {
                "id": "debug_or_serde_selected_material",
                "status": "closed",
                "evidence": "public account state, lifecycle snapshot/restore, post-spend transition, and accepting-certificate tests enforce digest-only public serialization and single post-acceptance selected-material export",
            },
        ],
        "blockers_remaining": blockers_remaining,
        "acceptance_to_close_helper_api_export_boundary": [
            "Apply visibility/API changes or feature gates for every surface tagged needs visibility restriction or requires Rust API redesign.",
            "Implement and pass all eight exact Rust wrapper tests listed in wrapper_tests_required.",
            "Confirm the production-reference facade does not glob-reexport selected-material helper structs/functions.",
            "Confirm public state, lifecycle, restore, target-chain DB, spent-ledger, diagnostics, and reject paths stay digest/commitment-only or opaque Err.",
        ],
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
            "production_reference": rel(PRODUCTION_REFERENCE),
            "production_reference_facade": rel(PRODUCTION_REFERENCE_FACADE),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Helper/API Export Boundary Closure",
        "",
        f"Status: `{payload['status']}`",
        f"Deployment ready: `{payload['deployment_ready']}`",
        f"Source artifact: `{payload['source_artifact']}`",
        "",
        "## Counts",
        "",
        "| Item | Count |",
        "| --- | ---: |",
    ]
    for key, value in payload["source_counts"].items():
        lines.append(f"| `{key}` | {value} |")

    lines.extend(
        [
            "",
            "## Wrapper Tests Required",
            "",
            "| Contract | Rust test | Acceptance criteria |",
            "| --- | --- | --- |",
        ]
    )
    for test in payload["wrapper_tests_required"]:
        criteria = "<br>".join(test["acceptance_criteria"])
        lines.append(f"| `{test['id']}` | `{test['rust_test']}` | {criteria} |")

    facade = payload["production_reference_facade_boundary"]
    lines.extend(
        [
            "",
            "## Production-Reference Facade Boundary",
            "",
            f"- Facade: `{facade['facade']}`",
            f"- Glob re-export present: `{facade['glob_reexport_present']}`",
            f"- Selected-material symbols still exported by facade: `{facade['selected_material_symbols_still_exported_by_facade']}`",
            "",
            "Withheld selected-material symbols:",
            "",
        ]
    )
    for symbol in facade["withheld_selected_material_symbols"]:
        lines.append(f"- `{symbol}`")
    lines.extend(["", "Remaining source-surface symbols exported by facade:", ""])
    for symbol in facade["remaining_source_surface_symbols_exported_by_facade"]:
        lines.append(f"- `{symbol}`")

    lines.extend(
        [
            "",
            "## Risky Surface Closure Matrix",
            "",
            "| Surface | Line | Decision | Closure action | Required tests |",
            "| --- | ---: | --- | --- | --- |",
        ]
    )
    for surface in payload["risky_surface_closure_matrix"]:
        tests = ", ".join(
            f"`{test['rust_test']}`" for test in surface["required_wrapper_tests"]
        )
        decisions = ", ".join(f"`{decision}`" for decision in surface["decision"])
        lines.append(
            f"| `{surface['symbol']}` | {surface['line']} | {decisions} | "
            f"{surface['closure_action']} | {tests} |"
        )

    lines.extend(
        [
            "",
            "## Remaining Public/Allowed Surfaces",
            "",
            "| Surface | Line | Decision | Acceptance |",
            "| --- | ---: | --- | --- |",
        ]
    )
    for surface in payload["surface_closure_matrix"]:
        if surface["id"] in SURFACE_DECISIONS:
            continue
        decisions = ", ".join(f"`{decision}`" for decision in surface["decision"])
        lines.append(
            f"| `{surface['symbol']}` | {surface['line']} | {decisions} | "
            f"{surface['surface_acceptance_criteria']} |"
        )

    lines.extend(
        [
            "",
            "## Applied Low-Risk Rust Changes",
            "",
        ]
    )
    for change in payload["low_risk_rust_changes_applied"]:
        lines.extend(
            [
                f"### `{change['id']}`",
                "",
                f"Symbol: `{change['symbol']}`",
                "",
                change["change"],
                "",
                f"Evidence: {change['evidence']}",
                "",
            ]
        )

    lines.extend(
        [
            "## Proposed Low-Risk Rust Changes Not Applied",
            "",
        ]
    )
    for proposal in payload["low_risk_rust_changes_proposed_not_applied"]:
        lines.extend(
            [
                f"### `{proposal['id']}`",
                "",
                f"Symbol: `{proposal['symbol']}`",
                "",
                proposal["proposal"],
                "",
                f"Risk: {proposal['risk']}",
                "",
            ]
        )

    lines.extend(
        [
            "## Blockers",
            "",
            "Closed by this artifact:",
            "",
        ]
    )
    for blocker in payload["blockers_closed_by_this_artifact"]:
        lines.append(f"- `{blocker['id']}`: {blocker['evidence']}")
    lines.extend(["", "Remaining:", ""])
    for blocker in payload["blockers_remaining"]:
        tests = ", ".join(f"`{test}`" for test in blocker["covered_by"])
        lines.append(f"- `{blocker['id']}`: {blocker['closes_when']} Covered by {tests}.")

    lines.extend(
        [
            "",
            "## Acceptance To Close Boundary",
            "",
        ]
    )
    for item in payload["acceptance_to_close_helper_api_export_boundary"]:
        lines.append(f"- {item}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))


if __name__ == "__main__":
    main()
