#!/usr/bin/env python3
"""Finalize the local third-party review packet in dependency order."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "third_party_review_packet_finalization_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "third_party_review_packet_finalization_n32_q12289.md"
FINALIZATION_VERIFICATION_JSON = (
    READINESS_DIR / "third_party_review_packet_finalization_verification_n32_q12289.json"
)
FINALIZATION_VERIFICATION_MD = (
    READINESS_DIR / "third_party_review_packet_finalization_verification_n32_q12289.md"
)
MANIFEST_JSON = READINESS_DIR / "third_party_review_manifest_n32_q12289.json"
EXPECTED_FINALIZE_COMMAND = "python3 scripts/research/finalize_third_party_review_packet.py"
EXPECTED_FINALIZATION_VERIFY_COMMAND = (
    "python3 scripts/research/verify_third_party_review_finalization.py"
)
FINALIZER_SCRIPT = "scripts/research/finalize_third_party_review_packet.py"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def script(path: str) -> str:
    return str(ROOT / path)


def run_command(name: str, command: list[str]) -> dict[str, Any]:
    started = time.perf_counter()
    completed = subprocess.run(
        command,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    parsed_stdout = parse_json_tail(completed.stdout)
    return {
        "name": name,
        "command": " ".join(command),
        "returncode": completed.returncode,
        "status": "pass" if completed.returncode == 0 else "fail",
        "elapsed_seconds": round(time.perf_counter() - started, 3),
        "stdout_json": parsed_stdout,
        "stdout_tail": completed.stdout[-4000:],
        "stderr_tail": completed.stderr[-4000:],
    }


def parse_json_tail(text: str) -> dict[str, Any] | None:
    stripped = text.strip()
    if not stripped:
        return None
    try:
        return json.loads(stripped)
    except json.JSONDecodeError:
        return None


def build_replay_command(args: argparse.Namespace) -> list[str]:
    command = [
        sys.executable,
        script("scripts/research/review_gate_replay.py"),
        "--timing-iterations",
        str(args.timing_iterations),
    ]
    if args.skip_timing:
        command.append("--skip-timing")
    if args.replay_exact:
        command.append("--replay-exact")
    return command


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def check_manifest_finalizer_contract() -> dict[str, Any]:
    manifest = load_json(MANIFEST_JSON)
    paths = {str(row.get("path")) for row in manifest.get("files", [])}
    terminal_reports = {
        rel(DEFAULT_JSON),
        rel(DEFAULT_MD),
        rel(FINALIZATION_VERIFICATION_JSON),
        rel(FINALIZATION_VERIFICATION_MD),
    }
    checks = {
        "recommended_finalize_command": manifest.get("recommended_finalize_command")
        == EXPECTED_FINALIZE_COMMAND,
        "finalizer_script_indexed": FINALIZER_SCRIPT in paths,
        "terminal_finalization_report_excluded": paths.isdisjoint(terminal_reports),
    }
    failures = [name for name, passed in checks.items() if not passed]
    return {
        "status": "pass" if not failures else "fail",
        "checks": checks,
        "failures": failures,
        "terminal_reports": sorted(terminal_reports),
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Third-Party Review Packet Finalization",
        "",
        "This report records the ordered local finalization command for the review",
        "packet: replay first, regenerate standalone verification artifacts,",
        "regenerate and verify the status index, regenerate the hashed manifest,",
        "and verify the manifest last.  This avoids making the replay report,",
        "status index, and manifest hash each other cyclically.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- status: `{payload['status']}`",
        f"- timing skipped: `{payload['skip_timing']}`",
        f"- exact response-link replay: `{payload['replay_exact']}`",
        f"- timing iterations: `{payload['timing_iterations']}`",
        f"- manifest verification status: `{payload['manifest_verification'].get('status')}`",
        f"- manifest files checked: `{payload['manifest_verification'].get('files_checked')}`",
        f"- manifest failed files: `{payload['manifest_verification'].get('failed_files')}`",
        f"- manifest-level failures: `{len(payload['manifest_verification'].get('manifest_failures', []))}`",
        f"- finalizer contract: `{payload['finalizer_contract']['status']}`",
        f"- terminal verification command: `{payload['recommended_terminal_verification_command']}`",
        "",
        "## Steps",
        "",
        "| step | status | seconds |",
        "|---|---|---:|",
    ]
    for step in payload["steps"]:
        lines.append(
            f"| {step['name']} | {step['status']} | {step['elapsed_seconds']} |"
        )
    lines.extend(
        [
            "",
            "## Finalizer Contract",
            "",
            "The terminal finalization reports are intentionally not indexed by",
            "the hashed manifest.  The manifest hashes the replay report, status",
            "index, and scripts; the finalization report then records that",
            "verified manifest.  Indexing the terminal finalization reports",
            "inside the manifest would make the packet claim a cyclic hash",
            "dependency.",
            "",
            "| check | passed |",
            "|---|:---:|",
        ]
    )
    for name, passed in payload["finalizer_contract"]["checks"].items():
        lines.append(f"| {name} | {'yes' if passed else 'no'} |")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--timing-iterations", type=int, default=10_000)
    parser.add_argument("--skip-timing", action="store_true", default=True)
    parser.add_argument("--include-timing", dest="skip_timing", action="store_false")
    parser.add_argument("--replay-exact", action="store_true", default=True)
    parser.add_argument(
        "--static-response-link",
        dest="replay_exact",
        action="store_false",
        help="Run the faster static response-link manifest audit instead of exact replay.",
    )
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    args = parser.parse_args()

    if args.timing_iterations <= 0:
        raise SystemExit("--timing-iterations must be positive")

    steps = []
    for name, command in [
        ("review_gate_replay", build_replay_command(args)),
        (
            "verify_production_preflight_certificate",
            [
                sys.executable,
                script("scripts/research/verify_production_preflight_certificate.py"),
            ],
        ),
        (
            "verify_compact_signature_todos_consistency",
            [
                sys.executable,
                script("scripts/research/verify_compact_signature_todos_consistency.py"),
            ],
        ),
        (
            "verify_selected_deployment_terminology",
            [
                sys.executable,
                script("scripts/research/verify_selected_deployment_terminology.py"),
            ],
        ),
        (
            "third_party_review_status_index",
            [sys.executable, script("scripts/research/third_party_review_status_index.py")],
        ),
        (
            "verify_third_party_review_status_index",
            [sys.executable, script("scripts/research/verify_third_party_review_status_index.py")],
        ),
        (
            "third_party_review_manifest",
            [sys.executable, script("scripts/research/third_party_review_manifest.py")],
        ),
        (
            "verify_third_party_review_manifest",
            [sys.executable, script("scripts/research/verify_third_party_review_manifest.py")],
        ),
    ]:
        result = run_command(name, command)
        steps.append(result)
        if result["status"] != "pass":
            break

    verifier_step = next(
        (step for step in steps if step["name"] == "verify_third_party_review_manifest"),
        {},
    )
    manifest_verification = verifier_step.get("stdout_json") or {}
    finalizer_contract = (
        check_manifest_finalizer_contract()
        if manifest_verification.get("status") == "pass"
        else {"status": "not_run", "checks": {}, "failures": []}
    )
    status = (
        "pass"
        if len(steps) == 8
        and all(step["status"] == "pass" for step in steps)
        and manifest_verification.get("status") == "pass"
        and finalizer_contract.get("status") == "pass"
        else "fail"
    )
    payload = {
        "schema": "Descender.third_party_review_packet_finalization.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": status,
        "timing_iterations": args.timing_iterations,
        "skip_timing": args.skip_timing,
        "replay_exact": args.replay_exact,
        "recommended_terminal_verification_command": EXPECTED_FINALIZATION_VERIFY_COMMAND,
        "steps": steps,
        "manifest_verification": manifest_verification,
        "finalizer_contract": finalizer_contract,
        "outputs": {
            "replay_report": rel(
                READINESS_DIR / "review_gate_replay_report_n32_q12289.json"
            ),
            "manifest": rel(MANIFEST_JSON),
            "terminal_finalization_report_json": rel(args.json),
            "terminal_finalization_report_md": rel(args.md),
        },
    }

    args.json.parent.mkdir(parents=True, exist_ok=True)
    args.json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    args.md.write_text(markdown(payload))
    print(
        json.dumps(
            {"json": str(args.json), "markdown": str(args.md), "status": status},
            indent=2,
        )
    )
    if status != "pass":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
