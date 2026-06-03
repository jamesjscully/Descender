#!/usr/bin/env python3
"""Generate the Path 2 transparent proof-family gate."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_transparent_proof_family_gate.json"
OUT_MD = ROOT / "research/path2_transparent_proof_family_gate.md"


FAMILIES = [
    {
        "id": "vanilla_merkle_fri_trace",
        "verdict": "dead_for_hidden_columns",
        "description": (
            "Commit a trace with Merkle roots and open raw trace cells plus FRI/"
            "consistency paths at verifier-selected positions."
        ),
        "api_fit": {
            "parent_root_binding": "yes",
            "role_subcommitment_binding": "possible_with_column_subroots",
            "hidden_column_zk": "no",
            "query_schedule_binding": "yes",
        },
        "why": (
            "It can bind a trace, but sampled openings reveal pad-source, "
            "projection, opening-tuple, digit/range, or blinding lanes unless the "
            "trace is redesigned. This fails the no-export theorem."
        ),
        "next_action": "keep only as a negative control and byte-model baseline",
    },
    {
        "id": "zk_masked_trace_iop",
        "verdict": "primary_candidate",
        "description": (
            "Use a transparent IOP whose committed hidden columns are masked or "
            "secret-shared so query openings are simulatable, while the proof "
            "system still extracts one committed trace and typed role "
            "subcommitments."
        ),
        "api_fit": {
            "parent_root_binding": "yes_if_commitment_binding_theorem_exists",
            "role_subcommitment_binding": "yes_if_role_subroots_are_authenticated",
            "hidden_column_zk": "conditional_core_burden",
            "query_schedule_binding": "yes_if_fs_binds_masks_and_roots",
        },
        "why": (
            "This is the only route that preserves the transparent hash-proof "
            "surface while matching the current compact theorem. The main cost is "
            "more columns/constraints for mask consistency and a real simulator "
            "proof."
        ),
        "next_action": (
            "Specify the masked-column transform and update the 80-lane/66-row "
            "model with mask columns, role subroots, and simulator obligations."
        ),
    },
    {
        "id": "explicit_algebraic_opening_rows",
        "verdict": "fallback_if_zk_trace_masks_fail",
        "description": (
            "Keep transparent proof soundness, but add explicit q12289/SIS "
            "commitment-opening equations tying role commitments to hidden trace "
            "values."
        ),
        "api_fit": {
            "parent_root_binding": "yes",
            "role_subcommitment_binding": "via_explicit_rows",
            "hidden_column_zk": "still_requires_masking_or_private_opening",
            "query_schedule_binding": "yes",
        },
        "why": (
            "It removes ambiguity about role links, but it may increase rows and "
            "can recreate public-linear-commitment leakage unless wrapped in ZK."
        ),
        "next_action": "reserve as fallback and rerun byte model if selected",
    },
    {
        "id": "mpc_in_the_head_hash_zk",
        "verdict": "fallback_larger_payload",
        "description": (
            "Use a hash-based ZK proof of the same relation via MPC-in-the-head or "
            "related symmetric-key ZK rather than a FRI-style trace proof."
        ),
        "api_fit": {
            "parent_root_binding": "not_native",
            "role_subcommitment_binding": "must_be_encoded_in_statement",
            "hidden_column_zk": "yes_by_design_if_instantiated_correctly",
            "query_schedule_binding": "yes",
        },
        "why": (
            "ZK is more native, but payload sizes and verifier work are likely "
            "larger, and the trace-subcommitment story would need a different "
            "public-object mapping."
        ),
        "next_action": "keep as backup if transparent masked trace cannot be made sound and ZK",
    },
]


GATE_REQUIREMENTS = [
    {
        "id": "no_raw_hidden_openings",
        "requirement": (
            "No verifier-visible query value may be an unmasked pad-source, "
            "projection, opening-tuple, digit/range, blinding, noise, or mask seed "
            "lane."
        ),
        "redesign_if_false": True,
    },
    {
        "id": "extractable_parent_trace",
        "requirement": (
            "Knowledge soundness must extract one committed parent trace under "
            "trace_commitment_root, not merely a set of satisfying values."
        ),
        "redesign_if_false": True,
    },
    {
        "id": "typed_role_subroots",
        "requirement": (
            "The proof family must authenticate role subcommitments/column roots "
            "matching Path2CreationProofBindingV1 role handles."
        ),
        "redesign_if_false": True,
    },
    {
        "id": "mask_consistency_without_secret_export",
        "requirement": (
            "Mask/share consistency must be checked without revealing reusable "
            "mask equations across rows, slots, accounts, or queries."
        ),
        "redesign_if_false": True,
    },
    {
        "id": "byte_budget_update",
        "requirement": (
            "Any selected ZK masking transform must update the byte model; the old "
            "4896/6744/8848-byte planning rows were for the unmasked 80-lane/"
            "66-row comparator."
        ),
        "redesign_if_false": False,
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_transparent_proof_family_gate.v1",
        "status": "primary_candidate_selected_conditionally",
        "finding": (
            "A raw transparent Merkle/FRI trace proof is not compatible with the "
            "Path 2 live-slot hiding theorem. The compact proof path remains "
            "viable only through a ZK masked-trace IOP or an equivalent simulator "
            "theorem that keeps hidden columns from being opened raw."
        ),
        "families": FAMILIES,
        "selected_candidate": "zk_masked_trace_iop",
        "gate_requirements": GATE_REQUIREMENTS,
        "theorem_delta": (
            "The transparent proof assumption must be named as "
            "zkMaskedTraceIOPBindingAndHiding: accepted transcripts extract one "
            "committed parent trace with authenticated role subroots, while the "
            "public transcript is simulatable from public inputs and "
            "masked_projection alone."
        ),
        "byte_model_delta": (
            "The current transparent byte model is now a lower-bound comparator. "
            "A real candidate must add mask/share columns, role-subroot metadata "
            "if not already carried in Path2CreationProofBindingV1, and any "
            "mask-consistency openings."
        ),
        "next_blocker": (
            "Define the masked-column transform for R_core_66: which values are "
            "committed, which values are opened, how masked_projection_i = "
            "pad_i + projection_i mod q is checked without raw openings, and how "
            "role subroots authenticate the unmasked extractor witness."
        ),
        "non_claims": [
            "this does not instantiate a concrete STARK/IOP",
            "this does not prove the simulator theorem",
            "this does not update final proof byte counts",
            "this does not make Path 2 deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Transparent Proof-Family Gate",
        "",
        "Status: primary candidate selected conditionally.",
        "",
        model["finding"],
        "",
        "## Family Options",
        "",
        "| id | verdict | description | API fit | why | next action |",
        "|---|---|---|---|---|---|",
    ]
    for item in model["families"]:
        api_fit = "<br>".join(
            f"`{key}`: {value}" for key, value in item["api_fit"].items()
        )
        lines.append(
            "| `{}` | `{}` | {} | {} | {} | {} |".format(
                item["id"],
                item["verdict"],
                item["description"],
                api_fit,
                item["why"],
                item["next_action"],
            )
        )
    lines.extend(
        [
            "",
            "## Selected Candidate",
            "",
            f"`{model['selected_candidate']}`",
            "",
            "## Gate Requirements",
            "",
            "| id | requirement | redesign if false |",
            "|---|---|---|",
        ]
    )
    for item in model["gate_requirements"]:
        lines.append(
            "| `{id}` | {requirement} | `{redesign_if_false}` |".format(**item)
        )
    lines.extend(
        [
            "",
            "## Theorem Delta",
            "",
            model["theorem_delta"],
            "",
            "## Byte Model Delta",
            "",
            model["byte_model_delta"],
            "",
            "## Next Blocker",
            "",
            model["next_blocker"],
            "",
            "## Non-Claims",
            "",
        ]
    )
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
