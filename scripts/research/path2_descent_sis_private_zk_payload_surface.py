#!/usr/bin/env python3
"""Generate the private-ZK payload surface for Path 2 descent/SIS."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_descent_sis_private_zk_payload_surface.json"
OUT_MD = ROOT / "research/path2_descent_sis_private_zk_payload_surface.md"


PUBLIC_STATEMENT = [
    {
        "field": "leaf_context_digest",
        "source": "Path2PrevalidatedSlotLeafV2 / slot authorization context",
        "role": "binds account id, public-key identity, slot id, verifier version, state root, lifecycle anchor, and live marker",
    },
    {
        "field": "creation_proof_binding_digest",
        "source": "Path2CreationProofBindingV1",
        "role": "binds packed witness commitment, randomizer commitment, pad/opening/projection proof commitments, statement digest, and aggregation context",
    },
    {
        "field": "masked_projection[6]",
        "source": "public quotient lanes",
        "role": "public relation target: masked_projection_i = pad_i + account_projection_i mod q",
    },
    {
        "field": "matrix_domain_transcript_digest",
        "source": "Path 2 descent/SIS matrix role domains and row band",
        "role": "prevents prover-selected matrix or row-band substitution",
    },
    {
        "field": "zk_transcript_digest",
        "source": "payload proof transcript",
        "role": "Fiat-Shamir/QROM binding for the account-creation proof",
    },
]

PRIVATE_WITNESS = [
    {
        "field": "pad_source_opening",
        "role": "hidden pad source lanes plus proof randomness",
        "must_not_export": True,
    },
    {
        "field": "projection_relation_opening",
        "role": "hidden account projection lanes plus proof randomness",
        "must_not_export": True,
    },
    {
        "field": "opening_tuple",
        "role": "hidden source/account opening tuple and range witnesses",
        "must_not_export": True,
    },
    {
        "field": "digit_or_range_witnesses",
        "role": "canonical q12289 representation and modular-wrap exclusion",
        "must_not_export": True,
    },
    {
        "field": "private_linear_commitment_messages",
        "role": "internal SIS/ZK protocol commitments, if required by the proof system",
        "must_not_export": True,
    },
]

OBLIGATION_REMAP = [
    {
        "old_row": "p2_sis_binding",
        "private_zk_interpretation": (
            "no longer means public C=A*v binding for pad/projection/opening "
            "roles; it becomes either knowledge soundness of the ZK proof, or "
            "binding of any proof-internal commitment that the extractor rewinds"
        ),
        "closure": "extractor exports a valid private witness or `path2_sis_binding_break`/proof-soundness break",
    },
    {
        "old_row": "p2_sis_hiding_sampler",
        "private_zk_interpretation": (
            "primary hiding claim moves to the ZK simulator; public payload must "
            "be simulatable from the statement without pad/projection/opening lanes"
        ),
        "closure": "payload simulator plus no-export KATs",
    },
    {
        "old_row": "p2_sis_norm_range",
        "private_zk_interpretation": (
            "still required: the proof must enforce canonical digit/range bounds "
            "so extracted witnesses cannot use modular wraparound"
        ),
        "closure": "range constraints and malformed-range negative controls",
    },
    {
        "old_row": "p2_sis_extractor",
        "private_zk_interpretation": (
            "becomes the central theorem row: accepting proof transcripts extract "
            "pad/projection/opening witnesses satisfying the hidden-source relation"
        ),
        "closure": "special-soundness/knowledge-soundness theorem or named proof-system assumption",
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_descent_sis_private_zk_payload_surface.v1",
        "status": "private_zk_route_pinned_not_instantiated",
        "decision_pressure": (
            "The current public linear commitment is non-hiding. The private-ZK "
            "route avoids publishing C=A*v for secret role openings and shifts "
            "the main proof obligation to a ZK proof of knowledge over the "
            "descent/SIS relation."
        ),
        "public_statement": PUBLIC_STATEMENT,
        "private_witness": PRIVATE_WITNESS,
        "acceptance_claim": (
            "If the account-creation payload verifier accepts, then either the "
            "prover knows pad_source_opening, projection_relation_opening, and "
            "opening_tuple values satisfying masked_projection_i = pad_i + "
            "account_projection_i mod q and the leaf/proof-binding context, or "
            "the adversary exports a named proof-system, binding, range, or QROM break."
        ),
        "obligation_remap": OBLIGATION_REMAP,
        "proof_system_requirements": [
            "zero knowledge for private witness lanes and internal commitment messages",
            "knowledge soundness or special soundness for the hidden-source relation",
            "Fiat-Shamir domain separation bound to leaf_context_digest and creation_proof_binding_digest",
            "range/digit constraints for q12289 semantic lanes",
            "malformed transcript rejection before any private witness is serialized or logged",
            "batch/aggregation-neutral transcript fields so future aggregation can be added without changing direct-mode semantics",
        ],
        "byte_model_implication": (
            "The old 725/893 byte lower bounds no longer apply as a proof-size "
            "estimate. They remain useful only as internal relation dimensions. "
            "The next byte model must be proof-system-specific."
        ),
        "next_probe": (
            "Choose a concrete lattice ZK proof family for a linear SIS relation "
            "with range/digit constraints, then estimate account-creation payload "
            "bytes and verifier q-ops against the transparent-proof fallback."
        ),
        "non_claims": [
            "this does not instantiate a concrete ZK proof system",
            "this does not close hiding, binding, extractor, or QROM rows",
            "this does not change spend signature bytes",
            "this does not make the old public C=A*v commitments safe",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Descent/SIS Private-ZK Payload Surface",
        "",
        "Status: private-ZK route pinned; not instantiated.",
        "",
        "## Decision Pressure",
        "",
        model["decision_pressure"],
        "",
        "## Public Statement",
        "",
        "| field | source | role |",
        "|---|---|---|",
    ]
    for item in model["public_statement"]:
        lines.append("| {field} | {source} | {role} |".format(**item))
    lines.extend(["", "## Private Witness", "", "| field | role | no export |", "|---|---|---|"])
    for item in model["private_witness"]:
        lines.append(
            "| {field} | {role} | `{must_not_export}` |".format(**item)
        )
    lines.extend(
        [
            "",
            "## Acceptance Claim",
            "",
            model["acceptance_claim"],
            "",
            "## Obligation Remap",
            "",
            "| row | private-ZK interpretation | closure |",
            "|---|---|---|",
        ]
    )
    for item in model["obligation_remap"]:
        lines.append(
            "| `{old_row}` | {private_zk_interpretation} | {closure} |".format(**item)
        )
    lines.extend(["", "## Proof-System Requirements", ""])
    for item in model["proof_system_requirements"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Byte Model Implication",
            "",
            model["byte_model_implication"],
            "",
            "## Next Probe",
            "",
            model["next_probe"],
            "",
            "## Non-Claims",
            "",
        ]
    )
    for item in model["non_claims"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "This is the most conservative way to preserve the original Path 2",
            "intuition. The account-creation proof is allowed to be larger than the",
            "spend signature, and it only needs to convince the verifier once that the",
            "hidden source relation was valid. Publishing non-hiding linear",
            "commitments is unnecessary if the proof system can prove the relation",
            "directly with a simulatable transcript.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    model = build_model()
    OUT_JSON.write_text(json.dumps(model, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(write_markdown(model))
    print(json.dumps({"json": str(OUT_JSON), "markdown": str(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
