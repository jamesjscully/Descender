#!/usr/bin/env python3
"""Probe the lifecycle surface if one pad seed is reused across slots."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_signature_projection_pad_seed_reuse_surface_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_signature_projection_pad_seed_reuse_surface_n32_q12289.md"

Q = 12_289
GROUPS = 16
LOCAL_FREE = 2
PAD_SEED_BYTES = 32
PAD_FRAME_BYTES = 46
PAD_DOMAIN = b"Descender.Signature.q12289.local_delta.pad.v1"
PAD_SEED_OPENING_DOMAIN = b"Descender.Signature.q12289.local_delta.reviewed_fq12_pad_seed_opening.v1"
MASK_DOMAIN = b"Descender.Signature.q12289.local_delta.reviewed_fq12_packed_witness_state18_projection_mask.v1"
PROFILE = b"n32.q12289.local_delta.v1"
ACCOUNT_PROJECTION_LANES = 6


def append_frame(out: bytearray, label: bytes, data: bytes) -> None:
    out.extend(len(label).to_bytes(2, "little"))
    out.extend(label)
    out.extend(len(data).to_bytes(8, "little"))
    out.extend(data)


def append_u64(out: bytearray, label: bytes, value: int) -> None:
    append_frame(out, label, value.to_bytes(8, "little"))


def pad_seed() -> bytes:
    return hashlib.sha256(b"Descender.pad_seed_reuse_surface.seed").digest()


def rejection_values(frame_digest_fn, count: int) -> list[int]:
    cutoff = (1 << 16) // Q * Q
    out: list[int] = []
    counter = 0
    while len(out) < count:
        digest = frame_digest_fn(counter)
        for offset in range(0, len(digest), 2):
            candidate = int.from_bytes(digest[offset : offset + 2], "little")
            if candidate < cutoff:
                out.append(candidate % Q)
                if len(out) == count:
                    break
        counter += 1
    return out


def pad_frame(block: int) -> bytes:
    frame = bytearray(PAD_FRAME_BYTES)
    frame[: len(PAD_DOMAIN)] = PAD_DOMAIN
    frame[len(PAD_DOMAIN)] = block
    return bytes(frame)


def pad_residues(seed: bytes, block: int) -> list[int]:
    frame = pad_frame(block)
    return rejection_values(
        lambda counter: hashlib.sha256(seed + frame + counter.to_bytes(4, "little")).digest(),
        LOCAL_FREE,
    )


def pad_opening_statement_digest(seed: bytes, observed_blocks: list[bool]) -> str:
    frame = bytearray()
    append_frame(frame, b"domain", b"Descender.Signature.q12289.local_delta.reviewed_fq12_pad_opening_view.v1")
    append_frame(frame, b"profile", PROFILE)
    append_u64(frame, b"q", Q)
    append_u64(frame, b"groups", GROUPS)
    append_u64(frame, b"local_free", LOCAL_FREE)
    observed_bitmap = 0
    for block, observed in enumerate(observed_blocks):
        if observed:
            observed_bitmap |= 1 << block
    append_frame(frame, b"observed_bitmap", observed_bitmap.to_bytes(8, "little"))
    for block, observed in enumerate(observed_blocks):
        if observed:
            append_u64(frame, b"block", block)
            for value in pad_residues(seed, block):
                append_frame(frame, b"pad", value.to_bytes(2, "little"))
    return hashlib.sha256(frame).hexdigest()


def pad_seed_opening_digest(seed: bytes) -> str:
    frame = bytearray()
    append_frame(frame, b"domain", PAD_SEED_OPENING_DOMAIN)
    append_frame(frame, b"profile", PROFILE)
    append_u64(frame, b"q", Q)
    append_u64(frame, b"pad_seed_bytes", PAD_SEED_BYTES)
    append_frame(frame, b"pad_seed", seed)
    return hashlib.sha256(frame).hexdigest()


def mask_values(seed: bytes, slot_id: int) -> list[int]:
    mask_seed = seed + slot_id.to_bytes(8, "little")
    values = []
    for lane in range(ACCOUNT_PROJECTION_LANES):
        values.append(
            rejection_values(
                lambda counter, lane=lane: hashlib.sha256(
                    MASK_DOMAIN
                    + mask_seed
                    + slot_id.to_bytes(8, "little")
                    + lane.to_bytes(8, "little")
                    + counter.to_bytes(4, "little")
                ).digest(),
                1,
            )[0]
        )
    return values


def build_payload() -> dict[str, Any]:
    seed = pad_seed()
    observed_blocks = [False] * GROUPS
    for block in (1, 2, 6, 7, 11, 12):
        observed_blocks[block] = True
    slot0_masks = mask_values(seed, 0)
    slot1_masks = mask_values(seed, 1)
    slot0_statement = pad_opening_statement_digest(seed, observed_blocks)
    slot1_statement = pad_opening_statement_digest(seed, observed_blocks)
    seed_digest0 = pad_seed_opening_digest(seed)
    seed_digest1 = pad_seed_opening_digest(seed)
    return {
        "schema": "Descender.pre_signature_projection_pad_seed_reuse_surface.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "pad_seed_reuse_surface_fail_closed",
        "deployment_ready": False,
        "q": Q,
        "same_seed_slots": [0, 1],
        "observed_blocks": [idx for idx, observed in enumerate(observed_blocks) if observed],
        "slot0_masks": slot0_masks,
        "slot1_masks": slot1_masks,
        "masks_equal_across_slots": slot0_masks == slot1_masks,
        "pad_opening_statement_digest_slot0": slot0_statement,
        "pad_opening_statement_digest_slot1": slot1_statement,
        "pad_opening_statement_repeats": slot0_statement == slot1_statement,
        "pad_seed_digest_slot0": seed_digest0,
        "pad_seed_digest_slot1": seed_digest1,
        "pad_seed_digest_repeats": seed_digest0 == seed_digest1,
        "current_public_spend_record_fields": [
            "slot_id",
            "snapshot_sequence",
            "hidden_source_opening_commitment",
            "spent_record",
        ],
        "finding": {
            "summary": "Slot id separates the projection masks, but pad-seed reuse repeats the pad-seed digest and repeats the public pad-opening statement for the same observed block set.",
            "security_impact": "This does not by itself break the masked projection construction, but it means single-use pad-seed policy is a real lifecycle obligation rather than a theorem consequence of slot-id masking.",
            "recommended_next_control": "Track or reject repeated pad_seed_digest values at the selected-projection consensus/spent-record boundary, or prove that same-seed multi-slot reuse is harmless under the final PRF/leakage theorem.",
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Pad-Seed Reuse Surface",
        "",
        "This probe checks what repeats if the same pad seed is accidentally reused",
        "across two state-assisted slots.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- same-seed slots: `{payload['same_seed_slots']}`",
        f"- observed blocks: `{payload['observed_blocks']}`",
        f"- masks equal across slots: `{payload['masks_equal_across_slots']}`",
        f"- pad-opening statement repeats: `{payload['pad_opening_statement_repeats']}`",
        f"- pad-seed digest repeats: `{payload['pad_seed_digest_repeats']}`",
        "",
        "## Finding",
        "",
        payload["finding"]["summary"],
        "",
        f"Security impact: {payload['finding']['security_impact']}",
        "",
        f"Recommended next control: {payload['finding']['recommended_next_control']}",
        "",
        "## Current Public Spend Record Fields",
        "",
    ]
    lines.extend(f"- `{item}`" for item in payload["current_public_spend_record_fields"])
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
                "masks_equal_across_slots": payload["masks_equal_across_slots"],
                "pad_seed_digest_repeats": payload["pad_seed_digest_repeats"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
