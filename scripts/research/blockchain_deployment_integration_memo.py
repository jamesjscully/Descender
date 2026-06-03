#!/usr/bin/env python3
"""Generate the blockchain deployment integration memo."""

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
PRODUCTION_LIFECYCLE_HARDENING_MEMO = (
    READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.json"
)
DEFAULT_JSON = READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.md"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_payload() -> dict[str, Any]:
    gap = load_json(PRODUCTION_REFERENCE_GAP_REPORT)
    lifecycle = load_json(PRODUCTION_LIFECYCLE_HARDENING_MEMO)
    facade = gap.get("production_facade", {})
    local_surfaces = {
        "account_lifecycle_manager": facade.get("has_library_lifecycle_manager"),
        "account_lifecycle_file_store": facade.get("has_library_lifecycle_file_store"),
        "selected_projection_chain_state_file_stores": facade.get(
            "has_library_selected_projection_lifecycle_file_store"
        ),
        "selected_projection_target_chain_adapter": facade.get(
            "has_library_selected_projection_lifecycle_file_store"
        ),
        "selected_projection_transactional_kv_binding": facade.get(
            "has_library_selected_projection_lifecycle_file_store"
        ),
        "target_chain_database_adapter_profile": facade.get(
            "has_library_selected_projection_lifecycle_file_store"
        ),
        "signature_binding_boundary": facade.get("has_library_sign_verify_boundary"),
        "verified_consumption": facade.get("has_library_verified_consumption"),
        "canonical_signature_codecs": facade.get("has_library_signature_codecs"),
        "challenge_frame": facade.get("has_library_challenge_frame"),
    }
    return {
        "schema": "Descender.blockchain_deployment_integration_memo.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source_artifacts": [
            rel(PRODUCTION_REFERENCE_GAP_REPORT),
            rel(PRODUCTION_LIFECYCLE_HARDENING_MEMO),
        ],
        "target": {
            "profile": "Descender.Signature.q12289.local_delta.stateful.v1",
            "transaction_bytes": facade.get("target_transaction_signature_bytes"),
            "account_link_bytes": facade.get("account_commitment_bytes"),
            "executable_signature_bytes": facade.get("executable_signature_bytes"),
            "executable_matches_target": facade.get("executable_matches_target"),
        },
        "local_reference_surfaces": local_surfaces,
        "required_deployment_bindings": [
            {
                "name": "consensus_spent_slot_state",
                "binding": (
                    "Full nodes must maintain spent state keyed by the canonical "
                    "public-key identity and slot id, equivalent to "
                    "ProductionSpentSlotLedger."
                ),
                "local_reference": (
                    "ProductionSpentSlotLedger, ProductionSpentSlotLedgerFileStore, "
                    "ProductionChainStateFileStores"
                ),
                "failure_if_missing": "a restored or forked account state can replay a valid slot",
            },
            {
                "name": "trusted_restore_anchor",
                "binding": (
                    "Wallets must store a trusted minimum snapshot sequence outside "
                    "ordinary restorable account backups."
                ),
                "local_reference": (
                    "ProductionLifecycleRestoreAnchor, load_manager_with_restore_anchor, "
                    "ProductionTrustedRestoreAnchorFileStore, and the selected "
                    "projection state-assisted lifecycle store"
                ),
                "failure_if_missing": "old backups can resurrect locally consumed slots",
            },
            {
                "name": "selected_projection_account_state",
                "binding": (
                    "Consensus-facing account state for the selected 308-byte/Fq12 "
                    "route must preserve hidden_source_opening_commitment and "
                    "restore_anchor_digest in each slot record; current state-assisted "
                    "records also preserve pad_seed_digest for replay and seed-reuse "
                    "exclusion."
                ),
                "local_reference": (
                    "ProductionReviewedFq12StateAssistedLifecycleSnapshot and "
                    "ProductionReviewedFq12StateAssistedLifecycleFileStore"
                ),
                "failure_if_missing": "nodes can verify against state that lost the hidden-source opening or pad-seed binding",
            },
            {
                "name": "selected_projection_consensus_transition",
                "binding": (
                    "The selected projection acceptance path must treat restore-anchor "
                    "freshness, expanded-opening-frame verification, slot consumption, "
                    "and spent-slot recording as one state transition."
                ),
                "local_reference": (
                    "apply_selected_projection_consensus_opening_frame, "
                    "ProductionSelectedProjectionConsensusTransition, and "
                    "ProductionSelectedProjectionChainStateFileStores; "
                    "ProductionSelectedProjectionTargetChainDbAdapter is the "
                    "target-chain database adapter contract"
                ),
                "failure_if_missing": "a node can update account state without nullifying the consumed slot",
            },
            {
                "name": "target_chain_database_adapter",
                "binding": (
                    "A concrete node backend must implement the selected projection "
                    "target-chain database contract: load lifecycle state, load "
                    "spent records, load trusted anchor, and atomically commit the "
                    "transition output plus monotonic trusted anchor. A production "
                    "claim must also carry a validated adapter profile naming the "
                    "canonical database engine, chain id, trusted-anchor placement, "
                    "key/value schema set, public-key identity rule, and external "
                    "storage review digest."
                ),
                "local_reference": (
                    "ProductionSelectedProjectionTargetChainDb, "
                    "ProductionSelectedProjectionTargetChainDbAdapter, and "
                    "ProductionInMemorySelectedProjectionTargetChainDb; "
                    "ProductionSelectedProjectionTargetChainKvDb binds the "
                    "contract to deterministic transactional KV keys; "
                    "ProductionTargetChainDatabaseAdapterProfile validates the "
                    "chain-specific production claim; "
                    "ProductionProfiledTargetChainKvDb gates the KV-backed "
                    "adapter behind that validated profile and rejects reference "
                    "KV engines; ProductionCanonicalNodeKvStore and "
                    "ProductionCanonicalNodeKvAdapter are the concrete node DB "
                    "boundary that receives the validated profile on every "
                    "committed read/write batch; "
                    "ProductionLocalCanonicalNodeKvStore is the concrete local "
                    "canonical JSON backend for executable storage review before "
                    "a target chain database is selected; "
                    "production_canonical_node_store_conformance_check is the "
                    "reusable gate a candidate chain store must pass for "
                    "initialize, commit, reopen, anchor/spent reload, and replay "
                    "rejection; ProductionSqliteCanonicalNodeKvStore is the "
                    "embedded SQLite canonical-store adapter that passes the same "
                    "gate with SQL transactions and durable reopen behavior; "
                    "ProductionMockBlockchain is the finalized-block simulator "
                    "over the local backend"
                ),
                "failure_if_missing": "deployment code can drift from the reviewed consensus transition semantics",
            },
            {
                "name": "transactional_kv_database_binding",
                "binding": (
                    "If the target node exposes a key-value store, the deployment "
                    "must implement atomic commit_batch semantics for the lifecycle, "
                    "spent-slot, and trusted-anchor keys produced by the selected "
                    "projection KV binding."
                ),
                "local_reference": (
                    "ProductionTargetChainTransactionalKv, "
                    "ProductionSelectedProjectionTargetChainKvDb, "
                    "ProductionTargetChainKvWrite, and "
                    "ProductionInMemoryTransactionalKv; "
                    "ProductionFileBackedTransactionalKv is the durable local "
                    "reference engine"
                ),
                "failure_if_missing": "a node database can partially commit consumed account state without replay state or anchor state",
            },
            {
                "name": "reorg_reveal_burn",
                "binding": (
                    "Any signature revealed to a mempool, block, or orphaned fork "
                    "must burn its checked slot, even when the transaction is later "
                    "reorged out."
                ),
                "local_reference": (
                    "burn_reorg_revealed_checked_signature and "
                    "burn_reorg_revealed_checked_signature_and_record_spent"
                ),
                "failure_if_missing": "chain rollback can reuse a revealed hidden slot",
            },
            {
                "name": "state_transition_order",
                "binding": (
                    "Chain integration must update spent-slot state no later than "
                    "the account-state transition; the reference persists the spent "
                    "ledger before the account snapshot."
                ),
                "local_reference": "ProductionChainStateFileStores.verify_consume_record_and_save",
                "failure_if_missing": "account and replay state can diverge after crash",
            },
            {
                "name": "storage_atomicity",
                "binding": (
                    "Production storage must provide atomic writes, writer "
                    "serialization, crash recovery, and stale-lock/lease policy for "
                    "both account and spent-ledger state."
                ),
                "local_reference": (
                    "ProductionLifecycleFileStore and "
                    "ProductionSpentSlotLedgerFileStore reference behavior"
                ),
                "failure_if_missing": "concurrent or interrupted writes can corrupt replay state",
            },
        ],
        "node_validation_flow": [
            "Decode and validate public key, account state, and signature bytes.",
            "Run the account-binding verifier and final account-carried verifier.",
            "Check the consensus spent-slot ledger before accepting the spend.",
            "Record the spent key atomically with the chain/account transition.",
            "For the selected projection route, use the consensus transition shape that returns both next account lifecycle snapshot and next spent-slot records.",
            "In the mock-chain reference, finalize a block by applying every selected-projection transaction through the profiled canonical DB and publishing the resulting finalized state root.",
            "On reorg or mempool reveal, burn checked revealed slots rather than restoring them.",
        ],
        "wallet_flow": [
            "Acquire the lifecycle store lock before reserve/sign/write.",
            "Reserve the slot durably before signature bytes can leave the signer.",
            "Advance the snapshot sequence on reservation and consumption.",
            "Reject restore if the snapshot sequence is below the trusted anchor.",
            "For the selected projection route, persist hidden_source_opening_commitment and restore_anchor_digest with each slot.",
            "For branch-C state-shift experiments, treat projection commitments, certificate roots, carrier bytes, or authorization leaves as consensus state rather than wallet-only metadata.",
            "Treat ambiguous reserved slots after crash as burned.",
        ],
        "still_not_deployment_claims": [
            "The executable facade still wraps the legacy 254-byte proof callback.",
            "The selected 308-byte/Fq12 projection-carrier verifier is fail-closed until exact fallback certificate and leakage semantics are implemented.",
            "The stores plus in-memory and file-backed adapter/KV backends are reference semantics; ProductionLocalCanonicalNodeKvStore and ProductionMockBlockchain are executable local canonical/mock-chain backends, ProductionSqliteCanonicalNodeKvStore is an embedded SQL canonical-store adapter for database-level conformance, and production target chains must implement ProductionCanonicalNodeKvStore against their canonical database transaction layer, pass production_canonical_node_store_conformance_check, and pass the adapter-profile validator with an external storage review digest.",
            "The trusted restore anchor has reference placements but no target deployment root-of-trust selection.",
            "Constant-time and side-channel review remains open.",
        ],
        "open_external_review": [
            "Review whether the spent-slot key should be exactly SHA256(public key) || slot_id or chain-specific account id || slot_id.",
            "Review chain rollback/mempool policy for when a revealed signature is considered burned.",
            "Review whether light clients need inclusion/non-inclusion proofs for spent-slot state.",
            "Review storage atomicity requirements for the target blockchain node database.",
            "Review wallet trusted-anchor placement, such as hardware counter, chain-observed state, or server-assisted monotonic state.",
            "Review whether branch-C extractability state is cheaper than signature bytes under the target chain's account-state rent, pruning, and light-client proof rules.",
        ],
        "lifecycle_open_blockers": lifecycle.get("open_blockers", []),
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    lines = [
        "# Blockchain Deployment Integration Memo",
        "",
        "This memo states the environment bindings needed to deploy the q12289",
        "account-carried signature target in a blockchain setting.  It bridges the",
        "local lifecycle/reference-store artifacts to node, wallet, and chain-state",
        "obligations.",
        "",
        "## Target",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- transaction target bytes: `{target['transaction_bytes']}`",
        f"- account-link bytes: `{target['account_link_bytes']}`",
        f"- executable signature bytes: `{target['executable_signature_bytes']}`",
        f"- executable matches target: `{target['executable_matches_target']}`",
        "",
        "## Local Reference Surfaces",
        "",
    ]
    lines.extend(
        f"- {name}: `{value}`"
        for name, value in payload["local_reference_surfaces"].items()
    )
    lines.extend(["", "## Required Deployment Bindings", ""])
    for item in payload["required_deployment_bindings"]:
        lines.append(
            f"- `{item['name']}`: {item['binding']}  Local reference: "
            f"{item['local_reference']}.  Failure mode: {item['failure_if_missing']}."
        )
    lines.extend(["", "## Node Validation Flow", ""])
    lines.extend(f"- {item}" for item in payload["node_validation_flow"])
    lines.extend(["", "## Wallet Flow", ""])
    lines.extend(f"- {item}" for item in payload["wallet_flow"])
    lines.extend(["", "## Still Not Deployment Claims", ""])
    lines.extend(f"- {item}" for item in payload["still_not_deployment_claims"])
    lines.extend(["", "## Open External Review", ""])
    lines.extend(f"- {item}" for item in payload["open_external_review"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
