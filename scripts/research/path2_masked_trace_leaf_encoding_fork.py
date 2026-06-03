#!/usr/bin/env python3
"""Generate the Path 2 masked-trace leaf encoding fork."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_masked_trace_leaf_encoding_fork.json"
OUT_MD = ROOT / "research/path2_masked_trace_leaf_encoding_fork.md"


OPTIONS = [
    {
        "id": "masked_public_leaves",
        "verdict": "preferred_first_probe",
        "description": (
            "Merkle/authentication leaves that may be opened publicly contain only "
            "masked role values, public values, and domain-separated metadata. "
            "Unmasked role values are extracted through the proof-family witness "
            "commitment theorem, not by opening public leaves."
        ),
        "proof_burden": (
            "Need mask-consistency constraints tying masked leaves to the hidden "
            "extractor trace and proving R_core_66 on unmasked values."
        ),
        "byte_effect": "adds mask columns and consistency openings already approximated by the 194-lane floor",
        "risk": (
            "Extractor theorem is more subtle: the public Merkle leaves do not by "
            "themselves contain the unmasked witness."
        ),
    },
    {
        "id": "hidden_raw_leaves_with_zk_opening",
        "verdict": "viable_if_proof_family_supports_private_openings",
        "description": (
            "The trace commitment binds raw hidden leaves, but the proof system "
            "never opens them directly; it proves queried consistency through a "
            "ZK opening protocol."
        ),
        "proof_burden": (
            "Requires a transparent commitment scheme with private openings or "
            "simulatable query answers, which is stronger than a vanilla Merkle "
            "tree."
        ),
        "byte_effect": "unknown; could exceed current masked floors",
        "risk": (
            "If implemented as ordinary Merkle authentication, it collapses back "
            "to the rejected raw-opening path."
        ),
    },
    {
        "id": "raw_public_leaves",
        "verdict": "dead_path",
        "description": (
            "Public authentication paths expose raw hidden role leaves when the "
            "verifier queries the trace."
        ),
        "proof_burden": "none; reject",
        "byte_effect": "small but insecure",
        "risk": "directly violates live-slot no-export",
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_masked_trace_leaf_encoding_fork.v1",
        "status": "masked_leaf_encoding_preferred",
        "finding": (
            "Typed role subroots are not enough. The leaf encoding under those "
            "subroots determines whether query authentication leaks hidden values. "
            "Ordinary raw Merkle leaves are a dead path."
        ),
        "options": OPTIONS,
        "selected_probe": "masked_public_leaves",
        "required_checks": [
            "opened leaves contain no unmasked hidden role, range, mask seed, or account-binding witness values",
            "masked leaves are domain-separated by account, slot, role, row, column, and query context",
            "mask-consistency constraints do not reveal reusable mask equations",
            "extractor theorem still recovers unmasked role vectors under the parent root",
            "simulator can sample opened masked leaves and paths without hidden witnesses",
        ],
        "theorem_delta": (
            "The role subcommitment binding theorem must bind both the public "
            "masked leaf view and the hidden extractor trace. Soundness uses the "
            "hidden trace; zero knowledge uses the masked public leaf view."
        ),
        "next_blocker": (
            "State the two-view trace theorem: one parent commitment authenticates "
            "a public masked view and a hidden extractor view, with ZK consistency "
            "between them."
        ),
        "non_claims": [
            "this does not implement a Merkle layout",
            "this does not prove two-view consistency",
            "this does not update byte floors beyond the 194-lane planning floor",
            "this does not make Path 2 deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Masked-Trace Leaf Encoding Fork",
        "",
        "Status: masked leaf encoding preferred.",
        "",
        model["finding"],
        "",
        "## Options",
        "",
        "| id | verdict | description | proof burden | byte effect | risk |",
        "|---|---|---|---|---|---|",
    ]
    for item in model["options"]:
        lines.append(
            "| `{id}` | `{verdict}` | {description} | {proof_burden} | {byte_effect} | {risk} |".format(
                **item
            )
        )
    lines.extend(["", "## Selected Probe", "", f"`{model['selected_probe']}`", "", "## Required Checks", ""])
    for item in model["required_checks"]:
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
