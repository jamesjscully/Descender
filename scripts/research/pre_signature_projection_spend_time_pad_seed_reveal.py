#!/usr/bin/env python3
"""Assess spend-time raw pad-seed reveal as a hidden-source proof alternative."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"
LEAN_REFERENCE = ROOT / "Descender/LocalDeltaByteCommitment.lean"

DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_spend_time_pad_seed_reveal_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_spend_time_pad_seed_reveal_n32_q12289.md"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def source_text(path: Path) -> str:
    return path.read_text()


def present(path: Path, needle: str) -> bool:
    return needle in source_text(path)


def evidence(path: Path, needle: str, role: str) -> dict[str, Any]:
    return {
        "path": rel(path),
        "needle": needle,
        "present": present(path, needle),
        "role": role,
    }


def build_payload() -> dict[str, Any]:
    base_fq12_signature_bytes = 252
    fallback_carrier_bytes = 56
    selected_projection_signature_bytes = base_fq12_signature_bytes + fallback_carrier_bytes
    raw_pad_seed_reveal_bytes = 32
    selected_with_raw_seed_bytes = selected_projection_signature_bytes + raw_pad_seed_reveal_bytes
    hidden_source_proof_reference_bytes = 32
    hidden_source_proof_reference_total = base_fq12_signature_bytes + hidden_source_proof_reference_bytes

    byte_placement = {
        "signature": [
            {
                "field": "reviewed_fq12_account_carried_signature",
                "bytes": base_fq12_signature_bytes,
                "status": "existing selected checked-core envelope",
            },
            {
                "field": "projection_carrier",
                "bytes": fallback_carrier_bytes,
                "status": "existing selected hidden-source fallback carrier",
            },
            {
                "field": "raw_pad_seed_reveal_candidate",
                "bytes": raw_pad_seed_reveal_bytes,
                "status": "new spend-time public reveal field under this option",
            },
        ],
        "state": [
            {
                "field": "pad_seed_digest",
                "bytes": 32,
                "status": "already stored in live selected-projection slot records and spent records",
            },
            {
                "field": "raw_pad_seed",
                "bytes": 0,
                "status": "must remain absent from account state, snapshots, spent ledgers, and key material before spend",
            },
        ],
        "key": [
            {
                "field": "raw_pad_seed",
                "bytes": 0,
                "status": "no key-format placement found or recommended for this option",
            }
        ],
        "totals": {
            "selected_projection_signature_bytes": selected_projection_signature_bytes,
            "selected_plus_raw_pad_seed_signature_bytes": selected_with_raw_seed_bytes,
            "delta_vs_selected_projection_signature_bytes": raw_pad_seed_reveal_bytes,
            "reference_hidden_source_proof_bytes": hidden_source_proof_reference_bytes,
            "reference_hidden_source_proof_total_bytes": hidden_source_proof_reference_total,
            "note": (
                "This branch evaluates the user's +32-byte reveal option. It does not "
                "change the existing 308-byte selected projection carrier accounting."
            ),
        },
    }

    reconstruction = {
        "verifier_can_reconstruct": [
            "pad_seed_digest and public pad-opening statement from the revealed pad_seed",
            "six projection mask residues from pad_seed and slot_id",
            "six account_projection residues from public masked_projection minus projection masks mod q",
            "account_witness_projection_opening_frame and its digest for the selected six residues",
            "six pad-source projection residues consumed by the carrier relation",
            "the 12-lane selected projection carrier relation checked by the current tuple verifier",
        ],
        "verifier_still_does_not_get": [
            "a witness-hiding proof for the hidden source",
            "full hidden account witness outside the selected six projection residues",
            "a proof that raw pad_seed was single-use unless lifecycle state enforces digest uniqueness",
            "pre-spend secrecy for any other live slot that accidentally reused the same pad_seed",
        ],
        "current_code_path_match": (
            "The selected tuple verifier already accepts pad_seed as verifier witness input "
            "and derives projection masks/account_projection before checking carrier binding. "
            "Making pad_seed a signature field makes this reconstruction public after spend, "
            "rather than cryptographically hidden."
        ),
    }

    leakage_mode = {
        "mode": "post_spend_one_time_secret_reveal",
        "leaks_immediately_after_acceptance": [
            "raw 32-byte pad_seed",
            "all pad PRF outputs for every block under that seed, not only observed blocks",
            "all projection masks keyed by that pad_seed for any slot id the mask PRF accepts",
            "selected six account_projection residues for the spent slot",
        ],
        "does_not_expand_pre_spend_view_if": [
            "raw pad_seed is only present in the spend signature and never serialized in live account state or key material",
            "verification rejects unless revealed pad_seed hashes to the stored pad_seed_digest",
            "account-state construction/restore rejects duplicate live pad_seed_digest values",
            "spent-ledger/target-chain commit rejects duplicate historical pad_seed_digest values atomically with slot consumption",
        ],
        "fatal_if": [
            "two live slots share the same pad_seed_digest before either is spent",
            "a spent/burn ledger can roll back or omit the first reveal",
            "mempool or reorg policy lets a revealed seed become eligible again",
            "reject paths return derived masks, account_projection, opening frames, or raw pad_seed diagnostics",
        ],
    }

    replay_surface = {
        "existing_support": [
            "state-assisted account state rejects zero and duplicate pad_seed_digest values per account",
            "slot opening binds opening.pad_seed_digest to the slot record",
            "spent ledger rejects duplicate slot id and duplicate pad_seed_digest",
            "selected projection consensus spend record stores pad_seed_digest and hidden_source_opening_commitment",
            "no-export tests cover digest-only account records, snapshots, reject paths, and spent replay surfaces",
        ],
        "remaining_gap": (
            "The evidence supports one-time safety for reference/profiled selected-projection "
            "lifecycle paths, but deployment safety still depends on the production DB/profile "
            "binding, atomic commit, external storage review, and reorg/finality policy."
        ),
    }

    theorem_changes = [
        {
            "id": "replace_witness_hiding_with_spend_reveal",
            "change": (
                "The public-opening theorem can no longer require witness hiding for pad_seed. "
                "It must be restated as pre-spend hiding plus post-spend public reconstruction."
            ),
        },
        {
            "id": "soundness_from_reconstruction_not_pok",
            "change": (
                "Soundness becomes deterministic reconstruction of the selected relation from "
                "signature pad_seed, state pad_seed_digest, slot_id, packed witness, and carrier "
                "binding. It is not a general proof of knowledge of a hidden preimage."
            ),
        },
        {
            "id": "single_use_lifecycle_assumption",
            "change": (
                "Add a theorem hypothesis that pad_seed_digest is unique across live state and "
                "terminal spent/burn history for the consensus account identity."
            ),
        },
        {
            "id": "reorg_mempool_reveal_terminality",
            "change": (
                "Any observed raw pad_seed reveal must make the slot terminal, even if the "
                "transaction is reorged, evicted, or not finalized."
            ),
        },
        {
            "id": "no_export_reject_boundary",
            "change": (
                "Reject-before-acceptance paths must remain public-error-only and must not "
                "materialize derived masks/projection openings outside the accepted-spend path."
            ),
        },
    ]

    feasibility = {
        "preliminary_conclusion": "feasible_only_as_a_different_security_model",
        "can_avoid_full_hidden_source_pok": True,
        "why": (
            "Revealing pad_seed gives the verifier enough to recompute the selected "
            "projection-mask relation already used by the selected verifier path, so a "
            "separate hidden-source PoK for those selected lanes is unnecessary under a "
            "post-spend reveal model."
        ),
        "why_not_drop_all_proofs": (
            "The checked-core signature relation, carrier binding, digest binding, account-state "
            "opening, and lifecycle/spent-ledger invariants remain mandatory. The result is not "
            "witness-hiding and does not prove anything about unselected hidden coordinates."
        ),
        "recommendation": (
            "Do not promote yet. Prototype only behind a research flag after adding a raw "
            "pad_seed signature field, fail-closed no-export tests for reject paths, and a "
            "formal theorem split between pre-spend hiding and post-spend one-time reveal."
        ),
    }

    return {
        "schema": "Descender.pre_signature_projection_spend_time_pad_seed_reveal.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "target": {
            "n": 32,
            "q": 12289,
            "path": "selected_308_byte_fq12_state_assisted_masked_projection_carrier",
        },
        "deployment_ready": False,
        "byte_placement": byte_placement,
        "reconstruction": reconstruction,
        "leakage_mode": leakage_mode,
        "replay_surface": replay_surface,
        "required_theorem_changes": theorem_changes,
        "feasibility": feasibility,
        "evidence": [
            evidence(
                PRODUCTION_REFERENCE,
                "pub const REVIEWED_FQ12_PAD_SEED_BYTES: usize = 32;",
                "raw reveal candidate size",
            ),
            evidence(
                PRODUCTION_REFERENCE,
                "REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES == 308",
                "selected projection carrier envelope size",
            ),
            evidence(
                PRODUCTION_REFERENCE,
                "reviewed_fq12_packed_witness_state18_projection_masks(pad_seed, slot_id)",
                "verifier-side mask reconstruction from pad_seed and slot_id",
            ),
            evidence(
                PRODUCTION_REFERENCE,
                "reviewed_fq12_pad_seed_opening_digest(pad_seed)",
                "state digest binding for revealed pad_seed",
            ),
            evidence(
                PRODUCTION_REFERENCE,
                "state-assisted pad seed digest must be unique per account",
                "live account-state duplicate digest rejection",
            ),
            evidence(
                PRODUCTION_REFERENCE,
                "pad seed digest is already in spent ledger",
                "spent-ledger duplicate digest rejection",
            ),
            evidence(
                PRODUCTION_REFERENCE,
                "assert_no_selected_projection_material",
                "no-export regression coverage for selected projection material",
            ),
            evidence(
                LEAN_REFERENCE,
                "WitnessHidingPublicOpeningProofSystem",
                "current proof boundary still models witness hiding",
            ),
            evidence(
                LEAN_REFERENCE,
                "revealedPreimageProofSystem",
                "Lean already has a non-hiding reveal-style proof-system shape",
            ),
        ],
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    totals = payload["byte_placement"]["totals"]
    feasibility = payload["feasibility"]
    lines = [
        "# Spend-Time Pad-Seed Reveal Assessment",
        "",
        "Preliminary assessment of replacing a hidden-source proof with a one-time",
        "raw `pad_seed` reveal at selected-projection spend verification.",
        "",
        "## Byte Placement",
        "",
        f"- selected projection signature bytes: `{totals['selected_projection_signature_bytes']}`",
        f"- raw pad_seed reveal bytes: `{totals['delta_vs_selected_projection_signature_bytes']}`",
        f"- selected plus raw pad_seed bytes: `{totals['selected_plus_raw_pad_seed_signature_bytes']}`",
        f"- live state/key raw pad_seed bytes: `0`",
        f"- live/spent state keeps public `pad_seed_digest`: `32` bytes",
        "",
        "## Reconstruction",
        "",
        payload["reconstruction"]["current_code_path_match"],
        "",
        "Verifier can reconstruct after reveal:",
        "",
    ]
    lines.extend(f"- {item}" for item in payload["reconstruction"]["verifier_can_reconstruct"])
    lines.extend(["", "Verifier still does not get:", ""])
    lines.extend(f"- {item}" for item in payload["reconstruction"]["verifier_still_does_not_get"])

    leak = payload["leakage_mode"]
    lines.extend(["", "## Leakage Mode", "", f"- mode: `{leak['mode']}`", "", "Leaks after acceptance:"])
    lines.extend(f"- {item}" for item in leak["leaks_immediately_after_acceptance"])
    lines.extend(["", "Safe only if:"])
    lines.extend(f"- {item}" for item in leak["does_not_expand_pre_spend_view_if"])
    lines.extend(["", "Fatal if:"])
    lines.extend(f"- {item}" for item in leak["fatal_if"])

    replay = payload["replay_surface"]
    lines.extend(["", "## Replay Surface", "", "Existing support:"])
    lines.extend(f"- {item}" for item in replay["existing_support"])
    lines.extend(["", f"Remaining gap: {replay['remaining_gap']}"])

    lines.extend(["", "## Required Theorem Changes", ""])
    for item in payload["required_theorem_changes"]:
        lines.append(f"- `{item['id']}`: {item['change']}")

    lines.extend(
        [
            "",
            "## Conclusion",
            "",
            f"- preliminary conclusion: `{feasibility['preliminary_conclusion']}`",
            f"- can avoid full hidden-source PoK for selected lanes: `{feasibility['can_avoid_full_hidden_source_pok']}`",
            f"- why: {feasibility['why']}",
            f"- limit: {feasibility['why_not_drop_all_proofs']}",
            f"- recommendation: {feasibility['recommendation']}",
            "",
            "## Evidence",
            "",
            "| present | path | role | needle |",
            "|---|---|---|---|",
        ]
    )
    for item in payload["evidence"]:
        lines.append(
            f"| `{item['present']}` | `{item['path']}` | {item['role']} | `{item['needle']}` |"
        )
    lines.append("")
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
                "status": payload["feasibility"]["preliminary_conclusion"],
                "selected_plus_raw_pad_seed_signature_bytes": payload["byte_placement"][
                    "totals"
                ]["selected_plus_raw_pad_seed_signature_bytes"],
                "can_avoid_full_hidden_source_pok": payload["feasibility"][
                    "can_avoid_full_hidden_source_pok"
                ],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
