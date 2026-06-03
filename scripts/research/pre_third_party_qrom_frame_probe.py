#!/usr/bin/env python3
"""Probe the proof-bound Fiat-Shamir/QROM transcript frame.

The probe is intentionally small and byte-level: it replays the recorded
challenge-frame spec, mutates the frame ordering/labels/data, and expands the
single-target query ledger into a multi-target ledger.
"""

from __future__ import annotations

import hashlib
import json
import re
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
PRODUCTION_REFERENCE_LIB = ROOT / "tools/parameter-search/src/production_reference.rs"
QROM_MEMO = READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.json"
QROM_REVIEW = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "pre_third_party_qrom_frame_probe_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_third_party_qrom_frame_probe_n32_q12289.md"


@dataclass(frozen=True)
class Field:
    label: str
    data: bytes


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rust_string_const(source: str, name: str, fallback: str) -> str:
    match = re.search(rf'pub const {name}:\s*&str\s*=\s*"([^"]+)";', source)
    return match.group(1) if match else fallback


def rust_int_const(source: str, name: str, fallback: int) -> int:
    match = re.search(
        rf"pub const {name}:\s*(?:u(?:8|16|32|64)|usize|i(?:8|16|32|64))\s*=\s*([0-9_]+);",
        source,
    )
    return int(match.group(1).replace("_", "")) if match else fallback


def append_field(out: bytearray, label: str, data: bytes) -> None:
    label_bytes = label.encode("ascii")
    out.extend(len(label_bytes).to_bytes(2, "little"))
    out.extend(label_bytes)
    out.extend(len(data).to_bytes(8, "little"))
    out.extend(data)


def encode_fields(fields: list[Field]) -> bytes:
    out = bytearray()
    for field in fields:
        append_field(out, field.label, field.data)
    return bytes(out)


def parse_frame(frame: bytes) -> tuple[list[dict[str, Any]], list[str]]:
    offset = 0
    parsed: list[dict[str, Any]] = []
    errors: list[str] = []
    while offset < len(frame):
        field_start = offset
        if offset + 2 > len(frame):
            errors.append(f"truncated label length at byte {offset}")
            break
        label_len = int.from_bytes(frame[offset : offset + 2], "little")
        offset += 2
        if offset + label_len > len(frame):
            errors.append(f"truncated label at byte {field_start}")
            break
        label_bytes = frame[offset : offset + label_len]
        offset += label_len
        try:
            label = label_bytes.decode("ascii")
        except UnicodeDecodeError:
            label = label_bytes.hex()
            errors.append(f"non-ascii label at byte {field_start}")
        if offset + 8 > len(frame):
            errors.append(f"truncated data length for {label!r}")
            break
        data_len = int.from_bytes(frame[offset : offset + 8], "little")
        offset += 8
        data_start = offset
        data_end = offset + data_len
        if data_end > len(frame):
            errors.append(
                f"truncated data for {label!r}: needs {data_len} bytes at byte {data_start}"
            )
            break
        parsed.append(
            {
                "label": label,
                "label_len": label_len,
                "data_len": data_len,
                "field_offset": field_start,
                "data_offset": data_start,
                "end_offset": data_end,
                "sha256": hashlib.sha256(frame[data_start:data_end]).hexdigest(),
            }
        )
        offset = data_end
    return parsed, errors


def validate_against_spec(
    parsed: list[dict[str, Any]],
    parse_errors: list[str],
    required_labels: list[str],
    required_lengths: dict[str, int | None],
    required_values: dict[str, bytes],
) -> tuple[bool, list[str]]:
    labels = [field["label"] for field in parsed]
    errors = list(parse_errors)
    if labels != required_labels:
        errors.append(f"labels/order mismatch: {labels}")
    for field in parsed:
        expected = required_lengths.get(field["label"])
        if expected is not None and field["data_len"] != expected:
            errors.append(
                f"{field['label']} has {field['data_len']} bytes, expected {expected}"
            )
        expected_value = required_values.get(field["label"])
        if expected_value is not None and field["sha256"] != hashlib.sha256(expected_value).hexdigest():
            errors.append(f"{field['label']} value does not match the recorded constant")
    return not errors, errors


