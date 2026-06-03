#!/usr/bin/env python3
"""REQ-CARRIER reject-equivalence discharge ledger.

This artifact is intentionally fail-closed.  It narrows the open
projection-carrier reject-equivalence class to concrete carrier failure cases,
existing tuple/rebinding evidence, missing selected-route timing and diagnostic
coverage, and the exact KAT/timing harness required to close the class.
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
    / "pre_signature_projection_req_carrier_reject_discharge_n32_q12289.json"
)
OUT_MD = (
    READINESS_DIR
    / "pre_signature_projection_req_carrier_reject_discharge_n32_q12289.md"
)

REJECT_EQUIVALENCE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_n32_q12289.json"
)
REJECT_EQUIVALENCE_DISCHARGE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_reject_equivalence_discharge_n32_q12289.json"
)
OVERREAD_CONTROLS_JSON = (
    READINESS_DIR
    / "pre_signature_projection_projection_overread_controls_n32_q12289.json"
)
FAULT_CAMPAIGN_JSON = (
    READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
)
FACADE_TIMING_JSON = (
    READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.json"
)
THEOREM_COMPOSITION_JSON = (
    READINESS_DIR
    / "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
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


def rows_by_class(fault_campaign: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = fault_campaign.get("summary", {}).get("annotated_rows", [])
    return {
        row["class"]: {
            "class": row.get("class"),
            "group": row.get("group"),
            "operation": row.get("operation"),
            "expected_outcome": row.get("expected_outcome"),
            "observed_outcome": row.get("observed_outcome"),
            "mean_avg_ns": row.get("mean_avg_ns"),
            "ratio_to_group_baseline": row.get("ratio_to_group_baseline"),
            "samples": row.get("samples"),
        }
        for row in rows
        if isinstance(row, dict) and "class" in row
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


def presence_check(id_: str, needle: str, role: str) -> dict[str, Any]:
    return {
        "id": id_,
        "path": rel(PRODUCTION_REFERENCE),
        "needle": needle,
        "present": contains(PRODUCTION_REFERENCE, needle),
        "role": role,
    }


def build_payload() -> dict[str, Any]:
    reject = load_json(REJECT_EQUIVALENCE_JSON)
    reject_discharge = load_json(REJECT_EQUIVALENCE_DISCHARGE_JSON)
    overread = load_json(OVERREAD_CONTROLS_JSON)
    fault_campaign = load_json(FAULT_CAMPAIGN_JSON)
    facade_timing = load_json(FACADE_TIMING_JSON)
    theorem = load_json(THEOREM_COMPOSITION_JSON)

    req_carrier_contract = next(
        row for row in reject["failure_classes"] if row["id"] == "REQ-CARRIER"
    )
    req_carrier_parent = next(
        row
        for row in reject_discharge["failure_class_discharge"]
        if row["id"] == "REQ-CARRIER"
    )
    carried = next(
        item
        for item in theorem.get("carried_assumptions", [])
        if item.get("id") == "A_reject_equivalence"
    )
    campaign_rows = rows_by_class(fault_campaign)

    carrier_failure_cases = [
        {
            "id": "carrier_decode_wrong_length",
            "predicate": "projection-carrier byte segment is missing, truncated, or extended before tuple verification",
            "public_surface": "reject_fail_closed_invalid_projection_carrier",
            "expected_diagnostic": "opaque carrier decode rejection only",
            "selected_kat": "selected_projection_reject_carrier_wrong_length_56_segment",
        },
        {
            "id": "carrier_decode_noncanonical_lane",
            "predicate": "one of the six masked projection residues decodes to a value >= q12289",
            "public_surface": "reject_fail_closed_invalid_projection_carrier",
            "expected_diagnostic": "must not reveal lane index or whether the value came from real or simulator lanes",
            "selected_kat": "selected_projection_reject_carrier_noncanonical_lane_each_of_6",
        },
        {
            "id": "carrier_check_bytes_mismatch",
            "predicate": "carrier check bytes do not equal the binding recomputed from opening commitment, statement, and decoded lanes",
            "public_surface": "reject_fail_closed_invalid_projection_carrier",
            "expected_diagnostic": "same public carrier class as lane and transcript mismatch",
            "selected_kat": "selected_projection_reject_carrier_check_byte_mutation",
        },
        {
            "id": "carrier_transcript_binding_mismatch",
            "predicate": "carrier transcript binding digest does not match message/public-key/signature/opening/slot transcript",
            "public_surface": "reject_fail_closed_invalid_projection_carrier",
            "expected_diagnostic": "same public carrier class, no proof-internal callback detail",
            "selected_kat": "selected_projection_reject_carrier_transcript_binding_mutation",
        },
        {
            "id": "carrier_slot_rebinding",
            "predicate": "carrier lanes/check bytes are rebound to a different public slot id or account state",
            "public_surface": "reject_fail_closed_invalid_projection_carrier",
            "expected_diagnostic": "no state transition and no selected opening material",
            "selected_kat": "selected_projection_reject_carrier_rebound_slot_id",
        },
        {
            "id": "carrier_pad_seed_digest_rebinding",
            "predicate": "carrier passes byte decoding but is inconsistent with the pad seed digest/opening used by the selected route",
            "public_surface": "reject_fail_closed_invalid_projection_carrier",
            "expected_diagnostic": "must not distinguish pad PRF lane failure from raw projection or packed-witness lane failure",
            "selected_kat": "selected_projection_reject_carrier_rebound_pad_seed_digest",
        },
        {
            "id": "carrier_hidden_material_export",
            "predicate": "hidden-material replay/export branch is reached before acceptance",
            "public_surface": "reject_fail_closed_invalid_projection_carrier",
            "expected_diagnostic": "external verifier returns Err, never Export or selected opening payload",
            "selected_kat": "selected_projection_reject_carrier_hidden_material_export_to_opaque_err",
        },
    ]

    code_evidence = [
        presence_check(
            "tuple_verifier_present",
            "pub fn verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple",
            "selected masked witness state18 projection-carrier tuple verifier exists",
        ),
        presence_check(
            "slot_tuple_wrapper_present",
            "pub fn verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple",
            "slot-bound tuple wrapper exists",
        ),
        presence_check(
            "candidate_verifier_present",
            "pub fn verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            "selected candidate verifier invokes tuple verifier but returns checked-core parts only",
        ),
        presence_check(
            "soundness_certificate_present",
            "pub fn verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate",
            "accepting certificate is the selected-material export path",
        ),
        presence_check(
            "carrier_binding_present",
            "pub fn reviewed_fq12_state_assisted_projection_carrier_binding",
            "carrier check-byte and transcript binding recomputation exists",
        ),
        presence_check(
            "export_converted_to_error",
            "masked witness state18 projection carrier tuple exported a hidden-material break",
            "hidden-material Export branch is converted to Err at tuple boundary",
        ),
        presence_check(
            "check_byte_mismatch_error",
            "reviewed Fq12 projection carrier check bytes mismatch",
            "carrier check-byte mismatch rejection exists",
        ),
        presence_check(
            "transcript_binding_mismatch_error",
            "reviewed Fq12 projection carrier transcript binding mismatch",
            "carrier transcript-binding mismatch rejection exists",
        ),
        presence_check(
            "deployment_blocker_present",
            "PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS",
            "deployment verifier remains fail-closed rather than accepted",
        ),
    ]

    rebinding_and_tuple_evidence = {
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
        "presence_checks": code_evidence,
        "all_presence_checks_passed": all(item["present"] for item in code_evidence),
    }

    exact_kat_harness_needed = [
        {
            "id": case["selected_kat"],
            "mutation": case["predicate"],
            "assertions": [
                "entrypoint is verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier or the final hardened equivalent",
                "return is Err with one public invalid_projection_carrier class",
                "slot/account state, spent ledger, and lifecycle snapshot are byte-identical before and after rejection",
                "no returned/logged/debug payload contains account_witness_projection_opening_frame, account_projection, projection_masks, pad_seed bytes, mask PRF lane outputs, retry counts, or lane indices",
            ],
        }
        for case in carrier_failure_cases
    ]

    exact_timing_harness_needed = {
        "id": "selected_projection_reject_carrier_timing",
        "entrypoint": "one hardened selected 308-byte/Fq12 projection-carrier verifier entrypoint",
        "baseline_rows": [
            "valid selected carrier spend/candidate row",
            "selected_projection_reject_pad_seed_digest_mismatch",
            "selected_projection_reject_witness_commitment_mismatch",
            "selected_projection_reject_transcript_binding_mismatch",
            "selected_projection_reject_core_relation_failure",
        ],
        "carrier_rows": [case["selected_kat"] for case in carrier_failure_cases],
        "measurement_requirements": [
            "same account snapshot shape, same message length, same public-key shape, same signature envelope length, same opening-frame length",
            "mutate exactly one carrier predicate per row after constructing an otherwise-valid selected envelope",
            "sample every carrier lane position separately for lane and unmasked-projection mismatch rows",
            "compare aggregate carrier rows against the valid row and against pad, witness, transcript, and core reject rows",
            "reuse the PTC-006 0.80..1.25 ratio gate or a stricter predeclared selected-route gate",
            "record diagnostic class counts and reject payload sizes next to timing rows",
        ],
        "required_outputs": [
            "JSON timing table with per-row mean/median/min/max/stddev and ratio_to_valid_baseline",
            "KAT transcript manifest naming the mutated byte/lane but proving it is not externally reported",
            "state-preservation hashes before and after every reject row",
            "diagnostic-equivalence table showing one public carrier reject class for all carrier rows",
        ],
    }

    facade_report = facade_timing.get("timing_report", {})
    existing_timing_rows = {
        "parent_req_carrier_rows": req_carrier_parent.get("timing_rows", []),
        "fallback_pad_projection_zero_seed": campaign_rows.get(
            "fallback_pad_projection_zero_seed"
        ),
        "facade_timing_caveat": facade_report.get("caveat"),
        "facade_signature_bytes": facade_report.get("signature_bytes"),
        "facade_reviewed_target_signature_bytes": facade_report.get(
            "reviewed_target_signature_bytes"
        ),
    }

    return {
        "schema": "Descender.pre_signature_projection_req_carrier_reject_discharge.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "REQ-CARRIER_fail_closed_not_discharged",
        "deployment_ready": False,
        "target": reject["target"],
        "carried_term": carried,
        "decision": {
            "classification": "fail_closed_not_discharged",
            "can_close_req_carrier": False,
            "can_remove_A_reject_equivalence": False,
            "can_claim_selected_route_carrier_timing_equivalence": False,
            "reason": (
                "Carrier tuple and overread controls exist and are fail-closed, but "
                "there is no selected-route KAT/timing/diagnostic harness proving "
                "that all projection-carrier rejects share one public diagnostic and "
                "constant-shape timing under the final 308-byte/Fq12 verifier."
            ),
        },
        "req_carrier_contract": req_carrier_contract,
        "parent_discharge_row": {
            key: req_carrier_parent[key]
            for key in [
                "id",
                "surface",
                "public_reject_surface",
                "classification",
                "basis",
                "existing_evidence",
                "selected_route_tests_needed",
                "timing_harness_needed",
                "blockers",
            ]
        },
        "carrier_failure_cases": carrier_failure_cases,
        "rebinding_and_tuple_evidence": rebinding_and_tuple_evidence,
        "existing_timing_coverage": existing_timing_rows,
        "missing_selected_route_coverage": [
            "No KAT mutates carrier lanes, check bytes, transcript binding, slot id, pad seed digest, and hidden-material Export on the final selected verifier path.",
            "No diagnostic-equivalence table proves all carrier failures collapse to one public invalid_projection_carrier reject class.",
            "No timing table measures those carrier rows against valid, pad, witness, transcript, and checked-core rows under the same hardened entrypoint.",
            "No state-preservation KAT proves carrier rejects leave account slot state, spent ledger, and lifecycle snapshots unchanged.",
            "Facade timing uses deterministic dummy callbacks and 250/254-byte facade rows, not accepting 308-byte/Fq12 projection-carrier proof semantics.",
        ],
        "exact_kat_harness_needed": exact_kat_harness_needed,
        "exact_timing_harness_needed": exact_timing_harness_needed,
        "source_artifacts": [
            source_status(REJECT_EQUIVALENCE_JSON, "REQ-CARRIER contract source"),
            source_status(
                REJECT_EQUIVALENCE_DISCHARGE_JSON,
                "parent A_reject_equivalence discharge ledger",
            ),
            source_status(
                OVERREAD_CONTROLS_JSON,
                "carrier rebinding and no-material export controls",
            ),
            source_status(FAULT_CAMPAIGN_JSON, "existing diagnostic timing rows"),
            source_status(FACADE_TIMING_JSON, "facade timing caveat"),
            source_status(THEOREM_COMPOSITION_JSON, "carried-term parent theorem"),
        ],
        "source_files_reviewed": [
            "scripts/research/pre_signature_projection_reject_equivalence_discharge.py",
            "scripts/research/pre_signature_projection_projection_overread_controls.py",
            "tools/parameter-search/src/production_reference.rs",
        ],
        "artifacts": {"json": rel(OUT_JSON), "markdown": rel(OUT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    decision = payload["decision"]
    lines = [
        "# REQ-CARRIER Reject-Equivalence Discharge",
        "",
        "Concrete fail-closed ledger for the projection-carrier reject class on the",
        "selected 308-byte/Fq12 state-assisted masked projection-carrier path.",
        "",
        "## Decision",
        "",
        f"- status: `{payload['status']}`",
        f"- classification: `{decision['classification']}`",
        f"- can close REQ-CARRIER: `{decision['can_close_req_carrier']}`",
        f"- can remove A_reject_equivalence: `{decision['can_remove_A_reject_equivalence']}`",
        f"- can claim selected-route carrier timing equivalence: `{decision['can_claim_selected_route_carrier_timing_equivalence']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- reason: {decision['reason']}",
        "",
        "## Carrier Failure Cases",
        "",
        "| case | public surface | selected KAT | diagnostic requirement |",
        "|---|---|---|---|",
    ]
    for case in payload["carrier_failure_cases"]:
        lines.append(
            f"| `{case['id']}` | `{case['public_surface']}` | "
            f"`{case['selected_kat']}` | {case['expected_diagnostic']} |"
        )

    evidence = payload["rebinding_and_tuple_evidence"]
    lines.extend(
        [
            "",
            "## Existing Rebinding And Tuple Evidence",
            "",
            f"- all production-reference presence checks passed: `{evidence['all_presence_checks_passed']}`",
            "",
            "| check | present | role |",
            "|---|---|---|",
        ]
    )
    for check in evidence["presence_checks"]:
        lines.append(f"| `{check['id']}` | `{check['present']}` | {check['role']} |")

    lines.extend(["", "Overread-control negative tests already specified:", ""])
    for item in evidence["overread_negative_tests"]:
        lines.append(f"- `{item['id']}`: {item['expected_result']}")

    lines.extend(["", "Overread-control surfaces already enumerated:", ""])
    for item in evidence["overread_surfaces"]:
        lines.append(f"- `{item['id']}`: `{item['status']}` - {item['control']}")

    timing = payload["existing_timing_coverage"]
    lines.extend(
        [
            "",
            "## Existing Timing Coverage",
            "",
            f"- parent REQ-CARRIER timing rows: `{len(timing['parent_req_carrier_rows'])}`",
            f"- fallback pad/projection row present: `{timing['fallback_pad_projection_zero_seed'] is not None}`",
            f"- facade signature bytes: `{timing['facade_signature_bytes']}`",
            f"- facade reviewed target signature bytes: `{timing['facade_reviewed_target_signature_bytes']}`",
            f"- facade caveat: {timing['facade_timing_caveat']}",
            "",
            "## Missing Selected-Route Coverage",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["missing_selected_route_coverage"])

    lines.extend(["", "## Exact KAT Harness Needed", ""])
    for item in payload["exact_kat_harness_needed"]:
        lines.append(f"- `{item['id']}`: {item['mutation']}")

    harness = payload["exact_timing_harness_needed"]
    lines.extend(
        [
            "",
            "## Exact Timing Harness Needed",
            "",
            f"- id: `{harness['id']}`",
            f"- entrypoint: {harness['entrypoint']}",
            "",
            "Carrier rows:",
            "",
        ]
    )
    lines.extend(f"- `{row}`" for row in harness["carrier_rows"])
    lines.extend(["", "Baseline rows:", ""])
    lines.extend(f"- `{row}`" for row in harness["baseline_rows"])
    lines.extend(["", "Measurement requirements:", ""])
    lines.extend(f"- {row}" for row in harness["measurement_requirements"])
    lines.extend(["", "Required outputs:", ""])
    lines.extend(f"- {row}" for row in harness["required_outputs"])

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
                "carrier_failure_cases": len(payload["carrier_failure_cases"]),
                "kat_rows_needed": len(payload["exact_kat_harness_needed"]),
                "presence_checks_passed": payload["rebinding_and_tuple_evidence"][
                    "all_presence_checks_passed"
                ],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
