#!/usr/bin/env python3
"""Check the consolidated tracker for process-level integrity.

This deliberately checks mechanical invariants, not whether an item is
cryptographically complete:

- every tracker row has a unique ID and allowed status;
- the tracker has the approved top-level section layout;
- evidence/next-action cells are non-empty;
- selected/proposed/in-progress/implemented/blocked rows carry actionable closure language;
- `verified` rows cite concrete verification evidence;
- archived/superseded rows carry historical retention or replacement routing;
- path-like code spans in evidence cells point at existing repo paths;
- ADR references point at existing append-only decision records.
- every ADR file is referenced from the consolidated tracker.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TRACKER = ROOT / "TRACKER.md"

TRACKER_STATUSES = {
    "proposed",
    "selected",
    "in_progress",
    "implemented",
    "verified",
    "blocked",
    "archived",
    "superseded",
}
TRACKER_STATUS_CONTRACTS = [
    ("proposed", "candidate work, not selected"),
    ("selected", "accepted direction, not yet implemented"),
    ("in_progress", "implementation or proof work underway"),
    ("implemented", "code/docs exist, verification incomplete"),
    ("verified", "tests/proofs/evidence cover the stated item"),
    ("blocked", "cannot progress without named dependency"),
    ("archived", "retained for history/negative controls"),
    ("superseded", "replaced by another item or ADR"),
]
TRACKER_SECTION_CONTRACTS = [
    ("Direct Mode: First Deployable Target", ("SPEC-", "ADR-", "DIR-")),
    ("Aggregation / Rollup Mode", ("AGG-",)),
    ("Policy Layer", ("POL-",)),
    ("Process / Cleanup", ("PROC-",)),
    ("Hardening / Adoption", ("ADOPT-",)),
]

TRACKER_ITEM_ID = re.compile(r"^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3}$")
CODE_SPAN = re.compile(r"`([^`]+)`")
ADR_REF = re.compile(r"\bADR-(\d{4})\b")
ADR_ROW_ID = re.compile(r"^ADR-(\d{3})$")
VERIFIED_EVIDENCE = re.compile(
    r"(scripts/|\.github/workflows/ci\.yml|test|tests|proof|audit|gate|gates|"
    r"check|checks|reports 0|verified|requires|wired)",
    re.I,
)
ACTIONABLE_STATUSES = {"proposed", "selected", "in_progress", "implemented", "blocked"}
HISTORICAL_STATUSES = {"archived", "superseded"}
ACTIONABLE_NEXT_ACTION = re.compile(
    r"\b("
    r"add|audit|choose|close|connect|convert|define|document|finish|gate|"
    r"generate|implement|instantiate|map|name|obtain|promote|prove|refactor|"
    r"replace|retire|review|run|select|specify|split|wire"
    r")\b|"
    r"\b(needs|required|requires|still|until|unless|before|after|blocked by)\b",
    re.I,
)
HISTORICAL_ROUTING = re.compile(
    r"\b("
    r"archive|archived|history|historical|negative control|negative controls|"
    r"retained|retire|retired|replaced|superseded"
    r")\b|ADR-\d{4}|[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3}",
    re.I,
)

PATH_EXTENSIONS = (
    ".md",
    ".py",
    ".rs",
    ".json",
    ".toml",
    ".lean",
    ".sh",
    ".yml",
    ".yaml",
)

TOP_LEVEL_PATHS = {"README.md", "TRACKER.md"}


def split_table_row(line: str) -> list[str]:
    return [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]


def is_separator_row(cells: list[str]) -> bool:
    return bool(cells) and all(set(cell) <= {"-", ":", " "} for cell in cells)


def section(text: str, heading: str, errors: list[str]) -> str:
    pattern = re.compile(rf"^## {re.escape(heading)}\s*$", re.MULTILINE)
    match = pattern.search(text)
    if match is None:
        errors.append(f"TRACKER.md: missing section ## {heading}")
        return ""
    next_heading = re.search(r"^## ", text[match.end() :], re.MULTILINE)
    if next_heading is None:
        return text[match.end() :]
    return text[match.end() : match.end() + next_heading.start()]


def h2_headings(text: str) -> list[str]:
    return re.findall(r"^## (.+)$", text, flags=re.MULTILINE)


def table_rows(block: str) -> list[list[str]]:
    rows: list[list[str]] = []
    for line in block.splitlines():
        if not line.startswith("|"):
            continue
        cells = split_table_row(line)
        if is_separator_row(cells):
            continue
        rows.append(cells)
    return rows


def adr_file_exists(number: int) -> bool:
    return any((ROOT / "decisions").glob(f"ADR-{number:04d}-*.md"))


def adr_file_numbers() -> set[int]:
    numbers: set[int] = set()
    for path in (ROOT / "decisions").glob("ADR-*.md"):
        match = re.match(r"ADR-(\d{4})-", path.name)
        if match:
            numbers.add(int(match.group(1)))
    return numbers


def looks_path_like(token: str) -> bool:
    if token in TOP_LEVEL_PATHS:
        return True
    if "/" in token:
        return True
    return token.endswith(PATH_EXTENSIONS)


def referenced_path(token: str) -> str | None:
    """Return a path candidate from a Markdown code span, if it names one."""

    parts = token.split()
    if not parts:
        return None
    command = parts[0]
    candidate = command.rstrip(".,:;")
    if not looks_path_like(candidate):
        return None
    return candidate


def check_tracker_framing_and_status_vocabulary(text: str, errors: list[str]) -> None:
    if "Status: ACTIVE_TRACKER" not in "\n".join(text.splitlines()[:8]):
        errors.append("TRACKER.md: missing Status: ACTIVE_TRACKER marker near top")

    required_phrases = [
        "consolidated tracker for the pivot to a chain-native",
        "post-quantum authorization stack",
        "Old trackers are historical unless an item is copied here",
    ]
    for phrase in required_phrases:
        if phrase not in text:
            errors.append(f"TRACKER.md: missing tracker framing phrase {phrase!r}")

    if set(TRACKER_STATUSES) != {status for status, _meaning in TRACKER_STATUS_CONTRACTS}:
        errors.append("scripts/check_tracker_integrity.py: status set and contract table diverge")

    block = section(text, "Status Values", errors)
    if "Use only these status values:" not in block:
        errors.append("TRACKER.md ## Status Values: missing use-only framing")
    rows = table_rows(block)
    if not rows:
        errors.append("TRACKER.md ## Status Values: missing status table")
        return
    if rows[0] != ["Status", "Meaning"]:
        errors.append("TRACKER.md ## Status Values: header must be ['Status', 'Meaning']")
        return
    data_rows = rows[1:]
    expected_rows = [list(row) for row in TRACKER_STATUS_CONTRACTS]
    if data_rows != expected_rows:
        errors.append(
            "TRACKER.md ## Status Values: rows must exactly match "
            f"{expected_rows!r}; got {data_rows!r}"
        )


def check_tracker_section_layout(text: str, errors: list[str]) -> None:
    expected_headings = ["Status Values"] + [
        heading for heading, _prefixes in TRACKER_SECTION_CONTRACTS
    ]
    headings = h2_headings(text)
    if headings != expected_headings:
        errors.append(
            "TRACKER.md: top-level sections must exactly match "
            f"{expected_headings!r}; got {headings!r}"
        )

    expected_header = ["ID", "Status", "Item", "Evidence / next action"]
    for heading, allowed_prefixes in TRACKER_SECTION_CONTRACTS:
        rows = table_rows(section(text, heading, errors))
        if not rows:
            errors.append(f"TRACKER.md ## {heading}: missing tracker row table")
            continue
        if rows[0] != expected_header:
            errors.append(
                f"TRACKER.md ## {heading}: header must be {expected_header!r}"
            )
            continue
        if len(rows) == 1:
            errors.append(f"TRACKER.md ## {heading}: missing tracker rows")
            continue
        for row in rows[1:]:
            if len(row) < 4 or not TRACKER_ITEM_ID.match(row[0]):
                continue
            item_id = row[0]
            if not item_id.startswith(allowed_prefixes):
                errors.append(
                    f"TRACKER.md ## {heading}: row {item_id} must use one of "
                    f"the allowed prefixes {allowed_prefixes!r}"
                )


def check_tracker(errors: list[str]) -> None:
    if not TRACKER.exists():
        errors.append("TRACKER.md: missing consolidated tracker")
        return

    text = TRACKER.read_text(encoding="utf-8")
    check_tracker_framing_and_status_vocabulary(text, errors)
    check_tracker_section_layout(text, errors)

    rows = 0
    seen: dict[str, int] = {}
    referenced_adrs: set[int] = set()
    for line_no, line in enumerate(text.splitlines(), start=1):
        if not line.startswith("|"):
            continue
        cells = split_table_row(line)
        if len(cells) < 4 or not TRACKER_ITEM_ID.match(cells[0]):
            continue

        rows += 1
        item_id, status, item, evidence = cells[:4]
        if item_id in seen:
            errors.append(
                f"TRACKER.md:{line_no}: duplicate tracker ID {item_id}; "
                f"first seen on line {seen[item_id]}"
            )
        else:
            seen[item_id] = line_no

        if status not in TRACKER_STATUSES:
            errors.append(f"TRACKER.md:{line_no}: unknown tracker status {status!r}")
        if not item:
            errors.append(f"TRACKER.md:{line_no}: empty tracker item text")
        if not evidence:
            errors.append(f"TRACKER.md:{line_no}: empty evidence / next-action cell")
        elif status in ACTIONABLE_STATUSES and not ACTIONABLE_NEXT_ACTION.search(evidence):
            errors.append(
                f"TRACKER.md:{line_no}: {status} row must name an actionable "
                "next action or blocking dependency"
            )
        elif status == "verified" and not VERIFIED_EVIDENCE.search(evidence):
            errors.append(
                f"TRACKER.md:{line_no}: verified row must cite concrete "
                "verification evidence"
            )
        elif status in HISTORICAL_STATUSES and not HISTORICAL_ROUTING.search(evidence):
            errors.append(
                f"TRACKER.md:{line_no}: {status} row must name historical "
                "retention, negative-control value, or replacement routing"
            )

        adr_row = ADR_ROW_ID.match(item_id)
        if adr_row and not adr_file_exists(int(adr_row.group(1))):
            errors.append(f"TRACKER.md:{line_no}: {item_id} has no matching decision record")

        for match in ADR_REF.finditer(line):
            number = int(match.group(1))
            referenced_adrs.add(number)
            if not adr_file_exists(number):
                errors.append(f"TRACKER.md:{line_no}: ADR-{number:04d} does not exist")

        for token in CODE_SPAN.findall(evidence):
            path = referenced_path(token)
            if path is None:
                continue
            if not (ROOT / path).exists():
                errors.append(f"TRACKER.md:{line_no}: referenced path does not exist: {path}")

    if rows == 0:
        errors.append("TRACKER.md: no tracker rows found")

    for number in sorted(adr_file_numbers() - referenced_adrs):
        errors.append(f"TRACKER.md: ADR-{number:04d} is not referenced by any tracker row")


def main() -> int:
    errors: list[str] = []
    check_tracker(errors)
    if errors:
        print("tracker integrity check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("tracker integrity check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
