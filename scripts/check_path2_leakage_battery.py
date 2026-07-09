#!/usr/bin/env python3
"""Validate the Path 2 leakage battery manifest and first public-artifact scan.

This is a fail-closed boundary gate. It does not claim signer leakage readiness;
it pins the required leakage surfaces from spec/leakage_profile.md and checks
that accepted public evidence artifacts keep deterministic proof seeds labeled
as test-only while avoiding exact forbidden secret-material fields.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
MANIFEST_PATH = ROOT / "formal/kat/path2_leakage_battery_v1.json"
SPEC_PATH = ROOT / "spec/leakage_profile.md"
TRACKER_PATH = ROOT / "TRACKER.md"

EXPECTED_SURFACES = (
    "keygen",
    "sampler_rejection_loop",
    "pad_expansion_mask_prf",
    "witness_handling",
    "proof_generation",
    "spend_authorization",
    "signer_api",
    "storage_rollback",
)

SPEC_SURFACE_LABELS = {
    "keygen": "keygen",
    "sampler_rejection_loop": "sampler/rejection loop",
    "pad_expansion_mask_prf": "pad expansion and mask PRF",
    "witness_handling": "witness handling",
    "proof_generation": "proof generation",
    "spend_authorization": "spend authorization",
    "signer_api": "signer API",
    "storage_rollback": "storage and rollback",
}

REQUIRED_SPEC_PHRASES = (
    "Leakage Harness Requirements",
    "Fixed-Shape Failure Requirements",
    "Serialization And Logging Negative Tests",
    "Target And Build Profiles",
    "Before the constant-time/leakage surface can be marked `verified`",
)

TEST_ONLY_POLICY_PHRASES = (
    "deterministic KAT evidence only",
    "deployment producers still use fresh proof randomness",
)


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text())
    except Exception as exc:  # pragma: no cover - surfaced as gate text
        raise SystemExit(f"failed to read JSON {path}: {exc}") from exc


def pointer(value: Any, json_pointer: str) -> Any:
    if not json_pointer.startswith("/"):
        raise ValueError(f"invalid JSON pointer {json_pointer!r}")
    current = value
    for raw_part in json_pointer.split("/")[1:]:
        part = raw_part.replace("~1", "/").replace("~0", "~")
        if isinstance(current, dict):
            if part not in current:
                raise KeyError(json_pointer)
            current = current[part]
        elif isinstance(current, list):
            current = current[int(part)]
        else:
            raise KeyError(json_pointer)
    return current


def walk_keys(value: Any) -> set[str]:
    keys: set[str] = set()
    if isinstance(value, dict):
        for key, child in value.items():
            keys.add(key)
            keys.update(walk_keys(child))
    elif isinstance(value, list):
        for child in value:
            keys.update(walk_keys(child))
    return keys


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def main() -> None:
    manifest_doc = load_json(MANIFEST_PATH)
    manifest = manifest_doc.get("manifest")
    require(isinstance(manifest, dict), "leakage battery manifest missing /manifest object")

    require(
        manifest.get("schema") == "Descender.Path2.LeakageBatteryManifest.v1",
        "leakage battery schema mismatch",
    )
    require(
        manifest.get("frozen_status") == "draft_fail_closed_leakage_battery",
        "leakage battery frozen_status must remain fail-closed",
    )
    require(manifest.get("deployment_ready") is False, "leakage battery must not be deployment-ready")

    spec = SPEC_PATH.read_text()
    tracker = TRACKER_PATH.read_text()
    for phrase in REQUIRED_SPEC_PHRASES:
        require(phrase in spec, f"spec/leakage_profile.md missing required phrase {phrase!r}")
    require(
        "formal/kat/path2_leakage_battery_v1.json" in tracker,
        "TRACKER.md must cite the leakage battery manifest",
    )
    require(
        "scripts/check_path2_leakage_battery.py" in tracker,
        "TRACKER.md must cite the leakage battery checker",
    )

    authority = manifest.get("authority", {})
    require(authority.get("spec") == "spec/leakage_profile.md", "leakage manifest spec authority mismatch")
    require(
        authority.get("check") == "scripts/check_path2_leakage_battery.py",
        "leakage manifest checker authority mismatch",
    )
    require(
        authority.get("tracker_rows") == ["SPEC-LEAKAGE-001", "DIR-CT-001"],
        "leakage manifest tracker ownership mismatch",
    )

    target_profile = manifest.get("target_profile", {})
    require(target_profile.get("deployment_ready") is False, "target profile must remain not ready")
    require(
        "no hardware wallet" in target_profile.get("resource_profile", ""),
        "target profile must explicitly avoid hardware/custody claims",
    )

    surfaces = manifest.get("required_surfaces")
    require(isinstance(surfaces, list), "required_surfaces must be a list")
    surface_ids = [row.get("id") for row in surfaces if isinstance(row, dict)]
    require(tuple(surface_ids) == EXPECTED_SURFACES, "leakage required surface order/count drifted")
    for row in surfaces:
        row_id = row["id"]
        expected_label = SPEC_SURFACE_LABELS[row_id]
        require(row.get("spec_surface") == expected_label, f"{row_id}: spec surface label mismatch")
        require(expected_label in spec, f"{row_id}: spec surface label not found in leakage profile")
        require(row.get("deployment_ready") is False, f"{row_id}: deployment_ready must be false")
        require(row.get("status") in {"not_ready", "partial"}, f"{row_id}: invalid status")
        require(row.get("blocker"), f"{row_id}: blocker must be explicit")
    require(
        any(row.get("status") == "not_ready" for row in surfaces),
        "leakage battery must remain fail-closed until real target evidence exists",
    )

    forbidden = set(manifest.get("forbidden_exact_public_artifact_fields", []))
    require(forbidden, "forbidden exact public artifact field list is empty")
    require(
        manifest.get("test_only_deterministic_seed_field") == "proof_mask_seed",
        "test-only deterministic seed field drifted",
    )

    artifact_rows = manifest.get("public_artifact_negative_tests")
    require(isinstance(artifact_rows, list) and artifact_rows, "missing public artifact negative tests")
    for row in artifact_rows:
        artifact_path = ROOT / row["path"]
        artifact = load_json(artifact_path)
        policy = pointer(artifact, row["proof_byte_policy_pointer"])
        require(isinstance(policy, str), f"{row['id']}: proof byte policy is not a string")
        for phrase in TEST_ONLY_POLICY_PHRASES:
            require(phrase in policy, f"{row['id']}: proof byte policy missing {phrase!r}")

        seed = pointer(artifact, row["deterministic_seed_pointer"])
        require(isinstance(seed, int), f"{row['id']}: deterministic proof seed must be an integer")

        payload_pointer = row.get("public_payload_exports_private_material_pointer")
        if payload_pointer is not None:
            payload_exports = pointer(artifact, payload_pointer)
            require(
                payload_exports is False,
                f"{row['id']}: public payload exports private material",
            )

        keys = walk_keys(artifact)
        leaked = sorted(forbidden & keys)
        require(not leaked, f"{row['id']}: public artifact has forbidden exact fields {leaked}")

    print(
        "Path 2 leakage battery manifest check passed "
        f"({len(surfaces)} surfaces, {len(artifact_rows)} public artifact scans)."
    )


if __name__ == "__main__":
    main()
