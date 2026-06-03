#!/usr/bin/env python3
"""Bind lifecycle same-game controls to deployment DB/profile requirements."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"

LIFECYCLE_DISCHARGE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json"
)
EXTERNAL_STORAGE_PACKET_JSON = (
    READINESS_DIR / "external_storage_review_packet_n32_q12289.json"
)
BLOCKCHAIN_MEMO_JSON = READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json"
PRODUCTION_REFERENCE_GAP_REPORT_JSON = (
    READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
)

DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_lifecycle_db_profile_binding_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_lifecycle_db_profile_binding_n32_q12289.md"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def review_check(packet: dict[str, Any], name: str) -> dict[str, Any]:
    for item in packet.get("review_checks", []):
        if item.get("name") == name:
            return item
    raise KeyError(f"missing external storage review check: {name}")


def deployment_binding(memo: dict[str, Any], name: str) -> dict[str, Any]:
    for item in memo.get("required_deployment_bindings", []):
        if item.get("name") == name:
            return item
    raise KeyError(f"missing deployment binding: {name}")


def discharge_item(discharge: dict[str, Any], name: str) -> dict[str, Any]:
    for item in discharge.get("discharge_items", []):
        if item.get("id") == name:
            return item
    raise KeyError(f"missing lifecycle discharge item: {name}")


def build_control_bindings(
    discharge: dict[str, Any], packet: dict[str, Any], memo: dict[str, Any]
) -> list[dict[str, Any]]:
    spend_burn = discharge_item(discharge, "spend_burn_ledger_behavior")
    pad_seed = discharge_item(discharge, "pad_seed_digest_uniqueness")
    restore_reorg = discharge_item(discharge, "restore_reorg_replay")

    return [
        {
            "id": "durable_spent_burn_ledger",
            "lifecycle_terms": spend_burn.get("advantage_terms", []),
            "reference_controls": [
                "ProductionSpentSlotLedger persists spent records keyed by canonical public-key hash plus slot id.",
                "Selected-projection spends persist pad_seed_digest in the spent-slot record.",
                "Duplicate slot id and duplicate pad_seed_digest records reject before a later accept.",
                "Reorg/recovery policy burns ambiguous reserved or revealed checked slots instead of restoring eligibility.",
            ],
            "deployment_db_requirements": [
                "Consensus state must include a durable spent/burn ledger, not only wallet-local state.",
                "The ledger key must include chain id, canonical public-key identity, and slot id; the spent record must also store pad_seed_digest and transition metadata.",
                "Spent and burned states must be terminal across node restart, wallet restore, mempool eviction, and chain reorg.",
                "A commit that consumes account state must atomically insert the spent/burn record or fail without making the slot eligible.",
            ],
            "adapter_profile_binding": [
                "ProductionTargetChainDatabaseAdapterProfile must name a non-reference canonical DB engine and the deterministic spent-ledger schema.",
                "transaction_semantics must be single transaction, or a reviewed write-ahead/idempotent equivalent that gives the same observable atomicity.",
                "public_key_identity_rule must be the canonical public-key hash rule used for spent keys and restore anchors.",
            ],
            "external_storage_review_gaps": [
                review_check(packet, "identity_bound_spent_keys").get("requirement"),
                review_check(packet, "crash_recovery_order").get("requirement"),
                "No final production DB engine, DDL/column-family layout, compaction/retention rule, or reorg/finality retention rule is bound by this artifact.",
            ],
            "status": "binding_required_for_deployment_db",
        },
        {
            "id": "pad_seed_digest_uniqueness",
            "lifecycle_terms": pad_seed.get("advantage_terms", []),
            "reference_controls": [
                "Reference account-state construction rejects duplicate or zero public pad_seed_digest values.",
                "Slot opening binds the stored public pad_seed_digest before accepting an opening.",
                "Target-chain commit validation rejects a duplicate pad_seed_digest already present in the spent ledger.",
            ],
            "deployment_db_requirements": [
                "The canonical DB must enforce pad_seed_digest uniqueness for active selected-projection slots and for historical spent/burn records.",
                "The digest must be nonzero, public, and serialized in both account-state slot records and spent/burn records.",
                "The uniqueness check must run in the same transaction as selected-projection account transition acceptance.",
                "Database restore, snapshot import, index rebuild, or reorg selection must fail closed when duplicate digest state appears.",
            ],
            "adapter_profile_binding": [
                "The profile must name the pad_seed_digest schema/index and bind it to the external storage review digest.",
                "Schema drift that drops or weakens the uniqueness index must invalidate the profile digest.",
            ],
            "external_storage_review_gaps": [
                review_check(packet, "target_chain_adapter_binding").get("requirement"),
                "The external storage packet asks for deterministic KV schemas, but the final production DB uniqueness index and migration policy are not selected here.",
            ],
            "status": "binding_required_for_deployment_db",
        },
        {
            "id": "restore_anchor",
            "lifecycle_terms": restore_reorg.get("advantage_terms", []),
            "reference_controls": [
                "The reference restore anchor is monotonic and identity-bound.",
                "Snapshot replay older than the trusted restore anchor rejects.",
                "The selected-projection transition advances the trusted anchor with the accepted spend.",
            ],
            "deployment_db_requirements": [
                "The anchor must be stored outside ordinary account-state backup/restore and must not roll back with wallet backups.",
                "Anchor updates must be monotonic compare-and-set writes scoped by chain id and canonical public-key identity.",
                "Replay from a snapshot, export, light-client proof, or local backup with sequence below the anchor must reject before eligibility or opening verification.",
                "Anchor update and spent/burn ledger insert must be committed atomically with account-state consumption.",
            ],
            "adapter_profile_binding": [
                "trusted_anchor_placement must be an approved non-account-backup placement.",
                "The profile digest must bind the anchor schema, identity scope, monotonic update rule, and durability class.",
            ],
            "external_storage_review_gaps": [
                review_check(packet, "trusted_anchor_separation").get("requirement"),
                "The final deployment has not named the trusted anchor placement, backup exclusion mechanism, or monotonic write primitive.",
            ],
            "status": "binding_required_for_deployment_db",
        },
        {
            "id": "reorg_behavior",
            "lifecycle_terms": restore_reorg.get("advantage_terms", []),
            "reference_controls": [
                "Revealed checked signatures are burned on reorg/orphan handling instead of becoming eligible again.",
                "Recovery of ambiguous reserved selected-projection slots records burn state.",
            ],
            "deployment_db_requirements": [
                "A signature revealed to mempool, block validation, orphaned fork, or later-reorged block must durably burn the checked slot.",
                "Burn records must not be pruned or rolled back by normal chain reorg logic before the profile's reviewed retention horizon.",
                "Node and wallet behavior must agree on when a revealed slot becomes terminal, including non-inclusion and eviction cases.",
                "Light-client proofs must expose enough inclusion/non-inclusion or burn evidence to prevent local slot reanimation.",
            ],
            "adapter_profile_binding": [
                "The profile must bind the reorg/finality policy, burn-ledger namespace, retention horizon, and mempool exposure rule.",
                "Any deployment profile that allows reorg rollback of burn records is incompatible with discharging Adv_reorg_unburn(q_reorg).",
            ],
            "external_storage_review_gaps": [
                deployment_binding(memo, "reorg_reveal_burn").get("binding"),
                "The external storage packet requires reviewer mapping to mempool/reorg policy, but no target-chain finality or orphan-retention rule is present.",
            ],
            "status": "binding_required_for_deployment_db",
        },
        {
            "id": "crash_consistency",
            "lifecycle_terms": spend_burn.get("advantage_terms", []),
            "reference_controls": [
                "Reference target-chain adapter exposes lifecycle snapshot, spent-slot record, and trusted-anchor update as one commit hook.",
                "Reference KV bindings write deterministic lifecycle, spent-ledger, and anchor keys through commit_batch.",
                "Local reference stores serialize writers, use temp-file/rename behavior, and ignore uncommitted temp state.",
            ],
            "deployment_db_requirements": [
                "The production DB must provide atomic all-or-nothing visibility for account snapshot, spent/burn ledger, and restore anchor.",
                "After crash at any write boundary, recovery must either observe the terminal spent/burn state or reject/reburn the slot.",
                "Concurrent writers for the same account identity must be serialized by DB transaction isolation, explicit locks, or leases with stale-lock policy.",
                "Durability settings must be reviewed for the concrete engine, including fsync/synchronous mode, WAL behavior, batch semantics, and idempotent replay.",
            ],
            "adapter_profile_binding": [
                "transaction_semantics must bind the actual engine settings, not just adapter API shape.",
                "The external storage review digest must cover crash tests for power loss before, during, and after account/spent/anchor commit.",
            ],
            "external_storage_review_gaps": [
                review_check(packet, "atomic_triple_write").get("requirement"),
                review_check(packet, "transactional_kv_binding").get("requirement"),
                review_check(packet, "adapter_profile_fail_closed").get("requirement"),
            ],
            "status": "binding_required_for_deployment_db",
        },
        {
            "id": "profile_external_review_binding",
            "lifecycle_terms": ["Adv_lifecycle_db_profile_mismatch(q_life)"],
            "reference_controls": [
                "Reference profile validation rejects in-memory/file-backed reference engines, unsafe anchor placement, non-single-transaction semantics, schema drift, ambiguous public-key identity, and missing external storage review digest.",
                "Profiled KV adapter refuses to wrap reference KV engines.",
                "Local JSON and SQLite canonical-store adapters exercise profile-digest persistence, reopen persistence, anchor/spent reload, replay rejection, and profile-mismatch rejection.",
            ],
            "deployment_db_requirements": [
                "A production claim must name the concrete canonical node DB engine and version, chain id, schema set, identity rule, trusted-anchor placement, transaction semantics, and external review digest.",
                "The production DB must pass the same conformance gates against that exact profile, with reference engines disabled for deployment-ready status.",
                "Profile digest mismatch on reopen, migration, or node configuration change must reject before serving selected-projection state.",
            ],
            "adapter_profile_binding": [
                "This is the binding layer for Adv_lifecycle_db_profile_mismatch(q_life): the advantage term remains live until the named deployment profile and reviewed storage digest are present.",
                "The profile digest must be computed from reviewed production DB semantics, not from the reference file, in-memory, local JSON, or SQLite control backends alone.",
            ],
            "external_storage_review_gaps": [
                "The external storage packet status is ready_for_external_storage_review, not reviewed_and_bound.",
                "No external reviewer digest, target-chain database name/version, production schema migration policy, or deployment conformance transcript is present in the input artifacts.",
            ],
            "status": "residual_blocker",
        },
    ]


def build_payload() -> dict[str, Any]:
    discharge = load_json(LIFECYCLE_DISCHARGE_JSON)
    packet = load_json(EXTERNAL_STORAGE_PACKET_JSON)
    memo = load_json(BLOCKCHAIN_MEMO_JSON)
    gap = load_json(PRODUCTION_REFERENCE_GAP_REPORT_JSON)

    bindings = build_control_bindings(discharge, packet, memo)
    residual_ids = [
        item["id"]
        for item in bindings
        if item["status"] in {"residual_blocker", "binding_required_for_deployment_db"}
    ]
    external_packet_ready_only = packet.get("status") == "ready_for_external_storage_review"
    reference_engine_non_claim = "reference file stores" in packet.get("non_claim", "")

    return {
        "schema": "Descender.pre_signature_projection_lifecycle_db_profile_binding.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "target": {
            "profile": "Descender.Signature.q12289.local_delta.stateful.v1",
            "residual_advantage_term": "Adv_lifecycle_db_profile_mismatch(q_life)",
        },
        "source_artifacts": [
            rel(LIFECYCLE_DISCHARGE_JSON),
            rel(EXTERNAL_STORAGE_PACKET_JSON),
            rel(BLOCKCHAIN_MEMO_JSON),
            rel(PRODUCTION_REFERENCE_GAP_REPORT_JSON),
        ],
        "input_status": {
            "lifecycle_discharge": discharge.get("status"),
            "lifecycle_deployment_ready": discharge.get("deployment_ready"),
            "external_storage_review": packet.get("status"),
            "external_storage_deployment_ready": packet.get("deployment_ready"),
            "reference_facade_selected_projection_store": gap.get(
                "production_facade", {}
            ).get("has_library_selected_projection_lifecycle_file_store"),
        },
        "decision": {
            "status": "residual_blocker",
            "deployment_ready": False,
            "reason": (
                "Reference lifecycle controls and local canonical-store conformance "
                "tests are sufficient to define required DB semantics, but they do "
                "not bind a reviewed production database profile. "
                "Adv_lifecycle_db_profile_mismatch(q_life) remains live until "
                "a concrete non-reference node DB profile and external storage "
                "review digest are attached and pass conformance."
            ),
        },
        "control_bindings": bindings,
        "minimum_production_profile_fields": [
            "chain_id",
            "canonical_database_engine_and_version",
            "transaction_semantics_and_durability_settings",
            "trusted_anchor_placement",
            "canonical_public_key_identity_rule",
            "account_state_schema",
            "spent_burn_ledger_schema",
            "pad_seed_digest_uniqueness_schema_or_index",
            "restore_anchor_schema",
            "reorg_burn_retention_policy",
            "crash_recovery_and_idempotent_replay_policy",
            "schema_migration_and_profile_digest_policy",
            "external_storage_review_digest",
        ],
        "deployment_db_gap_summary": [
            "No concrete production DB engine/profile is selected in the input artifacts.",
            "No external storage review digest is bound to a production profile.",
            "Durable spent/burn ledger semantics are specified, but production retention, pruning, and reorg/finality behavior are not reviewed.",
            "pad_seed_digest uniqueness is enforced by reference controls, but no production DB uniqueness index or migration policy is bound.",
            "Restore-anchor separation is required, but final placement outside ordinary backups is not named.",
            "Crash consistency is required as an atomic triple write, but concrete engine durability settings and crash-test transcript are absent.",
        ],
        "consistency_checks": {
            "external_packet_ready_only": external_packet_ready_only,
            "external_packet_non_claims_reference_engines": reference_engine_non_claim,
            "lifecycle_discharge_not_deployment_ready": discharge.get("deployment_ready")
            is False,
            "all_bindings_have_external_gap": all(
                bool(item.get("external_storage_review_gaps")) for item in bindings
            ),
        },
        "close_condition": (
            "Replace residual_blocker with discharged only after the deployment "
            "profile names the production DB engine and schemas, carries a nonzero "
            "external storage review digest, rejects reference backends, passes "
            "canonical-store conformance including reopen/replay/profile-mismatch "
            "cases, and documents reorg plus crash behavior for the target chain."
        ),
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Lifecycle DB/Profile Binding",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- status: `{payload['decision']['status']}`",
        f"- deployment ready: `{payload['decision']['deployment_ready']}`",
        f"- residual term: `{payload['target']['residual_advantage_term']}`",
        f"- source artifacts: {', '.join(f'`{item}`' for item in payload['source_artifacts'])}",
        "",
        "## Decision",
        "",
        payload["decision"]["reason"],
        "",
        "## Control Bindings",
        "",
    ]
    for item in payload["control_bindings"]:
        lines.extend(
            [
                f"### `{item['id']}`",
                "",
                f"- status: `{item['status']}`",
                f"- lifecycle terms: {', '.join(f'`{term}`' for term in item['lifecycle_terms'])}",
                f"- reference controls: {'; '.join(item['reference_controls'])}",
                f"- deployment DB requirements: {'; '.join(item['deployment_db_requirements'])}",
                f"- adapter/profile binding: {'; '.join(item['adapter_profile_binding'])}",
                f"- external storage gaps: {'; '.join(item['external_storage_review_gaps'])}",
                "",
            ]
        )
    lines.extend(
        [
            "## Minimum Production Profile Fields",
            "",
        ]
    )
    for field in payload["minimum_production_profile_fields"]:
        lines.append(f"- `{field}`")
    lines.extend(["", "## Deployment DB Gaps", ""])
    for gap in payload["deployment_db_gap_summary"]:
        lines.append(f"- {gap}")
    lines.extend(
        [
            "",
            "## Close Condition",
            "",
            payload["close_condition"],
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
                "status": payload["decision"]["status"],
                "deployment_ready": payload["decision"]["deployment_ready"],
                "bindings": len(payload["control_bindings"]),
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
