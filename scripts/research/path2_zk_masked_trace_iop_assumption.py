#!/usr/bin/env python3
"""Generate the Path 2 ZK masked-trace IOP assumption statement."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_zk_masked_trace_iop_assumption.json"
OUT_MD = ROOT / "research/path2_zk_masked_trace_iop_assumption.md"


ASSUMPTION_CLAUSES = [
    {
        "id": "binding_extractability",
        "claim": (
            "For fixed public inputs and transcript roots, an accepting proof has "
            "an extractor that outputs one parent trace and authenticated typed "
            "role subtraces, or exports transparentProofSoundnessBreak."
        ),
        "needed_for": "turns payload acceptance into a witness object rather than an after-the-fact satisfying tuple",
    },
    {
        "id": "role_subcommitment_binding",
        "claim": (
            "A role handle in Path2CreationProofBindingV1 cannot authenticate two "
            "different role vectors under the same parent trace root except as "
            "inner_commitment_binding_break."
        ),
        "needed_for": "binds R_core_66 witness lanes to account-creation public objects",
    },
    {
        "id": "masked_query_zero_knowledge",
        "claim": (
            "The public transcript distribution is simulatable from public inputs, "
            "masked_projection, trace shape, and proof-size metadata without "
            "hidden pad/projection/opening/range/account witnesses."
        ),
        "needed_for": "prevents account-creation proof bytes from changing live-slot leakage",
    },
    {
        "id": "mask_freshness",
        "claim": (
            "Opened masked values use domain-separated one-time masks or a "
            "simulatable equivalent; reuse across rows, slots, accounts, roles, "
            "or query domains exports live_slot_leakage_or_keygen_break."
        ),
        "needed_for": "rules out linear recovery from multiple masked openings",
    },
    {
        "id": "fiat_shamir_binding",
        "claim": (
            "All query indices and proof challenges are derived after binding the "
            "leaf context, creation-proof binding digest, parent trace root, role "
            "handles, matrix-domain transcript, aggregation context, and proof "
            "system id."
        ),
        "needed_for": "prevents rebinding a valid proof across account/state/slot contexts",
    },
]


FAILBACKS = [
    {
        "condition": "raw hidden trace leaves must be opened",
        "action": "reject this proof family; vanilla Merkle/FRI is a negative control",
    },
    {
        "condition": "typed role subcommitments are unavailable",
        "action": "add explicit algebraic opening rows or move trace roots/role roots into the binding object",
    },
    {
        "condition": "range-gadget ZK requires too many auxiliary openings",
        "action": "rerun byte model and compare against lattice-sigma or MPC-in-the-head fallback",
    },
    {
        "condition": "concrete proof overhead exceeds the fallback threshold",
        "action": "stop verifier work and revisit the proof-family decision",
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_zk_masked_trace_iop_assumption.v1",
        "status": "assumption_statement_pinned_not_proved",
        "assumption_name": "zkMaskedTraceIOPBindingAndHiding",
        "statement": (
            "For the Path 2 transparent account-creation payload relation, the "
            "selected proof family is a transparent post-quantum IOP whose "
            "accepted transcripts are knowledge-sound for a parent committed "
            "masked trace with typed role subcommitments, and whose public query "
            "openings are zero-knowledge for all hidden role, range, mask, and "
            "account-binding columns."
        ),
        "clauses": ASSUMPTION_CLAUSES,
        "consumed_by": [
            "direct_replayable_relation_certificate_soundness_or_break",
            "transparent_relation_knowledge_soundness",
            "query_opening_zero_knowledge_no_export",
            "public_binding_and_leaf_context_composition",
        ],
        "current_byte_floor": {
            "source": "research/path2_transparent_proof_byte_model.md",
            "range_account_lane_floor": 194,
            "floor_256_rows": 6144,
            "floor_512_rows": 8304,
            "floor_1024_rows": 10720,
        },
        "fallback_threshold": (
            "If concrete masked-trace proof overhead approaches or exceeds the "
            "16416-byte lattice-sigma repetition floor without materially better "
            "assumption quality, stop this route before verifier implementation."
        ),
        "failbacks": FAILBACKS,
        "non_claims": [
            "this does not prove the assumption",
            "this does not identify a concrete proof-system implementation",
            "this does not finalize query counts",
            "this does not make Path 2 deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    floor = model["current_byte_floor"]
    lines = [
        "# Path 2 ZK Masked-Trace IOP Assumption",
        "",
        "Status: assumption statement pinned; not proved.",
        "",
        f"Assumption: `{model['assumption_name']}`",
        "",
        model["statement"],
        "",
        "## Clauses",
        "",
        "| id | claim | needed for |",
        "|---|---|---|",
    ]
    for item in model["clauses"]:
        lines.append("| `{id}` | {claim} | {needed_for} |".format(**item))
    lines.extend(["", "## Consumed By", ""])
    for item in model["consumed_by"]:
        lines.append(f"- `{item}`")
    lines.extend(
        [
            "",
            "## Current Byte Floor",
            "",
            f"- source: `{floor['source']}`",
            f"- range/account lane floor: `{floor['range_account_lane_floor']}`",
            f"- 256-row floor: `{floor['floor_256_rows']}` bytes",
            f"- 512-row floor: `{floor['floor_512_rows']}` bytes",
            f"- 1024-row floor: `{floor['floor_1024_rows']}` bytes",
            "",
            "## Fallback Threshold",
            "",
            model["fallback_threshold"],
            "",
            "## Failbacks",
            "",
            "| condition | action |",
            "|---|---|",
        ]
    )
    for item in model["failbacks"]:
        lines.append("| {condition} | {action} |".format(**item))
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
