#!/usr/bin/env python3
"""Validate the Path 2 leakage battery manifest and public leakage scans.

This is a fail-closed boundary gate. It does not claim signer leakage readiness;
it pins the required leakage surfaces from spec/leakage_profile.md, checks that
accepted public evidence artifacts keep deterministic proof seeds labeled as
test-only while avoiding exact forbidden secret-material fields, and verifies
that manual empirical harness metadata stays wired to its reproducibility source.
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

FORBIDDEN_OBSERVED_ERROR_SUBSTRINGS = (
    "secret",
    "private key",
    "witness bytes",
    "raw dependency",
    "callback diagnostic",
)

EXPECTED_FIXED_SHAPE_FUNCTIONS = (
    "path2_direct_single_slot_spend_rejections_preserve_consume_once_state",
    "path2_public_result_class_v1_names_match_consensus_spec",
    "path2_direct_single_slot_spend_malformed_signature_kats_are_panic_free",
    "path2_direct_single_slot_spend_public_object_rejection_kats_are_panic_free",
    "path2_direct_single_slot_spend_public_result_wrapper_hides_internal_diagnostics",
)

EXPECTED_CALLBACK_FIXED_SHAPE_FUNCTIONS = (
    "sanitized_callback_wrappers_do_not_echo_secret_errors",
    "sanitized_callback_wrappers_have_bounded_invocation_shape",
    "callback_boundary_classification_marks_only_sanitized_paths_deployment_facing",
    "deployment_callback_boundary_gate_rejects_diagnostic_paths",
    "default_deployment_callback_profile_is_complete_and_sanitized",
)

EXPECTED_STORAGE_FIXED_SHAPE_FUNCTIONS = (
    "chain_state_file_stores_lifecycle_save_failure_is_fail_closed_after_ledger_persist",
)

EXPECTED_PUBLIC_RESULT_CLASSES = (
    "accept_admit_slot",
    "accept_spend_slot",
    "reject_malformed_public_input",
    "reject_unsupported_version",
    "reject_binding_mismatch",
    "reject_creation_evidence",
    "reject_checked_core",
    "reject_lifecycle_state",
    "reject_policy",
    "reject_resource_limit",
    "internal_error_fail_closed",
)


def require_hex_digest(value: Any, label: str) -> None:
    require(isinstance(value, str), f"{label}: digest is not a string")
    require(len(value) == 64, f"{label}: digest must be 64 lowercase hex characters")
    require(
        all(ch in "0123456789abcdef" for ch in value),
        f"{label}: digest must be lowercase hex",
    )
    require(value != "0" * 64, f"{label}: digest must be nonzero")


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


def require_public_value(value: Any, label: str) -> None:
    if isinstance(value, bool):
        return
    if isinstance(value, int):
        return
    if isinstance(value, str):
        require(value != "", f"{label}: public transcript field is empty")
        return
    if isinstance(value, list):
        require(bool(value), f"{label}: public transcript list is empty")
        return
    if isinstance(value, dict):
        require(bool(value), f"{label}: public transcript object is empty")
        return
    raise SystemExit(f"{label}: unsupported public transcript value type {type(value).__name__}")


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

    accepted_shape_rows = manifest.get("accepted_transcript_shape_tests")
    require(
        isinstance(accepted_shape_rows, list) and accepted_shape_rows,
        "missing accepted transcript shape tests",
    )
    for row in accepted_shape_rows:
        artifact = load_json(ROOT / row["path"])
        evidence = pointer(artifact, row["evidence_pointer"])
        require(isinstance(evidence, dict), f"{row['id']}: evidence pointer is not an object")
        for field in row["required_public_fields"]:
            require(field in evidence, f"{row['id']}: missing public transcript field {field}")
            require_public_value(evidence[field], f"{row['id']}/{field}")
        for field in row["required_true_fields"]:
            require(evidence.get(field) is True, f"{row['id']}: expected {field} = true")
        for field in row["required_false_fields"]:
            require(evidence.get(field) is False, f"{row['id']}: expected {field} = false")
        leaked = sorted(forbidden & walk_keys(evidence))
        require(not leaked, f"{row['id']}: accepted transcript has forbidden exact fields {leaked}")

    direct_redaction_rows = manifest.get("direct_spend_transcript_redaction_tests")
    require(
        isinstance(direct_redaction_rows, list) and direct_redaction_rows,
        "missing direct spend transcript redaction tests",
    )
    for row in direct_redaction_rows:
        artifact = load_json(ROOT / row["path"])
        evidence = pointer(artifact, row["evidence_pointer"])
        field_rows = pointer(artifact, row["field_rows_pointer"])
        require(isinstance(evidence, dict), f"{row['id']}: evidence pointer is not an object")
        require(isinstance(field_rows, list) and field_rows, f"{row['id']}: field rows are missing")

        logical_tuple_field = row["logical_tuple_field"]
        accepted_tuple_fields = evidence.get("accepted_tuple_fields")
        require(
            isinstance(accepted_tuple_fields, list) and logical_tuple_field in accepted_tuple_fields,
            f"{row['id']}: logical tuple field {logical_tuple_field!r} is not accounted",
        )
        matching_rows = [
            field_row
            for field_row in field_rows
            if field_row.get("accepted_tuple_field") == logical_tuple_field
        ]
        require(len(matching_rows) == 1, f"{row['id']}: expected exactly one logical field row")
        field_row = matching_rows[0]
        require(
            field_row.get("accepted_path_evidence_present") is True,
            f"{row['id']}: logical signature field lacks accepted-path evidence",
        )
        evidence_source = field_row.get("accepted_path_evidence_source", "")
        require(isinstance(evidence_source, str) and evidence_source, f"{row['id']}: evidence source is empty")
        for marker in row["required_evidence_source_markers"]:
            require(marker in evidence_source, f"{row['id']}: evidence source missing {marker!r}")

        for field in row["required_digest_fields"]:
            require(field in evidence, f"{row['id']}: missing digest field {field}")
            require_hex_digest(evidence[field], f"{row['id']}/{field}")

        raw_forbidden = set(row["forbidden_raw_field_keys"])
        leaked_raw = sorted(raw_forbidden & walk_keys(artifact))
        require(not leaked_raw, f"{row['id']}: public artifact exposes raw signature fields {leaked_raw}")
        require(row.get("deployment_ready") is False, f"{row['id']}: redaction row must not be deployment-ready")

    rejection_shape_rows = manifest.get("rejection_transcript_shape_tests")
    require(
        isinstance(rejection_shape_rows, list) and rejection_shape_rows,
        "missing rejection transcript shape tests",
    )
    for row in rejection_shape_rows:
        corpus = load_json(ROOT / row["path"])
        cases = pointer(corpus, row["cases_pointer"])
        expected_rejection_count = pointer(corpus, row["rejection_count_pointer"])
        require(isinstance(cases, list) and cases, f"{row['id']}: cases must be a nonempty list")
        accept_cases = [case for case in cases if case.get("kind") == "accept"]
        rejection_cases = [case for case in cases if case.get("kind") == "reject"]
        require(len(accept_cases) == 1, f"{row['id']}: expected exactly one accept case")
        require(
            len(rejection_cases) == expected_rejection_count,
            f"{row['id']}: rejection count does not match manifest",
        )
        allowed_classes = set(row["allowed_public_rejection_classes"])
        require(allowed_classes, f"{row['id']}: allowed public rejection class set is empty")
        for case in rejection_cases:
            case_id = case.get("case_id", "<unknown>")
            for field in row["required_rejection_fields"]:
                require(field in case, f"{row['id']}/{case_id}: missing {field}")
                require_public_value(case[field], f"{row['id']}/{case_id}/{field}")
            public_class = case["public_rejection_class"]
            require(
                public_class in allowed_classes,
                f"{row['id']}/{case_id}: unexpected public rejection class {public_class!r}",
            )
            observed_error = case.get("observed_error", "")
            lowered_error = observed_error.casefold()
            for needle in FORBIDDEN_OBSERVED_ERROR_SUBSTRINGS:
                require(
                    needle not in lowered_error,
                    f"{row['id']}/{case_id}: observed error contains forbidden substring {needle!r}",
                )
            leaked = sorted(forbidden & walk_keys(case))
            require(not leaked, f"{row['id']}/{case_id}: rejection case has forbidden exact fields {leaked}")

    known_surfaces = {row["id"] for row in surfaces}
    fixed_shape_rows = manifest.get("fixed_shape_failure_tests")
    require(
        isinstance(fixed_shape_rows, list) and fixed_shape_rows,
        "missing fixed-shape failure test rows",
    )
    fixed_shape_row_ids = [row.get("id") for row in fixed_shape_rows if isinstance(row, dict)]
    require(
        tuple(fixed_shape_row_ids)
        == (
            "direct_spend_public_result_fixed_shape",
            "deployment_callback_boundary_fixed_shape",
            "chain_state_storage_failure_fail_closed",
        ),
        "fixed-shape failure row set/order drifted",
    )
    for row in fixed_shape_rows:
        row_id = row["id"]
        require(row.get("deployment_ready") is False, f"{row_id}: fixed-shape row must not be deployment-ready")
        surface_ids = row.get("surface_ids")
        require(isinstance(surface_ids, list) and surface_ids, f"{row_id}: surface_ids must be nonempty")
        unknown = sorted(set(surface_ids) - known_surfaces)
        require(not unknown, f"{row_id}: unknown leakage surfaces {unknown}")
        require(
            set(surface_ids) >= {"spend_authorization", "signer_api"},
            f"{row_id}: fixed-shape row must cover spend authorization and signer API",
        )

        source = (ROOT / row["source_path"]).read_text()
        fast_gate = (ROOT / row["fast_gate_path"]).read_text()
        commands = row.get("reproducibility_commands")
        require(isinstance(commands, list) and commands, f"{row_id}: reproducibility commands missing")
        for command in commands:
            require(command in fast_gate, f"{row_id}: command missing from fast gate: {command}")

        covered_cases = row.get("covered_failure_cases")
        open_cases = row.get("open_failure_cases")
        require(isinstance(covered_cases, list) and len(covered_cases) >= 5, f"{row_id}: covered cases missing")
        require(isinstance(open_cases, list) and len(open_cases) >= 5, f"{row_id}: open cases missing")

        if row_id == "direct_spend_public_result_fixed_shape":
            require(
                tuple(row.get("required_test_functions", [])) == EXPECTED_FIXED_SHAPE_FUNCTIONS,
                f"{row_id}: fixed-shape test function set drifted",
            )
            for test_fn in EXPECTED_FIXED_SHAPE_FUNCTIONS:
                require(f"fn {test_fn}(" in source, f"{row_id}: missing fixed-shape test function {test_fn}")

            require(
                tuple(row.get("required_public_result_classes", [])) == EXPECTED_PUBLIC_RESULT_CLASSES,
                f"{row_id}: public result class set drifted",
            )
            for class_name in EXPECTED_PUBLIC_RESULT_CLASSES:
                require(class_name in source, f"{row_id}: missing public result class name {class_name}")
        elif row_id == "deployment_callback_boundary_fixed_shape":
            require(
                tuple(row.get("required_test_functions", [])) == EXPECTED_CALLBACK_FIXED_SHAPE_FUNCTIONS,
                f"{row_id}: callback fixed-shape test function set drifted",
            )
            for test_fn in EXPECTED_CALLBACK_FIXED_SHAPE_FUNCTIONS:
                require(f"fn {test_fn}(" in source, f"{row_id}: missing callback fixed-shape test function {test_fn}")
            require(
                not row.get("required_public_result_classes"),
                f"{row_id}: callback row must not pretend to close public result-class coverage",
            )
            require(
                any("callback failure" in case for case in covered_cases),
                f"{row_id}: callback failure coverage must be explicit",
            )
            require(
                any("diagnostic" in case for case in covered_cases),
                f"{row_id}: diagnostic-boundary coverage must be explicit",
            )
            require(
                any("non-callback dependency failure" in case for case in open_cases),
                f"{row_id}: non-callback dependency failures must remain open",
            )
        elif row_id == "chain_state_storage_failure_fail_closed":
            require(
                tuple(row.get("required_test_functions", [])) == EXPECTED_STORAGE_FIXED_SHAPE_FUNCTIONS,
                f"{row_id}: storage fixed-shape test function set drifted",
            )
            for test_fn in EXPECTED_STORAGE_FIXED_SHAPE_FUNCTIONS:
                require(f"fn {test_fn}(" in source, f"{row_id}: missing storage fixed-shape test function {test_fn}")
            require(
                not row.get("required_public_result_classes"),
                f"{row_id}: storage row must not pretend to close public result-class coverage",
            )
            require(
                any("lifecycle snapshot write failure" in case for case in covered_cases),
                f"{row_id}: lifecycle snapshot write failure coverage must be explicit",
            )
            require(
                any("retry is blocked by the persisted spent ledger" in case for case in covered_cases),
                f"{row_id}: persisted-ledger retry blocking must be explicit",
            )
            require(
                any("ledger write failure before durable spent-slot record" in case for case in open_cases),
                f"{row_id}: ledger-write failure must remain open",
            )
        else:  # pragma: no cover - row id set is checked above
            raise SystemExit(f"unknown fixed-shape row {row_id}")

        require(any("entropy-source failure" in case for case in open_cases), f"{row_id}: entropy failure must remain open")
        require(any("storage write failure" in case for case in open_cases), f"{row_id}: storage failure must remain open")
        require(any("resource-limit failure" in case for case in open_cases), f"{row_id}: resource-limit failure must remain open")
        require(row.get("negative_control_status"), f"{row_id}: negative-control status is required")
        for marker in row["required_source_markers"]:
            require(marker in source, f"{row_id}: source missing fixed-shape marker {marker!r}")

    harness_rows = manifest.get("manual_empirical_harnesses")
    require(
        isinstance(harness_rows, list) and harness_rows,
        "missing manual empirical leakage harness rows",
    )
    target_id = target_profile.get("target_id")
    for row in harness_rows:
        row_id = row["id"]
        require(row.get("deployment_ready") is False, f"{row_id}: manual harness must not be deployment-ready")
        require(row.get("target_id") == target_id, f"{row_id}: target profile mismatch")
        surface_ids = row.get("surface_ids")
        require(isinstance(surface_ids, list) and surface_ids, f"{row_id}: surface_ids must be nonempty")
        unknown = sorted(set(surface_ids) - known_surfaces)
        require(not unknown, f"{row_id}: unknown leakage surfaces {unknown}")
        require(row.get("sample_count") == 4000, f"{row_id}: sample count drifted")
        require("chi2 < 120.0" in row.get("threshold", ""), f"{row_id}: threshold must pin chi2 < 120.0")
        require(row.get("negative_control_status"), f"{row_id}: negative-control status is required")

        source_path = ROOT / row["source_path"]
        slow_gate_path = ROOT / row["slow_gate_path"]
        source = source_path.read_text()
        slow_gate = slow_gate_path.read_text()
        command = row["reproducibility_command"]
        require(command in slow_gate, f"{row_id}: reproducibility command missing from slow gate")
        require("let n_samples = 4000usize;" in source, f"{row_id}: source sample count drifted")
        require("let crit = 120.0;" in source, f"{row_id}: source chi-square threshold drifted")
        for test_fn in row["required_test_functions"]:
            require(f"fn {test_fn}(" in source, f"{row_id}: missing required test function {test_fn}")
        for marker in row["required_source_markers"]:
            require(marker in source, f"{row_id}: source missing marker {marker!r}")

    print(
        "Path 2 leakage battery manifest check passed "
        f"({len(surfaces)} surfaces, {len(artifact_rows)} public artifact scans, "
        f"{len(accepted_shape_rows)} accepted transcript scans, "
        f"{len(direct_redaction_rows)} direct spend redaction scans, "
        f"{len(rejection_shape_rows)} rejection transcript scans, "
        f"{len(fixed_shape_rows)} fixed-shape failure scans, "
        f"{len(harness_rows)} manual empirical harness scans)."
    )


if __name__ == "__main__":
    main()
