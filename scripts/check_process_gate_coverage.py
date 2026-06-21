#!/usr/bin/env python3
"""Check that spec/process.md maps process areas to gates and active tracker owners."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PROCESS = ROOT / "spec" / "process.md"
LOCAL = ROOT / "scripts" / "check_build.sh"
CI = ROOT / ".github" / "workflows" / "ci.yml"
TRACKER = ROOT / "TRACKER.md"

EXPECTED_AREAS = (
    "Process spec coverage",
    "Authority entrypoints",
    "Spec spine",
    "ADR process",
    "Tracker consolidation",
    "Archive and classification",
    "Stale-path framing",
    "Tool and source framing",
    "Research automation",
    "Wire and requirement authority",
    "CI and gate ownership",
)
TRACKER_STATUS_ORDER = (
    "proposed",
    "selected",
    "in_progress",
    "implemented",
    "verified",
    "blocked",
    "archived",
    "superseded",
)
ACTIVE_TRACKER_STATUSES = {
    "proposed",
    "selected",
    "in_progress",
    "implemented",
    "verified",
    "blocked",
}
TRACKER_SECTION_PREFIXES = (
    ("Direct Mode: First Deployable Target", ("SPEC-*", "ADR-*", "DIR-*")),
    ("Aggregation / Rollup Mode", ("AGG-*",)),
    ("Policy Layer", ("POL-*",)),
    ("Process / Cleanup", ("PROC-*",)),
    ("Hardening / Adoption", ("ADOPT-*",)),
)
ADR_STATUS_ORDER = (
    "Accepted",
    "Proposed",
    "Rejected",
    "Superseded",
)
ADR_REQUIRED_SECTION_NAMES = (
    "## Context",
    "## Decision",
    "## Consequences",
    "## Supersedes",
    "## Superseded By",
    "## Affected Specs / Tracker Items",
)

CODE_SPAN = re.compile(r"`([^`]+)`")
TABLE_ROW = re.compile(r"^\|\s*(.+?)\s*\|\s*(.+?)\s*\|\s*(.+?)\s*\|$")
TRACKER_ITEM_ID = re.compile(r"^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3}$")


def markdown_section(text: str, heading: str) -> list[str]:
    lines = text.splitlines()
    start: int | None = None
    for index, line in enumerate(lines):
        if line.strip() == heading:
            start = index + 1
            break
    if start is None:
        return []

    section: list[str] = []
    for line in lines[start:]:
        if line.startswith("## "):
            break
        section.append(line)
    return section


def tracker_item_statuses() -> dict[str, str]:
    statuses: dict[str, str] = {}
    for line in TRACKER.read_text(encoding="utf-8").splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]
        if len(cells) >= 2 and TRACKER_ITEM_ID.match(cells[0]):
            statuses[cells[0]] = cells[1]
    return statuses


def process_rows(errors: list[str]) -> dict[str, tuple[list[str], list[str]]]:
    section = markdown_section(PROCESS.read_text(encoding="utf-8"), "## Process Gate Coverage")
    if not section:
        errors.append("spec/process.md: missing ## Process Gate Coverage section")
        return {}

    rows: dict[str, tuple[list[str], list[str]]] = {}
    for line in section:
        match = TABLE_ROW.match(line)
        if not match:
            continue
        area, gates_cell, owners_cell = [part.strip() for part in match.groups()]
        if area in {"Area", "---"}:
            continue
        gates = CODE_SPAN.findall(gates_cell)
        owners = CODE_SPAN.findall(owners_cell)
        if area in rows:
            errors.append(f"spec/process.md: duplicate process gate coverage area: {area}")
        rows[area] = (gates, owners)
    return rows


def split_markdown_table_row(line: str) -> list[str] | None:
    if not line.startswith("|"):
        return None
    cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
    if cells and all(set(cell) <= {"-", ":", " "} for cell in cells):
        return None
    return cells


def check_decision_rules_section(errors: list[str]) -> None:
    section = markdown_section(PROCESS.read_text(encoding="utf-8"), "## Decision Rules")
    if not section:
        errors.append("spec/process.md: missing ## Decision Rules section")
        return

    text = "\n".join(section)
    required_phrases = [
        "ADRs are append-only. A changed decision gets a new ADR; old ADRs are not edited to make history look cleaner.",
        "Every ADR must have:",
        "- a filename `ADR-NNNN-short-name.md`;",
        "- a matching `# ADR-NNNN: Title` heading;",
        "- `Status:` and `Date:` fields near the top;",
        "- `## Context`, `## Decision`, and `## Consequences` sections;",
        "- `## Supersedes`, `## Superseded By`, and `## Affected Specs / Tracker Items` sections.",
        "Use `None.` when an ADR has no superseded predecessor or successor.",
        "rather than rewriting the old decision.",
        "must cite current repo paths, `ADR-NNNN` records, or `TRACKER.md` row IDs in backticks.",
        "must reciprocally cite the ADR number",
        "For final ADRs (`Accepted`, `Superseded`, or `Rejected`), the `## Context`, `## Decision`, and `## Consequences` sections are immutable decision text.",
        "Their hashes must be listed in `decisions/ADR_IMMUTABLE_SECTIONS.sha256`.",
        "status, supersession, and affected tracker references remain mutable routing metadata.",
        "`decisions/README.md` must include an `## ADR Index` table listing every ADR by number, status, date, title, and filename.",
        "Active support files with `decision` in the filename are research decision notes, not append-only ADRs.",
    ]
    normalized = re.sub(r"\s+", " ", text)
    for phrase in required_phrases:
        normalized_phrase = re.sub(r"\s+", " ", phrase)
        if normalized_phrase not in normalized:
            errors.append(f"spec/process.md ## Decision Rules: missing phrase {phrase!r}")

    for heading in ADR_REQUIRED_SECTION_NAMES:
        if f"`{heading}`" not in text:
            errors.append(f"spec/process.md ## Decision Rules: missing ADR section name {heading}")

    status_phrase = (
        "Accepted decision status values are "
        + ", ".join(f"`{status}`" for status in ADR_STATUS_ORDER[:-1])
        + f", and `{ADR_STATUS_ORDER[-1]}`."
    )
    if re.sub(r"\s+", " ", status_phrase) not in normalized:
        errors.append(
            "spec/process.md ## Decision Rules: status vocabulary must be "
            f"{ADR_STATUS_ORDER!r}"
        )


def check_tracker_rules_section(errors: list[str]) -> None:
    section = markdown_section(PROCESS.read_text(encoding="utf-8"), "## Tracker Rules")
    if not section:
        errors.append("spec/process.md: missing ## Tracker Rules section")
        return

    text = "\n".join(section)
    required_phrases = [
        "`TRACKER.md` is the only top-level implementation/proof/process tracker.",
        "Old trackers are evidence only unless a row in `TRACKER.md` links to them.",
        "The consolidated tracker's top-level work sections and row-ID prefixes are fixed:",
        "Use `implemented` when code/docs exist but the evidence is not yet sufficient",
        "Use `verified` only when the named tests, proof artifacts, or review evidence cover the stated item.",
        "Rows with `proposed`, `selected`, `in_progress`, `implemented`, or `blocked`",
        "Passive placeholders such as a bare ADR, a topic list, or an imported support file are not enough",
        "Rows with `archived` or `superseded` status must name historical retention, negative-control value, or replacement routing",
    ]
    normalized = re.sub(r"\s+", " ", text)
    for phrase in required_phrases:
        normalized_phrase = re.sub(r"\s+", " ", phrase)
        if normalized_phrase not in normalized:
            errors.append(f"spec/process.md ## Tracker Rules: missing phrase {phrase!r}")

    status_match = re.search(r"Tracker rows must use one of:\n\n```text\n(.*?)\n```", text, re.DOTALL)
    if status_match is None:
        errors.append("spec/process.md ## Tracker Rules: missing tracker status code block")
    else:
        statuses = tuple(status_match.group(1).splitlines())
        if statuses != TRACKER_STATUS_ORDER:
            errors.append(
                "spec/process.md ## Tracker Rules: status list must be "
                f"{TRACKER_STATUS_ORDER!r}; got {statuses!r}"
            )

    expected_rows = [
        ["Section", "Allowed row-ID prefixes"],
        *[
            [section_name, ", ".join(f"`{prefix}`" for prefix in prefixes)]
            for section_name, prefixes in TRACKER_SECTION_PREFIXES
        ],
    ]
    table_rows: list[list[str]] = []
    for line in section:
        cells = split_markdown_table_row(line)
        if cells is None or len(cells) != 2:
            continue
        if cells[0] == "Section" or any(cells[0] == row[0] for row in expected_rows[1:]):
            table_rows.append(cells)
    if table_rows != expected_rows:
        errors.append(
            "spec/process.md ## Tracker Rules: section/prefix table must be "
            f"{expected_rows!r}; got {table_rows!r}"
        )


def check_active_support_import_rules(errors: list[str]) -> None:
    text = PROCESS.read_text(encoding="utf-8")
    required_phrases = [
        "The active-support import audit prevents `ACTIVE_SUPPORT` rows from growing as free-floating support claims.",
        "An active-support row is anchored only when a current authority file cites its path as a literal Markdown code span",
        "incidental plain-text mentions do not count.",
        "The gate currently requires zero unanchored rows",
        "new `ACTIVE_SUPPORT` rows must be imported immediately or the build fails",
        "python3 scripts/check_active_support_imports.py --max-unanchored 0",
        "Use `--list` to inspect unanchored rows and `--show-importers` to inspect the authority file locations",
    ]
    normalized = re.sub(r"\s+", " ", text)
    for phrase in required_phrases:
        normalized_phrase = re.sub(r"\s+", " ", phrase)
        if normalized_phrase not in normalized:
            errors.append(f"spec/process.md active-support import rules: missing phrase {phrase!r}")


def check_ci_gate_ownership_rules(errors: list[str]) -> None:
    text = PROCESS.read_text(encoding="utf-8")
    required_phrases = [
        "The local gate and CI must stay in sync.",
        "`scripts/check_build.sh` and `.github/workflows/ci.yml` both cover the same required documentation, tracker, wire-format, Lean, Rust, KAT, QROM, and cryptanalysis support gates.",
        "It also compares discovered `python3 scripts/...` gate commands",
        "Each named required gate must also have at least one `TRACKER.md` owner row with an active tracker status",
        "build checks remain tied to live consolidated-tracker work rather than becoming a separate task list.",
        "The gate inventory itself must have unique labels, non-empty local patterns, and non-empty CI patterns",
        "Every named gate must also have a matching visible local `echo \"== ... ==\"` label and a matching GitHub Actions `name:` label",
        "local and CI failures remain identifiable by the same canonical gate names",
        "python3 scripts/check_ci_local_gate_parity.py",
    ]
    normalized = re.sub(r"\s+", " ", text)
    for phrase in required_phrases:
        normalized_phrase = re.sub(r"\s+", " ", phrase)
        if normalized_phrase not in normalized:
            errors.append(f"spec/process.md CI gate ownership rules: missing phrase {phrase!r}")


def main() -> int:
    errors: list[str] = []
    check_decision_rules_section(errors)
    check_tracker_rules_section(errors)
    check_active_support_import_rules(errors)
    check_ci_gate_ownership_rules(errors)
    rows = process_rows(errors)
    expected = set(EXPECTED_AREAS)
    actual = set(rows)

    missing = sorted(expected - actual)
    extra = sorted(actual - expected)
    if missing:
        errors.append("spec/process.md: Process Gate Coverage missing areas: " + ", ".join(missing))
    if extra:
        errors.append("spec/process.md: Process Gate Coverage has unexpected areas: " + ", ".join(extra))

    local_text = LOCAL.read_text(encoding="utf-8")
    ci_text = CI.read_text(encoding="utf-8")
    tracker_statuses = tracker_item_statuses()

    for area in EXPECTED_AREAS:
        gates, owners = rows.get(area, ([], []))
        if not gates:
            errors.append(f"spec/process.md: {area} needs at least one gate command")
        if not owners:
            errors.append(f"spec/process.md: {area} needs at least one tracker owner")

        for gate in gates:
            if not gate.startswith("python3 scripts/"):
                errors.append(
                    f"spec/process.md: {area} has unsupported gate command {gate!r}; "
                    "use explicit python3 scripts/... gates in this coverage table"
                )
                continue
            if gate not in local_text:
                errors.append(f"scripts/check_build.sh: missing process coverage gate for {area}: {gate}")
            if gate not in ci_text:
                errors.append(f".github/workflows/ci.yml: missing process coverage gate for {area}: {gate}")

        for owner in owners:
            if not TRACKER_ITEM_ID.match(owner):
                errors.append(f"spec/process.md: {area} has invalid tracker owner ID: {owner}")
            else:
                status = tracker_statuses.get(owner)
                if status is None:
                    errors.append(f"spec/process.md: {area} references missing tracker owner: {owner}")
                elif status not in ACTIVE_TRACKER_STATUSES:
                    errors.append(
                        f"spec/process.md: {area} references inactive tracker owner "
                        f"{owner} with status {status!r}"
                    )

    if errors:
        print("process gate coverage check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print(f"process gate coverage check passed ({len(EXPECTED_AREAS)} areas)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
