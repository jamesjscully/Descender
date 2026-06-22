#!/usr/bin/env python3
"""Check active wire-format authority against the Rust reference."""

from __future__ import annotations

import re
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "spec/wire_formats.md"
RUST = ROOT / "tools/parameter-search/src/production_reference.rs"

CHECKED_CORE_FIELDS = [
    "native_boundary_version",
    "parameter_set_id",
    "verifier_family_id",
    "account_creation_domain_digest",
    "spend_verifier_domain_digest",
    "chain_or_application_domain_digest",
    "fork_or_epoch_scope_digest",
    "aggregation_mode",
    "native_boundary_digest",
    "p_account_digest",
    "p_spend_digest",
    "p_lifecycle_digest",
    "p_transcript_digest",
    "p_state_digest",
    "message_digest",
    "current_state_digest",
    "proof_commitment",
]

ACCOUNT_STARK_PAYLOAD_DIGEST_FIELDS = [
    "schema",
    "payload_version",
    "account_stark_verifier_version",
    "leaf_context_digest",
    "creation_proof_binding_digest",
    "native_role_root_commitment",
    "aggregation_context_digest",
    "slot_id",
    "masked_projection_lane_count",
    "masked_projection_le",
    "account_stark_bundle",
]


def rust_str(source: str, name: str) -> str:
    match = re.search(rf'pub const {name}:\s*&str\s*=\s*"([^"]+)";', source)
    if not match:
        raise SystemExit(f"FAIL: Rust string const {name} not found")
    return match.group(1)


def rust_int(source: str, name: str) -> int:
    match = re.search(
        rf"pub const {name}:\s*(?:u(?:8|16|32|64)|usize|i(?:8|16|32|64))\s*=\s*(0x[0-9a-fA-F_]+|[0-9_]+);",
        source,
    )
    if not match:
        raise SystemExit(f"FAIL: Rust integer const {name} not found")
    return int(match.group(1).replace("_", ""), 0)


def check_equal(errors: list[str], label: str, actual: Any, expected: Any) -> None:
    if actual != expected:
        errors.append(f"{label}: got {actual!r}, expected {expected!r}")
    else:
        print(f"[OK ] {label}: {actual!r}")


def domain_rows(spec: str) -> dict[str, tuple[str, int]]:
    rows: dict[str, tuple[str, int]] = {}
    pattern = re.compile(r"^\|\s+`([^`]+)`\s+\|\s+`([^`]+)`\s+\|\s+([0-9]+)\s+\|", re.M)
    for const, value, byte_len in pattern.findall(spec):
        rows[const] = (value, int(byte_len))
    return rows


def section(text: str, heading: str, next_heading: str) -> str:
    start = text.find(heading)
    if start < 0:
        raise SystemExit(f"FAIL: section {heading!r} not found")
    end = text.find(next_heading, start + len(heading))
    if end < 0:
        raise SystemExit(f"FAIL: next section {next_heading!r} not found")
    return text[start:end]


def checked_core_spec_fields(spec: str) -> list[str]:
    checked_core = section(
        spec,
        "## Path2NativeCheckedCoreFrameV1",
        "## SpendAuthorizationRelationV1",
    )
    blocks = re.findall(r"```text\n(.*?)\n```", checked_core, re.S)
    block = next(
        (candidate for candidate in blocks if "NativeCheckedCoreFrame.v1" in candidate),
        None,
    )
    if block is None:
        raise SystemExit("FAIL: checked-core frame byte-encoding block not found")
    fields: list[str] = []
    for raw_line in block.splitlines():
        line = raw_line.strip()
        if not line:
            continue
        if line.startswith('ASCII("'):
            fields.append("domain")
            continue
        fields.append(re.split(r"[:\[]", line, maxsplit=1)[0])
    return fields


