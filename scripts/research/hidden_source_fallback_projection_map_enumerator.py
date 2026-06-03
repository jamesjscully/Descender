#!/usr/bin/env python3
"""Generate deterministic candidate maps for the 56-byte fallback projection."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "hidden_source_fallback_projection_map_enumerator_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "hidden_source_fallback_projection_map_enumerator_n32_q12289.md"
SEARCH_JSON = READINESS_DIR / "hidden_source_fallback_projection_map_search_n32_q12289.json"

Q = 12289
N = 32
PROJECTION_ELEMENTS = 12
GROUPS = 16
LOCAL_FREE = 2
DOMAIN = b"Descender.hidden_source.fallback_projection_map_enumerator.v1"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def hash_u32(label: str, index: int, modulo: int) -> int:
    h = hashlib.sha256()
    h.update(DOMAIN)
    h.update(label.encode())
    h.update(index.to_bytes(2, "little"))
    return int.from_bytes(h.digest()[:4], "little") % modulo


def descriptor(kind: str, lane: int, block: int, coord: int, coefficient: int) -> dict[str, Any]:
    return {
        "kind": kind,
        "lane": lane,
        "block": block,
        "coord": coord,
        "coefficient_mod_q": coefficient % Q,
    }


def response_link_anchored_subview() -> list[dict[str, Any]]:
    rows = [
        descriptor("response_link_anchor", 0, 0, 0, 1),
        descriptor("response_link_anchor", 1, 1, 1, 1),
    ]
    for i in range(PROJECTION_ELEMENTS - len(rows)):
        rows.append(
            descriptor(
                "active_response_projection",
                i + 2,
                hash_u32("response_link.block", i, GROUPS),
                hash_u32("response_link.coord", i, LOCAL_FREE),
                1 + hash_u32("response_link.coeff", i, Q - 1),
            )
        )
    return rows


def balanced_witness_pad_subview() -> list[dict[str, Any]]:
    rows = []
    for i in range(6):
        rows.append(
            descriptor(
                "account_witness_projection",
                i,
                (3 * i + 1) % GROUPS,
                i % LOCAL_FREE,
                1 + hash_u32("balanced.witness.coeff", i, Q - 1),
            )
        )
    for i in range(6):
        rows.append(
            descriptor(
                "pad_seed_projection",
                i + 6,
                (5 * i + 2) % GROUPS,
                (i + 1) % LOCAL_FREE,
                1 + hash_u32("balanced.pad.coeff", i, Q - 1),
            )
        )
    return rows


def hash_seeded_random_projection() -> list[dict[str, Any]]:
    kinds = ["mixed_hidden_source_projection", "mixed_pad_projection", "mixed_witness_projection"]
    return [
        descriptor(
            kinds[hash_u32("hash.kind", i, len(kinds))],
            i,
            hash_u32("hash.block", i, GROUPS),
            hash_u32("hash.coord", i, LOCAL_FREE),
            1 + hash_u32("hash.coeff", i, Q - 1),
        )
        for i in range(PROJECTION_ELEMENTS)
    ]


def two_block_local_recovery_projection() -> list[dict[str, Any]]:
    blocks = [hash_u32("local.block.0", 0, GROUPS), hash_u32("local.block.1", 0, GROUPS)]
    rows = []
    for lane in range(PROJECTION_ELEMENTS):
        rows.append(
            descriptor(
                "local_recovery_projection" if lane < 8 else "cross_block_checksum",
                lane,
                blocks[lane % 2],
                lane % LOCAL_FREE,
                1 + hash_u32("local.coeff", lane, Q - 1),
            )
        )
    return rows


def map_digest(rows: list[dict[str, Any]]) -> str:
    h = hashlib.sha256()
    h.update(DOMAIN)
    for row in rows:
        h.update(str(row["kind"]).encode())
        h.update(int(row["lane"]).to_bytes(2, "little"))
        h.update(int(row["block"]).to_bytes(2, "little"))
        h.update(int(row["coord"]).to_bytes(2, "little"))
        h.update(int(row["coefficient_mod_q"]).to_bytes(2, "little"))
    return h.hexdigest()


def score_map(family: str, rows: list[dict[str, Any]]) -> dict[str, Any]:
    kinds = {row["kind"] for row in rows}
    blocks = {row["block"] for row in rows}
    witness_rows = sum(1 for row in rows if "witness" in row["kind"])
    pad_rows = sum(1 for row in rows if "pad" in row["kind"])
    response_rows = sum(1 for row in rows if "response" in row["kind"] or "link" in row["kind"])
    local_rows = sum(1 for row in rows if "local" in row["kind"] or "cross" in row["kind"])
    extraction_directness = min(5, 2 + min(witness_rows, 3) + min(pad_rows, 3))
    if family == "balanced_witness_pad_subview":
        extraction_directness = 5
    soundness_margin = min(5, 2 + len(blocks) // 3 + (1 if response_rows else 0))
    leakage_budget = 5 if family in {"balanced_witness_pad_subview", "hash_seeded_random_projection"} else 3
    certificate_replay_cost = 5 if response_rows else 3
    byte_stability = 5
    weighted_total = (
        soundness_margin * 5
        + extraction_directness * 5
        + leakage_budget * 4
        + certificate_replay_cost * 3
        + byte_stability * 2
    )
    return {
        "soundness_margin": soundness_margin,
        "extraction_directness": extraction_directness,
        "leakage_budget": leakage_budget,
        "certificate_replay_cost": certificate_replay_cost,
        "byte_stability": byte_stability,
        "weighted_total": weighted_total,
        "distinct_blocks": len(blocks),
        "distinct_kinds": sorted(kinds),
    }


def build_payload() -> dict[str, Any]:
    search = load_json(SEARCH_JSON)
    builders = {
        "response_link_anchored_subview": response_link_anchored_subview,
        "balanced_witness_pad_subview": balanced_witness_pad_subview,
        "hash_seeded_random_projection": hash_seeded_random_projection,
        "two_block_local_recovery_projection": two_block_local_recovery_projection,
    }
    candidates = []
    for family, builder in builders.items():
        rows = builder()
        scores = score_map(family, rows)
        candidates.append(
            {
                "family": family,
                "projection_elements": len(rows),
                "map_digest": map_digest(rows),
                "rows": rows,
                "scores": scores,
                "rejection_gate_status": "not_checked",
                "deployment_ready": False,
            }
        )
    candidates.sort(key=lambda row: row["scores"]["weighted_total"], reverse=True)
    return {
        "schema": "Descender.hidden_source_fallback_projection_map_enumerator.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "fallback_projection_maps_enumerated_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "projection_map_search": rel(SEARCH_JSON),
            "projection_map_search_schema": search.get("schema"),
        },
        "target": {
            "n": N,
            "q": Q,
            "fallback_carrier_bytes": 56,
            "fallback_envelope_bytes": 308,
            "projection_elements": PROJECTION_ELEMENTS,
            "selected_branch_remains": "fail-closed `284 B/Fq12` account-carried hidden-source envelope",
        },
        "candidate_maps": candidates,
        "best_candidate": {
            "family": candidates[0]["family"],
            "map_digest": candidates[0]["map_digest"],
            "weighted_total": candidates[0]["scores"]["weighted_total"],
        },
        "remaining_checks": [
            "run exact bounded-kernel replay for the selected candidate map",
            "test semantic negative controls against the fallback decoder",
            "prove leakage bound for the chosen twelve q12289 projection elements",
            "prove projection extraction or activate a larger fallback carrier",
        ],
        "decision": {
            "can_accept_now": False,
            "selected_for_certificate_attempt": candidates[0]["family"],
            "reason": "the enumerator ranks deterministic candidate maps, but none has exact bounded-kernel, semantic-control, or leakage certificates yet",
            "next_artifact": "fallback_projection_map_certificate_attempt",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Hidden-Source Fallback Projection Map Enumerator",
        "",
        "This artifact materializes deterministic candidate maps for the twelve",
        "q12289 projection elements in the 56-byte fallback carrier.  It remains",
        "fail-closed: the maps are candidates, not accepted proof semantics.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- fallback carrier bytes: `{payload['target']['fallback_carrier_bytes']}`",
        f"- fallback envelope bytes: `{payload['target']['fallback_envelope_bytes']}`",
        f"- projection elements: `{payload['target']['projection_elements']}`",
        f"- selected branch remains: {payload['target']['selected_branch_remains']}",
        f"- best candidate: `{payload['best_candidate']['family']}`",
        f"- best candidate digest: `{payload['best_candidate']['map_digest']}`",
        "",
        "## Candidate Ranking",
        "",
        "| rank | family | score | distinct blocks | digest |",
        "|---:|---|---:|---:|---|",
    ]
    for rank, row in enumerate(payload["candidate_maps"], 1):
        lines.append(
            f"| `{rank}` | `{row['family']}` | `{row['scores']['weighted_total']}` | "
            f"`{row['scores']['distinct_blocks']}` | `{row['map_digest']}` |"
        )
    lines.extend(["", "## Best Candidate Rows", ""])
    best = payload["candidate_maps"][0]
    lines.extend(
        [
            "| lane | kind | block | coord | coefficient mod q |",
            "|---:|---|---:|---:|---:|",
        ]
    )
    for row in best["rows"]:
        lines.append(
            f"| `{row['lane']}` | `{row['kind']}` | `{row['block']}` | "
            f"`{row['coord']}` | `{row['coefficient_mod_q']}` |"
        )
    lines.extend(["", "## Remaining Checks", ""])
    lines.extend(f"- {check}" for check in payload["remaining_checks"])
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- selected for certificate attempt: `{payload['decision']['selected_for_certificate_attempt']}`",
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
