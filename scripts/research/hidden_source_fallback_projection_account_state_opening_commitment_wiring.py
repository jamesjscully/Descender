#!/usr/bin/env python3
"""Record account-state wiring for the projection expanded-opening commitment."""

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
    / "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.md"
)
EXPANDED_BINDING_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.json"
)
PRODUCTION_LIB = ROOT / "tools/production-reference/src/lib.rs"
PARAMETER_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.account_state_opening_commitment_wiring.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    expanded_binding = load_json(EXPANDED_BINDING_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    implemented_symbols = [
        "HiddenSourceFallbackProjectionAccountSlotFrameSeed",
        "ProductionReviewedFq12StateAssistedAccountSlotRecord",
        "hidden_source_fallback_projection_account_state_from_slot_frames",
        "reviewed_fq12_state_assisted_account_state_from_slot_records",
        "ProductionReviewedFq12StateAssistedLifecycleSnapshot",
        "ProductionReviewedFq12StateAssistedAccountLifecycleManager",
        "ProductionReviewedFq12StateAssistedLifecycleFileStore",
        "PRODUCTION_REVIEWED_FQ12_STATE_ASSISTED_LIFECYCLE_STORE_SCHEMA",
        "verify_expanded_opening_frame_and_consume",
        "verify_expanded_opening_frame_consume_and_record_spent",
        "ProductionSelectedProjectionConsensusSpendRecord",
        "ProductionSelectedProjectionConsensusTransition",
        "apply_selected_projection_consensus_opening_frame",
        "record_slot_spent",
        "ProductionTrustedRestoreAnchorRecord",
        "ProductionTrustedRestoreAnchorFileStore",
        "ProductionSelectedProjectionChainStateFileStores",
        "ProductionSelectedProjectionTargetChainDb",
        "ProductionSelectedProjectionTargetChainDbAdapter",
        "ProductionInMemorySelectedProjectionTargetChainDb",
        "ProductionTargetChainTransactionalKv",
        "ProductionTargetChainKvWrite",
        "ProductionSelectedProjectionTargetChainKvDb",
        "ProductionInMemoryTransactionalKv",
        "ProductionFileBackedTransactionalKv",
        "ProductionFileBackedTransactionalKvLock",
        "PRODUCTION_TRUSTED_RESTORE_ANCHOR_SCHEMA",
        "PRODUCTION_TARGET_CHAIN_KV_NAMESPACE",
        "PRODUCTION_FILE_BACKED_TRANSACTIONAL_KV_SCHEMA",
        "ProductionTargetChainDatabaseAdapterProfile",
        "ProductionProfiledTargetChainKvDb",
        "ProductionCanonicalNodeKvStore",
        "ProductionCanonicalNodeKvAdapter",
        "ProductionTargetChainCanonicalStoreConformanceReport",
        "production_canonical_node_store_conformance_check",
        "ProductionLocalCanonicalNodeKvStore",
        "ProductionLocalCanonicalNodeKvStoreLock",
        "ProductionSqliteCanonicalNodeKvStore",
        "ProductionMockBlockchain",
        "ProductionMockBlockchainTransaction",
        "ProductionMockBlockchainBlock",
        "PRODUCTION_LOCAL_CANONICAL_NODE_KV_STORE_SCHEMA",
        "PRODUCTION_SQLITE_CANONICAL_NODE_KV_STORE_SCHEMA",
        "production_profiled_target_chain_db_from_canonical_store",
        "PRODUCTION_TARGET_CHAIN_DATABASE_ADAPTER_PROFILE_SCHEMA",
        "validate_production_target_chain_database_adapter_profile",
        "apply_opening_frame_and_save",
        "apply_opening_frame_and_commit",
        "commit_selected_projection_transition",
        "initialize_account_state",
        "commit_batch",
        "update_monotonic",
        "require_anchor",
        "verify_hidden_source_fallback_expanded_opening_state_slot_frame",
        "verify_hidden_source_fallback_expanded_opening_state_slot",
        "decode_hidden_source_fallback_account_witness_opening_frame",
        "ProductionReviewedFq12StateAssistedAccountSlot",
        "hidden_source_opening_commitment",
        "verify_hidden_source_fallback_expanded_opening_frame_commitment",
        "verify_hidden_source_fallback_expanded_opening_commitment",
    ]
    implemented = all(source_contains(symbol, sources) for symbol in implemented_symbols)
    test_symbols = [
        "verify_hidden_source_fallback_expanded_opening_state_slot",
        "verify_hidden_source_fallback_expanded_opening_state_slot_frame",
        "projection account state persists expanded opening commitment at creation",
        "hidden_source_opening_commitment",
        "restore_anchor_digest",
        "selected_projection_lifecycle_store_persists_opening_commitment_and_anchor",
        "consensus transition consumes and records selected projection slot",
        "slot is already in spent ledger",
        "missing trusted restore anchor record",
        "trusted restore anchor placement is not deployment-approved",
        "selected projection chain stores apply consensus transition",
        "projection expanded opening matches stored slot commitment",
        "projection expanded opening frame matches stored slot commitment",
        "hidden-source fallback expanded account opening commitment mismatch",
    ]
    tests_present = all(source_contains(symbol, [PRODUCTION_LIB.read_text()]) for symbol in test_symbols)
    status = (
        "implemented_fail_closed_reference_consensus_transition_target_chain_storage_open"
        if implemented and tests_present
        else "open"
    )
    digest_payload = {
        "expanded_binding_digest": expanded_binding.get("target", {}).get(
            "binding_digest"
        ),
        "implemented_symbols": implemented_symbols,
        "status": status,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_account_state_opening_commitment_wiring.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": status,
        "deployment_ready": False,
        "source_artifacts": {
            "expanded_opening_commitment_binding": rel(EXPANDED_BINDING_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "expanded_opening_commitment_binding_schema": expanded_binding.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "signature_byte_increase": 0,
            "account_state_wiring_digest": digest(digest_payload),
        },
        "wiring": {
            "stored_field": "ProductionReviewedFq12StateAssistedAccountSlot.hidden_source_opening_commitment",
            "stored_value": "SHA-256 expanded account-opening commitment for the selected projection lanes",
            "creation_condition": "account-state creation decodes canonical account opening-frame bytes and stores the recomputed expanded-opening commitment before challenge derivation",
            "persistence_record": "ProductionReviewedFq12StateAssistedAccountSlotRecord serializes slot id, account commitments, hidden_source_opening_commitment, restore_anchor_digest, and unused bit",
            "accept_condition": "slot is unused, account commitment validates, and recomputed expanded-opening commitment from frame bytes equals the stored slot commitment",
            "mismatch_action": "reject before acceptance",
            "implemented_symbols": implemented_symbols,
            "tests_present": tests_present,
        },
        "remaining_gaps": [
            {
                "id": "full_opening_frame_decoder",
                "status": "implemented_fail_closed",
                "detail": "the state-slot check now decodes the six q12289 source values from the canonical account opening frame before recomputing the expanded-opening digest",
            },
            {
                "id": "account_creation_persistence",
                "status": "implemented_reference_boundary",
                "detail": "the reference creation helper derives the projection expanded-opening commitment from canonical opening-frame bytes and the serializable state-assisted slot record persists it as hidden_source_opening_commitment",
            },
            {
                "id": "deployment_storage_hardening",
                "status": "implemented_reference_consensus_transition_target_chain_adapter_open",
                "detail": "the selected projection route now has a state-assisted lifecycle snapshot/store plus a consensus-facing transition that preserves hidden_source_opening_commitment and restore_anchor_digest, requires restore-anchor freshness, rejects spent-ledger replay, consumes the slot, records the spent key, wires those updates through separate lifecycle, spent-ledger, and trusted-anchor stores, exposes the same semantics through a target-chain database adapter contract with an executable in-memory conformance backend, binds that contract to deterministic schema-tagged transactional KV keys, includes a durable file-backed transactional KV reference engine with writer locks, temp-file fsync, and atomic rename, has a typed adapter-profile validator for canonical DB engine, approved anchor placement, KV schemas, identity binding, and external-review digest, gates KV adapter construction behind that profile while refusing reference KV engines, provides a canonical-node store adapter boundary that passes the validated profile into every chain DB read/write batch, includes a reusable canonical-store conformance check for initialize/commit/reopen/replay behavior, includes a concrete local canonical JSON backend with profile-digest persistence, writer locks, temp-file fsync, atomic rename, reload tests, profile-mismatch rejection, and conformance coverage, includes a concrete SQLite canonical-store adapter with schema metadata, profile-digest binding, WAL journaling, FULL synchronous durability, SQL transactions, reopen tests, profile-mismatch rejection, and conformance coverage, and includes a mock finalized blockchain simulator that finalizes selected-projection spends, publishes deterministic state roots, rejects replay, and reloads finalized state; selecting a production target chain's canonical database engine and third-party storage review remain open",
            },
            {
                "id": "hidden_relation_composition",
                "status": "open",
                "detail": "this wiring binds the selected account projection values to state; checked-core response-difference derivation and external review remain separate proof obligations",
            },
        ],
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    wiring = payload["wiring"]
    lines = [
        "# Hidden-Source Projection Account-State Opening Commitment Wiring",
        "",
        "This artifact records the account-state hook for the selected 308-byte",
        "projection-carrier route. The stored per-slot hidden-source opening",
        "commitment is interpreted as the expanded account-opening commitment for",
        "the selected projection lanes.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- signature byte increase: `{target['signature_byte_increase']}`",
        f"- account-state wiring digest: `{target['account_state_wiring_digest']}`",
        "",
        "## Wiring",
        "",
        f"- stored field: `{wiring['stored_field']}`",
        f"- stored value: {wiring['stored_value']}",
        f"- creation condition: {wiring['creation_condition']}",
        f"- persistence record: {wiring['persistence_record']}",
        f"- accept condition: {wiring['accept_condition']}",
        f"- mismatch action: `{wiring['mismatch_action']}`",
        f"- tests present: `{wiring['tests_present']}`",
        "",
        "## Remaining Gaps",
        "",
        "| gap | status | detail |",
        "|---|---|---|",
    ]
    for gap in payload["remaining_gaps"]:
        lines.append(f"| `{gap['id']}` | `{gap['status']}` | {gap['detail']} |")
    lines.extend(["", "## Source Artifacts", ""])
    for key, value in payload["source_artifacts"].items():
        lines.append(f"- `{key}`: `{value}`")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(f"wrote {rel(DEFAULT_JSON)}")
    print(f"wrote {rel(DEFAULT_MD)}")


if __name__ == "__main__":
    main()
