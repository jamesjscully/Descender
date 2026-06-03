#!/usr/bin/env python3
"""Verify the terminal third-party review packet finalization report."""

from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
FINALIZATION_JSON = READINESS_DIR / "third_party_review_packet_finalization_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "third_party_review_packet_finalization_verification_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "third_party_review_packet_finalization_verification_n32_q12289.md"

EXPECTED_SCHEMA = "Descender.third_party_review_packet_finalization.v1"
EXPECTED_STEP_NAMES = [
    "review_gate_replay",
    "verify_production_preflight_certificate",
    "verify_compact_signature_todos_consistency",
    "verify_selected_deployment_terminology",
    "third_party_review_status_index",
    "verify_third_party_review_status_index",
    "third_party_review_manifest",
    "verify_third_party_review_manifest",
]
EXPECTED_CONTRACT_CHECKS = {
    "recommended_finalize_command",
    "finalizer_script_indexed",
    "terminal_finalization_report_excluded",
}
EXPECTED_TERMINAL_VERIFICATION_COMMAND = (
    "python3 scripts/research/verify_third_party_review_finalization.py"
)
EXPECTED_TERMINAL_REPORTS = {
    "research/cryptanalysis/signature_readiness_20260516/third_party_review_packet_finalization_n32_q12289.json",
    "research/cryptanalysis/signature_readiness_20260516/third_party_review_packet_finalization_n32_q12289.md",
    "research/cryptanalysis/signature_readiness_20260516/third_party_review_packet_finalization_verification_n32_q12289.json",
    "research/cryptanalysis/signature_readiness_20260516/third_party_review_packet_finalization_verification_n32_q12289.md",
}
EXPECTED_MANIFEST_INDEXED_SCRIPTS = {
    "scripts/research/finalize_third_party_review_packet.py",
    "scripts/research/verify_third_party_review_finalization.py",
}
MANIFEST_VERIFIER_SCRIPT = ROOT / "scripts/research/verify_third_party_review_manifest.py"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def load_manifest_verifier() -> Any:
    spec = importlib.util.spec_from_file_location(
        "descender_verify_third_party_review_manifest", MANIFEST_VERIFIER_SCRIPT
    )
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {MANIFEST_VERIFIER_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def verify_manifest_independently(payload: dict[str, Any]) -> list[dict[str, Any]]:
    failures: list[dict[str, Any]] = []
    manifest_path_value = (payload.get("outputs") or {}).get("manifest")
    if not isinstance(manifest_path_value, str):
        return [{"check": "manifest_output_path_present", "key": "manifest"}]
    manifest_path = ROOT / manifest_path_value
    if not manifest_path.exists():
        return [
            {
                "check": "manifest_output_path_exists",
                "path": manifest_path_value,
            }
        ]
    manifest = load_json(manifest_path)
    paths = {str(row.get("path")) for row in manifest.get("files", [])}
    indexed_terminal_reports = sorted(paths.intersection(EXPECTED_TERMINAL_REPORTS))
    if indexed_terminal_reports:
        failures.append(
            {
                "check": "manifest_excludes_terminal_reports",
                "unexpected": indexed_terminal_reports,
            }
        )
    missing_scripts = sorted(EXPECTED_MANIFEST_INDEXED_SCRIPTS.difference(paths))
    if missing_scripts:
        failures.append(
            {
                "check": "manifest_indexes_finalization_scripts",
                "missing": missing_scripts,
            }
        )
    if manifest.get("recommended_finalize_command") != (
        "python3 scripts/research/finalize_third_party_review_packet.py"
    ):
        failures.append(
            {
                "check": "manifest_recommended_finalize_command",
                "actual": manifest.get("recommended_finalize_command"),
            }
        )
    summary = manifest.get("summary") or {}
    if summary.get("files_missing") != 0:
        failures.append(
            {
                "check": "manifest_files_missing",
                "expected": 0,
                "actual": summary.get("files_missing"),
            }
        )
    manifest_verifier = load_manifest_verifier()
    manifest_shape_failures = manifest_verifier.verify_manifest_shape(manifest)
    rows = []
    for row in manifest.get("files", []):
        row_failures = manifest_verifier.verify_file(row)
        rows.append(
            {
                "path": row.get("path"),
                "failures": row_failures,
            }
        )
    failed_rows = [row for row in rows if row["failures"]]
    if manifest_shape_failures:
        failures.append(
            {
                "check": "independent_manifest_shape",
                "failures": manifest_shape_failures,
            }
        )
    if failed_rows:
        failures.append(
            {
                "check": "independent_manifest_files",
                "failed_files": len(failed_rows),
                "failures": failed_rows[:20],
            }
        )
    return failures


def verify(payload: dict[str, Any]) -> list[dict[str, Any]]:
    failures: list[dict[str, Any]] = []
    if payload.get("schema") != EXPECTED_SCHEMA:
        failures.append(
            {
                "check": "schema",
                "expected": EXPECTED_SCHEMA,
                "actual": payload.get("schema"),
            }
        )
    if payload.get("status") != "pass":
        failures.append(
            {"check": "status", "expected": "pass", "actual": payload.get("status")}
        )
    if (
        payload.get("recommended_terminal_verification_command")
        != EXPECTED_TERMINAL_VERIFICATION_COMMAND
    ):
        failures.append(
            {
                "check": "recommended_terminal_verification_command",
                "expected": EXPECTED_TERMINAL_VERIFICATION_COMMAND,
                "actual": payload.get("recommended_terminal_verification_command"),
            }
        )
    steps = payload.get("steps")
    if not isinstance(steps, list):
        failures.append({"check": "steps_shape", "expected": "list"})
        steps = []
    step_names = [step.get("name") for step in steps if isinstance(step, dict)]
    if step_names != EXPECTED_STEP_NAMES:
        failures.append(
            {
                "check": "step_order",
                "expected": EXPECTED_STEP_NAMES,
                "actual": step_names,
            }
        )
    for step in steps:
        if not isinstance(step, dict):
            failures.append({"check": "step_entry_shape", "actual": repr(step)})
            continue
        if step.get("status") != "pass" or step.get("returncode") != 0:
            failures.append(
                {
                    "check": "step_passed",
                    "step": step.get("name"),
                    "status": step.get("status"),
                    "returncode": step.get("returncode"),
                }
            )
    manifest_verification = payload.get("manifest_verification") or {}
    if manifest_verification.get("status") != "pass":
        failures.append(
            {
                "check": "manifest_verification_status",
                "expected": "pass",
                "actual": manifest_verification.get("status"),
            }
        )
    if manifest_verification.get("failed_files") != 0:
        failures.append(
            {
                "check": "manifest_failed_files",
                "expected": 0,
                "actual": manifest_verification.get("failed_files"),
            }
        )
    if manifest_verification.get("manifest_failures"):
        failures.append(
            {
                "check": "manifest_level_failures",
                "actual": manifest_verification.get("manifest_failures"),
            }
        )
    finalizer_contract = payload.get("finalizer_contract") or {}
    if finalizer_contract.get("status") != "pass":
        failures.append(
            {
                "check": "finalizer_contract_status",
                "expected": "pass",
                "actual": finalizer_contract.get("status"),
            }
        )
    checks = finalizer_contract.get("checks") or {}
    if set(checks) != EXPECTED_CONTRACT_CHECKS:
        failures.append(
            {
                "check": "finalizer_contract_check_set",
                "expected": sorted(EXPECTED_CONTRACT_CHECKS),
                "actual": sorted(checks),
            }
        )
    for name in EXPECTED_CONTRACT_CHECKS:
        if checks.get(name) is not True:
            failures.append(
                {
                    "check": "finalizer_contract_check",
                    "name": name,
                    "expected": True,
                    "actual": checks.get(name),
                }
            )
    terminal_reports = set(finalizer_contract.get("terminal_reports") or [])
    if terminal_reports != EXPECTED_TERMINAL_REPORTS:
        failures.append(
            {
                "check": "terminal_report_set",
                "expected": sorted(EXPECTED_TERMINAL_REPORTS),
                "actual": sorted(terminal_reports),
            }
        )
    outputs = payload.get("outputs") or {}
    for key in [
        "replay_report",
        "manifest",
        "terminal_finalization_report_json",
        "terminal_finalization_report_md",
    ]:
        path_value = outputs.get(key)
        if not isinstance(path_value, str):
            failures.append({"check": "output_path_present", "key": key})
            continue
        if not (ROOT / path_value).exists():
            failures.append(
                {"check": "output_path_exists", "key": key, "path": path_value}
            )
    failures.extend(verify_manifest_independently(payload))
    return failures


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Third-Party Review Finalization Verification",
        "",
        "This terminal verifier checks the finalization report after the full",
        "review-packet finalizer has run.  Its report is intentionally not part",
        "of the hashed manifest because it depends on the finalization report,",
        "which records the manifest verification.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- finalization report: `{payload['finalization_report']}`",
        f"- step count: `{payload['step_count']}`",
        f"- independent manifest checks: `{payload['independent_manifest_check_count']}`",
        f"- self-test status: `{payload['self_test']['status']}`",
        f"- self-test cases: `{len(payload['self_test']['cases'])}`",
        f"- failures: `{len(payload['failures'])}`",
        "",
        "## Artifacts",
        "",
        f"- JSON: `{payload['artifacts']['json']}`",
        f"- Markdown: `{payload['artifacts']['markdown']}`",
        "",
    ]
    if payload["failures"]:
        lines.extend(["## Failures", ""])
        for failure in payload["failures"]:
            lines.append(f"- `{failure['check']}`: `{failure}`")
        lines.append("")
    return "\n".join(lines)


