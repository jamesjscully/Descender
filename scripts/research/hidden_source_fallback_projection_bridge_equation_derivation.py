#!/usr/bin/env python3
"""Derive and classify bridge equations for the selected projection route."""

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
    / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.md"
)
BRIDGE_REPLAY_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.json"
)
SEMANTIC_BRIDGE_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json"
)
EXTRACTOR_THEOREM_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json"
)

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.projection_bridge_equation_derivation.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def lane_equation_rows(lane_frame: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for lane in lane_frame:
        rows.append(
            {
                "id": f"lane_{lane['lane']}_public_projection_definition",
                "class": "derived_public_definition",
                "status": "derived",
                "equation_mod_q": (
                    f"decoded_lane[{lane['lane']}] = "
                    f"{lane['coefficient_mod_q']}*{lane['projection_column_symbol']}"
                ),
                "depends_on": [
                    "canonical_q12289_lane_decoder",
                    "selected_projection_row_digest",
                    "carrier_transcript_binding",
                ],
                "adds_new_checked_core_coupling": False,
            }
        )
    return rows


def build_payload() -> dict[str, Any]:
    replay = load_json(BRIDGE_REPLAY_JSON)
    semantic = load_json(SEMANTIC_BRIDGE_JSON)
    extractor = load_json(EXTRACTOR_THEOREM_JSON)
    lane_rows = lane_equation_rows(semantic["lane_frame"])
    named_break_ids = [row["id"] for row in extractor.get("named_breaks", [])]
    derivation_rows = lane_rows + [
        {
            "id": "projection_to_checked_core_coupling",
            "class": "missing_algebraic_coupling",
            "status": "open",
            "equation_mod_q": "not_derived",
            "needed_equation_shape": "linear or named-nonlinear relation mapping checked_core_response_difference variables to account_witness_projection and pad_seed_projection variables",
            "acceptable_closure": [
                "derive explicit public equations and replay rank/bounded infeasibility",
                "replace with a named checked_core_or_response_link_break reduction",
            ],
            "candidate_named_break": "checked_core_or_response_link_break",
            "adds_new_checked_core_coupling": True,
        },
        {
            "id": "projection_to_hidden_source_opening_coupling",
            "class": "missing_semantic_coupling",
            "status": "open",
            "equation_mod_q": "not_derived",
            "needed_equation_shape": "relation proving selected projection coordinates are computed from the same account-witness and pad-source openings as the hidden-source statement",
            "acceptable_closure": [
                "derive explicit opening equations and replay semantic negative controls",
                "replace with a named hidden_relation_proof_soundness_break or account_commitment_binding_break reduction",
            ],
            "candidate_named_breaks": [
                "hidden_relation_proof_soundness_break",
                "account_commitment_binding_break",
                "pad_prf_or_pad_opening_break",
            ],
            "adds_new_checked_core_coupling": False,
        },
        {
            "id": "semantic_decoder_negative_controls",
            "class": "implementation_coupling",
            "status": "open",
            "equation_mod_q": "byte_predicates_not_equations",
            "needed_equation_shape": "strict byte decoder plus public rejection predicates for canonical lanes, map identity, transcript binding, and digest-only carriers",
            "acceptable_closure": [
                "implement fail-closed decoder and negative tests",
                "include decoder predicates in final exact replay preconditions",
            ],
            "adds_new_checked_core_coupling": False,
        },
    ]
    derived_count = sum(1 for row in derivation_rows if row["status"] == "derived")
    open_count = sum(1 for row in derivation_rows if row["status"] == "open")
    rank_impact = {
        "carrier_lane_definition_equations": len(lane_rows),
        "carrier_lane_definition_equations_derived": derived_count,
        "new_checked_core_coupling_equations_derived": 0,
        "open_coupling_classes": open_count,
        "current_nullity_before_missing_bridges": replay["rank_models"][0][
            "nullity_before_missing_bridges"
        ],
        "can_run_bounded_solver": False,
    }
    closure_plan = [
        {
            "step": "derive_checked_core_projection_map",
            "status": "next",
            "output": "explicit matrix rows or a named checked_core_or_response_link_break reduction",
        },
        {
            "step": "derive_hidden_source_opening_map",
            "status": "pending",
            "output": "explicit opening equations or named hidden_relation/account_commitment/pad break",
        },
        {
            "step": "implement_semantic_decoder_controls",
            "status": "pending",
            "output": "byte-level decoder plus negative tests for all semantic rejection predicates",
        },
        {
            "step": "run_final_bridge_bounded_solver",
            "status": "blocked",
            "output": "exact infeasible result or exported smallest bad vector",
        },
    ]
    digest_payload = {
        "bridge_replay_digest": replay["target"]["bridge_replay_digest"],
        "bridge_spec_digest": semantic["target"]["bridge_spec_digest"],
        "named_break_ids": named_break_ids,
        "derivation_rows": derivation_rows,
        "rank_impact": rank_impact,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_bridge_equation_derivation.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "selected_projection_bridge_equation_derivation_open_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "bridge_exact_replay": rel(BRIDGE_REPLAY_JSON),
            "semantic_bridge_spec": rel(SEMANTIC_BRIDGE_JSON),
            "extractor_theorem": rel(EXTRACTOR_THEOREM_JSON),
            "bridge_exact_replay_schema": replay.get("schema"),
            "semantic_bridge_schema": semantic.get("schema"),
            "extractor_theorem_schema": extractor.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "derivation_digest": digest(digest_payload),
        },
        "rank_impact": rank_impact,
        "derivation_rows": derivation_rows,
        "named_breaks_available": named_break_ids,
        "closure_plan": closure_plan,
        "structural_controls": [
            {
                "id": "all_lane_definition_equations_derived",
                "status": "pass" if derived_count == len(lane_rows) == 12 else "fail",
                "observed": derived_count,
                "required": 12,
            },
            {
                "id": "checked_core_coupling_not_faked",
                "status": "pass",
                "observed": rank_impact["new_checked_core_coupling_equations_derived"],
                "required": 0,
            },
            {
                "id": "bounded_solver_still_blocked",
                "status": "pass" if not rank_impact["can_run_bounded_solver"] else "fail",
                "observed": rank_impact["can_run_bounded_solver"],
                "required": False,
            },
        ],
        "decision": {
            "can_accept_now": False,
            "can_run_exact_solver_now": False,
            "reason": "public carrier lane equations are derived, but no equation yet couples projection variables to checked-core response differences or hidden-source openings",
            "next_artifact": "hidden_source_fallback_projection_checked_core_coupling_options",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    rank = payload["rank_impact"]
    lines = [
        "# Hidden-Source Projection Bridge Equation Derivation",
        "",
        "This artifact classifies which bridge equations are actually derived.",
        "The twelve carrier lane definition equations are derived from public",
        "serialization and the selected projection map.  They do not create the",
        "missing checked-core or hidden-source coupling.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- derivation digest: `{target['derivation_digest']}`",
        f"- lane equations derived: `{rank['carrier_lane_definition_equations_derived']}`",
        f"- new checked-core coupling equations derived: `{rank['new_checked_core_coupling_equations_derived']}`",
        f"- current nullity before missing bridges: `{rank['current_nullity_before_missing_bridges']}`",
        f"- can run bounded solver: `{rank['can_run_bounded_solver']}`",
        "",
        "## Derivation Rows",
        "",
        "| id | class | status | equation |",
        "|---|---|---|---|",
    ]
    for row in payload["derivation_rows"]:
        lines.append(
            f"| `{row['id']}` | `{row['class']}` | `{row['status']}` | `{row['equation_mod_q']}` |"
        )
    lines.extend(
        [
            "",
            "## Closure Plan",
            "",
            "| step | status | output |",
            "|---|---|---|",
        ]
    )
    for row in payload["closure_plan"]:
        lines.append(f"| `{row['step']}` | `{row['status']}` | {row['output']} |")
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
            f"- can run exact solver now: `{payload['decision']['can_run_exact_solver_now']}`",
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
