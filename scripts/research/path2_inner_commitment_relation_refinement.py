#!/usr/bin/env python3
"""Generate the Path 2 inner-commitment relation refinement note."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_inner_commitment_relation_refinement.json"
OUT_MD = ROOT / "research/path2_inner_commitment_relation_refinement.md"


RELATION_LAYERS = [
    {
        "id": "R_core_66",
        "status": "semantic_core_only",
        "public_inputs": [
            "masked_projection[6]",
            "relation_id",
            "leaf_context_digest",
            "creation_proof_binding_digest",
            "matrix_domain_transcript_digest",
            "aggregation_context_digest",
        ],
        "private_outputs": [
            "pad_source_opening",
            "projection_relation_opening",
            "opening_tuple",
            "base128_digit_range_witnesses",
        ],
        "rows": "66",
        "claim": (
            "Extracted base128 digits reconstruct canonical q12289 residues and "
            "satisfy masked_projection_i = pad_source_i + projection_i mod q."
        ),
        "missing_from_layer": [
            "proof that extracted residues open the proof-friendly commitments named by creation_proof_binding_digest",
            "proof that opening_tuple encodes the account witness/lifecycle/pad-source semantics",
            "proof of any SHA-256 preimage relation",
        ],
    },
    {
        "id": "R_inner_commitment_opening",
        "status": "must_be_carried_or_instantiated_next",
        "public_inputs": [
            "proof-friendly inner commitment descriptors",
            "creation_proof_binding_digest",
            "matrix_domain_transcript_digest",
        ],
        "private_outputs": [
            "same pad/projection/opening values consumed by R_core_66",
            "inner commitment randomness or proof-system witness handles",
        ],
        "rows": "unknown_or_inherent",
        "claim": (
            "The same hidden values extracted from R_core_66 are openings of the "
            "inner commitments committed by the Path2CreationProofBindingV1 object."
        ),
        "missing_from_layer": [
            "choice between proof-system-inherent trace commitment binding and explicit algebraic opening equations",
            "concrete row/byte cost if opening equations are explicit",
            "named binding break if two openings verify for one inner commitment",
        ],
    },
    {
        "id": "R_account_binding",
        "status": "open_semantic_predicate",
        "public_inputs": [
            "leaf_context_digest",
            "account_id",
            "public_key_identity",
            "slot_id",
            "account_state_root_before",
            "lifecycle_anchor",
            "verifier_version",
        ],
        "private_outputs": [
            "opening_tuple semantic fields",
            "account witness commitment opening",
            "pad-source and projection-link handles",
        ],
        "rows": "not_in_66_row_count",
        "claim": (
            "The opening tuple is not arbitrary short data: it encodes the hidden "
            "source/account witness material for exactly the public leaf context."
        ),
        "missing_from_layer": [
            "exact scalarization of opening_tuple fields",
            "row count or external theorem for lifecycle/account-witness equality",
            "negative controls for moving a valid tuple between accounts, slots, or state roots",
        ],
    },
    {
        "id": "R_outer_sha_binding",
        "status": "outside_compact_arithmetic_relation",
        "public_inputs": [
            "canonical leaf bytes",
            "canonical creation-proof-binding bytes",
            "payload digest",
            "certificate digest",
        ],
        "private_outputs": [],
        "rows": "not_paid_inside_compact_relation",
        "claim": (
            "SHA-256 binds canonical public bytes and data availability. It does "
            "not supply compact in-circuit proof of hidden preimages."
        ),
        "missing_from_layer": [
            "none if theorem only claims public byte binding and available payloads",
            "full SHA circuits if the theorem later claims SHA preimage knowledge",
        ],
    },
]


COMMITMENT_BINDING_OPTIONS = [
    {
        "id": "trace_commitment_is_inner_commitment",
        "verdict": "preferred_first_probe",
        "description": (
            "Treat the proof-friendly committed trace itself as the inner "
            "commitment layer. The theorem consumes the transparent proof "
            "family's vector-commitment binding and ZK simulator, and R_core_66 "
            "runs over committed hidden columns."
        ),
        "burden": (
            "State the proof-family commitment-binding theorem precisely: an "
            "accepting transcript extracts one committed trace whose hidden "
            "columns satisfy R_core_66 and whose public trace digest is bound "
            "through the accepted transparent payload/certificate path together "
            "with Path2CreationProofBindingV1."
        ),
        "row_impact": "no extra arithmetic rows beyond proof-family commitments",
        "risk": (
            "If the trace columns are not linked to the role commitments named "
            "by Path2CreationProofBindingV1, validators may have a proof about "
            "a transcript-bound trace that does not open the intended account-"
            "creation object."
        ),
    },
    {
        "id": "explicit_algebraic_opening_rows",
        "verdict": "fallback_if_trace_binding_is_too_implicit",
        "description": (
            "Add rows checking C_role = A_role * v_role + blinding/noise for each "
            "proof-friendly commitment role."
        ),
        "burden": (
            "Instantiate the commitment family, prove binding/hiding, and update "
            "row/byte estimates for all commitment-opening equations."
        ),
        "row_impact": "increases the relation; not part of the current 66-row count",
        "risk": (
            "May reproduce the earlier public linear-hiding problem unless the "
            "commitments are private inside a ZK layer or computationally hiding."
        ),
    },
    {
        "id": "outer_sha_only",
        "verdict": "dead_path",
        "description": (
            "Keep only SHA-256 digests and ask the 66-row relation to imply the "
            "hidden account witness opening."
        ),
        "burden": "none; reject as an overclaim",
        "row_impact": "appears compact but proves the wrong theorem",
        "risk": (
            "The extractor can output residues satisfying R_core_66 without "
            "showing they are the values committed by the account-creation object."
        ),
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_inner_commitment_relation_refinement.v1",
        "status": "66_row_core_split_from_full_payload_relation",
        "finding": (
            "The 66-row target is only R_core_66. It is necessary, but it is not "
            "the full account-creation payload theorem unless the accepted proof "
            "also binds the extracted hidden values to the inner commitments and "
            "account semantics named by the public leaf context."
        ),
        "relation_layers": RELATION_LAYERS,
        "commitment_binding_options": COMMITMENT_BINDING_OPTIONS,
        "selected_next_probe": "trace_commitment_is_inner_commitment",
        "theorem_rewrite": (
            "Accepted transparent account creation implies extraction of a single "
            "proof-family committed trace. That trace satisfies R_core_66, its "
            "public commitment/transcript digest is bound by the accepted "
            "certificate/payload path together with Path2CreationProofBindingV1, "
            "and the relation links the trace columns to the binding object's "
            "role commitments and nonrecursive leaf context; otherwise the "
            "adversary exports a transparent proof binding/soundness break, "
            "inner commitment binding break, account-binding break, range break, "
            "no-export break, or outer SHA availability/binding break."
        ),
        "blocking_questions": [
            "Does the two-hop transcript/payload binding in research/path2_trace_commitment_binding_audit.md suffice, or must trace roots move into Path2CreationProofBindingV1?",
            "Can the extractor name one committed hidden trace, not merely satisfying values after the fact?",
            "What exact public fields does opening_tuple bind, and are those checks inside R_account_binding or supplied by a separate hidden-source theorem?",
            "Does the selected transparent proof family provide ZK for hidden trace columns while retaining extractable binding to the public trace commitment?",
        ],
        "tracker_update": (
            "Do not call the 66-row relation the full payload relation. Treat it "
            "as R_core_66 until R_inner_commitment_opening and R_account_binding "
            "are either explicitly arithmetized or consumed as named proof-family "
            "and hidden-source assumptions."
        ),
        "non_claims": [
            "this does not instantiate the transparent proof family",
            "this does not close inner commitment binding",
            "this does not close opening_tuple account semantics",
            "this does not change spend signature, public key, or state sizes",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Inner-Commitment Relation Refinement",
        "",
        "Status: 66-row core split from the full payload relation.",
        "",
        model["finding"],
        "",
        "## Relation Layers",
        "",
        "| layer | status | rows | claim | missing from this layer |",
        "|---|---|---:|---|---|",
    ]
    for item in model["relation_layers"]:
        missing = "<br>".join(item["missing_from_layer"])
        lines.append(
            "| `{id}` | `{status}` | {rows} | {claim} | {missing} |".format(
                **item, missing=missing
            )
        )
    lines.extend(
        [
            "",
            "## Binding Options",
            "",
            "| id | verdict | description | burden | row impact | risk |",
            "|---|---|---|---|---|---|",
        ]
    )
    for item in model["commitment_binding_options"]:
        lines.append(
            "| `{id}` | `{verdict}` | {description} | {burden} | {row_impact} | {risk} |".format(
                **item
            )
        )
    lines.extend(
        [
            "",
            "## Selected Next Probe",
            "",
            f"`{model['selected_next_probe']}`",
            "",
            "## Theorem Rewrite",
            "",
            model["theorem_rewrite"],
            "",
            "## Blocking Questions",
            "",
        ]
    )
    for item in model["blocking_questions"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Tracker Update", "", model["tracker_update"], "", "## Non-Claims", ""])
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
