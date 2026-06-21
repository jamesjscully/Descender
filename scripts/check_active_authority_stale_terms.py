#!/usr/bin/env python3
"""Guard active authority docs against stale product-path language.

The active docs may mention legacy routes, but only as explicit history,
non-claims, or archive/process rules. New mentions of the old KEM,
projection-carrier, or 260/308/314-byte target lines should be deliberate and
reviewed instead of quietly becoming current product language again.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

STALE_TERMS = re.compile(
    r"(?i)(\bKEM\b|key encapsulation|projection[- ]carrier|"
    r"\b(?:260|308|314)-byte\b|deployable[_ -]signature target)"
)

FIXED_ACTIVE_FILES = {
    "README.md",
    "TRACKER.md",
    "research/README.md",
    "scripts/research/README.md",
    "tools/README.md",
}

ALLOWED_STALE_CONTEXT: dict[str, tuple[re.Pattern[str], ...]] = {
    "README.md": (
        re.compile(r"260-byte legacy linear-sigma / KAT v0 path"),
        re.compile(r"308/314-byte Fq12 projection-carrier envelope"),
        re.compile(r"old projection-carrier transcript"),
    ),
    "TRACKER.md": (
        re.compile(r"PROC-STALE-001"),
        re.compile(r"PROC-LEGACY-GATE-001"),
    ),
    "spec/README.md": (),
    "spec/process.md": (
        re.compile(r"Legacy KEM/descent tasks require"),
        re.compile(r"legacy q12289, KAT v0, projection-carrier"),
        re.compile(r"references to legacy KEM"),
        re.compile(r"projection-carrier, and 260/308/314-byte target lines"),
    ),
    "spec/wire_formats.md": (
        re.compile(r"314-byte `checked_core \|\| projection_carrier` envelope"),
        re.compile(r"308-byte projection-carrier labels"),
        re.compile(r"260-byte linear-sigma/KAT v0 path"),
    ),
    "research/README.md": (
        re.compile(r"Legacy KEM/formalization"),
        re.compile(r"old projection-carrier deployment tasks"),
        re.compile(r"older deployable-cryptography preparation package for the q12289"),
    ),
    "scripts/research/README.md": (
        re.compile(r"308-byte/Fq12 projection-carrier path"),
    ),
    "tools/README.md": (
        re.compile(r"Legacy fail-closed production-shaped baseline"),
    ),
}

SPEC_README_NON_ACTIVE_LINES = {
    "- the 260-byte legacy linear-sigma / KAT v0 path;",
    "- the 308/314-byte Fq12 projection-carrier envelope as the final product story;",
    "- spend-time hidden-witness extraction from the old projection-carrier transcript;",
}


def active_files() -> list[Path]:
    paths = [ROOT / rel for rel in sorted(FIXED_ACTIVE_FILES)]
    paths.extend(sorted((ROOT / "spec").glob("*.md")))
    return paths


def check_file(path: Path, errors: list[str]) -> None:
    rel = path.relative_to(ROOT).as_posix()
    allowed = ALLOWED_STALE_CONTEXT.get(rel, ())
    heading = ""
    for line_no, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        if line.startswith("## "):
            heading = line.strip()
        if not STALE_TERMS.search(line):
            continue
        if (
            rel == "spec/README.md"
            and heading == "## Current Non-Active Paths"
            and line.strip() in SPEC_README_NON_ACTIVE_LINES
        ):
            continue
        if any(pattern.search(line) for pattern in allowed):
            continue
        errors.append(f"{rel}:{line_no}: unreviewed stale-path reference: {line.strip()}")


def main() -> int:
    errors: list[str] = []
    for path in active_files():
        if path.exists():
            check_file(path, errors)

    if errors:
        print("active authority stale-term check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("active authority stale-term check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
