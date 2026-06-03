#!/usr/bin/env python3
"""Generate no-export and non-public-link contracts for the private-ZK payload."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_descent_sis_private_zk_no_export_contract.json"
OUT_MD = ROOT / "research/path2_descent_sis_private_zk_no_export_contract.md"


PUBLIC_FIELDS = [
    "proof_system_id",
    "row_band",
    "leaf_context_digest",
    "creation_proof_binding_digest",
    "matrix_domain_transcript_digest",
    "aggregation_context_digest",
    "masked_projection[6]",
    "public_proof_messages",
    "zk_transcript_digest",
]

FORBIDDEN_PRIVATE_FIELDS = [
    "pad_source_opening",
    "pad_lane",
    "pad_seed",
    "projection_relation_opening",
    "account_projection_lane",
    "opening_tuple",
    "opening_scalar",
    "digit_or_range_witness",
    "private_linear_commitment_message",
    "private_blinding",
    "private_noise",
    "witness_frame",
    "mask_lane",
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_descent_sis_private_zk_no_export_contract.v1",
        "status": "no_export_and_non_public_link_contract_pinned",
        "public_serializer_allowlist": PUBLIC_FIELDS,
        "private_field_denylist": FORBIDDEN_PRIVATE_FIELDS,
        "non_public_link_rule": (
            "The verifier may see masked_projection and public proof messages, "
            "but must not see pad lanes or account-projection lanes separately. "
            "The equation masked_projection_i = pad_i + account_projection_i mod q "
            "is checked only inside the zero-knowledge relation."
        ),
        "reject_path_rule": (
            "Malformed payloads must reject using public parse/check state only. "
            "Reject errors, debug output, logs, and serialized diagnostics must "
            "not include private witness bytes, private field names with values, "
            "or proof-internal linear commitment messages."
        ),
        "required_negative_controls": [
            "public serializer contains every allowlisted field and no denylisted field",
            "rejecting malformed proof bytes does not serialize private witnesses",
            "rejecting wrong masked projection does not reveal pad or account-projection lanes",
            "rejecting wrong range proof does not reveal digit/range witnesses",
            "public projection shortcut proof containing pad/account_projection lanes rejects",
            "debug profile cannot opt into private payload dumps",
        ],
        "closure_gate": (
            "Add a typed Rust payload public-view serializer and KATs that scan "
            "accepted and rejected outputs for the denylist before marking "
            "`p2_sis_no_export` or `p2_sis_non_public_link_equation` closed."
        ),
        "non_claims": [
            "this does not implement the serializer",
            "this does not prove ZK simulation",
            "this does not close leakage for a future concrete proof family",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Descent/SIS Private-ZK No-Export Contract",
        "",
        "Status: no-export and non-public-link contract pinned.",
        "",
        "## Public Serializer Allowlist",
        "",
    ]
    for item in model["public_serializer_allowlist"]:
        lines.append(f"- `{item}`")
    lines.extend(["", "## Private Field Denylist", ""])
    for item in model["private_field_denylist"]:
        lines.append(f"- `{item}`")
    lines.extend(
        [
            "",
            "## Non-Public Link Rule",
            "",
            model["non_public_link_rule"],
            "",
            "## Reject Path Rule",
            "",
            model["reject_path_rule"],
            "",
            "## Required Negative Controls",
            "",
        ]
    )
    for item in model["required_negative_controls"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Closure Gate",
            "",
            model["closure_gate"],
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
