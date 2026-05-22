#!/usr/bin/env python3
"""Verify the closed quotient-level link research bundle.

This checker is intentionally read-only.  It verifies the promotion-gate
decision, artifact/script hashes, and the import firebreak audit that keeps the
sidecar Lean modules out of the deployable aggregate.
"""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
PROMOTION_GATE = READINESS_DIR / "quotient_level_link_promotion_gate.json"
FIREBREAK_AUDIT = READINESS_DIR / "quotient_level_link_firebreak_audit.json"


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def check_record(kind: str, record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    rel_path = record["path"]
    path = ROOT / rel_path
    if not path.exists():
        return [f"{kind} missing: {rel_path}"]
    actual_sha = sha256(path)
    actual_bytes = path.stat().st_size
    if record.get("sha256") != actual_sha:
        errors.append(
            f"{kind} hash mismatch for {rel_path}: "
            f"recorded {record.get('sha256')} actual {actual_sha}"
        )
    if record.get("bytes") != actual_bytes:
        errors.append(
            f"{kind} byte-size mismatch for {rel_path}: "
            f"recorded {record.get('bytes')} actual {actual_bytes}"
        )
    if record.get("exists") is not True:
        errors.append(f"{kind} record does not mark existing file: {rel_path}")
    return errors


def verify() -> dict[str, Any]:
    gate = load_json(PROMOTION_GATE)
    firebreak = load_json(FIREBREAK_AUDIT)
    errors: list[str] = []

    decision = gate.get("final_decision", {})
    if decision.get("classification") != "no_replacement":
        errors.append("promotion gate classification is not no_replacement")
    if decision.get("decision") != "keep_current_path":
        errors.append("promotion gate decision is not keep_current_path")
    if decision.get("migration_plan_required") is not False:
        errors.append("promotion gate unexpectedly requires a migration plan")
    if gate.get("missing_records") != []:
        errors.append(f"promotion gate has missing records: {gate.get('missing_records')}")

    if firebreak.get("passed") is not True:
        errors.append("firebreak audit did not pass")
    if firebreak.get("violations") != []:
        errors.append(f"firebreak audit has violations: {firebreak.get('violations')}")
    if firebreak.get("root_sidecar_imports") != []:
        errors.append(
            f"root aggregate imports sidecar modules: {firebreak.get('root_sidecar_imports')}"
        )

    for record in gate.get("artifact_hashes", []):
        errors.extend(check_record("artifact", record))
    for record in gate.get("script_hashes", []):
        errors.extend(check_record("script", record))

    return {
        "status": "verified" if not errors else "failed",
        "errors": errors,
        "artifact_records_checked": len(gate.get("artifact_hashes", [])),
        "script_records_checked": len(gate.get("script_hashes", [])),
        "promotion_gate": str(PROMOTION_GATE.relative_to(ROOT)),
        "firebreak_audit": str(FIREBREAK_AUDIT.relative_to(ROOT)),
    }


def main() -> None:
    result = verify()
    print(json.dumps(result, indent=2, sort_keys=True))
    if result["errors"]:
        sys.exit(1)


if __name__ == "__main__":
    main()
