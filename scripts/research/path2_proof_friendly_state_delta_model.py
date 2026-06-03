#!/usr/bin/env python3
"""Generate the Path 2 proof-friendly commitment state/byte delta model."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_proof_friendly_state_delta_model.json"
OUT_MD = ROOT / "research/path2_proof_friendly_state_delta_model.md"

DIGEST_BYTES = 32
CURRENT_LEAF_PREIMAGE_BYTES = 353
SPEND_SIGNATURE_BYTES = 252


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_payload() -> dict[str, Any]:
    scenarios = [
        {
            "id": "payload_only_binding_digest",
            "leaf_preimage_bytes": CURRENT_LEAF_PREIMAGE_BYTES,
            "leaf_delta_bytes": 0,
            "state_delta_bytes": 0,
            "creation_payload_public_digest_bytes": DIGEST_BYTES,
            "spend_signature_delta_bytes": 0,
            "binding_strength": "payload_bound",
            "tradeoff": "no state growth, but creation proof binding is not itself part of the leaf meaning unless certificate/payload availability is enforced",
        },
        {
            "id": "leaf_bound_creation_proof_binding_digest",
            "leaf_preimage_bytes": CURRENT_LEAF_PREIMAGE_BYTES + DIGEST_BYTES,
            "leaf_delta_bytes": DIGEST_BYTES,
            "state_delta_bytes": DIGEST_BYTES,
            "creation_payload_public_digest_bytes": DIGEST_BYTES,
            "spend_signature_delta_bytes": 0,
            "binding_strength": "leaf_bound",
            "tradeoff": "adds one digest to account creation/state but makes proof-friendly binding part of the authorization leaf consumed by spend",
        },
        {
            "id": "direct_state_stores_full_binding_object",
            "leaf_preimage_bytes": CURRENT_LEAF_PREIMAGE_BYTES + DIGEST_BYTES,
            "leaf_delta_bytes": DIGEST_BYTES,
            "state_delta_bytes": DIGEST_BYTES * 13 + 8 + 2,
            "creation_payload_public_digest_bytes": 0,
            "spend_signature_delta_bytes": 0,
            "binding_strength": "leaf_bound_and_directly_available",
            "tradeoff": "maximizes validator convenience but adds hundreds of state bytes; useful as a diagnostic upper bound, not the preferred blockchain path",
        },
    ]
    recommendation = {
        "selected_for_next_try": "leaf_bound_creation_proof_binding_digest",
        "reason": (
            "One extra 32-byte digest in account state is a modest cost and "
            "prevents the proof-friendly binding from living only in a payload "
            "side channel. Spend signatures remain unchanged."
        ),
        "required_code_change": "Path2PrevalidatedSlotLeafV2 or a compatible extension carrying creation_proof_binding_digest",
        "required_theorem_change": "slot_authorization_digest must bind creation_proof_binding_digest through the leaf preimage",
        "do_not_change": "252-byte checked-core spend signature",
    }
    executable_trial = {
        "status": "implemented_fail_closed_trial",
        "rust_items": [
            "Path2PrevalidatedSlotLeafV2",
            "path2_prevalidated_slot_leaf_v2_from_v1",
            "path2_prevalidated_slot_leaf_v2_digest",
            "path2_slot_authorization_v2_digest",
        ],
        "kat": "path2_prevalidated_slot_leaf_v2_binds_creation_proof_binding_digest",
        "claims": [
            "V2 leaf preimage is 385 bytes",
            "V2 leaf is exactly 32 bytes larger than V1",
            "V2 uses distinct leaf and authorization domains",
            "creation_proof_binding_digest changes the V2 leaf digest and authorization digest",
            "checked-core spend signature remains 252 bytes",
        ],
    }
    return {
        "schema": "Descender.path2_proof_friendly_state_delta_model.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "state_delta_modeled",
        "deployment_ready": False,
        "baseline": {
            "current_leaf_preimage_bytes": CURRENT_LEAF_PREIMAGE_BYTES,
            "spend_signature_bytes": SPEND_SIGNATURE_BYTES,
        },
        "scenarios": scenarios,
        "recommendation": recommendation,
        "executable_trial": executable_trial,
        "artifacts": {
            "json": rel(OUT_JSON),
            "markdown": rel(OUT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    rec = payload["recommendation"]
    trial = payload["executable_trial"]
    lines = [
        "# Path 2 Proof-Friendly State Delta Model",
        "",
        "Status: byte/state model for the proof-friendly inner commitment path.",
        "",
        "## Baseline",
        "",
        f"- current leaf preimage bytes: `{payload['baseline']['current_leaf_preimage_bytes']}`",
        f"- spend signature bytes: `{payload['baseline']['spend_signature_bytes']}`",
        "",
        "## Scenarios",
        "",
        "| scenario | leaf bytes | leaf delta | state delta | spend delta | binding strength | tradeoff |",
        "|---|---:|---:|---:|---:|---|---|",
    ]
    for row in payload["scenarios"]:
        lines.append(
            f"| `{row['id']}` | `{row['leaf_preimage_bytes']}` | "
            f"`{row['leaf_delta_bytes']}` | `{row['state_delta_bytes']}` | "
            f"`{row['spend_signature_delta_bytes']}` | `{row['binding_strength']}` | "
            f"{row['tradeoff']} |"
        )
    lines.extend(
        [
            "",
            "## Recommendation",
            "",
            f"- selected for next try: `{rec['selected_for_next_try']}`",
            f"- reason: {rec['reason']}",
            f"- required code change: {rec['required_code_change']}",
            f"- required theorem change: {rec['required_theorem_change']}",
            f"- do not change: {rec['do_not_change']}",
            "",
            "## Executable Trial",
            "",
            f"- status: `{trial['status']}`",
            f"- KAT: `{trial['kat']}`",
            "",
            "Rust items:",
        ]
    )
    lines.extend(f"- `{item}`" for item in trial["rust_items"])
    lines.extend(["", "Claims:"])
    lines.extend(f"- {item}" for item in trial["claims"])
    lines.extend(
        [
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(markdown(payload))
    print(json.dumps({"json": rel(OUT_JSON), "markdown": rel(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
