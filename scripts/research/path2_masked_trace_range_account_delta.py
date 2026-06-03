#!/usr/bin/env python3
"""Generate range/account-binding deltas for the Path 2 masked trace."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_masked_trace_range_account_delta.json"
OUT_MD = ROOT / "research/path2_masked_trace_range_account_delta.md"


DELTAS = [
    {
        "id": "range_witness_zk_masks",
        "lane_delta": 40,
        "why": (
            "The 40 existing digit/range rows cannot expose raw range witnesses. "
            "A conservative lower bound gives each range-witness lane a masked "
            "query view or simulator handle."
        ),
        "proof_status": "required_unless_range_gadget_has_native_zk",
    },
    {
        "id": "account_binding_link_lanes",
        "lane_delta": 16,
        "why": (
            "Opening-tuple semantics need link lanes for leaf context, packed-"
            "witness projection agreement, lifecycle/account witness flags, and "
            "role-subroot equality checks. The exact scalarization may reduce or "
            "increase this, but zero extra lanes is no longer a credible estimate."
        ),
        "proof_status": "required_for_full_payload_theorem",
    },
    {
        "id": "role_subroot_metadata",
        "lane_delta": 0,
        "why": (
            "Role handles are carried through Path2CreationProofBindingV1 and "
            "payload digests. No q-lane increase is counted here, but the public "
            "payload may need availability copies depending on the concrete proof "
            "format."
        ),
        "proof_status": "byte_metadata_not_q_lane",
    },
]


def build_model() -> dict[str, Any]:
    base_masked_lanes = 138
    delta_lanes = sum(item["lane_delta"] for item in DELTAS)
    return {
        "schema": "Descender.path2_masked_trace_range_account_delta.v1",
        "status": "conservative_delta_pinned_not_final",
        "base_masked_lane_floor": base_masked_lanes,
        "deltas": DELTAS,
        "total_delta_lanes": delta_lanes,
        "range_account_lane_floor": base_masked_lanes + delta_lanes,
        "finding": (
            "After adding conservative ZK range-witness masks and account-binding "
            "link lanes, the masked trace floor rises from 138 to 194 private "
            "lanes. This is still a planning floor, but it is a better next "
            "comparator than the old 80-lane model."
        ),
        "theorem_impact": [
            "range checks can remain private only if range-witness openings are masked or natively simulatable",
            "opening_tuple is not accepted as arbitrary short data; it must link to leaf context and packed-witness/role commitments",
            "role subroots stay a proof-system commitment assumption, not SHA preimage checks",
        ],
        "next_blocker": (
            "Propagate the 194-lane floor into the transparent byte model and set "
            "an explicit fallback threshold against the 16416-byte lattice-sigma "
            "floor and blockchain account-creation payload expectations."
        ),
        "non_claims": [
            "this is not a final column count",
            "this does not instantiate a range gadget",
            "this does not prove account-binding semantics",
            "this does not add spend-signature bytes",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Masked Trace Range/Account Delta",
        "",
        "Status: conservative delta pinned; not final.",
        "",
        model["finding"],
        "",
        f"- base masked lane floor: `{model['base_masked_lane_floor']}`",
        f"- total delta lanes: `{model['total_delta_lanes']}`",
        f"- range/account lane floor: `{model['range_account_lane_floor']}`",
        "",
        "## Deltas",
        "",
        "| id | lane delta | proof status | why |",
        "|---|---:|---|---|",
    ]
    for item in model["deltas"]:
        lines.append(
            "| `{id}` | {lane_delta} | `{proof_status}` | {why} |".format(
                **item
            )
        )
    lines.extend(["", "## Theorem Impact", ""])
    for item in model["theorem_impact"]:
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
