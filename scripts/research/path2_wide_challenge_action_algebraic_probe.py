#!/usr/bin/env python3
"""Algebraic soundness probe for Path 2 wide challenge-action routes."""

from __future__ import annotations

import json
import math
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
JSON_OUT = OUT_DIR / "path2_wide_challenge_action_algebraic_probe_n32_q12289.json"
MD_OUT = OUT_DIR / "path2_wide_challenge_action_algebraic_probe_n32_q12289.md"
AUDIT_OUT = ROOT / "audit/active_path_wide_challenge_action_algebraic_soundness.md"

BLOCKS = 16
REPS = 26
MULTIFORK_DRAWS = 6
LOCAL_FREE = 2
ETA = 12
CURRENT_RESPONSE_BOUND = 48


def bits(probability: float) -> float:
    if probability <= 0:
        return math.inf
    return -math.log2(probability)


def coefficient_subset_safe_probability(subset_blocks: int, coeff_count: int) -> float:
    """One-repetition probability no block in a fixed subset gets two coefficients.

    Draws are independent active compact challenge draws. Each draw chooses one
    of `BLOCKS` blocks and one of `coeff_count` coefficients. The compact
    extraction event for a block is two same-block openings with different
    coefficients; the DP keeps only how many subset blocks have one coefficient
    already observed.
    """
    dp = [0.0] * (subset_blocks + 1)
    dp[0] = 1.0
    for _draw in range(MULTIFORK_DRAWS):
        next_dp = [0.0] * (subset_blocks + 1)
        for observed, mass in enumerate(dp):
            if mass == 0:
                continue
            next_dp[observed] += mass * ((BLOCKS - subset_blocks) / BLOCKS)
            next_dp[observed] += mass * (observed / (BLOCKS * coeff_count))
            if observed < subset_blocks:
                next_dp[observed + 1] += mass * ((subset_blocks - observed) / BLOCKS)
        dp = next_dp
    return sum(dp)


def coefficient_any_block_miss(coeff_count: int) -> dict[str, Any]:
    probability = 0.0
    terms = []
    for subset_blocks in range(1, BLOCKS + 1):
        one_rep_safe = coefficient_subset_safe_probability(subset_blocks, coeff_count)
        subset_safe = one_rep_safe**REPS
        term = math.comb(BLOCKS, subset_blocks) * subset_safe
        if subset_blocks % 2 == 1:
            probability += term
            signed = term
        else:
            probability -= term
            signed = -term
        terms.append(
            {
                "subset_blocks": subset_blocks,
                "one_rep_safe_probability": one_rep_safe,
                "all_reps_safe_probability": subset_safe,
                "inclusion_exclusion_term": signed,
            }
        )
    return {
        "coefficient_count": coeff_count,
        "any_block_miss_probability": probability,
        "any_block_miss_bits": bits(probability),
        "terms": terms,
    }


def response_bound_model(max_abs_coeff: int) -> dict[str, Any]:
    shift_bound = max_abs_coeff * ETA
    exact_box_mask_bound = CURRENT_RESPONSE_BOUND + shift_bound
    return {
        "max_abs_coeff": max_abs_coeff,
        "secret_shift_bound_if_secret_eta": shift_bound,
        "current_response_bound": CURRENT_RESPONSE_BOUND,
        "exact_box_mask_bound_for_same_response_bound": exact_box_mask_bound,
        "response_bound_status": (
            "unchanged one-byte response bound can be kept only if the signer "
            "samples masks from the enlarged interval and proves exact or "
            "bounded-leakage rejection for every shifted coefficient"
        ),
    }


def payload() -> dict[str, Any]:
    coeff_rows = [coefficient_any_block_miss(c) for c in (2, 4, 8, 16)]
    return {
        "schema": "Descender.path2_wide_challenge_action_algebraic_probe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "parameters": {
            "blocks": BLOCKS,
            "repetitions": REPS,
            "multifork_draws_per_commitment": MULTIFORK_DRAWS,
            "local_free_coordinates": LOCAL_FREE,
            "secret_eta_assumption": ETA,
            "current_response_bound": CURRENT_RESPONSE_BOUND,
        },
        "routes": [
            {
                "id": "auxiliary_fq12_action_bits",
                "description": (
                    "Append canonical auxiliary challenge bits after the existing "
                    "130 compact-local bits. The Fq12 checked-core action consumes "
                    "the whole longer challenge byte string while the compact local "
                    "block/sign decoder keeps its current bit offsets."
                ),
                "algebraic_status": "highest probability of proof closure",
                "why": [
                    "native checked-core acceptance already computes challenge = H(frame)",
                    "Fq12 action hashes the whole challenge byte string into 12 q-residues",
                    "appending bits after the compact-local prefix preserves the 26 existing 5-bit block/sign fields",
                    "compact local response/opening semantics can remain 4-bit block plus 1-bit sign",
                    "does not multiply active response coordinates by larger local coefficients",
                ],
                "new_obligations": [
                    "update Lean/Rust challenge-byte constants and canonical high-bit mask",
                    "prove the Fq12 action-collision and QROM budget for the longer byte string",
                    "prove extra bits are bound into the selected checked-core frame and are not ignored by the Fq12 sigma equation",
                    "keep any older account-carried compact-local verifier either updated or explicitly non-active",
                ],
            },
            {
                "id": "wide_compact_coefficient_alphabet",
                "description": (
                    "Decode 7 bits as 4-bit block index plus an 8-value nonzero "
                    "coefficient alphabet for the compact local equation."
                ),
                "algebraic_status": "possible but high redesign risk",
                "why": [
                    "product-block verifier equation supports arbitrary q scalars",
                    "current noise-preservation theorem only covers {-1,0,1}",
                    "Rust challenge_mul rejects every coefficient except -1, 0, 1",
                    "larger coefficients stress the [-48,48] one-byte response bound",
                    "fixed-repetition compact multifork coverage remains weak for weight-one active blocks",
                ],
                "coefficient_fork_coverage": coeff_rows,
                "response_bound_models": [
                    response_bound_model(1),
                    response_bound_model(2),
                    response_bound_model(4),
                ],
            },
            {
                "id": "larger_block_universe",
                "description": "Decode 6 or 7 index bits as 64 or 128 possible active blocks.",
                "algebraic_status": "not a local patch",
                "why": [
                    "current public key, block-diagonal view, pad openings, and Lean block type are 16-block objects",
                    "changing the block count rewrites public-key size, coverage, local state, and generated certificates",
                    "does not preserve the fixed active response/opening byte accounting in the current theorem",
                ],
            },
        ],
        "recommendation": {
            "primary": "auxiliary_fq12_action_bits",
            "reason": (
                "It addresses the QROM/multi-target horizon with the smallest "
                "algebraic blast radius: the active native checked-core verifier "
                "already consumes whole challenge bytes through the Fq12 action, "
                "while compact local block/sign semantics and response bounds can stay unchanged."
            ),
            "do_not_commit_to": "wide_compact_coefficient_alphabet",
            "blocker": (
                "It needs new coefficient-noise theorems, Rust verifier semantics, "
                "signer/rejection-sampling proofs, and still does not obviously buy "
                "useful compact multifork coverage at fixed 26 reps."
            ),
        },
    }


