#!/usr/bin/env python3
"""Generate the Path 2 masked-trace commitment layout."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_masked_trace_commitment_layout.json"
OUT_MD = ROOT / "research/path2_masked_trace_commitment_layout.md"


ROLE_LAYOUT = [
    {
        "role": "pad_source",
        "role_id": 1,
        "binding_field": "pad_source_commitment",
        "column_sets": [
            "pad_source_digits_hidden",
            "pad_source_digit_masks",
            "pad_source_opened_masked_view",
        ],
        "leaf_visibility": "hidden columns never appear as raw Merkle leaves in public openings",
    },
    {
        "role": "projection_relation",
        "role_id": 2,
        "binding_field": "projection_relation_commitment",
        "column_sets": [
            "projection_digits_hidden",
            "projection_digit_masks",
            "projection_opened_masked_view",
        ],
        "leaf_visibility": "hidden columns never appear as raw Merkle leaves in public openings",
    },
    {
        "role": "opening_tuple",
        "role_id": 3,
        "binding_field": "opening_tuple_commitment",
        "column_sets": [
            "opening_tuple_digits_hidden",
            "opening_tuple_digit_masks",
            "account_binding_link_lanes",
            "opening_tuple_opened_masked_view",
        ],
        "leaf_visibility": "hidden columns and account-binding witnesses stay private",
    },
    {
        "role": "range",
        "role_id": 4,
        "binding_field": "implicit_under_column_profile",
        "column_sets": [
            "range_witnesses_hidden",
            "range_witness_masks_or_simulator_handles",
        ],
        "leaf_visibility": "range witnesses stay private unless the range gadget proves native ZK",
    },
    {
        "role": "packed_witness_projection",
        "role_id": 5,
        "binding_field": "packed_witness_commitment",
        "column_sets": [
            "packed_witness_projection_link",
            "masked_projection_public_lanes",
        ],
        "leaf_visibility": "only masked_projection public lanes are opened directly",
    },
]


ROOTS = [
    {
        "name": "trace_commitment_root",
        "meaning": (
            "Parent commitment to all role subroots and column-set descriptors for "
            "one trace shape."
        ),
        "public": True,
    },
    {
        "name": "constraint_commitment_root",
        "meaning": (
            "Commitment to quotient/constraint trace used by the transparent IOP; "
            "must share the same transcript context."
        ),
        "public": True,
    },
    {
        "name": "role_subroot",
        "meaning": (
            "Domain-separated authenticated subcommitment for a role id and column "
            "set. For roles with Path2CreationProofBindingV1 fields, this is the "
            "semantic meaning of the 32-byte binding field."
        ),
        "public": "through Path2CreationProofBindingV1 digest or payload availability",
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_masked_trace_commitment_layout.v1",
        "status": "interface_layout_pinned_not_implemented",
        "purpose": (
            "Define the minimal commitment layout needed for the active "
            "zk_masked_trace_iop candidate to satisfy typed_role_subcommitment_api "
            "without making role commitments into SHA preimage claims."
        ),
        "roots": ROOTS,
        "role_layout": ROLE_LAYOUT,
        "canonical_role_handle": (
            "role_handle = H(domain || proof_system_id || relation_id || "
            "column_profile || trace_commitment_root || role_id || column_set_id "
            "|| role_subroot). The handle is stored in "
            "Path2CreationProofBindingV1 for pad_source, projection_relation, and "
            "opening_tuple roles."
        ),
        "parent_root_rule": (
            "trace_commitment_root authenticates every role_subroot and column-set "
            "descriptor. An accepted transcript must reject if a role handle names "
            "a role_subroot not authenticated under the parent root."
        ),
        "opening_policy": [
            "public columns may open raw",
            "opened_masked columns may open only masked values",
            "committed_hidden columns are extractor-only",
            "role_subroot authentication paths may be public only if leaves do not contain raw hidden values",
        ],
        "extractor_theorem_hook": (
            "The extractor first binds the parent root and role handles, then "
            "extracts unmasked role vectors from committed_hidden columns and "
            "checks that opened_masked leaves are consistent masked views of those "
            "vectors."
        ),
        "simulator_theorem_hook": (
            "The simulator programs role handles, roots, masked openings, and "
            "paths with the same public distribution while never using hidden "
            "role vectors. This requires role handles to be hiding commitments, "
            "not deterministic hashes of raw role leaves."
        ),
        "open_risk": (
            "If role_handle is a deterministic hash of role_subroot and role_subroot "
            "is itself a Merkle root over raw hidden leaves, the handle is binding "
            "but not automatically hiding under query openings. The concrete proof "
            "family must define masked leaves or commitment blinding."
        ),
        "next_blocker": (
            "Define the candidate leaf encoding: masked leaves versus hidden leaves "
            "plus ZK consistency commitments. This determines whether the simulator "
            "theorem is plausible."
        ),
        "non_claims": [
            "this does not implement Merkle trees or FRI",
            "this does not prove role_handle hiding",
            "this does not finalize public payload bytes",
            "this does not make Path 2 deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Masked-Trace Commitment Layout",
        "",
        "Status: interface layout pinned; not implemented.",
        "",
        model["purpose"],
        "",
        "## Roots",
        "",
        "| name | public | meaning |",
        "|---|---|---|",
    ]
    for item in model["roots"]:
        lines.append("| `{name}` | `{public}` | {meaning} |".format(**item))
    lines.extend(
        [
            "",
            "## Role Layout",
            "",
            "| role | id | binding field | column sets | leaf visibility |",
            "|---|---:|---|---|---|",
        ]
    )
    for item in model["role_layout"]:
        column_sets = ", ".join(f"`{col}`" for col in item["column_sets"])
        lines.append(
            "| `{}` | {} | `{}` | {} | {} |".format(
                item["role"],
                item["role_id"],
                item["binding_field"],
                column_sets,
                item["leaf_visibility"],
            )
        )
    lines.extend(
        [
            "",
            "## Canonical Role Handle",
            "",
            model["canonical_role_handle"],
            "",
            "## Parent Root Rule",
            "",
            model["parent_root_rule"],
            "",
            "## Opening Policy",
            "",
        ]
    )
    for item in model["opening_policy"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Extractor Theorem Hook",
            "",
            model["extractor_theorem_hook"],
            "",
            "## Simulator Theorem Hook",
            "",
            model["simulator_theorem_hook"],
            "",
            "## Open Risk",
            "",
            model["open_risk"],
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
