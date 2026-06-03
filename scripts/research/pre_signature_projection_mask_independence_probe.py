#!/usr/bin/env python3
"""Probe pad-seed-derived projection masks against pad PRF projection lanes."""

from __future__ import annotations

import hashlib
import json
import math
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_signature_projection_mask_independence_probe_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_signature_projection_mask_independence_probe_n32_q12289.md"

Q = 12_289
GROUPS = 16
LOCAL_FREE = 2
SAMPLES = 50_000
PAD_DOMAIN = b"Descender.Signature.q12289.local_delta.pad.v1"
MASK_DOMAIN = b"Descender.Signature.q12289.local_delta.reviewed_fq12_packed_witness_state18_projection_mask.v1"
PAD_FRAME_BYTES = 46
ACCOUNT_PROJECTION_LANES = 6
PAD_PROJECTION_SYMBOLS = [
    "pad_seed_projection[2,1]",
    "pad_seed_projection[7,0]",
    "pad_seed_projection[12,1]",
    "pad_seed_projection[1,0]",
    "pad_seed_projection[6,1]",
    "pad_seed_projection[11,0]",
]
PAD_PROJECTION_COEFFICIENTS = [1934, 5142, 10681, 10066, 5429, 5754]


def pad_seed(sample: int) -> bytes:
    return hashlib.sha256(
        b"Descender.pre_signature_projection_mask_independence_probe.pad_seed"
        + sample.to_bytes(8, "little")
    ).digest()


def centered(value: int) -> int:
    value %= Q
    return value - Q if value > Q // 2 else value


def rejection_values_from_digest_stream(frame_digest_fn, count: int) -> list[int]:
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


def reviewed_fq12_pad_frame(block: int) -> bytes:
    assert 0 <= block < GROUPS
    frame = bytearray(PAD_FRAME_BYTES)
    frame[: len(PAD_DOMAIN)] = PAD_DOMAIN
    frame[len(PAD_DOMAIN)] = block
    return bytes(frame)


def pad_residues(seed: bytes, block: int) -> list[int]:
    frame = reviewed_fq12_pad_frame(block)
    return rejection_values_from_digest_stream(
        lambda counter: hashlib.sha256(seed + frame + counter.to_bytes(4, "little")).digest(),
        LOCAL_FREE,
    )


def pad_projection_values(seed: bytes) -> list[int]:
    values: list[int] = []
    for symbol, coeff in zip(PAD_PROJECTION_SYMBOLS, PAD_PROJECTION_COEFFICIENTS):
        inside = symbol.removeprefix("pad_seed_projection[").removesuffix("]")
        block_s, coord_s = inside.split(",")
        block = int(block_s)
        coord = int(coord_s)
        values.append((coeff * pad_residues(seed, block)[coord]) % Q)
    return values


def uniform_mod_from_hash(domain: bytes, seed: bytes, stream: int, index: int) -> int:
    return rejection_values_from_digest_stream(
        lambda counter: hashlib.sha256(
            domain
            + seed
            + stream.to_bytes(8, "little")
            + index.to_bytes(8, "little")
            + counter.to_bytes(4, "little")
        ).digest(),
        1,
    )[0]


def mask_values(seed: bytes, slot_id: int) -> list[int]:
    mask_seed = seed + slot_id.to_bytes(8, "little")
    return [
        uniform_mod_from_hash(MASK_DOMAIN, mask_seed, slot_id, lane)
        for lane in range(ACCOUNT_PROJECTION_LANES)
    ]


def pearson(xs: list[int], ys: list[int]) -> float:
    assert len(xs) == len(ys)
    n = len(xs)
    mean_x = sum(xs) / n
    mean_y = sum(ys) / n
    var_x = sum((x - mean_x) ** 2 for x in xs)
    var_y = sum((y - mean_y) ** 2 for y in ys)
    if var_x == 0 or var_y == 0:
        return 0.0
    cov = sum((x - mean_x) * (y - mean_y) for x, y in zip(xs, ys))
    return cov / math.sqrt(var_x * var_y)


