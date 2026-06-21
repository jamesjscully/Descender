#!/usr/bin/env python3
"""Check direct tool Cargo manifests for authority-safe framing."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

PACKAGE_NAME = re.compile(r'^name\s*=\s*"([^"]+)"', re.MULTILINE)
DESCRIPTION = re.compile(r'^description\s*=\s*"([^"]+)"', re.MULTILINE)

LEGACY_PACKAGE = "descender-production-reference"
LEGACY_REQUIRED_SNIPPETS = (
    "Legacy fail-closed",
    "not active product authority",
    "chain-native Path 2",
)

ACTIVE_BANNED_PATTERNS = (
    re.compile(r"\bproduction\b", re.IGNORECASE),
    re.compile(r"\bdeploy(?:ed|ment|able|ing)?\b", re.IGNORECASE),
    re.compile(r"\bKEM\b", re.IGNORECASE),
    re.compile(r"signature primitive", re.IGNORECASE),
)


def first_match(pattern: re.Pattern[str], text: str) -> str | None:
    match = pattern.search(text)
    return match.group(1) if match else None


def check_manifest(path: Path, errors: list[str]) -> None:
    rel = path.relative_to(ROOT).as_posix()
    text = path.read_text(encoding="utf-8", errors="replace")
    name = first_match(PACKAGE_NAME, text)
    description = first_match(DESCRIPTION, text)

    if not name:
        errors.append(f"{rel}: missing [package] name")
        return
    if not description:
        errors.append(f"{rel}: direct tool manifest must have a description")
        return

    if name == LEGACY_PACKAGE:
        for snippet in LEGACY_REQUIRED_SNIPPETS:
            if snippet not in description:
                errors.append(
                    f"{rel}: legacy manifest description missing {snippet!r}"
                )
        return

    for line_no, line in enumerate(text.splitlines(), start=1):
        for pattern in ACTIVE_BANNED_PATTERNS:
            if pattern.search(line):
                errors.append(
                    f"{rel}:{line_no}: active tool manifest has deployment/"
                    f"legacy product framing: {line.strip()}"
                )
                break


def main() -> int:
    errors: list[str] = []
    for path in sorted((ROOT / "tools").glob("*/Cargo.toml")):
        check_manifest(path, errors)

    if errors:
        print("tool manifest framing check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("tool manifest framing check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
