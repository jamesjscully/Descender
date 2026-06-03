#!/usr/bin/env python3
"""Deterministic PTC-004 pad-PRF framing and independence probe.

This is a local adversarial sanity harness, not a proof of SHA-256 PRF
security.  It reimplements the reviewed Rust/Lean pad framing directly and
records deterministic statistical and differential checks that are useful
before third-party cryptoanalysis.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_third_party_pad_prf_seed_sweep_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_third_party_pad_prf_seed_sweep_n32_q12289.md"

Q = 12_289
GROUPS = 16
LOCAL_FREE = 2
PAD_SEED_BYTES = 32
PAD_DOMAIN = b"Descender.Signature.q12289.local_delta.pad.v1"
PAD_FRAME_BYTES = 46
PAD_PRF_COUNTER_LIMIT = 16
REJECTION_CUTOFF = ((1 << 16) // Q) * Q
PROJECTION_LANE_COEFFICIENTS = [
    6234,
    2199,
    6252,
    1488,
    2113,
    823,
    1934,
    5142,
    10681,
    10066,
    5429,
    5754,
]
PAD_PROJECTION_LANE_MAP = [
    (6, 2, 1),
    (7, 7, 0),
    (8, 12, 1),
    (9, 1, 0),
    (10, 6, 1),
    (11, 11, 0),
]
PAD_PROJECTION_SYMBOLS = [
    "pad_seed_projection[2,1]",
    "pad_seed_projection[7,0]",
    "pad_seed_projection[12,1]",
    "pad_seed_projection[1,0]",
    "pad_seed_projection[6,1]",
    "pad_seed_projection[11,0]",
]
SEED42_PROJECTION_VECTOR = [12169, 2446, 4817, 3574, 5914, 8556]
SEED42_BLOCK_RESIDUES = {
    1: [2674, 9115],
    2: [1311, 6011],
    6: [4034, 7068],
    7: [6329, 9937],
    11: [6870, 8128],
    12: [9169, 11132],
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def sha256(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()


def bit_count(data: bytes) -> int:
    return sum(byte.bit_count() for byte in data)


def hamming_bytes(left: bytes, right: bytes) -> int:
    return bit_count(bytes(a ^ b for a, b in zip(left, right)))


def deterministic_seed(index: int) -> bytes:
    return sha256(
        b"Descender.pre_third_party_pad_prf.seed_sweep.seed.v1"
        + index.to_bytes(8, "little")
    )


def one_bit_flip(data: bytes, bit_index: int) -> bytes:
    out = bytearray(data)
    out[bit_index // 8] ^= 1 << (bit_index % 8)
    return bytes(out)


def pad_frame(block: int, *, domain: bytes = PAD_DOMAIN) -> bytes:
    if not 0 <= block < GROUPS:
        raise ValueError(f"block out of range: {block}")
    out = bytearray(PAD_FRAME_BYTES)
    out[: len(domain)] = domain
    out[len(domain)] = block
    return bytes(out)


def pad_prf_digest(
    seed: bytes, block: int, counter: int, *, domain: bytes = PAD_DOMAIN
) -> bytes:
    if len(seed) != PAD_SEED_BYTES:
        raise ValueError("pad seed must be exactly 32 bytes")
    return sha256(seed + pad_frame(block, domain=domain) + counter.to_bytes(4, "little"))


def accepted_residues_from_digest(digest: bytes) -> list[int]:
    residues: list[int] = []
    for offset in range(0, len(digest), 2):
        word = int.from_bytes(digest[offset : offset + 2], "little")
        if word < REJECTION_CUTOFF:
            residues.append(word % Q)
    return residues


def pad_residues_bounded(
    seed: bytes,
    block: int,
    *,
    domain: bytes = PAD_DOMAIN,
    counter_limit: int = PAD_PRF_COUNTER_LIMIT,
) -> tuple[list[int] | None, dict[str, int]]:
    accepted: list[int] = []
    rejected_words = 0
    words_seen = 0
    counters_used = 0
    for counter in range(counter_limit):
        counters_used += 1
        digest = pad_prf_digest(seed, block, counter, domain=domain)
        for offset in range(0, len(digest), 2):
            words_seen += 1
            word = int.from_bytes(digest[offset : offset + 2], "little")
            if word < REJECTION_CUTOFF:
                accepted.append(word % Q)
                if len(accepted) == LOCAL_FREE:
                    return accepted, {
                        "counters_used": counters_used,
                        "accepted_words": len(accepted),
                        "rejected_words": rejected_words,
                        "words_seen": words_seen,
                    }
            else:
                rejected_words += 1
    return None, {
        "counters_used": counter_limit,
        "accepted_words": len(accepted),
        "rejected_words": rejected_words,
        "words_seen": words_seen,
    }


def pad_projection_values(
    seed: bytes,
    *,
    domain: bytes = PAD_DOMAIN,
    counter_limit: int = PAD_PRF_COUNTER_LIMIT,
) -> tuple[list[int] | None, list[dict[str, Any]]]:
    values: list[int] = []
    lane_details: list[dict[str, Any]] = []
    for lane, block, coord in PAD_PROJECTION_LANE_MAP:
        residues, stats = pad_residues_bounded(
            seed, block, domain=domain, counter_limit=counter_limit
        )
        detail: dict[str, Any] = {
            "lane": lane,
            "symbol": PAD_PROJECTION_SYMBOLS[len(lane_details)],
            "block": block,
            "coord": coord,
            "derivation_stats": stats,
        }
        if residues is None:
            detail["status"] = "exhausted"
            lane_details.append(detail)
            return None, lane_details
        value = (PROJECTION_LANE_COEFFICIENTS[lane] * residues[coord]) % Q
        detail.update({"status": "ok", "residues": residues, "value": value})
        values.append(value)
        lane_details.append(detail)
    return values, lane_details


def decode_pad_seed(seed_bytes: Iterable[int]) -> list[int] | None:
    seed = list(seed_bytes)
    if len(seed) != PAD_SEED_BYTES:
        return None
    if any(byte < 0 or byte > 255 for byte in seed):
        return None
    return seed


def bucket_uniformity(values: list[int], modulus: int) -> dict[str, Any]:
    counts = Counter(value % modulus for value in values)
    expected = len(values) / modulus
    chi_square = (
        sum(((counts.get(bucket, 0) - expected) ** 2) / expected for bucket in range(modulus))
        if expected
        else 0.0
    )
    return {
        "modulus": modulus,
        "sample_count": len(values),
        "expected_per_bucket": expected,
        "min_bucket_count": min(counts.get(bucket, 0) for bucket in range(modulus)),
        "max_bucket_count": max(counts.get(bucket, 0) for bucket in range(modulus)),
        "chi_square": chi_square,
        "normalized_chi_square": chi_square / max(1, modulus - 1),
    }


def correlation(xs: list[float], ys: list[float]) -> float:
    if len(xs) != len(ys) or not xs:
        return 0.0
    mean_x = sum(xs) / len(xs)
    mean_y = sum(ys) / len(ys)
    dx = [x - mean_x for x in xs]
    dy = [y - mean_y for y in ys]
    denom_x = math.sqrt(sum(x * x for x in dx))
    denom_y = math.sqrt(sum(y * y for y in dy))
    if denom_x == 0 or denom_y == 0:
        return 0.0
    return sum(x * y for x, y in zip(dx, dy)) / (denom_x * denom_y)


def digest_avalanche(seed: bytes, block: int, counter: int, mutated_seed: bytes) -> int:
    return hamming_bytes(
        pad_prf_digest(seed, block, counter),
        pad_prf_digest(mutated_seed, block, counter),
    )


def build_payload(seed_count: int) -> dict[str, Any]:
    seeds = [deterministic_seed(index) for index in range(seed_count)]
    all_residues: list[int] = []
    all_projection_values: list[int] = []
    counter_hist = Counter()
    rejection_hist = Counter()
    exhausted_examples: list[dict[str, Any]] = []
    projection_failures = 0

    for seed_index, seed in enumerate(seeds):
        projection, _details = pad_projection_values(seed)
        if projection is None:
            projection_failures += 1
        else:
            all_projection_values.extend(projection)
        for block in range(GROUPS):
            residues, stats = pad_residues_bounded(seed, block)
            counter_hist[stats["counters_used"]] += 1
            rejection_hist[stats["rejected_words"]] += 1
            if residues is None:
                exhausted_examples.append(
                    {"seed_index": seed_index, "block": block, "stats": stats}
                )
            else:
                all_residues.extend(residues)

    seed42 = bytes([0x42] * PAD_SEED_BYTES)
    seed42_projection, seed42_details = pad_projection_values(seed42)
    seed42_block_rows = []
    for block, expected in SEED42_BLOCK_RESIDUES.items():
        observed, stats = pad_residues_bounded(seed42, block)
        seed42_block_rows.append(
            {
                "block": block,
                "expected": expected,
                "observed": observed,
                "stats": stats,
                "status": "pass" if observed == expected else "fail",
            }
        )

    domain_mutation = bytes([PAD_DOMAIN[0] ^ 1]) + PAD_DOMAIN[1:]
    domain_sep_rows = []
    domain_collisions = 0
    for seed_index, seed in enumerate(seeds[: min(seed_count, 256)]):
        canonical, _ = pad_projection_values(seed)
        mutated, _ = pad_projection_values(seed, domain=domain_mutation)
        equal = canonical == mutated
        domain_collisions += int(equal)
        if seed_index < 8:
            domain_sep_rows.append(
                {
                    "seed_index": seed_index,
                    "canonical_projection": canonical,
                    "mutated_domain_projection": mutated,
                    "equal": equal,
                }
            )

    avalanche_rows = []
    seed_hamming_values: list[int] = []
    block_hamming_values: list[int] = []
    projection_distances: list[int] = []
    for seed_index, seed in enumerate(seeds[: min(seed_count, 256)]):
        mutated_seed = one_bit_flip(seed, seed_index % (PAD_SEED_BYTES * 8))
        block = seed_index % GROUPS
        seed_hamming = digest_avalanche(seed, block, 0, mutated_seed)
        block_hamming = hamming_bytes(
            pad_prf_digest(seed, block, 0),
            pad_prf_digest(seed, block ^ 1, 0),
        )
        projection, _ = pad_projection_values(seed)
        mutated_projection, _ = pad_projection_values(mutated_seed)
        projection_distance = (
            sum(a != b for a, b in zip(projection, mutated_projection))
            if projection is not None and mutated_projection is not None
            else 0
        )
        seed_hamming_values.append(seed_hamming)
        block_hamming_values.append(block_hamming)
        projection_distances.append(projection_distance)
        if seed_index < 8:
            avalanche_rows.append(
                {
                    "seed_index": seed_index,
                    "block": block,
                    "flipped_seed_bit": seed_index % (PAD_SEED_BYTES * 8),
                    "seed_digest_hamming_bits": seed_hamming,
                    "block_frame_hamming_bits": block_hamming,
                    "projection_lanes_changed": projection_distance,
                }
            )

    message_slot_controls = []
    message_slot_invariant_count = 0
    for seed_index, seed in enumerate(seeds[: min(seed_count, 128)]):
        projection, _ = pad_projection_values(seed)
        message_a = sha256(b"message A" + seed_index.to_bytes(4, "little"))
        message_b = one_bit_flip(message_a, 0)
        slot_a = seed_index
        slot_b = seed_index ^ 1
        context_a = sha256(
            b"Descender.pre_third_party_pad_prf.context_control.v1"
            + message_a
            + slot_a.to_bytes(8, "little")
        )
        context_b = sha256(
            b"Descender.pre_third_party_pad_prf.context_control.v1"
            + message_b
            + slot_b.to_bytes(8, "little")
        )
        invariant = projection == pad_projection_values(seed)[0]
        message_slot_invariant_count += int(invariant)
        if seed_index < 8:
            message_slot_controls.append(
                {
                    "seed_index": seed_index,
                    "message_slot_context_hamming_bits": hamming_bytes(context_a, context_b),
                    "pad_projection_unchanged_for_fixed_seed": invariant,
                    "interpretation": "message and slot are not pad-PRF frame inputs in the reviewed boundary",
                }
            )

    active_condition_rows = []
    forbidden_condition_changed_count = 0
    for seed_index, seed in enumerate(seeds[: min(seed_count, 128)]):
        projection, _ = pad_projection_values(seed)
        active_condition_a = sha256(
            b"active local opening bytes" + seed_index.to_bytes(4, "little")
        )
        active_condition_b = one_bit_flip(active_condition_a, 7)
        forbidden_comp_a = sha256(
            seed + active_condition_a + b"forbidden-active-composition"
        )
        forbidden_comp_b = sha256(
            seed + active_condition_b + b"forbidden-active-composition"
        )
        unchanged = projection == pad_projection_values(seed)[0]
        forbidden_condition_changed_count += int(not unchanged)
        if seed_index < 8:
            active_condition_rows.append(
                {
                    "seed_index": seed_index,
                    "active_condition_hamming_bits": hamming_bytes(
                        active_condition_a, active_condition_b
                    ),
                    "forbidden_composed_digest_hamming_bits": hamming_bytes(
                        forbidden_comp_a, forbidden_comp_b
                    ),
                    "pad_projection_unchanged_for_fixed_seed": unchanged,
                    "classification": "forbidden active-condition bytes are absent from pad PRF input",
                }
            )

    malformed_seed_cases = [
        ("seed42", [0x42] * 32, True),
        ("short_31", [0x42] * 31, False),
        ("long_33", [0x42] * 33, False),
        ("non_uint8_high", [256] + [0x42] * 31, False),
        ("non_uint8_negative", [-1] + [0x42] * 31, False),
    ]
    malformed_seed_rows = []
    for name, material, expected_accept in malformed_seed_cases:
        decoded = decode_pad_seed(material)
        malformed_seed_rows.append(
            {
                "case": name,
                "decoded": decoded is not None,
                "expected_decoded": expected_accept,
                "status": "pass" if (decoded is not None) == expected_accept else "fail",
            }
        )

    zero_counter_projection, zero_counter_details = pad_projection_values(
        seed42, counter_limit=0
    )
    mutated_lane_projection = list(SEED42_PROJECTION_VECTOR)
    mutated_lane_projection[0] = (
        PROJECTION_LANE_COEFFICIENTS[6] * SEED42_BLOCK_RESIDUES[2][0]
    ) % Q

    checks = [
        {
            "id": "seed42_projection_matches_rust_lean",
            "status": "pass" if seed42_projection == SEED42_PROJECTION_VECTOR else "fail",
            "observed": seed42_projection,
            "expected": SEED42_PROJECTION_VECTOR,
        },
        {
            "id": "seed42_block_residues_match_rust_lean",
            "status": "pass"
            if all(row["status"] == "pass" for row in seed42_block_rows)
            else "fail",
            "rows": seed42_block_rows,
        },
        {
            "id": "bounded_counter_exhaustion_absent_in_sweep",
            "status": "pass" if not exhausted_examples and projection_failures == 0 else "fail",
            "exhausted_block_count": len(exhausted_examples),
            "projection_failure_count": projection_failures,
        },
        {
            "id": "zero_counter_limit_exhausts_fail_closed",
            "status": "pass" if zero_counter_projection is None else "fail",
            "details": zero_counter_details,
        },
        {
            "id": "domain_mutation_changes_sampled_projection_vectors",
            "status": "pass" if domain_collisions == 0 else "warn",
            "sampled_domain_collision_count": domain_collisions,
        },
        {
            "id": "malformed_seed_decode_behavior",
            "status": "pass"
            if all(row["status"] == "pass" for row in malformed_seed_rows)
            else "fail",
            "rows": malformed_seed_rows,
        },
        {
            "id": "mutated_projection_lane_map_differs",
            "status": "pass" if mutated_lane_projection != SEED42_PROJECTION_VECTOR else "fail",
            "mutated_lane_projection": mutated_lane_projection,
            "expected_projection": SEED42_PROJECTION_VECTOR,
        },
        {
            "id": "message_slot_independence_control",
            "status": "pass"
            if message_slot_invariant_count == min(seed_count, 128)
            else "fail",
            "unchanged_count": message_slot_invariant_count,
            "sample_count": min(seed_count, 128),
        },
        {
            "id": "forbidden_active_condition_not_composed",
            "status": "pass" if forbidden_condition_changed_count == 0 else "fail",
            "changed_count": forbidden_condition_changed_count,
            "sample_count": min(seed_count, 128),
        },
    ]

    failed_checks = [check["id"] for check in checks if check["status"] == "fail"]

    uniformity = {
        "all_residues_mod_64": bucket_uniformity(all_residues, 64),
        "all_residues_mod_251": bucket_uniformity(all_residues, 251),
        "projection_values_mod_64": bucket_uniformity(all_projection_values, 64),
        "projection_values_mod_251": bucket_uniformity(all_projection_values, 251),
        "residue_sample_count": len(all_residues),
        "projection_value_sample_count": len(all_projection_values),
        "note": "Bucket tests are sanity screens only; sparse q=12289 full-residue testing is intentionally not treated as proof.",
    }

    payload: dict[str, Any] = {
        "schema": "Descender.pre_third_party_pad_prf_seed_sweep.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "worker": "PTC-004",
        "campaign": "pad_prf_and_sampler_independence",
        "status": "pass" if not failed_checks else "fail",
        "failed_checks": failed_checks,
        "parameters": {
            "q": Q,
            "groups": GROUPS,
            "local_free": LOCAL_FREE,
            "pad_seed_bytes": PAD_SEED_BYTES,
            "pad_domain": PAD_DOMAIN.decode(),
            "pad_domain_len": len(PAD_DOMAIN),
            "pad_frame_bytes": PAD_FRAME_BYTES,
            "counter_encoding": "u32_le",
            "counter_limit": PAD_PRF_COUNTER_LIMIT,
            "rejection_cutoff": REJECTION_CUTOFF,
            "seed_count": seed_count,
            "block_count_per_seed": GROUPS,
            "projection_lane_map": PAD_PROJECTION_LANE_MAP,
            "projection_symbols": PAD_PROJECTION_SYMBOLS,
        },
        "source_references": {
            "rust_reference": rel(ROOT / "tools/parameter-search/src/production_reference.rs"),
            "rust_test_facade": rel(ROOT / "tools/production-reference/src/lib.rs"),
            "lean_reference": rel(ROOT / "Descender/HiddenSourceBucket1.lean"),
            "boundary_memo": rel(
                READINESS_DIR / "pad_sampler_boundary_memo_n32_q12289.json"
            ),
            "boundary_review": rel(
                READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json"
            ),
        },
        "checks": checks,
        "uniformity_sanity": uniformity,
        "counter_exhaustion": {
            "counter_histogram": dict(sorted(counter_hist.items())),
            "rejected_word_histogram": dict(sorted(rejection_hist.items())),
            "exhausted_examples": exhausted_examples[:8],
            "bounded_success_rate": (
                (seed_count * GROUPS - len(exhausted_examples)) / (seed_count * GROUPS)
            ),
        },
        "domain_separation": {
            "mutated_domain": domain_mutation.decode(errors="replace"),
            "sample_count": min(seed_count, 256),
            "collision_count": domain_collisions,
            "sample_rows": domain_sep_rows,
        },
        "avalanche": {
            "sample_count": min(seed_count, 256),
            "seed_bit_digest_hamming_min": min(seed_hamming_values),
            "seed_bit_digest_hamming_max": max(seed_hamming_values),
            "seed_bit_digest_hamming_mean": sum(seed_hamming_values)
            / len(seed_hamming_values),
            "block_frame_digest_hamming_min": min(block_hamming_values),
            "block_frame_digest_hamming_max": max(block_hamming_values),
            "block_frame_digest_hamming_mean": sum(block_hamming_values)
            / len(block_hamming_values),
            "projection_lanes_changed_min": min(projection_distances),
            "projection_lanes_changed_max": max(projection_distances),
            "projection_lanes_changed_mean": sum(projection_distances)
            / len(projection_distances),
            "sample_rows": avalanche_rows,
            "message_slot_control": {
                "sample_count": min(seed_count, 128),
                "unchanged_count": message_slot_invariant_count,
                "sample_rows": message_slot_controls,
                "interpretation": "The reviewed pad PRF frame does not include message or slot. Message/slot mutation is therefore an independence control, not a PRF avalanche claim.",
            },
        },
        "independence_sanity": {
            "seed_index_vs_first_projection_correlation": correlation(
                [float(index) for index in range(seed_count)],
                [
                    float(pad_projection_values(seed)[0][0])
                    for seed in seeds
                    if pad_projection_values(seed)[0] is not None
                ],
            ),
            "forbidden_active_condition_control": {
                "sample_count": min(seed_count, 128),
                "changed_count": forbidden_condition_changed_count,
                "sample_rows": active_condition_rows,
            },
        },
        "malformed_seed_behavior": malformed_seed_rows,
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
        "non_claims": [
            "This does not prove SHA-256 is a PRF.",
            "This does not replace the Lean bounded-total semantics.",
            "This does not test constant-time behavior.",
            "Message and slot are tested as independence controls because they are not pad-PRF frame inputs.",
        ],
    }
    return payload


def markdown(payload: dict[str, Any]) -> str:
    params = payload["parameters"]
    uniformity = payload["uniformity_sanity"]
    avalanche = payload["avalanche"]
    counter = payload["counter_exhaustion"]
    lines = [
        "# Pre-Third-Party Pad PRF Seed Sweep",
        "",
        "This PTC-004 artifact is a deterministic differential probe for the",
        "reviewed pad PRF framing and sampler-independence boundary. It is a",
        "sanity harness, not a proof of SHA-256 pseudorandomness.",
        "",
        "## Verdict",
        "",
        f"- status: `{payload['status']}`",
        f"- failed checks: `{len(payload['failed_checks'])}`",
        f"- seed count: `{params['seed_count']}`",
        f"- q: `{params['q']}`",
        f"- pad domain: `{params['pad_domain']}`",
        f"- counter limit: `{params['counter_limit']}`",
        f"- rejection cutoff: `{params['rejection_cutoff']}`",
        "",
        "## Checks",
        "",
        "| check | status |",
        "|---|---|",
    ]
    for check in payload["checks"]:
        lines.append(f"| `{check['id']}` | `{check['status']}` |")
    lines.extend(
        [
            "",
            "## Uniformity Sanity",
            "",
            f"- residue samples: `{uniformity['residue_sample_count']}`",
            f"- projection value samples: `{uniformity['projection_value_sample_count']}`",
            "- residue mod-64 min/max bucket: "
            f"`{uniformity['all_residues_mod_64']['min_bucket_count']}` / "
            f"`{uniformity['all_residues_mod_64']['max_bucket_count']}`",
            "- residue mod-251 normalized chi-square: "
            f"`{uniformity['all_residues_mod_251']['normalized_chi_square']:.4f}`",
            "- projection mod-64 min/max bucket: "
            f"`{uniformity['projection_values_mod_64']['min_bucket_count']}` / "
            f"`{uniformity['projection_values_mod_64']['max_bucket_count']}`",
            "- projection mod-251 normalized chi-square: "
            f"`{uniformity['projection_values_mod_251']['normalized_chi_square']:.4f}`",
            "",
            "## Counter Exhaustion",
            "",
            f"- bounded success rate: `{counter['bounded_success_rate']:.6f}`",
            f"- exhausted blocks in sweep: `{len(counter['exhausted_examples'])}`",
            f"- counter histogram: `{counter['counter_histogram']}`",
            "",
            "## Differential Controls",
            "",
            "- seed-bit digest hamming mean: "
            f"`{avalanche['seed_bit_digest_hamming_mean']:.2f}` bits",
            "- block-frame digest hamming mean: "
            f"`{avalanche['block_frame_digest_hamming_mean']:.2f}` bits",
            "- projection lanes changed mean under one seed-bit flip: "
            f"`{avalanche['projection_lanes_changed_mean']:.2f}` / 6",
            "- mutated-domain projection collisions: "
            f"`{payload['domain_separation']['collision_count']}`",
            "- message/slot fixed-seed unchanged count: "
            f"`{avalanche['message_slot_control']['unchanged_count']}` / "
            f"`{avalanche['message_slot_control']['sample_count']}`",
            "- forbidden active-condition changed count: "
            f"`{payload['independence_sanity']['forbidden_active_condition_control']['changed_count']}` / "
            f"`{payload['independence_sanity']['forbidden_active_condition_control']['sample_count']}`",
            "",
            "Message and slot are not pad-PRF frame inputs in the reviewed boundary,",
            "so their one-bit mutations are recorded as independence controls rather",
            "than avalanche claims.",
            "",
            "## Source References",
            "",
        ]
    )
    for key, value in payload["source_references"].items():
        lines.append(f"- `{key}`: `{value}`")
    lines.extend(["", "## Non-Claims", ""])
    for item in payload["non_claims"]:
        lines.append(f"- {item}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--seeds", type=int, default=4096)
    parser.add_argument("--json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md", type=Path, default=DEFAULT_MD)
    args = parser.parse_args()

    if args.seeds <= 0:
        raise SystemExit("--seeds must be positive")
    payload = build_payload(args.seeds)
    args.json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    args.md.write_text(markdown(payload))
    print(f"wrote {rel(args.json)}")
    print(f"wrote {rel(args.md)}")
    print(f"status {payload['status']}")


if __name__ == "__main__":
    main()