def hash_to_challenge(frame: bytes, challenge_bytes: int, algebraic_bytes: int, mask: int) -> bytes:
    out = bytearray(hashlib.sha256(frame).digest()[:challenge_bytes])
    out[algebraic_bytes - 1] &= mask
    return bytes(out)


def make_sample_fields(memo: dict[str, Any], source: str) -> list[Field]:
    public_key_bytes = rust_int_const(source, "PK_BYTES", 172)
    digest_bytes = rust_int_const(source, "DIGEST_BYTES", 32)
    proof_bytes = rust_int_const(source, "LINEAR_SIGMA_PROOF_COMMITMENT_BYTES", 32)
    message = b"descender qrom frame probe v1"
    return [
        Field("domain", memo["challenge_frame"]["domain"].encode("ascii")),
        Field("profile", memo["challenge_frame"]["profile"].encode("ascii")),
        Field("q", rust_int_const(source, "Q", 12289).to_bytes(8, "little")),
        Field("groups", rust_int_const(source, "GROUPS", 16).to_bytes(8, "little")),
        Field("local_pivot", rust_int_const(source, "LOCAL_PIVOT", 3).to_bytes(8, "little")),
        Field("local_free", rust_int_const(source, "LOCAL_FREE", 2).to_bytes(8, "little")),
        Field("repetitions", rust_int_const(source, "REPS", 26).to_bytes(8, "little")),
        Field("slot_id", (7).to_bytes(8, "little")),
        Field("public_key", bytes((i * 17 + 3) % 256 for i in range(public_key_bytes))),
        Field("message", message),
        Field("slot_digest", bytes((i * 29 + 11) % 256 for i in range(digest_bytes))),
        Field("proof_commitment", bytes((i * 43 + 19) % 256 for i in range(proof_bytes))),
    ]


def mutate_cases(fields: list[Field]) -> list[dict[str, Any]]:
    cases: list[tuple[str, str, list[Field]]] = [
        ("canonical", "unmodified proof-bound frame", fields),
        (
            "omit_proof_commitment",
            "legacy/stateful prefix without proof first-message binding",
            [field for field in fields if field.label != "proof_commitment"],
        ),
        (
            "omit_slot_digest",
            "drops spent-slot digest from the oracle input",
            [field for field in fields if field.label != "slot_digest"],
        ),
        (
            "omit_message",
            "drops signed message from the oracle input",
            [field for field in fields if field.label != "message"],
        ),
        (
            "swap_message_and_public_key",
            "reorders two variable public byte fields",
            fields[:8] + [fields[9], fields[8]] + fields[10:],
        ),
        (
            "move_proof_commitment_before_slot_digest",
            "keeps proof commitment but changes transcript order",
            fields[:10] + [fields[11], fields[10]],
        ),
        (
            "rename_proof_commitment",
            "keeps proof bytes under an unreviewed label",
            fields[:-1] + [Field("commitment", fields[-1].data)],
        ),
        (
            "duplicate_message",
            "adds a second message field after the proof commitment",
            fields + [Field("message", fields[9].data)],
        ),
        (
            "mutate_domain_byte",
            "changes the domain separator while preserving the label",
            [Field("domain", b"X" + fields[0].data[1:])] + fields[1:],
        ),
        (
            "mutate_proof_commitment_byte",
            "one-bit proof-commitment rebinding attempt",
            fields[:-1]
            + [Field("proof_commitment", bytes([fields[-1].data[0] ^ 1]) + fields[-1].data[1:])],
        ),
    ]
    return [
        {"case": name, "description": description, "frame": encode_fields(case_fields)}
        for name, description, case_fields in cases
    ]