def account_stark_payload_spec_fields(spec: str) -> list[str]:
    payload = section(
        spec,
        "## Path2AccountStarkCreationPayloadV1",
        "## Path2NativeCheckedCoreFrameV1",
    )
    blocks = re.findall(r"```text\n(.*?)\n```", payload, re.S)
    block = next(
        (
            candidate
            for candidate in blocks
            if "path2_account_stark_creation_payload_v1_digest" in candidate
        ),
        None,
    )
    if block is None:
        raise SystemExit("FAIL: account-STARK payload digest block not found")
    fields: list[str] = []
    for raw_line in block.splitlines():
        line = raw_line.strip()
        if not line or line.startswith("path2_") or line.startswith("SHA256") or line == "))":
            continue
        if line.endswith(","):
            fields.append(line[:-1])
    return fields


def account_stark_payload_digest_fields(source: str) -> list[str]:
    start = source.find("pub fn path2_account_stark_creation_payload_v1_digest(")
    end = source.find("\n}\n\n/// FEATURE-GATED", start)
    if start < 0 or end < 0:
        raise SystemExit("FAIL: account-STARK payload digest function not found")
    body = source[start:end]
    return re.findall(r'append_(?:frame|u64)\(\s*&mut frame,\s*b"([^"]+)"', body)


def rust_struct_fields(source: str) -> list[str]:
    match = re.search(
        r"pub struct Path2NativeCheckedCoreFrameV1 \{(.*?)\n\}",
        source,
        re.S,
    )
    if not match:
        raise SystemExit("FAIL: Rust Path2NativeCheckedCoreFrameV1 struct not found")
    return re.findall(r"pub\s+([a-zA-Z0-9_]+):", match.group(1))


def rust_encoder_fields(source: str) -> list[str]:
    match = re.search(
        r"pub fn path2_native_checked_core_frame_v1_bytes\(.*?\n\}",
        source,
        re.S,
    )
    if not match:
        raise SystemExit("FAIL: Rust checked-core frame encoder not found")
    fields = re.findall(r"frame\.([a-zA-Z0-9_]+)", match.group(0))
    deduped: list[str] = []
    for field in fields:
        if not deduped or deduped[-1] != field:
            deduped.append(field)
    return deduped


def checked_core_test_body(source: str) -> str:
    start = source.find("\n    fn path2_native_checked_core_frame_v1_golden_vector_and_roundtrip()")
    end = source.find("\n    fn assert_no_active_native_private_material", start)
    if start < 0 or end < 0:
        raise SystemExit("FAIL: checked-core frame test block not found")
    return source[start:end]


