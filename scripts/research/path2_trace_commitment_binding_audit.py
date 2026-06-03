#!/usr/bin/env python3
"""Generate the Path 2 transparent trace-commitment binding audit."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_trace_commitment_binding_audit.json"
OUT_MD = ROOT / "research/path2_trace_commitment_binding_audit.md"


BINDING_CHAIN = [
    {
        "step": "transparent_public_view",
        "bound_fields": [
            "trace_commitment_root",
            "constraint_commitment_root",
            "creation_proof_binding_digest",
            "leaf_context_digest",
            "matrix_domain_transcript_digest",
            "masked_projection",
            "aggregation_context_digest",
        ],
        "mechanism": (
            "path2_transparent_descent_transcript_digest hashes the trace roots "
            "together with creation_proof_binding_digest and the public relation "
            "context."
        ),
        "status": "executable_boundary_exists",
    },
    {
        "step": "transparent_payload_digest",
        "bound_fields": [
            "transparent_transcript_digest",
            "proof_system_id",
            "relation_id",
            "creation_proof_binding_digest",
            "proof_bytes_digest",
            "query_openings_digest",
            "merkle_paths_digest",
            "consistency_messages_digest",
            "payload_byte_len",
        ],
        "mechanism": (
            "path2_transparent_descent_proof_payload_digest hashes the transcript "
            "and proof component digests into the public replay payload digest."
        ),
        "status": "executable_boundary_exists",
    },
    {
        "step": "certificate_and_evidence",
        "bound_fields": [
            "certificate.public_replay_payload_digest",
            "payload.payload_digest",
            "evidence.evidence_payload_digest",
        ],
        "mechanism": (
            "verify_path2_direct_creation_transparent_descent_payload_boundary "
            "checks the payload digest against the certificate/evidence path "
            "before failing closed."
        ),
        "status": "executable_boundary_exists",
    },
    {
        "step": "creation_proof_binding",
        "bound_fields": [
            "packed_witness_commitment",
            "randomizer_commitment",
            "pad_source_commitment",
            "opening_tuple_commitment",
            "projection_relation_commitment",
            "creation_statement_digest",
            "aggregation_context_digest",
        ],
        "mechanism": (
            "Path2CreationProofBindingV1 binds role commitments and leaf context, "
            "but it does not currently include trace_commitment_root or "
            "constraint_commitment_root as fields."
        ),
        "status": "role_commitment_binding_exists_trace_root_not_direct",
    },
]


OPTIONS = [
    {
        "id": "two_hop_payload_trace_binding",
        "verdict": "preferred_no_state_change",
        "description": (
            "Keep trace roots in the transparent payload/public view. The theorem "
            "says the accepted certificate binds one transparent transcript whose "
            "trace root is associated with the same creation_proof_binding_digest."
        ),
        "proof_burden": (
            "Prove the transparent verifier extracts from that exact trace root, "
            "and prove the relation contains equality/link checks tying trace "
            "columns to the role commitments in Path2CreationProofBindingV1."
        ),
        "state_or_signature_impact": "no spend signature, public key, or account-state byte change",
        "risk": (
            "If the role-commitment equality/link checks are not in the relation "
            "or proof-family statement, the proof may bind a trace that is merely "
            "co-located with the binding digest, not semantically opened by it."
        ),
    },
    {
        "id": "add_trace_root_to_creation_proof_binding",
        "verdict": "fallback_if_two_hop_binding_is_too_implicit",
        "description": (
            "Add trace_commitment_root and constraint_commitment_root to "
            "Path2CreationProofBindingV1, then bind the resulting digest in the "
            "leaf/certificate path."
        ),
        "proof_burden": (
            "Simplifies the theorem statement but requires a binding-object "
            "version bump and KAT/state-byte review."
        ),
        "state_or_signature_impact": (
            "likely no spend signature change; may add 64 bytes to the creation "
            "binding preimage and possibly to state if the full binding object, "
            "not only its digest, is persisted"
        ),
        "risk": (
            "The trace root may depend on proof-system randomness and proof shape; "
            "putting it in a first-class binding object can make account creation "
            "less modular."
        ),
    },
    {
        "id": "treat_role_commitments_as_trace_roots",
        "verdict": "dead_until_reparameterized",
        "description": (
            "Identify pad_source_commitment, projection_relation_commitment, and "
            "opening_tuple_commitment with parts of the transparent trace root."
        ),
        "proof_burden": (
            "Requires a concrete trace commitment layout where role commitments "
            "are domain-separated subroots of one extractable vector commitment."
        ),
        "state_or_signature_impact": "unknown until trace layout is concrete",
        "risk": (
            "Current structs use digest-sized role commitments plus a separate "
            "trace_commitment_root; pretending they are the same would overclaim."
        ),
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_trace_commitment_binding_audit.v1",
        "status": "two_hop_trace_binding_identified_relation_link_open",
        "finding": (
            "The current executable boundary binds transparent trace roots through "
            "the transparent transcript and payload digest, not directly inside "
            "Path2CreationProofBindingV1. This is probably workable, but only if "
            "the transparent relation explicitly links the extracted trace values "
            "to the role commitments named by Path2CreationProofBindingV1."
        ),
        "binding_chain": BINDING_CHAIN,
        "options": OPTIONS,
        "selected_next": "two_hop_payload_trace_binding",
        "required_relation_links": [
            "trace pad-source columns open the pad_source_commitment named by Path2CreationProofBindingV1 or the proof-system exports inner_commitment_binding_break",
            "trace projection columns open the projection_relation_commitment named by Path2CreationProofBindingV1 or exports inner_commitment_binding_break",
            "trace opening-tuple columns open the opening_tuple_commitment/leaf opening commitment or exports account_binding_break",
            "trace packed-witness/projection lanes agree with packed_witness_commitment where that commitment is consumed by the relation, or exports account_binding_break",
            "leaf_context_digest and creation_proof_binding_digest are both transcript-bound before Fiat-Shamir queries are derived",
        ],
        "theorem_delta": (
            "Replace the phrase 'trace commitment is bound by "
            "Path2CreationProofBindingV1' with 'the accepted certificate binds a "
            "transparent transcript containing a trace commitment root and the "
            "Path2CreationProofBindingV1 digest; the relation/proof then links "
            "that trace to the role commitments named by the binding object.'"
        ),
        "next_blocker": (
            "Use research/path2_role_commitment_link_equation_options.md to "
            "define the link equations between committed trace columns and the "
            "Path2CreationProofBindingV1 role commitments. If typed trace "
            "subcommitments are not compact or not ZK, fall back to explicit "
            "algebraic opening rows or adding trace roots to the binding object."
        ),
        "non_claims": [
            "this does not close transparent proof soundness",
            "this does not instantiate the trace commitment scheme",
            "this does not prove role commitments are hiding",
            "this does not require a spend signature size change",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Trace-Commitment Binding Audit",
        "",
        "Status: two-hop trace binding identified; relation link remains open.",
        "",
        model["finding"],
        "",
        "## Binding Chain",
        "",
        "| step | status | bound fields | mechanism |",
        "|---|---|---|---|",
    ]
    for item in model["binding_chain"]:
        lines.append(
            "| `{step}` | `{status}` | {fields} | {mechanism} |".format(
                step=item["step"],
                status=item["status"],
                fields=", ".join(f"`{field}`" for field in item["bound_fields"]),
                mechanism=item["mechanism"],
            )
        )
    lines.extend(
        [
            "",
            "## Options",
            "",
            "| id | verdict | description | proof burden | impact | risk |",
            "|---|---|---|---|---|---|",
        ]
    )
    for item in model["options"]:
        lines.append(
            "| `{id}` | `{verdict}` | {description} | {proof_burden} | {state_or_signature_impact} | {risk} |".format(
                **item
            )
        )
    lines.extend(
        [
            "",
            "## Selected Next",
            "",
            f"`{model['selected_next']}`",
            "",
            "## Required Relation Links",
            "",
        ]
    )
    for item in model["required_relation_links"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Theorem Delta",
            "",
            model["theorem_delta"],
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
