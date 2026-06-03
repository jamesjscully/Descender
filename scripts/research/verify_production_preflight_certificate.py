#!/usr/bin/env python3
"""Verify the Rust-exported production preflight certificate wiring."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_GAP_REPORT = READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
DEFAULT_STATUS_INDEX = READINESS_DIR / "third_party_review_status_index_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "production_preflight_certificate_verification_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "production_preflight_certificate_verification_n32_q12289.md"
PRODUCTION_REFERENCE_MANIFEST = ROOT / "tools/production-reference/Cargo.toml"
EXPORTER_SOURCE = (
    ROOT / "tools/production-reference/src/bin/production_reference_preflight_certificate.rs"
)
EXPECTED_SCHEMA = "Descender.production_reference_preflight_certificate.v1"
HEX_32 = re.compile(r"^[0-9a-f]{64}$")


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def run_rust_exporter() -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "--quiet",
            "--manifest-path",
            str(PRODUCTION_REFERENCE_MANIFEST),
            "--bin",
            "production_reference_preflight_certificate",
        ],
        cwd=ROOT,
        check=True,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    return json.loads(completed.stdout)


def exported_with_report_fields(exported: dict[str, Any]) -> dict[str, Any]:
    expected = dict(exported)
    expected["export_status"] = "pass"
    expected["binary_source"] = rel(EXPORTER_SOURCE)
    return expected


def status_certificate_summary(exported: dict[str, Any]) -> dict[str, Any]:
    digests = exported.get("digests", {})
    return {
        "schema": exported.get("schema"),
        "export_status": "pass",
        "binary_source": rel(EXPORTER_SOURCE),
        "deployment_ready": exported.get("deployment_ready"),
        "blocker_count": exported.get("blocker_count"),
        "primary_blocker": exported.get("primary_blocker"),
        "certificate_digest": digests.get("certificate"),
        "preflight_report_digest": digests.get("preflight_report"),
        "blocker_report_digest": digests.get("blocker_report"),
        "verdict_digest": digests.get("verdict"),
        "ready_assertion_digest": digests.get("ready_assertion"),
        "cross_checks": exported.get("cross_checks", {}),
        "verification_artifact_json": rel(DEFAULT_JSON),
        "verification_artifact_md": rel(DEFAULT_MD),
    }


def append_failure(
    failures: list[dict[str, Any]], check: str, expected: Any, actual: Any
) -> None:
    failures.append({"check": check, "expected": expected, "actual": actual})


def validate_export_shape(exported: dict[str, Any], failures: list[dict[str, Any]]) -> None:
    if exported.get("schema") != EXPECTED_SCHEMA:
        append_failure(failures, "export_schema", EXPECTED_SCHEMA, exported.get("schema"))

    blockers = exported.get("blockers")
    if not isinstance(blockers, list):
        append_failure(failures, "blockers_type", "list", type(blockers).__name__)
        blockers = []

    expected_indices = list(range(len(blockers)))
    actual_indices = [row.get("index") for row in blockers if isinstance(row, dict)]
    if actual_indices != expected_indices:
        append_failure(failures, "blocker_indices", expected_indices, actual_indices)

    blocker_count = exported.get("blocker_count")
    if blocker_count != len(blockers):
        append_failure(failures, "blocker_count", len(blockers), blocker_count)

    deployment_ready = exported.get("deployment_ready")
    if deployment_ready != (len(blockers) == 0):
        append_failure(
            failures,
            "deployment_ready_matches_blocker_count",
            len(blockers) == 0,
            deployment_ready,
        )

    expected_primary = blockers[0].get("blocker") if blockers else "none"
    if exported.get("primary_blocker") != expected_primary:
        append_failure(
            failures, "primary_blocker", expected_primary, exported.get("primary_blocker")
        )

    for row in blockers:
        if not isinstance(row, dict):
            append_failure(failures, "blocker_row_type", "dict", type(row).__name__)
            continue
        digest = row.get("digest")
        if not isinstance(digest, str) or not HEX_32.match(digest):
            append_failure(failures, f"blocker_digest_{row.get('index')}", "64 hex", digest)

    for name, digest in (exported.get("digests") or {}).items():
        if not isinstance(digest, str) or not HEX_32.match(digest):
            append_failure(failures, f"digest_{name}", "64 hex", digest)

    cross_checks = exported.get("cross_checks", {})
    failing_checks = [
        name for name, passed in cross_checks.items() if passed is not True
    ]
    if failing_checks:
        append_failure(failures, "cross_checks", "all true", failing_checks)


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Production Preflight Certificate Verification",
        "",
        "This verifier reruns the Rust preflight certificate exporter and checks",
        "that the production gap report and third-party status index consume the",
        "same certificate root, blocker rows, and linked digest chain.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- certificate digest: `{payload['certificate_digest']}`",
        f"- blocker count: `{payload['blocker_count']}`",
        f"- primary blocker: {payload['primary_blocker']}",
        f"- gap report: `{payload['gap_report']}`",
        f"- status index: `{payload['status_index']}`",
        f"- exporter source: `{payload['exporter_source']}`",
        f"- failures: `{len(payload['failures'])}`",
        "",
        "## Failures",
        "",
    ]
    if payload["failures"]:
        for failure in payload["failures"]:
            lines.append(f"- `{failure['check']}`")
    else:
        lines.append("- none")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--gap-report", type=Path, default=DEFAULT_GAP_REPORT)
    parser.add_argument("--status-index", type=Path, default=DEFAULT_STATUS_INDEX)
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    args = parser.parse_args()

    exported = run_rust_exporter()
    expected_report_export = exported_with_report_fields(exported)
    expected_status_certificate = status_certificate_summary(exported)
    exported_blockers = [
        row.get("blocker")
        for row in exported.get("blockers", [])
        if isinstance(row, dict)
    ]

    failures: list[dict[str, Any]] = []
    validate_export_shape(exported, failures)

    gap_report = load_json(args.gap_report)
    preflight = gap_report.get("deployment_preflight", {})
    if preflight.get("certificate_export") != expected_report_export:
        append_failure(
            failures,
            "gap_report_certificate_export",
            expected_report_export,
            preflight.get("certificate_export"),
        )
    if preflight.get("blockers") != exported_blockers:
        append_failure(
            failures, "gap_report_blockers", exported_blockers, preflight.get("blockers")
        )
    if preflight.get("deployment_ready") != exported.get("deployment_ready"):
        append_failure(
            failures,
            "gap_report_deployment_ready",
            exported.get("deployment_ready"),
            preflight.get("deployment_ready"),
        )

    status_index = load_json(args.status_index)
    if status_index.get("preflight_certificate") != expected_status_certificate:
        append_failure(
            failures,
            "status_index_preflight_certificate",
            expected_status_certificate,
            status_index.get("preflight_certificate"),
        )
    if status_index.get("terminal_blockers") != exported_blockers:
        append_failure(
            failures,
            "status_index_terminal_blockers",
            exported_blockers,
            status_index.get("terminal_blockers"),
        )
    status_preflight = status_index.get("deployment_preflight", {})
    if status_preflight.get("certificate_export") != expected_report_export:
        append_failure(
            failures,
            "status_index_nested_certificate_export",
            expected_report_export,
            status_preflight.get("certificate_export"),
        )

    output = {
        "schema": "Descender.production_preflight_certificate_verification.v1",
        "status": "pass" if not failures else "fail",
        "failures": failures,
        "certificate_digest": (exported.get("digests") or {}).get("certificate"),
        "blocker_count": exported.get("blocker_count"),
        "primary_blocker": exported.get("primary_blocker"),
        "gap_report": rel(args.gap_report),
        "status_index": rel(args.status_index),
        "exporter_source": rel(EXPORTER_SOURCE),
    }
    args.json.parent.mkdir(parents=True, exist_ok=True)
    args.json.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")
    args.md.write_text(markdown(output))
    print(json.dumps(output, indent=2, sort_keys=True))
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
