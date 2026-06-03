#!/usr/bin/env python3
"""Replay the exact-bridge boundary for the selected projection carrier."""

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
    / "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_bridge_exact_replay_n32_q12289.md"
)
SEMANTIC_BRIDGE_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_bridge_spec_n32_q12289.json"
)
CHECKED_CORE_LIFT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_checked_core_lift_n32_q12289.json"
)

Q = 12289
CHECKED_CORE_COLUMNS = 23
PROJECTION_COLUMNS = 64
CARRIER_LANES = 12


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rref_rank(rows: list[list[int]], cols: int) -> int:
    work = [[value % Q for value in row] for row in rows]
    rank = 0
    for col in range(cols):
        pivot = next((idx for idx in range(rank, len(work)) if work[idx][col] % Q), None)
        if pivot is None:
            continue
        work[rank], work[pivot] = work[pivot], work[rank]
        inv = pow(work[rank][col] % Q, -1, Q)
        work[rank] = [(value * inv) % Q for value in work[rank]]
        for row_idx in range(len(work)):
            if row_idx == rank or work[row_idx][col] % Q == 0:
                continue
            factor = work[row_idx][col] % Q
            work[row_idx] = [
                (work[row_idx][idx] - factor * work[rank][idx]) % Q
                for idx in range(cols)
            ]
        rank += 1
        if rank == len(work):
            break
    return rank


def projection_column(kind: str, block: int, coord: int) -> int:
    kind_base = 0 if kind == "account_witness_projection" else 32
    return kind_base + block * 2 + coord


def carrier_bridge_rows(lane_frame: list[dict[str, Any]]) -> list[list[int]]:
    """Rows over checked-core || projection || carrier-lane variables."""

    cols = CHECKED_CORE_COLUMNS + PROJECTION_COLUMNS + CARRIER_LANES
    rows: list[list[int]] = []
    for item in lane_frame:
        row = [0] * cols
        projection_idx = CHECKED_CORE_COLUMNS + projection_column(
            item["kind"], int(item["block"]), int(item["coord"])
        )
        carrier_idx = CHECKED_CORE_COLUMNS + PROJECTION_COLUMNS + int(item["lane"])
        row[projection_idx] = (-int(item["coefficient_mod_q"])) % Q
        row[carrier_idx] = 1
        rows.append(row)
    return rows


