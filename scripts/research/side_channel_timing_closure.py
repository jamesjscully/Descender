#!/usr/bin/env python3
"""Produce an executable PTC-006 side-channel timing closure gate."""

from __future__ import annotations

import argparse
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_CAMPAIGN_JSON = (
    READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
)
DEFAULT_FACADE_TIMING_JSON = (
    READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.json"
)
DEFAULT_JSON = READINESS_DIR / "side_channel_timing_closure_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "side_channel_timing_closure_n32_q12289.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def selected_projection_evidence(
    facade: dict[str, Any] | None,
    facade_path: Path,
    min_ratio: float,
    max_ratio: float,
) -> dict[str, Any]:
    timing = (facade or {}).get("timing_report", {})
    rows = timing.get("selected_projection_timings", [])
    wanted_classes = {"valid", "pad", "witness", "carrier", "transcript", "checked_core"}
    observed_classes = {row.get("reject_class") for row in rows if row.get("ok")}
    ratios = [
        row.get("ratio_to_valid_baseline")
        for row in rows
        if isinstance(row.get("ratio_to_valid_baseline"), (int, float))
    ]
    timing_policy_issue_rows = [
        row
        for row in rows
        if isinstance(row.get("ratio_to_valid_baseline"), (int, float))
        and (
            row["ratio_to_valid_baseline"] < min_ratio
            or row["ratio_to_valid_baseline"] > max_ratio
        )
    ]
    return {
        "source": rel(facade_path),
        "present": bool(rows),
        "schema": timing.get("schema"),
        "signature_bytes": timing.get("reviewed_target_signature_bytes"),
        "selected_signature_bytes": timing.get("reviewed_fq12_fallback_projection_bytes")
        or timing.get("reviewed_fq12_with_fallback_projection_bytes"),
        "row_count": len(rows),
        "classes_required": sorted(wanted_classes),
        "classes_observed_ok": sorted(observed_classes),
        "all_required_classes_observed_ok": wanted_classes <= observed_classes,
        "timing_ratio_policy": {
            "min_ratio": min_ratio,
            "max_ratio": max_ratio,
            "issue_count": len(timing_policy_issue_rows),
        },
        "timing_policy_issue_rows": timing_policy_issue_rows,
        "timing_ratio_policy_passed": len(timing_policy_issue_rows) == 0,
        "min_ratio_to_valid": min(ratios) if ratios else None,
        "max_ratio_to_valid": max(ratios) if ratios else None,
        "caveat": timing.get("selected_projection_caveat"),
        "rows": rows,
    }


def build_payload(
    campaign: dict[str, Any],
    campaign_path: Path,
    facade: dict[str, Any] | None,
    facade_path: Path,
) -> dict[str, Any]:
    summary = campaign["summary"]
    classification = campaign["classification"]
    hardening_gate = classification["hardening_gate"]
    fail_closed_gate = bool(hardening_gate["fail_closed_gate"])
    timing_ratio_gate = bool(hardening_gate["timing_ratio_gate"])
    selected_evidence = selected_projection_evidence(
        facade,
        facade_path,
        hardening_gate["policy_min_ratio"],
        hardening_gate["policy_max_ratio"],
    )
    selected_reference_gate = bool(
        selected_evidence["present"]
        and selected_evidence["all_required_classes_observed_ok"]
    )
    selected_timing_ratio_gate = bool(
        selected_reference_gate and selected_evidence["timing_ratio_policy_passed"]
    )
    final_verifier_gate = False
    constant_time_review_gate = False
    closure_status = (
        "closed"
        if fail_closed_gate
        and timing_ratio_gate
        and selected_reference_gate
        and selected_timing_ratio_gate
        and final_verifier_gate
        and constant_time_review_gate
        else "open"
    )
    blockers = []
    if not fail_closed_gate:
        blockers.append("sampled fail-closed behavior regressed")
    if not timing_ratio_gate:
        blockers.append("sampled timing-ratio policy has cliffs")
    if not selected_reference_gate:
        blockers.append("same-harness selected 308-byte/Fq12 reference rows are missing or incomplete")
    if selected_reference_gate and not selected_timing_ratio_gate:
        blockers.append("same-harness selected 308-byte/Fq12 reference rows have timing-ratio cliffs")
    if not final_verifier_gate:
        blockers.append("final production verifier semantics are not wired into this probe")
    if not constant_time_review_gate:
        blockers.append("constant-time implementation review is not complete")

    return {
        "schema": "Descender.side_channel_timing_closure.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "campaign_id": "PTC-006",
        "source_campaign": rel(campaign_path),
        "closure_status": closure_status,
        "hardened_claim": False,
        "gates": {
            "sampled_fail_closed": fail_closed_gate,
            "sampled_timing_ratio_policy": timing_ratio_gate,
            "selected_308_byte_fq12_same_harness_reference_rows": selected_reference_gate,
            "selected_308_byte_fq12_same_harness_timing_ratio_policy": selected_timing_ratio_gate,
            "final_production_verifier_semantics": final_verifier_gate,
            "constant_time_implementation_review": constant_time_review_gate,
        },
        "policy": {
            "min_timing_ratio": hardening_gate["policy_min_ratio"],
            "max_timing_ratio": hardening_gate["policy_max_ratio"],
            "timing_policy_issue_count": summary["timing_policy_issue_count"],
            "fail_closed_issue_count": summary["fail_closed_issue_count"],
        },
        "selected_projection_reference_evidence": selected_evidence,
        "timing_policy_issue_rows": summary["timing_policy_issue_rows"],
        "blockers": blockers,
        "executable_requirements": [
            {
                "id": "PTC-006-FC",
                "description": "All fail-closed rows must match their expected rejection or acceptance outcome.",
                "passed": fail_closed_gate,
            },
            {
                "id": "PTC-006-TIME-RATIO",
                "description": "Every sampled row must remain inside the configured timing-ratio policy window relative to its group baseline.",
                "passed": timing_ratio_gate,
            },
            {
                "id": "PTC-006-SELECTED-REFERENCE-ROWS",
                "description": "The reference timing artifact must include same-harness selected 308-byte/Fq12 rows for valid, pad, witness, carrier, transcript, and checked-core reject classes.",
                "passed": selected_reference_gate,
            },
            {
                "id": "PTC-006-SELECTED-TIME-RATIO",
                "description": "Every same-harness selected 308-byte/Fq12 reference row must remain inside the configured timing-ratio policy window relative to the valid selected baseline.",
                "passed": selected_timing_ratio_gate,
            },
            {
                "id": "PTC-006-FINAL-VERIFIER",
                "description": "Probe must run against the final production verifier semantics, not dummy callbacks or fail-closed placeholder semantics.",
                "passed": final_verifier_gate,
            },
            {
                "id": "PTC-006-CT-REVIEW",
                "description": "A constant-time implementation review must cover secret-dependent branches, memory access, retry shape, and callback boundaries.",
                "passed": constant_time_review_gate,
            },
        ],
    }


