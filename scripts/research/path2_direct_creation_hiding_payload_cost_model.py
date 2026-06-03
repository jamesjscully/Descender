#!/usr/bin/env python3
"""Generate the Path 2 direct creation hiding-payload cost model."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_direct_creation_hiding_payload_cost_model.json"
OUT_MD = ROOT / "research/path2_direct_creation_hiding_payload_cost_model.md"

Q = 12289
GROUPS = 16
LOCAL_FREE = 2
PROJECTION_LANES = 6
PACKED_WITNESS_RESIDUES = 18
CHECKED_CORE_RESIDUES = 12
PAD_SEED_BYTES = 32
DIGEST_BYTES = 32

# Order-of-magnitude model for SHA-256 inside a general arithmetic proof.  This
# is intentionally a range because the exact number depends on the proof system
# and SHA gadget.  The ranking, not the precise count, is the useful output.
SHA256_COMPRESS_CONSTRAINTS_LOW = 20_000
SHA256_COMPRESS_CONSTRAINTS_HIGH = 30_000
Q_ARITH_CONSTRAINTS_PER_BASIC_OP = 1


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def sha_constraints(compressions: int) -> dict[str, int]:
    return {
        "compressions": compressions,
        "constraints_low": compressions * SHA256_COMPRESS_CONSTRAINTS_LOW,
        "constraints_high": compressions * SHA256_COMPRESS_CONSTRAINTS_HIGH,
    }


def build_payload() -> dict[str, Any]:
    components = [
        {
            "id": "leaf_evidence_certificate_binding",
            "proof_visibility": "public verifier precheck",
            "private_witness": False,
            "sha256_compressions_inside_hiding_proof": 0,
            "q_arithmetic_ops_inside_hiding_proof": 0,
            "notes": "already executable outside the hiding proof; do not duplicate unless the proof system must bind its own transcript",
        },
        {
            "id": "pad_seed_digest_opening",
            "proof_visibility": "private pad seed, public digest",
            "private_witness": True,
            "sha256_compressions_inside_hiding_proof": 1,
            "q_arithmetic_ops_inside_hiding_proof": 0,
            "notes": "SHA-256 preimage check for a 32-byte pad seed digest",
        },
        {
            "id": "state_assisted_opening_commitment",
            "proof_visibility": "private opening tuple, public opening commitment",
            "private_witness": True,
            "sha256_compressions_inside_hiding_proof": 4,
            "q_arithmetic_ops_inside_hiding_proof": 0,
            "notes": "opening tuple frame is digest-shaped; proving it directly is SHA-heavy",
        },
        {
            "id": "packed_witness_repacking",
            "proof_visibility": "private 18 q12289 residues, public 32-byte packed witness commitment",
            "private_witness": True,
            "sha256_compressions_inside_hiding_proof": 0,
            "q_arithmetic_ops_inside_hiding_proof": PACKED_WITNESS_RESIDUES,
            "notes": "mostly range/packing arithmetic if the 32-byte witness field is interpreted as canonical packed residues, not a hash",
        },
        {
            "id": "projection_mask_prf",
            "proof_visibility": "private pad seed and masks, public slot/profile/domain",
            "private_witness": True,
            "sha256_compressions_inside_hiding_proof": PROJECTION_LANES * 2,
            "q_arithmetic_ops_inside_hiding_proof": PROJECTION_LANES,
            "notes": "six SHA-derived mask lanes dominate the unmasking relation if proven inside the circuit",
        },
        {
            "id": "carrier_lane_checks",
            "proof_visibility": "private unmasked projection residues, public carrier/check digest or proof statement",
            "private_witness": True,
            "sha256_compressions_inside_hiding_proof": 0,
            "q_arithmetic_ops_inside_hiding_proof": 12,
            "notes": "cheap algebraically if carrier lanes are represented as field elements inside the proof",
        },
        {
            "id": "pad_source_projection_lanes",
            "proof_visibility": "private pad seed, public/hidden pad-source projection relation",
            "private_witness": True,
            "sha256_compressions_inside_hiding_proof": 6,
            "q_arithmetic_ops_inside_hiding_proof": 6,
            "notes": "lower-bound model for proving six pad-source lanes; full public pad statement costs more",
        },
        {
            "id": "full_public_pad_statement",
            "proof_visibility": "private pad seed, public full pad-opening statement",
            "private_witness": True,
            "sha256_compressions_inside_hiding_proof": GROUPS * 2,
            "q_arithmetic_ops_inside_hiding_proof": GROUPS * LOCAL_FREE,
            "notes": "worst direct proof if every block pad residue is derived inside the proof",
        },
    ]
    lower_bound_sha = sum(
        row["sha256_compressions_inside_hiding_proof"]
        for row in components
        if row["id"] != "full_public_pad_statement"
    )
    full_pad_sha = lower_bound_sha + next(
        row["sha256_compressions_inside_hiding_proof"]
        for row in components
        if row["id"] == "full_public_pad_statement"
    )
    lower_bound_q_ops = sum(
        row["q_arithmetic_ops_inside_hiding_proof"]
        for row in components
        if row["id"] != "full_public_pad_statement"
    )
    full_pad_q_ops = lower_bound_q_ops + GROUPS * LOCAL_FREE
    scenarios = [
        {
            "id": "existing_sha_commitments_hash_circuit_lower_bound",
            "description": "prove current digest commitments, projection masks, and six pad-source lanes inside one hiding proof",
            **sha_constraints(lower_bound_sha),
            "q_arithmetic_ops": lower_bound_q_ops,
            "classification": "sha_dominated",
        },
        {
            "id": "existing_sha_commitments_full_pad_statement",
            "description": "also prove the full 16-block public pad-opening statement from the hidden pad seed",
            **sha_constraints(full_pad_sha),
            "q_arithmetic_ops": full_pad_q_ops,
            "classification": "strongly_sha_dominated",
        },
        {
            "id": "proof_friendly_inner_commitments",
            "description": "keep SHA-256 as outer consensus binding but add proof-friendly inner commitments for opening tuple, pad source, and relation statement",
            **sha_constraints(0),
            "q_arithmetic_ops": lower_bound_q_ops + 64,
            "classification": "algebraic_dominated_after_redesign",
        },
    ]
    options = [
        {
            "id": "hash_heavy_circuit",
            "summary": "instantiate a SNARK/STARK-style proof over the current SHA-256 commitment design",
            "pros": [
                "minimal consensus schema redesign",
                "keeps existing leaf and digest fields",
                "clear soundness story if the hash circuit is implemented correctly",
            ],
            "cons": [
                "SHA-256 preimage checks dominate prover cost",
                "proof payload likely needs a real general-purpose proof system",
                "debugging relation failures is harder because every commitment is opaque",
            ],
            "estimated_bottleneck": "16 to 55 SHA-256 compressions inside the proof, depending on pad-statement scope",
            "recommendation": "use only as a baseline or if proof-system tooling makes SHA cheap enough",
        },
        {
            "id": "proof_friendly_inner_commitments",
            "summary": "add algebraic commitments/proof statements at account creation while retaining SHA-256 outer handles for consensus binding",
            "pros": [
                "moves the hiding proof into q12289-friendly arithmetic",
                "keeps the 252-byte spend path unchanged",
                "lets validators check a public proof without raw openings",
            ],
            "cons": [
                "requires account-creation/state schema expansion",
                "must prove inner commitments are bound to outer SHA leaf fields",
                "adds a new commitment assumption and proof API",
            ],
            "estimated_bottleneck": "commitment redesign and binding theorem, not arithmetic relation size",
            "recommendation": "most promising deployable path",
        },
        {
            "id": "opaque_ro_pok_handle",
            "summary": "treat a compact digest-sized proof handle as a named random-oracle proof-of-knowledge assumption",
            "pros": [
                "smallest byte footprint",
                "matches earlier 32-byte handle research track",
            ],
            "cons": [
                "nonstandard and likely too strong without external review",
                "does not give ordinary validators independently checkable proof semantics",
                "negative controls are oracle-model controls, not concrete verifier controls",
            ],
            "estimated_bottleneck": "assumption acceptability",
            "recommendation": "keep as fail-closed research control, not the first deployable target",
        },
    ]
    decision = {
        "identified_bottleneck": "SHA-256 preimage/PRF proving for pad seed, opening commitment, projection masks, and pad-source lanes dominates the hiding payload if current commitments are used directly.",
        "recommended_next_path": "proof_friendly_inner_commitments",
        "next_artifacts": [
            "Path2DirectCreationProofPayloadV1 relation statement",
            "proof-friendly inner commitment binding sketch",
            "byte/state delta estimate for account creation",
            "negative-control KAT plan for the new payload verifier",
        ],
        "do_not_do": [
            "do not accept public_replay_payload_digest as proof semantics",
            "do not instantiate a public opening payload unless deliberately switching to public_creation_opening_model",
            "do not add spend-signature bytes for this proof; keep it account-creation/admission scoped",
        ],
    }
    return {
        "schema": "Descender.path2_direct_creation_hiding_payload_cost_model.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "bottleneck_identified",
        "deployment_ready": False,
        "parameters": {
            "n": 32,
            "q": Q,
            "groups": GROUPS,
            "local_free": LOCAL_FREE,
            "projection_lanes": PROJECTION_LANES,
            "packed_witness_residues": PACKED_WITNESS_RESIDUES,
            "checked_core_residues": CHECKED_CORE_RESIDUES,
            "pad_seed_bytes": PAD_SEED_BYTES,
            "digest_bytes": DIGEST_BYTES,
            "sha256_compression_constraint_model": {
                "low": SHA256_COMPRESS_CONSTRAINTS_LOW,
                "high": SHA256_COMPRESS_CONSTRAINTS_HIGH,
                "note": "order-of-magnitude gadget cost, used only to rank bottlenecks",
            },
        },
        "relation_components": components,
        "scenarios": scenarios,
        "options": options,
        "decision": decision,
        "artifacts": {
            "json": rel(OUT_JSON),
            "markdown": rel(OUT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Direct Creation Hiding Payload Cost Model",
        "",
        "Status: bottleneck model for the remaining Path 2 proof-payload issue.",
        "",
        "This artifact separates the current relation into public wrapper checks,",
        "SHA-heavy hidden checks, and q12289 arithmetic checks. It is not a",
        "deployment proof; it identifies where a hiding payload will get expensive.",
        "",
        "## Decision",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- identified bottleneck: {payload['decision']['identified_bottleneck']}",
        f"- recommended next path: `{payload['decision']['recommended_next_path']}`",
        "",
        "## Scenario Estimates",
        "",
        "| scenario | SHA-256 compressions | constraint range | q ops | classification |",
        "|---|---:|---:|---:|---|",
    ]
    for row in payload["scenarios"]:
        lines.append(
            f"| `{row['id']}` | `{row['compressions']}` | "
            f"`{row['constraints_low']}..{row['constraints_high']}` | "
            f"`{row['q_arithmetic_ops']}` | `{row['classification']}` |"
        )
    lines.extend(
        [
            "",
            "## Relation Components",
            "",
            "| component | private witness | SHA compressions in proof | q ops | notes |",
            "|---|---|---:|---:|---|",
        ]
    )
    for row in payload["relation_components"]:
        lines.append(
            f"| `{row['id']}` | `{row['private_witness']}` | "
            f"`{row['sha256_compressions_inside_hiding_proof']}` | "
            f"`{row['q_arithmetic_ops_inside_hiding_proof']}` | {row['notes']} |"
        )
    lines.extend(["", "## Options", ""])
    for option in payload["options"]:
        lines.extend(
            [
                f"### {option['id']}",
                "",
                option["summary"],
                "",
                f"- estimated bottleneck: {option['estimated_bottleneck']}",
                f"- recommendation: {option['recommendation']}",
                "",
                "Pros:",
            ]
        )
        lines.extend(f"- {item}" for item in option["pros"])
        lines.append("")
        lines.append("Cons:")
        lines.extend(f"- {item}" for item in option["cons"])
        lines.append("")
    lines.extend(
        [
            "## Next Artifacts",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["decision"]["next_artifacts"])
    lines.extend(["", "## Do Not Do", ""])
    lines.extend(f"- {item}" for item in payload["decision"]["do_not_do"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(markdown(payload))
    print(json.dumps({"json": rel(OUT_JSON), "markdown": rel(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
