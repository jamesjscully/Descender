#!/usr/bin/env python3
"""Generate timing artifacts for the production-reference crate facade."""

from __future__ import annotations

import argparse
import json
import platform
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "tools/production-reference/Cargo.toml"
BINARY = ROOT / "tools/production-reference/target/release/production_reference_facade_timing"
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.md"


def build_binary() -> None:
    subprocess.run(
        [
            "cargo",
            "build",
            "--release",
            "--manifest-path",
            str(MANIFEST),
            "--bin",
            "production_reference_facade_timing",
        ],
        check=True,
    )


def run_benchmark(iterations: int) -> dict[str, Any]:
    completed = subprocess.run(
        [str(BINARY), "--iterations", str(iterations)],
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    )
    return json.loads(completed.stdout)


def markdown_report(payload: dict[str, Any]) -> str:
    timing = payload["timing_report"]
    lines = [
        "# Production Reference Facade Timing Report",
        "",
        "This report times the `descender-production-reference` crate boundary only.",
        "The benchmark uses deterministic dummy callbacks and therefore does not",
        "measure the final account-carried verifier, real signing work, or",
        "constant-time behavior.",
        "",
        "## Environment",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- platform: `{payload['environment']['platform']}`",
        f"- python: `{payload['environment']['python_version']}`",
        f"- manifest: `{payload['manifest']}`",
        f"- binary: `{payload['binary']}`",
        "",
        "## Parameters",
        "",
        f"- iterations: `{timing['iterations']}`",
        f"- executable signature bytes: `{timing['signature_bytes']}`",
        f"- reviewed target signature bytes: `{timing['reviewed_target_signature_bytes']}`",
        f"- selected 308-byte/Fq12 fallback-projection bytes: `{timing.get('reviewed_fq12_fallback_projection_bytes')}`",
        f"- account-link bytes: `{timing['account_link_bytes']}`",
        f"- public-key bytes: `{timing['public_key_bytes']}`",
        "",
        "## Timings",
        "",
        "| operation | iterations | avg us | avg ns | ok |",
        "|---|---:|---:|---:|:---:|",
    ]
    for row in timing["timings"]:
        lines.append(
            f"| {row['operation']} | {row['iterations']} | "
            f"{row['avg_us']:.3f} | {row['avg_ns']} | "
            f"{'yes' if row['accepted_or_ok'] else 'no'} |"
        )
    selected_rows = timing.get("selected_projection_timings", [])
    if selected_rows:
        lines.extend(
            [
                "",
                "## Selected 308-byte/Fq12 Candidate-Verifier Timings",
                "",
                "| operation | class | iterations | avg us | avg ns | ratio to valid | ok | public reason |",
                "|---|---|---:|---:|---:|---:|:---:|---|",
            ]
        )
        for row in selected_rows:
            lines.append(
                f"| {row['operation']} | {row['reject_class']} | "
                f"{row['iterations']} | {row['avg_us']:.3f} | {row['avg_ns']} | "
                f"{row['ratio_to_valid_baseline']:.3f} | "
                f"{'yes' if row['ok'] else 'no'} | "
                f"{row.get('observed_public_reason') or 'accepted'} |"
            )
        lines.extend(
            [
                "",
                "Selected-route caveat:",
                "",
                timing.get(
                    "selected_projection_caveat",
                    "Selected-route rows are reference timing evidence only.",
                ),
            ]
        )
    lines.extend(
        [
            "",
            "## Caveat",
            "",
            timing["caveat"],
            "",
            "The reserved-sign row creates a fresh lifecycle manager and reservation",
            "for each iteration because the facade correctly consumes the slot on",
            "success.  That row is an API-shape cost, not the cost of a reused",
            "production signing context.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--iterations", type=int, default=10_000)
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()

    if args.iterations <= 0:
        raise SystemExit("--iterations must be positive")
    if not args.skip_build:
        build_binary()
    timing_report = run_benchmark(args.iterations)
    payload = {
        "schema": "Descender.production_reference_facade_timing_report.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "manifest": str(MANIFEST.relative_to(ROOT)),
        "binary": str(BINARY.relative_to(ROOT)),
        "environment": {
            "platform": platform.platform(),
            "python_version": platform.python_version(),
        },
        "timing_report": timing_report,
    }
    args.json.parent.mkdir(parents=True, exist_ok=True)
    args.json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    args.md.write_text(markdown_report(payload))
    print(json.dumps({"json": str(args.json), "markdown": str(args.md)}, indent=2))


if __name__ == "__main__":
    main()
