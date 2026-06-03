#!/usr/bin/env python3
"""Generate the Path 2 descent/SIS private-ZK Fiat-Shamir/QROM frame."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_descent_sis_private_zk_qrom_frame.json"
OUT_MD = ROOT / "research/path2_descent_sis_private_zk_qrom_frame.md"


FRAME_FIELDS = [
    ("domain", "DESCENDER_PATH2_DESCENT_SIS_PRIVATE_ZK_FS_V1", "separates this payload proof from checked-core, leaf, and matrix derivation domains"),
    ("proof_system_id", "32 bytes", "names the concrete ZK proof family and parameter set"),
    ("row_band", "u16 little-endian", "binds the 48/64-row matrix choice"),
    ("matrix_domain_transcript_digest", "32 bytes", "binds role-separated matrix domains and row band"),
    ("leaf_context_digest", "32 bytes", "binds account/slot/state/lifecycle public context"),
    ("creation_proof_binding_digest", "32 bytes", "binds Path2CreationProofBindingV1"),
    ("masked_projection", "six canonical q12289 residues", "public quotient target"),
    ("aggregation_context_digest", "32 bytes", "fixed even in direct mode so future aggregation does not reframe the direct theorem"),
    ("public_commitment_digest", "32 bytes", "hash of only public proof commitments/messages"),
    ("round_index", "u16 little-endian", "separates repeated FS challenges when the selected proof family requires repetition"),
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_descent_sis_private_zk_qrom_frame.v1",
        "status": "qrom_frame_contract_pinned",
        "frame_fields": [
            {"field": field, "encoding": encoding, "role": role}
            for field, encoding, role in FRAME_FIELDS
        ],
        "challenge_derivation": (
            "challenge_round_i = SHA256(frame fields in order) reduced by the "
            "selected proof family's challenge sampler; rejection sampling must "
            "be domain-separated from matrix generation"
        ),
        "query_budget_symbols": [
            "Q_path2_private_zk_fs_round",
            "Q_path2_private_zk_simulator",
            "Q_path2_private_zk_extractor_rewind",
        ],
        "required_negative_controls": [
            "mutating leaf_context_digest changes every derived challenge",
            "mutating creation_proof_binding_digest changes every derived challenge",
            "mutating matrix_domain_transcript_digest changes every derived challenge",
            "mutating row_band changes every derived challenge",
            "mutating aggregation_context_digest changes every derived challenge",
            "proof bytes from another proof_system_id reject",
            "round_index reuse inside a repeated proof rejects or exports qrom_or_domain_separation_break",
        ],
        "closure_gate": (
            "Add Rust constants/KATs once the proof family is selected, then add "
            "these query symbols to the Path 2 hash/QROM ledger."
        ),
        "non_claims": [
            "this does not choose the concrete proof family",
            "this does not quantify QROM loss",
            "this does not replace checked-core challenge framing",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Descent/SIS Private-ZK QROM Frame",
        "",
        "Status: QROM frame contract pinned.",
        "",
        "## Frame Fields",
        "",
        "| field | encoding | role |",
        "|---|---|---|",
    ]
    for field in model["frame_fields"]:
        lines.append("| {field} | {encoding} | {role} |".format(**field))
    lines.extend(
        [
            "",
            "## Challenge Derivation",
            "",
            model["challenge_derivation"],
            "",
            "## Query Budget Symbols",
            "",
        ]
    )
    for item in model["query_budget_symbols"]:
        lines.append(f"- `{item}`")
    lines.extend(["", "## Required Negative Controls", ""])
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
