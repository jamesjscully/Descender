#!/usr/bin/env python3
"""Generate the production-reference gap report for the account-carried target."""

from __future__ import annotations

import json
import re
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
SOURCE = ROOT / "tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs"
PRODUCTION_REFERENCE_LIB = ROOT / "tools/parameter-search/src/production_reference.rs"
PRODUCTION_REFERENCE_CRATE = ROOT / "tools/production-reference"
PRODUCTION_REFERENCE_CRATE_MANIFEST = PRODUCTION_REFERENCE_CRATE / "Cargo.toml"
PRODUCTION_REFERENCE_CRATE_LIB = PRODUCTION_REFERENCE_CRATE / "src/lib.rs"
PRODUCTION_REFERENCE_CRATE_TIMING_BIN = (
    PRODUCTION_REFERENCE_CRATE / "src/bin/production_reference_facade_timing.rs"
)
PRODUCTION_REFERENCE_PREFLIGHT_CERTIFICATE_BIN = (
    PRODUCTION_REFERENCE_CRATE / "src/bin/production_reference_preflight_certificate.rs"
)
COST_MODEL = (
    ROOT
    / "research/parameter_search/committed_sigma_account_carried_20260521/"
    "committed_sigma_account_carried_cost_model.json"
)
TIMING_REPORT = READINESS_DIR / "reference_timing_report_n32_q12289.json"
PRODUCTION_REFERENCE_FACADE_TIMING_REPORT = (
    READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.json"
)
DEFAULT_JSON = READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "production_reference_gap_report_n32_q12289.md"


def has_symbol(source: str, symbol: str) -> bool:
    return f"fn {symbol}" in source or f"struct {symbol}" in source


def load_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    return json.loads(path.read_text())


