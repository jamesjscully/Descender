#!/usr/bin/env python3
"""Statistical public-view probe for the proposed 18-residue witness state.

This complements the linear leakage probe.  It asks whether the six account
projection residues can be treated as uniform-looking public rows, or whether
they look like small centered witness coordinates when prepublished in account
state.
"""

from __future__ import annotations

import hashlib
import json
import math
import random
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_signature_projection_statistical_probe_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_signature_projection_statistical_probe_n32_q12289.md"

Q = 12_289
SAMPLES = 20_000
SLOTS_PER_ACCOUNT = 8
SECRET_ETA = 12
SYNDROME_ROWS = 12
PROJECTION_ROWS = 6
MASK_DOMAIN = b"Descender.Signature.q12289.local_delta.reviewed_fq12_packed_witness_state18_projection_mask.v1"
SELECTED_ACCOUNT_PROJECTION_SYMBOLS = [
    "account_witness_projection[1,0]",
    "account_witness_projection[4,1]",
    "account_witness_projection[7,0]",
    "account_witness_projection[10,1]",
    "account_witness_projection[13,0]",
    "account_witness_projection[0,1]",
]


def centered(value: int, q: int = Q) -> int:
    value %= q
    return value - q if value > q // 2 else value


def sha_u64(label: bytes, idx: int) -> int:
    digest = hashlib.sha256(label + idx.to_bytes(8, "little")).digest()
    return int.from_bytes(digest[:8], "little")


def uniform_mod_from_hash(domain: bytes, seed: bytes, stream: int, index: int, modulus: int = Q) -> int:
    cutoff = (1 << 16) // modulus * modulus
    counter = 0
    while True:
        digest = hashlib.sha256(
            domain
            + seed
            + stream.to_bytes(8, "little")
            + index.to_bytes(8, "little")
            + counter.to_bytes(4, "little")
        ).digest()
        for offset in range(0, len(digest), 2):
            candidate = int.from_bytes(digest[offset : offset + 2], "little")
            if candidate < cutoff:
                return candidate % modulus
        counter += 1


def sample_uniform_residue(rng: random.Random) -> int:
    return rng.randrange(Q)


def sample_centered_secret_residue(rng: random.Random, eta: int = SECRET_ETA) -> int:
    return rng.randint(-eta, eta) % Q


def pad_seed_for_account_slot(account: int, slot: int) -> bytes:
    return hashlib.sha256(
        b"Descender.pre_signature_projection_statistical_probe.pad_seed"
        + account.to_bytes(8, "little")
        + slot.to_bytes(8, "little")
    ).digest()


def masked_projection_residue(raw: int, pad_seed: bytes, slot: int, row: int) -> int:
    seed = pad_seed + slot.to_bytes(8, "little")
    mask = uniform_mod_from_hash(MASK_DOMAIN, seed, slot, row)
    return (raw + mask) % Q


