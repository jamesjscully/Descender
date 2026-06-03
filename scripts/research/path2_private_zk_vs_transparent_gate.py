#!/usr/bin/env python3
"""Compare the private lattice-sigma branch against transparent fallback."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SIGMA_JSON = ROOT / "research/path2_descent_sis_lattice_sigma_toy_model.json"
OUT_JSON = ROOT / "research/path2_private_zk_vs_transparent_gate.json"
OUT_MD = ROOT / "research/path2_private_zk_vs_transparent_gate.md"


def load_sigma() -> dict[str, Any]:
    return json.loads(SIGMA_JSON.read_text())


def build_model() -> dict[str, Any]:
    sigma = load_sigma()
    plausible_sigma = [
        row for row in sigma["byte_floor_rows"] if row["repetitions"] >= 64
    ]
    best_plausible = min(plausible_sigma, key=lambda row: row["proof_bytes_floor"])
    transparent_floor = {
        "relation_witness_lanes": 80,
        "relation_rows": 66,
        "hash_commitment_bytes_floor": 32,
        "opening_path_bytes_floor_range": [1024, 4096],
        "proof_bytes_floor_range": [4096, 12288],
        "verifier_hash_or_field_ops_floor": "proof-system-dependent",
        "status": "fallback_needs_real_model",
    }
    recommendation = (
        "Do not reject Path 2, but stop treating the private lattice-sigma branch "
        "as obviously compact. Its optimistic 64-round floor already exceeds the "
        "transparent fallback's low/mid planning band. The next fair move is a "
        "real transparent-proof byte model and, in parallel, a search for a "
        "non-repetition-heavy lattice ZK argument."
    )
    return {
        "schema": "Descender.path2_private_zk_vs_transparent_gate.v1",
        "status": "comparison_gate_pinned",
        "private_sigma_best_plausible_floor": best_plausible,
        "transparent_fallback_planning_floor": transparent_floor,
        "decision": "compare_before_further_private_sigma_optimization",
        "recommendation": recommendation,
        "path_status": {
            "private_lattice_sigma": (
                "still viable as a theorem route, but not currently a compact "
                "payload route under repetition-heavy sigma assumptions"
            ),
            "transparent_hash_proof": (
                "now promoted from passive fallback to active comparator because "
                "it may be byte-competitive at account creation"
            ),
            "small_noise_public_lwe_style": (
                "kept as algebraic public-object fallback if transparent proof is "
                "too costly and private sigma remains too large"
            ),
        },
        "next_artifacts": [
            "transparent proof byte model for the exact private-ZK relation",
            "non-repetition-heavy lattice ZK argument scan",
            "decision memo selecting private sigma, transparent, or small-noise public fallback",
        ],
        "non_claims": [
            "transparent byte ranges are planning bands, not measured proof sizes",
            "private sigma floor omits rejection overhead and concrete proof-family overhead",
            "this does not close the theorem",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    sigma = model["private_sigma_best_plausible_floor"]
    transparent = model["transparent_fallback_planning_floor"]
    lines = [
        "# Path 2 Private-ZK vs Transparent Gate",
        "",
        "Status: comparison gate pinned.",
        "",
        "## Best Plausible Private Lattice-Sigma Floor",
        "",
        "| repetitions | first msg/round | response/round | proof floor | verifier q-ops floor |",
        "|---:|---:|---:|---:|---:|",
        "| {repetitions} | {first_message_bytes_per_round_floor} | "
        "{response_bytes_per_round_floor} | {proof_bytes_floor} | "
        "{verifier_q_ops_floor} |".format(**sigma),
        "",
        "## Transparent Fallback Planning Floor",
        "",
        "| witness lanes | relation rows | opening path bytes floor range | proof bytes floor range | status |",
        "|---:|---:|---|---|---|",
        "| {relation_witness_lanes} | {relation_rows} | {opening_path_bytes_floor_range} | "
        "{proof_bytes_floor_range} | {status} |".format(**transparent),
        "",
        "## Recommendation",
        "",
        model["recommendation"],
        "",
        "## Path Status",
        "",
    ]
    for name, status in model["path_status"].items():
        lines.append(f"- `{name}`: {status}")
    lines.extend(["", "## Next Artifacts", ""])
    for item in model["next_artifacts"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Non-Claims", ""])
    for item in model["non_claims"]:
        lines.append(f"- {item}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    model = build_model()
    OUT_JSON.write_text(json.dumps(model, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(write_markdown(model))
    print(json.dumps({"json": str(OUT_JSON), "markdown": str(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
