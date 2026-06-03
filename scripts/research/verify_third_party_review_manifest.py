#!/usr/bin/env python3
"""Verify the hashed third-party review manifest."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_MANIFEST = (
    ROOT
    / "research/cryptanalysis/signature_readiness_20260516/"
    "third_party_review_manifest_n32_q12289.json"
)
EXPECTED_MANIFEST_SCHEMA = "Descender.third_party_review_manifest.v1"


def file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def local_path(path_text: str) -> Path:
    path = Path(path_text)
    if path.is_absolute():
        return path
    return ROOT / path


def verify_file(row: dict[str, Any]) -> list[dict[str, Any]]:
    failures = []
    path = local_path(str(row["path"]))
    expected_present = bool(row.get("present"))
    if not path.exists():
        if expected_present:
            failures.append({"check": "present", "expected": True, "actual": False})
        return failures
    if not expected_present:
        failures.append({"check": "present", "expected": False, "actual": True})
    expected_bytes = row.get("bytes")
    actual_bytes = path.stat().st_size
    if expected_bytes != actual_bytes:
        failures.append(
            {"check": "bytes", "expected": expected_bytes, "actual": actual_bytes}
        )
    expected_sha256 = row.get("sha256")
    actual_sha256 = file_sha256(path)
    if expected_sha256 != actual_sha256:
        failures.append(
            {
                "check": "sha256",
                "expected": expected_sha256,
                "actual": actual_sha256,
            }
        )
    if path.suffix == ".json":
        try:
            schema = load_json(path).get("schema")
        except json.JSONDecodeError as exc:
            failures.append({"check": "json", "error": str(exc)})
        else:
            expected_schema = row.get("schema")
            if expected_schema != schema:
                failures.append(
                    {
                        "check": "schema",
                        "expected": expected_schema,
                        "actual": schema,
                    }
                )
    return failures


def verify_manifest_shape(manifest: dict[str, Any]) -> list[dict[str, Any]]:
    failures = []
    schema = manifest.get("schema")
    if schema != EXPECTED_MANIFEST_SCHEMA:
        failures.append(
            {
                "check": "manifest_schema",
                "expected": EXPECTED_MANIFEST_SCHEMA,
                "actual": schema,
            }
        )

    files = manifest.get("files")
    if not isinstance(files, list):
        failures.append(
            {
                "check": "files_list",
                "expected": "list",
                "actual": type(files).__name__,
            }
        )
        return failures

    paths = [str(row.get("path")) for row in files if isinstance(row, dict)]
    path_counts = Counter(paths)
    duplicate_paths = sorted(
        path for path, count in path_counts.items() if count > 1
    )
    if duplicate_paths:
        failures.append({"check": "duplicate_paths", "actual": duplicate_paths})

    summary = manifest.get("summary", {})
    present_count = sum(
        1 for row in files if isinstance(row, dict) and row.get("present")
    )
    missing_count = len(files) - present_count
    expected_summary = {
        "files_indexed": len(files),
        "files_present": present_count,
        "files_missing": missing_count,
    }
    for key, expected in expected_summary.items():
        actual = summary.get(key)
        if actual != expected:
            failures.append(
                {"check": f"summary.{key}", "expected": expected, "actual": actual}
            )
    return failures


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    args = parser.parse_args()

    manifest = load_json(args.manifest)
    manifest_failures = verify_manifest_shape(manifest)
    rows = []
    for row in manifest.get("files", []):
        failures = verify_file(row)
        rows.append(
            {
                "path": row["path"],
                "role": row.get("role"),
                "status": "pass" if not failures else "fail",
                "failures": failures,
            }
        )
    failed = [row for row in rows if row["status"] != "pass"]
    output = {
        "schema": "Descender.third_party_review_manifest_verification.v1",
        "manifest": str(args.manifest),
        "manifest_schema": manifest.get("schema"),
        "manifest_failures": manifest_failures,
        "files_checked": len(rows),
        "failed_files": len(failed),
        "status": "pass" if not failed and not manifest_failures else "fail",
        "rows": rows,
    }
    print(json.dumps(output, indent=2, sort_keys=True))
    if failed or manifest_failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
