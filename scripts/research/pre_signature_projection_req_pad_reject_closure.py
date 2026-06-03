#!/usr/bin/env python3
"""REQ-PAD selected-route reject KAT closure artifact.

This script records the five concrete REQ-PAD negative KATs that close the
pad-opening reject blocker from the selected 308-byte/Fq12 candidate verifier
surface.  It deliberately separates KAT closure from the broader timing-policy
claim left open by the parent reject-equivalence discharge.
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
    / "pre_signature_projection_req_pad_reject_closure_n32_q12289.json"
)
OUT_MD = (
    READINESS_DIR / "pre_signature_projection_req_pad_reject_closure_n32_q12289.md"
)

REQ_PAD_DISCHARGE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_req_pad_reject_discharge_n32_q12289.json"
)
REJECT_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_discharge_n32_q12289.json"
)
PRODUCTION_REFERENCE_RS = ROOT / "tools/parameter-search/src/production_reference.rs"
PRODUCTION_REFERENCE_TESTS_RS = ROOT / "tools/production-reference/src/lib.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def artifact(path: Path, role: str) -> dict[str, Any]:
    payload = load_json(path)
    return {
        "artifact": rel(path),
        "schema": payload.get("schema"),
        "status": (
            payload.get("status")
            or payload.get("closure_status")
            or payload.get("deployment_ready")
            or "present_no_status_field"
        ),
        "role": role,
    }


def contains(path: Path, needle: str) -> bool:
    return needle in path.read_text()


def kat_rows() -> list[dict[str, Any]]:
    common = {
        "entrypoint": "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
        "state_mutation_assertion": "account clone compared equal after candidate-verifier reject and after soundness-certificate reject",
        "partial_export_assertion": "verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate returns Err; no ProductionReviewedFq12MaskedProjectionSoundnessCertificate or packed opening material is returned",
        "public_material_policy": [
            "no pad_seed bytes returned",
            "no PRF block output returned",
            "no packed witness opening returned",
            "no account_witness_projection_opening_frame returned",
            "no projection mask or carrier lane material returned",
        ],
    }
    return [
        {
            **common,
            "id": "REQ_PAD_KAT_001_wrong_pad_seed",
            "failure_case": "PAD-STREAM-RESPONSE-MISMATCH",
            "mutation": "flip pad_seed[0] for an otherwise accepting selected 308-byte/Fq12 envelope",
            "expected_reason": "reviewed Fq12 masked state18 packed witness projection digest mismatch",
            "source_test_label": "wrong pad seed",
        },
        {
            **common,
            "id": "REQ_PAD_KAT_002_wrong_public_pad_statement",
            "failure_case": "PAD-DIGEST-MISMATCH",
            "mutation": "flip statement.pad_opening_statement.digest[0] and recompute the outer hidden-source statement digest",
            "expected_reason": "reviewed Fq12 state-assisted hidden-source statement mismatch",
            "source_test_label": "wrong public pad statement",
        },
        {
            **common,
            "id": "REQ_PAD_KAT_003_observed_block_mutation",
            "failure_case": "PAD-DOMAIN-SLOT-LANE-COUNTER",
            "mutation": "toggle one public observed_blocks bit, recompute observed_block_count, and recompute the outer statement digest",
            "expected_reason": "reviewed Fq12 state-assisted hidden-source statement mismatch",
            "source_test_label": "observed block mutation",
        },
        {
            **common,
            "id": "REQ_PAD_KAT_004_local_opening_mutation",
            "failure_case": "PAD-MISSING-MALFORMED",
            "mutation": "flip the first byte in the local opening section of the selected signature before rewrapping the 308-byte projection-carrier envelope",
            "expected_reason": "public pad-opening mismatch for block 2 in round 11",
            "source_test_label": "local opening mutation",
        },
        {
            **common,
            "id": "REQ_PAD_KAT_005_committed_opening_mismatch",
            "failure_case": "PAD-DIGEST-MISMATCH",
            "mutation": "flip opening.pad_seed_digest[0] while preserving the public account slot",
            "expected_reason": "state-assisted pad seed digest mismatch",
            "source_test_label": "committed opening mismatch",
        },
    ]


def evidence_checks() -> list[dict[str, Any]]:
    checks = [
        (
            "closure_rust_test_present",
            PRODUCTION_REFERENCE_TESTS_RS,
            "fn req_pad_reject_kats_preserve_state_and_export_no_partial_material()",
            "single focused Rust test implements all five selected-route REQ-PAD KATs",
        ),
        (
            "candidate_verifier_entrypoint_present",
            PRODUCTION_REFERENCE_RS,
            "pub fn verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            "selected 308-byte/Fq12 candidate verifier entrypoint exists",
        ),
        (
            "soundness_certificate_no_partial_material_gate_present",
            PRODUCTION_REFERENCE_RS,
            "pub fn verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate",
            "partial material is available only on certificate success; closure test asserts Err for every reject",
        ),
        (
            "state_assisted_pad_digest_check_present",
            PRODUCTION_REFERENCE_RS,
            '"state-assisted pad seed digest mismatch"',
            "committed opening mismatch rejects before acceptance",
        ),
        (
            "pad_statement_witness_check_present",
            PRODUCTION_REFERENCE_RS,
            '"reviewed Fq12 pad-opening statement witness mismatch"',
            "public pad statement witness binding exists under the selected tuple",
        ),
    ]
    return [
        {
            "id": check_id,
            "path": rel(path),
            "needle": needle,
            "present": contains(path, needle),
            "role": role,
        }
        for check_id, path, needle, role in checks
    ]


def parent_req_pad_status(parent: dict[str, Any]) -> dict[str, Any] | None:
    for row in parent.get("failure_class_discharge", []):
        if row.get("id") == "REQ-PAD":
            return {
                "classification": row.get("classification"),
                "basis": row.get("basis"),
                "blockers": row.get("blockers", []),
                "selected_route_tests_needed": row.get("selected_route_tests_needed", []),
                "timing_harness_needed": row.get("timing_harness_needed", []),
            }
    return None


def build_payload() -> dict[str, Any]:
    req_pad_discharge = load_json(REQ_PAD_DISCHARGE_JSON)
    reject_discharge = load_json(REJECT_DISCHARGE_JSON)
    checks = evidence_checks()
    missing = [row["id"] for row in checks if not row["present"]]
    kats = kat_rows()
    implemented = contains(
        PRODUCTION_REFERENCE_TESTS_RS,
        "req_pad_reject_kats_preserve_state_and_export_no_partial_material",
    )
    all_closed = implemented and not missing

    decision = {
        "classification": (
            "REQ_PAD_five_KAT_blocker_closed_timing_residual"
            if all_closed
            else "REQ_PAD_five_KAT_blocker_not_closed"
        ),
        "five_required_kats_implemented": all_closed,
        "can_remove_five_kat_blocker": all_closed,
        "can_claim_selected_route_pad_reject_constant_time": False,
        "can_remove_req_pad_from_A_reject_equivalence_without_timing_review": False,
        "reason": (
            "The selected-route Rust KAT now covers wrong pad seed, wrong public "
            "pad statement, observed block mutation, local opening mutation, and "
            "committed opening mismatch.  Each row asserts no account-state "
            "mutation and no soundness-certificate/partial-opening export.  The "
            "older timing-equivalence claim remains separate and is not asserted "
            "by this artifact."
        ),
    }

    return {
        "schema": "Descender.pre_signature_projection_req_pad_reject_closure.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": decision["classification"],
        "target": req_pad_discharge.get("target"),
        "deployment_ready": False,
        "decision": decision,
        "kat_count": len(kats),
        "kats": kats,
        "evidence": {
            "code_presence_checks": checks,
            "missing_code_presence_checks": missing,
            "rust_test": {
                "path": rel(PRODUCTION_REFERENCE_TESTS_RS),
                "name": "req_pad_reject_kats_preserve_state_and_export_no_partial_material",
                "command": "cargo test req_pad_reject_kats_preserve_state_and_export_no_partial_material",
                "crate": "tools/production-reference",
            },
            "parent_req_pad_discharge": req_pad_discharge.get("decision"),
            "parent_reject_equivalence_req_pad_row": parent_req_pad_status(
                reject_discharge
            ),
        },
        "residuals": [
            "Run the broader selected-route timing harness before claiming constant-time reject equivalence for REQ-PAD.",
            "Propagate this KAT closure into the parent seven-class reject-equivalence ledger if that ledger is regenerated.",
        ],
        "source_artifacts": [
            artifact(REQ_PAD_DISCHARGE_JSON, "previous REQ-PAD fail-closed discharge"),
            artifact(REJECT_DISCHARGE_JSON, "parent seven-class reject-equivalence ledger"),
        ],
        "source_files_reviewed": [
            "scripts/research/pre_signature_projection_req_pad_reject_discharge.py",
            "research/cryptanalysis/signature_readiness_20260516/pre_signature_projection_req_pad_reject_discharge_n32_q12289.json",
            "scripts/research/pre_signature_projection_reject_equivalence_discharge.py",
            "tools/parameter-search/src/production_reference.rs",
            "tools/production-reference/src/lib.rs",
        ],
        "artifacts": {"json": rel(OUT_JSON), "markdown": rel(OUT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    decision = payload["decision"]
    lines = [
        "# REQ-PAD Reject Closure",
        "",
        "## Decision",
        "",
        f"- status: `{payload['status']}`",
        f"- five required KATs implemented: `{decision['five_required_kats_implemented']}`",
        f"- can remove five-KAT blocker: `{decision['can_remove_five_kat_blocker']}`",
        f"- can claim selected-route constant-time reject equivalence: `{decision['can_claim_selected_route_pad_reject_constant_time']}`",
        f"- reason: {decision['reason']}",
        "",
        "## KATs",
        "",
        "| id | mutation | expected reject |",
        "|---|---|---|",
    ]
    for row in payload["kats"]:
        lines.append(
            f"| `{row['id']}` | {row['mutation']} | `{row['expected_reason']}` |"
        )

    lines.extend(
        [
            "",
            "Every KAT asserts account state equality after rejection and asserts the",
            "soundness-certificate path also rejects, so no packed opening, projection",
            "mask, carrier lane, account-opening frame, or pad-seed material is exported.",
            "",
            "## Evidence",
            "",
            f"- Rust test: `{payload['evidence']['rust_test']['name']}` in `{payload['evidence']['rust_test']['path']}`",
            f"- command: `{payload['evidence']['rust_test']['command']}` from `{payload['evidence']['rust_test']['crate']}`",
            f"- missing code checks: `{payload['evidence']['missing_code_presence_checks']}`",
            "",
            "## Residuals",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["residuals"])
    lines.extend(
        [
            "",
            "## Source Artifacts",
            "",
            "| artifact | status | role |",
            "|---|---|---|",
        ]
    )
    for row in payload["source_artifacts"]:
        lines.append(f"| `{row['artifact']}` | `{row['status']}` | {row['role']} |")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(markdown(payload))


if __name__ == "__main__":
    main()
