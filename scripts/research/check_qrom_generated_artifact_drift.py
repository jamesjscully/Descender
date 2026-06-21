#!/usr/bin/env python3
"""Fail if regenerated active QROM artifacts changed committed outputs."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = Path("research/cryptanalysis/signature_readiness_20260516")

GENERATED_ARTIFACTS = (
    READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.json",
    READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.md",
    READINESS_DIR / "qrom_security_loss_review_n32_q12289.json",
    READINESS_DIR / "qrom_security_loss_review_n32_q12289.md",
    READINESS_DIR / "pre_third_party_qrom_frame_probe_n32_q12289.json",
    READINESS_DIR / "pre_third_party_qrom_frame_probe_n32_q12289.md",
    READINESS_DIR / "qrom_multitarget_margin_n32_q12289.json",
    READINESS_DIR / "qrom_multitarget_margin_n32_q12289.md",
)


def main() -> int:
    missing = [path.as_posix() for path in GENERATED_ARTIFACTS if not (ROOT / path).exists()]
    if missing:
        print("QROM generated artifact drift check failed: missing artifacts", file=sys.stderr)
        for path in missing:
            print(f"  - {path}", file=sys.stderr)
        return 1

    result = subprocess.run(
        ["git", "diff", "--exit-code", "--", *[path.as_posix() for path in GENERATED_ARTIFACTS]],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    if result.returncode != 0:
        print("QROM generated artifact drift check failed:", file=sys.stderr)
        print(result.stdout, file=sys.stderr, end="")
        return result.returncode

    print("QROM generated artifact drift check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
