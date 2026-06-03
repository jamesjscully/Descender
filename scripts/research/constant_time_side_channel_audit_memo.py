#!/usr/bin/env python3
"""Generate the constant-time and side-channel audit memo.

This is a scoping artifact, not a side-channel proof.  It classifies the current
production-reference surfaces into public-state code paths, secret-side callback
boundaries, and storage/logging channels that must stay free of hidden witness
material.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"
STATEFUL_BIN = ROOT / "tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs"
PRODUCTION_REFERENCE_GAP_REPORT = (
    READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
)
PRODUCTION_LIFECYCLE_HARDENING_MEMO = (
    READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.json"
)
BLOCKCHAIN_DEPLOYMENT_INTEGRATION_MEMO = (
    READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json"
)
DEFAULT_JSON = READINESS_DIR / "constant_time_side_channel_audit_memo_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "constant_time_side_channel_audit_memo_n32_q12289.md"


def load_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def contains(source: str, needle: str) -> bool:
    return needle in source


def detected_symbols(source: str, symbols: list[str]) -> dict[str, bool]:
    return {symbol: contains(source, symbol) for symbol in symbols}


def build_payload() -> dict[str, Any]:
    production_source = PRODUCTION_REFERENCE.read_text()
    bin_source = STATEFUL_BIN.read_text()
    gap = load_json(PRODUCTION_REFERENCE_GAP_REPORT) or {}
    lifecycle = load_json(PRODUCTION_LIFECYCLE_HARDENING_MEMO) or {}
    deployment = load_json(BLOCKCHAIN_DEPLOYMENT_INTEGRATION_MEMO) or {}
    facade = gap.get("production_facade", {})

    audited_surfaces = [
        {
            "surface": "public_only_parsing_and_decoding",
            "status": "public_data_only_review_needed",
            "claim": (
                "Current parsing and byte-codec helpers are intended to operate "
                "only on public key, public account state, signature bytes, and "
                "message/transcript bytes; their rejection timing is not a hidden "
                "witness channel under that classification."
            ),
            "symbols": [
                "ProductionPublicKey::new",
                "validate_public_key_bytes",
                "decode_production_signature",
                "decode_challenge",
                "decode_responses",
                "decode_openings",
                "decode_linear_sigma_target_proof",
                "verify_account_binding",
                "verify_with_account_binding",
            ],
            "detected_symbols": detected_symbols(
                production_source,
                [
                    "impl ProductionPublicKey",
                    "validate_public_key_bytes",
                    "decode_production_signature",
                    "decode_challenge",
                    "decode_responses",
                    "decode_openings",
                    "decode_linear_sigma_target_proof",
                    "verify_account_binding",
                    "verify_with_account_binding",
                ],
            ),
        },
        {
            "surface": "secret_side_signing_boundary",
            "status": "sanitized_callback_error_policy_present_ct_open",
            "claim": (
                "The library boundary validates the selected public account slot "
                "before and after invoking the signer callback, but it delegates "
                "secret-key use, sampling, and proof construction to the callback. "
                "Production-facing sanitized wrappers collapse callback-originated "
                "error strings to fixed public reasons, but no constant-time "
                "property is established for that callback."
            ),
            "symbols": [
                "sign_with_account_binding",
                "sign_with_account_binding_sanitized",
                "sign_with_secret_key",
                "sign_with_secret_key_sanitized",
                "ProductionSecretKey",
                "reference_sign_with_proof",
                "production_sign",
            ],
            "detected_symbols": {
                **detected_symbols(
                    production_source,
                    [
                        "sign_with_account_binding",
                        "sign_with_account_binding_sanitized",
                        "sign_with_secret_key",
                        "sign_with_secret_key_sanitized",
                        "ProductionSecretKey",
                    ],
                ),
                **detected_symbols(
                    bin_source,
                    ["reference_sign_with_proof", "production_sign"],
                ),
            },
        },
        {
            "surface": "sampler_and_keygen_secret_material",
            "status": "open",
            "claim": (
                "The current executable still uses the research keygen/sampler "
                "path.  Matrix/pad/sampler assumptions are documented elsewhere, "
                "but production entropy separation, fixed failure behavior, and "
                "secret-independent sampling have not been audited in code."
            ),
            "symbols": [
                "production_keygen",
                "reference_keygen",
                "sample_conditioned_quotient_fiber_representative",
                "pad_sampler_certificate",
            ],
            "detected_symbols": detected_symbols(
                bin_source,
                [
                    "production_keygen",
                    "reference_keygen",
                    "sample_conditioned_quotient_fiber_representative",
                    "pad_sampler_certificate",
                ],
            ),
        },
        {
            "surface": "account_lifecycle_state",
            "status": "public_state_serialization_no_hidden_witness_material_intended",
            "claim": (
                "Lifecycle snapshots, reservation records, restore anchors, "
                "reorg-burn records, and spent-ledger entries are intended to "
                "contain public account commitments, public-key hashes, slot ids, "
                "sequence numbers, and public source labels only."
            ),
            "symbols": [
                "ProductionLifecycleSnapshot",
                "ProductionLifecycleRestoreAnchor",
                "ProductionReorgBurnRecord",
                "ProductionSpentSlotLedger",
                "ProductionChainStateFileStores",
            ],
            "detected_symbols": detected_symbols(
                production_source,
                [
                    "ProductionLifecycleSnapshot",
                    "ProductionLifecycleRestoreAnchor",
                    "ProductionReorgBurnRecord",
                    "ProductionSpentSlotLedger",
                    "ProductionChainStateFileStores",
                ],
            ),
        },
        {
            "surface": "file_stores_and_locks",
            "status": "public_state_only_reference_backend",
            "claim": (
                "The reference file stores persist public account commitment "
                "records and spent-slot records.  They do not intentionally store "
                "secret keys, hidden witnesses, pad material, or randomizer "
                "preimages, but this needs negative serialization tests."
            ),
            "symbols": [
                "ProductionLifecycleFileStore",
                "ProductionLifecycleLockRecord",
                "ProductionSpentSlotLedgerFileStore",
                "PRODUCTION_LIFECYCLE_STORE_SCHEMA",
                "PRODUCTION_SPENT_SLOT_LEDGER_SCHEMA",
            ],
            "detected_symbols": detected_symbols(
                production_source,
                [
                    "ProductionLifecycleFileStore",
                    "ProductionLifecycleLockRecord",
                    "ProductionSpentSlotLedgerFileStore",
                    "PRODUCTION_LIFECYCLE_STORE_SCHEMA",
                    "PRODUCTION_SPENT_SLOT_LEDGER_SCHEMA",
                ],
            ),
        },
        {
            "surface": "logging_errors_and_reports",
            "status": "public_metadata_only_but_audit_needed",
            "claim": (
                "Current rejection reports expose expected byte lengths, optional "
                "slot id, claimed challenge bytes, and reason strings.  This is "
                "acceptable only if these fields remain derived from public "
                "signature/account data and never include hidden witness material."
            ),
            "symbols": [
                "ProductionVerificationRejection",
                "ProductionAccountBindingError",
                "ProductionLifecycleError",
                "rejection_report_fields",
                "ProductionRejectionReportFields",
                "verify_with_account_binding_sanitized",
                "sign_reserved_and_consume_sanitized",
                "verify_and_consume_sanitized",
                "verify_consume_and_record_spent_sanitized",
            ],
            "detected_symbols": detected_symbols(
                production_source,
                [
                    "ProductionVerificationRejection",
                    "ProductionAccountBindingError",
                    "ProductionLifecycleError",
                    "rejection_report_fields",
                    "ProductionRejectionReportFields",
                    "verify_with_account_binding_sanitized",
                    "sign_reserved_and_consume_sanitized",
                    "verify_and_consume_sanitized",
                    "verify_consume_and_record_spent_sanitized",
                ],
            ),
        },
    ]

    forbidden_secret_material = [
        "reference secret key",
        "hidden account witness",
        "randomizer preimage",
        "active pad material",
        "sampler randomness",
        "entropy seed material",
        "conditioned sampler rejection/failure internals",
    ]
    public_material = [
        "public key bytes",
        "public key SHA-256 spent-ledger key",
        "signature bytes",
        "slot id",
        "account witness commitment digest",
        "account randomizer commitment digest",
        "response-link code",
        "spent-ledger records",
        "snapshot sequence",
        "reservation epoch",
        "lock pid/timestamp/store path",
    ]
    serialization_kats_present = all(
        contains(production_source, symbol)
        for symbol in [
            "lifecycle_file_store_serializes_only_public_account_state",
            "spent_slot_ledger_file_store_serializes_only_public_spent_records",
        ]
    )
    generated_rejection_report_kats_present = contains(
        production_source,
        "generated_rejection_reports_expose_only_public_metadata",
    )
    sanitized_callback_wrappers_present = all(
        contains(production_source, symbol)
        for symbol in [
            "PRODUCTION_SIGNER_CALLBACK_FAILURE_REASON",
            "PRODUCTION_VERIFIER_CALLBACK_FAILURE_REASON",
            "sign_with_secret_key_sanitized",
            "verify_with_account_binding_sanitized",
            "sign_reserved_and_consume_sanitized",
            "verify_and_consume_sanitized",
            "verify_consume_and_record_spent_sanitized",
            "verify_consume_record_and_save_sanitized",
            "sanitized_callback_wrappers_do_not_echo_secret_errors",
            "sanitized_callback_wrappers_have_bounded_invocation_shape",
            "PRODUCTION_CALLBACK_BOUNDARIES",
            "ProductionCallbackErrorPolicy::SanitizedFixedPublicErrors",
            "require_deployment_callback_boundaries",
            "PRODUCTION_DEPLOYMENT_REQUIRED_CALLBACK_BOUNDARIES",
            "verify_default_deployment_callback_boundaries",
            "ProductionDeploymentPreflight",
            "production_deployment_preflight",
            "deployment_preflight_fails_closed_until_selected_verifier_and_reviews_land",
            "callback_boundary_classification_marks_only_sanitized_paths_deployment_facing",
            "deployment_callback_boundary_gate_rejects_diagnostic_paths",
            "default_deployment_callback_profile_is_complete_and_sanitized",
        ]
    )
    open_blockers = [
        "The selected 308-byte/Fq12 projection-carrier verifier is fail-closed until hidden-source proof semantics are implemented; the current executable facade still wraps the legacy 254-byte proof callback.",
        "The secret-side signer callback has no constant-time guarantee and still calls the research signing path.",
        "Production keygen, sampler, pad expansion, and entropy separation have no dudect/ctgrind-style leakage harness.",
        "Sanitized callback wrappers now hide callback-originated error strings, preserve account/ledger state on callback failure, and have bounded wrapper-level callback invocation shape; secret-side callback internals are not yet proven constant-time or independent of hidden witness state.",
        "Research wrappers still preserve raw callback diagnostics and are explicitly classified as non-deployment diagnostic paths; a deployment boundary gate rejects them.",
        "Compiler, target, allocator, and filesystem-specific side-channel behavior have not been audited.",
    ]
    required_instrumentation = [
        {
            "name": "public_state_serialization_kats",
            "status": (
                "initial_reference_kats_present"
                if serialization_kats_present
                else "open"
            ),
            "requirement": (
                "Generate KATs for lifecycle snapshots, spent ledgers, lock "
                "records, rejection reports, and account commitments, and assert "
                "that no forbidden secret-material labels or raw test secrets "
                "appear in serialized bytes."
            ),
        },
        {
            "name": "negative_logging_tests",
            "status": (
                "generated_public_report_and_callback_sanitizer_kats_present"
                if generated_rejection_report_kats_present
                and sanitized_callback_wrappers_present
                else "open"
            ),
            "requirement": (
                "Exercise every rejection/error path with instrumented test "
                "secrets and assert logs/reports expose only public metadata."
            ),
        },
        {
            "name": "signer_sampler_leakage_tests",
            "status": "open",
            "requirement": (
                "Add dudect/ctgrind-style tests or an equivalent reviewed harness "
                "for signer, sampler, pad expansion, and keygen paths."
            ),
        },
        {
            "name": "fixed_failure_behavior_tests",
            "status": (
                "sanitized_callback_failure_and_wrapper_retry_shape_kats_present_callback_timing_open"
                if sanitized_callback_wrappers_present
                else "open"
            ),
            "requirement": (
                "Check that secret-side failures have bounded retry counts, "
                "constant-shape return behavior, and no hidden-state-dependent "
                "reason strings."
            ),
        },
        {
            "name": "constant_time_code_review_checklist",
            "status": "open",
            "requirement": (
                "Review secret-side arithmetic, memory access, branching, "
                "hash/PRF framing, compiler target, and dependency behavior "
                "before deployment claims."
            ),
        },
    ]

    return {
        "schema": "Descender.constant_time_side_channel_audit_memo.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source_artifacts": [
            rel(PRODUCTION_REFERENCE),
            rel(STATEFUL_BIN),
            rel(PRODUCTION_REFERENCE_GAP_REPORT),
            rel(PRODUCTION_LIFECYCLE_HARDENING_MEMO),
            rel(BLOCKCHAIN_DEPLOYMENT_INTEGRATION_MEMO),
        ],
        "current_claim": "not_constant_time_review_audit_memo_only",
        "target": {
            "profile": "Descender.Signature.q12289.local_delta.stateful.v1",
            "n": 32,
            "q": 12289,
            "reviewed_transaction_target_bytes": 250,
            "reviewed_account_link_bytes": 68,
            "executable_signature_bytes": facade.get("executable_signature_bytes"),
            "executable_matches_target": facade.get("executable_matches_target"),
        },
        "artifact_context": {
            "production_gap_status": gap.get("schema"),
            "lifecycle_memo_status": lifecycle.get("schema"),
            "deployment_memo_status": deployment.get("schema"),
            "lifecycle_open_blockers": lifecycle.get("open_blockers", []),
            "deployment_not_claims": deployment.get("still_not_deployment_claims", []),
        },
        "audited_surfaces": audited_surfaces,
        "secret_material_classification": {
            "forbidden_in_logs_stores_errors": forbidden_secret_material,
            "public_or_public_commitment_material": public_material,
        },
        "instrumentation_evidence": {
            "public_state_serialization_kats_present": serialization_kats_present,
            "generated_rejection_report_kats_present": generated_rejection_report_kats_present,
            "sanitized_callback_wrappers_present": sanitized_callback_wrappers_present,
            "lifecycle_serialization_kat": contains(
                production_source,
                "lifecycle_file_store_serializes_only_public_account_state",
            ),
            "spent_ledger_serialization_kat": contains(
                production_source,
                "spent_slot_ledger_file_store_serializes_only_public_spent_records",
            ),
            "generated_rejection_report_kat": contains(
                production_source,
                "generated_rejection_reports_expose_only_public_metadata",
            ),
            "sanitized_callback_wrapper_kat": contains(
                production_source,
                "sanitized_callback_wrappers_do_not_echo_secret_errors",
            ),
            "sanitized_callback_failure_preserves_state_kat": all(
                contains(production_source, symbol)
                for symbol in [
                    "assert_eq!(manager.snapshot_sequence(), 0)",
                    "assert!(manager.account().get_slot(0).unwrap().is_unused())",
                    "assert!(ledger.is_empty())",
                    "assert!(stores.spent_ledger_store().load_ledger().unwrap().is_none())",
                ]
            ),
            "sanitized_callback_invocation_shape_kat": contains(
                production_source,
                "sanitized_callback_wrappers_have_bounded_invocation_shape",
            ),
            "callback_boundary_classification_table": contains(
                production_source,
                "PRODUCTION_CALLBACK_BOUNDARIES",
            ),
            "callback_boundary_classification_kat": contains(
                production_source,
                "callback_boundary_classification_marks_only_sanitized_paths_deployment_facing",
            ),
            "deployment_callback_boundary_gate": contains(
                production_source,
                "require_deployment_callback_boundaries",
            ),
            "deployment_callback_boundary_gate_kat": contains(
                production_source,
                "deployment_callback_boundary_gate_rejects_diagnostic_paths",
            ),
            "default_deployment_callback_profile": contains(
                production_source,
                "PRODUCTION_DEPLOYMENT_REQUIRED_CALLBACK_BOUNDARIES",
            ),
            "default_deployment_callback_profile_kat": contains(
                production_source,
                "default_deployment_callback_profile_is_complete_and_sanitized",
            ),
            "deployment_preflight": contains(
                production_source,
                "production_deployment_preflight",
            ),
            "deployment_preflight_fails_closed_kat": contains(
                production_source,
                "deployment_preflight_fails_closed_until_selected_verifier_and_reviews_land",
            ),
        },
        "open_blockers": open_blockers,
        "required_instrumentation": required_instrumentation,
        "interpretation": (
            "The current production reference has a useful typed boundary: public "
            "parsing, account commitments, lifecycle state, and spent-ledger state "
            "are separated from the secret-key signing callback.  That separation "
            "is not a constant-time proof.  Deployment requires replacing the "
            "legacy proof callback, auditing the secret-side implementation, and "
            "adding leakage/regression tests that enforce this classification."
        ),
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Constant-Time And Side-Channel Audit Memo",
        "",
        "This memo classifies the current q12289 production-reference surfaces for",
        "constant-time and secret-state review.  It is not a constant-time proof",
        "and not a deployment claim.",
        "",
        "## Current Claim",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- claim: `{payload['current_claim']}`",
        f"- executable signature bytes: `{payload['target']['executable_signature_bytes']}`",
        f"- reviewed transaction target bytes: `{payload['target']['reviewed_transaction_target_bytes']}`",
        f"- executable matches reviewed target: `{payload['target']['executable_matches_target']}`",
        f"- source artifacts: `{', '.join(payload['source_artifacts'])}`",
        "",
        "## Audited Surfaces",
        "",
        "| surface | status | claim |",
        "|---|---|---|",
    ]
    for surface in payload["audited_surfaces"]:
        lines.append(
            f"| {surface['surface']} | {surface['status']} | {surface['claim']} |"
        )
    lines.extend(
        [
            "",
            "## Secret Material Classification",
            "",
            "Forbidden in logs, stores, errors, and reports:",
        ]
    )
    for item in payload["secret_material_classification"][
        "forbidden_in_logs_stores_errors"
    ]:
        lines.append(f"- {item}")
    lines.extend(["", "Public or public-commitment material:"])
    for item in payload["secret_material_classification"][
        "public_or_public_commitment_material"
    ]:
        lines.append(f"- {item}")
    lines.extend(["", "## Open Blockers", ""])
    for blocker in payload["open_blockers"]:
        lines.append(f"- {blocker}")
    lines.extend(["", "## Required Instrumentation", ""])
    for item in payload["required_instrumentation"]:
        lines.append(
            f"- `{item['name']}` (`{item['status']}`): {item['requirement']}"
        )
    lines.extend(["", "## Interpretation", "", payload["interpretation"], ""])
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
