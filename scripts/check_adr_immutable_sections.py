#!/usr/bin/env python3
"""Check review-visible hashes for immutable ADR sections.

Accepted, superseded, and rejected ADRs are historical decisions. Their
Context, Decision, and Consequences sections should not be rewritten in place;
changes to those sections should be made by adding a new ADR and updating the
old ADR's lifecycle fields. This gate cannot replace code review, but it makes
any rewrite of those sections show up as an explicit manifest change.
"""

from __future__ import annotations

import argparse
import hashlib
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "decisions" / "ADR_IMMUTABLE_SECTIONS.sha256"
ADR_FILE = re.compile(r"^ADR-(\d{4})-[a-z0-9][a-z0-9-]*\.md$")
IMMUTABLE_SECTIONS = ("## Context", "## Decision", "## Consequences")
FINAL_STATUSES = {"Accepted", "Superseded", "Rejected"}


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


def adr_status(text: str) -> str:
    match = re.search(r"^Status:\s+(.+)$", text, re.MULTILINE)
    return match.group(1).strip() if match else ""


def immutable_payload(path: Path) -> str:
    text = path.read_text(encoding="utf-8")
    parts: list[str] = []
    for heading in IMMUTABLE_SECTIONS:
        section = markdown_section(text, heading)
        if not section or not any(line.strip() for line in section):
            raise ValueError(f"{path.relative_to(ROOT)}: missing non-empty {heading}")
        parts.append(heading)
        parts.extend(section)
    return "\n".join(line.rstrip() for line in parts).rstrip() + "\n"


def current_hashes() -> dict[str, str]:
    entries: dict[str, str] = {}
    for path in sorted((ROOT / "decisions").glob("ADR-*.md")):
        if not ADR_FILE.match(path.name):
            continue
        text = path.read_text(encoding="utf-8")
        if adr_status(text) not in FINAL_STATUSES:
            continue
        rel = path.relative_to(ROOT).as_posix()
        digest = hashlib.sha256(immutable_payload(path).encode("utf-8")).hexdigest()
        entries[rel] = digest
    return entries


def read_manifest() -> dict[str, str]:
    if not MANIFEST.exists():
        raise FileNotFoundError("decisions/ADR_IMMUTABLE_SECTIONS.sha256: missing")

    entries: dict[str, str] = {}
    previous_rel = ""
    for line_no, line in enumerate(MANIFEST.read_text(encoding="utf-8").splitlines(), start=1):
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        parts = stripped.split()
        if len(parts) != 2:
            raise ValueError(f"{MANIFEST.relative_to(ROOT)}:{line_no}: expected '<sha256> <path>'")
        digest, rel = parts
        if not re.fullmatch(r"[0-9a-f]{64}", digest):
            raise ValueError(f"{MANIFEST.relative_to(ROOT)}:{line_no}: invalid sha256 digest")
        if not rel.startswith("decisions/ADR-") or not rel.endswith(".md"):
            raise ValueError(f"{MANIFEST.relative_to(ROOT)}:{line_no}: invalid ADR path {rel!r}")
        if rel in entries:
            raise ValueError(f"{MANIFEST.relative_to(ROOT)}:{line_no}: duplicate ADR path {rel}")
        if rel <= previous_rel:
            raise ValueError(f"{MANIFEST.relative_to(ROOT)}:{line_no}: manifest entries must be sorted by path")
        previous_rel = rel
        entries[rel] = digest
    return entries


def print_current() -> None:
    print("# Descender ADR immutable-section hashes")
    print("# Format: <sha256> <path>")
    print("# Covers final ADR Context, Decision, and Consequences sections.")
    for rel, digest in current_hashes().items():
        print(f"{digest} {rel}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--print-current",
        action="store_true",
        help="print the current manifest contents and exit",
    )
    args = parser.parse_args()

    if args.print_current:
        print_current()
        return 0

    errors: list[str] = []
    try:
        expected = current_hashes()
        actual = read_manifest()
    except (OSError, ValueError) as exc:
        print(f"ADR immutable-section check failed: {exc}", file=sys.stderr)
        return 1

    missing = sorted(set(expected) - set(actual))
    extra = sorted(set(actual) - set(expected))
    for rel in missing:
        errors.append(f"manifest missing final ADR entry: {rel}")
    for rel in extra:
        errors.append(f"manifest has entry for non-final or missing ADR: {rel}")
    for rel in sorted(expected.keys() & actual.keys()):
        if expected[rel] != actual[rel]:
            errors.append(
                f"{rel}: immutable section hash changed; add a new ADR or update "
                "the manifest with an explicit review note"
            )

    if errors:
        print("ADR immutable-section check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print(f"ADR immutable-section check passed ({len(expected)} final ADRs)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
