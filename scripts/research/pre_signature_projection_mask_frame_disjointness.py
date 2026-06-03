#!/usr/bin/env python3
"""Emit a byte-frame disjointness certificate for pad PRF and projection masks."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_signature_projection_mask_frame_disjointness_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_signature_projection_mask_frame_disjointness_n32_q12289.md"

Q = 12_289
GROUPS = 16
PAD_SEED_BYTES = 32
PAD_FRAME_BYTES = 46
PAD_PRF_COUNTER_LIMIT = 16
ACCOUNT_PROJECTION_LANES = 6
PAD_DOMAIN = b"Descender.Signature.q12289.local_delta.pad.v1"
MASK_DOMAIN = b"Descender.Signature.q12289.local_delta.reviewed_fq12_packed_witness_state18_projection_mask.v1"


def sample_seed() -> bytes:
    return bytes((idx * 17 + 3) % 256 for idx in range(PAD_SEED_BYTES))


def pad_prf_input(seed: bytes, block: int, counter: int) -> bytes:
    assert len(seed) == PAD_SEED_BYTES
    assert 0 <= block < GROUPS
    frame = bytearray(PAD_FRAME_BYTES)
    frame[: len(PAD_DOMAIN)] = PAD_DOMAIN
    frame[len(PAD_DOMAIN)] = block
    return seed + bytes(frame) + counter.to_bytes(4, "little")


def mask_prf_input(seed: bytes, slot_id: int, lane: int, counter: int) -> bytes:
    assert len(seed) == PAD_SEED_BYTES
    assert 0 <= lane < ACCOUNT_PROJECTION_LANES
    mask_seed = seed + slot_id.to_bytes(8, "little")
    return (
        MASK_DOMAIN
        + mask_seed
        + slot_id.to_bytes(8, "little")
        + lane.to_bytes(8, "little")
        + counter.to_bytes(4, "little")
    )


def build_payload() -> dict[str, Any]:
    seed = sample_seed()
    pad_len = len(pad_prf_input(seed, 0, 0))
    mask_len = len(mask_prf_input(seed, 0, 0, 0))
    checked_pairs = 0
    collisions = []
    for block in range(GROUPS):
        for lane in range(ACCOUNT_PROJECTION_LANES):
            for counter in range(PAD_PRF_COUNTER_LIMIT):
                pad_input = pad_prf_input(seed, block, counter)
                mask_input = mask_prf_input(seed, block, lane, counter)
                checked_pairs += 1
                if pad_input == mask_input:
                    collisions.append(
                        {"block": block, "slot_id": block, "lane": lane, "counter": counter}
                    )

    return {
        "schema": "Descender.pre_signature_projection_mask_frame_disjointness.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "pre_signature_projection_mask_frame_disjointness_completed",
        "deployment_ready": False,
        "q": Q,
        "pad_domain": PAD_DOMAIN.decode(),
        "mask_domain": MASK_DOMAIN.decode(),
        "pad_domain_len": len(PAD_DOMAIN),
        "mask_domain_len": len(MASK_DOMAIN),
        "pad_prf_input_len": pad_len,
        "mask_prf_input_len": mask_len,
        "lengths_disjoint": pad_len != mask_len,
        "checked_bounded_pairs": checked_pairs,
        "bounded_byte_equal_collisions": collisions,
        "bounded_disjoint": not collisions,
        "reason": "Pad PRF input is seed || fixed 46-byte pad frame || counter; mask PRF input is mask-domain || seed || slot_id || slot_id || lane || counter. The selected layouts have different byte lengths.",
        "not_proved": [
            "SHA-256 PRF security",
            "random-oracle independence of different byte strings",
            "security under pad-seed reuse outside the state lifecycle",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Mask Frame Disjointness",
        "",
        "This certificate checks the byte layouts used as SHA-256 inputs for the",
        "existing pad PRF and the new packed-witness projection mask stream.",
        "",
        "## Summary",
        "",
        f"- q: `{payload['q']}`",
        f"- pad domain length: `{payload['pad_domain_len']}`",
        f"- mask domain length: `{payload['mask_domain_len']}`",
        f"- pad PRF input length: `{payload['pad_prf_input_len']}`",
        f"- mask PRF input length: `{payload['mask_prf_input_len']}`",
        f"- lengths disjoint: `{payload['lengths_disjoint']}`",
        f"- bounded pairs checked: `{payload['checked_bounded_pairs']}`",
        f"- bounded byte-equal collisions: `{len(payload['bounded_byte_equal_collisions'])}`",
        "",
        "## Reason",
        "",
        payload["reason"],
        "",
        "Not proved:",
        "",
    ]
    lines.extend(f"- {item}" for item in payload["not_proved"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "status": payload["status"],
                "lengths_disjoint": payload["lengths_disjoint"],
                "bounded_disjoint": payload["bounded_disjoint"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