def markdown(data: dict[str, Any]) -> str:
    params = data["parameters"]
    wide = next(route for route in data["routes"] if route["id"] == "wide_compact_coefficient_alphabet")
    lines = [
        "# Active Path Wide Challenge-Action Algebraic Soundness",
        "",
        f"- generated at: `{data['generated_at']}`",
        f"- blocks: `{params['blocks']}`",
        f"- repetitions: `{params['repetitions']}`",
        f"- multifork draws per commitment: `{params['multifork_draws_per_commitment']}`",
        f"- current response bound: `[-{params['current_response_bound']}, {params['current_response_bound']}]`",
        "",
        "## Finding",
        "",
        "There are two different challenge-action surfaces in the active Path 2/Fq12 stack.",
        "The compact local surface decodes each repetition as a 4-bit block index and a 1-bit sign.",
        "The checked-core Fq12 surface hashes the whole canonical challenge byte string into a 12-residue field action.",
        "",
        "The realistic widening route is to add auxiliary canonical challenge bits consumed by the Fq12 action, while leaving the compact local block/sign semantics unchanged.",
        "Changing the compact local coefficient alphabet is a much larger redesign.",
        "",
        "## Route Assessment",
        "",
    ]
    for route in data["routes"]:
        lines.extend([f"### {route['id']}", "", f"- status: `{route['algebraic_status']}`", f"- description: {route['description']}"])
        for reason in route["why"]:
            lines.append(f"- {reason}")
        if route["id"] == "auxiliary_fq12_action_bits":
            lines.append("")
            lines.append("New obligations:")
            for item in route["new_obligations"]:
                lines.append(f"- {item}")
        lines.append("")
    lines.extend(
        [
            "## Coefficient-Alphabet Probe",
            "",
            "This measures only the compact local multifork event for a weight-one active block design.",
            "A block is extractable in one repetition if two same-commitment draws hit that block with different coefficients.",
            "",
            "| coefficient choices | any-block miss probability | miss bits |",
            "|---:|---:|---:|",
        ]
    )
    for row in wide["coefficient_fork_coverage"]:
        lines.append(
            f"| {row['coefficient_count']} | {row['any_block_miss_probability']:.12f} | "
            f"{row['any_block_miss_bits']:.6f} |"
        )
    lines.extend(
        [
            "",
            "The result is bad for coefficient widening as a compact-extraction strategy: even 8 coefficients at 26 repetitions and 6 draws gives only about 0.0016 bits against any missed block.",
            "",
            "## Response-Bound Pressure",
            "",
            "| max coefficient | shift bound if secret eta=12 | exact-box mask bound for same response bound |",
            "|---:|---:|---:|",
        ]
    )
    for row in wide["response_bound_models"]:
        lines.append(
            f"| {row['max_abs_coeff']} | {row['secret_shift_bound_if_secret_eta']} | "
            f"{row['exact_box_mask_bound_for_same_response_bound']} |"
        )
    lines.extend(
        [
            "",
            "The one-byte response bound can remain at 48 only if the signer samples from a larger mask interval and proves exact rejection or bounded leakage for every coefficient-induced shift.",
            "",
            "## Recommendation",
            "",
            f"- primary route: `{data['recommendation']['primary']}`",
            f"- reason: {data['recommendation']['reason']}",
            f"- do not commit to: `{data['recommendation']['do_not_commit_to']}`",
            f"- blocker: {data['recommendation']['blocker']}",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    data = payload()
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    JSON_OUT.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")
    text = markdown(data)
    MD_OUT.write_text(text)
    AUDIT_OUT.write_text(text)
    print(
        json.dumps(
            {
                "json": str(JSON_OUT),
                "markdown": str(MD_OUT),
                "audit": str(AUDIT_OUT),
                "recommendation": data["recommendation"]["primary"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
