#!/usr/bin/env python3
"""Generate the projection-carrier reject-equivalence artifact.

This is a fail-closed proof contract for the 308-byte/Fq12 state-assisted
masked projection-carrier path.  It does not modify the simulator.  Instead it
spells out the real/simulated reject classes that must collapse to the same
public boundary behavior, and records which timing/equalization evidence already
exists versus which deployment gates remain open.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
OUT_JSON = READINESS_DIR / "pre_signature_projection_reject_equivalence_n32_q12289.json"
OUT_MD = READINESS_DIR / "pre_signature_projection_reject_equivalence_n32_q12289.md"

SIMULATOR_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.json"
)
CT_SUMMARY_JSON = READINESS_DIR / "constant_time_issue_review_summary_n32_q12289.json"
CT_AUDIT_JSON = READINESS_DIR / "constant_time_side_channel_audit_memo_n32_q12289.json"
TIMING_CLOSURE_JSON = READINESS_DIR / "side_channel_timing_closure_n32_q12289.json"
FACADE_TIMING_JSON = (
    READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.json"
)

SIMULATOR_SCRIPT = (
    ROOT / "scripts/research/pre_signature_projection_mask_prf_leakage_simulator.py"
)
CT_AUDIT_SCRIPT = ROOT / "scripts/research/constant_time_side_channel_audit_memo.py"
CT_SUMMARY_SCRIPT = ROOT / "scripts/research/constant_time_issue_review_summary.py"
FACADE_TIMING_RS = (
    ROOT / "tools/production-reference/src/bin/production_reference_facade_timing.rs"
)
STATEFUL_RS = (
    ROOT / "tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    return json.loads(path.read_text())


def artifact_row(path: Path, role: str, status: str | None = None) -> dict[str, Any]:
    payload = load_json(path)
    inferred_status = "missing"
    if payload:
        inferred_status = (
            payload.get("status")
            or payload.get("closure_status")
            or payload.get("current_claim")
            or "present_no_status_field"
        )
    return {
        "artifact": rel(path),
        "present": payload is not None,
        "schema": payload.get("schema") if payload else None,
        "status": status or inferred_status,
        "role": role,
    }


def failure_classes() -> list[dict[str, Any]]:
    return [
        {
            "id": "REQ-PARSE",
            "surface": "parsing",
            "real_failure_classes": [
                "signature length is not the selected 308-byte transaction length",
                "slot-id field is unavailable or noncanonical",
                "challenge frame has nonzero slack or invalid algebraic mask bits",
                "response/opening/proof-carrier segments are truncated, extended, or noncanonical",
            ],
            "simulated_failure_classes": [
                "simulated transcript cannot be parsed into the selected public frame",
                "simulated carrier bytes do not decode to canonical q12289 residues",
                "simulated opening/proof segment does not match the public byte layout",
            ],
            "public_reject_surface": "reject_fail_closed_malformed_public_frame",
            "secret_dependent_diagnostics_allowed": False,
            "state_transition_on_reject": "none",
            "equivalence_rule": (
                "Both worlds may reveal only public byte-layout failure.  The "
                "boundary must not reveal which hidden lane or witness-derived "
                "component would have failed after parsing."
            ),
        },
        {
            "id": "REQ-SLOT",
            "surface": "slot_state",
            "real_failure_classes": [
                "slot id is outside the public account-state slot table",
                "slot is already consumed or reserved under an incompatible epoch",
                "restore anchor is stale or not trusted by the canonical state adapter",
                "spent-ledger key for public-key hash and slot id is already present",
            ],
            "simulated_failure_classes": [
                "simulator slot oracle has no live slot for the public slot id",
                "simulated lifecycle trace marks the slot consumed or replayed",
                "simulated restore-anchor freshness check fails",
            ],
            "public_reject_surface": "reject_fail_closed_public_slot_state",
            "secret_dependent_diagnostics_allowed": False,
            "state_transition_on_reject": "none",
            "equivalence_rule": (
                "Slot-state rejects are public-account-state rejects in both "
                "worlds.  They may expose public slot/replay status but no pad "
                "seed, witness opening, or projection residue information."
            ),
        },
        {
            "id": "REQ-PAD",
            "surface": "pad_opening",
            "real_failure_classes": [
                "pad opening frame is missing or malformed",
                "pad seed digest does not bind to the public account commitment",
                "pad PRF domain, slot id, lane id, or counter frame is inconsistent",
                "derived local pads do not match the opened local response frame",
            ],
            "simulated_failure_classes": [
                "simulated pad-opening token is absent for an accepted spend",
                "simulated pad digest or domain frame is inconsistent with public state",
                "simulated pad stream does not satisfy the public opening predicate",
            ],
            "public_reject_surface": "reject_fail_closed_invalid_opening",
            "secret_dependent_diagnostics_allowed": False,
            "state_transition_on_reject": "none",
            "equivalence_rule": (
                "Pad-opening failures collapse to invalid opening.  The boundary "
                "must not report pad-seed bytes, retry counts, rejection-sampler "
                "internals, or lane-local mismatch positions."
            ),
        },
        {
            "id": "REQ-WITNESS",
            "surface": "packed_witness_opening",
            "real_failure_classes": [
                "packed witness opening is absent, wrong length, or has nonzero slack",
                "18 q12289 residues do not unpack canonically",
                "12 checked-core residues do not bind to the public commitment digest",
                "packed opening does not match hidden-source opening commitment",
            ],
            "simulated_failure_classes": [
                "simulated packed witness opening cannot be produced for the slot",
                "simulated 18-row vector is noncanonical",
                "simulated checked-core prefix fails the public commitment relation",
            ],
            "public_reject_surface": "reject_fail_closed_invalid_opening",
            "secret_dependent_diagnostics_allowed": False,
            "state_transition_on_reject": "none",
            "equivalence_rule": (
                "Witness-opening failures are indistinguishable from other invalid "
                "opening rejects except for public frame class.  No diagnostic may "
                "identify one of the six hidden projection rows."
            ),
        },
        {
            "id": "REQ-CARRIER",
            "surface": "projection_carrier",
            "real_failure_classes": [
                "projection-carrier commitment or proof segment is malformed",
                "six masked projection residues are noncanonical modulo 12289",
                "carrier lanes do not bind to the selected slot id, pad seed digest, and account commitment",
                "unmasked projection residues do not match the opened packed witness under the mask PRF frame",
            ],
            "simulated_failure_classes": [
                "simulated carrier lanes are malformed or noncanonical",
                "simulated uniform masked lanes do not pass the public carrier binding predicate",
                "simulated opening condition cannot reconcile carrier lanes with the accepted witness opening",
            ],
            "public_reject_surface": "reject_fail_closed_invalid_projection_carrier",
            "secret_dependent_diagnostics_allowed": False,
            "state_transition_on_reject": "none",
            "equivalence_rule": (
                "The real and simulated carrier reject on the same public predicate. "
                "The production boundary may not say whether the failed input was "
                "a mask PRF lane, raw projection residue, or packed-witness lane."
            ),
        },
        {
            "id": "REQ-TRANSCRIPT",
            "surface": "transcript_binding",
            "real_failure_classes": [
                "message, public key, account state, slot id, or proof commitment is not bound into the challenge",
                "Fiat-Shamir challenge bytes are inconsistent with the transcript frame",
                "domain label or profile id does not match the selected 308-byte/Fq12 route",
            ],
            "simulated_failure_classes": [
                "simulated transcript frame does not match the adversary-visible public transcript",
                "simulated challenge oracle transcript is inconsistent with public inputs",
                "simulated profile/domain label is outside the selected route",
            ],
            "public_reject_surface": "reject_fail_closed_invalid_transcript_binding",
            "secret_dependent_diagnostics_allowed": False,
            "state_transition_on_reject": "none",
            "equivalence_rule": (
                "Transcript-binding rejects may depend on public transcript bytes "
                "only.  No callback or proof-internal diagnostic can be surfaced."
            ),
        },
        {
            "id": "REQ-CORE",
            "surface": "checked_core_failures",
            "real_failure_classes": [
                "12-row checked-core packed residues fail canonical q12289 decoding",
                "checked-core syndrome or sigma equation fails",
                "response bounds or local-opening equations fail",
                "accepted core would not consume exactly the public slot being opened",
            ],
            "simulated_failure_classes": [
                "simulated checked-core residues fail the same public equation",
                "simulated response/opening tuple is outside the accepted relation",
                "simulated consume target does not equal the public slot id",
            ],
            "public_reject_surface": "reject_fail_closed_invalid_checked_core",
            "secret_dependent_diagnostics_allowed": False,
            "state_transition_on_reject": "none",
            "equivalence_rule": (
                "Checked-core failures are relation failures over public checked "
                "fields.  The boundary must keep the same reject schedule whether "
                "the hidden projection lanes are real masked residues or simulator "
                "uniform replacements."
            ),
        },
    ]


def build_payload() -> dict[str, Any]:
    simulator = load_json(SIMULATOR_JSON) or {}
    ct_summary = load_json(CT_SUMMARY_JSON) or {}
    ct_audit = load_json(CT_AUDIT_JSON) or {}
    timing = load_json(TIMING_CLOSURE_JSON) or {}
    facade = load_json(FACADE_TIMING_JSON) or {}

    timing_report = facade.get("timing_report", {})
    facade_timings = {
        row["operation"]: row
        for row in timing_report.get("timings", [])
        if isinstance(row, dict) and "operation" in row
    }

    criterion = {
        "name": "constant_time_reject_equivalence",
        "definition": (
            "For every adversary-visible reject produced by the selected "
            "308-byte/Fq12 state-assisted masked projection-carrier verifier, the "
            "real and simulated executions must expose the same public reject "
            "surface, perform no state transition, emit no secret-dependent "
            "diagnostic, and stay inside a reviewed secret-independent timing "
            "envelope at the production boundary."
        ),
        "public_surface_components": [
            "accepted_or_rejected bit",
            "coarse public reject code derived from public byte layout, public account state, or public relation class",
            "public slot id only if it was parseable from adversary-supplied bytes",
            "public expected length/profile labels",
            "unchanged account/lifecycle/spent-ledger state on reject",
        ],
        "forbidden_surface_components": [
            "pad seed bytes or pad PRF lane outputs",
            "raw selected projection residues",
            "packed witness opening preimage",
            "hidden-source opening preimage",
            "secret-key, sampler randomness, entropy seed, or rejection-sampler retry counts",
            "fine-grained mismatch indices for hidden projection, pad, or packed-witness lanes",
            "callback-originated error strings at deployment boundary",
        ],
        "timing_requirement": (
            "The production path must use a constant-shape parser/verifier/signing "
            "boundary for all proof-internal reject classes, with public-input "
            "length/message costs either fixed by profile or explicitly outside "
            "the hidden-witness leakage game."
        ),
    }

    existing_evidence = [
        artifact_row(
            SIMULATOR_JSON,
            "source simulator contract naming reject-equivalence as the remaining negligible_rejection_mismatch gap",
        ),
        artifact_row(
            CT_SUMMARY_JSON,
            "aggregated issue review: sampled rows have no proven secret-dependent leak but known timing cliffs remain",
        ),
        artifact_row(
            CT_AUDIT_JSON,
            "surface classification and sanitizer evidence for public reports and callback-originated errors",
        ),
        artifact_row(
            TIMING_CLOSURE_JSON,
            "fault-campaign timing closure status; fail-closed outcomes pass but timing-ratio policy is open",
        ),
        artifact_row(
            FACADE_TIMING_JSON,
            "production-reference crate facade timings with deterministic dummy callbacks only",
        ),
    ]

    timing_and_equalization_evidence = {
        "fail_closed_fault_campaign_passed": timing.get("gates", {}).get(
            "sampled_fail_closed"
        ),
        "sampled_timing_ratio_policy_passed": timing.get("gates", {}).get(
            "sampled_timing_ratio_policy"
        ),
        "final_production_verifier_semantics_passed": timing.get("gates", {}).get(
            "final_production_verifier_semantics"
        ),
        "constant_time_implementation_review_passed": timing.get("gates", {}).get(
            "constant_time_implementation_review"
        ),
        "timing_policy_issue_count": timing.get("policy", {}).get(
            "timing_policy_issue_count"
        ),
        "ct_issue_counts": {
            "issues_reviewed": ct_summary.get("issue_count"),
            "not_secret_dependent": ct_summary.get("not_secret_dependent_count"),
            "diagnostic_harness_issues": ct_summary.get(
                "diagnostic_harness_issue_count"
            ),
            "secret_adjacent": ct_summary.get("secret_adjacent_count"),
        },
        "sanitizer_evidence": ct_audit.get("instrumentation_evidence", {}),
        "facade_timing_caveat": timing_report.get("caveat"),
        "facade_timing_operations": facade_timings,
    }

    remaining_gaps = [
        {
            "id": "selected_308_fq12_verifier_semantics",
            "status": "open",
            "blocker": (
                "Existing production-reference timing rows use deterministic dummy "
                "callbacks and legacy 254-byte proof facade semantics, not an "
                "accepting 308-byte/Fq12 projection-carrier verifier."
            ),
        },
        {
            "id": "constant_shape_projection_carrier_verifier",
            "status": "open",
            "blocker": (
                "No implementation-level review or timing harness yet shows that "
                "pad opening, packed witness opening, projection-carrier binding, "
                "transcript binding, and checked-core failures run under one "
                "secret-independent proof-internal schedule."
            ),
        },
        {
            "id": "parser_and_public_state_equalization",
            "status": "open",
            "blocker": (
                "Existing CT issue artifacts diagnose public-input and account-state "
                "timing cliffs; these are not hidden-witness leaks under the current "
                "classification, but deployment still requires constant-shape public "
                "parser/verifier remediation and remeasurement."
            ),
        },
        {
            "id": "secret_side_signer_sampler_boundary",
            "status": "open",
            "blocker": (
                "Signer callback, sampler, keygen, entropy separation, and retry "
                "shape remain unaudited for constant-time behavior."
            ),
        },
        {
            "id": "production_boundary_error_policy_for_selected_route",
            "status": "open",
            "blocker": (
                "Sanitized callback wrappers and public rejection-report KATs exist, "
                "but the selected projection-carrier route still needs route-specific "
                "negative tests proving hidden-source/projection diagnostics are not emitted."
            ),
        },
    ]

    fail_closed_result = {
        "constant_time_reject_equivalence_gap_status": "closed_as_fail_closed_contract",
        "deployment_ready": False,
        "simulator_gap_resolution": (
            "The simulator can replace negligible_rejection_mismatch with this "
            "explicit fail-closed reject-equivalence obligation: all listed "
            "real/simulated rejects collapse to public reject surfaces, while "
            "deployment timing and selected-verifier implementation evidence "
            "remain open blockers."
        ),
        "can_consume_in_mask_simulator": True,
        "can_claim_constant_time": False,
        "can_claim_selected_308_byte_route_deployment_ready": False,
    }

    return {
        "schema": "Descender.pre_signature_projection_reject_equivalence.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "constant_time_reject_equivalence_fail_closed_artifact",
        "target": {
            "profile": "Descender.Signature.q12289.local_delta.stateful.masked_projection_carrier.v1",
            "n": 32,
            "q": 12289,
            "selected_signature_bytes": 308,
            "packed_witness_rows": 18,
            "checked_core_rows": 12,
            "masked_projection_rows": 6,
            "source_simulator_status": simulator.get("status"),
        },
        "criterion": criterion,
        "failure_classes": failure_classes(),
        "existing_evidence": existing_evidence,
        "timing_and_equalization_evidence": timing_and_equalization_evidence,
        "remaining_gaps": remaining_gaps,
        "fail_closed_result": fail_closed_result,
        "source_files_reviewed": [
            rel(SIMULATOR_SCRIPT),
            rel(SIMULATOR_JSON),
            rel(CT_AUDIT_SCRIPT),
            rel(CT_SUMMARY_SCRIPT),
            rel(FACADE_TIMING_RS),
            rel(STATEFUL_RS),
            rel(CT_AUDIT_JSON),
            rel(CT_SUMMARY_JSON),
            rel(TIMING_CLOSURE_JSON),
            rel(FACADE_TIMING_JSON),
        ],
        "artifacts": {"json": rel(OUT_JSON), "markdown": rel(OUT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    result = payload["fail_closed_result"]
    timing = payload["timing_and_equalization_evidence"]
    lines = [
        "# Pre-Signature Projection Reject Equivalence",
        "",
        "This artifact closes the mask-simulator reject-equivalence gap as a",
        "fail-closed contract for the selected 308-byte/Fq12 state-assisted",
        "masked projection-carrier path.  It is not a deployment timing claim.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- simulator gap status: `{result['constant_time_reject_equivalence_gap_status']}`",
        f"- can consume in mask simulator: `{result['can_consume_in_mask_simulator']}`",
        f"- can claim constant time: `{result['can_claim_constant_time']}`",
        f"- deployment ready: `{result['deployment_ready']}`",
        f"- selected signature bytes: `{payload['target']['selected_signature_bytes']}`",
        f"- packed witness rows: `{payload['target']['packed_witness_rows']}`",
        f"- checked-core rows: `{payload['target']['checked_core_rows']}`",
        f"- masked projection rows: `{payload['target']['masked_projection_rows']}`",
        "",
        "## Equivalence Criterion",
        "",
        payload["criterion"]["definition"],
        "",
        "Allowed public surface:",
    ]
    lines.extend(f"- {item}" for item in payload["criterion"]["public_surface_components"])
    lines.extend(["", "Forbidden surface:"])
    lines.extend(f"- {item}" for item in payload["criterion"]["forbidden_surface_components"])
    lines.extend(
        [
            "",
            f"Timing requirement: {payload['criterion']['timing_requirement']}",
            "",
            "## Failure Classes",
            "",
            "| id | surface | public reject surface | equivalence rule |",
            "|---|---|---|---|",
        ]
    )
    for item in payload["failure_classes"]:
        lines.append(
            f"| `{item['id']}` | `{item['surface']}` | `{item['public_reject_surface']}` | {item['equivalence_rule']} |"
        )

    lines.extend(["", "## Real/Simulated Class Matrix", ""])
    for item in payload["failure_classes"]:
        lines.extend(
            [
                f"### {item['id']} `{item['surface']}`",
                "",
                "Real failures:",
            ]
        )
        lines.extend(f"- {row}" for row in item["real_failure_classes"])
        lines.append("")
        lines.append("Simulated failures:")
        lines.extend(f"- {row}" for row in item["simulated_failure_classes"])
        lines.append("")

    lines.extend(
        [
            "## Timing And Equalization Evidence",
            "",
            f"- fail-closed sampled outcomes passed: `{timing['fail_closed_fault_campaign_passed']}`",
            f"- sampled timing-ratio policy passed: `{timing['sampled_timing_ratio_policy_passed']}`",
            f"- final production verifier semantics passed: `{timing['final_production_verifier_semantics_passed']}`",
            f"- constant-time implementation review passed: `{timing['constant_time_implementation_review_passed']}`",
            f"- timing policy issue count: `{timing['timing_policy_issue_count']}`",
            f"- CT issues reviewed: `{timing['ct_issue_counts']['issues_reviewed']}`",
            f"- not secret-dependent sampled issues: `{timing['ct_issue_counts']['not_secret_dependent']}`",
            f"- diagnostic harness issues: `{timing['ct_issue_counts']['diagnostic_harness_issues']}`",
            f"- secret-adjacent sampled issues: `{timing['ct_issue_counts']['secret_adjacent']}`",
            f"- facade timing caveat: {timing['facade_timing_caveat']}",
            "",
            "Facade timing rows:",
            "",
            "| operation | avg ns | accepted/ok |",
            "|---|---:|---|",
        ]
    )
    for row in timing["facade_timing_operations"].values():
        lines.append(
            f"| `{row['operation']}` | {row['avg_ns']} | `{row['accepted_or_ok']}` |"
        )

    lines.extend(
        [
            "",
            "## Evidence Artifacts",
            "",
            "| artifact | present | status | role |",
            "|---|---|---|---|",
        ]
    )
    for item in payload["existing_evidence"]:
        lines.append(
            f"| `{item['artifact']}` | `{item['present']}` | `{item['status']}` | {item['role']} |"
        )

    lines.extend(
        [
            "",
            "## Remaining Gaps",
            "",
            "| gap | status | blocker |",
            "|---|---|---|",
        ]
    )
    for gap in payload["remaining_gaps"]:
        lines.append(f"| `{gap['id']}` | `{gap['status']}` | {gap['blocker']} |")

    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- {result['simulator_gap_resolution']}",
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
                "json": str(OUT_JSON),
                "markdown": str(OUT_MD),
                "status": payload["status"],
                "gap_status": payload["fail_closed_result"][
                    "constant_time_reject_equivalence_gap_status"
                ],
                "deployment_ready": payload["fail_closed_result"]["deployment_ready"],
                "remaining_gaps": len(payload["remaining_gaps"]),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
