#!/usr/bin/env python3
"""Audit ACTIVE_SUPPORT archive rows against the current authority chain.

`ACTIVE_SUPPORT` should mean "this support note is imported by current
authority", not merely "this note still looks interesting." Imports must be
literal code-span references from current authority files so incidental prose
mentions do not anchor a support row by accident. Superseded or rejected ADRs
are decision history, not current import anchors.
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ARCHIVE_INDEX = ROOT / "research/ARCHIVE_INDEX.md"

ARCHIVE_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+`([^`]+)`\s+\|")
CODE_SPAN = re.compile(r"`([^`]+)`")
ACTIVE_ADR_STATUSES = {"Accepted", "Proposed"}


def adr_status(path: Path) -> str | None:
    match = re.search(r"^Status:\s+(.+)$", path.read_text(encoding="utf-8"), re.MULTILINE)
    return match.group(1).strip() if match else None


def archive_rows() -> list[tuple[str, str]]:
    rows: list[tuple[str, str]] = []
    for line in ARCHIVE_INDEX.read_text(encoding="utf-8").splitlines():
        match = ARCHIVE_ROW.match(line)
        if not match:
            continue
        path, label = match.groups()
        if path in {"File", "Package", "Label"}:
            continue
        rows.append((path, label))
    return rows


def authority_files() -> list[Path]:
    files = [
        ROOT / "README.md",
        ROOT / "TRACKER.md",
        ROOT / "decisions/README.md",
    ]
    files.extend(sorted((ROOT / "spec").glob("*.md")))
    files.extend(
        path
        for path in sorted((ROOT / "decisions").glob("ADR-*.md"))
        if adr_status(path) in ACTIVE_ADR_STATUSES
    )
    return [path for path in files if path.exists()]


def importers_by_path(support_paths: set[str]) -> dict[str, list[str]]:
    importers = {path: [] for path in support_paths}
    for authority in authority_files():
        rel_authority = authority.relative_to(ROOT).as_posix()
        for line_no, line in enumerate(
            authority.read_text(encoding="utf-8", errors="replace").splitlines(),
            start=1,
        ):
            for ref in CODE_SPAN.findall(line):
                if ref in importers:
                    importers[ref].append(f"{rel_authority}:{line_no}")
    return importers


def main() -> int:
    parser = argparse.ArgumentParser(
        description=(
            "Audit ACTIVE_SUPPORT archive rows for direct import by current "
            "authority files."
        )
    )
    parser.add_argument(
        "--max-unanchored",
        type=int,
        default=None,
        help="fail if more than this many ACTIVE_SUPPORT rows are unanchored",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="equivalent to --max-unanchored 0",
    )
    parser.add_argument(
        "--list",
        action="store_true",
        help="print unanchored ACTIVE_SUPPORT rows even when within the baseline",
    )
    parser.add_argument(
        "--show-importers",
        action="store_true",
        help="print importing authority file locations for anchored ACTIVE_SUPPORT rows",
    )
    args = parser.parse_args()

    support = [path for path, label in archive_rows() if label == "ACTIVE_SUPPORT"]
    importers = importers_by_path(set(support))
    imported = {path for path, locations in importers.items() if locations}
    unanchored = sorted(path for path, locations in importers.items() if not locations)

    print(f"ACTIVE_SUPPORT rows: {len(support)}")
    print(f"directly imported by current authority: {len(imported)}")
    print(f"unanchored ACTIVE_SUPPORT rows: {len(unanchored)}")

    max_unanchored = 0 if args.strict else args.max_unanchored
    if args.list or (max_unanchored is not None and len(unanchored) > max_unanchored):
        for path in unanchored:
            print(path)
    if args.show_importers:
        for path in sorted(imported):
            print(f"{path}: {', '.join(importers[path])}")

    if max_unanchored is not None and len(unanchored) > max_unanchored:
        print(
            "active-support import audit failed: "
            f"{len(unanchored)} unanchored rows exceed baseline {max_unanchored}"
        )
        return 1

    if max_unanchored is None:
        print("active-support import audit completed in advisory mode")
    else:
        print("active-support import audit passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