def markdown_report(payload: dict[str, Any]) -> str:
    lines = [
        "# PTC-006 Side-Channel Timing Closure",
        "",
        f"- source campaign: `{payload['source_campaign']}`",
        f"- closure status: `{payload['closure_status']}`",
        f"- hardened claim: `{payload['hardened_claim']}`",
        f"- fail-closed issues: `{payload['policy']['fail_closed_issue_count']}`",
        f"- timing policy issues: `{payload['policy']['timing_policy_issue_count']}`",
        f"- timing ratio window: `{payload['policy']['min_timing_ratio']:.2f}..{payload['policy']['max_timing_ratio']:.2f}`",
        "",
        "## Gates",
        "",
        "| gate | passed |",
        "|---|---:|",
    ]
    for gate, passed in payload["gates"].items():
        lines.append(f"| {gate} | `{passed}` |")

    lines.extend(["", "## Blockers", ""])
    for blocker in payload["blockers"]:
        lines.append(f"- {blocker}")

    lines.extend(["", "## Timing Policy Issue Rows", ""])
    if payload["timing_policy_issue_rows"]:
        lines.extend(
            [
                "| group | class | ratio | mean ns |",
                "|---|---|---:|---:|",
            ]
        )
        for row in payload["timing_policy_issue_rows"]:
            lines.append(
                f"| {row['group']} | {row['class']} | "
                f"{row['ratio_to_group_baseline']:.3f} | {row['mean_avg_ns']:.1f} |"
            )
    else:
        lines.append("No sampled timing-ratio policy issues were reported.")

    selected = payload["selected_projection_reference_evidence"]
    lines.extend(
        [
            "",
            "## Selected 308-byte/Fq12 Reference Rows",
            "",
            f"- source: `{selected['source']}`",
            f"- present: `{selected['present']}`",
            f"- row count: `{selected['row_count']}`",
            f"- required classes observed: `{selected['all_required_classes_observed_ok']}`",
            f"- selected timing policy issues: `{selected['timing_ratio_policy']['issue_count']}`",
            f"- min/max ratio to valid: `{selected['min_ratio_to_valid']}` / `{selected['max_ratio_to_valid']}`",
            "",
        ]
    )
    if selected["rows"]:
        lines.extend(
            [
                "| class | operation | avg ns | ratio to valid | ok | public reason |",
                "|---|---|---:|---:|:---:|---|",
            ]
        )
        for row in selected["rows"]:
            lines.append(
                f"| {row['reject_class']} | {row['operation']} | {row['avg_ns']} | "
                f"{row['ratio_to_valid_baseline']:.3f} | `{row['ok']}` | "
                f"{row.get('observed_public_reason') or 'accepted'} |"
            )
    else:
        lines.append("No selected-route same-harness rows were present.")

    lines.extend(["", "## Executable Requirements", ""])
    for requirement in payload["executable_requirements"]:
        lines.append(
            f"- `{requirement['id']}` passed=`{requirement['passed']}`: "
            f"{requirement['description']}"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--campaign-json", type=Path, default=DEFAULT_CAMPAIGN_JSON)
    parser.add_argument("--facade-timing-json", type=Path, default=DEFAULT_FACADE_TIMING_JSON)
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    parser.add_argument("--require-closed", action="store_true")
    args = parser.parse_args()

    campaign = load_json(args.campaign_json)
    facade = load_json(args.facade_timing_json) if args.facade_timing_json.exists() else None
    payload = build_payload(campaign, args.campaign_json, facade, args.facade_timing_json)
    args.json.parent.mkdir(parents=True, exist_ok=True)
    args.json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    args.md.write_text(markdown_report(payload))
    print(json.dumps({"json": str(args.json), "markdown": str(args.md)}, indent=2))

    if args.require_closed and payload["closure_status"] != "closed":
        raise SystemExit("PTC-006 side-channel/timing closure remains open")


if __name__ == "__main__":
    main()
