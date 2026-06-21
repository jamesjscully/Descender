#!/usr/bin/env python3
"""Check source entrypoint docs for current-authority framing."""

from __future__ import annotations

import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

# These are source entrypoints, not every internal implementation file. The
# invariant is that top-of-file docs for active code do not advertise themselves
# as product authority or deployment-ready production artifacts.
REQUIREMENTS = {
    "tools/descender-account-stark/src/lib.rs": (
        "Path 2 account-creation ZK-STARK implementation artifact",
        "Product authority still starts in `spec/`, ADRs, and",
        "`TRACKER.md`; this crate is implementation evidence",
        "Signer-side",
    ),
    "tools/descender-account-stark/src/config.rs": (
        "Pinned Path 2 configuration",
        "active implementation config",
    ),
    "tools/descender-mask-prf/src/lib.rs": (
        "Signer-side no-Plonky3 projection-mask PRF for Path 2",
        "signer-side `tools/parameter-search` default builds",
    ),
    "tools/descender-account-verifier/src/lib.rs": (
        "Path 2 account-creation verifier transport",
        "account-admission verification path",
        "Signer-side `descender-parameter-search`",
    ),
    "tools/stark-field-bench/src/prod_bench.rs": (
        "Historical `prod_bench` research benchmark",
        "benchmark evidence, not product",
        "current authority starts in `spec/`, ADRs, and `TRACKER.md`",
    ),
}

PROHIBITED_HEAD_SNIPPETS = {
    "tools/descender-account-stark/src/lib.rs": (
        "Production account-creation",
        "deployable artifact",
        "deployed `tools/parameter-search` signer/verifier",
    ),
    "tools/descender-account-stark/src/config.rs": (
        "PINNED PRODUCTION configuration",
        "deployable config",
    ),
    "tools/descender-mask-prf/src/lib.rs": (
        "DEPLOYED signer",
        "deployed signer (`tools/parameter-search`)",
    ),
    "tools/descender-account-verifier/src/lib.rs": (
        "Deployment-facing account-creation verifier",
        "deployed account-admission path",
        "deployed signer crate",
    ),
    "tools/stark-field-bench/src/prod_bench.rs": (
        "PRODUCTION (deployable)",
    ),
}


def head(path: Path, lines: int = 32) -> str:
    return "\n".join(path.read_text(encoding="utf-8", errors="replace").splitlines()[:lines])


def main() -> int:
    errors: list[str] = []
    for rel, snippets in REQUIREMENTS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing")
            continue
        text = head(path)
        for snippet in snippets:
            if snippet not in text:
                errors.append(f"{rel}: missing current-source framing: {snippet!r}")
        for snippet in PROHIBITED_HEAD_SNIPPETS.get(rel, ()):
            if snippet in text:
                errors.append(f"{rel}: stale source-entrypoint framing remains: {snippet!r}")

    if errors:
        print("active source framing check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("active source framing check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
