#!/usr/bin/env python3
"""Generate the fallback carrier projection-map search plan."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "hidden_source_fallback_projection_map_search_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "hidden_source_fallback_projection_map_search_n32_q12289.md"
PROJECTION_RELATION_JSON = (
    READINESS_DIR / "hidden_source_fallback_carrier_projection_relation_n32_q12289.json"
)
RESPONSE_LINK_CERT_JSON = (
    READINESS_DIR
    / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    projection_relation = load_json(PROJECTION_RELATION_JSON)
    response_link_certificate = load_json(RESPONSE_LINK_CERT_JSON)
    candidate_families = [
        {
            "id": "response_link_anchored_subview",
            "projection_elements": 12,
            "seed_coordinates": [0, 3],
            "description": "start from the reviewed response-link coordinates and add ten transcript-derived active-response projections",
            "expected_strength": "best continuity with the existing bounded-kernel certificate",
            "risk": "may overfit the response-link proof and fail to bind pad/witness openings",
        },
        {
            "id": "balanced_witness_pad_subview",
            "projection_elements": 12,
            "seed_coordinates": [],
            "description": "allocate six projected elements to account-witness opening checks and six to pad-seed/opening checks",
            "expected_strength": "best semantic match to hidden-source extraction",
            "risk": "requires a new projection-soundness argument rather than direct reuse of response-link coordinates",
        },
        {
            "id": "hash_seeded_random_projection",
            "projection_elements": 12,
            "seed_coordinates": [],
            "description": "derive a public 12-row projection from the fallback frame domain, profile, statement digest, and opening commitment",
            "expected_strength": "reduces manual coordinate-selection bias and gives a clean Fiat-Shamir-style selection rule",
            "risk": "harder to precompute exact bounded-kernel certificates across all statements",
        },
        {
            "id": "two_block_local_recovery_projection",
            "projection_elements": 12,
            "seed_coordinates": [],
            "description": "choose projections that expose two local block consistency checks plus cross-block checksum rows",
            "expected_strength": "may give the most direct relation to the existing local recovery structure",
            "risk": "could leak more structured information about witness/pad state than a balanced random subview",
        },
    ]
    scoring = [
        {
            "criterion": "soundness_margin",
            "weight": 5,
            "measurement": "no low-norm bad vector or false hidden-source witness survives the twelve projections within the accepted response-difference box",
        },
        {
            "criterion": "extraction_directness",
            "weight": 5,
            "measurement": "acceptance yields named witness/pad openings or a named binding/hash/checked-core break with minimal auxiliary assumptions",
        },
        {
            "criterion": "leakage_budget",
            "weight": 4,
            "measurement": "the twelve q12289 elements have an explicit leakage charge and do not expose account witness or pad seed directly",
        },
        {
            "criterion": "certificate_replay_cost",
            "weight": 3,
            "measurement": "exact replay can be completed and audited with the existing certificate infrastructure or a small extension",
        },
        {
            "criterion": "byte_stability",
            "weight": 2,
            "measurement": "projection map uses the fixed 56-byte layout without adding escape hatches or variable-length payloads",
        },
    ]
    rejection_gates = [
        "projection accepts digest-only carrier without relation equations",
        "projection can be replayed under a different statement digest or opening commitment",
        "projection has an uncharged direct leak of account witness syndrome, pad seed, or carrier secret",
        "projection cannot be checked without secret verifier inputs",
        "projection requires more than twelve q12289 elements or more than three check/flag bytes",
        "projection loses the fixed public rejection surface",
    ]
    return {
        "schema": "Descender.hidden_source_fallback_projection_map_search.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "fallback_projection_map_search_plan_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "fallback_projection_relation": rel(PROJECTION_RELATION_JSON),
            "fallback_projection_relation_schema": projection_relation.get("schema"),
            "response_link_certificate": rel(RESPONSE_LINK_CERT_JSON),
            "response_link_certificate_schema": response_link_certificate.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "fallback_carrier_bytes": 56,
            "fallback_envelope_bytes": 308,
            "projection_elements": 12,
            "projection_payload_bytes": 21,
            "check_flag_bytes": 3,
            "selected_branch_remains": "fail-closed `284 B/Fq12` account-carried hidden-source envelope",
        },
        "candidate_families": candidate_families,
        "scoring": scoring,
        "rejection_gates": rejection_gates,
        "required_experiments": [
            {
                "id": "enumerate_candidate_maps",
                "output": "ranked projection maps with deterministic domain-separated derivation records",
            },
            {
                "id": "bounded_kernel_replay",
                "output": "exact no-bad-vector certificate for the selected projection map and response-difference bounds",
            },
            {
                "id": "semantic_negative_controls",
                "output": "byte-level false-statement, pad-substitution, replay, and malformed-packing rejections",
            },
            {
                "id": "leakage_accounting",
                "output": "explicit leakage theorem charging all twelve q12289 elements and three check/flag bytes",
            },
        ],
        "decision": {
            "can_accept_now": False,
            "recommended_first_family": "balanced_witness_pad_subview",
            "reason": "the fallback exists to avoid relying on opaque proof-handle extraction, so the first projection search should optimize semantic witness/pad extraction rather than only reuse response-link coordinates",
            "next_artifact": "fallback_projection_map_enumerator",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    lines = [
        "# Hidden-Source Fallback Projection Map Search",
        "",
        "This artifact defines how to choose the twelve q12289 projection elements",
        "for the 56-byte fallback carrier if the 32-byte RO-PoK route is rejected.",
        "It remains fail-closed and does not select the fallback.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- fallback carrier bytes: `{target['fallback_carrier_bytes']}`",
        f"- fallback envelope bytes: `{target['fallback_envelope_bytes']}`",
        f"- projection elements: `{target['projection_elements']}`",
        f"- selected branch remains: {target['selected_branch_remains']}",
        f"- source relation: `{payload['source_artifacts']['fallback_projection_relation']}`",
        "",
        "## Candidate Families",
        "",
        "| id | elements | seed coordinates | expected strength | risk |",
        "|---|---:|---|---|---|",
    ]
    for row in payload["candidate_families"]:
        seed = ",".join(str(coord) for coord in row["seed_coordinates"]) or "-"
        lines.append(
            f"| `{row['id']}` | `{row['projection_elements']}` | `{seed}` | "
            f"{row['expected_strength']} | {row['risk']} |"
        )
    lines.extend(
        [
            "",
            "## Scoring",
            "",
            "| criterion | weight | measurement |",
            "|---|---:|---|",
        ]
    )
    for row in payload["scoring"]:
        lines.append(f"| `{row['criterion']}` | `{row['weight']}` | {row['measurement']} |")
    lines.extend(["", "## Rejection Gates", ""])
    lines.extend(f"- {gate}" for gate in payload["rejection_gates"])
    lines.extend(
        [
            "",
            "## Required Experiments",
            "",
            "| id | output |",
            "|---|---|",
        ]
    )
    for row in payload["required_experiments"]:
        lines.append(f"| `{row['id']}` | {row['output']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- recommended first family: `{payload['decision']['recommended_first_family']}`",
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
