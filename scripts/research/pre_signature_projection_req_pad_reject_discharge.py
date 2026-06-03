#!/usr/bin/env python3
"""Fail-closed REQ-PAD reject-equivalence discharge artifact.

This script isolates the pad-opening reject class from
pre_signature_projection_reject_equivalence and records the concrete evidence
that exists today, the exact selected-route cases that remain uncovered, and
the KAT/timing harness required before A_reject_equivalence can be discharged
for REQ-PAD.
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
    / "pre_signature_projection_req_pad_reject_discharge_n32_q12289.json"
)
OUT_MD = (
    READINESS_DIR / "pre_signature_projection_req_pad_reject_discharge_n32_q12289.md"
)

REJECT_EQUIVALENCE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_n32_q12289.json"
)
REJECT_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_discharge_n32_q12289.json"
)
TIMING_CLOSURE_JSON = READINESS_DIR / "side_channel_timing_closure_n32_q12289.json"
FAULT_CAMPAIGN_JSON = (
    READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
)
PAD_SWEEP_JSON = READINESS_DIR / "pre_third_party_pad_prf_seed_sweep_n32_q12289.json"
LIFECYCLE_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json"
)
OVERREAD_CONTROLS_JSON = (
    READINESS_DIR / "pre_signature_projection_projection_overread_controls_n32_q12289.json"
)
SPEND_TIME_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_budget_discharge_n32_q12289.json"
)
PACKED_WITNESS_CONTROLS_JSON = (
    READINESS_DIR / "pre_signature_projection_packed_witness_rewrite_controls_n32_q12289.json"
)
PRODUCTION_REFERENCE_RS = ROOT / "tools/parameter-search/src/production_reference.rs"
PRODUCTION_REFERENCE_TESTS_RS = ROOT / "tools/production-reference/src/lib.rs"

SCRIPT_SOURCES = [
    Path("scripts/research/pre_signature_projection_reject_equivalence.py"),
    Path("scripts/research/pre_signature_projection_reject_equivalence_discharge.py"),
    Path("scripts/research/pre_signature_projection_lifecycle_same_game_discharge.py"),
    Path("scripts/research/pre_signature_projection_projection_overread_controls.py"),
    Path("tools/parameter-search/src/production_reference.rs"),
]


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
            or payload.get("current_claim")
            or payload.get("deployment_ready")
            or "present_no_status_field"
        ),
        "role": role,
    }


def contains(path: Path, needle: str) -> bool:
    return needle in path.read_text()


def fault_rows_by_class(fault_campaign: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        row["class"]: row
        for row in fault_campaign.get("summary", {}).get("annotated_rows", [])
        if isinstance(row, dict) and "class" in row
    }


def selected_fault_rows(fault_campaign: dict[str, Any]) -> list[dict[str, Any]]:
    rows = fault_rows_by_class(fault_campaign)
    wanted = [
        "pad_prf_zero_seed",
        "pad_prf_one_seed",
        "pad_prf_ff_seed",
        "fallback_pad_projection_zero_seed",
    ]
    return [rows[name] for name in wanted if name in rows]


def find_failure_class(reject: dict[str, Any]) -> dict[str, Any]:
    for row in reject.get("failure_classes", []):
        if row.get("id") == "REQ-PAD":
            return row
    raise KeyError("REQ-PAD not found in reject-equivalence artifact")


def find_req_pad_discharge(discharge: dict[str, Any]) -> dict[str, Any]:
    for row in discharge.get("failure_class_discharge", []):
        if row.get("id") == "REQ-PAD":
            return row
    raise KeyError("REQ-PAD not found in reject-equivalence discharge artifact")


def evidence_presence() -> list[dict[str, Any]]:
    checks = [
        (
            "selected_route_pad_digest_verifier",
            PRODUCTION_REFERENCE_RS,
            "verify_reviewed_fq12_pad_seed_opening(statement, pad_seed, &opening.pad_seed_digest)?",
            "selected candidate verifier binds pad_seed to the public pad_seed_digest",
        ),
        (
            "selected_route_slot_pad_digest_check",
            PRODUCTION_REFERENCE_RS,
            '"state-assisted pad seed digest mismatch"',
            "slot-bound verifier rejects pad_seed_digest mismatch before acceptance",
        ),
        (
            "selected_route_candidate_verifier",
            PRODUCTION_REFERENCE_RS,
            "pub fn verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            "selected projection candidate verifier exists as a single entrypoint to instrument",
        ),
        (
            "selected_route_soundness_certificate",
            PRODUCTION_REFERENCE_RS,
            "pub fn verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate",
            "accepting certificate path is separate from reject-before-acceptance paths",
        ),
        (
            "rust_wrong_pad_seed_negative",
            PRODUCTION_REFERENCE_TESTS_RS,
            "let mut wrong_pad_seed = pad_seed;",
            "Rust tests mutate pad_seed and expect rejection",
        ),
        (
            "rust_pad_digest_mismatch_negative",
            PRODUCTION_REFERENCE_TESTS_RS,
            '"state-assisted pad seed digest mismatch"',
            "Rust tests mutate pad_seed_digest and expect public mismatch rejection",
        ),
        (
            "rust_selected_projection_regression",
            PRODUCTION_REFERENCE_TESTS_RS,
            "fn masked_witness_state18_unmasks_projection_with_pad_seed()",
            "selected 18-row masked projection regression covers wrong-pad and carrier-rebinding rejects",
        ),
        (
            "rust_pad_digest_uniqueness_negative",
            PRODUCTION_REFERENCE_TESTS_RS,
            "fn state_assisted_account_schema_enforces_unique_pad_seed_digest()",
            "account-state construction rejects duplicate pad_seed_digest values",
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


def pad_failure_cases(req_pad: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        {
            "id": "PAD-MISSING-MALFORMED",
            "source_contract_text": req_pad["real_failure_classes"][0],
            "public_surface": req_pad["public_reject_surface"],
            "expected_reject": "reject_fail_closed_invalid_opening",
            "state_transition": "none",
            "covered_today": False,
            "existing_evidence": [
                "projection-overread controls require malformed opening paths to return Err only",
            ],
            "gap": "no selected-route KAT removes, truncates, extends, or corrupts the pad opening frame after full 308-byte decode",
        },
        {
            "id": "PAD-DIGEST-MISMATCH",
            "source_contract_text": req_pad["real_failure_classes"][1],
            "public_surface": req_pad["public_reject_surface"],
            "expected_reject": "reject_fail_closed_invalid_opening",
            "state_transition": "none",
            "covered_today": "partial",
            "existing_evidence": [
                "slot-bound verifier code checks state-assisted pad seed digest mismatch",
                "Rust tests mutate pad_seed_digest and wrong pad_seed",
                "lifecycle discharge records pad_seed_digest uniqueness and spent-ledger persistence",
            ],
            "gap": "existing tests do not run under one selected-route diagnostic/timing harness or prove diagnostic collapse to invalid_opening",
        },
        {
            "id": "PAD-DOMAIN-SLOT-LANE-COUNTER",
            "source_contract_text": req_pad["real_failure_classes"][2],
            "public_surface": req_pad["public_reject_surface"],
            "expected_reject": "reject_fail_closed_invalid_opening",
            "state_transition": "none",
            "covered_today": False,
            "existing_evidence": [
                "pad PRF seed sweep provides deterministic PRF and avalanche controls",
                "mask/frame disjointness artifacts define separate PRF domains",
            ],
            "gap": "no KAT mutates pad PRF domain, slot id, lane id, or counter inside the selected verifier and asserts identical public rejection",
        },
        {
            "id": "PAD-STREAM-RESPONSE-MISMATCH",
            "source_contract_text": req_pad["real_failure_classes"][3],
            "public_surface": req_pad["public_reject_surface"],
            "expected_reject": "reject_fail_closed_invalid_opening",
            "state_transition": "none",
            "covered_today": "partial",
            "existing_evidence": [
                "selected masked projection tests reject wrong pad_seed",
                "overread controls require wrong-pad rejects to return no partial opening material",
            ],
            "gap": "no selected-route harness mutates each derived local pad/response lane and checks timing plus diagnostic equivalence across all lanes",
        },
    ]


def required_kats() -> list[dict[str, Any]]:
    return [
        {
            "id": "selected_projection_reject_pad_opening_missing",
            "input_mutation": "remove the pad-opening token/frame while keeping a syntactically valid selected 308-byte projection-carrier transaction envelope",
            "entrypoint": "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            "expected": "Err mapped to reject_fail_closed_invalid_opening; account state, spent ledger, and pad digest ledger unchanged",
            "diagnostic_assertions": [
                "no pad_seed bytes",
                "no pad PRF counters or rejected sampler words",
                "no lane-local mismatch index",
                "no packed opening or projection material",
            ],
        },
        {
            "id": "selected_projection_reject_pad_opening_malformed_length",
            "input_mutation": "truncate, extend, and noncanonically encode the pad-opening frame after the public signature frame decodes",
            "entrypoint": "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            "expected": "same public invalid_opening reject for each length/canonicality variant",
            "diagnostic_assertions": [
                "all malformed-frame variants share one public error class",
                "no state transition",
            ],
        },
        {
            "id": "selected_projection_reject_pad_seed_digest_mismatch",
            "input_mutation": "flip one bit of opening.pad_seed_digest and separately flip one bit of the supplied pad_seed",
            "entrypoint": "verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple",
            "expected": "public invalid_opening reject before accepting certificate construction",
            "diagnostic_assertions": [
                "no returned ProductionReviewedFq12MaskedState18PackedWitnessOpening",
                "no account_witness_projection_opening_frame",
                "no projection_masks/account_projection",
            ],
        },
        {
            "id": "selected_projection_reject_pad_domain_slot_lane_counter_mismatch",
            "input_mutation": "derive pads with a wrong domain, wrong slot id, wrong lane id, and wrong counter while preserving all other witness/opening bytes",
            "entrypoint": "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            "expected": "same public invalid_opening reject for all four derivation-frame mismatches",
            "diagnostic_assertions": [
                "error does not identify domain, slot, lane, or counter",
                "no retry count or sampler rejection count",
            ],
        },
        {
            "id": "selected_projection_reject_pad_stream_response_mismatch_all_lanes",
            "input_mutation": "for each selected lane, mutate one derived local pad/response relation while preserving pad_seed_digest and carrier bytes",
            "entrypoint": "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            "expected": "same public invalid_opening reject for all lane positions",
            "diagnostic_assertions": [
                "error does not report lane position",
                "no partial opening, unmasked residue, or mask residue is returned",
            ],
        },
    ]


def required_timing_harness() -> dict[str, Any]:
    return {
        "id": "selected_projection_reject_pad_timing",
        "entrypoint": "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
        "baseline_rows": [
            "valid_selected_projection_spend",
            "REQ-PAD PAD-MISSING-MALFORMED",
            "REQ-PAD PAD-DIGEST-MISMATCH",
            "REQ-PAD PAD-DOMAIN-SLOT-LANE-COUNTER",
            "REQ-PAD PAD-STREAM-RESPONSE-MISMATCH",
            "REQ-WITNESS packed opening mismatch control",
            "REQ-CARRIER carrier lane mismatch control",
            "REQ-TRANSCRIPT transcript binding mismatch control",
            "REQ-CORE checked-core mismatch control",
        ],
        "measurement_requirements": [
            "run all rows through the same selected 308-byte/Fq12 verifier entrypoint",
            "normalize account snapshot shape and spent-ledger shape across valid and reject rows",
            "collect per-row timing batches with the PTC-006 0.80..1.25 ratio policy or a stricter selected-route policy",
            "record state hashes before and after each reject row and require byte-identical state on reject",
            "record sanitized public diagnostic class and require one REQ-PAD public class for every pad mutation",
            "run lane-position sweeps for every selected pad/response lane and aggregate max/min/median timing by lane",
        ],
        "fail_conditions": [
            "any pad reject returns selected projection material, pad_seed bytes, PRF output, retry count, or lane index",
            "any pad reject mutates account state, reservation state, spent ledger, or pad digest ledger",
            "any pad reject timing row falls outside the selected ratio policy",
            "pad rejects are distinguishable from witness/carrier/core rejects by internal diagnostic payload rather than only public reject surface",
        ],
    }


def build_payload() -> dict[str, Any]:
    reject = load_json(REJECT_EQUIVALENCE_JSON)
    discharge = load_json(REJECT_DISCHARGE_JSON)
    timing = load_json(TIMING_CLOSURE_JSON)
    fault_campaign = load_json(FAULT_CAMPAIGN_JSON)
    pad_sweep = load_json(PAD_SWEEP_JSON)
    lifecycle = load_json(LIFECYCLE_DISCHARGE_JSON)
    overread = load_json(OVERREAD_CONTROLS_JSON)
    spend_time = load_json(SPEND_TIME_DISCHARGE_JSON)

    req_pad = find_failure_class(reject)
    req_pad_discharge = find_req_pad_discharge(discharge)
    presence = evidence_presence()
    missing_presence = [row["id"] for row in presence if not row["present"]]

    decision = {
        "classification": "fail_closed_not_discharged",
        "can_remove_req_pad_from_A_reject_equivalence": False,
        "can_claim_selected_route_pad_reject_constant_time": False,
        "can_consume_as_fail_closed_artifact": True,
        "reason": (
            "Pad-opening reject semantics and several local negative controls "
            "exist, but there is no selected-route harness that mutates all pad "
            "opening predicates under the final 308-byte/Fq12 verifier while "
            "checking constant-shape timing, sanitized diagnostics, and no-state "
            "mutation."
        ),
    }

    return {
        "schema": "Descender.pre_signature_projection_req_pad_reject_discharge.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "REQ_PAD_fail_closed_not_discharged",
        "deployment_ready": False,
        "target": reject["target"],
        "req_pad_contract": req_pad,
        "decision": decision,
        "failure_cases": pad_failure_cases(req_pad),
        "existing_evidence": {
            "req_pad_parent_discharge": {
                "classification": req_pad_discharge["classification"],
                "basis": req_pad_discharge["basis"],
                "blockers": req_pad_discharge["blockers"],
                "selected_route_tests_needed": req_pad_discharge[
                    "selected_route_tests_needed"
                ],
                "timing_harness_needed": req_pad_discharge["timing_harness_needed"],
            },
            "code_presence_checks": presence,
            "missing_code_presence_checks": missing_presence,
            "pad_prf_timing_rows": selected_fault_rows(fault_campaign),
            "pad_prf_seed_sweep": {
                "campaign": pad_sweep.get("campaign"),
                "checks": pad_sweep.get("checks", [])[:2],
                "avalanche_summary": {
                    "sample_count": pad_sweep.get("avalanche", {}).get("sample_count"),
                    "projection_lanes_changed_mean": pad_sweep.get(
                        "avalanche", {}
                    ).get("projection_lanes_changed_mean"),
                    "seed_bit_digest_hamming_mean": pad_sweep.get("avalanche", {}).get(
                        "seed_bit_digest_hamming_mean"
                    ),
                },
            },
            "lifecycle_pad_digest_controls": [
                item
                for item in lifecycle.get("discharge_items", [])
                if item.get("id")
                in {
                    "pad_seed_digest_uniqueness",
                    "spend_burn_ledger_behavior",
                }
            ],
            "projection_overread_summary": {
                "status": overread.get("status"),
                "evidence_summary": overread.get("evidence_summary"),
                "reject_before_acceptance_controls": [
                    row
                    for row in overread.get("negative_tests", [])
                    if "pad" in json.dumps(row).lower()
                    or "opening" in json.dumps(row).lower()
                ],
            },
            "spend_time_budget_decision": spend_time.get("decision"),
        },
        "missing_selected_route_coverage": [
            "no KAT removes or corrupts a pad-opening frame after full selected-frame decode",
            "no KAT mutates pad PRF domain, slot id, lane id, and counter frames in the selected verifier",
            "no all-lane derived-pad/response mismatch sweep records one public invalid_opening class",
            "no selected-route diagnostic capture proves the public report omits pad_seed bytes, PRF outputs, retry counts, and lane-local mismatch indices",
            "no selected-route timing matrix compares REQ-PAD rows with valid, REQ-WITNESS, REQ-CARRIER, REQ-TRANSCRIPT, and REQ-CORE rows under one account snapshot shape",
        ],
        "required_kats": required_kats(),
        "required_timing_harness": required_timing_harness(),
        "timing_closure_inputs": {
            "campaign_id": timing.get("campaign_id"),
            "closure_status": timing.get("closure_status"),
            "gates": timing.get("gates"),
            "policy": timing.get("policy"),
            "blockers": timing.get("blockers"),
        },
        "source_artifacts": [
            artifact(REJECT_EQUIVALENCE_JSON, "REQ-PAD source reject contract"),
            artifact(REJECT_DISCHARGE_JSON, "parent seven-class discharge ledger"),
            artifact(TIMING_CLOSURE_JSON, "PTC-006 timing closure gates"),
            artifact(FAULT_CAMPAIGN_JSON, "pad PRF timing rows"),
            artifact(PAD_SWEEP_JSON, "pad PRF KAT/seed sweep evidence"),
            artifact(LIFECYCLE_DISCHARGE_JSON, "pad_seed_digest lifecycle controls"),
            artifact(OVERREAD_CONTROLS_JSON, "reject export/no partial opening controls"),
            artifact(SPEND_TIME_DISCHARGE_JSON, "spend-time budget residuals"),
            artifact(
                PACKED_WITNESS_CONTROLS_JSON,
                "slot-bound packed-witness/pad digest controls",
            ),
        ],
        "source_files_reviewed": [str(path) for path in SCRIPT_SOURCES],
        "artifacts": {"json": rel(OUT_JSON), "markdown": rel(OUT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    decision = payload["decision"]
    lines = [
        "# REQ-PAD Reject-Equivalence Discharge",
        "",
        "This artifact isolates `REQ-PAD` from `A_reject_equivalence` for the",
        "selected 308-byte/Fq12 state-assisted masked projection-carrier path.",
        "",
        "## Decision",
        "",
        f"- status: `{payload['status']}`",
        f"- classification: `{decision['classification']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- can remove REQ-PAD from A_reject_equivalence: `{decision['can_remove_req_pad_from_A_reject_equivalence']}`",
        f"- can claim selected-route pad reject constant time: `{decision['can_claim_selected_route_pad_reject_constant_time']}`",
        f"- reason: {decision['reason']}",
        "",
        "## Pad Failure Cases",
        "",
        "| case | coverage | expected reject | gap |",
        "|---|---|---|---|",
    ]
    for row in payload["failure_cases"]:
        lines.append(
            f"| `{row['id']}` | `{row['covered_today']}` | `{row['expected_reject']}` | {row['gap']} |"
        )

    lines.extend(
        [
            "",
            "## Existing Evidence",
            "",
            f"- parent REQ-PAD status: `{payload['existing_evidence']['req_pad_parent_discharge']['classification']}`",
            f"- timing gates: `{json.dumps(payload['timing_closure_inputs']['gates'], sort_keys=True)}`",
            f"- timing policy: `{json.dumps(payload['timing_closure_inputs']['policy'], sort_keys=True)}`",
            f"- code presence checks missing: `{payload['existing_evidence']['missing_code_presence_checks']}`",
            "",
            "Pad PRF timing rows:",
        ]
    )
    for row in payload["existing_evidence"]["pad_prf_timing_rows"]:
        lines.append(
            f"- `{row['class']}` `{row['operation']}` mean `{row['mean_avg_ns']}` ns ratio `{row['ratio_to_group_baseline']}`"
        )

    lines.extend(["", "## Missing Selected-Route Coverage", ""])
    lines.extend(f"- {item}" for item in payload["missing_selected_route_coverage"])

    lines.extend(["", "## Required KATs", ""])
    for row in payload["required_kats"]:
        diagnostic_assertions = "; ".join(row["diagnostic_assertions"])
        lines.extend(
            [
                f"### {row['id']}",
                "",
                f"- mutation: {row['input_mutation']}",
                f"- entrypoint: `{row['entrypoint']}`",
                f"- expected: {row['expected']}",
                f"- diagnostic assertions: {diagnostic_assertions}",
            ]
        )
        lines.append("")

    harness = payload["required_timing_harness"]
    lines.extend(
        [
            "## Required Timing Harness",
            "",
            f"- id: `{harness['id']}`",
            f"- entrypoint: `{harness['entrypoint']}`",
            f"- rows: `{'; '.join(harness['baseline_rows'])}`",
            f"- measurement requirements: {'; '.join(harness['measurement_requirements'])}",
            f"- fail conditions: {'; '.join(harness['fail_conditions'])}",
        ]
    )

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
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(markdown(payload) + "\n")
    print(
        json.dumps(
            {
                "json": rel(OUT_JSON),
                "markdown": rel(OUT_MD),
                "status": payload["status"],
                "classification": payload["decision"]["classification"],
                "can_remove_req_pad_from_A_reject_equivalence": payload["decision"][
                    "can_remove_req_pad_from_A_reject_equivalence"
                ],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
