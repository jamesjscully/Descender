#!/usr/bin/env python3
"""Generate deterministic test vectors for state-assisted opening frames."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.md"
)
OPENING_FRAME_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def sha256(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()


def synthetic_bytes(label: str, length: int) -> bytes:
    output = b""
    counter = 0
    while len(output) < length:
        output += sha256(f"Descender.test_vector.v1:{label}:{counter}".encode())
        counter += 1
    return output[:length]


def field_bytes(field: dict[str, Any], overrides: dict[str, bytes]) -> bytes:
    name = field["field"]
    if name in overrides:
        value = overrides[name]
    elif field["encoding"] == "ascii bytes":
        value = str(field["value"]).encode()
    elif field["encoding"] == "uint64 little-endian":
        value = (7).to_bytes(8, "little")
    else:
        value = synthetic_bytes(name, int(field["length_bytes"]))
    expected_len = int(field["length_bytes"])
    if len(value) != expected_len:
        raise ValueError(f"{name} encoded to {len(value)} bytes, expected {expected_len}")
    return value


def serialize_frame(frame: list[dict[str, Any]], overrides: dict[str, bytes] | None = None) -> bytes:
    active_overrides = overrides or {}
    return b"".join(field_bytes(field, active_overrides) for field in frame)


def mutate_one_byte(value: bytes) -> bytes:
    if not value:
        raise ValueError("cannot mutate empty value")
    return bytes([value[0] ^ 1]) + value[1:]


def build_payload() -> dict[str, Any]:
    opening_frame = load_json(OPENING_FRAME_JSON)
    opening_fields = opening_frame["opening_commitment"]["frame"]
    carrier_fields = opening_frame["carrier_handle"]["frame"]

    opening_bytes = serialize_frame(opening_fields)
    opening_digest = sha256(opening_bytes)
    carrier_bytes = serialize_frame(
        carrier_fields,
        {"hidden_source_opening_commitment": opening_digest},
    )
    carrier_digest = sha256(carrier_bytes)

    swapped_opening_fields = list(opening_fields)
    witness_index = next(
        index
        for index, field in enumerate(swapped_opening_fields)
        if field["field"] == "account_witness_commitment"
    )
    randomizer_index = next(
        index
        for index, field in enumerate(swapped_opening_fields)
        if field["field"] == "account_randomizer_commitment"
    )
    swapped_opening_fields[witness_index], swapped_opening_fields[randomizer_index] = (
        swapped_opening_fields[randomizer_index],
        swapped_opening_fields[witness_index],
    )
    swapped_digest = sha256(serialize_frame(swapped_opening_fields))

    opening_domain = next(field for field in opening_fields if field["field"] == "domain")
    opening_domain_digest = sha256(
        serialize_frame(
            opening_fields,
            {"domain": mutate_one_byte(str(opening_domain["value"]).encode())},
        )
    )
    pad_seed = field_bytes(
        next(field for field in opening_fields if field["field"] == "pad_seed_digest"),
        {},
    )
    pad_mutation_digest = sha256(
        serialize_frame(opening_fields, {"pad_seed_digest": mutate_one_byte(pad_seed)})
    )
    message_digest = field_bytes(
        next(field for field in carrier_fields if field["field"] == "message_digest"),
        {},
    )
    carrier_message_mutation_digest = sha256(
        serialize_frame(
            carrier_fields,
            {
                "hidden_source_opening_commitment": opening_digest,
                "message_digest": mutate_one_byte(message_digest),
            },
        )
    )
    carrier_slot_mutation_digest = sha256(
        serialize_frame(
            carrier_fields,
            {
                "hidden_source_opening_commitment": opening_digest,
                "slot_id": (8).to_bytes(8, "little"),
            },
        )
    )

    mutation_vectors = [
        {
            "id": "field_order_mutation_rejects",
            "frame": "opening_commitment",
            "mutation": "swap account_witness_commitment and account_randomizer_commitment fields",
            "expected_digest_hex": swapped_digest.hex(),
            "digest_changes": swapped_digest != opening_digest,
        },
        {
            "id": "domain_profile_mutation_rejects",
            "frame": "opening_commitment",
            "mutation": "flip one bit in opening domain",
            "expected_digest_hex": opening_domain_digest.hex(),
            "digest_changes": opening_domain_digest != opening_digest,
        },
        {
            "id": "private_opening_mutation_rejects",
            "frame": "opening_commitment",
            "mutation": "flip one bit in pad_seed_digest",
            "expected_digest_hex": pad_mutation_digest.hex(),
            "digest_changes": pad_mutation_digest != opening_digest,
        },
        {
            "id": "public_context_mutation_rejects_message",
            "frame": "carrier_handle",
            "mutation": "flip one bit in message_digest with carrier bytes otherwise fixed",
            "expected_digest_hex": carrier_message_mutation_digest.hex(),
            "digest_changes": carrier_message_mutation_digest != carrier_digest,
        },
        {
            "id": "public_context_mutation_rejects_slot",
            "frame": "carrier_handle",
            "mutation": "change slot_id from 7 to 8",
            "expected_digest_hex": carrier_slot_mutation_digest.hex(),
            "digest_changes": carrier_slot_mutation_digest != carrier_digest,
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_opening_frame_test_vectors.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "opening_frame_test_vectors_generated_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "opening_frame": rel(OPENING_FRAME_JSON),
            "opening_frame_schema": opening_frame.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_carrier",
        },
        "canonical_vectors": {
            "slot_id": 7,
            "opening_frame_bytes": len(opening_bytes),
            "opening_frame_sha256": opening_digest.hex(),
            "carrier_frame_bytes": len(carrier_bytes),
            "carrier_frame_sha256": carrier_digest.hex(),
        },
        "mutation_vectors": mutation_vectors,
        "all_mutations_change_digest": all(
            vector["digest_changes"] for vector in mutation_vectors
        ),
        "decision": {
            "can_accept_now": False,
            "reason": "test vectors bind serialization, but an executable accepting carrier verifier and extractor theorem remain open",
            "next_artifact": "state_assisted_carrier_verifier_contract",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    canonical = payload["canonical_vectors"]
    lines = [
        "# Hidden-Source State-Assisted Opening-Frame Test Vectors",
        "",
        "This artifact turns the pinned opening and carrier byte frames into",
        "deterministic SHA-256 test vectors. It remains fail-closed: these vectors",
        "test canonical serialization and mutation rejection, not proof soundness.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- opening frame: `{payload['source_artifacts']['opening_frame']}`",
        f"- selected candidate: `{payload['target']['selected_candidate']}`",
        f"- n: `{payload['target']['n']}`",
        f"- q: `{payload['target']['q']}`",
        f"- carrier bytes: `{payload['target']['carrier_bytes']}`",
        f"- envelope bytes: `{payload['target']['envelope_bytes']}`",
        f"- all mutations change digest: `{payload['all_mutations_change_digest']}`",
        "",
        "## Canonical Vectors",
        "",
        f"- slot id: `{canonical['slot_id']}`",
        f"- opening frame bytes: `{canonical['opening_frame_bytes']}`",
        f"- opening frame SHA-256: `{canonical['opening_frame_sha256']}`",
        f"- carrier frame bytes: `{canonical['carrier_frame_bytes']}`",
        f"- carrier frame SHA-256: `{canonical['carrier_frame_sha256']}`",
        "",
        "## Mutation Vectors",
        "",
        "| vector | frame | mutation | digest changes | expected digest |",
        "|---|---|---|:---:|---|",
    ]
    for vector in payload["mutation_vectors"]:
        lines.append(
            f"| `{vector['id']}` | `{vector['frame']}` | {vector['mutation']} | `{vector['digest_changes']}` | `{vector['expected_digest_hex']}` |"
        )
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- reason: {payload['decision']['reason']}",
            f"- next artifact: `{payload['decision']['next_artifact']}`",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
