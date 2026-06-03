#!/usr/bin/env python3
"""Define the public bridge predicate for the projection-carrier fallback."""

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
    / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.md"
)
BREAK_REDUCTION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.json"
)
SEMANTIC_BRIDGE_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json"
)
MATRIX_BUILDER_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
)
BRIDGE_REPLAY_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.json"
)

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.public_bridge_predicate.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def lane_checks(lane_frame: list[dict[str, Any]]) -> list[dict[str, Any]]:
    checks = []
    for lane in lane_frame:
        checks.append(
            {
                "id": f"carrier_lane_{lane['lane']}_canonical_projection_check",
                "lane": lane["lane"],
                "byte_offset": lane["byte_offset"],
                "byte_length": lane["byte_length"],
                "encoding": lane["encoding"],
                "field_bound": f"decoded_u16 < {Q}",
                "projection_symbol": lane["projection_column_symbol"],
                "coefficient_mod_q": lane["coefficient_mod_q"],
                "predicate": (
                    f"decoded_lane[{lane['lane']}] == "
                    f"{lane['coefficient_mod_q']}*{lane['projection_column_symbol']} mod {Q}"
                ),
                "status": "specified",
            }
        )
    return checks


def mutation_tests(lanes: list[dict[str, Any]]) -> list[dict[str, Any]]:
    tests = []
    for lane in lanes:
        tests.append(
            {
                "id": f"mutate_lane_{lane['lane']}_canonical_or_value_byte",
                "target": f"bytes[{lane['byte_offset']}:{lane['byte_offset'] + lane['byte_length']}]",
                "expected_result": "reject_or_export_checked_core_or_response_link_break",
                "status": "specified_not_implemented",
            }
        )
    tests.extend(
        [
            {
                "id": "mutate_carrier_check_bytes",
                "target": "bytes[24:27]",
                "expected_result": "reject_before_acceptance",
                "status": "specified_not_implemented",
            },
            {
                "id": "mutate_transcript_binding_bytes",
                "target": "bytes[27:56]",
                "expected_result": "reject_before_acceptance",
                "status": "specified_not_implemented",
            },
        ]
    )
    return tests


