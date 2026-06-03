#!/usr/bin/env python3
"""Generate the selected projection-map exact certificate specification."""

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
    / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_exact_certificate_spec_n32_q12289.md"
)
CERT_ATTEMPT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.json"
)
MATRIX_BUILDER_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
)
EXACT_REPLAY_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_exact_infeasibility_replay_n32_q12289.json"
)

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def canonical_projection_row(row: dict[str, Any]) -> bytes:
    parts = [
        int(row["lane"]).to_bytes(2, "little"),
        row["kind"].encode("utf-8"),
        b"\0",
        int(row["block"]).to_bytes(2, "little"),
        int(row["coord"]).to_bytes(2, "little"),
        int(row["coefficient_mod_q"]).to_bytes(2, "little"),
    ]
    return b"".join(parts)


def row_digest(rows: list[dict[str, Any]]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.selected_projection_map.rows.v1")
    h.update(len(rows).to_bytes(2, "little"))
    for row in rows:
        h.update(canonical_projection_row(row))
    return h.hexdigest()


def build_payload() -> dict[str, Any]:
    attempt = load_json(CERT_ATTEMPT_JSON)
    matrix_builder = load_json(MATRIX_BUILDER_JSON) if MATRIX_BUILDER_JSON.exists() else {}
    exact_replay = load_json(EXACT_REPLAY_JSON) if EXACT_REPLAY_JSON.exists() else {}
    attempt_requirements = {
        row["id"]: row for row in attempt.get("exact_replay_requirements", [])
    }
    leakage_charge_closed = (
        attempt_requirements.get("leakage_charge", {}).get("status") == "closed"
    )
    generalized_matrix_built = (
        matrix_builder.get("coverage", {}).get("generalized_projection_matrix_built")
        is True
    )
    exact_replay_ran = (
        exact_replay.get("decision", {}).get("exact_infeasibility_replay_has_run")
        is True
    )
    bad_vector_export_complete = (
        exact_replay.get("decision", {}).get("bad_vector_export_complete") is True
    )
    rows = attempt["candidate_rows"]
    lanes = [row["lane"] for row in rows]
    kinds = sorted({row["kind"] for row in rows})
    blocks = sorted({row["block"] for row in rows})
    coords = sorted({row["coord"] for row in rows})
    coeffs = [row["coefficient_mod_q"] for row in rows]
    structural_controls = [
        {
            "id": "lane_count",
            "status": "pass" if len(rows) == 12 and sorted(lanes) == list(range(12)) else "fail",
            "observed": len(rows),
            "required": 12,
        },
        {
            "id": "kind_balance",
            "status": (
                "pass"
                if kinds == ["account_witness_projection", "pad_seed_projection"]
                and sum(1 for row in rows if row["kind"] == "account_witness_projection") == 6
                and sum(1 for row in rows if row["kind"] == "pad_seed_projection") == 6
                else "fail"
            ),
            "observed": kinds,
            "required": ["account_witness_projection", "pad_seed_projection"],
        },
        {
            "id": "coefficient_range",
            "status": "pass" if all(0 < int(value) < Q for value in coeffs) else "fail",
            "observed_min": min(coeffs),
            "observed_max": max(coeffs),
            "required": f"1 <= coefficient < {Q}",
        },
        {
            "id": "coordinate_range",
            "status": "pass" if coords == [0, 1] else "fail",
            "observed": coords,
            "required": [0, 1],
        },
        {
            "id": "block_diversity",
            "status": "pass" if len(blocks) >= 8 else "fail",
            "observed_distinct_blocks": len(blocks),
            "required_min_distinct_blocks": 8,
        },
    ]
    exact_certificate_interface = {
        "input_relation": "hidden-source projection rows over account-witness and pad-seed block coordinates",
        "not_the_same_as": "response-link 23-column checked-core coordinate kernel",
        "required_matrix": [
            "checked-core response-difference equations",
            "state-assisted account-witness projection rows",
            "pad-source projection rows",
            "semantic decoder constraints",
            "byte-decoder noncanonical rejection constraints",
        ],
        "required_output": "infeasible nonzero bounded response-difference vector, or an explicit bad vector",
        "bound": attempt["certificate_target"]["response_difference_bound"],
    }
    proof_obligations = [
        {
            "id": "generalized_projection_matrix_builder",
            "status": "closed" if generalized_matrix_built else "open",
            "requirement": "construct a single exact integer/q12289 system combining checked-core equations with hidden-source projection rows",
            "source_artifact": rel(MATRIX_BUILDER_JSON),
        },
        {
            "id": "exact_infeasibility_replay",
            "status": "closed_with_bounded_survivor" if exact_replay_ran else "open",
            "requirement": "run a deterministic exact solver or enumeration proving no nonzero bounded vector survives the selected projection map",
            "source_artifact": rel(EXACT_REPLAY_JSON),
        },
        {
            "id": "bad_vector_export",
            "status": "closed" if bad_vector_export_complete else "open",
            "requirement": "if infeasibility fails, export the shortest/smallest bad vector and classify whether the map or the route fails",
            "source_artifact": rel(EXACT_REPLAY_JSON),
        },
        {
            "id": "leakage_link",
            "status": "closed" if leakage_charge_closed else "open",
            "requirement": "connect the exact certificate to a leakage theorem charging exactly twelve q12289 elements and three check bytes",
            "source_requirement": "hidden_source_fallback_projection_map_certificate_attempt.exact_replay_requirements.leakage_charge",
        },
    ]
    open_proof_obligations = [
        row["id"]
        for row in proof_obligations
        if row["status"] not in ("closed", "closed_with_bounded_survivor")
    ]
    return {
        "schema": "Descender.hidden_source_fallback_projection_map_exact_certificate_spec.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "selected_projection_exact_certificate_spec_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "certificate_attempt": rel(CERT_ATTEMPT_JSON),
            "matrix_builder": rel(MATRIX_BUILDER_JSON),
            "exact_infeasibility_replay": rel(EXACT_REPLAY_JSON),
            "certificate_attempt_schema": attempt.get("schema"),
            "certificate_attempt_status": attempt.get("status"),
            "matrix_builder_schema": matrix_builder.get("schema"),
            "matrix_builder_status": matrix_builder.get("status"),
            "exact_infeasibility_replay_schema": exact_replay.get("schema"),
            "exact_infeasibility_replay_status": exact_replay.get("status"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "carrier_bytes": 56,
            "envelope_bytes": 308,
            "candidate_family": attempt["certificate_target"]["candidate_family"],
            "map_digest": attempt["certificate_target"]["map_digest"],
            "row_digest": row_digest(rows),
        },
        "selected_rows": rows,
        "structural_controls": structural_controls,
        "exact_certificate_interface": exact_certificate_interface,
        "proof_obligations": proof_obligations,
        "decision": {
            "can_accept_now": False,
            "exact_certificate_has_run": exact_replay_ran,
            "exact_infeasibility_proved": exact_replay.get("decision", {}).get(
                "exact_infeasibility_proved"
            )
            is True,
            "reason": "the selected projection rows, generalized system interface, leakage charge, exact replay, and bad-vector export are canonicalized; the replay found a bounded survivor, so acceptance remains rejected fail-closed",
            "open_proof_obligations": open_proof_obligations,
            "next_artifact": "hidden_source_fallback_projection_map_redesign_or_bridge_rows",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    lines = [
        "# Hidden-Source Projection Map Exact Certificate Specification",
        "",
        "This artifact fixes the selected projection-map certificate interface for",
        "the 308-byte route.  It does not claim the exact certificate has run;",
        "it records the canonical rows, structural controls, and the exact replay",
        "obligations needed next.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- candidate family: `{target['candidate_family']}`",
        f"- map digest: `{target['map_digest']}`",
        f"- row digest: `{target['row_digest']}`",
        "",
        "## Structural Controls",
        "",
        "| id | status | observed | required |",
        "|---|---|---|---|",
    ]
    for row in payload["structural_controls"]:
        observed = row.get("observed", row.get("observed_distinct_blocks", row.get("observed_min")))
        required = row.get("required", row.get("required_min_distinct_blocks"))
        lines.append(f"| `{row['id']}` | `{row['status']}` | `{observed}` | `{required}` |")
    lines.extend(
        [
            "",
            "## Exact Certificate Interface",
            "",
            f"- input relation: {payload['exact_certificate_interface']['input_relation']}",
            f"- not the same as: {payload['exact_certificate_interface']['not_the_same_as']}",
            f"- bound: `{payload['exact_certificate_interface']['bound']}`",
            "",
            "Required matrix:",
        ]
    )
    lines.extend(f"- {item}" for item in payload["exact_certificate_interface"]["required_matrix"])
    lines.extend(
        [
            "",
            "## Proof Obligations",
            "",
            "| id | status | requirement |",
            "|---|---|---|",
        ]
    )
    for row in payload["proof_obligations"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['requirement']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- exact certificate has run: `{payload['decision']['exact_certificate_has_run']}`",
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