def summarize_residues(values: list[int], small_bound: int = SECRET_ETA) -> dict[str, Any]:
    centered_values = [centered(value) for value in values]
    n = len(values)
    small = sum(1 for value in centered_values if abs(value) <= small_bound)
    zero = sum(1 for value in centered_values if value == 0)
    mean = sum(centered_values) / n
    variance = sum((value - mean) ** 2 for value in centered_values) / n
    bins = 32
    counts = Counter(min(bins - 1, value * bins // Q) for value in values)
    expected = n / bins
    chi_square = sum((counts[idx] - expected) ** 2 / expected for idx in range(bins))
    return {
        "samples": n,
        "small_bound": small_bound,
        "small_centered_count": small,
        "small_centered_fraction": small / n,
        "zero_count": zero,
        "zero_fraction": zero / n,
        "mean_centered": mean,
        "stddev_centered": math.sqrt(variance),
        "max_abs_centered": max(abs(value) for value in centered_values),
        "distinct_residues": len(set(values)),
        "coarse_32_bin_chi_square": chi_square,
        "coarse_32_bin_counts": [counts[idx] for idx in range(bins)],
    }


def build_samples(model: str, samples: int = SAMPLES, slots_per_account: int = SLOTS_PER_ACCOUNT) -> dict[str, Any]:
    rng = random.Random(sha_u64(b"Descender.pre_signature_projection_statistical_probe", 0))
    syndrome_values: list[int] = []
    projection_values: list[int] = []
    per_account_small_projection_counts: list[int] = []
    for account in range(samples):
        small_projection_count = 0
        for slot in range(slots_per_account):
            for _ in range(SYNDROME_ROWS):
                syndrome_values.append(sample_uniform_residue(rng))
            pad_seed = pad_seed_for_account_slot(account, slot)
            for row in range(PROJECTION_ROWS):
                if model == "uniform_projection_rows":
                    value = sample_uniform_residue(rng)
                elif model == "centered_secret_projection_rows":
                    value = sample_centered_secret_residue(rng)
                elif model == "pad_seed_masked_centered_projection_rows":
                    raw = sample_centered_secret_residue(rng)
                    value = masked_projection_residue(raw, pad_seed, slot, row)
                else:
                    raise ValueError(f"unknown model: {model}")
                projection_values.append(value)
                if abs(centered(value)) <= SECRET_ETA:
                    small_projection_count += 1
        per_account_small_projection_counts.append(small_projection_count)
    projection_summary = summarize_residues(projection_values)
    syndrome_summary = summarize_residues(syndrome_values)
    total_projection_per_account = slots_per_account * PROJECTION_ROWS
    all_projection_small_accounts = sum(
        1 for count in per_account_small_projection_counts if count == total_projection_per_account
    )
    any_projection_small_accounts = sum(
        1 for count in per_account_small_projection_counts if count > 0
    )
    return {
        "model": model,
        "accounts_sampled": samples,
        "slots_per_account": slots_per_account,
        "projection_rows_per_slot": PROJECTION_ROWS,
        "syndrome_rows_per_slot": SYNDROME_ROWS,
        "projection_summary": projection_summary,
        "syndrome_summary": syndrome_summary,
        "per_account_small_projection": {
            "total_projection_values_per_account": total_projection_per_account,
            "all_projection_values_small_accounts": all_projection_small_accounts,
            "all_projection_values_small_fraction": all_projection_small_accounts / samples,
            "any_projection_value_small_accounts": any_projection_small_accounts,
            "any_projection_value_small_fraction": any_projection_small_accounts / samples,
            "mean_small_projection_values_per_account": sum(
                per_account_small_projection_counts
            )
            / samples,
        },
    }


def build_payload() -> dict[str, Any]:
    uniform = build_samples("uniform_projection_rows")
    centered_model = build_samples("centered_secret_projection_rows")
    masked_model = build_samples("pad_seed_masked_centered_projection_rows")
    uniform_small = uniform["projection_summary"]["small_centered_fraction"]
    centered_small = centered_model["projection_summary"]["small_centered_fraction"]
    masked_small = masked_model["projection_summary"]["small_centered_fraction"]
    distinguisher_advantage = centered_small - uniform_small
    severe_distinguisher = (
        centered_model["projection_summary"]["max_abs_centered"] <= SECRET_ETA
        and uniform["projection_summary"]["small_centered_fraction"] < 0.01
    )
    masked_close_to_uniform = abs(masked_small - uniform_small) < 0.001
    return {
        "schema": "Descender.pre_signature_projection_statistical_probe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "pre_signature_projection_statistical_probe_completed_fail_closed",
        "deployment_ready": False,
        "q": Q,
        "samples": SAMPLES,
        "slots_per_account": SLOTS_PER_ACCOUNT,
        "secret_eta": SECRET_ETA,
        "selected_account_projection_symbols": SELECTED_ACCOUNT_PROJECTION_SYMBOLS,
        "models": {
            "uniform_projection_rows": uniform,
            "centered_secret_projection_rows": centered_model,
            "pad_seed_masked_centered_projection_rows": masked_model,
        },
        "distinguisher": {
            "test": f"all projection residues are centered within +/-{SECRET_ETA}",
            "uniform_projection_small_fraction": uniform_small,
            "centered_secret_projection_small_fraction": centered_small,
            "pad_seed_masked_projection_small_fraction": masked_small,
            "small_fraction_gap": distinguisher_advantage,
            "masked_vs_uniform_small_fraction_gap": masked_small - uniform_small,
            "uniform_all_projection_values_small_fraction": uniform[
                "per_account_small_projection"
            ]["all_projection_values_small_fraction"],
            "centered_all_projection_values_small_fraction": centered_model[
                "per_account_small_projection"
            ]["all_projection_values_small_fraction"],
            "pad_seed_masked_all_projection_values_small_fraction": masked_model[
                "per_account_small_projection"
            ]["all_projection_values_small_fraction"],
            "severe_if_projection_rows_are_centered_secret_coordinates": severe_distinguisher,
            "pad_seed_masked_rows_close_to_uniform_in_this_probe": masked_close_to_uniform,
        },
        "interpretation": {
            "uniform_like_projection_rows": (
                "No immediate marginal distinguisher in this bounded probe; "
                "linear leakage remains the main charged cost."
            ),
            "centered_secret_projection_rows": (
                "Prepublishing selected projection coordinates is trivially "
                "distinguishable if those coordinates are small centered witness "
                "coordinates; the account-state view would reveal all selected "
                "projection coordinates for every live slot."
            ),
            "pad_seed_masked_centered_projection_rows": (
                "Domain-separated pad-seed masks make the public projection "
                "residues match the uniform-looking marginal test in this probe; "
                "this still needs a proof that the mask seed is precommitted, "
                "single-use, domain-separated from pad PRF lanes, and opened only "
                "through the checked state-assisted tuple."
            ),
            "next_gate": (
                "Before promoting the masked 18-residue model, prove the "
                "pad-seed-derived mask is independent of the raw account projection "
                "values under the final state lifecycle and does not introduce "
                "correlation with the pad projection lanes."
            ),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    uniform = payload["models"]["uniform_projection_rows"]
    centered_model = payload["models"]["centered_secret_projection_rows"]
    masked_model = payload["models"]["pad_seed_masked_centered_projection_rows"]
    dist = payload["distinguisher"]
    lines = [
        "# Pre-Signature Projection Statistical Probe",
        "",
        "This probe tests the timing risk of moving six account projection",
        "residues into public account state. It is fail-closed and separates",
        "two models: uniform-looking projection residues and small centered",
        "witness-coordinate residues.",
        "",
        "## Configuration",
        "",
        f"- q: `{payload['q']}`",
        f"- accounts sampled: `{payload['samples']}`",
        f"- slots per account: `{payload['slots_per_account']}`",
        f"- secret eta for centered model: `{payload['secret_eta']}`",
        f"- selected symbols: `{', '.join(payload['selected_account_projection_symbols'])}`",
        "",
        "## Marginal Projection Statistics",
        "",
        "| model | projection samples | small centered fraction | max abs centered | distinct residues | chi-square 32 bins |",
        "|---|---:|---:|---:|---:|---:|",
        f"| uniform projection rows | {uniform['projection_summary']['samples']} | {uniform['projection_summary']['small_centered_fraction']:.6f} | {uniform['projection_summary']['max_abs_centered']} | {uniform['projection_summary']['distinct_residues']} | {uniform['projection_summary']['coarse_32_bin_chi_square']:.2f} |",
        f"| centered secret projection rows | {centered_model['projection_summary']['samples']} | {centered_model['projection_summary']['small_centered_fraction']:.6f} | {centered_model['projection_summary']['max_abs_centered']} | {centered_model['projection_summary']['distinct_residues']} | {centered_model['projection_summary']['coarse_32_bin_chi_square']:.2f} |",
        f"| pad-seed masked centered projection rows | {masked_model['projection_summary']['samples']} | {masked_model['projection_summary']['small_centered_fraction']:.6f} | {masked_model['projection_summary']['max_abs_centered']} | {masked_model['projection_summary']['distinct_residues']} | {masked_model['projection_summary']['coarse_32_bin_chi_square']:.2f} |",
        "",
        "## Multi-Slot Distinguisher",
        "",
        f"- test: `{dist['test']}`",
        f"- uniform small-centered fraction: `{dist['uniform_projection_small_fraction']:.6f}`",
        f"- centered-secret small-centered fraction: `{dist['centered_secret_projection_small_fraction']:.6f}`",
        f"- pad-seed masked small-centered fraction: `{dist['pad_seed_masked_projection_small_fraction']:.6f}`",
        f"- uniform all projection values small per account: `{dist['uniform_all_projection_values_small_fraction']:.6f}`",
        f"- centered all projection values small per account: `{dist['centered_all_projection_values_small_fraction']:.6f}`",
        f"- pad-seed masked all projection values small per account: `{dist['pad_seed_masked_all_projection_values_small_fraction']:.6f}`",
        f"- severe if projection rows are centered secret coordinates: `{dist['severe_if_projection_rows_are_centered_secret_coordinates']}`",
        f"- pad-seed masked rows close to uniform in this probe: `{dist['pad_seed_masked_rows_close_to_uniform_in_this_probe']}`",
        "",
        "## Interpretation",
        "",
        f"- uniform-like projection rows: {payload['interpretation']['uniform_like_projection_rows']}",
        f"- centered secret projection rows: {payload['interpretation']['centered_secret_projection_rows']}",
        f"- pad-seed masked centered projection rows: {payload['interpretation']['pad_seed_masked_centered_projection_rows']}",
        f"- next gate: {payload['interpretation']['next_gate']}",
        "",
    ]
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
                "severe_centered_secret_distinguisher": payload["distinguisher"][
                    "severe_if_projection_rows_are_centered_secret_coordinates"
                ],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
