#!/usr/bin/env python3
"""Generate the projection-mask PRF/domain-separation assumption API."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_signature_projection_mask_prf_assumption_api_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_signature_projection_mask_prf_assumption_api_n32_q12289.md"
LEAKAGE_JSON = READINESS_DIR / "pre_signature_projection_leakage_probe_n32_q12289.json"
STATISTICAL_JSON = READINESS_DIR / "pre_signature_projection_statistical_probe_n32_q12289.json"
INDEPENDENCE_JSON = READINESS_DIR / "pre_signature_projection_mask_independence_probe_n32_q12289.json"
FRAME_JSON = READINESS_DIR / "pre_signature_projection_mask_frame_disjointness_n32_q12289.json"
REUSE_JSON = READINESS_DIR / "pre_signature_projection_pad_seed_reuse_surface_n32_q12289.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    leakage = load_json(LEAKAGE_JSON)
    statistical = load_json(STATISTICAL_JSON)
    independence = load_json(INDEPENDENCE_JSON)
    frame = load_json(FRAME_JSON)
    reuse = load_json(REUSE_JSON)
    game = {
        "id": "reviewed_fq12_packed_witness_state18_projection_mask_prf",
        "status": "explicit_required_assumption",
        "model": "domain-separated SHA-256 PRF/random-oracle stream",
        "statement": (
            "For any lifecycle-valid, prechallenge-fixed pad seed and slot id, "
            "the six projection masks derived under the projection-mask domain "
            "are computationally indistinguishable from independent uniform "
            "q12289 residues to an adversary that sees the public account state, "
            "pad-opening statement, selected projection carrier, transcript "
            "bindings, and all pad PRF outputs revealed by the verifier."
        ),
        "break_event": (
            "An adversary distinguishes masked projection residues from uniform, "
            "correlates them with existing pad PRF projection lanes, predicts a "
            "mask lane, or causes postchallenge pad-seed/slot selection while "
            "all byte-level lifecycle checks accept."
        ),
    }
    adversary_view = [
        "public key and account-state slot records",
        "32-byte packed witness commitment containing 12 checked-core residues plus six masked projection residues",
        "state-assisted hidden-source opening commitment and restore anchor digest",
        "public pad-opening statement and verifier-revealed local pad residues",
        "selected 56-byte projection carrier and decoded carrier lanes",
        "message, signature bytes, challenge bytes, transcript digests, and rejection class",
    ]
    forbidden_view = [
        "raw selected account projection residues before masking",
        "pad seed before the slot opening is validly revealed",
        "secret key or account witness preimage outside the committed packed state",
        "postchallenge ability to choose or rewrite pad_seed, slot_id, or packed witness bytes",
    ]
    implemented_evidence = [
        {
            "artifact": rel(LEAKAGE_JSON),
            "claim": "18-row packed state adds six public rows per live slot and no small row dependency was found in the bounded linear probe",
            "deployment_credit": "bounded leakage model evidence only",
        },
        {
            "artifact": rel(STATISTICAL_JSON),
            "claim": "raw small centered projection residues are distinguishable, while pad-seed-masked residues match the uniform marginal probe",
            "deployment_credit": "bounded marginal statistical evidence only",
        },
        {
            "artifact": rel(INDEPENDENCE_JSON),
            "claim": (
                "50,000-sample probe found max centered Pearson "
                f"{independence['summary']['max_abs_pearson_centered']:.6f}, mask-pad equality "
                f"{independence['summary']['total_mask_pad_equal_fraction']:.6f}, and adjacent slot mask equality "
                f"{independence['summary']['same_seed_adjacent_slot_mask_equal_fraction']:.6f}"
            ),
            "deployment_credit": "bounded pairwise stream-correlation evidence only",
        },
        {
            "artifact": rel(FRAME_JSON),
            "claim": (
                f"pad PRF input length {frame['pad_prf_input_len']} and projection-mask PRF input length "
                f"{frame['mask_prf_input_len']} are byte-disjoint; bounded byte-equal collisions "
                f"{len(frame['bounded_byte_equal_collisions'])}"
            ),
            "deployment_credit": "executable byte-frame disjointness evidence only",
        },
        {
            "artifact": rel(REUSE_JSON),
            "claim": (
                "same pad seed across slots gives different projection masks but repeats the pad-seed digest "
                f"({reuse['pad_seed_digest_repeats']}) and pad-opening statement "
                f"({reuse['pad_opening_statement_repeats']}) for the same observed block set"
            ),
            "deployment_credit": "lifecycle hazard evidence; no positive theorem credit",
        },
    ]
    theorem_obligations = [
        {
            "obligation": "prf_indistinguishability",
            "status": "open",
            "required_to_close": "state a SHA-256 PRF/RO game with query budget and reduction loss for the projection-mask domain",
        },
        {
            "obligation": "domain_separation",
            "status": "partially_supported_by_frame_certificate",
            "required_to_close": "prove distinct byte inputs imply independent RO queries or account for hash/PRF multi-domain security",
        },
        {
            "obligation": "lifecycle_prechallenge_fixation",
            "status": "partially_supported_by_slot_bound_regression",
            "required_to_close": "formalize that pad_seed, slot_id, and packed witness bytes are committed before challenge and cannot be rewritten after message selection",
        },
        {
            "obligation": "composition_with_pad_opening_leakage",
            "status": "open",
            "required_to_close": "show revealed pad-opening rows and carrier lanes do not condition the projection-mask stream beyond the modeled adversary view",
        },
        {
            "obligation": "single_use_pad_seed_policy",
            "status": "open",
            "required_to_close": "prove or enforce that pad seeds are unique to the state-assisted slot lifecycle, including crash/reorg recovery paths",
        },
    ]
    return {
        "schema": "Descender.pre_signature_projection_mask_prf_assumption_api.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "mask_prf_assumption_api_fail_closed",
        "deployment_ready": False,
        "target": {
            "n": 32,
            "q": 12289,
            "packed_witness_rows": 18,
            "checked_core_rows": 12,
            "masked_projection_rows": 6,
            "selected_signature_bytes": 308,
        },
        "source_artifacts": {
            "leakage_probe": rel(LEAKAGE_JSON),
            "statistical_probe": rel(STATISTICAL_JSON),
            "mask_independence_probe": rel(INDEPENDENCE_JSON),
            "mask_frame_disjointness": rel(FRAME_JSON),
            "pad_seed_reuse_surface": rel(REUSE_JSON),
            "leakage_probe_status": leakage.get("status"),
            "statistical_probe_status": statistical.get("status"),
            "pad_seed_reuse_surface_status": reuse.get("status"),
        },
        "game": game,
        "adversary_view": adversary_view,
        "forbidden_view": forbidden_view,
        "implemented_evidence": implemented_evidence,
        "theorem_obligations": theorem_obligations,
        "decision": {
            "can_promote_to_lean_or_account_state_validation": False,
            "reason": "implementation evidence supports Option A, but PRF/domain-separation and lifecycle-composition theorem obligations remain open",
            "next_artifact": "formal mask PRF/domain-separation theorem or explicit SHA-256 PRF assumption consumed by the deployable security theorem",
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Mask PRF Assumption API",
        "",
        "This artifact pins the exact assumption boundary for Option A. It is not",
        "a deployment claim; it identifies what remains to prove after the byte",
        "frame, statistical, and lifecycle regressions.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected signature bytes: `{payload['target']['selected_signature_bytes']}`",
        f"- packed witness rows: `{payload['target']['packed_witness_rows']}`",
        f"- masked projection rows: `{payload['target']['masked_projection_rows']}`",
        "",
        "## Game",
        "",
        f"- id: `{payload['game']['id']}`",
        f"- status: `{payload['game']['status']}`",
        f"- model: `{payload['game']['model']}`",
        "",
        payload["game"]["statement"],
        "",
        f"Break event: {payload['game']['break_event']}",
        "",
        "## Adversary View",
        "",
    ]
    lines.extend(f"- {item}" for item in payload["adversary_view"])
    lines.extend(["", "## Forbidden View", ""])
    lines.extend(f"- {item}" for item in payload["forbidden_view"])
    lines.extend(
        [
            "",
            "## Implemented Evidence",
            "",
            "| artifact | claim | deployment credit |",
            "|---|---|---|",
        ]
    )
    for row in payload["implemented_evidence"]:
        lines.append(f"| `{row['artifact']}` | {row['claim']} | {row['deployment_credit']} |")
    lines.extend(
        [
            "",
            "## Theorem Obligations",
            "",
            "| obligation | status | required to close |",
            "|---|---|---|",
        ]
    )
    for row in payload["theorem_obligations"]:
        lines.append(f"| `{row['obligation']}` | {row['status']} | {row['required_to_close']} |")
    decision = payload["decision"]
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can promote to Lean/account-state validation: `{decision['can_promote_to_lean_or_account_state_validation']}`",
            f"- reason: {decision['reason']}",
            f"- next artifact: {decision['next_artifact']}",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "status": payload["status"],
                "deployment_ready": payload["deployment_ready"],
                "open_obligations": sum(
                    1 for row in payload["theorem_obligations"] if row["status"] == "open"
                ),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
