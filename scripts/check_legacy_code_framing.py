#!/usr/bin/env python3
"""Check that legacy deployment-shaped code/docs are framed as non-authoritative."""

from __future__ import annotations

import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

REQUIREMENTS = {
    "tools/parameter-search/src/production_reference.rs": (
        "MIGRATION NOTE: the PRODUCTION_DEPLOYMENT_* identifiers below are legacy",
        "fail-closed preflight/evidence names",
        "retained for regression compatibility and negative-control",
        "must not be read as current product authority",
        "chain-native authority starts at spec/README.md",
        "decisions/",
        "TRACKER.md",
    ),
    "tools/production-reference/src/lib.rs": (
        "not a deployment-ready cryptographic implementation",
        "legacy-selected 314-byte",
        "fail-closed negative-control surface",
        "chain-native authority lives in `spec/`",
        "ADRs",
        "`TRACKER.md`",
    ),
    "tools/production-reference/README.md": (
        "MIGRATION STATUS",
        "`LEGACY_FORMAL`",
        "not the active product specification",
        "Current",
        "authority starts at `spec/README.md`",
        "This is an intermediate split, not a deployment claim",
    ),
    "formal/spec/descender-signature-spec.md": (
        "MIGRATION STATUS",
        "`LEGACY_FORMAL`",
        "no longer the active product specification",
        "legacy/current-at-the-time post-AP2-004 constants",
        "legacy formal target, not active product",
        "Legacy Security Target",
        "not active product tasks unless re-imported by `spec/`, ADRs, or `TRACKER.md`",
    ),
}

FORBIDDEN_SNIPPETS = {
    "formal/spec/descender-signature-spec.md": (
        "deployed target",
        "deployed post-AP2-004 constants",
    ),
}


def main() -> int:
    errors: list[str] = []
    for rel, snippets in REQUIREMENTS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing")
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        for snippet in snippets:
            if snippet not in text:
                errors.append(f"{rel}: missing required legacy framing: {snippet!r}")
        for snippet in FORBIDDEN_SNIPPETS.get(rel, ()):
            if snippet in text:
                errors.append(f"{rel}: forbidden legacy-active phrasing remains: {snippet!r}")

    if errors:
        print("legacy code framing check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("legacy code framing check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