def replay_digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.projection_bridge_exact_replay.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def build_payload() -> dict[str, Any]:
    semantic = load_json(SEMANTIC_BRIDGE_JSON)
    lift = load_json(CHECKED_CORE_LIFT_JSON)
    lane_frame = semantic["lane_frame"]
    bridge_rows = carrier_bridge_rows(lane_frame)
    carrier_bridge_rank = rref_rank(
        bridge_rows, CHECKED_CORE_COLUMNS + PROJECTION_COLUMNS + CARRIER_LANES
    )
    lift_rank = lift["rank_accounting"]
    checked_core_rank = int(lift_rank["checked_core_combined_rank"])
    projection_rank = int(lift_rank["projection_rank"])

    # Model A is the verifier model: carrier lanes are public constants, so
    # lane equations are exactly the selected projection constraints already
    # represented by the projection rank.
    public_lane_model = {
        "model": "public_carrier_lane_constants",
        "variables": CHECKED_CORE_COLUMNS + PROJECTION_COLUMNS,
        "rank_before_missing_bridges": checked_core_rank + projection_rank,
        "nullity_before_missing_bridges": (
            CHECKED_CORE_COLUMNS + PROJECTION_COLUMNS - checked_core_rank - projection_rank
        ),
        "interpretation": "the 12 public carrier lanes impose the same 12 projection constraints counted by projection_rank; do not add carrier variables",
    }

    # Model B is a sanity model: if carrier differences are modeled as extra
    # variables, the lane equations add the same number of independent equations
    # and variables, so the bridge does not reduce nullity.
    variable_lane_model = {
        "model": "carrier_lane_delta_variables",
        "variables": CHECKED_CORE_COLUMNS + PROJECTION_COLUMNS + CARRIER_LANES,
        "carrier_bridge_equation_count": len(bridge_rows),
        "carrier_bridge_rank": carrier_bridge_rank,
        "rank_before_missing_bridges": checked_core_rank + projection_rank + carrier_bridge_rank,
        "nullity_before_missing_bridges": (
            CHECKED_CORE_COLUMNS
            + PROJECTION_COLUMNS
            + CARRIER_LANES
            - checked_core_rank
            - projection_rank
            - carrier_bridge_rank
        ),
        "interpretation": "use only as a double-counting guard; verifier acceptance treats lanes as public decoded values",
    }
    missing_bridge_obligations = [
        {
            "id": "projection_to_checked_core_equations",
            "status": "open",
            "needed_for": "connect the 64 projection-space variables to checked-core response-difference variables",
            "failure_mode": "projection carrier can be well-formed while independent of the checked-core response relation",
        },
        {
            "id": "projection_to_hidden_source_opening_equations",
            "status": "open",
            "needed_for": "prove the selected lanes force account-witness and pad-source openings or name a break",
            "failure_mode": "projection lanes may be public but semantically unrelated to the hidden-source witness relation",
        },
        {
            "id": "semantic_decoder_implementation",
            "status": "open",
            "needed_for": "turn specified rejection predicates into byte-level verifier code and negative tests",
            "failure_mode": "noncanonical or replayed carriers may pass outside the algebraic model",
        },
        {
            "id": "bounded_solver_on_instantiated_bridge",
            "status": "blocked",
            "needed_for": "run exact infeasibility or export a bad vector after missing bridge equations are instantiated",
            "failure_mode": "solver would currently certify an underconstrained direct sum",
        },
    ]
    structural_controls = [
        {
            "id": "all_carrier_lane_equations_rank_independent",
            "status": "pass" if carrier_bridge_rank == CARRIER_LANES else "fail",
            "observed": carrier_bridge_rank,
            "required": CARRIER_LANES,
        },
        {
            "id": "public_lane_model_matches_lift_nullity",
            "status": "pass"
            if public_lane_model["nullity_before_missing_bridges"]
            == int(lift_rank["direct_sum_nullity_without_bridges"])
            else "fail",
            "observed": public_lane_model["nullity_before_missing_bridges"],
            "required": int(lift_rank["direct_sum_nullity_without_bridges"]),
        },
        {
            "id": "variable_lane_model_does_not_falsely_reduce_nullity",
            "status": "pass"
            if variable_lane_model["nullity_before_missing_bridges"]
            == public_lane_model["nullity_before_missing_bridges"]
            else "fail",
            "observed": variable_lane_model["nullity_before_missing_bridges"],
            "required": public_lane_model["nullity_before_missing_bridges"],
        },
        {
            "id": "bounded_solver_not_run_on_underconstrained_bridge",
            "status": "pass",
            "observed": "blocked",
            "required": "do not run exact infeasibility until missing bridge equations are instantiated",
        },
    ]
    digest_payload = {
        "semantic_bridge_digest": semantic["target"]["bridge_spec_digest"],
        "lift_digest": lift["target"]["lift_digest"],
        "carrier_bridge_rank": carrier_bridge_rank,
        "public_lane_model": public_lane_model,
        "variable_lane_model": variable_lane_model,
        "missing_bridge_obligations": missing_bridge_obligations,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_bridge_exact_replay.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "selected_projection_bridge_replay_underconstrained_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "semantic_bridge_spec": rel(SEMANTIC_BRIDGE_JSON),
            "checked_core_lift": rel(CHECKED_CORE_LIFT_JSON),
            "semantic_bridge_schema": semantic.get("schema"),
            "checked_core_lift_schema": lift.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "bridge_replay_digest": replay_digest(digest_payload),
        },
        "carrier_bridge": {
            "equation_count": len(bridge_rows),
            "rank": carrier_bridge_rank,
            "columns_with_carrier_variables": CHECKED_CORE_COLUMNS
            + PROJECTION_COLUMNS
            + CARRIER_LANES,
        },
        "rank_models": [public_lane_model, variable_lane_model],
        "structural_controls": structural_controls,
        "missing_bridge_obligations": missing_bridge_obligations,
        "decision": {
            "can_accept_now": False,
            "exact_infeasibility_replay_has_run": False,
            "reason": "carrier lane equations replay cleanly, but they only bind bytes to selected projection coordinates; the projection-to-checked-core and projection-to-hidden-source bridges remain absent",
            "next_artifact": "hidden_source_fallback_projection_bridge_equation_derivation",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    carrier = payload["carrier_bridge"]
    lines = [
        "# Hidden-Source Projection Bridge Exact Replay",
        "",
        "This artifact replays the exact carrier-to-projection lane equations from",
        "the semantic bridge spec.  It deliberately does not run a bounded",
        "infeasibility solver: the bridge is still underconstrained until the",
        "projection-to-checked-core and projection-to-hidden-source equations are",
        "derived.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- bridge replay digest: `{target['bridge_replay_digest']}`",
        f"- carrier bridge equations: `{carrier['equation_count']}`",
        f"- carrier bridge rank: `{carrier['rank']}`",
        "",
        "## Rank Models",
        "",
        "| model | variables | rank before missing bridges | nullity before missing bridges | interpretation |",
        "|---|---:|---:|---:|---|",
    ]
    for row in payload["rank_models"]:
        lines.append(
            f"| `{row['model']}` | `{row['variables']}` | "
            f"`{row['rank_before_missing_bridges']}` | "
            f"`{row['nullity_before_missing_bridges']}` | {row['interpretation']} |"
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
            "## Missing Bridge Obligations",
            "",
            "| id | status | needed for | failure mode |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["missing_bridge_obligations"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | {row['needed_for']} | {row['failure_mode']} |"
        )
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- exact infeasibility replay has run: `{payload['decision']['exact_infeasibility_replay_has_run']}`",
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
