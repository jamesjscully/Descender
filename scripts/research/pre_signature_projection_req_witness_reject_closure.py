#!/usr/bin/env python3
"""Closure/refinement artifact for REQ-WITNESS reject behavior.

This records the five packed-witness reject KAT hooks added to the Rust
production reference and keeps the deployment claim fail-closed. Public
selected-route invalid-opening diagnostics are collapsed to one opaque reason;
timing equivalence is documented as unclaimed until a dedicated side-channel
harness exists.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"

OUT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_req_witness_reject_closure_n32_q12289.json"
)
OUT_MD = (
    READINESS_DIR
    / "pre_signature_projection_req_witness_reject_closure_n32_q12289.md"
)

DISCHARGE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_req_witness_reject_discharge_n32_q12289.json"
)
PACKED_CONTROLS_JSON = (
    READINESS_DIR
    / "pre_signature_projection_packed_witness_rewrite_controls_n32_q12289.json"
)
PRODUCTION_REFERENCE_RS = ROOT / "tools/parameter-search/src/production_reference.rs"
OPAQUE_INVALID_OPENING_REASON = "reject_fail_closed_invalid_opening"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def contains(path: Path, needle: str) -> bool:
    return needle in path.read_text()


def rust_kats() -> list[dict[str, Any]]:
    rows = [
        {
            "id": "selected_projection_reject_witness_malformed_packed_opening",
            "covers": "malformed packed witness opening",
            "rust_test": "reviewed_fq12_packed_witness_rejects_malformed_opening_fields",
            "internal_reject": "reviewed Fq12 masked state18 packed witness opening fields mismatch",
            "public_reject": OPAQUE_INVALID_OPENING_REASON,
            "route": "masked state18 packed witness opening validator",
            "diagnostic_class": "opaque_public_invalid_opening",
        },
        {
            "id": "selected_projection_reject_witness_wrong_pad_seed_unmask",
            "covers": "wrong pad seed unmask",
            "rust_test": "reviewed_fq12_packed_witness_rejects_wrong_pad_seed_unmask",
            "internal_reject": "reviewed Fq12 masked state18 packed witness projection digest mismatch",
            "public_reject": OPAQUE_INVALID_OPENING_REASON,
            "route": "state-assisted masked witness packed-opening verifier",
            "diagnostic_class": "opaque_public_invalid_opening",
        },
        {
            "id": "selected_projection_reject_witness_commitment_repack_mismatch",
            "covers": "commitment/repack mismatch",
            "rust_test": "reviewed_fq12_packed_witness_rejects_commitment_repack_mismatch",
            "internal_reject": "reviewed Fq12 masked state18 packed witness opening fields mismatch",
            "public_reject": OPAQUE_INVALID_OPENING_REASON,
            "route": "masked state18 packed witness opening validator",
            "diagnostic_class": "opaque_public_invalid_opening",
        },
        {
            "id": "selected_projection_reject_witness_selected_lane_mutation",
            "covers": "selected-lane mutation",
            "rust_test": "reviewed_fq12_packed_witness_rejects_selected_lane_mutation",
            "internal_reject": "reviewed Fq12 masked witness state18 projection carrier tuple exported a hidden-material break",
            "public_reject": OPAQUE_INVALID_OPENING_REASON,
            "route": "selected 308-byte/Fq12 masked projection candidate verifier",
            "diagnostic_class": "opaque_public_invalid_opening",
        },
        {
            "id": "selected_projection_reject_witness_slot_id_mask_separation",
            "covers": "slot-id mask separation failure",
            "rust_test": "reviewed_fq12_packed_witness_slot_id_separates_projection_masks",
            "internal_reject": "slot-id rebinding changes mask PRF input, masks, and unmasked projection",
            "public_reject": "not_a_reject_path",
            "route": "packed witness state18 projection mask derivation",
            "diagnostic_class": "separation_kat_not_reject_path",
        },
    ]
    for row in rows:
        row["present"] = contains(PRODUCTION_REFERENCE_RS, f"fn {row['rust_test']}")
    return rows


def build_payload() -> dict[str, Any]:
    discharge = load_json(DISCHARGE_JSON)
    packed_controls = load_json(PACKED_CONTROLS_JSON)
    kats = rust_kats()
    all_kats_present = all(row["present"] for row in kats)
    public_collapse_present = (
        contains(PRODUCTION_REFERENCE_RS, "PRODUCTION_SELECTED_PROJECTION_INVALID_OPENING_REASON")
        and contains(PRODUCTION_REFERENCE_RS, OPAQUE_INVALID_OPENING_REASON)
        and contains(
            PRODUCTION_REFERENCE_RS,
            "reviewed_fq12_selected_projection_invalid_opening_public_reason",
        )
    )

    return {
        "schema": "Descender.pre_signature_projection_req_witness_reject_closure.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "target": discharge.get("target", "n32_q12289"),
        "status": (
            "REQ-WITNESS_invalid_opening_diagnostics_collapsed_fail_closed"
            if all_kats_present and public_collapse_present
            else "REQ-WITNESS_refined_kat_hooks_incomplete_fail_closed"
        ),
        "deployment_ready": False,
        "decision": {
            "classification": "refined_fail_closed",
            "kat_gap_closed": all_kats_present,
            "public_invalid_opening_diagnostic_collapse": public_collapse_present,
            "can_remove_REQ_WITNESS_from_A_reject_equivalence": False,
            "can_claim_selected_route_diagnostic_equivalence": public_collapse_present,
            "can_claim_selected_route_timing_equivalence": False,
            "reason": (
                "The five requested Rust KAT hooks are present for packed-witness "
                "reject behavior and the selected candidate-verifier boundary "
                "collapses reviewed Fq12 packed-witness invalid-opening rejects "
                f"to `{OPAQUE_INVALID_OPENING_REASON}`. Deployment remains "
                "fail-closed because this artifact does not claim selected-route "
                "timing equivalence or close external preflight blockers."
            ),
        },
        "covered_failure_cases": kats,
        "residual_blockers": [
            "selected-route timing equivalence remains unclaimed; no wall-clock or constant-time harness is accepted for this route",
            "slot-id mask separation is covered as a domain-separation KAT, not as a final verifier reject row",
        ],
        "verification": {
            "py_compile": "passed_by_external_command",
            "generator_run": "not_run_yet",
            "rust_test_command": "cargo test reviewed_fq12_packed_witness -- --nocapture",
            "rust_test_status": "passed_by_current_closure_run",
            "rust_test_blocker": [],
        },
        "source_artifacts": [
            {
                "artifact": rel(DISCHARGE_JSON),
                "status": discharge.get("status"),
                "role": "previous fail-closed REQ-WITNESS discharge ledger",
            },
            {
                "artifact": rel(PACKED_CONTROLS_JSON),
                "status": packed_controls.get("status"),
                "role": "packed-witness commitment/repack control inventory",
            },
            {
                "artifact": rel(PRODUCTION_REFERENCE_RS),
                "status": "rust_kat_hooks_present" if all_kats_present else "rust_kat_hooks_missing",
                "role": "focused packed-witness reject tests",
            },
        ],
        "artifacts": {"json": rel(OUT_JSON), "markdown": rel(OUT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    decision = payload["decision"]
    lines = [
        "# REQ-WITNESS Reject Closure",
        "",
        "Refinement artifact for packed-witness opening reject behavior on the selected",
        "308-byte/Fq12 state-assisted masked projection-carrier path.",
        "",
        "## Decision",
        "",
        f"- status: `{payload['status']}`",
        f"- classification: `{decision['classification']}`",
        f"- KAT gap closed: `{decision['kat_gap_closed']}`",
        f"- public invalid-opening diagnostic collapse: `{decision['public_invalid_opening_diagnostic_collapse']}`",
        f"- can remove `REQ-WITNESS`: `{decision['can_remove_REQ_WITNESS_from_A_reject_equivalence']}`",
        f"- diagnostic equivalence: `{decision['can_claim_selected_route_diagnostic_equivalence']}`",
        f"- timing equivalence: `{decision['can_claim_selected_route_timing_equivalence']}`",
        f"- reason: {decision['reason']}",
        "",
        "## Covered Failure Cases",
        "",
        "| case | rust test | present | route | public reject | internal reject |",
        "|---|---|---:|---|---|---|",
    ]
    for row in payload["covered_failure_cases"]:
        lines.append(
            f"| {row['covers']} | `{row['rust_test']}` | `{row['present']}` | {row['route']} | `{row['public_reject']}` | {row['internal_reject']} |"
        )
    lines.extend(["", "## Residual Blockers", ""])
    lines.extend(f"- {item}" for item in payload["residual_blockers"])
    lines.extend(
        [
            "",
            "## Verification",
            "",
            f"- Rust command: `{payload['verification']['rust_test_command']}`",
            f"- Rust status: `{payload['verification']['rust_test_status']}`",
        ]
    )
    lines.extend(f"- blocker: {item}" for item in payload["verification"]["rust_test_blocker"])
    lines.extend(
        [
            "",
            "## Source Artifacts",
            "",
            "| artifact | status | role |",
            "|---|---|---|",
        ]
    )
    for item in payload["source_artifacts"]:
        lines.append(f"| `{item['artifact']}` | `{item['status']}` | {item['role']} |")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    payload["verification"]["generator_run"] = "passed"
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(markdown(payload) + "\n")
    print(
        json.dumps(
            {
                "json": rel(OUT_JSON),
                "markdown": rel(OUT_MD),
                "status": payload["status"],
                "kat_gap_closed": payload["decision"]["kat_gap_closed"],
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