def clone_json(value: dict[str, Any]) -> dict[str, Any]:
    return json.loads(json.dumps(value))


def run_self_test(input_path: Path) -> dict[str, Any]:
    valid = load_json(input_path)
    cases = []

    base_failures = verify(valid)
    cases.append(
        {
            "case": "current_report_passes",
            "passed": not base_failures,
            "expected_checks": [],
            "actual_checks": [failure["check"] for failure in base_failures],
        }
    )

    mutations: list[tuple[str, list[str], Any]] = [
        (
            "missing_terminal_command",
            ["recommended_terminal_verification_command"],
            lambda payload: payload.pop("recommended_terminal_verification_command", None),
        ),
        (
            "step_order_drift",
            ["step_order"],
            lambda payload: payload["steps"].reverse(),
        ),
        (
            "step_failure",
            ["step_passed"],
            lambda payload: payload["steps"][0].update(
                {"status": "fail", "returncode": 1}
            ),
        ),
        (
            "terminal_report_set_drift",
            ["terminal_report_set"],
            lambda payload: payload["finalizer_contract"]["terminal_reports"].pop(),
        ),
        (
            "missing_manifest_output",
            ["output_path_exists", "manifest_output_path_exists"],
            lambda payload: payload["outputs"].update(
                {"manifest": "research/cryptanalysis/signature_readiness_20260516/does_not_exist.json"}
            ),
        ),
    ]
    for name, expected_checks, mutate in mutations:
        payload = clone_json(valid)
        mutate(payload)
        checks = [failure["check"] for failure in verify(payload)]
        cases.append(
            {
                "case": name,
                "passed": all(check in checks for check in expected_checks),
                "expected_checks": expected_checks,
                "actual_checks": checks,
            }
        )
    return {
        "schema": "Descender.third_party_review_packet_finalization_verification_self_test.v1",
        "status": "pass" if all(case["passed"] for case in cases) else "fail",
        "input": rel(input_path),
        "cases": cases,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, default=FINALIZATION_JSON)
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    parser.add_argument("--no-write", action="store_true")
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="Run in-memory negative controls for the finalization verifier.",
    )
    args = parser.parse_args()

    if args.self_test:
        payload = run_self_test(args.input)
        print(json.dumps(payload, indent=2, sort_keys=True))
        if payload["status"] != "pass":
            raise SystemExit(1)
        return

    finalization = load_json(args.input)
    failures = verify(finalization)
    self_test = run_self_test(args.input)
    if self_test["status"] != "pass":
        failures.append(
            {
                "check": "self_test",
                "status": self_test["status"],
                "failed_cases": [
                    case["case"] for case in self_test["cases"] if not case["passed"]
                ],
            }
        )
    payload = {
        "schema": "Descender.third_party_review_packet_finalization_verification.v1",
        "status": "pass" if not failures else "fail",
        "finalization_report": rel(args.input),
        "artifacts": {
            "json": rel(args.json),
            "markdown": rel(args.md),
        },
        "step_count": len(finalization.get("steps", [])),
        "independent_manifest_check_count": 6,
        "self_test": self_test,
        "expected_step_names": EXPECTED_STEP_NAMES,
        "failures": failures,
    }
    if not args.no_write:
        args.json.parent.mkdir(parents=True, exist_ok=True)
        args.md.parent.mkdir(parents=True, exist_ok=True)
        args.json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
        args.md.write_text(markdown(payload))
    print(json.dumps(payload, indent=2, sort_keys=True))
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