def build_payload() -> dict[str, Any]:
    reduction = load_json(BREAK_REDUCTION_JSON)
    semantic = load_json(SEMANTIC_BRIDGE_JSON)
    matrix = load_json(MATRIX_BUILDER_JSON)
    replay = load_json(BRIDGE_REPLAY_JSON)
    lanes = semantic.get("lane_frame", [])
    checks = lane_checks(lanes)
    tests = mutation_tests(lanes)
    active_symbols = {
        f"{row['kind']}[{row['block']},{row['coord']}]"
        for row in matrix.get("active_columns", [])
    }
    lane_symbols = {row["projection_symbol"] for row in checks}
    layout_predicates = [
        {
            "id": "carrier_length",
            "status": "specified",
            "predicate": "len(projection_carrier) == 56",
        },
        {
            "id": "canonical_q12289_lanes",
            "status": "specified",
            "predicate": "all twelve two-byte little-endian lanes decode to integers below q=12289",
        },
        {
            "id": "lane_region",
            "status": "specified",
            "predicate": "bytes[0:24] are exactly the twelve canonical q12289 lanes",
        },
        {
            "id": "check_region",
            "status": "specified_not_implemented",
            "predicate": "bytes[24:27] bind carrier version, map id, and reserved fail-closed flags",
        },
        {
            "id": "transcript_binding_region",
            "status": "specified_not_implemented",
            "predicate": "bytes[27:56] bind checked-core transcript digest, projection matrix digest, and statement identity",
        },
    ]
    bridge_classes = [
        {
            "id": "carrier_to_projection",
            "status": "specified",
            "rows": len(checks),
            "closure": "already rank-replayed by bridge_exact_replay",
        },
        {
            "id": "projection_to_checked_core",
            "status": "open",
            "rows": 0,
            "closure": "must be discharged by checked_core_or_response_link_break export contract",
        },
        {
            "id": "projection_to_hidden_source_opening",
            "status": "open",
            "rows": 0,
            "closure": "must be discharged by hidden relation opening/binding break export contract",
        },
    ]
    digest_payload = {
        "reduction_digest": reduction.get("target", {}).get("reduction_digest"),
        "bridge_spec_digest": semantic.get("target", {}).get("bridge_spec_digest"),
        "matrix_sha256": matrix.get("target", {}).get("matrix_sha256"),
        "bridge_replay_digest": replay.get("target", {}).get("bridge_replay_digest"),
        "layout_predicates": layout_predicates,
        "lane_checks": checks,
        "bridge_classes": bridge_classes,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_public_bridge_predicate.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "public_bridge_predicate_specified_partial_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "checked_core_break_reduction": rel(BREAK_REDUCTION_JSON),
            "semantic_bridge_spec": rel(SEMANTIC_BRIDGE_JSON),
            "projection_matrix_builder": rel(MATRIX_BUILDER_JSON),
            "bridge_exact_replay": rel(BRIDGE_REPLAY_JSON),
            "checked_core_break_reduction_schema": reduction.get("schema"),
            "semantic_bridge_schema": semantic.get("schema"),
            "projection_matrix_schema": matrix.get("schema"),
            "bridge_exact_replay_schema": replay.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "predicate_digest": digest(digest_payload),
        },
        "predicate": {
            "layout_predicates": layout_predicates,
            "lane_checks": checks,
            "bridge_classes": bridge_classes,
            "negative_tests": tests,
        },
        "coverage": {
            "lane_checks_specified": len(checks),
            "lane_symbols_equal_active_projection_symbols": lane_symbols == active_symbols,
            "active_projection_symbols": sorted(active_symbols),
            "lane_projection_symbols": sorted(lane_symbols),
            "implemented_predicate_classes": 1,
            "open_predicate_classes": 2,
            "negative_tests_specified": len(tests),
        },
        "decision": {
            "can_accept_now": False,
            "can_run_exact_solver_now": False,
            "reason": "the public carrier-to-projection predicate is specified, but checked-core and hidden-source opening bridge exports remain open",
            "next_artifact": "hidden_source_fallback_projection_break_export_contract",
        },
        "structural_controls": [
            {
                "id": "all_twelve_lane_checks_specified",
                "status": "pass" if len(checks) == 12 else "fail",
                "observed": len(checks),
                "required": 12,
            },
            {
                "id": "lane_symbols_match_selected_projection_symbols",
                "status": "pass" if lane_symbols == active_symbols else "fail",
                "observed": sorted(lane_symbols),
                "required": sorted(active_symbols),
            },
            {
                "id": "acceptance_still_fail_closed",
                "status": "pass",
                "observed": False,
                "required": False,
            },
        ],
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    coverage = payload["coverage"]
    lines = [
        "# Hidden-Source Projection Public Bridge Predicate",
        "",
        "This artifact defines the public verifier predicate available for the",
        "projection-carrier fallback. It specifies the byte-level lane checks and",
        "names the two bridge classes that still require break-export contracts.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- predicate digest: `{target['predicate_digest']}`",
        f"- lane checks specified: `{coverage['lane_checks_specified']}`",
        f"- lane symbols match selected projection symbols: `{coverage['lane_symbols_equal_active_projection_symbols']}`",
        f"- negative tests specified: `{coverage['negative_tests_specified']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Layout Predicates",
        "",
        "| predicate | status | rule |",
        "|---|---|---|",
    ]
    for row in payload["predicate"]["layout_predicates"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['predicate']} |")
    lines.extend(
        [
            "",
            "## Lane Checks",
            "",
            "| lane | bytes | symbol | coefficient | status |",
            "|---:|---|---|---:|---|",
        ]
    )
    for row in payload["predicate"]["lane_checks"]:
        end = row["byte_offset"] + row["byte_length"]
        lines.append(
            f"| `{row['lane']}` | `{row['byte_offset']}:{end}` | "
            f"`{row['projection_symbol']}` | `{row['coefficient_mod_q']}` | "
            f"`{row['status']}` |"
        )
    lines.extend(
        [
            "",
            "## Bridge Classes",
            "",
            "| class | status | rows | closure |",
            "|---|---|---:|---|",
        ]
    )
    for row in payload["predicate"]["bridge_classes"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['rows']}` | {row['closure']} |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The public predicate now covers the carrier bytes that the verifier can",
            "check directly. It does not prove checked-core coupling by itself. The",
            "remaining closure must export concrete `checked_core_or_response_link_break`",
            "or hidden-source opening/binding break witnesses when these predicates are",
            "satisfied but the semantic bridge fails.",
            "",
            "## Source Artifacts",
            "",
        ]
    )
    for key, value in payload["source_artifacts"].items():
        lines.append(f"- `{key}`: `{value}`")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(f"wrote {rel(DEFAULT_JSON)}")
    print(f"wrote {rel(DEFAULT_MD)}")


if __name__ == "__main__":
    main()
