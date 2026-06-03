#!/usr/bin/env python3
"""Aggregate per-row constant-time issue reviews."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
OUT_JSON = READINESS_DIR / "constant_time_issue_review_summary_n32_q12289.json"
OUT_MD = READINESS_DIR / "constant_time_issue_review_summary_n32_q12289.md"


ISSUES = [
    {
        "id": "CT-001",
        "class": "empty_signature",
        "artifact": "ct_issue_empty_signature.json",
        "root_cause_class": "public_length_guard",
        "secret_risk": "not_secret_dependent",
        "remediation_class": "constant_schedule_public_parser",
    },
    {
        "id": "CT-002",
        "class": "slot_only_signature",
        "artifact": "ct_issue_slot_only_signature.json",
        "root_cause_class": "public_length_guard",
        "secret_risk": "not_secret_dependent",
        "remediation_class": "constant_schedule_public_parser",
    },
    {
        "id": "CT-003",
        "class": "truncated_by_one",
        "artifact": "ct_issue_truncated_by_one_n32_q12289.json",
        "root_cause_class": "public_length_guard",
        "secret_risk": "not_secret_dependent",
        "remediation_class": "constant_schedule_public_parser",
    },
    {
        "id": "CT-004",
        "class": "extended_by_one",
        "artifact": "ct_issue_extended_by_one.json",
        "root_cause_class": "public_length_guard",
        "secret_risk": "not_secret_dependent",
        "remediation_class": "constant_schedule_public_parser",
    },
    {
        "id": "CT-005",
        "class": "invalid_challenge_mask",
        "artifact": "ct_issue_invalid_challenge_mask_n32_q12289.json",
        "root_cause_class": "public_canonicality_guard",
        "secret_risk": "not_secret_dependent",
        "remediation_class": "accumulated_error_full_decode",
    },
    {
        "id": "CT-006",
        "class": "slot_only_signature_equalized",
        "artifact": "ct_issue_slot_only_signature_equalized_n32_q12289.json",
        "root_cause_class": "additive_not_path_equivalent_equalization",
        "secret_risk": "diagnostic_harness_issue",
        "remediation_class": "path_equivalent_equalized_parser_probe",
    },
    {
        "id": "CT-007",
        "class": "truncated_by_one_equalized",
        "artifact": "ct_issue_truncated_by_one_equalized_n32_q12289.json",
        "root_cause_class": "additive_not_path_equivalent_equalization",
        "secret_risk": "diagnostic_harness_issue",
        "remediation_class": "path_equivalent_equalized_parser_probe",
    },
    {
        "id": "CT-008",
        "class": "extended_by_one_equalized",
        "artifact": "ct_issue_extended_by_one_equalized_n32_q12289.json",
        "root_cause_class": "additive_not_path_equivalent_equalization",
        "secret_risk": "diagnostic_harness_issue",
        "remediation_class": "path_equivalent_equalized_parser_probe",
    },
    {
        "id": "CT-009",
        "class": "invalid_challenge_mask_equalized",
        "artifact": "ct_issue_invalid_challenge_mask_equalized_n32_q12289.json",
        "root_cause_class": "additive_not_path_equivalent_equalization",
        "secret_risk": "diagnostic_harness_issue",
        "remediation_class": "path_equivalent_equalized_parser_probe",
    },
    {
        "id": "CT-010",
        "class": "slot_out_of_range",
        "artifact": "ct_issue_slot_out_of_range_n32_q12289.json",
        "root_cause_class": "public_account_state_guard",
        "secret_risk": "not_secret_dependent",
        "remediation_class": "constant_shape_account_verifier",
    },
    {
        "id": "CT-011",
        "class": "slot_already_consumed",
        "artifact": "ct_issue_slot_already_consumed_n32_q12289.json",
        "root_cause_class": "public_account_state_guard",
        "secret_risk": "not_secret_dependent",
        "remediation_class": "constant_shape_account_verifier",
    },
    {
        "id": "CT-012",
        "class": "signer_callback_failure",
        "artifact": "ct_issue_signer_callback_failure.json",
        "root_cause_class": "signer_callback_failure_short_path",
        "secret_risk": "secret_adjacent_not_proven_secret_dependent",
        "remediation_class": "constant_shape_signer_callback_boundary",
    },
    {
        "id": "CT-013",
        "class": "challenge_hash_long_message",
        "artifact": "ct_issue_challenge_hash_long_message_n32_q12289.json",
        "root_cause_class": "public_message_length_hashing",
        "secret_risk": "not_secret_dependent",
        "remediation_class": "fixed_length_or_prehashed_message_frame",
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def counts(rows: list[dict[str, Any]], key: str) -> dict[str, int]:
    out: dict[str, int] = {}
    for row in rows:
        out[row[key]] = out.get(row[key], 0) + 1
    return dict(sorted(out.items()))


def build_payload() -> dict[str, Any]:
    rows: list[dict[str, Any]] = []
    missing: list[str] = []
    invalid_json: list[str] = []
    for issue in ISSUES:
        path = READINESS_DIR / issue["artifact"]
        artifact_present = path.exists()
        schema = None
        if not artifact_present:
            missing.append(rel(path))
        else:
            try:
                schema = load_json(path).get("schema")
            except Exception:
                invalid_json.append(rel(path))
        rows.append(
            {
                **issue,
                "artifact": rel(path),
                "artifact_present": artifact_present,
                "schema": schema,
                "closure_status": "diagnosed_patch_not_applied",
            }
        )
    secret_adjacent = [r for r in rows if "secret_adjacent" in r["secret_risk"]]
    diagnostic = [r for r in rows if r["secret_risk"] == "diagnostic_harness_issue"]
    public_only = [r for r in rows if r["secret_risk"] == "not_secret_dependent"]
    return {
        "schema": "Descender.constant_time_issue_review_summary.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "issue_count": len(rows),
        "missing_artifact_count": len(missing),
        "invalid_json_count": len(invalid_json),
        "secret_adjacent_count": len(secret_adjacent),
        "diagnostic_harness_issue_count": len(diagnostic),
        "not_secret_dependent_count": len(public_only),
        "root_cause_counts": counts(rows, "root_cause_class"),
        "remediation_counts": counts(rows, "remediation_class"),
        "rows": rows,
        "bottom_line": (
            "The constant-time issue review did not find a proven secret-dependent "
            "leak in these sampled rows.  It did find one secret-adjacent signer "
            "callback short path, eight public-input/account-state timing cliffs, "
            "and four diagnostic equalization flaws.  The side-channel closure "
            "therefore remains open until constant-shape parser, verifier, signer, "
            "and fixed/prehashed message-frame remediations are implemented and remeasured."
        ),
    }


def write_markdown(payload: dict[str, Any]) -> None:
    lines = [
        "# Constant-Time Issue Review Summary",
        "",
        f"- issues reviewed: `{payload['issue_count']}`",
        f"- missing artifacts: `{payload['missing_artifact_count']}`",
        f"- invalid JSON artifacts: `{payload['invalid_json_count']}`",
        f"- not secret-dependent: `{payload['not_secret_dependent_count']}`",
        f"- diagnostic harness issues: `{payload['diagnostic_harness_issue_count']}`",
        f"- secret-adjacent issues: `{payload['secret_adjacent_count']}`",
        f"- bottom line: {payload['bottom_line']}",
        "",
        "## Root Causes",
        "",
        "| root cause | count |",
        "|---|---:|",
    ]
    for key, value in payload["root_cause_counts"].items():
        lines.append(f"| `{key}` | {value} |")
    lines.extend(
        [
            "",
            "## Remediation Classes",
            "",
            "| remediation | count |",
            "|---|---:|",
        ]
    )
    for key, value in payload["remediation_counts"].items():
        lines.append(f"| `{key}` | {value} |")
    lines.extend(
        [
            "",
            "## Issues",
            "",
            "| id | class | secret risk | root cause | remediation | artifact |",
            "|---|---|---|---|---|---|",
        ]
    )
    for row in payload["rows"]:
        lines.append(
            f"| `{row['id']}` | `{row['class']}` | `{row['secret_risk']}` | `{row['root_cause_class']}` | `{row['remediation_class']}` | `{row['artifact']}` |"
        )
    lines.append("")
    OUT_MD.write_text("\n".join(lines))


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    write_markdown(payload)
    print(
        json.dumps(
            {
                "json": rel(OUT_JSON),
                "markdown": rel(OUT_MD),
                "issue_count": payload["issue_count"],
                "secret_adjacent_count": payload["secret_adjacent_count"],
                "missing_artifact_count": payload["missing_artifact_count"],
                "status": "written",
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
