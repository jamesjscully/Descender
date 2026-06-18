#!/usr/bin/env python3
"""Generate two-view column profile rules for Path 2 masked traces."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_two_view_column_profile_rules.json"
OUT_MD = ROOT / "research/path2_two_view_column_profile_rules.md"


RULES = [
    {
        "id": "unique_mask_domain",
        "level": "mandatory",
        "rule": (
            "Every opened_masked cell has a mask domain including proof_system_id, "
            "account_id, slot_id, role_id, column_set_id, row_id, column_id, "
            "query_index, and transcript digest."
        ),
        "evidence": "path2_two_view_linear_leakage_probe shows mask reuse across different hidden values leaks rank",
    },
    {
        "id": "no_role_wide_masks",
        "level": "mandatory",
        "rule": (
            "A single mask cannot be shared by all digits in one role, nor by two "
            "roles, unless a separate simulator lemma proves no leakage for that "
            "specific linear system."
        ),
        "evidence": "all_digits_reuse_one_role_mask leaks 7 dimensions in the probe",
    },
    {
        "id": "hidden_view_not_public",
        "level": "mandatory",
        "rule": (
            "Columns marked committed_hidden are never serialized in public query "
            "openings, Merkle leaves, diagnostics, or reject reasons."
        ),
        "evidence": "raw hidden openings are a dead path in the proof-family gate",
    },
    {
        "id": "masked_view_columns_are_separate",
        "level": "mandatory",
        "rule": (
            "Public masked-view columns are separate from hidden extractor-view "
            "columns; the column profile must not identify a masked value column "
            "with its hidden source column."
        ),
        "evidence": "two-view theorem requires public masked view plus hidden extractor view",
    },
    {
        "id": "range_auxiliary_policy",
        "level": "mandatory",
        "rule": (
            "Range auxiliary columns are committed_hidden unless the selected "
            "range gadget has a native simulator lemma for their public openings."
        ),
        "evidence": "range witness exposure is explicitly denied by the masked transform",
    },
    {
        "id": "role_subroot_column_sets",
        "level": "mandatory",
        "rule": (
            "Each Path2CreationProofBindingV1 role handle names exactly one role "
            "id and a fixed list of hidden-view and masked-view column sets."
        ),
        "evidence": "prevents moving a valid role subroot to another role or column profile",
    },
    {
        "id": "unknown_column_reject",
        "level": "mandatory",
        "rule": (
            "Unknown role ids, duplicate role ids, unknown column classes, and "
            "column-profile extensions reject unless a new proof_system_id is used."
        ),
        "evidence": "prevents silent theorem drift under the same proof-system id",
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_two_view_column_profile_rules.v1",
        "status": "column_profile_rules_pinned",
        "source_probe": "research/path2_two_view_linear_leakage_probe.md",
        "rules": RULES,
        "profile_version_rule": (
            "Any change to role ids, column-set ids, mask-domain labels, or "
            "public/hidden column classes requires a new proof_system_id and "
            "column_profile value."
        ),
        "next_blocker": (
            "The concrete Rust column-profile enum and query-opening manifest now "
            "consume these rules. The next backend blocker is to make the "
            "unique-mask-domain manifest rule a proof obligation inside the "
            "concrete full-relation backend certificate, not only a "
            "verifier-side public-view/no-export check."
        ),
        "non_claims": [
            "this does not prove simulator security",
            "this does not model nonlinear range leakage",
            "this does not make Path 2 deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Two-View Column Profile Rules",
        "",
        "Status: column profile rules pinned.",
        "",
        f"Source probe: `{model['source_probe']}`.",
        "",
        "## Rules",
        "",
        "| id | level | rule | evidence |",
        "|---|---|---|---|",
    ]
    for item in model["rules"]:
        lines.append("| `{id}` | `{level}` | {rule} | {evidence} |".format(**item))
    lines.extend(
        [
            "",
            "## Profile Version Rule",
            "",
            model["profile_version_rule"],
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
