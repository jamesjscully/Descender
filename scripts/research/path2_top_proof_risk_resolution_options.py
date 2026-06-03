#!/usr/bin/env python3
"""Generate resolution options for the top Path 2 proof risks."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_top_proof_risk_resolution_options.json"
OUT_MD = ROOT / "research/path2_top_proof_risk_resolution_options.md"


LEAKAGE_OPTIONS = [
    {
        "id": "raw_transparent_openings",
        "verdict": "dead_path",
        "reason": (
            "Opening raw witness trace cells exposes pad-source digits, projection "
            "digits, opening-tuple digits, or range witnesses with non-negligible "
            "probability. This is incompatible with the live-slot hiding theorem."
        ),
        "proof_burden": "none; reject as theorem target",
    },
    {
        "id": "zk_transparent_iop_with_masked_witness_columns",
        "verdict": "preferred_path",
        "reason": (
            "Keep the transparent proof route, but require witness columns to be "
            "committed with zero-knowledge masking/blinding so verifier queries are "
            "simulatable. Public columns remain directly openable."
        ),
        "proof_burden": (
            "state a proof-family ZK/simulator theorem for committed_hidden and "
            "derived_hidden columns, plus domain-separated blinding randomness"
        ),
    },
    {
        "id": "make_every_opened_hidden_cell_one_time_masked",
        "verdict": "fallback_path",
        "reason": (
            "Can preserve a simpler query-opening story, but every hidden cell "
            "opened by the verifier must be paired with a fresh mask and an "
            "independent mask-consistency proof."
        ),
        "proof_burden": (
            "large relation expansion; prove masks are one-time and do not create "
            "reuse equations across slots, queries, or rows"
        ),
    },
]


DIGEST_OPTIONS = [
    {
        "id": "prove_sha256_preimages_in_relation",
        "verdict": "dead_path_for_compactness",
        "reason": (
            "The earlier cost model was SHA-circuit dominated. Paying for all "
            "pad/opening SHA preimages defeats the point of the compact creation "
            "payload."
        ),
        "theorem_change": "none, but byte/compute budget likely fails",
    },
    {
        "id": "outer_sha_binds_inner_proof_friendly_commitments",
        "verdict": "preferred_path",
        "reason": (
            "Use SHA-256 only to bind public consensus objects and data "
            "availability. The proof opens proof-friendly algebraic commitments "
            "inside Path2CreationProofBindingV1, whose SHA digest is bound by the "
            "leaf/certificate."
        ),
        "theorem_change": (
            "pad_seed_digest and opening_commitment are not claimed to be opened "
            "inside the 66-row relation; the theorem consumes the algebraic inner "
            "commitment openings plus outer SHA binding/availability"
        ),
    },
    {
        "id": "replace_outer_sha_fields_with_algebraic_commitments",
        "verdict": "fallback_path",
        "reason": (
            "Cleanest algebraically but disruptive to consensus framing and "
            "existing lifecycle/state objects."
        ),
        "theorem_change": "new leaf/state format, larger migration surface",
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_top_proof_risk_resolution_options.v1",
        "status": "top_risk_resolution_path_pinned_not_proved",
        "active_resolution": {
            "query_opening_leakage": "zk_transparent_iop_with_masked_witness_columns",
            "digest_to_algebra_gap": "outer_sha_binds_inner_proof_friendly_commitments",
        },
        "leakage_options": LEAKAGE_OPTIONS,
        "digest_options": DIGEST_OPTIONS,
        "resulting_theorem_shape": (
            "An accepted account-creation payload proves knowledge of openings to "
            "proof-friendly inner commitments in Path2CreationProofBindingV1. "
            "Outer SHA-256 digests bind canonical leaf/certificate/payload bytes "
            "and data availability, but the transparent arithmetic relation does "
            "not claim SHA-256 preimage knowledge except where explicitly paid for."
        ),
        "required_assumptions": [
            {
                "id": "transparent_zk_iop_simulator",
                "meaning": (
                    "query openings for committed_hidden and derived_hidden columns "
                    "are simulatable and reveal no raw pad/projection/opening cells"
                ),
            },
            {
                "id": "inner_commitment_binding",
                "meaning": (
                    "two accepted openings of the same proof-friendly commitment "
                    "export a named algebraic binding break"
                ),
            },
            {
                "id": "inner_commitment_hiding",
                "meaning": (
                    "public inner commitments and transparent proof messages hide "
                    "pad-source, projection, and opening-tuple witnesses"
                ),
            },
            {
                "id": "outer_sha_binding_availability",
                "meaning": (
                    "SHA-256 binds canonical public bytes and available payloads; "
                    "it is not used as a compact in-circuit preimage proof"
                ),
            },
        ],
        "immediate_next": (
            "Discharge the relation-layer split in "
            "research/path2_inner_commitment_relation_refinement.md: decide "
            "whether the two-hop trace binding audited in "
            "research/path2_trace_commitment_binding_audit.md is sufficient once "
            "the relation links trace columns to Path2CreationProofBindingV1 role "
            "commitments, or whether trace roots/explicit opening rows must be "
            "added to the binding model. The current preferred compact link path "
            "is recorded in research/path2_role_commitment_link_equation_options.md."
        ),
        "redesign_triggers": [
            "no transparent ZK IOP family can provide simulatable openings within the creation proof byte budget",
            "inner commitments cannot bind the opening tuple to account witness semantics without adding too many rows",
            "outer SHA binding is insufficient for validators to retrieve and authenticate the full proof-binding object",
        ],
        "non_claims": [
            "this does not prove the transparent ZK IOP simulator theorem",
            "this does not instantiate the inner commitment family",
            "this does not close the direct relation certificate theorem",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Top Proof Risk Resolution Options",
        "",
        "Status: top-risk resolution path pinned; not proved.",
        "",
        "## Active Resolution",
        "",
    ]
    for key, value in model["active_resolution"].items():
        lines.append(f"- `{key}`: `{value}`")
    lines.extend(["", "## Query-Opening Leakage Options", "", "| id | verdict | reason | proof burden |", "|---|---|---|---|"])
    for item in model["leakage_options"]:
        lines.append("| `{id}` | `{verdict}` | {reason} | {proof_burden} |".format(**item))
    lines.extend(["", "## Digest-To-Algebra Options", "", "| id | verdict | reason | theorem change |", "|---|---|---|---|"])
    for item in model["digest_options"]:
        lines.append("| `{id}` | `{verdict}` | {reason} | {theorem_change} |".format(**item))
    lines.extend(["", "## Resulting Theorem Shape", "", model["resulting_theorem_shape"], "", "## Required Assumptions", "", "| id | meaning |", "|---|---|"])
    for item in model["required_assumptions"]:
        lines.append("| `{id}` | {meaning} |".format(**item))
    lines.extend(["", "## Immediate Next", "", model["immediate_next"], "", "## Redesign Triggers", ""])
    for item in model["redesign_triggers"]:
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
