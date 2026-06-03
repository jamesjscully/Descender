#!/usr/bin/env python3
"""Generate reference timing artifacts for the current stateful signature binary."""

from __future__ import annotations

import argparse
import json
import platform
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_BINARY = (
    ROOT / "tools/parameter-search/target/release/signature_q12289_local_delta_stateful"
)
DEFAULT_JSON = (
    ROOT
    / "research/cryptanalysis/signature_readiness_20260516/"
    "reference_timing_report_n32_q12289.json"
)
DEFAULT_MD = (
    ROOT
    / "research/cryptanalysis/signature_readiness_20260516/"
    "reference_timing_report_n32_q12289.md"
)


def run_benchmark(binary: Path, iterations: int) -> dict[str, Any]:
    completed = subprocess.run(
        [str(binary), "--bench", "--iterations", str(iterations)],
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    )
    return json.loads(completed.stdout)


def markdown_report(payload: dict[str, Any]) -> str:
    timing = payload["timing_report"]
    lines = [
        "# Descender Reference Timing Report",
        "",
        "This is a reference/research timing artifact, not a production performance",
        "claim.  The benchmark uses the current stateful local-delta reference binary",
        "and includes both direct reference rows and production-facade rows.  The",
        "facade rows still wrap the legacy 254-byte proof callback rather than",
        "accepting the selected 308-byte/Fq12 projection-carrier proof envelope.",
        "",
        "## Environment",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- platform: `{payload['environment']['platform']}`",
        f"- python: `{payload['environment']['python_version']}`",
        f"- binary: `{payload['binary']}`",
        "",
        "## Parameters",
        "",
        f"- iterations: `{timing['iterations']}`",
        f"- signature bytes: `{timing['signature_bytes']}`",
        f"- public key bytes: `{timing['public_key_bytes']}`",
        f"- account slots: `{timing['account_slots']}`",
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
    lines.extend(
        [
            "",
            "## Caveat",
            "",
            timing["caveat"],
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, default=DEFAULT_BINARY)
    parser.add_argument("--iterations", type=int, default=10_000)
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    args = parser.parse_args()

    timing_report = run_benchmark(args.binary, args.iterations)
    payload = {
        "schema": "Descender.reference_timing_report.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "binary": str(args.binary),
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
