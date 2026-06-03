#!/usr/bin/env python3
"""Specify the semantic bridge for the selected projection-carrier route."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.md"
)
CHECKED_CORE_LIFT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_checked_core_lift_n32_q12289.json"
)
PROJECTION_MATRIX_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
)
PROJECTION_SPEC_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"
)

Q = 12289
CARRIER_BYTES = 56
LANE_BYTES = 2
LANE_COUNT = 12
CHECK_BYTES = 3
RESERVED_BYTES = CARRIER_BYTES - LANE_COUNT * LANE_BYTES - CHECK_BYTES


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def canonical_digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.projection_semantic_bridge_spec.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def lane_frame(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    frame = []
    for row in rows:
        lane = int(row["lane"])
        frame.append(
            {
                "lane": lane,
                "byte_offset": lane * LANE_BYTES,
                "byte_length": LANE_BYTES,
                "encoding": "little_endian_u16_canonical_q12289",
                "kind": row["kind"],
                "block": int(row["block"]),
                "coord": int(row["coord"]),
                "coefficient_mod_q": int(row["coefficient_mod_q"]),
                "projection_column_symbol": f"{row['kind']}[{int(row['block'])},{int(row['coord'])}]",
            }
        )
    return frame


def bridge_equation_templates(frame: list[dict[str, Any]]) -> list[dict[str, Any]]:
    equations = []
    for item in frame:
        equations.append(
            {
                "id": f"carrier_lane_{item['lane']}_projection_binding",
                "status": "specified_not_instantiated",
                "equation_mod_q": (
                    f"carrier_lane[{item['lane']}] - "
                    f"{item['coefficient_mod_q']}*{item['projection_column_symbol']} = 0"
                ),
                "variables": [
                    f"carrier_lane[{item['lane']}]",
                    item["projection_column_symbol"],
                ],
                "reject_if": [
                    "lane encoding is noncanonical",
                    "lane value is >= q",
                    "lane is not included in the projection transcript digest",
                ],
            }
        )
    return equations


def build_payload() -> dict[str, Any]:
    lift = load_json(CHECKED_CORE_LIFT_JSON)
    matrix = load_json(PROJECTION_MATRIX_JSON)
    projection_spec = load_json(PROJECTION_SPEC_JSON)
    selected_rows = projection_spec["selected_rows"]
    frame = lane_frame(selected_rows)
    equations = bridge_equation_templates(frame)
    byte_layout = {
        "carrier_bytes": CARRIER_BYTES,
        "lane_count": LANE_COUNT,
        "lane_bytes": LANE_BYTES,
        "lane_region_bytes": LANE_COUNT * LANE_BYTES,
        "check_bytes": CHECK_BYTES,
        "reserved_or_transcript_bytes": RESERVED_BYTES,
        "layout": [
            {
                "field": "projection_lanes",
                "offset": 0,
                "bytes": LANE_COUNT * LANE_BYTES,
                "semantics": "twelve canonical q12289 residues bound to selected projection rows",
            },
            {
                "field": "projection_check_bytes",
                "offset": LANE_COUNT * LANE_BYTES,
                "bytes": CHECK_BYTES,
                "semantics": "domain-separated carrier version, map id, and rejection-profile check bytes",
            },
            {
                "field": "reserved_or_transcript_binding",
                "offset": LANE_COUNT * LANE_BYTES + CHECK_BYTES,
                "bytes": RESERVED_BYTES,
                "semantics": "reserved for transcript binding material or must be fixed-zero by the decoder profile",
            },
        ],
    }
    semantic_rejection_predicates = [
        {
            "id": "canonical_q12289_lanes",
            "status": "specified_not_implemented",
            "rejects": [
                "any two-byte lane decoding to a value >= 12289",
                "short carrier",
                "long carrier",
            ],
        },
        {
            "id": "map_identity_binding",
            "status": "specified_not_implemented",
            "rejects": [
                "carrier check bytes do not bind the selected map digest",
                "carrier generated for an archived 284-byte RO-PoK route",
            ],
        },
        {
            "id": "checked_core_transcript_binding",
            "status": "specified_not_implemented",
            "rejects": [
                "carrier replayed across public keys",
                "carrier replayed across messages",
                "carrier replayed across checked-core signature bodies",
                "carrier replayed across account-state slots",
            ],
        },
        {
            "id": "fixed_public_rejection",
            "status": "specified_not_implemented",
            "rejects": [
                "secret-dependent decoder branch",
                "secret-dependent error code",
                "partial acceptance of malformed carrier",
            ],
        },
        {
            "id": "digest_only_carrier_rejection",
            "status": "specified_not_implemented",
            "rejects": [
                "carrier interpreted only as an opaque hash handle",
                "missing lane-by-lane projection residue checks",
            ],
        },
    ]
    bridge_groups = [
        {
            "id": "carrier_to_projection_lanes",
            "status": "specified",
            "equation_count": len(equations),
            "role": "binds serialized residues to selected hidden-source projection coordinates",
        },
        {
            "id": "projection_lanes_to_checked_core",
            "status": "open",
            "equation_count": "not_instantiated",
            "role": "must link projection variables to checked-core response differences or export a named break",
        },
        {
            "id": "projection_lanes_to_hidden_source_relation",
            "status": "open",
            "equation_count": "not_instantiated",
            "role": "must prove accepted lanes force account-witness and pad-source openings under the selected map",
        },
    ]
    digest_payload = {
        "source_lift_digest": lift["target"]["lift_digest"],
        "projection_matrix_sha256": matrix["target"]["matrix_sha256"],
        "row_digest": projection_spec["target"]["row_digest"],
        "byte_layout": byte_layout,
        "equations": equations,
        "semantic_rejection_predicates": semantic_rejection_predicates,
        "bridge_groups": bridge_groups,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_semantic_bridge_spec.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "selected_projection_semantic_bridge_spec_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "checked_core_lift": rel(CHECKED_CORE_LIFT_JSON),
            "projection_matrix": rel(PROJECTION_MATRIX_JSON),
            "projection_exact_certificate_spec": rel(PROJECTION_SPEC_JSON),
            "checked_core_lift_schema": lift.get("schema"),
            "projection_matrix_schema": matrix.get("schema"),
            "projection_exact_certificate_spec_schema": projection_spec.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "checked_core_signature_bytes": 252,
            "projection_carrier_bytes": CARRIER_BYTES,
            "envelope_bytes": 308,
            "bridge_spec_digest": canonical_digest(digest_payload),
        },
        "byte_layout": byte_layout,
        "lane_frame": frame,
        "bridge_equation_templates": equations,
        "semantic_rejection_predicates": semantic_rejection_predicates,
        "bridge_groups": bridge_groups,
        "structural_controls": [
            {
                "id": "carrier_byte_accounting",
                "status": "pass" if RESERVED_BYTES >= 0 and CARRIER_BYTES == 56 else "fail",
                "observed": CARRIER_BYTES,
                "required": 56,
            },
            {
                "id": "lane_count_matches_selected_rows",
                "status": "pass" if len(frame) == len(selected_rows) == LANE_COUNT else "fail",
                "observed": len(frame),
                "required": LANE_COUNT,
            },
            {
                "id": "all_lanes_have_equation_templates",
                "status": "pass" if len(equations) == LANE_COUNT else "fail",
                "observed": len(equations),
                "required": LANE_COUNT,
            },
            {
                "id": "checked_core_bridge_not_instantiated",
                "status": "expected_open",
                "observed": "projection_lanes_to_checked_core",
                "required": "must be instantiated before acceptance",
            },
        ],
        "decision": {
            "can_accept_now": False,
            "semantic_bridge_complete": False,
            "reason": "carrier-to-projection lane semantics are specified, but the projection-to-checked-core and projection-to-hidden-source equations are not instantiated",
            "next_artifact": "hidden_source_fallback_projection_bridge_exact_replay",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    layout = payload["byte_layout"]
    lines = [
        "# Hidden-Source Projection Semantic Bridge Specification",
        "",
        "This artifact fixes the byte-level and equation-template boundary for the",
        "selected 56-byte projection carrier.  It is not an accepting proof: the",
        "projection-to-checked-core and projection-to-hidden-source bridges remain",
        "open and must be instantiated by the exact replay artifact.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- bridge spec digest: `{target['bridge_spec_digest']}`",
        f"- carrier bytes: `{layout['carrier_bytes']}`",
        f"- projection lane bytes: `{layout['lane_region_bytes']}`",
        f"- check bytes: `{layout['check_bytes']}`",
        f"- reserved/transcript bytes: `{layout['reserved_or_transcript_bytes']}`",
        "",
        "## Lane Frame",
        "",
        "| lane | offset | kind | block | coord | coeff mod q |",
        "|---:|---:|---|---:|---:|---:|",
    ]
    for row in payload["lane_frame"]:
        lines.append(
            f"| `{row['lane']}` | `{row['byte_offset']}` | `{row['kind']}` | "
            f"`{row['block']}` | `{row['coord']}` | `{row['coefficient_mod_q']}` |"
        )
    lines.extend(
        [
            "",
            "## Bridge Groups",
            "",
            "| id | status | equation count | role |",
            "|---|---|---:|---|",
        ]
    )
    for row in payload["bridge_groups"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['equation_count']}` | {row['role']} |"
        )
    lines.extend(
        [
            "",
            "## Semantic Rejection Predicates",
            "",
            "| id | status | rejects |",
            "|---|---|---|",
        ]
    )
    for row in payload["semantic_rejection_predicates"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | {'; '.join(row['rejects'])} |"
        )
    lines.extend(
        [
            "",
            "## Structural Controls",
            "",
            "| id | status | observed | required |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["structural_controls"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['observed']}` | `{row['required']}` |"
        )
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- semantic bridge complete: `{payload['decision']['semantic_bridge_complete']}`",
            f"- reason: {payload['decision']['reason']}",
            f"- next artifact: `{payload['decision']['next_artifact']}`",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
