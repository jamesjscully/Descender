#!/usr/bin/env python3
"""Estimate account-state byte costs for state-assisted extractability routes."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_extractability_state_costs_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "hidden_source_state_assisted_extractability_state_costs_n32_q12289.md"
)

DIGEST_BYTES = 32
SLOT_ID_BYTES = 4
BOOL_BYTES = 1
ACCOUNT_LINK_BYTES = 68
CHECKED_CORE_SIGNATURE_BYTES = 252
FALLBACK_PROJECTION_CARRIER_BYTES = 56
FALLBACK_PROJECTION_LANE_BYTES = 24
FALLBACK_PROJECTION_CHECK_BYTES = 3
FALLBACK_PROJECTION_TRANSCRIPT_BINDING_BYTES = 29
TARGET_TRANSACTION_BYTES = 250


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def current_state_slot_bytes() -> int:
    return (
        SLOT_ID_BYTES
        + ACCOUNT_LINK_BYTES
        + DIGEST_BYTES  # hidden_source_opening_commitment
        + DIGEST_BYTES  # restore_anchor_digest
        + DIGEST_BYTES  # pad_seed_digest
        + BOOL_BYTES
    )


def scenario(
    name: str,
    added_per_slot_bytes: int,
    resulting_signature_bytes: int,
    state_payload: str,
    theorem_impact: str,
    lifecycle_effect: str,
    recommendation: str,
    account_level_bytes: int = 0,
) -> dict[str, Any]:
    base = current_state_slot_bytes()
    return {
        "name": name,
        "additional_account_level_bytes": account_level_bytes,
        "additional_per_slot_bytes": added_per_slot_bytes,
        "raw_slot_bytes_after_change": base + added_per_slot_bytes,
        "state_payload": state_payload,
        "signature_bytes_after_change": resulting_signature_bytes,
        "signature_delta_vs_308": resulting_signature_bytes
        - (CHECKED_CORE_SIGNATURE_BYTES + FALLBACK_PROJECTION_CARRIER_BYTES),
        "target_delta_vs_250": resulting_signature_bytes - TARGET_TRANSACTION_BYTES,
        "lifecycle_effect": lifecycle_effect,
        "theorem_impact": theorem_impact,
        "recommendation": recommendation,
    }


def build_payload() -> dict[str, Any]:
    current_slot = current_state_slot_bytes()
    current_extra_over_account_link = current_slot - ACCOUNT_LINK_BYTES
    scenarios = [
        scenario(
            name="status_quo_308_byte_projection_carrier",
            added_per_slot_bytes=0,
            resulting_signature_bytes=CHECKED_CORE_SIGNATURE_BYTES
            + FALLBACK_PROJECTION_CARRIER_BYTES,
            state_payload="current slot: slot_id, 68-byte account commitment, hidden_source_opening_commitment, restore_anchor_digest, pad_seed_digest, unused bit",
            lifecycle_effect="existing selected projection lifecycle still burns slot, records pad_seed_digest, and preserves restore anchor",
            theorem_impact="keeps the current proof burden: signature-carried carrier must be promoted to a hidden-source accepting/extractor theorem",
            recommendation="baseline; no state growth but remains 58 bytes over the 250-byte transaction target",
        ),
        scenario(
            name="state_stored_projection_commitment_only",
            added_per_slot_bytes=DIGEST_BYTES,
            resulting_signature_bytes=CHECKED_CORE_SIGNATURE_BYTES
            + FALLBACK_PROJECTION_CARRIER_BYTES,
            state_payload="one extra digest committing to the projection carrier, certificate, or setup transcript",
            lifecycle_effect="must be fixed at account setup and consumed with the slot; replay policy unchanged except the digest must be included in spent records or committed snapshots",
            theorem_impact="authorization-only handle, not extraction by itself; still needs either the 56-byte carrier in the signature or an external availability proof",
            recommendation="useful as a binding hook, but not enough to remove signature bytes",
        ),
        scenario(
            name="state_stored_projection_lanes_and_check",
            added_per_slot_bytes=FALLBACK_PROJECTION_LANE_BYTES
            + FALLBACK_PROJECTION_CHECK_BYTES,
            resulting_signature_bytes=CHECKED_CORE_SIGNATURE_BYTES
            + FALLBACK_PROJECTION_TRANSCRIPT_BINDING_BYTES,
            state_payload="12 q12289 projection lanes plus 3 check bytes; transcript binding remains spend-contextual",
            lifecycle_effect="slot setup must prove lanes/check were derived from the hidden opening before challenge; spend still binds message/signature context",
            theorem_impact="splits the carrier theorem into setup extraction for algebraic lanes and spend-time binding for the 29-byte transcript field",
            recommendation="most plausible byte split if transcript binding cannot be setup-time; saves 27 transaction bytes for 27 permanent slot bytes",
        ),
        scenario(
            name="state_stored_full_projection_carrier",
            added_per_slot_bytes=FALLBACK_PROJECTION_CARRIER_BYTES,
            resulting_signature_bytes=CHECKED_CORE_SIGNATURE_BYTES,
            state_payload="all 56 carrier bytes moved into the slot",
            lifecycle_effect="normal spend flow only works if transcript binding is redefined as slot/setup-derived or if the slot is pre-authorized for a specific spend context",
            theorem_impact="could make the later spend authorization-only, but shifts soundness to account creation and requires a no-postchallenge-bias setup theorem",
            recommendation="saves all 56 carrier bytes but creates a strong setup/lifecycle theorem burden; only better if account state is cheap and slots are scarce",
        ),
        scenario(
            name="state_stored_certificate_root_and_carrier",
            added_per_slot_bytes=FALLBACK_PROJECTION_CARRIER_BYTES + DIGEST_BYTES,
            resulting_signature_bytes=CHECKED_CORE_SIGNATURE_BYTES,
            state_payload="56-byte carrier plus a 32-byte certificate/root digest for setup-time extractability evidence",
            lifecycle_effect="spent records must preserve both carrier and certificate root or commit to a snapshot containing them; wallet backups must treat certificate-root rollback as slot resurrection",
            theorem_impact="cleaner extraction story than bare carrier state, but proof still depends on setup certificate verification and state availability",
            recommendation="better theorem shape than full-carrier-only, but 88 permanent bytes per slot is unlikely to beat 56 transient signature bytes in blockchain constraints",
        ),
        scenario(
            name="account_root_plus_per_slot_authorization_leaf",
            account_level_bytes=DIGEST_BYTES,
            added_per_slot_bytes=DIGEST_BYTES,
            resulting_signature_bytes=CHECKED_CORE_SIGNATURE_BYTES,
            state_payload="one account-level setup root plus one per-slot authorization/extractability leaf digest",
            lifecycle_effect="requires consensus-visible leaf state or an inclusion proof; if leaves are not stored by full nodes, Merkle proofs move bytes back into the transaction",
            theorem_impact="turns spend verification into authorization against a setup root; extraction is proved at account setup, not from each spend transcript",
            recommendation="creative but only attractive for accounts with many precreated slots and consensus-visible leaves",
        ),
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_extractability_state_costs.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "target": {
            "n": 32,
            "q": 12289,
            "transaction_target_bytes": TARGET_TRANSACTION_BYTES,
            "checked_core_signature_bytes": CHECKED_CORE_SIGNATURE_BYTES,
            "selected_projection_signature_bytes": CHECKED_CORE_SIGNATURE_BYTES
            + FALLBACK_PROJECTION_CARRIER_BYTES,
            "fallback_projection_carrier_bytes": FALLBACK_PROJECTION_CARRIER_BYTES,
            "fallback_projection_lane_bytes": FALLBACK_PROJECTION_LANE_BYTES,
            "fallback_projection_check_bytes": FALLBACK_PROJECTION_CHECK_BYTES,
            "fallback_projection_transcript_binding_bytes": FALLBACK_PROJECTION_TRANSCRIPT_BINDING_BYTES,
        },
        "current_slot_byte_model": {
            "raw_slot_bytes": current_slot,
            "extra_raw_slot_bytes_over_68_byte_account_link": current_extra_over_account_link,
            "fields": [
                {"name": "slot_id", "bytes": SLOT_ID_BYTES},
                {"name": "account_commitment", "bytes": ACCOUNT_LINK_BYTES},
                {"name": "hidden_source_opening_commitment", "bytes": DIGEST_BYTES},
                {"name": "restore_anchor_digest", "bytes": DIGEST_BYTES},
                {"name": "pad_seed_digest", "bytes": DIGEST_BYTES},
                {"name": "unused", "bytes": BOOL_BYTES},
            ],
            "note": "Raw field sum only; Rust layout and JSON/database encoding overhead are intentionally excluded.",
        },
        "scenarios": scenarios,
        "preliminary_recommendation": {
            "summary": "Do not replace the 56 signature-carried projection bytes with permanent per-slot state unless the chain has cheap consensus account state and a setup-time proof path.",
            "best_research_direction": "state_stored_projection_lanes_and_check",
            "why": (
                "It isolates the algebraic extractability material in setup state while leaving the "
                "spend-context transcript binding in the signature; the byte trade is roughly neutral "
                "at 27 permanent bytes for 27 saved transaction bytes, but the theorem split is more credible "
                "than a fully precomputed message-bound carrier."
            ),
            "not_recommended_now": [
                "state_stored_full_projection_carrier",
                "state_stored_certificate_root_and_carrier",
            ],
            "reason_not_recommended": "They save at most 56 transaction bytes while adding 56 to 88 permanent bytes per slot and expanding rollback, backup, and setup-soundness obligations.",
        },
        "source_artifacts": {
            "account_state_schema_contract": rel(
                READINESS_DIR
                / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json"
            ),
            "blockchain_deployment_memo": rel(
                READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json"
            ),
            "production_reference": "tools/parameter-search/src/production_reference.rs",
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    current = payload["current_slot_byte_model"]
    target = payload["target"]
    lines = [
        "# Hidden-Source State-Assisted Extractability State Costs",
        "",
        "This preliminary artifact estimates whether branch C can move enough",
        "extractability material into account setup/state to avoid a larger",
        "hidden-source proof or to turn the later spend into authorization-only",
        "bytes.",
        "",
        "## Baseline",
        "",
        f"- transaction target bytes: `{target['transaction_target_bytes']}`",
        f"- selected projection signature bytes: `{target['selected_projection_signature_bytes']}`",
        f"- checked-core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- fallback projection carrier bytes: `{target['fallback_projection_carrier_bytes']}`",
        f"- current raw state-assisted slot bytes: `{current['raw_slot_bytes']}`",
        f"- current extra raw slot bytes over 68-byte account link: `{current['extra_raw_slot_bytes_over_68_byte_account_link']}`",
        "",
        "Raw field accounting excludes Rust alignment and database/JSON encoding overhead.",
        "",
        "## Current Slot Fields",
        "",
        "| field | bytes |",
        "|---|---:|",
    ]
    for field in current["fields"]:
        lines.append(f"| `{field['name']}` | {field['bytes']} |")
    lines.extend(
        [
            "",
            "## Scenarios",
            "",
            "| scenario | added account bytes | slot bytes | signature bytes | target delta | theorem impact | recommendation |",
            "|---|---:|---:|---:|---:|---|---|",
        ]
    )
    for row in payload["scenarios"]:
        account_bytes = row["additional_per_slot_bytes"]
        if row["additional_account_level_bytes"]:
            account_bytes = f"{account_bytes}/slot + {row['additional_account_level_bytes']}/account"
        lines.append(
            "| `{name}` | {account} | {slot} | {sig} | {target_delta:+d} | {theorem} | {rec} |".format(
                name=row["name"],
                account=account_bytes,
                slot=row["raw_slot_bytes_after_change"],
                sig=row["signature_bytes_after_change"],
                target_delta=row["target_delta_vs_250"],
                theorem=row["theorem_impact"],
                rec=row["recommendation"],
            )
        )
    rec = payload["preliminary_recommendation"]
    lines.extend(
        [
            "",
            "## Preliminary Recommendation",
            "",
            f"- summary: {rec['summary']}",
            f"- best research direction: `{rec['best_research_direction']}`",
            f"- why: {rec['why']}",
            f"- not recommended now: `{', '.join(rec['not_recommended_now'])}`",
            f"- reason: {rec['reason_not_recommended']}",
            "",
            "## Lifecycle Effects",
            "",
        ]
    )
    for row in payload["scenarios"]:
        lines.append(f"- `{row['name']}`: {row['lifecycle_effect']}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
