#!/usr/bin/env python3
"""Generate the Path 2 two-view trace theorem interface."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_two_view_trace_theorem_interface.json"
OUT_MD = ROOT / "research/path2_two_view_trace_theorem_interface.md"


VIEWS = [
    {
        "id": "public_masked_view",
        "visibility": "publicly queried",
        "contains": [
            "public columns",
            "masked role values",
            "masked range auxiliaries",
            "role/column metadata",
        ],
        "must_not_contain": [
            "raw pad-source digits",
            "raw projection digits",
            "raw opening-tuple digits",
            "range witnesses",
            "mask seeds",
            "account-binding witnesses",
        ],
    },
    {
        "id": "hidden_extractor_view",
        "visibility": "extractor only",
        "contains": [
            "unmasked role digits",
            "semantic residues",
            "range witnesses",
            "masks",
            "account-binding/link witnesses",
        ],
        "must_not_contain": [
            "post-query programmable values",
            "fields not authenticated by the parent commitment context",
        ],
    },
]


THEOREM_CLAUSES = [
    {
        "id": "same_parent_context",
        "claim": (
            "Both views are authenticated under one parent trace context binding "
            "proof_system_id, relation_id, column_profile, trace_commitment_root, "
            "role handles, leaf context, creation-proof binding, and matrix domain."
        ),
    },
    {
        "id": "view_consistency",
        "claim": (
            "Public masked values equal hidden unmasked values plus fresh masks "
            "under domain-separated row/column/query labels."
        ),
    },
    {
        "id": "hidden_relation_soundness",
        "claim": (
            "R_core_66 and account-binding predicates are checked on the hidden "
            "extractor view, not merely on public masked values."
        ),
    },
    {
        "id": "public_view_simulation",
        "claim": (
            "The public masked view and all opened authentication material are "
            "simulatable from public inputs and masked_projection."
        ),
    },
    {
        "id": "role_handle_uniqueness",
        "claim": (
            "A Path2CreationProofBindingV1 role handle cannot authenticate two "
            "different hidden role vectors under the same parent context except "
            "as inner_commitment_binding_break."
        ),
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_two_view_trace_theorem_interface.v1",
        "status": "two_view_interface_pinned_not_proved",
        "purpose": (
            "State the exact theorem shape needed after selecting masked public "
            "leaves. The proof needs two coherent views: a public masked view for "
            "query openings and a hidden extractor view for soundness."
        ),
        "views": VIEWS,
        "theorem_clauses": THEOREM_CLAUSES,
        "breaks": [
            "transparentProofSoundnessBreak",
            "inner_commitment_binding_break",
            "account_binding_break",
            "path2_sis_norm_or_range_break",
            "live_slot_leakage_or_keygen_break",
            "qrom_or_domain_separation_break",
        ],
        "selected_path": (
            "Continue the transparent route only if a concrete proof family can "
            "instantiate this two-view theorem with payload estimates below the "
            "fallback threshold."
        ),
        "next_blocker": (
            "Map the two-view theorem into the payload contract and extractor "
            "bridge so they no longer describe a single raw trace."
        ),
        "non_claims": [
            "this does not prove two-view consistency",
            "this does not instantiate the commitment scheme",
            "this does not alter spend signature bytes",
            "this does not make Path 2 deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Two-View Trace Theorem Interface",
        "",
        "Status: two-view interface pinned; not proved.",
        "",
        model["purpose"],
        "",
        "## Views",
        "",
        "| id | visibility | contains | must not contain |",
        "|---|---|---|---|",
    ]
    for item in model["views"]:
        contains = ", ".join(f"`{value}`" for value in item["contains"])
        must_not = ", ".join(f"`{value}`" for value in item["must_not_contain"])
        lines.append(
            "| `{}` | {} | {} | {} |".format(
                item["id"], item["visibility"], contains, must_not
            )
        )
    lines.extend(
        [
            "",
            "## Theorem Clauses",
            "",
            "| id | claim |",
            "|---|---|",
        ]
    )
    for item in model["theorem_clauses"]:
        lines.append("| `{id}` | {claim} |".format(**item))
    lines.extend(["", "## Breaks", ""])
    for item in model["breaks"]:
        lines.append(f"- `{item}`")
    lines.extend(
        [
            "",
            "## Selected Path",
            "",
            model["selected_path"],
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
