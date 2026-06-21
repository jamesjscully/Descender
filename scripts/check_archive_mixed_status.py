#!/usr/bin/env python3
"""Audit MIXED_STATUS archive rows.

MIXED_STATUS is an intentionally temporary archive label: the file still has
useful evidence, but it also carries stale or potentially confusing authority
framing. This checker makes that cleanup backlog visible and ratchetable.
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ARCHIVE = ROOT / "research/ARCHIVE_INDEX.md"

ARCHIVE_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+`([^`]+)`\s+\|\s+(.+?)\s+\|")
ROUTING_LANGUAGE = re.compile(
    r"\b("
    r"archived|caveat|correction|current|history|historical|legacy|not|"
    r"older|retained|split|superseded|useful"
    r")\b",
    re.I,
)


@dataclass(frozen=True)
class MixedRow:
    line_no: int
    target: str
    interpretation: str


def mixed_rows() -> list[MixedRow]:
    rows: list[MixedRow] = []
    for line_no, line in enumerate(ARCHIVE.read_text(encoding="utf-8").splitlines(), start=1):
        match = ARCHIVE_ROW.match(line)
        if not match:
            continue
        target, label, interpretation = match.groups()
        if target in {"File", "Package"}:
            continue
        if label == "MIXED_STATUS":
            rows.append(MixedRow(line_no, target, interpretation.strip()))
    return rows


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--max-mixed",
        type=int,
        default=None,
        help="fail if more than this many MIXED_STATUS rows remain",
    )
    parser.add_argument(
        "--list",
        action="store_true",
        help="print each MIXED_STATUS row",
    )
    args = parser.parse_args()

    rows = mixed_rows()
    errors: list[str] = []

    for row in rows:
        if not ROUTING_LANGUAGE.search(row.interpretation):
            errors.append(
                f"research/ARCHIVE_INDEX.md:{row.line_no}: MIXED_STATUS row lacks "
                f"routing/cleanup language: {row.target}"
            )

    if args.max_mixed is not None and len(rows) > args.max_mixed:
        errors.append(
            f"MIXED_STATUS rows: {len(rows)} exceeds allowed maximum {args.max_mixed}"
        )

    print(f"MIXED_STATUS rows: {len(rows)}")
    if args.list:
        for row in rows:
            print(f"{row.target} :: {row.interpretation}")

    if errors:
        print("archive mixed-status audit failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    if args.max_mixed is None:
        print("archive mixed-status audit completed in advisory mode")
    else:
        print("archive mixed-status audit passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
