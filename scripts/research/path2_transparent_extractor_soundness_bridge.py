#!/usr/bin/env python3
"""Generate the Path 2 transparent extractor/soundness bridge."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_transparent_extractor_soundness_bridge.json"
OUT_MD = ROOT / "research/path2_transparent_extractor_soundness_bridge.md"


INPUT_BINDINGS = [
    (
        "transparent_public_view",
        "Path2TransparentDescentPublicViewV1",
        "public proof metadata carrying relation id, transcript digest, two-view trace shape, masked projection, and public context digests",
    ),
    (
        "transparent_payload",
        "Path2DirectCreationProofPayloadV1.transparent_hash_proof_over_path2_descent_relation",
        "opaque proof bytes and openings for the selected transparent proof family",
    ),
    (
        "leaf_context_digest",
        "32 bytes",
        "binds account id, public key identity, slot id, verifier version, state root, lifecycle, live marker, hidden-source statement digest, and relation transcript digest without recursively including the leaf or payload digest",
    ),
    (
        "creation_proof_binding_digest",
        "32 bytes",
        "binds Path2CreationProofBindingV1 and prevents rebinding the proof payload to another creation statement",
    ),
    (
        "matrix_domain_transcript_digest",
        "32 bytes",
        "binds role-separated Path 2 descent/SIS matrix domains and row band",
    ),
    (
        "masked_projection",
        "six canonical q12289 residues",
        "public quotient target checked by the hidden-source relation",
    ),
    (
        "relation_id",
        "32 bytes",
        "names the 80-lane / 66-row Path 2 descent relation",
    ),
    (
        "transparent_transcript_digest",
        "32 bytes",
        "Fiat-Shamir binding for domain, proof system, relation, two-view shape, public context, roots, role handles, and query index",
    ),
]


EXTRACTED_OPENING = [
    (
        "parent_trace_context",
        "authenticates the public masked view and hidden extractor view under one transcript context",
    ),
    (
        "public_masked_view",
        "contains only public columns, masked role values, masked range auxiliaries, and role/column metadata",
    ),
    (
        "hidden_extractor_view",
        "contains the unmasked role vectors, masks, range witnesses, and account-binding witnesses used for soundness",
    ),
    (
        "pad_source_opening",
        "opens the leaf-bound pad seed digest and per-slot pad lanes used by the relation",
    ),
    (
        "projection_relation_opening",
        "opens the hidden source projection lanes whose masked public image is masked_projection",
    ),
    (
        "opening_tuple",
        "opens the account-witness tuple carried by the creation-time hidden-source relation",
    ),
    (
        "base128_digit_range_witnesses",
        "prove canonical/range-constrained decomposition for q12289 semantic lanes without relying on raw unbounded residues",
    ),
]


NAMED_BREAKS = [
    (
        "transparentProofSoundnessBreak",
        "transparent verifier accepts but no valid two-view trace satisfying the selected relation and consistency checks exists",
    ),
    (
        "creation_time_hidden_source_soundness_break",
        "extracted hidden view does not satisfy CreationHiddenSourceRelationV1 for the accepted leaf/evidence/certificate",
    ),
    (
        "path2_sis_norm_or_range_break",
        "extracted lanes are noncanonical, out of range, or exploit modular wrap outside the carried bound",
    ),
    (
        "qrom_or_domain_separation_break",
        "the transcript is accepted under a wrong domain, relation id, proof binding, trace shape, or query schedule",
    ),
    (
        "live_slot_leakage_or_keygen_break",
        "accepted public proof or reject path exports private live-slot material outside the no-export policy",
    ),
]


COMPOSITION_STEPS = [
    (
        "bind_public_view",
        "Recompute Path2TransparentDescentPublicViewV1 from the accepted leaf/evidence/certificate and reject unless all public digests, masked_projection, relation_id, and transcript digest match.",
        "prevalidation_leaf_binding_break or qrom_or_domain_separation_break",
    ),
    (
        "verify_transparent_payload",
        "Run the selected two-view transparent verifier against the public masked view and payload bytes.",
        "transparentProofSoundnessBreak",
    ),
    (
        "extract_two_view_trace",
        "Use the proof-system extractor/soundness theorem to obtain a parent trace context, public masked view, and hidden extractor view.",
        "transparentProofSoundnessBreak or qrom_or_domain_separation_break",
    ),
    (
        "decode_path2_opening",
        "Decode the hidden extractor view into pad source, projection relation, opening tuple, and base-128 range witnesses.",
        "path2_sis_norm_or_range_break",
    ),
    (
        "replay_creation_relation",
        "Replay the 66-row Path 2 descent relation and the CreationHiddenSourceRelationV1 checks against the accepted leaf/evidence/certificate.",
        "creation_time_hidden_source_soundness_break",
    ),
    (
        "charge_no_export",
        "Check that public outputs contain only allowlisted public masked-view material and that hidden-view values are available only to the extractor.",
        "live_slot_leakage_or_keygen_break",
    ),
]


BLOCKERS = [
        "concrete transparent proof verifier is not implemented",
        "two-view trace/relation encoding for R_core_66 plus range/account-binding predicates is not implemented",
        "proof-system-specific extractor/soundness theorem is not instantiated with concrete query counts",
        "full concrete query-opening no-export KATs are still pending",
        "integration with Path2DirectCreationProofPayloadV1 has a fail-closed boundary verifier but no accepting transparent verifier",
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_transparent_extractor_soundness_bridge.v1",
        "status": "extractor_soundness_bridge_pinned_not_proved",
        "active_candidate": "transparent_hash_proof_over_path2_descent_relation",
        "consumes_contract": "research/path2_transparent_payload_contract.md",
        "feeds_theorem": "direct_replayable_relation_certificate_soundness_or_break",
        "bridge_claim": (
            "If the selected transparent payload verifier accepts a public view "
            "bound to the Path 2 account-creation leaf/evidence/certificate, then "
            "either the extractor obtains a two-view trace whose hidden extractor "
            "view yields pad_source_opening, projection_relation_opening, "
            "opening_tuple, and base128 digit/range witnesses satisfying R_core_66 "
            "plus account-binding/range predicates for that public view, or the "
            "verifier exports a named transparent proof, hidden-source, norm/range, "
            "QROM/domain, or no-export break."
        ),
        "input_bindings": [
            {"input": name, "encoding": encoding, "role": role}
            for name, encoding, role in INPUT_BINDINGS
        ],
        "extracted_opening": [
            {"output": name, "role": role} for name, role in EXTRACTED_OPENING
        ],
        "named_breaks": [
            {"break": name, "meaning": meaning} for name, meaning in NAMED_BREAKS
        ],
        "composition_into_direct_certificate": [
            {"step": step, "claim": claim, "if_false": if_false}
            for step, claim, if_false in COMPOSITION_STEPS
        ],
        "direct_theorem_connection": (
            "The direct certificate theorem consumes this bridge at the existing "
            "replay_relation_checks step: an accepted certificate either obtains "
            "a concrete hidden opening satisfying CreationHiddenSourceRelationV1 "
            "or maps the failure to one of the direct theorem's named breaks. "
            "The bridge does not replace leaf binding, data availability, "
            "lifecycle, consume-once, or spend-time checked-core obligations."
        ),
        "remaining_blockers": BLOCKERS,
        "negative_controls_to_add": [
            "malformed transparent proof bytes reject without private-field diagnostics",
            "wrong relation_id rejects before any relation replay is accepted",
            "wrong trace_shape rejects or changes query derivation",
            "wrong creation_proof_binding_digest rejects or changes transcript digest",
            "query opening carrying denylisted private lane labels rejects/no-exports",
            "raw hidden-view leaf opening rejects/no-exports",
            "public masked view inconsistent with hidden extractor view rejects or exports transparentProofSoundnessBreak",
            "proof_system_id for parked private-ZK route rejects in transparent mode",
        ],
        "non_claims": [
            "this does not implement the transparent proof verifier",
            "this does not prove transparent proof soundness",
            "this does not quantify the concrete soundness parameter",
            "this does not close direct_replayable_relation_certificate_soundness_or_break",
            "this does not make the system production deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Transparent Extractor/Soundness Bridge",
        "",
        "Status: extractor/soundness bridge pinned; not proved.",
        "",
        f"Active candidate: `{model['active_candidate']}`.",
        "",
        f"Feeds theorem: `{model['feeds_theorem']}`.",
        "",
        "## Bridge Claim",
        "",
        model["bridge_claim"],
        "",
        "## Input Bindings",
        "",
        "| input | encoding | role |",
        "|---|---|---|",
    ]
    for item in model["input_bindings"]:
        lines.append("| {input} | {encoding} | {role} |".format(**item))
    lines.extend(["", "## Extracted Opening", ""])
    for item in model["extracted_opening"]:
        lines.append(f"- `{item['output']}`: {item['role']}")
    lines.extend(["", "## Named Breaks", "", "| break | meaning |", "|---|---|"])
    for item in model["named_breaks"]:
        lines.append("| `{break}` | {meaning} |".format(**item))
    lines.extend(
        [
            "",
            "## Composition Into Direct Certificate",
            "",
            "| step | claim | if false |",
            "|---|---|---|",
        ]
    )
    for item in model["composition_into_direct_certificate"]:
        lines.append("| `{step}` | {claim} | `{if_false}` |".format(**item))
    lines.extend(
        [
            "",
            "## Direct Theorem Connection",
            "",
            model["direct_theorem_connection"],
            "",
            "## Remaining Blockers",
            "",
        ]
    )
    for item in model["remaining_blockers"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Negative Controls To Add", ""])
    for item in model["negative_controls_to_add"]:
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
