#!/usr/bin/env python3
"""Validate the fail-closed Path 2 review package manifest."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
MANIFEST_PATH = ROOT / "formal/kat/path2_review_package_v1.json"
SPEC_PATH = ROOT / "spec/review_package.md"
TRACKER_PATH = ROOT / "TRACKER.md"
LOCAL_GATE = ROOT / "scripts/check_build.sh"
CI_GATE = ROOT / ".github/workflows/ci.yml"

EXPECTED_LANES = (
    "cryptographic_core",
    "proof_system",
    "qrom_hash_framing",
    "consensus_engineering",
    "lifecycle_deployment",
    "signer_custody",
    "aggregation_policy",
)
INCLUDED_LANES = set(EXPECTED_LANES[:-1])
REQUIRED_RESIDUALS = {
    "external_review_not_complete",
    "coded_sis_beyond_box_response_link",
    "fri_aok_extractor",
    "account_stark_transcript_qrom",
    "simulator_no_export_theorem",
    "active_kat_corpus_freeze",
    "target_leakage_and_fixed_shape",
    "deployment_profile_acceptance",
}
REQUIRED_ARTIFACTS = {
    "spec/README.md",
    "spec/security_model.md",
    "spec/review_package.md",
    "spec/blockchain_requirements.md",
    "TRACKER.md",
    "formal/kat/path2_direct_theorem_boundary_v1.json",
    "formal/kat/path2_account_creation_proof_boundary_v1.json",
    "formal/kat/path2_account_stark_fri_qrom_residual_v1.json",
    "formal/kat/path2_account_stark_fri_aok_boundary_v1.json",
    "formal/kat/path2_direct_accepted_spend_tuple_evidence_v1.json",
    "formal/kat/path2_direct_accepted_composition_evidence_v1.json",
    "formal/kat/path2_leakage_battery_v1.json",
}
REQUIRED_COMMANDS = {
    "python3 scripts/check_path2_review_package.py",
    "bash scripts/check_build.sh",
    "python3 scripts/check_path2_leakage_battery.py",
    "cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_theorem_boundary_manifest",
    "cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_direct_theorem_obligation_vector_consistency",
    "cargo run --quiet --manifest-path tools/parameter-search/Cargo.toml --bin path2_preflight_boundary_consistency",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:  # pragma: no cover - gate error text
        raise SystemExit(f"failed to read JSON {path}: {exc}") from exc


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def require_hex_digest(value: Any, label: str) -> None:
    require(isinstance(value, str), f"{label}: digest is not a string")
    require(len(value) == 64, f"{label}: digest must be 64 lowercase hex characters")
    require(all(ch in "0123456789abcdef" for ch in value), f"{label}: digest must be lowercase hex")


def main() -> int:
    doc = load_json(MANIFEST_PATH)
    manifest = doc.get("manifest")
    require(isinstance(manifest, dict), "review package manifest missing /manifest object")
    require(
        manifest.get("schema") == "Descender.Path2.ReviewPackageManifest.v1",
        "review package schema mismatch",
    )
    require(
        manifest.get("frozen_status") == "draft_fail_closed_review_candidate",
        "review package must remain a fail-closed draft",
    )
    require(manifest.get("review_complete") is False, "review package must not mark review complete")
    require(manifest.get("deployment_ready") is False, "review package must not be deployment-ready")

    authority = manifest.get("authority")
    require(isinstance(authority, dict), "review package authority object missing")
    require(authority.get("spec") == "spec/review_package.md", "review package spec authority mismatch")
    require(
        authority.get("check") == "scripts/check_path2_review_package.py",
        "review package checker authority mismatch",
    )
    require(
        authority.get("tracker_rows") == ["SPEC-REVIEW-001", "DIR-REVIEW-001", "DIR-SEC-001"],
        "review package tracker owner list mismatch",
    )

    spec = SPEC_PATH.read_text(encoding="utf-8")
    tracker = TRACKER_PATH.read_text(encoding="utf-8")
    local_gate = LOCAL_GATE.read_text(encoding="utf-8")
    ci_gate = CI_GATE.read_text(encoding="utf-8")
    for phrase in (
        "current `spec/` authority snapshot and ADR index",
        "current review package manifest and verifier",
        "formal/kat/path2_review_package_v1.json",
        "scripts/check_path2_review_package.py",
    ):
        require(phrase in spec or phrase in tracker, f"missing review package authority phrase: {phrase}")
    require("formal/kat/path2_review_package_v1.json" in tracker, "TRACKER.md must cite review package manifest")
    require("scripts/check_path2_review_package.py" in tracker, "TRACKER.md must cite review package checker")
    require(
        "python3 scripts/check_path2_review_package.py" in local_gate,
        "local build gate must run review package checker",
    )
    require(
        "python3 scripts/check_path2_review_package.py" in ci_gate,
        "CI must run review package checker",
    )

    snapshot = manifest.get("authority_snapshot")
    require(isinstance(snapshot, dict), "authority_snapshot must be an object")
    require(snapshot.get("scope") == "direct_mode_q128_review_candidate", "authority snapshot scope drifted")
    for path in snapshot.get("authority_files", []):
        require((ROOT / path).exists(), f"authority snapshot path is missing: {path}")
    for path in snapshot.get("boundary_artifacts", []):
        require((ROOT / path).exists(), f"boundary artifact path is missing: {path}")

    claim_scope = manifest.get("claim_scope")
    require(isinstance(claim_scope, dict), "claim_scope must be an object")
    included_scope = set(claim_scope.get("included", []))
    excluded_scope = set(claim_scope.get("excluded", []))
    require("direct_mode_theorem_boundary" in included_scope, "direct theorem boundary must be in scope")
    require("deployment_ready_claim" in excluded_scope, "deployment-ready claim must be excluded")
    require("mode_b_aggregation_readiness" in excluded_scope, "Mode B readiness must be excluded")
    require("hardware_wallet_or_hsm_readiness" in excluded_scope, "hardware/HSM readiness must be excluded")

    artifacts = manifest.get("artifact_hashes")
    require(isinstance(artifacts, list) and artifacts, "artifact_hashes must be a non-empty list")
    artifact_paths = [row.get("path") for row in artifacts if isinstance(row, dict)]
    require(set(artifact_paths) == REQUIRED_ARTIFACTS, "review package artifact set drifted")
    require(len(artifact_paths) == len(set(artifact_paths)), "review package artifact paths must be unique")
    for row in artifacts:
        path = row.get("path")
        require(isinstance(path, str), "artifact path must be a string")
        expected = row.get("sha256")
        require_hex_digest(expected, path)
        actual = sha256_file(ROOT / path)
        require(actual == expected, f"{path}: sha256 mismatch {actual} != {expected}")

    commands = manifest.get("reproduction_commands")
    require(isinstance(commands, list) and commands, "reproduction_commands must be non-empty")
    command_set = {row.get("command") for row in commands if isinstance(row, dict)}
    require(command_set == REQUIRED_COMMANDS, "review package reproduction command set drifted")

    lanes = manifest.get("review_lanes")
    require(isinstance(lanes, list), "review_lanes must be a list")
    lane_ids = [row.get("id") for row in lanes if isinstance(row, dict)]
    require(tuple(lane_ids) == EXPECTED_LANES, "review lane order/count drifted")
    for row in lanes:
        lane_id = row["id"]
        require(row.get("review_complete") is False, f"{lane_id}: review_complete must be false")
        require(isinstance(row.get("required_focus"), str) and row["required_focus"], f"{lane_id}: focus missing")
        if lane_id in INCLUDED_LANES:
            require(row.get("included") is True, f"{lane_id}: lane must be included")
        else:
            require(row.get("included") is False, f"{lane_id}: aggregation/policy lane must be omitted")

    residuals = manifest.get("residual_assumptions")
    require(isinstance(residuals, list), "residual_assumptions must be a list")
    residual_ids = {row.get("id") for row in residuals if isinstance(row, dict)}
    require(residual_ids == REQUIRED_RESIDUALS, "review residual set drifted")
    for row in residuals:
        residual_id = row["id"]
        require(row.get("status") == "open", f"{residual_id}: residual must remain open")
        tracker_rows = row.get("tracker_rows")
        require(isinstance(tracker_rows, list) and tracker_rows, f"{residual_id}: tracker rows missing")
        for tracker_row in tracker_rows:
            require(f"| {tracker_row} |" in tracker, f"{residual_id}: missing tracker row {tracker_row}")
        require(
            isinstance(row.get("required_closure"), str) and row["required_closure"],
            f"{residual_id}: required closure missing",
        )

    outputs = manifest.get("reviewer_outputs")
    require(isinstance(outputs, list) and outputs, "reviewer_outputs must be non-empty")
    output_ids = {row.get("id") for row in outputs if isinstance(row, dict)}
    require(
        output_ids == {"lane_report", "reproduction_log", "objection_disposition"},
        "reviewer output set drifted",
    )
    for row in outputs:
        require(row.get("required") is True, f"{row.get('id')}: reviewer output must be required")
        require(isinstance(row.get("accepted_statuses"), list) and row["accepted_statuses"], f"{row.get('id')}: statuses missing")

    require(
        "evidence only unless imported by current spec/ADR/TRACKER authority"
        in manifest.get("old_research_import_policy", ""),
        "old research import policy must remain fail-closed",
    )

    print(
        "Path 2 review package manifest check passed "
        f"({len(artifacts)} artifacts, {len(lanes)} lanes, {len(residuals)} residuals)."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
