#!/usr/bin/env python3
"""Verify the reviewer-facing status index against its source artifacts."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_STATUS_INDEX = READINESS_DIR / "third_party_review_status_index_n32_q12289.json"
EXPECTED_SCHEMA = "Descender.third_party_review_status_index.v1"

sys.path.insert(0, str(ROOT / "scripts/research"))
import third_party_review_status_index as status_index  # noqa: E402


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def stable_payload(payload: dict[str, Any]) -> dict[str, Any]:
    stable = dict(payload)
    stable.pop("generated_at", None)
    return stable


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--status-index", type=Path, default=DEFAULT_STATUS_INDEX)
    args = parser.parse_args()

    actual = load_json(args.status_index)
    expected = status_index.build_payload()
    failures = []

    if actual.get("schema") != EXPECTED_SCHEMA:
        failures.append(
            {
                "check": "schema",
                "expected": EXPECTED_SCHEMA,
                "actual": actual.get("schema"),
            }
        )
    if stable_payload(actual) != stable_payload(expected):
        failures.append(
            {
                "check": "stable_payload",
                "expected": stable_payload(expected),
                "actual": stable_payload(actual),
            }
        )

    output = {
        "schema": "Descender.third_party_review_status_index_verification.v1",
        "status_index": str(args.status_index),
        "status_index_schema": actual.get("schema"),
        "failures": failures,
        "status": "pass" if not failures else "fail",
    }
    print(json.dumps(output, indent=2, sort_keys=True))
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
