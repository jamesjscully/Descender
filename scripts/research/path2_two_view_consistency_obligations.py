#!/usr/bin/env python3
"""Generate Path 2 two-view consistency obligations."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_two_view_consistency_obligations.json"
OUT_MD = ROOT / "research/path2_two_view_consistency_obligations.md"


OBLIGATIONS = [
    {
        "id": "masked_leaf_equations",
        "risk": "highest",
        "statement": (
            "Every public masked leaf value must equal the corresponding hidden "
            "extractor-view value plus a fresh mask under q12289, with the role, "
            "row, column, query, account, and slot in the mask domain."
        ),
        "failure_break": "live_slot_leakage_or_keygen_break or transparentProofSoundnessBreak",
    },
    {
        "id": "mask_non_reuse_rank",
        "risk": "highest",
        "statement": (
            "For any set of opened masked leaves accepted under one or many "
            "transcripts, the public linear system in hidden values has rank zero "
            "after masks are existentially quantified, unless the adversary reuses "
            "a mask domain."
        ),
        "failure_break": "live_slot_leakage_or_keygen_break",
    },
    {
        "id": "hidden_relation_check",
        "risk": "high",
        "statement": (
            "R_core_66, range predicates, and account-binding predicates are "
            "checked on hidden extractor-view columns, not only on public masked "
            "columns."
        ),
        "failure_break": "transparentProofSoundnessBreak or creation_time_hidden_source_soundness_break",
    },
    {
        "id": "role_subroot_same_view_binding",
        "risk": "high",
        "statement": (
            "Role handles authenticate the hidden extractor-view role vectors and "
            "the public masked-view column sets as one linked object under the "
            "same parent context."
        ),
        "failure_break": "inner_commitment_binding_break",
    },
    {
        "id": "public_view_simulator",
        "risk": "high",
        "statement": (
            "The simulator can sample public masked leaves, role handles, paths, "
            "and consistency messages from public inputs and masked_projection "
            "without hidden extractor-view values."
        ),
        "failure_break": "live_slot_leakage_or_keygen_break",
    },
    {
        "id": "fiat_shamir_view_binding",
        "risk": "medium_high",
        "statement": (
            "Fiat-Shamir challenges bind both views' parent context before query "
            "selection, including role handles, column profile, trace root, leaf "
            "context, creation-proof binding, and matrix transcript."
        ),
        "failure_break": "qrom_or_domain_separation_break",
    },
]


PROBE_PLAN = [
    {
        "step": "linear_leakage_probe",
        "purpose": (
            "Model opened masked leaves as linear equations and verify that hidden "
            "digits/residues are not solvable under the proposed mask domains."
        ),
    },
    {
        "step": "view_binding_probe",
        "purpose": (
            "Check whether the role-handle layout can authenticate both public "
            "masked and hidden extractor column sets without exposing hidden leaves."
        ),
    },
    {
        "step": "range_gadget_probe",
        "purpose": (
            "Determine whether the range predicate can be checked on hidden "
            "digits while the public query view remains simulatable."
        ),
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_two_view_consistency_obligations.v1",
        "status": "two_view_consistency_obligations_pinned",
        "purpose": (
            "Split the two-view theorem into checkable proof obligations. This is "
            "the next blocker after mapping the two-view model into the payload "
            "contract and extractor bridge."
        ),
        "obligations": OBLIGATIONS,
        "probe_plan": PROBE_PLAN,
        "current_decision": (
            "Continue the transparent path only if the linear leakage probe and "
            "view-binding probe both pass. A failure means the proof family must "
            "fall back to private openings, explicit algebraic opening rows, or "
            "the larger MPC/hash-ZK route."
        ),
        "next_blocker": (
            "Run the linear leakage probe for the two-view masked leaf equations "
            "before selecting a concrete proof-family implementation."
        ),
        "non_claims": [
            "this does not prove two-view consistency",
            "this does not instantiate the simulator",
            "this does not update byte counts",
            "this does not make Path 2 deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Two-View Consistency Obligations",
        "",
        "Status: two-view consistency obligations pinned.",
        "",
        model["purpose"],
        "",
        "## Obligations",
        "",
        "| id | risk | statement | failure break |",
        "|---|---|---|---|",
    ]
    for item in model["obligations"]:
        lines.append(
            "| `{id}` | `{risk}` | {statement} | `{failure_break}` |".format(
                **item
            )
        )
    lines.extend(["", "## Probe Plan", "", "| step | purpose |", "|---|---|"])
    for item in model["probe_plan"]:
        lines.append("| `{step}` | {purpose} |".format(**item))
    lines.extend(
        [
            "",
            "## Current Decision",
            "",
            model["current_decision"],
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
