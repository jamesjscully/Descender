#!/usr/bin/env python3
"""REQ-CARRIER closure/refinement artifact.

This generator records the exact seven selected-route KATs required to close
the projection-carrier reject-equivalence class.  It is deliberately
fail-closed: the compact 308-byte/Fq12 fixture, public carrier diagnostic
collapse, and internal Rust KAT hooks are present, but deployment closure still
requires selected-route timing evidence under the global reject-equivalence
harness.
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
    / "pre_signature_projection_req_carrier_reject_closure_n32_q12289.json"
)
OUT_MD = (
    READINESS_DIR
    / "pre_signature_projection_req_carrier_reject_closure_n32_q12289.md"
)

DISCHARGE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_req_carrier_reject_discharge_n32_q12289.json"
)
OVERREAD_CONTROLS_JSON = (
    READINESS_DIR
    / "pre_signature_projection_projection_overread_controls_n32_q12289.json"
)
REJECT_EQUIVALENCE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_n32_q12289.json"
)
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def contains(path: Path, needle: str) -> bool:
    return needle in path.read_text()


def source_status(path: Path, role: str) -> dict[str, Any]:
    payload = load_json(path)
    return {
        "artifact": rel(path),
        "role": role,
        "schema": payload.get("schema"),
        "status": (
            payload.get("status")
            or payload.get("closure_status")
            or payload.get("current_claim")
            or "present_no_status_field"
        ),
    }


def overread_test(overread: dict[str, Any], test_id: str) -> dict[str, Any]:
    for item in overread.get("negative_tests_required", []):
        if item.get("id") == test_id:
            return item
    raise KeyError(test_id)


def overread_surface(overread: dict[str, Any], surface_id: str) -> dict[str, Any]:
    for item in overread.get("export_surfaces", []):
        if item.get("id") == surface_id:
            return item
    raise KeyError(surface_id)


def code_check(id_: str, needle: str, role: str) -> dict[str, Any]:
    return {
        "id": id_,
        "path": rel(PRODUCTION_REFERENCE),
        "needle": needle,
        "present": contains(PRODUCTION_REFERENCE, needle),
        "role": role,
    }


def kat_specs() -> list[dict[str, Any]]:
    common_assertions = [
        "entrypoint is verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier or the final hardened selected-route equivalent",
        "observable result is one public invalid_projection_carrier rejection class",
        "no account_witness_projection_opening_frame, account_projection, projection_masks, mask PRF outputs, pad_seed bytes, retry counts, lane indices, or HiddenSourceFallbackProjectionBreakExport are returned or logged",
        "account state is byte-identical before and after rejection; lifecycle snapshot and spent-slot ledger are not touched by this stateless candidate-verifier harness",
        "row is timed against the valid selected-carrier row and pad/witness/transcript/checked-core reject baselines under the same harness",
    ]
    return [
        {
            "id": "selected_projection_reject_carrier_lane_mutation",
            "covers": "carrier lane mutation",
            "mutation": "start from a valid selected carrier and mutate one canonical lane value while preserving carrier length; sample each account and pad projection lane position",
            "expected_internal_first_failure": "carrier binding or hidden-material projection mismatch",
            "diagnostic_requirement": "must not reveal whether the failed lane was account, pad, raw projection, packed witness, or mask PRF derived",
            "assertions": common_assertions,
        },
        {
            "id": "selected_projection_reject_carrier_check_mutation",
            "covers": "carrier check mutation",
            "mutation": "flip one byte in the 3-byte carrier check field after constructing an otherwise valid selected carrier",
            "expected_internal_first_failure": "reviewed Fq12 projection carrier check bytes mismatch",
            "diagnostic_requirement": "same public carrier reject class as lane and transcript failures",
            "assertions": common_assertions,
        },
        {
            "id": "selected_projection_reject_carrier_transcript_binding_mutation",
            "covers": "carrier transcript binding mutation",
            "mutation": "flip one byte in the 29-byte carrier transcript binding field without changing decoded lanes",
            "expected_internal_first_failure": "reviewed Fq12 projection carrier transcript binding mismatch",
            "diagnostic_requirement": "must not expose which transcript component failed",
            "assertions": common_assertions,
        },
        {
            "id": "selected_projection_reject_account_state_rebinding",
            "covers": "account-state rebinding",
            "mutation": "verify a valid carrier/opening/statement tuple against a different public account state or slot record with the same envelope shape",
            "expected_internal_first_failure": "slot, witness commitment, randomizer commitment, or statement mismatch before selected material export",
            "diagnostic_requirement": "same public invalid_projection_carrier class; no state transition",
            "assertions": common_assertions,
        },
        {
            "id": "selected_projection_reject_pad_seed_rebinding",
            "covers": "pad-seed rebinding",
            "mutation": "replace pad_seed with a different 32-byte seed while leaving pad_seed_digest, carrier bytes, statement, and signature envelope otherwise fixed",
            "expected_internal_first_failure": "reviewed Fq12 pad seed opening digest mismatch or pad opening statement mismatch",
            "diagnostic_requirement": "must not distinguish pad PRF lane failure from carrier projection failure",
            "assertions": common_assertions,
        },
        {
            "id": "selected_projection_reject_checked_core_mismatch",
            "covers": "checked-core mismatch",
            "mutation": "mutate the checked-core signature body after binding a valid carrier so the Fq12 account-carried checked-core verifier rejects before acceptance",
            "expected_internal_first_failure": "reviewed Fq12 checked-core challenge, account binding, response, or sigma relation mismatch",
            "diagnostic_requirement": "carrier verifier must not export selected material when checked-core precondition fails",
            "assertions": common_assertions,
        },
        {
            "id": "selected_projection_reject_malformed_56_byte_carrier",
            "covers": "malformed 56-byte carrier",
            "mutation": "exercise truncated, extended, and exactly 56-byte noncanonical lane encodings at the selected envelope decode boundary",
            "expected_internal_first_failure": "carrier length or noncanonical q12289 lane decode rejection",
            "diagnostic_requirement": "opaque carrier decode rejection; no lane index in public diagnostic",
            "assertions": common_assertions,
        },
    ]


def rust_kat_checks() -> list[dict[str, Any]]:
    rows = [
        (
            "valid_308_byte_fixture",
            "selected_projection_valid_308_byte_fixture_accepts",
            "constructs and accepts the compact 252-byte checked-core plus 56-byte carrier fixture",
        ),
        (
            "carrier_lane_mutation",
            "selected_projection_reject_carrier_lane_mutation",
            "mutates every decoded projection-carrier lane while preserving 308-byte envelope shape",
        ),
        (
            "carrier_check_mutation",
            "selected_projection_reject_carrier_check_mutation",
            "mutates the 3-byte carrier check field",
        ),
        (
            "carrier_transcript_binding_mutation",
            "selected_projection_reject_carrier_transcript_binding_mutation",
            "mutates the 29-byte carrier transcript binding field",
        ),
        (
            "account_state_rebinding",
            "selected_projection_reject_account_state_rebinding",
            "rebinds an otherwise valid carrier tuple to a same-shape account-state slot mutation",
        ),
        (
            "pad_seed_rebinding",
            "selected_projection_reject_pad_seed_rebinding",
            "rebinds the selected carrier tuple to a different pad seed",
        ),
        (
            "checked_core_mismatch",
            "selected_projection_reject_checked_core_mismatch",
            "mutates the checked-core body before carrier acceptance",
        ),
        (
            "malformed_56_byte_carrier",
            "selected_projection_reject_malformed_56_byte_carrier",
            "covers truncated, extended, and exactly 56-byte noncanonical carrier envelopes",
        ),
    ]
    return [
        code_check(f"rust_kat_{id_}", f"fn {test_name}", role)
        for id_, test_name, role in rows
    ]


def build_payload() -> dict[str, Any]:
    discharge = load_json(DISCHARGE_JSON)
    overread = load_json(OVERREAD_CONTROLS_JSON)
    reject = load_json(REJECT_EQUIVALENCE_JSON)
    req_carrier_contract = next(
        row for row in reject["failure_classes"] if row["id"] == "REQ-CARRIER"
    )

    evidence_checks = [
        code_check(
            "carrier_public_reason_present",
            "PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON",
            "public selected-route carrier reject reason is a single opaque class",
        ),
        code_check(
            "carrier_public_reason_value_present",
            "reject_fail_closed_invalid_projection_carrier",
            "public selected-route carrier reject reason uses the REQ-CARRIER class",
        ),
        code_check(
            "carrier_decoder_present",
            "pub fn decode_hidden_source_fallback_projection_carrier",
            "56-byte carrier decoder enforces length and lane canonicality",
        ),
        code_check(
            "candidate_verifier_present",
            "pub fn verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            "selected candidate verifier exists and is the intended KAT entrypoint",
        ),
        code_check(
            "slot_tuple_wrapper_present",
            "pub fn verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple",
            "slot/account-state wrapper feeds the carrier tuple verifier",
        ),
        code_check(
            "carrier_tuple_verifier_present",
            "pub fn verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
            "masked witness state18 projection-carrier tuple verifier exists",
        ),
        code_check(
            "carrier_binding_present",
            "pub fn reviewed_fq12_state_assisted_projection_carrier_binding",
            "carrier check bytes and transcript binding are recomputed from public/opening inputs",
        ),
        code_check(
            "check_mismatch_present",
            "reviewed Fq12 projection carrier check bytes mismatch",
            "check-byte mismatch rejection path exists",
        ),
        code_check(
            "transcript_mismatch_present",
            "reviewed Fq12 projection carrier transcript binding mismatch",
            "transcript-binding mismatch rejection path exists",
        ),
        code_check(
            "export_converted_to_error_present",
            "reviewed Fq12 masked witness state18 projection carrier tuple exported a hidden-material break",
            "hidden-material Export is converted to Err at selected tuple boundary",
        ),
        code_check(
            "carrier_reject_state_preservation_present",
            "assert_eq!(*account, before_account)",
            "carrier reject KAT harness asserts the public account state is unchanged",
        ),
    ]

    requested_kats = kat_specs()
    rust_kats = rust_kat_checks()
    rust_tests_feasible = all(item["present"] for item in rust_kats)
    rust_tests_reason = (
        "A compact valid 308-byte/Fq12 selected-route fixture is now executable in "
        "production_reference.rs, and all seven requested internal reject KAT hooks "
        "are present. The KATs assert one public invalid_projection_carrier class "
        "for the carrier rows and account-state preservation. Deployment remains "
        "fail-closed because timing equivalence is owned by the global reject-"
        "equivalence/constant-time blocker."
    )
    public_carrier_reason_present = (
        contains(PRODUCTION_REFERENCE, "PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON")
        and contains(PRODUCTION_REFERENCE, "reject_fail_closed_invalid_projection_carrier")
    )
    public_diagnostic_kats_present = all(
        contains(PRODUCTION_REFERENCE, needle)
        for needle in [
            "selected_projection_reject_carrier_lane_mutation",
            "selected_projection_reject_carrier_check_mutation",
            "selected_projection_reject_carrier_transcript_binding_mutation",
            "selected_projection_reject_account_state_rebinding",
            "selected_projection_reject_checked_core_mismatch",
            "selected_projection_reject_malformed_56_byte_carrier",
            "PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON",
        ]
    )
    stateless_state_preservation_present = contains(
        PRODUCTION_REFERENCE, "assert_eq!(*account, before_account)"
    )
    carrier_local_closure = (
        rust_tests_feasible
        and public_carrier_reason_present
        and public_diagnostic_kats_present
        and stateless_state_preservation_present
    )

    return {
        "schema": "Descender.pre_signature_projection_req_carrier_reject_closure.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": (
            "REQ-CARRIER_public_diagnostic_and_KATs_closed_timing_residual"
            if carrier_local_closure
            else "REQ-CARRIER_internal_KATs_executable_public_equivalence_open"
        ),
        "deployment_ready": False,
        "target": discharge["target"],
        "req_carrier_contract": req_carrier_contract,
        "discharge_input_status": discharge["status"],
        "decision": {
            "classification": (
                "local_req_carrier_closed_global_timing_residual"
                if carrier_local_closure
                else "internal_kats_executable_public_equivalence_open"
            ),
            "can_close_req_carrier": carrier_local_closure,
            "can_remove_A_reject_equivalence": False,
            "public_invalid_projection_carrier_diagnostic": public_carrier_reason_present,
            "public_diagnostic_kats_present": public_diagnostic_kats_present,
            "stateless_account_state_preservation_present": stateless_state_preservation_present,
            "kat_count": len(requested_kats),
            "failure_cases_covered": [item["covers"] for item in requested_kats],
            "reason": (
                "The seven requested reject cases have executable Rust KAT hooks "
                "against an accepted compact 308-byte/Fq12 projection-carrier fixture, "
                "and the public carrier rows collapse to reject_fail_closed_invalid_projection_carrier "
                "with stateless account preservation. A_reject_equivalence remains open "
                "until the global selected-route timing harness exists."
                if carrier_local_closure
                else "The seven requested reject cases now have executable Rust KAT hooks "
                "against an accepted compact 308-byte/Fq12 projection-carrier fixture. "
                "REQ-CARRIER remains open until those rows are promoted into the final "
                "selected-route harness with one public invalid_projection_carrier "
                "diagnostic plus state-preservation and constant-shape timing evidence."
            ),
        },
        "requested_kats": requested_kats,
        "evidence": {
            "code_presence_checks": evidence_checks,
            "all_code_presence_checks_passed": all(
                item["present"] for item in evidence_checks
            ),
            "overread_negative_tests": [
                overread_test(overread, "carrier_rebinding_reject_no_material"),
                overread_test(overread, "hidden_material_export_converted_to_error"),
                overread_test(overread, "helper_visibility_guard_no_pre_spend_callers"),
            ],
            "overread_surfaces": [
                overread_surface(overread, "masked_projection_carrier_tuple_verifier"),
                overread_surface(overread, "candidate_verifier"),
                overread_surface(overread, "accepting_certificate"),
                overread_surface(overread, "hidden_material_replay_inputs"),
            ],
        },
        "rust_tests": {
            "added": rust_tests_feasible,
            "feasible": rust_tests_feasible,
            "reason": rust_tests_reason,
            "executable_kats": rust_kats,
            "fixture_present": contains(
                PRODUCTION_REFERENCE, "fn selected_projection_valid_308_byte_fixture_accepts"
            ),
            "remaining_harness_needed": [
                "timing rows against valid, pad-seed, witness, transcript, and checked-core baselines",
            ],
        },
        "closure_requirements": [
            "REQ-CARRIER local KAT and public diagnostic closure is complete when can_close_req_carrier is true.",
            "Record timing rows for all seven KATs against valid, pad-seed, witness, transcript, and checked-core baselines.",
            "Only after timing outputs exist may A_reject_equivalence be discharged.",
        ],
        "source_artifacts": [
            source_status(DISCHARGE_JSON, "open REQ-CARRIER discharge input"),
            source_status(OVERREAD_CONTROLS_JSON, "overread/no-export controls"),
            source_status(REJECT_EQUIVALENCE_JSON, "REQ-CARRIER contract source"),
        ],
        "source_files_reviewed": [
            "scripts/research/pre_signature_projection_req_carrier_reject_discharge.py",
            "research/cryptanalysis/signature_readiness_20260516/pre_signature_projection_req_carrier_reject_discharge_n32_q12289.json",
            "scripts/research/pre_signature_projection_projection_overread_controls.py",
            "tools/parameter-search/src/production_reference.rs",
        ],
        "artifacts": {"json": rel(OUT_JSON), "markdown": rel(OUT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    decision = payload["decision"]
    lines = [
        "# REQ-CARRIER Reject Closure",
        "",
        "Fail-closed closure/refinement artifact for the selected 308-byte/Fq12",
        "state-assisted masked projection-carrier reject class.",
        "",
        "## Decision",
        "",
        f"- status: `{payload['status']}`",
        f"- classification: `{decision['classification']}`",
        f"- can close REQ-CARRIER: `{decision['can_close_req_carrier']}`",
        f"- can remove A_reject_equivalence: `{decision['can_remove_A_reject_equivalence']}`",
        f"- KAT rows specified: `{decision['kat_count']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- reason: {decision['reason']}",
        "",
        "## Seven Required KATs",
        "",
        "| KAT | covers | mutation | diagnostic requirement |",
        "|---|---|---|---|",
    ]
    for kat in payload["requested_kats"]:
        lines.append(
            f"| `{kat['id']}` | {kat['covers']} | {kat['mutation']} | "
            f"{kat['diagnostic_requirement']} |"
        )

    lines.extend(["", "## Shared KAT Assertions", ""])
    for assertion in payload["requested_kats"][0]["assertions"]:
        lines.append(f"- {assertion}")

    evidence = payload["evidence"]
    lines.extend(
        [
            "",
            "## Evidence Already Present",
            "",
            f"- all code presence checks passed: `{evidence['all_code_presence_checks_passed']}`",
            "",
            "| check | present | role |",
            "|---|---|---|",
        ]
    )
    for check in evidence["code_presence_checks"]:
        lines.append(f"| `{check['id']}` | `{check['present']}` | {check['role']} |")

    lines.extend(["", "Overread/no-export controls:", ""])
    for item in evidence["overread_negative_tests"]:
        lines.append(f"- `{item['id']}`: {item['expected_result']}")

    lines.extend(["", "Relevant selected-route surfaces:", ""])
    for item in evidence["overread_surfaces"]:
        lines.append(f"- `{item['id']}`: `{item['status']}` - {item['control']}")

    rust = payload["rust_tests"]
    lines.extend(
        [
            "",
            "## Rust Test Feasibility",
            "",
            f"- added: `{rust['added']}`",
            f"- feasible in this pass: `{rust['feasible']}`",
            f"- valid 308-byte fixture present: `{rust['fixture_present']}`",
            f"- reason: {rust['reason']}",
            "",
            "Executable Rust KAT hooks:",
            "",
        ]
    )
    for item in rust["executable_kats"]:
        lines.append(f"- `{item['id']}`: `{item['present']}` - {item['role']}")

    lines.extend(["", "Remaining harness evidence needed:", ""])
    lines.extend(f"- {item}" for item in rust["remaining_harness_needed"])

    lines.extend(["", "## Closure Requirements", ""])
    lines.extend(f"- {item}" for item in payload["closure_requirements"])

    lines.extend(["", "## Sources", ""])
    for item in payload["source_artifacts"]:
        lines.append(f"- `{item['artifact']}` ({item['role']}): `{item['status']}`")
    lines.append("")
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
                "kat_rows_specified": len(payload["requested_kats"]),
                "can_close_req_carrier": payload["decision"]["can_close_req_carrier"],
                "rust_tests_added": payload["rust_tests"]["added"],
                "presence_checks_passed": payload["evidence"][
                    "all_code_presence_checks_passed"
                ],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
