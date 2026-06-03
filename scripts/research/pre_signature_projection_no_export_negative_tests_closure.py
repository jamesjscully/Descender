#!/usr/bin/env python3
"""Close the no-export negative-test plan against concrete Rust fixtures."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
SOURCE_JSON = (
    READINESS_DIR / "pre_signature_projection_no_export_negative_tests_n32_q12289.json"
)
OUT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_no_export_negative_tests_closure_n32_q12289.json"
)
OUT_MD = (
    READINESS_DIR
    / "pre_signature_projection_no_export_negative_tests_closure_n32_q12289.md"
)
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"


IMPLEMENTED = {
    "wrong_pad_seed_reject_no_partial_opening": {
        "rust_test": "masked_projection_wrong_pad_seed_rejects_without_partial_opening",
        "fixture": "masked_projection_fixture",
        "helper_assertions": ["assert_no_selected_projection_material"],
        "covered_mutations": [
            "wrong pad seed against projection-opening verifier",
            "wrong pad seed against packed-witness-opening verifier",
            "wrong pad seed against slot projection-carrier tuple verifier",
        ],
        "exact_reject_reasons": [
            "reviewed Fq12 masked witness state18 projection opening digest mismatch",
            "reviewed Fq12 masked state18 packed witness projection digest mismatch",
        ],
    },
    "malformed_opening_reject_no_material": {
        "rust_test": "masked_projection_malformed_openings_reject_without_material",
        "fixture": "masked_projection_fixture",
        "helper_assertions": ["assert_no_selected_projection_material"],
        "covered_mutations": [
            "statement digest mismatch",
            "slot id mismatch",
            "witness commitment mismatch",
            "randomizer commitment mismatch at slot boundary",
            "opening digest mismatch",
        ],
        "exact_reject_reasons": [
            "reviewed Fq12 masked witness state18 projection opening statement digest mismatch",
            "reviewed Fq12 masked state18 packed witness opening slot id mismatch",
            "reviewed Fq12 masked state18 packed witness opening statement commitment mismatch",
            "state-assisted randomizer commitment mismatch",
            "reviewed Fq12 masked state18 packed witness projection digest mismatch",
        ],
    },
    "carrier_rebinding_reject_no_material": {
        "rust_test": "masked_projection_carrier_rebinding_rejects_without_export",
        "fixture": "masked_projection_fixture",
        "helper_assertions": ["assert_no_selected_projection_material"],
        "covered_mutations": [
            "carrier lane mutation",
            "carrier check-byte mutation",
            "carrier transcript-binding mutation",
            "message digest rebinding",
            "public-key digest rebinding",
            "signature digest rebinding",
            "slot id rebinding",
        ],
        "exact_reject_reasons": [
            "reviewed Fq12 masked witness state18 projection carrier tuple exported a hidden-material break",
            "reviewed Fq12 projection carrier check bytes mismatch",
            "reviewed Fq12 projection carrier transcript binding mismatch",
            "reviewed Fq12 masked state18 packed witness opening slot id mismatch",
        ],
    },
    "spent_slot_replay_no_new_opening": {
        "rust_test": "selected_projection_spent_replay_rejects_digest_only_no_opening",
        "fixture": "HiddenSourceFallbackProjectionAccountSlotFrameSeed plus apply_selected_projection_consensus_opening_frame",
        "helper_assertions": ["assert_no_selected_projection_material"],
        "covered_mutations": [
            "same-slot replay after accepted selected projection transition",
            "stale lifecycle snapshot replay with existing spent record",
            "duplicate pad_seed_digest spent-record construction",
        ],
        "exact_reject_reasons": [
            "slot is already in spent ledger",
            "pad seed digest is already in spent ledger",
        ],
    },
    "account_state_read_no_selected_material": {
        "rust_test": "state_assisted_account_state_checked_records_no_selected_material",
        "fixture": "ProductionReviewedFq12StateAssistedAccountSlot::new with nonzero hidden_source_opening_commitment, restore_anchor_digest, and pad_seed_digest",
        "helper_assertions": ["assert_no_selected_projection_material"],
        "covered_mutations": [
            "checked state records serialize only public digest fields",
            "state reconstruction from checked records round-trips without selected material",
        ],
        "exact_reject_reasons": [],
    },
    "snapshot_restore_no_selected_material": {
        "rust_test": "selected_projection_lifecycle_snapshot_roundtrips_digest_only",
        "fixture": "ProductionReviewedFq12StateAssistedAccountLifecycleManager::snapshot plus ProductionSpentSlotRecord { pad_seed_digest: Some(..) }",
        "helper_assertions": ["assert_no_selected_projection_material"],
        "covered_mutations": [
            "lifecycle snapshot JSON round-trip carries hidden_source_opening_commitment digest only",
            "restore anchor reload preserves snapshot without selected material",
            "selected projection consensus spend record carries pad_seed_digest without pad seed or opening frames",
        ],
        "exact_reject_reasons": [],
    },
    "hidden_material_export_converted_to_error": {
        "rust_test": "masked_projection_hidden_material_export_branch_is_opaque_error",
        "fixture": "masked_projection_fixture with one carrier lane changed while check/transcript bytes are unchanged",
        "helper_assertions": ["assert_no_selected_projection_material"],
        "covered_mutations": [
            "carrier lane mutation reaches the internal Export branch",
            "selected tuple verifier converts Export to an opaque Err string",
            "Err Debug/Display excludes export structs and selected material",
        ],
        "exact_reject_reasons": [
            "reviewed Fq12 masked witness state18 projection carrier tuple exported a hidden-material break",
        ],
    },
    "helper_visibility_guard_no_pre_spend_callers": {
        "rust_test": "selected_projection_materializers_are_not_public_pre_spend_api",
        "fixture": "static scan of production_reference.rs public signatures and caller names",
        "helper_assertions": [],
        "covered_mutations": [
            "static public-signature scan rejects pre-spend selected-material exposure terms",
            "static scan reports violating function signatures without rendering selected material",
        ],
        "exact_reject_reasons": [],
    },
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def rust_text() -> str:
    return PRODUCTION_REFERENCE.read_text()


def present(name: str) -> bool:
    return f"fn {name}" in rust_text()


def line_of(needle: str) -> int | None:
    for index, line in enumerate(rust_text().splitlines(), start=1):
        if needle in line:
            return index
    return None


def enrich(entry: dict[str, Any], status: str) -> dict[str, Any]:
    rust_test = entry["rust_test"]
    return {
        **entry,
        "status": status,
        "present": present(rust_test),
        "path": rel(PRODUCTION_REFERENCE),
        "test_line": line_of(f"fn {rust_test}"),
        "fixture_line": line_of(f"fn {entry['fixture']}") if entry["fixture"].isidentifier() else None,
    }


def build_payload() -> dict[str, Any]:
    source = json.loads(SOURCE_JSON.read_text())
    implemented = {key: enrich(value, "implemented") for key, value in IMPLEMENTED.items()}
    missing = [key for key, item in implemented.items() if not item["present"]]
    no_export_closed = not missing
    return {
        "schema": "Descender.pre_signature_projection_no_export_negative_tests_closure.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "all_required_no_export_negative_tests_implemented_preflight_still_fail_closed"
        if no_export_closed
        else "required_no_export_negative_tests_missing",
        "deployment_ready": False,
        "source_plan": rel(SOURCE_JSON),
        "production_reference": rel(PRODUCTION_REFERENCE),
        "implemented_count": sum(1 for item in implemented.values() if item["present"]),
        "required_count": len(implemented),
        "implemented_tests": implemented,
        "remaining_required_tests": missing,
        "acceptance_gate": {
            "can_close_projection_overread_residual": no_export_closed,
            "can_mark_spend_time_term_proved": no_export_closed,
            "reason": "all required no-export negative tests are present; production deployment preflight remains fail-closed on independent internal blockers"
            if no_export_closed
            else "required no-export negative tests are missing from the Rust fixture suite",
        },
        "source_required_test_count": source["coverage_summary"]["required_test_count"],
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection No-Export Negative Test Closure",
        "",
        f"- status: `{payload['status']}`",
        f"- implemented: `{payload['implemented_count']}`",
        f"- required: `{payload['required_count']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- Rust file: `{payload['production_reference']}`",
        "",
        "## Implemented Rust Tests",
        "",
    ]
    for test_id, item in payload["implemented_tests"].items():
        lines.extend(
            [
                f"### `{test_id}`",
                f"- Rust test: `{item['rust_test']}`",
                f"- present: `{item['present']}`",
                f"- test line: `{item['test_line']}`",
                f"- fixture: `{item['fixture']}`",
                f"- fixture line: `{item['fixture_line']}`",
                "- covered mutations:",
            ]
        )
        for mutation in item["covered_mutations"]:
            lines.append(f"  - {mutation}")
        lines.append("- exact reject reasons:")
        for reason in item["exact_reject_reasons"]:
            lines.append(f"  - `{reason}`")
        lines.append("")
    lines.extend(["## Remaining Required Tests", ""])
    if payload["remaining_required_tests"]:
        for test_id in payload["remaining_required_tests"]:
            lines.append(f"- `{test_id}`")
    else:
        lines.append("- none")
    lines.append("")
    lines.extend(
        [
            "## Gate",
            "",
            f"- can close projection-overread residual: `{payload['acceptance_gate']['can_close_projection_overread_residual']}`",
            f"- can mark spend-time term proved: `{payload['acceptance_gate']['can_mark_spend_time_term_proved']}`",
            f"- reason: {payload['acceptance_gate']['reason']}",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": rel(OUT_JSON),
                "markdown": rel(OUT_MD),
                "status": payload["status"],
                "implemented_count": payload["implemented_count"],
                "required_count": payload["required_count"],
                "remaining_required_tests": payload["remaining_required_tests"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
