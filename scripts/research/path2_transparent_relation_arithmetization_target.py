#!/usr/bin/env python3
"""Generate the Path 2 transparent relation arithmetization target."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_transparent_relation_arithmetization_target.json"
OUT_MD = ROOT / "research/path2_transparent_relation_arithmetization_target.md"

Q = 12_289

ROLE_MODEL = [
    {
        "role": "pad_source",
        "semantic_residues": 6,
        "base128_digits": 12,
        "range_witnesses": 12,
        "extractor_output": "pad_source_opening",
    },
    {
        "role": "projection_relation",
        "semantic_residues": 6,
        "base128_digits": 12,
        "range_witnesses": 12,
        "extractor_output": "projection_relation_opening",
    },
    {
        "role": "opening_tuple",
        "semantic_residues": 8,
        "base128_digits": 16,
        "range_witnesses": 16,
        "extractor_output": "opening_tuple",
    },
]


ROW_GROUPS = [
    {
        "id": "masked_projection_equations",
        "rows": 6,
        "constraint": (
            "masked_projection_i = pad_source_i + projection_relation_i mod q "
            "for i in 0..5"
        ),
        "public_inputs": ["masked_projection[6]"],
        "hidden_inputs": ["pad_source residues", "projection_relation residues"],
    },
    {
        "id": "digit_reconstruction",
        "rows": 20,
        "constraint": (
            "semantic_residue_j = digit_lo_j + 128 * digit_hi_j mod q for "
            "the 6 pad, 6 projection, and 8 opening-tuple residues"
        ),
        "public_inputs": [],
        "hidden_inputs": ["base128 digit pairs"],
    },
    {
        "id": "digit_range_checks",
        "rows": 40,
        "constraint": (
            "each base128 digit is constrained to [0,127] using the selected "
            "transparent proof range gadget"
        ),
        "public_inputs": [],
        "hidden_inputs": ["base128 digits", "range witnesses"],
    },
]


TRACE_CLASSES = [
    {
        "class": "public",
        "columns": [
            "masked_projection[6]",
            "relation_id",
            "leaf_context_digest",
            "creation_proof_binding_digest",
            "matrix_domain_transcript_digest",
            "aggregation_context_digest",
        ],
        "opening_policy": "may be opened directly",
    },
    {
        "class": "committed_hidden",
        "columns": [
            "pad_source base128 digits",
            "projection_relation base128 digits",
            "opening_tuple base128 digits",
            "range witnesses",
        ],
        "opening_policy": (
            "must not be opened raw unless the proof family supplies a simulator/"
            "masking argument for sampled openings"
        ),
    },
    {
        "class": "derived_hidden",
        "columns": [
            "pad_source residues",
            "projection_relation residues",
            "opening_tuple residues",
        ],
        "opening_policy": (
            "derived from hidden digits; sampled openings are leakage unless "
            "one-time masked or proven simulatable"
        ),
    },
]


def build_model() -> dict[str, Any]:
    private_lanes = sum(
        item["base128_digits"] + item["range_witnesses"] for item in ROLE_MODEL
    )
    relation_rows = sum(item["rows"] for item in ROW_GROUPS)
    return {
        "schema": "Descender.path2_transparent_relation_arithmetization_target.v1",
        "status": "semantic_core_66_rows_pinned_full_payload_relation_open",
        "q": Q,
        "private_witness_lanes": private_lanes,
        "relation_rows": relation_rows,
        "relation_scope": (
            "The 66 rows are R_core_66 only: masked projection, digit "
            "reconstruction, and digit/range checks. Full account-creation "
            "payload soundness also needs inner-commitment binding and "
            "opening-tuple account semantics."
        ),
        "role_model": ROLE_MODEL,
        "row_groups": ROW_GROUPS,
        "trace_classes": TRACE_CLASSES,
        "extractor_target": {
            "outputs": [
                "pad_source_opening",
                "projection_relation_opening",
                "opening_tuple",
                "base128_digit_range_witnesses",
            ],
            "success_condition": (
                "extracted digits reconstruct canonical q12289 residues, satisfy "
                "the six masked-projection equations, and replay into "
                "CreationHiddenSourceRelationV1 for the accepted public view"
            ),
        },
        "highest_risk_findings": [
            {
                "id": "raw_query_opening_leakage",
                "finding": (
                    "A generic transparent proof that opens raw trace cells would "
                    "reveal hidden digits/residues. The proof family must either "
                    "use masked/simulatable openings or keep witness cells hidden "
                    "behind a ZK layer."
                ),
                "redesign_if_false": True,
            },
            {
                "id": "digest_to_algebra_gap",
                "finding": (
                    "The 66-row arithmetic target checks masked projection and "
                    "digit/range semantics, but it does not by itself prove SHA-256 "
                    "preimages for pad_seed_digest, opening_commitment, or other "
                    "outer digest commitments. The proof must use proof-friendly "
                    "inner commitments or explicitly pay for digest circuits."
                ),
                "redesign_if_false": True,
            },
            {
                "id": "opening_tuple_binding_gap",
                "finding": (
                    "The opening_tuple role has range/digit rows, but the final "
                    "theorem still needs an explicit algebraic predicate tying that "
                    "tuple to the public creation-proof binding and account witness "
                    "semantics."
                ),
                "redesign_if_false": True,
            },
        ],
        "selected_resolution": {
            "artifact": "research/path2_top_proof_risk_resolution_options.md",
            "refinement_artifact": "research/path2_inner_commitment_relation_refinement.md",
            "trace_binding_artifact": "research/path2_trace_commitment_binding_audit.md",
            "query_opening_leakage": (
                "raw transparent openings are a dead path; require a ZK "
                "transparent IOP with masked/simulatable openings for hidden "
                "columns"
            ),
            "digest_to_algebra_gap": (
                "do not prove outer SHA preimages in the 66-row relation; prove "
                "openings to proof-friendly inner commitments and use SHA for "
                "public byte binding / data availability"
            ),
        },
        "next_proof_questions": [
            "Does the two-hop transcript/payload binding for trace roots suffice, or must trace roots move into Path2CreationProofBindingV1?",
            "Can we choose a transparent proof family whose query openings are zero-knowledge for committed_hidden and derived_hidden columns?",
            "Which proof-friendly inner commitment replaces SHA preimage checks inside the 66-row relation?",
            "Does the opening_tuple predicate need more rows than the current 8 semantic residues / 16 digit columns model?",
            "Does base128 range checking fit the byte budget once represented in the selected proof family?",
        ],
        "non_claims": [
            "this is not a verifier implementation",
            "this does not prove knowledge soundness",
            "this does not prove zero knowledge/no-export",
            "this does not close the digest-to-algebra gap",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Transparent Relation Arithmetization Target",
        "",
        "Status: 66-row semantic core pinned; full payload relation remains open.",
        "",
        f"- q: `{model['q']}`",
        f"- private witness lanes: `{model['private_witness_lanes']}`",
        f"- relation rows: `{model['relation_rows']}`",
        f"- relation scope: {model['relation_scope']}",
        "",
        "## Role Model",
        "",
        "| role | semantic residues | base128 digits | range witnesses | extractor output |",
        "|---|---:|---:|---:|---|",
    ]
    for item in model["role_model"]:
        lines.append(
            "| {role} | {semantic_residues} | {base128_digits} | "
            "{range_witnesses} | `{extractor_output}` |".format(**item)
        )
    lines.extend(["", "## Row Groups", "", "| id | rows | constraint |", "|---|---:|---|"])
    for item in model["row_groups"]:
        lines.append("| `{id}` | {rows} | {constraint} |".format(**item))
    lines.extend(
        [
            "",
            "## Trace Classes And Opening Policy",
            "",
            "| class | columns | opening policy |",
            "|---|---|---|",
        ]
    )
    for item in model["trace_classes"]:
        lines.append(
            "| `{}` | {} | {} |".format(
                item["class"],
                ", ".join(f"`{col}`" for col in item["columns"]),
                item["opening_policy"],
            )
        )
    lines.extend(["", "## Extractor Target", ""])
    lines.append(model["extractor_target"]["success_condition"])
    lines.extend(["", "Outputs:", ""])
    for item in model["extractor_target"]["outputs"]:
        lines.append(f"- `{item}`")
    lines.extend(["", "## Highest-Risk Findings", "", "| id | finding | redesign if false |", "|---|---|---|"])
    for item in model["highest_risk_findings"]:
        lines.append(
            "| `{id}` | {finding} | `{redesign_if_false}` |".format(**item)
        )
    resolution = model["selected_resolution"]
    lines.extend(
        [
            "",
            "## Selected Resolution",
            "",
            f"Artifact: `{resolution['artifact']}`.",
            f"Refinement: `{resolution['refinement_artifact']}`.",
            f"Trace binding audit: `{resolution['trace_binding_artifact']}`.",
            "",
            f"- `query_opening_leakage`: {resolution['query_opening_leakage']}",
            f"- `digest_to_algebra_gap`: {resolution['digest_to_algebra_gap']}",
        ]
    )
    lines.extend(["", "## Next Proof Questions", ""])
    for item in model["next_proof_questions"]:
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
