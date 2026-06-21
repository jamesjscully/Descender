#!/usr/bin/env python3
"""Audit non-active archive label counts and high-risk authority imports.

This prevents stale-path inventory from growing quietly during the pivot. It is
not a claim that the remaining legacy rows are bad; it is a ratchet around the
known archived/legacy/negative-control surface. It also prevents superseded
tracker/decision files from being re-imported by the current authority chain.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ARCHIVE = ROOT / "research/ARCHIVE_INDEX.md"

ARCHIVE_ROW = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+`([^`]+)`\s+\|")

LEGACY_LABELS = {
    "ARCHIVED_TARGET",
    "LEGACY_FORMAL",
    "MIXED_STATUS",
    "NEGATIVE_CONTROL",
    "SUPERSEDED_DECISION",
    "SUPERSEDED_TRACKER",
}

BLOCKED_AUTHORITY_IMPORT_LABELS = {
    "SUPERSEDED_DECISION",
    "SUPERSEDED_TRACKER",
}


def archive_rows() -> list[tuple[str, str]]:
    rows: list[tuple[str, str]] = []
    for line in ARCHIVE.read_text(encoding="utf-8").splitlines():
        match = ARCHIVE_ROW.match(line)
        if not match:
            continue
        target, label = match.groups()
        if target in {"File", "Package"}:
            continue
        rows.append((target, label))
    return rows


def archive_label_counts(rows: list[tuple[str, str]]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for _target, label in rows:
        counts[label] = counts.get(label, 0) + 1
    return counts


def authority_files() -> list[Path]:
    files = [
        ROOT / "README.md",
        ROOT / "TRACKER.md",
        ROOT / "decisions/README.md",
    ]
    files.extend(sorted((ROOT / "spec").glob("*.md")))
    files.extend(sorted((ROOT / "decisions").glob("ADR-*.md")))
    return [path for path in files if path.exists()]


def blocked_authority_imports(rows: list[tuple[str, str]]) -> list[tuple[str, str, str]]:
    blocked = [
        (target, label)
        for target, label in rows
        if label in BLOCKED_AUTHORITY_IMPORT_LABELS
    ]
    files = authority_files()
    imports: list[tuple[str, str, str]] = []
    for target, label in blocked:
        for path in files:
            if target in path.read_text(encoding="utf-8", errors="replace"):
                imports.append((target, label, path.relative_to(ROOT).as_posix()))
    return imports


def parse_label_limit(raw: str) -> tuple[str, int]:
    if "=" not in raw:
        raise argparse.ArgumentTypeError("expected LABEL=N")
    label, value = raw.split("=", 1)
    label = label.strip()
    if label not in LEGACY_LABELS:
        raise argparse.ArgumentTypeError(f"unknown legacy label {label!r}")
    try:
        limit = int(value)
    except ValueError as exc:
        raise argparse.ArgumentTypeError(f"invalid count in {raw!r}") from exc
    if limit < 0:
        raise argparse.ArgumentTypeError("count must be non-negative")
    return label, limit


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--max-non-active",
        type=int,
        required=True,
        help="fail if total non-active legacy rows exceed this count",
    )
    parser.add_argument(
        "--max",
        dest="label_limits",
        action="append",
        type=parse_label_limit,
        default=[],
        metavar="LABEL=N",
        help="per-label maximum for a non-active archive label",
    )
    parser.add_argument(
        "--list",
        action="store_true",
        help="print the non-active labels and counts",
    )
    args = parser.parse_args()

    rows = archive_rows()
    counts = archive_label_counts(rows)
    non_active_counts = {label: counts.get(label, 0) for label in sorted(LEGACY_LABELS)}
    total = sum(non_active_counts.values())
    blocked_imports = blocked_authority_imports(rows)

    print(f"non-active archive rows: {total}")
    for label, count in non_active_counts.items():
        print(f"{label}: {count}")
    print(f"blocked superseded authority imports: {len(blocked_imports)}")

    errors: list[str] = []
    if total > args.max_non_active:
        errors.append(
            f"non-active archive rows: {total} exceeds allowed maximum "
            f"{args.max_non_active}"
        )

    for label, limit in args.label_limits:
        count = non_active_counts[label]
        if count > limit:
            errors.append(f"{label}: {count} exceeds allowed maximum {limit}")

    for target, label, path in blocked_imports:
        errors.append(
            f"{path}: imports {label} archive row that must not be current "
            f"authority: {target}"
        )

    if args.list:
        for label in sorted(LEGACY_LABELS):
            print(f"{label} rows:")
            for line_no, line in enumerate(ARCHIVE.read_text(encoding="utf-8").splitlines(), start=1):
                match = ARCHIVE_ROW.match(line)
                if not match:
                    continue
                target, row_label = match.groups()
                if row_label == label and target not in {"File", "Package"}:
                    print(f"  {line_no}: {target}")

    if errors:
        print("archive legacy-count audit failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("archive legacy-count audit passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
