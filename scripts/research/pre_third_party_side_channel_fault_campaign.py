#!/usr/bin/env python3
"""Run PTC-006 side-channel and fault probes against the reference facade."""

from __future__ import annotations

import argparse
import json
import platform
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from statistics import mean
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "tools/production-reference/Cargo.toml"
BIN_NAME = "pre_third_party_side_channel_fault_probe"
BINARY = ROOT / "tools/production-reference/target/release" / BIN_NAME
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_third_party_side_channel_fault_campaign_n32_q12289.md"
TIMING_RATIO_POLICY_MIN = 0.80
TIMING_RATIO_POLICY_MAX = 1.25


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_probe() -> None:
    subprocess.run(
        [
            "cargo",
            "build",
            "--release",
            "--manifest-path",
            str(MANIFEST),
            "--bin",
            BIN_NAME,
        ],
        check=True,
    )


def run_probe(samples: int, batch_iterations: int) -> dict[str, Any]:
    completed = subprocess.run(
        [
            str(BINARY),
            "--samples",
            str(samples),
            "--batch-iterations",
            str(batch_iterations),
        ],
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    )
    return json.loads(completed.stdout)


def row_by_class(rows: list[dict[str, Any]], class_name: str) -> dict[str, Any]:
    for row in rows:
        if row.get("class") == class_name:
            return row
    raise KeyError(class_name)


def ratio(row: dict[str, Any], baseline: dict[str, Any]) -> float:
    base = float(baseline["mean_avg_ns"])
    if base == 0:
        return 0.0
    return float(row["mean_avg_ns"]) / base


def summarize_rows(probe: dict[str, Any]) -> dict[str, Any]:
    rows = probe["rows"]
    parse_baseline = row_by_class(rows, "valid_linear_sigma_signature")
    equalized_parse_baseline = row_by_class(rows, "valid_linear_sigma_signature_equalized")
    reject_baseline = row_by_class(rows, "verifier_callback_failure")
    prf_baseline = row_by_class(rows, "pad_prf_zero_seed")
    hash_baseline = row_by_class(rows, "challenge_hash_baseline")
    equalized_hash_baseline = row_by_class(rows, "challenge_hash_baseline_equalized")
    annotated = []
    for row in rows:
        if row["group"] == "parse_malformed_input":
            baseline = parse_baseline
        elif row["group"] == "equalized_parse_diagnostic":
            baseline = equalized_parse_baseline
        elif row["group"] in {"rejection_class", "fault_injection_callback"}:
            baseline = reject_baseline
        elif row["group"] == "prf_seed_class":
            baseline = prf_baseline
        elif row["group"] == "hash_frame_public_input":
            baseline = hash_baseline
        elif row["group"] == "equalized_hash_frame_diagnostic":
            baseline = equalized_hash_baseline
        else:
            baseline = row
        annotated.append(
            {
                "group": row["group"],
                "class": row["class"],
                "operation": row["operation"],
                "expected_outcome": row["expected_outcome"],
                "observed_outcome": row["observed_outcome"],
                "mean_avg_ns": row["mean_avg_ns"],
                "median_avg_ns": row["median_avg_ns"],
                "min_avg_ns": row["min_avg_ns"],
                "max_avg_ns": row["max_avg_ns"],
                "ratio_to_group_baseline": ratio(row, baseline),
                "ok_batches": row["ok_batches"],
                "samples": row["samples"],
            }
        )
    fail_closed_issues = [
        item
        for item in annotated
        if item["expected_outcome"].startswith("reject")
        and item["observed_outcome"] != "all_batches_matched"
    ]
    timing_ratios = [
        item["ratio_to_group_baseline"]
        for item in annotated
        if item["class"] not in {
            "valid_linear_sigma_signature",
            "verifier_callback_failure",
            "pad_prf_zero_seed",
            "challenge_hash_baseline",
        }
    ]
    timing_policy_rows = [
        item
        for item in annotated
        if item["ratio_to_group_baseline"] > TIMING_RATIO_POLICY_MAX
        or item["ratio_to_group_baseline"] < TIMING_RATIO_POLICY_MIN
    ]
    return {
        "annotated_rows": annotated,
        "fail_closed_issue_count": len(fail_closed_issues),
        "fail_closed_issues": fail_closed_issues,
        "max_timing_ratio_to_group_baseline": max(timing_ratios) if timing_ratios else 0.0,
        "min_timing_ratio_to_group_baseline": min(timing_ratios) if timing_ratios else 0.0,
        "mean_timing_ratio_to_group_baseline": mean(timing_ratios) if timing_ratios else 0.0,
        "timing_ratio_policy": {
            "min_ratio": TIMING_RATIO_POLICY_MIN,
            "max_ratio": TIMING_RATIO_POLICY_MAX,
        },
        "timing_policy_issue_count": len(timing_policy_rows),
        "timing_policy_issue_rows": timing_policy_rows,
    }


