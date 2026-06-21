#!/usr/bin/env python3
"""Check that active research automation routes through the pivot guard."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCRIPT_DIR = ROOT / "scripts/research"
README = SCRIPT_DIR / "README.md"

REQUIRED_ACTIVE_SCRIPTS = {
    "director.py",
    "launch_agent.py",
    "quality_check.py",
    "common.py",
    "pivot_guard.py",
    "audit_task_pivot_alignment.py",
    "quarantine_non_pivot_state.py",
    "check_research_automation_routing.py",
    "qrom_challenge_budget_memo.py",
    "qrom_security_loss_review.py",
    "pre_third_party_qrom_frame_probe.py",
    "qrom_multitarget_margin.py",
    "check_qrom_budget_sync.py",
    "check_qrom_generated_artifact_drift.py",
    "monitor_agents.py",
    "cleanup_paseo_agents.py",
}

SCRIPT_SUFFIXES = {".py", ".sh"}
MAX_UNLISTED_RESEARCH_SCRIPTS = 231

REQUIRED_SNIPPETS = {
    "README.md": (
        "Status: ACTIVE_RESEARCH_AUTOMATION_ENTRYPOINT",
        "DESCENDER_ALLOW_LEGACY_TASKS=1",
        "Legacy Artifact Generators",
        "New task generators should cite the current authority chain",
    ),
    "pivot_guard.py": (
        "LEGACY_PRODUCT_PHRASES",
        "LEGACY_ALLOWED_CONTEXT_PHRASES",
        "task_pivot_alignment_reason",
        "DESCENDER_ALLOW_LEGACY_TASKS",
    ),
    "common.py": (
        "from pivot_guard import task_pivot_alignment_reason",
        "task_pivot_alignment_reason(item)",
        "descender.queue.skipped",
    ),
    "director.py": (
        "from pivot_guard import task_pivot_aligned, task_pivot_alignment_reason, text_fields",
        "organized as a chain-native post-quantum authorization stack",
        "Current authority starts at `spec/README.md`, append-only ADRs in",
        "`research/ARCHIVE_INDEX.md` classifies old research",
        "descender.task.skipped",
    ),
    "launch_agent.py": (
        "Current project authority starts at spec/README.md, decisions/, and TRACKER.md.",
        "Use research/ARCHIVE_INDEX.md before treating older research notes as current.",
        "Old 308-byte/Fq12 projection-carrier scripts and notes are archive",
    ),
    "quality_check.py": (
        "Check whether the result respects the current project authority chain",
        "Treat work that advances legacy KEM/formalization goals",
        "chain-native PQ authorization pivot",
    ),
    "audit_task_pivot_alignment.py": (
        "from pivot_guard import task_pivot_alignment_reason",
        "--strict-queue",
        "--strict-state",
    ),
    "quarantine_non_pivot_state.py": (
        "from pivot_guard import task_pivot_alignment_reason",
        "pivot_archived",
        "descender.state.pivot_archived",
    ),
}


def split_table_row(line: str) -> list[str]:
    return [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]


def active_control_scripts(readme: str) -> set[str]:
    section_match = re.search(
        r"^## Active Control Scripts\s*(.*?)(?:^## |\Z)",
        readme,
        re.MULTILINE | re.DOTALL,
    )
    if section_match is None:
        return set()

    scripts: set[str] = set()
    for line in section_match.group(1).splitlines():
        if not line.startswith("|"):
            continue
        cells = split_table_row(line)
        if len(cells) < 2 or cells[0] in {"Script", "---"}:
            continue
        if cells[0].endswith(".py"):
            scripts.add(cells[0])
    return scripts


def research_scripts() -> set[str]:
    return {
        path.name
        for path in SCRIPT_DIR.iterdir()
        if path.is_file() and path.suffix in SCRIPT_SUFFIXES
    }


def require_snippets(rel: str, snippets: tuple[str, ...], errors: list[str]) -> None:
    path = README if rel == "README.md" else SCRIPT_DIR / rel
    if not path.exists():
        errors.append(f"scripts/research/{rel}: missing")
        return
    text = path.read_text(encoding="utf-8")
    for snippet in snippets:
        if snippet not in text:
            errors.append(f"scripts/research/{rel}: missing required snippet: {snippet!r}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--list-unlisted",
        action="store_true",
        help="print scripts retained as unlisted legacy helpers/generators",
    )
    args = parser.parse_args()

    errors: list[str] = []
    if not README.exists():
        errors.append("scripts/research/README.md: missing")
        readme_text = ""
    else:
        readme_text = README.read_text(encoding="utf-8")

    listed = active_control_scripts(readme_text)
    missing_from_readme = sorted(REQUIRED_ACTIVE_SCRIPTS - listed)
    if missing_from_readme:
        errors.append(
            "scripts/research/README.md: Active Control Scripts table is missing: "
            + ", ".join(missing_from_readme)
        )
    unexpected_active = sorted(listed - REQUIRED_ACTIVE_SCRIPTS)
    if unexpected_active:
        errors.append(
            "scripts/research/README.md: Active Control Scripts table has "
            "unexpected active scripts: "
            + ", ".join(unexpected_active)
        )

    for script in sorted(listed):
        if not (SCRIPT_DIR / script).exists():
            errors.append(f"scripts/research/README.md: listed active script does not exist: {script}")

    discovered = research_scripts()
    unlisted = sorted(discovered - listed)
    if len(unlisted) > MAX_UNLISTED_RESEARCH_SCRIPTS:
        errors.append(
            "scripts/research/: unlisted legacy helper/generator scripts grew "
            f"from baseline {MAX_UNLISTED_RESEARCH_SCRIPTS} to {len(unlisted)}"
        )
    if args.list_unlisted:
        print(f"unlisted research scripts: {len(unlisted)}")
        for script in unlisted:
            print(script)

    for rel, snippets in REQUIRED_SNIPPETS.items():
        require_snippets(rel, snippets, errors)

    if errors:
        print("research automation routing check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print(f"active research control scripts: {len(listed)}")
    print(f"unlisted research legacy/helper scripts: {len(unlisted)}")
    print("research automation routing check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
