#!/usr/bin/env python3
"""Generate Path 2 role-commitment link-equation options."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_role_commitment_link_equation_options.json"
OUT_MD = ROOT / "research/path2_role_commitment_link_equation_options.md"


OPTIONS = [
    {
        "id": "role_commitments_are_trace_subcommitments",
        "verdict": "preferred_compact_path",
        "meaning": (
            "Interpret pad_source_commitment, projection_relation_commitment, and "
            "opening_tuple_commitment as typed handles/subcommitments under the "
            "transparent proof's hiding vector commitment, not as SHA preimages "
            "to be opened inside R_core_66."
        ),
        "link_statement": (
            "The accepted proof opens one trace commitment root; domain-separated "
            "column/role subcommitments extracted from that root equal the role "
            "commitment handles in Path2CreationProofBindingV1."
        ),
        "proof_burden": (
            "Need a concrete transparent commitment API with subroot/column-root "
            "binding, hiding for role subcommitments, and an extractor that names "
            "the same trace values used by R_core_66."
        ),
        "row_impact": "no R_core_66 row increase if the proof-family commitment theorem carries the link",
        "byte_impact": (
            "likely no spend signature or account-state increase; creation payload "
            "may need to carry typed subroot metadata already covered by payload_digest"
        ),
        "risk": (
            "Requires the proof system's commitment abstraction to be first-class "
            "in the theorem. If the selected transparent proof only exposes a "
            "monolithic Merkle root with raw queried cells, hiding/no-export may fail."
        ),
    },
    {
        "id": "role_commitments_are_algebraic_q_commitments",
        "verdict": "fallback_explicit_relation",
        "meaning": (
            "Make each role commitment an algebraic q12289 commitment with an "
            "explicit opening relation against trace columns."
        ),
        "link_statement": (
            "For each role, public C_role and hidden trace vector v_role satisfy "
            "C_role = A_role * v_role + blinding/noise under the selected SIS/"
            "lattice commitment scheme."
        ),
        "proof_burden": (
            "Instantiate binding and hiding for the algebraic commitment, define "
            "noise/range semantics, and add or account for commitment-opening "
            "constraints."
        ),
        "row_impact": "increases relation size unless handled as a native proof-system opening",
        "byte_impact": (
            "could increase creation payload; no spend signature change expected "
            "unless the authorization digest frame changes"
        ),
        "risk": (
            "Can reproduce the earlier full-rank public linear commitment leakage "
            "unless hiding noise or private-ZK wrapping is done carefully."
        ),
    },
    {
        "id": "role_commitments_are_sha_digest_handles",
        "verdict": "dead_for_compact_opening",
        "meaning": (
            "Treat role commitments as ordinary SHA-256 digest handles to hidden "
            "objects and ask the compact q-relation to prove their openings."
        ),
        "link_statement": (
            "Would require proving SHA-256 preimages for pad source, projection "
            "relation, and opening tuple inside the payload proof."
        ),
        "proof_burden": "SHA-dominated circuit/proof, contrary to the selected compact path",
        "row_impact": "large SHA circuit, not part of R_core_66",
        "byte_impact": "likely destroys the account-creation proof byte target",
        "risk": "compact theorem overclaims if this is not rejected",
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_role_commitment_link_equation_options.v1",
        "status": "preferred_link_equation_path_selected_not_instantiated",
        "finding": (
            "The role-commitment link is the current proof bottleneck. The 32-byte "
            "role fields in Path2CreationProofBindingV1 cannot be treated as "
            "ordinary SHA digest preimages inside the compact q12289 relation. "
            "They must be typed proof-system subcommitments, algebraic q "
            "commitments, or dead-path SHA handles."
        ),
        "options": OPTIONS,
        "selected_path": "role_commitments_are_trace_subcommitments",
        "required_spec_delta": [
            "Define role commitment handles as typed transparent commitment subroots or column roots.",
            "Bind those handles in Path2CreationProofBindingV1 exactly as today, but give them proof-system semantics rather than SHA-preimage semantics.",
            "Require the transparent proof transcript to bind the parent trace_commitment_root and the same role handles before query derivation.",
            "State extractor output as one committed trace plus role subcommitment openings satisfying R_core_66.",
            "Export inner_commitment_binding_break if one role handle opens to two different traces or role vectors.",
            "Export live_slot_leakage_or_keygen_break if the subcommitment/query schedule reveals unmasked hidden lanes.",
        ],
        "theorem_shape": (
            "An accepting payload binds a parent transparent trace commitment and "
            "typed role subcommitments named by Path2CreationProofBindingV1. "
            "The proof-family extractor outputs the unique committed role vectors "
            "for pad source, projection relation, and opening tuple; those vectors "
            "satisfy R_core_66 and the account-binding predicate, or the adversary "
            "exports a transparent proof, inner commitment, range, no-export, or "
            "outer binding/availability break."
        ),
        "next_blocker": (
            "Use research/path2_transparent_commitment_api.md as the next theorem "
            "interface: parent root, role subroots, column profile, hiding/"
            "blinding rule, extractor guarantee, and exact public payload bytes."
        ),
        "non_claims": [
            "this does not instantiate the transparent commitment scheme",
            "this does not prove vector-commitment binding or hiding",
            "this does not add SHA preimage checks to R_core_66",
            "this does not require a spend signature size change",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Role-Commitment Link Equation Options",
        "",
        "Status: preferred link-equation path selected; not instantiated.",
        "",
        model["finding"],
        "",
        "## Options",
        "",
        "| id | verdict | meaning | link statement | proof burden | row impact | byte impact | risk |",
        "|---|---|---|---|---|---|---|---|",
    ]
    for item in model["options"]:
        lines.append(
            "| `{id}` | `{verdict}` | {meaning} | {link_statement} | {proof_burden} | {row_impact} | {byte_impact} | {risk} |".format(
                **item
            )
        )
    lines.extend(
        [
            "",
            "## Selected Path",
            "",
            f"`{model['selected_path']}`",
            "",
            "## Required Spec Delta",
            "",
        ]
    )
    for item in model["required_spec_delta"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Theorem Shape",
            "",
            model["theorem_shape"],
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