def build_payload(probe: dict[str, Any]) -> dict[str, Any]:
    summary = summarize_rows(probe)
    diagnostic_only_reasons = [
        "The facade benchmark uses deterministic dummy callbacks and not the deployment account-carried verifier.",
        "Malformed-input timing includes early parser exits and public rejection classes; this is useful for finding cliffs, not for proving constant time.",
        "Hash and PRF fault injection is only public-input/seed-class differential timing because safe APIs do not expose injectable SHA-256 or PRF failures.",
        "The production preflight remains blocked on final verifier and hidden-source semantics, so hardening status cannot be inferred from this probe.",
    ]
    if summary["fail_closed_issue_count"] != 0:
        verdict = "diagnostic_reference_with_fail_closed_regression"
    elif summary["timing_policy_issue_count"] != 0:
        verdict = "diagnostic_reference_with_timing_cliffs"
    else:
        verdict = "diagnostic_reference_no_sampled_timing_cliffs"
    hardening_gate = {
        "fail_closed_gate": summary["fail_closed_issue_count"] == 0,
        "timing_ratio_gate": summary["timing_policy_issue_count"] == 0,
        "policy_min_ratio": TIMING_RATIO_POLICY_MIN,
        "policy_max_ratio": TIMING_RATIO_POLICY_MAX,
        "closed": False,
        "reason": (
            "PTC-006 remains open until fail-closed behavior and timing-ratio "
            "policy both pass on the final production verifier with constant-time review."
        ),
    }
    return {
        "schema": "Descender.pre_third_party_side_channel_fault_campaign.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "campaign_id": "PTC-006",
        "campaign": "side_channel_and_fault_behavior",
        "scope": "n32_q12289 account-carried committed-sigma signature readiness",
        "environment": {
            "platform": platform.platform(),
            "python_version": platform.python_version(),
        },
        "inputs": {
            "manifest": rel(MANIFEST),
            "probe_binary": rel(BINARY),
            "probe_source": "tools/production-reference/src/bin/pre_third_party_side_channel_fault_probe.rs",
            "closure_check_script": "scripts/research/side_channel_timing_closure.py",
            "reference_timing_artifacts": [
                "research/cryptanalysis/signature_readiness_20260516/reference_timing_report_n32_q12289.json",
                "research/cryptanalysis/signature_readiness_20260516/production_reference_facade_timing_report_n32_q12289.json",
                "research/cryptanalysis/signature_readiness_20260516/constant_time_side_channel_audit_memo_n32_q12289.json",
            ],
        },
        "probe": probe,
        "summary": summary,
        "coverage": {
            "malformed_input_classes": [
                "empty_signature",
                "slot_only_signature",
                "truncated_by_one",
                "extended_by_one",
                "invalid_challenge_mask",
            ],
            "equalized_diagnostic_classes": [
                "valid_linear_sigma_signature_equalized",
                "empty_signature_equalized",
                "slot_only_signature_equalized",
                "truncated_by_one_equalized",
                "extended_by_one_equalized",
                "invalid_challenge_mask_equalized",
                "challenge_hash_baseline_equalized",
                "challenge_hash_long_message_equalized",
            ],
            "rejection_classes": [
                "verifier_callback_failure",
                "slot_out_of_range",
                "slot_already_consumed",
                "proof_commitment_mismatch",
            ],
            "fault_injection_classes": [
                "signer_callback_failure",
                "public challenge hash frame mutation by message length",
                "pad PRF seed class differentials",
                "fallback pad projection PRF differential",
            ],
            "infeasible_without_library_or_hardware_hooks": probe[
                "infeasible_fault_injections"
            ],
        },
        "classification": {
            "current_reference_treatment": verdict,
            "hardened_claim": False,
            "hardening_gate": hardening_gate,
            "diagnostic_only_reasons": diagnostic_only_reasons,
            "required_before_hardened_claim": [
                "Replace dummy callbacks with final verifier semantics.",
                "Make the timing-ratio policy executable and passing on final verifier rows.",
                "Add constant-time implementation review for secret-dependent branches and memory access.",
                "Add fault hooks or harnesses for hash/PRF failures at explicit boundaries.",
                "Run platform-level timing/leakage analysis on deployment code, not only this reference crate.",
            ],
        },
    }


