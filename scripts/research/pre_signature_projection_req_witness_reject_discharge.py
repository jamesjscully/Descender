#!/usr/bin/env python3
"""Fail-closed discharge artifact for REQ-WITNESS reject equivalence.

This generator isolates the packed-witness opening class from the broader
reject-equivalence ledger.  It records the local commitment/repack controls
that exist today, then names the selected-route KAT and timing evidence needed
before the class can be discharged.
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
    / "pre_signature_projection_req_witness_reject_discharge_n32_q12289.json"
)
OUT_MD = (
    READINESS_DIR
    / "pre_signature_projection_req_witness_reject_discharge_n32_q12289.md"
)

REJECT_EQUIVALENCE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_n32_q12289.json"
)
REJECT_EQUIVALENCE_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_discharge_n32_q12289.json"
)
PACKED_WITNESS_CONTROLS_JSON = (
    READINESS_DIR
    / "pre_signature_projection_packed_witness_rewrite_controls_n32_q12289.json"
)
CT_AUDIT_JSON = READINESS_DIR / "constant_time_side_channel_audit_memo_n32_q12289.json"
TIMING_CLOSURE_JSON = READINESS_DIR / "side_channel_timing_closure_n32_q12289.json"
FAULT_CAMPAIGN_JSON = (
    READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
)
PRODUCTION_REFERENCE_RS = ROOT / "tools/parameter-search/src/production_reference.rs"


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
            or "present_no_status_field"
        ),
        "role": role,
    }


def witness_contract(reject: dict[str, Any]) -> dict[str, Any]:
    return next(row for row in reject["failure_classes"] if row["id"] == "REQ-WITNESS")


def witness_discharge_row(reject_discharge: dict[str, Any]) -> dict[str, Any]:
    return next(
        row
        for row in reject_discharge["failure_class_discharge"]
        if row["id"] == "REQ-WITNESS"
    )


def campaign_rows_by_class(fault_campaign: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = fault_campaign.get("summary", {}).get("annotated_rows", [])
    return {
        row["class"]: {
            "class": row["class"],
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


def code_presence_from_controls(
    packed_controls: dict[str, Any], ids: list[str]
) -> list[dict[str, Any]]:
    wanted = set(ids)
    return [
        {
            "id": item["id"],
            "path": item["path"],
            "present": item["present"],
            "rationale": item["rationale"],
        }
        for item in packed_controls["automated_code_presence_checks"]
        if item["id"] in wanted
    ]


def all_present(rows: list[dict[str, Any]]) -> bool:
    return all(bool(row["present"]) for row in rows)


def witness_failure_cases(contract: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        {
            "id": "packed_witness_opening_absent",
            "contract_predicate": contract["real_failure_classes"][0],
            "expected_public_reject": contract["public_reject_surface"],
            "current_evidence": [],
            "classification": "open_no_selected_route_negative",
            "missing_kat": "selected_projection_reject_witness_opening_missing",
            "diagnostic_requirement": "collapse to invalid_opening without naming missing packed witness material",
        },
        {
            "id": "packed_witness_wrong_length_or_nonzero_slack",
            "contract_predicate": contract["real_failure_classes"][0],
            "expected_public_reject": contract["public_reject_surface"],
            "current_evidence": [],
            "classification": "open_no_selected_route_negative",
            "missing_kat": "selected_projection_reject_witness_wrong_length_or_nonzero_slack",
            "diagnostic_requirement": "collapse length and slack faults to the same public invalid_opening class",
        },
        {
            "id": "packed_witness_noncanonical_q12289_residue",
            "contract_predicate": contract["real_failure_classes"][1],
            "expected_public_reject": contract["public_reject_surface"],
            "current_evidence": [],
            "classification": "open_no_selected_route_negative",
            "missing_kat": "selected_projection_reject_witness_noncanonical_q12289_residue",
            "diagnostic_requirement": "do not expose residue index or decoded value",
        },
        {
            "id": "checked_core_residues_do_not_bind_commitment",
            "contract_predicate": contract["real_failure_classes"][2],
            "expected_public_reject": contract["public_reject_surface"],
            "current_evidence": [
                "rust_packed_opening_repack_check",
                "rust_packed_opening_fields_mismatch_reject",
            ],
            "classification": "slot_bound_subset_supported_not_selected_route_discharged",
            "missing_kat": "selected_projection_reject_witness_commitment_mismatch",
            "diagnostic_requirement": "do not distinguish repack mismatch from checked-core witness mismatch",
        },
        {
            "id": "packed_opening_hidden_source_commitment_mismatch",
            "contract_predicate": contract["real_failure_classes"][3],
            "expected_public_reject": contract["public_reject_surface"],
            "current_evidence": [
                "rust_masked_projection_statement_commitment_check",
                "rust_projection_carrier_hidden_material_break_reject",
            ],
            "classification": "slot_bound_subset_supported_not_selected_route_discharged",
            "missing_kat": "selected_projection_reject_hidden_source_opening_mismatch",
            "diagnostic_requirement": "do not distinguish hidden-source opening mismatch from carrier/witness binding mismatch",
        },
    ]


def selected_route_kats() -> list[dict[str, Any]]:
    return [
        {
            "id": "selected_projection_reject_witness_opening_missing",
            "mutation": "remove the packed state18 witness opening while keeping the 308-byte carrier frame otherwise well-formed",
            "assertions": [
                "verifier returns the single public invalid_opening reject class",
                "no account slot is consumed",
                "no hidden witness bytes, row ids, or opening commitments appear in diagnostics",
            ],
        },
        {
            "id": "selected_projection_reject_witness_wrong_length_or_nonzero_slack",
            "mutation": "truncate/extend the packed witness opening and set unused packed bits nonzero",
            "assertions": [
                "wrong length and slack faults map to the same public reject class",
                "failure is indistinguishable from commitment mismatch at the diagnostic boundary",
                "state snapshot remains byte-identical before and after rejection",
            ],
        },
        {
            "id": "selected_projection_reject_witness_noncanonical_q12289_residue",
            "mutation": "encode at least one of the 18 packed rows or 12 checked-core rows as q12289 or above",
            "assertions": [
                "reject does not expose the failing row index",
                "reject report omits decoded residue values",
                "selected verifier still executes the common proof-internal reject schedule",
            ],
        },
        {
            "id": "selected_projection_reject_witness_commitment_mismatch",
            "mutation": "repack the witness opening to a commitment different from the account slot commitment",
            "assertions": [
                "slot-bound account_witness_commitment remains unchanged",
                "projection material is not exported",
                "reject report matches the other witness-opening rows",
            ],
        },
        {
            "id": "selected_projection_reject_hidden_source_opening_mismatch",
            "mutation": "bind the carrier to one hidden-source opening commitment and the packed witness to another",
            "assertions": [
                "verifier rejects before any accepting certificate path",
                "carrier, hidden-source, and witness mismatch diagnostics collapse to invalid_opening",
                "spent-ledger and lifecycle stores are unchanged",
            ],
        },
    ]


def timing_harness_spec() -> dict[str, Any]:
    return {
        "id": "selected_projection_reject_witness_timing",
        "entrypoint": "final selected 308-byte/Fq12 state-assisted masked projection-carrier verifier",
        "baseline_rows": [
            "valid_308_byte_fq12_projection_carrier",
            "selected_projection_reject_pad_seed_digest_mismatch",
            "selected_projection_reject_carrier_unmasked_projection_mismatch",
            "selected_projection_reject_transcript_slot_id_rebinding",
            "selected_projection_reject_core_syndrome_equation_failure",
        ],
        "witness_rows": [
            "selected_projection_reject_witness_opening_missing",
            "selected_projection_reject_witness_wrong_length_or_nonzero_slack",
            "selected_projection_reject_witness_noncanonical_q12289_residue",
            "selected_projection_reject_witness_commitment_mismatch",
            "selected_projection_reject_hidden_source_opening_mismatch",
        ],
        "sample_policy": {
            "minimum_batches_per_row": 25,
            "ratio_window": "0.80..1.25 against selected-route valid baseline and proof-internal reject aggregate",
            "required_outcome": "all witness rows pass timing window and return identical public diagnostic class",
        },
        "diagnostic_capture": [
            "public reject enum only",
            "constant-size rejection report",
            "no row index, residue value, packed-witness length, commitment digest, retry count, or hidden-source mismatch reason",
        ],
    }


def build_payload() -> dict[str, Any]:
    reject = load_json(REJECT_EQUIVALENCE_JSON)
    reject_discharge = load_json(REJECT_EQUIVALENCE_DISCHARGE_JSON)
    packed_controls = load_json(PACKED_WITNESS_CONTROLS_JSON)
    ct_audit = load_json(CT_AUDIT_JSON)
    timing = load_json(TIMING_CLOSURE_JSON)
    fault_campaign = load_json(FAULT_CAMPAIGN_JSON)

    contract = witness_contract(reject)
    broader_row = witness_discharge_row(reject_discharge)
    campaign_rows = campaign_rows_by_class(fault_campaign)
    expected_missing_rows = [
        "selected_projection_reject_witness_opening_missing",
        "selected_projection_reject_witness_wrong_length_or_nonzero_slack",
        "selected_projection_reject_witness_noncanonical_q12289_residue",
        "selected_projection_reject_witness_commitment_mismatch",
        "selected_projection_reject_hidden_source_opening_mismatch",
    ]

    commitment_check_ids = [
        "rust_slot_opening_witness_commitment_mismatch",
        "rust_slot_opening_restore_anchor_binding",
        "rust_slot_opening_pad_seed_binding",
        "rust_packed_opening_repack_check",
        "rust_packed_opening_fields_mismatch_reject",
        "rust_masked_projection_statement_commitment_check",
        "rust_projection_carrier_hidden_material_break_reject",
    ]
    lifecycle_check_ids = [
        "rust_sign_reserved_and_consume_sanitized",
        "rust_verify_with_account_binding_sanitized",
        "rust_fq12_transcript_binding_sign_rejects_stale_signature",
        "rust_sanitized_lifecycle_sign_failure_no_state_mutation",
        "rust_restore_anchor_rejects_old_snapshot",
        "rust_state_assisted_account_state_from_slot_records",
        "rust_reviewed_fq12_sign_reserved_and_consume_records_pad_digest",
    ]
    commitment_checks = code_presence_from_controls(
        packed_controls, commitment_check_ids
    )
    lifecycle_checks = code_presence_from_controls(packed_controls, lifecycle_check_ids)
    timing_rows = [campaign_rows[name] for name in expected_missing_rows if name in campaign_rows]

    return {
        "schema": "Descender.pre_signature_projection_req_witness_reject_discharge.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "REQ-WITNESS_fail_closed_not_discharged",
        "deployment_ready": False,
        "target": reject["target"],
        "reject_class": {
            "id": contract["id"],
            "surface": contract["surface"],
            "public_reject_surface": contract["public_reject_surface"],
            "equivalence_rule": contract["equivalence_rule"],
            "real_failure_classes": contract["real_failure_classes"],
            "simulated_failure_classes": contract["simulated_failure_classes"],
            "state_transition_on_reject": contract["state_transition_on_reject"],
            "secret_dependent_diagnostics_allowed": contract[
                "secret_dependent_diagnostics_allowed"
            ],
        },
        "decision": {
            "classification": "fail_closed_not_discharged",
            "can_remove_REQ_WITNESS_from_A_reject_equivalence": False,
            "can_claim_selected_route_diagnostic_equivalence": False,
            "can_claim_selected_route_timing_equivalence": False,
            "can_use_existing_commitment_repack_controls": all_present(commitment_checks),
            "reason": (
                "Existing code and artifacts support slot-bound commitment and "
                "repacked-opening checks, but no selected-route KAT or timing row "
                "exercises packed-witness opening rejection through the final "
                "308-byte/Fq12 verifier. The class therefore remains fail-closed."
            ),
        },
        "witness_failure_cases": witness_failure_cases(contract),
        "existing_commitment_and_repacked_tests": {
            "source": rel(PACKED_WITNESS_CONTROLS_JSON),
            "slot_bound_subset_discharged": packed_controls[
                "existing_commitment_and_repacked_checks"
            ]["slot_bound_subset_discharged"],
            "lifecycle_controls_present": packed_controls[
                "existing_commitment_and_repacked_checks"
            ]["lifecycle_controls_present"],
            "covered": packed_controls["existing_commitment_and_repacked_checks"][
                "covered"
            ],
            "not_covered": packed_controls["existing_commitment_and_repacked_checks"][
                "not_covered"
            ],
            "commitment_presence_checks": commitment_checks,
            "lifecycle_presence_checks": lifecycle_checks,
        },
        "missing_selected_route_evidence": {
            "selected_route_kats_present": False,
            "selected_route_timing_rows_present": bool(timing_rows),
            "selected_route_timing_rows": timing_rows,
            "selected_route_diagnostic_capture_present": False,
            "broader_reject_equivalence_row": {
                "classification": broader_row["classification"],
                "basis": broader_row["basis"],
                "blockers": broader_row["blockers"],
            },
            "missing_rows": expected_missing_rows,
        },
        "required_kats": selected_route_kats(),
        "required_timing_harness": timing_harness_spec(),
        "fail_closed_blockers": [
            "no selected-route packed witness negative KATs",
            "no selected-route witness timing rows under the final 308-byte/Fq12 verifier",
            "no diagnostic capture proving packed witness, hidden-source, carrier, pad, transcript, and core proof-internal rejects collapse to one public class",
            "packed-witness same-game mutation remains only partially controlled by slot-bound commitment/repack checks",
        ],
        "global_evidence": {
            "timing_gates": timing.get("gates", {}),
            "timing_policy": timing.get("policy", {}),
            "timing_blockers": timing.get("blockers", []),
            "audit_instrumentation_evidence": ct_audit.get(
                "instrumentation_evidence", {}
            ),
        },
        "source_artifacts": [
            artifact(REJECT_EQUIVALENCE_JSON, "source reject-equivalence contract"),
            artifact(
                REJECT_EQUIVALENCE_DISCHARGE_JSON,
                "broader fail-closed reject-equivalence ledger",
            ),
            artifact(
                PACKED_WITNESS_CONTROLS_JSON,
                "commitment/repacked witness controls and residual same-game blockers",
            ),
            artifact(CT_AUDIT_JSON, "diagnostic and hidden-material policy"),
            artifact(TIMING_CLOSURE_JSON, "PTC-006 timing policy"),
            artifact(FAULT_CAMPAIGN_JSON, "sampled timing/fault campaign rows"),
        ],
        "source_files_reviewed": [
            "scripts/research/pre_signature_projection_reject_equivalence_discharge.py",
            "scripts/research/pre_signature_projection_packed_witness_rewrite_controls.py",
            rel(PRODUCTION_REFERENCE_RS),
        ],
        "artifacts": {"json": rel(OUT_JSON), "markdown": rel(OUT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    decision = payload["decision"]
    missing = payload["missing_selected_route_evidence"]
    lines = [
        "# REQ-WITNESS Reject Discharge",
        "",
        "Fail-closed artifact for packed-witness opening rejection equivalence on",
        "the selected 308-byte/Fq12 state-assisted masked projection-carrier path.",
        "",
        "## Decision",
        "",
        f"- status: `{payload['status']}`",
        f"- classification: `{decision['classification']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- can remove `REQ-WITNESS`: `{decision['can_remove_REQ_WITNESS_from_A_reject_equivalence']}`",
        f"- selected-route diagnostic equivalence: `{decision['can_claim_selected_route_diagnostic_equivalence']}`",
        f"- selected-route timing equivalence: `{decision['can_claim_selected_route_timing_equivalence']}`",
        f"- existing commitment/repack controls usable: `{decision['can_use_existing_commitment_repack_controls']}`",
        f"- reason: {decision['reason']}",
        "",
        "## Witness Failure Cases",
        "",
        "| case | classification | missing KAT | diagnostic requirement |",
        "|---|---|---|---|",
    ]
    for item in payload["witness_failure_cases"]:
        lines.append(
            f"| `{item['id']}` | `{item['classification']}` | `{item['missing_kat']}` | {item['diagnostic_requirement']} |"
        )

    controls = payload["existing_commitment_and_repacked_tests"]
    lines.extend(
        [
            "",
            "## Existing Commitment And Repacked Tests",
            "",
            f"- source: `{controls['source']}`",
            f"- slot-bound subset discharged: `{controls['slot_bound_subset_discharged']}`",
            f"- lifecycle controls present: `{controls['lifecycle_controls_present']}`",
            "",
            "Covered:",
        ]
    )
    lines.extend(f"- {item}" for item in controls["covered"])
    lines.extend(["", "Not covered:"])
    lines.extend(f"- {item}" for item in controls["not_covered"])

    lines.extend(["", "Presence checks:"])
    for item in controls["commitment_presence_checks"]:
        lines.append(f"- `{item['id']}`: `{item['present']}`")

    lines.extend(
        [
            "",
            "## Missing Selected-Route Evidence",
            "",
            f"- selected-route KATs present: `{missing['selected_route_kats_present']}`",
            f"- selected-route timing rows present: `{missing['selected_route_timing_rows_present']}`",
            f"- selected-route diagnostic capture present: `{missing['selected_route_diagnostic_capture_present']}`",
            f"- broader row classification: `{missing['broader_reject_equivalence_row']['classification']}`",
            f"- broader row basis: {missing['broader_reject_equivalence_row']['basis']}",
            "",
            "Missing rows:",
        ]
    )
    lines.extend(f"- `{item}`" for item in missing["missing_rows"])

    lines.extend(["", "## Required KATs", ""])
    for item in payload["required_kats"]:
        lines.append(f"### {item['id']}")
        lines.append("")
        lines.append(f"- mutation: {item['mutation']}")
        lines.append("- assertions:")
        lines.extend(f"  - {assertion}" for assertion in item["assertions"])
        lines.append("")

    harness = payload["required_timing_harness"]
    lines.extend(
        [
            "## Required Timing Harness",
            "",
            f"- id: `{harness['id']}`",
            f"- entrypoint: {harness['entrypoint']}",
            f"- sample policy: `{json.dumps(harness['sample_policy'], sort_keys=True)}`",
            "",
            "Witness timing rows:",
        ]
    )
    lines.extend(f"- `{item}`" for item in harness["witness_rows"])
    lines.extend(["", "Diagnostic capture:"])
    lines.extend(f"- {item}" for item in harness["diagnostic_capture"])

    lines.extend(["", "## Fail-Closed Blockers", ""])
    lines.extend(f"- {item}" for item in payload["fail_closed_blockers"])

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
    OUT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": rel(OUT_JSON),
                "markdown": rel(OUT_MD),
                "status": payload["status"],
                "can_remove_REQ_WITNESS": payload["decision"][
                    "can_remove_REQ_WITNESS_from_A_reject_equivalence"
                ],
                "required_kats": len(payload["required_kats"]),
                "selected_route_timing_rows_present": payload[
                    "missing_selected_route_evidence"
                ]["selected_route_timing_rows_present"],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
