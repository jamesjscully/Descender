#!/usr/bin/env python3
"""Discharge ledger for A_reject_equivalence.

This artifact is intentionally fail-closed.  It maps the seven reject classes
from the selected 308-byte/Fq12 state-assisted masked projection-carrier
contract to the timing, equalization, and diagnostic evidence that exists today,
then names the selected-route tests needed before any deployment discharge.
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
    / "pre_signature_projection_reject_equivalence_discharge_n32_q12289.json"
)
OUT_MD = (
    READINESS_DIR
    / "pre_signature_projection_reject_equivalence_discharge_n32_q12289.md"
)

REJECT_EQUIVALENCE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_n32_q12289.json"
)
THEOREM_COMPOSITION_JSON = (
    READINESS_DIR
    / "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
)
CT_SUMMARY_JSON = READINESS_DIR / "constant_time_issue_review_summary_n32_q12289.json"
CT_AUDIT_JSON = READINESS_DIR / "constant_time_side_channel_audit_memo_n32_q12289.json"
TIMING_CLOSURE_JSON = READINESS_DIR / "side_channel_timing_closure_n32_q12289.json"
FAULT_CAMPAIGN_JSON = (
    READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
)
FACADE_TIMING_JSON = (
    READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.json"
)

SCRIPT_SOURCES = [
    Path("scripts/research/pre_signature_projection_mask_prf_leakage_theorem_composition.py"),
    Path("scripts/research/pre_signature_projection_reject_equivalence.py"),
    Path("scripts/research/constant_time_side_channel_audit_memo.py"),
    Path("scripts/research/constant_time_issue_review_summary.py"),
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
            or "present_no_status_field"
        ),
        "role": role,
    }


def rows_by_class(fault_campaign: dict[str, Any]) -> dict[str, dict[str, Any]]:
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


def issue_rows(ct_summary: dict[str, Any], classes: list[str]) -> list[dict[str, Any]]:
    wanted = set(classes)
    return [
        {
            "id": row["id"],
            "class": row["class"],
            "secret_risk": row["secret_risk"],
            "root_cause_class": row["root_cause_class"],
            "remediation_class": row["remediation_class"],
            "artifact": row["artifact"],
        }
        for row in ct_summary.get("rows", [])
        if row.get("class") in wanted
    ]


def timing_rows(
    campaign_rows: dict[str, dict[str, Any]], classes: list[str]
) -> list[dict[str, Any]]:
    return [campaign_rows[name] for name in classes if name in campaign_rows]


def selected_rows_by_class(facade: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = facade.get("timing_report", {}).get("selected_projection_timings", [])
    return {
        row["reject_class"]: {
            "operation": row.get("operation"),
            "reject_class": row.get("reject_class"),
            "iterations": row.get("iterations"),
            "avg_ns": row.get("avg_ns"),
            "avg_us": row.get("avg_us"),
            "ratio_to_valid_baseline": row.get("ratio_to_valid_baseline"),
            "expected_ok": row.get("expected_ok"),
            "ok": row.get("ok"),
            "expected_public_reason": row.get("expected_public_reason"),
            "observed_public_reason": row.get("observed_public_reason"),
        }
        for row in rows
        if isinstance(row, dict) and "reject_class" in row
    }


def class_entry(
    failure_class: dict[str, Any],
    *,
    status: str,
    basis: str,
    evidence: list[str],
    timing: list[dict[str, Any]],
    ct_issues: list[dict[str, Any]],
    selected_route_tests_needed: list[str],
    timing_harness_needed: list[str],
    blockers: list[str],
    selected_reference_timing: list[dict[str, Any]] | None = None,
) -> dict[str, Any]:
    return {
        "id": failure_class["id"],
        "surface": failure_class["surface"],
        "public_reject_surface": failure_class["public_reject_surface"],
        "classification": status,
        "basis": basis,
        "existing_evidence": evidence,
        "timing_rows": timing,
        "selected_reference_timing_rows": selected_reference_timing or [],
        "ct_issue_rows": ct_issues,
        "selected_route_tests_needed": selected_route_tests_needed,
        "timing_harness_needed": timing_harness_needed,
        "blockers": blockers,
    }


def build_payload() -> dict[str, Any]:
    reject = load_json(REJECT_EQUIVALENCE_JSON)
    theorem = load_json(THEOREM_COMPOSITION_JSON)
    ct_summary = load_json(CT_SUMMARY_JSON)
    ct_audit = load_json(CT_AUDIT_JSON)
    timing = load_json(TIMING_CLOSURE_JSON)
    fault_campaign = load_json(FAULT_CAMPAIGN_JSON)
    facade = load_json(FACADE_TIMING_JSON)

    failure_classes = {row["id"]: row for row in reject["failure_classes"]}
    campaign_rows = rows_by_class(fault_campaign)
    selected_rows = selected_rows_by_class(facade)
    audit_evidence = ct_audit.get("instrumentation_evidence", {})

    parse_classes = [
        "empty_signature",
        "slot_only_signature",
        "truncated_by_one",
        "extended_by_one",
        "invalid_challenge_mask",
        "slot_only_signature_equalized",
        "truncated_by_one_equalized",
        "extended_by_one_equalized",
        "invalid_challenge_mask_equalized",
    ]
    slot_classes = ["slot_out_of_range", "slot_already_consumed"]
    transcript_classes = [
        "challenge_hash_long_message",
        "challenge_hash_long_message_equalized",
    ]
    core_classes = ["proof_commitment_mismatch"]

    selected_route_blocker = (
        "existing timing/fault rows exercise diagnostic reference APIs, dummy "
        "callbacks, or legacy facade semantics; they do not run the final "
        "accepting 308-byte/Fq12 projection-carrier verifier"
    )
    timing_policy_blocker = (
        "PTC-006 fail-closed behavior passes, but sampled timing-ratio policy, "
        "final verifier semantics, and constant-time implementation review do not"
    )

    discharge = [
        class_entry(
            failure_classes["REQ-PARSE"],
            status="partially_supported",
            basis=(
                "Malformed public-frame rows reject fail-closed and equalized "
                "diagnostic parser rows exist.  Public parser timing cliffs remain "
                "and the route is not the selected 308-byte/Fq12 verifier."
            ),
            evidence=[
                "PTC-006 malformed-input fail-closed rows",
                "PTC-006 equalized_decode_diagnostic rows",
                "CT-001..CT-009 public parser issue review",
            ],
            timing=timing_rows(campaign_rows, parse_classes),
            ct_issues=issue_rows(ct_summary, parse_classes),
            selected_route_tests_needed=[
                "selected_projection_reject_parse_empty_308_frame",
                "selected_projection_reject_parse_truncated_307_frame",
                "selected_projection_reject_parse_extended_309_frame",
                "selected_projection_reject_parse_noncanonical_slot_id",
                "selected_projection_reject_parse_noncanonical_carrier_residue",
            ],
            timing_harness_needed=[
                "selected_projection_reject_parse_timing: compare valid 308-byte baseline, empty, short, long, bad slot, bad challenge slack, and bad carrier residue under one hardened verifier entrypoint",
                "require all parser rows to pass the 0.80..1.25 PTC-006 policy window after fixed-buffer normalization",
            ],
            blockers=[selected_route_blocker, timing_policy_blocker],
        ),
        class_entry(
            failure_classes["REQ-SLOT"],
            status="partially_supported",
            basis=(
                "Public account-state slot rejects are sampled and fail closed; "
                "audit artifacts classify lifecycle/spent-ledger records as public "
                "state.  Slot timing cliffs remain, and selected-route stateful "
                "carrier verification is not measured."
            ),
            evidence=[
                "PTC-006 slot_out_of_range and slot_already_consumed rows",
                "constant-time audit public account lifecycle classification",
                "spent-ledger and lifecycle serialization KAT evidence",
            ],
            timing=timing_rows(campaign_rows, slot_classes),
            ct_issues=issue_rows(ct_summary, slot_classes),
            selected_route_tests_needed=[
                "selected_projection_reject_slot_out_of_range_preserves_state",
                "selected_projection_reject_slot_already_consumed_preserves_state",
                "selected_projection_reject_stale_restore_anchor_preserves_state",
                "selected_projection_reject_duplicate_spent_key_preserves_state",
            ],
            timing_harness_needed=[
                "selected_projection_reject_slot_timing: compare missing, consumed, stale-anchor, duplicate-spent, proof-mismatch, and valid-spend rows with identical account snapshot shape",
                "assert lifecycle, trusted-anchor, and spent-ledger stores are byte-identical before/after every reject row",
            ],
            blockers=[selected_route_blocker, timing_policy_blocker],
        ),
        class_entry(
            failure_classes["REQ-PAD"],
            status="same_harness_reference_timed_not_discharged",
            basis=(
                "General sanitizer and PRF diagnostic evidence exists, and the "
                "same-harness selected 308-byte/Fq12 candidate-verifier timing "
                "report now includes a pad-seed rebinding reject row.  This does "
                "not cover the full pad-opening schedule or prove production "
                "constant-time behavior."
            ),
            evidence=[
                "constant-time audit sanitized callback and rejection-report KATs",
                "PTC-006 pad PRF seed-class diagnostic rows",
                "production_reference_facade_timing selected_projection_reject_pad_seed_rebinding_308_byte_fq12 row",
            ],
            timing=timing_rows(
                campaign_rows,
                ["pad_prf_zero_seed", "pad_prf_one_seed", "pad_prf_ff_seed"],
            ),
            selected_reference_timing=[
                selected_rows["pad"]
            ] if "pad" in selected_rows else [],
            ct_issues=[],
            selected_route_tests_needed=[
                "selected_projection_reject_pad_opening_missing",
                "selected_projection_reject_pad_seed_digest_mismatch",
                "selected_projection_reject_pad_domain_slot_lane_counter_mismatch",
                "selected_projection_reject_pad_stream_response_mismatch",
            ],
            timing_harness_needed=[
                "selected_projection_reject_pad_timing: mutate one pad predicate at a time after full 308-byte decode and compare against carrier, witness, transcript, core, and valid rows",
                "assert rejection reports never include pad seed bytes, PRF lane outputs, retry counts, or lane-local mismatch indices",
            ],
            blockers=[
                "same-harness selected 308-byte/Fq12 pad reject timing row now exists for the reference candidate verifier only",
                "no implementation-level review of selected pad-opening reject schedule",
            ],
        ),
        class_entry(
            failure_classes["REQ-WITNESS"],
            status="same_harness_reference_timed_not_discharged",
            basis=(
                "A same-harness selected 308-byte/Fq12 candidate-verifier row now "
                "mutates the witness-opening digest and observes the collapsed "
                "public reject reason.  This is reference timing evidence only; "
                "it is not a final verifier or constant-time proof."
            ),
            evidence=[
                "constant-time audit forbids hidden account witness material in logs/stores/errors",
                "reject-equivalence contract names packed witness failures",
                "production_reference_facade_timing selected_projection_reject_witness_opening_digest_308_byte_fq12 row",
            ],
            timing=[],
            selected_reference_timing=[
                selected_rows["witness"]
            ] if "witness" in selected_rows else [],
            ct_issues=[],
            selected_route_tests_needed=[
                "selected_projection_reject_witness_opening_missing",
                "selected_projection_reject_witness_wrong_length_or_nonzero_slack",
                "selected_projection_reject_witness_noncanonical_q12289_residue",
                "selected_projection_reject_witness_commitment_mismatch",
                "selected_projection_reject_hidden_source_opening_mismatch",
            ],
            timing_harness_needed=[
                "selected_projection_reject_witness_timing: compare each 18-row packed-witness mutation with pad, carrier, transcript, core, and valid rows",
                "assert diagnostics collapse to invalid_opening and never name one of the six hidden projection rows",
            ],
            blockers=[
                "same-harness selected 308-byte/Fq12 witness reject timing row exists for the reference candidate verifier only",
                "final production verifier semantics and implementation-level constant-time review remain open",
            ],
        ),
        class_entry(
            failure_classes["REQ-CARRIER"],
            status="same_harness_reference_timed_not_discharged",
            basis=(
                "The selected timing report now includes a carrier-lane mutation "
                "row in the same candidate-verifier harness as the valid 308-byte "
                "baseline.  It still does not establish production constant-time "
                "behavior or remove the carried term."
            ),
            evidence=[
                "reject-equivalence contract defines the projection-carrier public reject surface",
                "production_reference_facade_timing selected_projection_reject_carrier_lane_308_byte_fq12 row",
            ],
            timing=timing_rows(
                campaign_rows,
                ["fallback_pad_projection_zero_seed"],
            ),
            selected_reference_timing=[
                selected_rows["carrier"]
            ] if "carrier" in selected_rows else [],
            ct_issues=[],
            selected_route_tests_needed=[
                "selected_projection_reject_carrier_commitment_malformed",
                "selected_projection_reject_carrier_noncanonical_masked_residue",
                "selected_projection_reject_carrier_rebound_slot_id",
                "selected_projection_reject_carrier_rebound_pad_seed_digest",
                "selected_projection_reject_carrier_unmasked_projection_mismatch",
            ],
            timing_harness_needed=[
                "selected_projection_reject_carrier_timing: run all six masked-lane mutation positions and compare aggregate timing to pad, witness, transcript, core, and valid rows",
                "assert public reports never distinguish mask PRF lane failure from raw projection or packed-witness lane failure",
            ],
            blockers=[
                "same-harness selected 308-byte/Fq12 carrier reject timing row exists for the reference candidate verifier only",
                "final production verifier semantics and implementation-level constant-time review remain open",
            ],
        ),
        class_entry(
            failure_classes["REQ-TRANSCRIPT"],
            status="same_harness_reference_timed_not_discharged",
            basis=(
                "Challenge-frame and public message-length timing diagnostics "
                "exist, including an equalized hash-frame control.  A same-harness "
                "selected candidate-verifier row now mutates the transcript message "
                "digest, but final verifier semantics and CT review remain open."
            ),
            evidence=[
                "PTC-006 challenge_hash_long_message row",
                "PTC-006 equalized_challenge_hash_frame_1024 control row",
                "CT-013 public message length hash-frame issue review",
                "production_reference_facade_timing selected_projection_reject_transcript_message_digest_308_byte_fq12 row",
            ],
            timing=timing_rows(campaign_rows, transcript_classes),
            selected_reference_timing=[
                selected_rows["transcript"]
            ] if "transcript" in selected_rows else [],
            ct_issues=issue_rows(ct_summary, ["challenge_hash_long_message"]),
            selected_route_tests_needed=[
                "selected_projection_reject_transcript_message_rebinding",
                "selected_projection_reject_transcript_public_key_rebinding",
                "selected_projection_reject_transcript_account_state_rebinding",
                "selected_projection_reject_transcript_slot_id_rebinding",
                "selected_projection_reject_transcript_wrong_profile_domain_label",
            ],
            timing_harness_needed=[
                "selected_projection_reject_transcript_timing: use fixed/prehashed message frames and compare challenge mismatch, domain mismatch, account-state mismatch, carrier mismatch, and valid rows",
                "assert reports expose only public transcript-binding class and no proof-internal callback diagnostics",
            ],
            blockers=[selected_route_blocker, timing_policy_blocker],
        ),
        class_entry(
            failure_classes["REQ-CORE"],
            status="same_harness_reference_timed_not_discharged",
            basis=(
                "A proof_commitment_mismatch row rejects fail-closed and is inside "
                "the sampled ratio window.  A same-harness selected candidate-verifier "
                "row now mutates the checked-core proof commitment inside the 308-byte "
                "envelope, but this is still reference evidence only."
            ),
            evidence=[
                "PTC-006 proof_commitment_mismatch row",
                "constant-time audit public rejection report classification",
                "production_reference_facade_timing selected_projection_reject_checked_core_proof_commitment_308_byte_fq12 row",
            ],
            timing=timing_rows(campaign_rows, core_classes),
            selected_reference_timing=[
                selected_rows["checked_core"]
            ] if "checked_core" in selected_rows else [],
            ct_issues=[],
            selected_route_tests_needed=[
                "selected_projection_reject_core_noncanonical_checked_residue",
                "selected_projection_reject_core_syndrome_equation_failure",
                "selected_projection_reject_core_response_bound_failure",
                "selected_projection_reject_core_local_opening_equation_failure",
                "selected_projection_reject_core_wrong_consume_slot",
            ],
            timing_harness_needed=[
                "selected_projection_reject_core_timing: compare checked-core equation failures with pad, witness, carrier, transcript, and valid rows under the same verifier schedule",
                "assert hidden projection lanes are not read or reported differently between real masked lanes and simulator-uniform replacements",
            ],
            blockers=[selected_route_blocker],
        ),
    ]

    status_counts: dict[str, int] = {}
    for row in discharge:
        status_counts[row["classification"]] = status_counts.get(row["classification"], 0) + 1

    carried = next(
        item
        for item in theorem.get("carried_assumptions", [])
        if item.get("id") == "A_reject_equivalence"
    )

    return {
        "schema": "Descender.pre_signature_projection_reject_equivalence_discharge.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "A_reject_equivalence_fail_closed_not_discharged",
        "deployment_ready": False,
        "target": reject["target"],
        "carried_term": carried,
        "decision": {
            "classification": "fail_closed_not_discharged",
            "can_consume_in_theorem_composition": True,
            "can_remove_A_reject_equivalence_term": False,
            "can_claim_selected_route_constant_time": False,
            "reason": (
                "The seven reject classes have a public-surface contract and "
                "some diagnostic/fail-closed evidence, but no final selected "
                "308-byte/Fq12 projection-carrier verifier harness establishes "
                "constant-shape timing and diagnostic equivalence."
            ),
        },
        "status_counts": dict(sorted(status_counts.items())),
        "global_evidence": {
            "timing_gates": timing.get("gates", {}),
            "timing_policy": timing.get("policy", {}),
            "timing_blockers": timing.get("blockers", []),
            "ct_issue_counts": {
                "issues_reviewed": ct_summary.get("issue_count"),
                "not_secret_dependent": ct_summary.get("not_secret_dependent_count"),
                "diagnostic_harness_issues": ct_summary.get(
                    "diagnostic_harness_issue_count"
                ),
                "secret_adjacent": ct_summary.get("secret_adjacent_count"),
            },
            "audit_instrumentation_evidence": audit_evidence,
            "facade_timing_caveat": facade.get("timing_report", {}).get("caveat"),
            "selected_projection_timing_caveat": facade.get("timing_report", {}).get(
                "selected_projection_caveat"
            ),
            "selected_projection_reference_rows": list(selected_rows.values()),
        },
        "failure_class_discharge": discharge,
        "source_artifacts": [
            artifact(REJECT_EQUIVALENCE_JSON, "source seven-class reject contract"),
            artifact(THEOREM_COMPOSITION_JSON, "carried-term parent composition"),
            artifact(CT_SUMMARY_JSON, "constant-time issue summary"),
            artifact(CT_AUDIT_JSON, "side-channel audit and diagnostic policy"),
            artifact(TIMING_CLOSURE_JSON, "PTC-006 closure gates"),
            artifact(FAULT_CAMPAIGN_JSON, "PTC-006 sampled timing/fault rows"),
            artifact(FACADE_TIMING_JSON, "facade timing caveat and dummy rows"),
        ],
        "source_files_reviewed": [str(path) for path in SCRIPT_SOURCES],
        "artifacts": {"json": rel(OUT_JSON), "markdown": rel(OUT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    decision = payload["decision"]
    evidence = payload["global_evidence"]
    lines = [
        "# A_reject_equivalence Discharge Ledger",
        "",
        "This is a fail-closed discharge artifact for carried term 5,",
        "`A_reject_equivalence`, on the selected 308-byte/Fq12",
        "state-assisted masked projection-carrier path.",
        "",
        "## Decision",
        "",
        f"- status: `{payload['status']}`",
        f"- classification: `{decision['classification']}`",
        f"- can consume in theorem composition: `{decision['can_consume_in_theorem_composition']}`",
        f"- can remove carried term: `{decision['can_remove_A_reject_equivalence_term']}`",
        f"- can claim selected-route constant time: `{decision['can_claim_selected_route_constant_time']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- reason: {decision['reason']}",
        "",
        "## Global Evidence",
        "",
        f"- timing gates: `{json.dumps(evidence['timing_gates'], sort_keys=True)}`",
        f"- timing policy: `{json.dumps(evidence['timing_policy'], sort_keys=True)}`",
        f"- CT issue counts: `{json.dumps(evidence['ct_issue_counts'], sort_keys=True)}`",
        f"- facade timing caveat: {evidence['facade_timing_caveat']}",
        f"- selected timing caveat: {evidence['selected_projection_timing_caveat']}",
        "",
        "## Status By Failure Class",
        "",
        "| class | surface | status | existing basis |",
        "|---|---|---|---|",
    ]
    for row in payload["failure_class_discharge"]:
        lines.append(
            f"| `{row['id']}` | `{row['surface']}` | `{row['classification']}` | {row['basis']} |"
        )

    lines.extend(["", "## Required Selected-Route Work", ""])
    for row in payload["failure_class_discharge"]:
        lines.extend(
            [
                f"### {row['id']} `{row['classification']}`",
                "",
                "Existing evidence:",
            ]
        )
        lines.extend(f"- {item}" for item in row["existing_evidence"])
        lines.append("")
        if row["selected_reference_timing_rows"]:
            lines.append("Selected 308-byte/Fq12 reference timing rows:")
            for timing_row in row["selected_reference_timing_rows"]:
                lines.append(
                    f"- `{timing_row['operation']}` avg_ns=`{timing_row['avg_ns']}` "
                    f"ratio_to_valid=`{timing_row['ratio_to_valid_baseline']:.3f}` "
                    f"ok=`{timing_row['ok']}` reason=`{timing_row.get('observed_public_reason') or 'accepted'}`"
                )
            lines.append("")
        lines.append("Selected-route tests needed:")
        lines.extend(f"- `{item}`" for item in row["selected_route_tests_needed"])
        lines.append("")
        lines.append("Timing harness needed:")
        lines.extend(f"- {item}" for item in row["timing_harness_needed"])
        lines.append("")
        lines.append("Blockers:")
        lines.extend(f"- {item}" for item in row["blockers"])
        lines.append("")

    lines.extend(
        [
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
                "status_counts": payload["status_counts"],
                "can_remove_A_reject_equivalence_term": payload["decision"][
                    "can_remove_A_reject_equivalence_term"
                ],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