def markdown_report(payload: dict[str, Any]) -> str:
    summary = payload["summary"]
    classification = payload["classification"]
    lines = [
        "# PTC-006 Side-Channel and Fault Campaign",
        "",
        "This local campaign probes the current reference facade for malformed-input",
        "timing, rejection-class timing, sanitized callback faults, public hash-frame",
        "differentials, and pad-PRF seed/projection differentials.",
        "",
        "## Verdict",
        "",
        f"- treatment: `{classification['current_reference_treatment']}`",
        f"- hardened claim: `{classification['hardened_claim']}`",
        f"- fail-closed issues: `{summary['fail_closed_issue_count']}`",
        f"- timing policy issues: `{summary['timing_policy_issue_count']}`",
        f"- timing policy ratio window: `{summary['timing_ratio_policy']['min_ratio']:.2f}..{summary['timing_ratio_policy']['max_ratio']:.2f}`",
        f"- max timing ratio to group baseline: `{summary['max_timing_ratio_to_group_baseline']:.3f}`",
        f"- min timing ratio to group baseline: `{summary['min_timing_ratio_to_group_baseline']:.3f}`",
        "",
        "The current reference should be treated as diagnostic only, not hardened.",
        "The measurements are useful attack-surface evidence, but they do not prove",
        "constant-time behavior and do not exercise the final production verifier.",
        "Equalized rows are diagnostic controls; they are not deployment wrappers.",
        "",
        "## Probe Parameters",
        "",
        f"- samples: `{payload['probe']['samples']}`",
        f"- batch iterations per sample: `{payload['probe']['batch_iterations']}`",
        f"- signature bytes: `{payload['probe']['signature_bytes']}`",
        f"- reviewed Fq12 + hidden-source bytes: `{payload['probe']['reviewed_fq12_hidden_source_bytes']}`",
        f"- reviewed Fq12 + fallback projection bytes: `{payload['probe']['reviewed_fq12_fallback_projection_bytes']}`",
        "",
        "## Differential Rows",
        "",
        "| group | class | mean ns | median ns | ratio | outcome |",
        "|---|---|---:|---:|---:|---|",
    ]
    for row in summary["annotated_rows"]:
        lines.append(
            f"| {row['group']} | {row['class']} | "
            f"{row['mean_avg_ns']:.1f} | {row['median_avg_ns']:.1f} | "
            f"{row['ratio_to_group_baseline']:.3f} | {row['observed_outcome']} |"
        )
    lines.extend(["", "## Timing Policy Issues", ""])
    if summary["timing_policy_issue_rows"]:
        lines.extend(
            [
                "| group | class | ratio | mean ns | baseline policy |",
                "|---|---|---:|---:|---|",
            ]
        )
        for row in summary["timing_policy_issue_rows"]:
            lines.append(
                f"| {row['group']} | {row['class']} | "
                f"{row['ratio_to_group_baseline']:.3f} | {row['mean_avg_ns']:.1f} | "
                f"{TIMING_RATIO_POLICY_MIN:.2f}..{TIMING_RATIO_POLICY_MAX:.2f} |"
            )
    else:
        lines.append("No sampled rows exceeded the timing-ratio policy window.")
    lines.extend(["", "## Fault Injection Limits", ""])
    for item in payload["coverage"]["infeasible_without_library_or_hardware_hooks"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Hardening Gap", ""])
    for item in classification["required_before_hardened_claim"]:
        lines.append(f"- {item}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--samples", type=int, default=25)
    parser.add_argument("--batch-iterations", type=int, default=1_000)
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()

    if args.samples <= 0:
        raise SystemExit("--samples must be positive")
    if args.batch_iterations <= 0:
        raise SystemExit("--batch-iterations must be positive")
    if not args.skip_build:
        build_probe()
    probe = run_probe(args.samples, args.batch_iterations)
    payload = build_payload(probe)
    args.json.parent.mkdir(parents=True, exist_ok=True)
    args.json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    args.md.write_text(markdown_report(payload))
    print(json.dumps({"json": str(args.json), "markdown": str(args.md)}, indent=2))


if __name__ == "__main__":
    main()
