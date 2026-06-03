#!/usr/bin/env python3
"""Verify that the compact-signature TODO checklist matches live review artifacts."""

from __future__ import annotations

import argparse
import importlib.util
import json
import re
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
TODOS = READINESS_DIR / "compact_signature_todos.md"
REPLAY_REPORT = READINESS_DIR / "review_gate_replay_report_n32_q12289.json"
MANIFEST_SCRIPT = ROOT / "scripts/research/third_party_review_manifest.py"
DEFAULT_JSON = READINESS_DIR / "compact_signature_todos_consistency_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "compact_signature_todos_consistency_n32_q12289.md"

FORBIDDEN_TODO_PATTERNS: tuple[tuple[str, str], ...] = (
    ("selected_250_order", r"selected[^.\n]{0,80}250\s*(?:B|byte)"),
    ("250_selected_order", r"250\s*(?:B|byte)[^.\n]{0,80}selected"),
    ("stale_manifest_59", r"(?:currently\s+indexes\s+`?59`?|59\s+indexed)"),
    ("stale_replay_17", r"`?17`?\s+generation/audit"),
    ("stale_replay_19", r"`?19`?\s+valid\s+JSON"),
    ("stale_replay_56_3", r"56\.3\s*s"),
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def load_manifest_module() -> Any:
    spec = importlib.util.spec_from_file_location(
        "descender_third_party_review_manifest", MANIFEST_SCRIPT
    )
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {MANIFEST_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def find_forbidden_todo_patterns(text: str) -> list[dict[str, Any]]:
    failures = []
    for label, pattern in FORBIDDEN_TODO_PATTERNS:
        for match in re.finditer(pattern, text, flags=re.IGNORECASE):
            failures.append(
                {
                    "label": label,
                    "line": text.count("\n", 0, match.start()) + 1,
                    "match": match.group(0),
                }
            )
    return failures


def required_snippets(
    replay_report: dict[str, Any], manifest_file_count: int
) -> dict[str, str]:
    replay_steps = len(replay_report.get("steps", []))
    hashed_json = replay_report.get("hashed_json_artifact_count")
    return {
        "replay_step_count": f"`{replay_steps}` generation/audit",
        "hashed_json_artifact_count": f"`{hashed_json}` valid JSON artifacts",
        "manifest_file_count": f"currently indexes `{manifest_file_count}`",
        "exact_replay_step": "standalone response-link certificate replay",
        "selected_deployment_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
    }


def check_text(
    text: str, replay_report: dict[str, Any], manifest_file_count: int
) -> tuple[dict[str, str], list[dict[str, str]], list[dict[str, Any]]]:
    snippets = required_snippets(replay_report, manifest_file_count)
    missing = [
        {"label": label, "snippet": snippet}
        for label, snippet in snippets.items()
        if snippet not in text
    ]
    forbidden = find_forbidden_todo_patterns(text)
    return snippets, missing, forbidden


def run_self_test(replay_report: dict[str, Any], manifest_file_count: int) -> dict[str, Any]:
    snippets = required_snippets(replay_report, manifest_file_count)
    clean_text = "\n".join(snippets.values())
    cases = [
        {
            "case": "current_required_snippets_pass",
            "text": clean_text,
            "expected_forbidden_labels": [],
            "expected_missing_labels": [],
        },
        {
            "case": "stale_manifest_count_detected",
            "text": clean_text.replace(
                snippets["manifest_file_count"], "currently indexes `59`"
            ),
            "expected_forbidden_labels": ["stale_manifest_59"],
            "expected_missing_labels": ["manifest_file_count"],
        },
        {
            "case": "stale_replay_count_detected",
            "text": clean_text.replace(
                snippets["replay_step_count"], "`17` generation/audit"
            ),
            "expected_forbidden_labels": ["stale_replay_17"],
            "expected_missing_labels": ["replay_step_count"],
        },
        {
            "case": "selected_250_wording_detected",
            "text": clean_text + "\n" + "selected " + "250 B target",
            "expected_forbidden_labels": ["selected_250_order"],
            "expected_missing_labels": [],
        },
    ]
    rows = []
    for case in cases:
        _snippets, missing, forbidden = check_text(
            case["text"], replay_report, manifest_file_count
        )
        forbidden_labels = [item["label"] for item in forbidden]
        missing_labels = [item["label"] for item in missing]
        rows.append(
            {
                "case": case["case"],
                "passed": sorted(forbidden_labels)
                == sorted(case["expected_forbidden_labels"])
                and sorted(missing_labels) == sorted(case["expected_missing_labels"]),
                "expected_forbidden_labels": case["expected_forbidden_labels"],
                "actual_forbidden_labels": forbidden_labels,
                "expected_missing_labels": case["expected_missing_labels"],
                "actual_missing_labels": missing_labels,
            }
        )
    return {
        "schema": "Descender.compact_signature_todos_consistency_self_test.v1",
        "status": "pass" if all(row["passed"] for row in rows) else "fail",
        "cases": rows,
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Compact Signature TODO Consistency",
        "",
        "This verifier checks that the main compact-signature TODO checklist does",
        "not drift away from live review-packet replay and manifest counts.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- TODO file: `{payload['todo_file']}`",
        f"- replay steps: `{payload['observed']['replay_step_count']}`",
        f"- hashed JSON artifacts: `{payload['observed']['hashed_json_artifact_count']}`",
        f"- manifest files: `{payload['observed']['manifest_file_count']}`",
        f"- forbidden pattern failures: `{len(payload['forbidden_pattern_failures'])}`",
        f"- missing required snippets: `{len(payload['missing_required_snippets'])}`",
        f"- self-test status: `{payload['self_test']['status']}`",
        f"- self-test cases: `{len(payload['self_test']['cases'])}`",
        "",
        "## Artifacts",
        "",
        f"- JSON: `{payload['artifacts']['json']}`",
        f"- Markdown: `{payload['artifacts']['markdown']}`",
        "",
    ]
    if payload["forbidden_pattern_failures"]:
        lines.extend(["## Forbidden Pattern Failures", ""])
        for failure in payload["forbidden_pattern_failures"]:
            lines.append(
                f"- line `{failure['line']}` `{failure['label']}` matched `{failure['match']}`"
            )
        lines.append("")
    if payload["missing_required_snippets"]:
        lines.extend(["## Missing Required Snippets", ""])
        for item in payload["missing_required_snippets"]:
            lines.append(f"- `{item['label']}` expected `{item['snippet']}`")
        lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    parser.add_argument("--no-write", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    replay_report = load_json(REPLAY_REPORT)
    manifest_module = load_manifest_module()
    manifest_file_count = len(manifest_module.packet_files())
    if args.self_test:
        self_test = run_self_test(replay_report, manifest_file_count)
        print(json.dumps(self_test, indent=2, sort_keys=True))
        if self_test["status"] != "pass":
            raise SystemExit(1)
        return

    text = TODOS.read_text()
    snippets, missing, forbidden = check_text(text, replay_report, manifest_file_count)
    self_test = run_self_test(replay_report, manifest_file_count)
    if self_test["status"] != "pass":
        forbidden.append(
            {
                "label": "self_test",
                "line": 0,
                "match": "self_test_failed",
            }
        )
    status = "pass" if not missing and not forbidden else "fail"
    payload = {
        "schema": "Descender.compact_signature_todos_consistency.v1",
        "status": status,
        "todo_file": rel(TODOS),
        "artifacts": {
            "json": rel(args.json),
            "markdown": rel(args.md),
        },
        "observed": {
            "replay_step_count": len(replay_report.get("steps", [])),
            "hashed_json_artifact_count": replay_report.get(
                "hashed_json_artifact_count"
            ),
            "manifest_file_count": manifest_file_count,
        },
        "required_snippets": snippets,
        "missing_required_snippets": missing,
        "forbidden_patterns": [label for label, _pattern in FORBIDDEN_TODO_PATTERNS],
        "forbidden_pattern_failures": forbidden,
        "self_test": self_test,
    }
    if not args.no_write:
        args.json.parent.mkdir(parents=True, exist_ok=True)
        args.md.parent.mkdir(parents=True, exist_ok=True)
        args.json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
        args.md.write_text(markdown(payload))
    print(json.dumps(payload, indent=2, sort_keys=True))
    if status != "pass":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
