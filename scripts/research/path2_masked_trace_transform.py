#!/usr/bin/env python3
"""Generate the Path 2 masked trace transform for R_core_66."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_masked_trace_transform.json"
OUT_MD = ROOT / "research/path2_masked_trace_transform.md"


COLUMN_CLASSES = [
    {
        "class": "public",
        "examples": [
            "masked_projection[6]",
            "relation_id",
            "leaf_context_digest",
            "creation_proof_binding_digest",
            "matrix_domain_transcript_digest",
            "aggregation_context_digest",
        ],
        "query_policy": "may be opened directly",
    },
    {
        "class": "opened_masked",
        "examples": [
            "pad_source_digit_plus_mask",
            "projection_digit_plus_mask",
            "opening_tuple_digit_plus_mask",
            "masked role aggregate checks",
        ],
        "query_policy": "may be opened only if each value is statistically masked for that row/query domain",
    },
    {
        "class": "committed_hidden",
        "examples": [
            "unmasked base128 digits",
            "semantic residues",
            "role masks",
            "range witnesses",
            "account-binding witness lanes",
        ],
        "query_policy": "must not be opened raw",
    },
    {
        "class": "derived_public_relation",
        "examples": [
            "masked_projection_i = pad_i + projection_i mod q",
            "role-subroot equality flags",
            "range-check accept flags",
        ],
        "query_policy": "may be checked through ZK consistency, not by revealing hidden operands",
    },
]


TRANSFORM_ROWS = [
    {
        "id": "masked_projection_equations",
        "old_rows": 6,
        "masked_form": (
            "For each lane i, the committed hidden trace contains pad_i, proj_i, "
            "rho_pad_i, rho_proj_i, and rho_sum_i with rho_sum_i = rho_pad_i + "
            "rho_proj_i. Opened values may include pad_i + rho_pad_i, proj_i + "
            "rho_proj_i, and rho_sum_i only if rho_sum_i is itself simulatable or "
            "domain-separated one-time masked. The verifier-visible equation is "
            "(pad_i+rho_pad_i) + (proj_i+rho_proj_i) - rho_sum_i = "
            "masked_projection_i mod q."
        ),
        "leakage_condition": (
            "rho_pad_i, rho_proj_i, and any reusable decomposition of rho_sum_i "
            "must remain hidden; masks cannot repeat across accounts, slots, rows, "
            "or Fiat-Shamir query domains."
        ),
    },
    {
        "id": "digit_reconstruction",
        "old_rows": 20,
        "masked_form": (
            "Unmasked digit columns remain extractor witness columns. Query-visible "
            "digit openings must be digit_plus_mask values. Reconstruction is "
            "checked inside the ZK IOP against hidden unmasked digits and residues, "
            "or by an equivalent masked linear identity whose masks cancel without "
            "revealing digits."
        ),
        "leakage_condition": (
            "Opening both digit_plus_mask and digit_mask for the same cell is "
            "forbidden; opening multiple linear combinations must not solve for a "
            "digit."
        ),
    },
    {
        "id": "digit_range_checks",
        "old_rows": 40,
        "masked_form": (
            "Range membership applies to hidden unmasked digits. The proof may "
            "open masked auxiliary values only if the range gadget has a simulator "
            "that does not reveal the digit or a reusable range witness."
        ),
        "leakage_condition": (
            "Range witness openings are private unless the selected range gadget "
            "has a separate zero-knowledge proof of simulatable openings."
        ),
    },
    {
        "id": "role_subroot_links",
        "old_rows": 0,
        "masked_form": (
            "Role subroots authenticate the hidden unmasked role columns and any "
            "opened masked views as one typed commitment object. The extractor "
            "recovers unmasked role vectors from the committed trace; the public "
            "verifier sees only role handles and masked/query transcript bytes."
        ),
        "leakage_condition": (
            "Subroot openings must not reveal raw role leaves. If Merkle leaves "
            "contain raw hidden columns, this transform fails."
        ),
    },
]


def build_model() -> dict[str, Any]:
    old_lanes = 80
    minimum_mask_lanes = 40
    relation_mask_lanes = 18
    return {
        "schema": "Descender.path2_masked_trace_transform.v1",
        "status": "masked_transform_pinned_not_costed",
        "finding": (
            "The transparent proof can stay compact only if hidden role columns "
            "are never opened raw. R_core_66 must be interpreted as a hidden "
            "extractor relation plus a masked query view, not as a vanilla opened "
            "trace."
        ),
        "column_classes": COLUMN_CLASSES,
        "transform_rows": TRANSFORM_ROWS,
        "lane_floor": {
            "old_private_lanes": old_lanes,
            "minimum_digit_mask_lanes": minimum_mask_lanes,
            "minimum_relation_mask_lanes": relation_mask_lanes,
            "new_private_lane_floor": old_lanes + minimum_mask_lanes + relation_mask_lanes,
            "note": (
                "This is a lower bound before role-subroot metadata, range-gadget "
                "ZK auxiliaries, and account-binding columns. It means the old "
                "80-lane byte model is no longer the real candidate size."
            ),
        },
        "simulator_requirement": (
            "For every opened_masked value, there must be a simulator that samples "
            "the same distribution from public inputs, masked_projection, and "
            "transcript randomness without knowing pad-source, projection, "
            "opening-tuple, digit, range, or account-binding witnesses."
        ),
        "extractor_requirement": (
            "For every accepting transcript, the extractor obtains the unmasked "
            "role columns and masks from the committed trace, checks R_core_66 on "
            "the unmasked columns, and checks that the opened masked values are "
            "consistent with those columns and masks."
        ),
        "redesign_triggers": [
            "the proof family must open raw hidden trace leaves",
            "mask reuse creates solvable linear equations for any hidden digit or residue",
            "range checking requires opening unmasked digits or reusable range witnesses",
            "role subroot authentication cannot bind hidden unmasked columns and masked query views to one parent trace",
            "updated byte model exceeds the account-creation payload budget by more than the explicit fallback threshold",
        ],
        "next_blocker": (
            "Update the transparent byte model using the masked lane floor, then "
            "decide whether the increased proof size is still acceptable before "
            "spending effort on verifier implementation."
        ),
        "non_claims": [
            "this does not prove the simulator requirement",
            "this does not select a concrete range gadget",
            "this does not close role-subroot binding",
            "this does not update the final byte model yet",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Masked Trace Transform",
        "",
        "Status: masked transform pinned; not costed.",
        "",
        model["finding"],
        "",
        "## Column Classes",
        "",
        "| class | examples | query policy |",
        "|---|---|---|",
    ]
    for item in model["column_classes"]:
        examples = ", ".join(f"`{example}`" for example in item["examples"])
        lines.append(
            "| `{}` | {} | {} |".format(
                item["class"],
                examples,
                item["query_policy"],
            )
        )
    lines.extend(
        [
            "",
            "## Transform Rows",
            "",
            "| id | old rows | masked form | leakage condition |",
            "|---|---:|---|---|",
        ]
    )
    for item in model["transform_rows"]:
        lines.append(
            "| `{id}` | {old_rows} | {masked_form} | {leakage_condition} |".format(
                **item
            )
        )
    floor = model["lane_floor"]
    lines.extend(
        [
            "",
            "## Lane Floor",
            "",
            f"- old private lanes: `{floor['old_private_lanes']}`",
            f"- minimum digit mask lanes: `{floor['minimum_digit_mask_lanes']}`",
            f"- minimum relation mask lanes: `{floor['minimum_relation_mask_lanes']}`",
            f"- new private lane floor: `{floor['new_private_lane_floor']}`",
            f"- note: {floor['note']}",
            "",
            "## Simulator Requirement",
            "",
            model["simulator_requirement"],
            "",
            "## Extractor Requirement",
            "",
            model["extractor_requirement"],
            "",
            "## Redesign Triggers",
            "",
        ]
    )
    for item in model["redesign_triggers"]:
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
