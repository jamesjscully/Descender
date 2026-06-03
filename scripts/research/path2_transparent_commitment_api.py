#!/usr/bin/env python3
"""Generate the Path 2 transparent commitment API theorem interface."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_transparent_commitment_api.json"
OUT_MD = ROOT / "research/path2_transparent_commitment_api.md"


ROLE_HANDLES = [
    {
        "role": "pad_source",
        "binding_field": "pad_source_commitment",
        "trace_columns": "pad_source base128 digit columns and role-local blinding columns",
        "extractor_output": "pad_source_opening",
        "link_rule": (
            "binding_field equals the domain-separated role subcommitment handle "
            "authenticated under trace_commitment_root for the pad-source column set"
        ),
    },
    {
        "role": "projection_relation",
        "binding_field": "projection_relation_commitment",
        "trace_columns": "projection relation base128 digit columns and role-local blinding columns",
        "extractor_output": "projection_relation_opening",
        "link_rule": (
            "binding_field equals the domain-separated role subcommitment handle "
            "authenticated under trace_commitment_root for the projection column set"
        ),
    },
    {
        "role": "opening_tuple",
        "binding_field": "opening_tuple_commitment",
        "trace_columns": "opening-tuple base128 digit columns, account-binding columns, and role-local blinding columns",
        "extractor_output": "opening_tuple",
        "link_rule": (
            "binding_field equals the domain-separated role subcommitment handle "
            "authenticated under trace_commitment_root for the opening-tuple/account-binding column set"
        ),
    },
    {
        "role": "packed_witness_projection",
        "binding_field": "packed_witness_commitment",
        "trace_columns": "masked projection source columns consumed by R_core_66",
        "extractor_output": "packed witness projection link",
        "link_rule": (
            "trace projection lanes agree with the packed-witness commitment through "
            "a proof-friendly handle or else the theorem exports account_binding_break"
        ),
    },
]


API_OBLIGATIONS = [
    {
        "id": "parent_root_binding",
        "statement": (
            "For a fixed proof_system_id, relation_id, column_profile, and "
            "trace_commitment_root, an accepting proof extracts at most one hidden "
            "trace except with transparentProofSoundnessBreak."
        ),
        "why": "prevents after-the-fact choice of a trace satisfying R_core_66",
    },
    {
        "id": "role_subcommitment_binding",
        "statement": (
            "Each role handle in Path2CreationProofBindingV1 is authenticated as "
            "a typed subcommitment of the extracted parent trace commitment."
        ),
        "why": "links R_core_66 witness lanes to the public account-creation object",
    },
    {
        "id": "hidden_column_zk",
        "statement": (
            "Verifier-visible role handles, query openings, paths, and consistency "
            "messages are simulatable without pad-source, projection, opening-"
            "tuple, range, blinding, or mask witnesses."
        ),
        "why": "keeps the creation payload from changing the live-slot leakage mode",
    },
    {
        "id": "query_schedule_binding",
        "statement": (
            "Fiat-Shamir query indices are derived after binding leaf_context_digest, "
            "creation_proof_binding_digest, trace_commitment_root, role handles "
            "through that digest, matrix_domain_transcript_digest, and "
            "aggregation_context_digest."
        ),
        "why": "prevents moving a valid trace between accounts, slots, roots, or bindings",
    },
    {
        "id": "no_sha_preimage_claim",
        "statement": (
            "Role handles are proof-system commitment handles. The compact relation "
            "does not claim SHA-256 preimage knowledge for pad/opening/projection "
            "objects."
        ),
        "why": "keeps the theorem aligned with the byte model",
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_transparent_commitment_api.v1",
        "status": "theorem_interface_pinned_not_instantiated",
        "purpose": (
            "Give the selected compact link-equation path a precise commitment "
            "interface before implementing a verifier. This is the proof-system "
            "contract that lets R_core_66 stay at 66 rows without treating role "
            "commitments as SHA preimages."
        ),
        "public_inputs": [
            "proof_system_id",
            "relation_id",
            "column_profile",
            "trace_commitment_root",
            "constraint_commitment_root",
            "leaf_context_digest",
            "creation_proof_binding_digest",
            "matrix_domain_transcript_digest",
            "masked_projection[6]",
            "aggregation_context_digest",
            "transparent_transcript_digest",
        ],
        "role_handles": ROLE_HANDLES,
        "api_obligations": API_OBLIGATIONS,
        "extractor_contract": (
            "Given an accepting payload, the extractor returns one committed trace "
            "under trace_commitment_root, role openings authenticated by the "
            "Path2CreationProofBindingV1 handles, and R_core_66 witnesses. If two "
            "different role vectors authenticate under the same role handle, the "
            "extractor exports inner_commitment_binding_break."
        ),
        "simulator_contract": (
            "The simulator can produce public role handles, trace/constraint roots, "
            "query openings, paths, and consistency messages from public inputs and "
            "the allowed masked_projection without exporting hidden role columns."
        ),
        "column_profile_requirements": [
            "domain-separated role identifiers for pad_source, projection_relation, opening_tuple, and packed_witness_projection",
            "fixed column ranges or authenticated column-set descriptors for every role",
            "explicit marking of public, committed_hidden, derived_hidden, blinding, and range columns",
            "query policy stating which openings are public, masked, or simulatable",
            "negative-control rule rejecting unknown role ids, duplicate role ids, and role handles not authenticated under the parent root",
        ],
        "bytes_to_serialize": [
            "parent trace_commitment_root",
            "constraint_commitment_root when used",
            "proof bytes digest",
            "query openings digest",
            "Merkle/authentication paths digest",
            "consistency messages digest",
            "role handles through Path2CreationProofBindingV1 digest, not duplicated unless the payload format needs availability copies",
        ],
        "next_blocker": (
            "Choose whether the transparent proof family can supply authenticated "
            "typed role subcommitments with ZK query openings. If not, the compact "
            "Path 2 theorem must fall back to explicit algebraic opening rows."
        ),
        "non_claims": [
            "this is not a concrete Merkle/FRI/STARK implementation",
            "this does not choose query counts",
            "this does not prove the simulator theorem",
            "this does not change spend signature or account-state sizes",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Transparent Commitment API",
        "",
        "Status: theorem interface pinned; not instantiated.",
        "",
        model["purpose"],
        "",
        "## Public Inputs",
        "",
    ]
    for item in model["public_inputs"]:
        lines.append(f"- `{item}`")
    lines.extend(
        [
            "",
            "## Role Handles",
            "",
            "| role | binding field | trace columns | extractor output | link rule |",
            "|---|---|---|---|---|",
        ]
    )
    for item in model["role_handles"]:
        lines.append(
            "| `{role}` | `{binding_field}` | {trace_columns} | `{extractor_output}` | {link_rule} |".format(
                **item
            )
        )
    lines.extend(
        [
            "",
            "## API Obligations",
            "",
            "| id | statement | why |",
            "|---|---|---|",
        ]
    )
    for item in model["api_obligations"]:
        lines.append("| `{id}` | {statement} | {why} |".format(**item))
    lines.extend(["", "## Extractor Contract", "", model["extractor_contract"]])
    lines.extend(["", "## Simulator Contract", "", model["simulator_contract"]])
    lines.extend(["", "## Column Profile Requirements", ""])
    for item in model["column_profile_requirements"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Bytes To Serialize", ""])
    for item in model["bytes_to_serialize"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Next Blocker", "", model["next_blocker"], "", "## Non-Claims", ""])
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