def export_preflight_certificate() -> dict[str, Any]:
    try:
        completed = subprocess.run(
            [
                "cargo",
                "run",
                "--quiet",
                "--manifest-path",
                str(PRODUCTION_REFERENCE_CRATE_MANIFEST),
                "--bin",
                "production_reference_preflight_certificate",
            ],
            check=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        payload = json.loads(completed.stdout)
        payload["export_status"] = "pass"
        payload["binary_source"] = str(
            PRODUCTION_REFERENCE_PREFLIGHT_CERTIFICATE_BIN.relative_to(ROOT)
        )
        return payload
    except (subprocess.CalledProcessError, json.JSONDecodeError) as exc:
        return {
            "schema": "Descender.production_reference_preflight_certificate.v1",
            "export_status": "fail",
            "binary_source": str(
                PRODUCTION_REFERENCE_PREFLIGHT_CERTIFICATE_BIN.relative_to(ROOT)
            ),
            "error": str(exc),
        }


def const_usize(source: str, name: str, fallback: int) -> int:
    values: dict[str, int] = {}
    expressions = {
        match.group(1): " ".join(match.group(2).split())
        for match in re.finditer(
            r"(?:pub\s+)?const\s+([A-Z0-9_]+):\s+usize\s*=\s*(.*?);",
            source,
            flags=re.S,
        )
    }

    def eval_expr(expr_name: str) -> int:
        if expr_name in values:
            return values[expr_name]
        expr = expressions[expr_name]
        total = 0
        for term in (part.strip() for part in expr.split("+")):
            if not term:
                continue
            if term.replace("_", "").isdigit():
                total += int(term.replace("_", ""))
            elif term in expressions:
                total += eval_expr(term)
            else:
                raise ValueError(f"unsupported usize const term {term!r} in {expr_name}")
        values[expr_name] = total
        return total

    try:
        return eval_expr(name)
    except (KeyError, ValueError):
        return fallback


def has_all(source: str, needles: list[str]) -> bool:
    return all(needle in source for needle in needles)


def library_capabilities(lib_source: str) -> dict[str, bool]:
    return {
        "has_library_account_commitment": "struct ProductionAccountCommitment"
        in lib_source
        and "REVIEWED_ACCOUNT_LINK_BYTES" in lib_source,
        "has_library_account_state_assembly": has_all(
            lib_source,
            [
                "struct ProductionAccountSlotSeed",
                "fn account_state_from_slot_seeds",
            ],
        ),
        "has_library_state_boundary": has_all(
            lib_source,
            [
                "struct ProductionPublicKey",
                "struct ProductionAccountSlot",
                "struct ProductionAccountState",
                "struct ProductionSignature",
            ],
        ),
        "has_library_account_state_export": has_all(
            lib_source,
            [
                "struct ProductionAccountSlotRecord",
                "fn checked_slot_records",
                "fn project_checked_slot_records",
            ],
        ),
        "has_library_verified_consumption": has_all(
            lib_source,
            [
                "struct ProductionVerification",
                "struct ProductionVerificationRejection",
                "struct ProductionRejectionReportFields",
                "fn rejection_report_fields",
                "PRODUCTION_REFERENCE_FACADE_PROOF_MODEL",
                "fn complete_verified_signature",
                "fn consume_verified_signature",
            ],
        ),
        "has_library_lifecycle_manager": has_all(
            lib_source,
            [
                "struct ProductionAccountLifecycleManager",
                "struct ProductionSlotReservation",
                "struct ProductionLifecycleError",
                "struct ProductionLifecycleSnapshot",
                "fn reserve_slot",
                "fn sign_reserved_and_consume",
                "fn verify_and_consume",
                "fn snapshot",
                "fn from_snapshot",
                "fn recover_snapshot_burning_reservations",
                "fn recover_reserved_slots_as_consumed",
            ],
        ),
        "has_library_lifecycle_file_store": has_all(
            lib_source,
            [
                "struct ProductionLifecycleFileStore",
                "PRODUCTION_LIFECYCLE_STORE_SCHEMA",
                "fn load_snapshot",
                "fn save_snapshot",
                "fn save_manager",
                "fn load_manager",
                "fn recover_manager_burning_reservations",
                "struct ProductionLifecycleStoreLock",
                "struct ProductionLifecycleLockRecord",
                "struct ProductionLifecycleRestoreAnchor",
                "struct ProductionReorgBurnRecord",
                "struct ProductionSpentSlotKey",
                "struct ProductionSpentSlotRecord",
                "struct ProductionSpentSlotLedger",
                "struct ProductionSpentSlotLedgerFileStore",
                "struct ProductionSpentSlotLedgerStoreLock",
                "struct ProductionChainStateFileStores",
                "PRODUCTION_LIFECYCLE_LOCK_SCHEMA",
                "PRODUCTION_SPENT_SLOT_LEDGER_SCHEMA",
                "fn lock_path",
                "fn try_lock",
                "fn try_lock_at",
                "fn load_lock_record",
                "fn break_stale_lock",
                "fn from_snapshot_with_restore_anchor",
                "fn load_manager_with_restore_anchor",
                "fn burn_reorg_revealed_checked_signature",
                "fn verify_consume_and_record_spent",
                "fn burn_reorg_revealed_checked_signature_and_record_spent",
                "fn record_checked_signature",
                "fn record_reorg_burn",
                "fn require_unspent",
                "fn load_ledger",
                "fn save_ledger",
                "fn from_records",
                "fn lifecycle_store",
                "fn spent_ledger_store",
                "fn verify_consume_record_and_save",
                "snapshot_sequence",
            ],
        ),
        "has_library_selected_projection_lifecycle_file_store": has_all(
            lib_source,
            [
                "struct ProductionReviewedFq12StateAssistedLifecycleSnapshot",
                "struct ProductionReviewedFq12StateAssistedAccountLifecycleManager",
                "struct ProductionReviewedFq12StateAssistedLifecycleFileStore",
                "PRODUCTION_REVIEWED_FQ12_STATE_ASSISTED_LIFECYCLE_STORE_SCHEMA",
                "struct ProductionSelectedProjectionConsensusSpendRecord",
                "struct ProductionSelectedProjectionConsensusTransition",
                "struct ProductionTrustedRestoreAnchorRecord",
                "struct ProductionTrustedRestoreAnchorFileStore",
                "struct ProductionTrustedRestoreAnchorStoreLock",
                "struct ProductionSelectedProjectionChainStateFileStores",
                "trait ProductionSelectedProjectionTargetChainDb",
                "struct ProductionSelectedProjectionTargetChainDbAdapter",
                "struct ProductionInMemorySelectedProjectionTargetChainDb",
                "trait ProductionTargetChainTransactionalKv",
                "struct ProductionTargetChainKvWrite",
                "struct ProductionSelectedProjectionTargetChainKvDb",
                "struct ProductionInMemoryTransactionalKv",
                "struct ProductionFileBackedTransactionalKv",
                "struct ProductionFileBackedTransactionalKvLock",
                "struct ProductionTargetChainDatabaseAdapterProfile",
                "struct ProductionProfiledTargetChainKvDb",
                "trait ProductionCanonicalNodeKvStore",
                "struct ProductionCanonicalNodeKvAdapter",
                "struct ProductionTargetChainCanonicalStoreConformanceReport",
                "fn production_canonical_node_store_conformance_check",
                "struct ProductionLocalCanonicalNodeKvStore",
                "struct ProductionLocalCanonicalNodeKvStoreLock",
                "struct ProductionSqliteCanonicalNodeKvStore",
                "struct ProductionMockBlockchain",
                "struct ProductionMockBlockchainTransaction",
                "struct ProductionMockBlockchainBlock",
                "PRODUCTION_TRUSTED_RESTORE_ANCHOR_SCHEMA",
                "PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT",
                "PRODUCTION_TARGET_CHAIN_KV_NAMESPACE",
                "PRODUCTION_TARGET_CHAIN_KV_LIFECYCLE_SCHEMA",
                "PRODUCTION_TARGET_CHAIN_KV_SPENT_SCHEMA",
                "PRODUCTION_TARGET_CHAIN_KV_ANCHOR_SCHEMA",
                "PRODUCTION_FILE_BACKED_TRANSACTIONAL_KV_SCHEMA",
                "PRODUCTION_LOCAL_CANONICAL_NODE_KV_STORE_SCHEMA",
                "PRODUCTION_SQLITE_CANONICAL_NODE_KV_STORE_SCHEMA",
                "PRODUCTION_TARGET_CHAIN_DATABASE_ADAPTER_PROFILE_SCHEMA",
                "PRODUCTION_TARGET_CHAIN_SINGLE_TRANSACTION_SEMANTICS",
                "fn hidden_source_fallback_projection_account_state_from_slot_frames",
                "fn verify_expanded_opening_frame_and_consume",
                "fn verify_expanded_opening_frame_consume_and_record_spent",
                "fn apply_selected_projection_consensus_opening_frame",
                "fn apply_opening_frame_and_save",
                "fn apply_opening_frame_and_commit",
                "fn commit_selected_projection_transition",
                "fn load_selected_projection_lifecycle_snapshot",
                "fn load_selected_projection_spent_slot_records",
                "fn load_selected_projection_trusted_anchor",
                "fn initialize_account_state",
                "fn commit_batch",
                "fn lifecycle_key",
                "fn spent_slots_key",
                "fn trusted_anchor_key",
                "fn tmp_path",
                "fn try_lock",
                "fn validate_production_target_chain_database_adapter_profile",
                "fn into_parts",
                "fn canonical_database_engine",
                "fn load_committed_value",
                "fn commit_canonical_batch",
                "fn production_profiled_target_chain_db_from_canonical_store",
                "fn profile_digest",
                "fn finalize_block",
                "fn finalized_state_root",
                "fn update_monotonic",
                "fn require_anchor",
                "fn record_slot_spent",
                "fn reviewed_fq12_state_assisted_account_state_from_slot_records",
                "hidden_source_opening_commitment",
                "restore_anchor_digest",
            ],
        ),
        "has_library_public_key_validation": has_all(
            lib_source,
            [
                "fn validate_public_key_bytes",
                "fn unpack_q_elements",
                "pub const PK_BYTES",
            ],
        ),
        "has_library_keypair_boundary": has_all(
            lib_source,
            [
                "struct ProductionSecretKey",
                "struct ProductionKeypair",
                "fn reference_keypair",
            ],
        ),
        "has_library_signature_codecs": has_all(
            lib_source,
            [
                "fn decode_production_signature",
                "fn encode_production_signature",
                "fn decode_challenge",
                "fn decode_responses",
                "fn encode_responses",
                "fn decode_openings",
                "fn encode_openings",
                "fn decode_linear_sigma_target_proof",
                "fn encode_linear_sigma_target_proof",
            ],
        ),
        "has_library_reviewed_250b_codec": has_all(
            lib_source,
            [
                "struct ProductionReviewedAccountCarriedProof",
                "struct ProductionReviewedAccountCarriedSignatureParts",
                "REVIEWED_ACCOUNT_CARRIED_MASKED_RESPONSE_BYTES",
                "REVIEWED_ACCOUNT_CARRIED_PROOF_BYTES",
                "fn encode_reviewed_account_carried_signature",
                "fn decode_reviewed_account_carried_signature",
                "fn encode_reviewed_account_carried_masked_response",
                "fn unpack_reviewed_account_carried_masked_response",
                "fn sign_reviewed_account_carried_with_account_binding",
                "fn sign_reviewed_account_carried_with_transcript_binding",
                "fn verify_reviewed_account_carried_account_binding",
                "fn verify_reviewed_account_carried_transcript_binding",
                "fn reviewed_account_carried_matrix_entry",
                "fn reviewed_account_carried_response_syndrome",
                "fn reviewed_account_carried_response_syndrome_digest",
                "fn pack_reviewed_account_carried_syndrome_digest",
                "fn unpack_reviewed_account_carried_syndrome_digest",
                "reviewed_account_carried_signature_codec_is_250_bytes",
                "reviewed_account_carried_quotient_map_packs_strict_10_residue_digest",
                "reviewed_account_carried_transcript_binding_checks_fiat_shamir_frame",
            ],
        ),
        "has_library_reviewed_252b_fq12_codec": has_all(
            lib_source,
            [
                "struct ProductionReviewedFq12AccountCarriedProof",
                "struct ProductionReviewedFq12AccountCarriedSignatureParts",
                "REVIEWED_FQ12_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES",
                "REVIEWED_FQ12_ACCOUNT_CARRIED_MASKED_RESPONSE_BYTES",
                "fn encode_reviewed_fq12_account_carried_signature",
                "fn decode_reviewed_fq12_account_carried_signature",
                "fn encode_reviewed_fq12_account_carried_masked_response",
                "fn unpack_reviewed_fq12_account_carried_masked_response",
                "fn sign_reviewed_fq12_account_carried_with_transcript_binding",
                "fn verify_reviewed_fq12_account_carried_transcript_binding",
                "fn reviewed_fq12_account_carried_matrix_entry",
                "fn reviewed_fq12_account_carried_response_syndrome",
                "fn reviewed_fq12_account_carried_response_syndrome_digest",
                "fn pack_reviewed_fq12_account_carried_syndrome_digest",
                "fn unpack_reviewed_fq12_account_carried_syndrome_digest",
                "reviewed_fq12_account_carried_signature_codec_is_252_bytes",
                "reviewed_fq12_account_carried_quotient_map_packs_strict_12_residue_digest",
                "reviewed_fq12_account_carried_transcript_binding_checks_fiat_shamir_frame",
            ],
        ),
        "has_library_reviewed_252b_fq12_sigma_equation": has_all(
            lib_source,
            [
                "REVIEWED_FQ12_ACCOUNT_CARRIED_CHALLENGE_ACTION_DOMAIN",
                "REVIEWED_FQ12_PAD_OPENING_VIEW_DOMAIN",
                "REVIEWED_FQ12_ACCOUNT_CARRIED_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH",
                "struct ProductionPublicLocalConsistency",
                "struct ProductionPublicPadOpeningConsistency",
                "struct ProductionPublicPadOpeningStatement",
                "fn verify_public_local_fiber_consistency",
                "fn verify_public_pad_opening_consistency",
                "fn public_pad_opening_statement_from_consistency",
                "fn public_pad_opening_statement",
                "fn reviewed_fq12_account_carried_pad_opening_statement",
                "fn reviewed_fq12_challenge_action_vector",
                "fn reviewed_fq12_account_carried_expected_response_syndrome",
                "fn verify_reviewed_fq12_account_carried_sigma_equation",
                "fn verify_reviewed_fq12_account_carried_checked_core",
                "reviewed_fq12_account_carried_sigma_equation_checks_action_relation",
            ],
        ),
        "has_library_reviewed_252b_fq12_pad_seed_witness_relation": has_all(
            lib_source,
            [
                "REVIEWED_FQ12_PAD_DOMAIN",
                "REVIEWED_FQ12_PAD_SEED_OPENING_DOMAIN",
                "REVIEWED_FQ12_HIDDEN_SOURCE_STATEMENT_DOMAIN",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_DOMAIN",
                "REVIEWED_FQ12_DEPLOYMENT_BLOCKER_REPORT_DOMAIN",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_SEMANTICS_GATE_DOMAIN",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_ACCEPTANCE_CONTRACT_DOMAIN",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_ACCEPTANCE_OBLIGATION_DOMAIN",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_ACCEPTANCE_OBLIGATION_REPORT_DOMAIN",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_READINESS_REPORT_DOMAIN",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_READINESS_VERDICT_DOMAIN",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_SEMANTICS_PROFILE",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_READINESS_VERDICT_BLOCKED",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_READINESS_VERDICT_DEPLOYABLE",
                "REVIEWED_FQ12_PAD_SEED_BYTES",
                "REVIEWED_FQ12_PAD_FRAME_BYTES",
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_BYTES",
                "REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_STATEMENT_BINDING",
                "REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_WITNESS_COMMITMENT_OPENING",
                "REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_PAD_SEED_OPENING",
                "REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_TRANSCRIPT_BINDING",
                "REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_SOUNDNESS_ANALYSIS",
                "REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_ZERO_KNOWLEDGE_ANALYSIS",
                "REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATIONS",
                "REVIEWED_FQ12_ACCEPTANCE_REQUIRED_OBLIGATION_COUNT",
                "REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_BITS",
                "REVIEWED_FQ12_ACCEPTANCE_OBLIGATION_LABELS",
                "HIDDEN_SOURCE_FALLBACK_DIRECT_EXPANDED_OPENING_DEPLOYMENT_STATUS",
                "REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_HIDDEN_SOURCE_PROOF_BYTES",
                "fn reviewed_fq12_pad_residues_from_seed",
                "fn reviewed_fq12_pad_opening_consistency_from_seed",
                "fn reviewed_fq12_pad_opening_statement_from_seed",
                "fn verify_reviewed_fq12_pad_opening_statement_witness",
                "fn reviewed_fq12_pad_seed_opening_digest",
                "fn verify_reviewed_fq12_pad_seed_opening",
                "fn verify_reviewed_fq12_state_assisted_pad_seed_opening",
                "REVIEWED_FQ12_ACCOUNT_WITNESS_PROJECTION_OPENING_DOMAIN",
                "fn reviewed_fq12_account_witness_projection_opening_digest",
                "fn verify_reviewed_fq12_state_assisted_account_witness_projection_opening",
                "REVIEWED_FQ12_STATE_ASSISTED_PROJECTION_CARRIER_BINDING_DOMAIN",
                "fn reviewed_fq12_state_assisted_projection_carrier_binding",
                "fn verify_reviewed_fq12_state_assisted_projection_carrier_tuple",
                "struct ProductionReviewedFq12HiddenSourceWitness",
                "struct ProductionReviewedFq12HiddenSourceStatement",
                "struct ProductionReviewedFq12HiddenSourceProof",
                "struct ProductionReviewedFq12HiddenSourceProofTranscript",
                "struct ProductionReviewedFq12HiddenSourceProofContext",
                "struct ProductionReviewedFq12DeploymentBlockerReport",
                "struct ProductionReviewedFq12HiddenSourceProofSemanticsGate",
                "struct ProductionReviewedFq12HiddenSourceProofAcceptanceContract",
                "struct ProductionReviewedFq12HiddenSourceProofAcceptanceObligation",
                "struct ProductionReviewedFq12HiddenSourceProofAcceptanceObligationReport",
                "struct ProductionReviewedFq12HiddenSourceProofReadinessReport",
                "struct ProductionReviewedFq12HiddenSourceProofReadinessVerdict",
                "required_obligation_bitmap",
                "implemented_obligation_bitmap",
                "missing_obligation_bitmap",
                "required_obligation_count",
                "implemented_obligation_count",
                "missing_obligation_count",
                "struct ProductionReviewedFq12AccountCarriedWithHiddenSourceProof",
                "fn from_bytes",
                "fn as_bytes",
                "fn reviewed_fq12_account_carried_hidden_source_statement",
                "fn verify_reviewed_fq12_hidden_source_statement_witness",
                "fn reviewed_fq12_hidden_source_statement_digest",
                "fn reviewed_fq12_hidden_source_proof_transcript",
                "fn reviewed_fq12_hidden_source_proof_transcript_digest",
                "fn reviewed_fq12_deployment_blocker_report_digest",
                "fn reviewed_fq12_hidden_source_proof_semantics_gate",
                "fn reviewed_fq12_hidden_source_proof_semantics_gate_digest",
                "fn verify_reviewed_fq12_hidden_source_proof_semantics_gate",
                "fn reviewed_fq12_hidden_source_proof_acceptance_contract",
                "fn reviewed_fq12_hidden_source_proof_acceptance_contract_digest",
                "fn validate_reviewed_fq12_hidden_source_proof_acceptance_contract",
                "fn reviewed_fq12_hidden_source_proof_acceptance_obligations",
                "fn reviewed_fq12_hidden_source_proof_acceptance_obligation_digest",
                "fn validate_reviewed_fq12_hidden_source_proof_acceptance_obligation",
                "fn reviewed_fq12_hidden_source_proof_acceptance_obligation_report",
                "fn reviewed_fq12_hidden_source_proof_acceptance_obligation_report_digest",
                "fn validate_reviewed_fq12_hidden_source_proof_acceptance_obligation_report",
                "fn reviewed_fq12_hidden_source_proof_readiness_report",
                "fn reviewed_fq12_hidden_source_proof_readiness_report_digest",
                "fn validate_reviewed_fq12_hidden_source_proof_readiness_report",
                "fn reviewed_fq12_hidden_source_proof_readiness_verdict",
                "fn reviewed_fq12_hidden_source_proof_readiness_verdict_digest",
                "fn validate_reviewed_fq12_hidden_source_proof_readiness_verdict",
                "fn require_reviewed_fq12_hidden_source_proof_deployable_verdict",
                "fn verify_reviewed_fq12_hidden_source_proof_acceptance_contract",
                "fn verify_reviewed_fq12_account_carried_hidden_source_witness",
                "PRODUCTION_HIDDEN_SOURCE_PROOF_FAILURE_REASON",
                "verify_reviewed_fq12_account_carried_hidden_source_proof",
                "verify_reviewed_fq12_account_carried_hidden_source_proof_with_transcript",
                "fn encode_reviewed_fq12_account_carried_with_hidden_source_proof",
                "fn decode_reviewed_fq12_account_carried_with_hidden_source_proof",
                "fn reviewed_fq12_account_carried_signature_carried_hidden_source_proof_context",
                "fn reviewed_fq12_account_carried_signature_carried_deployment_blocker_report",
                "fn reviewed_fq12_account_carried_signature_carried_readiness_report",
                "fn require_reviewed_fq12_account_carried_signature_carried_deployable",
                "fn verify_reviewed_fq12_account_carried_signature_carried_hidden_source_proof",
                "fn verify_reviewed_fq12_account_carried_signature_carried_hidden_source_proof_with_transcript",
                "PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS",
                "fn verify_reviewed_fq12_account_carried_signature_carried_deployment_verifier",
                "reviewed_fq12_hidden_source_proof_callback_present",
            ],
        ),
        "has_library_reviewed_308b_fallback_projection_codec": has_all(
            lib_source,
            [
                "HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES",
                "REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES",
                "struct HiddenSourceFallbackProjectionCarrier",
                "struct ProductionReviewedFq12AccountCarriedWithFallbackProjection",
                "fn encode_reviewed_fq12_account_carried_with_fallback_projection",
                "fn decode_reviewed_fq12_account_carried_with_fallback_projection",
                "decode_reviewed_fq12_account_carried_with_fallback_projection",
            ],
        ),
        "has_library_reviewed_252b_fq12_fail_closed_deployment_verifier": has_all(
            lib_source,
            [
                "PRODUCTION_DEPLOYMENT_BLOCKER_PAD_OPENING_BINDING",
                "fn verify_reviewed_fq12_account_carried_deployment_verifier",
                "reviewed_fq12_deployment_verifier_present",
            ],
        ),
        "has_library_challenge_frame": has_all(
            lib_source,
            [
                "fn challenge_hash_frame",
                "fn legacy_stateful_challenge_hash_frame",
                "fn hash_frame_to_challenge_bytes",
            ],
        ),
        "has_library_sign_verify_boundary": has_all(
            lib_source,
            [
                "fn sign_with_account_binding",
                "fn sign_with_secret_key",
                "fn verify_account_binding",
                "fn verify_with_account_binding",
                "struct ProductionVerifiedSignature",
                "struct ProductionVerificationRejection",
            ],
        ),
        "has_library_sanitized_callback_errors": has_all(
            lib_source,
            [
                "PRODUCTION_SIGNER_CALLBACK_FAILURE_REASON",
                "PRODUCTION_VERIFIER_CALLBACK_FAILURE_REASON",
                "fn sign_with_account_binding_sanitized",
                "fn sign_with_secret_key_sanitized",
                "fn verify_with_account_binding_sanitized",
                "fn sign_reserved_and_consume_sanitized",
                "fn verify_and_consume_sanitized",
                "fn verify_consume_and_record_spent_sanitized",
                "fn verify_consume_record_and_save_sanitized",
            ],
        ),
        "has_library_deployment_preflight": has_all(
            lib_source,
            [
                "PRODUCTION_DEPLOYMENT_PREFLIGHT_REPORT_DOMAIN",
                "PRODUCTION_DEPLOYMENT_PREFLIGHT_BLOCKER_DOMAIN",
                "PRODUCTION_DEPLOYMENT_PREFLIGHT_BLOCKER_REPORT_DOMAIN",
                "PRODUCTION_DEPLOYMENT_PREFLIGHT_VERDICT_DOMAIN",
                "PRODUCTION_DEPLOYMENT_PREFLIGHT_READY_ASSERTION_DOMAIN",
                "PRODUCTION_DEPLOYMENT_PREFLIGHT_CERTIFICATE_DOMAIN",
                "PRODUCTION_DEPLOYMENT_PREFLIGHT_VERDICT_BLOCKED",
                "PRODUCTION_DEPLOYMENT_PREFLIGHT_VERDICT_DEPLOYABLE",
                "PRODUCTION_DEPLOYMENT_PREFLIGHT_NO_BLOCKER",
                "PRODUCTION_DEPLOYMENT_BLOCKER_SELECTED_VERIFIER",
                "struct ProductionDeploymentPreflight",
                "struct ProductionDeploymentPreflightReport",
                "struct ProductionDeploymentPreflightBlocker",
                "struct ProductionDeploymentPreflightBlockerReport",
                "struct ProductionDeploymentPreflightVerdict",
                "struct ProductionDeploymentPreflightReadyAssertion",
                "struct ProductionDeploymentPreflightCertificate",
                "fn production_deployment_preflight",
                "fn production_deployment_preflight_report",
                "fn production_deployment_preflight_report_digest",
                "fn validate_production_deployment_preflight_report",
                "fn production_deployment_preflight_blockers",
                "fn production_deployment_preflight_blocker_digest",
                "fn validate_production_deployment_preflight_blocker",
                "fn production_deployment_preflight_blocker_report",
                "fn production_deployment_preflight_blocker_report_digest",
                "fn validate_production_deployment_preflight_blocker_report",
                "fn production_deployment_preflight_verdict",
                "fn production_deployment_preflight_verdict_digest",
                "fn validate_production_deployment_preflight_verdict",
                "fn production_deployment_preflight_ready_assertion",
                "fn production_deployment_preflight_ready_assertion_digest",
                "fn validate_production_deployment_preflight_ready_assertion",
                "fn production_deployment_preflight_certificate",
                "fn production_deployment_preflight_certificate_digest",
                "fn validate_production_deployment_preflight_certificate",
                "fn require_production_deployment_preflight_ready",
                "PRODUCTION_DEPLOYMENT_BLOCKER_PAD_OPENING_BINDING",
                "PRODUCTION_DEPLOYMENT_BLOCKER_HIDDEN_SOURCE_PROOF_SEMANTICS",
                "PRODUCTION_DEPLOYMENT_BLOCKER_CONSTANT_TIME",
                "PRODUCTION_DEPLOYMENT_BLOCKER_STORAGE",
                "PRODUCTION_DEPLOYMENT_BLOCKER_EXTERNAL_REVIEW",
                "deployment_preflight_fails_closed_until_selected_verifier_and_reviews_land",
            ],
        ),
    }


def production_reference_crate_status() -> dict[str, Any]:
    manifest_present = PRODUCTION_REFERENCE_CRATE_MANIFEST.exists()
    lib_present = PRODUCTION_REFERENCE_CRATE_LIB.exists()
    manifest = PRODUCTION_REFERENCE_CRATE_MANIFEST.read_text() if manifest_present else ""
    lib_source = PRODUCTION_REFERENCE_CRATE_LIB.read_text() if lib_present else ""
    reexports_library_surface = (
        "pub use descender_parameter_search::production_reference::*;" in lib_source
    )
    fail_closed_tests = has_all(
        lib_source,
        [
            "crate_boundary_fails_closed_until_selected_verifier_lands",
            "production_deployment_preflight",
            "PRODUCTION_DEPLOYMENT_BLOCKER_FINAL_VERIFIER",
        ],
    )
    sanitized_profile_test = has_all(
        lib_source,
        [
            "crate_boundary_exposes_default_sanitized_callback_profile",
            "verify_default_deployment_callback_boundaries",
        ],
    )
    facade_helpers = has_all(
        lib_source,
        [
            "deployment_sign_with_secret_key",
            "deployment_sign_reserved_and_consume",
            "deployment_verify_with_account_binding",
        ],
    )
    facade_tests = has_all(
        lib_source,
        [
            "facade_sign_uses_sanitized_secret_key_boundary",
            "facade_verify_uses_sanitized_boundary_and_consumes_only_on_accept",
            "facade_reserved_sign_consumes_reserved_slot",
        ],
    )
    facade_timing_binary = PRODUCTION_REFERENCE_CRATE_TIMING_BIN.exists()
    path_dependency = 'descender-parameter-search = { path = "../parameter-search" }' in manifest
    present = (
        manifest_present
        and lib_present
        and path_dependency
        and reexports_library_surface
        and fail_closed_tests
        and sanitized_profile_test
        and facade_helpers
        and facade_tests
        and facade_timing_binary
    )
    return {
        "present": present,
        "manifest": str(PRODUCTION_REFERENCE_CRATE_MANIFEST.relative_to(ROOT)),
        "library": str(PRODUCTION_REFERENCE_CRATE_LIB.relative_to(ROOT)),
        "facade_timing_binary": str(
            PRODUCTION_REFERENCE_CRATE_TIMING_BIN.relative_to(ROOT)
        ),
        "manifest_present": manifest_present,
        "library_present": lib_present,
        "path_dependency_on_parameter_search": path_dependency,
        "reexports_production_reference_surface": reexports_library_surface,
        "fail_closed_preflight_tests": fail_closed_tests,
        "sanitized_callback_profile_test": sanitized_profile_test,
        "deployment_facade_helpers": facade_helpers,
        "deployment_facade_tests": facade_tests,
        "deployment_facade_timing_binary_present": facade_timing_binary,
        "remaining_gap": (
            "This is a production API boundary over the existing library module "
            "with sanitized facade helpers. The exact 250-byte row remains "
            "blocked on a pinned 10-row action and is tracked as a research "
            "target, while the selected reviewed 252-byte/Fq12 fallback codec, "
            "checked-core verifier, canonical public pad-opening "
            "statement, canonical hidden-source statement digest, executable "
            "pad-seed/hidden-source witness relation, a 32-byte opaque "
            "hidden-source proof carrier/parser, archived strict 284-byte signature-carried "
            "proof envelope, typed canonical recomputed-statement proof transcript object, public signature-carried proof context builder, canonical fail-closed deployment blocker report digest, canonical fail-closed proof-semantics gate digest, hashable acceptance-contract obligations including the explicit missing mask-PRF domain-separation obligation, sanitized transcript-aware hidden-source proof callback boundary, and selected strict 308-byte projection-carrier envelope, and "
            "deployment verifier with an explicit projection-carrier proof-semantics blocker are "
            "now exposed. The deployment verifier still refuses to accept until "
            "active-lane verifier promotion, hidden-source proof semantics, "
            "constant-time review, storage hardening, and external review close."
        ),
    }


def api_rows(
    source: str, lib_source: str, capabilities: dict[str, bool]
) -> list[dict[str, Any]]:
    has_library_account_commitment = capabilities["has_library_account_commitment"]
    has_library_account_state_assembly = capabilities[
        "has_library_account_state_assembly"
    ]
    has_library_verified_consumption = capabilities["has_library_verified_consumption"]
    has_library_lifecycle_manager = capabilities["has_library_lifecycle_manager"]
    has_library_lifecycle_file_store = capabilities["has_library_lifecycle_file_store"]
    has_library_public_key_validation = capabilities["has_library_public_key_validation"]
    has_library_keypair_boundary = capabilities["has_library_keypair_boundary"]
    has_library_signature_codecs = capabilities["has_library_signature_codecs"]
    has_library_reviewed_250b_codec = capabilities["has_library_reviewed_250b_codec"]
    has_library_reviewed_252b_fq12_codec = capabilities[
        "has_library_reviewed_252b_fq12_codec"
    ]
    has_library_reviewed_252b_fq12_sigma_equation = capabilities[
        "has_library_reviewed_252b_fq12_sigma_equation"
    ]
    has_library_reviewed_252b_fq12_pad_seed_witness_relation = capabilities[
        "has_library_reviewed_252b_fq12_pad_seed_witness_relation"
    ]
    has_library_reviewed_308b_fallback_projection_codec = capabilities[
        "has_library_reviewed_308b_fallback_projection_codec"
    ]
    has_library_reviewed_252b_fq12_fail_closed_deployment_verifier = capabilities[
        "has_library_reviewed_252b_fq12_fail_closed_deployment_verifier"
    ]
    has_production_facade = all(
        has_symbol(source, symbol)
        for symbol in [
            "production_keygen",
            "production_sign",
            "production_verify",
        ]
    ) and has_library_account_state_assembly
    rows = [
        {
            "api": "keygen(slot_count)",
            "current_surface": "production_keygen(slot_count) facade over reference_keygen(slot_count), returning a library-validated ProductionPublicKey",
            "present_in_fixture": has_symbol(source, "production_keygen")
            and has_library_public_key_validation
            and has_library_keypair_boundary,
            "production_requirement": "Generate public key and account slots without ideal verifier shortcuts; expose deterministic test vectors and randomized production entropy hooks.",
            "remaining_gap": "Public-key byte length, canonical q-coordinate validation, and the production keypair/secret-key wrapper now live in the library boundary, but keygen is still a research-binary facade and its secret side still wraps reference key material.",
            "status": "partial",
        },
        {
            "api": "account_commit(slot)",
            "current_surface": "ProductionAccountCommitment library type + account_state_from_slot_seeds(slot_seeds, randomizer_commitment)",
            "present_in_fixture": has_library_account_commitment
            and has_library_account_state_assembly,
            "production_requirement": "Expose the account-carried witness commitment, randomizer commitment, and 4-byte response-link code as canonical account state.",
            "remaining_gap": "The 68-byte layout and checked account-state assembly are now in the library module and used by keygen; remaining work is to move the heavy sign/verify callbacks out of the research binary and replace the fixed research randomizer commitment with the reviewed account-carried construction.",
            "status": "partial" if has_production_facade else "open",
        },
        {
            "api": "sign(secret, account_slot, message)",
            "current_surface": "production_sign(secret_key, account_slot, message) delegates through library sign_with_secret_key; the library also exposes canonical 250-byte account-carried codec plus the 252-byte/Fq12 fallback codec and transcript-binding checkers",
            "present_in_fixture": has_symbol(source, "production_sign"),
            "production_requirement": "Produce the account-carried bounded-kernel spend transaction with no public post-challenge representative shortcut. The current robust implementation target is the 252-byte/Fq12 fallback unless a 250-byte action adapter is proven.",
            "remaining_gap": "Library sign boundary now owns the production secret-key handoff, slot state, witness commitment handoff, slot-id binding, randomizer commitment binding, reviewed Fiat-Shamir challenge binding, sanitized production callback error wrappers, the canonical 250-byte account-carried codec, the 252-byte/Fq12 fallback codec, and the selected strict 308-byte projection-carrier envelope. It remains fail-closed until active-lane verifier promotion and hidden-source proof semantics are implemented.",
            "status": "partial",
        },
        {
            "api": "verify(public_key, account_state, message, signature)",
            "current_surface": "production_verify(public_key, account_state, message, signature) delegates through library verify_with_account_binding; reviewed account-carried transcript checkers decode the 250-byte exact target and the 252-byte/Fq12 fallback and check account randomizer/slot binding plus the reviewed Fiat-Shamir frame",
            "present_in_fixture": has_symbol(source, "production_verify")
            and has_library_reviewed_250b_codec
            and has_library_reviewed_252b_fq12_codec,
            "production_requirement": "Verify via the account-link proof system and reject any public post-challenge representative reconstruction.",
            "remaining_gap": "Library verify boundary now owns public-key/account/signature handoff, 250-byte and 252-byte canonical decoding, slot-state checks, account commitment validation, randomizer binding, Fiat-Shamir challenge recomputation over public key/message/slot/account digest/proof commitment, the 252-byte/Fq12 checked-core verifier, public local-fiber consistency, public pad-opening consistency across repeated blocks, a canonical public pad-opening statement digest, a canonical pad-seed opening digest, a state-assisted pad-seed opening check against the prechallenge hidden-source opening commitment, a canonical account-witness projection-opening digest, a state-assisted account-witness projection-opening check against the hidden-source statement and opening tuple, a selected state-assisted opening tuple commitment model with the direct expanded-opening slot path archived as non-deployment, a selected-path state-assisted projection-carrier tuple verifier that composes slot opening, projection-opening digest, pad-seed opening, hidden-material projection lanes, and 56-byte carrier check/transcript binding, a canonical hidden-source public statement digest, an executable hidden-source witness relation tying the account witness syndrome and pad seed to that statement, a 32-byte opaque proof carrier/parser, an archived strict 284-byte signature-carried proof envelope, a typed canonical hidden-source proof transcript object that recomputes the statement digest from statement fields, a public signature-carried proof context builder that returns statement/proof/transcript, a fail-closed deployment blocker report carrying that context and blocker reason with a canonical recomputed-context digest, a typed proof-semantics gate pinned to the current partial executable-relation profile with its own canonical digest, a hashable acceptance contract that now marks statement binding, pad-seed opening, and transcript binding implemented while leaving full account-witness commitment opening, soundness, zero-knowledge, and mask-PRF domain-separation obligations missing, a sanitized transcript-aware hidden-source proof callback boundary, selected strict 308-byte projection-carrier encode/decode, a fail-closed deployment verifier with an explicit projection-carrier proof-semantics blocker, checked account-state record export/projection, typed rejection propagation/report-field extraction, sanitized production callback error wrappers, and checked-context callback dispatch. Remaining work is to implement full witness-opening proof semantics, soundness, zero-knowledge semantics, mask-PRF/domain-separation proof semantics, and promote the selected account-carried verifier into the executable facade.",
            "status": "partial",
        },
        {
            "api": "consume_slot(account_state, slot_id)",
            "current_surface": "ProductionAccountLifecycleManager plus ProductionLifecycleFileStore reserve slots, consume successful reserved signing, consume accepted verification, reject replay, snapshot active reservations, persist atomically, lock writers, and burn ambiguous reservations on recovery. The selected projection route also has ProductionReviewedFq12StateAssistedAccountLifecycleManager plus ProductionReviewedFq12StateAssistedLifecycleFileStore, which persist hidden_source_opening_commitment and restore_anchor_digest in the durable slot record, plus apply_selected_projection_consensus_opening_frame tying restore-anchor freshness, opening-frame verification, consumption, and spent-slot recording into one consensus-facing transition. ProductionSelectedProjectionChainStateFileStores wires this transition to separate lifecycle, spent-ledger, and trusted-anchor stores; ProductionSelectedProjectionTargetChainDbAdapter exposes the same transition as a target-chain database contract; ProductionSelectedProjectionTargetChainKvDb binds that contract to deterministic schema-tagged transactional KV keys; ProductionFileBackedTransactionalKv provides a durable local node-style KV engine with writer locks, temp-file fsync, and atomic rename; ProductionTargetChainDatabaseAdapterProfile records the chain-specific canonical database engine, trusted-anchor placement, KV schemas, identity binding, and external storage review digest required for a production claim; ProductionProfiledTargetChainKvDb only exposes the KV-backed target-chain database contract after the profile validates and refuses the in-memory/file-backed reference KV engines; ProductionCanonicalNodeKvStore plus ProductionCanonicalNodeKvAdapter force a concrete node DB integration to receive the validated profile on every committed read/write batch before it can be used as the profiled target-chain DB; production_canonical_node_store_conformance_check is the reusable conformance gate for candidate chain stores, exercising initialization, selected-projection commit, reopen persistence, anchor/spent-ledger reload, and replay rejection; ProductionLocalCanonicalNodeKvStore is the concrete local canonical JSON backend with profile-digest binding, writer lock, temp-file fsync, atomic rename, reload tests, profile-mismatch rejection, and conformance-check coverage; ProductionSqliteCanonicalNodeKvStore is a concrete embedded SQLite canonical-store adapter with schema metadata, profile-digest binding, FULL synchronous durability, WAL journaling, SQL transactions, reopen tests, profile-mismatch rejection, and conformance-check coverage; ProductionMockBlockchain wraps the local backend in a deterministic finalized-block simulator that initializes account state, finalizes selected-projection spends, computes finalized state roots, rejects replay, and reloads finalized state.",
            "present_in_fixture": "fn mark_consumed" in lib_source
            and has_library_verified_consumption
            and has_library_lifecycle_manager
            and has_library_lifecycle_file_store
            and capabilities["has_library_selected_projection_lifecycle_file_store"]
            and has_symbol(source, "production_verify"),
            "production_requirement": "Durable monotonic consume/reserve API with crash-safe and concurrent signing semantics.",
            "remaining_gap": "The library now has an in-memory lifecycle manager plus file-backed reference stores with reservation, reserved-sign-and-consume, accepted verify-and-consume, replay rejection, snapshot/restore validation, epoch-checked reservation tokens, monotonic snapshot sequences, restore-anchor rollback rejection, reorg revealed-signature burn records, public-key-hash/slot-id spent-ledger keys, spent-ledger duplicate rejection, schema-checked spent-ledger persistence, combined chain-state file-store transitions, atomic temp-write/rename persistence, schema checks, lock-file writer serialization, timestamped lock records, stale-lock breaking, recovery burn semantics, selected projection state-assisted lifecycle snapshots that preserve hidden_source_opening_commitment and restore_anchor_digest, a consensus-facing selected projection transition that requires restore-anchor freshness plus spent-ledger uniqueness before returning next state, reference chain-state database wiring with separate lifecycle, spent-ledger, and trusted-anchor writer locks, a target-chain database adapter contract with an executable in-memory conformance backend, deterministic transactional-KV binding for concrete node database integration, a durable file-backed transactional KV reference engine, a typed adapter-profile validator that rejects reference engines, restorable backup anchors, non-atomic transaction semantics, schema drift, and missing external-review digest, a profiled KV adapter that refuses to wrap the reference engines, a canonical-node store adapter boundary that passes the validated profile into chain DB reads/writes, a reusable canonical-store conformance check, a concrete local canonical JSON backend, a concrete SQLite canonical-store adapter, and a mock finalized blockchain simulator over the local backend. Remaining work is selecting the production chain's actual node database, running the conformance check against that adapter, and completing external storage review.",
            "status": "partial",
        },
        {
            "api": "canonical encoders/decoders",
            "current_surface": "library codecs for the legacy 254-byte proof path, the reviewed 250-byte account-carried exact target, the 252-byte/Fq12 fallback format, the archived strict 284-byte signature-carried hidden-source proof envelope, and the selected strict 308-byte projection-carrier envelope",
            "present_in_fixture": has_library_signature_codecs
            and has_library_reviewed_250b_codec
            and has_library_reviewed_252b_fq12_codec
            and has_library_reviewed_308b_fallback_projection_codec,
            "production_requirement": "Move canonical parsers/encoders into audited library modules with fuzz/property tests.",
            "remaining_gap": "Signature split, challenge/response/opening/proof codecs, reviewed 37-byte masked-response packing, strict 10-residue digest packing, deterministic 10x21 quotient map, 39-byte Fq12 fallback masked-response packing, strict 12-residue digest packing, deterministic 12x22 quotient map, challenge hash-frame construction, account-carried transcript challenge binding, 252-byte/Fq12 checked-core verification, public local-fiber consistency, public pad-opening consistency, canonical pad-opening statement hashing, canonical hidden-source statement hashing, the executable hidden-source witness relation, 32-byte opaque proof carrier/parser, archived strict 284-byte signature-carried proof envelope, typed canonical hidden-source proof transcript hashing over the recomputed statement digest, public signature-carried proof context building, canonical fail-closed deployment blocker report hashing, typed fail-closed proof-semantics gate hashing, hashable acceptance-contract obligations including the explicit missing mask-PRF domain-separation obligation, sanitized transcript-aware hidden-source proof callback boundary, selected strict 308-byte projection-carrier envelope, and a fail-closed deployment verifier with explicit projection-carrier proof-semantics blocker now live in the library; broader fuzz/property coverage, exact fallback certificate semantics, leakage semantics, and facade promotion remain open.",
            "status": "partial",
        },
        {
            "api": "constant_time_signing",
            "current_surface": "constant-time review remains open; sanitized callback-error wrappers and public-report KATs now exist in the library",
            "present_in_fixture": capabilities["has_library_sanitized_callback_errors"],
            "production_requirement": "No secret-dependent rejection/timing leakage in signing or account precomputation.",
            "remaining_gap": "Sanitized production wrappers now collapse callback-originated error strings to fixed public reasons, but constant-time signing, sampler/keygen timing, retry shape, and the selected 308-byte/Fq12 projection-carrier verifier semantics remain open.",
            "status": "open",
        },
    ]
    return rows


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Production Reference Gap Report",
        "",
        "This report tracks the gap between the current research fixture and a",
        "production-shaped account-carried reference implementation.  It is not a",
        "security proof; it is an implementation readiness checklist generated from",
        "the current source and review artifacts.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- target transaction bytes: `{payload['target']['transaction_bytes']}`",
        f"- target account bytes: `{payload['target']['account_link_bytes']}`",
        f"- target total online plus account bytes: `{payload['target']['total_online_plus_account_bytes']}`",
        f"- current timing signature bytes: `{payload['timing']['signature_bytes']}`",
        f"- production facade account commitment bytes: `{payload['production_facade']['account_commitment_bytes']}`",
        f"- account commitment byte source: `{payload['production_facade']['account_commitment_byte_source']}`",
        f"- production facade executable signature bytes: `{payload['production_facade']['executable_signature_bytes']}`",
        f"- executable byte source: `{payload['production_facade']['executable_signature_byte_source']}`",
        f"- reviewed target transaction bytes: `{payload['production_facade']['target_transaction_signature_bytes']}`",
        f"- reviewed target byte source: `{payload['production_facade']['target_transaction_signature_byte_source']}`",
        f"- cost model target matches reviewed target: `{payload['production_facade']['cost_model_target_matches_reviewed_target']}`",
        f"- cost model account bytes match library: `{payload['production_facade']['cost_model_account_bytes_match_library']}`",
        f"- facade matches 250-byte target: `{payload['production_facade']['executable_matches_target']}`",
        f"- account commitment layout extracted to library: `{payload['production_facade']['has_library_account_commitment']}`",
        f"- account-state assembly extracted to library: `{payload['production_facade']['has_library_account_state_assembly']}`",
        f"- account/signature state boundary extracted to library: `{payload['production_facade']['has_library_state_boundary']}`",
        f"- account-state checked export extracted to library: `{payload['production_facade']['has_library_account_state_export']}`",
        f"- verified-slot completion transition extracted to library: `{payload['production_facade']['has_library_verified_consumption']}`",
        f"- lifecycle manager extracted to library: `{payload['production_facade']['has_library_lifecycle_manager']}`",
        f"- lifecycle file store extracted to library: `{payload['production_facade']['has_library_lifecycle_file_store']}`",
        f"- public-key canonical validation extracted to library: `{payload['production_facade']['has_library_public_key_validation']}`",
        f"- production keypair wrapper extracted to library: `{payload['production_facade']['has_library_keypair_boundary']}`",
        f"- verifier-facing signature codecs extracted to library: `{payload['production_facade']['has_library_signature_codecs']}`",
        f"- reviewed 250-byte account-carried codec extracted to library: `{payload['production_facade']['has_library_reviewed_250b_codec']}`",
        f"- reviewed 252-byte/Fq12 fallback codec extracted to library: `{payload['production_facade']['has_library_reviewed_252b_fq12_codec']}`",
        f"- reviewed 308-byte/Fq12 projection-carrier codec extracted to library: `{payload['production_facade']['has_library_reviewed_308b_fallback_projection_codec']}`",
        f"- reviewed 252-byte/Fq12 sigma equation extracted to library: `{payload['production_facade']['has_library_reviewed_252b_fq12_sigma_equation']}`",
        f"- reviewed 252-byte/Fq12 hidden-source statement/witness relation extracted to library: `{payload['production_facade']['has_library_reviewed_252b_fq12_pad_seed_witness_relation']}`",
        f"- reviewed 252-byte/Fq12 fail-closed deployment verifier extracted to library: `{payload['production_facade']['has_library_reviewed_252b_fq12_fail_closed_deployment_verifier']}`",
        f"- challenge hash-frame extracted to library: `{payload['production_facade']['has_library_challenge_frame']}`",
        f"- sign/verify binding boundary extracted to library: `{payload['production_facade']['has_library_sign_verify_boundary']}`",
        f"- sanitized callback-error wrappers extracted to library: `{payload['production_facade']['has_library_sanitized_callback_errors']}`",
        f"- fail-closed deployment preflight extracted to library: `{payload['production_facade']['has_library_deployment_preflight']}`",
        f"- separate production-reference crate boundary present: `{payload['production_reference_crate']['present']}`",
        f"- production-reference facade timing artifact present: `{payload['production_reference_crate_timing']['present']}`",
        f"- deployment preflight ready: `{payload['deployment_preflight']['deployment_ready']}`",
        f"- deployment preflight blockers: `{len(payload['deployment_preflight']['blockers'])}`",
        "",
        "## API Gaps",
        "",
        "| API | status | current surface | remaining gap |",
        "|---|---|---|---|",
    ]
    for row in payload["api_rows"]:
        present = "present" if row["present_in_fixture"] else "missing"
        lines.append(
            f"| {row['api']} | {row['status']} ({present}) | "
            f"{row['current_surface']} | {row['remaining_gap']} |"
        )
    crate = payload["production_reference_crate"]
    lines.extend(
        [
            "",
            "## Production Reference Crate Boundary",
            "",
            f"- present: `{crate['present']}`",
            f"- manifest: `{crate['manifest']}`",
            f"- library: `{crate['library']}`",
            f"- facade timing binary: `{crate['facade_timing_binary']}`",
            f"- path dependency on parameter-search library: `{crate['path_dependency_on_parameter_search']}`",
            f"- reexports production-reference surface: `{crate['reexports_production_reference_surface']}`",
            f"- deployment facade helpers: `{crate['deployment_facade_helpers']}`",
            f"- deployment facade tests: `{crate['deployment_facade_tests']}`",
            f"- deployment facade timing binary present: `{crate['deployment_facade_timing_binary_present']}`",
            f"- fail-closed preflight tests: `{crate['fail_closed_preflight_tests']}`",
            f"- sanitized callback profile test: `{crate['sanitized_callback_profile_test']}`",
            f"- remaining gap: {crate['remaining_gap']}",
        ]
    )
    timing_artifact = payload["production_reference_crate_timing"]
    lines.extend(
        [
            "",
            "## Production Reference Crate Facade Timing",
            "",
            f"- present: `{timing_artifact['present']}`",
            f"- artifact: `{timing_artifact['artifact']}`",
            f"- iterations: `{timing_artifact['iterations']}`",
            f"- executable signature bytes: `{timing_artifact['signature_bytes']}`",
            f"- reviewed target signature bytes: `{timing_artifact['reviewed_target_signature_bytes']}`",
            f"- caveat: {timing_artifact['caveat']}",
            "",
            "| operation | avg us | avg ns | ok |",
            "|---|---:|---:|:---:|",
        ]
    )
    for row in timing_artifact["timings"]:
        lines.append(
            f"| {row['operation']} | {row['avg_us']:.3f} | {row['avg_ns']} | "
            f"{'yes' if row['accepted_or_ok'] else 'no'} |"
        )
    lines.extend(["", "## Deployment Preflight", ""])
    preflight = payload["deployment_preflight"]
    certificate = preflight.get("certificate_export", {})
    certificate_digests = certificate.get("digests", {})
    lines.extend(
        [
            f"- present in library: `{preflight['present_in_library']}`",
            f"- canonical preflight report present: `{preflight['canonical_report_present']}`",
            f"- canonical preflight blocker rows present: `{preflight['canonical_blocker_rows_present']}`",
            f"- canonical preflight blocker report present: `{preflight['canonical_blocker_report_present']}`",
            f"- canonical preflight verdict present: `{preflight['canonical_verdict_present']}`",
            f"- preflight verdict binds blocker report: `{preflight['verdict_binds_blocker_report']}`",
            f"- deployment-ready assertion present: `{preflight['deployment_ready_assertion_present']}`",
            f"- canonical deployment-ready assertion present: `{preflight['canonical_deployment_ready_assertion_present']}`",
            f"- canonical preflight certificate present: `{preflight['canonical_preflight_certificate_present']}`",
            f"- preflight certificate export status: `{certificate.get('export_status')}`",
            f"- preflight certificate digest: `{certificate_digests.get('certificate')}`",
            f"- preflight report digest: `{certificate_digests.get('preflight_report')}`",
            f"- preflight blocker-report digest: `{certificate_digests.get('blocker_report')}`",
            f"- preflight verdict digest: `{certificate_digests.get('verdict')}`",
            f"- preflight ready-assertion digest: `{certificate_digests.get('ready_assertion')}`",
            f"- callback boundaries ok: `{preflight['callback_boundaries_ok']}`",
            f"- executable signature bytes: `{preflight['executable_signature_bytes']}`",
            f"- reviewed target signature bytes: `{preflight['reviewed_target_signature_bytes']}`",
            f"- reviewed 252-byte/Fq12 fallback signature bytes: `{preflight['reviewed_252b_fq12_fallback_signature_bytes']}`",
            f"- reviewed 252-byte/Fq12 hidden-source proof carrier bytes: `{preflight['reviewed_252b_fq12_hidden_source_proof_bytes']}`",
            f"- reviewed 252-byte/Fq12 with hidden-source proof carrier bytes: `{preflight['reviewed_252b_fq12_with_hidden_source_proof_bytes']}`",
            f"- reviewed 252-byte/Fq12 fallback projection carrier bytes: `{preflight['reviewed_252b_fq12_fallback_projection_carrier_bytes']}`",
            f"- reviewed 252-byte/Fq12 with fallback projection carrier bytes: `{preflight['reviewed_252b_fq12_with_fallback_projection_bytes']}`",
            f"- reviewed account-link bytes: `{preflight['reviewed_account_link_bytes']}`",
            f"- reviewed target codec present: `{preflight['reviewed_target_codec_present']}`",
            f"- reviewed 252-byte/Fq12 fallback codec present: `{preflight['reviewed_252b_fq12_fallback_codec_present']}`",
            f"- reviewed 252-byte/Fq12 fallback projection codec present: `{preflight['reviewed_252b_fq12_fallback_projection_codec_present']}`",
            f"- reviewed 252-byte/Fq12 sigma equation present: `{preflight['reviewed_252b_fq12_sigma_equation_present']}`",
            f"- reviewed 252-byte/Fq12 hidden-source statement/witness relation present: `{preflight['reviewed_252b_fq12_pad_seed_witness_relation_present']}`",
            f"- reviewed 252-byte/Fq12 hidden-source proof callback present: `{preflight['reviewed_252b_fq12_hidden_source_proof_callback_present']}`",
            f"- reviewed 252-byte/Fq12 transcript-aware proof callback present: `{preflight['reviewed_252b_fq12_hidden_source_proof_transcript_callback_present']}`",
            f"- reviewed 252-byte/Fq12 hidden-source readiness report present: `{preflight['reviewed_252b_fq12_hidden_source_readiness_report_present']}`",
            f"- reviewed 252-byte/Fq12 hidden-source readiness verdict present: `{preflight['reviewed_252b_fq12_hidden_source_readiness_verdict_present']}`",
            f"- reviewed 252-byte/Fq12 hidden-source deployability assertion present: `{preflight['reviewed_252b_fq12_hidden_source_deployability_assertion_present']}`",
            f"- reviewed 252-byte/Fq12 fail-closed deployment verifier present: `{preflight['reviewed_252b_fq12_fail_closed_deployment_verifier_present']}`",
            f"- reviewed target account binding present: `{preflight['reviewed_target_account_binding_present']}`",
            f"- reviewed target challenge binding present: `{preflight['reviewed_target_challenge_binding_present']}`",
            f"- executable matches reviewed target: `{preflight['executable_matches_reviewed_target']}`",
            f"- deployment ready: `{preflight['deployment_ready']}`",
            "",
            "Blockers:",
        ]
    )
    for blocker in preflight["blockers"]:
        lines.append(f"- {blocker}")
    if certificate.get("blockers"):
        lines.extend(
            [
                "",
                "Canonical Blocker Rows:",
                "",
                "| index | blocker | digest |",
                "|---:|---|---|",
            ]
        )
        for row in certificate["blockers"]:
            lines.append(
                f"| {row.get('index')} | {row.get('blocker')} | `{row.get('digest')}` |"
            )
    lines.extend(
        [
            "",
            "## Next Cut",
            "",
            "The next implementation cut should promote one reviewed transaction",
            "format to the executable facade. The exact 250-byte format remains",
            "blocked on a pinned 10-row challenge action with multi-query margin;",
            "the 252-byte/Fq12 fallback now has a canonical codec, strict 12-residue",
            "quotient digest map, transcript-binding checker, and sigma-equation",
            "checker, public local-fiber consistency, public pad-opening consistency,",
            "canonical pad-opening statement hashing, a canonical hidden-source",
            "statement digest, an executable witness relation tying the account",
            "witness syndrome and 32-byte pad seed to that public statement, a",
            "32-byte opaque proof carrier/parser, a sanitized hidden-source proof",
            "callback boundary, a sanitized transcript-aware proof callback",
            "boundary, an archived strict 284-byte signature-carried proof envelope,",
            "a selected strict 308-byte projection-carrier envelope,",
            "a selected state-assisted opening tuple commitment model with the direct expanded-opening slot path archived as non-deployment,",
            "a selected-path tuple verifier composing slot opening, projection opening, pad seed, hidden-material lanes, and carrier check/transcript binding,",
            "a typed canonical hidden-source proof transcript object over the recomputed statement digest,",
            "a public context builder returning statement/proof/transcript for that envelope,",
            "a fail-closed deployment blocker report carrying that context, blocker reason, and canonical digest,",
            "a typed proof-semantics gate pinned to the current partial executable-relation profile with its own canonical digest,",
            "a hashable acceptance contract enumerating the obligations needed before acceptance can turn on,",
            "and a fail-closed deployment verifier.",
            "The next cut is to give the carrier bytes real compact-proof",
            "semantics and then flip the deployment verifier from fail-closed to",
            "accepting for the selected account-carried path. The",
            "current library cut is useful because it gives the 68-byte account-link",
            "layout and account-state transition boundary one source of truth without",
            "claiming deployment readiness.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    source = SOURCE.read_text()
    lib_source = PRODUCTION_REFERENCE_LIB.read_text()
    cost_model = load_json(COST_MODEL) or {}
    timing_report = load_json(TIMING_REPORT) or {}
    facade_timing_report = load_json(PRODUCTION_REFERENCE_FACADE_TIMING_REPORT) or {}
    frontier = (cost_model.get("pareto_frontier") or [{}])[0]
    timing = timing_report.get("timing_report", {})
    facade_timing = facade_timing_report.get("timing_report", {})
    target_transaction_bytes = frontier.get("transaction_bytes", 250)
    target_account_bytes = frontier.get("account_link_bytes", 68)
    account_commitment_bytes = const_usize(lib_source, "REVIEWED_ACCOUNT_LINK_BYTES", 68)
    reviewed_target_transaction_bytes = const_usize(
        lib_source, "REVIEWED_TARGET_TRANSACTION_SIGNATURE_BYTES", target_transaction_bytes
    )
    executable_signature_bytes = const_usize(
        lib_source, "LINEAR_SIGMA_TARGET_TRANSACTION_SIGNATURE_BYTES", 254
    )
    executable_matches_reviewed_target = (
        executable_signature_bytes == reviewed_target_transaction_bytes
    )
    capabilities = library_capabilities(lib_source)
    production_crate = production_reference_crate_status()
    preflight_certificate = export_preflight_certificate()
    fallback_deployment_preflight_blockers = []
    if not capabilities["has_library_deployment_preflight"]:
        fallback_deployment_preflight_blockers.append(
            "deployment preflight API is missing from production_reference.rs"
        )
    if not executable_matches_reviewed_target:
        fallback_deployment_preflight_blockers.append(
            "selected 308-byte/Fq12 active-lane projection-carrier deployment verifier is fail-closed until active-lane preflight promotion and hidden-source semantics are implemented"
        )
    fallback_deployment_preflight_blockers.extend(
        [
            "constant-time signer, sampler, keygen, and verifier audit remains open",
            "target-specific chain database binding, trusted restore-anchor placement review, and external storage review remain open",
            "external security review remains open",
        ]
    )
    exported_blocker_rows = preflight_certificate.get("blockers") or []
    exported_blockers = [
        str(row.get("blocker"))
        for row in exported_blocker_rows
        if isinstance(row, dict) and row.get("blocker")
    ]
    deployment_preflight_blockers = (
        exported_blockers
        if preflight_certificate.get("export_status") == "pass" and exported_blockers
        else fallback_deployment_preflight_blockers
    )
    deployment_preflight_ready = (
        preflight_certificate.get("deployment_ready") is True
        if preflight_certificate.get("export_status") == "pass"
        else len(deployment_preflight_blockers) == 0
    )
    payload = {
        "schema": "Descender.production_reference_gap_report.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source": str(SOURCE.relative_to(ROOT)),
        "library_source": str(PRODUCTION_REFERENCE_LIB.relative_to(ROOT)),
        "target": {
            "transaction_bytes": target_transaction_bytes,
            "account_link_bytes": target_account_bytes,
            "total_online_plus_account_bytes": frontier.get(
                "total_online_plus_account_bytes", 318
            ),
        },
        "production_facade": {
            "account_commitment_bytes": account_commitment_bytes,
            "account_commitment_byte_source": str(
                PRODUCTION_REFERENCE_LIB.relative_to(ROOT)
            ),
            "executable_signature_bytes": executable_signature_bytes,
            "executable_signature_byte_source": str(
                PRODUCTION_REFERENCE_LIB.relative_to(ROOT)
            ),
            "target_transaction_signature_bytes": reviewed_target_transaction_bytes,
            "target_transaction_signature_byte_source": str(
                PRODUCTION_REFERENCE_LIB.relative_to(ROOT)
            ),
            "cost_model_target_transaction_bytes": target_transaction_bytes,
            "cost_model_target_matches_reviewed_target": target_transaction_bytes
            == reviewed_target_transaction_bytes,
            "cost_model_account_link_bytes": target_account_bytes,
            "cost_model_account_bytes_match_library": target_account_bytes
            == account_commitment_bytes,
            "executable_matches_target": executable_matches_reviewed_target,
            **capabilities,
            "has_production_keygen": has_symbol(source, "production_keygen"),
            "has_production_account_commit": "fn account_state_from_slot_seeds"
            in lib_source,
            "has_production_sign": has_symbol(source, "production_sign"),
            "has_production_verify": has_symbol(source, "production_verify"),
        },
        "production_reference_crate": production_crate,
        "production_reference_crate_timing": {
            "present": bool(facade_timing_report),
            "artifact": str(PRODUCTION_REFERENCE_FACADE_TIMING_REPORT.relative_to(ROOT)),
            "schema": facade_timing_report.get("schema"),
            "iterations": facade_timing.get("iterations"),
            "signature_bytes": facade_timing.get("signature_bytes"),
            "reviewed_target_signature_bytes": facade_timing.get(
                "reviewed_target_signature_bytes"
            ),
            "account_link_bytes": facade_timing.get("account_link_bytes"),
            "caveat": facade_timing.get(
                "caveat",
                "Facade timing artifact is absent or malformed.",
            ),
            "timings": facade_timing.get("timings", []),
        },
        "deployment_preflight": {
            "present_in_library": capabilities["has_library_deployment_preflight"],
            "canonical_report_present": capabilities["has_library_deployment_preflight"],
            "canonical_blocker_rows_present": capabilities[
                "has_library_deployment_preflight"
            ],
            "canonical_blocker_report_present": capabilities[
                "has_library_deployment_preflight"
            ],
            "canonical_verdict_present": capabilities["has_library_deployment_preflight"],
            "verdict_binds_blocker_report": capabilities[
                "has_library_deployment_preflight"
            ],
            "deployment_ready_assertion_present": capabilities[
                "has_library_deployment_preflight"
            ],
            "canonical_deployment_ready_assertion_present": capabilities[
                "has_library_deployment_preflight"
            ],
            "canonical_preflight_certificate_present": capabilities[
                "has_library_deployment_preflight"
            ],
            "certificate_export": preflight_certificate,
            "callback_boundaries_ok": capabilities["has_library_sanitized_callback_errors"],
            "executable_signature_bytes": executable_signature_bytes,
            "reviewed_target_signature_bytes": reviewed_target_transaction_bytes,
            "reviewed_252b_fq12_fallback_signature_bytes": const_usize(
                lib_source,
                "REVIEWED_FQ12_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES",
                252,
            ),
            "reviewed_252b_fq12_hidden_source_proof_bytes": const_usize(
                lib_source,
                "REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_BYTES",
                32,
            ),
            "reviewed_252b_fq12_with_hidden_source_proof_bytes": const_usize(
                lib_source,
                "REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_HIDDEN_SOURCE_PROOF_BYTES",
                284,
            ),
            "reviewed_252b_fq12_fallback_projection_carrier_bytes": const_usize(
                lib_source,
                "HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES",
                56,
            ),
            "reviewed_252b_fq12_with_fallback_projection_bytes": const_usize(
                lib_source,
                "REVIEWED_FQ12_ACCOUNT_CARRIED_WITH_FALLBACK_PROJECTION_BYTES",
                308,
            ),
            "reviewed_account_link_bytes": account_commitment_bytes,
            "reviewed_target_codec_present": capabilities["has_library_reviewed_250b_codec"],
            "reviewed_252b_fq12_fallback_codec_present": capabilities[
                "has_library_reviewed_252b_fq12_codec"
            ],
            "reviewed_252b_fq12_fallback_projection_codec_present": capabilities[
                "has_library_reviewed_308b_fallback_projection_codec"
            ],
            "reviewed_252b_fq12_sigma_equation_present": capabilities[
                "has_library_reviewed_252b_fq12_sigma_equation"
            ],
            "reviewed_252b_fq12_pad_seed_witness_relation_present": capabilities[
                "has_library_reviewed_252b_fq12_pad_seed_witness_relation"
            ],
            "reviewed_252b_fq12_hidden_source_proof_callback_present": capabilities[
                "has_library_reviewed_252b_fq12_pad_seed_witness_relation"
            ],
            "reviewed_252b_fq12_hidden_source_proof_transcript_callback_present": capabilities[
                "has_library_reviewed_252b_fq12_pad_seed_witness_relation"
            ],
            "reviewed_252b_fq12_hidden_source_readiness_report_present": capabilities[
                "has_library_reviewed_252b_fq12_pad_seed_witness_relation"
            ],
            "reviewed_252b_fq12_hidden_source_readiness_verdict_present": capabilities[
                "has_library_reviewed_252b_fq12_pad_seed_witness_relation"
            ],
            "reviewed_252b_fq12_hidden_source_deployability_assertion_present": capabilities[
                "has_library_reviewed_252b_fq12_pad_seed_witness_relation"
            ],
            "reviewed_252b_fq12_fail_closed_deployment_verifier_present": capabilities[
                "has_library_reviewed_252b_fq12_fail_closed_deployment_verifier"
            ],
            "reviewed_target_account_binding_present": capabilities[
                "has_library_reviewed_250b_codec"
            ],
            "reviewed_target_challenge_binding_present": capabilities[
                "has_library_reviewed_250b_codec"
            ],
            "executable_matches_reviewed_target": executable_matches_reviewed_target,
            "deployment_ready": deployment_preflight_ready,
            "blockers": deployment_preflight_blockers,
        },
        "timing": {
            "artifact": str(TIMING_REPORT.relative_to(ROOT)),
            "signature_bytes": timing.get("signature_bytes"),
            "caveat": timing.get("caveat"),
        },
        "api_rows": api_rows(source, lib_source, capabilities),
    }
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