def main() -> int:
    spec = SPEC.read_text(encoding="utf-8")
    source = RUST.read_text(encoding="utf-8")
    errors: list[str] = []

    if "Status: ACTIVE_NORMATIVE_FOR_AUTHORITY_MAP" not in "\n".join(spec.splitlines()[:8]):
        errors.append("spec/wire_formats.md is missing active authority marker")

    rows = domain_rows(spec)
    for const in [
        "PATH2_PREVALIDATED_SLOT_LEAF_DOMAIN",
        "PATH2_SLOT_AUTHORIZATION_DOMAIN",
        "PATH2_SLOT_PREVALIDATE_SINGLE_DOMAIN",
        "PATH2_NATIVE_CHECKED_CORE_FRAME_DOMAIN",
        "PATH2_ACCOUNT_STARK_CREATION_PAYLOAD_SCHEMA",
    ]:
        if const not in rows:
            errors.append(f"spec domain table missing {const}")
            continue
        spec_value, spec_len = rows[const]
        rust_value = rust_str(source, const)
        check_equal(errors, f"{const} value", rust_value, spec_value)
        check_equal(errors, f"{const} byte length", len(rust_value), spec_len)

    leaf_size = int(re.search(r"fixed\s+([0-9]+)-byte preimage", spec).group(1))
    check_equal(
        errors,
        "PrevalidatedSlotLeafV1 bytes",
        rust_int(source, "PATH2_PREVALIDATED_SLOT_LEAF_V1_BYTES"),
        leaf_size,
    )

    checked_core = section(
        spec,
        "## Path2NativeCheckedCoreFrameV1",
        "## SpendAuthorizationRelationV1",
    )
    spec_body = int(re.search(r"([0-9]+)-byte body", checked_core).group(1))
    spec_frame = int(re.search(r"current size is `([0-9]+)` bytes", checked_core).group(1))
    domain = rust_str(source, "PATH2_NATIVE_CHECKED_CORE_FRAME_DOMAIN")
    expected_body = 2 + 2 + 2 + 1 + 32 * 13
    check_equal(errors, "checked-core body bytes", spec_body, expected_body)
    check_equal(errors, "checked-core frame bytes", spec_frame, len(domain) + expected_body)
    if "2 + 2 + 2 + 1 + DIGEST_BYTES * 13" not in source:
        errors.append("Rust checked-core body-byte expression drifted from documented shape")

    fixed_values = {
        "PATH2_NATIVE_CHECKED_CORE_FRAME_VERSION": 1,
        "PATH2_NATIVE_CHECKED_CORE_PARAMETER_SET_Q12289": 1,
        "PATH2_NATIVE_CHECKED_CORE_VERIFIER_FAMILY_FQ12": 1,
        "PATH2_NATIVE_AGGREGATION_MODE_DIRECT": 0,
        "PATH2_NATIVE_AGGREGATION_MODE_AGGREGATE_COMPATIBLE": 1,
    }
    for const, expected in fixed_values.items():
        check_equal(errors, const, rust_int(source, const), expected)

    spec_fields = checked_core_spec_fields(spec)
    check_equal(errors, "checked-core spec field order", spec_fields, ["domain"] + CHECKED_CORE_FIELDS)
    check_equal(errors, "checked-core Rust struct field order", rust_struct_fields(source), CHECKED_CORE_FIELDS)
    check_equal(errors, "checked-core Rust encoder field order", rust_encoder_fields(source), CHECKED_CORE_FIELDS)

    account_payload_spec_fields = account_stark_payload_spec_fields(spec)
    check_equal(
        errors,
        "account-STARK payload digest spec field order",
        account_payload_spec_fields,
        ACCOUNT_STARK_PAYLOAD_DIGEST_FIELDS,
    )
    check_equal(
        errors,
        "account-STARK payload digest Rust field order",
        account_stark_payload_digest_fields(source),
        ACCOUNT_STARK_PAYLOAD_DIGEST_FIELDS,
    )
    for marker in [
        "path2_account_stark_creation_payload_v1_digest",
        "path2_account_stark_creation_roundtrip_and_binds",
        "account-creation STARK: payload digest mismatch",
        "account-creation STARK production verification is fail-closed (external-review-gated)",
        "deployed anchor mismatch: native_role_root_commitment",
    ]:
        if marker not in source:
            errors.append(f"missing account-STARK payload boundary marker {marker}")
        else:
            print(f"[OK ] account-STARK payload marker present: {marker}")

    tests = checked_core_test_body(source)
    required_test_markers = [
        "path2_native_checked_core_frame_v1_golden_vector_and_roundtrip",
        "path2_native_checked_core_frame_v1_mutations_change_challenge",
        "path2_native_checked_core_frame_v1_rejects_invalid_context",
        "path2_native_checked_core_frame_v1_excludes_legacy_private_handles",
        "path2_checked_core_authorization_challenge_binds_slot_authorization",
        "path2_checked_core_authorization_consumes_auxiliary_challenge_bytes_in_fq12_action",
    ]
    for marker in required_test_markers:
        if marker not in source:
            errors.append(f"missing Rust checked-core KAT/test marker {marker}")
        else:
            print(f"[OK ] Rust test marker present: {marker}")
    for field in CHECKED_CORE_FIELDS[3:]:
        if field not in tests:
            errors.append(f"checked-core KAT block does not mention field {field}")

    for denied in [
        "opening_commitment",
        "pad_seed_digest",
        "hidden_source_statement_digest",
        "fallback_carrier_lane_bytes",
    ]:
        if denied not in tests:
            errors.append(f"checked-core private-handle exclusion missing {denied}")

    if errors:
        print("FAIL: active wire-format authority drifted")
        for error in errors:
            print(f"  - {error}")
        return 1
    print("OK: active wire-format spec matches the Rust reference.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