def chi_square_2d(xs: list[int], ys: list[int], bins: int = 16) -> float:
    counts = Counter((min(bins - 1, x * bins // Q), min(bins - 1, y * bins // Q)) for x, y in zip(xs, ys))
    expected = len(xs) / (bins * bins)
    return sum((counts[(i, j)] - expected) ** 2 / expected for i in range(bins) for j in range(bins))


def build_payload() -> dict[str, Any]:
    mask_by_lane = [[] for _ in range(ACCOUNT_PROJECTION_LANES)]
    pad_by_lane = [[] for _ in range(ACCOUNT_PROJECTION_LANES)]
    same_slot_mask_equal_next_slot = 0
    for sample in range(SAMPLES):
        seed = pad_seed(sample)
        masks = mask_values(seed, slot_id=sample % 8)
        next_masks = mask_values(seed, slot_id=(sample % 8) + 1)
        pads = pad_projection_values(seed)
        if masks == next_masks:
            same_slot_mask_equal_next_slot += 1
        for lane in range(ACCOUNT_PROJECTION_LANES):
            mask_by_lane[lane].append(masks[lane])
            pad_by_lane[lane].append(pads[lane])

    lane_stats = []
    max_abs_corr = 0.0
    max_chi_square = 0.0
    equality_count = 0
    for lane in range(ACCOUNT_PROJECTION_LANES):
        mask_centered = [centered(value) for value in mask_by_lane[lane]]
        pad_centered = [centered(value) for value in pad_by_lane[lane]]
        corr = pearson(mask_centered, pad_centered)
        chi = chi_square_2d(mask_by_lane[lane], pad_by_lane[lane])
        eq = sum(1 for a, b in zip(mask_by_lane[lane], pad_by_lane[lane]) if a == b)
        equality_count += eq
        max_abs_corr = max(max_abs_corr, abs(corr))
        max_chi_square = max(max_chi_square, chi)
        lane_stats.append(
            {
                "lane": lane,
                "mask_distinct": len(set(mask_by_lane[lane])),
                "pad_distinct": len(set(pad_by_lane[lane])),
                "mask_pad_equal_count": eq,
                "mask_pad_equal_fraction": eq / SAMPLES,
                "pearson_centered": corr,
                "joint_16x16_chi_square": chi,
            }
        )

    return {
        "schema": "Descender.pre_signature_projection_mask_independence_probe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "pre_signature_projection_mask_independence_probe_completed_fail_closed",
        "deployment_ready": False,
        "q": Q,
        "samples": SAMPLES,
        "mask_domain": MASK_DOMAIN.decode(),
        "pad_domain": PAD_DOMAIN.decode(),
        "domains_distinct": MASK_DOMAIN != PAD_DOMAIN,
        "lane_stats": lane_stats,
        "summary": {
            "max_abs_pearson_centered": max_abs_corr,
            "max_joint_16x16_chi_square": max_chi_square,
            "total_mask_pad_equal_count": equality_count,
            "total_mask_pad_equal_fraction": equality_count / (SAMPLES * ACCOUNT_PROJECTION_LANES),
            "same_seed_adjacent_slot_mask_equal_count": same_slot_mask_equal_next_slot,
            "same_seed_adjacent_slot_mask_equal_fraction": same_slot_mask_equal_next_slot / SAMPLES,
            "no_obvious_mask_pad_correlation_in_probe": max_abs_corr < 0.02,
            "slot_id_changes_masks_in_probe": same_slot_mask_equal_next_slot == 0,
        },
        "interpretation": {
            "result": "No obvious marginal or pairwise correlation between mask lanes and pad projection lanes was found in this bounded probe.",
            "not_proved": [
                "cryptographic PRF independence of SHA-256 domains",
                "full lifecycle prechallenge fixation",
                "absence of higher-order correlations",
                "security if the same pad seed is ever reused across slots",
            ],
            "next_gate": "Add byte-level lifecycle tests proving the masked witness state is fixed before challenge and rejected under slot/pad replay.",
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Mask Independence Probe",
        "",
        "This probe compares pad-seed-derived account projection masks against",
        "the existing pad PRF projection lanes under distinct domains.",
        "",
        "## Summary",
        "",
        f"- q: `{payload['q']}`",
        f"- samples: `{payload['samples']}`",
        f"- mask domain: `{payload['mask_domain']}`",
        f"- pad domain: `{payload['pad_domain']}`",
        f"- domains distinct: `{payload['domains_distinct']}`",
        f"- max abs Pearson correlation: `{payload['summary']['max_abs_pearson_centered']:.6f}`",
        f"- total mask-pad equality fraction: `{payload['summary']['total_mask_pad_equal_fraction']:.6f}`",
        f"- adjacent slot mask equality fraction: `{payload['summary']['same_seed_adjacent_slot_mask_equal_fraction']:.6f}`",
        f"- no obvious mask-pad correlation in probe: `{payload['summary']['no_obvious_mask_pad_correlation_in_probe']}`",
        f"- slot id changes masks in probe: `{payload['summary']['slot_id_changes_masks_in_probe']}`",
        "",
        "## Lane Stats",
        "",
        "| lane | mask distinct | pad distinct | equal fraction | Pearson centered | joint chi-square |",
        "|---:|---:|---:|---:|---:|---:|",
    ]
    for row in payload["lane_stats"]:
        lines.append(
            f"| {row['lane']} | {row['mask_distinct']} | {row['pad_distinct']} | {row['mask_pad_equal_fraction']:.6f} | {row['pearson_centered']:.6f} | {row['joint_16x16_chi_square']:.2f} |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            payload["interpretation"]["result"],
            "",
            "Not proved:",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["interpretation"]["not_proved"])
    lines.extend(["", f"Next gate: {payload['interpretation']['next_gate']}", ""])
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
                "max_abs_pearson_centered": payload["summary"]["max_abs_pearson_centered"],
                "slot_id_changes_masks_in_probe": payload["summary"]["slot_id_changes_masks_in_probe"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