def build_payload() -> dict[str, Any]:
    memo = load_json(QROM_MEMO)
    review = load_json(QROM_REVIEW)
    source = PRODUCTION_REFERENCE_LIB.read_text()

    required_labels = [field["label"] for field in memo["challenge_frame"]["fields"]]
    required_lengths: dict[str, int | None] = {
        "q": 8,
        "groups": 8,
        "local_pivot": 8,
        "local_free": 8,
        "repetitions": 8,
        "slot_id": 8,
        "slot_digest": 32,
        "proof_commitment": 32,
    }
    sample_fields = make_sample_fields(memo, source)
    fixed_values = {
        field.label: field.data
        for field in sample_fields
        if field.label
        in {
            "domain",
            "profile",
            "q",
            "groups",
            "local_pivot",
            "local_free",
            "repetitions",
        }
    }
    canonical_frame = encode_fields(sample_fields)
    canonical_parsed, canonical_parse_errors = parse_frame(canonical_frame)
    canonical_ok, canonical_errors = validate_against_spec(
        canonical_parsed,
        canonical_parse_errors,
        required_labels,
        required_lengths,
        fixed_values,
    )

    derivation = memo["challenge_derivation"]
    challenge_bytes = int(derivation["output_bytes_taken"])
    algebraic_bytes = int(derivation["algebraic_challenge_bytes"])
    algebraic_mask = int(derivation["algebraic_last_byte_mask"], 16)
    canonical_challenge = hash_to_challenge(
        canonical_frame, challenge_bytes, algebraic_bytes, algebraic_mask
    )

    mutations = []
    for case in mutate_cases(sample_fields):
        parsed, parse_errors = parse_frame(case["frame"])
        spec_ok, spec_errors = validate_against_spec(
            parsed, parse_errors, required_labels, required_lengths, fixed_values
        )
        challenge = hash_to_challenge(case["frame"], challenge_bytes, algebraic_bytes, algebraic_mask)
        frame_sha256 = hashlib.sha256(case["frame"]).hexdigest()
        mutations.append(
            {
                "case": case["case"],
                "description": case["description"],
                "frame_bytes": len(case["frame"]),
                "frame_sha256": frame_sha256,
                "challenge_hex": challenge.hex(),
                "same_frame_hash_as_canonical": frame_sha256
                == hashlib.sha256(canonical_frame).hexdigest(),
                "same_challenge_as_canonical": challenge == canonical_challenge,
                "spec_accepts_frame_shape": spec_ok,
                "spec_errors": spec_errors,
                "labels": [field["label"] for field in parsed],
            }
        )

    rust_constants = {
        "CHALLENGE_HASH_DOMAIN": rust_string_const(
            source, "CHALLENGE_HASH_DOMAIN", memo["challenge_frame"]["domain"]
        ),
        "PROFILE": rust_string_const(source, "PROFILE", memo["challenge_frame"]["profile"]),
        "Q": rust_int_const(source, "Q", 12289),
        "GROUPS": rust_int_const(source, "GROUPS", 16),
        "LOCAL_PIVOT": rust_int_const(source, "LOCAL_PIVOT", 3),
        "LOCAL_FREE": rust_int_const(source, "LOCAL_FREE", 2),
        "REPS": rust_int_const(source, "REPS", 26),
        "CHALLENGE_BYTES": rust_int_const(source, "CHALLENGE_BYTES", challenge_bytes),
        "ALGEBRAIC_CHALLENGE_BYTES": rust_int_const(
            source, "ALGEBRAIC_CHALLENGE_BYTES", algebraic_bytes
        ),
        "CHALLENGE_HARDENING_BYTES": rust_int_const(
            source, "CHALLENGE_HARDENING_BYTES", int(derivation["hardening_bytes"])
        ),
        "LINEAR_SIGMA_PROOF_COMMITMENT_BYTES": rust_int_const(
            source, "LINEAR_SIGMA_PROOF_COMMITMENT_BYTES", 32
        ),
    }
    source_checks = [
        {
            "description": "domain constant matches recorded QROM memo",
            "holds": rust_constants["CHALLENGE_HASH_DOMAIN"] == memo["challenge_frame"]["domain"],
            "evidence": {
                "rust": rust_constants["CHALLENGE_HASH_DOMAIN"],
                "memo": memo["challenge_frame"]["domain"],
            },
        },
        {
            "description": "profile constant matches recorded QROM memo",
            "holds": rust_constants["PROFILE"] == memo["challenge_frame"]["profile"],
            "evidence": {
                "rust": rust_constants["PROFILE"],
                "memo": memo["challenge_frame"]["profile"],
            },
        },
        {
            "description": "proof commitment is appended after the prefix in production_reference.rs",
            "holds": "append_frame(&mut frame, b\"proof_commitment\", proof_commitment)" in source,
            "evidence": {"source": rel(PRODUCTION_REFERENCE_LIB)},
        },
        {
            "description": "legacy frame constructor omits proof_commitment and is outside the proof-bound path",
            "holds": "legacy_stateful_challenge_hash_frame" in source,
            "evidence": {
                "legacy_mutation_spec_accepts": next(
                    item
                    for item in mutations
                    if item["case"] == "omit_proof_commitment"
                )["spec_accepts_frame_shape"]
            },
        },
    ]

    total_bits = int(derivation["total_transcript_challenge_bits"])
    algebraic_bits = int(derivation["algebraic_challenge_bits"])
    target_bits = int(memo["query_budget"]["target_security_bits"])
    q_quantum_log2 = int(memo["query_budget"]["deployment_quantum_hash_query_bound_log2"])
    q_classical_log2 = int(memo["query_budget"]["deployment_classical_hash_query_bound_log2"])
    target_exponents = [0, 1, 2, 4, 8, 16, 24, 32]
    multi_target_terms = []
    for target_log2 in target_exponents:
        quantum_bits = total_bits - 2 * q_quantum_log2 - target_log2
        classical_bits = total_bits - q_classical_log2 - target_log2
        algebraic_multi_bits = algebraic_bits - target_log2
        min_bits = min(quantum_bits, classical_bits, algebraic_multi_bits)
        multi_target_terms.append(
            {
                "target_count_log2": target_log2,
                "quantum_frame_term_bits": quantum_bits,
                "classical_frame_term_bits": classical_bits,
                "algebraic_action_term_bits": algebraic_multi_bits,
                "minimum_term_bits": min_bits,
                "margin_bits": min_bits - target_bits,
                "meets_128": min_bits >= target_bits,
            }
        )

    mutation_failures = [
        item
        for item in mutations
        if item["case"] != "canonical"
        and item["same_challenge_as_canonical"]
    ]
    failed_source_checks = [item for item in source_checks if not item["holds"]]
    positive = next(item for item in mutations if item["case"] == "canonical")
    status = (
        "checks_passed"
        if canonical_ok
        and positive["spec_accepts_frame_shape"]
        and not mutation_failures
        and not failed_source_checks
        else "checks_failed"
    )
    return {
        "schema": "Descender.pre_third_party_qrom_frame_probe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "campaign": "fiat_shamir_qrom_and_challenge_budget",
        "worker": "PTC-003",
        "sources": {
            "qrom_memo": rel(QROM_MEMO),
            "qrom_security_loss_review": rel(QROM_REVIEW),
            "production_reference": rel(PRODUCTION_REFERENCE_LIB),
        },
        "frame_spec": {
            "encoding": memo["challenge_frame"]["encoding"],
            "required_labels_in_order": required_labels,
            "required_public_byte_fields": [
                {
                    "label": field["label"],
                    "type": field["type"],
                    "secret": field["secret"],
                    "required_length": required_lengths.get(field["label"]),
                }
                for field in memo["challenge_frame"]["fields"]
            ],
            "proof_commitment_binding": {
                "field": memo["challenge_frame"]["first_message_field"],
                "bytes": memo["challenge_frame"]["first_message_bytes"],
                "role": "prechallenge first message included before challenge derivation",
                "same_commitment_qrom_condition": "forks must keep this field identical while challenges differ",
            },
        },
        "canonical_probe": {
            "frame_bytes": len(canonical_frame),
            "frame_sha256": hashlib.sha256(canonical_frame).hexdigest(),
            "challenge_hex": canonical_challenge.hex(),
            "spec_accepts_frame_shape": canonical_ok,
            "spec_errors": canonical_errors,
            "decoded_fields": canonical_parsed,
        },
        "source_checks": source_checks,
        "mutation_results": mutations,
        "mutation_failures": mutation_failures,
        "query_budget": {
            "single_target_terms": review["derived_terms"],
            "loss_models": review["adversary_ledger"],
            "multi_target_terms": multi_target_terms,
            "max_target_count_log2_at_128_bits": max(
                item["target_count_log2"] for item in multi_target_terms if item["meets_128"]
            ),
            "interpretation": (
                "The recorded deployment ledger has single-target margin. A union over more "
                "than 2 transcript targets must be charged by another budget term or tighter "
                "query bound before claiming 128-bit security."
            ),
        },
        "status": status,
        "residual_review_questions": [
            "Confirm the production verifier never routes deployed proof signatures through the legacy no-proof-commitment frame.",
            "Decide how many concurrent public keys/messages/accounts are covered by the final theorem statement.",
            "Review whether the QROM measure-and-reprogram theorem charges the same multi-target term modeled here.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    frame = payload["frame_spec"]
    canonical = payload["canonical_probe"]
    lines = [
        "# Pre-Third-Party QROM Frame Probe",
        "",
        f"- campaign: `{payload['campaign']}`",
        f"- worker: `{payload['worker']}`",
        f"- status: `{payload['status']}`",
        f"- generated at: `{payload['generated_at']}`",
        "",
        "## Sources",
        "",
    ]
    lines.extend(f"- {name}: `{path}`" for name, path in payload["sources"].items())
    lines.extend(
        [
            "",
            "## Required Frame",
            "",
            f"- encoding: `{frame['encoding']}`",
            f"- required order: `{' -> '.join(frame['required_labels_in_order'])}`",
            f"- proof commitment: `{frame['proof_commitment_binding']['field']}` "
            f"({frame['proof_commitment_binding']['bytes']} bytes)",
            "",
            "| label | type | length | secret? |",
            "|---|---|---:|:---:|",
        ]
    )
    for field in frame["required_public_byte_fields"]:
        length = "*" if field["required_length"] is None else str(field["required_length"])
        lines.append(
            f"| {field['label']} | {field['type']} | {length} | "
            f"{'yes' if field['secret'] else 'no'} |"
        )
    lines.extend(
        [
            "",
            "## Canonical Sample",
            "",
            f"- frame bytes: `{canonical['frame_bytes']}`",
            f"- frame sha256: `{canonical['frame_sha256']}`",
            f"- challenge: `{canonical['challenge_hex']}`",
            f"- shape accepted by recorded spec: `{canonical['spec_accepts_frame_shape']}`",
            "",
            "## Mutation Probe",
            "",
            "| case | shape accepted? | same challenge? | result |",
            "|---|:---:|:---:|---|",
        ]
    )
    for item in payload["mutation_results"]:
        result = "accepted canonical" if item["case"] == "canonical" else "rejected/drift"
        if item in payload["mutation_failures"]:
            result = "unexpected survivor"
        lines.append(
            f"| {item['case']} | {item['spec_accepts_frame_shape']} | "
            f"{item['same_challenge_as_canonical']} | {result} |"
        )
    lines.extend(
        [
            "",
            "## Source Checks",
            "",
        ]
    )
    for check in payload["source_checks"]:
        lines.append(f"- `{check['holds']}` {check['description']}")
    lines.extend(
        [
            "",
            "## Multi-Target Ledger",
            "",
            "| log2(targets) | quantum bits | classical bits | algebraic bits | min bits | margin |",
            "|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for item in payload["query_budget"]["multi_target_terms"]:
        lines.append(
            f"| {item['target_count_log2']} | {item['quantum_frame_term_bits']} | "
            f"{item['classical_frame_term_bits']} | {item['algebraic_action_term_bits']} | "
            f"{item['minimum_term_bits']} | {item['margin_bits']} |"
        )
    lines.extend(
        [
            "",
            payload["query_budget"]["interpretation"],
            "",
            "## Residual Review Questions",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["residual_review_questions"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
