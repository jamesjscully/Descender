#!/usr/bin/env python3
"""Generate transparent payload contracts for the selected Path 2 proof path."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_transparent_payload_contract.json"
OUT_MD = ROOT / "research/path2_transparent_payload_contract.md"
EXTRACTOR_SOUNDNESS_BRIDGE_DOC = (
    ROOT / "research/path2_transparent_extractor_soundness_bridge.md"
)


FRAME_FIELDS = [
    ("domain", "DESCENDER_PATH2_TRANSPARENT_DESCENT_PROOF_V1", "separates selected transparent payload proof from checked-core, leaf, matrix, and parked private-ZK domains"),
    ("proof_system_id", "32 bytes", "names transparent hash proof family and parameter set"),
    ("relation_id", "32 bytes", "binds the 80-lane / 66-row Path 2 descent relation"),
    ("trace_shape", "trace_rows_u16 || query_count_u16 || column_profile_u16", "binds the planning/proof shape"),
    ("leaf_context_digest", "32 bytes", "binds account, slot, verifier version, state root, lifecycle, and live marker"),
    ("creation_proof_binding_digest", "32 bytes", "binds Path2CreationProofBindingV1"),
    ("matrix_domain_transcript_digest", "32 bytes", "binds role-separated descent/SIS matrix domains and row band"),
    ("masked_projection", "six canonical q12289 residues", "public quotient target"),
    ("aggregation_context_digest", "32 bytes", "direct-mode fixed field for future aggregation compatibility"),
    ("trace_commitment_root", "32 bytes", "parent commitment to the two-view masked trace context"),
    ("constraint_commitment_root", "32 bytes", "commitment to constraint/quotient trace if used"),
    ("query_index", "u16 little-endian", "separates proof queries"),
]


PUBLIC_ALLOWLIST = [
    "proof_system_id",
    "relation_id",
    "trace_shape",
    "leaf_context_digest",
    "creation_proof_binding_digest",
    "matrix_domain_transcript_digest",
    "aggregation_context_digest",
    "masked_projection[6]",
    "trace_commitment_root",
    "constraint_commitment_root",
    "query_openings",
    "merkle_paths",
    "fri_or_consistency_messages",
    "transparent_transcript_digest",
]

PRIVATE_DENYLIST = [
    "pad_lane",
    "pad_source_opening",
    "account_projection_lane",
    "projection_relation_opening",
    "opening_tuple",
    "opening_scalar",
    "digit_or_range_witness",
    "witness_frame",
    "private_trace_row",
    "private_trace_column",
    "private_blinding",
    "private_noise",
    "mask_lane",
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_transparent_payload_contract.v1",
        "status": "transparent_contract_pinned_not_implemented",
        "active_candidate": "transparent_hash_proof_over_path2_descent_relation",
        "frame_fields": [
            {"field": field, "encoding": encoding, "role": role}
            for field, encoding, role in FRAME_FIELDS
        ],
        "query_budget_symbols": [
            "Q_path2_transparent_trace_commit",
            "Q_path2_transparent_query_index",
            "Q_path2_transparent_merkle_open",
            "Q_path2_transparent_consistency",
            "Q_path2_transparent_extractor",
            "Q_path2_transparent_simulator",
        ],
        "rust_anchors": {
            "constants": [
                "PATH2_TRANSPARENT_DESCENT_PROOF_DOMAIN",
                "PATH2_TRANSPARENT_DESCENT_RELATION_DOMAIN",
                "PATH2_TRANSPARENT_DESCENT_PROOF_SYSTEM_PROFILE",
                "PATH2_TRANSPARENT_PUBLIC_VIEW_SCHEMA",
                "PATH2_TRANSPARENT_DESCENT_WITNESS_LANES",
                "PATH2_TRANSPARENT_DESCENT_RELATION_ROWS",
                "PATH2_DIRECT_CREATION_PROOF_KIND_TRANSPARENT_DESCENT",
            ],
            "functions": [
                "path2_transparent_descent_proof_system_id",
                "path2_transparent_descent_relation_id",
                "path2_transparent_descent_transcript_digest",
                "path2_prevalidated_slot_leaf_context_digest",
                "path2_transparent_descent_public_view_v1",
                "path2_transparent_descent_public_view_v1_json",
                "path2_transparent_descent_proof_payload_digest",
                "verify_path2_direct_creation_transparent_descent_payload_boundary",
            ],
            "kats": [
                "path2_transparent_descent_transcript_binds_public_context",
                "path2_transparent_descent_public_view_serializes_only_public_material",
                "path2_transparent_descent_payload_boundary_binds_public_view_before_fail_closed",
                "path2_domain_constants_are_disjoint_and_mode_separated",
            ],
        },
        "extractor_soundness_surface": {
            "claim": (
                "An accepting transparent payload verifies the public masked view "
                "of a two-view trace and extracts a hidden view satisfying "
                "R_core_66 plus account-binding/range predicates, or exports "
                "transparentProofSoundnessBreak, qrom_or_domain_separation_break, "
                "live_slot_leakage_or_keygen_break, or path2_sis_norm_or_range_break."
            ),
            "bridge_artifact": str(EXTRACTOR_SOUNDNESS_BRIDGE_DOC.relative_to(ROOT)),
            "extracts": [
                "parent trace context",
                "public masked view",
                "hidden extractor view",
                "pad_source_opening",
                "projection_relation_opening",
                "opening_tuple",
                "base128 digit/range witnesses",
            ],
            "named_breaks": [
                "transparentProofSoundnessBreak",
                "creation_time_hidden_source_soundness_break",
                "path2_sis_norm_or_range_break",
                "qrom_or_domain_separation_break",
                "live_slot_leakage_or_keygen_break",
            ],
        },
        "zero_knowledge_no_export_surface": {
            "claim": (
                "The public proof consists only of allowlisted transcript fields, "
                "commitment roots, masked query openings, Merkle/authentication "
                "paths, and consistency messages; it does not serialize private "
                "hidden-view lanes outside the transparent proof's own zero-"
                "knowledge opening policy."
            ),
            "public_serializer_allowlist": PUBLIC_ALLOWLIST,
            "private_denylist": PRIVATE_DENYLIST,
            "named_break": "live_slot_leakage_or_keygen_break",
        },
        "required_negative_controls": [
            "mutating leaf_context_digest changes query derivation or rejects",
            "mutating creation_proof_binding_digest changes query derivation or rejects",
            "mutating relation_id changes query derivation or rejects",
            "mutating trace_shape changes query derivation or rejects",
            "wrong masked_projection rejects without exposing pad/account-projection lanes",
            "public projection shortcut payload containing pad/account_projection lanes rejects",
            "reject-path diagnostics contain no denylisted private field values",
            "raw hidden-view leaf opening rejects/no-exports",
            "public masked view inconsistent with hidden extractor view rejects or exports transparentProofSoundnessBreak",
            "parked private-ZK proof_system_id rejects under transparent verifier",
        ],
        "closure_gate": (
            "Transparent proof constants, transcript binding KATs, public-view "
            "serializer/no-export KATs, query-budget symbols, and a fail-closed "
            "transparent payload boundary verifier now exist. The extractor/"
            "soundness bridge into "
            "direct_replayable_relation_certificate_soundness_or_break is now "
            "pinned, not proved. Remaining executable work is the concrete "
            "two-view transparent proof verifier, trace/relation encoding, proof-system-"
            "specific soundness parameters, and full no-export KATs for concrete "
            "query openings."
        ),
        "non_claims": [
            "this does not implement a transparent proof verifier",
            "this does not quantify concrete soundness",
            "this does not prove zero knowledge",
            "this does not close production preflight",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Transparent Payload Contract",
        "",
        "Status: transparent contract pinned; not implemented.",
        "",
        f"Active candidate: `{model['active_candidate']}`.",
        "",
        "## Frame Fields",
        "",
        "| field | encoding | role |",
        "|---|---|---|",
    ]
    for field in model["frame_fields"]:
        lines.append("| {field} | {encoding} | {role} |".format(**field))
    lines.extend(["", "## Query Budget Symbols", ""])
    for item in model["query_budget_symbols"]:
        lines.append(f"- `{item}`")
    anchors = model["rust_anchors"]
    lines.extend(["", "## Rust Anchors", "", "Constants:", ""])
    for item in anchors["constants"]:
        lines.append(f"- `{item}`")
    lines.extend(["", "Functions:", ""])
    for item in anchors["functions"]:
        lines.append(f"- `{item}`")
    lines.extend(["", "KATs:", ""])
    for item in anchors["kats"]:
        lines.append(f"- `{item}`")
    lines.extend(
        ["", "## Extractor/Soundness Surface", "", model["extractor_soundness_surface"]["claim"]]
    )
    lines.extend(
        [
            "",
            "Bridge artifact: `{}`.".format(
                model["extractor_soundness_surface"]["bridge_artifact"]
            ),
            "",
            "Extracts:",
            "",
        ]
    )
    for item in model["extractor_soundness_surface"]["extracts"]:
        lines.append(f"- `{item}`")
    lines.extend(["", "Named breaks:", ""])
    for item in model["extractor_soundness_surface"]["named_breaks"]:
        lines.append(f"- `{item}`")
    zks = model["zero_knowledge_no_export_surface"]
    lines.extend(["", "## Zero-Knowledge / No-Export Surface", "", zks["claim"], "", "Public serializer allowlist:", ""])
    for item in zks["public_serializer_allowlist"]:
        lines.append(f"- `{item}`")
    lines.extend(["", "Private denylist:", ""])
    for item in zks["private_denylist"]:
        lines.append(f"- `{item}`")
    lines.extend(["", f"Named break: `{zks['named_break']}`.", "", "## Required Negative Controls", ""])
    for item in model["required_negative_controls"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Closure Gate", "", model["closure_gate"], "", "## Non-Claims", ""])
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
