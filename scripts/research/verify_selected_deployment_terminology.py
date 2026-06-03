#!/usr/bin/env python3
"""Fail if review-facing artifacts regress to stale 250-byte verifier wording."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
COST_MODEL_DIR = (
    ROOT / "research/parameter_search/committed_sigma_account_carried_20260521"
)
DEFAULT_JSON = READINESS_DIR / "selected_deployment_terminology_verification_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "selected_deployment_terminology_verification_n32_q12289.md"
SELF_GENERATED_REPORT_STEMS = {
    "selected_deployment_terminology_verification_n32_q12289",
    "third_party_review_packet_finalization_n32_q12289",
}

FORBIDDEN_PATTERNS: tuple[tuple[str, str], ...] = (
    (
        "final_250_byte_verifier",
        r"final\s+`?250\s*B`?\s+verifier|final\s+250-byte\s+verifier",
    ),
    ("reviewed_250_byte_verifier", r"reviewed\s+250-byte\s+verifier"),
    (
        "250_byte_account_carried_verifier",
        r"250-byte\s+account-carried\s+verifier",
    ),
    (
        "250_byte_verifier_replacement",
        r"250-byte\s+verifier\s+replacement",
    ),
    (
        "legacy_254_to_250_verifier",
        r"replace[^.\n]*254-byte[^.\n]*250-byte[^.\n]*verifier",
    ),
    ("rather_than_the_final", r"rather\s+than\s+the\s+final"),
    ("pending_final_verifier", r"pending\s+final\s+verifier"),
)

REQUIRED_PHRASES = (
    "selected 308-byte/Fq12 active-lane projection-carrier deployment verifier is fail-closed until active-lane preflight promotion and hidden-source semantics are implemented",
    "accepting 308-byte/Fq12",
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def default_paths() -> list[Path]:
    paths: list[Path] = []
    for base in [
        READINESS_DIR,
        COST_MODEL_DIR,
        ROOT / "scripts/research",
        ROOT / "scripts/parameter_search",
        ROOT / "tools/production-reference",
    ]:
        if not base.exists():
            continue
        paths.extend(
            path
            for path in base.rglob("*")
            if path.is_file()
            and path.suffix in {".md", ".json", ".py", ".rs", ".toml"}
            and "target/" not in path.as_posix()
        )
    paths.extend(
        [
            ROOT / "tools/parameter-search/src/production_reference.rs",
            ROOT / "tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs",
        ]
    )
    deduped = sorted(
        path.resolve()
        for path in paths
        if path.stem not in SELF_GENERATED_REPORT_STEMS
    )
    return [Path(path) for path in deduped]


def scan_path(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    text = path.read_text(errors="replace")
    return scan_text(text, path=rel(path))


def scan_text(text: str, path: str) -> list[dict[str, Any]]:
    failures = []
    for label, pattern in FORBIDDEN_PATTERNS:
        for match in re.finditer(pattern, text, flags=re.IGNORECASE):
            line = text.count("\n", 0, match.start()) + 1
            failures.append(
                {
                    "path": path,
                    "line": line,
                    "label": label,
                    "match": match.group(0),
                }
            )
    return failures


def required_phrase_hits(text: str) -> dict[str, list[str]]:
    return {
        phrase: ["synthetic"] if phrase in text else []
        for phrase in REQUIRED_PHRASES
    }


def run_self_test() -> dict[str, Any]:
    good = "\n".join(REQUIRED_PHRASES)
    cases = [
        {
            "case": "clean_required_phrases_pass",
            "expected_forbidden_labels": [],
            "expected_missing_required_count": 0,
            "text": good,
        },
        {
            "case": "forbidden_final_250_label_detected",
            "expected_forbidden_labels": ["final_250_byte_verifier"],
            "expected_missing_required_count": 0,
            "text": good + "\n" + "final " + "250-byte verifier",
        },
        {
            "case": "forbidden_account_carried_250_label_detected",
            "expected_forbidden_labels": ["250_byte_account_carried_verifier"],
            "expected_missing_required_count": 0,
            "text": good + "\n" + "250-byte " + "account-carried verifier",
        },
        {
            "case": "missing_required_phrases_detected",
            "expected_forbidden_labels": [],
            "expected_missing_required_count": len(REQUIRED_PHRASES),
            "text": "review packet without the selected deployment phrases",
        },
    ]
    rows = []
    for case in cases:
        failures = scan_text(case["text"], path="synthetic")
        labels = [failure["label"] for failure in failures]
        hits = required_phrase_hits(case["text"])
        missing_required = [
            phrase for phrase, phrase_hits in hits.items() if not phrase_hits
        ]
        rows.append(
            {
                "case": case["case"],
                "passed": sorted(labels) == sorted(case["expected_forbidden_labels"])
                and len(missing_required) == case["expected_missing_required_count"],
                "expected_forbidden_labels": case["expected_forbidden_labels"],
                "actual_forbidden_labels": labels,
                "expected_missing_required_count": case[
                    "expected_missing_required_count"
                ],
                "actual_missing_required_count": len(missing_required),
            }
        )
    return {
        "schema": "Descender.selected_deployment_terminology_verification_self_test.v1",
        "status": "pass" if all(row["passed"] for row in rows) else "fail",
        "cases": rows,
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Selected Deployment Terminology Verification",
        "",
        "This verifier fails if review-facing artifacts regress to stale language",
        "that treats an exact 250-byte verifier as the selected deployment target.",
        "The selected deployment target is now the fail-closed 308-byte/Fq12",
        "projection-carrier verifier until exact fallback certificate and",
        "leakage semantics are implemented.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- files checked: `{payload['files_checked']}`",
        f"- forbidden pattern count: `{len(payload['forbidden_patterns'])}`",
        f"- failures: `{len(payload['failures'])}`",
        f"- missing required phrases: `{len(payload['missing_required_phrases'])}`",
        f"- self-test status: `{payload['self_test']['status']}`",
        f"- self-test cases: `{len(payload['self_test']['cases'])}`",
        "",
        "## Artifacts",
        "",
        f"- JSON: `{payload['artifacts']['json']}`",
        f"- Markdown: `{payload['artifacts']['markdown']}`",
        "",
        "## Forbidden Labels",
        "",
    ]
    lines.extend(f"- `{label}`" for label in payload["forbidden_patterns"])
    lines.extend(
        [
            "",
            "## Required Phrase Hits",
            "",
        ]
    )
    for phrase, hits in payload["required_phrase_hits"].items():
        lines.append(f"- `{phrase}`: `{len(hits)}` recorded hits")
    if payload["failures"]:
        lines.extend(["", "## Failures", ""])
        for failure in payload["failures"]:
            lines.append(
                f"- `{failure['path']}:{failure['line']}` "
                f"`{failure['label']}` matched `{failure['match']}`"
            )
    if payload["missing_required_phrases"]:
        lines.extend(["", "## Missing Required Phrases", ""])
        lines.extend(f"- `{phrase}`" for phrase in payload["missing_required_phrases"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("paths", nargs="*", type=Path)
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    parser.add_argument(
        "--no-write",
        action="store_true",
        help="Print the verification result without updating review artifacts.",
    )
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="Run synthetic negative controls without scanning review files.",
    )
    args = parser.parse_args()

    if args.self_test:
        self_test = run_self_test()
        print(json.dumps(self_test, indent=2, sort_keys=True))
        if self_test["status"] != "pass":
            raise SystemExit(1)
        return

    paths = args.paths or default_paths()
    failures = []
    required_hits = {phrase: [] for phrase in REQUIRED_PHRASES}
    for path in paths:
        if not path.exists() or not path.is_file():
            continue
        text = path.read_text(errors="replace")
        failures.extend(scan_path(path))
        for phrase in REQUIRED_PHRASES:
            if phrase in text:
                required_hits[phrase].append(rel(path))

    missing_required = [
        phrase for phrase, hits in required_hits.items() if not hits
    ]
    self_test = run_self_test()
    if self_test["status"] != "pass":
        failures.append({"path": "self_test", "line": 0, "label": "self_test", "match": "self_test_failed"})
    output = {
        "schema": "Descender.selected_deployment_terminology_verification.v1",
        "status": "pass" if not failures and not missing_required else "fail",
        "artifacts": {
            "json": rel(args.json),
            "markdown": rel(args.md),
        },
        "files_checked": len(paths),
        "forbidden_patterns": [label for label, _pattern in FORBIDDEN_PATTERNS],
        "failures": failures,
        "required_phrase_hits": {
            phrase: hits[:20] for phrase, hits in required_hits.items()
        },
        "missing_required_phrases": missing_required,
        "self_test": self_test,
    }
    if not args.no_write:
        args.json.parent.mkdir(parents=True, exist_ok=True)
        args.md.parent.mkdir(parents=True, exist_ok=True)
        args.json.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")
        args.md.write_text(markdown(output))
    print(json.dumps(output, indent=2, sort_keys=True))
    if output["status"] != "pass":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
