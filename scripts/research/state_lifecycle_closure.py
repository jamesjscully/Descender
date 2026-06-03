#!/usr/bin/env python3
"""Generate the PTC-007 state lifecycle closure artifact."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
LIFECYCLE_MEMO = READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.json"
BLOCKCHAIN_MEMO = READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json"
ADVERSARY = READINESS_DIR / "pre_third_party_state_lifecycle_adversary_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "state_lifecycle_closure_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "state_lifecycle_closure_n32_q12289.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    lifecycle = load_json(LIFECYCLE_MEMO)
    blockchain = load_json(BLOCKCHAIN_MEMO)
    adversary = load_json(ADVERSARY)
    return {
        "schema": "Descender.state_lifecycle_closure.v1",
        "campaign_id": "PTC-007",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "closure_status": "closed by policy artifact plus executable negative controls",
        "source_artifacts": [rel(LIFECYCLE_MEMO), rel(BLOCKCHAIN_MEMO), rel(ADVERSARY)],
        "trusted_anchor_policy": {
            "monotonicity_rule": (
                "For each public-key identity, the trusted restore anchor is the "
                "maximum snapshot sequence reached by any reservation, accepted "
                "verify-and-consume, or revealed-slot burn observed by the deployment."
            ),
            "placement_rule": (
                "The trusted restore anchor must be outside ordinary restorable "
                "wallet/account backups and must be non-exportable by the backup "
                "format."
            ),
            "allowed_roots_of_trust": [
                "chain-observed anchor derived from finalized or burned spent-slot state",
                "hardware monotonic counter sealed to the public-key identity",
                "quorum/server monotonic anchor with rollback-audited writes",
            ],
            "forbidden_roots_of_trust": [
                "same file or archive as ordinary account state",
                "user-editable preference or cache",
                "backup metadata restored before anchor comparison",
                "anchor value accepted without public-key identity binding",
            ],
        },
        "restore_anchor_constraints": [
            {
                "name": "restore_before_use_gate",
                "rule": (
                    "Before any restored state can reserve, sign, or verify-consume, "
                    "its snapshot sequence must be compared with the trusted anchor."
                ),
                "reject_if": "restored_snapshot_sequence < trusted_anchor_sequence",
            },
            {
                "name": "identity_bound_anchor",
                "rule": (
                    "Anchor keys are scoped by canonical public-key identity; an "
                    "anchor for one identity cannot authorize another account state."
                ),
                "reject_if": "anchor_public_key_hash != restored_public_key_hash",
            },
            {
                "name": "anchor_not_backup_mutated",
                "rule": (
                    "Restore must not lower the trusted anchor from backup contents; "
                    "backup anchors are advisory only unless independently witnessed."
                ),
                "reject_if": "backup_anchor_sequence < current_trusted_anchor_sequence",
            },
        ],
        "reorg_rollback_policy": [
            {
                "name": "revealed_signature_burn",
                "rule": (
                    "Any checked signature revealed to mempool, block, or orphaned "
                    "fork burns its public-key-hash/slot-id key."
                ),
            },
            {
                "name": "no_reorg_unspend",
                "rule": (
                    "A chain reorg can remove transaction effects but cannot make a "
                    "revealed slot reservable or spendable again."
                ),
            },
            {
                "name": "rollback_advances_anchor",
                "rule": (
                    "Rollback/recovery that encounters an ambiguous reserved or "
                    "revealed slot records a burn and advances the trusted anchor."
                ),
            },
        ],
        "slot_reuse_prevention": [
            "Reservation rejects any slot key present in reserved, spent, or burned state.",
            "Verify-and-consume rejects any slot key already present in spent state.",
            "Mempool and reorg burns are persisted under the same key type as accepted spends.",
            "Slot cursors are hints only; spent/burned/reserved ledgers are authoritative.",
        ],
        "deployment_acceptance_criteria": [
            "Adversary artifact reports zero unsafe_acceptances under closure policy.",
            "Backup-restore and trusted-anchor negative controls still accept replay when anchor rules are disabled.",
            "Implementation documents one allowed root of trust for the anchor before deployment.",
            "Restore tests cover stale snapshot, co-restored anchor, wrong identity, and ambiguous reserved-slot recovery.",
            "Reorg tests cover mempool reveal, orphaned block reveal, and attempted reuse after rollback.",
            "Consensus/node validation uses the same public-key-hash/slot-id spent key as wallet burn recovery.",
        ],
        "memo_alignment": {
            "required_deployment_bindings": [
                item["name"] for item in blockchain.get("required_deployment_bindings", [])
            ],
            "lifecycle_requirements": [
                item["name"] for item in lifecycle.get("lifecycle_requirements", [])
            ],
            "previous_adversary_bottom_line": adversary.get("bottom_line"),
        },
        "ptc_007_resolution": (
            "The trusted-anchor concern is no longer an open implementation risk in "
            "the pre-third-party packet: deployment is accepted only with an "
            "identity-bound, non-exportable, monotonic anchor plus revealed-slot burn "
            "semantics.  Implementations without that anchor placement fail the "
            "negative-control scenarios."
        ),
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# State Lifecycle Closure",
        "",
        "PTC-007 closure artifact for trusted-anchor placement and state lifecycle",
        "policy.  This is an acceptance gate: a deployment that cannot satisfy these",
        "rules remains non-deployable even if local replay tests pass.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- closure status: `{payload['closure_status']}`",
        f"- source artifacts: {', '.join(f'`{item}`' for item in payload['source_artifacts'])}",
        f"- resolution: {payload['ptc_007_resolution']}",
        "",
        "## Trusted Anchor Policy",
        "",
        f"- monotonicity: {payload['trusted_anchor_policy']['monotonicity_rule']}",
        f"- placement: {payload['trusted_anchor_policy']['placement_rule']}",
        "- allowed roots of trust:",
    ]
    lines.extend(
        f"  - {item}" for item in payload["trusted_anchor_policy"]["allowed_roots_of_trust"]
    )
    lines.append("- forbidden roots of trust:")
    lines.extend(
        f"  - {item}"
        for item in payload["trusted_anchor_policy"]["forbidden_roots_of_trust"]
    )
    lines.extend(["", "## Restore Anchor Constraints", ""])
    for item in payload["restore_anchor_constraints"]:
        lines.append(f"- `{item['name']}`: {item['rule']} Reject if `{item['reject_if']}`.")
    lines.extend(["", "## Reorg And Rollback Policy", ""])
    lines.extend(
        f"- `{item['name']}`: {item['rule']}"
        for item in payload["reorg_rollback_policy"]
    )
    lines.extend(["", "## Slot Reuse Prevention", ""])
    lines.extend(f"- {item}" for item in payload["slot_reuse_prevention"])
    lines.extend(["", "## Deployment Acceptance Criteria", ""])
    lines.extend(f"- {item}" for item in payload["deployment_acceptance_criteria"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": rel(DEFAULT_JSON),
                "markdown": rel(DEFAULT_MD),
                "closure_status": payload["closure_status"],
                "acceptance_criteria": len(payload["deployment_acceptance_criteria"]),
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
