#!/usr/bin/env python3
"""Generate the external storage review packet."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
PRODUCTION_REFERENCE_GAP_REPORT = (
    READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
)
BLOCKCHAIN_MEMO = READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "external_storage_review_packet_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "external_storage_review_packet_n32_q12289.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    gap = load_json(PRODUCTION_REFERENCE_GAP_REPORT)
    blockchain = load_json(BLOCKCHAIN_MEMO)
    facade = gap.get("production_facade", {})
    checks = [
        {
            "name": "atomic_triple_write",
            "requirement": (
                "Target-chain storage must commit lifecycle snapshot, spent-slot "
                "record, and trusted-anchor update in one database transaction or "
                "a recoverable write-ahead sequence with replay-safe idempotence; "
                "the reference path serializes all three writer surfaces and the "
                "target-chain adapter exposes this as one commit hook."
            ),
            "reference_surface": "ProductionSelectedProjectionChainStateFileStores.apply_opening_frame_and_save and ProductionSelectedProjectionTargetChainDbAdapter.apply_opening_frame_and_commit",
        },
        {
            "name": "target_chain_adapter_binding",
            "requirement": (
                "A deployment must implement ProductionSelectedProjectionTargetChainDb "
                "against the canonical node database with the same load and commit "
                "semantics as the in-memory conformance backend. The concrete "
                "deployment must also validate a ProductionTargetChainDatabaseAdapterProfile "
                "that names a non-reference canonical DB engine, approved trusted-anchor "
                "placement, deterministic KV schemas, public-key identity rule, and "
                "nonzero external storage review digest."
            ),
            "reference_surface": "ProductionSelectedProjectionTargetChainDb, ProductionInMemorySelectedProjectionTargetChainDb, ProductionTargetChainDatabaseAdapterProfile, and ProductionProfiledTargetChainKvDb",
        },
        {
            "name": "adapter_profile_fail_closed",
            "requirement": (
                "The adapter profile must reject in-memory/file-backed reference engines, "
                "ordinary account-backup trusted anchors, non-single-transaction semantics, "
                "schema drift, ambiguous public-key identity rules, and missing external "
                "storage review digests before any production-ready claim is accepted. "
                "The profiled KV adapter must also refuse to wrap the reference KV "
                "engines even when a caller supplies a valid-looking profile, and "
                "the canonical node adapter must pass the validated profile into "
                "the chain store on every committed read/write batch. The local "
                "canonical JSON backend must persist the profile digest with the "
                "store, reload committed batches, reject profile mismatches, "
                "serialize writers, and ignore uncommitted temp files. The "
                "canonical-store conformance check must exercise initialization, "
                "selected-projection commit, reopen persistence, anchor reload, "
                "spent-ledger reload, and replay rejection against any candidate "
                "target-chain store. The SQLite canonical-store adapter must pass "
                "the same gate using schema metadata, profile-digest binding, "
                "FULL synchronous durability, WAL journaling, SQL transactions, "
                "reopen persistence, and profile-mismatch rejection. The mock "
                "blockchain must finalize selected-projection spends through that "
                "profiled store, publish a deterministic finalized state root, "
                "reject replay, and reload finalized state from disk."
            ),
            "reference_surface": "validate_production_target_chain_database_adapter_profile, ProductionProfiledTargetChainKvDb::new, ProductionCanonicalNodeKvStore, ProductionCanonicalNodeKvAdapter, production_canonical_node_store_conformance_check, ProductionLocalCanonicalNodeKvStore, ProductionSqliteCanonicalNodeKvStore, and ProductionMockBlockchain",
        },
        {
            "name": "transactional_kv_binding",
            "requirement": (
                "For key-value node databases, commit_batch must be atomic across "
                "the lifecycle, spent-slot, and trusted-anchor keys emitted by "
                "ProductionSelectedProjectionTargetChainKvDb. The file-backed "
                "reference engine must reload committed batches and ignore "
                "uncommitted temp files."
            ),
            "reference_surface": "ProductionTargetChainTransactionalKv, ProductionSelectedProjectionTargetChainKvDb, ProductionInMemoryTransactionalKv, and ProductionFileBackedTransactionalKv",
        },
        {
            "name": "trusted_anchor_separation",
            "requirement": (
                "The trusted anchor store must be outside ordinary account-state "
                "backup/restore and must reject unapproved placements such as "
                "account_state_backup."
            ),
            "reference_surface": "ProductionTrustedRestoreAnchorFileStore and ProductionTrustedRestoreAnchorRecord",
        },
        {
            "name": "identity_bound_spent_keys",
            "requirement": (
                "Spent keys and trusted anchors are scoped by canonical public-key "
                "hash plus slot id; chain-specific account identifiers must map "
                "unambiguously to this identity."
            ),
            "reference_surface": "ProductionSpentSlotLedger::key",
        },
        {
            "name": "crash_recovery_order",
            "requirement": (
                "After crash at any write boundary, recovery must either observe "
                "the spent key and consumed slot, or reject/reburn the slot; it "
                "must never expose an accepting unspent consumed slot."
            ),
            "reference_surface": "state lifecycle closure and selected projection consensus transition",
        },
        {
            "name": "external_review_required",
            "requirement": (
                "A reviewer must map the reference file stores to the target node "
                "database, wallet backup system, mempool/reorg policy, and light "
                "client inclusion/non-inclusion proof model."
            ),
            "reference_surface": "blockchain deployment integration memo",
        },
    ]
    return {
        "schema": "Descender.external_storage_review_packet.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "ready_for_external_storage_review",
        "deployment_ready": False,
        "source_artifacts": [rel(PRODUCTION_REFERENCE_GAP_REPORT), rel(BLOCKCHAIN_MEMO)],
        "capability_evidence": {
            "selected_projection_chain_state_file_stores": facade.get(
                "has_library_selected_projection_lifecycle_file_store"
            ),
            "required_deployment_bindings": [
                row.get("name")
                for row in blockchain.get("required_deployment_bindings", [])
            ],
        },
        "review_checks": checks,
        "pass_condition": (
            "External reviewers confirm that target-chain storage gives the same "
            "atomicity, identity binding, monotonic anchor, spent-ledger replay "
            "rejection, adapter commit semantics, adapter-profile binding, and "
            "reorg-burn semantics as the reference transition."
        ),
        "non_claim": (
            "This packet does not claim the reference file stores or in-memory/"
            "file-backed adapter/KV backends are production database engines; they are "
            "executable semantics for the target chain database mapping."
        ),
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# External Storage Review Packet",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- source artifacts: {', '.join(f'`{item}`' for item in payload['source_artifacts'])}",
        "",
        "## Review Checks",
        "",
    ]
    for item in payload["review_checks"]:
        lines.append(
            f"- `{item['name']}`: {item['requirement']} Reference: {item['reference_surface']}."
        )
    lines.extend(
        [
            "",
            "## Pass Condition",
            "",
            payload["pass_condition"],
            "",
            "## Non-Claim",
            "",
            payload["non_claim"],
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
                "json": rel(DEFAULT_JSON),
                "markdown": rel(DEFAULT_MD),
                "status": payload["status"],
                "checks": len(payload["review_checks"]),
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
